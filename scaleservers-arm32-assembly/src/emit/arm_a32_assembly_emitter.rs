// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// UAL (Unified Assembly Language) emitter for ArmA32Instruction -- the A32 ("ARM" state) analogue of the T32
// emitter. The disassembler (arm32dasm) renders each decoded A32 instruction through this layer, and the
// differential oracle re-assembles the output to confirm it round-trips byte-for-byte against GNU `as`.
//
// A32 rendering conventions (chosen to match GNU objdump / the assembler so output re-assembles):
//   * the condition code is a mnemonic suffix (`addeq`, `ldrne`); AL prints as no suffix;
//   * the flag-setting S bit is the `s` suffix (`adds`, `movs`);
//   * registers are lowercase with r13/r14/r15 shown as sp/lr/pc; S/D/Q FP registers as sN/dN/qN;
//   * immediates print per `ArmAssemblySyntax` (decimal `#N` for GNU, hex for LLVM);
//   * PC-relative branch targets print as offsets in the raw form, or absolute `0x...` in the address-aware form.

// `String`/`Vec`/`ToString` (for `&str::to_string`) are not in the `no_std` prelude; pull them from `alloc`
// (the `format!` macro comes from the crate-level `#[macro_use] extern crate alloc`).
use crate::ArmA32Instruction;
use crate::emit::ArmAssemblySyntax;
use crate::enums::{
    Arm32BlockAddressMode, Arm32Condition, Arm32CpsMode, Arm32DirectedRound,
    Arm32DoublePrecisionRegister, Arm32ExtendType, Arm32FpDataOperation2, Arm32FpDataOperation3,
    Arm32GeneralPurposeRegister, Arm32IndexMode, Arm32MemoryOffset, Arm32MemoryOffset8,
    Arm32NeonAesOp, Arm32NeonBitwiseOp, Arm32NeonDiffLongOp, Arm32NeonDiffNarrowOp,
    Arm32NeonDiffWideOp, Arm32NeonFloatOp, Arm32NeonIntegerOp, Arm32NeonLoadStoreAddress,
    Arm32NeonMisc2FixedOp, Arm32NeonMisc2SizedOp, Arm32NeonNarrowOp, Arm32NeonScalarLongOp,
    Arm32NeonScalarOp, Arm32NeonSha2Op, Arm32NeonSha3Op, Arm32NeonShiftNarrowOp, Arm32NeonShiftOp,
    Arm32NeonSize, Arm32QuadwordRegister, Arm32RegisterShift, Arm32ShiftType,
    Arm32SinglePrecisionRegister, Arm32VmovLaneSize, Arm32VrintMode, Arm32VselCondition,
};
use crate::floating_point_immediate::vfp_expand_imm8_to_f64;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

impl ArmA32Instruction {
    /// Render this instruction as a UAL (Unified Assembly Language) string in the requested
    /// [`ArmAssemblySyntax`] (LLVM or GNU). With no surrounding address context, PC-relative branch
    /// operands are shown as signed offsets.
    pub fn to_assembly_string(&self, syntax: ArmAssemblySyntax) -> String {
        self.render(None, syntax)
    }

    // Address-aware UAL: PC-relative branch operands are resolved to absolute targets, given the address at
    // which this instruction begins. Used by the disassembler.
    pub fn to_assembly_string_at(
        &self,
        instruction_address: u32,
        syntax: ArmAssemblySyntax,
    ) -> String {
        self.render(Some(instruction_address), syntax)
    }

    fn render(&self, instruction_address: Option<u32>, syntax: ArmAssemblySyntax) -> String {
        use ArmA32Instruction::*;
        match self {
            // ---- data processing: Rd, Rn, op2 ----
            And_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("and", c, *s, rd, rn, *imm32, syntax),
            And_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("and", c, *s, rd, rn, rm, sh),
            And_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("and", c, *s, rd, rn, rm, *ty, rs)
            }
            Eor_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("eor", c, *s, rd, rn, *imm32, syntax),
            Eor_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("eor", c, *s, rd, rn, rm, sh),
            Eor_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("eor", c, *s, rd, rn, rm, *ty, rs)
            }
            Sub_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("sub", c, *s, rd, rn, *imm32, syntax),
            Sub_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("sub", c, *s, rd, rn, rm, sh),
            Sub_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("sub", c, *s, rd, rn, rm, *ty, rs)
            }
            Rsb_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("rsb", c, *s, rd, rn, *imm32, syntax),
            Rsb_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("rsb", c, *s, rd, rn, rm, sh),
            Rsb_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("rsb", c, *s, rd, rn, rm, *ty, rs)
            }
            Add_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("add", c, *s, rd, rn, *imm32, syntax),
            Add_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("add", c, *s, rd, rn, rm, sh),
            Add_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("add", c, *s, rd, rn, rm, *ty, rs)
            }
            Adc_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("adc", c, *s, rd, rn, *imm32, syntax),
            Adc_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("adc", c, *s, rd, rn, rm, sh),
            Adc_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("adc", c, *s, rd, rn, rm, *ty, rs)
            }
            Sbc_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("sbc", c, *s, rd, rn, *imm32, syntax),
            Sbc_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("sbc", c, *s, rd, rn, rm, sh),
            Sbc_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("sbc", c, *s, rd, rn, rm, *ty, rs)
            }
            Rsc_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("rsc", c, *s, rd, rn, *imm32, syntax),
            Rsc_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("rsc", c, *s, rd, rn, rm, sh),
            Rsc_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("rsc", c, *s, rd, rn, rm, *ty, rs)
            }
            Orr_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("orr", c, *s, rd, rn, *imm32, syntax),
            Orr_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("orr", c, *s, rd, rn, rm, sh),
            Orr_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("orr", c, *s, rd, rn, rm, *ty, rs)
            }
            Bic_Immediate_A1(c, s, rd, rn, imm32) => dp_imm("bic", c, *s, rd, rn, *imm32, syntax),
            Bic_Register_A1(c, s, rd, rn, rm, sh) => dp_reg("bic", c, *s, rd, rn, rm, sh),
            Bic_RegisterShiftedRegister_A1(c, s, rd, rn, rm, ty, rs) => {
                dp_rsr("bic", c, *s, rd, rn, rm, *ty, rs)
            }

            // ---- moves (Rd, op2) ----
            Mov_Immediate_A1(c, s, rd, imm32) => format!(
                "mov{}{} {}, {}",
                s_flag(*s),
                cc(c),
                gpr(rd),
                imm(syntax, *imm32 as i64)
            ),
            Mov_Register_A1(c, s, rd, rm, sh) => render_mov_shift(c, *s, rd, rm, sh),
            Mov_RegisterShiftedRegister_A1(c, s, rd, rm, ty, rs) => format!(
                "{}{}{} {}, {}, {}",
                shift_type_mnemonic(*ty),
                s_flag(*s),
                cc(c),
                gpr(rd),
                gpr(rm),
                gpr(rs)
            ),
            Mvn_Immediate_A1(c, s, rd, imm32) => format!(
                "mvn{}{} {}, {}",
                s_flag(*s),
                cc(c),
                gpr(rd),
                imm(syntax, *imm32 as i64)
            ),
            Mvn_Register_A1(c, s, rd, rm, sh) => format!(
                "mvn{}{} {}, {}{}",
                s_flag(*s),
                cc(c),
                gpr(rd),
                gpr(rm),
                shift_suffix(sh)
            ),
            Mvn_RegisterShiftedRegister_A1(c, s, rd, rm, ty, rs) => format!(
                "mvn{}{} {}, {}, {} {}",
                s_flag(*s),
                cc(c),
                gpr(rd),
                gpr(rm),
                shift_type_mnemonic(*ty),
                gpr(rs)
            ),

            // ---- compares (Rn, op2): no Rd, always set flags ----
            Tst_Immediate_A1(c, rn, imm32) => {
                format!("tst{} {}, {}", cc(c), gpr(rn), imm(syntax, *imm32 as i64))
            }
            Tst_Register_A1(c, rn, rm, sh) => {
                format!("tst{} {}, {}{}", cc(c), gpr(rn), gpr(rm), shift_suffix(sh))
            }
            Tst_RegisterShiftedRegister_A1(c, rn, rm, ty, rs) => format!(
                "tst{} {}, {}, {} {}",
                cc(c),
                gpr(rn),
                gpr(rm),
                shift_type_mnemonic(*ty),
                gpr(rs)
            ),
            Teq_Immediate_A1(c, rn, imm32) => {
                format!("teq{} {}, {}", cc(c), gpr(rn), imm(syntax, *imm32 as i64))
            }
            Teq_Register_A1(c, rn, rm, sh) => {
                format!("teq{} {}, {}{}", cc(c), gpr(rn), gpr(rm), shift_suffix(sh))
            }
            Teq_RegisterShiftedRegister_A1(c, rn, rm, ty, rs) => format!(
                "teq{} {}, {}, {} {}",
                cc(c),
                gpr(rn),
                gpr(rm),
                shift_type_mnemonic(*ty),
                gpr(rs)
            ),
            Cmp_Immediate_A1(c, rn, imm32) => {
                format!("cmp{} {}, {}", cc(c), gpr(rn), imm(syntax, *imm32 as i64))
            }
            Cmp_Register_A1(c, rn, rm, sh) => {
                format!("cmp{} {}, {}{}", cc(c), gpr(rn), gpr(rm), shift_suffix(sh))
            }
            Cmp_RegisterShiftedRegister_A1(c, rn, rm, ty, rs) => format!(
                "cmp{} {}, {}, {} {}",
                cc(c),
                gpr(rn),
                gpr(rm),
                shift_type_mnemonic(*ty),
                gpr(rs)
            ),
            Cmn_Immediate_A1(c, rn, imm32) => {
                format!("cmn{} {}, {}", cc(c), gpr(rn), imm(syntax, *imm32 as i64))
            }
            Cmn_Register_A1(c, rn, rm, sh) => {
                format!("cmn{} {}, {}{}", cc(c), gpr(rn), gpr(rm), shift_suffix(sh))
            }
            Cmn_RegisterShiftedRegister_A1(c, rn, rm, ty, rs) => format!(
                "cmn{} {}, {}, {} {}",
                cc(c),
                gpr(rn),
                gpr(rm),
                shift_type_mnemonic(*ty),
                gpr(rs)
            ),

            // ---- 16-bit immediate moves ----
            Movw_A2(c, rd, imm16) => {
                format!("movw{} {}, {}", cc(c), gpr(rd), imm(syntax, *imm16 as i64))
            }
            Movt_A1(c, rd, imm16) => {
                format!("movt{} {}, {}", cc(c), gpr(rd), imm(syntax, *imm16 as i64))
            }

            // ---- multiply ----
            Mul_A1(c, s, rd, rn, rm) => format!(
                "mul{}{} {}, {}, {}",
                s_flag(*s),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Mla_A1(c, s, rd, rn, rm, ra) => format!(
                "mla{}{} {}, {}, {}, {}",
                s_flag(*s),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Mls_A1(c, rd, rn, rm, ra) => format!(
                "mls{} {}, {}, {}, {}",
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Umull_A1(c, s, lo, hi, rn, rm) => format!(
                "umull{}{} {}, {}, {}, {}",
                s_flag(*s),
                cc(c),
                gpr(lo),
                gpr(hi),
                gpr(rn),
                gpr(rm)
            ),
            Umlal_A1(c, s, lo, hi, rn, rm) => format!(
                "umlal{}{} {}, {}, {}, {}",
                s_flag(*s),
                cc(c),
                gpr(lo),
                gpr(hi),
                gpr(rn),
                gpr(rm)
            ),
            Smull_A1(c, s, lo, hi, rn, rm) => format!(
                "smull{}{} {}, {}, {}, {}",
                s_flag(*s),
                cc(c),
                gpr(lo),
                gpr(hi),
                gpr(rn),
                gpr(rm)
            ),
            Smlal_A1(c, s, lo, hi, rn, rm) => format!(
                "smlal{}{} {}, {}, {}, {}",
                s_flag(*s),
                cc(c),
                gpr(lo),
                gpr(hi),
                gpr(rn),
                gpr(rm)
            ),
            Umaal_A1(c, lo, hi, rn, rm) => format!(
                "umaal{} {}, {}, {}, {}",
                cc(c),
                gpr(lo),
                gpr(hi),
                gpr(rn),
                gpr(rm)
            ),

            // ---- saturating arithmetic ----
            Qadd_A1(c, rd, rm, rn) => {
                format!("qadd{} {}, {}, {}", cc(c), gpr(rd), gpr(rm), gpr(rn))
            }
            Qsub_A1(c, rd, rm, rn) => {
                format!("qsub{} {}, {}, {}", cc(c), gpr(rd), gpr(rm), gpr(rn))
            }
            Qdadd_A1(c, rd, rm, rn) => {
                format!("qdadd{} {}, {}, {}", cc(c), gpr(rd), gpr(rm), gpr(rn))
            }
            Qdsub_A1(c, rd, rm, rn) => {
                format!("qdsub{} {}, {}, {}", cc(c), gpr(rd), gpr(rm), gpr(rn))
            }

            // ---- signed multiplies ----
            Smla_A1(c, rd, rn, rm, ra, n, m) => format!(
                "smla{}{}{} {}, {}, {}, {}",
                tb(*n),
                tb(*m),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Smlaw_A1(c, rd, rn, rm, ra, m) => format!(
                "smlaw{}{} {}, {}, {}, {}",
                tb(*m),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Smulw_A1(c, rd, rn, rm, m) => format!(
                "smulw{}{} {}, {}, {}",
                tb(*m),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Smlal_Halfword_A1(c, lo, hi, rn, rm, n, m) => format!(
                "smlal{}{}{} {}, {}, {}, {}",
                tb(*n),
                tb(*m),
                cc(c),
                gpr(lo),
                gpr(hi),
                gpr(rn),
                gpr(rm)
            ),
            Smul_A1(c, rd, rn, rm, n, m) => format!(
                "smul{}{}{} {}, {}, {}",
                tb(*n),
                tb(*m),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Smlad_A1(c, rd, rn, rm, ra, x) => format!(
                "smlad{}{} {}, {}, {}, {}",
                xx(*x),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Smuad_A1(c, rd, rn, rm, x) => format!(
                "smuad{}{} {}, {}, {}",
                xx(*x),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Smlsd_A1(c, rd, rn, rm, ra, x) => format!(
                "smlsd{}{} {}, {}, {}, {}",
                xx(*x),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Smusd_A1(c, rd, rn, rm, x) => format!(
                "smusd{}{} {}, {}, {}",
                xx(*x),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Smmla_A1(c, rd, rn, rm, ra, r) => format!(
                "smmla{}{} {}, {}, {}, {}",
                rr(*r),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Smmul_A1(c, rd, rn, rm, r) => format!(
                "smmul{}{} {}, {}, {}",
                rr(*r),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Smmls_A1(c, rd, rn, rm, ra, r) => format!(
                "smmls{}{} {}, {}, {}, {}",
                rr(*r),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),
            Smlald_A1(c, lo, hi, rn, rm, x) => format!(
                "smlald{}{} {}, {}, {}, {}",
                xx(*x),
                cc(c),
                gpr(lo),
                gpr(hi),
                gpr(rn),
                gpr(rm)
            ),
            Smlsld_A1(c, lo, hi, rn, rm, x) => format!(
                "smlsld{}{} {}, {}, {}, {}",
                xx(*x),
                cc(c),
                gpr(lo),
                gpr(hi),
                gpr(rn),
                gpr(rm)
            ),

            // ---- parallel add/sub + select ----
            ParallelAddSub_A1(c, op, prefix, rd, rn, rm) => format!(
                "{}{}{} {}, {}, {}",
                prefix.mnemonic(),
                op.mnemonic(),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm)
            ),
            Sel_A1(c, rd, rn, rm) => format!("sel{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm)),

            // ---- extend / extend-and-add ----
            Extend_A1(c, ty, rd, rm, rot) => format!(
                "{}{} {}, {}{}",
                extend_mnemonic(*ty),
                cc(c),
                gpr(rd),
                gpr(rm),
                rotation(*rot)
            ),
            ExtendAndAdd_A1(c, ty, rd, rn, rm, rot) => format!(
                "{}{} {}, {}, {}{}",
                extend_add_mnemonic(*ty),
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                rotation(*rot)
            ),

            // ---- reverse / clz ----
            Rev_A1(c, rd, rm) => format!("rev{} {}, {}", cc(c), gpr(rd), gpr(rm)),
            Rev16_A1(c, rd, rm) => format!("rev16{} {}, {}", cc(c), gpr(rd), gpr(rm)),
            Revsh_A1(c, rd, rm) => format!("revsh{} {}, {}", cc(c), gpr(rd), gpr(rm)),
            Rbit_A1(c, rd, rm) => format!("rbit{} {}, {}", cc(c), gpr(rd), gpr(rm)),
            Clz_A1(c, rd, rm) => format!("clz{} {}, {}", cc(c), gpr(rd), gpr(rm)),

            // ---- pack / saturate / sad ----
            Pkhbt_A1(c, rd, rn, rm, lsl) => {
                if *lsl == 0 {
                    format!("pkhbt{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm))
                } else {
                    format!(
                        "pkhbt{} {}, {}, {}, lsl #{}",
                        cc(c),
                        gpr(rd),
                        gpr(rn),
                        gpr(rm),
                        lsl
                    )
                }
            }
            Pkhtb_A1(c, rd, rn, rm, asr) => format!(
                "pkhtb{} {}, {}, {}, asr #{}",
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                asr
            ),
            Ssat_A1(c, rd, sat, rm, sh) => format!(
                "ssat{} {}, {}, {}{}",
                cc(c),
                gpr(rd),
                imm(syntax, *sat as i64),
                gpr(rm),
                shift_suffix(sh)
            ),
            Usat_A1(c, rd, sat, rm, sh) => format!(
                "usat{} {}, {}, {}{}",
                cc(c),
                gpr(rd),
                imm(syntax, *sat as i64),
                gpr(rm),
                shift_suffix(sh)
            ),
            Ssat16_A1(c, rd, sat, rn) => format!(
                "ssat16{} {}, {}, {}",
                cc(c),
                gpr(rd),
                imm(syntax, *sat as i64),
                gpr(rn)
            ),
            Usat16_A1(c, rd, sat, rn) => format!(
                "usat16{} {}, {}, {}",
                cc(c),
                gpr(rd),
                imm(syntax, *sat as i64),
                gpr(rn)
            ),
            Usad8_A1(c, rd, rn, rm) => {
                format!("usad8{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm))
            }
            Usada8_A1(c, rd, rn, rm, ra) => format!(
                "usada8{} {}, {}, {}, {}",
                cc(c),
                gpr(rd),
                gpr(rn),
                gpr(rm),
                gpr(ra)
            ),

            // ---- bitfield ----
            Bfc_A1(c, rd, lsb, width) => format!(
                "bfc{} {}, {}, {}",
                cc(c),
                gpr(rd),
                imm(syntax, *lsb as i64),
                imm(syntax, *width as i64)
            ),
            Bfi_A1(c, rd, rn, lsb, width) => format!(
                "bfi{} {}, {}, {}, {}",
                cc(c),
                gpr(rd),
                gpr(rn),
                imm(syntax, *lsb as i64),
                imm(syntax, *width as i64)
            ),
            Sbfx_A1(c, rd, rn, lsb, width) => format!(
                "sbfx{} {}, {}, {}, {}",
                cc(c),
                gpr(rd),
                gpr(rn),
                imm(syntax, *lsb as i64),
                imm(syntax, *width as i64)
            ),
            Ubfx_A1(c, rd, rn, lsb, width) => format!(
                "ubfx{} {}, {}, {}, {}",
                cc(c),
                gpr(rd),
                gpr(rn),
                imm(syntax, *lsb as i64),
                imm(syntax, *width as i64)
            ),

            // ---- load/store single ----
            Ldr_A1(c, rt, rn, off, idx) => {
                format!("ldr{} {}, {}", cc(c), gpr(rt), mem12(rn, off, *idx, syntax))
            }
            Str_A1(c, rt, rn, off, idx) => {
                format!("str{} {}, {}", cc(c), gpr(rt), mem12(rn, off, *idx, syntax))
            }
            Ldrb_A1(c, rt, rn, off, idx) => format!(
                "ldrb{} {}, {}",
                cc(c),
                gpr(rt),
                mem12(rn, off, *idx, syntax)
            ),
            Strb_A1(c, rt, rn, off, idx) => format!(
                "strb{} {}, {}",
                cc(c),
                gpr(rt),
                mem12(rn, off, *idx, syntax)
            ),
            Ldrt_A1(c, rt, rn, off) => {
                format!("ldrt{} {}, {}", cc(c), gpr(rt), mem12_post(rn, off, syntax))
            }
            Strt_A1(c, rt, rn, off) => {
                format!("strt{} {}, {}", cc(c), gpr(rt), mem12_post(rn, off, syntax))
            }
            Ldrbt_A1(c, rt, rn, off) => format!(
                "ldrbt{} {}, {}",
                cc(c),
                gpr(rt),
                mem12_post(rn, off, syntax)
            ),
            Strbt_A1(c, rt, rn, off) => format!(
                "strbt{} {}, {}",
                cc(c),
                gpr(rt),
                mem12_post(rn, off, syntax)
            ),

            // ---- load/store halfword / dual / signed ----
            Ldrh_A1(c, rt, rn, off, idx) => {
                format!("ldrh{} {}, {}", cc(c), gpr(rt), mem8(rn, off, *idx, syntax))
            }
            Strh_A1(c, rt, rn, off, idx) => {
                format!("strh{} {}, {}", cc(c), gpr(rt), mem8(rn, off, *idx, syntax))
            }
            Ldrsb_A1(c, rt, rn, off, idx) => format!(
                "ldrsb{} {}, {}",
                cc(c),
                gpr(rt),
                mem8(rn, off, *idx, syntax)
            ),
            Ldrsh_A1(c, rt, rn, off, idx) => format!(
                "ldrsh{} {}, {}",
                cc(c),
                gpr(rt),
                mem8(rn, off, *idx, syntax)
            ),
            Ldrd_A1(c, rt, rn, off, idx) => format!(
                "ldrd{} {}, {}, {}",
                cc(c),
                gpr(rt),
                gpr(&next_reg(rt)),
                mem8(rn, off, *idx, syntax)
            ),
            Strd_A1(c, rt, rn, off, idx) => format!(
                "strd{} {}, {}, {}",
                cc(c),
                gpr(rt),
                gpr(&next_reg(rt)),
                mem8(rn, off, *idx, syntax)
            ),
            Ldrht_A1(c, rt, rn, off) => {
                format!("ldrht{} {}, {}", cc(c), gpr(rt), mem8_post(rn, off, syntax))
            }
            Strht_A1(c, rt, rn, off) => {
                format!("strht{} {}, {}", cc(c), gpr(rt), mem8_post(rn, off, syntax))
            }
            Ldrsbt_A1(c, rt, rn, off) => format!(
                "ldrsbt{} {}, {}",
                cc(c),
                gpr(rt),
                mem8_post(rn, off, syntax)
            ),
            Ldrsht_A1(c, rt, rn, off) => format!(
                "ldrsht{} {}, {}",
                cc(c),
                gpr(rt),
                mem8_post(rn, off, syntax)
            ),

            // ---- load/store multiple (PUSH/POP spellings for sp! IA/DB) ----
            Ldm_A1(c, mode, rn, wb, user, regs) => {
                render_ldm_stm("ldm", c, *mode, rn, *wb, *user, regs)
            }
            Stm_A1(c, mode, rn, wb, user, regs) => {
                render_ldm_stm("stm", c, *mode, rn, *wb, *user, regs)
            }

            // ---- synchronization ----
            Ldrex_A1(c, rt, rn) => format!("ldrex{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Strex_A1(c, rd, rt, rn) => {
                format!("strex{} {}, {}, [{}]", cc(c), gpr(rd), gpr(rt), gpr(rn))
            }
            Ldrexb_A1(c, rt, rn) => format!("ldrexb{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Strexb_A1(c, rd, rt, rn) => {
                format!("strexb{} {}, {}, [{}]", cc(c), gpr(rd), gpr(rt), gpr(rn))
            }
            Ldrexh_A1(c, rt, rn) => format!("ldrexh{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Strexh_A1(c, rd, rt, rn) => {
                format!("strexh{} {}, {}, [{}]", cc(c), gpr(rd), gpr(rt), gpr(rn))
            }
            Ldrexd_A1(c, rt, rn) => format!(
                "ldrexd{} {}, {}, [{}]",
                cc(c),
                gpr(rt),
                gpr(&next_reg(rt)),
                gpr(rn)
            ),
            Strexd_A1(c, rd, rt, rn) => format!(
                "strexd{} {}, {}, {}, [{}]",
                cc(c),
                gpr(rd),
                gpr(rt),
                gpr(&next_reg(rt)),
                gpr(rn)
            ),
            Clrex_A1 => "clrex".to_string(),
            Swp_A1(c, rt, rt2, rn) => {
                format!("swp{} {}, {}, [{}]", cc(c), gpr(rt), gpr(rt2), gpr(rn))
            }
            Swpb_A1(c, rt, rt2, rn) => {
                format!("swpb{} {}, {}, [{}]", cc(c), gpr(rt), gpr(rt2), gpr(rn))
            }

            // ---- status / system ----
            Mrs_A1(c, spsr, rd) => format!("mrs{} {}, {}", cc(c), gpr(rd), psr(*spsr)),
            Msr_Register_A1(c, spsr, mask, rn) => {
                format!("msr{} {}, {}", cc(c), psr_fields(*spsr, *mask), gpr(rn))
            }
            Msr_Immediate_A1(c, spsr, mask, imm32) => format!(
                "msr{} {}, {}",
                cc(c),
                psr_fields(*spsr, *mask),
                imm(syntax, *imm32 as i64)
            ),
            MrsBanked_A1(c, spsr, sysm, rd) => {
                format!("mrs{} {}, {}", cc(c), gpr(rd), banked_reg(*spsr, *sysm))
            }
            MsrBanked_A1(c, spsr, sysm, rn) => {
                format!("msr{} {}, {}", cc(c), banked_reg(*spsr, *sysm), gpr(rn))
            }
            Cps_A1(mode, a, i, f, new_mode) => render_cps(*mode, *a, *i, *f, *new_mode, syntax),
            Setend_A1(big_endian) => format!("setend {}", if *big_endian { "be" } else { "le" }),
            Setpan_A1(pan) => format!("setpan {}", imm(syntax, if *pan { 1 } else { 0 })),

            // ---- coprocessor ----
            Mcr_A1(c, cp, opc1, rt, crn, crm, opc2) => {
                render_mcr("mcr", cc(c), *cp, *opc1, gpr(rt), *crn, *crm, *opc2)
            }
            Mrc_A1(c, cp, opc1, rt, crn, crm, opc2) => {
                render_mcr("mrc", cc(c), *cp, *opc1, gpr(rt), *crn, *crm, *opc2)
            }
            Mcr2_A1(cp, opc1, rt, crn, crm, opc2) => {
                render_mcr("mcr2", "", *cp, *opc1, gpr(rt), *crn, *crm, *opc2)
            }
            Mrc2_A1(cp, opc1, rt, crn, crm, opc2) => {
                render_mcr("mrc2", "", *cp, *opc1, gpr(rt), *crn, *crm, *opc2)
            }
            Cdp_A1(c, cp, opc1, crd, crn, crm, opc2) => format!(
                "cdp{} p{}, {}, c{}, c{}, c{}, {}",
                cc(c),
                cp,
                opc1,
                crd,
                crn,
                crm,
                opc2
            ),
            Cdp2_A1(cp, opc1, crd, crn, crm, opc2) => format!(
                "cdp2 p{}, {}, c{}, c{}, c{}, {}",
                cp, opc1, crd, crn, crm, opc2
            ),
            Mcrr_A1(c, cp, opc1, rt, rt2, crm) => format!(
                "mcrr{} p{}, {}, {}, {}, c{}",
                cc(c),
                cp,
                opc1,
                gpr(rt),
                gpr(rt2),
                crm
            ),
            Mrrc_A1(c, cp, opc1, rt, rt2, crm) => format!(
                "mrrc{} p{}, {}, {}, {}, c{}",
                cc(c),
                cp,
                opc1,
                gpr(rt),
                gpr(rt2),
                crm
            ),
            Mcrr2_A1(cp, opc1, rt, rt2, crm) => format!(
                "mcrr2 p{}, {}, {}, {}, c{}",
                cp,
                opc1,
                gpr(rt),
                gpr(rt2),
                crm
            ),
            Mrrc2_A1(cp, opc1, rt, rt2, crm) => format!(
                "mrrc2 p{}, {}, {}, {}, c{}",
                cp,
                opc1,
                gpr(rt),
                gpr(rt2),
                crm
            ),
            Ldc_A1(c, cp, long, crd, rn, add, imm8, idx) => render_ldc(
                &format!("ldc{}{}", long_suffix(*long), cc(c)),
                *cp,
                *crd,
                rn,
                *add,
                *imm8,
                *idx,
                syntax,
            ),
            Stc_A1(c, cp, long, crd, rn, add, imm8, idx) => render_ldc(
                &format!("stc{}{}", long_suffix(*long), cc(c)),
                *cp,
                *crd,
                rn,
                *add,
                *imm8,
                *idx,
                syntax,
            ),
            Ldc2_A1(cp, long, crd, rn, add, imm8, idx) => render_ldc(
                &format!("ldc2{}", long_suffix(*long)),
                *cp,
                *crd,
                rn,
                *add,
                *imm8,
                *idx,
                syntax,
            ),
            Stc2_A1(cp, long, crd, rn, add, imm8, idx) => render_ldc(
                &format!("stc2{}", long_suffix(*long)),
                *cp,
                *crd,
                rn,
                *add,
                *imm8,
                *idx,
                syntax,
            ),

            // ---- hints / barriers / exceptions ----
            Nop_A1(c) => format!("nop{}", cc(c)),
            Yield_A1(c) => format!("yield{}", cc(c)),
            Wfe_A1(c) => format!("wfe{}", cc(c)),
            Wfi_A1(c) => format!("wfi{}", cc(c)),
            Sev_A1(c) => format!("sev{}", cc(c)),
            Sevl_A1(c) => format!("sevl{}", cc(c)),
            Csdb_A1(c) => format!("csdb{}", cc(c)),
            Esb_A1(c) => format!("esb{}", cc(c)),
            Dbg_A1(c, option) => format!("dbg{} {}", cc(c), imm(syntax, *option as i64)),
            Dmb_A1(option) => format!("dmb {}", barrier(*option)),
            Dsb_A1(option) => format!("dsb {}", barrier(*option)),
            Isb_A1(option) => format!("isb {}", barrier(*option)),
            Sb_A1 => "sb".to_string(),
            Bkpt_A1(c, imm16) => format!("bkpt{} {}", cc(c), imm(syntax, *imm16 as i64)),
            Hlt_A1(c, imm16) => format!("hlt{} {}", cc(c), imm(syntax, *imm16 as i64)),
            Hvc_A1(c, imm16) => format!("hvc{} {}", cc(c), imm(syntax, *imm16 as i64)),
            Smc_A1(c, imm4) => format!("smc{} {}", cc(c), imm(syntax, *imm4 as i64)),
            Udf_A1(c, imm16) => format!("udf{} {}", cc(c), imm(syntax, *imm16 as i64)),
            Eret_A1(c) => format!("eret{}", cc(c)),
            Svc_A1(c, imm24) => format!("svc{} {}", cc(c), imm(syntax, *imm24 as i64)),

            // ---- CRC32 ----
            Crc32b_A1(c, rd, rn, rm) => {
                format!("crc32b{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm))
            }
            Crc32h_A1(c, rd, rn, rm) => {
                format!("crc32h{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm))
            }
            Crc32w_A1(c, rd, rn, rm) => {
                format!("crc32w{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm))
            }
            Crc32cb_A1(c, rd, rn, rm) => {
                format!("crc32cb{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm))
            }
            Crc32ch_A1(c, rd, rn, rm) => {
                format!("crc32ch{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm))
            }
            Crc32cw_A1(c, rd, rn, rm) => {
                format!("crc32cw{} {}, {}, {}", cc(c), gpr(rd), gpr(rn), gpr(rm))
            }

            // ---- load-acquire / store-release ----
            Lda_A1(c, rt, rn) => format!("lda{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Ldab_A1(c, rt, rn) => format!("ldab{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Ldah_A1(c, rt, rn) => format!("ldah{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Stl_A1(c, rt, rn) => format!("stl{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Stlb_A1(c, rt, rn) => format!("stlb{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Stlh_A1(c, rt, rn) => format!("stlh{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Ldaex_A1(c, rt, rn) => format!("ldaex{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Ldaexb_A1(c, rt, rn) => format!("ldaexb{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Ldaexh_A1(c, rt, rn) => format!("ldaexh{} {}, [{}]", cc(c), gpr(rt), gpr(rn)),
            Ldaexd_A1(c, rt, rn) => format!(
                "ldaexd{} {}, {}, [{}]",
                cc(c),
                gpr(rt),
                gpr(&next_reg(rt)),
                gpr(rn)
            ),
            Stlex_A1(c, rd, rt, rn) => {
                format!("stlex{} {}, {}, [{}]", cc(c), gpr(rd), gpr(rt), gpr(rn))
            }
            Stlexb_A1(c, rd, rt, rn) => {
                format!("stlexb{} {}, {}, [{}]", cc(c), gpr(rd), gpr(rt), gpr(rn))
            }
            Stlexh_A1(c, rd, rt, rn) => {
                format!("stlexh{} {}, {}, [{}]", cc(c), gpr(rd), gpr(rt), gpr(rn))
            }
            Stlexd_A1(c, rd, rt, rn) => format!(
                "stlexd{} {}, {}, {}, [{}]",
                cc(c),
                gpr(rd),
                gpr(rt),
                gpr(&next_reg(rt)),
                gpr(rn)
            ),

            // ---- VFP load/store ----
            Vldr_Single_A1(c, sd, rn, off) => format!(
                "vldr{} {}, {}",
                cc(c),
                single(sd),
                fp_mem(rn, *off as i64, syntax)
            ),
            Vstr_Single_A1(c, sd, rn, off) => format!(
                "vstr{} {}, {}",
                cc(c),
                single(sd),
                fp_mem(rn, *off as i64, syntax)
            ),
            Vldr_Double_A1(c, dd, rn, off) => format!(
                "vldr{} {}, {}",
                cc(c),
                double(dd),
                fp_mem(rn, *off as i64, syntax)
            ),
            Vstr_Double_A1(c, dd, rn, off) => format!(
                "vstr{} {}, {}",
                cc(c),
                double(dd),
                fp_mem(rn, *off as i64, syntax)
            ),
            Vldm_Single_A1(c, rn, wb, db, first, count) => {
                render_vldm("vldm", c, rn, *wb, *db, single_range(first, *count), true)
            }
            Vstm_Single_A1(c, rn, wb, db, first, count) => {
                render_vldm("vstm", c, rn, *wb, *db, single_range(first, *count), false)
            }
            Vldm_Double_A1(c, rn, wb, db, first, count) => {
                render_vldm("vldm", c, rn, *wb, *db, double_range(first, *count), true)
            }
            Vstm_Double_A1(c, rn, wb, db, first, count) => {
                render_vldm("vstm", c, rn, *wb, *db, double_range(first, *count), false)
            }

            // ---- VFP data-processing ----
            FpDataProcess3_Single_A1(c, op, sd, sn, sm) => format!(
                "{}{}.f32 {}, {}, {}",
                fp3_mnemonic(*op),
                cc(c),
                single(sd),
                single(sn),
                single(sm)
            ),
            FpDataProcess3_Double_A1(c, op, dd, dn, dm) => format!(
                "{}{}.f64 {}, {}, {}",
                fp3_mnemonic(*op),
                cc(c),
                double(dd),
                double(dn),
                double(dm)
            ),
            FpDataProcess2_Single_A1(c, op, sd, sm) => format!(
                "{}{}.f32 {}, {}",
                fp2_mnemonic(*op),
                cc(c),
                single(sd),
                single(sm)
            ),
            FpDataProcess2_Double_A1(c, op, dd, dm) => format!(
                "{}{}.f64 {}, {}",
                fp2_mnemonic(*op),
                cc(c),
                double(dd),
                double(dm)
            ),

            // ---- VFP compare / transfer / immediate ----
            Vcmp_Single_A1(c, sd, sm, e) => format!(
                "vcmp{}{}.f32 {}, {}",
                if *e { "e" } else { "" },
                cc(c),
                single(sd),
                single(sm)
            ),
            Vcmp_Double_A1(c, dd, dm, e) => format!(
                "vcmp{}{}.f64 {}, {}",
                if *e { "e" } else { "" },
                cc(c),
                double(dd),
                double(dm)
            ),
            Vcmp_Zero_Single_A1(c, sd, e) => format!(
                "vcmp{}{}.f32 {}, #0",
                if *e { "e" } else { "" },
                cc(c),
                single(sd)
            ),
            Vcmp_Zero_Double_A1(c, dd, e) => format!(
                "vcmp{}{}.f64 {}, #0",
                if *e { "e" } else { "" },
                cc(c),
                double(dd)
            ),
            Vmrs_A1(c, rt) => format!("vmrs{} {}, fpscr", cc(c), gpr(rt)),
            Vmrs_Apsr_Nzcv_A1(c) => format!("vmrs{} apsr_nzcv, fpscr", cc(c)),
            Vmsr_A1(c, rt) => format!("vmsr{} fpscr, {}", cc(c), gpr(rt)),
            Vmov_Core_To_Single_A1(c, sn, rt) => {
                format!("vmov{} {}, {}", cc(c), single(sn), gpr(rt))
            }
            Vmov_Single_To_Core_A1(c, rt, sn) => {
                format!("vmov{} {}, {}", cc(c), gpr(rt), single(sn))
            }
            Vmov_Core_To_Scalar_A1(c, size, index, dd, rt) => format!(
                "vmov{}.{} {}[{}], {}",
                cc(c),
                size.suffix(),
                double(dd),
                index,
                gpr(rt)
            ),
            Vmov_Scalar_To_Core_A1(c, unsigned, size, index, rt, dn) => {
                let dt = match size {
                    Arm32VmovLaneSize::Word => "32".to_string(),
                    s => format!("{}{}", if *unsigned { "u" } else { "s" }, s.suffix()),
                };
                format!(
                    "vmov{}.{} {}, {}[{}]",
                    cc(c),
                    dt,
                    gpr(rt),
                    double(dn),
                    index
                )
            }
            Vmov_Immediate_Single_A1(c, sd, imm8) => {
                format!("vmov{}.f32 {}, {}", cc(c), single(sd), fp_imm(*imm8))
            }
            Vmov_Immediate_Double_A1(c, dd, imm8) => {
                format!("vmov{}.f64 {}, {}", cc(c), double(dd), fp_imm(*imm8))
            }
            Vmov_Double_To_CorePair_A1(c, rt, rt2, dm) => {
                format!("vmov{} {}, {}, {}", cc(c), gpr(rt), gpr(rt2), double(dm))
            }
            Vmov_CorePair_To_Double_A1(c, dm, rt, rt2) => {
                format!("vmov{} {}, {}, {}", cc(c), double(dm), gpr(rt), gpr(rt2))
            }
            Vmov_Singles_To_CorePair_A1(c, rt, rt2, sm) => format!(
                "vmov{} {}, {}, {}, {}",
                cc(c),
                gpr(rt),
                gpr(rt2),
                single(sm),
                single(&single_next(sm))
            ),
            Vmov_CorePair_To_Singles_A1(c, sm, rt, rt2) => format!(
                "vmov{} {}, {}, {}, {}",
                cc(c),
                single(sm),
                single(&single_next(sm)),
                gpr(rt),
                gpr(rt2)
            ),

            // ---- VFP conversions ----
            Vcvt_FloatToInt_FromSingle_A1(c, sd, sm, signed, round) => format!(
                "vcvt{}{}.{}.f32 {}, {}",
                if *round { "" } else { "r" },
                cc(c),
                su32(*signed),
                single(sd),
                single(sm)
            ),
            Vcvt_FloatToInt_FromDouble_A1(c, sd, dm, signed, round) => format!(
                "vcvt{}{}.{}.f64 {}, {}",
                if *round { "" } else { "r" },
                cc(c),
                su32(*signed),
                single(sd),
                double(dm)
            ),
            Vcvt_IntToFloat_ToSingle_A1(c, sd, sm, signed) => format!(
                "vcvt{}.f32.{} {}, {}",
                cc(c),
                su32(*signed),
                single(sd),
                single(sm)
            ),
            Vcvt_IntToFloat_ToDouble_A1(c, dd, sm, signed) => format!(
                "vcvt{}.f64.{} {}, {}",
                cc(c),
                su32(*signed),
                double(dd),
                single(sm)
            ),
            Vcvt_Single_To_Double_A1(c, dd, sm) => {
                format!("vcvt{}.f64.f32 {}, {}", cc(c), double(dd), single(sm))
            }
            Vcvt_Double_To_Single_A1(c, sd, dm) => {
                format!("vcvt{}.f32.f64 {}, {}", cc(c), single(sd), double(dm))
            }
            Vcvt_HalfToSingle_A1(c, sd, sm, top) => format!(
                "vcvt{}{}.f32.f16 {}, {}",
                if *top { "t" } else { "b" },
                cc(c),
                single(sd),
                single(sm)
            ),
            Vcvt_SingleToHalf_A1(c, sd, sm, top) => format!(
                "vcvt{}{}.f16.f32 {}, {}",
                if *top { "t" } else { "b" },
                cc(c),
                single(sd),
                single(sm)
            ),
            Vcvt_HalfToDouble_A1(c, dd, sm, top) => format!(
                "vcvt{}{}.f64.f16 {}, {}",
                if *top { "t" } else { "b" },
                cc(c),
                double(dd),
                single(sm)
            ),
            Vcvt_DoubleToHalf_A1(c, sd, dm, top) => format!(
                "vcvt{}{}.f16.f64 {}, {}",
                if *top { "t" } else { "b" },
                cc(c),
                single(sd),
                double(dm)
            ),
            Vjcvt_A1(c, sd, dm) => format!("vjcvt{}.s32.f64 {}, {}", cc(c), single(sd), double(dm)),
            Vcvt_FloatToFixed_Single_A1(c, sd, signed, bits32, frac) => format!(
                "vcvt{}.{}.f32 {}, {}, #{}",
                cc(c),
                fixed_type(*signed, *bits32),
                single(sd),
                single(sd),
                frac
            ),
            Vcvt_FloatToFixed_Double_A1(c, dd, signed, bits32, frac) => format!(
                "vcvt{}.{}.f64 {}, {}, #{}",
                cc(c),
                fixed_type(*signed, *bits32),
                double(dd),
                double(dd),
                frac
            ),
            Vcvt_FixedToFloat_Single_A1(c, sd, signed, bits32, frac) => format!(
                "vcvt{}.f32.{} {}, {}, #{}",
                cc(c),
                fixed_type(*signed, *bits32),
                single(sd),
                single(sd),
                frac
            ),
            Vcvt_FixedToFloat_Double_A1(c, dd, signed, bits32, frac) => format!(
                "vcvt{}.f64.{} {}, {}, #{}",
                cc(c),
                fixed_type(*signed, *bits32),
                double(dd),
                double(dd),
                frac
            ),

            // ---- ARMv8 FP additions ----
            Vsel_Single_A1(cc_, sd, sn, sm) => format!(
                "vsel{}.f32 {}, {}, {}",
                vsel_cond(*cc_),
                single(sd),
                single(sn),
                single(sm)
            ),
            Vsel_Double_A1(cc_, dd, dn, dm) => format!(
                "vsel{}.f64 {}, {}, {}",
                vsel_cond(*cc_),
                double(dd),
                double(dn),
                double(dm)
            ),
            Vmaxnm_Single_A1(sd, sn, sm) => {
                format!("vmaxnm.f32 {}, {}, {}", single(sd), single(sn), single(sm))
            }
            Vmaxnm_Double_A1(dd, dn, dm) => {
                format!("vmaxnm.f64 {}, {}, {}", double(dd), double(dn), double(dm))
            }
            Vminnm_Single_A1(sd, sn, sm) => {
                format!("vminnm.f32 {}, {}, {}", single(sd), single(sn), single(sm))
            }
            Vminnm_Double_A1(dd, dn, dm) => {
                format!("vminnm.f64 {}, {}, {}", double(dd), double(dn), double(dm))
            }
            Vrint_Directed_Single_A1(mode, sd, sm) => format!(
                "vrint{}.f32 {}, {}",
                directed_round(*mode),
                single(sd),
                single(sm)
            ),
            Vrint_Directed_Double_A1(mode, dd, dm) => format!(
                "vrint{}.f64 {}, {}",
                directed_round(*mode),
                double(dd),
                double(dm)
            ),
            Vrint_Cond_Single_A1(c, mode, sd, sm) => format!(
                "vrint{}{}.f32 {}, {}",
                vrint_mode(*mode),
                cc(c),
                single(sd),
                single(sm)
            ),
            Vrint_Cond_Double_A1(c, mode, dd, dm) => format!(
                "vrint{}{}.f64 {}, {}",
                vrint_mode(*mode),
                cc(c),
                double(dd),
                double(dm)
            ),
            Vcvt_Directed_FromSingle_A1(mode, sd, sm, signed) => format!(
                "vcvt{}.{}.f32 {}, {}",
                directed_round(*mode),
                su32(*signed),
                single(sd),
                single(sm)
            ),
            Vcvt_Directed_FromDouble_A1(mode, sd, dm, signed) => format!(
                "vcvt{}.{}.f64 {}, {}",
                directed_round(*mode),
                su32(*signed),
                single(sd),
                double(dm)
            ),

            // ---- NEON 3-reg-same ----
            NeonInt3Same_D_A1(op, size, dd, dn, dm) => format!(
                "{} {}, {}, {}",
                neon_int_type(*op, *size),
                double(dd),
                double(dn),
                double(dm)
            ),
            NeonInt3Same_Q_A1(op, size, qd, qn, qm) => format!(
                "{} {}, {}, {}",
                neon_int_type(*op, *size),
                quad(qd),
                quad(qn),
                quad(qm)
            ),
            NeonFloat3Same_D_A1(op, dd, dn, dm) => format!(
                "{}.f32 {}, {}, {}",
                neon_float_mnemonic(*op),
                double(dd),
                double(dn),
                double(dm)
            ),
            NeonFloat3Same_Q_A1(op, qd, qn, qm) => format!(
                "{}.f32 {}, {}, {}",
                neon_float_mnemonic(*op),
                quad(qd),
                quad(qn),
                quad(qm)
            ),
            NeonBitwise3Same_D_A1(op, dd, dn, dm) => format!(
                "{} {}, {}, {}",
                neon_bitwise_mnemonic(*op),
                double(dd),
                double(dn),
                double(dm)
            ),
            NeonBitwise3Same_Q_A1(op, qd, qn, qm) => format!(
                "{} {}, {}, {}",
                neon_bitwise_mnemonic(*op),
                quad(qd),
                quad(qn),
                quad(qm)
            ),

            // ---- NEON 2-reg-misc ----
            NeonMisc2Sized_D_A1(op, size, dd, dm) => {
                neon_misc2_sized(*op, *size, &double(dd), &double(dm))
            }
            NeonMisc2Sized_Q_A1(op, size, qd, qm) => {
                neon_misc2_sized(*op, *size, &quad(qd), &quad(qm))
            }
            NeonMisc2Fixed_D_A1(op, dd, dm) => neon_misc2_fixed(*op, &double(dd), &double(dm)),
            NeonMisc2Fixed_Q_A1(op, qd, qm) => neon_misc2_fixed(*op, &quad(qd), &quad(qm)),
            NeonMisc2Narrow_A1(op, size, dd, qm) => format!(
                "{} {}, {}",
                neon_narrow_type(*op, *size),
                double(dd),
                quad(qm)
            ),
            NeonShllMax_A1(size, qd, dm) => format!(
                "vshll.i{} {}, {}, #{}",
                nbits(*size),
                quad(qd),
                double(dm),
                nbits(*size)
            ),

            // ---- NEON 3-reg-different ----
            NeonDiffLong_A1(op, size, qd, dn, dm) => format!(
                "{} {}, {}, {}",
                neon_difflong_type(*op, *size),
                quad(qd),
                double(dn),
                double(dm)
            ),
            NeonDiffWide_A1(op, size, qd, qn, dm) => format!(
                "{} {}, {}, {}",
                neon_diffwide_type(*op, *size),
                quad(qd),
                quad(qn),
                double(dm)
            ),
            NeonDiffNarrow_A1(op, size, dd, qn, qm) => format!(
                "{}.i{} {}, {}, {}",
                neon_diffnarrow_mnemonic(*op),
                nbits(*size),
                double(dd),
                quad(qn),
                quad(qm)
            ),

            // ---- NEON 2-reg-and-a-scalar ----
            NeonScalar_D_A1(op, size, dd, dn, dm, idx) => format!(
                "{} {}, {}, {}",
                neon_scalar_type(*op, *size),
                double(dd),
                double(dn),
                scalar(dm, *idx)
            ),
            NeonScalar_Q_A1(op, size, qd, qn, dm, idx) => format!(
                "{} {}, {}, {}",
                neon_scalar_type(*op, *size),
                quad(qd),
                quad(qn),
                scalar(dm, *idx)
            ),
            NeonScalarLong_A1(op, size, qd, dn, dm, idx) => format!(
                "{} {}, {}, {}",
                neon_scalarlong_type(*op, *size),
                quad(qd),
                double(dn),
                scalar(dm, *idx)
            ),

            // ---- NEON 2-reg-and-a-shift ----
            NeonShift_D_A1(op, size, shift, dd, dm) => format!(
                "{} {}, {}, #{}",
                neon_shift_type(*op, *size),
                double(dd),
                double(dm),
                shift
            ),
            NeonShift_Q_A1(op, size, shift, qd, qm) => format!(
                "{} {}, {}, #{}",
                neon_shift_type(*op, *size),
                quad(qd),
                quad(qm),
                shift
            ),
            NeonShiftNarrow_A1(op, size, shift, dd, qm) => format!(
                "{} {}, {}, #{}",
                neon_shiftnarrow_type(*op, *size),
                double(dd),
                quad(qm),
                shift
            ),
            // NB: the `signed` field is the raw U bit (U=1 is unsigned), so the type letter is inverted here.
            NeonShiftLong_A1(signed, size, shift, qd, dm) => {
                let ty = if *signed { "u" } else { "s" };
                if *shift == 0 {
                    format!("vmovl.{}{} {}, {}", ty, nbits(*size), quad(qd), double(dm))
                } else {
                    format!(
                        "vshll.{}{} {}, {}, #{}",
                        ty,
                        nbits(*size),
                        quad(qd),
                        double(dm),
                        shift
                    )
                }
            }

            // ---- NEON extract / table / duplicate / immediate ----
            NeonExt_D_A1(off, dd, dn, dm) => format!(
                "vext.8 {}, {}, {}, #{}",
                double(dd),
                double(dn),
                double(dm),
                off
            ),
            NeonExt_Q_A1(off, qd, qn, qm) => {
                format!("vext.8 {}, {}, {}, #{}", quad(qd), quad(qn), quad(qm), off)
            }
            NeonTableLookup_A1(is_vtbx, length, dd, dn, dm) => format!(
                "{} {}, {}, {}",
                if *is_vtbx { "vtbx.8" } else { "vtbl.8" },
                double(dd),
                table_list(dn, *length),
                double(dm)
            ),
            NeonVdupScalar_D_A1(size, idx, dd, dm) => {
                format!("vdup.{} {}, {}", nbits(*size), double(dd), scalar(dm, *idx))
            }
            NeonVdupScalar_Q_A1(size, idx, qd, dm) => {
                format!("vdup.{} {}, {}", nbits(*size), quad(qd), scalar(dm, *idx))
            }
            NeonVdupCore_D_A1(c, size, dd, rt) => {
                format!("vdup{}.{} {}, {}", cc(c), nbits(*size), double(dd), gpr(rt))
            }
            NeonVdupCore_Q_A1(c, size, qd, rt) => {
                format!("vdup{}.{} {}, {}", cc(c), nbits(*size), quad(qd), gpr(rt))
            }
            NeonModifiedImmediate_D_A1(cmode, op, imm8, dd) => {
                neon_modified_immediate(*cmode, *op, *imm8, &double(dd))
            }
            NeonModifiedImmediate_Q_A1(cmode, op, imm8, qd) => {
                neon_modified_immediate(*cmode, *op, *imm8, &quad(qd))
            }

            // ---- NEON element/structure load/store ----
            NeonLoadStoreMultiple_A1(is_load, type_bits, size, align, first, rn, address) => {
                neon_ldst_multiple(*is_load, *type_bits, *size, *align, first, rn, *address)
            }
            NeonLoadStoreSingleLane_A1(is_load, n, size, index_align, first, rn, address) => {
                neon_ldst_single_lane(*is_load, *n, *size, *index_align, first, rn, *address)
            }
            NeonLoadStoreAllLanes_A1(n, size, t, a, first, rn, address) => {
                neon_ldst_all_lanes(*n, *size, *t, *a, first, rn, *address)
            }

            // ---- ARMv8 crypto ----
            NeonAes_A1(op, qd, qm) => format!("{}.8 {}, {}", aes_mnemonic(*op), quad(qd), quad(qm)),
            NeonSha3Reg_A1(op, qd, qn, qm) => format!(
                "{}.32 {}, {}, {}",
                sha3_mnemonic(*op),
                quad(qd),
                quad(qn),
                quad(qm)
            ),
            NeonSha2Reg_A1(op, qd, qm) => {
                format!("{}.32 {}, {}", sha2_mnemonic(*op), quad(qd), quad(qm))
            }

            // ---- preload ----
            Pld_A1(rn, offset) => format!("pld {}", preload_mem(rn, offset)),
            Pldw_A1(rn, offset) => format!("pldw {}", preload_mem(rn, offset)),
            Pli_A1(rn, offset) => format!("pli {}", preload_mem(rn, offset)),

            // ---- exception save / return ----
            Rfe_A1(mode, rn, wb) => format!(
                "rfe{} {}{}",
                block_suffix(*mode),
                gpr(rn),
                if *wb { "!" } else { "" }
            ),
            Srs_A1(mode, wb, mode_num) => format!(
                "srs{} sp{}, #{}",
                block_suffix(*mode),
                if *wb { "!" } else { "" },
                mode_num
            ),

            // ---- branch / interwork ----
            B_A1(c, offset) => render_branch(
                &format!("b{}", cc(c)),
                instruction_address,
                *offset as i64,
                syntax,
            ),
            Bl_A1(c, offset) => render_branch(
                &format!("bl{}", cc(c)),
                instruction_address,
                *offset as i64,
                syntax,
            ),
            Blx_Immediate_A1(offset) => {
                render_branch("blx", instruction_address, *offset as i64, syntax)
            }
            Bx_A1(c, rm) => format!("bx{} {}", cc(c), gpr(rm)),
            Blx_Register_A1(c, rm) => format!("blx{} {}", cc(c), gpr(rm)),
            Bxj_A1(c, rm) => format!("bxj{} {}", cc(c), gpr(rm)),
        }
    }
}

// ================= condition / flag / register helpers =================

// A32 condition suffix: AL (the "always" code) renders as nothing.
fn cc(condition: &Arm32Condition) -> &'static str {
    match condition {
        Arm32Condition::Equal => "eq",
        Arm32Condition::NotEqual => "ne",
        Arm32Condition::CarrySet => "cs",
        Arm32Condition::CarryClear => "cc",
        Arm32Condition::MinusNegative => "mi",
        Arm32Condition::PlusPositiveOrZero => "pl",
        Arm32Condition::Overflow => "vs",
        Arm32Condition::NoOverflow => "vc",
        Arm32Condition::UnsignedHigher => "hi",
        Arm32Condition::UnsignedLowerOrSame => "ls",
        Arm32Condition::SignedGreaterThanOrEqual => "ge",
        Arm32Condition::SignedLessThan => "lt",
        Arm32Condition::SignedGreaterThan => "gt",
        Arm32Condition::SignedLessThanOrEqual => "le",
        Arm32Condition::AlwaysUnconditional => "",
        Arm32Condition::Undefined(_) => "",
    }
}

fn s_flag(set_flags: bool) -> &'static str {
    if set_flags { "s" } else { "" }
}

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
    use Arm32GeneralPurposeRegister::*;
    match register {
        R0 => "r0",
        R1 => "r1",
        R2 => "r2",
        R3 => "r3",
        R4 => "r4",
        R5 => "r5",
        R6 => "r6",
        R7 => "r7",
        R8 => "r8",
        R9 => "r9",
        R10 => "r10",
        R11 => "r11",
        R12 => "r12",
        R13 => "sp",
        R14 => "lr",
        R15 => "pc",
    }
}

fn next_reg(register: &Arm32GeneralPurposeRegister) -> Arm32GeneralPurposeRegister {
    Arm32GeneralPurposeRegister::from_operand_bits((register.as_operand_bits() + 1) & 0xF)
}

fn single(register: &Arm32SinglePrecisionRegister) -> String {
    format!("s{}", register.number())
}
fn double(register: &Arm32DoublePrecisionRegister) -> String {
    format!("d{}", register.number())
}
fn quad(register: &Arm32QuadwordRegister) -> String {
    format!("q{}", register.number())
}
fn single_next(register: &Arm32SinglePrecisionRegister) -> Arm32SinglePrecisionRegister {
    Arm32SinglePrecisionRegister::new(register.number() + 1).unwrap_or(*register)
}
fn scalar(register: &Arm32DoublePrecisionRegister, index: u8) -> String {
    format!("d{}[{}]", register.number(), index)
}

// ================= data-processing helpers =================

fn dp_imm(
    mnemonic: &str,
    c: &Arm32Condition,
    s: bool,
    rd: &Arm32GeneralPurposeRegister,
    rn: &Arm32GeneralPurposeRegister,
    imm32: u32,
    syntax: ArmAssemblySyntax,
) -> String {
    format!(
        "{}{}{} {}, {}, {}",
        mnemonic,
        s_flag(s),
        cc(c),
        gpr(rd),
        gpr(rn),
        imm(syntax, imm32 as i64)
    )
}
fn dp_reg(
    mnemonic: &str,
    c: &Arm32Condition,
    s: bool,
    rd: &Arm32GeneralPurposeRegister,
    rn: &Arm32GeneralPurposeRegister,
    rm: &Arm32GeneralPurposeRegister,
    shift: &Arm32RegisterShift,
) -> String {
    format!(
        "{}{}{} {}, {}, {}{}",
        mnemonic,
        s_flag(s),
        cc(c),
        gpr(rd),
        gpr(rn),
        gpr(rm),
        shift_suffix(shift)
    )
}
fn dp_rsr(
    mnemonic: &str,
    c: &Arm32Condition,
    s: bool,
    rd: &Arm32GeneralPurposeRegister,
    rn: &Arm32GeneralPurposeRegister,
    rm: &Arm32GeneralPurposeRegister,
    ty: Arm32ShiftType,
    rs: &Arm32GeneralPurposeRegister,
) -> String {
    format!(
        "{}{}{} {}, {}, {}, {} {}",
        mnemonic,
        s_flag(s),
        cc(c),
        gpr(rd),
        gpr(rn),
        gpr(rm),
        shift_type_mnemonic(ty),
        gpr(rs)
    )
}

// MOV (register) with an immediate shift renders as the shift mnemonic (or plain mov / rrx).
fn render_mov_shift(
    c: &Arm32Condition,
    s: bool,
    rd: &Arm32GeneralPurposeRegister,
    rm: &Arm32GeneralPurposeRegister,
    shift: &Arm32RegisterShift,
) -> String {
    match shift {
        Arm32RegisterShift::Lsl(0) => format!("mov{}{} {}, {}", s_flag(s), cc(c), gpr(rd), gpr(rm)),
        Arm32RegisterShift::Lsl(n) => {
            format!("lsl{}{} {}, {}, #{}", s_flag(s), cc(c), gpr(rd), gpr(rm), n)
        }
        Arm32RegisterShift::Lsr(n) => {
            format!("lsr{}{} {}, {}, #{}", s_flag(s), cc(c), gpr(rd), gpr(rm), n)
        }
        Arm32RegisterShift::Asr(n) => {
            format!("asr{}{} {}, {}, #{}", s_flag(s), cc(c), gpr(rd), gpr(rm), n)
        }
        Arm32RegisterShift::Ror(n) => {
            format!("ror{}{} {}, {}, #{}", s_flag(s), cc(c), gpr(rd), gpr(rm), n)
        }
        Arm32RegisterShift::Rrx => format!("rrx{}{} {}, {}", s_flag(s), cc(c), gpr(rd), gpr(rm)),
    }
}

fn shift_suffix(shift: &Arm32RegisterShift) -> String {
    match shift {
        Arm32RegisterShift::Lsl(0) => String::new(),
        Arm32RegisterShift::Lsl(n) => format!(", lsl #{}", n),
        Arm32RegisterShift::Lsr(n) => format!(", lsr #{}", n),
        Arm32RegisterShift::Asr(n) => format!(", asr #{}", n),
        Arm32RegisterShift::Ror(n) => format!(", ror #{}", n),
        Arm32RegisterShift::Rrx => ", rrx".to_string(),
    }
}

fn shift_type_mnemonic(ty: Arm32ShiftType) -> &'static str {
    match ty {
        Arm32ShiftType::Lsl => "lsl",
        Arm32ShiftType::Lsr => "lsr",
        Arm32ShiftType::Asr => "asr",
        Arm32ShiftType::Ror => "ror",
    }
}

fn tb(top: bool) -> &'static str {
    if top { "t" } else { "b" }
}
fn xx(cross: bool) -> &'static str {
    if cross { "x" } else { "" }
}
fn rr(round: bool) -> &'static str {
    if round { "r" } else { "" }
}
fn su(signed: bool) -> &'static str {
    if signed { "s" } else { "u" }
}
fn su32(signed: bool) -> &'static str {
    if signed { "s32" } else { "u32" }
}
fn fixed_type(signed: bool, bits32: bool) -> String {
    format!("{}{}", su(signed), if bits32 { "32" } else { "16" })
}
fn rotation(rot: u8) -> String {
    if rot == 0 {
        String::new()
    } else {
        format!(", ror #{}", rot)
    }
}
fn long_suffix(long: bool) -> &'static str {
    if long { "l" } else { "" }
}

fn extend_mnemonic(ty: Arm32ExtendType) -> &'static str {
    match ty {
        Arm32ExtendType::Sxtb16 => "sxtb16",
        Arm32ExtendType::Sxtb => "sxtb",
        Arm32ExtendType::Sxth => "sxth",
        Arm32ExtendType::Uxtb16 => "uxtb16",
        Arm32ExtendType::Uxtb => "uxtb",
        Arm32ExtendType::Uxth => "uxth",
    }
}
fn extend_add_mnemonic(ty: Arm32ExtendType) -> &'static str {
    match ty {
        Arm32ExtendType::Sxtb16 => "sxtab16",
        Arm32ExtendType::Sxtb => "sxtab",
        Arm32ExtendType::Sxth => "sxtah",
        Arm32ExtendType::Uxtb16 => "uxtab16",
        Arm32ExtendType::Uxtb => "uxtab",
        Arm32ExtendType::Uxth => "uxtah",
    }
}

// ================= memory addressing helpers =================

fn mem12(
    rn: &Arm32GeneralPurposeRegister,
    off: &Arm32MemoryOffset,
    idx: Arm32IndexMode,
    syntax: ArmAssemblySyntax,
) -> String {
    let operand = mem12_operand(off, syntax);
    index_addressing(rn, &operand, idx)
}
fn mem12_post(
    rn: &Arm32GeneralPurposeRegister,
    off: &Arm32MemoryOffset,
    syntax: ArmAssemblySyntax,
) -> String {
    format!("[{}], {}", gpr(rn), mem12_operand(off, syntax))
}
fn mem12_operand(off: &Arm32MemoryOffset, syntax: ArmAssemblySyntax) -> String {
    match off {
        Arm32MemoryOffset::Immediate { add, imm12 } => signed_imm(syntax, *add, *imm12 as i64),
        Arm32MemoryOffset::Register { add, rm, shift } => {
            format!("{}{}{}", sign(*add), gpr(rm), shift_suffix(shift))
        }
    }
}
fn mem8(
    rn: &Arm32GeneralPurposeRegister,
    off: &Arm32MemoryOffset8,
    idx: Arm32IndexMode,
    syntax: ArmAssemblySyntax,
) -> String {
    let operand = mem8_operand(off, syntax);
    index_addressing(rn, &operand, idx)
}
fn mem8_post(
    rn: &Arm32GeneralPurposeRegister,
    off: &Arm32MemoryOffset8,
    syntax: ArmAssemblySyntax,
) -> String {
    format!("[{}], {}", gpr(rn), mem8_operand(off, syntax))
}
fn mem8_operand(off: &Arm32MemoryOffset8, syntax: ArmAssemblySyntax) -> String {
    match off {
        Arm32MemoryOffset8::Immediate { add, imm8 } => signed_imm(syntax, *add, *imm8 as i64),
        Arm32MemoryOffset8::Register { add, rm } => format!("{}{}", sign(*add), gpr(rm)),
    }
}
// Combine a base register and an offset operand under an index mode into the full `[...]` form. An immediate
// "#0" offset prints as a bare `[Rn]` in the offset mode.
fn index_addressing(
    rn: &Arm32GeneralPurposeRegister,
    operand: &str,
    idx: Arm32IndexMode,
) -> String {
    match idx {
        Arm32IndexMode::Offset => {
            if operand == "#0" {
                format!("[{}]", gpr(rn))
            } else {
                format!("[{}, {}]", gpr(rn), operand)
            }
        }
        Arm32IndexMode::PreIndex => format!("[{}, {}]!", gpr(rn), operand),
        Arm32IndexMode::PostIndex => format!("[{}], {}", gpr(rn), operand),
    }
}
fn sign(add: bool) -> &'static str {
    if add { "" } else { "-" }
}
fn signed_imm(syntax: ArmAssemblySyntax, add: bool, magnitude: i64) -> String {
    imm(syntax, if add { magnitude } else { -magnitude })
}
fn preload_mem(rn: &Arm32GeneralPurposeRegister, off: &Arm32MemoryOffset) -> String {
    match off {
        Arm32MemoryOffset::Immediate { add, imm12 } => {
            if *imm12 == 0 {
                format!("[{}]", gpr(rn))
            } else {
                format!("[{}, #{}{}]", gpr(rn), sign(*add), imm12)
            }
        }
        Arm32MemoryOffset::Register { add, rm, shift } => format!(
            "[{}, {}{}{}]",
            gpr(rn),
            sign(*add),
            gpr(rm),
            shift_suffix(shift)
        ),
    }
}

fn fp_mem(rn: &Arm32GeneralPurposeRegister, off: i64, syntax: ArmAssemblySyntax) -> String {
    if off == 0 {
        format!("[{}]", gpr(rn))
    } else {
        format!("[{}, {}]", gpr(rn), imm(syntax, off))
    }
}

fn single_range(first: &Arm32SinglePrecisionRegister, count: u8) -> String {
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
fn double_range(first: &Arm32DoublePrecisionRegister, count: u8) -> String {
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

// VLDM/VSTM, rendered as vpush/vpop when the base is SP with writeback.
fn render_vldm(
    mnemonic: &str,
    c: &Arm32Condition,
    rn: &Arm32GeneralPurposeRegister,
    wb: bool,
    db: bool,
    list: String,
    is_load: bool,
) -> String {
    let is_sp = *rn == Arm32GeneralPurposeRegister::R13;
    if is_sp && wb && db && !is_load {
        return format!("vpush{} {}", cc(c), list);
    } // vstmdb sp!
    if is_sp && wb && !db && is_load {
        return format!("vpop{} {}", cc(c), list);
    } // vldmia sp!
    let mode = if db { "db" } else { "ia" };
    format!(
        "{}{}{} {}{}, {}",
        mnemonic,
        mode,
        cc(c),
        gpr(rn),
        if wb { "!" } else { "" },
        list
    )
}

// LDM/STM, rendered as push/pop for the sp! IA/DB stack idioms.
fn render_ldm_stm(
    mnemonic: &str,
    c: &Arm32Condition,
    mode: Arm32BlockAddressMode,
    rn: &Arm32GeneralPurposeRegister,
    wb: bool,
    user: bool,
    regs: &[Arm32GeneralPurposeRegister],
) -> String {
    let is_sp = *rn == Arm32GeneralPurposeRegister::R13;
    let list = register_list(regs);
    if !user && is_sp && wb {
        if mnemonic == "ldm" && matches!(mode, Arm32BlockAddressMode::IncrementAfter) {
            return format!("pop{} {}", cc(c), list);
        }
        if mnemonic == "stm" && matches!(mode, Arm32BlockAddressMode::DecrementBefore) {
            return format!("push{} {}", cc(c), list);
        }
    }
    format!(
        "{}{}{} {}{}, {}{}",
        mnemonic,
        block_mode_suffix(mode),
        cc(c),
        gpr(rn),
        if wb { "!" } else { "" },
        list,
        if user { "^" } else { "" }
    )
}
fn register_list(regs: &[Arm32GeneralPurposeRegister]) -> String {
    let names: Vec<&str> = regs.iter().map(gpr).collect();
    format!("{{{}}}", names.join(", "))
}
fn block_mode_suffix(mode: Arm32BlockAddressMode) -> &'static str {
    match mode {
        Arm32BlockAddressMode::IncrementAfter => "ia",
        Arm32BlockAddressMode::IncrementBefore => "ib",
        Arm32BlockAddressMode::DecrementAfter => "da",
        Arm32BlockAddressMode::DecrementBefore => "db",
    }
}
fn block_suffix(mode: Arm32BlockAddressMode) -> &'static str {
    block_mode_suffix(mode)
}

// ================= status / coproc helpers =================

fn psr(spsr: bool) -> &'static str {
    if spsr { "SPSR" } else { "CPSR" }
}
fn psr_fields(spsr: bool, mask: u8) -> String {
    let mut suffix = String::new();
    if mask & 0b0001 != 0 {
        suffix.push('c');
    }
    if mask & 0b0010 != 0 {
        suffix.push('x');
    }
    if mask & 0b0100 != 0 {
        suffix.push('s');
    }
    if mask & 0b1000 != 0 {
        suffix.push('f');
    }
    format!("{}_{}", psr(spsr), suffix)
}
// SYSm -> banked-register name. The encoding is m:m1; this covers the architecturally-named banked registers.
fn banked_reg(spsr: bool, sysm: u8) -> String {
    if spsr {
        match sysm {
            0b01110 => "SPSR_fiq".to_string(),
            0b10000 => "SPSR_irq".to_string(),
            0b10010 => "SPSR_svc".to_string(),
            0b10100 => "SPSR_abt".to_string(),
            0b10110 => "SPSR_und".to_string(),
            0b11100 => "SPSR_mon".to_string(),
            0b11110 => "SPSR_hyp".to_string(),
            other => format!("SPSR_{}", other),
        }
    } else {
        match sysm {
            0b00000 => "R8_usr".to_string(),
            0b00001 => "R9_usr".to_string(),
            0b00010 => "R10_usr".to_string(),
            0b00011 => "R11_usr".to_string(),
            0b00100 => "R12_usr".to_string(),
            0b00101 => "SP_usr".to_string(),
            0b00110 => "LR_usr".to_string(),
            0b01000 => "R8_fiq".to_string(),
            0b01001 => "R9_fiq".to_string(),
            0b01010 => "R10_fiq".to_string(),
            0b01011 => "R11_fiq".to_string(),
            0b01100 => "R12_fiq".to_string(),
            0b01101 => "SP_fiq".to_string(),
            0b01110 => "LR_fiq".to_string(),
            0b10000 => "LR_irq".to_string(),
            0b10001 => "SP_irq".to_string(),
            0b10010 => "LR_svc".to_string(),
            0b10011 => "SP_svc".to_string(),
            0b10100 => "LR_abt".to_string(),
            0b10101 => "SP_abt".to_string(),
            0b10110 => "LR_und".to_string(),
            0b10111 => "SP_und".to_string(),
            0b11100 => "LR_mon".to_string(),
            0b11101 => "SP_mon".to_string(),
            0b11110 => "ELR_hyp".to_string(),
            0b11111 => "SP_hyp".to_string(),
            other => format!("R{}_banked", other),
        }
    }
}

fn render_cps(
    mode: Arm32CpsMode,
    a: bool,
    i: bool,
    f: bool,
    new_mode: Option<u8>,
    syntax: ArmAssemblySyntax,
) -> String {
    let flags: String = [(a, 'a'), (i, 'i'), (f, 'f')]
        .iter()
        .filter(|(set, _)| *set)
        .map(|(_, ch)| *ch)
        .collect();
    let base = match mode {
        Arm32CpsMode::NoChange => "cps".to_string(),
        Arm32CpsMode::Enable => format!("cpsie {}", flags),
        Arm32CpsMode::Disable => format!("cpsid {}", flags),
    };
    match new_mode {
        Some(m) => match mode {
            Arm32CpsMode::NoChange => format!("cps {}", imm(syntax, m as i64)),
            _ => format!("{}, {}", base, imm(syntax, m as i64)),
        },
        None => base,
    }
}

#[allow(clippy::too_many_arguments)]
fn render_mcr(
    mnemonic: &str,
    cond: &str,
    cp: u8,
    opc1: u8,
    rt: &str,
    crn: u8,
    crm: u8,
    opc2: u8,
) -> String {
    format!(
        "{}{} p{}, {}, {}, c{}, c{}, {}",
        mnemonic, cond, cp, opc1, rt, crn, crm, opc2
    )
}
#[allow(clippy::too_many_arguments)]
fn render_ldc(
    mnemonic: &str,
    cp: u8,
    crd: u8,
    rn: &Arm32GeneralPurposeRegister,
    add: bool,
    imm8: u8,
    idx: Arm32IndexMode,
    syntax: ArmAssemblySyntax,
) -> String {
    let offset = signed_imm(syntax, add, (imm8 as i64) * 4);
    let address = match idx {
        Arm32IndexMode::Offset => {
            if imm8 == 0 {
                format!("[{}]", gpr(rn))
            } else {
                format!("[{}, {}]", gpr(rn), offset)
            }
        }
        Arm32IndexMode::PreIndex => format!("[{}, {}]!", gpr(rn), offset),
        Arm32IndexMode::PostIndex => format!("[{}], {}", gpr(rn), offset),
    };
    format!("{} p{}, c{}, {}", mnemonic, cp, crd, address)
}

fn barrier(option: u8) -> String {
    match option {
        0xF => "sy".to_string(),
        0xE => "st".to_string(),
        0xB => "ish".to_string(),
        0xA => "ishst".to_string(),
        0x7 => "nsh".to_string(),
        0x6 => "nshst".to_string(),
        0x3 => "osh".to_string(),
        0x2 => "oshst".to_string(),
        other => format!("#{}", other),
    }
}

// ================= VFP data-processing op names =================

fn fp3_mnemonic(op: Arm32FpDataOperation3) -> &'static str {
    use Arm32FpDataOperation3::*;
    match op {
        Vmla => "vmla",
        Vmls => "vmls",
        Vnmla => "vnmla",
        Vnmls => "vnmls",
        Vmul => "vmul",
        Vnmul => "vnmul",
        Vadd => "vadd",
        Vsub => "vsub",
        Vdiv => "vdiv",
        Vfma => "vfma",
        Vfms => "vfms",
        Vfnma => "vfnma",
        Vfnms => "vfnms",
    }
}
fn fp2_mnemonic(op: Arm32FpDataOperation2) -> &'static str {
    use Arm32FpDataOperation2::*;
    match op {
        Vmov => "vmov",
        Vabs => "vabs",
        Vneg => "vneg",
        Vsqrt => "vsqrt",
    }
}

// VFP modified immediate -> a printed floating-point literal that GNU re-encodes to the same imm8.
fn fp_imm(imm8: u8) -> String {
    let value = vfp_expand_imm8_to_f64(imm8);
    format_float(value)
}
// VMOV.f32/f64 immediates round-trip through GNU as plain decimals (e.g. #1.0, #0.5, #-2.5).
fn format_float(value: f64) -> String {
    // `f64::trunc` is std-only (libm), so under `no_std` test integrality via an `i64` round-trip instead.
    // The guard `value.abs() < 1e15` keeps us well inside i64's range, where `value as i64` saturates
    // deterministically and `(value as i64) as f64 == value` is exactly `value == value.trunc()`; for any
    // value outside that range the `&&` already selects the else branch, so behavior is byte-identical.
    if value == (value as i64) as f64 && value.abs() < 1e15 {
        format!("#{:.1}", value)
    } else {
        let text = format!("{}", value);
        format!("#{}", text)
    }
}

fn vsel_cond(cc_: Arm32VselCondition) -> &'static str {
    match cc_ {
        Arm32VselCondition::Equal => "eq",
        Arm32VselCondition::Overflow => "vs",
        Arm32VselCondition::GreaterEqual => "ge",
        Arm32VselCondition::GreaterThan => "gt",
    }
}
fn directed_round(mode: Arm32DirectedRound) -> &'static str {
    match mode {
        Arm32DirectedRound::A => "a",
        Arm32DirectedRound::N => "n",
        Arm32DirectedRound::P => "p",
        Arm32DirectedRound::M => "m",
    }
}
fn vrint_mode(mode: Arm32VrintMode) -> &'static str {
    match mode {
        Arm32VrintMode::R => "r",
        Arm32VrintMode::Z => "z",
        Arm32VrintMode::X => "x",
    }
}

// ================= NEON op-name helpers =================

fn nbits(size: Arm32NeonSize) -> u8 {
    match size {
        Arm32NeonSize::I8 => 8,
        Arm32NeonSize::I16 => 16,
        Arm32NeonSize::I32 => 32,
        Arm32NeonSize::I64 => 64,
    }
}

// integer 3-reg-same: "<mnemonic>.<type><size>"; the type letter is empty for VTST.
fn neon_int_type(op: Arm32NeonIntegerOp, size: Arm32NeonSize) -> String {
    use Arm32NeonIntegerOp::*;
    let (mnemonic, ty) = match op {
        Vadd => ("vadd", "i"),
        Vsub => ("vsub", "i"),
        Vtst => ("vtst", ""),
        Vceq => ("vceq", "i"),
        Vmla => ("vmla", "i"),
        Vmls => ("vmls", "i"),
        Vmul => ("vmul", "i"),
        VmulPoly => ("vmul", "p"),
        VqaddS => ("vqadd", "s"),
        VqaddU => ("vqadd", "u"),
        VhaddS => ("vhadd", "s"),
        VhaddU => ("vhadd", "u"),
        VqsubS => ("vqsub", "s"),
        VqsubU => ("vqsub", "u"),
        VhsubS => ("vhsub", "s"),
        VhsubU => ("vhsub", "u"),
        VrhaddS => ("vrhadd", "s"),
        VrhaddU => ("vrhadd", "u"),
        VabdS => ("vabd", "s"),
        VabdU => ("vabd", "u"),
        VabaS => ("vaba", "s"),
        VabaU => ("vaba", "u"),
        VmaxS => ("vmax", "s"),
        VmaxU => ("vmax", "u"),
        VminS => ("vmin", "s"),
        VminU => ("vmin", "u"),
        VcgeS => ("vcge", "s"),
        VcgeU => ("vcge", "u"),
        VcgtS => ("vcgt", "s"),
        VcgtU => ("vcgt", "u"),
        Vpadd => ("vpadd", "i"),
        VpmaxS => ("vpmax", "s"),
        VpmaxU => ("vpmax", "u"),
        VpminS => ("vpmin", "s"),
        VpminU => ("vpmin", "u"),
        VqdmulhS => ("vqdmulh", "s"),
        VqrdmulhS => ("vqrdmulh", "s"),
    };
    format!("{}.{}{}", mnemonic, ty, nbits(size))
}
fn neon_float_mnemonic(op: Arm32NeonFloatOp) -> &'static str {
    use Arm32NeonFloatOp::*;
    match op {
        Vadd => "vadd",
        Vsub => "vsub",
        Vmul => "vmul",
        Vmla => "vmla",
        Vmls => "vmls",
        Vabd => "vabd",
        Vpadd => "vpadd",
        Vceq => "vceq",
        Vcge => "vcge",
        Vcgt => "vcgt",
        Vmax => "vmax",
        Vmin => "vmin",
        Vpmax => "vpmax",
        Vpmin => "vpmin",
        Vrecps => "vrecps",
        Vrsqrts => "vrsqrts",
        Vfma => "vfma",
        Vfms => "vfms",
    }
}
fn neon_bitwise_mnemonic(op: Arm32NeonBitwiseOp) -> &'static str {
    use Arm32NeonBitwiseOp::*;
    match op {
        Vand => "vand",
        Vbic => "vbic",
        Vorr => "vorr",
        Vorn => "vorn",
        Veor => "veor",
        Vbsl => "vbsl",
        Vbit => "vbit",
        Vbif => "vbif",
    }
}

fn neon_misc2_sized(op: Arm32NeonMisc2SizedOp, size: Arm32NeonSize, dd: &str, dm: &str) -> String {
    use Arm32NeonMisc2SizedOp::*;
    let bits = nbits(size);
    match op {
        Vrev64 => format!("vrev64.{} {}, {}", bits, dd, dm),
        Vrev32 => format!("vrev32.{} {}, {}", bits, dd, dm),
        Vrev16 => format!("vrev16.{} {}, {}", bits, dd, dm),
        VpaddlS => format!("vpaddl.s{} {}, {}", bits, dd, dm),
        VpaddlU => format!("vpaddl.u{} {}, {}", bits, dd, dm),
        VclsS => format!("vcls.s{} {}, {}", bits, dd, dm),
        VclzI => format!("vclz.i{} {}, {}", bits, dd, dm),
        VpadalS => format!("vpadal.s{} {}, {}", bits, dd, dm),
        VpadalU => format!("vpadal.u{} {}, {}", bits, dd, dm),
        VqabsS => format!("vqabs.s{} {}, {}", bits, dd, dm),
        VqnegS => format!("vqneg.s{} {}, {}", bits, dd, dm),
        VcgtZeroS => format!("vcgt.s{} {}, {}, #0", bits, dd, dm),
        VcgeZeroS => format!("vcge.s{} {}, {}, #0", bits, dd, dm),
        VceqZeroI => format!("vceq.i{} {}, {}, #0", bits, dd, dm),
        VcleZeroS => format!("vcle.s{} {}, {}, #0", bits, dd, dm),
        VcltZeroS => format!("vclt.s{} {}, {}, #0", bits, dd, dm),
        VabsS => format!("vabs.s{} {}, {}", bits, dd, dm),
        VnegS => format!("vneg.s{} {}, {}", bits, dd, dm),
        Vtrn => format!("vtrn.{} {}, {}", bits, dd, dm),
        Vuzp => format!("vuzp.{} {}, {}", bits, dd, dm),
        Vzip => format!("vzip.{} {}, {}", bits, dd, dm),
    }
}
fn neon_misc2_fixed(op: Arm32NeonMisc2FixedOp, dd: &str, dm: &str) -> String {
    use Arm32NeonMisc2FixedOp::*;
    match op {
        Vmvn => format!("vmvn {}, {}", dd, dm),
        Vswp => format!("vswp {}, {}", dd, dm),
        Vcnt => format!("vcnt.8 {}, {}", dd, dm),
        VcgtZeroF => format!("vcgt.f32 {}, {}, #0", dd, dm),
        VcgeZeroF => format!("vcge.f32 {}, {}, #0", dd, dm),
        VceqZeroF => format!("vceq.f32 {}, {}, #0", dd, dm),
        VcleZeroF => format!("vcle.f32 {}, {}, #0", dd, dm),
        VcltZeroF => format!("vclt.f32 {}, {}, #0", dd, dm),
        VabsF => format!("vabs.f32 {}, {}", dd, dm),
        VnegF => format!("vneg.f32 {}, {}", dd, dm),
        VrintN => format!("vrintn.f32 {}, {}", dd, dm),
        VrintX => format!("vrintx.f32 {}, {}", dd, dm),
        VrintA => format!("vrinta.f32 {}, {}", dd, dm),
        VrintZ => format!("vrintz.f32 {}, {}", dd, dm),
        VrintM => format!("vrintm.f32 {}, {}", dd, dm),
        VrintP => format!("vrintp.f32 {}, {}", dd, dm),
        VrecpeU => format!("vrecpe.u32 {}, {}", dd, dm),
        VrsqrteU => format!("vrsqrte.u32 {}, {}", dd, dm),
        VrecpeF => format!("vrecpe.f32 {}, {}", dd, dm),
        VrsqrteF => format!("vrsqrte.f32 {}, {}", dd, dm),
        VcvtF32FromS32 => format!("vcvt.f32.s32 {}, {}", dd, dm),
        VcvtF32FromU32 => format!("vcvt.f32.u32 {}, {}", dd, dm),
        VcvtS32FromF32 => format!("vcvt.s32.f32 {}, {}", dd, dm),
        VcvtU32FromF32 => format!("vcvt.u32.f32 {}, {}", dd, dm),
        VcvtaS => format!("vcvta.s32.f32 {}, {}", dd, dm),
        VcvtaU => format!("vcvta.u32.f32 {}, {}", dd, dm),
        VcvtnS => format!("vcvtn.s32.f32 {}, {}", dd, dm),
        VcvtnU => format!("vcvtn.u32.f32 {}, {}", dd, dm),
        VcvtpS => format!("vcvtp.s32.f32 {}, {}", dd, dm),
        VcvtpU => format!("vcvtp.u32.f32 {}, {}", dd, dm),
        VcvtmS => format!("vcvtm.s32.f32 {}, {}", dd, dm),
        VcvtmU => format!("vcvtm.u32.f32 {}, {}", dd, dm),
    }
}
fn neon_narrow_type(op: Arm32NeonNarrowOp, size: Arm32NeonSize) -> String {
    use Arm32NeonNarrowOp::*;
    let bits = nbits(size);
    match op {
        Vmovn => format!("vmovn.i{}", bits),
        Vqmovun => format!("vqmovun.s{}", bits),
        VqmovnS => format!("vqmovn.s{}", bits),
        VqmovnU => format!("vqmovn.u{}", bits),
    }
}

fn neon_difflong_type(op: Arm32NeonDiffLongOp, size: Arm32NeonSize) -> String {
    use Arm32NeonDiffLongOp::*;
    let bits = nbits(size);
    let (mnemonic, ty) = match op {
        VaddlS => ("vaddl", "s"),
        VaddlU => ("vaddl", "u"),
        VsublS => ("vsubl", "s"),
        VsublU => ("vsubl", "u"),
        VabalS => ("vabal", "s"),
        VabalU => ("vabal", "u"),
        VabdlS => ("vabdl", "s"),
        VabdlU => ("vabdl", "u"),
        VmlalS => ("vmlal", "s"),
        VmlalU => ("vmlal", "u"),
        VmlslS => ("vmlsl", "s"),
        VmlslU => ("vmlsl", "u"),
        VmullS => ("vmull", "s"),
        VmullU => ("vmull", "u"),
        VmullP => ("vmull", "p"),
        Vqdmlal => ("vqdmlal", "s"),
        Vqdmlsl => ("vqdmlsl", "s"),
        Vqdmull => ("vqdmull", "s"),
    };
    // VMULL.p64 is the I32-size encoding of VMULL.p.
    if matches!(op, VmullP) && matches!(size, Arm32NeonSize::I32) {
        return "vmull.p64".to_string();
    }
    format!("{}.{}{}", mnemonic, ty, bits)
}
fn neon_diffwide_type(op: Arm32NeonDiffWideOp, size: Arm32NeonSize) -> String {
    use Arm32NeonDiffWideOp::*;
    let (mnemonic, ty) = match op {
        VaddwS => ("vaddw", "s"),
        VaddwU => ("vaddw", "u"),
        VsubwS => ("vsubw", "s"),
        VsubwU => ("vsubw", "u"),
    };
    format!("{}.{}{}", mnemonic, ty, nbits(size))
}
fn neon_diffnarrow_mnemonic(op: Arm32NeonDiffNarrowOp) -> &'static str {
    use Arm32NeonDiffNarrowOp::*;
    match op {
        Vaddhn => "vaddhn",
        Vraddhn => "vraddhn",
        Vsubhn => "vsubhn",
        Vrsubhn => "vrsubhn",
    }
}

fn neon_scalar_type(op: Arm32NeonScalarOp, size: Arm32NeonSize) -> String {
    use Arm32NeonScalarOp::*;
    let bits = nbits(size);
    match op {
        Vmla => format!("vmla.i{}", bits),
        VmlaF => "vmla.f32".to_string(),
        Vmls => format!("vmls.i{}", bits),
        VmlsF => "vmls.f32".to_string(),
        Vmul => format!("vmul.i{}", bits),
        VmulF => "vmul.f32".to_string(),
        Vqdmulh => format!("vqdmulh.s{}", bits),
        Vqrdmulh => format!("vqrdmulh.s{}", bits),
    }
}
fn neon_scalarlong_type(op: Arm32NeonScalarLongOp, size: Arm32NeonSize) -> String {
    use Arm32NeonScalarLongOp::*;
    let bits = nbits(size);
    let (mnemonic, ty) = match op {
        VmlalS => ("vmlal", "s"),
        VmlalU => ("vmlal", "u"),
        VmlslS => ("vmlsl", "s"),
        VmlslU => ("vmlsl", "u"),
        VmullS => ("vmull", "s"),
        VmullU => ("vmull", "u"),
        Vqdmlal => ("vqdmlal", "s"),
        Vqdmlsl => ("vqdmlsl", "s"),
        Vqdmull => ("vqdmull", "s"),
    };
    format!("{}.{}{}", mnemonic, ty, bits)
}

fn neon_shift_type(op: Arm32NeonShiftOp, size: Arm32NeonSize) -> String {
    use Arm32NeonShiftOp::*;
    let bits = nbits(size);
    let (mnemonic, ty) = match op {
        VshrS => ("vshr", "s"),
        VshrU => ("vshr", "u"),
        VsraS => ("vsra", "s"),
        VsraU => ("vsra", "u"),
        VrshrS => ("vrshr", "s"),
        VrshrU => ("vrshr", "u"),
        VrsraS => ("vrsra", "s"),
        VrsraU => ("vrsra", "u"),
        Vsri => ("vsri", ""),
        Vshl => ("vshl", "i"),
        Vsli => ("vsli", ""),
        Vqshlu => ("vqshlu", "s"),
        VqshlS => ("vqshl", "s"),
        VqshlU => ("vqshl", "u"),
    };
    format!("{}.{}{}", mnemonic, ty, bits)
}
fn neon_shiftnarrow_type(op: Arm32NeonShiftNarrowOp, size: Arm32NeonSize) -> String {
    use Arm32NeonShiftNarrowOp::*;
    let bits = nbits(size);
    match op {
        Vshrn => format!("vshrn.i{}", bits),
        Vrshrn => format!("vrshrn.i{}", bits),
        Vqshrun => format!("vqshrun.s{}", bits),
        Vqrshrun => format!("vqrshrun.s{}", bits),
        VqshrnS => format!("vqshrn.s{}", bits),
        VqrshrnS => format!("vqrshrn.s{}", bits),
        VqshrnU => format!("vqshrn.u{}", bits),
        VqrshrnU => format!("vqrshrn.u{}", bits),
    }
}

fn table_list(first: &Arm32DoublePrecisionRegister, length: u8) -> String {
    if length <= 1 {
        format!("{{d{}}}", first.number())
    } else {
        format!(
            "{{d{}-d{}}}",
            first.number(),
            first.number() as u16 + length as u16 - 1
        )
    }
}

// VMOV/VMVN/VORR/VBIC modified immediate -> the rendered (mnemonic, size, value) that re-encodes the same.
fn neon_modified_immediate(cmode: u8, op: bool, imm8: u8, vd: &str) -> String {
    let imm = imm8 as u64;
    // (mnemonic, ".type", value) per the AdvSIMDExpandImm cmode/op table.
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
        (0b111, 0, true) => ("vmov", ".i64", expand_imm64(imm8)),
        (0b111, 1, false) => ("vmov", ".f32", 0), // handled below
        _ => ("vmov", ".i32", imm),
    };
    if cmode == 0b1111 && !op {
        return format!("vmov.f32 {}, {}", vd, fp_imm(imm8));
    }
    if suffix == ".i64" {
        return format!("vmov.i64 {}, #0x{:016x}", vd, value);
    }
    format!("{}{} {}, #0x{:x}", mnemonic, suffix, vd, value)
}
// cmode 1110 op=1: each bit of imm8 expands to a 0x00/0xFF byte of the 64-bit value.
fn expand_imm64(imm8: u8) -> u64 {
    let mut value: u64 = 0;
    for bit in 0..8 {
        if imm8 & (1 << bit) != 0 {
            value |= 0xFFu64 << (bit * 8);
        }
    }
    value
}

// ================= NEON load/store =================

// The register list for a multiple-element transfer, derived from the `type` field.
fn neon_multiple_list(type_bits: u8, first: u8) -> (String, &'static str) {
    // (count, structure-mnemonic) for each type code
    let (count, mnemonic): (u8, &str) = match type_bits {
        0b0111 => (1, "vld1"),
        0b1010 => (2, "vld1"),
        0b0110 => (3, "vld1"),
        0b0010 => (4, "vld1"),
        0b1000 => (2, "vld2"),
        0b1001 => (2, "vld2"),
        0b0011 => (4, "vld2"),
        0b0100 => (3, "vld3"),
        0b0101 => (3, "vld3"),
        0b0000 => (4, "vld4"),
        0b0001 => (4, "vld4"),
        _ => (1, "vld1"),
    };
    let stride = if matches!(type_bits, 0b1001 | 0b0101 | 0b0001) {
        2
    } else {
        1
    };
    let list = register_d_list(first, count, stride);
    (list, mnemonic)
}
fn register_d_list(first: u8, count: u8, stride: u8) -> String {
    // u16 arithmetic so a degenerate/hostile decode (large count/stride/first) renders without overflow.
    let regs: Vec<String> = (0..count)
        .map(|i| format!("d{}", first as u16 + i as u16 * stride as u16))
        .collect();
    if stride == 1 && count > 1 {
        format!("{{d{}-d{}}}", first, first as u16 + count as u16 - 1)
    } else {
        format!("{{{}}}", regs.join(", "))
    }
}
fn neon_ls_mnemonic(is_load: bool, base: &str) -> String {
    if is_load {
        base.to_string()
    } else {
        format!("vst{}", &base[3..])
    }
}
fn neon_address(
    rn: &Arm32GeneralPurposeRegister,
    align: Option<u32>,
    address: Arm32NeonLoadStoreAddress,
) -> String {
    let align_text = match align {
        Some(bits) if bits != 0 => format!(":{}", 32 << bits),
        _ => String::new(),
    };
    match address {
        Arm32NeonLoadStoreAddress::Offset => format!("[{}{}]", gpr(rn), align_text),
        Arm32NeonLoadStoreAddress::IncrementWriteback => format!("[{}{}]!", gpr(rn), align_text),
        Arm32NeonLoadStoreAddress::PostIndexed(rm) => {
            format!("[{}{}], {}", gpr(rn), align_text, gpr(&rm))
        }
    }
}

fn neon_ldst_multiple(
    is_load: bool,
    type_bits: u8,
    size: Arm32NeonSize,
    align: u8,
    first: &Arm32DoublePrecisionRegister,
    rn: &Arm32GeneralPurposeRegister,
    address: Arm32NeonLoadStoreAddress,
) -> String {
    let (list, base) = neon_multiple_list(type_bits, first.number());
    let mnemonic = neon_ls_mnemonic(is_load, base);
    format!(
        "{}.{} {}, {}",
        mnemonic,
        nbits(size),
        list,
        neon_address(rn, Some(align as u32), address)
    )
}
fn neon_ldst_single_lane(
    is_load: bool,
    n: u8,
    size: u8,
    index_align: u8,
    first: &Arm32DoublePrecisionRegister,
    rn: &Arm32GeneralPurposeRegister,
    address: Arm32NeonLoadStoreAddress,
) -> String {
    let element_bits = 8u8 << size;
    let (index, stride) = single_lane_index_stride(size, index_align);
    let list = lane_list(first.number(), n, stride, index);
    let mnemonic = neon_ls_mnemonic(is_load, &format!("vld{}", n));
    format!(
        "{}.{} {}, {}",
        mnemonic,
        element_bits,
        list,
        neon_address(rn, None, address)
    )
}
fn single_lane_index_stride(size: u8, index_align: u8) -> (u8, u8) {
    match size {
        0 => ((index_align >> 1) & 0b111, 1),
        1 => (
            (index_align >> 2) & 0b11,
            if index_align & 0b10 != 0 { 2 } else { 1 },
        ),
        _ => (
            (index_align >> 3) & 0b1,
            if index_align & 0b100 != 0 { 2 } else { 1 },
        ),
    }
}
fn lane_list(first: u8, n: u8, stride: u8, index: u8) -> String {
    let regs: Vec<String> = (0..n)
        .map(|i| format!("d{}[{}]", first + i * stride, index))
        .collect();
    format!("{{{}}}", regs.join(", "))
}
fn neon_ldst_all_lanes(
    n: u8,
    size: u8,
    t: bool,
    _a: bool,
    first: &Arm32DoublePrecisionRegister,
    rn: &Arm32GeneralPurposeRegister,
    address: Arm32NeonLoadStoreAddress,
) -> String {
    let element_bits = 8u8 << size;
    let stride = if t { 2 } else { 1 };
    let regs: Vec<String> = (0..n)
        .map(|i| format!("d{}[]", first.number() + i * stride))
        .collect();
    format!(
        "vld{}.{} {{{}}}, {}",
        n,
        element_bits,
        regs.join(", "),
        neon_address(rn, None, address)
    )
}

// ================= crypto =================

fn aes_mnemonic(op: Arm32NeonAesOp) -> &'static str {
    use Arm32NeonAesOp::*;
    match op {
        Aese => "aese",
        Aesd => "aesd",
        Aesmc => "aesmc",
        Aesimc => "aesimc",
    }
}
fn sha3_mnemonic(op: Arm32NeonSha3Op) -> &'static str {
    use Arm32NeonSha3Op::*;
    match op {
        Sha1c => "sha1c",
        Sha1p => "sha1p",
        Sha1m => "sha1m",
        Sha1su0 => "sha1su0",
        Sha256h => "sha256h",
        Sha256h2 => "sha256h2",
        Sha256su1 => "sha256su1",
    }
}
fn sha2_mnemonic(op: Arm32NeonSha2Op) -> &'static str {
    use Arm32NeonSha2Op::*;
    match op {
        Sha1h => "sha1h",
        Sha1su1 => "sha1su1",
        Sha256su0 => "sha256su0",
    }
}

// ================= PC-relative branch =================

// In ARM state the value read from PC is the instruction's address + 8.
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
            (address as i64 + 8 + offset) as u32
        ),
        None => format!("{} {}", mnemonic, imm(syntax, offset)),
    }
}
