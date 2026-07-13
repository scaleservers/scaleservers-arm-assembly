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

