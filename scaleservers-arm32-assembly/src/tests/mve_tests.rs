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

#[test]
fn mve_vmla_by_scalar_with_bit28_set_does_not_decode_as_vmla() {
    // GNU mis-encodes `vmla.u16 q3,q4,r5` as 0xfe19_6e45 (bit 28 = 1). DDI0553 C2.4.380 fixes bit 28 to (0),
    // so that pattern is NOT a valid VMLA. After the fix the spurious `*U` base words are gone, so our decoder
    // must not resurrect it as a VMLA (it falls through -- Err/None/other -- but never MveVecScalarInt(Vmla)).
    let bytes = [0x19u8, 0xfe, 0x45, 0x6e]; // hw1 = 0xfe19 (bit 28 set), hw2 = 0x6e45
    let mut offset = 0;
    if let Ok(Some(instruction)) = ArmT32Instruction::decode(&mut bytes.iter(), &mut offset) {
        assert!(
            !matches!(
                instruction,
                ArmT32Instruction::MveVecScalarInt(Arm32MveVecScalarIntOp::Vmla, ..)
            ),
            "bit-28-set VMLA pattern wrongly decoded as VMLA: {instruction:?}"
        );
    }
}

#[test]
fn round_trip__mve_vector_by_scalar_and_vdup() {
    let gprs = [R::R0, R::R3, R::R7, R::R12, R::R14];
    // integer vector-by-scalar: every op x every size x register combos
    for op in Arm32MveVecScalarIntOp::ALL {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            for (d, n) in [(0u8, 0u8), (7, 3), (2, 7)] {
                for rm in gprs {
                    round_trip(&ArmT32Instruction::MveVecScalarInt(
                        op,
                        size,
                        q(d),
                        q(n),
                        rm,
                    ));
                }
            }
        }
    }
    // float vector-by-scalar: every op x both sizes
    for op in Arm32MveVecScalarFloatOp::ALL {
        for size in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
            for rm in gprs {
                round_trip(&ArmT32Instruction::MveVecScalarFloat(
                    op,
                    size,
                    q(1),
                    q(4),
                    rm,
                ));
            }
        }
    }
    // VDUP: every size x every Qd x assorted Rt
    for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
        for d in 0..8u8 {
            for rt in gprs {
                round_trip(&ArmT32Instruction::MveVdup(size, q(d), rt));
            }
        }
    }
}

#[test]
fn encode__mve_shift_by_immediate_exact_bytes() {
    use Arm32MveShiftImmOp::*;
    use ArmT32Instruction::MveShiftImm;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveShiftImm(VshrS, Arm32MveSize::I8, 1, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x8f, 0xef, 0x52, 0x00]
    ); // vshr.s8  q0, q1, #1
    assert_eq!(
        MveShiftImm(VshrU, Arm32MveSize::I32, 32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xa0, 0xff, 0x52, 0x00]
    ); // vshr.u32 q0, q1, #32
    assert_eq!(
        MveShiftImm(VrshrS, Arm32MveSize::I16, 2, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x9e, 0xef, 0x52, 0x02]
    ); // vrshr.s16 q0, q1, #2
    assert_eq!(
        MveShiftImm(VshlI, Arm32MveSize::I32, 3, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xa3, 0xef, 0x52, 0x05]
    ); // vshl.i32 q0, q1, #3
    assert_eq!(
        MveShiftImm(VqshlS, Arm32MveSize::I8, 1, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x89, 0xef, 0x52, 0x07]
    ); // vqshl.s8 q0, q1, #1
    assert_eq!(
        MveShiftImm(VqshluS, Arm32MveSize::I32, 5, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xa5, 0xff, 0x52, 0x06]
    ); // vqshlu.s32 q0, q1, #5
    assert_eq!(
        MveShiftImm(Vsli, Arm32MveSize::I32, 7, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xa7, 0xff, 0x52, 0x05]
    ); // vsli.32 q0, q1, #7
    assert_eq!(
        MveShiftImm(Vsri, Arm32MveSize::I32, 8, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb8, 0xff, 0x52, 0x04]
    ); // vsri.32 q0, q1, #8
}

#[test]
fn round_trip__mve_shift_by_immediate() {
    // for each op, sweep every element size and every valid amount for that size+direction
    for op in Arm32MveShiftImmOp::ALL {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            let esize: u8 = match size {
                Arm32MveSize::I8 => 8,
                Arm32MveSize::I16 => 16,
                Arm32MveSize::I32 => 32,
            };
            let amounts: Vec<u8> = if op.is_left_shift() {
                (0..esize).collect()
            } else {
                (1..=esize).collect()
            };
            for amount in amounts {
                for (d, m) in [(0u8, 0u8), (7, 1), (3, 5), (2, 7)] {
                    round_trip(&ArmT32Instruction::MveShiftImm(
                        op,
                        size,
                        amount,
                        q(d),
                        q(m),
                    ));
                }
            }
        }
    }
}

#[test]
fn encode__mve_modified_immediate_exact_bytes() {
    use ArmT32Instruction::MveModifiedImmediate as MI;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MI(0b1110, false, 0x12, q(0)).encode().unwrap(),
        vec![0x81, 0xef, 0x52, 0x0e]
    ); // vmov.i8  q0, #0x12
    assert_eq!(
        MI(0b1000, false, 0x12, q(0)).encode().unwrap(),
        vec![0x81, 0xef, 0x52, 0x08]
    ); // vmov.i16 q0, #0x12
    assert_eq!(
        MI(0b0000, false, 0x12, q(0)).encode().unwrap(),
        vec![0x81, 0xef, 0x52, 0x00]
    ); // vmov.i32 q0, #0x12
    assert_eq!(
        MI(0b0110, false, 0x12, q(0)).encode().unwrap(),
        vec![0x81, 0xef, 0x52, 0x06]
    ); // vmov.i32 q0, #0x12000000
    assert_eq!(
        MI(0b1100, false, 0x12, q(0)).encode().unwrap(),
        vec![0x81, 0xef, 0x52, 0x0c]
    ); // vmov.i32 q0, #0x12ff
    assert_eq!(
        MI(0b1000, true, 0x12, q(0)).encode().unwrap(),
        vec![0x81, 0xef, 0x72, 0x08]
    ); // vmvn.i16 q0, #0x12
    assert_eq!(
        MI(0b1110, true, 0xaa, q(0)).encode().unwrap(),
        vec![0x82, 0xff, 0x7a, 0x0e]
    ); // vmov.i64 q0, #0xff00ff00ff00ff00
    assert_eq!(
        MI(0b1111, false, 0x70, q(0)).encode().unwrap(),
        vec![0x87, 0xef, 0x50, 0x0f]
    ); // vmov.f32 q0, #1.0
    assert_eq!(
        MI(0b1110, false, 0xff, q(7)).encode().unwrap(),
        vec![0x87, 0xff, 0x5f, 0xee]
    ); // vmov.i8  q7, #0xff
}

#[test]
fn round_trip__mve_modified_immediate_codec() {
    // the raw (cmode, op, imm8, qd) bits must survive encode -> decode -> encode for EVERY combination
    for cmode in 0u8..16 {
        for op in [false, true] {
            for imm8 in [0x00u8, 0x12, 0x55, 0xaa, 0xff, 0x80, 0x01] {
                for d in 0..8u8 {
                    round_trip(&ArmT32Instruction::MveModifiedImmediate(
                        cmode,
                        op,
                        imm8,
                        q(d),
                    ));
                }
            }
        }
    }
}

#[test]
fn encode__mve_2reg_misc_exact_bytes() {
    use Arm32MveMisc2Op::*;
    use ArmT32Instruction::{MveMisc2, MveMisc2Float, MveMvnRegister};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveMisc2(Vrev64, Arm32MveSize::I8, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb0, 0xff, 0x42, 0x00]
    ); // vrev64.8  q0, q1
    assert_eq!(
        MveMisc2(Vrev32, Arm32MveSize::I16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb4, 0xff, 0xc2, 0x00]
    ); // vrev32.16 q0, q1
    assert_eq!(
        MveMisc2(Vcls, Arm32MveSize::I16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb4, 0xff, 0x42, 0x04]
    ); // vcls.s16  q0, q1
    assert_eq!(
        MveMisc2(Vclz, Arm32MveSize::I32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb8, 0xff, 0xc2, 0x04]
    ); // vclz.i32  q0, q1
    assert_eq!(
        MveMisc2(Vabs, Arm32MveSize::I8, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb1, 0xff, 0x42, 0x03]
    ); // vabs.s8   q0, q1
    assert_eq!(
        MveMisc2(Vneg, Arm32MveSize::I32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb9, 0xff, 0xc2, 0x03]
    ); // vneg.s32  q0, q1
    assert_eq!(
        MveMisc2(Vqabs, Arm32MveSize::I8, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb0, 0xff, 0x42, 0x07]
    ); // vqabs.s8  q0, q1
    assert_eq!(
        MveMisc2Float(
            Arm32MveMisc2FloatOp::Vabs,
            Arm32MveFloatSize::F16,
            q(0),
            q(1)
        )
        .encode()
        .unwrap(),
        vec![0xb5, 0xff, 0x42, 0x07]
    ); // vabs.f16 q0, q1
    assert_eq!(
        MveMisc2Float(
            Arm32MveMisc2FloatOp::Vneg,
            Arm32MveFloatSize::F32,
            q(0),
            q(1)
        )
        .encode()
        .unwrap(),
        vec![0xb9, 0xff, 0xc2, 0x07]
    ); // vneg.f32 q0, q1
    assert_eq!(
        MveMvnRegister(q(0), q(1)).encode().unwrap(),
        vec![0xb0, 0xff, 0xc2, 0x05]
    ); // vmvn q0, q1
}

#[test]
fn round_trip__mve_2reg_misc() {
    for op in Arm32MveMisc2Op::ALL {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            for (d, m) in [(0u8, 0u8), (7, 1), (3, 5), (2, 7)] {
                round_trip(&ArmT32Instruction::MveMisc2(op, size, q(d), q(m)));
            }
        }
    }
    for op in Arm32MveMisc2FloatOp::ALL {
        for size in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
            for (d, m) in [(0u8, 0u8), (7, 1), (4, 6)] {
                round_trip(&ArmT32Instruction::MveMisc2Float(op, size, q(d), q(m)));
            }
        }
    }
    for d in 0..8u8 {
        for m in 0..8u8 {
            round_trip(&ArmT32Instruction::MveMvnRegister(q(d), q(m)));
        }
    }
}

#[test]
fn mve_vmaxa_vmina_and_float_twins() {
    use Arm32MveFloatSize::{F16, F32};
    use Arm32MveSize::{I8, I16, I32};
    use ArmT32Instruction::{MveVmaxaMina, MveVmaxnmaMinnma};
    // bytes dual-oracle verified (arm-none-eabi-as -march=armv8.1-m.main+mve.fp+fp.dp + llvm-mc thumbv8.1m.main)
    assert_eq!(
        MveVmaxaMina(false, I8, q(0), q(1)).encode().unwrap(),
        vec![0x33, 0xee, 0x83, 0x0e]
    ); // vmaxa.s8  q0, q1
    assert_eq!(
        MveVmaxaMina(false, I16, q(0), q(1)).encode().unwrap(),
        vec![0x37, 0xee, 0x83, 0x0e]
    ); // vmaxa.s16 q0, q1
    assert_eq!(
        MveVmaxaMina(false, I32, q(0), q(1)).encode().unwrap(),
        vec![0x3b, 0xee, 0x83, 0x0e]
    ); // vmaxa.s32 q0, q1
    assert_eq!(
        MveVmaxaMina(true, I8, q(0), q(1)).encode().unwrap(),
        vec![0x33, 0xee, 0x83, 0x1e]
    ); // vmina.s8  q0, q1
    assert_eq!(
        MveVmaxaMina(true, I32, q(0), q(1)).encode().unwrap(),
        vec![0x3b, 0xee, 0x83, 0x1e]
    ); // vmina.s32 q0, q1
    assert_eq!(
        MveVmaxnmaMinnma(false, F16, q(0), q(1)).encode().unwrap(),
        vec![0x3f, 0xfe, 0x83, 0x0e]
    ); // vmaxnma.f16 q0, q1
    assert_eq!(
        MveVmaxnmaMinnma(false, F32, q(0), q(1)).encode().unwrap(),
        vec![0x3f, 0xee, 0x83, 0x0e]
    ); // vmaxnma.f32 q0, q1
    assert_eq!(
        MveVmaxnmaMinnma(true, F16, q(0), q(1)).encode().unwrap(),
        vec![0x3f, 0xfe, 0x83, 0x1e]
    ); // vminnma.f16 q0, q1
    assert_eq!(
        MveVmaxnmaMinnma(true, F32, q(0), q(1)).encode().unwrap(),
        vec![0x3f, 0xee, 0x83, 0x1e]
    ); // vminnma.f32 q0, q1
    assert_eq!(
        MveVmaxaMina(false, I8, q(0), q(1)).to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vmaxa.s8 q0, q1"
    );
    assert_eq!(
        MveVmaxnmaMinnma(true, F32, q(0), q(1))
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vminnma.f32 q0, q1"
    );
    // full round-trip (also exercises the decode collision net across the 0xEE space)
    for is_min in [false, true] {
        for (d, m) in [(0u8, 0u8), (7, 1), (3, 5), (2, 7)] {
            for size in [I8, I16, I32] {
                round_trip(&MveVmaxaMina(is_min, size, q(d), q(m)));
            }
            for size in [F16, F32] {
                round_trip(&MveVmaxnmaMinnma(is_min, size, q(d), q(m)));
            }
        }
    }
}

#[test]
fn mve_vmov_two_lanes() {
    use ArmT32Instruction::MveVmovTwoLane;
    // bytes dual-oracle verified (arm-none-eabi-as -march=armv8.1-m.main+mve.fp+fp.dp + llvm-mc thumbv8.1m.main)
    assert_eq!(
        MveVmovTwoLane(true, 2, q(0), R::R0, R::R1)
            .encode()
            .unwrap(),
        vec![0x11, 0xec, 0x00, 0x0f]
    ); // vmov q0[2], q0[0], r0, r1
    assert_eq!(
        MveVmovTwoLane(true, 3, q(0), R::R0, R::R1)
            .encode()
            .unwrap(),
        vec![0x11, 0xec, 0x10, 0x0f]
    ); // vmov q0[3], q0[1], r0, r1
    assert_eq!(
        MveVmovTwoLane(true, 2, q(7), R::R0, R::R1)
            .encode()
            .unwrap(),
        vec![0x11, 0xec, 0x00, 0xef]
    ); // vmov q7[2], q7[0], r0, r1
    assert_eq!(
        MveVmovTwoLane(true, 2, q(0), R::R8, R::R9)
            .encode()
            .unwrap(),
        vec![0x19, 0xec, 0x08, 0x0f]
    ); // vmov q0[2], q0[0], r8, r9
    assert_eq!(
        MveVmovTwoLane(false, 2, q(0), R::R0, R::R1)
            .encode()
            .unwrap(),
        vec![0x01, 0xec, 0x00, 0x0f]
    ); // vmov r0, r1, q0[2], q0[0]
    assert_eq!(
        MveVmovTwoLane(false, 3, q(0), R::R0, R::R1)
            .encode()
            .unwrap(),
        vec![0x01, 0xec, 0x10, 0x0f]
    ); // vmov r0, r1, q0[3], q0[1]
    assert_eq!(
        MveVmovTwoLane(false, 3, q(7), R::R10, R::R11)
            .encode()
            .unwrap(),
        vec![0x0b, 0xec, 0x1a, 0xef]
    ); // vmov r10, r11, q7[3], q7[1]
    assert_eq!(
        MveVmovTwoLane(true, 2, q(0), R::R0, R::R1)
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vmov q0[2], q0[0], r0, r1"
    );
    assert_eq!(
        MveVmovTwoLane(false, 3, q(7), R::R10, R::R11)
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vmov r10, r11, q7[3], q7[1]"
    );
    // full round-trip over both directions, both lane pairs, and a spread of registers
    for to_vector in [false, true] {
        for idx1 in [2u8, 3] {
            for d in 0..8u8 {
                round_trip(&MveVmovTwoLane(to_vector, idx1, q(d), R::R0, R::R1));
                round_trip(&MveVmovTwoLane(to_vector, idx1, q(d), R::R12, R::R7));
            }
        }
    }
}

#[test]
fn encode__mve_load_store_exact_bytes() {
    use Arm32MveSize::*;
    use ArmT32IndexMode::*;
    use ArmT32Instruction::MveLoadStore as LS;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        LS(true, I32, q(0), R::R0, 0, Offset).encode().unwrap(),
        vec![0x90, 0xed, 0x00, 0x1f]
    ); // vldrw.u32 q0, [r0]
    assert_eq!(
        LS(true, I32, q(0), R::R0, 16, Offset).encode().unwrap(),
        vec![0x90, 0xed, 0x04, 0x1f]
    ); // vldrw.u32 q0, [r0, #16]
    assert_eq!(
        LS(true, I32, q(0), R::R0, -32, Offset).encode().unwrap(),
        vec![0x10, 0xed, 0x08, 0x1f]
    ); // vldrw.u32 q0, [r0, #-32]
    assert_eq!(
        LS(true, I32, q(0), R::R0, 16, PreIndex).encode().unwrap(),
        vec![0xb0, 0xed, 0x04, 0x1f]
    ); // vldrw.u32 q0, [r0, #16]!
    assert_eq!(
        LS(true, I32, q(0), R::R0, 16, PostIndex).encode().unwrap(),
        vec![0xb0, 0xec, 0x04, 0x1f]
    ); // vldrw.u32 q0, [r0], #16
    assert_eq!(
        LS(false, I32, q(0), R::R0, 0, Offset).encode().unwrap(),
        vec![0x80, 0xed, 0x00, 0x1f]
    ); // vstrw.32  q0, [r0]
    assert_eq!(
        LS(true, I8, q(0), R::R0, 0, Offset).encode().unwrap(),
        vec![0x90, 0xed, 0x00, 0x1e]
    ); // vldrb.u8  q0, [r0]
    assert_eq!(
        LS(true, I16, q(0), R::R0, 16, Offset).encode().unwrap(),
        vec![0x90, 0xed, 0x88, 0x1e]
    ); // vldrh.u16 q0, [r0, #16]
    assert_eq!(
        LS(false, I8, q(0), R::R0, 10, Offset).encode().unwrap(),
        vec![0x80, 0xed, 0x0a, 0x1e]
    ); // vstrb.8   q0, [r0, #10]
}

#[test]
fn round_trip__mve_load_store() {
    let modes = [
        ArmT32IndexMode::Offset,
        ArmT32IndexMode::PreIndex,
        ArmT32IndexMode::PostIndex,
    ];
    for is_load in [true, false] {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            let step: i32 = match size {
                Arm32MveSize::I8 => 1,
                Arm32MveSize::I16 => 2,
                Arm32MveSize::I32 => 4,
            };
            for offset in [0, step, -step, 5 * step, -127 * step, 127 * step] {
                for mode in modes {
                    for (d, n) in [(0u8, 0u8), (7, 5), (3, 14)] {
                        round_trip(&ArmT32Instruction::MveLoadStore(
                            is_load,
                            size,
                            q(d),
                            R::from_operand_bits(n),
                            offset,
                            mode,
                        ));
                    }
                }
            }
        }
    }
}

#[test]
fn encode__mve_reductions_exact_bytes() {
    use Arm32MveReduceOp::*;
    use Arm32MveSize::*;
    use ArmT32Instruction::{MveReduce, MveVabav};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveReduce(VaddvS, I8, R::R0, q(1)).encode().unwrap(),
        vec![0xf1, 0xee, 0x02, 0x0f]
    ); // vaddv.s8  r0, q1
    assert_eq!(
        MveReduce(VaddvU, I32, R::R0, q(1)).encode().unwrap(),
        vec![0xf9, 0xfe, 0x02, 0x0f]
    ); // vaddv.u32 r0, q1
    assert_eq!(
        MveReduce(VaddvaS, I32, R::R0, q(1)).encode().unwrap(),
        vec![0xf9, 0xee, 0x22, 0x0f]
    ); // vaddva.s32 r0, q1
    assert_eq!(
        MveReduce(VminvS, I8, R::R0, q(1)).encode().unwrap(),
        vec![0xe2, 0xee, 0x82, 0x0f]
    ); // vminv.s8  r0, q1
    assert_eq!(
        MveReduce(VmaxvU, I8, R::R0, q(1)).encode().unwrap(),
        vec![0xe2, 0xfe, 0x02, 0x0f]
    ); // vmaxv.u8  r0, q1
    assert_eq!(
        MveReduce(Vminav, I8, R::R0, q(1)).encode().unwrap(),
        vec![0xe0, 0xee, 0x82, 0x0f]
    ); // vminav.s8 r0, q1
    assert_eq!(
        MveReduce(Vmaxav, I32, R::R0, q(1)).encode().unwrap(),
        vec![0xe8, 0xee, 0x02, 0x0f]
    ); // vmaxav.s32 r0, q1
    assert_eq!(
        MveVabav(true, I8, R::R0, q(1), q(2)).encode().unwrap(),
        vec![0x82, 0xee, 0x05, 0x0f]
    ); // vabav.s8  r0, q1, q2
    assert_eq!(
        MveVabav(false, I16, R::R0, q(1), q(2)).encode().unwrap(),
        vec![0x92, 0xfe, 0x05, 0x0f]
    ); // vabav.u16 r0, q1, q2
}

#[test]
fn round_trip__mve_reductions() {
    let even_regs = [R::R0, R::R2, R::R4, R::R12];
    for op in Arm32MveReduceOp::ALL {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            for rd in even_regs {
                for m in [1u8, 5, 7] {
                    round_trip(&ArmT32Instruction::MveReduce(op, size, rd, q(m)));
                }
            }
        }
    }
    for signed in [true, false] {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            for rd in [R::R0, R::R1, R::R7, R::R14] {
                round_trip(&ArmT32Instruction::MveVabav(signed, size, rd, q(2), q(5)));
            }
        }
    }
}

#[test]
fn encode__mve_vrint_vcvt_exact_bytes() {
    use Arm32MveFloatSize::*;
    use Arm32MveVrintOp::*;
    use ArmT32Instruction::{MveVcvtFloatInt, MveVrint};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveVrint(Vrintn, F32, q(0), q(1)).encode().unwrap(),
        vec![0xba, 0xff, 0x42, 0x04]
    ); // vrintn.f32 q0, q1
    assert_eq!(
        MveVrint(Vrinta, F32, q(0), q(1)).encode().unwrap(),
        vec![0xba, 0xff, 0x42, 0x05]
    ); // vrinta.f32 q0, q1
    assert_eq!(
        MveVrint(Vrintx, F32, q(0), q(1)).encode().unwrap(),
        vec![0xba, 0xff, 0xc2, 0x04]
    ); // vrintx.f32 q0, q1
    assert_eq!(
        MveVrint(Vrintz, F32, q(0), q(1)).encode().unwrap(),
        vec![0xba, 0xff, 0xc2, 0x05]
    ); // vrintz.f32 q0, q1
    assert_eq!(
        MveVrint(Vrintm, F32, q(0), q(1)).encode().unwrap(),
        vec![0xba, 0xff, 0xc2, 0x06]
    ); // vrintm.f32 q0, q1
    assert_eq!(
        MveVrint(Vrintp, F32, q(0), q(1)).encode().unwrap(),
        vec![0xba, 0xff, 0xc2, 0x07]
    ); // vrintp.f32 q0, q1
    assert_eq!(
        MveVrint(Vrintn, F16, q(0), q(1)).encode().unwrap(),
        vec![0xb6, 0xff, 0x42, 0x04]
    ); // vrintn.f16 q0, q1
    // VCVT float<->int
    assert_eq!(
        MveVcvtFloatInt(false, false, F32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xbb, 0xff, 0x42, 0x06]
    ); // vcvt.f32.s32 q0, q1
    assert_eq!(
        MveVcvtFloatInt(false, true, F32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xbb, 0xff, 0xc2, 0x06]
    ); // vcvt.f32.u32 q0, q1
    assert_eq!(
        MveVcvtFloatInt(true, false, F32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xbb, 0xff, 0x42, 0x07]
    ); // vcvt.s32.f32 q0, q1
    assert_eq!(
        MveVcvtFloatInt(true, true, F32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xbb, 0xff, 0xc2, 0x07]
    ); // vcvt.u32.f32 q0, q1
    assert_eq!(
        MveVcvtFloatInt(false, false, F16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0xb7, 0xff, 0x42, 0x06]
    ); // vcvt.f16.s16 q0, q1
}

#[test]
fn round_trip__mve_vrint_vcvt() {
    for op in Arm32MveVrintOp::ALL {
        for size in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
            for (d, m) in [(0u8, 0u8), (7, 1), (3, 5)] {
                round_trip(&ArmT32Instruction::MveVrint(op, size, q(d), q(m)));
            }
        }
    }
    for to_int in [false, true] {
        for unsigned in [false, true] {
            for size in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
                for (d, m) in [(0u8, 0u8), (7, 1), (4, 6)] {
                    round_trip(&ArmT32Instruction::MveVcvtFloatInt(
                        to_int,
                        unsigned,
                        size,
                        q(d),
                        q(m),
                    ));
                }
            }
        }
    }
}

#[test]
fn encode__mve_vmovl_vmovn_vaddlv_exact_bytes() {
    use Arm32MveSize::*;
    use ArmT32Instruction::{MveVaddlv, MveVmovl, MveVmovn};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveVmovl(false, false, I8, q(0), q(1)).encode().unwrap(),
        vec![0xa8, 0xee, 0x42, 0x0f]
    ); // vmovlb.s8  q0, q1
    assert_eq!(
        MveVmovl(true, false, I8, q(0), q(1)).encode().unwrap(),
        vec![0xa8, 0xee, 0x42, 0x1f]
    ); // vmovlt.s8  q0, q1
    assert_eq!(
        MveVmovl(false, true, I8, q(0), q(1)).encode().unwrap(),
        vec![0xa8, 0xfe, 0x42, 0x0f]
    ); // vmovlb.u8  q0, q1
    assert_eq!(
        MveVmovl(false, false, I16, q(0), q(1)).encode().unwrap(),
        vec![0xb0, 0xee, 0x42, 0x0f]
    ); // vmovlb.s16 q0, q1
    assert_eq!(
        MveVmovl(true, true, I16, q(0), q(1)).encode().unwrap(),
        vec![0xb0, 0xfe, 0x42, 0x1f]
    ); // vmovlt.u16 q0, q1
    assert_eq!(
        MveVmovn(false, I16, q(0), q(1)).encode().unwrap(),
        vec![0x31, 0xfe, 0x83, 0x0e]
    ); // vmovnb.i16 q0, q1
    assert_eq!(
        MveVmovn(true, I16, q(0), q(1)).encode().unwrap(),
        vec![0x31, 0xfe, 0x83, 0x1e]
    ); // vmovnt.i16 q0, q1
    assert_eq!(
        MveVmovn(false, I32, q(0), q(1)).encode().unwrap(),
        vec![0x35, 0xfe, 0x83, 0x0e]
    ); // vmovnb.i32 q0, q1
    assert_eq!(
        MveVaddlv(false, false, R::R0, R::R1, q(2))
            .encode()
            .unwrap(),
        vec![0x89, 0xee, 0x04, 0x0f]
    ); // vaddlv.s32  r0, r1, q2
    assert_eq!(
        MveVaddlv(false, true, R::R0, R::R1, q(2)).encode().unwrap(),
        vec![0x89, 0xfe, 0x04, 0x0f]
    ); // vaddlv.u32  r0, r1, q2
    assert_eq!(
        MveVaddlv(true, false, R::R0, R::R1, q(2)).encode().unwrap(),
        vec![0x89, 0xee, 0x24, 0x0f]
    ); // vaddlva.s32 r0, r1, q2
    assert_eq!(
        MveVaddlv(false, false, R::R4, R::R5, q(2))
            .encode()
            .unwrap(),
        vec![0xa9, 0xee, 0x04, 0x4f]
    ); // vaddlv.s32  r4, r5, q2
    // RdLo and RdHi are INDEPENDENT (even/odd, not necessarily consecutive)
    assert_eq!(
        MveVaddlv(false, false, R::R2, R::R5, q(2))
            .encode()
            .unwrap(),
        vec![0xa9, 0xee, 0x04, 0x2f]
    ); // vaddlv.s32  r2, r5, q2
    assert_eq!(
        MveVaddlv(false, false, R::R12, R::R1, q(2))
            .encode()
            .unwrap(),
        vec![0x89, 0xee, 0x04, 0xcf]
    ); // vaddlv.s32 r12, r1, q2
}

#[test]
fn encode__mve_vqmovn_vqmovun_exact_bytes() {
    use Arm32MveQMovnKind::*;
    use Arm32MveSize::*;
    use ArmT32Instruction::MveVqmovn;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveVqmovn(Vqmovn, false, false, I16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x33, 0xee, 0x03, 0x0e]
    ); // vqmovnb.s16  q0, q1
    assert_eq!(
        MveVqmovn(Vqmovn, false, true, I16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x33, 0xee, 0x03, 0x1e]
    ); // vqmovnt.s16  q0, q1
    assert_eq!(
        MveVqmovn(Vqmovn, false, false, I32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x37, 0xee, 0x03, 0x0e]
    ); // vqmovnb.s32  q0, q1
    assert_eq!(
        MveVqmovn(Vqmovn, true, false, I16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x33, 0xfe, 0x03, 0x0e]
    ); // vqmovnb.u16  q0, q1
    assert_eq!(
        MveVqmovn(Vqmovn, true, true, I32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x37, 0xfe, 0x03, 0x1e]
    ); // vqmovnt.u32  q0, q1
    assert_eq!(
        MveVqmovn(Vqmovun, false, false, I16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x31, 0xee, 0x83, 0x0e]
    ); // vqmovunb.s16 q0, q1
    assert_eq!(
        MveVqmovn(Vqmovun, false, true, I16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x31, 0xee, 0x83, 0x1e]
    ); // vqmovunt.s16 q0, q1
    assert_eq!(
        MveVqmovn(Vqmovun, false, false, I32, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x35, 0xee, 0x83, 0x0e]
    ); // vqmovunb.s32 q0, q1
    // register placement (Qd[15:13], Qm[3:1])
    assert_eq!(
        MveVqmovn(Vqmovn, false, false, I16, q(4), q(5))
            .encode()
            .unwrap(),
        vec![0x33, 0xee, 0x0b, 0x8e]
    ); // vqmovnb.s16  q4, q5
    assert_eq!(
        MveVqmovn(Vqmovn, false, false, I16, q(2), q(7))
            .encode()
            .unwrap(),
        vec![0x33, 0xee, 0x0f, 0x4e]
    ); // vqmovnb.s16  q2, q7
}

#[test]
fn round_trip__mve_vqmovn_vqmovun() {
    for top in [false, true] {
        for size in [Arm32MveSize::I16, Arm32MveSize::I32] {
            for (d, m) in [(0u8, 0u8), (7, 1), (2, 7), (4, 5)] {
                for unsigned in [false, true] {
                    round_trip(&ArmT32Instruction::MveVqmovn(
                        Arm32MveQMovnKind::Vqmovn,
                        unsigned,
                        top,
                        size,
                        q(d),
                        q(m),
                    ));
                }
                round_trip(&ArmT32Instruction::MveVqmovn(
                    Arm32MveQMovnKind::Vqmovun,
                    false,
                    top,
                    size,
                    q(d),
                    q(m),
                ));
            }
        }
    }
}

#[test]
fn encode__mve_vmull_vmulh_vqdmull_exact_bytes() {
    use Arm32MveSize::*;
    use ArmT32Instruction::{MveVmulh, MveVmull, MveVqdmull, MveVqdmullScalar};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    // VMULL integer (widening, b/t): U=bit28, size[21:20]
    assert_eq!(
        MveVmull(false, false, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x03, 0xee, 0x04, 0x0e]
    ); // vmullb.s8  q0,q1,q2
    assert_eq!(
        MveVmull(false, false, true, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x03, 0xee, 0x04, 0x1e]
    ); // vmullt.s8  q0,q1,q2
    assert_eq!(
        MveVmull(false, false, false, I16, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x13, 0xee, 0x04, 0x0e]
    ); // vmullb.s16 q0,q1,q2
    assert_eq!(
        MveVmull(false, false, false, I32, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x23, 0xee, 0x04, 0x0e]
    ); // vmullb.s32 q0,q1,q2
    assert_eq!(
        MveVmull(false, true, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x03, 0xfe, 0x04, 0x0e]
    ); // vmullb.u8  q0,q1,q2
    // VMULL polynomial: size[21:20]=11, bit28 = P8(0)/P16(1)
    assert_eq!(
        MveVmull(true, false, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x33, 0xee, 0x04, 0x0e]
    ); // vmullb.p8  q0,q1,q2
    assert_eq!(
        MveVmull(true, false, true, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x33, 0xee, 0x04, 0x1e]
    ); // vmullt.p8  q0,q1,q2
    assert_eq!(
        MveVmull(true, false, false, I16, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x33, 0xfe, 0x04, 0x0e]
    ); // vmullb.p16 q0,q1,q2
    // register placement (Qd[15:13], Qn[19:17], Qm[3:1])
    assert_eq!(
        MveVmull(false, false, false, I8, q(4), q(5), q(6))
            .encode()
            .unwrap(),
        vec![0x0b, 0xee, 0x0c, 0x8e]
    ); // vmullb.s8  q4,q5,q6
    assert_eq!(
        MveVmull(false, false, false, I8, q(3), q(0), q(7))
            .encode()
            .unwrap(),
        vec![0x01, 0xee, 0x0e, 0x6e]
    ); // vmullb.s8  q3,q0,q7
    // VMULH / VRMULH (high half): U=bit28, rounding=bit12
    assert_eq!(
        MveVmulh(false, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x03, 0xee, 0x05, 0x0e]
    ); // vmulh.s8   q0,q1,q2
    assert_eq!(
        MveVmulh(false, true, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x03, 0xfe, 0x05, 0x0e]
    ); // vmulh.u8   q0,q1,q2
    assert_eq!(
        MveVmulh(false, false, I32, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x23, 0xee, 0x05, 0x0e]
    ); // vmulh.s32  q0,q1,q2
    assert_eq!(
        MveVmulh(true, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x03, 0xee, 0x05, 0x1e]
    ); // vrmulh.s8  q0,q1,q2
    assert_eq!(
        MveVmulh(true, true, I16, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x13, 0xfe, 0x05, 0x1e]
    ); // vrmulh.u16 q0,q1,q2
    // VQDMULL (saturating doubling long): sz=bit28 (.s16=0/.s32=1), vector + scalar
    assert_eq!(
        MveVqdmull(false, false, q(0), q(1), q(2)).encode().unwrap(),
        vec![0x32, 0xee, 0x05, 0x0f]
    ); // vqdmullb.s16 q0,q1,q2
    assert_eq!(
        MveVqdmull(true, true, q(0), q(1), q(2)).encode().unwrap(),
        vec![0x32, 0xfe, 0x05, 0x1f]
    ); // vqdmullt.s32 q0,q1,q2
    assert_eq!(
        MveVqdmullScalar(false, false, q(0), q(1), R::R2)
            .encode()
            .unwrap(),
        vec![0x32, 0xee, 0x62, 0x0f]
    ); // vqdmullb.s16 q0,q1,r2
    assert_eq!(
        MveVqdmullScalar(true, true, q(0), q(1), R::R3)
            .encode()
            .unwrap(),
        vec![0x32, 0xfe, 0x63, 0x1f]
    ); // vqdmullt.s32 q0,q1,r3
}

#[test]
fn round_trip__mve_vmull_vmulh_vqdmull() {
    let regs = [(0u8, 0u8, 0u8), (7, 1, 3), (2, 5, 7), (4, 6, 1)];
    for &(d, n, m) in &regs {
        for top in [false, true] {
            for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
                for unsigned in [false, true] {
                    round_trip(&ArmT32Instruction::MveVmull(
                        false,
                        unsigned,
                        top,
                        size,
                        q(d),
                        q(n),
                        q(m),
                    ));
                    round_trip(&ArmT32Instruction::MveVmulh(
                        false,
                        unsigned,
                        size,
                        q(d),
                        q(n),
                        q(m),
                    ));
                    round_trip(&ArmT32Instruction::MveVmulh(
                        true,
                        unsigned,
                        size,
                        q(d),
                        q(n),
                        q(m),
                    ));
                }
            }
            for size in [Arm32MveSize::I8, Arm32MveSize::I16] {
                round_trip(&ArmT32Instruction::MveVmull(
                    true,
                    false,
                    top,
                    size,
                    q(d),
                    q(n),
                    q(m),
                )); // poly P8/P16
            }
            for size32 in [false, true] {
                round_trip(&ArmT32Instruction::MveVqdmull(
                    top,
                    size32,
                    q(d),
                    q(n),
                    q(m),
                ));
                round_trip(&ArmT32Instruction::MveVqdmullScalar(
                    top,
                    size32,
                    q(d),
                    q(n),
                    R::from_operand_bits(n),
                ));
            }
        }
    }
}

#[test]
fn encode__mve_vqdmladh_vqdmlsdh_exact_bytes() {
    use Arm32MveSize::*;
    use ArmT32Instruction::MveVqdmladh;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    // (subtract, rounding, exchange, size, qd, qn, qm)
    assert_eq!(
        MveVqdmladh(false, false, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xee, 0x04, 0x0e]
    ); // vqdmladh.s8
    assert_eq!(
        MveVqdmladh(false, false, true, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xee, 0x04, 0x1e]
    ); // vqdmladhx.s8
    assert_eq!(
        MveVqdmladh(false, false, false, I16, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x12, 0xee, 0x04, 0x0e]
    ); // vqdmladh.s16
    assert_eq!(
        MveVqdmladh(false, false, false, I32, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x22, 0xee, 0x04, 0x0e]
    ); // vqdmladh.s32
    assert_eq!(
        MveVqdmladh(false, true, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xee, 0x05, 0x0e]
    ); // vqrdmladh.s8
    assert_eq!(
        MveVqdmladh(false, true, true, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xee, 0x05, 0x1e]
    ); // vqrdmladhx.s8
    assert_eq!(
        MveVqdmladh(true, false, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xfe, 0x04, 0x0e]
    ); // vqdmlsdh.s8
    assert_eq!(
        MveVqdmladh(true, false, true, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xfe, 0x04, 0x1e]
    ); // vqdmlsdhx.s8
    assert_eq!(
        MveVqdmladh(true, false, false, I32, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x22, 0xfe, 0x04, 0x0e]
    ); // vqdmlsdh.s32
    assert_eq!(
        MveVqdmladh(true, true, true, I16, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x12, 0xfe, 0x05, 0x1e]
    ); // vqrdmlsdhx.s16
    // register placement
    assert_eq!(
        MveVqdmladh(false, false, false, I8, q(4), q(5), q(6))
            .encode()
            .unwrap(),
        vec![0x0a, 0xee, 0x0c, 0x8e]
    ); // vqdmladh.s8 q4,q5,q6
    assert_eq!(
        MveVqdmladh(false, false, false, I8, q(3), q(0), q(7))
            .encode()
            .unwrap(),
        vec![0x00, 0xee, 0x0e, 0x6e]
    ); // vqdmladh.s8 q3,q0,q7
}

#[test]
fn round_trip__mve_vqdmladh_vqdmlsdh() {
    for &(d, n, m) in &[(0u8, 0u8, 0u8), (7, 1, 3), (2, 5, 7), (4, 6, 1)] {
        for subtract in [false, true] {
            for rounding in [false, true] {
                for exchange in [false, true] {
                    for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
                        round_trip(&ArmT32Instruction::MveVqdmladh(
                            subtract,
                            rounding,
                            exchange,
                            size,
                            q(d),
                            q(n),
                            q(m),
                        ));
                    }
                }
            }
        }
    }
}

#[test]
fn encode__mve_register_shifts_and_vshll_exact_bytes() {
    use Arm32MveSize::*;
    use ArmT32Instruction::{MveShiftByScalar, MveShiftByVector, MveVshll};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    // by vector (rounding, saturating, unsigned, size, qd, qm, qn)
    assert_eq!(
        MveShiftByVector(false, false, false, I8, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x04, 0xef, 0x42, 0x04]
    ); // vshl.s8   q0,q1,q2
    assert_eq!(
        MveShiftByVector(true, false, true, I16, q(3), q(4), q(5))
            .encode()
            .unwrap(),
        vec![0x1a, 0xff, 0x48, 0x65]
    ); // vrshl.u16 q3,q4,q5
    assert_eq!(
        MveShiftByVector(false, true, false, I32, q(7), q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x22, 0xef, 0x50, 0xe4]
    ); // vqshl.s32 q7,q0,q1
    assert_eq!(
        MveShiftByVector(true, true, true, I8, q(2), q(6), q(3))
            .encode()
            .unwrap(),
        vec![0x06, 0xff, 0x5c, 0x45]
    ); // vqrshl.u8 q2,q6,q3
    // by GPR scalar (rounding, saturating, unsigned, size, qda, rm)
    assert_eq!(
        MveShiftByScalar(false, false, false, I8, q(0), R::R2)
            .encode()
            .unwrap(),
        vec![0x31, 0xee, 0x62, 0x1e]
    ); // vshl.s8   q0,r2
    assert_eq!(
        MveShiftByScalar(true, false, true, I32, q(5), R::R7)
            .encode()
            .unwrap(),
        vec![0x3b, 0xfe, 0x67, 0xbe]
    ); // vrshl.u32 q5,r7
    assert_eq!(
        MveShiftByScalar(false, true, false, I8, q(3), R::R10)
            .encode()
            .unwrap(),
        vec![0x31, 0xee, 0xea, 0x7e]
    ); // vqshl.s8  q3,r10
    assert_eq!(
        MveShiftByScalar(true, true, true, I16, q(6), R::R0)
            .encode()
            .unwrap(),
        vec![0x37, 0xfe, 0xe0, 0xde]
    ); // vqrshl.u16 q6,r0
    // VSHLL T1 (shift 1..esize-1)
    assert_eq!(
        MveVshll(false, false, I8, 1, q(0), q(1)).encode().unwrap(),
        vec![0xa9, 0xee, 0x42, 0x0f]
    ); // vshllb.s8  q0,q1,#1
    assert_eq!(
        MveVshll(true, false, I16, 15, q(0), q(1)).encode().unwrap(),
        vec![0xbf, 0xee, 0x42, 0x1f]
    ); // vshllt.s16 q0,q1,#15
    assert_eq!(
        MveVshll(false, true, I8, 5, q(0), q(1)).encode().unwrap(),
        vec![0xad, 0xfe, 0x42, 0x0f]
    ); // vshllb.u8  q0,q1,#5
    assert_eq!(
        MveVshll(false, false, I8, 2, q(4), q(5)).encode().unwrap(),
        vec![0xaa, 0xee, 0x4a, 0x8f]
    ); // vshllb.s8  q4,q5,#2
    // VSHLL T2 (shift == esize)
    assert_eq!(
        MveVshll(false, false, I8, 8, q(0), q(1)).encode().unwrap(),
        vec![0x31, 0xee, 0x03, 0x0e]
    ); // vshllb.s8  q0,q1,#8
    assert_eq!(
        MveVshll(false, false, I16, 16, q(0), q(1))
            .encode()
            .unwrap(),
        vec![0x35, 0xee, 0x03, 0x0e]
    ); // vshllb.s16 q0,q1,#16
    assert_eq!(
        MveVshll(false, true, I8, 8, q(0), q(1)).encode().unwrap(),
        vec![0x31, 0xfe, 0x03, 0x0e]
    ); // vshllb.u8  q0,q1,#8
}

#[test]
fn round_trip__mve_register_shifts_and_vshll() {
    for rounding in [false, true] {
        for saturating in [false, true] {
            for unsigned in [false, true] {
                for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
                    for &(d, m, n) in &[(0u8, 0u8, 0u8), (7, 1, 3), (2, 6, 5)] {
                        round_trip(&ArmT32Instruction::MveShiftByVector(
                            rounding,
                            saturating,
                            unsigned,
                            size,
                            q(d),
                            q(m),
                            q(n),
                        ));
                    }
                    for &(d, rm) in &[(0u8, 0u8), (7, 12), (3, 5)] {
                        round_trip(&ArmT32Instruction::MveShiftByScalar(
                            rounding,
                            saturating,
                            unsigned,
                            size,
                            q(d),
                            R::from_operand_bits(rm),
                        ));
                    }
                }
            }
        }
    }
    // VSHLL: every legal shift for each source size + b/t + s/u
    for top in [false, true] {
        for unsigned in [false, true] {
            for (size, esize) in [(Arm32MveSize::I8, 8u8), (Arm32MveSize::I16, 16u8)] {
                for shift in 1..=esize {
                    for &(d, m) in &[(0u8, 1u8), (4, 5), (7, 2)] {
                        round_trip(&ArmT32Instruction::MveVshll(
                            top,
                            unsigned,
                            size,
                            shift,
                            q(d),
                            q(m),
                        ));
                    }
                }
            }
        }
    }
}

#[test]
fn encode__vmovx_vins_exact_bytes() {
    use ArmT32Instruction::Vmovx_T1;
    let s = |n: u8| crate::enums::Arm32SinglePrecisionRegister::new(n).unwrap();
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        Vmovx_T1(false, s(0), s(1)).encode().unwrap(),
        vec![0xb0, 0xfe, 0x60, 0x0a]
    ); // vmovx.f16 s0, s1
    assert_eq!(
        Vmovx_T1(false, s(5), s(10)).encode().unwrap(),
        vec![0xf0, 0xfe, 0x45, 0x2a]
    ); // vmovx.f16 s5, s10
    assert_eq!(
        Vmovx_T1(false, s(31), s(30)).encode().unwrap(),
        vec![0xf0, 0xfe, 0x4f, 0xfa]
    ); // vmovx.f16 s31, s30
    assert_eq!(
        Vmovx_T1(true, s(0), s(1)).encode().unwrap(),
        vec![0xb0, 0xfe, 0xe0, 0x0a]
    ); // vins.f16  s0, s1
    assert_eq!(
        Vmovx_T1(true, s(4), s(7)).encode().unwrap(),
        vec![0xb0, 0xfe, 0xe3, 0x2a]
    ); // vins.f16  s4, s7
}

#[test]
fn round_trip__vmovx_vins() {
    let s = |n: u8| crate::enums::Arm32SinglePrecisionRegister::new(n).unwrap();
    for insert in [false, true] {
        for &(d, m) in &[(0u8, 0u8), (1, 2), (15, 16), (31, 30), (7, 24)] {
            round_trip(&ArmT32Instruction::Vmovx_T1(insert, s(d), s(m)));
        }
    }
}

#[test]
fn encode__lda_stl_acquire_release_exact_bytes() {
    use ArmT32Instruction::{LoadAcquire_T1, StoreRelease_T1, StoreReleaseExclusive_T1};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main` (Thumb). size: 0=byte, 1=half, 2=word.
    assert_eq!(
        LoadAcquire_T1(2, false, R::R0, R::R1).encode().unwrap(),
        vec![0xd1, 0xe8, 0xaf, 0x0f]
    ); // lda    r0, [r1]
    assert_eq!(
        LoadAcquire_T1(0, false, R::R2, R::R3).encode().unwrap(),
        vec![0xd3, 0xe8, 0x8f, 0x2f]
    ); // ldab   r2, [r3]
    assert_eq!(
        LoadAcquire_T1(1, false, R::R4, R::R5).encode().unwrap(),
        vec![0xd5, 0xe8, 0x9f, 0x4f]
    ); // ldah   r4, [r5]
    assert_eq!(
        StoreRelease_T1(2, R::R0, R::R1).encode().unwrap(),
        vec![0xc1, 0xe8, 0xaf, 0x0f]
    ); // stl    r0, [r1]
    assert_eq!(
        StoreRelease_T1(0, R::R2, R::R3).encode().unwrap(),
        vec![0xc3, 0xe8, 0x8f, 0x2f]
    ); // stlb   r2, [r3]
    assert_eq!(
        StoreRelease_T1(1, R::R4, R::R5).encode().unwrap(),
        vec![0xc5, 0xe8, 0x9f, 0x4f]
    ); // stlh   r4, [r5]
    assert_eq!(
        LoadAcquire_T1(2, true, R::R0, R::R1).encode().unwrap(),
        vec![0xd1, 0xe8, 0xef, 0x0f]
    ); // ldaex  r0, [r1]
    assert_eq!(
        LoadAcquire_T1(0, true, R::R2, R::R3).encode().unwrap(),
        vec![0xd3, 0xe8, 0xcf, 0x2f]
    ); // ldaexb r2, [r3]
    assert_eq!(
        LoadAcquire_T1(1, true, R::R4, R::R5).encode().unwrap(),
        vec![0xd5, 0xe8, 0xdf, 0x4f]
    ); // ldaexh r4, [r5]
    assert_eq!(
        StoreReleaseExclusive_T1(2, R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0xc2, 0xe8, 0xe0, 0x1f]
    ); // stlex  r0, r1, [r2]
    assert_eq!(
        StoreReleaseExclusive_T1(0, R::R3, R::R4, R::R5)
            .encode()
            .unwrap(),
        vec![0xc5, 0xe8, 0xc3, 0x4f]
    ); // stlexb r3, r4, [r5]
    assert_eq!(
        StoreReleaseExclusive_T1(1, R::R6, R::R7, R::R8)
            .encode()
            .unwrap(),
        vec![0xc8, 0xe8, 0xd6, 0x7f]
    ); // stlexh r6, r7, [r8]
}

#[test]
fn encode__hints_barriers_clrm_vsel_exact_bytes() {
    use ArmT32Instruction::{
        Clrm_T1, Dbg_T1, Esb_T1, Pssbb_T1, Sb_T1, Ssbb_T1, Vjcvt_T1, Vsel_Double_T1, Vsel_Single_T1,
    };
    let s = |n: u8| crate::enums::Arm32SinglePrecisionRegister::new(n).unwrap();
    let d = |n: u8| crate::enums::Arm32DoublePrecisionRegister::new(n).unwrap();
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp+fp.dp` (ESB from DDI0553 spec)
    assert_eq!(Dbg_T1(5).encode().unwrap(), vec![0xaf, 0xf3, 0xf5, 0x80]); // dbg #5
    assert_eq!(Esb_T1.encode().unwrap(), vec![0xaf, 0xf3, 0x10, 0x80]); // esb
    assert_eq!(Ssbb_T1.encode().unwrap(), vec![0xbf, 0xf3, 0x40, 0x8f]); // ssbb
    assert_eq!(Pssbb_T1.encode().unwrap(), vec![0xbf, 0xf3, 0x44, 0x8f]); // pssbb
    assert_eq!(Sb_T1.encode().unwrap(), vec![0xbf, 0xf3, 0x70, 0x8f]); // sb
    assert_eq!(
        Vjcvt_T1(s(0), d(1)).encode().unwrap(),
        vec![0xb9, 0xee, 0xc1, 0x0b]
    ); // vjcvt.s32.f64 s0, d1
    assert_eq!(
        Vjcvt_T1(s(3), d(5)).encode().unwrap(),
        vec![0xf9, 0xee, 0xc5, 0x1b]
    ); // vjcvt.s32.f64 s3, d5
    assert_eq!(
        Clrm_T1(0x0007).encode().unwrap(),
        vec![0x9f, 0xe8, 0x07, 0x00]
    ); // clrm {r0, r1, r2}
    assert_eq!(
        Clrm_T1(0x410F).encode().unwrap(),
        vec![0x9f, 0xe8, 0x0f, 0x41]
    ); // clrm {r0-r3, r8, lr}
    // VSEL: cond EQ=0, VS=1, GE=2, GT=3
    assert_eq!(
        Vsel_Single_T1(3, s(0), s(1), s(2)).encode().unwrap(),
        vec![0x30, 0xfe, 0x81, 0x0a]
    ); // vselgt.f32 s0, s1, s2
    assert_eq!(
        Vsel_Double_T1(2, d(0), d(1), d(2)).encode().unwrap(),
        vec![0x21, 0xfe, 0x02, 0x0b]
    ); // vselge.f64 d0, d1, d2
    assert_eq!(
        Vsel_Single_T1(0, s(4), s(5), s(6)).encode().unwrap(),
        vec![0x02, 0xfe, 0x83, 0x2a]
    ); // vseleq.f32 s4, s5, s6
    assert_eq!(
        Vsel_Double_T1(1, d(4), d(5), d(6)).encode().unwrap(),
        vec![0x15, 0xfe, 0x06, 0x4b]
    ); // vselvs.f64 d4, d5, d6
}

#[test]
fn encode__csel_family_exact_bytes() {
    use crate::enums::ArmT32InstructionCondition as C;
    use ArmT32Instruction::Csel_T1;
    let pc = R::from_operand_bits(15);
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main` (Thumb). op: 0=csel,1=csinc,2=csinv,3=csneg.
    assert_eq!(
        Csel_T1(0, R::R0, R::R1, R::R2, C::Equal).encode().unwrap(),
        vec![0x51, 0xea, 0x02, 0x80]
    ); // csel  r0,r1,r2,eq
    assert_eq!(
        Csel_T1(1, R::R0, R::R1, R::R2, C::SignedGreaterThanOrEqual)
            .encode()
            .unwrap(),
        vec![0x51, 0xea, 0xa2, 0x90]
    ); // csinc r0,r1,r2,ge
    assert_eq!(
        Csel_T1(2, R::R3, R::R4, R::R5, C::SignedLessThan)
            .encode()
            .unwrap(),
        vec![0x54, 0xea, 0xb5, 0xa3]
    ); // csinv r3,r4,r5,lt
    assert_eq!(
        Csel_T1(3, R::R6, R::R7, R::R8, C::SignedGreaterThan)
            .encode()
            .unwrap(),
        vec![0x57, 0xea, 0xc8, 0xb6]
    ); // csneg r6,r7,r8,gt
    // alias-equivalent encodings (Rn==Rm, inverted condition)
    assert_eq!(
        Csel_T1(1, R::R0, pc, pc, C::NotEqual).encode().unwrap(),
        vec![0x5f, 0xea, 0x1f, 0x90]
    ); // cset  r0,eq  (= csinc r0,pc,pc,ne)
    assert_eq!(
        Csel_T1(2, R::R1, pc, pc, C::Equal).encode().unwrap(),
        vec![0x5f, 0xea, 0x0f, 0xa1]
    ); // csetm r1,ne  (= csinv r1,pc,pc,eq)
    assert_eq!(
        Csel_T1(1, R::R2, R::R3, R::R3, C::SignedLessThan)
            .encode()
            .unwrap(),
        vec![0x53, 0xea, 0xb3, 0x92]
    ); // cinc  r2,r3,ge (= csinc r2,r3,r3,lt)
    assert_eq!(
        Csel_T1(3, R::R6, R::R7, R::R7, C::SignedLessThanOrEqual)
            .encode()
            .unwrap(),
        vec![0x57, 0xea, 0xd7, 0xb6]
    ); // cneg  r6,r7,gt (= csneg r6,r7,r7,le)
}

#[test]
fn encode__long_shifts_exact_bytes() {
    use ArmT32Instruction::{LongShiftImm_T1, LongShiftReg_T1};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve`. op: 0=lsll, 1=lsrl, 2=asrl.
    assert_eq!(
        LongShiftImm_T1(2, R::R0, R::R1, 5).encode().unwrap(),
        vec![0x50, 0xea, 0x6f, 0x11]
    ); // asrl r0,r1,#5
    assert_eq!(
        LongShiftImm_T1(0, R::R0, R::R1, 10).encode().unwrap(),
        vec![0x50, 0xea, 0x8f, 0x21]
    ); // lsll r0,r1,#10
    assert_eq!(
        LongShiftImm_T1(1, R::R4, R::R5, 20).encode().unwrap(),
        vec![0x54, 0xea, 0x1f, 0x55]
    ); // lsrl r4,r5,#20
    assert_eq!(
        LongShiftReg_T1(2, R::R2, R::R3, R::R4).encode().unwrap(),
        vec![0x52, 0xea, 0x2d, 0x43]
    ); // asrl r2,r3,r4
    assert_eq!(
        LongShiftReg_T1(0, R::R2, R::R3, R::R4).encode().unwrap(),
        vec![0x52, 0xea, 0x0d, 0x43]
    ); // lsll r2,r3,r4
}

#[test]
fn encode__saturating_shifts_exact_bytes() {
    use ArmT32Instruction::{
        SatShiftImm_T1, SatShiftLongImm_T1, SatShiftLongReg_T1, SatShiftReg_T1,
    };
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve`. imm op: 0=uqshl,1=urshr,2=srshr,3=sqshl.
    assert_eq!(
        SatShiftImm_T1(3, R::R0, 3).encode().unwrap(),
        vec![0x50, 0xea, 0xff, 0x0f]
    ); // sqshl r0, #3
    assert_eq!(
        SatShiftImm_T1(2, R::R2, 7).encode().unwrap(),
        vec![0x52, 0xea, 0xef, 0x1f]
    ); // srshr r2, #7
    assert_eq!(
        SatShiftImm_T1(0, R::R4, 1).encode().unwrap(),
        vec![0x54, 0xea, 0x4f, 0x0f]
    ); // uqshl r4, #1
    assert_eq!(
        SatShiftImm_T1(1, R::R6, 15).encode().unwrap(),
        vec![0x56, 0xea, 0xdf, 0x3f]
    ); // urshr r6, #15
    assert_eq!(
        SatShiftLongImm_T1(3, R::R0, R::R1, 5).encode().unwrap(),
        vec![0x51, 0xea, 0x7f, 0x11]
    ); // sqshll r0,r1,#5
    assert_eq!(
        SatShiftLongImm_T1(0, R::R2, R::R3, 10).encode().unwrap(),
        vec![0x53, 0xea, 0x8f, 0x23]
    ); // uqshll r2,r3,#10
    assert_eq!(
        SatShiftLongImm_T1(2, R::R4, R::R5, 2).encode().unwrap(),
        vec![0x55, 0xea, 0xaf, 0x05]
    ); // srshrl r4,r5,#2
    assert_eq!(
        SatShiftLongImm_T1(1, R::R6, R::R7, 30).encode().unwrap(),
        vec![0x57, 0xea, 0x9f, 0x77]
    ); // urshrl r6,r7,#30
    assert_eq!(
        SatShiftReg_T1(true, R::R0, R::R1).encode().unwrap(),
        vec![0x50, 0xea, 0x2d, 0x1f]
    ); // sqrshr r0, r1
    assert_eq!(
        SatShiftReg_T1(false, R::R2, R::R3).encode().unwrap(),
        vec![0x52, 0xea, 0x0d, 0x3f]
    ); // uqrshl r2, r3
    assert_eq!(
        SatShiftLongReg_T1(true, R::R0, R::R1, R::R4, false)
            .encode()
            .unwrap(),
        vec![0x51, 0xea, 0x2d, 0x41]
    ); // sqrshrl r0,r1,#64,r4
    assert_eq!(
        SatShiftLongReg_T1(false, R::R2, R::R3, R::R5, true)
            .encode()
            .unwrap(),
        vec![0x53, 0xea, 0x8d, 0x53]
    ); // uqrshll r2,r3,#48,r5
}

#[test]
fn encode_round_trip__cde() {
    use ArmT32Instruction::{Cde_Cx1_T1, Cde_Cx2_T1, Cde_Cx3_T1};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+cdecp0+cdecp1` (Thumb)
    assert_eq!(
        Cde_Cx1_T1(false, false, 0, R::R0, 100).encode().unwrap(),
        vec![0x00, 0xee, 0xa4, 0x00]
    ); // cx1  p0, r0, #100
    assert_eq!(
        Cde_Cx1_T1(true, false, 0, R::R1, 200).encode().unwrap(),
        vec![0x01, 0xfe, 0x88, 0x10]
    ); // cx1a p0, r1, #200
    assert_eq!(
        Cde_Cx1_T1(false, true, 0, R::R2, 50).encode().unwrap(),
        vec![0x00, 0xee, 0x72, 0x20]
    ); // cx1d p0, r2, r3, #50
    assert_eq!(
        Cde_Cx2_T1(false, false, 1, R::R4, R::R5, 10)
            .encode()
            .unwrap(),
        vec![0x45, 0xee, 0x0a, 0x41]
    ); // cx2  p1, r4, r5, #10
    assert_eq!(
        Cde_Cx2_T1(true, false, 1, R::R6, R::R7, 20)
            .encode()
            .unwrap(),
        vec![0x47, 0xfe, 0x14, 0x61]
    ); // cx2a p1, r6, r7, #20
    assert_eq!(
        Cde_Cx3_T1(false, false, 0, R::R8, R::R9, R::R10, 5)
            .encode()
            .unwrap(),
        vec![0x89, 0xee, 0x98, 0xa0]
    ); // cx3  p0, r8, r9, r10, #5
    assert_eq!(
        Cde_Cx3_T1(true, false, 0, R::R0, R::R1, R::R2, 3)
            .encode()
            .unwrap(),
        vec![0x81, 0xfe, 0x30, 0x20]
    ); // cx3a p0, r0, r1, r2, #3
    // round-trips (CDE decodes coproc 0-7 before the generic coprocessor)
    for acc in [false, true] {
        for dual in [false, true] {
            for cp in 0..=7u8 {
                round_trip(&ArmT32Instruction::Cde_Cx1_T1(
                    acc,
                    dual,
                    cp,
                    R::R0,
                    0x1ABC & 0x1FFF,
                ));
                round_trip(&ArmT32Instruction::Cde_Cx2_T1(
                    acc,
                    dual,
                    cp,
                    R::R2,
                    R::R5,
                    0x19C,
                ));
                round_trip(&ArmT32Instruction::Cde_Cx3_T1(
                    acc,
                    dual,
                    cp,
                    R::R4,
                    R::R7,
                    R::R9,
                    0x2A,
                ));
            }
        }
    }
}

#[test]
fn encode_round_trip__branch_future() {
    use ArmT32Instruction::{Bf_T1, Bfcsel_T2, Bfl_T4, Bflx_T5, Bfx_T3};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main` (Thumb). `boff` is the raw 4-bit
    // b_label field (b_label = PC + 2*boff); `offset` is the byte displacement of the target from PC.
    assert_eq!(Bf_T1(1, 4).encode().unwrap(), vec![0xc0, 0xf0, 0x03, 0xe0]); // bf  PC+2, PC+4
    assert_eq!(Bf_T1(1, 16).encode().unwrap(), vec![0xc0, 0xf0, 0x09, 0xe0]); // bf  PC+2, PC+16
    assert_eq!(Bf_T1(3, 12).encode().unwrap(), vec![0xc0, 0xf1, 0x07, 0xe0]); // bf  PC+6, PC+12
    assert_eq!(Bfl_T4(1, 8).encode().unwrap(), vec![0x80, 0xf0, 0x05, 0xc0]); // bfl PC+2, PC+8
    assert_eq!(
        Bfx_T3(1, R::R3).encode().unwrap(),
        vec![0xe3, 0xf0, 0x01, 0xe0]
    ); // bfx  PC+2, r3
    assert_eq!(
        Bflx_T5(1, R::R10).encode().unwrap(),
        vec![0xfa, 0xf0, 0x01, 0xe0]
    ); // bflx PC+2, r10
    assert_eq!(
        Bfcsel_T2(1, 8, 1, false).encode().unwrap(),
        vec![0x84, 0xf0, 0x05, 0xe0]
    ); // bfcsel PC+2, PC+8, ..., ne
    assert_eq!(
        Bfcsel_T2(1, 8, 11, false).encode().unwrap(),
        vec![0xac, 0xf0, 0x05, 0xe0]
    ); // bfcsel PC+2, PC+8, ..., lt
    // round-trips: sweep boff (1..=15), signed even offsets incl. the range extremes, registers, conditions, T.
    for boff in 1..=15u8 {
        for off in [-65536, -4096, -100, -2, 0, 2, 100, 4094, 65534] {
            round_trip(&Bf_T1(boff, off));
            round_trip(&Bfl_T4(boff, off));
        }
        for off in [-4096, -2, 0, 2, 4094] {
            for cond in 0..=13u8 {
                for t in [false, true] {
                    round_trip(&Bfcsel_T2(boff, off, cond, t));
                }
            }
        }
        for rn in [0u8, 1, 3, 7, 12, 14] {
            round_trip(&Bfx_T3(boff, R::from_operand_bits(rn)));
            round_trip(&Bflx_T5(boff, R::from_operand_bits(rn)));
        }
    }
}

#[test]
fn encode_round_trip__vcx() {
    use ArmT32Instruction::{Vcx1_T1, Vcx2_T1, Vcx3_T1};
    // exact bytes vs `arm-none-eabi-as -march=armv8.1-m.main+mve.fp+cdecp0+cdecp1` (Thumb). kind: 0=S 1=D 2=Q.
    assert_eq!(
        Vcx1_T1(false, 0, 0, 0, 0).encode().unwrap(),
        vec![0x20, 0xec, 0x00, 0x00]
    ); // vcx1  p0, s0, #0
    assert_eq!(
        Vcx1_T1(false, 0, 0, 1, 1).encode().unwrap(),
        vec![0x60, 0xec, 0x01, 0x00]
    ); // vcx1  p0, s1, #1
    assert_eq!(
        Vcx1_T1(true, 0, 0, 0, 0).encode().unwrap(),
        vec![0x20, 0xfc, 0x00, 0x00]
    ); // vcx1a p0, s0, #0
    assert_eq!(
        Vcx1_T1(false, 1, 0, 0, 0).encode().unwrap(),
        vec![0x20, 0xed, 0x00, 0x00]
    ); // vcx1  p0, d0, #0
    assert_eq!(
        Vcx1_T1(false, 2, 0, 0, 0).encode().unwrap(),
        vec![0x20, 0xec, 0x40, 0x00]
    ); // vcx1  p0, q0, #0
    assert_eq!(
        Vcx1_T1(false, 2, 0, 7, 0).encode().unwrap(),
        vec![0x20, 0xec, 0x40, 0xe0]
    ); // vcx1  p0, q7, #0
    assert_eq!(
        Vcx1_T1(false, 0, 0, 0, 2047).encode().unwrap(),
        vec![0x2f, 0xec, 0xbf, 0x00]
    ); // vcx1  p0, s0, #2047
    assert_eq!(
        Vcx2_T1(false, 0, 0, 0, 1, 0).encode().unwrap(),
        vec![0x30, 0xec, 0x20, 0x00]
    ); // vcx2  p0, s0, s1, #0
    assert_eq!(
        Vcx2_T1(true, 0, 0, 2, 3, 63).encode().unwrap(),
        vec![0x3f, 0xfc, 0xb1, 0x10]
    ); // vcx2a p0, s2, s3, #63
    assert_eq!(
        Vcx2_T1(false, 1, 0, 0, 1, 0).encode().unwrap(),
        vec![0x30, 0xed, 0x01, 0x00]
    ); // vcx2  p0, d0, d1, #0
    assert_eq!(
        Vcx2_T1(false, 2, 0, 0, 1, 0).encode().unwrap(),
        vec![0x30, 0xec, 0x42, 0x00]
    ); // vcx2  p0, q0, q1, #0
    assert_eq!(
        Vcx3_T1(false, 0, 0, 0, 1, 2, 0).encode().unwrap(),
        vec![0x80, 0xec, 0x81, 0x00]
    ); // vcx3  p0, s0, s1, s2, #0
    assert_eq!(
        Vcx3_T1(true, 0, 0, 3, 4, 5, 7).encode().unwrap(),
        vec![0xf2, 0xfc, 0x32, 0x10]
    ); // vcx3a p0, s3, s4, s5, #7
    assert_eq!(
        Vcx3_T1(false, 1, 0, 0, 1, 2, 0).encode().unwrap(),
        vec![0x81, 0xed, 0x02, 0x00]
    ); // vcx3  p0, d0, d1, d2, #0
    assert_eq!(
        Vcx3_T1(false, 2, 0, 0, 1, 2, 0).encode().unwrap(),
        vec![0x82, 0xec, 0x44, 0x00]
    ); // vcx3  p0, q0, q1, q2, #0
    // round-trips: sweep acc, coproc, kind (S/D/Q with valid register ranges) and immediates.
    for acc in [false, true] {
        for cp in 0..=7u8 {
            for &(kind, rmax) in &[(0u8, 31u8), (1u8, 15u8), (2u8, 7u8)] {
                for &rd in &[0u8, 1, rmax] {
                    for &imm in &[0u16, 1, 1023, 2047] {
                        round_trip(&Vcx1_T1(acc, kind, cp, rd, imm));
                    }
                    for &rn in &[0u8, rmax] {
                        for &imm in &[0u8, 1, 63] {
                            round_trip(&Vcx2_T1(acc, kind, cp, rd, rn, imm));
                        }
                        for &rm in &[0u8, rmax] {
                            for &imm in &[0u8, 1, 7] {
                                round_trip(&Vcx3_T1(acc, kind, cp, rd, rn, rm, imm));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn encode_round_trip__vscclrm() {
    use ArmT32Instruction::Vscclrm_T1;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        Vscclrm_T1(false, 0, 32).encode().unwrap(),
        vec![0x9f, 0xec, 0x20, 0x0a]
    ); // vscclrm {s0-s31, vpr}
    assert_eq!(
        Vscclrm_T1(false, 4, 28).encode().unwrap(),
        vec![0x9f, 0xec, 0x1c, 0x2a]
    ); // vscclrm {s4-s31, vpr}
    assert_eq!(
        Vscclrm_T1(true, 0, 32).encode().unwrap(),
        vec![0x9f, 0xec, 0x20, 0x0b]
    ); // vscclrm {d0-d15, vpr}
    assert_eq!(
        Vscclrm_T1(true, 0, 0).encode().unwrap(),
        vec![0x9f, 0xec, 0x00, 0x0b]
    ); // vscclrm {vpr}
    round_trip(&ArmT32Instruction::Vscclrm_T1(false, 0, 32));
    round_trip(&ArmT32Instruction::Vscclrm_T1(false, 4, 28));
    round_trip(&ArmT32Instruction::Vscclrm_T1(true, 0, 32));
    round_trip(&ArmT32Instruction::Vscclrm_T1(true, 0, 0));
    round_trip(&ArmT32Instruction::Vscclrm_T1(true, 2, 8));
}

#[test]
fn encode_round_trip__pacbti() {
    use ArmT32Instruction::{PacbtiData_T1, PacbtiHint_T1};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+pacbti` (Thumb)
    assert_eq!(
        PacbtiHint_T1(0).encode().unwrap(),
        vec![0xaf, 0xf3, 0x0f, 0x80]
    ); // bti
    assert_eq!(
        PacbtiHint_T1(1).encode().unwrap(),
        vec![0xaf, 0xf3, 0x1d, 0x80]
    ); // pac r12, lr, sp
    assert_eq!(
        PacbtiHint_T1(2).encode().unwrap(),
        vec![0xaf, 0xf3, 0x2d, 0x80]
    ); // aut r12, lr, sp
    assert_eq!(
        PacbtiHint_T1(3).encode().unwrap(),
        vec![0xaf, 0xf3, 0x0d, 0x80]
    ); // pacbti r12, lr, sp
    assert_eq!(
        PacbtiData_T1(2, R::R0, R::R1, R::R2).encode().unwrap(),
        vec![0x51, 0xfb, 0x12, 0x0f]
    ); // bxaut r0, r1, r2
    assert_eq!(
        PacbtiData_T1(0, R::R3, R::R4, R::R5).encode().unwrap(),
        vec![0x64, 0xfb, 0x05, 0xf3]
    ); // pacg  r3, r4, r5
    assert_eq!(
        PacbtiData_T1(1, R::R6, R::R7, R::R8).encode().unwrap(),
        vec![0x57, 0xfb, 0x08, 0x6f]
    ); // autg  r6, r7, r8
    for kind in 0..=3u8 {
        round_trip(&ArmT32Instruction::PacbtiHint_T1(kind));
    }
    for op in 0..=2u8 {
        for &(d, n, m) in &[(0u8, 1u8, 2u8), (3, 4, 5), (12, 11, 9)] {
            round_trip(&ArmT32Instruction::PacbtiData_T1(
                op,
                R::from_operand_bits(d),
                R::from_operand_bits(n),
                R::from_operand_bits(m),
            ));
        }
    }
}

#[test]
fn encode_round_trip__coprocessor() {
    use ArmT32Instruction::{Coproc_Cdp_T1, Coproc_Ldc_T1, Coproc_Mcr_T1, Coproc_Mcrr_T1};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main` (Thumb)
    assert_eq!(
        Coproc_Mcr_T1(false, false, 15, 0, R::R1, 0, 0, 0)
            .encode()
            .unwrap(),
        vec![0x00, 0xee, 0x10, 0x1f]
    ); // mcr  p15,0,r1,c0,c0,0
    assert_eq!(
        Coproc_Mcr_T1(true, false, 14, 1, R::R2, 3, 4, 2)
            .encode()
            .unwrap(),
        vec![0x23, 0xfe, 0x54, 0x2e]
    ); // mcr2 p14,1,r2,c3,c4,2
    assert_eq!(
        Coproc_Mcr_T1(false, true, 15, 0, R::R5, 0, 0, 0)
            .encode()
            .unwrap(),
        vec![0x10, 0xee, 0x10, 0x5f]
    ); // mrc  p15,0,r5,c0,c0,0
    assert_eq!(
        Coproc_Mcr_T1(true, true, 14, 7, R::R6, 1, 2, 5)
            .encode()
            .unwrap(),
        vec![0xf1, 0xfe, 0xb2, 0x6e]
    ); // mrc2 p14,7,r6,c1,c2,5
    assert_eq!(
        Coproc_Cdp_T1(false, 7, 4, 1, 2, 3, 5).encode().unwrap(),
        vec![0x42, 0xee, 0xa3, 0x17]
    ); // cdp  p7,4,c1,c2,c3,5
    assert_eq!(
        Coproc_Cdp_T1(true, 10, 1, 5, 6, 7, 2).encode().unwrap(),
        vec![0x16, 0xfe, 0x47, 0x5a]
    ); // cdp2 p10,1,c5,c6,c7,2
    assert_eq!(
        Coproc_Mcrr_T1(false, false, 15, 5, R::R0, R::R1, 2)
            .encode()
            .unwrap(),
        vec![0x41, 0xec, 0x52, 0x0f]
    ); // mcrr  p15,5,r0,r1,c2
    assert_eq!(
        Coproc_Mcrr_T1(false, true, 15, 0, R::R2, R::R3, 4)
            .encode()
            .unwrap(),
        vec![0x53, 0xec, 0x04, 0x2f]
    ); // mrrc  p15,0,r2,r3,c4
    assert_eq!(
        Coproc_Ldc_T1(false, false, true, 14, 5, R::R0, 16)
            .encode()
            .unwrap(),
        vec![0x90, 0xed, 0x04, 0x5e]
    ); // ldc  p14,c5,[r0,#16]
    assert_eq!(
        Coproc_Ldc_T1(false, true, true, 13, 0, R::R2, -8)
            .encode()
            .unwrap(),
        vec![0x52, 0xed, 0x02, 0x0d]
    ); // ldcl p13,c0,[r2,#-8]
    assert_eq!(
        Coproc_Ldc_T1(true, true, false, 12, 4, R::R3, 240)
            .encode()
            .unwrap(),
        vec![0xc3, 0xfd, 0x3c, 0x4c]
    ); // stc2l p12,c4,[r3,#240]
}

// NOTE: the generic coprocessor instructions are NOT round-trip-tested. Their 0xEC/0xEE encoding space
// pervasively overlaps the MVE/FP extensions (e.g. `cdp p15,...` shares bytes with MVE VSHRN, `ldc p14,...`
// with an MVE contiguous load). This decoder is MVE/FP-enabled, so it deliberately decodes those bytes as the
// MVE/FP form. The coprocessor *encoder* output is GNU-correct regardless (covered by the exact-byte test).

// Rule R4 (family-wide): "same bytes, different meaning" is resolved by an explicit `ArmDecodeContext`, never
// by guessing. The one T32 case is the CDE custom-datapath (CX*/VCX*) vs. generic coprocessor (CDP/MCR/LDC/...)
// overlap on coprocessors 0-7. `decode` keeps the historical canonical (coprocessor 0-7 = CDE) so it is
// non-breaking; `decode_with(context)` lets a caller decode chosen coprocessors as generic instead, and the
// byte-stable encode<->decode round-trip then holds PER CONTEXT.
#[test]
fn decode_with__cde_vs_generic_coprocessor_rule_r4() {
    use ArmT32Instruction::{Coproc_Ldc_T1, Vcx1_T1, Vcx2_T1, Vcx3_T1};

    // The exact collision the cargo-fuzz `t32_instruction_stream` target originally surfaced: `ldc2 p0, c4,
    // [r4]` and a CDE `vcx3a p0, ...` encode to the IDENTICAL word (0xFD94_0000). ENCODE is unambiguous (two
    // distinct variants); only DECODE has to choose.
    let ldc2 = Coproc_Ldc_T1(true, false, true, 0, 4, R::R4, 0); // ldc2 p0, c4, [r4]  (coprocessor 0)
    let bytes = ldc2.encode().unwrap();

    let decode_ctx = |context: &ArmDecodeContext| {
        let mut offset = 0;
        let instruction = ArmT32Instruction::decode_with(&mut bytes.iter(), &mut offset, context)
            .unwrap()
            .unwrap();
        assert_eq!(offset, bytes.len(), "consumed wrong byte count");
        instruction
    };

    // (1) The bare `decode` is unchanged and equals `decode_with(default)` / `decode_with(all_cde)`. The
    //     default treats coprocessor 0 as CDE, so the shared bytes decode as the CDE form -- NOT the LDC2.
    let mut offset = 0;
    let bare = ArmT32Instruction::decode(&mut bytes.iter(), &mut offset)
        .unwrap()
        .unwrap();
    assert_eq!(
        bare,
        decode_ctx(&ArmDecodeContext::default()),
        "decode must equal decode_with(default)"
    );
    assert_eq!(bare, decode_ctx(&ArmDecodeContext::all_cde()));
    assert_ne!(
        bare, ldc2,
        "default decode is the CDE canonical, not the generic LDC2"
    );
    assert!(
        matches!(bare, Vcx1_T1(..) | Vcx2_T1(..) | Vcx3_T1(..)),
        "the canonical is a CDE custom-datapath form"
    );
    assert_eq!(
        bare.encode().unwrap(),
        bytes,
        "the CDE canonical itself round-trips to the same bytes"
    );

    // (2) A context where coprocessor 0 is NOT CDE decodes the SAME bytes as the generic LDC2 -- round-tripping it.
    assert_eq!(
        decode_ctx(&ArmDecodeContext::no_cde()),
        ldc2,
        "no_cde must recover the generic LDC2"
    );

    // (3) Per-COPROCESSOR selectivity: a context with only coprocessor 0 CDE keeps the CDE reading; a context
    //     with coprocessor 0 cleared (but others set) flips these bytes to the LDC2.
    assert!(matches!(
        decode_ctx(&ArmDecodeContext::with_cde_coprocessors(0b0000_0001)),
        Vcx1_T1(..) | Vcx2_T1(..) | Vcx3_T1(..)
    ));
    assert_eq!(
        decode_ctx(&ArmDecodeContext::with_cde_coprocessors(0b1111_1110)),
        ldc2
    );

    // (4) The CDE forms across coprocessors 0-7 still round-trip under the default context (behaviour unchanged).
    for cp in 0..8u8 {
        round_trip(&Vcx1_T1(false, 0, cp, 0, 5));
    }

    // (5) The context predicate: coprocessors 8-15 can never be CDE.
    let context = ArmDecodeContext::with_cde_coprocessors(0b0000_0101);
    assert!(context.is_cde_coprocessor(0) && context.is_cde_coprocessor(2));
    assert!(!context.is_cde_coprocessor(1) && !context.is_cde_coprocessor(3));
    assert!(
        !context.is_cde_coprocessor(8)
            && !context.is_cde_coprocessor(10)
            && !context.is_cde_coprocessor(15)
    );
}

#[test]
fn encode_round_trip__fldmdbx() {
    use ArmT32Instruction::FldmdbxFstmdbx_T1;
    let d = |n: u8| crate::enums::Arm32DoublePrecisionRegister::new(n).unwrap();
    assert_eq!(
        FldmdbxFstmdbx_T1(true, R::R0, d(0), 4).encode().unwrap(),
        vec![0x30, 0xed, 0x09, 0x0b]
    ); // fldmdbx r0!, {d0-d3}
    assert_eq!(
        FldmdbxFstmdbx_T1(false, R::R1, d(4), 4).encode().unwrap(),
        vec![0x21, 0xed, 0x09, 0x4b]
    ); // fstmdbx r1!, {d4-d7}
    assert_eq!(
        FldmdbxFstmdbx_T1(true, R::R2, d(0), 16).encode().unwrap(),
        vec![0x32, 0xed, 0x21, 0x0b]
    ); // fldmdbx r2!, {d0-d15}
    for load in [false, true] {
        for first in [0u8, 4, 8] {
            for count in 1..=8u8 {
                round_trip(&ArmT32Instruction::FldmdbxFstmdbx_T1(
                    load,
                    R::R3,
                    d(first),
                    count,
                ));
            }
        }
    }
}

#[test]
fn encode_round_trip__vrintr() {
    use ArmT32Instruction::{Vrintr_Double_T1, Vrintr_Single_T1};
    let s = |n: u8| crate::enums::Arm32SinglePrecisionRegister::new(n).unwrap();
    let d = |n: u8| crate::enums::Arm32DoublePrecisionRegister::new(n).unwrap();
    assert_eq!(
        Vrintr_Single_T1(s(0), s(1)).encode().unwrap(),
        vec![0xb6, 0xee, 0x60, 0x0a]
    ); // vrintr.f32 s0, s1
    assert_eq!(
        Vrintr_Double_T1(d(0), d(1)).encode().unwrap(),
        vec![0xb6, 0xee, 0x41, 0x0b]
    ); // vrintr.f64 d0, d1
    assert_eq!(
        Vrintr_Single_T1(s(5), s(10)).encode().unwrap(),
        vec![0xf6, 0xee, 0x45, 0x2a]
    ); // vrintr.f32 s5, s10
    for &(a, b) in &[(0u8, 0u8), (31, 30), (15, 16)] {
        round_trip(&ArmT32Instruction::Vrintr_Single_T1(s(a), s(b)));
    }
    for &(a, b) in &[(0u8, 0u8), (15, 14), (10, 5)] {
        round_trip(&ArmT32Instruction::Vrintr_Double_T1(d(a), d(b)));
    }
}

#[test]
fn round_trip__saturating_shifts() {
    for op in 0..=3u8 {
        for rda in [0u8, 5, 12] {
            for imm in 1..=31u8 {
                round_trip(&ArmT32Instruction::SatShiftImm_T1(
                    op,
                    R::from_operand_bits(rda),
                    imm,
                ));
            }
        }
        for &(lo, hi) in &[(0u8, 1u8), (4, 5), (10, 11)] {
            for imm in 1..=31u8 {
                round_trip(&ArmT32Instruction::SatShiftLongImm_T1(
                    op,
                    R::from_operand_bits(lo),
                    R::from_operand_bits(hi),
                    imm,
                ));
            }
        }
    }
    for signed in [false, true] {
        for &(rda, rm) in &[(0u8, 1u8), (5, 8), (12, 14)] {
            round_trip(&ArmT32Instruction::SatShiftReg_T1(
                signed,
                R::from_operand_bits(rda),
                R::from_operand_bits(rm),
            ));
        }
        for &(lo, hi) in &[(0u8, 1u8), (4, 5), (10, 11)] {
            for sat48 in [false, true] {
                round_trip(&ArmT32Instruction::SatShiftLongReg_T1(
                    signed,
                    R::from_operand_bits(lo),
                    R::from_operand_bits(hi),
                    R::from_operand_bits(8),
                    sat48,
                ));
            }
        }
    }
}

#[test]
fn round_trip__long_shifts() {
    for op in 0..=2u8 {
        for &(lo, hi) in &[(0u8, 1u8), (4, 5), (10, 11)] {
            for imm in 1..=31u8 {
                round_trip(&ArmT32Instruction::LongShiftImm_T1(
                    op,
                    R::from_operand_bits(lo),
                    R::from_operand_bits(hi),
                    imm,
                ));
            }
            for rm in [0u8, 7, 12] {
                round_trip(&ArmT32Instruction::LongShiftReg_T1(
                    op,
                    R::from_operand_bits(lo),
                    R::from_operand_bits(hi),
                    R::from_operand_bits(rm),
                ));
            }
        }
    }
}

#[test]
fn round_trip__csel_family() {
    use crate::enums::ArmT32InstructionCondition as C;
    let conds = [
        C::Equal,
        C::NotEqual,
        C::SignedGreaterThanOrEqual,
        C::SignedLessThan,
        C::SignedGreaterThan,
        C::SignedLessThanOrEqual,
        C::CarrySet,
        C::Overflow,
    ];
    for op in 0..=3u8 {
        for &cond in &conds {
            // Rd/Rn/Rm are GPRs (PC/SP excluded -- Rd=15 would alias the saturating shifts, which is UNPREDICTABLE)
            for &(d, n, m) in &[(0u8, 1u8, 2u8), (14, 14, 14), (3, 3, 3), (12, 0, 8)] {
                round_trip(&ArmT32Instruction::Csel_T1(
                    op,
                    R::from_operand_bits(d),
                    R::from_operand_bits(n),
                    R::from_operand_bits(m),
                    cond,
                ));
            }
        }
    }
}

#[test]
fn round_trip__hints_barriers_clrm_vsel() {
    let s = |n: u8| crate::enums::Arm32SinglePrecisionRegister::new(n).unwrap();
    let d = |n: u8| crate::enums::Arm32DoublePrecisionRegister::new(n).unwrap();
    for opt in 0..=15u8 {
        round_trip(&ArmT32Instruction::Dbg_T1(opt));
    }
    round_trip(&ArmT32Instruction::Esb_T1);
    round_trip(&ArmT32Instruction::Ssbb_T1);
    round_trip(&ArmT32Instruction::Pssbb_T1);
    round_trip(&ArmT32Instruction::Sb_T1);
    for &(sd, dm) in &[(0u8, 1u8), (3, 5), (31, 15), (12, 8)] {
        round_trip(&ArmT32Instruction::Vjcvt_T1(s(sd), d(dm)));
    }
    for list in [0x0007u16, 0x410F, 0x0001, 0xCFFF, 0x8000, 0x4000] {
        round_trip(&ArmT32Instruction::Clrm_T1(list));
    }
    for cond in 0..=3u8 {
        for &(a, b, c) in &[(0u8, 1u8, 2u8), (31, 30, 5), (10, 20, 15)] {
            round_trip(&ArmT32Instruction::Vsel_Single_T1(cond, s(a), s(b), s(c)));
        }
        for &(a, b, c) in &[(0u8, 1u8, 2u8), (15, 14, 8), (10, 5, 12)] {
            round_trip(&ArmT32Instruction::Vsel_Double_T1(cond, d(a), d(b), d(c)));
        }
    }
}

#[test]
fn round_trip__lda_stl_acquire_release() {
    for size in 0..=2u8 {
        for &(a, b) in &[(0u8, 1u8), (3, 7), (10, 12)] {
            round_trip(&ArmT32Instruction::LoadAcquire_T1(
                size,
                false,
                R::from_operand_bits(a),
                R::from_operand_bits(b),
            ));
            round_trip(&ArmT32Instruction::LoadAcquire_T1(
                size,
                true,
                R::from_operand_bits(a),
                R::from_operand_bits(b),
            ));
            round_trip(&ArmT32Instruction::StoreRelease_T1(
                size,
                R::from_operand_bits(a),
                R::from_operand_bits(b),
            ));
            round_trip(&ArmT32Instruction::StoreReleaseExclusive_T1(
                size,
                R::from_operand_bits(a),
                R::from_operand_bits(b),
                R::from_operand_bits((a + 2) & 0xF),
            ));
        }
    }
}

#[test]
fn round_trip__mve_vmovl_vmovn_vaddlv() {
    for top in [false, true] {
        for unsigned in [false, true] {
            for size in [Arm32MveSize::I8, Arm32MveSize::I16] {
                for (d, m) in [(0u8, 0u8), (7, 1), (3, 5)] {
                    round_trip(&ArmT32Instruction::MveVmovl(
                        top,
                        unsigned,
                        size,
                        q(d),
                        q(m),
                    ));
                }
            }
        }
        for size in [Arm32MveSize::I16, Arm32MveSize::I32] {
            for (d, m) in [(0u8, 0u8), (7, 1), (4, 6)] {
                round_trip(&ArmT32Instruction::MveVmovn(top, size, q(d), q(m)));
            }
        }
    }
    for accumulate in [false, true] {
        for unsigned in [false, true] {
            // RdLo even, RdHi odd, independent (covers consecutive and non-consecutive pairs)
            for (lo, hi) in [(0u8, 1u8), (4, 5), (2, 7), (12, 1), (10, 13)] {
                round_trip(&ArmT32Instruction::MveVaddlv(
                    accumulate,
                    unsigned,
                    R::from_operand_bits(lo),
                    R::from_operand_bits(hi),
                    q(3),
                ));
            }
        }
    }
}

#[test]
fn encode__mve_complex_exact_bytes() {
    use Arm32MveFloatSize::{F16, F32};
    use Arm32MveSize as S;
    use ArmT32Instruction::{MveVcaddFloat, MveVcaddInt, MveVcmla, MveVcmul};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveVcaddInt(false, S::I8, false, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x02, 0xfe, 0x04, 0x0f]
    ); // vcadd.i8 q0,q1,q2,#90
    assert_eq!(
        MveVcaddInt(false, S::I32, false, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x22, 0xfe, 0x04, 0x0f]
    ); // vcadd.i32 q0,q1,q2,#90
    assert_eq!(
        MveVcaddInt(true, S::I16, true, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x12, 0xee, 0x04, 0x1f]
    ); // vhcadd.s16 q0,q1,q2,#270
    assert_eq!(
        MveVcaddFloat(F16, false, q(0), q(1), q(2))
            .encode()
            .unwrap(),
        vec![0x82, 0xfc, 0x44, 0x08]
    ); // vcadd.f16 q0,q1,q2,#90
    assert_eq!(
        MveVcaddFloat(F32, true, q(0), q(1), q(2)).encode().unwrap(),
        vec![0x92, 0xfd, 0x44, 0x08]
    ); // vcadd.f32 q0,q1,q2,#270
    assert_eq!(
        MveVcmul(F32, 0, q(0), q(1), q(2)).encode().unwrap(),
        vec![0x32, 0xfe, 0x04, 0x0e]
    ); // vcmul.f32 q0,q1,q2,#0
    assert_eq!(
        MveVcmul(F32, 1, q(0), q(1), q(2)).encode().unwrap(),
        vec![0x32, 0xfe, 0x05, 0x0e]
    ); // vcmul.f32 q0,q1,q2,#90
    assert_eq!(
        MveVcmul(F16, 2, q(0), q(1), q(2)).encode().unwrap(),
        vec![0x32, 0xee, 0x04, 0x1e]
    ); // vcmul.f16 q0,q1,q2,#180
    assert_eq!(
        MveVcmla(F32, 0, q(0), q(1), q(2)).encode().unwrap(),
        vec![0x32, 0xfc, 0x44, 0x08]
    ); // vcmla.f32 q0,q1,q2,#0
    assert_eq!(
        MveVcmla(F32, 1, q(0), q(1), q(2)).encode().unwrap(),
        vec![0xb2, 0xfc, 0x44, 0x08]
    ); // vcmla.f32 q0,q1,q2,#90
    assert_eq!(
        MveVcmla(F16, 3, q(0), q(1), q(2)).encode().unwrap(),
        vec![0xa2, 0xfd, 0x44, 0x08]
    ); // vcmla.f16 q0,q1,q2,#270
}

#[test]
fn round_trip__mve_complex() {
    let triples = [(0u8, 0u8, 0u8), (7, 1, 2), (3, 5, 7)];
    for halving in [false, true] {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            for rot270 in [false, true] {
                for (d, n, m) in triples {
                    round_trip(&ArmT32Instruction::MveVcaddInt(
                        halving,
                        size,
                        rot270,
                        q(d),
                        q(n),
                        q(m),
                    ));
                }
            }
        }
    }
    for size in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
        for rot270 in [false, true] {
            for (d, n, m) in triples {
                round_trip(&ArmT32Instruction::MveVcaddFloat(
                    size,
                    rot270,
                    q(d),
                    q(n),
                    q(m),
                ));
            }
        }
        for rotate in 0..4u8 {
            for (d, n, m) in triples {
                round_trip(&ArmT32Instruction::MveVcmul(size, rotate, q(d), q(n), q(m)));
                round_trip(&ArmT32Instruction::MveVcmla(size, rotate, q(d), q(n), q(m)));
            }
        }
    }
}

#[test]
fn encode_round_trip__mve_vpsel_vpnot() {
    use ArmT32Instruction::{MveVpnot, MveVpsel};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveVpsel(q(0), q(1), q(2)).encode().unwrap(),
        vec![0x33, 0xfe, 0x05, 0x0f]
    ); // vpsel q0, q1, q2
    assert_eq!(
        MveVpsel(q(7), q(3), q(4)).encode().unwrap(),
        vec![0x37, 0xfe, 0x09, 0xef]
    ); // vpsel q7, q3, q4
    assert_eq!(MveVpnot.encode().unwrap(), vec![0x31, 0xfe, 0x4d, 0x0f]); // vpnot
    round_trip(&MveVpnot);
    for d in 0..8u8 {
        for n in 0..8u8 {
            for m in 0..8u8 {
                round_trip(&ArmT32Instruction::MveVpsel(q(d), q(n), q(m)));
            }
        }
    }
}

#[test]
fn encode__mve_vcmp_exact_bytes() {
    use Arm32MveFloatSize::{F16, F32};
    use Arm32MveSize::*;
    use Arm32MveVcmpCondition::*;
    use ArmT32Instruction::{MveVcmpFloatReg, MveVcmpFloatScalar, MveVcmpReg, MveVcmpScalar};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveVcmpReg(Eq, I32, q(0), q(1)).encode().unwrap(),
        vec![0x21, 0xfe, 0x02, 0x0f]
    ); // vcmp.i32 eq, q0, q1
    assert_eq!(
        MveVcmpReg(Gt, I32, q(0), q(1)).encode().unwrap(),
        vec![0x21, 0xfe, 0x03, 0x1f]
    ); // vcmp.s32 gt, q0, q1
    assert_eq!(
        MveVcmpReg(Hi, I8, q(0), q(1)).encode().unwrap(),
        vec![0x01, 0xfe, 0x83, 0x0f]
    ); // vcmp.u8 hi, q0, q1
    assert_eq!(
        MveVcmpScalar(Eq, I32, q(0), R::R2).encode().unwrap(),
        vec![0x21, 0xfe, 0x42, 0x0f]
    ); // vcmp.i32 eq, q0, r2
    assert_eq!(
        MveVcmpScalar(Gt, I32, q(0), R::R2).encode().unwrap(),
        vec![0x21, 0xfe, 0x62, 0x1f]
    ); // vcmp.s32 gt, q0, r2
    assert_eq!(
        MveVcmpFloatReg(Eq, F32, q(0), q(1)).encode().unwrap(),
        vec![0x31, 0xee, 0x02, 0x0f]
    ); // vcmp.f32 eq, q0, q1
    assert_eq!(
        MveVcmpFloatReg(Ge, F16, q(0), q(1)).encode().unwrap(),
        vec![0x31, 0xfe, 0x02, 0x1f]
    ); // vcmp.f16 ge, q0, q1
    assert_eq!(
        MveVcmpFloatScalar(Eq, F32, q(0), R::R2).encode().unwrap(),
        vec![0x31, 0xee, 0x42, 0x0f]
    ); // vcmp.f32 eq, q0, r2
}

#[test]
fn round_trip__mve_vcmp() {
    let conds = Arm32MveVcmpCondition::ALL;
    for cond in conds {
        for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
            for (n, m) in [(0u8, 0u8), (7, 1), (3, 5)] {
                round_trip(&ArmT32Instruction::MveVcmpReg(cond, size, q(n), q(m)));
            }
            for (n, rm) in [(0u8, R::R0), (5, R::R7), (7, R::R14)] {
                round_trip(&ArmT32Instruction::MveVcmpScalar(cond, size, q(n), rm));
            }
        }
        // float VCMP supports only eq/ne/ge/lt/gt/le (not the cs/hi slots, which are reserved/VPSEL)
        if !matches!(cond, Arm32MveVcmpCondition::Cs | Arm32MveVcmpCondition::Hi) {
            for size in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
                round_trip(&ArmT32Instruction::MveVcmpFloatReg(cond, size, q(2), q(4)));
                round_trip(&ArmT32Instruction::MveVcmpFloatScalar(
                    cond,
                    size,
                    q(1),
                    R::R3,
                ));
            }
        }
    }
}

#[test]
fn encode_round_trip__mve_vpst() {
    use ArmT32Instruction::MveVpst;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveVpst(0b1000).encode().unwrap(),
        vec![0x71, 0xfe, 0x4d, 0x0f]
    ); // vpst
    assert_eq!(
        MveVpst(0b0100).encode().unwrap(),
        vec![0x31, 0xfe, 0x4d, 0x8f]
    ); // vpstt
    assert_eq!(
        MveVpst(0b1100).encode().unwrap(),
        vec![0x71, 0xfe, 0x4d, 0x8f]
    ); // vpste
    assert_eq!(
        MveVpst(0b1110).encode().unwrap(),
        vec![0x71, 0xfe, 0x4d, 0xcf]
    ); // vpstet
    assert_eq!(
        MveVpst(0b0001).encode().unwrap(),
        vec![0x31, 0xfe, 0x4d, 0x2f]
    ); // vpstttt
    assert_eq!(
        MveVpst(0b1001).encode().unwrap(),
        vec![0x71, 0xfe, 0x4d, 0x2f]
    ); // vpsteee
    // mask 0 still decodes as VPNOT (VPST and VPNOT share the opcode)
    round_trip(&ArmT32Instruction::MveVpnot);
    for mask in 1..=15u8 {
        round_trip(&ArmT32Instruction::MveVpst(mask));
    }
}

#[test]
fn encode__mve_vpt_exact_bytes() {
    use Arm32MveFloatSize::{F16, F32};
    use Arm32MveSize::*;
    use Arm32MveVcmpCondition::*;
    use ArmT32Instruction::{MveVptFloatReg, MveVptFloatScalar, MveVptReg, MveVptScalar};
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb); VPT = VCMP | mask bits
    assert_eq!(
        MveVptReg(Eq, I16, q(0), q(1), 0b1000).encode().unwrap(),
        vec![0x51, 0xfe, 0x02, 0x0f]
    ); // vpt.i16 eq, q0, q1
    assert_eq!(
        MveVptReg(Ne, I32, q(2), q(3), 0b0100).encode().unwrap(),
        vec![0x25, 0xfe, 0x86, 0x8f]
    ); // vptt.i32 ne, q2, q3
    assert_eq!(
        MveVptReg(Gt, I8, q(0), q(1), 0b1100).encode().unwrap(),
        vec![0x41, 0xfe, 0x03, 0x9f]
    ); // vpte.s8 gt, q0, q1
    assert_eq!(
        MveVptScalar(Eq, I32, q(0), R::R2, 0b1000).encode().unwrap(),
        vec![0x61, 0xfe, 0x42, 0x0f]
    ); // vpt.i32 eq, q0, r2
    assert_eq!(
        MveVptFloatReg(Ge, F32, q(0), q(1), 0b0100)
            .encode()
            .unwrap(),
        vec![0x31, 0xee, 0x02, 0x9f]
    ); // vptt.f32 ge, q0, q1
    assert_eq!(
        MveVptFloatScalar(Eq, F16, q(0), R::R2, 0b1000)
            .encode()
            .unwrap(),
        vec![0x71, 0xfe, 0x42, 0x0f]
    ); // vpt.f16 eq, q0, r2
}

#[test]
fn round_trip__mve_vpt() {
    for cond in Arm32MveVcmpCondition::ALL {
        for mask in 1..=15u8 {
            for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
                round_trip(&ArmT32Instruction::MveVptReg(cond, size, q(0), q(1), mask));
                round_trip(&ArmT32Instruction::MveVptScalar(
                    cond,
                    size,
                    q(2),
                    R::R5,
                    mask,
                ));
            }
            // float VPT excludes the cs/hi conditions (reserved slots)
            if !matches!(cond, Arm32MveVcmpCondition::Cs | Arm32MveVcmpCondition::Hi) {
                for size in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
                    round_trip(&ArmT32Instruction::MveVptFloatReg(
                        cond,
                        size,
                        q(3),
                        q(4),
                        mask,
                    ));
                    round_trip(&ArmT32Instruction::MveVptFloatScalar(
                        cond,
                        size,
                        q(1),
                        R::R7,
                        mask,
                    ));
                }
            }
        }
    }
}

#[test]
fn encode_round_trip__mve_float_reductions() {
    use Arm32MveFloatReduceOp::*;
    use Arm32MveFloatSize::{F16, F32};
    use ArmT32Instruction::MveFloatReduce;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb)
    assert_eq!(
        MveFloatReduce(Vmaxnmv, F32, R::R0, q(1)).encode().unwrap(),
        vec![0xee, 0xee, 0x02, 0x0f]
    ); // vmaxnmv.f32  r0, q1
    assert_eq!(
        MveFloatReduce(Vminnmv, F32, R::R0, q(1)).encode().unwrap(),
        vec![0xee, 0xee, 0x82, 0x0f]
    ); // vminnmv.f32  r0, q1
    assert_eq!(
        MveFloatReduce(Vmaxnmav, F32, R::R0, q(1)).encode().unwrap(),
        vec![0xec, 0xee, 0x02, 0x0f]
    ); // vmaxnmav.f32 r0, q1
    assert_eq!(
        MveFloatReduce(Vminnmav, F32, R::R0, q(1)).encode().unwrap(),
        vec![0xec, 0xee, 0x82, 0x0f]
    ); // vminnmav.f32 r0, q1
    assert_eq!(
        MveFloatReduce(Vmaxnmv, F16, R::R0, q(1)).encode().unwrap(),
        vec![0xee, 0xfe, 0x02, 0x0f]
    ); // vmaxnmv.f16  r0, q1
    assert_eq!(
        MveFloatReduce(Vmaxnmv, F32, R::R4, q(7)).encode().unwrap(),
        vec![0xee, 0xee, 0x0e, 0x4f]
    ); // vmaxnmv.f32  r4, q7
    for op in Arm32MveFloatReduceOp::ALL {
        for size in [F16, F32] {
            for rd in [R::R0, R::R1, R::R8, R::R14] {
                for m in [0u8, 3, 7] {
                    round_trip(&MveFloatReduce(op, size, rd, q(m)));
                }
            }
        }
    }
}

#[test]
fn encode_round_trip__mve_vcvt_round() {
    use Arm32MveFloatSize::{F16, F32};
    use ArmT32Instruction::MveVcvtRound;
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb).
    // rounding: 0=a 1=n 2=p 3=m, second arg = unsigned.
    assert_eq!(
        MveVcvtRound(0, false, F32, q(0), q(1)).encode().unwrap(),
        vec![0xbb, 0xff, 0x42, 0x00]
    ); // vcvta.s32.f32 q0, q1
    assert_eq!(
        MveVcvtRound(1, false, F32, q(0), q(1)).encode().unwrap(),
        vec![0xbb, 0xff, 0x42, 0x01]
    ); // vcvtn.s32.f32 q0, q1
    assert_eq!(
        MveVcvtRound(2, false, F32, q(0), q(1)).encode().unwrap(),
        vec![0xbb, 0xff, 0x42, 0x02]
    ); // vcvtp.s32.f32 q0, q1
    assert_eq!(
        MveVcvtRound(3, false, F32, q(0), q(1)).encode().unwrap(),
        vec![0xbb, 0xff, 0x42, 0x03]
    ); // vcvtm.s32.f32 q0, q1
    assert_eq!(
        MveVcvtRound(0, true, F32, q(0), q(1)).encode().unwrap(),
        vec![0xbb, 0xff, 0xc2, 0x00]
    ); // vcvta.u32.f32 q0, q1
    assert_eq!(
        MveVcvtRound(3, true, F32, q(0), q(1)).encode().unwrap(),
        vec![0xbb, 0xff, 0xc2, 0x03]
    ); // vcvtm.u32.f32 q0, q1
    assert_eq!(
        MveVcvtRound(0, false, F16, q(0), q(1)).encode().unwrap(),
        vec![0xb7, 0xff, 0x42, 0x00]
    ); // vcvta.s16.f16 q0, q1
    assert_eq!(
        MveVcvtRound(3, true, F16, q(0), q(1)).encode().unwrap(),
        vec![0xb7, 0xff, 0xc2, 0x03]
    ); // vcvtm.u16.f16 q0, q1
    assert_eq!(
        MveVcvtRound(0, false, F32, q(2), q(5)).encode().unwrap(),
        vec![0xbb, 0xff, 0x4a, 0x40]
    ); // vcvta.s32.f32 q2, q5
    assert_eq!(
        MveVcvtRound(2, true, F16, q(7), q(3)).encode().unwrap(),
        vec![0xb7, 0xff, 0xc6, 0xe2]
    ); // vcvtp.u16.f16 q7, q3
    for rounding in 0u8..4 {
        for unsigned in [false, true] {
            for size in [F16, F32] {
                for d in [0u8, 3, 7] {
                    for m in [0u8, 4, 7] {
                        round_trip(&MveVcvtRound(rounding, unsigned, size, q(d), q(m)));
                    }
                }
            }
        }
    }
}

#[test]
fn encode_round_trip__mve_dual_mac() {
    use Arm32MveSize::{I8, I16, I32};
    use ArmT32Instruction::MveDualMac;
    // MveDualMac(subtract, exchange, accumulate, unsigned, size, rda, qn, qm).
    // bytes verified against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (Thumb).
    assert_eq!(
        MveDualMac(false, false, false, false, I16, R::R0, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf2, 0xee, 0x04, 0x0e]
    ); // vmladav.s16   r0, q1, q2
    assert_eq!(
        MveDualMac(false, false, true, false, I16, R::R14, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf2, 0xee, 0x24, 0xee]
    ); // vmladava.s16  lr, q1, q2
    assert_eq!(
        MveDualMac(false, true, false, false, I16, R::R4, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf2, 0xee, 0x04, 0x5e]
    ); // vmladavx.s16  r4, q1, q2
    assert_eq!(
        MveDualMac(false, true, true, false, I16, R::R0, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf2, 0xee, 0x24, 0x1e]
    ); // vmladavax.s16 r0, q1, q2
    assert_eq!(
        MveDualMac(false, false, false, true, I16, R::R0, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf2, 0xfe, 0x04, 0x0e]
    ); // vmladav.u16   r0, q1, q2
    assert_eq!(
        MveDualMac(true, false, false, false, I16, R::R0, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf2, 0xee, 0x05, 0x0e]
    ); // vmlsdav.s16   r0, q1, q2
    assert_eq!(
        MveDualMac(true, true, false, false, I16, R::R0, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf2, 0xee, 0x05, 0x1e]
    ); // vmlsdavx.s16  r0, q1, q2
    // the subtract form's size encoding is irregular: .s8 sets bit28 (not bit8), .s32 sets bit16
    assert_eq!(
        MveDualMac(true, false, false, false, I8, R::R0, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf2, 0xfe, 0x05, 0x0e]
    ); // vmlsdav.s8    r0, q1, q2
    assert_eq!(
        MveDualMac(true, false, false, false, I32, R::R0, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf3, 0xee, 0x05, 0x0e]
    ); // vmlsdav.s32   r0, q1, q2
    assert_eq!(
        MveDualMac(false, false, false, false, I8, R::R0, q(3), q(4))
            .encode()
            .unwrap(),
        vec![0xf6, 0xee, 0x08, 0x0f]
    ); // vmladav.s8    r0, q3, q4
    assert_eq!(
        MveDualMac(false, false, false, false, I32, R::R0, q(7), q(0))
            .encode()
            .unwrap(),
        vec![0xff, 0xee, 0x00, 0x0e]
    ); // vmladav.s32   r0, q7, q0
    assert_eq!(
        MveDualMac(false, false, false, true, I32, R::R2, q(1), q(2))
            .encode()
            .unwrap(),
        vec![0xf3, 0xfe, 0x04, 0x2e]
    ); // vmladav.u32   r2, q1, q2
    // exhaustive round-trip over the VALID matrix (exchange & subtract are signed-only; unsigned has neither)
    for size in [I8, I16, I32] {
        for rda in [R::R0, R::R2, R::R12, R::R14] {
            for n in [0u8, 3, 7] {
                for m in [0u8, 4, 7] {
                    for &(subtract, exchange) in
                        &[(false, false), (false, true), (true, false), (true, true)]
                    {
                        for accumulate in [false, true] {
                            round_trip(&MveDualMac(
                                subtract,
                                exchange,
                                accumulate,
                                false,
                                size,
                                rda,
                                q(n),
                                q(m),
                            ));
                        }
                    }
                    for accumulate in [false, true] {
                        round_trip(&MveDualMac(
                            false,
                            false,
                            accumulate,
                            true,
                            size,
                            rda,
                            q(n),
                            q(m),
                        ));
                    }
                }
            }
        }
    }
}

