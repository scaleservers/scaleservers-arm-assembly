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
