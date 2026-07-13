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

