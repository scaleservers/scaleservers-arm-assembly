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
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::ArmT32Instruction;
use crate::emit::ArmAssemblySyntax;
use crate::enums::{
    Arm32GeneralPurposeRegister,
    Arm32LowGeneralPurposeRegister,
    Arm32SinglePrecisionRegister,
    Arm32DoublePrecisionRegister,
    Arm32VmovLaneSize,
    Arm32DirectedRound,
    Arm32MveVectorRegister,
    ArmT32CpsPrimaskEffect,
    ArmT32IndexMode,
    ArmT32InstructionCondition,
    ArmT32MemoryBarrierOption,
    ArmT32RegisterShift,
    ArmT32SpecialRegister,
};

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
// Both the disassembler (arm32dasm) and the differential oracle render IT members through this function,
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
    let base = if wide.is_empty() { strip_it_flag_setting_suffix(base) } else { base };
    format!("{}{}{}{}", base, condition.ual_suffix(), wide, rest)
}

// Append a VPT-block then/else letter (`t`/`e`) to a rendered MVE instruction, inserted into the mnemonic
// just before its `.<type>` suffix (e.g. `vadd.i32 q0, q1, q2` + 't' -> `vaddt.i32 q0, q1, q2`). The
// predicated and plain forms share the same encoding, so this is purely a disassembly nicety.
pub fn apply_vpt_block_suffix(rendered: &str, letter: char) -> String {
    let mnemonic_end = rendered.find(char::is_whitespace).unwrap_or(rendered.len());
    let (mnemonic, rest) = rendered.split_at(mnemonic_end);
    let insert_at = mnemonic.find('.').unwrap_or(mnemonic.len());
    format!("{}{}{}{}", &mnemonic[..insert_at], letter, &mnemonic[insert_at..], rest)
}

// Map a narrow flag-setting data-processing mnemonic to its non-flag-setting spelling for use inside an
// IT block ("movs" -> "mov"). Anything not in this exact set is returned unchanged, so a non-DP mnemonic
// that merely ends in `s` is never mangled.
fn strip_it_flag_setting_suffix(base: &str) -> &str {
    match base {
        "movs" => "mov", "mvns" => "mvn",
        "adds" => "add", "subs" => "sub",
        "adcs" => "adc", "sbcs" => "sbc",
        "rsbs" => "rsb", "muls" => "mul",
        "ands" => "and", "bics" => "bic",
        "orrs" => "orr", "eors" => "eor",
        "lsls" => "lsl", "lsrs" => "lsr",
        "asrs" => "asr", "rors" => "ror",
        other => other,
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
        (0b110, _, false) => ("vmov", ".i32", if cmode & 1 == 0 { (imm << 8) | 0xFF } else { (imm << 16) | 0xFFFF }),
        (0b110, _, true) => ("vmvn", ".i32", if cmode & 1 == 0 { (imm << 8) | 0xFF } else { (imm << 16) | 0xFFFF }),
        (0b111, 0, false) => ("vmov", ".i8", imm),
        (0b111, 0, true) => ("vmov", ".i64", mve_expand_imm64(imm8)),
        (0b111, 1, false) => ("vmov", ".f32", 0), // handled below
        _ => ("vmov", ".i32", imm),
    };
    if cmode == 0b1111 && !op {
        return format!("vmov.f32 {}, #{:?}", qd, crate::vfp_expand_imm8_to_f32(imm8) as f64);
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
fn render_indexed(rn: &Arm32GeneralPurposeRegister, offset: i64, mode: ArmT32IndexMode, syntax: ArmAssemblySyntax) -> String {
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
    if rotation == 0 { String::new() } else { format!(", ror #{}", rotation) }
}

// floating-point register names and the FP `[Rn{, #+/-off}]` addressing operand.
fn single(register: &Arm32SinglePrecisionRegister) -> String {
    format!("s{}", register.number())
}
fn double(register: &Arm32DoublePrecisionRegister) -> String {
    format!("d{}", register.number())
}
fn render_fp_mem(rn: &Arm32GeneralPurposeRegister, offset: i64, syntax: ArmAssemblySyntax) -> String {
    if offset == 0 {
        format!("[{}]", gpr(rn))
    } else {
        format!("[{}, {}]", gpr(rn), imm(syntax, offset))
    }
}
// a contiguous FP register range `{s0-s3}` (or `{s0}` for a single register)
fn fp_single_range(first: &Arm32SinglePrecisionRegister, count: u8) -> String {
    if count <= 1 { format!("{{s{}}}", first.number()) }
    else { format!("{{s{}-s{}}}", first.number(), first.number() as u16 + count as u16 - 1) }
}
fn fp_double_range(first: &Arm32DoublePrecisionRegister, count: u8) -> String {
    if count <= 1 { format!("{{d{}}}", first.number()) }
    else { format!("{{d{}-d{}}}", first.number(), first.number() as u16 + count as u16 - 1) }
}

// signed-multiply mnemonic suffix letters: t/b (top/bottom half), x (exchange), r (rounded).
fn top_or_bottom(top: bool) -> &'static str { if top { "t" } else { "b" } }
fn exchange(cross: bool) -> &'static str { if cross { "x" } else { "" } }
fn rounded(round: bool) -> &'static str { if round { "r" } else { "" } }
fn int_type(signed: bool) -> &'static str { if signed { "s32" } else { "u32" } }
fn fixed_type(signed: bool, bits32: bool) -> String { format!("{}{}", if signed { "s" } else { "u" }, if bits32 { "32" } else { "16" }) }

// `[Rn, Rm]` or `[Rn, Rm, lsl #n]` (the LSL amount is always decimal).
fn render_register_offset(rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, lsl: u8) -> String {
    if lsl == 0 {
        format!("[{}, {}]", gpr(rn), gpr(rm))
    } else {
        format!("[{}, {}, lsl #{}]", gpr(rn), gpr(rm), lsl)
    }
}

// MOV (register) and the shift register-by-immediate mnemonics share one encoding; render the canonical
// UAL: no shift -> `mov{s}.w`; LSL/LSR/ASR/ROR -> that mnemonic with a count; RRX -> `rrx{s}` (no `.w`).
fn render_mov_register(rd: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, shift: &ArmT32RegisterShift, set_flags: bool) -> String {
    match shift {
        ArmT32RegisterShift::Lsl(0) => format!("mov{} {}, {}", wide_flag(set_flags), gpr(rd), gpr(rm)),
        ArmT32RegisterShift::Lsl(amount) => format!("lsl{} {}, {}, #{}", wide_flag(set_flags), gpr(rd), gpr(rm), amount),
        ArmT32RegisterShift::Lsr(amount) => format!("lsr{} {}, {}, #{}", wide_flag(set_flags), gpr(rd), gpr(rm), amount),
        ArmT32RegisterShift::Asr(amount) => format!("asr{} {}, {}, #{}", wide_flag(set_flags), gpr(rd), gpr(rm), amount),
        ArmT32RegisterShift::Ror(amount) => format!("ror{} {}, {}, #{}", wide_flag(set_flags), gpr(rd), gpr(rm), amount),
        ArmT32RegisterShift::Rrx => format!("rrx{} {}, {}", flag_suffix(set_flags), gpr(rd), gpr(rm)),
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
            if value < 0 { format!("#-0x{:x}", -value) } else { format!("#0x{:x}", value) }
        },
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

fn render_branch(mnemonic: &str, instruction_address: Option<u32>, offset: i64, syntax: ArmAssemblySyntax) -> String {
    match instruction_address {
        Some(address) => format!("{} 0x{:08x}", mnemonic, pc_relative_target(address, offset, false)),
        None => format!("{} {}", mnemonic, imm(syntax, offset)),
    }
}

// The b_label operand of a Branch Future: the absolute address (PC + 2*boff) when known, otherwise the raw
// `boff` field as a `#<boff>` immediate, matching the assembler's numeric syntax.
fn bf_blabel(instruction_address: Option<u32>, boff: u8, syntax: ArmAssemblySyntax) -> String {
    match instruction_address {
        Some(address) => format!("0x{:08x}", pc_relative_target(address, 2 * boff as i64, false)),
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
    match kind { 0 => format!("s{}", num), 1 => format!("d{}", num), _ => format!("q{}", num) }
}

// CBZ / CBNZ: like render_branch but with the test register before the target.
fn render_compare_branch(mnemonic: &str, rn: &str, instruction_address: Option<u32>, offset: i64, syntax: ArmAssemblySyntax) -> String {
    match instruction_address {
        Some(address) => format!("{} {}, 0x{:08x}", mnemonic, rn, pc_relative_target(address, offset, false)),
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

fn render_literal_load(mnemonic: &str, rt: &str, instruction_address: Option<u32>, offset: i64, syntax: ArmAssemblySyntax) -> String {
    match instruction_address {
        Some(address) => format!("{} {}, [pc, {}]  ; 0x{:08x}", mnemonic, rt, imm(syntax, offset), pc_relative_target(address, offset, true)),
        None => format!("{} {}, [pc, {}]", mnemonic, rt, imm(syntax, offset)),
    }
}

#[cfg(test)]
mod it_block_condition_tests {
    use super::apply_it_block_condition;
    use crate::enums::ArmT32InstructionCondition as Cond;

    #[test]
    fn inserts_condition_after_mnemonic_and_before_w() {
        assert_eq!(apply_it_block_condition("mov r0, r1", Cond::Equal), "moveq r0, r1");
        assert_eq!(apply_it_block_condition("add.w r0, r1, r2", Cond::NotEqual), "addne.w r0, r1, r2");
    }

    #[test]
    fn drops_flag_setting_s_on_narrow_dp_inside_it() {
        // The 16-bit DP encodings do not set flags inside an IT block, so UAL drops the `s`. Verified
        // against LLVM and GNU objdump: movs/adds/ands -> movle/addeq/andne.
        assert_eq!(apply_it_block_condition("movs r2, #0", Cond::SignedLessThanOrEqual), "movle r2, #0");
        assert_eq!(apply_it_block_condition("adds r0, r1, r2", Cond::Equal), "addeq r0, r1, r2");
        assert_eq!(apply_it_block_condition("ands r3, r4", Cond::NotEqual), "andne r3, r4");
    }

    #[test]
    fn keeps_s_on_wide_form_and_non_dp_mnemonics() {
        // A 32-bit flag-setting form has an explicit S bit and keeps its `s` inside an IT block.
        assert_eq!(apply_it_block_condition("adds.w r0, r1, #1", Cond::Equal), "addseq.w r0, r1, #1");
        // A mnemonic that ends in `s` but is not a narrow flag-setting DP op is left untouched.
        assert_eq!(apply_it_block_condition("vmrs apsr_nzcv, fpscr", Cond::Equal), "vmrseq apsr_nzcv, fpscr");
    }
}
