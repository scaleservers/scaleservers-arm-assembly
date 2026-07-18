// Copyright (c) Scaleservers LLC

// Differential oracle for the A32 ("ARM" state) instruction set against the GNU assembler
// (`arm-none-eabi-as -march=armv8-a+crc`, ARM mode). The expected UAL text is written BY HAND alongside each
// `ArmA32Instruction` -- deliberately independent of the crate's A32 emitter, so an emitter bug can never
// mask an encoder bug (the emitter is itself validated by the re-assembly round-trip below). For each sample:
//
//   ENCODER check: GNU's bytes for the hand-written UAL must equal our `encode()`.
//   DECODER check: our `decode()` of GNU's bytes must reproduce the exact sample instruction.
//
// All A32 instructions are one 32-bit word, so the byte stream is a flat 4-bytes-per-instruction sequence.
// If no GNU toolchain is found the test SKIPS green. (clang is the other backend the T32 oracle uses, but
// it is not required here.)

use std::path::PathBuf;
use std::process::Command;
mod common;

use scaleservers_arm32_assembly::Arm32BlockAddressMode::*;
use scaleservers_arm32_assembly::Arm32Condition as C;
use scaleservers_arm32_assembly::Arm32CpsMode;
use scaleservers_arm32_assembly::Arm32DirectedRound as DRnd;
use scaleservers_arm32_assembly::Arm32ExtendType as Ext;
use scaleservers_arm32_assembly::Arm32FpDataOperation2 as F2;
use scaleservers_arm32_assembly::Arm32FpDataOperation3 as F3;
use scaleservers_arm32_assembly::Arm32GeneralPurposeRegister as R;
use scaleservers_arm32_assembly::Arm32IndexMode as Idx;
use scaleservers_arm32_assembly::Arm32MemoryOffset as Mem;
use scaleservers_arm32_assembly::Arm32MemoryOffset8 as Mem8;
use scaleservers_arm32_assembly::Arm32NeonAesOp as NAes;
use scaleservers_arm32_assembly::Arm32NeonBitwiseOp as NBit;
use scaleservers_arm32_assembly::Arm32NeonDiffLongOp as NDL;
use scaleservers_arm32_assembly::Arm32NeonDiffNarrowOp as NDN;
use scaleservers_arm32_assembly::Arm32NeonDiffWideOp as NDW;
use scaleservers_arm32_assembly::Arm32NeonFloatOp as NFlt;
use scaleservers_arm32_assembly::Arm32NeonIntegerOp as NInt;
use scaleservers_arm32_assembly::Arm32NeonLoadStoreAddress as NLsa;
use scaleservers_arm32_assembly::Arm32NeonMisc2FixedOp as NMF;
use scaleservers_arm32_assembly::Arm32NeonMisc2SizedOp as NMS;
use scaleservers_arm32_assembly::Arm32NeonNarrowOp as NMN;
use scaleservers_arm32_assembly::Arm32NeonScalarLongOp as NScL;
use scaleservers_arm32_assembly::Arm32NeonScalarOp as NSc;
use scaleservers_arm32_assembly::Arm32NeonSha2Op as NSha2;
use scaleservers_arm32_assembly::Arm32NeonSha3Op as NSha3;
use scaleservers_arm32_assembly::Arm32NeonShiftNarrowOp as NShN;
use scaleservers_arm32_assembly::Arm32NeonShiftOp as NSh;
use scaleservers_arm32_assembly::Arm32NeonSize as NSz;
use scaleservers_arm32_assembly::Arm32ParallelOperation as Pop;
use scaleservers_arm32_assembly::Arm32ParallelPrefix as Ppre;
use scaleservers_arm32_assembly::Arm32QuadwordRegister;
use scaleservers_arm32_assembly::Arm32RegisterShift as Shift;
use scaleservers_arm32_assembly::Arm32ShiftType as ShiftType;
use scaleservers_arm32_assembly::Arm32VmovLaneSize as VLS;
use scaleservers_arm32_assembly::Arm32VrintMode as VRnd;
use scaleservers_arm32_assembly::Arm32VselCondition as Vsel;
use scaleservers_arm32_assembly::ArmA32Instruction;
use scaleservers_arm32_assembly::ArmA32Instruction as I;
use scaleservers_arm32_assembly::{Arm32DoublePrecisionRegister, Arm32SinglePrecisionRegister};

fn s(number: u8) -> Arm32SinglePrecisionRegister {
    Arm32SinglePrecisionRegister::new(number).unwrap()
}
fn d(number: u8) -> Arm32DoublePrecisionRegister {
    Arm32DoublePrecisionRegister::new(number).unwrap()
}
fn q(number: u8) -> Arm32QuadwordRegister {
    Arm32QuadwordRegister::new(number).unwrap()
}

// Miri can neither spawn processes nor touch the filesystem, and these oracle tests shell out to gas /
// llvm-mc. `cargo +nightly miri test --lib` already excludes this `tests/` integration binary; the
// `#[cfg_attr(miri, ignore)]` is defensive and a no-op under normal `cargo test` (`cfg(miri)` is false there),
// so the differential oracle keeps running exactly as before.
#[test]
#[cfg_attr(miri, ignore)]
fn gnu_assembler_matches_our_a32_encoder_and_decoder() {
    let backend = match discover_gnu_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require(
                "differential_oracle_a32: no GNU arm-none-eabi assembler found",
            );
            return;
        }
    };

    let samples = sample_instructions();

    // one ARM-mode .s file; assemble it and compare 4 bytes per instruction
    let mut source = String::from(".syntax unified\n.arm\n.text\n");
    for (ual, _) in &samples {
        source.push_str(ual);
        source.push('\n');
    }
    let actual = backend.assemble_and_extract(&source);

    assert_eq!(
        actual.len(),
        samples.len() * 4,
        "GNU produced {} bytes for {} A32 samples",
        actual.len(),
        samples.len()
    );

    for (index, (ual, instruction)) in samples.iter().enumerate() {
        let our = instruction.encode().unwrap();
        let theirs = &actual[index * 4..index * 4 + 4];
        assert_eq!(
            our.as_slice(),
            theirs,
            "ENCODER disagreement on `{}`:\n  instruction: {:?}\n  our bytes:   {:02x?}\n  GNU bytes:   {:02x?}",
            ual,
            instruction,
            our,
            theirs
        );

        let mut iter = theirs.iter();
        let mut consumed = 0;
        let decoded = ArmA32Instruction::decode(&mut iter, &mut consumed)
            .expect("decode GNU bytes")
            .expect("non-empty");
        assert_eq!(
            &decoded, instruction,
            "DECODER disagreement on `{}`: GNU bytes {:02x?} decoded to {:?}",
            ual, theirs, decoded
        );
    }

    eprintln!(
        "differential_oracle_a32: GNU validated {} A32 instructions (encoder + decoder)",
        samples.len()
    );
}

// The SECOND oracle for A32: LLVM (`llvm-mc`). GNU = Linux servers, LLVM = Apple-silicon macOS -- both PRIMARY,
// because a single oracle silently validates its own bugs (this is exactly how the MVE VMLA-by-scalar bug hid:
// GNU and our model shared it; LLVM + the spec exposed it). A full GNU-vs-LLVM sweep of this sample set found
// the two assemblers agree on every A32 encoding, so this locks that in as a regression. Assembled PER
// INSTRUCTION so a form llvm-mc legitimately refuses (e.g. `cdp` to a coprocessor deprecated in ARMv8-A -- our
// GNU-matching encoding is still spec-correct; llvm-mc emits the same bytes at `-triple=armv7a`) is isolated
// and skipped, not a hard failure. Skips green if no llvm-mc is found.
#[test]
#[cfg_attr(miri, ignore)]
fn llvm_assembler_matches_our_a32_encoder() {
    let backend = match discover_llvm_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require("differential_oracle_a32 (LLVM): no llvm-mc found");
            return;
        }
    };

    let samples = sample_instructions();
    let (mut checked, mut skipped) = (0usize, 0usize);
    for (ual, instruction) in &samples {
        let Some(theirs) = backend.assemble_one(ual) else {
            skipped += 1;
            eprintln!(
                "  llvm-mc declined `{ual}` (e.g. ARMv8-deprecated coprocessor) -- validated against GNU instead"
            );
            continue;
        };
        let our = instruction.encode().unwrap();
        assert_eq!(
            our.as_slice(),
            theirs.as_slice(),
            "LLVM ENCODER disagreement on `{}`:\n  instruction: {:?}\n  our bytes:  {:02x?}\n  LLVM bytes: {:02x?}",
            ual,
            instruction,
            our,
            theirs
        );
        checked += 1;
    }
    eprintln!(
        "differential_oracle_a32 (LLVM): validated {checked} A32 instructions, skipped {skipped} llvm-incompatible"
    );
    assert!(
        checked > 400,
        "LLVM validated only {checked} A32 samples -- oracle likely misconfigured (wrong triple/features?)"
    );
    assert!(
        skipped <= 3,
        "llvm-mc declined {skipped} A32 samples (expected ~1, the deprecated-coprocessor CDP) -- a new GNU/LLVM divergence to investigate"
    );
}

// The A32 condition field is a uniform 4-bit prefix on (almost) every instruction, but the hand-written
// samples above are nearly all `al`. Sweep the FULL condition set on a representative instruction from three
// classes (data-processing register, data-processing immediate, and a load) and differential every code
// against GNU -- closing the condition-encoding gap (the hand-curated set tests only a couple of conditions).
#[test]
#[cfg_attr(miri, ignore)]
fn gnu_validates_the_full_condition_set() {
    let backend = match discover_gnu_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require(
                "differential_oracle_a32 condition sweep: no GNU arm-none-eabi assembler found",
            );
            return;
        }
    };

    // every conditional code (the unconditional `al` is already covered by the main sample set)
    let conditions = [
        ("eq", C::Equal),
        ("ne", C::NotEqual),
        ("cs", C::CarrySet),
        ("cc", C::CarryClear),
        ("mi", C::MinusNegative),
        ("pl", C::PlusPositiveOrZero),
        ("vs", C::Overflow),
        ("vc", C::NoOverflow),
        ("hi", C::UnsignedHigher),
        ("ls", C::UnsignedLowerOrSame),
        ("ge", C::SignedGreaterThanOrEqual),
        ("lt", C::SignedLessThan),
        ("gt", C::SignedGreaterThan),
        ("le", C::SignedLessThanOrEqual),
    ];

    let mut samples: Vec<(String, ArmA32Instruction)> = Vec::new();
    for (suffix, condition) in conditions {
        samples.push((
            format!("add{suffix} r0, r1, r2"),
            I::Add_Register_A1(condition, false, R::R0, R::R1, R::R2, Shift::Lsl(0)),
        ));
        samples.push((
            format!("sub{suffix} r3, r4, #5"),
            I::Sub_Immediate_A1(condition, false, R::R3, R::R4, 5),
        ));
        samples.push((
            format!("ldr{suffix} r0, [r1, #4]"),
            I::Ldr_A1(
                condition,
                R::R0,
                R::R1,
                Mem::Immediate {
                    add: true,
                    imm12: 4,
                },
                Idx::Offset,
            ),
        ));
    }

    let mut source = String::from(".syntax unified\n.arm\n.text\n");
    for (ual, _) in &samples {
        source.push_str(ual);
        source.push('\n');
    }
    let actual = backend.assemble_and_extract(&source);
    assert_eq!(
        actual.len(),
        samples.len() * 4,
        "GNU produced {} bytes for {} conditional samples",
        actual.len(),
        samples.len()
    );

    for (index, (ual, instruction)) in samples.iter().enumerate() {
        let our = instruction.encode().unwrap();
        let theirs = &actual[index * 4..index * 4 + 4];
        assert_eq!(
            our.as_slice(),
            theirs,
            "ENCODER disagreement on `{}`:\n  our bytes: {:02x?}\n  GNU bytes: {:02x?}",
            ual,
            our,
            theirs
        );
        let mut iter = theirs.iter();
        let mut consumed = 0;
        let decoded = ArmA32Instruction::decode(&mut iter, &mut consumed)
            .expect("decode GNU bytes")
            .expect("non-empty");
        assert_eq!(
            &decoded, instruction,
            "DECODER disagreement on `{}`: GNU bytes {:02x?} decoded to {:?}",
            ual, theirs, decoded
        );
    }
    eprintln!(
        "differential_oracle_a32: GNU validated {} conditional instructions across the full condition set",
        samples.len()
    );
}

// Load/store addressing-mode variants beyond the main sample set: scaled register offsets (LSR/ASR/ROR), a
// negated register offset, and register pre-/post-indexed writeback -- the forms the hand-curated samples
// (mostly immediate offsets + an LSL register offset) don't reach.
#[test]
#[cfg_attr(miri, ignore)]
fn gnu_validates_addressing_mode_variants() {
    let backend = match discover_gnu_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require(
                "differential_oracle_a32 addressing modes: no GNU arm-none-eabi assembler found",
            );
            return;
        }
    };
    let al = C::AlwaysUnconditional;
    let samples: Vec<(&str, ArmA32Instruction)> = vec![
        (
            "ldr r0, [r1, r2, lsr #3]",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Register {
                    add: true,
                    rm: R::R2,
                    shift: Shift::Lsr(3),
                },
                Idx::Offset,
            ),
        ),
        (
            "ldr r0, [r1, r2, asr #1]",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Register {
                    add: true,
                    rm: R::R2,
                    shift: Shift::Asr(1),
                },
                Idx::Offset,
            ),
        ),
        (
            "ldr r0, [r1, r2, ror #4]",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Register {
                    add: true,
                    rm: R::R2,
                    shift: Shift::Ror(4),
                },
                Idx::Offset,
            ),
        ),
        (
            "ldr r0, [r1, -r2, lsl #1]",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Register {
                    add: false,
                    rm: R::R2,
                    shift: Shift::Lsl(1),
                },
                Idx::Offset,
            ),
        ),
        (
            "ldr r0, [r1, r2]!",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Register {
                    add: true,
                    rm: R::R2,
                    shift: Shift::Lsl(0),
                },
                Idx::PreIndex,
            ),
        ),
        (
            "ldr r0, [r1], r2",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Register {
                    add: true,
                    rm: R::R2,
                    shift: Shift::Lsl(0),
                },
                Idx::PostIndex,
            ),
        ),
        (
            "str r5, [r6, #-8]!",
            I::Str_A1(
                al,
                R::R5,
                R::R6,
                Mem::Immediate {
                    add: false,
                    imm12: 8,
                },
                Idx::PreIndex,
            ),
        ),
        (
            "str r5, [r6], #-12",
            I::Str_A1(
                al,
                R::R5,
                R::R6,
                Mem::Immediate {
                    add: false,
                    imm12: 12,
                },
                Idx::PostIndex,
            ),
        ),
    ];

    let mut source = String::from(".syntax unified\n.arm\n.text\n");
    for (ual, _) in &samples {
        source.push_str(ual);
        source.push('\n');
    }
    let actual = backend.assemble_and_extract(&source);
    assert_eq!(
        actual.len(),
        samples.len() * 4,
        "GNU produced {} bytes for {} addressing-mode samples",
        actual.len(),
        samples.len()
    );

    for (index, (ual, instruction)) in samples.iter().enumerate() {
        let our = instruction.encode().unwrap();
        let theirs = &actual[index * 4..index * 4 + 4];
        assert_eq!(
            our.as_slice(),
            theirs,
            "ENCODER disagreement on `{}`:\n  our bytes: {:02x?}\n  GNU bytes: {:02x?}",
            ual,
            our,
            theirs
        );
        let mut iter = theirs.iter();
        let mut consumed = 0;
        let decoded = ArmA32Instruction::decode(&mut iter, &mut consumed)
            .expect("decode GNU bytes")
            .expect("non-empty");
        assert_eq!(
            &decoded, instruction,
            "DECODER disagreement on `{}`: GNU bytes {:02x?} decoded to {:?}",
            ual, theirs, decoded
        );
    }
    eprintln!(
        "differential_oracle_a32: GNU validated {} addressing-mode variants",
        samples.len()
    );
}

// The disassembler round-trip: render each instruction via the A32 UAL emitter, then re-assemble that text
// with GNU and confirm the bytes match `encode()`. This proves the emitter's output is faithful (it
// re-assembles to the same instruction) for every form the oracle covers.
#[test]
#[cfg_attr(miri, ignore)]
fn our_emitter_output_reassembles_via_gnu() {
    use scaleservers_arm32_assembly::ArmAssemblySyntax;
    let backend = match discover_gnu_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require(
                "differential_oracle_a32 emitter check: no GNU arm-none-eabi assembler found",
            );
            return;
        }
    };

    let samples = sample_instructions();
    let rendered: Vec<String> = samples
        .iter()
        .map(|(_, instruction)| instruction.to_assembly_string(ArmAssemblySyntax::Gnu))
        .collect();

    let mut source = String::from(".syntax unified\n.arm\n.text\n");
    for text in &rendered {
        source.push_str(text);
        source.push('\n');
    }
    let actual = backend.assemble_and_extract(&source);
    assert_eq!(
        actual.len(),
        samples.len() * 4,
        "GNU produced {} bytes for {} emitted A32 samples",
        actual.len(),
        samples.len()
    );

    for (index, (ual, instruction)) in samples.iter().enumerate() {
        let our = instruction.encode().unwrap();
        let theirs = &actual[index * 4..index * 4 + 4];
        assert_eq!(
            our.as_slice(),
            theirs,
            "EMITTER disagreement: our `{}` rendered as `{}` which GNU assembled to {:02x?}, but encode() is {:02x?}\n  instruction: {:?}",
            ual,
            rendered[index],
            theirs,
            our,
            instruction
        );
    }
    eprintln!(
        "differential_oracle_a32: GNU validated {} A32 emitter renderings",
        samples.len()
    );
}

// A broad hand-written cross-section: one (GNU UAL, model) pair per representative form, across batches.
// PC-relative forms (B/BL, LDR literal, ADR) are excluded (they need labels to assemble).
fn sample_instructions() -> Vec<(&'static str, I)> {
    let al = C::AlwaysUnconditional;
    vec![
        // -- data processing --
        (
            "and r0, r1, r2",
            I::And_Register_A1(al, false, R::R0, R::R1, R::R2, Shift::Lsl(0)),
        ),
        (
            "eor r0, r1, r2, lsr #4",
            I::Eor_Register_A1(al, false, R::R0, R::R1, R::R2, Shift::Lsr(4)),
        ),
        (
            "orrs r5, r6, r7, ror #8",
            I::Orr_Register_A1(al, true, R::R5, R::R6, R::R7, Shift::Ror(8)),
        ),
        (
            "mvn r0, r1",
            I::Mvn_Register_A1(al, false, R::R0, R::R1, Shift::Lsl(0)),
        ),
        (
            "sub r3, r4, #5",
            I::Sub_Immediate_A1(al, false, R::R3, R::R4, 5),
        ),
        (
            "add r0, r1, #0xff000000",
            I::Add_Immediate_A1(al, false, R::R0, R::R1, 0xFF00_0000),
        ),
        (
            "mov r0, #0x100",
            I::Mov_Immediate_A1(al, false, R::R0, 0x100),
        ),
        ("cmp r0, #0", I::Cmp_Immediate_A1(al, R::R0, 0)),
        ("tst r1, #1", I::Tst_Immediate_A1(al, R::R1, 1)),
        (
            "cmn r4, r5",
            I::Cmn_Register_A1(al, R::R4, R::R5, Shift::Lsl(0)),
        ),
        (
            "addgt r0, r1, r2",
            I::Add_Register_A1(
                C::SignedGreaterThan,
                false,
                R::R0,
                R::R1,
                R::R2,
                Shift::Lsl(0),
            ),
        ),
        (
            "add r0, r1, r2, lsl r3",
            I::Add_RegisterShiftedRegister_A1(
                al,
                false,
                R::R0,
                R::R1,
                R::R2,
                ShiftType::Lsl,
                R::R3,
            ),
        ),
        (
            "mov r0, r1, asr r2",
            I::Mov_RegisterShiftedRegister_A1(al, false, R::R0, R::R1, ShiftType::Asr, R::R2),
        ),
        ("movw r0, #0x1234", I::Movw_A2(al, R::R0, 0x1234)),
        ("movt r1, #0xabcd", I::Movt_A1(al, R::R1, 0xABCD)),
        // -- multiply --
        ("mul r0, r1, r2", I::Mul_A1(al, false, R::R0, R::R1, R::R2)),
        (
            "mla r0, r1, r2, r3",
            I::Mla_A1(al, false, R::R0, R::R1, R::R2, R::R3),
        ),
        (
            "mls r0, r1, r2, r3",
            I::Mls_A1(al, R::R0, R::R1, R::R2, R::R3),
        ),
        (
            "umull r0, r1, r2, r3",
            I::Umull_A1(al, false, R::R0, R::R1, R::R2, R::R3),
        ),
        (
            "smull r0, r1, r2, r3",
            I::Smull_A1(al, false, R::R0, R::R1, R::R2, R::R3),
        ),
        (
            "umlal r0, r1, r2, r3",
            I::Umlal_A1(al, false, R::R0, R::R1, R::R2, R::R3),
        ),
        (
            "umaal r0, r1, r2, r3",
            I::Umaal_A1(al, R::R0, R::R1, R::R2, R::R3),
        ),
        // -- saturating + signed multiply --
        ("qadd r0, r1, r2", I::Qadd_A1(al, R::R0, R::R1, R::R2)),
        ("qdsub r6, r7, r8", I::Qdsub_A1(al, R::R6, R::R7, R::R8)),
        (
            "smlabb r0, r1, r2, r3",
            I::Smla_A1(al, R::R0, R::R1, R::R2, R::R3, false, false),
        ),
        (
            "smultt r0, r1, r2",
            I::Smul_A1(al, R::R0, R::R1, R::R2, true, true),
        ),
        (
            "smlawb r0, r1, r2, r3",
            I::Smlaw_A1(al, R::R0, R::R1, R::R2, R::R3, false),
        ),
        (
            "smlalbb r0, r1, r2, r3",
            I::Smlal_Halfword_A1(al, R::R0, R::R1, R::R2, R::R3, false, false),
        ),
        (
            "smuad r0, r1, r2",
            I::Smuad_A1(al, R::R0, R::R1, R::R2, false),
        ),
        (
            "smlad r0, r1, r2, r3",
            I::Smlad_A1(al, R::R0, R::R1, R::R2, R::R3, false),
        ),
        (
            "smmul r0, r1, r2",
            I::Smmul_A1(al, R::R0, R::R1, R::R2, false),
        ),
        (
            "smmls r0, r1, r2, r3",
            I::Smmls_A1(al, R::R0, R::R1, R::R2, R::R3, false),
        ),
        (
            "smlald r0, r1, r2, r3",
            I::Smlald_A1(al, R::R0, R::R1, R::R2, R::R3, false),
        ),
        // -- parallel + select --
        (
            "sadd16 r0, r1, r2",
            I::ParallelAddSub_A1(al, Pop::Add16, Ppre::Signed, R::R0, R::R1, R::R2),
        ),
        (
            "uqsub8 r3, r4, r5",
            I::ParallelAddSub_A1(al, Pop::Sub8, Ppre::UnsignedSaturating, R::R3, R::R4, R::R5),
        ),
        ("sel r0, r1, r2", I::Sel_A1(al, R::R0, R::R1, R::R2)),
        // -- extend / reverse / clz --
        ("sxtb r0, r1", I::Extend_A1(al, Ext::Sxtb, R::R0, R::R1, 0)),
        ("uxth r3, r4", I::Extend_A1(al, Ext::Uxth, R::R3, R::R4, 0)),
        (
            "sxtab r0, r1, r2",
            I::ExtendAndAdd_A1(al, Ext::Sxtb, R::R0, R::R1, R::R2, 0),
        ),
        ("rev r0, r1", I::Rev_A1(al, R::R0, R::R1)),
        ("rev16 r2, r3", I::Rev16_A1(al, R::R2, R::R3)),
        ("revsh r4, r5", I::Revsh_A1(al, R::R4, R::R5)),
        ("rbit r6, r7", I::Rbit_A1(al, R::R6, R::R7)),
        ("clz r0, r1", I::Clz_A1(al, R::R0, R::R1)),
        // -- pack / saturate / sad --
        (
            "pkhbt r0, r1, r2, lsl #4",
            I::Pkhbt_A1(al, R::R0, R::R1, R::R2, 4),
        ),
        (
            "pkhtb r0, r1, r2, asr #1",
            I::Pkhtb_A1(al, R::R0, R::R1, R::R2, 1),
        ),
        (
            "ssat r0, #1, r1",
            I::Ssat_A1(al, R::R0, 1, R::R1, Shift::Lsl(0)),
        ),
        (
            "usat r3, #15, r4, lsl #5",
            I::Usat_A1(al, R::R3, 15, R::R4, Shift::Lsl(5)),
        ),
        ("ssat16 r0, #1, r1", I::Ssat16_A1(al, R::R0, 1, R::R1)),
        ("usad8 r0, r1, r2", I::Usad8_A1(al, R::R0, R::R1, R::R2)),
        (
            "usada8 r0, r1, r2, r3",
            I::Usada8_A1(al, R::R0, R::R1, R::R2, R::R3),
        ),
        // -- bitfield --
        ("bfi r2, r3, #4, #8", I::Bfi_A1(al, R::R2, R::R3, 4, 8)),
        ("bfc r0, #0, #32", I::Bfc_A1(al, R::R0, 0, 32)),
        ("sbfx r2, r3, #4, #8", I::Sbfx_A1(al, R::R2, R::R3, 4, 8)),
        ("ubfx r0, r1, #0, #32", I::Ubfx_A1(al, R::R0, R::R1, 0, 32)),
        // -- load/store single --
        (
            "ldr r0, [r1, #4]",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Immediate {
                    add: true,
                    imm12: 4,
                },
                Idx::Offset,
            ),
        ),
        (
            "ldr r0, [r1, #-4]",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Immediate {
                    add: false,
                    imm12: 4,
                },
                Idx::Offset,
            ),
        ),
        (
            "str r2, [r3, #8]!",
            I::Str_A1(
                al,
                R::R2,
                R::R3,
                Mem::Immediate {
                    add: true,
                    imm12: 8,
                },
                Idx::PreIndex,
            ),
        ),
        (
            "ldr r0, [r1], #4",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Immediate {
                    add: true,
                    imm12: 4,
                },
                Idx::PostIndex,
            ),
        ),
        (
            "ldrb r0, [r1, r2]",
            I::Ldrb_A1(
                al,
                R::R0,
                R::R1,
                Mem::Register {
                    add: true,
                    rm: R::R2,
                    shift: Shift::Lsl(0),
                },
                Idx::Offset,
            ),
        ),
        (
            "ldr r0, [r1, r2, lsl #2]",
            I::Ldr_A1(
                al,
                R::R0,
                R::R1,
                Mem::Register {
                    add: true,
                    rm: R::R2,
                    shift: Shift::Lsl(2),
                },
                Idx::Offset,
            ),
        ),
        (
            "ldrt r0, [r1], #4",
            I::Ldrt_A1(
                al,
                R::R0,
                R::R1,
                Mem::Immediate {
                    add: true,
                    imm12: 4,
                },
            ),
        ),
        // -- load/store halfword/dual/signed --
        (
            "ldrh r0, [r1, #4]",
            I::Ldrh_A1(
                al,
                R::R0,
                R::R1,
                Mem8::Immediate { add: true, imm8: 4 },
                Idx::Offset,
            ),
        ),
        (
            "strh r2, [r3, #16]!",
            I::Strh_A1(
                al,
                R::R2,
                R::R3,
                Mem8::Immediate {
                    add: true,
                    imm8: 16,
                },
                Idx::PreIndex,
            ),
        ),
        (
            "ldrsb r0, [r1, r2]",
            I::Ldrsb_A1(
                al,
                R::R0,
                R::R1,
                Mem8::Register {
                    add: true,
                    rm: R::R2,
                },
                Idx::Offset,
            ),
        ),
        (
            "ldrsh r4, [r5, #10]",
            I::Ldrsh_A1(
                al,
                R::R4,
                R::R5,
                Mem8::Immediate {
                    add: true,
                    imm8: 10,
                },
                Idx::Offset,
            ),
        ),
        (
            "ldrd r0, r1, [r1, #8]",
            I::Ldrd_A1(
                al,
                R::R0,
                R::R1,
                Mem8::Immediate { add: true, imm8: 8 },
                Idx::Offset,
            ),
        ),
        (
            "strd r2, r3, [r3, #16]",
            I::Strd_A1(
                al,
                R::R2,
                R::R3,
                Mem8::Immediate {
                    add: true,
                    imm8: 16,
                },
                Idx::Offset,
            ),
        ),
        // -- load/store multiple --
        (
            "ldm r0, {r1, r2, r3}",
            I::Ldm_A1(
                al,
                IncrementAfter,
                R::R0,
                false,
                false,
                vec![R::R1, R::R2, R::R3],
            ),
        ),
        (
            "push {r4, r5, lr}",
            I::Stm_A1(
                al,
                DecrementBefore,
                R::R13,
                true,
                false,
                vec![R::R4, R::R5, R::R14],
            ),
        ),
        (
            "pop {r4, r5, pc}",
            I::Ldm_A1(
                al,
                IncrementAfter,
                R::R13,
                true,
                false,
                vec![R::R4, R::R5, R::R15],
            ),
        ),
        (
            "ldmib r0, {r1}",
            I::Ldm_A1(al, IncrementBefore, R::R0, false, false, vec![R::R1]),
        ),
        (
            "stmda r0, {r4, r5}",
            I::Stm_A1(al, DecrementAfter, R::R0, false, false, vec![R::R4, R::R5]),
        ),
        // -- synchronization --
        ("ldrex r0, [r1]", I::Ldrex_A1(al, R::R0, R::R1)),
        ("strex r0, r1, [r2]", I::Strex_A1(al, R::R0, R::R1, R::R2)),
        ("ldrexb r0, [r1]", I::Ldrexb_A1(al, R::R0, R::R1)),
        ("clrex", I::Clrex_A1),
        // (SWP/SWPB are obsoleted in ARMv8 and rejected by `as -march=armv8-a`; covered by the unit tests.)
        // -- branch/interwork (register forms only; PC-relative excluded) --
        ("bx lr", I::Bx_A1(al, R::R14)),
        ("blx r0", I::Blx_Register_A1(al, R::R0)),
        // -- status / system --
        ("mrs r0, cpsr", I::Mrs_A1(al, false, R::R0)),
        (
            "msr cpsr_f, r0",
            I::Msr_Register_A1(al, false, 0b1000, R::R0),
        ),
        (
            "msr cpsr_f, #0xf0000000",
            I::Msr_Immediate_A1(al, false, 0b1000, 0xF000_0000),
        ),
        (
            "cpsie i",
            I::Cps_A1(Arm32CpsMode::Enable, false, true, false, None),
        ),
        (
            "cps #0x13",
            I::Cps_A1(Arm32CpsMode::NoChange, false, false, false, Some(0x13)),
        ),
        ("setend be", I::Setend_A1(true)),
        // -- coprocessor --
        (
            "mcr p15, 0, r0, c1, c0, 0",
            I::Mcr_A1(al, 15, 0, R::R0, 1, 0, 0),
        ),
        (
            "mrc p15, 0, r1, c1, c0, 0",
            I::Mrc_A1(al, 15, 0, R::R1, 1, 0, 0),
        ),
        ("cdp p7, 1, c2, c3, c4, 5", I::Cdp_A1(al, 7, 1, 2, 3, 4, 5)),
        (
            "mcrr p15, 5, r0, r1, c2",
            I::Mcrr_A1(al, 15, 5, R::R0, R::R1, 2),
        ),
        (
            "ldc p14, c5, [r0, #8]",
            I::Ldc_A1(al, 14, false, 5, R::R0, true, 2, Idx::Offset),
        ),
        // -- hints / barriers / exceptions --
        ("nop", I::Nop_A1(al)),
        ("yield", I::Yield_A1(al)),
        ("wfe", I::Wfe_A1(al)),
        ("sev", I::Sev_A1(al)),
        ("dmb sy", I::Dmb_A1(0xF)),
        ("dsb sy", I::Dsb_A1(0xF)),
        ("isb sy", I::Isb_A1(0xF)),
        ("bkpt #0xabcd", I::Bkpt_A1(al, 0xABCD)),
        ("svc #0x123456", I::Svc_A1(al, 0x123456)),
        ("udf #0xdead", I::Udf_A1(al, 0xDEAD)),
        // -- preload --
        (
            "pld [r0, #8]",
            I::Pld_A1(
                R::R0,
                Mem::Immediate {
                    add: true,
                    imm12: 8,
                },
            ),
        ),
        (
            "pli [r3, #32]",
            I::Pli_A1(
                R::R3,
                Mem::Immediate {
                    add: true,
                    imm12: 32,
                },
            ),
        ),
        // -- ARMv8-A additions --
        ("crc32b r0, r1, r2", I::Crc32b_A1(al, R::R0, R::R1, R::R2)),
        ("crc32cw r3, r4, r5", I::Crc32cw_A1(al, R::R3, R::R4, R::R5)),
        ("lda r0, [r1]", I::Lda_A1(al, R::R0, R::R1)),
        ("stl r0, [r1]", I::Stl_A1(al, R::R0, R::R1)),
        ("ldaex r0, [r1]", I::Ldaex_A1(al, R::R0, R::R1)),
        ("stlex r0, r1, [r2]", I::Stlex_A1(al, R::R0, R::R1, R::R2)),
        ("sevl", I::Sevl_A1(al)),
        // -- VFP scalar load/store --
        ("vldr s0, [r0]", I::Vldr_Single_A1(al, s(0), R::R0, 0)),
        ("vldr s1, [r1, #4]", I::Vldr_Single_A1(al, s(1), R::R1, 4)),
        (
            "vstr s7, [r0, #1020]",
            I::Vstr_Single_A1(al, s(7), R::R0, 1020),
        ),
        ("vldr d0, [r0]", I::Vldr_Double_A1(al, d(0), R::R0, 0)),
        ("vldr d5, [r3, #16]", I::Vldr_Double_A1(al, d(5), R::R3, 16)),
        (
            "vldmia r0, {s0-s3}",
            I::Vldm_Single_A1(al, R::R0, false, false, s(0), 4),
        ),
        (
            "vldmia r0!, {s4-s7}",
            I::Vldm_Single_A1(al, R::R0, true, false, s(4), 4),
        ),
        (
            "vpush {s0-s3}",
            I::Vstm_Single_A1(al, R::R13, true, true, s(0), 4),
        ),
        (
            "vldmia r0, {d0-d1}",
            I::Vldm_Double_A1(al, R::R0, false, false, d(0), 2),
        ),
        (
            "vpush {d0-d3}",
            I::Vstm_Double_A1(al, R::R13, true, true, d(0), 4),
        ),
        // -- VFP data-processing --
        (
            "vadd.f32 s0, s1, s2",
            I::FpDataProcess3_Single_A1(al, F3::Vadd, s(0), s(1), s(2)),
        ),
        (
            "vsub.f32 s3, s4, s5",
            I::FpDataProcess3_Single_A1(al, F3::Vsub, s(3), s(4), s(5)),
        ),
        (
            "vmul.f32 s6, s7, s8",
            I::FpDataProcess3_Single_A1(al, F3::Vmul, s(6), s(7), s(8)),
        ),
        (
            "vdiv.f32 s0, s1, s2",
            I::FpDataProcess3_Single_A1(al, F3::Vdiv, s(0), s(1), s(2)),
        ),
        (
            "vnmul.f32 s0, s1, s2",
            I::FpDataProcess3_Single_A1(al, F3::Vnmul, s(0), s(1), s(2)),
        ),
        (
            "vfma.f32 s0, s1, s2",
            I::FpDataProcess3_Single_A1(al, F3::Vfma, s(0), s(1), s(2)),
        ),
        (
            "vadd.f64 d0, d1, d2",
            I::FpDataProcess3_Double_A1(al, F3::Vadd, d(0), d(1), d(2)),
        ),
        (
            "vmul.f64 d3, d4, d5",
            I::FpDataProcess3_Double_A1(al, F3::Vmul, d(3), d(4), d(5)),
        ),
        (
            "vmov.f32 s10, s11",
            I::FpDataProcess2_Single_A1(al, F2::Vmov, s(10), s(11)),
        ),
        (
            "vabs.f32 s0, s1",
            I::FpDataProcess2_Single_A1(al, F2::Vabs, s(0), s(1)),
        ),
        (
            "vneg.f64 d6, d7",
            I::FpDataProcess2_Double_A1(al, F2::Vneg, d(6), d(7)),
        ),
        (
            "vsqrt.f64 d8, d9",
            I::FpDataProcess2_Double_A1(al, F2::Vsqrt, d(8), d(9)),
        ),
        // -- VFP compare / transfer / immediate --
        ("vcmp.f32 s0, s1", I::Vcmp_Single_A1(al, s(0), s(1), false)),
        ("vcmpe.f32 s2, s3", I::Vcmp_Single_A1(al, s(2), s(3), true)),
        ("vcmp.f64 d0, d1", I::Vcmp_Double_A1(al, d(0), d(1), false)),
        ("vcmp.f32 s4, #0", I::Vcmp_Zero_Single_A1(al, s(4), false)),
        ("vcmpe.f64 d2, #0", I::Vcmp_Zero_Double_A1(al, d(2), true)),
        ("vmrs r0, fpscr", I::Vmrs_A1(al, R::R0)),
        ("vmrs apsr_nzcv, fpscr", I::Vmrs_Apsr_Nzcv_A1(al)),
        ("vmsr fpscr, r1", I::Vmsr_A1(al, R::R1)),
        ("vmov s0, r1", I::Vmov_Core_To_Single_A1(al, s(0), R::R1)),
        ("vmov r2, s3", I::Vmov_Single_To_Core_A1(al, R::R2, s(3))),
        (
            "vmov.f32 s0, #1.0",
            I::Vmov_Immediate_Single_A1(al, s(0), 0x70),
        ),
        (
            "vmov.f64 d0, #1.0",
            I::Vmov_Immediate_Double_A1(al, d(0), 0x70),
        ),
        (
            "vmov r0, r1, d2",
            I::Vmov_Double_To_CorePair_A1(al, R::R0, R::R1, d(2)),
        ),
        (
            "vmov d3, r4, r5",
            I::Vmov_CorePair_To_Double_A1(al, d(3), R::R4, R::R5),
        ),
        (
            "vmov r6, r7, s8, s9",
            I::Vmov_Singles_To_CorePair_A1(al, R::R6, R::R7, s(8)),
        ),
        (
            "vmov s10, s11, r2, r3",
            I::Vmov_CorePair_To_Singles_A1(al, s(10), R::R2, R::R3),
        ),
        // -- VFP conversions (VCVT) --
        (
            "vcvt.s32.f32 s0, s1",
            I::Vcvt_FloatToInt_FromSingle_A1(al, s(0), s(1), true, true),
        ),
        (
            "vcvt.u32.f32 s2, s3",
            I::Vcvt_FloatToInt_FromSingle_A1(al, s(2), s(3), false, true),
        ),
        (
            "vcvtr.s32.f32 s4, s5",
            I::Vcvt_FloatToInt_FromSingle_A1(al, s(4), s(5), true, false),
        ),
        (
            "vcvt.s32.f64 s0, d1",
            I::Vcvt_FloatToInt_FromDouble_A1(al, s(0), d(1), true, true),
        ),
        (
            "vcvt.f32.s32 s0, s1",
            I::Vcvt_IntToFloat_ToSingle_A1(al, s(0), s(1), true),
        ),
        (
            "vcvt.f32.u32 s2, s3",
            I::Vcvt_IntToFloat_ToSingle_A1(al, s(2), s(3), false),
        ),
        (
            "vcvt.f64.s32 d0, s1",
            I::Vcvt_IntToFloat_ToDouble_A1(al, d(0), s(1), true),
        ),
        (
            "vcvt.f64.f32 d0, s1",
            I::Vcvt_Single_To_Double_A1(al, d(0), s(1)),
        ),
        (
            "vcvt.f32.f64 s0, d1",
            I::Vcvt_Double_To_Single_A1(al, s(0), d(1)),
        ),
        (
            "vcvtb.f32.f16 s0, s1",
            I::Vcvt_HalfToSingle_A1(al, s(0), s(1), false),
        ),
        (
            "vcvtt.f32.f16 s2, s3",
            I::Vcvt_HalfToSingle_A1(al, s(2), s(3), true),
        ),
        (
            "vcvtb.f16.f32 s0, s1",
            I::Vcvt_SingleToHalf_A1(al, s(0), s(1), false),
        ),
        // -- FEAT_FP16 half<->double (VCVTB/T), FEAT_JSCVT (VJCVT), RAS (ESB), CSDB, FEAT_SB (SB) --
        (
            "vcvtb.f64.f16 d0, s1",
            I::Vcvt_HalfToDouble_A1(al, d(0), s(1), false),
        ),
        (
            "vcvtt.f64.f16 d3, s7",
            I::Vcvt_HalfToDouble_A1(al, d(3), s(7), true),
        ),
        (
            "vcvtb.f16.f64 s0, d1",
            I::Vcvt_DoubleToHalf_A1(al, s(0), d(1), false),
        ),
        (
            "vcvtt.f16.f64 s5, d9",
            I::Vcvt_DoubleToHalf_A1(al, s(5), d(9), true),
        ),
        ("vjcvt.s32.f64 s0, d1", I::Vjcvt_A1(al, s(0), d(1))),
        ("csdb", I::Csdb_A1(al)),
        ("esb", I::Esb_A1(al)),
        ("sb", I::Sb_A1),
        ("hlt #0xabcd", I::Hlt_A1(al, 0xABCD)),
        ("setpan #0", I::Setpan_A1(false)),
        ("setpan #1", I::Setpan_A1(true)),
        // -- VMOV between a core register and a vector scalar lane --
        (
            "vmov.32 d0[0], r1",
            I::Vmov_Core_To_Scalar_A1(al, VLS::Word, 0, d(0), R::R1),
        ),
        (
            "vmov.8 d2[3], r4",
            I::Vmov_Core_To_Scalar_A1(al, VLS::Byte, 3, d(2), R::R4),
        ),
        (
            "vmov.s8 r0, d1[2]",
            I::Vmov_Scalar_To_Core_A1(al, false, VLS::Byte, 2, R::R0, d(1)),
        ),
        (
            "vmov.u16 r5, d3[1]",
            I::Vmov_Scalar_To_Core_A1(al, true, VLS::Half, 1, R::R5, d(3)),
        ),
        (
            "vmov.32 r7, d9[0]",
            I::Vmov_Scalar_To_Core_A1(al, false, VLS::Word, 0, R::R7, d(9)),
        ),
        (
            "vcvt.s16.f32 s0, s0, #1",
            I::Vcvt_FloatToFixed_Single_A1(al, s(0), true, false, 1),
        ),
        (
            "vcvt.f32.s16 s0, s0, #2",
            I::Vcvt_FixedToFloat_Single_A1(al, s(0), true, false, 2),
        ),
        (
            "vcvt.u32.f64 d0, d0, #4",
            I::Vcvt_FloatToFixed_Double_A1(al, d(0), false, true, 4),
        ),
        (
            "vcvt.f64.u32 d0, d0, #8",
            I::Vcvt_FixedToFloat_Double_A1(al, d(0), false, true, 8),
        ),
        // -- ARMv8-A FP additions (VSEL / VMAXNM / VMINNM / VRINT / VCVTA-N-P-M) --
        (
            "vseleq.f32 s6, s7, s8",
            I::Vsel_Single_A1(Vsel::Equal, s(6), s(7), s(8)),
        ),
        (
            "vselvs.f32 s0, s1, s2",
            I::Vsel_Single_A1(Vsel::Overflow, s(0), s(1), s(2)),
        ),
        (
            "vselge.f32 s0, s1, s2",
            I::Vsel_Single_A1(Vsel::GreaterEqual, s(0), s(1), s(2)),
        ),
        (
            "vselgt.f32 s3, s4, s5",
            I::Vsel_Single_A1(Vsel::GreaterThan, s(3), s(4), s(5)),
        ),
        (
            "vselge.f64 d0, d1, d2",
            I::Vsel_Double_A1(Vsel::GreaterEqual, d(0), d(1), d(2)),
        ),
        (
            "vmaxnm.f32 s0, s1, s2",
            I::Vmaxnm_Single_A1(s(0), s(1), s(2)),
        ),
        (
            "vminnm.f32 s3, s4, s5",
            I::Vminnm_Single_A1(s(3), s(4), s(5)),
        ),
        (
            "vmaxnm.f64 d0, d1, d2",
            I::Vmaxnm_Double_A1(d(0), d(1), d(2)),
        ),
        (
            "vminnm.f64 d3, d4, d5",
            I::Vminnm_Double_A1(d(3), d(4), d(5)),
        ),
        (
            "vrinta.f32 s0, s1",
            I::Vrint_Directed_Single_A1(DRnd::A, s(0), s(1)),
        ),
        (
            "vrintn.f32 s2, s3",
            I::Vrint_Directed_Single_A1(DRnd::N, s(2), s(3)),
        ),
        (
            "vrintp.f32 s4, s5",
            I::Vrint_Directed_Single_A1(DRnd::P, s(4), s(5)),
        ),
        (
            "vrintm.f32 s6, s7",
            I::Vrint_Directed_Single_A1(DRnd::M, s(6), s(7)),
        ),
        (
            "vrinta.f64 d0, d1",
            I::Vrint_Directed_Double_A1(DRnd::A, d(0), d(1)),
        ),
        (
            "vrintr.f32 s0, s1",
            I::Vrint_Cond_Single_A1(al, VRnd::R, s(0), s(1)),
        ),
        (
            "vrintz.f32 s2, s3",
            I::Vrint_Cond_Single_A1(al, VRnd::Z, s(2), s(3)),
        ),
        (
            "vrintx.f32 s4, s5",
            I::Vrint_Cond_Single_A1(al, VRnd::X, s(4), s(5)),
        ),
        (
            "vrintzne.f64 d2, d3",
            I::Vrint_Cond_Double_A1(C::NotEqual, VRnd::Z, d(2), d(3)),
        ),
        (
            "vrintx.f64 d4, d5",
            I::Vrint_Cond_Double_A1(al, VRnd::X, d(4), d(5)),
        ),
        (
            "vcvta.s32.f32 s0, s1",
            I::Vcvt_Directed_FromSingle_A1(DRnd::A, s(0), s(1), true),
        ),
        (
            "vcvtn.s32.f32 s2, s3",
            I::Vcvt_Directed_FromSingle_A1(DRnd::N, s(2), s(3), true),
        ),
        (
            "vcvtp.u32.f32 s4, s5",
            I::Vcvt_Directed_FromSingle_A1(DRnd::P, s(4), s(5), false),
        ),
        (
            "vcvtm.u32.f32 s6, s7",
            I::Vcvt_Directed_FromSingle_A1(DRnd::M, s(6), s(7), false),
        ),
        (
            "vcvta.s32.f64 s0, d1",
            I::Vcvt_Directed_FromDouble_A1(DRnd::A, s(0), d(1), true),
        ),
        (
            "vcvtm.u32.f64 s2, d3",
            I::Vcvt_Directed_FromDouble_A1(DRnd::M, s(2), d(3), false),
        ),
        // -- NEON 3-reg-same-length: integer --
        (
            "vadd.i8 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::Vadd, NSz::I8, d(0), d(1), d(2)),
        ),
        (
            "vadd.i64 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::Vadd, NSz::I64, d(0), d(1), d(2)),
        ),
        (
            "vadd.i32 q0, q1, q2",
            I::NeonInt3Same_Q_A1(NInt::Vadd, NSz::I32, q(0), q(1), q(2)),
        ),
        (
            "vsub.i16 q3, q4, q5",
            I::NeonInt3Same_Q_A1(NInt::Vsub, NSz::I16, q(3), q(4), q(5)),
        ),
        (
            "vqadd.s8 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VqaddS, NSz::I8, d(0), d(1), d(2)),
        ),
        (
            "vqadd.u16 d6, d7, d8",
            I::NeonInt3Same_D_A1(NInt::VqaddU, NSz::I16, d(6), d(7), d(8)),
        ),
        (
            "vqsub.s32 q0, q1, q2",
            I::NeonInt3Same_Q_A1(NInt::VqsubS, NSz::I32, q(0), q(1), q(2)),
        ),
        (
            "vhadd.s8 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VhaddS, NSz::I8, d(0), d(1), d(2)),
        ),
        (
            "vhsub.u16 d3, d4, d5",
            I::NeonInt3Same_D_A1(NInt::VhsubU, NSz::I16, d(3), d(4), d(5)),
        ),
        (
            "vrhadd.u8 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VrhaddU, NSz::I8, d(0), d(1), d(2)),
        ),
        (
            "vtst.16 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::Vtst, NSz::I16, d(0), d(1), d(2)),
        ),
        (
            "vceq.i32 q0, q1, q2",
            I::NeonInt3Same_Q_A1(NInt::Vceq, NSz::I32, q(0), q(1), q(2)),
        ),
        (
            "vmla.i16 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::Vmla, NSz::I16, d(0), d(1), d(2)),
        ),
        (
            "vmls.i8 d3, d4, d5",
            I::NeonInt3Same_D_A1(NInt::Vmls, NSz::I8, d(3), d(4), d(5)),
        ),
        (
            "vmul.i16 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::Vmul, NSz::I16, d(0), d(1), d(2)),
        ),
        (
            "vmul.p8 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VmulPoly, NSz::I8, d(0), d(1), d(2)),
        ),
        (
            "vabd.s8 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VabdS, NSz::I8, d(0), d(1), d(2)),
        ),
        (
            "vaba.u16 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VabaU, NSz::I16, d(0), d(1), d(2)),
        ),
        (
            "vmax.s16 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VmaxS, NSz::I16, d(0), d(1), d(2)),
        ),
        (
            "vmin.u32 q0, q1, q2",
            I::NeonInt3Same_Q_A1(NInt::VminU, NSz::I32, q(0), q(1), q(2)),
        ),
        (
            "vcge.s16 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VcgeS, NSz::I16, d(0), d(1), d(2)),
        ),
        (
            "vcgt.u32 q0, q1, q2",
            I::NeonInt3Same_Q_A1(NInt::VcgtU, NSz::I32, q(0), q(1), q(2)),
        ),
        (
            "vpadd.i8 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::Vpadd, NSz::I8, d(0), d(1), d(2)),
        ),
        (
            "vpmax.s16 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VpmaxS, NSz::I16, d(0), d(1), d(2)),
        ),
        (
            "vpmin.u8 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VpminU, NSz::I8, d(0), d(1), d(2)),
        ),
        (
            "vqdmulh.s16 d0, d1, d2",
            I::NeonInt3Same_D_A1(NInt::VqdmulhS, NSz::I16, d(0), d(1), d(2)),
        ),
        (
            "vqrdmulh.s32 q0, q1, q2",
            I::NeonInt3Same_Q_A1(NInt::VqrdmulhS, NSz::I32, q(0), q(1), q(2)),
        ),
        // -- NEON 3-reg-same-length: float (f32) --
        (
            "vadd.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vadd, d(0), d(1), d(2)),
        ),
        (
            "vsub.f32 q0, q1, q2",
            I::NeonFloat3Same_Q_A1(NFlt::Vsub, q(0), q(1), q(2)),
        ),
        (
            "vmul.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vmul, d(0), d(1), d(2)),
        ),
        (
            "vmla.f32 q0, q1, q2",
            I::NeonFloat3Same_Q_A1(NFlt::Vmla, q(0), q(1), q(2)),
        ),
        (
            "vmls.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vmls, d(0), d(1), d(2)),
        ),
        (
            "vabd.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vabd, d(0), d(1), d(2)),
        ),
        (
            "vpadd.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vpadd, d(0), d(1), d(2)),
        ),
        (
            "vceq.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vceq, d(0), d(1), d(2)),
        ),
        (
            "vcge.f32 q0, q1, q2",
            I::NeonFloat3Same_Q_A1(NFlt::Vcge, q(0), q(1), q(2)),
        ),
        (
            "vcgt.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vcgt, d(0), d(1), d(2)),
        ),
        (
            "vmax.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vmax, d(0), d(1), d(2)),
        ),
        (
            "vmin.f32 q0, q1, q2",
            I::NeonFloat3Same_Q_A1(NFlt::Vmin, q(0), q(1), q(2)),
        ),
        (
            "vpmax.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vpmax, d(0), d(1), d(2)),
        ),
        (
            "vpmin.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vpmin, d(0), d(1), d(2)),
        ),
        (
            "vrecps.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vrecps, d(0), d(1), d(2)),
        ),
        (
            "vrsqrts.f32 q0, q1, q2",
            I::NeonFloat3Same_Q_A1(NFlt::Vrsqrts, q(0), q(1), q(2)),
        ),
        (
            "vfma.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vfma, d(0), d(1), d(2)),
        ),
        (
            "vfms.f32 d0, d1, d2",
            I::NeonFloat3Same_D_A1(NFlt::Vfms, d(0), d(1), d(2)),
        ),
        // -- NEON 3-reg-same-length: bitwise --
        (
            "vand d0, d1, d2",
            I::NeonBitwise3Same_D_A1(NBit::Vand, d(0), d(1), d(2)),
        ),
        (
            "vbic q0, q1, q2",
            I::NeonBitwise3Same_Q_A1(NBit::Vbic, q(0), q(1), q(2)),
        ),
        (
            "vorr d0, d1, d2",
            I::NeonBitwise3Same_D_A1(NBit::Vorr, d(0), d(1), d(2)),
        ),
        (
            "vorn d0, d1, d2",
            I::NeonBitwise3Same_D_A1(NBit::Vorn, d(0), d(1), d(2)),
        ),
        (
            "veor q0, q1, q2",
            I::NeonBitwise3Same_Q_A1(NBit::Veor, q(0), q(1), q(2)),
        ),
        (
            "vbsl d0, d1, d2",
            I::NeonBitwise3Same_D_A1(NBit::Vbsl, d(0), d(1), d(2)),
        ),
        (
            "vbit d0, d1, d2",
            I::NeonBitwise3Same_D_A1(NBit::Vbit, d(0), d(1), d(2)),
        ),
        (
            "vbif q0, q1, q2",
            I::NeonBitwise3Same_Q_A1(NBit::Vbif, q(0), q(1), q(2)),
        ),
        // -- NEON 2-reg-misc: element-sized --
        (
            "vrev64.8 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::Vrev64, NSz::I8, d(0), d(1)),
        ),
        (
            "vrev64.32 q0, q1",
            I::NeonMisc2Sized_Q_A1(NMS::Vrev64, NSz::I32, q(0), q(1)),
        ),
        (
            "vrev32.16 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::Vrev32, NSz::I16, d(0), d(1)),
        ),
        (
            "vrev16.8 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::Vrev16, NSz::I8, d(0), d(1)),
        ),
        (
            "vpaddl.s8 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::VpaddlS, NSz::I8, d(0), d(1)),
        ),
        (
            "vpaddl.u16 q0, q1",
            I::NeonMisc2Sized_Q_A1(NMS::VpaddlU, NSz::I16, q(0), q(1)),
        ),
        (
            "vpadal.s32 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::VpadalS, NSz::I32, d(0), d(1)),
        ),
        (
            "vcls.s8 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::VclsS, NSz::I8, d(0), d(1)),
        ),
        (
            "vclz.i32 q0, q1",
            I::NeonMisc2Sized_Q_A1(NMS::VclzI, NSz::I32, q(0), q(1)),
        ),
        (
            "vqabs.s16 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::VqabsS, NSz::I16, d(0), d(1)),
        ),
        (
            "vqneg.s8 q0, q1",
            I::NeonMisc2Sized_Q_A1(NMS::VqnegS, NSz::I8, q(0), q(1)),
        ),
        (
            "vcgt.s8 d0, d1, #0",
            I::NeonMisc2Sized_D_A1(NMS::VcgtZeroS, NSz::I8, d(0), d(1)),
        ),
        (
            "vcge.s16 d0, d1, #0",
            I::NeonMisc2Sized_D_A1(NMS::VcgeZeroS, NSz::I16, d(0), d(1)),
        ),
        (
            "vceq.i32 q0, q1, #0",
            I::NeonMisc2Sized_Q_A1(NMS::VceqZeroI, NSz::I32, q(0), q(1)),
        ),
        (
            "vcle.s8 d0, d1, #0",
            I::NeonMisc2Sized_D_A1(NMS::VcleZeroS, NSz::I8, d(0), d(1)),
        ),
        (
            "vclt.s16 d0, d1, #0",
            I::NeonMisc2Sized_D_A1(NMS::VcltZeroS, NSz::I16, d(0), d(1)),
        ),
        (
            "vabs.s32 q0, q1",
            I::NeonMisc2Sized_Q_A1(NMS::VabsS, NSz::I32, q(0), q(1)),
        ),
        (
            "vneg.s8 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::VnegS, NSz::I8, d(0), d(1)),
        ),
        (
            "vtrn.8 d0, d1",
            I::NeonMisc2Sized_D_A1(NMS::Vtrn, NSz::I8, d(0), d(1)),
        ),
        (
            "vuzp.16 q0, q1",
            I::NeonMisc2Sized_Q_A1(NMS::Vuzp, NSz::I16, q(0), q(1)),
        ),
        (
            "vzip.32 q0, q1",
            I::NeonMisc2Sized_Q_A1(NMS::Vzip, NSz::I32, q(0), q(1)),
        ),
        // -- NEON 2-reg-misc: fixed-size --
        ("vmvn d0, d1", I::NeonMisc2Fixed_D_A1(NMF::Vmvn, d(0), d(1))),
        ("vmvn q0, q1", I::NeonMisc2Fixed_Q_A1(NMF::Vmvn, q(0), q(1))),
        ("vswp d0, d1", I::NeonMisc2Fixed_D_A1(NMF::Vswp, d(0), d(1))),
        (
            "vcnt.8 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::Vcnt, d(0), d(1)),
        ),
        (
            "vcgt.f32 d0, d1, #0",
            I::NeonMisc2Fixed_D_A1(NMF::VcgtZeroF, d(0), d(1)),
        ),
        (
            "vcge.f32 q0, q1, #0",
            I::NeonMisc2Fixed_Q_A1(NMF::VcgeZeroF, q(0), q(1)),
        ),
        (
            "vceq.f32 d0, d1, #0",
            I::NeonMisc2Fixed_D_A1(NMF::VceqZeroF, d(0), d(1)),
        ),
        (
            "vcle.f32 d0, d1, #0",
            I::NeonMisc2Fixed_D_A1(NMF::VcleZeroF, d(0), d(1)),
        ),
        (
            "vclt.f32 d0, d1, #0",
            I::NeonMisc2Fixed_D_A1(NMF::VcltZeroF, d(0), d(1)),
        ),
        (
            "vabs.f32 q0, q1",
            I::NeonMisc2Fixed_Q_A1(NMF::VabsF, q(0), q(1)),
        ),
        (
            "vneg.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VnegF, d(0), d(1)),
        ),
        (
            "vrecpe.u32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VrecpeU, d(0), d(1)),
        ),
        (
            "vrsqrte.u32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VrsqrteU, d(0), d(1)),
        ),
        (
            "vrecpe.f32 q0, q1",
            I::NeonMisc2Fixed_Q_A1(NMF::VrecpeF, q(0), q(1)),
        ),
        (
            "vrsqrte.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VrsqrteF, d(0), d(1)),
        ),
        (
            "vcvt.f32.s32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VcvtF32FromS32, d(0), d(1)),
        ),
        (
            "vcvt.f32.u32 q0, q1",
            I::NeonMisc2Fixed_Q_A1(NMF::VcvtF32FromU32, q(0), q(1)),
        ),
        (
            "vcvt.s32.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VcvtS32FromF32, d(0), d(1)),
        ),
        (
            "vcvt.u32.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VcvtU32FromF32, d(0), d(1)),
        ),
        // -- NEON 2-reg-misc: v8 round-to-integral + anchored convert --
        (
            "vrintn.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VrintN, d(0), d(1)),
        ),
        (
            "vrinta.f32 q0, q1",
            I::NeonMisc2Fixed_Q_A1(NMF::VrintA, q(0), q(1)),
        ),
        (
            "vrintp.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VrintP, d(0), d(1)),
        ),
        (
            "vrintm.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VrintM, d(0), d(1)),
        ),
        (
            "vrintx.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VrintX, d(0), d(1)),
        ),
        (
            "vrintz.f32 q0, q1",
            I::NeonMisc2Fixed_Q_A1(NMF::VrintZ, q(0), q(1)),
        ),
        (
            "vcvta.s32.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VcvtaS, d(0), d(1)),
        ),
        (
            "vcvtn.u32.f32 q0, q1",
            I::NeonMisc2Fixed_Q_A1(NMF::VcvtnU, q(0), q(1)),
        ),
        (
            "vcvtp.s32.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VcvtpS, d(0), d(1)),
        ),
        (
            "vcvtm.u32.f32 d0, d1",
            I::NeonMisc2Fixed_D_A1(NMF::VcvtmU, d(0), d(1)),
        ),
        // -- NEON 2-reg-misc: narrowing + VSHLL(max) --
        (
            "vmovn.i16 d0, q1",
            I::NeonMisc2Narrow_A1(NMN::Vmovn, NSz::I16, d(0), q(1)),
        ),
        (
            "vmovn.i32 d0, q1",
            I::NeonMisc2Narrow_A1(NMN::Vmovn, NSz::I32, d(0), q(1)),
        ),
        (
            "vqmovn.s32 d0, q1",
            I::NeonMisc2Narrow_A1(NMN::VqmovnS, NSz::I32, d(0), q(1)),
        ),
        (
            "vqmovn.u64 d0, q1",
            I::NeonMisc2Narrow_A1(NMN::VqmovnU, NSz::I64, d(0), q(1)),
        ),
        (
            "vqmovun.s16 d0, q1",
            I::NeonMisc2Narrow_A1(NMN::Vqmovun, NSz::I16, d(0), q(1)),
        ),
        (
            "vshll.i8 q0, d1, #8",
            I::NeonShllMax_A1(NSz::I8, q(0), d(1)),
        ),
        (
            "vshll.i16 q0, d1, #16",
            I::NeonShllMax_A1(NSz::I16, q(0), d(1)),
        ),
        (
            "vshll.i32 q0, d1, #32",
            I::NeonShllMax_A1(NSz::I32, q(0), d(1)),
        ),
        // -- NEON 3-reg-different: long (Qd <- Dn, Dm) --
        (
            "vaddl.s8 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VaddlS, NSz::I8, q(0), d(1), d(2)),
        ),
        (
            "vaddl.u16 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VaddlU, NSz::I16, q(0), d(1), d(2)),
        ),
        (
            "vsubl.s32 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VsublS, NSz::I32, q(0), d(1), d(2)),
        ),
        (
            "vabal.s8 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VabalS, NSz::I8, q(0), d(1), d(2)),
        ),
        (
            "vabdl.u16 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VabdlU, NSz::I16, q(0), d(1), d(2)),
        ),
        (
            "vmlal.s8 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VmlalS, NSz::I8, q(0), d(1), d(2)),
        ),
        (
            "vmlsl.u16 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VmlslU, NSz::I16, q(0), d(1), d(2)),
        ),
        (
            "vmull.s8 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VmullS, NSz::I8, q(0), d(1), d(2)),
        ),
        (
            "vmull.u32 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VmullU, NSz::I32, q(0), d(1), d(2)),
        ),
        (
            "vmull.p8 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VmullP, NSz::I8, q(0), d(1), d(2)),
        ),
        (
            "vqdmlal.s16 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::Vqdmlal, NSz::I16, q(0), d(1), d(2)),
        ),
        (
            "vqdmlsl.s32 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::Vqdmlsl, NSz::I32, q(0), d(1), d(2)),
        ),
        (
            "vqdmull.s16 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::Vqdmull, NSz::I16, q(0), d(1), d(2)),
        ),
        // -- NEON 3-reg-different: wide (Qd <- Qn, Dm) --
        (
            "vaddw.s8 q0, q1, d2",
            I::NeonDiffWide_A1(NDW::VaddwS, NSz::I8, q(0), q(1), d(2)),
        ),
        (
            "vsubw.u16 q0, q1, d2",
            I::NeonDiffWide_A1(NDW::VsubwU, NSz::I16, q(0), q(1), d(2)),
        ),
        // -- NEON 3-reg-different: narrowing high-half (Dd <- Qn, Qm) --
        (
            "vaddhn.i16 d0, q1, q2",
            I::NeonDiffNarrow_A1(NDN::Vaddhn, NSz::I16, d(0), q(1), q(2)),
        ),
        (
            "vraddhn.i32 d0, q1, q2",
            I::NeonDiffNarrow_A1(NDN::Vraddhn, NSz::I32, d(0), q(1), q(2)),
        ),
        (
            "vsubhn.i64 d0, q1, q2",
            I::NeonDiffNarrow_A1(NDN::Vsubhn, NSz::I64, d(0), q(1), q(2)),
        ),
        (
            "vrsubhn.i16 d0, q1, q2",
            I::NeonDiffNarrow_A1(NDN::Vrsubhn, NSz::I16, d(0), q(1), q(2)),
        ),
        // -- NEON 2-reg-and-a-scalar: same length --
        (
            "vmul.i16 d0, d1, d2[0]",
            I::NeonScalar_D_A1(NSc::Vmul, NSz::I16, d(0), d(1), d(2), 0),
        ),
        (
            "vmul.i32 q0, q1, d2[1]",
            I::NeonScalar_Q_A1(NSc::Vmul, NSz::I32, q(0), q(1), d(2), 1),
        ),
        (
            "vmul.f32 d0, d1, d2[0]",
            I::NeonScalar_D_A1(NSc::VmulF, NSz::I32, d(0), d(1), d(2), 0),
        ),
        (
            "vmul.f32 q0, q1, d2[1]",
            I::NeonScalar_Q_A1(NSc::VmulF, NSz::I32, q(0), q(1), d(2), 1),
        ),
        (
            "vmla.i16 d0, d1, d2[2]",
            I::NeonScalar_D_A1(NSc::Vmla, NSz::I16, d(0), d(1), d(2), 2),
        ),
        (
            "vmla.i32 q0, q1, d2[0]",
            I::NeonScalar_Q_A1(NSc::Vmla, NSz::I32, q(0), q(1), d(2), 0),
        ),
        (
            "vmla.f32 d0, d1, d2[1]",
            I::NeonScalar_D_A1(NSc::VmlaF, NSz::I32, d(0), d(1), d(2), 1),
        ),
        (
            "vmls.i16 d0, d1, d3[3]",
            I::NeonScalar_D_A1(NSc::Vmls, NSz::I16, d(0), d(1), d(3), 3),
        ),
        (
            "vmls.f32 q0, q1, d2[0]",
            I::NeonScalar_Q_A1(NSc::VmlsF, NSz::I32, q(0), q(1), d(2), 0),
        ),
        (
            "vqdmulh.s16 d0, d1, d2[0]",
            I::NeonScalar_D_A1(NSc::Vqdmulh, NSz::I16, d(0), d(1), d(2), 0),
        ),
        (
            "vqdmulh.s32 q0, q1, d2[1]",
            I::NeonScalar_Q_A1(NSc::Vqdmulh, NSz::I32, q(0), q(1), d(2), 1),
        ),
        (
            "vqrdmulh.s16 d0, d1, d2[2]",
            I::NeonScalar_D_A1(NSc::Vqrdmulh, NSz::I16, d(0), d(1), d(2), 2),
        ),
        // -- NEON 2-reg-and-a-scalar: long --
        (
            "vmull.s16 q0, d1, d2[0]",
            I::NeonScalarLong_A1(NScL::VmullS, NSz::I16, q(0), d(1), d(2), 0),
        ),
        (
            "vmull.u32 q0, d1, d2[1]",
            I::NeonScalarLong_A1(NScL::VmullU, NSz::I32, q(0), d(1), d(2), 1),
        ),
        (
            "vmlal.s16 q0, d1, d2[3]",
            I::NeonScalarLong_A1(NScL::VmlalS, NSz::I16, q(0), d(1), d(2), 3),
        ),
        (
            "vmlsl.s32 q0, d1, d2[0]",
            I::NeonScalarLong_A1(NScL::VmlslS, NSz::I32, q(0), d(1), d(2), 0),
        ),
        (
            "vqdmull.s16 q0, d1, d2[1]",
            I::NeonScalarLong_A1(NScL::Vqdmull, NSz::I16, q(0), d(1), d(2), 1),
        ),
        (
            "vqdmlal.s32 q0, d1, d2[0]",
            I::NeonScalarLong_A1(NScL::Vqdmlal, NSz::I32, q(0), d(1), d(2), 0),
        ),
        (
            "vqdmlsl.s16 q0, d1, d2[2]",
            I::NeonScalarLong_A1(NScL::Vqdmlsl, NSz::I16, q(0), d(1), d(2), 2),
        ),
        // -- NEON 2-reg-and-a-shift-amount: same width, right shifts --
        (
            "vshr.s8 d0, d1, #1",
            I::NeonShift_D_A1(NSh::VshrS, NSz::I8, 1, d(0), d(1)),
        ),
        (
            "vshr.u16 d0, d1, #3",
            I::NeonShift_D_A1(NSh::VshrU, NSz::I16, 3, d(0), d(1)),
        ),
        (
            "vshr.s32 q0, q1, #5",
            I::NeonShift_Q_A1(NSh::VshrS, NSz::I32, 5, q(0), q(1)),
        ),
        (
            "vshr.u64 d0, d1, #7",
            I::NeonShift_D_A1(NSh::VshrU, NSz::I64, 7, d(0), d(1)),
        ),
        (
            "vsra.s8 d0, d1, #2",
            I::NeonShift_D_A1(NSh::VsraS, NSz::I8, 2, d(0), d(1)),
        ),
        (
            "vrshr.u16 d0, d1, #4",
            I::NeonShift_D_A1(NSh::VrshrU, NSz::I16, 4, d(0), d(1)),
        ),
        (
            "vrsra.s32 q0, q1, #6",
            I::NeonShift_Q_A1(NSh::VrsraS, NSz::I32, 6, q(0), q(1)),
        ),
        (
            "vsri.8 d0, d1, #1",
            I::NeonShift_D_A1(NSh::Vsri, NSz::I8, 1, d(0), d(1)),
        ),
        (
            "vsri.64 q0, q1, #40",
            I::NeonShift_Q_A1(NSh::Vsri, NSz::I64, 40, q(0), q(1)),
        ),
        // -- same width, left shifts --
        (
            "vshl.i8 d0, d1, #1",
            I::NeonShift_D_A1(NSh::Vshl, NSz::I8, 1, d(0), d(1)),
        ),
        (
            "vshl.i32 q0, q1, #5",
            I::NeonShift_Q_A1(NSh::Vshl, NSz::I32, 5, q(0), q(1)),
        ),
        (
            "vsli.16 d0, d1, #3",
            I::NeonShift_D_A1(NSh::Vsli, NSz::I16, 3, d(0), d(1)),
        ),
        (
            "vqshl.s8 d0, d1, #1",
            I::NeonShift_D_A1(NSh::VqshlS, NSz::I8, 1, d(0), d(1)),
        ),
        (
            "vqshl.u16 q0, q1, #2",
            I::NeonShift_Q_A1(NSh::VqshlU, NSz::I16, 2, q(0), q(1)),
        ),
        (
            "vqshlu.s32 d0, d1, #4",
            I::NeonShift_D_A1(NSh::Vqshlu, NSz::I32, 4, d(0), d(1)),
        ),
        // -- narrowing (Dd <- Qm) --
        (
            "vshrn.i16 d0, q1, #2",
            I::NeonShiftNarrow_A1(NShN::Vshrn, NSz::I16, 2, d(0), q(1)),
        ),
        (
            "vrshrn.i32 d0, q1, #4",
            I::NeonShiftNarrow_A1(NShN::Vrshrn, NSz::I32, 4, d(0), q(1)),
        ),
        (
            "vqshrn.s32 d0, q1, #3",
            I::NeonShiftNarrow_A1(NShN::VqshrnS, NSz::I32, 3, d(0), q(1)),
        ),
        (
            "vqrshrn.u64 d0, q1, #5",
            I::NeonShiftNarrow_A1(NShN::VqrshrnU, NSz::I64, 5, d(0), q(1)),
        ),
        (
            "vqshrun.s16 d0, q1, #1",
            I::NeonShiftNarrow_A1(NShN::Vqshrun, NSz::I16, 1, d(0), q(1)),
        ),
        (
            "vqrshrun.s32 d0, q1, #6",
            I::NeonShiftNarrow_A1(NShN::Vqrshrun, NSz::I32, 6, d(0), q(1)),
        ),
        // -- widening VSHLL / VMOVL (Qd <- Dm) --
        (
            "vshll.s8 q0, d1, #3",
            I::NeonShiftLong_A1(false, NSz::I8, 3, q(0), d(1)),
        ),
        (
            "vshll.u16 q0, d1, #5",
            I::NeonShiftLong_A1(true, NSz::I16, 5, q(0), d(1)),
        ),
        (
            "vmovl.s8 q0, d1",
            I::NeonShiftLong_A1(false, NSz::I8, 0, q(0), d(1)),
        ),
        (
            "vmovl.u16 q0, d1",
            I::NeonShiftLong_A1(true, NSz::I16, 0, q(0), d(1)),
        ),
        (
            "vmovl.s32 q0, d1",
            I::NeonShiftLong_A1(false, NSz::I32, 0, q(0), d(1)),
        ),
        // -- NEON VEXT (byte extract) --
        (
            "vext.8 d0, d1, d2, #3",
            I::NeonExt_D_A1(3, d(0), d(1), d(2)),
        ),
        (
            "vext.8 q0, q1, q2, #5",
            I::NeonExt_Q_A1(5, q(0), q(1), q(2)),
        ),
        (
            "vext.8 q3, q4, q5, #15",
            I::NeonExt_Q_A1(15, q(3), q(4), q(5)),
        ),
        // -- NEON VTBL / VTBX --
        (
            "vtbl.8 d0, {d1}, d2",
            I::NeonTableLookup_A1(false, 1, d(0), d(1), d(2)),
        ),
        (
            "vtbl.8 d0, {d1, d2}, d3",
            I::NeonTableLookup_A1(false, 2, d(0), d(1), d(3)),
        ),
        (
            "vtbl.8 d0, {d1, d2, d3}, d4",
            I::NeonTableLookup_A1(false, 3, d(0), d(1), d(4)),
        ),
        (
            "vtbl.8 d0, {d1, d2, d3, d4}, d5",
            I::NeonTableLookup_A1(false, 4, d(0), d(1), d(5)),
        ),
        (
            "vtbx.8 d0, {d2}, d3",
            I::NeonTableLookup_A1(true, 1, d(0), d(2), d(3)),
        ),
        (
            "vtbx.8 d0, {d2, d3, d4}, d5",
            I::NeonTableLookup_A1(true, 3, d(0), d(2), d(5)),
        ),
        // -- NEON VDUP (scalar) --
        (
            "vdup.8 d0, d1[3]",
            I::NeonVdupScalar_D_A1(NSz::I8, 3, d(0), d(1)),
        ),
        (
            "vdup.16 q0, d1[2]",
            I::NeonVdupScalar_Q_A1(NSz::I16, 2, q(0), d(1)),
        ),
        (
            "vdup.32 d0, d1[1]",
            I::NeonVdupScalar_D_A1(NSz::I32, 1, d(0), d(1)),
        ),
        // -- NEON VDUP (from ARM core register) --
        (
            "vdup.8 d0, r1",
            I::NeonVdupCore_D_A1(al, NSz::I8, d(0), R::R1),
        ),
        (
            "vdup.16 q0, r2",
            I::NeonVdupCore_Q_A1(al, NSz::I16, q(0), R::R2),
        ),
        (
            "vdup.32 d0, r3",
            I::NeonVdupCore_D_A1(al, NSz::I32, d(0), R::R3),
        ),
        // -- NEON VMOV / VMVN / VORR / VBIC (modified immediate) --
        (
            "vmov.i32 d0, #1",
            I::NeonModifiedImmediate_D_A1(0b0000, false, 1, d(0)),
        ),
        (
            "vmov.i32 q0, #0xff",
            I::NeonModifiedImmediate_Q_A1(0b0000, false, 0xff, q(0)),
        ),
        (
            "vmov.i16 d0, #0x100",
            I::NeonModifiedImmediate_D_A1(0b1010, false, 1, d(0)),
        ),
        (
            "vmov.i8 d0, #0x55",
            I::NeonModifiedImmediate_D_A1(0b1110, false, 0x55, d(0)),
        ),
        (
            "vmov.i32 d0, #0xff00",
            I::NeonModifiedImmediate_D_A1(0b0010, false, 0xff, d(0)),
        ),
        (
            "vmov.i32 d0, #0xff0000",
            I::NeonModifiedImmediate_D_A1(0b0100, false, 0xff, d(0)),
        ),
        (
            "vmov.i32 d0, #0xff000000",
            I::NeonModifiedImmediate_D_A1(0b0110, false, 0xff, d(0)),
        ),
        (
            "vmov.i32 d0, #0xffff",
            I::NeonModifiedImmediate_D_A1(0b1100, false, 0xff, d(0)),
        ),
        (
            "vmov.i32 d0, #0xffffff",
            I::NeonModifiedImmediate_D_A1(0b1101, false, 0xff, d(0)),
        ),
        (
            "vmov.i64 d0, #0xff00ff00ff00ff00",
            I::NeonModifiedImmediate_D_A1(0b1110, true, 0xAA, d(0)),
        ),
        (
            "vmov.f32 q0, #1.5",
            I::NeonModifiedImmediate_Q_A1(0b1111, false, 0x78, q(0)),
        ),
        (
            "vmvn.i32 d0, #1",
            I::NeonModifiedImmediate_D_A1(0b0000, true, 1, d(0)),
        ),
        (
            "vmvn.i16 d0, #0x100",
            I::NeonModifiedImmediate_D_A1(0b1010, true, 1, d(0)),
        ),
        (
            "vorr.i32 d0, #0x100",
            I::NeonModifiedImmediate_D_A1(0b0011, false, 1, d(0)),
        ),
        (
            "vbic.i16 q0, #0xff00",
            I::NeonModifiedImmediate_Q_A1(0b1011, true, 0xff, q(0)),
        ),
        // -- NEON element/structure load & store: multiple-element, plain [Rn] --
        (
            "vld1.8 {d0}, [r0]",
            I::NeonLoadStoreMultiple_A1(true, 0b0111, NSz::I8, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld1.8 {d0, d1}, [r0]",
            I::NeonLoadStoreMultiple_A1(true, 0b1010, NSz::I8, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld1.8 {d0, d1, d2}, [r0]",
            I::NeonLoadStoreMultiple_A1(true, 0b0110, NSz::I8, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld1.8 {d0, d1, d2, d3}, [r0]",
            I::NeonLoadStoreMultiple_A1(true, 0b0010, NSz::I8, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld2.8 {d0, d1}, [r0]",
            I::NeonLoadStoreMultiple_A1(true, 0b1000, NSz::I8, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld2.8 {d0, d1, d2, d3}, [r0]",
            I::NeonLoadStoreMultiple_A1(true, 0b0011, NSz::I8, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld3.8 {d0, d1, d2}, [r0]",
            I::NeonLoadStoreMultiple_A1(true, 0b0100, NSz::I8, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld4.8 {d0, d1, d2, d3}, [r0]",
            I::NeonLoadStoreMultiple_A1(true, 0b0000, NSz::I8, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vst1.16 {d0}, [r0]",
            I::NeonLoadStoreMultiple_A1(false, 0b0111, NSz::I16, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vst2.32 {d0, d1}, [r0]",
            I::NeonLoadStoreMultiple_A1(false, 0b1000, NSz::I32, 0, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vst4.16 {d0, d1, d2, d3}, [r0]",
            I::NeonLoadStoreMultiple_A1(false, 0b0000, NSz::I16, 0, d(0), R::R0, NLsa::Offset),
        ),
        // -- alignment + writeback + register post-index --
        (
            "vld1.8 {d0}, [r0:64]",
            I::NeonLoadStoreMultiple_A1(true, 0b0111, NSz::I8, 1, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld1.8 {d0}, [r0]!",
            I::NeonLoadStoreMultiple_A1(
                true,
                0b0111,
                NSz::I8,
                0,
                d(0),
                R::R0,
                NLsa::IncrementWriteback,
            ),
        ),
        (
            "vld1.8 {d0}, [r0], r2",
            I::NeonLoadStoreMultiple_A1(
                true,
                0b0111,
                NSz::I8,
                0,
                d(0),
                R::R0,
                NLsa::PostIndexed(R::R2),
            ),
        ),
        (
            "vld2.8 {d0, d1}, [r0:128]!",
            I::NeonLoadStoreMultiple_A1(
                true,
                0b1000,
                NSz::I8,
                2,
                d(0),
                R::R0,
                NLsa::IncrementWriteback,
            ),
        ),
        // -- single element to one lane --
        (
            "vld1.8 {d0[3]}, [r0]",
            I::NeonLoadStoreSingleLane_A1(true, 1, 0, 6, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld1.16 {d0[2]}, [r0]",
            I::NeonLoadStoreSingleLane_A1(true, 1, 1, 8, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld1.32 {d0[1]}, [r0]",
            I::NeonLoadStoreSingleLane_A1(true, 1, 2, 8, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vst1.32 {d0[1]}, [r0]",
            I::NeonLoadStoreSingleLane_A1(false, 1, 2, 8, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld2.16 {d0[1], d1[1]}, [r0]",
            I::NeonLoadStoreSingleLane_A1(true, 2, 1, 4, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld3.8 {d0[2], d1[2], d2[2]}, [r0]",
            I::NeonLoadStoreSingleLane_A1(true, 3, 0, 4, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld4.32 {d0[0], d1[0], d2[0], d3[0]}, [r0]",
            I::NeonLoadStoreSingleLane_A1(true, 4, 2, 0, d(0), R::R0, NLsa::Offset),
        ),
        // -- single element to all lanes --
        (
            "vld1.8 {d0[]}, [r0]",
            I::NeonLoadStoreAllLanes_A1(1, 0, false, false, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld2.16 {d0[], d1[]}, [r0]",
            I::NeonLoadStoreAllLanes_A1(2, 1, false, false, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld3.32 {d0[], d1[], d2[]}, [r0]",
            I::NeonLoadStoreAllLanes_A1(3, 2, false, false, d(0), R::R0, NLsa::Offset),
        ),
        (
            "vld4.8 {d0[], d1[], d2[], d3[]}, [r0]",
            I::NeonLoadStoreAllLanes_A1(4, 0, false, false, d(0), R::R0, NLsa::Offset),
        ),
        // -- ARMv8 cryptography extension --
        ("aese.8 q0, q1", I::NeonAes_A1(NAes::Aese, q(0), q(1))),
        ("aesd.8 q2, q3", I::NeonAes_A1(NAes::Aesd, q(2), q(3))),
        ("aesmc.8 q0, q1", I::NeonAes_A1(NAes::Aesmc, q(0), q(1))),
        ("aesimc.8 q4, q5", I::NeonAes_A1(NAes::Aesimc, q(4), q(5))),
        (
            "sha1c.32 q0, q1, q2",
            I::NeonSha3Reg_A1(NSha3::Sha1c, q(0), q(1), q(2)),
        ),
        (
            "sha1p.32 q3, q4, q5",
            I::NeonSha3Reg_A1(NSha3::Sha1p, q(3), q(4), q(5)),
        ),
        (
            "sha1m.32 q0, q1, q2",
            I::NeonSha3Reg_A1(NSha3::Sha1m, q(0), q(1), q(2)),
        ),
        (
            "sha1su0.32 q0, q1, q2",
            I::NeonSha3Reg_A1(NSha3::Sha1su0, q(0), q(1), q(2)),
        ),
        (
            "sha256h.32 q0, q1, q2",
            I::NeonSha3Reg_A1(NSha3::Sha256h, q(0), q(1), q(2)),
        ),
        (
            "sha256h2.32 q3, q4, q5",
            I::NeonSha3Reg_A1(NSha3::Sha256h2, q(3), q(4), q(5)),
        ),
        (
            "sha256su1.32 q0, q1, q2",
            I::NeonSha3Reg_A1(NSha3::Sha256su1, q(0), q(1), q(2)),
        ),
        (
            "sha1h.32 q0, q1",
            I::NeonSha2Reg_A1(NSha2::Sha1h, q(0), q(1)),
        ),
        (
            "sha1su1.32 q2, q3",
            I::NeonSha2Reg_A1(NSha2::Sha1su1, q(2), q(3)),
        ),
        (
            "sha256su0.32 q0, q1",
            I::NeonSha2Reg_A1(NSha2::Sha256su0, q(0), q(1)),
        ),
        (
            "vmull.p64 q0, d1, d2",
            I::NeonDiffLong_A1(NDL::VmullP, NSz::I32, q(0), d(1), d(2)),
        ),
        // -- banked MRS / MSR (ARMv7VE) --
        ("mrs r0, SP_usr", I::MrsBanked_A1(al, false, 5, R::R0)),
        ("mrs r1, LR_irq", I::MrsBanked_A1(al, false, 16, R::R1)),
        ("mrs r2, SPSR_hyp", I::MrsBanked_A1(al, true, 30, R::R2)),
        ("mrs r3, ELR_hyp", I::MrsBanked_A1(al, false, 30, R::R3)),
        ("mrs r4, SP_hyp", I::MrsBanked_A1(al, false, 31, R::R4)),
        ("mrs r5, LR_usr", I::MrsBanked_A1(al, false, 6, R::R5)),
        ("msr SP_usr, r0", I::MsrBanked_A1(al, false, 5, R::R0)),
        ("msr LR_irq, r1", I::MsrBanked_A1(al, false, 16, R::R1)),
        ("msr SPSR_hyp, r2", I::MsrBanked_A1(al, true, 30, R::R2)),
        ("msr ELR_hyp, r3", I::MsrBanked_A1(al, false, 30, R::R3)),
    ]
}

// ---- minimal GNU backend (ARM mode) ----

struct GnuBackend {
    assembler: String,
    objcopy: String,
}

impl GnuBackend {
    fn assemble_and_extract(&self, source: &str) -> Vec<u8> {
        let work = WorkDir::new("arm32_oracle_a32");
        let source_path = work.path.join("oracle.s");
        let object_path = work.path.join("oracle.o");
        let binary_path = work.path.join("oracle.bin");
        std::fs::write(&source_path, source).expect("write .s");

        run(
            Command::new(&self.assembler)
                // +simd: Advanced SIMD + VFP; +crypto: AES/SHA/VMULL.p64. v8.3-a (a superset of v8.0-a) + fp16 + ras
                // + sb so the FEAT_FP16 (VCVTB/T half<->double), FEAT_JSCVT (VJCVT), RAS (ESB) and FEAT_SB (SB) samples assemble.
                .args(["-march=armv8.3-a+crc+simd+crypto+fp16+ras+sb"])
                .arg(&source_path)
                .arg("-o")
                .arg(&object_path),
            source,
        );
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

fn discover_gnu_backend() -> Option<GnuBackend> {
    let roots = gnu_toolchain_search_roots();
    match (
        locate(&["ARM_NONE_EABI_AS"], &["arm-none-eabi-as"], &roots),
        locate(
            &["ARM_NONE_EABI_OBJCOPY"],
            &["arm-none-eabi-objcopy"],
            &roots,
        ),
    ) {
        (Some(assembler), Some(objcopy)) => Some(GnuBackend { assembler, objcopy }),
        _ => None,
    }
}

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
            for candidate in [root.join(name), root.join(format!("{}.exe", name))] {
                if candidate.is_file() {
                    return Some(candidate.to_string_lossy().into_owned());
                }
            }
        }
    }
    None
}

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

// ---- minimal LLVM backend (ARM mode, `llvm-mc`) -- the second oracle ----
//
// Assembles ONE ARM-state instruction at a time (vs GNU's single batch) so that a form llvm-mc refuses isolates
// to a `None`/skip instead of misaligning the 4-bytes-per-instruction stream. ARMv8-A triple with the optional
// CRC + crypto features the sample set exercises (AES/SHA/VMULL.p64). Discovery is independent + optional
// (env overrides `LLVM_MC` / `LLVM_OBJCOPY`); `llvm-mc`/`llvm-objcopy` are normally on PATH.

struct LlvmBackend {
    mc: String,
    objcopy: String,
}
impl LlvmBackend {
    fn assemble_one(&self, ual: &str) -> Option<Vec<u8>> {
        let work = WorkDir::new("arm32_oracle_a32_llvm");
        let source = work.path.join("one.s");
        let object = work.path.join("one.o");
        let binary = work.path.join("one.bin");
        std::fs::write(&source, format!(".syntax unified\n.arm\n.text\n{ual}\n")).ok()?;

        let assembled = Command::new(&self.mc)
            .args([
                "-triple=armv8.3a",
                "-mattr=+crc,+aes,+sha2,+fullfp16,+ras,+sb",
                "-filetype=obj",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&object)
            .output()
            .ok()?;
        if !assembled.status.success() {
            return None; // llvm-mc refuses this form (e.g. a coprocessor deprecated in ARMv8-A)
        }
        let extracted = Command::new(&self.objcopy)
            .args(["-O", "binary", "-j", ".text"])
            .arg(&object)
            .arg(&binary)
            .output()
            .ok()?;
        if !extracted.status.success() {
            return None;
        }
        let bytes = std::fs::read(&binary).ok()?;
        (bytes.len() == 4).then_some(bytes)
    }
}

fn discover_llvm_backend() -> Option<LlvmBackend> {
    match (
        locate(&["LLVM_MC"], &["llvm-mc"], &[]),
        locate(&["LLVM_OBJCOPY"], &["llvm-objcopy"], &[]),
    ) {
        (Some(mc), Some(objcopy)) => Some(LlvmBackend { mc, objcopy }),
        _ => None,
    }
}
