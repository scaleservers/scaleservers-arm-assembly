// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// UAL (Unified Assembly Language) emitter for ArmT32Instruction. See emit.rs for the entry points.
//
// VALIDATION: every rendering here is checked END-TO-END, not by hand-written string equality. The
// differential oracle (`tests/differential_oracle*.rs`) renders each sample via this emitter, feeds the text
// back to the real GNU/LLVM assembler, and asserts the assembled bytes equal our `encode()` -- so a wrong
// mnemonic/operand spelling fails the build (with ARM32_REQUIRE_ORACLE it is a hard failure in CI). That
// round-trip covers ~352 T32 forms (incl. the whole MVE + v8-M scalar-FP surface). `--lib` line coverage of
// this file therefore reads lower than the emitter's true validated fraction: the rarer forms are exercised
// by that integration oracle plus the never-panic / render-in-both-flavors sweeps in `robustness_tests.rs`.
//
// House notes on the rendering choices (chosen so the differential oracle can compare against the real
// assemblers/disassemblers):
//   * registers render lowercase, with r13/r14/r15 shown as their UAL aliases sp/lr/pc;
//   * the ARMv6-M data-processing T1 encodings are the flag-setting forms, so they carry the `s` suffix
//     (ADDS/SUBS/LSLS/...); the high-register T2/T1 move/add and CMP/CMN/TST forms do not;
//   * LDM/STM use the `ldmia`/`stmia` spelling;
//   * immediates print per `ArmAssemblySyntax` -- decimal `#N` (GNU) or hex `#0xN` (LLVM);
//   * PC-relative branch/literal operands print as offsets in the raw form, or as absolute `0x...` targets
//     in the address-aware form used by the disassembler.

// `String`/`Vec`/`ToString` (for `&str::to_string`) are not in the `no_std` prelude; pull them from `alloc`
// (the `format!` macro comes from the crate-level `#[macro_use] extern crate alloc`).
use crate::ArmT32Instruction;
use crate::emit::ArmAssemblySyntax;
use crate::enums::{
    Arm32DirectedRound, Arm32DoublePrecisionRegister, Arm32GeneralPurposeRegister,
    Arm32LowGeneralPurposeRegister, Arm32MveVectorRegister, Arm32SinglePrecisionRegister,
    Arm32VmovLaneSize, ArmT32CpsPrimaskEffect, ArmT32IndexMode, ArmT32InstructionCondition,
    ArmT32MemoryBarrierOption, ArmT32RegisterShift, ArmT32SpecialRegister,
};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// Render an already-emitted instruction string as it must appear inside an IT block under `condition`.
//
// Two adjustments turn the unconditional UAL into its in-IT form, matching LLVM and GNU `objdump` so the
// disassembly re-assembles with an external toolchain:
//   1. the condition code is spliced in after the base mnemonic but before any `.w` width qualifier
//      ("mov r0, r1" + EQ -> "moveq r0, r1");
//   2. for a *narrow* data-processing instruction the flag-setting `s` is dropped, because the 16-bit
//      encodings do NOT set the flags inside an IT block (`setflags = !InITBlock()`): "movs r2, #0" + LE
//      -> "movle r2, #0", not "movsle". A 32-bit `...s.w` form has an explicit S bit and keeps its `s`.
//
// Both the disassembler (armdasm) and the differential oracle render IT members through this function,
// so the in-IT spelling has a single source of truth.
pub fn apply_it_block_condition(rendered: &str, condition: ArmT32InstructionCondition) -> String {
    let (mnemonic, rest) = match rendered.find(char::is_whitespace) {
        Some(index) => (&rendered[..index], &rendered[index..]),
        None => (rendered, ""),
    };
    let (base, wide) = match mnemonic.strip_suffix(".w") {
        Some(stripped) => (stripped, ".w"),
        None => (mnemonic, ""),
    };
    let base = if wide.is_empty() {
        strip_it_flag_setting_suffix(base)
    } else {
        base
    };
    format!("{}{}{}{}", base, condition.ual_suffix(), wide, rest)
}

// Append a VPT-block then/else letter (`t`/`e`) to a rendered MVE instruction, inserted into the mnemonic
// just before its `.<type>` suffix (e.g. `vadd.i32 q0, q1, q2` + 't' -> `vaddt.i32 q0, q1, q2`). The
// predicated and plain forms share the same encoding, so this is purely a disassembly nicety.
pub fn apply_vpt_block_suffix(rendered: &str, letter: char) -> String {
    let mnemonic_end = rendered.find(char::is_whitespace).unwrap_or(rendered.len());
    let (mnemonic, rest) = rendered.split_at(mnemonic_end);
    let insert_at = mnemonic.find('.').unwrap_or(mnemonic.len());
    format!(
        "{}{}{}{}",
        &mnemonic[..insert_at],
        letter,
        &mnemonic[insert_at..],
        rest
    )
}

// Map a narrow flag-setting data-processing mnemonic to its non-flag-setting spelling for use inside an
// IT block ("movs" -> "mov"). Anything not in this exact set is returned unchanged, so a non-DP mnemonic
// that merely ends in `s` is never mangled.
fn strip_it_flag_setting_suffix(base: &str) -> &str {
    match base {
        "movs" => "mov",
        "mvns" => "mvn",
        "adds" => "add",
        "subs" => "sub",
        "adcs" => "adc",
        "sbcs" => "sbc",
        "rsbs" => "rsb",
        "muls" => "mul",
        "ands" => "and",
        "bics" => "bic",
        "orrs" => "orr",
        "eors" => "eor",
        "lsls" => "lsl",
        "lsrs" => "lsr",
        "asrs" => "asr",
        "rors" => "ror",
        other => other,
    }
}

impl ArmT32Instruction {
    /// Render this instruction as a UAL (Unified Assembly Language) string in the requested
    /// [`ArmAssemblySyntax`] (LLVM or GNU). With no surrounding address context, PC-relative branch /
    /// literal operands are shown as signed offsets.
    pub fn to_assembly_string(&self, syntax: ArmAssemblySyntax) -> String {
        self.render(None, syntax)
    }

    // Address-aware UAL: PC-relative branch / literal operands are resolved to absolute targets, given
    // the address at which this instruction begins. Used by the disassembler.
    pub fn to_assembly_string_at(
        &self,
        instruction_address: u32,
        syntax: ArmAssemblySyntax,
    ) -> String {
        self.render(Some(instruction_address), syntax)
    }

    fn render(&self, instruction_address: Option<u32>, syntax: ArmAssemblySyntax) -> String {
        match self {
            // ---- data processing (flag-setting T1 forms carry the `s` suffix) ----
            Self::Adc_Register_T1(rdn, rm) => format!("adcs {}, {}", low(rdn), low(rm)),
            Self::Add_Immediate_T1(rd, rn, imm3) => format!(
                "adds {}, {}, {}",
                low(rd),
                low(rn),
                imm(syntax, *imm3 as i64)
            ),
            Self::Add_Immediate_T2(rdn, imm8) => {
                format!("adds {}, {}", low(rdn), imm(syntax, *imm8 as i64))
            }
            Self::Add_Register_T1(rd, rn, rm) => {
                format!("adds {}, {}, {}", low(rd), low(rn), low(rm))
            }
            Self::Add_Register_T2(rdn, rm) => format!("add {}, {}", gpr(rdn), gpr(rm)),
            Self::And_Register_T1(rdn, rm) => format!("ands {}, {}", low(rdn), low(rm)),
            Self::Asr_Immediate_T1(rd, rm, decoded_imm5) => format!(
                "asrs {}, {}, {}",
                low(rd),
                low(rm),
                imm(syntax, *decoded_imm5 as i64)
            ),
            Self::Asr_Register_T1(rdn, rm) => format!("asrs {}, {}", low(rdn), low(rm)),
            Self::Bic_Register_T1(rdn, rm) => format!("bics {}, {}", low(rdn), low(rm)),
            Self::Eor_Register_T1(rdn, rm) => format!("eors {}, {}", low(rdn), low(rm)),
            Self::Lsl_Immediate_T1(rd, rm, imm5) => format!(
                "lsls {}, {}, {}",
                low(rd),
                low(rm),
                imm(syntax, *imm5 as i64)
            ),
            Self::Lsl_Register_T1(rdn, rm) => format!("lsls {}, {}", low(rdn), low(rm)),
            Self::Lsr_Immediate_T1(rd, rm, decoded_imm5) => format!(
                "lsrs {}, {}, {}",
                low(rd),
                low(rm),
                imm(syntax, *decoded_imm5 as i64)
            ),
            Self::Lsr_Register_T1(rdn, rm) => format!("lsrs {}, {}", low(rdn), low(rm)),
            Self::Mov_Immediate_T1(rd, imm8) => {
                format!("movs {}, {}", low(rd), imm(syntax, *imm8 as i64))
            }
            Self::Mov_Register_T1(rd, rm) => format!("mov {}, {}", gpr(rd), gpr(rm)),
            Self::Mov_Register_T2(rd, rm) => format!("movs {}, {}", low(rd), low(rm)),
            Self::Mul_T1(rdm, rn) => format!("muls {}, {}, {}", low(rdm), low(rn), low(rdm)),
            Self::Mvn_Register_T1(rd, rm) => format!("mvns {}, {}", low(rd), low(rm)),
            Self::Orr_Register_T1(rdn, rm) => format!("orrs {}, {}", low(rdn), low(rm)),
            Self::Ror_Register_T1(rdn, rm) => format!("rors {}, {}", low(rdn), low(rm)),
            Self::Rsb_Immediate_T1(rd, rn) => format!("rsbs {}, {}, #0", low(rd), low(rn)),
            Self::Sbc_Register_T1(rdn, rm) => format!("sbcs {}, {}", low(rdn), low(rm)),
            Self::Sub_Immediate_T1(rd, rn, imm3) => format!(
                "subs {}, {}, {}",
                low(rd),
                low(rn),
                imm(syntax, *imm3 as i64)
            ),
            Self::Sub_Immediate_T2(rdn, imm8) => {
                format!("subs {}, {}", low(rdn), imm(syntax, *imm8 as i64))
            }
            Self::Sub_Register_T1(rd, rn, rm) => {
                format!("subs {}, {}, {}", low(rd), low(rn), low(rm))
            }
            Self::Cmn_Register_T1(rn, rm) => format!("cmn {}, {}", low(rn), low(rm)),
            Self::Cmp_Immediate_T1(rn, imm8) => {
                format!("cmp {}, {}", low(rn), imm(syntax, *imm8 as i64))
            }
            Self::Cmp_Register_T1(rn, rm) => format!("cmp {}, {}", low(rn), low(rm)),
            Self::Cmp_Register_T2(rn, rm) => format!("cmp {}, {}", gpr(rn), gpr(rm)),
            Self::Tst_Register_T1(rn, rm) => format!("tst {}, {}", low(rn), low(rm)),

            // ---- stack-pointer arithmetic & address generation ----
            Self::Add_SpPlusImmediate_T1(rd, const10) => {
                format!("add {}, sp, {}", low(rd), imm(syntax, *const10 as i64))
            }
            Self::Add_SpPlusImmediate_T2(const9) => {
                format!("add sp, sp, {}", imm(syntax, *const9 as i64))
            }
            Self::Add_SpPlusRegister_T1(m) => format!("add {}, sp, {}", gpr(m), gpr(m)),
            Self::Add_SpPlusRegister_T2(rm) => format!("add sp, {}", gpr(rm)),
            Self::Sub_SpMinusImmediate_T1(const9) => {
                format!("sub sp, sp, {}", imm(syntax, *const9 as i64))
            }
            Self::Adr_T1(rd, const10) => match instruction_address {
                Some(address) => format!(
                    "adr {}, 0x{:08x}",
                    low(rd),
                    pc_relative_target(address, *const10 as i64, true)
                ),
                None => format!("adr {}, {}", low(rd), imm(syntax, *const10 as i64)),
            },

            // ---- branches ----
            Self::B_T1(cond, decoded_signed_imm9) => render_branch(
                &format!("b{}", cond_suffix(cond)),
                instruction_address,
                *decoded_signed_imm9 as i64,
                syntax,
            ),
            Self::B_T2(decoded_signed_imm12) => render_branch(
                "b",
                instruction_address,
                *decoded_signed_imm12 as i64,
                syntax,
            ),
            Self::Bl_T1(decoded_signed_imm25) => render_branch(
                "bl",
                instruction_address,
                *decoded_signed_imm25 as i64,
                syntax,
            ),
            // M7m wide branches and compare-and-branch
            Self::B_T4(offset) => render_branch("b.w", instruction_address, *offset as i64, syntax),
            Self::B_T3(cond, offset) => render_branch(
                &format!("b{}.w", cond_suffix(cond)),
                instruction_address,
                *offset as i64,
                syntax,
            ),
            Self::Cbz_T1(rn, offset) => {
                render_compare_branch("cbz", low(rn), instruction_address, *offset as i64, syntax)
            }
            Self::Cbnz_T1(rn, offset) => {
                render_compare_branch("cbnz", low(rn), instruction_address, *offset as i64, syntax)
            }
            // M7n IT -- `it{t/e}{t/e}{t/e} <firstcond>`; the suffix letters describe slots 2..N (the
            // disassembler applies the per-slot conditions to the following instructions, see armdasm).
            Self::It_T1(firstcond, mask) => render_it(firstcond, *mask),

            // ---- ARMv7E-M DSP M8a: saturating arithmetic ----
            Self::Qadd_T1(rd, rm, rn) => format!("qadd {}, {}, {}", gpr(rd), gpr(rm), gpr(rn)),
            Self::Qsub_T1(rd, rm, rn) => format!("qsub {}, {}, {}", gpr(rd), gpr(rm), gpr(rn)),
            Self::Qdadd_T1(rd, rm, rn) => format!("qdadd {}, {}, {}", gpr(rd), gpr(rm), gpr(rn)),
            Self::Qdsub_T1(rd, rm, rn) => format!("qdsub {}, {}, {}", gpr(rd), gpr(rm), gpr(rn)),

            // ---- ARMv7E-M DSP M8b: extend-and-add + 16-bit extends ----
            Self::Sxtab_T1(rd, rn, rm, rot) => format!(
                "sxtab {}, {}, {}{}",
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_rotation(*rot)
            ),
            Self::Uxtab_T1(rd, rn, rm, rot) => format!(
                "uxtab {}, {}, {}{}",
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_rotation(*rot)
            ),
            Self::Sxtah_T1(rd, rn, rm, rot) => format!(
                "sxtah {}, {}, {}{}",
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_rotation(*rot)
            ),
            Self::Uxtah_T1(rd, rn, rm, rot) => format!(
                "uxtah {}, {}, {}{}",
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_rotation(*rot)
            ),
            Self::Sxtab16_T1(rd, rn, rm, rot) => format!(
                "sxtab16 {}, {}, {}{}",
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_rotation(*rot)
            ),
            Self::Uxtab16_T1(rd, rn, rm, rot) => format!(
                "uxtab16 {}, {}, {}{}",
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_rotation(*rot)
            ),
            Self::Sxtb16_T1(rd, rm, rot) => {
                format!("sxtb16 {}, {}{}", gpr(rd), gpr(rm), render_rotation(*rot))
            }
            Self::Uxtb16_T1(rd, rm, rot) => {
                format!("uxtb16 {}, {}{}", gpr(rd), gpr(rm), render_rotation(*rot))
            }

            // ---- ARMv7E-M DSP M8c: pack / saturate16 / select / SAD ----
            Self::Pkhbt_T1(rd, rn, rm, lsl) => {
                if *lsl == 0 {
                    format!("pkhbt {}, {}, {}", gpr(rd), gpr(rn), gpr(rm))
                } else {
                    format!("pkhbt {}, {}, {}, lsl #{}", gpr(rd), gpr(rn), gpr(rm), lsl)
                }
            }
            Self::Pkhtb_T1(rd, rn, rm, asr) => {
                format!("pkhtb {}, {}, {}, asr #{}", gpr(rd), gpr(rn), gpr(rm), asr)
            }
            Self::Ssat16_T1(rd, sat_imm, rn) => format!(
                "ssat16 {}, {}, {}",
                gpr(rd),
                imm(syntax, *sat_imm as i64),
                gpr(rn)
            ),
            Self::Usat16_T1(rd, sat_imm, rn) => format!(
                "usat16 {}, {}, {}",
                gpr(rd),
                imm(syntax, *sat_imm as i64),
                gpr(rn)
            ),
            Self::Sel_T1(rd, rn, rm) => format!("sel {}, {}, {}", gpr(rd), gpr(rn), gpr(rm)),
            Self::Usad8_T1(rd, rn, rm) => format!("usad8 {}, {}, {}", gpr(rd), gpr(rn), gpr(rm)),
            Self::Usada8_T1(rd, rn, rm, ra) => {
                format!("usada8 {}, {}, {}, {}", gpr(rd), gpr(rn), gpr(rm), gpr(ra))
            }

            // ---- ARMv7E-M DSP M8d: parallel add/subtract (mnemonic = prefix + operation) ----
            Self::ParallelAddSub_T1(operation, prefix, rd, rn, rm) => format!(
                "{}{} {}, {}, {}",
                prefix.mnemonic(),
                operation.mnemonic(),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),

            // ---- ARMv7E-M DSP M8e: signed multiplies (the b/t halves and x/r suffixes are in the mnemonic) ----
            Self::Smul_T1(rd, rn, rm, n, m) => format!(
                "smul{}{} {}, {}, {}",
                top_or_bottom(*n),
                top_or_bottom(*m),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smulw_T1(rd, rn, rm, m) => format!(
                "smulw{} {}, {}, {}",
                top_or_bottom(*m),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smla_T1(rd, rn, rm, ra, n, m) => format!(
                "smla{}{} {}, {}, {}, {}",
                top_or_bottom(*n),
                top_or_bottom(*m),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Self::Smlaw_T1(rd, rn, rm, ra, m) => format!(
                "smlaw{} {}, {}, {}, {}",
                top_or_bottom(*m),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Self::Smlal_Halfword_T1(rdlo, rdhi, rn, rm, n, m) => format!(
                "smlal{}{} {}, {}, {}, {}",
                top_or_bottom(*n),
                top_or_bottom(*m),
                gpr(rdlo),
                gpr(rdhi),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smuad_T1(rd, rn, rm, x) => format!(
                "smuad{} {}, {}, {}",
                exchange(*x),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smusd_T1(rd, rn, rm, x) => format!(
                "smusd{} {}, {}, {}",
                exchange(*x),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smlad_T1(rd, rn, rm, ra, x) => format!(
                "smlad{} {}, {}, {}, {}",
                exchange(*x),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Self::Smlsd_T1(rd, rn, rm, ra, x) => format!(
                "smlsd{} {}, {}, {}, {}",
                exchange(*x),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Self::Smlald_T1(rdlo, rdhi, rn, rm, x) => format!(
                "smlald{} {}, {}, {}, {}",
                exchange(*x),
                gpr(rdlo),
                gpr(rdhi),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smlsld_T1(rdlo, rdhi, rn, rm, x) => format!(
                "smlsld{} {}, {}, {}, {}",
                exchange(*x),
                gpr(rdlo),
                gpr(rdhi),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smmul_T1(rd, rn, rm, round) => format!(
                "smmul{} {}, {}, {}",
                rounded(*round),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smmla_T1(rd, rn, rm, ra, round) => format!(
                "smmla{} {}, {}, {}, {}",
                rounded(*round),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Self::Smmls_T1(rd, rn, rm, ra, round) => format!(
                "smmls{} {}, {}, {}, {}",
                rounded(*round),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),

            // ---- ARMv7E-M FP M8f: load/store ----
            Self::Vldr_Single_T2(sd, rn, off) => format!(
                "vldr {}, {}",
                single(sd),
                render_fp_mem(rn, *off as i64, syntax)
            ),
            Self::Vstr_Single_T2(sd, rn, off) => format!(
                "vstr {}, {}",
                single(sd),
                render_fp_mem(rn, *off as i64, syntax)
            ),
            Self::Vldr_Double_T1(dd, rn, off) => format!(
                "vldr {}, {}",
                double(dd),
                render_fp_mem(rn, *off as i64, syntax)
            ),
            Self::Vstr_Double_T1(dd, rn, off) => format!(
                "vstr {}, {}",
                double(dd),
                render_fp_mem(rn, *off as i64, syntax)
            ),

            // ---- ARMv7E-M FP M8g: load/store multiple (SP+writeback prints as vpush/vpop) ----
            Self::Vldm_Single_T2(rn, wb, db, first, count) => {
                let list = fp_single_range(first, *count);
                if is_sp_register(rn) && *wb && !*db {
                    format!("vpop {}", list)
                } else {
                    format!(
                        "{} {}{}, {}",
                        if *db { "vldmdb" } else { "vldmia" },
                        gpr(rn),
                        writeback_suffix(*wb),
                        list
                    )
                }
            }
            Self::Vstm_Single_T2(rn, wb, db, first, count) => {
                let list = fp_single_range(first, *count);
                if is_sp_register(rn) && *wb && *db {
                    format!("vpush {}", list)
                } else {
                    format!(
                        "{} {}{}, {}",
                        if *db { "vstmdb" } else { "vstmia" },
                        gpr(rn),
                        writeback_suffix(*wb),
                        list
                    )
                }
            }
            Self::Vldm_Double_T1(rn, wb, db, first, count) => {
                let list = fp_double_range(first, *count);
                if is_sp_register(rn) && *wb && !*db {
                    format!("vpop {}", list)
                } else {
                    format!(
                        "{} {}{}, {}",
                        if *db { "vldmdb" } else { "vldmia" },
                        gpr(rn),
                        writeback_suffix(*wb),
                        list
                    )
                }
            }
            Self::Vstm_Double_T1(rn, wb, db, first, count) => {
                let list = fp_double_range(first, *count);
                if is_sp_register(rn) && *wb && *db {
                    format!("vpush {}", list)
                } else {
                    format!(
                        "{} {}{}, {}",
                        if *db { "vstmdb" } else { "vstmia" },
                        gpr(rn),
                        writeback_suffix(*wb),
                        list
                    )
                }
            }
            Self::FldmdbxFstmdbx_T1(load, rn, first, count) => format!(
                "{} {}!, {}",
                if *load { "fldmdbx" } else { "fstmdbx" },
                gpr(rn),
                fp_double_range(first, *count)
            ),
            // generic coprocessor instructions
            Self::Coproc_Mcr_T1(two, load, cp, opc1, rt, crn, crm, opc2) => format!(
                "{}{} p{}, {}, {}, c{}, c{}, {}",
                if *load { "mrc" } else { "mcr" },
                if *two { "2" } else { "" },
                cp,
                opc1,
                gpr(rt),
                crn,
                crm,
                opc2
            ),
            Self::Coproc_Cdp_T1(two, cp, opc1, crd, crn, crm, opc2) => format!(
                "cdp{} p{}, {}, c{}, c{}, c{}, {}",
                if *two { "2" } else { "" },
                cp,
                opc1,
                crd,
                crn,
                crm,
                opc2
            ),
            Self::Coproc_Mcrr_T1(two, load, cp, opc1, rt, rt2, crm) => format!(
                "{}{} p{}, {}, {}, {}, c{}",
                if *load { "mrrc" } else { "mcrr" },
                if *two { "2" } else { "" },
                cp,
                opc1,
                gpr(rt),
                gpr(rt2),
                crm
            ),
            Self::Coproc_Ldc_T1(two, long, load, cp, crd, rn, offset) => {
                let mem = if *offset == 0 {
                    format!("[{}]", gpr(rn))
                } else {
                    format!("[{}, #{}]", gpr(rn), offset)
                };
                format!(
                    "{}{}{} p{}, c{}, {}",
                    if *load { "ldc" } else { "stc" },
                    if *two { "2" } else { "" },
                    if *long { "l" } else { "" },
                    cp,
                    crd,
                    mem
                )
            }
            // PACBTI: BTI has no operands; PAC/AUT/PACBTI use the architectural R12, LR, SP.
            Self::PacbtiHint_T1(kind) => match kind {
                0 => "bti".to_string(),
                1 => "pac r12, lr, sp".to_string(),
                2 => "aut r12, lr, sp".to_string(),
                _ => "pacbti r12, lr, sp".to_string(),
            },
            Self::PacbtiData_T1(op, rd, rn, rm) => format!(
                "{} {}, {}, {}",
                ["pacg", "autg", "bxaut"][*op as usize],
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Self::Vscclrm_T1(double, first, count) => {
                // Widen to u16 and floor the register count at 1 so a degenerate/hostile decode (count 0/odd,
                // or first+count > 255) renders without an over/underflow panic.
                if *count == 0 {
                    "vscclrm {vpr}".to_string()
                } else if *double {
                    let n = (*count as u16 / 2).max(1);
                    format!("vscclrm {{d{}-d{}, vpr}}", first, *first as u16 + n - 1)
                } else {
                    format!(
                        "vscclrm {{s{}-s{}, vpr}}",
                        first,
                        *first as u16 + *count as u16 - 1
                    )
                }
            }
            // CDE: mnemonic = cx{1,2,3} + (d if dual) + (a if accumulate); the dual form names the Rd:Rd+1 pair.
            Self::Cde_Cx1_T1(acc, dual, cp, rd, imm) => {
                let m = format!(
                    "cx1{}{}",
                    if *dual { "d" } else { "" },
                    if *acc { "a" } else { "" }
                );
                let next = crate::enums::Arm32GeneralPurposeRegister::from_operand_bits(
                    rd.as_operand_bits() + 1,
                );
                if *dual {
                    format!("{} p{}, {}, {}, #{}", m, cp, gpr(rd), gpr(&next), imm)
                } else {
                    format!("{} p{}, {}, #{}", m, cp, gpr(rd), imm)
                }
            }
            Self::Cde_Cx2_T1(acc, dual, cp, rd, rn, imm) => {
                let m = format!(
                    "cx2{}{}",
                    if *dual { "d" } else { "" },
                    if *acc { "a" } else { "" }
                );
                let next = crate::enums::Arm32GeneralPurposeRegister::from_operand_bits(
                    rd.as_operand_bits() + 1,
                );
                if *dual {
                    format!(
                        "{} p{}, {}, {}, {}, #{}",
                        m,
                        cp,
                        gpr(rd),
                        gpr(&next),
                        gpr(rn),
                        imm
                    )
                } else {
                    format!("{} p{}, {}, {}, #{}", m, cp, gpr(rd), gpr(rn), imm)
                }
            }
            Self::Cde_Cx3_T1(acc, dual, cp, rd, rn, rm, imm) => {
                let m = format!(
                    "cx3{}{}",
                    if *dual { "d" } else { "" },
                    if *acc { "a" } else { "" }
                );
                let next = crate::enums::Arm32GeneralPurposeRegister::from_operand_bits(
                    rd.as_operand_bits() + 1,
                );
                if *dual {
                    format!(
                        "{} p{}, {}, {}, {}, {}, #{}",
                        m,
                        cp,
                        gpr(rd),
                        gpr(&next),
                        gpr(rn),
                        gpr(rm),
                        imm
                    )
                } else {
                    format!(
                        "{} p{}, {}, {}, {}, #{}",
                        m,
                        cp,
                        gpr(rd),
                        gpr(rn),
                        gpr(rm),
                        imm
                    )
                }
            }

            // ---- Branch Future. With a known address the b_label/target render as absolute hex; otherwise the
            // b_label shows the raw `boff` field and the target a `#<offset>` immediate (matching the assembler). ----
            Self::Bf_T1(boff, offset) => format!(
                "bf {}, {}",
                bf_blabel(instruction_address, *boff, syntax),
                bf_target(instruction_address, *offset as i64, syntax)
            ),
            Self::Bfl_T4(boff, offset) => format!(
                "bfl {}, {}",
                bf_blabel(instruction_address, *boff, syntax),
                bf_target(instruction_address, *offset as i64, syntax)
            ),
            Self::Bfx_T3(boff, rn) => format!(
                "bfx {}, {}",
                bf_blabel(instruction_address, *boff, syntax),
                gpr(rn)
            ),
            Self::Bflx_T5(boff, rn) => format!(
                "bflx {}, {}",
                bf_blabel(instruction_address, *boff, syntax),
                gpr(rn)
            ),
            Self::Bfcsel_T2(boff, offset, cond, t) => {
                let ba = bf_blabel(instruction_address, *boff + if *t { 2 } else { 1 }, syntax); // ba_label fall-through
                let c = cond_suffix(&ArmT32InstructionCondition::from_operand_bits(*cond));
                format!(
                    "bfcsel {}, {}, {}, {}",
                    bf_blabel(instruction_address, *boff, syntax),
                    bf_target(instruction_address, *offset as i64, syntax),
                    ba,
                    c
                )
            }

            // ---- CDE VCX1/VCX2/VCX3 (FP/vector custom datapath) ----
            Self::Vcx1_T1(acc, kind, cp, rd, imm) => format!(
                "vcx1{} p{}, {}, #{}",
                if *acc { "a" } else { "" },
                cp,
                vcx_reg(*kind, *rd),
                imm
            ),
            Self::Vcx2_T1(acc, kind, cp, rd, rn, imm) => format!(
                "vcx2{} p{}, {}, {}, #{}",
                if *acc { "a" } else { "" },
                cp,
                vcx_reg(*kind, *rd),
                vcx_reg(*kind, *rn),
                imm
            ),
            Self::Vcx3_T1(acc, kind, cp, rd, rn, rm, imm) => format!(
                "vcx3{} p{}, {}, {}, {}, #{}",
                if *acc { "a" } else { "" },
                cp,
                vcx_reg(*kind, *rd),
                vcx_reg(*kind, *rn),
                vcx_reg(*kind, *rm),
                imm
            ),

            // ---- ARMv7E-M FP M8h: data-processing ----
            Self::FpDataProcess3_Single(op, sd, sn, sm) => format!(
                "{}.f32 {}, {}, {}",
                op.mnemonic(),
                single(sd),
                single(sn),
                single(sm)
            ),
            Self::FpDataProcess3_Double(op, dd, dn, dm) => format!(
                "{}.f64 {}, {}, {}",
                op.mnemonic(),
                double(dd),
                double(dn),
                double(dm)
            ),
            Self::FpDataProcess2_Single(op, sd, sm) => {
                format!("{}.f32 {}, {}", op.mnemonic(), single(sd), single(sm))
            }
            Self::Vmovx_T1(insert, sd, sm) => format!(
                "{}.f16 {}, {}",
                if *insert { "vins" } else { "vmovx" },
                single(sd),
                single(sm)
            ),
            Self::Dbg_T1(option) => format!("dbg #{}", option),
            Self::Esb_T1 => "esb".to_string(),
            Self::Ssbb_T1 => "ssbb".to_string(),
            Self::Pssbb_T1 => "pssbb".to_string(),
            Self::Sb_T1 => "sb".to_string(),
            Self::Clrm_T1(list) => {
                let mut parts = Vec::new();
                for i in 0..=12u8 {
                    if list & (1 << i) != 0 {
                        parts.push(format!("r{}", i));
                    }
                }
                if list & (1 << 14) != 0 {
                    parts.push("lr".to_string());
                }
                if list & (1 << 15) != 0 {
                    parts.push("apsr".to_string());
                }
                format!("clrm {{{}}}", parts.join(", "))
            }
            Self::Vsel_Single_T1(cond, sd, sn, sm) => format!(
                "vsel{}.f32 {}, {}, {}",
                ["eq", "vs", "ge", "gt"][*cond as usize],
                single(sd),
                single(sn),
                single(sm)
            ),
            Self::Vsel_Double_T1(cond, dd, dn, dm) => format!(
                "vsel{}.f64 {}, {}, {}",
                ["eq", "vs", "ge", "gt"][*cond as usize],
                double(dd),
                double(dn),
                double(dm)
            ),
            Self::Csel_T1(op, rd, rn, rm, cond) => {
                // emit the CSET/CSETM/CINC/CINV/CNEG alias when Rn==Rm (inverted condition), matching disassemblers
                let same = rn.as_operand_bits() == rm.as_operand_bits();
                let is_pc = rn.as_operand_bits() == 15;
                let inv = crate::enums::ArmT32InstructionCondition::from_operand_bits(
                    cond.as_operand_bits() ^ 1,
                );
                match (*op, same, is_pc) {
                    (1, true, true) => format!("cset {}, {}", gpr(rd), inv.ual_suffix()),
                    (2, true, true) => format!("csetm {}, {}", gpr(rd), inv.ual_suffix()),
                    (1, true, false) => {
                        format!("cinc {}, {}, {}", gpr(rd), gpr(rn), inv.ual_suffix())
                    }
                    (2, true, false) => {
                        format!("cinv {}, {}, {}", gpr(rd), gpr(rn), inv.ual_suffix())
                    }
                    (3, true, _) => format!("cneg {}, {}, {}", gpr(rd), gpr(rn), inv.ual_suffix()),
                    _ => format!(
                        "{} {}, {}, {}, {}",
                        ["csel", "csinc", "csinv", "csneg"][*op as usize],
                        gpr(rd),
                        gpr(rn),
                        gpr(rm),
                        cond.ual_suffix()
                    ),
                }
            }
            Self::LongShiftImm_T1(op, rdalo, rdahi, imm) => format!(
                "{} {}, {}, #{}",
                ["lsll", "lsrl", "asrl"][*op as usize],
                gpr(rdalo),
                gpr(rdahi),
                imm
            ),
            Self::LongShiftReg_T1(op, rdalo, rdahi, rm) => format!(
                "{} {}, {}, {}",
                ["lsll", "lsrl", "asrl"][*op as usize],
                gpr(rdalo),
                gpr(rdahi),
                gpr(rm)
            ),
            Self::SatShiftImm_T1(op, rda, imm) => format!(
                "{} {}, #{}",
                ["uqshl", "urshr", "srshr", "sqshl"][*op as usize],
                gpr(rda),
                imm
            ),
            Self::SatShiftLongImm_T1(op, rdalo, rdahi, imm) => format!(
                "{} {}, {}, #{}",
                ["uqshll", "urshrl", "srshrl", "sqshll"][*op as usize],
                gpr(rdalo),
                gpr(rdahi),
                imm
            ),
            Self::SatShiftReg_T1(signed, rda, rm) => format!(
                "{} {}, {}",
                if *signed { "sqrshr" } else { "uqrshl" },
                gpr(rda),
                gpr(rm)
            ),
            Self::SatShiftLongReg_T1(signed, rdalo, rdahi, rm, sat48) => format!(
                "{} {}, {}, #{}, {}",
                if *signed { "sqrshrl" } else { "uqrshll" },
                gpr(rdalo),
                gpr(rdahi),
                if *sat48 { 48 } else { 64 },
                gpr(rm)
            ),
            Self::Vrintr_Single_T1(sd, sm) => format!("vrintr.f32 {}, {}", single(sd), single(sm)),
            Self::Vrintr_Double_T1(dd, dm) => format!("vrintr.f64 {}, {}", double(dd), double(dm)),
            Self::Vmaxnm_Single_T1(sd, sn, sm) => {
                format!("vmaxnm.f32 {}, {}, {}", single(sd), single(sn), single(sm))
            }
            Self::Vmaxnm_Double_T1(dd, dn, dm) => {
                format!("vmaxnm.f64 {}, {}, {}", double(dd), double(dn), double(dm))
            }
            Self::Vminnm_Single_T1(sd, sn, sm) => {
                format!("vminnm.f32 {}, {}, {}", single(sd), single(sn), single(sm))
            }
            Self::Vminnm_Double_T1(dd, dn, dm) => {
                format!("vminnm.f64 {}, {}, {}", double(dd), double(dn), double(dm))
            }
            Self::Vrint_Directed_Single_T1(mode, sd, sm) => format!(
                "vrint{}.f32 {}, {}",
                t32_directed_round(*mode),
                single(sd),
                single(sm)
            ),
            Self::Vrint_Directed_Double_T1(mode, dd, dm) => format!(
                "vrint{}.f64 {}, {}",
                t32_directed_round(*mode),
                double(dd),
                double(dm)
            ),
            Self::Vrintz_Single_T1(sd, sm) => format!("vrintz.f32 {}, {}", single(sd), single(sm)),
            Self::Vrintz_Double_T1(dd, dm) => format!("vrintz.f64 {}, {}", double(dd), double(dm)),
            Self::Vrintx_Single_T1(sd, sm) => format!("vrintx.f32 {}, {}", single(sd), single(sm)),
            Self::Vrintx_Double_T1(dd, dm) => format!("vrintx.f64 {}, {}", double(dd), double(dm)),
            Self::Vcvt_Directed_FromSingle_T1(mode, sd, sm, signed) => format!(
                "vcvt{}.{}.f32 {}, {}",
                t32_directed_round(*mode),
                if *signed { "s32" } else { "u32" },
                single(sd),
                single(sm)
            ),
            Self::Vcvt_Directed_FromDouble_T1(mode, sd, dm, signed) => format!(
                "vcvt{}.{}.f64 {}, {}",
                t32_directed_round(*mode),
                if *signed { "s32" } else { "u32" },
                single(sd),
                double(dm)
            ),
            Self::Vjcvt_T1(sd, dm) => format!("vjcvt.s32.f64 {}, {}", single(sd), double(dm)),
            Self::FpDataProcess2_Double(op, dd, dm) => {
                format!("{}.f64 {}, {}", op.mnemonic(), double(dd), double(dm))
            }

            // ---- ARMv7E-M FP M8i: compare / FPSCR transfer / core<->FP move ----
            Self::Vcmp_Single_T1(sd, sm, e) => format!(
                "vcmp{}.f32 {}, {}",
                if *e { "e" } else { "" },
                single(sd),
                single(sm)
            ),
            Self::Vcmp_Double_T1(dd, dm, e) => format!(
                "vcmp{}.f64 {}, {}",
                if *e { "e" } else { "" },
                double(dd),
                double(dm)
            ),
            Self::Vcmp_Zero_Single_T2(sd, e) => {
                format!("vcmp{}.f32 {}, #0.0", if *e { "e" } else { "" }, single(sd))
            }
            Self::Vcmp_Zero_Double_T2(dd, e) => {
                format!("vcmp{}.f64 {}, #0.0", if *e { "e" } else { "" }, double(dd))
            }
            Self::Vmrs_T1(rt) => format!("vmrs {}, fpscr", gpr(rt)),
            Self::Vmrs_Apsr_Nzcv_T1 => "vmrs APSR_nzcv, fpscr".to_string(),
            Self::Vmsr_T1(rt) => format!("vmsr fpscr, {}", gpr(rt)),
            Self::Vmov_Core_To_Single_T1(sn, rt) => format!("vmov {}, {}", single(sn), gpr(rt)),
            Self::Vmov_Single_To_Core_T1(rt, sn) => format!("vmov {}, {}", gpr(rt), single(sn)),
            Self::Vmov_Core_To_Scalar_T1(size, index, dd, rt) => format!(
                "vmov.{} {}[{}], {}",
                size.suffix(),
                double(dd),
                index,
                gpr(rt)
            ),
            Self::Vmov_Scalar_To_Core_T1(unsigned, size, index, rt, dn) => {
                let dt = match size {
                    Arm32VmovLaneSize::Word => "32".to_string(),
                    s => format!("{}{}", if *unsigned { "u" } else { "s" }, s.suffix()),
                };
                format!("vmov.{} {}, {}[{}]", dt, gpr(rt), double(dn), index)
            }
            Self::Vcvt_FloatToInt_FromSingle_T1(sd, sm, signed, round) => format!(
                "vcvt{}.{}.f32 {}, {}",
                if *round { "" } else { "r" },
                int_type(*signed),
                single(sd),
                single(sm)
            ),
            Self::Vcvt_FloatToInt_FromDouble_T1(sd, dm, signed, round) => format!(
                "vcvt{}.{}.f64 {}, {}",
                if *round { "" } else { "r" },
                int_type(*signed),
                single(sd),
                double(dm)
            ),
            Self::Vcvt_IntToFloat_ToSingle_T1(sd, sm, signed) => format!(
                "vcvt.f32.{} {}, {}",
                int_type(*signed),
                single(sd),
                single(sm)
            ),
            Self::Vcvt_IntToFloat_ToDouble_T1(dd, sm, signed) => format!(
                "vcvt.f64.{} {}, {}",
                int_type(*signed),
                double(dd),
                single(sm)
            ),
            Self::Vcvt_Single_To_Double_T1(dd, sm) => {
                format!("vcvt.f64.f32 {}, {}", double(dd), single(sm))
            }
            Self::Vcvt_Double_To_Single_T1(sd, dm) => {
                format!("vcvt.f32.f64 {}, {}", single(sd), double(dm))
            }

            // ---- ARMv7E-M FP M8i (final corners) ----
            Self::Vmov_Immediate_Single_T1(sd, imm8) => format!(
                "vmov.f32 {}, #{:?}",
                single(sd),
                crate::vfp_expand_imm8_to_f32(*imm8) as f64
            ),
            Self::Vmov_Immediate_Double_T1(dd, imm8) => format!(
                "vmov.f64 {}, #{:?}",
                double(dd),
                crate::vfp_expand_imm8_to_f64(*imm8)
            ),
            Self::Vmov_CorePair_To_Double_T1(dm, rt, rt2) => {
                format!("vmov {}, {}, {}", double(dm), gpr(rt), gpr(rt2))
            }
            Self::Vmov_Double_To_CorePair_T1(rt, rt2, dm) => {
                format!("vmov {}, {}, {}", gpr(rt), gpr(rt2), double(dm))
            }
            Self::Vmov_CorePair_To_Singles_T1(sm, rt, rt2) => format!(
                "vmov s{}, s{}, {}, {}",
                sm.number(),
                sm.number() + 1,
                gpr(rt),
                gpr(rt2)
            ),
            Self::Vmov_Singles_To_CorePair_T1(rt, rt2, sm) => format!(
                "vmov {}, {}, s{}, s{}",
                gpr(rt),
                gpr(rt2),
                sm.number(),
                sm.number() + 1
            ),
            Self::Vcvt_HalfToSingle_T1(sd, sm, top) => format!(
                "vcvt{}.f32.f16 {}, {}",
                top_or_bottom(*top),
                single(sd),
                single(sm)
            ),
            Self::Vcvt_SingleToHalf_T1(sd, sm, top) => format!(
                "vcvt{}.f16.f32 {}, {}",
                top_or_bottom(*top),
                single(sd),
                single(sm)
            ),
            Self::Vcvt_FloatToFixed_Single_T1(sd, signed, bits32, frac) => format!(
                "vcvt.{}.f32 {}, {}, #{}",
                fixed_type(*signed, *bits32),
                single(sd),
                single(sd),
                frac
            ),
            Self::Vcvt_FloatToFixed_Double_T1(dd, signed, bits32, frac) => format!(
                "vcvt.{}.f64 {}, {}, #{}",
                fixed_type(*signed, *bits32),
                double(dd),
                double(dd),
                frac
            ),
            Self::Vcvt_FixedToFloat_Single_T1(sd, signed, bits32, frac) => format!(
                "vcvt.f32.{} {}, {}, #{}",
                fixed_type(*signed, *bits32),
                single(sd),
                single(sd),
                frac
            ),
            Self::Vcvt_FixedToFloat_Double_T1(dd, signed, bits32, frac) => format!(
                "vcvt.f64.{} {}, {}, #{}",
                fixed_type(*signed, *bits32),
                double(dd),
                double(dd),
                frac
            ),
            Self::Blx_Register_T1(rm) => format!("blx {}", gpr(rm)),
            Self::Bx_T1(rm) => format!("bx {}", gpr(rm)),

            // ---- loads / stores (single) ----
            Self::Ldr_Immediate_T1(rt, rn, decoded_imm7) => format!(
                "ldr {}, [{}, {}]",
                low(rt),
                low(rn),
                imm(syntax, *decoded_imm7 as i64)
            ),
            Self::Ldr_Immediate_T2(rt, decoded_imm10) => format!(
                "ldr {}, [sp, {}]",
                low(rt),
                imm(syntax, *decoded_imm10 as i64)
            ),
            Self::Ldr_Literal_T1(rt, decoded_imm10) => render_literal_load(
                "ldr",
                low(rt),
                instruction_address,
                *decoded_imm10 as i64,
                syntax,
            ),
            Self::Ldr_Register_T1(rt, rn, rm) => {
                format!("ldr {}, [{}, {}]", low(rt), low(rn), low(rm))
            }
            Self::Ldrb_Immediate_T1(rt, rn, imm5) => format!(
                "ldrb {}, [{}, {}]",
                low(rt),
                low(rn),
                imm(syntax, *imm5 as i64)
            ),
            Self::Ldrb_Register_T1(rt, rn, rm) => {
                format!("ldrb {}, [{}, {}]", low(rt), low(rn), low(rm))
            }
            Self::Ldrh_Immediate_T1(rt, rn, decoded_imm6) => format!(
                "ldrh {}, [{}, {}]",
                low(rt),
                low(rn),
                imm(syntax, *decoded_imm6 as i64)
            ),
            Self::Ldrh_Register_T1(rt, rn, rm) => {
                format!("ldrh {}, [{}, {}]", low(rt), low(rn), low(rm))
            }
            Self::Ldrsb_Register_T1(rt, rn, rm) => {
                format!("ldrsb {}, [{}, {}]", low(rt), low(rn), low(rm))
            }
            Self::Ldrsh_Register_T1(rt, rn, rm) => {
                format!("ldrsh {}, [{}, {}]", low(rt), low(rn), low(rm))
            }
            Self::Str_Immediate_T1(rt, rn, decoded_imm7) => format!(
                "str {}, [{}, {}]",
                low(rt),
                low(rn),
                imm(syntax, *decoded_imm7 as i64)
            ),
            Self::Str_Immediate_T2(rt, decoded_imm10) => format!(
                "str {}, [sp, {}]",
                low(rt),
                imm(syntax, *decoded_imm10 as i64)
            ),
            Self::Str_Register_T1(rt, rn, rm) => {
                format!("str {}, [{}, {}]", low(rt), low(rn), low(rm))
            }
            Self::Strb_Immediate_T1(rt, rn, imm5) => format!(
                "strb {}, [{}, {}]",
                low(rt),
                low(rn),
                imm(syntax, *imm5 as i64)
            ),
            Self::Strb_Register_T1(rt, rn, rm) => {
                format!("strb {}, [{}, {}]", low(rt), low(rn), low(rm))
            }
            Self::Strh_Immediate_T1(rt, rn, decoded_imm6) => format!(
                "strh {}, [{}, {}]",
                low(rt),
                low(rn),
                imm(syntax, *decoded_imm6 as i64)
            ),
            Self::Strh_Register_T1(rt, rn, rm) => {
                format!("strh {}, [{}, {}]", low(rt), low(rn), low(rm))
            }

            // ---- loads / stores (multiple) ----
            Self::Ldm_T1(rn, registers) => {
                let writeback = if registers.iter().any(|register| register == rn) {
                    ""
                } else {
                    "!"
                };
                format!(
                    "ldmia {}{}, {}",
                    low(rn),
                    writeback,
                    low_register_list(registers)
                )
            }
            Self::Stm_T1(rn, registers) => {
                format!("stmia {}!, {}", low(rn), low_register_list(registers))
            }
            Self::Pop_T1(registers) => format!("pop {}", register_list(registers)),
            Self::Push_T1(registers) => format!("push {}", register_list(registers)),

            // ---- byte-reverse / extend ----
            Self::Rev_T1(rd, rm) => format!("rev {}, {}", low(rd), low(rm)),
            Self::Rev16_T1(rd, rm) => format!("rev16 {}, {}", low(rd), low(rm)),
            Self::Revsh_T1(rd, rm) => format!("revsh {}, {}", low(rd), low(rm)),
            Self::Sxtb_T1(rd, rm) => format!("sxtb {}, {}", low(rd), low(rm)),
            Self::Sxth_T1(rd, rm) => format!("sxth {}, {}", low(rd), low(rm)),
            Self::Uxtb_T1(rd, rm) => format!("uxtb {}, {}", low(rd), low(rm)),
            Self::Uxth_T1(rd, rm) => format!("uxth {}, {}", low(rd), low(rm)),

            // ---- system / special registers / hints / barriers ----
            Self::Mrs_T1(rd, spec_reg) => {
                format!("mrs {}, {}", gpr(rd), special_register(spec_reg))
            }
            Self::Msr_Register_T1(spec_reg, rn) => {
                format!("msr {}, {}", special_register(spec_reg), gpr(rn))
            }
            Self::Cps_T1(primask_effect) => match primask_effect {
                ArmT32CpsPrimaskEffect::InterruptEnable => "cpsie i".to_string(),
                ArmT32CpsPrimaskEffect::InterruptDisable => "cpsid i".to_string(),
            },
            Self::Setpan_T1(pan) => format!("setpan {}", imm(syntax, if *pan { 1 } else { 0 })),
            Self::Dmb_T1(option) => format!("dmb {}", barrier_option(option)),
            Self::Dsb_T1(option) => format!("dsb {}", barrier_option(option)),
            Self::Isb_T1(option) => format!("isb {}", barrier_option(option)),

            // ---- service / breakpoint / undefined ----
            Self::Bkpt_T1(imm8) => format!("bkpt {}", imm(syntax, *imm8 as i64)),
            Self::Hlt_T1(imm6) => format!("hlt {}", imm(syntax, *imm6 as i64)),
            Self::Svc_T1(imm8) => format!("svc {}", imm(syntax, *imm8 as i64)),
            Self::Udf_T1(imm8) => format!("udf {}", imm(syntax, *imm8 as i64)),
            Self::Udf_T2(imm16) => format!("udf.w {}", imm(syntax, *imm16 as i64)),

            // ---- no-operand hints ----
            Self::Nop_T1 => "nop".to_string(),
            Self::Sev_T1 => "sev".to_string(),
            Self::Wfe_T1 => "wfe".to_string(),
            Self::Wfi_T1 => "wfi".to_string(),
            Self::Yield_T1 => "yield".to_string(),

            // ---- ARMv7-M (Thumb-2) additions ----
            Self::Mov_Immediate_T3(rd, imm16) => {
                format!("movw {}, {}", gpr(rd), imm(syntax, *imm16 as i64))
            }
            Self::Movt_T1(rd, imm16) => format!("movt {}, {}", gpr(rd), imm(syntax, *imm16 as i64)),
            Self::Mul_T2(rd, rn, rm) => format!("mul {}, {}, {}", gpr(rd), gpr(rn), gpr(rm)),
            Self::Mla_T1(rd, rn, rm, ra) => {
                format!("mla {}, {}, {}, {}", gpr(rd), gpr(rn), gpr(rm), gpr(ra))
            }
            Self::Mls_T1(rd, rn, rm, ra) => {
                format!("mls {}, {}, {}, {}", gpr(rd), gpr(rn), gpr(rm), gpr(ra))
            }
            Self::Sdiv_T1(rd, rn, rm) => format!("sdiv {}, {}, {}", gpr(rd), gpr(rn), gpr(rm)),
            Self::Udiv_T1(rd, rn, rm) => format!("udiv {}, {}, {}", gpr(rd), gpr(rn), gpr(rm)),
            Self::Clz_T1(rd, rm) => format!("clz {}, {}", gpr(rd), gpr(rm)),

            // ---- ARMv7-M batch M7b ----
            Self::Rbit_T1(rd, rm) => format!("rbit {}, {}", gpr(rd), gpr(rm)),
            Self::Ubfx_T1(rd, rn, lsb, width) => format!(
                "ubfx {}, {}, {}, {}",
                gpr(rd),
                gpr(rn),
                imm(syntax, *lsb as i64),
                imm(syntax, *width as i64)
            ),
            Self::Sbfx_T1(rd, rn, lsb, width) => format!(
                "sbfx {}, {}, {}, {}",
                gpr(rd),
                gpr(rn),
                imm(syntax, *lsb as i64),
                imm(syntax, *width as i64)
            ),
            Self::Bfi_T1(rd, rn, lsb, width) => format!(
                "bfi {}, {}, {}, {}",
                gpr(rd),
                gpr(rn),
                imm(syntax, *lsb as i64),
                imm(syntax, *width as i64)
            ),
            Self::Bfc_T1(rd, lsb, width) => format!(
                "bfc {}, {}, {}",
                gpr(rd),
                imm(syntax, *lsb as i64),
                imm(syntax, *width as i64)
            ),
            Self::Ldr_Immediate_T3(rt, rn, imm12) => format!(
                "ldr.w {}, [{}, {}]",
                gpr(rt),
                gpr(rn),
                imm(syntax, *imm12 as i64)
            ),
            Self::Str_Immediate_T3(rt, rn, imm12) => format!(
                "str.w {}, [{}, {}]",
                gpr(rt),
                gpr(rn),
                imm(syntax, *imm12 as i64)
            ),

            // ---- ARMv7-M batch M7c ----
            Self::Ldrex_T1(rt, rn, offset) => {
                if *offset == 0 {
                    format!("ldrex {}, [{}]", gpr(rt), gpr(rn))
                } else {
                    format!(
                        "ldrex {}, [{}, {}]",
                        gpr(rt),
                        gpr(rn),
                        imm(syntax, *offset as i64)
                    )
                }
            }
            Self::Strex_T1(rd, rt, rn, offset) => {
                if *offset == 0 {
                    format!("strex {}, {}, [{}]", gpr(rd), gpr(rt), gpr(rn))
                } else {
                    format!(
                        "strex {}, {}, [{}, {}]",
                        gpr(rd),
                        gpr(rt),
                        gpr(rn),
                        imm(syntax, *offset as i64)
                    )
                }
            }
            Self::Ldrexb_T1(rt, rn) => format!("ldrexb {}, [{}]", gpr(rt), gpr(rn)),
            Self::Strexb_T1(rd, rt, rn) => {
                format!("strexb {}, {}, [{}]", gpr(rd), gpr(rt), gpr(rn))
            }
            // ARMv8-M load-acquire / store-release: size 0=byte (b), 1=halfword (h), 2=word ("")
            Self::LoadAcquire_T1(size, exclusive, rt, rn) => format!(
                "lda{}{} {}, [{}]",
                if *exclusive { "ex" } else { "" },
                ["b", "h", ""][*size as usize],
                gpr(rt),
                gpr(rn)
            ),
            Self::StoreRelease_T1(size, rt, rn) => format!(
                "stl{} {}, [{}]",
                ["b", "h", ""][*size as usize],
                gpr(rt),
                gpr(rn)
            ),
            Self::StoreReleaseExclusive_T1(size, rd, rt, rn) => format!(
                "stlex{} {}, {}, [{}]",
                ["b", "h", ""][*size as usize],
                gpr(rd),
                gpr(rt),
                gpr(rn)
            ),
            // unprivileged load/store (LDRT/STRT family); `[Rn]` is omitted when imm8 == 0
            Self::UnprivLoadStore_T1(load, signed, size, rt, rn, imm8) => {
                let mnemonic = if *load {
                    format!(
                        "ldr{}{}t",
                        if *signed { "s" } else { "" },
                        ["b", "h", ""][*size as usize]
                    )
                } else {
                    format!("str{}t", ["b", "h", ""][*size as usize])
                };
                if *imm8 == 0 {
                    format!("{} {}, [{}]", mnemonic, gpr(rt), gpr(rn))
                } else {
                    format!("{} {}, [{}, #{}]", mnemonic, gpr(rt), gpr(rn), imm8)
                }
            }
            Self::Ldrexh_T1(rt, rn) => format!("ldrexh {}, [{}]", gpr(rt), gpr(rn)),
            Self::Strexh_T1(rd, rt, rn) => {
                format!("strexh {}, {}, [{}]", gpr(rd), gpr(rt), gpr(rn))
            }
            Self::Clrex_T1 => "clrex".to_string(),
            Self::Tbb_T1(rn, rm) => format!("tbb [{}, {}]", gpr(rn), gpr(rm)),
            Self::Tbh_T1(rn, rm) => format!("tbh [{}, {}, lsl #1]", gpr(rn), gpr(rm)),

            // ---- ARMv7-M batch M7d: data processing (modified immediate); always `.w` (forces the wide form) ----
            Self::Mov_Immediate_T2(rd, constant, set_flags) => format!(
                "mov{} {}, {}",
                wide_flag(*set_flags),
                gpr(rd),
                imm(syntax, *constant as i64)
            ),
            Self::Mvn_Immediate_T1(rd, constant, set_flags) => format!(
                "mvn{} {}, {}",
                wide_flag(*set_flags),
                gpr(rd),
                imm(syntax, *constant as i64)
            ),
            Self::And_Immediate_T1(rd, rn, constant, set_flags) => format!(
                "and{} {}, {}, {}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Bic_Immediate_T1(rd, rn, constant, set_flags) => format!(
                "bic{} {}, {}, {}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Orr_Immediate_T1(rd, rn, constant, set_flags) => format!(
                "orr{} {}, {}, {}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Eor_Immediate_T1(rd, rn, constant, set_flags) => format!(
                "eor{} {}, {}, {}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Add_Immediate_T3(rd, rn, constant, set_flags) => format!(
                "add{} {}, {}, {}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Sub_Immediate_T3(rd, rn, constant, set_flags) => format!(
                "sub{} {}, {}, {}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Tst_Immediate_T1(rn, constant) => {
                format!("tst.w {}, {}", gpr(rn), imm(syntax, *constant as i64))
            }
            Self::Teq_Immediate_T1(rn, constant) => {
                format!("teq.w {}, {}", gpr(rn), imm(syntax, *constant as i64))
            }
            Self::Cmn_Immediate_T1(rn, constant) => {
                format!("cmn.w {}, {}", gpr(rn), imm(syntax, *constant as i64))
            }
            Self::Cmp_Immediate_T2(rn, constant) => {
                format!("cmp.w {}, {}", gpr(rn), imm(syntax, *constant as i64))
            }

            // ---- ARMv7-M batch M7e: emitted WITHOUT `.w` (no narrow form; clang rejects `.w` here) ----
            Self::Adc_Immediate_T1(rd, rn, constant, set_flags) => format!(
                "adc{} {}, {}, {}",
                flag_suffix(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Sbc_Immediate_T1(rd, rn, constant, set_flags) => format!(
                "sbc{} {}, {}, {}",
                flag_suffix(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Rsb_Immediate_T2(rd, rn, constant, set_flags) => format!(
                "rsb{} {}, {}, {}",
                flag_suffix(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),
            Self::Orn_Immediate_T1(rd, rn, constant, set_flags) => format!(
                "orn{} {}, {}, {}",
                flag_suffix(*set_flags),
                gpr(rd),
                gpr(rn),
                imm(syntax, *constant as i64)
            ),

            // ---- ARMv7-M batch M7f: data processing (shifted register), always `.w` ----
            Self::Add_Register_T3(rd, rn, rm, shift, set_flags) => format!(
                "add{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::Sub_Register_T2(rd, rn, rm, shift, set_flags) => format!(
                "sub{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::And_Register_T2(rd, rn, rm, shift, set_flags) => format!(
                "and{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::Orr_Register_T2(rd, rn, rm, shift, set_flags) => format!(
                "orr{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::Eor_Register_T2(rd, rn, rm, shift, set_flags) => format!(
                "eor{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::Bic_Register_T2(rd, rn, rm, shift, set_flags) => format!(
                "bic{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),

            // ---- ARMv7-M batch M7g: shifted-register alias forms ----
            // MOV (register) / the shift mnemonics / RRX all share one encoding; the shift picks the mnemonic.
            Self::Mov_Register_T3(rd, rm, shift, set_flags) => {
                render_mov_register(rd, rm, shift, *set_flags)
            }
            Self::Mvn_Register_T2(rd, rm, shift, set_flags) => format!(
                "mvn{} {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rm),
                render_shift(shift)
            ),
            // ADC/SBC/RSB/ORN register accept (and render with) `.w`
            Self::Adc_Register_T2(rd, rn, rm, shift, set_flags) => format!(
                "adc{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::Sbc_Register_T2(rd, rn, rm, shift, set_flags) => format!(
                "sbc{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::Rsb_Register_T1(rd, rn, rm, shift, set_flags) => format!(
                "rsb{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::Orn_Register_T1(rd, rn, rm, shift, set_flags) => format!(
                "orn{} {}, {}, {}{}",
                wide_flag(*set_flags),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                render_shift(shift)
            ),
            Self::Tst_Register_T2(rn, rm, shift) => {
                format!("tst.w {}, {}{}", gpr(rn), gpr(rm), render_shift(shift))
            }
            Self::Teq_Register_T1(rn, rm, shift) => {
                format!("teq.w {}, {}{}", gpr(rn), gpr(rm), render_shift(shift))
            }
            Self::Cmn_Register_T2(rn, rm, shift) => {
                format!("cmn.w {}, {}{}", gpr(rn), gpr(rm), render_shift(shift))
            }
            Self::Cmp_Register_T3(rn, rm, shift) => {
                format!("cmp.w {}, {}{}", gpr(rn), gpr(rm), render_shift(shift))
            }

            // ---- ARMv7-M batch M7h: wide byte/half load/store + register offset ----
            Self::Ldrb_Immediate_T2(rt, rn, imm12) => format!(
                "ldrb.w {}, [{}, {}]",
                gpr(rt),
                gpr(rn),
                imm(syntax, *imm12 as i64)
            ),
            Self::Strb_Immediate_T2(rt, rn, imm12) => format!(
                "strb.w {}, [{}, {}]",
                gpr(rt),
                gpr(rn),
                imm(syntax, *imm12 as i64)
            ),
            Self::Ldrh_Immediate_T2(rt, rn, imm12) => format!(
                "ldrh.w {}, [{}, {}]",
                gpr(rt),
                gpr(rn),
                imm(syntax, *imm12 as i64)
            ),
            Self::Strh_Immediate_T2(rt, rn, imm12) => format!(
                "strh.w {}, [{}, {}]",
                gpr(rt),
                gpr(rn),
                imm(syntax, *imm12 as i64)
            ),
            Self::Ldrsb_Immediate_T1(rt, rn, imm12) => format!(
                "ldrsb.w {}, [{}, {}]",
                gpr(rt),
                gpr(rn),
                imm(syntax, *imm12 as i64)
            ),
            Self::Ldrsh_Immediate_T1(rt, rn, imm12) => format!(
                "ldrsh.w {}, [{}, {}]",
                gpr(rt),
                gpr(rn),
                imm(syntax, *imm12 as i64)
            ),
            Self::Ldr_Register_T2(rt, rn, rm, lsl) => format!(
                "ldr.w {}, {}",
                gpr(rt),
                render_register_offset(rn, rm, *lsl)
            ),
            Self::Str_Register_T2(rt, rn, rm, lsl) => format!(
                "str.w {}, {}",
                gpr(rt),
                render_register_offset(rn, rm, *lsl)
            ),
            Self::Ldrb_Register_T2(rt, rn, rm, lsl) => format!(
                "ldrb.w {}, {}",
                gpr(rt),
                render_register_offset(rn, rm, *lsl)
            ),
            Self::Strb_Register_T2(rt, rn, rm, lsl) => format!(
                "strb.w {}, {}",
                gpr(rt),
                render_register_offset(rn, rm, *lsl)
            ),
            Self::Ldrh_Register_T2(rt, rn, rm, lsl) => format!(
                "ldrh.w {}, {}",
                gpr(rt),
                render_register_offset(rn, rm, *lsl)
            ),
            Self::Strh_Register_T2(rt, rn, rm, lsl) => format!(
                "strh.w {}, {}",
                gpr(rt),
                render_register_offset(rn, rm, *lsl)
            ),
            Self::Ldrsb_Register_T2(rt, rn, rm, lsl) => format!(
                "ldrsb.w {}, {}",
                gpr(rt),
                render_register_offset(rn, rm, *lsl)
            ),
            Self::Ldrsh_Register_T2(rt, rn, rm, lsl) => format!(
                "ldrsh.w {}, {}",
                gpr(rt),
                render_register_offset(rn, rm, *lsl)
            ),

            // ---- ARMv7-M batch M7k: long multiply ----
            Self::Smull_T1(rdlo, rdhi, rn, rm) => format!(
                "smull {}, {}, {}, {}",
                gpr(rdlo),
                gpr(rdhi),
                gpr(rn),
                gpr(rm)
            ),
            Self::Umull_T1(rdlo, rdhi, rn, rm) => format!(
                "umull {}, {}, {}, {}",
                gpr(rdlo),
                gpr(rdhi),
                gpr(rn),
                gpr(rm)
            ),
            Self::Smlal_T1(rdlo, rdhi, rn, rm) => format!(
                "smlal {}, {}, {}, {}",
                gpr(rdlo),
                gpr(rdhi),
                gpr(rn),
                gpr(rm)
            ),
            Self::Umlal_T1(rdlo, rdhi, rn, rm) => format!(
                "umlal {}, {}, {}, {}",
                gpr(rdlo),
                gpr(rdhi),
                gpr(rn),
                gpr(rm)
            ),
            Self::Umaal_T1(rdlo, rdhi, rn, rm) => format!(
                "umaal {}, {}, {}, {}",
                gpr(rdlo),
                gpr(rdhi),
                gpr(rn),
                gpr(rm)
            ),

            // ---- ARMv7-M batch M7l: wide extend (with ROR), wide byte-reverse, saturate ----
            Self::Sxtb_T2(rd, rm, rotation) => format!(
                "sxtb.w {}, {}{}",
                gpr(rd),
                gpr(rm),
                render_rotation(*rotation)
            ),
            Self::Uxtb_T2(rd, rm, rotation) => format!(
                "uxtb.w {}, {}{}",
                gpr(rd),
                gpr(rm),
                render_rotation(*rotation)
            ),
            Self::Sxth_T2(rd, rm, rotation) => format!(
                "sxth.w {}, {}{}",
                gpr(rd),
                gpr(rm),
                render_rotation(*rotation)
            ),
            Self::Uxth_T2(rd, rm, rotation) => format!(
                "uxth.w {}, {}{}",
                gpr(rd),
                gpr(rm),
                render_rotation(*rotation)
            ),
            Self::Rev_T2(rd, rm) => format!("rev.w {}, {}", gpr(rd), gpr(rm)),
            Self::Rev16_T2(rd, rm) => format!("rev16.w {}, {}", gpr(rd), gpr(rm)),
            Self::Revsh_T2(rd, rm) => format!("revsh.w {}, {}", gpr(rd), gpr(rm)),
            Self::Ssat_T1(rd, sat_imm, rn, shift) => format!(
                "ssat {}, {}, {}{}",
                gpr(rd),
                imm(syntax, *sat_imm as i64),
                gpr(rn),
                render_shift(shift)
            ),
            Self::Usat_T1(rd, sat_imm, rn, shift) => format!(
                "usat {}, {}, {}{}",
                gpr(rd),
                imm(syntax, *sat_imm as i64),
                gpr(rn),
                render_shift(shift)
            ),

            // ---- ARMv7-M batch M7i: indexed load/store, LDRD/STRD, literal loads, preload ----
            Self::Ldr_Immediate_T4(rt, rn, off, mode) => format!(
                "ldr {}, {}",
                gpr(rt),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Str_Immediate_T4(rt, rn, off, mode) => format!(
                "str {}, {}",
                gpr(rt),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Ldrb_Immediate_T3(rt, rn, off, mode) => format!(
                "ldrb {}, {}",
                gpr(rt),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Strb_Immediate_T3(rt, rn, off, mode) => format!(
                "strb {}, {}",
                gpr(rt),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Ldrh_Immediate_T3(rt, rn, off, mode) => format!(
                "ldrh {}, {}",
                gpr(rt),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Strh_Immediate_T3(rt, rn, off, mode) => format!(
                "strh {}, {}",
                gpr(rt),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Ldrsb_Immediate_T2(rt, rn, off, mode) => format!(
                "ldrsb {}, {}",
                gpr(rt),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Ldrsh_Immediate_T2(rt, rn, off, mode) => format!(
                "ldrsh {}, {}",
                gpr(rt),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Ldrd_Immediate_T1(rt, rt2, rn, off, mode) => format!(
                "ldrd {}, {}, {}",
                gpr(rt),
                gpr(rt2),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            Self::Strd_Immediate_T1(rt, rt2, rn, off, mode) => format!(
                "strd {}, {}, {}",
                gpr(rt),
                gpr(rt2),
                render_indexed(rn, *off as i64, *mode, syntax)
            ),
            // literal loads carry `.w` so the assembler does not pick the narrow PC-relative form
            Self::Ldr_Literal_T2(rt, off) => {
                format!("ldr.w {}, [pc, {}]", gpr(rt), imm(syntax, *off as i64))
            }
            Self::Ldrb_Literal_T1(rt, off) => {
                format!("ldrb.w {}, [pc, {}]", gpr(rt), imm(syntax, *off as i64))
            }
            Self::Ldrh_Literal_T1(rt, off) => {
                format!("ldrh.w {}, [pc, {}]", gpr(rt), imm(syntax, *off as i64))
            }
            Self::Ldrsb_Literal_T1(rt, off) => {
                format!("ldrsb.w {}, [pc, {}]", gpr(rt), imm(syntax, *off as i64))
            }
            Self::Ldrsh_Literal_T1(rt, off) => {
                format!("ldrsh.w {}, [pc, {}]", gpr(rt), imm(syntax, *off as i64))
            }
            Self::Pld_Immediate_T1(rn, off) => {
                format!("pld [{}, {}]", gpr(rn), imm(syntax, *off as i64))
            }
            Self::Pli_Immediate_T1(rn, off) => {
                format!("pli [{}, {}]", gpr(rn), imm(syntax, *off as i64))
            }

            // ---- ARMv7-M batch M7j: wide load/store multiple (SP+writeback prints as push.w/pop.w) ----
            Self::Ldmia_T2(rn, writeback, registers) => {
                if is_sp_register(rn) && *writeback {
                    format!("pop.w {}", register_list(registers))
                } else {
                    format!(
                        "ldm.w {}{}, {}",
                        gpr(rn),
                        writeback_suffix(*writeback),
                        register_list(registers)
                    )
                }
            }
            Self::Stmia_T2(rn, writeback, registers) => format!(
                "stm.w {}{}, {}",
                gpr(rn),
                writeback_suffix(*writeback),
                register_list(registers)
            ),
            Self::Ldmdb_T1(rn, writeback, registers) => format!(
                "ldmdb {}{}, {}",
                gpr(rn),
                writeback_suffix(*writeback),
                register_list(registers)
            ),
            Self::Stmdb_T1(rn, writeback, registers) => {
                if is_sp_register(rn) && *writeback {
                    format!("push.w {}", register_list(registers))
                } else {
                    format!(
                        "stmdb {}{}, {}",
                        gpr(rn),
                        writeback_suffix(*writeback),
                        register_list(registers)
                    )
                }
            }

            // ---- ARMv8-M Security Extension ----
            Self::Csdb_T1 => "csdb".to_string(),
            Self::Sg_T1 => "sg".to_string(),
            Self::Bxns_T1(rm) => format!("bxns {}", gpr(rm)),
            Self::Blxns_T1(rm) => format!("blxns {}", gpr(rm)),
            Self::Tt_T1(rd, rn, a, t) => format!(
                "tt{}{} {}, {}",
                if *a { "a" } else { "" },
                if *t { "t" } else { "" },
                gpr(rd),
                gpr(rn)
            ),
            Self::Vlstm_T1(rn) => format!("vlstm {}", gpr(rn)),
            Self::Vlldm_T1(rn) => format!("vlldm {}", gpr(rn)),

            // ---- ARMv8.1-M MVE 3-reg-same vector-vector ----
            // integer: `<op>.<i|s|u><width> Qd, Qn, Qm`; bitwise: `<op> Qd, Qn, Qm` (no type); float:
            // `<op>.f<width> Qd, Qn, Qm`.
            Self::MveIntArith(op, size, qd, qn, qm) => format!(
                "{}.{}{} {}, {}, {}",
                op.mnemonic(),
                op.type_prefix(),
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveBitwise(op, qd, qn, qm) => format!(
                "{} {}, {}, {}",
                op.mnemonic(),
                mve_q(qd),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveFloatArith(op, size, qd, qn, qm) => format!(
                "{}.f{} {}, {}, {}",
                op.mnemonic(),
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                mve_q(qm)
            ),

            // ---- MVE vector-by-scalar (Qd, Qn, Rm) and VDUP (Qd, Rt) ----
            Self::MveVecScalarInt(op, size, qd, qn, rm) => format!(
                "{}.{}{} {}, {}, {}",
                op.mnemonic(),
                op.type_prefix(),
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                gpr(rm)
            ),
            Self::MveVecScalarFloat(op, size, qd, qn, rm) => format!(
                "{}.f{} {}, {}, {}",
                op.mnemonic(),
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                gpr(rm)
            ),
            Self::MveVdup(size, qd, rt) => {
                format!("vdup.{} {}, {}", size.width_digits(), mve_q(qd), gpr(rt))
            }
            // shift by immediate: `<op>.<type><width> Qd, Qm, #amount` (VSLI/VSRI carry only the width)
            Self::MveShiftImm(op, size, amount, qd, qm) => {
                let type_part = match op.type_prefix() {
                    Some(prefix) => format!("{}{}", prefix, size.width_digits()),
                    None => size.width_digits().to_string(),
                };
                format!(
                    "{}.{} {}, {}, #{}",
                    op.mnemonic(),
                    type_part,
                    mve_q(qd),
                    mve_q(qm),
                    amount
                )
            }
            Self::MveModifiedImmediate(cmode, op, imm8, qd) => {
                mve_modified_immediate(*cmode, *op, *imm8, &mve_q(qd))
            }
            // 2-register misc: `<op>.<type><width> Qd, Qm` (VREV carries only the width; VMVN has no suffix)
            Self::MveMisc2(op, size, qd, qm) => {
                let type_part = match op.type_prefix() {
                    Some(prefix) => format!("{}{}", prefix, size.width_digits()),
                    None => size.width_digits().to_string(),
                };
                format!(
                    "{}.{} {}, {}",
                    op.mnemonic(),
                    type_part,
                    mve_q(qd),
                    mve_q(qm)
                )
            }
            Self::MveMisc2Float(op, size, qd, qm) => format!(
                "{}.f{} {}, {}",
                op.mnemonic(),
                size.width_digits(),
                mve_q(qd),
                mve_q(qm)
            ),
            Self::MveVmaxaMina(is_min, size, qda, qm) => format!(
                "{}.s{} {}, {}",
                if *is_min { "vmina" } else { "vmaxa" },
                size.width_digits(),
                mve_q(qda),
                mve_q(qm)
            ),
            Self::MveVmaxnmaMinnma(is_min, size, qda, qm) => format!(
                "{}.f{} {}, {}",
                if *is_min { "vminnma" } else { "vmaxnma" },
                size.width_digits(),
                mve_q(qda),
                mve_q(qm)
            ),
            Self::MveMvnRegister(qd, qm) => format!("vmvn {}, {}", mve_q(qd), mve_q(qm)),
            // contiguous vector load/store: `vldr{b,h,w}.u{8,16,32}` / `vstr{b,h,w}.{8,16,32} Qd, <addr>`
            Self::MveLoadStore(is_load, size, qd, rn, offset, mode) => {
                let (letter, width) = match size {
                    crate::enums::Arm32MveSize::I8 => ("b", "8"),
                    crate::enums::Arm32MveSize::I16 => ("h", "16"),
                    crate::enums::Arm32MveSize::I32 => ("w", "32"),
                };
                let mnemonic = if *is_load {
                    format!("vldr{}.u{}", letter, width)
                } else {
                    format!("vstr{}.{}", letter, width)
                };
                format!(
                    "{} {}, {}",
                    mnemonic,
                    mve_q(qd),
                    render_indexed(rn, *offset as i64, *mode, syntax)
                )
            }
            Self::MveGatherScatter(is_load, unsigned, esize, msize, scaled, qd, rn, qm) => {
                let letter = match *msize {
                    8 => "b",
                    16 => "h",
                    32 => "w",
                    _ => "d",
                };
                let mnemonic = if *is_load {
                    format!(
                        "vldr{}.{}{}",
                        letter,
                        if *unsigned { 'u' } else { 's' },
                        esize
                    )
                } else {
                    format!("vstr{}.{}", letter, esize) // the suffix is the element size (e.g. vstrb.16 = byte access of 16-bit lanes)
                };
                let addr = if *scaled {
                    format!(
                        "[{}, {}, uxtw #{}]",
                        gpr(rn),
                        mve_q(qm),
                        crate::enums::mve_mem_size_log(*msize)
                    )
                } else {
                    format!("[{}, {}]", gpr(rn), mve_q(qm))
                };
                format!("{} {}, {}", mnemonic, mve_q(qd), addr)
            }
            Self::MveGatherScatterBase(is_load, is_dword, writeback, qd, qn, offset) => {
                let (letter, width) = if *is_dword { ("d", "64") } else { ("w", "32") };
                let mnemonic = if *is_load {
                    format!("vldr{}.u{}", letter, width)
                } else {
                    format!("vstr{}.{}", letter, width)
                };
                let wb = if *writeback { "!" } else { "" };
                let addr = if *offset == 0 {
                    format!("[{}]{}", mve_q(qn), wb)
                } else {
                    format!("[{}, #{}]{}", mve_q(qn), offset, wb)
                };
                format!("{} {}, {}", mnemonic, mve_q(qd), addr)
            }
            Self::MveInterleave(is_load, is_quad, pass, size, qd, rn, writeback) => {
                let count: u32 = if *is_quad { 4 } else { 2 };
                let list: Vec<String> = (0..count)
                    .map(|i| {
                        mve_q(&crate::enums::Arm32MveVectorRegister::from_field(
                            (qd.field() + i) & 7,
                        ))
                    })
                    .collect();
                let mnemonic = format!(
                    "{}{}{}.{}",
                    if *is_load { "vld" } else { "vst" },
                    count,
                    pass,
                    size.width_digits()
                );
                let wb = if *writeback { "!" } else { "" };
                format!("{} {{{}}}, [{}]{}", mnemonic, list.join(", "), gpr(rn), wb)
            }
            // low-overhead loops
            Self::LobStart(is_while, tp_size, rn, offset) => {
                let mnemonic = match tp_size {
                    None => {
                        if *is_while {
                            "wls".to_string()
                        } else {
                            "dls".to_string()
                        }
                    }
                    Some(size) => format!("{}.{}", if *is_while { "wlstp" } else { "dlstp" }, size),
                };
                if *is_while {
                    match instruction_address {
                        Some(address) => format!(
                            "{} lr, {}, 0x{:08x}",
                            mnemonic,
                            gpr(rn),
                            pc_relative_target(address, *offset as i64, false)
                        ),
                        None => format!(
                            "{} lr, {}, {}",
                            mnemonic,
                            gpr(rn),
                            imm(syntax, *offset as i64)
                        ),
                    }
                } else {
                    format!("{} lr, {}", mnemonic, gpr(rn))
                }
            }
            Self::LobEnd(tail_predicated, offset) => {
                let mnemonic = if *tail_predicated { "letp" } else { "le" };
                match instruction_address {
                    Some(address) => format!(
                        "{} lr, 0x{:08x}",
                        mnemonic,
                        pc_relative_target(address, *offset as i64, false)
                    ),
                    None => format!("{} lr, {}", mnemonic, imm(syntax, *offset as i64)),
                }
            }
            Self::Lctp => "lctp".to_string(),
            Self::MveVctp(size, rn) => format!("vctp.{} {}", size, gpr(rn)),
            // cross-lane reductions to a GPR
            Self::MveReduce(op, size, rd, qm) => format!(
                "{}.{}{} {}, {}",
                op.mnemonic(),
                op.type_prefix(),
                size.width_digits(),
                gpr(rd),
                mve_q(qm)
            ),
            Self::MveVabav(signed, size, rd, qn, qm) => format!(
                "vabav.{}{} {}, {}, {}",
                if *signed { 's' } else { 'u' },
                size.width_digits(),
                gpr(rd),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveDualMac(subtract, exchange, accumulate, unsigned, size, rda, qn, qm) => {
                let mnemonic = format!(
                    "v{}{}{}",
                    if *subtract { "mlsdav" } else { "mladav" },
                    if *accumulate { "a" } else { "" },
                    if *exchange { "x" } else { "" }
                );
                format!(
                    "{}.{}{} {}, {}, {}",
                    mnemonic,
                    if *unsigned { 'u' } else { 's' },
                    size.width_digits(),
                    gpr(rda),
                    mve_q(qn),
                    mve_q(qm)
                )
            }
            Self::MveLongDualMac(
                op,
                exchange,
                accumulate,
                unsigned,
                size,
                rda_lo,
                rda_hi,
                qn,
                qm,
            ) => {
                let mnemonic = format!(
                    "{}{}{}",
                    op.mnemonic(),
                    if *accumulate { "a" } else { "" },
                    if *exchange { "x" } else { "" }
                );
                format!(
                    "{}.{}{} {}, {}, {}, {}",
                    mnemonic,
                    if *unsigned { 'u' } else { 's' },
                    size.width_digits(),
                    gpr(rda_lo),
                    gpr(rda_hi),
                    mve_q(qn),
                    mve_q(qm)
                )
            }
            Self::MveVmovTwoLane(to_vector, idx1, qd, rt, rt2) => {
                // `idx1` is 2 or 3 by construction; saturating_sub keeps to_assembly_string panic-free even if a
                // caller hand-builds an out-of-range model (encode() rejects those).
                let (lane_hi, lane_lo) = (
                    format!("{}[{}]", mve_q(qd), idx1),
                    format!("{}[{}]", mve_q(qd), idx1.saturating_sub(2)),
                );
                if *to_vector {
                    format!("vmov {}, {}, {}, {}", lane_hi, lane_lo, gpr(rt), gpr(rt2))
                } else {
                    format!("vmov {}, {}, {}, {}", gpr(rt), gpr(rt2), lane_hi, lane_lo)
                }
            }
            Self::MveVrint(op, size, qd, qm) => format!(
                "{}.f{} {}, {}",
                op.mnemonic(),
                size.width_digits(),
                mve_q(qd),
                mve_q(qm)
            ),
            Self::MveVcvtFloatInt(to_int, unsigned, size, qd, qm) => {
                let width = size.width_digits();
                let int_type = if *unsigned { 'u' } else { 's' };
                if *to_int {
                    format!(
                        "vcvt.{}{}.f{} {}, {}",
                        int_type,
                        width,
                        width,
                        mve_q(qd),
                        mve_q(qm)
                    )
                } else {
                    format!(
                        "vcvt.f{}.{}{} {}, {}",
                        width,
                        int_type,
                        width,
                        mve_q(qd),
                        mve_q(qm)
                    )
                }
            }
            Self::MveVcvtFixed(to_fixed, unsigned, size, fbits, qd, qm) => {
                let width = size.width_digits();
                let int_type = if *unsigned { 'u' } else { 's' };
                if *to_fixed {
                    format!(
                        "vcvt.{}{}.f{} {}, {}, #{}",
                        int_type,
                        width,
                        width,
                        mve_q(qd),
                        mve_q(qm),
                        fbits
                    )
                } else {
                    format!(
                        "vcvt.f{}.{}{} {}, {}, #{}",
                        width,
                        int_type,
                        width,
                        mve_q(qd),
                        mve_q(qm),
                        fbits
                    )
                }
            }
            Self::MveVcvtHalf(top, half_to_single, qd, qm) => {
                let types = if *half_to_single {
                    "f32.f16"
                } else {
                    "f16.f32"
                };
                format!(
                    "vcvt{}.{} {}, {}",
                    if *top { "t" } else { "b" },
                    types,
                    mve_q(qd),
                    mve_q(qm)
                )
            }
            Self::MveShiftNarrow(op, unsigned, top, src_is_32, shift, qd, qm) => {
                use crate::enums::Arm32MveShiftNarrowOp::*;
                let type_prefix = match op {
                    Vshrn | Vrshrn => 'i',
                    Vqshrn | Vqrshrn => {
                        if *unsigned {
                            'u'
                        } else {
                            's'
                        }
                    }
                    Vqshrun | Vqrshrun => 's',
                };
                let src = if *src_is_32 { 32 } else { 16 };
                format!(
                    "{}{}.{}{} {}, {}, #{}",
                    op.mnemonic(),
                    if *top { "t" } else { "b" },
                    type_prefix,
                    src,
                    mve_q(qd),
                    mve_q(qm),
                    shift
                )
            }
            // width-changing register moves
            Self::MveVmovl(top, unsigned, size, qd, qm) => format!(
                "vmovl{}.{}{} {}, {}",
                if *top { "t" } else { "b" },
                if *unsigned { 'u' } else { 's' },
                size.width_digits(),
                mve_q(qd),
                mve_q(qm)
            ),
            Self::MveVmovn(top, size, qd, qm) => format!(
                "vmovn{}.i{} {}, {}",
                if *top { "t" } else { "b" },
                size.width_digits(),
                mve_q(qd),
                mve_q(qm)
            ),
            Self::MveVqmovn(kind, unsigned, top, size, qd, qm) => {
                use crate::enums::Arm32MveQMovnKind::*;
                let (mnem, type_letter) = match kind {
                    Vqmovn => ("vqmovn", if *unsigned { 'u' } else { 's' }),
                    Vqmovun => ("vqmovun", 's'),
                };
                format!(
                    "{}{}.{}{} {}, {}",
                    mnem,
                    if *top { "t" } else { "b" },
                    type_letter,
                    size.width_digits(),
                    mve_q(qd),
                    mve_q(qm)
                )
            }
            Self::MveVmull(polynomial, unsigned, top, size, qd, qn, qm) => {
                let type_letter = if *polynomial {
                    'p'
                } else if *unsigned {
                    'u'
                } else {
                    's'
                };
                format!(
                    "vmull{}.{}{} {}, {}, {}",
                    if *top { "t" } else { "b" },
                    type_letter,
                    size.width_digits(),
                    mve_q(qd),
                    mve_q(qn),
                    mve_q(qm)
                )
            }
            Self::MveVmulh(rounding, unsigned, size, qd, qn, qm) => format!(
                "v{}mulh.{}{} {}, {}, {}",
                if *rounding { "r" } else { "" },
                if *unsigned { 'u' } else { 's' },
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveVqdmull(top, size32, qd, qn, qm) => format!(
                "vqdmull{}.s{} {}, {}, {}",
                if *top { "t" } else { "b" },
                if *size32 { 32 } else { 16 },
                mve_q(qd),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveVqdmullScalar(top, size32, qd, qn, rm) => format!(
                "vqdmull{}.s{} {}, {}, {}",
                if *top { "t" } else { "b" },
                if *size32 { 32 } else { 16 },
                mve_q(qd),
                mve_q(qn),
                gpr(rm)
            ),
            Self::MveVqdmladh(subtract, rounding, exchange, size, qd, qn, qm) => format!(
                "v{}{}{}.s{} {}, {}, {}",
                if *rounding { "qrd" } else { "qd" },
                if *subtract { "mlsdh" } else { "mladh" },
                if *exchange { "x" } else { "" },
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                mve_q(qm)
            ),
            // VSHL/VRSHL/VQSHL/VQRSHL: mnemonic = v + (q if saturating) + (r if rounding) + shl
            Self::MveShiftByVector(rounding, saturating, unsigned, size, qd, qm, qn) => format!(
                "v{}{}shl.{}{} {}, {}, {}",
                if *saturating { "q" } else { "" },
                if *rounding { "r" } else { "" },
                if *unsigned { 'u' } else { 's' },
                size.width_digits(),
                mve_q(qd),
                mve_q(qm),
                mve_q(qn)
            ),
            Self::MveShiftByScalar(rounding, saturating, unsigned, size, qda, rm) => format!(
                "v{}{}shl.{}{} {}, {}",
                if *saturating { "q" } else { "" },
                if *rounding { "r" } else { "" },
                if *unsigned { 'u' } else { 's' },
                size.width_digits(),
                mve_q(qda),
                gpr(rm)
            ),
            Self::MveVshll(top, unsigned, size, shift, qd, qm) => format!(
                "vshll{}.{}{} {}, {}, #{}",
                if *top { "t" } else { "b" },
                if *unsigned { 'u' } else { 's' },
                size.width_digits(),
                mve_q(qd),
                mve_q(qm),
                shift
            ),
            Self::MveVaddlv(accumulate, unsigned, rd_lo, rd_hi, qm) => format!(
                "vaddlv{}.{}32 {}, {}, {}",
                if *accumulate { "a" } else { "" },
                if *unsigned { 'u' } else { 's' },
                gpr(rd_lo),
                gpr(rd_hi),
                mve_q(qm)
            ),
            // complex-number ops: `<op>.<type> Qd, Qn, Qm, #<rotation>`
            Self::MveVcaddInt(halving, size, rot270, qd, qn, qm) => {
                let (mnemonic, type_part) = if *halving {
                    ("vhcadd", format!("s{}", size.width_digits()))
                } else {
                    ("vcadd", format!("i{}", size.width_digits()))
                };
                format!(
                    "{}.{} {}, {}, {}, #{}",
                    mnemonic,
                    type_part,
                    mve_q(qd),
                    mve_q(qn),
                    mve_q(qm),
                    if *rot270 { 270 } else { 90 }
                )
            }
            Self::MveVcaddFloat(size, rot270, qd, qn, qm) => format!(
                "vcadd.f{} {}, {}, {}, #{}",
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                mve_q(qm),
                if *rot270 { 270 } else { 90 }
            ),
            Self::MveVcmul(size, rotate, qd, qn, qm) => format!(
                "vcmul.f{} {}, {}, {}, #{}",
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                mve_q(qm),
                (*rotate as u32) * 90
            ),
            Self::MveVcmla(size, rotate, qd, qn, qm) => format!(
                "vcmla.f{} {}, {}, {}, #{}",
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                mve_q(qm),
                (*rotate as u32) * 90
            ),
            Self::MveVpsel(qd, qn, qm) => {
                format!("vpsel {}, {}, {}", mve_q(qd), mve_q(qn), mve_q(qm))
            }
            Self::MveVadc(subtract, init_carry, qd, qn, qm) => format!(
                "v{}c{}.i32 {}, {}, {}",
                if *subtract { "sb" } else { "ad" },
                if *init_carry { "i" } else { "" },
                mve_q(qd),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveVshlc(shift, qda, rdm) => {
                format!("vshlc {}, {}, #{}", mve_q(qda), gpr(rdm), shift)
            }
            Self::MveViddup(decrement, size, qd, rn, wrap_rm, step) => {
                let mnemonic = match (*decrement, wrap_rm.is_some()) {
                    (false, false) => "vidup",
                    (true, false) => "vddup",
                    (false, true) => "viwdup",
                    (true, true) => "vdwdup",
                };
                match wrap_rm {
                    Some(rm) => format!(
                        "{}.u{} {}, {}, {}, #{}",
                        mnemonic,
                        size.width_digits(),
                        mve_q(qd),
                        gpr(rn),
                        gpr(rm),
                        step
                    ),
                    None => format!(
                        "{}.u{} {}, {}, #{}",
                        mnemonic,
                        size.width_digits(),
                        mve_q(qd),
                        gpr(rn),
                        step
                    ),
                }
            }
            Self::MveVbrsr(size, qd, qn, rm) => format!(
                "vbrsr.{} {}, {}, {}",
                size.width_digits(),
                mve_q(qd),
                mve_q(qn),
                gpr(rm)
            ),
            Self::MveVpnot => "vpnot".to_string(),
            // VCMP: `vcmp.<type> <cond>, Qn, <Qm|Rm>` (integer type from the condition; float is always .fNN)
            Self::MveVcmpReg(cond, size, qn, qm) => format!(
                "vcmp.{}{} {}, {}, {}",
                cond.type_prefix(),
                size.width_digits(),
                cond.mnemonic(),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveVcmpScalar(cond, size, qn, rm) => format!(
                "vcmp.{}{} {}, {}, {}",
                cond.type_prefix(),
                size.width_digits(),
                cond.mnemonic(),
                mve_q(qn),
                gpr(rm)
            ),
            Self::MveVcmpFloatReg(cond, size, qn, qm) => format!(
                "vcmp.f{} {}, {}, {}",
                size.width_digits(),
                cond.mnemonic(),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveVcmpFloatScalar(cond, size, qn, rm) => format!(
                "vcmp.f{} {}, {}, {}",
                size.width_digits(),
                cond.mnemonic(),
                mve_q(qn),
                gpr(rm)
            ),
            // VPST: `vps` + the then/else letters (the first `t` is always present)
            Self::MveVpst(mask) => format!("vps{}", crate::enums::mve_predicate_mask_suffix(*mask)),
            // VPT: `vp` + the then/else letters + the VCMP-style `.<type> <cond>, Qn, <Qm|Rm>`
            Self::MveVptReg(cond, size, qn, qm, mask) => format!(
                "vp{}.{}{} {}, {}, {}",
                crate::enums::mve_predicate_mask_suffix(*mask),
                cond.type_prefix(),
                size.width_digits(),
                cond.mnemonic(),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveVptScalar(cond, size, qn, rm, mask) => format!(
                "vp{}.{}{} {}, {}, {}",
                crate::enums::mve_predicate_mask_suffix(*mask),
                cond.type_prefix(),
                size.width_digits(),
                cond.mnemonic(),
                mve_q(qn),
                gpr(rm)
            ),
            Self::MveVptFloatReg(cond, size, qn, qm, mask) => format!(
                "vp{}.f{} {}, {}, {}",
                crate::enums::mve_predicate_mask_suffix(*mask),
                size.width_digits(),
                cond.mnemonic(),
                mve_q(qn),
                mve_q(qm)
            ),
            Self::MveVptFloatScalar(cond, size, qn, rm, mask) => format!(
                "vp{}.f{} {}, {}, {}",
                crate::enums::mve_predicate_mask_suffix(*mask),
                size.width_digits(),
                cond.mnemonic(),
                mve_q(qn),
                gpr(rm)
            ),
            Self::MveFloatReduce(op, size, rd, qm) => format!(
                "{}.f{} {}, {}",
                op.mnemonic(),
                size.width_digits(),
                gpr(rd),
                mve_q(qm)
            ),
            Self::MveVcvtRound(rounding, unsigned, size, qd, qm) => {
                let mode = ['a', 'n', 'p', 'm'][(*rounding & 0b11) as usize];
                let width = size.width_digits();
                format!(
                    "vcvt{}.{}{}.f{} {}, {}",
                    mode,
                    if *unsigned { 'u' } else { 's' },
                    width,
                    width,
                    mve_q(qd),
                    mve_q(qm)
                )
            }
        }
    }
}

// MVE vector register name (`q0`..`q7`).
fn mve_q(register: &Arm32MveVectorRegister) -> String {
    format!("q{}", register.number())
}

// Render an MVE one-register modified immediate from its raw (cmode, op, imm8) per the AdvSIMDExpandImm
// cmode/op table (the same table the A32 NEON form uses): the pair selects the mnemonic, element size and
// the shift applied to imm8.
fn mve_modified_immediate(cmode: u8, op: bool, imm8: u8, qd: &str) -> String {
    let imm = imm8 as u64;
    let (mnemonic, suffix, value) = match (cmode >> 1, cmode & 1, op) {
        (0b000, 0, false) => ("vmov", ".i32", imm),
        (0b000, 0, true) => ("vmvn", ".i32", imm),
        (0b000, 1, false) => ("vorr", ".i32", imm),
        (0b000, 1, true) => ("vbic", ".i32", imm),
        (0b001, 0, false) => ("vmov", ".i32", imm << 8),
        (0b001, 0, true) => ("vmvn", ".i32", imm << 8),
        (0b001, 1, false) => ("vorr", ".i32", imm << 8),
        (0b001, 1, true) => ("vbic", ".i32", imm << 8),
        (0b010, 0, false) => ("vmov", ".i32", imm << 16),
        (0b010, 0, true) => ("vmvn", ".i32", imm << 16),
        (0b010, 1, false) => ("vorr", ".i32", imm << 16),
        (0b010, 1, true) => ("vbic", ".i32", imm << 16),
        (0b011, 0, false) => ("vmov", ".i32", imm << 24),
        (0b011, 0, true) => ("vmvn", ".i32", imm << 24),
        (0b011, 1, false) => ("vorr", ".i32", imm << 24),
        (0b011, 1, true) => ("vbic", ".i32", imm << 24),
        (0b100, 0, false) => ("vmov", ".i16", imm),
        (0b100, 0, true) => ("vmvn", ".i16", imm),
        (0b100, 1, false) => ("vorr", ".i16", imm),
        (0b100, 1, true) => ("vbic", ".i16", imm),
        (0b101, 0, false) => ("vmov", ".i16", imm << 8),
        (0b101, 0, true) => ("vmvn", ".i16", imm << 8),
        (0b101, 1, false) => ("vorr", ".i16", imm << 8),
        (0b101, 1, true) => ("vbic", ".i16", imm << 8),
        (0b110, _, false) => (
            "vmov",
            ".i32",
            if cmode & 1 == 0 {
                (imm << 8) | 0xFF
            } else {
                (imm << 16) | 0xFFFF
            },
        ),
        (0b110, _, true) => (
            "vmvn",
            ".i32",
            if cmode & 1 == 0 {
                (imm << 8) | 0xFF
            } else {
                (imm << 16) | 0xFFFF
            },
        ),
        (0b111, 0, false) => ("vmov", ".i8", imm),
        (0b111, 0, true) => ("vmov", ".i64", mve_expand_imm64(imm8)),
        (0b111, 1, false) => ("vmov", ".f32", 0), // handled below
        _ => ("vmov", ".i32", imm),
    };
    if cmode == 0b1111 && !op {
        return format!(
            "vmov.f32 {}, #{:?}",
            qd,
            crate::vfp_expand_imm8_to_f32(imm8) as f64
        );
    }
    if suffix == ".i64" {
        return format!("vmov.i64 {}, #0x{:016x}", qd, value);
    }
    format!("{}{} {}, #0x{:x}", mnemonic, suffix, qd, value)
}
// cmode 1110 op=1: each bit of imm8 expands to a 0x00/0xFF byte of the 64-bit value.
fn mve_expand_imm64(imm8: u8) -> u64 {
    let mut value: u64 = 0;
    for bit in 0..8 {
        if imm8 & (1 << bit) != 0 {
            value |= 0xFFu64 << (bit * 8);
        }
    }
    value
}

fn writeback_suffix(writeback: bool) -> &'static str {
    if writeback { "!" } else { "" }
}

fn is_sp_register(register: &Arm32GeneralPurposeRegister) -> bool {
    *register == Arm32GeneralPurposeRegister::R13
}

// The addressing operand of an indexed load/store: `[Rn, #+/-imm]` (offset), `[Rn, #+/-imm]!` (pre-index), or
// `[Rn], #+/-imm` (post-index). The immediate prints through `imm`, which already renders the sign.
fn render_indexed(
    rn: &Arm32GeneralPurposeRegister,
    offset: i64,
    mode: ArmT32IndexMode,
    syntax: ArmAssemblySyntax,
) -> String {
    match mode {
        ArmT32IndexMode::Offset => format!("[{}, {}]", gpr(rn), imm(syntax, offset)),
        ArmT32IndexMode::PreIndex => format!("[{}, {}]!", gpr(rn), imm(syntax, offset)),
        ArmT32IndexMode::PostIndex => format!("[{}], {}", gpr(rn), imm(syntax, offset)),
    }
}

// The directed-rounding suffix for VRINTA/N/P/M (and VCVTA/N/P/M).
fn t32_directed_round(mode: Arm32DirectedRound) -> &'static str {
    match mode {
        Arm32DirectedRound::A => "a",
        Arm32DirectedRound::N => "n",
        Arm32DirectedRound::P => "p",
        Arm32DirectedRound::M => "m",
    }
}

// the optional `, ror #n` on a wide extend; UAL omits it when the rotation is zero. Always decimal.
fn render_rotation(rotation: u8) -> String {
    if rotation == 0 {
        String::new()
    } else {
        format!(", ror #{}", rotation)
    }
}

// floating-point register names and the FP `[Rn{, #+/-off}]` addressing operand.
fn single(register: &Arm32SinglePrecisionRegister) -> String {
    format!("s{}", register.number())
}
fn double(register: &Arm32DoublePrecisionRegister) -> String {
    format!("d{}", register.number())
}
fn render_fp_mem(
    rn: &Arm32GeneralPurposeRegister,
    offset: i64,
    syntax: ArmAssemblySyntax,
) -> String {
    if offset == 0 {
        format!("[{}]", gpr(rn))
    } else {
        format!("[{}, {}]", gpr(rn), imm(syntax, offset))
    }
}
// a contiguous FP register range `{s0-s3}` (or `{s0}` for a single register)
fn fp_single_range(first: &Arm32SinglePrecisionRegister, count: u8) -> String {
    if count <= 1 {
        format!("{{s{}}}", first.number())
    } else {
        format!(
            "{{s{}-s{}}}",
            first.number(),
            first.number() as u16 + count as u16 - 1
        )
    }
}
fn fp_double_range(first: &Arm32DoublePrecisionRegister, count: u8) -> String {
    if count <= 1 {
        format!("{{d{}}}", first.number())
    } else {
        format!(
            "{{d{}-d{}}}",
            first.number(),
            first.number() as u16 + count as u16 - 1
        )
    }
}

// signed-multiply mnemonic suffix letters: t/b (top/bottom half), x (exchange), r (rounded).
fn top_or_bottom(top: bool) -> &'static str {
    if top { "t" } else { "b" }
}
fn exchange(cross: bool) -> &'static str {
    if cross { "x" } else { "" }
}
fn rounded(round: bool) -> &'static str {
    if round { "r" } else { "" }
}
fn int_type(signed: bool) -> &'static str {
    if signed { "s32" } else { "u32" }
}
fn fixed_type(signed: bool, bits32: bool) -> String {
    format!(
        "{}{}",
        if signed { "s" } else { "u" },
        if bits32 { "32" } else { "16" }
    )
}

// `[Rn, Rm]` or `[Rn, Rm, lsl #n]` (the LSL amount is always decimal).
fn render_register_offset(
    rn: &Arm32GeneralPurposeRegister,
    rm: &Arm32GeneralPurposeRegister,
    lsl: u8,
) -> String {
    if lsl == 0 {
        format!("[{}, {}]", gpr(rn), gpr(rm))
    } else {
        format!("[{}, {}, lsl #{}]", gpr(rn), gpr(rm), lsl)
    }
}

// MOV (register) and the shift register-by-immediate mnemonics share one encoding; render the canonical
// UAL: no shift -> `mov{s}.w`; LSL/LSR/ASR/ROR -> that mnemonic with a count; RRX -> `rrx{s}` (no `.w`).
fn render_mov_register(
    rd: &Arm32GeneralPurposeRegister,
    rm: &Arm32GeneralPurposeRegister,
    shift: &ArmT32RegisterShift,
    set_flags: bool,
) -> String {
    match shift {
        ArmT32RegisterShift::Lsl(0) => {
            format!("mov{} {}, {}", wide_flag(set_flags), gpr(rd), gpr(rm))
        }
        ArmT32RegisterShift::Lsl(amount) => format!(
            "lsl{} {}, {}, #{}",
            wide_flag(set_flags),
            gpr(rd),
            gpr(rm),
            amount
        ),
        ArmT32RegisterShift::Lsr(amount) => format!(
            "lsr{} {}, {}, #{}",
            wide_flag(set_flags),
            gpr(rd),
            gpr(rm),
            amount
        ),
        ArmT32RegisterShift::Asr(amount) => format!(
            "asr{} {}, {}, #{}",
            wide_flag(set_flags),
            gpr(rd),
            gpr(rm),
            amount
        ),
        ArmT32RegisterShift::Ror(amount) => format!(
            "ror{} {}, {}, #{}",
            wide_flag(set_flags),
            gpr(rd),
            gpr(rm),
            amount
        ),
        ArmT32RegisterShift::Rrx => {
            format!("rrx{} {}, {}", flag_suffix(set_flags), gpr(rd), gpr(rm))
        }
    }
}

// Render an Rm shift suffix (always decimal -- shift counts are not affected by the immediate-radix flavor):
// "" for no shift (LSL #0), otherwise e.g. ", lsl #3".
fn render_shift(shift: &ArmT32RegisterShift) -> String {
    match shift {
        ArmT32RegisterShift::Lsl(0) => String::new(),
        ArmT32RegisterShift::Lsl(amount) => format!(", lsl #{}", amount),
        ArmT32RegisterShift::Lsr(amount) => format!(", lsr #{}", amount),
        ArmT32RegisterShift::Asr(amount) => format!(", asr #{}", amount),
        ArmT32RegisterShift::Ror(amount) => format!(", ror #{}", amount),
        ArmT32RegisterShift::Rrx => ", rrx".to_string(),
    }
}

// `.w` (force wide) plus the optional flag-setting `s`: "s.w" or ".w".
fn wide_flag(set_flags: bool) -> &'static str {
    if set_flags { "s.w" } else { ".w" }
}

// just the optional flag-setting `s` (for forms that are not written with a `.w`).
fn flag_suffix(set_flags: bool) -> &'static str {
    if set_flags { "s" } else { "" }
}

// ---- operand renderers ----

fn imm(syntax: ArmAssemblySyntax, value: i64) -> String {
    match syntax {
        ArmAssemblySyntax::Gnu => format!("#{}", value),
        ArmAssemblySyntax::Llvm => {
            if value < 0 {
                format!("#-0x{:x}", -value)
            } else {
                format!("#0x{:x}", value)
            }
        }
    }
}

fn gpr(register: &Arm32GeneralPurposeRegister) -> &'static str {
    match register {
        Arm32GeneralPurposeRegister::R0 => "r0",
        Arm32GeneralPurposeRegister::R1 => "r1",
        Arm32GeneralPurposeRegister::R2 => "r2",
        Arm32GeneralPurposeRegister::R3 => "r3",
        Arm32GeneralPurposeRegister::R4 => "r4",
        Arm32GeneralPurposeRegister::R5 => "r5",
        Arm32GeneralPurposeRegister::R6 => "r6",
        Arm32GeneralPurposeRegister::R7 => "r7",
        Arm32GeneralPurposeRegister::R8 => "r8",
        Arm32GeneralPurposeRegister::R9 => "r9",
        Arm32GeneralPurposeRegister::R10 => "r10",
        Arm32GeneralPurposeRegister::R11 => "r11",
        Arm32GeneralPurposeRegister::R12 => "r12",
        Arm32GeneralPurposeRegister::R13 => "sp",
        Arm32GeneralPurposeRegister::R14 => "lr",
        Arm32GeneralPurposeRegister::R15 => "pc",
    }
}

fn low(register: &Arm32LowGeneralPurposeRegister) -> &'static str {
    match register {
        Arm32LowGeneralPurposeRegister::R0 => "r0",
        Arm32LowGeneralPurposeRegister::R1 => "r1",
        Arm32LowGeneralPurposeRegister::R2 => "r2",
        Arm32LowGeneralPurposeRegister::R3 => "r3",
        Arm32LowGeneralPurposeRegister::R4 => "r4",
        Arm32LowGeneralPurposeRegister::R5 => "r5",
        Arm32LowGeneralPurposeRegister::R6 => "r6",
        Arm32LowGeneralPurposeRegister::R7 => "r7",
    }
}

fn cond_suffix(condition: &ArmT32InstructionCondition) -> &'static str {
    match condition {
        ArmT32InstructionCondition::Equal => "eq",
        ArmT32InstructionCondition::NotEqual => "ne",
        ArmT32InstructionCondition::CarrySet => "cs",
        ArmT32InstructionCondition::CarryClear => "cc",
        ArmT32InstructionCondition::MinusNegative => "mi",
        ArmT32InstructionCondition::PlusPositiveOrZero => "pl",
        ArmT32InstructionCondition::Overflow => "vs",
        ArmT32InstructionCondition::NoOverflow => "vc",
        ArmT32InstructionCondition::UnsignedHigher => "hi",
        ArmT32InstructionCondition::UnsignedLowerOrSame => "ls",
        ArmT32InstructionCondition::SignedGreaterThanOrEqual => "ge",
        ArmT32InstructionCondition::SignedLessThan => "lt",
        ArmT32InstructionCondition::SignedGreaterThan => "gt",
        ArmT32InstructionCondition::SignedLessThanOrEqual => "le",
        ArmT32InstructionCondition::AlwaysUnconditional => "al",
        ArmT32InstructionCondition::Undefined(_) => "",
    }
}

fn special_register(register: &ArmT32SpecialRegister) -> String {
    match register {
        ArmT32SpecialRegister::Apsr => "APSR".to_string(),
        ArmT32SpecialRegister::Iapsr => "IAPSR".to_string(),
        ArmT32SpecialRegister::Eapsr => "EAPSR".to_string(),
        ArmT32SpecialRegister::Xpsr => "XPSR".to_string(),
        ArmT32SpecialRegister::Ipsr => "IPSR".to_string(),
        ArmT32SpecialRegister::Epsr => "EPSR".to_string(),
        ArmT32SpecialRegister::Iepsr => "IEPSR".to_string(),
        ArmT32SpecialRegister::Msp => "MSP".to_string(),
        ArmT32SpecialRegister::Psp => "PSP".to_string(),
        ArmT32SpecialRegister::Primask => "PRIMASK".to_string(),
        ArmT32SpecialRegister::Control => "CONTROL".to_string(),
        ArmT32SpecialRegister::Reserved(bits) => format!("#{}", bits),
    }
}

fn barrier_option(option: &ArmT32MemoryBarrierOption) -> String {
    match option {
        ArmT32MemoryBarrierOption::System => "sy".to_string(),
        ArmT32MemoryBarrierOption::Undefined(bits) => format!("#{}", bits),
    }
}

fn register_list(registers: &[Arm32GeneralPurposeRegister]) -> String {
    let names: Vec<&str> = registers.iter().map(|register| gpr(register)).collect();
    format!("{{{}}}", names.join(", "))
}

fn low_register_list(registers: &[Arm32LowGeneralPurposeRegister]) -> String {
    let names: Vec<&str> = registers.iter().map(|register| low(register)).collect();
    format!("{{{}}}", names.join(", "))
}

// ---- PC-relative target resolution ----

// In Thumb, the value read from PC is the instruction's address + 4 (independent of the instruction's
// own size). Literal/ADR loads additionally align that PC value down to a word boundary.
fn pc_relative_target(instruction_address: u32, offset: i64, word_align_pc: bool) -> u32 {
    let pc = instruction_address as i64 + 4;
    let base = if word_align_pc { pc & !3 } else { pc };
    (base + offset) as u32
}

fn render_branch(
    mnemonic: &str,
    instruction_address: Option<u32>,
    offset: i64,
    syntax: ArmAssemblySyntax,
) -> String {
    match instruction_address {
        Some(address) => format!(
            "{} 0x{:08x}",
            mnemonic,
            pc_relative_target(address, offset, false)
        ),
        None => format!("{} {}", mnemonic, imm(syntax, offset)),
    }
}

// The b_label operand of a Branch Future: the absolute address (PC + 2*boff) when known, otherwise the raw
// `boff` field as a `#<boff>` immediate, matching the assembler's numeric syntax.
fn bf_blabel(instruction_address: Option<u32>, boff: u8, syntax: ArmAssemblySyntax) -> String {
    match instruction_address {
        Some(address) => format!(
            "0x{:08x}",
            pc_relative_target(address, 2 * boff as i64, false)
        ),
        None => imm(syntax, boff as i64),
    }
}

// The target operand of a Branch Future: the absolute address when known, else a `#<offset>` immediate.
fn bf_target(instruction_address: Option<u32>, offset: i64, syntax: ArmAssemblySyntax) -> String {
    match instruction_address {
        Some(address) => format!("0x{:08x}", pc_relative_target(address, offset, false)),
        None => imm(syntax, offset),
    }
}

// A VCX register operand: single (kind 0 -> `s<n>`), double (1 -> `d<n>`) or vector (2 -> `q<n>`).
fn vcx_reg(kind: u8, num: u8) -> String {
    match kind {
        0 => format!("s{}", num),
        1 => format!("d{}", num),
        _ => format!("q{}", num),
    }
}

// CBZ / CBNZ: like render_branch but with the test register before the target.
fn render_compare_branch(
    mnemonic: &str,
    rn: &str,
    instruction_address: Option<u32>,
    offset: i64,
    syntax: ArmAssemblySyntax,
) -> String {
    match instruction_address {
        Some(address) => format!(
            "{} {}, 0x{:08x}",
            mnemonic,
            rn,
            pc_relative_target(address, offset, false)
        ),
        None => format!("{} {}, {}", mnemonic, rn, imm(syntax, offset)),
    }
}

// `it{t/e}...  <firstcond>`. The mask's lowest set bit gives the block length; each higher bit is `t` when
// it equals firstcond[0], else `e`.
fn render_it(firstcond: &ArmT32InstructionCondition, mask: u8) -> String {
    let firstcond_low_bit = firstcond.as_operand_bits() & 1;
    let length = 4 - mask.trailing_zeros() as usize; // 1..=4
    let mut mnemonic = String::from("it");
    for slot in 2..=length {
        let bit = (mask >> (5 - slot)) & 1;
        mnemonic.push(if bit == firstcond_low_bit { 't' } else { 'e' });
    }
    format!("{} {}", mnemonic, cond_suffix(firstcond))
}

fn render_literal_load(
    mnemonic: &str,
    rt: &str,
    instruction_address: Option<u32>,
    offset: i64,
    syntax: ArmAssemblySyntax,
) -> String {
    match instruction_address {
        Some(address) => format!(
            "{} {}, [pc, {}]  ; 0x{:08x}",
            mnemonic,
            rt,
            imm(syntax, offset),
            pc_relative_target(address, offset, true)
        ),
        None => format!("{} {}, [pc, {}]", mnemonic, rt, imm(syntax, offset)),
    }
}

#[cfg(test)]
mod it_block_condition_tests {
    use super::apply_it_block_condition;
    use crate::enums::ArmT32InstructionCondition as Cond;

    #[test]
    fn inserts_condition_after_mnemonic_and_before_w() {
        assert_eq!(
            apply_it_block_condition("mov r0, r1", Cond::Equal),
            "moveq r0, r1"
        );
        assert_eq!(
            apply_it_block_condition("add.w r0, r1, r2", Cond::NotEqual),
            "addne.w r0, r1, r2"
        );
    }

    #[test]
    fn drops_flag_setting_s_on_narrow_dp_inside_it() {
        // The 16-bit DP encodings do not set flags inside an IT block, so UAL drops the `s`. Verified
        // against LLVM and GNU objdump: movs/adds/ands -> movle/addeq/andne.
        assert_eq!(
            apply_it_block_condition("movs r2, #0", Cond::SignedLessThanOrEqual),
            "movle r2, #0"
        );
        assert_eq!(
            apply_it_block_condition("adds r0, r1, r2", Cond::Equal),
            "addeq r0, r1, r2"
        );
        assert_eq!(
            apply_it_block_condition("ands r3, r4", Cond::NotEqual),
            "andne r3, r4"
        );
    }

    #[test]
    fn keeps_s_on_wide_form_and_non_dp_mnemonics() {
        // A 32-bit flag-setting form has an explicit S bit and keeps its `s` inside an IT block.
        assert_eq!(
            apply_it_block_condition("adds.w r0, r1, #1", Cond::Equal),
            "addseq.w r0, r1, #1"
        );
        // A mnemonic that ends in `s` but is not a narrow flag-setting DP op is left untouched.
        assert_eq!(
            apply_it_block_condition("vmrs apsr_nzcv, fpscr", Cond::Equal),
            "vmrseq apsr_nzcv, fpscr"
        );
    }
}
