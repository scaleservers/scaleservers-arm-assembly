// Copyright (c) Scaleservers LLC

// Differential oracle for the scalar T32 (Thumb) set against EXTERNAL assemblers -- both LLVM (`clang`) and
// GNU (`arm-none-eabi-as`). For a representative cross-section of ARMv6-M through ARMv7E-M (the DSP and
// scalar-FP extensions included), for EACH discovered assembler:
//
//   ENCODER check:  our UAL --(assembler)--> bytes        must equal our own `encode()`.
//   DECODER check:  those bytes --(our decode)--> model   must equal the original sample instruction
//                   (and therefore re-encode to the same bytes).
//
// Together these validate the encoder, the emitter, AND the decoder against a trusted third party -- and
// do so twice, once per toolchain.
//
// Toolchain discovery (each independently optional):
//   * LLVM:  `clang` + `llvm-objcopy`            (env overrides: CLANG / LLVM_OBJCOPY)
//   * GNU:   `arm-none-eabi-as` + `-objcopy`     (env overrides: ARM_NONE_EABI_AS / ARM_NONE_EABI_OBJCOPY;
//                                                 also auto-scans the standard "Arm GNU Toolchain" install)
// If NONE is found the test SKIPS green, so CI without an ARM toolchain stays green.

use std::path::PathBuf;
use std::process::Command;
mod common;

use scaleservers_arm32_assembly::ArmAssemblySyntax;
use scaleservers_arm32_assembly::ArmT32FpDataOperation2 as Fp2;
use scaleservers_arm32_assembly::ArmT32FpDataOperation3 as Fp3;
use scaleservers_arm32_assembly::ArmT32IndexMode as Mode;
use scaleservers_arm32_assembly::ArmT32Instruction;
use scaleservers_arm32_assembly::ArmT32InstructionCondition;
use scaleservers_arm32_assembly::ArmT32ParallelOperation as Pop;
use scaleservers_arm32_assembly::ArmT32ParallelPrefix as Ppre;
use scaleservers_arm32_assembly::ArmT32RegisterShift as Shift;
use scaleservers_arm32_assembly::apply_it_block_condition;
use scaleservers_arm32_assembly::{Arm32DoublePrecisionRegister, Arm32SinglePrecisionRegister};

fn s(number: u8) -> Arm32SinglePrecisionRegister {
    Arm32SinglePrecisionRegister::new(number).unwrap()
}
fn d(number: u8) -> Arm32DoublePrecisionRegister {
    Arm32DoublePrecisionRegister::new(number).unwrap()
}
use scaleservers_arm32_assembly::{
    Arm32GeneralPurposeRegister as R, Arm32LowGeneralPurposeRegister as L,
    ArmT32MemoryBarrierOption, ArmT32SpecialRegister,
};

// Miri can neither spawn processes nor touch the filesystem, and these oracle tests shell out to gas /
// llvm-mc. `cargo +nightly miri test --lib` already excludes this `tests/` integration binary; the
// `#[cfg_attr(miri, ignore)]` is defensive and a no-op under normal `cargo test` (`cfg(miri)` is false there),
// so the differential oracle keeps running exactly as before.
#[test]
#[cfg_attr(miri, ignore)]
fn external_assemblers_match_our_encoder_and_decoder() {
    let backends = discover_backends();
    if backends.is_empty() {
        common::skip_or_require(
            "differential_oracle: no external ARM assembler found (clang or arm-none-eabi-as)",
        );
        return;
    }

    eprintln!(
        "differential_oracle: exercising backends {:?}",
        backends
            .iter()
            .map(|backend| backend.name)
            .collect::<Vec<_>>()
    );

    let samples = sample_instructions();

    // one .s file (GNU/decimal flavor, accepted by both assemblers), assembled fresh per backend
    let mut source = String::from(".syntax unified\n.thumb\n.text\n");
    let mut expected_bytes = Vec::<u8>::new();
    for instruction in &samples {
        source.push_str(&instruction.to_assembly_string(ArmAssemblySyntax::Gnu));
        source.push('\n');
        expected_bytes.extend_from_slice(&instruction.encode().expect("sample must encode"));
    }

    for backend in &backends {
        let actual_bytes = backend.assemble_and_extract(&source);

        // ---- ENCODER oracle: external bytes must equal our encode(), instruction by instruction ----
        let mut offset = 0usize;
        for instruction in &samples {
            let our = instruction.encode().unwrap();
            let theirs = actual_bytes.get(offset..offset + our.len());
            if theirs != Some(our.as_slice()) {
                panic!(
                    "[{}] ENCODER disagreement at byte offset {}:\n  instruction: {:?}\n  our UAL:     {}\n  our bytes:   {:02x?}\n  their bytes: {:02x?}",
                    backend.name,
                    offset,
                    instruction,
                    instruction.to_assembly_string(ArmAssemblySyntax::Gnu),
                    our,
                    theirs
                );
            }
            offset += our.len();
        }
        assert_eq!(
            offset,
            actual_bytes.len(),
            "[{}] external byte stream is longer than our encoding",
            backend.name
        );

        // ---- DECODER oracle: our decoder must turn the external bytes back into the exact samples ----
        let mut iterator = actual_bytes.iter();
        let mut consumed = 0usize;
        let mut index = 0usize;
        while let Some(decoded) =
            ArmT32Instruction::decode(&mut iterator, &mut consumed).expect("decode external bytes")
        {
            assert!(
                index < samples.len(),
                "[{}] decoder produced more instructions than samples",
                backend.name
            );
            assert_eq!(
                decoded, samples[index],
                "[{}] decoder produced the wrong instruction at index {}",
                backend.name, index
            );
            index += 1;
        }
        assert_eq!(
            index,
            samples.len(),
            "[{}] decoder produced {} instructions, expected {}",
            backend.name,
            index,
            samples.len()
        );
    }
}

// IT blocks need their own source: a member rendered unconditionally (e.g. "movs r2, #0") cannot be
// concatenated into the flat sample stream, because following an IT it would be a mismatched-condition
// member that the external assemblers reject. So we drive these from in-IT UAL (verified by probing the
// real toolchains) and validate BOTH directions against each backend.
#[test]
#[cfg_attr(miri, ignore)]
fn external_assemblers_match_it_blocks() {
    let backends = discover_backends();
    if backends.is_empty() {
        common::skip_or_require("differential_oracle (IT blocks): no external ARM assembler found");
        return;
    }

    // Coverage: IT mask shapes 0b1100 / 0b0100 / 0b1110; firstcond EQ / LE / GT / CC; an ITE else-branch;
    // and members that must drop the flag-setting `s` inside the block (movs/adds/subs/ands/orrs/lsls/...).
    let programs = [
        "itt le\nmovle r2, #0\naddle r0, r1, r2",
        "ite eq\nmoveq r0, #5\nandne r3, r4",
        "itt gt\nsubgt r3, r4, r5\norrgt r6, r7",
        "ittt cc\nlslcc r0, r1, #2\neorcc r2, r3\nmvncc r4, r5",
    ];

    for backend in &backends {
        for program in programs {
            let external_bytes = backend
                .assemble_and_extract(&format!(".syntax unified\n.thumb\n.text\n{}\n", program));
            let decoded = decode_all(&external_bytes);

            // (1) ENCODER + IT mask logic: re-encoding our decode of the toolchain's bytes must reproduce
            //     them exactly -- validates our IT firstcond/mask encoding and every member encoding.
            let mut our_bytes = Vec::<u8>::new();
            for instruction in &decoded {
                our_bytes.extend_from_slice(
                    &instruction
                        .encode()
                        .expect("decoded IT-block instruction must re-encode"),
                );
            }
            assert_eq!(
                our_bytes, external_bytes,
                "[{}] re-encoding our decode of `{}` did not reproduce the toolchain bytes:\n  decoded: {:?}",
                backend.name, program, decoded
            );

            // (2) DISASSEMBLER round-trip: our IT-aware listing must re-assemble, through the same backend,
            //     to the identical bytes -- proving the listing is faithful and free of non-standard
            //     spellings like `movsle` (which the external assemblers reject outright).
            let listing = render_it_block_listing(&decoded);
            let reassembled = backend
                .assemble_and_extract(&format!(".syntax unified\n.thumb\n.text\n{}\n", listing));
            assert_eq!(
                reassembled, external_bytes,
                "[{}] our disassembly of `{}` did not re-assemble to the same bytes\n  our listing:\n{}",
                backend.name, program, listing
            );
        }
    }
}

// Decode an entire byte buffer into its instruction stream (same driver shape as the main oracle).
fn decode_all(bytes: &[u8]) -> Vec<ArmT32Instruction> {
    let mut iterator = bytes.iter();
    let mut consumed = 0usize;
    let mut instructions = Vec::new();
    while let Some(instruction) =
        ArmT32Instruction::decode(&mut iterator, &mut consumed).expect("decode external bytes")
    {
        instructions.push(instruction);
    }
    instructions
}

// Re-render a decoded stream as an IT-aware UAL listing, exactly as the disassembler does: an IT
// instruction arms a queue of per-member conditions, and each following member is rendered through the
// library's `apply_it_block_condition`.
fn render_it_block_listing(instructions: &[ArmT32Instruction]) -> String {
    let mut lines = Vec::new();
    let mut pending: Vec<ArmT32InstructionCondition> = Vec::new();
    for instruction in instructions {
        let text = instruction.to_assembly_string(ArmAssemblySyntax::Gnu);
        if pending.is_empty() {
            lines.push(text);
            if let Some(members) = instruction.it_block_member_conditions() {
                pending = members;
            }
        } else {
            lines.push(apply_it_block_condition(&text, pending.remove(0)));
        }
    }
    lines.join("\n")
}

// A representative cross-section (excluding PC-relative forms, which need symbolic labels to assemble).
fn sample_instructions() -> Vec<ArmT32Instruction> {
    vec![
        // ---- ARMv6-M ----
        ArmT32Instruction::Adc_Register_T1(L::R0, L::R1),
        ArmT32Instruction::And_Register_T1(L::R2, L::R3),
        ArmT32Instruction::Orr_Register_T1(L::R0, L::R7),
        ArmT32Instruction::Mvn_Register_T1(L::R3, L::R4),
        ArmT32Instruction::Mul_T1(L::R0, L::R1),
        ArmT32Instruction::Add_Immediate_T1(L::R0, L::R1, 3),
        ArmT32Instruction::Add_Immediate_T2(L::R2, 200),
        ArmT32Instruction::Add_Register_T2(R::R0, R::R8),
        ArmT32Instruction::Add_SpPlusImmediate_T1(L::R0, 340),
        ArmT32Instruction::Add_SpPlusImmediate_T2(508),
        ArmT32Instruction::Sub_Immediate_T1(L::R0, L::R1, 5),
        ArmT32Instruction::Sub_SpMinusImmediate_T1(16),
        ArmT32Instruction::Cmp_Immediate_T1(L::R0, 85),
        ArmT32Instruction::Cmp_Register_T2(R::R10, R::R11),
        ArmT32Instruction::Asr_Immediate_T1(L::R0, L::R1, 5),
        ArmT32Instruction::Lsl_Immediate_T1(L::R0, L::R1, 4),
        ArmT32Instruction::Mov_Immediate_T1(L::R0, 200),
        ArmT32Instruction::Mov_Register_T1(R::R8, R::R0),
        ArmT32Instruction::Ldr_Immediate_T1(L::R0, L::R1, 4),
        ArmT32Instruction::Ldr_Immediate_T2(L::R0, 16),
        ArmT32Instruction::Ldr_Register_T1(L::R0, L::R1, L::R2),
        ArmT32Instruction::Ldrb_Immediate_T1(L::R0, L::R1, 3),
        ArmT32Instruction::Ldrh_Immediate_T1(L::R0, L::R1, 4),
        ArmT32Instruction::Str_Immediate_T1(L::R0, L::R1, 8),
        ArmT32Instruction::Strb_Register_T1(L::R0, L::R1, L::R2),
        ArmT32Instruction::Ldm_T1(L::R0, vec![L::R1, L::R2]),
        ArmT32Instruction::Stm_T1(L::R0, vec![L::R1, L::R2]),
        ArmT32Instruction::Push_T1(vec![R::R4, R::R5, R::R14]),
        ArmT32Instruction::Pop_T1(vec![R::R0, R::R1, R::R15]),
        ArmT32Instruction::Rev_T1(L::R0, L::R1),
        ArmT32Instruction::Sxtb_T1(L::R0, L::R1),
        ArmT32Instruction::Uxth_T1(L::R6, L::R7),
        ArmT32Instruction::Bx_T1(R::R14),
        ArmT32Instruction::Mrs_T1(R::R0, ArmT32SpecialRegister::Primask),
        ArmT32Instruction::Msr_Register_T1(ArmT32SpecialRegister::Control, R::R1),
        ArmT32Instruction::Dmb_T1(ArmT32MemoryBarrierOption::System),
        ArmT32Instruction::Isb_T1(ArmT32MemoryBarrierOption::System),
        ArmT32Instruction::Nop_T1,
        ArmT32Instruction::Wfi_T1,
        // ---- ARMv7-M (Thumb-2) batch ----
        ArmT32Instruction::Mov_Immediate_T3(R::R0, 0x1234),
        ArmT32Instruction::Movt_T1(R::R1, 0xABCD),
        ArmT32Instruction::Mul_T2(R::R3, R::R4, R::R5),
        ArmT32Instruction::Mla_T1(R::R0, R::R1, R::R2, R::R3),
        ArmT32Instruction::Mls_T1(R::R4, R::R5, R::R6, R::R7),
        ArmT32Instruction::Sdiv_T1(R::R8, R::R9, R::R10),
        ArmT32Instruction::Udiv_T1(R::R0, R::R1, R::R2),
        ArmT32Instruction::Clz_T1(R::R11, R::R12),
        // ---- ARMv7-M batch M7b: bitfield + RBIT + wide load/store ----
        ArmT32Instruction::Ubfx_T1(R::R0, R::R1, 4, 8),
        ArmT32Instruction::Sbfx_T1(R::R2, R::R3, 5, 7),
        ArmT32Instruction::Bfi_T1(R::R4, R::R5, 2, 6),
        ArmT32Instruction::Bfc_T1(R::R6, 3, 9),
        ArmT32Instruction::Rbit_T1(R::R7, R::R8),
        ArmT32Instruction::Ldr_Immediate_T3(R::R0, R::R1, 100),
        ArmT32Instruction::Str_Immediate_T3(R::R2, R::R3, 200),
        // ---- ARMv7-M batch M7c: synchronization + table branch ----
        ArmT32Instruction::Ldrex_T1(R::R0, R::R1, 16),
        ArmT32Instruction::Strex_T1(R::R2, R::R3, R::R4, 8),
        ArmT32Instruction::Ldrexb_T1(R::R0, R::R1),
        ArmT32Instruction::Strexb_T1(R::R2, R::R3, R::R4),
        ArmT32Instruction::Ldrexh_T1(R::R0, R::R1),
        ArmT32Instruction::Strexh_T1(R::R2, R::R3, R::R4),
        ArmT32Instruction::Clrex_T1,
        ArmT32Instruction::Tbb_T1(R::R0, R::R1),
        ArmT32Instruction::Tbh_T1(R::R0, R::R1),
        // ---- ARMv7-M batch M7d: data processing (modified immediate / ThumbExpandImm) ----
        ArmT32Instruction::Mov_Immediate_T2(R::R0, 0x00AB00AB, false),
        ArmT32Instruction::Mov_Immediate_T2(R::R1, 0xFF00FF00, true),
        ArmT32Instruction::Mov_Immediate_T2(R::R2, 0xAB000000, false),
        ArmT32Instruction::Mvn_Immediate_T1(R::R3, 0xFF, false),
        ArmT32Instruction::And_Immediate_T1(R::R0, R::R1, 0xABABABAB, false),
        ArmT32Instruction::Bic_Immediate_T1(R::R6, R::R7, 0x00FF00FF, true),
        ArmT32Instruction::Orr_Immediate_T1(R::R2, R::R3, 0xFF, false),
        ArmT32Instruction::Eor_Immediate_T1(R::R4, R::R5, 0xFF, true),
        ArmT32Instruction::Add_Immediate_T3(R::R0, R::R1, 0x100000, false),
        ArmT32Instruction::Sub_Immediate_T3(R::R4, R::R5, 0xFF000000, false),
        ArmT32Instruction::Tst_Immediate_T1(R::R2, 0x80000000),
        ArmT32Instruction::Teq_Immediate_T1(R::R3, 0xFF000000),
        ArmT32Instruction::Cmn_Immediate_T1(R::R1, 0x00010001),
        ArmT32Instruction::Cmp_Immediate_T2(R::R0, 0x1000),
        // ---- ARMv7-M batch M7e: rest of the modified-immediate family (ADC/SBC/RSB/ORN) ----
        ArmT32Instruction::Adc_Immediate_T1(R::R0, R::R1, 0xFF, false),
        ArmT32Instruction::Adc_Immediate_T1(R::R2, R::R3, 0x100, true),
        ArmT32Instruction::Sbc_Immediate_T1(R::R4, R::R5, 0xFF000000, false),
        ArmT32Instruction::Rsb_Immediate_T2(R::R6, R::R7, 0xAB00AB00, false),
        ArmT32Instruction::Orn_Immediate_T1(R::R2, R::R3, 0xFF00FF00, false),
        ArmT32Instruction::Orn_Immediate_T1(R::R4, R::R5, 0xFF, true),
        // ---- ARMv7-M batch M7f: data processing (shifted register) ----
        ArmT32Instruction::Add_Register_T3(R::R0, R::R1, R::R2, Shift::Lsl(0), false),
        ArmT32Instruction::Add_Register_T3(R::R0, R::R1, R::R2, Shift::Lsl(3), false),
        ArmT32Instruction::Sub_Register_T2(R::R3, R::R4, R::R5, Shift::Lsr(2), true),
        ArmT32Instruction::And_Register_T2(R::R6, R::R7, R::R8, Shift::Asr(1), false),
        ArmT32Instruction::Orr_Register_T2(R::R0, R::R1, R::R2, Shift::Ror(4), false),
        ArmT32Instruction::Eor_Register_T2(R::R3, R::R4, R::R5, Shift::Lsl(0), true),
        ArmT32Instruction::Bic_Register_T2(R::R6, R::R7, R::R8, Shift::Lsl(5), false),
        // ---- ARMv7-M batch M7g: shifted-register alias forms ----
        ArmT32Instruction::Mov_Register_T3(R::R0, R::R1, Shift::Lsl(0), false),
        ArmT32Instruction::Mov_Register_T3(R::R2, R::R3, Shift::Lsl(3), false),
        ArmT32Instruction::Mov_Register_T3(R::R4, R::R5, Shift::Lsr(4), false),
        ArmT32Instruction::Mov_Register_T3(R::R6, R::R7, Shift::Asr(5), false),
        ArmT32Instruction::Mov_Register_T3(R::R8, R::R9, Shift::Ror(8), false),
        ArmT32Instruction::Mov_Register_T3(R::R10, R::R11, Shift::Rrx, true),
        ArmT32Instruction::Mvn_Register_T2(R::R0, R::R1, Shift::Lsl(2), true),
        ArmT32Instruction::Adc_Register_T2(R::R0, R::R1, R::R2, Shift::Lsl(3), false),
        ArmT32Instruction::Sbc_Register_T2(R::R3, R::R4, R::R5, Shift::Asr(2), false),
        ArmT32Instruction::Rsb_Register_T1(R::R6, R::R7, R::R8, Shift::Lsr(1), false),
        ArmT32Instruction::Orn_Register_T1(R::R0, R::R1, R::R2, Shift::Ror(4), false),
        ArmT32Instruction::Tst_Register_T2(R::R0, R::R1, Shift::Lsl(3)),
        ArmT32Instruction::Teq_Register_T1(R::R2, R::R3, Shift::Lsl(0)),
        ArmT32Instruction::Cmn_Register_T2(R::R4, R::R5, Shift::Asr(1)),
        ArmT32Instruction::Cmp_Register_T3(R::R6, R::R7, Shift::Lsl(2)),
        // ---- ARMv7-M batch M7h: wide byte/half load/store + register offset ----
        ArmT32Instruction::Ldrb_Immediate_T2(R::R0, R::R1, 100),
        ArmT32Instruction::Strb_Immediate_T2(R::R2, R::R3, 200),
        ArmT32Instruction::Ldrh_Immediate_T2(R::R4, R::R5, 8),
        ArmT32Instruction::Strh_Immediate_T2(R::R6, R::R7, 16),
        ArmT32Instruction::Ldrsb_Immediate_T1(R::R0, R::R1, 4),
        ArmT32Instruction::Ldrsh_Immediate_T1(R::R2, R::R3, 12),
        ArmT32Instruction::Ldr_Register_T2(R::R0, R::R1, R::R2, 0),
        ArmT32Instruction::Ldr_Register_T2(R::R0, R::R1, R::R2, 2),
        ArmT32Instruction::Str_Register_T2(R::R3, R::R4, R::R5, 1),
        ArmT32Instruction::Ldrb_Register_T2(R::R6, R::R7, R::R8, 0),
        ArmT32Instruction::Strb_Register_T2(R::R0, R::R1, R::R2, 3),
        ArmT32Instruction::Ldrh_Register_T2(R::R3, R::R4, R::R5, 2),
        ArmT32Instruction::Strh_Register_T2(R::R6, R::R7, R::R8, 1),
        ArmT32Instruction::Ldrsb_Register_T2(R::R0, R::R1, R::R2, 0),
        ArmT32Instruction::Ldrsh_Register_T2(R::R3, R::R4, R::R5, 3),
        // ---- ARMv7-M batch M7k: long multiply ----
        ArmT32Instruction::Smull_T1(R::R0, R::R1, R::R2, R::R3),
        ArmT32Instruction::Umull_T1(R::R4, R::R5, R::R6, R::R7),
        ArmT32Instruction::Smlal_T1(R::R0, R::R1, R::R2, R::R3),
        ArmT32Instruction::Umlal_T1(R::R4, R::R5, R::R6, R::R7),
        ArmT32Instruction::Umaal_T1(R::R0, R::R1, R::R2, R::R3),
        ArmT32Instruction::Umaal_T1(R::R8, R::R9, R::R10, R::R11),
        // NOTE: ARMv8-M load-acquire/store-release (LDA/STL/LDAEX/STLEX) is NOT in this oracle -- it assembles
        // under -mcpu=cortex-m7 (ARMv7E-M), which predates those. They are covered by exact-byte + round-trip
        // lib tests (bytes from `arm-none-eabi-as -march=armv8.1-m.main`) instead.
        // Unprivileged load/store (LDRT/STRT family) -- ARMv7-M (size 0=B, 1=H, 2=W)
        ArmT32Instruction::UnprivLoadStore_T1(true, false, 2, R::R0, R::R1, 4),
        ArmT32Instruction::UnprivLoadStore_T1(true, false, 0, R::R2, R::R3, 8),
        ArmT32Instruction::UnprivLoadStore_T1(true, false, 1, R::R4, R::R5, 16),
        ArmT32Instruction::UnprivLoadStore_T1(true, true, 0, R::R6, R::R7, 32),
        ArmT32Instruction::UnprivLoadStore_T1(true, true, 1, R::R8, R::R9, 0),
        ArmT32Instruction::UnprivLoadStore_T1(false, false, 2, R::R0, R::R1, 4),
        ArmT32Instruction::UnprivLoadStore_T1(false, false, 0, R::R2, R::R3, 255),
        ArmT32Instruction::UnprivLoadStore_T1(false, false, 1, R::R4, R::R5, 100),
        // ---- ARMv7-M batch M7l: wide extend (with ROR), wide byte-reverse, saturate ----
        ArmT32Instruction::Sxtb_T2(R::R0, R::R1, 0),
        ArmT32Instruction::Sxtb_T2(R::R0, R::R1, 8),
        ArmT32Instruction::Uxtb_T2(R::R2, R::R3, 16),
        ArmT32Instruction::Sxth_T2(R::R4, R::R5, 24),
        ArmT32Instruction::Uxth_T2(R::R6, R::R7, 0),
        ArmT32Instruction::Rev_T2(R::R0, R::R1),
        ArmT32Instruction::Rev16_T2(R::R2, R::R3),
        ArmT32Instruction::Revsh_T2(R::R4, R::R5),
        ArmT32Instruction::Ssat_T1(R::R0, 5, R::R1, Shift::Lsl(0)),
        ArmT32Instruction::Usat_T1(R::R2, 7, R::R3, Shift::Lsl(0)),
        ArmT32Instruction::Ssat_T1(R::R0, 5, R::R1, Shift::Lsl(2)),
        ArmT32Instruction::Ssat_T1(R::R4, 10, R::R5, Shift::Asr(3)),
        // ---- ARMv7-M batch M7i: indexed load/store, LDRD/STRD, literal loads, preload ----
        ArmT32Instruction::Ldr_Immediate_T4(R::R0, R::R1, -4, Mode::Offset),
        ArmT32Instruction::Ldr_Immediate_T4(R::R0, R::R1, 4, Mode::PreIndex),
        ArmT32Instruction::Ldr_Immediate_T4(R::R0, R::R1, 4, Mode::PostIndex),
        ArmT32Instruction::Str_Immediate_T4(R::R2, R::R3, -8, Mode::Offset),
        ArmT32Instruction::Ldrb_Immediate_T3(R::R0, R::R1, 4, Mode::PreIndex),
        ArmT32Instruction::Ldrh_Immediate_T3(R::R0, R::R1, -2, Mode::Offset),
        ArmT32Instruction::Ldrsb_Immediate_T2(R::R0, R::R1, -1, Mode::PreIndex),
        ArmT32Instruction::Ldrsh_Immediate_T2(R::R0, R::R1, 2, Mode::PostIndex),
        ArmT32Instruction::Ldrd_Immediate_T1(R::R0, R::R1, R::R2, 8, Mode::Offset),
        ArmT32Instruction::Ldrd_Immediate_T1(R::R0, R::R1, R::R2, -8, Mode::PreIndex),
        ArmT32Instruction::Ldrd_Immediate_T1(R::R0, R::R1, R::R2, 16, Mode::PostIndex),
        ArmT32Instruction::Strd_Immediate_T1(R::R4, R::R5, R::R6, 16, Mode::Offset),
        ArmT32Instruction::Ldr_Literal_T2(R::R0, 100),
        ArmT32Instruction::Ldr_Literal_T2(R::R0, -100),
        ArmT32Instruction::Ldrb_Literal_T1(R::R1, 8),
        ArmT32Instruction::Ldrh_Literal_T1(R::R2, 16),
        ArmT32Instruction::Ldrsb_Literal_T1(R::R3, 4),
        ArmT32Instruction::Pld_Immediate_T1(R::R0, 4),
        ArmT32Instruction::Pld_Immediate_T1(R::R0, -4),
        ArmT32Instruction::Pld_Immediate_T1(R::R1, 255),
        ArmT32Instruction::Pli_Immediate_T1(R::R0, 4),
        // ---- ARMv7-M batch M7j: wide load/store multiple (register lists ascending) ----
        ArmT32Instruction::Ldmia_T2(R::R0, false, vec![R::R1, R::R2, R::R3]),
        ArmT32Instruction::Ldmia_T2(R::R0, true, vec![R::R1, R::R2, R::R3]),
        ArmT32Instruction::Stmia_T2(R::R0, false, vec![R::R4, R::R5, R::R6]),
        ArmT32Instruction::Stmia_T2(R::R0, true, vec![R::R4, R::R5, R::R6]),
        ArmT32Instruction::Ldmdb_T1(R::R0, false, vec![R::R1, R::R2, R::R3]),
        ArmT32Instruction::Ldmdb_T1(R::R0, true, vec![R::R1, R::R2, R::R3]),
        ArmT32Instruction::Stmdb_T1(R::R0, false, vec![R::R4, R::R5, R::R6]),
        ArmT32Instruction::Stmdb_T1(R::R0, true, vec![R::R4, R::R5, R::R6]),
        ArmT32Instruction::Stmdb_T1(R::R13, true, vec![R::R4, R::R5, R::R14]),
        ArmT32Instruction::Ldmia_T2(R::R13, true, vec![R::R4, R::R5, R::R15]),
        ArmT32Instruction::Ldmia_T2(R::R3, false, vec![R::R4, R::R12, R::R14]),
        // ---- ARMv7E-M DSP batch M8a: saturating arithmetic ----
        ArmT32Instruction::Qadd_T1(R::R0, R::R1, R::R2),
        ArmT32Instruction::Qsub_T1(R::R3, R::R4, R::R5),
        ArmT32Instruction::Qdadd_T1(R::R6, R::R7, R::R8),
        ArmT32Instruction::Qdsub_T1(R::R9, R::R10, R::R11),
        // ---- ARMv7E-M DSP batch M8b: extend-and-add + 16-bit extends ----
        ArmT32Instruction::Sxtab_T1(R::R0, R::R1, R::R2, 0),
        ArmT32Instruction::Sxtab_T1(R::R0, R::R1, R::R2, 8),
        ArmT32Instruction::Uxtab_T1(R::R3, R::R4, R::R5, 16),
        ArmT32Instruction::Sxtah_T1(R::R6, R::R7, R::R8, 24),
        ArmT32Instruction::Uxtah_T1(R::R0, R::R1, R::R2, 0),
        ArmT32Instruction::Sxtab16_T1(R::R3, R::R4, R::R5, 0),
        ArmT32Instruction::Uxtab16_T1(R::R6, R::R7, R::R8, 8),
        ArmT32Instruction::Sxtb16_T1(R::R0, R::R1, 0),
        ArmT32Instruction::Uxtb16_T1(R::R2, R::R3, 16),
        // ---- ARMv7E-M DSP batch M8c: pack / saturate16 / select / SAD ----
        ArmT32Instruction::Pkhbt_T1(R::R0, R::R1, R::R2, 0),
        ArmT32Instruction::Pkhbt_T1(R::R0, R::R1, R::R2, 4),
        ArmT32Instruction::Pkhtb_T1(R::R3, R::R4, R::R5, 8),
        ArmT32Instruction::Ssat16_T1(R::R0, 5, R::R1),
        ArmT32Instruction::Usat16_T1(R::R2, 7, R::R3),
        ArmT32Instruction::Sel_T1(R::R0, R::R1, R::R2),
        ArmT32Instruction::Usad8_T1(R::R3, R::R4, R::R5),
        ArmT32Instruction::Usada8_T1(R::R6, R::R7, R::R8, R::R9),
        // ---- ARMv7E-M DSP batch M8d: parallel add/subtract (all 6 prefixes via ADD16, all 6 ops via S) ----
        ArmT32Instruction::ParallelAddSub_T1(Pop::Add16, Ppre::Signed, R::R0, R::R1, R::R2),
        ArmT32Instruction::ParallelAddSub_T1(
            Pop::Add16,
            Ppre::SignedSaturating,
            R::R0,
            R::R1,
            R::R2,
        ),
        ArmT32Instruction::ParallelAddSub_T1(Pop::Add16, Ppre::SignedHalving, R::R0, R::R1, R::R2),
        ArmT32Instruction::ParallelAddSub_T1(Pop::Add16, Ppre::Unsigned, R::R0, R::R1, R::R2),
        ArmT32Instruction::ParallelAddSub_T1(
            Pop::Add16,
            Ppre::UnsignedSaturating,
            R::R0,
            R::R1,
            R::R2,
        ),
        ArmT32Instruction::ParallelAddSub_T1(
            Pop::Add16,
            Ppre::UnsignedHalving,
            R::R0,
            R::R1,
            R::R2,
        ),
        ArmT32Instruction::ParallelAddSub_T1(Pop::Asx, Ppre::Signed, R::R3, R::R4, R::R5),
        ArmT32Instruction::ParallelAddSub_T1(Pop::Sax, Ppre::Signed, R::R3, R::R4, R::R5),
        ArmT32Instruction::ParallelAddSub_T1(Pop::Sub16, Ppre::Signed, R::R3, R::R4, R::R5),
        ArmT32Instruction::ParallelAddSub_T1(Pop::Add8, Ppre::Signed, R::R3, R::R4, R::R5),
        ArmT32Instruction::ParallelAddSub_T1(Pop::Sub8, Ppre::Signed, R::R3, R::R4, R::R5),
        ArmT32Instruction::ParallelAddSub_T1(
            Pop::Sub8,
            Ppre::UnsignedSaturating,
            R::R6,
            R::R7,
            R::R8,
        ),
        // ---- ARMv7E-M DSP batch M8e: signed multiplies ----
        ArmT32Instruction::Smul_T1(R::R0, R::R1, R::R2, false, false),
        ArmT32Instruction::Smul_T1(R::R0, R::R1, R::R2, true, true),
        ArmT32Instruction::Smulw_T1(R::R3, R::R4, R::R5, true),
        ArmT32Instruction::Smla_T1(R::R0, R::R1, R::R2, R::R3, false, true),
        ArmT32Instruction::Smlaw_T1(R::R4, R::R5, R::R6, R::R7, false),
        ArmT32Instruction::Smlal_Halfword_T1(R::R0, R::R1, R::R2, R::R3, true, false),
        ArmT32Instruction::Smuad_T1(R::R0, R::R1, R::R2, false),
        ArmT32Instruction::Smusd_T1(R::R3, R::R4, R::R5, true),
        ArmT32Instruction::Smlad_T1(R::R0, R::R1, R::R2, R::R3, true),
        ArmT32Instruction::Smlsd_T1(R::R4, R::R5, R::R6, R::R7, false),
        ArmT32Instruction::Smlald_T1(R::R0, R::R1, R::R2, R::R3, false),
        ArmT32Instruction::Smlsld_T1(R::R4, R::R5, R::R6, R::R7, true),
        ArmT32Instruction::Smmul_T1(R::R0, R::R1, R::R2, true),
        ArmT32Instruction::Smmla_T1(R::R0, R::R1, R::R2, R::R3, false),
        ArmT32Instruction::Smmls_T1(R::R4, R::R5, R::R6, R::R7, true),
        // ---- ARMv7E-M FP batch M8f: load/store ----
        ArmT32Instruction::Vldr_Single_T2(s(0), R::R0, 0),
        ArmT32Instruction::Vldr_Single_T2(s(15), R::R1, 4),
        ArmT32Instruction::Vldr_Single_T2(s(31), R::R2, -8),
        ArmT32Instruction::Vstr_Single_T2(s(7), R::R0, 1020),
        ArmT32Instruction::Vldr_Double_T1(d(0), R::R0, 0),
        ArmT32Instruction::Vldr_Double_T1(d(15), R::R3, 16),
        ArmT32Instruction::Vstr_Double_T1(d(5), R::R4, -256),
        // ---- ARMv7E-M FP batch M8g: load/store multiple ----
        ArmT32Instruction::Vldm_Single_T2(R::R0, false, false, s(0), 4),
        ArmT32Instruction::Vldm_Single_T2(R::R0, true, false, s(4), 4),
        ArmT32Instruction::Vstm_Single_T2(R::R1, true, false, s(0), 1),
        ArmT32Instruction::Vldm_Single_T2(R::R2, true, true, s(8), 2),
        ArmT32Instruction::Vstm_Single_T2(R::R13, true, true, s(0), 4), // vpush {s0-s3}
        ArmT32Instruction::Vldm_Single_T2(R::R13, true, false, s(0), 4), // vpop {s0-s3}
        ArmT32Instruction::Vldm_Double_T1(R::R0, false, false, d(0), 2),
        ArmT32Instruction::Vstm_Double_T1(R::R3, true, true, d(5), 3),
        ArmT32Instruction::Vstm_Double_T1(R::R13, true, true, d(0), 4), // vpush {d0-d3}
        // ---- ARMv7E-M FP batch M8h: data-processing ----
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vadd, s(0), s(1), s(2)),
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vsub, s(3), s(4), s(5)),
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vmul, s(6), s(7), s(8)),
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vdiv, s(9), s(10), s(11)),
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vnmul, s(0), s(1), s(2)),
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vmla, s(0), s(1), s(2)),
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vnmls, s(0), s(1), s(2)),
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vfma, s(0), s(1), s(2)),
        ArmT32Instruction::FpDataProcess3_Single(Fp3::Vfnma, s(0), s(1), s(2)),
        ArmT32Instruction::FpDataProcess2_Single(Fp2::Vabs, s(0), s(1)),
        ArmT32Instruction::FpDataProcess2_Single(Fp2::Vneg, s(3), s(4)),
        ArmT32Instruction::FpDataProcess2_Single(Fp2::Vsqrt, s(5), s(6)),
        ArmT32Instruction::FpDataProcess2_Single(Fp2::Vmov, s(10), s(11)),
        ArmT32Instruction::FpDataProcess3_Double(Fp3::Vadd, d(0), d(1), d(2)),
        ArmT32Instruction::FpDataProcess3_Double(Fp3::Vmul, d(3), d(4), d(5)),
        ArmT32Instruction::FpDataProcess2_Double(Fp2::Vabs, d(6), d(7)),
        ArmT32Instruction::FpDataProcess2_Double(Fp2::Vsqrt, d(8), d(9)),
        ArmT32Instruction::FpDataProcess2_Double(Fp2::Vmov, d(10), d(11)),
        // ---- ARMv7E-M FP batch M8i: compare / FPSCR transfer / core<->single move ----
        ArmT32Instruction::Vcmp_Single_T1(s(0), s(1), false),
        ArmT32Instruction::Vcmp_Single_T1(s(2), s(3), true),
        ArmT32Instruction::Vcmp_Zero_Single_T2(s(4), false),
        ArmT32Instruction::Vcmp_Double_T1(d(0), d(1), false),
        ArmT32Instruction::Vcmp_Zero_Double_T2(d(2), true),
        ArmT32Instruction::Vmrs_Apsr_Nzcv_T1,
        ArmT32Instruction::Vmrs_T1(R::R0),
        ArmT32Instruction::Vmsr_T1(R::R1),
        ArmT32Instruction::Vmov_Core_To_Single_T1(s(0), R::R1),
        ArmT32Instruction::Vmov_Single_To_Core_T1(R::R2, s(3)),
        ArmT32Instruction::Vcvt_FloatToInt_FromSingle_T1(s(0), s(1), true, true),
        ArmT32Instruction::Vcvt_FloatToInt_FromSingle_T1(s(2), s(3), false, true),
        ArmT32Instruction::Vcvt_FloatToInt_FromSingle_T1(s(0), s(1), true, false),
        ArmT32Instruction::Vcvt_IntToFloat_ToSingle_T1(s(4), s(5), true),
        ArmT32Instruction::Vcvt_IntToFloat_ToSingle_T1(s(6), s(7), false),
        ArmT32Instruction::Vcvt_Single_To_Double_T1(d(0), s(2)),
        ArmT32Instruction::Vcvt_Double_To_Single_T1(s(3), d(4)),
        ArmT32Instruction::Vcvt_FloatToInt_FromDouble_T1(s(0), d(1), true, true),
        ArmT32Instruction::Vcvt_IntToFloat_ToDouble_T1(d(2), s(3), true),
        // ---- ARMv7E-M FP M8i final corners: VMOV immediate / core-pair, half + fixed-point VCVT ----
        ArmT32Instruction::Vmov_Immediate_Single_T1(s(0), 0x70),
        ArmT32Instruction::Vmov_Immediate_Double_T1(d(0), 0x70),
        ArmT32Instruction::Vmov_Double_To_CorePair_T1(R::R0, R::R1, d(2)),
        ArmT32Instruction::Vmov_CorePair_To_Double_T1(d(3), R::R4, R::R5),
        ArmT32Instruction::Vmov_Singles_To_CorePair_T1(R::R6, R::R7, s(8)),
        ArmT32Instruction::Vmov_CorePair_To_Singles_T1(s(10), R::R2, R::R3),
        ArmT32Instruction::Vcvt_HalfToSingle_T1(s(0), s(1), false),
        ArmT32Instruction::Vcvt_SingleToHalf_T1(s(4), s(5), true),
        ArmT32Instruction::Vcvt_FloatToFixed_Single_T1(s(0), true, false, 1),
        ArmT32Instruction::Vcvt_FixedToFloat_Single_T1(s(3), true, false, 3),
        ArmT32Instruction::Vcvt_FloatToFixed_Double_T1(d(2), false, true, 10),
    ]
}

// ---- backends ----

enum AssemblerKind {
    Llvm,
    Gnu,
}

struct Backend {
    name: &'static str,
    kind: AssemblerKind,
    assembler: String,
    objcopy: String,
}

impl Backend {
    fn assemble_and_extract(&self, source: &str) -> Vec<u8> {
        let work = WorkDir::new("arm32_oracle");
        let source_path = work.path.join("oracle.s");
        let object_path = work.path.join("oracle.o");
        let binary_path = work.path.join("oracle.bin");
        std::fs::write(&source_path, source).expect("write .s");

        let mut assemble = Command::new(&self.assembler);
        match self.kind {
            // both assemblers target a Cortex-M7 (ARMv7E-M + DSP + the FPv5 double-precision FPU) so the
            // v7-M, DSP, and FP samples all assemble; the lower-tier encodings are identical on that core.
            // (Our encoder is target-independent regardless.)
            AssemblerKind::Llvm => {
                assemble
                    .args([
                        "--target=thumbv7em-none-eabi",
                        "-mcpu=cortex-m7",
                        "-mfpu=fpv5-d16",
                        "-mfloat-abi=hard",
                        "-c",
                    ])
                    .arg(&source_path)
                    .arg("-o")
                    .arg(&object_path);
            }
            AssemblerKind::Gnu => {
                assemble
                    .args([
                        "-mcpu=cortex-m7",
                        "-mfpu=fpv5-d16",
                        "-mfloat-abi=hard",
                        "-mthumb",
                    ])
                    .arg(&source_path)
                    .arg("-o")
                    .arg(&object_path);
            }
        }
        run(&mut assemble, source);

        run(
            Command::new(&self.objcopy)
                .args(["-O", "binary"])
                .arg(&object_path)
                .arg(&binary_path),
            source,
        );

        std::fs::read(&binary_path).expect("read extracted .text")
    }
}

fn discover_backends() -> Vec<Backend> {
    let mut backends = Vec::new();

    if let (Some(assembler), Some(objcopy)) = (
        locate(&["CLANG"], &["clang"], &[]),
        locate(&["LLVM_OBJCOPY"], &["llvm-objcopy"], &[]),
    ) {
        backends.push(Backend {
            name: "LLVM",
            kind: AssemblerKind::Llvm,
            assembler,
            objcopy,
        });
    }

    let gnu_roots = gnu_toolchain_search_roots();
    if let (Some(assembler), Some(objcopy)) = (
        locate(&["ARM_NONE_EABI_AS"], &["arm-none-eabi-as"], &gnu_roots),
        locate(
            &["ARM_NONE_EABI_OBJCOPY"],
            &["arm-none-eabi-objcopy"],
            &gnu_roots,
        ),
    ) {
        backends.push(Backend {
            name: "GNU",
            kind: AssemblerKind::Gnu,
            assembler,
            objcopy,
        });
    }

    backends
}

// Resolve a tool: env override(s), then bare PATH name(s), then any of the given install roots.
fn locate(env_vars: &[&str], names: &[&str], install_roots: &[PathBuf]) -> Option<String> {
    for env_var in env_vars {
        if let Ok(path) = std::env::var(env_var)
            && !path.is_empty()
        {
            return Some(path);
        }
    }
    for name in names {
        if Command::new(name)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return Some(name.to_string());
        }
    }
    for root in install_roots {
        for name in names {
            let candidate = root.join(format!("{}.exe", name));
            if candidate.is_file() {
                return Some(candidate.to_string_lossy().into_owned());
            }
        }
    }
    None
}

// `bin` directories of any installed "Arm GNU Toolchain arm-none-eabi" (versioned subfolders).
fn gnu_toolchain_search_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for base in [
        "C:\\Program Files (x86)\\Arm GNU Toolchain arm-none-eabi",
        "C:\\Program Files\\Arm GNU Toolchain arm-none-eabi",
    ] {
        if let Ok(entries) = std::fs::read_dir(base) {
            for entry in entries.flatten() {
                let bin = entry.path().join("bin");
                if bin.is_dir() {
                    roots.push(bin);
                }
            }
        }
    }
    roots
}

fn run(command: &mut Command, source_for_diagnostics: &str) {
    let output = command.output().expect("failed to launch external tool");
    if !output.status.success() {
        panic!(
            "external tool failed ({:?})\n--- stderr ---\n{}\n--- source ---\n{}",
            command,
            String::from_utf8_lossy(&output.stderr),
            source_for_diagnostics,
        );
    }
}

struct WorkDir {
    path: PathBuf,
}
// A process-wide sequence so every WorkDir gets a distinct directory: the oracle's tests run on separate
// threads, and a shared path would let one test's Drop delete another's in-flight working files.
static WORKDIR_SEQUENCE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl WorkDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        let sequence = WORKDIR_SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        path.push(format!("{}_{}_{}", tag, std::process::id(), sequence));
        std::fs::create_dir_all(&path).expect("create work dir");
        Self { path }
    }
}
impl Drop for WorkDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
