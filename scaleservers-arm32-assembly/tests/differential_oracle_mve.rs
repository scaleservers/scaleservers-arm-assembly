// Copyright (c) Scaleservers LLC

// Differential oracle for the ARMv8.1-M MVE ("Helium") instructions against GNU
// (`arm-none-eabi-as -march=armv8.1-m.main+mve.fp`, Thumb mode -- i.e. a Cortex-M55/M85 vector core).
// MVE shares the FP encoding space, so the scalar-T32 `differential_oracle_t32.rs` (which targets a
// Cortex-M7 without MVE) cannot cover it; this is a separate harness exactly like `differential_oracle_a32.rs`.
//
//   ENCODER check:  GNU's bytes for the hand-written UAL must equal our `encode()` (4 bytes / instruction).
//   DECODER check:  those bytes --(our decode)--> model   must equal the original sample instruction.
//   EMITTER check:  our `to_assembly_string()` re-assembled by GNU must produce the same bytes.
//
// If no GNU arm-none-eabi assembler is found the tests SKIP green.

use std::path::PathBuf;
use std::process::Command;
mod common;

use scaleservers_arm32_assembly::ArmAssemblySyntax;
use scaleservers_arm32_assembly::ArmT32Instruction as I;
use scaleservers_arm32_assembly::{
    Arm32DirectedRound as DR, Arm32DoublePrecisionRegister as D, Arm32GeneralPurposeRegister as R,
    Arm32MveBitwiseOp as Bop, Arm32MveFloatArithOp as Fop, Arm32MveFloatReduceOp as FRop,
    Arm32MveFloatSize as FSize, Arm32MveIntArithOp as Iop, Arm32MveLongMacOp as LMop,
    Arm32MveMisc2FloatOp as M2Fop, Arm32MveMisc2Op as M2op, Arm32MveQMovnKind as QMov,
    Arm32MveReduceOp as RDop, Arm32MveShiftImmOp as SHop, Arm32MveSize as Size,
    Arm32MveVcmpCondition as CC, Arm32MveVecScalarFloatOp as VSFop,
    Arm32MveVecScalarIntOp as VSIop, Arm32MveVectorRegister, Arm32MveVrintOp as RIop,
    Arm32SinglePrecisionRegister as S, Arm32VmovLaneSize as VLS, ArmT32IndexMode as Idx,
};

fn q(number: u8) -> Arm32MveVectorRegister {
    Arm32MveVectorRegister::new(number).unwrap()
}
fn s(number: u8) -> S {
    S::new(number).unwrap()
}
fn d(number: u8) -> D {
    D::new(number).unwrap()
}

// Miri cannot spawn processes or do file I/O, so the external-assembler oracle tests are gated out of the
// Miri run (they shell out to gas/llvm-mc). `cargo +nightly miri test --lib` already excludes this `tests/`
// integration binary; the attribute is defensive and a no-op under normal `cargo test` (where `cfg(miri)` is
// false), so the oracle still runs exactly as before.
#[test]
#[cfg_attr(miri, ignore)]
fn gnu_assembler_matches_our_mve_encoder_and_decoder() {
    let backend = match discover_gnu_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require(
                "differential_oracle_mve: no GNU arm-none-eabi assembler found",
            );
            return;
        }
    };

    let samples = sample_instructions();

    let mut source = String::from(".syntax unified\n.thumb\n.text\n");
    for (ual, _) in &samples {
        source.push_str(ual);
        source.push('\n');
    }
    let actual = backend.assemble_and_extract(&source);
    assert_eq!(
        actual.len(),
        samples.len() * 4,
        "GNU produced {} bytes for {} MVE samples",
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
        let decoded = I::decode(&mut iter, &mut consumed)
            .expect("decode GNU bytes")
            .expect("non-empty");
        assert_eq!(
            &decoded, instruction,
            "DECODER disagreement on `{}`: GNU bytes {:02x?} decoded to {:?}",
            ual, theirs, decoded
        );
    }

    eprintln!(
        "differential_oracle_mve: GNU validated {} MVE instructions (encoder + decoder)",
        samples.len()
    );
}

// The emitter round-trip: render each instruction via the UAL emitter, re-assemble that text with GNU, and
// confirm the bytes match `encode()`.
#[test]
#[cfg_attr(miri, ignore)]
fn our_mve_emitter_output_reassembles_via_gnu() {
    let backend = match discover_gnu_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require(
                "differential_oracle_mve emitter check: no GNU arm-none-eabi assembler found",
            );
            return;
        }
    };

    let samples = sample_instructions();
    let rendered: Vec<String> = samples
        .iter()
        .map(|(_, instruction)| instruction.to_assembly_string(ArmAssemblySyntax::Gnu))
        .collect();

    let mut source = String::from(".syntax unified\n.thumb\n.text\n");
    for text in &rendered {
        source.push_str(text);
        source.push('\n');
    }
    let actual = backend.assemble_and_extract(&source);
    assert_eq!(
        actual.len(),
        samples.len() * 4,
        "GNU produced {} bytes for {} emitted MVE samples",
        actual.len(),
        samples.len()
    );

    for (index, (_ual, instruction)) in samples.iter().enumerate() {
        let our = instruction.encode().unwrap();
        let theirs = &actual[index * 4..index * 4 + 4];
        assert_eq!(
            our.as_slice(),
            theirs,
            "EMITTER disagreement: rendered as `{}` which GNU assembled to {:02x?}, but encode() is {:02x?}\n  instruction: {:?}",
            rendered[index],
            theirs,
            our,
            instruction
        );
    }
    eprintln!(
        "differential_oracle_mve: GNU validated {} MVE emitter renderings",
        samples.len()
    );
}

// VPT predication blocks. GNU requires a VPT to be followed by its predicated block members, so -- unlike the
// per-instruction samples above -- each VPT is assembled inside a COMPLETE block, and we verify the VPT
// instruction itself (the block's first 4 bytes) against GNU three ways: our `encode()` matches GNU's bytes,
// our decoder turns GNU's bytes back into the VPT model, and our emitter's rendering re-assembles (in the same
// block) to the same bytes. Known-good mask -> mnemonic: 0b1000 = `vpt`, 0b0100 = `vptt`, 0b1100 = `vpte`; the
// comparison condition fixes the type-suffix family (eq/ne -> `.i`, gt/ge/lt/le -> `.s`, float -> `.f`).
#[test]
#[cfg_attr(miri, ignore)]
fn gnu_validates_vpt_blocks() {
    let backend = match discover_gnu_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require(
                "differential_oracle_mve VPT: no GNU arm-none-eabi assembler found",
            );
            return;
        }
    };

    // (the VPT instruction's own UAL, the predicated members that complete its block, the VPT model)
    let blocks: Vec<(&str, &str, I)> = vec![
        (
            "vpt.i16 eq, q0, q1",
            "vaddt.i16 q2, q3, q4\n",
            I::MveVptReg(CC::Eq, Size::I16, q(0), q(1), 0b1000),
        ),
        (
            "vptt.i32 ne, q2, q3",
            "vaddt.i32 q0, q1, q2\nvaddt.i32 q0, q1, q2\n",
            I::MveVptReg(CC::Ne, Size::I32, q(2), q(3), 0b0100),
        ),
        (
            "vpte.s8 gt, q0, q1",
            "vaddt.s8 q2, q3, q4\nvadde.s8 q2, q3, q4\n",
            I::MveVptReg(CC::Gt, Size::I8, q(0), q(1), 0b1100),
        ),
        (
            "vpt.i32 eq, q0, r2",
            "vaddt.i32 q3, q4, q5\n",
            I::MveVptScalar(CC::Eq, Size::I32, q(0), R::R2, 0b1000),
        ),
        (
            "vptt.f32 ge, q0, q1",
            "vaddt.f32 q3, q4, q5\nvaddt.f32 q3, q4, q5\n",
            I::MveVptFloatReg(CC::Ge, FSize::F32, q(0), q(1), 0b0100),
        ),
        (
            "vpt.f16 eq, q0, r2",
            "vaddt.f16 q3, q4, q5\n",
            I::MveVptFloatScalar(CC::Eq, FSize::F16, q(0), R::R2, 0b1000),
        ),
    ];

    for (vpt_ual, members, vpt) in &blocks {
        let our = vpt.encode().expect("VPT must encode");

        // ENCODER: GNU's bytes for the hand-written block must match our encode() of the VPT (its first 4 bytes)
        let block = format!(".syntax unified\n.thumb\n.text\n{vpt_ual}\n{members}");
        let bytes = backend.assemble_and_extract(&block);
        assert!(
            bytes.len() >= 4,
            "GNU produced <4 bytes for VPT block `{vpt_ual}`"
        );
        assert_eq!(
            our.as_slice(),
            &bytes[..4],
            "VPT ENCODER disagreement on `{vpt_ual}`:\n  ours {:02x?}\n  GNU  {:02x?}",
            our,
            &bytes[..4]
        );

        // DECODER: our decoder must turn GNU's VPT bytes back into the exact VPT model
        let mut iter = bytes[..4].iter();
        let mut consumed = 0;
        let decoded = I::decode(&mut iter, &mut consumed)
            .expect("decode GNU VPT bytes")
            .expect("non-empty");
        assert_eq!(
            &decoded,
            vpt,
            "VPT DECODER disagreement on `{vpt_ual}`: GNU bytes {:02x?}",
            &bytes[..4]
        );

        // EMITTER: our UAL rendering, re-assembled in the same block, must produce the same VPT bytes
        let rendered = vpt.to_assembly_string(ArmAssemblySyntax::Gnu);
        let rendered_block = format!(".syntax unified\n.thumb\n.text\n{rendered}\n{members}");
        let reassembled = backend.assemble_and_extract(&rendered_block);
        assert_eq!(
            our.as_slice(),
            &reassembled[..4],
            "VPT EMITTER disagreement: rendered `{rendered}` which GNU assembled to {:02x?}",
            &reassembled[..4]
        );
    }

    eprintln!(
        "differential_oracle_mve: GNU validated {} VPT blocks (encoder + decoder + emitter)",
        blocks.len()
    );
}

// A cross-section: every batch-1 op, with assorted element sizes and register triples. (The exhaustive
// registerxsize sweep lives in the in-crate round-trip test; here we anchor to GNU's bytes.)
fn sample_instructions() -> Vec<(&'static str, I)> {
    vec![
        // -- integer arithmetic (size in the .i/.s/.u suffix) --
        (
            "vadd.i8 q0, q1, q2",
            I::MveIntArith(Iop::Vadd, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vadd.i16 q3, q4, q5",
            I::MveIntArith(Iop::Vadd, Size::I16, q(3), q(4), q(5)),
        ),
        (
            "vadd.i32 q7, q6, q0",
            I::MveIntArith(Iop::Vadd, Size::I32, q(7), q(6), q(0)),
        ),
        (
            "vsub.i8 q0, q1, q2",
            I::MveIntArith(Iop::Vsub, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vsub.i32 q1, q2, q3",
            I::MveIntArith(Iop::Vsub, Size::I32, q(1), q(2), q(3)),
        ),
        (
            "vmul.i16 q4, q5, q6",
            I::MveIntArith(Iop::Vmul, Size::I16, q(4), q(5), q(6)),
        ),
        (
            "vqadd.s8 q0, q1, q2",
            I::MveIntArith(Iop::VqaddS, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vqadd.u16 q0, q1, q2",
            I::MveIntArith(Iop::VqaddU, Size::I16, q(0), q(1), q(2)),
        ),
        (
            "vqsub.s32 q0, q1, q2",
            I::MveIntArith(Iop::VqsubS, Size::I32, q(0), q(1), q(2)),
        ),
        (
            "vqsub.u8 q5, q6, q7",
            I::MveIntArith(Iop::VqsubU, Size::I8, q(5), q(6), q(7)),
        ),
        (
            "vhadd.s8 q0, q1, q2",
            I::MveIntArith(Iop::VhaddS, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vhadd.u32 q0, q1, q2",
            I::MveIntArith(Iop::VhaddU, Size::I32, q(0), q(1), q(2)),
        ),
        (
            "vhsub.s16 q0, q1, q2",
            I::MveIntArith(Iop::VhsubS, Size::I16, q(0), q(1), q(2)),
        ),
        (
            "vhsub.u8 q0, q1, q2",
            I::MveIntArith(Iop::VhsubU, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vrhadd.s8 q0, q1, q2",
            I::MveIntArith(Iop::VrhaddS, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vrhadd.u16 q0, q1, q2",
            I::MveIntArith(Iop::VrhaddU, Size::I16, q(0), q(1), q(2)),
        ),
        (
            "vabd.s8 q0, q1, q2",
            I::MveIntArith(Iop::VabdS, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vabd.u32 q0, q1, q2",
            I::MveIntArith(Iop::VabdU, Size::I32, q(0), q(1), q(2)),
        ),
        (
            "vmax.s8 q0, q1, q2",
            I::MveIntArith(Iop::VmaxS, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vmax.u16 q0, q1, q2",
            I::MveIntArith(Iop::VmaxU, Size::I16, q(0), q(1), q(2)),
        ),
        (
            "vmin.s32 q0, q1, q2",
            I::MveIntArith(Iop::VminS, Size::I32, q(0), q(1), q(2)),
        ),
        (
            "vmin.u8 q0, q1, q2",
            I::MveIntArith(Iop::VminU, Size::I8, q(0), q(1), q(2)),
        ),
        // -- bitwise (no type suffix) --
        (
            "vand q0, q1, q2",
            I::MveBitwise(Bop::Vand, q(0), q(1), q(2)),
        ),
        (
            "vbic q3, q4, q5",
            I::MveBitwise(Bop::Vbic, q(3), q(4), q(5)),
        ),
        (
            "vorr q0, q1, q2",
            I::MveBitwise(Bop::Vorr, q(0), q(1), q(2)),
        ),
        (
            "vorn q6, q7, q0",
            I::MveBitwise(Bop::Vorn, q(6), q(7), q(0)),
        ),
        (
            "veor q0, q1, q2",
            I::MveBitwise(Bop::Veor, q(0), q(1), q(2)),
        ),
        // -- floating-point (.f16 / .f32) --
        (
            "vadd.f32 q0, q1, q2",
            I::MveFloatArith(Fop::Vadd, FSize::F32, q(0), q(1), q(2)),
        ),
        (
            "vadd.f16 q3, q4, q5",
            I::MveFloatArith(Fop::Vadd, FSize::F16, q(3), q(4), q(5)),
        ),
        (
            "vsub.f32 q0, q1, q2",
            I::MveFloatArith(Fop::Vsub, FSize::F32, q(0), q(1), q(2)),
        ),
        (
            "vmul.f16 q0, q1, q2",
            I::MveFloatArith(Fop::Vmul, FSize::F16, q(0), q(1), q(2)),
        ),
        (
            "vabd.f32 q0, q1, q2",
            I::MveFloatArith(Fop::Vabd, FSize::F32, q(0), q(1), q(2)),
        ),
        (
            "vmaxnm.f16 q0, q1, q2",
            I::MveFloatArith(Fop::Vmaxnm, FSize::F16, q(0), q(1), q(2)),
        ),
        (
            "vminnm.f32 q0, q1, q2",
            I::MveFloatArith(Fop::Vminnm, FSize::F32, q(0), q(1), q(2)),
        ),
        (
            "vfma.f32 q0, q1, q2",
            I::MveFloatArith(Fop::Vfma, FSize::F32, q(0), q(1), q(2)),
        ),
        (
            "vfms.f16 q0, q1, q2",
            I::MveFloatArith(Fop::Vfms, FSize::F16, q(0), q(1), q(2)),
        ),
        // -- vector by scalar (Qd, Qn, Rm) --
        (
            "vadd.i32 q0, q1, r2",
            I::MveVecScalarInt(VSIop::Vadd, Size::I32, q(0), q(1), R::R2),
        ),
        (
            "vadd.i8 q3, q4, r5",
            I::MveVecScalarInt(VSIop::Vadd, Size::I8, q(3), q(4), R::R5),
        ),
        (
            "vsub.i16 q0, q1, r10",
            I::MveVecScalarInt(VSIop::Vsub, Size::I16, q(0), q(1), R::R10),
        ),
        (
            "vmul.i32 q7, q0, r1",
            I::MveVecScalarInt(VSIop::Vmul, Size::I32, q(7), q(0), R::R1),
        ),
        (
            "vhadd.s32 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VhaddS, Size::I32, q(0), q(1), R::R2),
        ),
        (
            "vhadd.u8 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VhaddU, Size::I8, q(0), q(1), R::R2),
        ),
        (
            "vhsub.s16 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VhsubS, Size::I16, q(0), q(1), R::R2),
        ),
        (
            "vhsub.u32 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VhsubU, Size::I32, q(0), q(1), R::R2),
        ),
        (
            "vqadd.s8 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqaddS, Size::I8, q(0), q(1), R::R2),
        ),
        (
            "vqadd.u16 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqaddU, Size::I16, q(0), q(1), R::R2),
        ),
        (
            "vqsub.s32 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqsubS, Size::I32, q(0), q(1), R::R2),
        ),
        (
            "vqsub.u8 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqsubU, Size::I8, q(0), q(1), R::R2),
        ),
        (
            "vqdmulh.s16 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqdmulhS, Size::I16, q(0), q(1), R::R2),
        ),
        (
            "vqrdmulh.s32 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqrdmulhS, Size::I32, q(0), q(1), R::R2),
        ),
        (
            "vadd.f32 q0, q1, r2",
            I::MveVecScalarFloat(VSFop::Vadd, FSize::F32, q(0), q(1), R::R2),
        ),
        (
            "vsub.f16 q3, q4, r5",
            I::MveVecScalarFloat(VSFop::Vsub, FSize::F16, q(3), q(4), R::R5),
        ),
        (
            "vmul.f32 q0, q1, r2",
            I::MveVecScalarFloat(VSFop::Vmul, FSize::F32, q(0), q(1), R::R2),
        ),
        // multiply-accumulate vector-by-scalar
        // VMLA/VMLAS (vector by scalar) are intentionally NOT GNU-validated here: GNU is buggy -- it sets bit 28
        // for the `.u` form and rejects the spec-correct `.i` form (DDI0553 C2.4.380/C2.4.384: <dt> = I8/I16/I32,
        // bit 28 fixed (0)). They are anchored to LLVM + the spec in the library's
        // mve_tests.rs::encode__mve_multiply_accumulate_exact_bytes instead.
        (
            "vqdmlah.s8 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqdmlahS, Size::I8, q(0), q(1), R::R2),
        ),
        (
            "vqrdmlah.s16 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqrdmlahS, Size::I16, q(0), q(1), R::R2),
        ),
        (
            "vqdmlash.s32 q0, q1, r2",
            I::MveVecScalarInt(VSIop::VqdmlashS, Size::I32, q(0), q(1), R::R2),
        ),
        (
            "vqrdmlash.s8 q6, q7, r1",
            I::MveVecScalarInt(VSIop::VqrdmlashS, Size::I8, q(6), q(7), R::R1),
        ),
        (
            "vfma.f32 q0, q1, r2",
            I::MveVecScalarFloat(VSFop::Vfma, FSize::F32, q(0), q(1), R::R2),
        ),
        (
            "vfmas.f16 q3, q4, r5",
            I::MveVecScalarFloat(VSFop::Vfmas, FSize::F16, q(3), q(4), R::R5),
        ),
        // -- VDUP (Qd, Rt) --
        ("vdup.32 q0, r1", I::MveVdup(Size::I32, q(0), R::R1)),
        ("vdup.16 q5, r6", I::MveVdup(Size::I16, q(5), R::R6)),
        ("vdup.8 q7, r12", I::MveVdup(Size::I8, q(7), R::R12)),
        // -- shift by immediate (Qd, Qm, #amount) --
        (
            "vshr.s8 q0, q1, #1",
            I::MveShiftImm(SHop::VshrS, Size::I8, 1, q(0), q(1)),
        ),
        (
            "vshr.s16 q2, q3, #16",
            I::MveShiftImm(SHop::VshrS, Size::I16, 16, q(2), q(3)),
        ),
        (
            "vshr.u32 q0, q1, #32",
            I::MveShiftImm(SHop::VshrU, Size::I32, 32, q(0), q(1)),
        ),
        (
            "vrshr.s16 q0, q1, #2",
            I::MveShiftImm(SHop::VrshrS, Size::I16, 2, q(0), q(1)),
        ),
        (
            "vrshr.u8 q4, q5, #3",
            I::MveShiftImm(SHop::VrshrU, Size::I8, 3, q(4), q(5)),
        ),
        (
            "vsri.16 q0, q1, #2",
            I::MveShiftImm(SHop::Vsri, Size::I16, 2, q(0), q(1)),
        ),
        (
            "vsri.32 q6, q7, #8",
            I::MveShiftImm(SHop::Vsri, Size::I32, 8, q(6), q(7)),
        ),
        (
            "vshl.i8 q0, q1, #0",
            I::MveShiftImm(SHop::VshlI, Size::I8, 0, q(0), q(1)),
        ),
        (
            "vshl.i32 q0, q1, #3",
            I::MveShiftImm(SHop::VshlI, Size::I32, 3, q(0), q(1)),
        ),
        (
            "vsli.8 q0, q1, #1",
            I::MveShiftImm(SHop::Vsli, Size::I8, 1, q(0), q(1)),
        ),
        (
            "vsli.32 q0, q1, #7",
            I::MveShiftImm(SHop::Vsli, Size::I32, 7, q(0), q(1)),
        ),
        (
            "vqshl.s8 q0, q1, #1",
            I::MveShiftImm(SHop::VqshlS, Size::I8, 1, q(0), q(1)),
        ),
        (
            "vqshl.u16 q2, q3, #2",
            I::MveShiftImm(SHop::VqshlU, Size::I16, 2, q(2), q(3)),
        ),
        (
            "vqshlu.s8 q0, q1, #1",
            I::MveShiftImm(SHop::VqshluS, Size::I8, 1, q(0), q(1)),
        ),
        (
            "vqshlu.s32 q0, q1, #5",
            I::MveShiftImm(SHop::VqshluS, Size::I32, 5, q(0), q(1)),
        ),
        // -- one-register modified immediate (Qd, #imm) : raw (cmode, op, imm8) --
        (
            "vmov.i8 q0, #0x12",
            I::MveModifiedImmediate(0b1110, false, 0x12, q(0)),
        ),
        (
            "vmov.i16 q1, #0x12",
            I::MveModifiedImmediate(0b1000, false, 0x12, q(1)),
        ),
        (
            "vmov.i16 q0, #0x1200",
            I::MveModifiedImmediate(0b1010, false, 0x12, q(0)),
        ),
        (
            "vmov.i32 q0, #0x12",
            I::MveModifiedImmediate(0b0000, false, 0x12, q(0)),
        ),
        (
            "vmov.i32 q2, #0x1200",
            I::MveModifiedImmediate(0b0010, false, 0x12, q(2)),
        ),
        (
            "vmov.i32 q0, #0x120000",
            I::MveModifiedImmediate(0b0100, false, 0x12, q(0)),
        ),
        (
            "vmov.i32 q0, #0x12000000",
            I::MveModifiedImmediate(0b0110, false, 0x12, q(0)),
        ),
        (
            "vmov.i32 q0, #0x12ff",
            I::MveModifiedImmediate(0b1100, false, 0x12, q(0)),
        ),
        (
            "vmov.i32 q0, #0x12ffff",
            I::MveModifiedImmediate(0b1101, false, 0x12, q(0)),
        ),
        (
            "vmvn.i16 q0, #0x12",
            I::MveModifiedImmediate(0b1000, true, 0x12, q(0)),
        ),
        (
            "vmvn.i32 q0, #0x1200",
            I::MveModifiedImmediate(0b0010, true, 0x12, q(0)),
        ),
        (
            "vmov.i64 q0, #0xff00ff00ff00ff00",
            I::MveModifiedImmediate(0b1110, true, 0xaa, q(0)),
        ),
        (
            "vmov.f32 q0, #1.0",
            I::MveModifiedImmediate(0b1111, false, 0x70, q(0)),
        ),
        (
            "vmov.i8 q7, #0xff",
            I::MveModifiedImmediate(0b1110, false, 0xff, q(7)),
        ),
        (
            "vorr.i16 q0, #0x1200",
            I::MveModifiedImmediate(0b1011, false, 0x12, q(0)),
        ),
        (
            "vorr.i32 q0, #0x120000",
            I::MveModifiedImmediate(0b0101, false, 0x12, q(0)),
        ),
        (
            "vbic.i16 q0, #0x12",
            I::MveModifiedImmediate(0b1001, true, 0x12, q(0)),
        ),
        (
            "vbic.i32 q0, #0xff",
            I::MveModifiedImmediate(0b0001, true, 0xff, q(0)),
        ),
        // -- 2-register miscellaneous (Qd, Qm) --
        (
            "vrev64.8 q0, q1",
            I::MveMisc2(M2op::Vrev64, Size::I8, q(0), q(1)),
        ),
        (
            "vrev64.16 q2, q3",
            I::MveMisc2(M2op::Vrev64, Size::I16, q(2), q(3)),
        ),
        (
            "vrev64.32 q0, q1",
            I::MveMisc2(M2op::Vrev64, Size::I32, q(0), q(1)),
        ),
        (
            "vrev32.8 q4, q5",
            I::MveMisc2(M2op::Vrev32, Size::I8, q(4), q(5)),
        ),
        (
            "vrev32.16 q0, q1",
            I::MveMisc2(M2op::Vrev32, Size::I16, q(0), q(1)),
        ),
        (
            "vrev16.8 q6, q7",
            I::MveMisc2(M2op::Vrev16, Size::I8, q(6), q(7)),
        ),
        (
            "vcls.s8 q0, q1",
            I::MveMisc2(M2op::Vcls, Size::I8, q(0), q(1)),
        ),
        (
            "vcls.s32 q0, q1",
            I::MveMisc2(M2op::Vcls, Size::I32, q(0), q(1)),
        ),
        (
            "vclz.i8 q0, q1",
            I::MveMisc2(M2op::Vclz, Size::I8, q(0), q(1)),
        ),
        (
            "vclz.i32 q2, q3",
            I::MveMisc2(M2op::Vclz, Size::I32, q(2), q(3)),
        ),
        (
            "vabs.s8 q0, q1",
            I::MveMisc2(M2op::Vabs, Size::I8, q(0), q(1)),
        ),
        (
            "vabs.s32 q0, q1",
            I::MveMisc2(M2op::Vabs, Size::I32, q(0), q(1)),
        ),
        (
            "vneg.s16 q0, q1",
            I::MveMisc2(M2op::Vneg, Size::I16, q(0), q(1)),
        ),
        (
            "vqabs.s8 q0, q1",
            I::MveMisc2(M2op::Vqabs, Size::I8, q(0), q(1)),
        ),
        (
            "vqneg.s16 q4, q5",
            I::MveMisc2(M2op::Vqneg, Size::I16, q(4), q(5)),
        ),
        (
            "vabs.f16 q0, q1",
            I::MveMisc2Float(M2Fop::Vabs, FSize::F16, q(0), q(1)),
        ),
        (
            "vabs.f32 q0, q1",
            I::MveMisc2Float(M2Fop::Vabs, FSize::F32, q(0), q(1)),
        ),
        (
            "vneg.f16 q2, q3",
            I::MveMisc2Float(M2Fop::Vneg, FSize::F16, q(2), q(3)),
        ),
        (
            "vneg.f32 q0, q1",
            I::MveMisc2Float(M2Fop::Vneg, FSize::F32, q(0), q(1)),
        ),
        ("vmvn q0, q1", I::MveMvnRegister(q(0), q(1))),
        ("vmvn q6, q7", I::MveMvnRegister(q(6), q(7))),
        // -- contiguous vector load/store (Qd, [Rn{, #off}]{!}) --
        (
            "vldrw.u32 q0, [r0]",
            I::MveLoadStore(true, Size::I32, q(0), R::R0, 0, Idx::Offset),
        ),
        (
            "vldrw.u32 q7, [r5, #16]",
            I::MveLoadStore(true, Size::I32, q(7), R::R5, 16, Idx::Offset),
        ),
        (
            "vldrw.u32 q0, [r1, #-32]",
            I::MveLoadStore(true, Size::I32, q(0), R::R1, -32, Idx::Offset),
        ),
        (
            "vldrw.u32 q0, [r0, #508]",
            I::MveLoadStore(true, Size::I32, q(0), R::R0, 508, Idx::Offset),
        ),
        (
            "vldrw.u32 q0, [r0, #16]!",
            I::MveLoadStore(true, Size::I32, q(0), R::R0, 16, Idx::PreIndex),
        ),
        (
            "vldrw.u32 q0, [r0], #16",
            I::MveLoadStore(true, Size::I32, q(0), R::R0, 16, Idx::PostIndex),
        ),
        (
            "vstrw.32 q0, [r0]",
            I::MveLoadStore(false, Size::I32, q(0), R::R0, 0, Idx::Offset),
        ),
        (
            "vstrw.32 q3, [r1, #64]",
            I::MveLoadStore(false, Size::I32, q(3), R::R1, 64, Idx::Offset),
        ),
        (
            "vldrb.u8 q0, [r0]",
            I::MveLoadStore(true, Size::I8, q(0), R::R0, 0, Idx::Offset),
        ),
        (
            "vldrb.u8 q2, [r3, #127]",
            I::MveLoadStore(true, Size::I8, q(2), R::R3, 127, Idx::Offset),
        ),
        (
            "vldrh.u16 q0, [r0, #16]",
            I::MveLoadStore(true, Size::I16, q(0), R::R0, 16, Idx::Offset),
        ),
        (
            "vldrh.u16 q0, [r0, #254]",
            I::MveLoadStore(true, Size::I16, q(0), R::R0, 254, Idx::Offset),
        ),
        (
            "vstrb.8 q0, [r0, #10]",
            I::MveLoadStore(false, Size::I8, q(0), R::R0, 10, Idx::Offset),
        ),
        (
            "vstrh.16 q4, [r2, #32]!",
            I::MveLoadStore(false, Size::I16, q(4), R::R2, 32, Idx::PreIndex),
        ),
        (
            "vstrw.32 q0, [r7], #-16",
            I::MveLoadStore(false, Size::I32, q(0), R::R7, -16, Idx::PostIndex),
        ),
        // -- cross-lane reductions to a GPR (VADDV/VADDVA need an even Rd) --
        (
            "vaddv.s8 r0, q1",
            I::MveReduce(RDop::VaddvS, Size::I8, R::R0, q(1)),
        ),
        (
            "vaddv.u16 r2, q3",
            I::MveReduce(RDop::VaddvU, Size::I16, R::R2, q(3)),
        ),
        (
            "vaddv.s32 r4, q7",
            I::MveReduce(RDop::VaddvS, Size::I32, R::R4, q(7)),
        ),
        (
            "vaddva.s32 r8, q1",
            I::MveReduce(RDop::VaddvaS, Size::I32, R::R8, q(1)),
        ),
        (
            "vminv.s8 r1, q1",
            I::MveReduce(RDop::VminvS, Size::I8, R::R1, q(1)),
        ),
        (
            "vminv.u16 r3, q2",
            I::MveReduce(RDop::VminvU, Size::I16, R::R3, q(2)),
        ),
        (
            "vmaxv.s32 r5, q1",
            I::MveReduce(RDop::VmaxvS, Size::I32, R::R5, q(1)),
        ),
        (
            "vmaxv.u8 r7, q1",
            I::MveReduce(RDop::VmaxvU, Size::I8, R::R7, q(1)),
        ),
        (
            "vminav.s8 r1, q1",
            I::MveReduce(RDop::Vminav, Size::I8, R::R1, q(1)),
        ),
        (
            "vmaxav.s32 r9, q1",
            I::MveReduce(RDop::Vmaxav, Size::I32, R::R9, q(1)),
        ),
        (
            "vabav.s8 r0, q1, q2",
            I::MveVabav(true, Size::I8, R::R0, q(1), q(2)),
        ),
        (
            "vabav.u16 r1, q3, q4",
            I::MveVabav(false, Size::I16, R::R1, q(3), q(4)),
        ),
        (
            "vabav.s32 r11, q7, q3",
            I::MveVabav(true, Size::I32, R::R11, q(7), q(3)),
        ),
        // -- VMLADAV/VMLSDAV (non-long dual MAC reductions; X & subtract are signed-only) --
        (
            "vmladav.s16 r0, q1, q2",
            I::MveDualMac(false, false, false, false, Size::I16, R::R0, q(1), q(2)),
        ),
        (
            "vmladava.u8 r2, q3, q4",
            I::MveDualMac(false, false, true, true, Size::I8, R::R2, q(3), q(4)),
        ),
        (
            "vmladavx.s32 r4, q5, q6",
            I::MveDualMac(false, true, false, false, Size::I32, R::R4, q(5), q(6)),
        ),
        (
            "vmladavax.s16 r14, q7, q0",
            I::MveDualMac(false, true, true, false, Size::I16, R::R14, q(7), q(0)),
        ),
        (
            "vmlsdav.s8 r0, q1, q2",
            I::MveDualMac(true, false, false, false, Size::I8, R::R0, q(1), q(2)),
        ),
        (
            "vmlsdavax.s32 r12, q3, q4",
            I::MveDualMac(true, true, true, false, Size::I32, R::R12, q(3), q(4)),
        ),
        // -- VMLALDAV/VMLSLDAV/VRMLALDAVH/VRMLSLDAVH (long dual MAC into a GPR pair; RdaLo even, RdaHi odd) --
        (
            "vmlaldav.s16 r0, r1, q2, q3",
            I::MveLongDualMac(
                LMop::Vmlaldav,
                false,
                false,
                false,
                Size::I16,
                R::R0,
                R::R1,
                q(2),
                q(3),
            ),
        ),
        (
            "vmlaldava.u32 r2, r3, q4, q5",
            I::MveLongDualMac(
                LMop::Vmlaldav,
                false,
                true,
                true,
                Size::I32,
                R::R2,
                R::R3,
                q(4),
                q(5),
            ),
        ),
        (
            "vmlaldavx.s32 r8, r9, q0, q1",
            I::MveLongDualMac(
                LMop::Vmlaldav,
                true,
                false,
                false,
                Size::I32,
                R::R8,
                R::R9,
                q(0),
                q(1),
            ),
        ),
        (
            "vmlaldav.s16 r4, r1, q2, q3",
            I::MveLongDualMac(
                LMop::Vmlaldav,
                false,
                false,
                false,
                Size::I16,
                R::R4,
                R::R1,
                q(2),
                q(3),
            ),
        ),
        (
            "vmlsldav.s16 r0, r1, q2, q3",
            I::MveLongDualMac(
                LMop::Vmlsldav,
                false,
                false,
                false,
                Size::I16,
                R::R0,
                R::R1,
                q(2),
                q(3),
            ),
        ),
        (
            "vmlsldavax.s32 r10, r11, q6, q7",
            I::MveLongDualMac(
                LMop::Vmlsldav,
                true,
                true,
                false,
                Size::I32,
                R::R10,
                R::R11,
                q(6),
                q(7),
            ),
        ),
        (
            "vrmlaldavh.s32 r0, r1, q2, q3",
            I::MveLongDualMac(
                LMop::Vrmlaldavh,
                false,
                false,
                false,
                Size::I32,
                R::R0,
                R::R1,
                q(2),
                q(3),
            ),
        ),
        (
            "vrmlaldavh.u32 r4, r5, q2, q3",
            I::MveLongDualMac(
                LMop::Vrmlaldavh,
                false,
                false,
                true,
                Size::I32,
                R::R4,
                R::R5,
                q(2),
                q(3),
            ),
        ),
        (
            "vrmlaldavhx.s32 r0, r11, q2, q3",
            I::MveLongDualMac(
                LMop::Vrmlaldavh,
                true,
                false,
                false,
                Size::I32,
                R::R0,
                R::R11,
                q(2),
                q(3),
            ),
        ),
        (
            "vrmlsldavh.s32 r0, r1, q2, q3",
            I::MveLongDualMac(
                LMop::Vrmlsldavh,
                false,
                false,
                false,
                Size::I32,
                R::R0,
                R::R1,
                q(2),
                q(3),
            ),
        ),
        (
            "vrmlsldavhx.s32 r6, r7, q2, q3",
            I::MveLongDualMac(
                LMop::Vrmlsldavh,
                true,
                false,
                false,
                Size::I32,
                R::R6,
                R::R7,
                q(2),
                q(3),
            ),
        ),
        // -- VRINT (round to integral float) --
        (
            "vrintn.f32 q0, q1",
            I::MveVrint(RIop::Vrintn, FSize::F32, q(0), q(1)),
        ),
        (
            "vrinta.f32 q2, q3",
            I::MveVrint(RIop::Vrinta, FSize::F32, q(2), q(3)),
        ),
        (
            "vrintz.f32 q0, q1",
            I::MveVrint(RIop::Vrintz, FSize::F32, q(0), q(1)),
        ),
        (
            "vrintm.f16 q0, q1",
            I::MveVrint(RIop::Vrintm, FSize::F16, q(0), q(1)),
        ),
        (
            "vrintp.f32 q4, q5",
            I::MveVrint(RIop::Vrintp, FSize::F32, q(4), q(5)),
        ),
        (
            "vrintx.f16 q0, q1",
            I::MveVrint(RIop::Vrintx, FSize::F16, q(0), q(1)),
        ),
        // -- VCVT float<->int --
        (
            "vcvt.f32.s32 q0, q1",
            I::MveVcvtFloatInt(false, false, FSize::F32, q(0), q(1)),
        ),
        (
            "vcvt.f32.u32 q2, q3",
            I::MveVcvtFloatInt(false, true, FSize::F32, q(2), q(3)),
        ),
        (
            "vcvt.s32.f32 q0, q1",
            I::MveVcvtFloatInt(true, false, FSize::F32, q(0), q(1)),
        ),
        (
            "vcvt.u32.f32 q0, q1",
            I::MveVcvtFloatInt(true, true, FSize::F32, q(0), q(1)),
        ),
        (
            "vcvt.f16.s16 q4, q5",
            I::MveVcvtFloatInt(false, false, FSize::F16, q(4), q(5)),
        ),
        (
            "vcvt.u16.f16 q0, q1",
            I::MveVcvtFloatInt(true, true, FSize::F16, q(0), q(1)),
        ),
        // -- VCVTA/N/P/M (float -> int, explicit rounding mode) --
        (
            "vcvta.s32.f32 q0, q1",
            I::MveVcvtRound(0, false, FSize::F32, q(0), q(1)),
        ),
        (
            "vcvtn.s32.f32 q0, q1",
            I::MveVcvtRound(1, false, FSize::F32, q(0), q(1)),
        ),
        (
            "vcvtp.u32.f32 q2, q3",
            I::MveVcvtRound(2, true, FSize::F32, q(2), q(3)),
        ),
        (
            "vcvtm.u32.f32 q0, q1",
            I::MveVcvtRound(3, true, FSize::F32, q(0), q(1)),
        ),
        (
            "vcvta.s16.f16 q4, q5",
            I::MveVcvtRound(0, false, FSize::F16, q(4), q(5)),
        ),
        (
            "vcvtp.u16.f16 q7, q3",
            I::MveVcvtRound(2, true, FSize::F16, q(7), q(3)),
        ),
        // -- fixed-point VCVT (float <-> fixed, #fbits) --
        (
            "vcvt.s16.f16 q0, q1, #1",
            I::MveVcvtFixed(true, false, FSize::F16, 1, q(0), q(1)),
        ),
        (
            "vcvt.u32.f32 q0, q1, #1",
            I::MveVcvtFixed(true, true, FSize::F32, 1, q(0), q(1)),
        ),
        (
            "vcvt.f16.s16 q2, q3, #5",
            I::MveVcvtFixed(false, false, FSize::F16, 5, q(2), q(3)),
        ),
        (
            "vcvt.f32.u32 q7, q3, #17",
            I::MveVcvtFixed(false, true, FSize::F32, 17, q(7), q(3)),
        ),
        (
            "vcvt.s16.f16 q0, q1, #16",
            I::MveVcvtFixed(true, false, FSize::F16, 16, q(0), q(1)),
        ),
        (
            "vcvt.s32.f32 q4, q5, #32",
            I::MveVcvtFixed(true, false, FSize::F32, 32, q(4), q(5)),
        ),
        // -- VMOVL (long) / VMOVN (narrow) / VADDLV (64-bit reduction) --
        (
            "vmovlb.s8 q0, q1",
            I::MveVmovl(false, false, Size::I8, q(0), q(1)),
        ),
        (
            "vmovlt.u8 q2, q3",
            I::MveVmovl(true, true, Size::I8, q(2), q(3)),
        ),
        (
            "vmovlb.s16 q0, q1",
            I::MveVmovl(false, false, Size::I16, q(0), q(1)),
        ),
        (
            "vmovlt.u16 q4, q5",
            I::MveVmovl(true, true, Size::I16, q(4), q(5)),
        ),
        (
            "vmovnb.i16 q0, q1",
            I::MveVmovn(false, Size::I16, q(0), q(1)),
        ),
        (
            "vmovnt.i16 q6, q7",
            I::MveVmovn(true, Size::I16, q(6), q(7)),
        ),
        (
            "vmovnb.i32 q0, q1",
            I::MveVmovn(false, Size::I32, q(0), q(1)),
        ),
        (
            "vmovnt.i32 q2, q3",
            I::MveVmovn(true, Size::I32, q(2), q(3)),
        ),
        (
            "vqmovnb.s16 q0, q1",
            I::MveVqmovn(QMov::Vqmovn, false, false, Size::I16, q(0), q(1)),
        ),
        (
            "vqmovnt.s16 q6, q7",
            I::MveVqmovn(QMov::Vqmovn, false, true, Size::I16, q(6), q(7)),
        ),
        (
            "vqmovnb.u16 q2, q3",
            I::MveVqmovn(QMov::Vqmovn, true, false, Size::I16, q(2), q(3)),
        ),
        (
            "vqmovnt.u32 q0, q5",
            I::MveVqmovn(QMov::Vqmovn, true, true, Size::I32, q(0), q(5)),
        ),
        (
            "vqmovnb.s32 q4, q1",
            I::MveVqmovn(QMov::Vqmovn, false, false, Size::I32, q(4), q(1)),
        ),
        (
            "vqmovunb.s16 q0, q1",
            I::MveVqmovn(QMov::Vqmovun, false, false, Size::I16, q(0), q(1)),
        ),
        (
            "vqmovunt.s16 q3, q6",
            I::MveVqmovn(QMov::Vqmovun, false, true, Size::I16, q(3), q(6)),
        ),
        (
            "vqmovunb.s32 q7, q2",
            I::MveVqmovn(QMov::Vqmovun, false, false, Size::I32, q(7), q(2)),
        ),
        // VMULL (integer + polynomial), VMULH/VRMULH, VQDMULL (vector + scalar)
        (
            "vmullb.s8 q0, q1, q2",
            I::MveVmull(false, false, false, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vmullt.s16 q4, q5, q6",
            I::MveVmull(false, false, true, Size::I16, q(4), q(5), q(6)),
        ),
        (
            "vmullb.u32 q0, q2, q4",
            I::MveVmull(false, true, false, Size::I32, q(0), q(2), q(4)),
        ),
        (
            "vmullb.p8 q0, q1, q2",
            I::MveVmull(true, false, false, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vmullt.p16 q7, q3, q1",
            I::MveVmull(true, false, true, Size::I16, q(7), q(3), q(1)),
        ),
        (
            "vmulh.s8 q0, q1, q2",
            I::MveVmulh(false, false, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vmulh.u16 q3, q4, q5",
            I::MveVmulh(false, true, Size::I16, q(3), q(4), q(5)),
        ),
        (
            "vrmulh.s32 q0, q7, q1",
            I::MveVmulh(true, false, Size::I32, q(0), q(7), q(1)),
        ),
        (
            "vrmulh.u8 q2, q2, q2",
            I::MveVmulh(true, true, Size::I8, q(2), q(2), q(2)),
        ),
        (
            "vqdmullb.s16 q0, q1, q2",
            I::MveVqdmull(false, false, q(0), q(1), q(2)),
        ),
        (
            "vqdmullt.s32 q4, q5, q6",
            I::MveVqdmull(true, true, q(4), q(5), q(6)),
        ),
        (
            "vqdmullb.s16 q0, q1, r2",
            I::MveVqdmullScalar(false, false, q(0), q(1), R::R2),
        ),
        (
            "vqdmullt.s32 q6, q7, r10",
            I::MveVqdmullScalar(true, true, q(6), q(7), R::R10),
        ),
        // VQDMLADH/VQDMLSDH (+ rounding VQRD*, + exchange X)
        (
            "vqdmladh.s8 q0, q1, q2",
            I::MveVqdmladh(false, false, false, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vqdmladhx.s16 q4, q5, q6",
            I::MveVqdmladh(false, false, true, Size::I16, q(4), q(5), q(6)),
        ),
        (
            "vqrdmladh.s32 q0, q3, q7",
            I::MveVqdmladh(false, true, false, Size::I32, q(0), q(3), q(7)),
        ),
        (
            "vqrdmladhx.s8 q2, q2, q2",
            I::MveVqdmladh(false, true, true, Size::I8, q(2), q(2), q(2)),
        ),
        (
            "vqdmlsdh.s16 q0, q1, q2",
            I::MveVqdmladh(true, false, false, Size::I16, q(0), q(1), q(2)),
        ),
        (
            "vqdmlsdhx.s32 q5, q6, q7",
            I::MveVqdmladh(true, false, true, Size::I32, q(5), q(6), q(7)),
        ),
        (
            "vqrdmlsdh.s8 q0, q4, q1",
            I::MveVqdmladh(true, true, false, Size::I8, q(0), q(4), q(1)),
        ),
        (
            "vqrdmlsdhx.s16 q3, q0, q5",
            I::MveVqdmladh(true, true, true, Size::I16, q(3), q(0), q(5)),
        ),
        // VSHL/VRSHL/VQSHL/VQRSHL by vector (Qd, Qm, Qn) and by GPR scalar (Qda, Rm)
        (
            "vshl.s8 q0, q1, q2",
            I::MveShiftByVector(false, false, false, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vrshl.u16 q3, q4, q5",
            I::MveShiftByVector(true, false, true, Size::I16, q(3), q(4), q(5)),
        ),
        (
            "vqshl.s32 q7, q0, q1",
            I::MveShiftByVector(false, true, false, Size::I32, q(7), q(0), q(1)),
        ),
        (
            "vqrshl.u8 q2, q6, q3",
            I::MveShiftByVector(true, true, true, Size::I8, q(2), q(6), q(3)),
        ),
        (
            "vshl.s16 q0, r4",
            I::MveShiftByScalar(false, false, false, Size::I16, q(0), R::R4),
        ),
        (
            "vrshl.u32 q5, r7",
            I::MveShiftByScalar(true, false, true, Size::I32, q(5), R::R7),
        ),
        (
            "vqshl.s8 q3, r10",
            I::MveShiftByScalar(false, true, false, Size::I8, q(3), R::R10),
        ),
        (
            "vqrshl.u16 q6, r0",
            I::MveShiftByScalar(true, true, true, Size::I16, q(6), R::R0),
        ),
        // VSHLL (T1 imm and T2 max-shift), bottom/top, signed/unsigned
        (
            "vshllb.s8 q0, q1, #1",
            I::MveVshll(false, false, Size::I8, 1, q(0), q(1)),
        ),
        (
            "vshllt.s16 q0, q1, #15",
            I::MveVshll(true, false, Size::I16, 15, q(0), q(1)),
        ),
        (
            "vshllb.u8 q4, q5, #5",
            I::MveVshll(false, true, Size::I8, 5, q(4), q(5)),
        ),
        (
            "vshllb.s8 q0, q1, #8",
            I::MveVshll(false, false, Size::I8, 8, q(0), q(1)),
        ),
        (
            "vshllt.u16 q2, q3, #16",
            I::MveVshll(true, true, Size::I16, 16, q(2), q(3)),
        ),
        // VMOVX / VINS (half-precision FP move-extract / insert)
        (
            "vmovx.f16 s0, s1",
            I::Vmovx_T1(false, S::new(0).unwrap(), S::new(1).unwrap()),
        ),
        (
            "vmovx.f16 s5, s10",
            I::Vmovx_T1(false, S::new(5).unwrap(), S::new(10).unwrap()),
        ),
        (
            "vins.f16 s4, s7",
            I::Vmovx_T1(true, S::new(4).unwrap(), S::new(7).unwrap()),
        ),
        (
            "vmovx.f16 s31, s30",
            I::Vmovx_T1(false, S::new(31).unwrap(), S::new(30).unwrap()),
        ),
        // reduction aliases: VMLAV=VMLADAV, VMLALV=VMLALDAV, VRMLALVH=VRMLALDAVH (X=0). Encodes identically to
        // the canonical form; our emitter renders the canonical mnemonic (also accepted by GNU).
        (
            "vmlav.s8 r0, q1, q2",
            I::MveDualMac(false, false, false, false, Size::I8, R::R0, q(1), q(2)),
        ),
        (
            "vmlava.u16 r2, q3, q4",
            I::MveDualMac(false, false, true, true, Size::I16, R::R2, q(3), q(4)),
        ),
        (
            "vaddlv.s32 r0, r1, q2",
            I::MveVaddlv(false, false, R::R0, R::R1, q(2)),
        ),
        (
            "vaddlv.u32 r4, r5, q3",
            I::MveVaddlv(false, true, R::R4, R::R5, q(3)),
        ),
        (
            "vaddlva.s32 r2, r3, q7",
            I::MveVaddlv(true, false, R::R2, R::R3, q(7)),
        ),
        (
            "vaddlva.u32 r10, r11, q0",
            I::MveVaddlv(true, true, R::R10, R::R11, q(0)),
        ),
        (
            "vaddlv.s32 r2, r5, q2",
            I::MveVaddlv(false, false, R::R2, R::R5, q(2)),
        ), // non-consecutive pair
        (
            "vaddlv.s32 r12, r1, q3",
            I::MveVaddlv(false, false, R::R12, R::R1, q(3)),
        ), // non-consecutive pair
        // -- complex-number ops (Qd, Qn, Qm, #rotation) --
        (
            "vcadd.i8 q0, q1, q2, #90",
            I::MveVcaddInt(false, Size::I8, false, q(0), q(1), q(2)),
        ),
        (
            "vcadd.i32 q3, q4, q5, #270",
            I::MveVcaddInt(false, Size::I32, true, q(3), q(4), q(5)),
        ),
        (
            "vhcadd.s16 q0, q1, q2, #90",
            I::MveVcaddInt(true, Size::I16, false, q(0), q(1), q(2)),
        ),
        (
            "vhcadd.s8 q6, q7, q0, #270",
            I::MveVcaddInt(true, Size::I8, true, q(6), q(7), q(0)),
        ),
        (
            "vcadd.f16 q0, q1, q2, #90",
            I::MveVcaddFloat(FSize::F16, false, q(0), q(1), q(2)),
        ),
        (
            "vcadd.f32 q3, q4, q5, #270",
            I::MveVcaddFloat(FSize::F32, true, q(3), q(4), q(5)),
        ),
        (
            "vcmul.f32 q0, q1, q2, #0",
            I::MveVcmul(FSize::F32, 0, q(0), q(1), q(2)),
        ),
        (
            "vcmul.f32 q0, q1, q2, #90",
            I::MveVcmul(FSize::F32, 1, q(0), q(1), q(2)),
        ),
        (
            "vcmul.f16 q4, q5, q6, #180",
            I::MveVcmul(FSize::F16, 2, q(4), q(5), q(6)),
        ),
        (
            "vcmul.f32 q0, q1, q2, #270",
            I::MveVcmul(FSize::F32, 3, q(0), q(1), q(2)),
        ),
        (
            "vcmla.f32 q0, q1, q2, #0",
            I::MveVcmla(FSize::F32, 0, q(0), q(1), q(2)),
        ),
        (
            "vcmla.f32 q0, q1, q2, #90",
            I::MveVcmla(FSize::F32, 1, q(0), q(1), q(2)),
        ),
        (
            "vcmla.f16 q7, q0, q1, #180",
            I::MveVcmla(FSize::F16, 2, q(7), q(0), q(1)),
        ),
        (
            "vcmla.f32 q0, q1, q2, #270",
            I::MveVcmla(FSize::F32, 3, q(0), q(1), q(2)),
        ),
        // -- predication primitives --
        ("vpsel q0, q1, q2", I::MveVpsel(q(0), q(1), q(2))),
        ("vpsel q7, q3, q4", I::MveVpsel(q(7), q(3), q(4))),
        ("vpnot", I::MveVpnot),
        // -- VADC/VSBC (add/subtract with carry) and VSHLC (whole-vector shift with carry) --
        (
            "vadc.i32 q0, q1, q2",
            I::MveVadc(false, false, q(0), q(1), q(2)),
        ),
        (
            "vadci.i32 q3, q5, q7",
            I::MveVadc(false, true, q(3), q(5), q(7)),
        ),
        (
            "vsbc.i32 q0, q1, q2",
            I::MveVadc(true, false, q(0), q(1), q(2)),
        ),
        (
            "vsbci.i32 q4, q0, q1",
            I::MveVadc(true, true, q(4), q(0), q(1)),
        ),
        ("vshlc q0, r0, #1", I::MveVshlc(1, q(0), R::R0)),
        ("vshlc q3, r7, #8", I::MveVshlc(8, q(3), R::R7)),
        ("vshlc q7, r10, #32", I::MveVshlc(32, q(7), R::R10)),
        // -- VIDUP/VDDUP/VIWDUP/VDWDUP (index generators; Rn even, wrap Rm odd) --
        (
            "vidup.u8 q0, r0, #1",
            I::MveViddup(false, Size::I8, q(0), R::R0, None, 1),
        ),
        (
            "vidup.u32 q3, r2, #4",
            I::MveViddup(false, Size::I32, q(3), R::R2, None, 4),
        ),
        (
            "vddup.u16 q0, r8, #8",
            I::MveViddup(true, Size::I16, q(0), R::R8, None, 8),
        ),
        (
            "viwdup.u32 q0, r0, r1, #1",
            I::MveViddup(false, Size::I32, q(0), R::R0, Some(R::R1), 1),
        ),
        (
            "vdwdup.u32 q7, r4, r5, #2",
            I::MveViddup(true, Size::I32, q(7), R::R4, Some(R::R5), 2),
        ),
        (
            "viwdup.u8 q2, r6, r3, #8",
            I::MveViddup(false, Size::I8, q(2), R::R6, Some(R::R3), 8),
        ),
        // -- VBRSR (broadcast shift by GPR) --
        (
            "vbrsr.8 q0, q1, r2",
            I::MveVbrsr(Size::I8, q(0), q(1), R::R2),
        ),
        (
            "vbrsr.16 q3, q5, r7",
            I::MveVbrsr(Size::I16, q(3), q(5), R::R7),
        ),
        (
            "vbrsr.32 q7, q0, r0",
            I::MveVbrsr(Size::I32, q(7), q(0), R::R0),
        ),
        // -- gather/scatter (scalar base + vector offset) --
        (
            "vldrb.u8 q0, [r0, q1]",
            I::MveGatherScatter(true, true, 8, 8, false, q(0), R::R0, q(1)),
        ),
        (
            "vldrb.s16 q2, [r1, q3]",
            I::MveGatherScatter(true, false, 16, 8, false, q(2), R::R1, q(3)),
        ),
        (
            "vldrh.u32 q0, [r0, q1]",
            I::MveGatherScatter(true, true, 32, 16, false, q(0), R::R0, q(1)),
        ),
        (
            "vldrw.u32 q7, [r3, q5]",
            I::MveGatherScatter(true, true, 32, 32, false, q(7), R::R3, q(5)),
        ),
        (
            "vldrd.u64 q0, [r0, q1, uxtw #3]",
            I::MveGatherScatter(true, true, 64, 64, true, q(0), R::R0, q(1)),
        ),
        (
            "vldrw.u32 q0, [r0, q1, uxtw #2]",
            I::MveGatherScatter(true, true, 32, 32, true, q(0), R::R0, q(1)),
        ),
        (
            "vstrb.8 q0, [r0, q1]",
            I::MveGatherScatter(false, false, 8, 8, false, q(0), R::R0, q(1)),
        ),
        (
            "vstrb.16 q4, [r5, q6]",
            I::MveGatherScatter(false, false, 16, 8, false, q(4), R::R5, q(6)),
        ),
        (
            "vstrw.32 q0, [r0, q1, uxtw #2]",
            I::MveGatherScatter(false, false, 32, 32, true, q(0), R::R0, q(1)),
        ),
        // -- gather/scatter with a vector base + immediate --
        (
            "vldrw.u32 q0, [q1]",
            I::MveGatherScatterBase(true, false, false, q(0), q(1), 0),
        ),
        (
            "vldrw.u32 q0, [q1, #-4]",
            I::MveGatherScatterBase(true, false, false, q(0), q(1), -4),
        ),
        (
            "vldrw.u32 q3, [q5, #508]",
            I::MveGatherScatterBase(true, false, false, q(3), q(5), 508),
        ),
        (
            "vldrd.u64 q0, [q1, #8]!",
            I::MveGatherScatterBase(true, true, true, q(0), q(1), 8),
        ),
        (
            "vstrw.32 q0, [q1, #4]",
            I::MveGatherScatterBase(false, false, false, q(0), q(1), 4),
        ),
        (
            "vstrd.64 q7, [q2, #-16]!",
            I::MveGatherScatterBase(false, true, true, q(7), q(2), -16),
        ),
        // -- VLD2x/VLD4x/VST2x/VST4x (de-interleaving/interleaving load/store) --
        (
            "vld20.8 {q0, q1}, [r0]",
            I::MveInterleave(true, false, 0, Size::I8, q(0), R::R0, false),
        ),
        (
            "vld21.32 {q2, q3}, [r4]!",
            I::MveInterleave(true, false, 1, Size::I32, q(2), R::R4, true),
        ),
        (
            "vst20.16 {q6, q7}, [r1]",
            I::MveInterleave(false, false, 0, Size::I16, q(6), R::R1, false),
        ),
        (
            "vld40.8 {q0, q1, q2, q3}, [r0]",
            I::MveInterleave(true, true, 0, Size::I8, q(0), R::R0, false),
        ),
        (
            "vld43.16 {q4, q5, q6, q7}, [r2]",
            I::MveInterleave(true, true, 3, Size::I16, q(4), R::R2, false),
        ),
        (
            "vst42.32 {q0, q1, q2, q3}, [r0]!",
            I::MveInterleave(false, true, 2, Size::I32, q(0), R::R0, true),
        ),
        // -- low-overhead loops (the non-branch forms; the branches need labels and are tested elsewhere) --
        ("dls lr, r0", I::LobStart(false, None, R::R0, 0)),
        ("dlstp.16 lr, r3", I::LobStart(false, Some(16), R::R3, 0)),
        ("dlstp.64 lr, r0", I::LobStart(false, Some(64), R::R0, 0)),
        ("lctp", I::Lctp),
        ("vctp.8 r0", I::MveVctp(8, R::R0)),
        ("vctp.16 r1", I::MveVctp(16, R::R1)),
        ("vctp.32 r2", I::MveVctp(32, R::R2)),
        ("vctp.64 r5", I::MveVctp(64, R::R5)),
        // -- VCMP (compare into the VPR) : register, scalar; int, float --
        (
            "vcmp.i8 eq, q0, q1",
            I::MveVcmpReg(CC::Eq, Size::I8, q(0), q(1)),
        ),
        (
            "vcmp.i32 ne, q2, q3",
            I::MveVcmpReg(CC::Ne, Size::I32, q(2), q(3)),
        ),
        (
            "vcmp.s16 ge, q0, q1",
            I::MveVcmpReg(CC::Ge, Size::I16, q(0), q(1)),
        ),
        (
            "vcmp.s32 lt, q0, q1",
            I::MveVcmpReg(CC::Lt, Size::I32, q(0), q(1)),
        ),
        (
            "vcmp.s8 gt, q4, q5",
            I::MveVcmpReg(CC::Gt, Size::I8, q(4), q(5)),
        ),
        (
            "vcmp.s16 le, q0, q1",
            I::MveVcmpReg(CC::Le, Size::I16, q(0), q(1)),
        ),
        (
            "vcmp.u32 cs, q0, q1",
            I::MveVcmpReg(CC::Cs, Size::I32, q(0), q(1)),
        ),
        (
            "vcmp.u8 hi, q6, q7",
            I::MveVcmpReg(CC::Hi, Size::I8, q(6), q(7)),
        ),
        (
            "vcmp.i32 eq, q0, r2",
            I::MveVcmpScalar(CC::Eq, Size::I32, q(0), R::R2),
        ),
        (
            "vcmp.s16 gt, q4, r5",
            I::MveVcmpScalar(CC::Gt, Size::I16, q(4), R::R5),
        ),
        (
            "vcmp.u8 hi, q0, r10",
            I::MveVcmpScalar(CC::Hi, Size::I8, q(0), R::R10),
        ),
        (
            "vcmp.f32 eq, q0, q1",
            I::MveVcmpFloatReg(CC::Eq, FSize::F32, q(0), q(1)),
        ),
        (
            "vcmp.f16 lt, q2, q3",
            I::MveVcmpFloatReg(CC::Lt, FSize::F16, q(2), q(3)),
        ),
        (
            "vcmp.f32 le, q0, q1",
            I::MveVcmpFloatReg(CC::Le, FSize::F32, q(0), q(1)),
        ),
        (
            "vcmp.f32 eq, q0, r2",
            I::MveVcmpFloatScalar(CC::Eq, FSize::F32, q(0), R::R2),
        ),
        (
            "vcmp.f16 gt, q4, r5",
            I::MveVcmpFloatScalar(CC::Gt, FSize::F16, q(4), R::R5),
        ),
        // -- floating-point min/max reductions --
        (
            "vmaxnmv.f32 r0, q1",
            I::MveFloatReduce(FRop::Vmaxnmv, FSize::F32, R::R0, q(1)),
        ),
        (
            "vminnmv.f16 r2, q3",
            I::MveFloatReduce(FRop::Vminnmv, FSize::F16, R::R2, q(3)),
        ),
        (
            "vmaxnmav.f32 r5, q7",
            I::MveFloatReduce(FRop::Vmaxnmav, FSize::F32, R::R5, q(7)),
        ),
        (
            "vminnmav.f16 r9, q0",
            I::MveFloatReduce(FRop::Vminnmav, FSize::F16, R::R9, q(0)),
        ),
        // -- vector-by-vector saturating doubling multiply-high (signed-only) --
        (
            "vqdmulh.s8 q0, q1, q2",
            I::MveIntArith(Iop::VqdmulhS, Size::I8, q(0), q(1), q(2)),
        ),
        (
            "vqdmulh.s32 q3, q4, q5",
            I::MveIntArith(Iop::VqdmulhS, Size::I32, q(3), q(4), q(5)),
        ),
        (
            "vqrdmulh.s16 q0, q7, q1",
            I::MveIntArith(Iop::VqrdmulhS, Size::I16, q(0), q(7), q(1)),
        ),
        // -- elementwise absolute min/max (VMAXA/VMINA) and their FP twins (VMAXNMA/VMINNMA) --
        (
            "vmaxa.s8 q0, q1",
            I::MveVmaxaMina(false, Size::I8, q(0), q(1)),
        ),
        (
            "vmaxa.s32 q3, q4",
            I::MveVmaxaMina(false, Size::I32, q(3), q(4)),
        ),
        (
            "vmina.s16 q5, q6",
            I::MveVmaxaMina(true, Size::I16, q(5), q(6)),
        ),
        (
            "vmaxnma.f16 q0, q1",
            I::MveVmaxnmaMinnma(false, FSize::F16, q(0), q(1)),
        ),
        (
            "vmaxnma.f32 q2, q3",
            I::MveVmaxnmaMinnma(false, FSize::F32, q(2), q(3)),
        ),
        (
            "vminnma.f32 q7, q0",
            I::MveVmaxnmaMinnma(true, FSize::F32, q(7), q(0)),
        ),
        // -- VMOV between two GPRs and two 32-bit vector lanes --
        (
            "vmov q0[2], q0[0], r0, r1",
            I::MveVmovTwoLane(true, 2, q(0), R::R0, R::R1),
        ),
        (
            "vmov q7[3], q7[1], r4, r5",
            I::MveVmovTwoLane(true, 3, q(7), R::R4, R::R5),
        ),
        (
            "vmov r10, r11, q3[2], q3[0]",
            I::MveVmovTwoLane(false, 2, q(3), R::R10, R::R11),
        ),
        // -- scalar v8-M FP additions (armv8.1-m.main + fp.dp): VMAXNM/VMINNM, directed VRINT/VCVT, VRINTX/Z,
        //    and VMOV core <-> scalar-lane. These share the A32 unconditional-FP words in T32 framing. --
        (
            "vmaxnm.f32 s0, s1, s2",
            I::Vmaxnm_Single_T1(s(0), s(1), s(2)),
        ),
        (
            "vmaxnm.f64 d0, d1, d2",
            I::Vmaxnm_Double_T1(d(0), d(1), d(2)),
        ),
        (
            "vminnm.f64 d3, d4, d5",
            I::Vminnm_Double_T1(d(3), d(4), d(5)),
        ),
        (
            "vrinta.f32 s0, s1",
            I::Vrint_Directed_Single_T1(DR::A, s(0), s(1)),
        ),
        (
            "vrintm.f64 d0, d1",
            I::Vrint_Directed_Double_T1(DR::M, d(0), d(1)),
        ),
        ("vrintz.f32 s3, s4", I::Vrintz_Single_T1(s(3), s(4))),
        ("vrintx.f64 d3, d4", I::Vrintx_Double_T1(d(3), d(4))),
        (
            "vcvta.s32.f32 s0, s1",
            I::Vcvt_Directed_FromSingle_T1(DR::A, s(0), s(1), true),
        ),
        (
            "vcvtm.u32.f64 s8, d9",
            I::Vcvt_Directed_FromDouble_T1(DR::M, s(8), d(9), false),
        ),
        (
            "vmov.32 d0[0], r1",
            I::Vmov_Core_To_Scalar_T1(VLS::Word, 0, d(0), R::R1),
        ),
        (
            "vmov.s16 r5, d3[1]",
            I::Vmov_Scalar_To_Core_T1(false, VLS::Half, 1, R::R5, d(3)),
        ),
    ]
}

// ---- minimal GNU backend (Thumb / MVE mode) ----

struct GnuBackend {
    assembler: String,
    objcopy: String,
}

impl GnuBackend {
    fn assemble_and_extract(&self, source: &str) -> Vec<u8> {
        let work = WorkDir::new("arm32_oracle_mve");
        let source_path = work.path.join("oracle.s");
        let object_path = work.path.join("oracle.o");
        let binary_path = work.path.join("oracle.bin");
        std::fs::write(&source_path, source).expect("write .s");

        run(
            Command::new(&self.assembler)
                .args(["-march=armv8.1-m.main+mve.fp+fp.dp"]) // Cortex-M55/M85: MVE int+fp; +fp.dp for scalar f64 (VMAXNM.f64/VRINT.f64/...)
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

// ---- the SECOND oracle: LLVM (`llvm-mc`, Thumb / MVE) ----
//
// GNU = Linux servers, LLVM = Apple-silicon macOS -- both PRIMARY. This is the oracle pairing that EXPOSED the
// MVE VMLA-by-scalar bug (GNU and our model shared it; LLVM + DDI0553 caught it). Wiring llvm-mc in permanently
// locks the GNU-vs-LLVM agreement on the rest of the MVE surface as a regression. Assembled per instruction so
// any form llvm-mc declines is skipped, not a hard failure. Skips green if no llvm-mc is found.

struct LlvmBackend {
    mc: String,
    objcopy: String,
}
impl LlvmBackend {
    fn assemble_one(&self, ual: &str) -> Option<Vec<u8>> {
        let work = WorkDir::new("arm32_oracle_mve_llvm");
        let source = work.path.join("one.s");
        let object = work.path.join("one.o");
        let binary = work.path.join("one.bin");
        std::fs::write(&source, format!(".syntax unified\n.thumb\n.text\n{ual}\n")).ok()?;

        let assembled = Command::new(&self.mc)
            .args([
                "-triple=thumbv8.1m.main",
                "-mattr=+mve.fp,+fp64",
                "-filetype=obj",
            ])
            .arg(&source)
            .arg("-o")
            .arg(&object)
            .output()
            .ok()?;
        if !assembled.status.success() {
            return None;
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

#[test]
#[cfg_attr(miri, ignore)]
fn llvm_assembler_matches_our_mve_encoder() {
    let backend = match discover_llvm_backend() {
        Some(backend) => backend,
        None => {
            common::skip_or_require("differential_oracle_mve (LLVM): no llvm-mc found");
            return;
        }
    };

    let samples = sample_instructions();
    let (mut checked, mut skipped) = (0usize, 0usize);
    for (ual, instruction) in &samples {
        let Some(theirs) = backend.assemble_one(ual) else {
            skipped += 1;
            eprintln!("  llvm-mc declined `{ual}` -- validated against GNU instead");
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
        "differential_oracle_mve (LLVM): validated {checked} MVE instructions, skipped {skipped} llvm-incompatible"
    );
    assert!(
        checked > 250,
        "LLVM validated only {checked} MVE samples -- oracle likely misconfigured (wrong triple/features?)"
    );
    assert!(
        skipped <= 5,
        "llvm-mc declined {skipped} MVE samples -- a new GNU/LLVM divergence to investigate"
    );
}
