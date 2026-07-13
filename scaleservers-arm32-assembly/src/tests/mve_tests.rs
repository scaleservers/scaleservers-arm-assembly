// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// ARMv8.1-M MVE ("Helium") tests: the "three registers of the same length" vector-vector data-processing
// format (integer, bitwise and floating-point sub-families). Exact encodings are cross-checked against
// `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb mode); each form round-trips (encode -> decode ->
// encode) and is gated behind the v8.1-M ISA version + the Mve / MveFloat features (so a plain ARMv8-M
// Mainline target, which has no vector unit, REFUSES them).

use crate::enums::{
    Arm32GeneralPurposeRegister as R, Arm32MveBitwiseOp, Arm32MveFloatArithOp,
    Arm32MveFloatReduceOp, Arm32MveFloatSize, Arm32MveIntArithOp, Arm32MveLongMacOp,
    Arm32MveMisc2FloatOp, Arm32MveMisc2Op, Arm32MveQMovnKind, Arm32MveReduceOp, Arm32MveShiftImmOp,
    Arm32MveShiftNarrowOp, Arm32MveSize, Arm32MveVcmpCondition, Arm32MveVecScalarFloatOp,
    Arm32MveVecScalarIntOp, Arm32MveVectorRegister as Q, Arm32MveVrintOp, ArmT32IndexMode,
};
use crate::{
    ArmCpuFeature, ArmDecodeContext, ArmInstructionRequirement, ArmIsaVersion, ArmT32Instruction,
    ArmTargetProfile,
};

fn q(n: u8) -> Q {
    Q::new(n).unwrap()
}

fn round_trip(instruction: &ArmT32Instruction) {
    let bytes = instruction.encode().unwrap();
    let mut offset = 0;
    let decoded = ArmT32Instruction::decode(&mut bytes.iter(), &mut offset)
        .unwrap()
        .unwrap();
    assert_eq!(offset, bytes.len(), "consumed wrong byte count");
    assert_eq!(
        &decoded, instruction,
        "decode did not reproduce the encoded instruction"
    );
}

#[test]
fn encode__mve_3reg_exact_bytes() {
    use Arm32MveIntArithOp::*;
    use ArmT32Instruction::{MveBitwise, MveFloatArith, MveIntArith};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    // integer arithmetic
    assert_eq!(
        MveIntArith(Vadd, Arm32MveSize::I32, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x22, 0xef, 0x44, 0x08]
    ); // vadd.i32 q0, q1, q2
    assert_eq!(
        MveIntArith(Vadd, Arm32MveSize::I16, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x12, 0xef, 0x44, 0x08]
    ); // vadd.i16 q0, q1, q2
    assert_eq!(
        MveIntArith(Vsub, Arm32MveSize::I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xff, 0x44, 0x08]
    ); // vsub.i8  q0, q1, q2
    assert_eq!(
        MveIntArith(Vmul, Arm32MveSize::I32, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x22, 0xef, 0x54, 0x09]
    ); // vmul.i32 q0, q1, q2
    assert_eq!(
        MveIntArith(VqaddU, Arm32MveSize::I8, q(0), q(0), q(0))
            .encode()
            .unwrap(),
        vec![0x00, 0xff, 0x50, 0x00]
    ); // vqadd.u8 q0, q0, q0
    assert_eq!(
        MveIntArith(VhaddS, Arm32MveSize::I8, q(0), q(0), q(0))
            .encode()
            .unwrap(),
        vec![0x00, 0xef, 0x40, 0x00]
    ); // vhadd.s8 q0, q0, q0
    assert_eq!(
        MveIntArith(VqdmulhS, Arm32MveSize::I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xef, 0x44, 0x0b]
    ); // vqdmulh.s8  q0, q1, q2
    assert_eq!(
        MveIntArith(VqdmulhS, Arm32MveSize::I32, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x22, 0xef, 0x44, 0x0b]
    ); // vqdmulh.s32 q0, q1, q2
    assert_eq!(
        MveIntArith(VqrdmulhS, Arm32MveSize::I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xff, 0x44, 0x0b]
    ); // vqrdmulh.s8 q0, q1, q2
    // bitwise
    assert_eq!(
        MveBitwise(Arm32MveBitwiseOp::Vand, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xef, 0x54, 0x01]
    ); // vand q0, q1, q2
    assert_eq!(
        MveBitwise(Arm32MveBitwiseOp::Vorr, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x22, 0xef, 0x54, 0x01]
    ); // vorr q0, q1, q2
    assert_eq!(
        MveBitwise(Arm32MveBitwiseOp::Veor, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xff, 0x54, 0x01]
    ); // veor q0, q1, q2
    // floating-point
    assert_eq!(
        MveFloatArith(
            Arm32MveFloatArithOp::Vadd,
            Arm32MveFloatSize::F32,
            q(0),
            q(1),
            q(2)
        )
        .encode()
        .unwrap(),
        vec![0x02, 0xef, 0x44, 0x0d]
    ); // vadd.f32 q0, q1, q2
    assert_eq!(
        MveFloatArith(
            Arm32MveFloatArithOp::Vadd,
            Arm32MveFloatSize::F16,
            q(0),
            q(1),
            q(2)
        )
        .encode()
        .unwrap(),
        vec![0x12, 0xef, 0x44, 0x0d]
    ); // vadd.f16 q0, q1, q2
    assert_eq!(
        MveFloatArith(
            Arm32MveFloatArithOp::Vmul,
            Arm32MveFloatSize::F16,
            q(0),
            q(0),
            q(0)
        )
        .encode()
        .unwrap(),
        vec![0x10, 0xff, 0x50, 0x0d]
    ); // vmul.f16 q0, q0, q0
    assert_eq!(
        MveFloatArith(
            Arm32MveFloatArithOp::Vmaxnm,
            Arm32MveFloatSize::F32,
            q(0),
            q(0),
            q(0)
        )
        .encode()
        .unwrap(),
        vec![0x00, 0xff, 0x50, 0x0f]
    ); // vmaxnm.f32 q0, q0, q0
    assert_eq!(
        MveFloatArith(
            Arm32MveFloatArithOp::Vfma,
            Arm32MveFloatSize::F32,
            q(0),
            q(0),
            q(0)
        )
        .encode()
        .unwrap(),
        vec![0x00, 0xef, 0x50, 0x0c]
    ); // vfma.f32 q0, q0, q0
}

#[test]
fn round_trip__mve_3reg_exhaustive() {
    // every integer op x every element size x a few register triples
    let triples = [(0u8, 0u8, 0u8), (0, 1, 2), (7, 3, 5), (2, 7, 1), (5, 5, 7)];
    for op in Arm32MveIntArithOp::ALL {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            for (d, n, m) in triples {
                round_trip(&ArmT32Instruction::MveIntArith(op, size, q(d), q(n), q(m)));
            }
        }
    }
    // every bitwise op
    for op in Arm32MveBitwiseOp::ALL {
        for (d, n, m) in triples {
            round_trip(&ArmT32Instruction::MveBitwise(op, q(d), q(n), q(m)));
        }
    }
    // every float op x both sizes
    for op in Arm32MveFloatArithOp::ALL {
        for size in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
            for (d, n, m) in triples {
                round_trip(&ArmT32Instruction::MveFloatArith(
                    op,
                    size,
                    q(d),
                    q(n),
                    q(m),
                ));
            }
        }
    }
}

#[test]
fn encode__mve_vector_by_scalar_and_vdup_exact_bytes() {
    use ArmT32Instruction::{MveVdup, MveVecScalarFloat, MveVecScalarInt};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveVecScalarInt(
            Arm32MveVecScalarIntOp::Vadd,
            Arm32MveSize::I32,
            q(0),
            q(0),
            R::R0
        )
        .encode()
        .unwrap(),
        vec![0x21, 0xee, 0x40, 0x0f]
    ); // vadd.i32 q0, q0, r0
    assert_eq!(
        MveVecScalarInt(
            Arm32MveVecScalarIntOp::Vsub,
            Arm32MveSize::I8,
            q(0),
            q(0),
            R::R0
        )
        .encode()
        .unwrap(),
        vec![0x01, 0xee, 0x40, 0x1f]
    ); // vsub.i8  q0, q0, r0
    assert_eq!(
        MveVecScalarInt(
            Arm32MveVecScalarIntOp::VqdmulhS,
            Arm32MveSize::I32,
            q(0),
            q(0),
            R::R0
        )
        .encode()
        .unwrap(),
        vec![0x21, 0xee, 0x60, 0x0e]
    ); // vqdmulh.s32 q0, q0, r0
    assert_eq!(
        MveVecScalarInt(
            Arm32MveVecScalarIntOp::VhaddU,
            Arm32MveSize::I8,
            q(0),
            q(0),
            R::R0
        )
        .encode()
        .unwrap(),
        vec![0x00, 0xfe, 0x40, 0x0f]
    ); // vhadd.u8 q0, q0, r0
    assert_eq!(
        MveVecScalarFloat(
            Arm32MveVecScalarFloatOp::Vadd,
            Arm32MveFloatSize::F32,
            q(0),
            q(0),
            R::R0
        )
        .encode()
        .unwrap(),
        vec![0x30, 0xee, 0x40, 0x0f]
    ); // vadd.f32 q0, q0, r0
    assert_eq!(
        MveVecScalarFloat(
            Arm32MveVecScalarFloatOp::Vadd,
            Arm32MveFloatSize::F16,
            q(0),
            q(0),
            R::R0
        )
        .encode()
        .unwrap(),
        vec![0x30, 0xfe, 0x40, 0x0f]
    ); // vadd.f16 q0, q0, r0
    // VDUP: the {B,E} size pair and Qd[19:17] / Rt[15:12]
    assert_eq!(
        MveVdup(Arm32MveSize::I32, q(0), R::R1).encode().unwrap(),
        vec![0xa0, 0xee, 0x10, 0x1b]
    ); // vdup.32 q0, r1
    assert_eq!(
        MveVdup(Arm32MveSize::I16, q(0), R::R1).encode().unwrap(),
        vec![0xa0, 0xee, 0x30, 0x1b]
    ); // vdup.16 q0, r1
    assert_eq!(
        MveVdup(Arm32MveSize::I8, q(0), R::R1).encode().unwrap(),
        vec![0xe0, 0xee, 0x10, 0x1b]
    ); // vdup.8  q0, r1
    assert_eq!(
        MveVdup(Arm32MveSize::I32, q(7), R::R12).encode().unwrap(),
        vec![0xae, 0xee, 0x10, 0xcb]
    ); // vdup.32 q7, r12
}

#[test]
fn encode__mve_multiply_accumulate_exact_bytes() {
    use Arm32MveVecScalarIntOp::*;
    use ArmT32Instruction::{MveVecScalarFloat, MveVecScalarInt};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    // VMLA/VMLAS (vector by scalar) are signedness-agnostic: DDI0553 C2.4.380/C2.4.384 fix bit 28 = (0) and
    // list <dt> = I8/I16/I32 only. Bytes here are anchored to LLVM (`llvm-mc -triple=thumbv8.1m.main`) + the
    // spec, NOT GNU -- GNU wrongly sets bit 28 for `.u` and rejects the correct `.i` form (the dual-oracle find).
    assert_eq!(
        MveVecScalarInt(Vmla, Arm32MveSize::I8, q(0), q(1), R::R2)
            .encode()
            .unwrap(),
        vec![0x03, 0xee, 0x42, 0x0e]
    ); // vmla.i8  q0, q1, r2
    assert_eq!(
        MveVecScalarInt(Vmla, Arm32MveSize::I16, q(0), q(1), R::R2)
            .encode()
            .unwrap(),
        vec![0x13, 0xee, 0x42, 0x0e]
    ); // vmla.i16 q0, q1, r2 (bit 28 = 0, not GNU's 0xfe)
    assert_eq!(
        MveVecScalarInt(Vmlas, Arm32MveSize::I8, q(0), q(1), R::R2)
            .encode()
            .unwrap(),
        vec![0x03, 0xee, 0x42, 0x1e]
    ); // vmlas.i8 q0, q1, r2
    assert_eq!(
        MveVecScalarInt(VqdmlahS, Arm32MveSize::I8, q(0), q(1), R::R2)
            .encode()
            .unwrap(),
        vec![0x02, 0xee, 0x62, 0x0e]
    ); // vqdmlah.s8 q0, q1, r2
    assert_eq!(
        MveVecScalarInt(VqrdmlahS, Arm32MveSize::I16, q(0), q(1), R::R2)
            .encode()
            .unwrap(),
        vec![0x12, 0xee, 0x42, 0x0e]
    ); // vqrdmlah.s16 q0, q1, r2
    assert_eq!(
        MveVecScalarInt(VqdmlashS, Arm32MveSize::I8, q(0), q(1), R::R2)
            .encode()
            .unwrap(),
        vec![0x02, 0xee, 0x62, 0x1e]
    ); // vqdmlash.s8 q0, q1, r2
    assert_eq!(
        MveVecScalarInt(VqrdmlashS, Arm32MveSize::I32, q(0), q(1), R::R2)
            .encode()
            .unwrap(),
        vec![0x22, 0xee, 0x42, 0x1e]
    ); // vqrdmlash.s32 q0, q1, r2
    assert_eq!(
        MveVecScalarFloat(
            Arm32MveVecScalarFloatOp::Vfma,
            Arm32MveFloatSize::F32,
            q(0),
            q(1),
            R::R2
        )
        .encode()
        .unwrap(),
        vec![0x33, 0xee, 0x42, 0x0e]
    ); // vfma.f32 q0, q1, r2
    assert_eq!(
        MveVecScalarFloat(
            Arm32MveVecScalarFloatOp::Vfmas,
            Arm32MveFloatSize::F16,
            q(0),
            q(1),
            R::R2
        )
        .encode()
        .unwrap(),
        vec![0x33, 0xfe, 0x42, 0x1e]
    ); // vfmas.f16 q0, q1, r2
}

