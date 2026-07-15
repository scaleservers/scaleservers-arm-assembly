// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// Emit (UAL text) assertions for the T32 emitter, concentrating on the ARMv8.1-M MVE ("Helium") surface --
// the largest block the differential oracle exercises but `--lib` coverage misses (the oracle is an
// integration test in `tests/`). Every op-enum family is walked via its `::ALL` table so each mnemonic arm of
// the emit helpers is reached; the flag/standalone forms get representative instances. Expected strings were
// captured from the emitter and cross-checked against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp`.

use crate::enums::{
    Arm32GeneralPurposeRegister as R, Arm32LowGeneralPurposeRegister as L, Arm32MveBitwiseOp,
    Arm32MveFloatArithOp, Arm32MveFloatReduceOp, Arm32MveFloatSize, Arm32MveIntArithOp,
    Arm32MveLongMacOp, Arm32MveMisc2FloatOp, Arm32MveMisc2Op, Arm32MveQMovnKind, Arm32MveReduceOp,
    Arm32MveShiftImmOp, Arm32MveShiftNarrowOp, Arm32MveSize, Arm32MveVcmpCondition,
    Arm32MveVecScalarFloatOp, Arm32MveVecScalarIntOp, Arm32MveVectorRegister, Arm32MveVrintOp,
    ArmT32FpDataOperation2 as F2, ArmT32FpDataOperation3 as F3, ArmT32IndexMode as Idx,
    ArmT32InstructionCondition as Cc, ArmT32RegisterShift as Sh, ArmT32SpecialRegister as Spec,
};
use crate::{
    Arm32DirectedRound as DRnd, Arm32DoublePrecisionRegister, Arm32SinglePrecisionRegister,
    ArmAssemblySyntax, ArmT32Instruction,
};

const GNU: ArmAssemblySyntax = ArmAssemblySyntax::Gnu;

fn mq(number: u8) -> Arm32MveVectorRegister {
    Arm32MveVectorRegister::new(number).unwrap()
}
fn s(number: u8) -> Arm32SinglePrecisionRegister {
    Arm32SinglePrecisionRegister::new(number).unwrap()
}
fn d(number: u8) -> Arm32DoublePrecisionRegister {
    Arm32DoublePrecisionRegister::new(number).unwrap()
}

/// Every MVE instruction whose emit text is asserted below, in the same order as `EXPECTED_T32_MVE_GNU`.
fn t32_mve_cases() -> Vec<ArmT32Instruction> {
    use Arm32MveFloatSize::{F16, F32};
    use Arm32MveSize::{I16, I32, I8};
    use ArmT32Instruction::*;
    let mut v = Vec::new();

    // ---- 3-reg-same: integer / bitwise / float ----
    for op in Arm32MveIntArithOp::ALL {
        v.push(MveIntArith(op, I16, mq(0), mq(1), mq(2)));
    }
    for size in [I8, I32] {
        v.push(MveIntArith(Arm32MveIntArithOp::Vadd, size, mq(0), mq(1), mq(2)));
    }
    for op in Arm32MveBitwiseOp::ALL {
        v.push(MveBitwise(op, mq(0), mq(1), mq(2)));
    }
    for op in Arm32MveFloatArithOp::ALL {
        v.push(MveFloatArith(op, F32, mq(0), mq(1), mq(2)));
    }
    v.push(MveFloatArith(Arm32MveFloatArithOp::Vadd, F16, mq(0), mq(1), mq(2)));

    // ---- vector-by-scalar (Qd, Qn, Rm) ----
    for op in Arm32MveVecScalarIntOp::ALL {
        v.push(MveVecScalarInt(op, I16, mq(0), mq(1), R::R2));
    }
    for op in Arm32MveVecScalarFloatOp::ALL {
        v.push(MveVecScalarFloat(op, F32, mq(0), mq(1), R::R2));
    }

    // ---- VDUP / shift-by-immediate / modified-immediate ----
    for size in [I8, I16, I32] {
        v.push(MveVdup(size, mq(0), R::R1));
    }
    for op in Arm32MveShiftImmOp::ALL {
        v.push(MveShiftImm(op, I16, 3, mq(0), mq(1)));
    }
    for cmode in 0..16u8 {
        v.push(MveModifiedImmediate(cmode, false, 0x55, mq(0)));
    }
    v.push(MveModifiedImmediate(0, true, 1, mq(0)));
    v.push(MveModifiedImmediate(14, true, 0xFF, mq(0)));

    // ---- two-register miscellaneous ----
    for op in Arm32MveMisc2Op::ALL {
        v.push(MveMisc2(op, I8, mq(0), mq(1)));
    }
    for op in Arm32MveMisc2FloatOp::ALL {
        v.push(MveMisc2Float(op, F32, mq(0), mq(1)));
    }
    for is_min in [false, true] {
        v.push(MveVmaxaMina(is_min, I16, mq(0), mq(1)));
    }
    for is_min in [false, true] {
        v.push(MveVmaxnmaMinnma(is_min, F32, mq(0), mq(1)));
    }
    v.push(MveMvnRegister(mq(0), mq(1)));

    // ---- contiguous load/store + gather/scatter + interleave ----
    for size in [I8, I16, I32] {
        v.push(MveLoadStore(true, size, mq(0), R::R1, 8, Idx::Offset));
    }
    v.push(MveLoadStore(false, I32, mq(0), R::R1, 16, Idx::PreIndex));
    v.push(MveLoadStore(true, I16, mq(0), R::R1, -8, Idx::PostIndex));
    v.push(MveGatherScatter(true, false, 32, 32, true, mq(0), R::R1, mq(2)));
    v.push(MveGatherScatter(false, false, 16, 16, false, mq(0), R::R1, mq(2)));
    v.push(MveGatherScatterBase(true, false, false, mq(0), mq(1), 4));
    v.push(MveGatherScatterBase(false, true, true, mq(0), mq(1), 8));
    v.push(MveInterleave(true, false, 0, I8, mq(0), R::R1, false));
    v.push(MveInterleave(false, true, 0, I32, mq(0), R::R1, true));

    // ---- tail predication / cross-lane reductions ----
    for size in [8u8, 16, 32, 64] {
        v.push(MveVctp(size, R::R0));
    }
    for op in Arm32MveReduceOp::ALL {
        v.push(MveReduce(op, I16, R::R0, mq(1)));
    }
    for signed in [true, false] {
        v.push(MveVabav(signed, I16, R::R0, mq(1), mq(2)));
    }
    v.push(MveDualMac(false, false, false, false, I16, R::R0, mq(1), mq(2)));
    v.push(MveDualMac(true, true, true, false, I8, R::R0, mq(1), mq(2)));
    for op in [
        Arm32MveLongMacOp::Vmlaldav,
        Arm32MveLongMacOp::Vmlsldav,
        Arm32MveLongMacOp::Vrmlaldavh,
        Arm32MveLongMacOp::Vrmlsldavh,
    ] {
        v.push(MveLongDualMac(op, false, false, false, I32, R::R0, R::R1, mq(2), mq(3)));
    }
    for op in Arm32MveFloatReduceOp::ALL {
        v.push(MveFloatReduce(op, F32, R::R0, mq(1)));
    }

    // ---- VMOV two-lane / VRINT / VCVT family ----
    v.push(MveVmovTwoLane(true, 0, mq(0), R::R1, R::R2));
    v.push(MveVmovTwoLane(false, 1, mq(0), R::R1, R::R2));
    for op in Arm32MveVrintOp::ALL {
        v.push(MveVrint(op, F32, mq(0), mq(1)));
    }
    v.push(MveVcvtFloatInt(true, false, F32, mq(0), mq(1)));
    v.push(MveVcvtFloatInt(false, true, F16, mq(0), mq(1)));
    v.push(MveVcvtFixed(true, false, F32, 4, mq(0), mq(1)));
    v.push(MveVcvtFixed(false, true, F16, 2, mq(0), mq(1)));
    v.push(MveVcvtHalf(false, true, mq(0), mq(1)));
    v.push(MveVcvtHalf(true, false, mq(0), mq(1)));
    for rounding in [0u8, 1, 2, 3] {
        v.push(MveVcvtRound(rounding, false, F32, mq(0), mq(1)));
    }
    v.push(MveVcvtRound(1, true, F16, mq(0), mq(1)));

    // ---- shift-and-narrow / width-changing moves ----
    for op in [
        Arm32MveShiftNarrowOp::Vshrn,
        Arm32MveShiftNarrowOp::Vrshrn,
        Arm32MveShiftNarrowOp::Vqshrn,
        Arm32MveShiftNarrowOp::Vqrshrn,
        Arm32MveShiftNarrowOp::Vqshrun,
        Arm32MveShiftNarrowOp::Vqrshrun,
    ] {
        v.push(MveShiftNarrow(op, false, false, true, 3, mq(0), mq(1)));
    }
    v.push(MveShiftNarrow(Arm32MveShiftNarrowOp::Vqshrn, true, true, true, 5, mq(0), mq(1)));
    for (top, uns, size) in [(false, false, I8), (true, true, I16)] {
        v.push(MveVmovl(top, uns, size, mq(0), mq(1)));
    }
    for (top, size) in [(false, I16), (true, I32)] {
        v.push(MveVmovn(top, size, mq(0), mq(1)));
    }
    v.push(MveVqmovn(Arm32MveQMovnKind::Vqmovn, false, false, I16, mq(0), mq(1)));
    v.push(MveVqmovn(Arm32MveQMovnKind::Vqmovn, true, true, I32, mq(0), mq(1)));
    v.push(MveVqmovn(Arm32MveQMovnKind::Vqmovun, false, false, I16, mq(0), mq(1)));

    // ---- long / high multiplies ----
    v.push(MveVmull(false, false, false, I8, mq(0), mq(1), mq(2)));
    v.push(MveVmull(false, true, true, I16, mq(0), mq(1), mq(2)));
    v.push(MveVmull(true, false, false, I8, mq(0), mq(1), mq(2)));
    v.push(MveVmulh(false, false, I16, mq(0), mq(1), mq(2)));
    v.push(MveVmulh(true, true, I8, mq(0), mq(1), mq(2)));
    v.push(MveVqdmull(false, false, mq(0), mq(1), mq(2)));
    v.push(MveVqdmull(true, true, mq(0), mq(1), mq(2)));
    v.push(MveVqdmullScalar(false, true, mq(0), mq(1), R::R2));
    v.push(MveVqdmladh(false, false, false, I16, mq(0), mq(1), mq(2)));
    v.push(MveVqdmladh(true, true, true, I8, mq(0), mq(1), mq(2)));

    // ---- shift by vector / scalar + widening shift-left-long ----
    v.push(MveShiftByVector(false, false, false, I16, mq(0), mq(1), mq(2)));
    v.push(MveShiftByVector(true, true, true, I8, mq(0), mq(1), mq(2)));
    v.push(MveShiftByScalar(false, false, false, I16, mq(0), R::R1));
    v.push(MveShiftByScalar(true, true, true, I8, mq(0), R::R1));
    v.push(MveVshll(false, false, I8, 3, mq(0), mq(1)));
    v.push(MveVshll(true, true, I16, 8, mq(0), mq(1)));
    v.push(MveVaddlv(false, false, R::R0, R::R1, mq(2)));
    v.push(MveVaddlv(true, true, R::R0, R::R1, mq(2)));

    // ---- complex-number ops ----
    v.push(MveVcaddInt(false, I16, false, mq(0), mq(1), mq(2)));
    v.push(MveVcaddInt(true, I16, true, mq(0), mq(1), mq(2)));
    v.push(MveVcaddFloat(F32, false, mq(0), mq(1), mq(2)));
    for rot in [0u8, 1, 2, 3] {
        v.push(MveVcmul(F32, rot, mq(0), mq(1), mq(2)));
    }
    for rot in [0u8, 1, 2, 3] {
        v.push(MveVcmla(F16, rot, mq(0), mq(1), mq(2)));
    }

    // ---- predication + carry-chain + index generators ----
    v.push(MveVpsel(mq(0), mq(1), mq(2)));
    v.push(MveVpnot);
    v.push(MveVadc(false, false, mq(0), mq(1), mq(2)));
    v.push(MveVadc(true, true, mq(0), mq(1), mq(2)));
    v.push(MveVshlc(5, mq(0), R::R1));
    v.push(MveViddup(false, I16, mq(0), R::R0, None, 2));
    v.push(MveViddup(true, I8, mq(0), R::R0, Some(R::R3), 4));
    v.push(MveVbrsr(I16, mq(0), mq(1), R::R2));

    // ---- VCMP / VPT ----
    for cond in Arm32MveVcmpCondition::ALL {
        v.push(MveVcmpReg(cond, I16, mq(0), mq(1)));
    }
    v.push(MveVcmpScalar(Arm32MveVcmpCondition::Eq, I32, mq(0), R::R1));
    v.push(MveVcmpFloatReg(Arm32MveVcmpCondition::Ge, F32, mq(0), mq(1)));
    v.push(MveVcmpFloatScalar(Arm32MveVcmpCondition::Gt, F16, mq(0), R::R1));
    for mask in [0b1000u8, 0b1100, 0b1110, 0b1111] {
        v.push(MveVpst(mask));
    }
    v.push(MveVptReg(Arm32MveVcmpCondition::Eq, I16, mq(0), mq(1), 0b1000));
    v.push(MveVptScalar(Arm32MveVcmpCondition::Ne, I8, mq(0), R::R1, 0b1100));
    v.push(MveVptFloatReg(Arm32MveVcmpCondition::Ge, F32, mq(0), mq(1), 0b1000));
    v.push(MveVptFloatScalar(Arm32MveVcmpCondition::Lt, F16, mq(0), R::R1, 0b1110));

    // ---- low-overhead loops ----
    v.push(LobStart(true, None, R::R0, 0x100));
    v.push(LobStart(false, None, R::R0, 0x100));
    v.push(LobStart(true, Some(8), R::R0, 0x100));
    v.push(LobStart(false, Some(16), R::R0, 0x100));
    v.push(LobEnd(false, 0x40));
    v.push(LobEnd(true, 0x40));
    v.push(Lctp);

    v
}

/// GNU-flavor UAL text for each entry of `t32_mve_cases()`, in the same order. Captured from the emitter and
/// cross-checked against `arm-none-eabi-as -march=armv8.1-m.main+mve.fp` (the differential oracle).
const EXPECTED_T32_MVE_GNU: &[&str] = &[
    "vadd.i16 q0, q1, q2",
    "vsub.i16 q0, q1, q2",
    "vmul.i16 q0, q1, q2",
    "vqadd.s16 q0, q1, q2",
    "vqadd.u16 q0, q1, q2",
    "vqsub.s16 q0, q1, q2",
    "vqsub.u16 q0, q1, q2",
    "vhadd.s16 q0, q1, q2",
    "vhadd.u16 q0, q1, q2",
    "vhsub.s16 q0, q1, q2",
    "vhsub.u16 q0, q1, q2",
    "vrhadd.s16 q0, q1, q2",
    "vrhadd.u16 q0, q1, q2",
    "vabd.s16 q0, q1, q2",
    "vabd.u16 q0, q1, q2",
    "vmax.s16 q0, q1, q2",
    "vmax.u16 q0, q1, q2",
    "vmin.s16 q0, q1, q2",
    "vmin.u16 q0, q1, q2",
    "vqdmulh.s16 q0, q1, q2",
    "vqrdmulh.s16 q0, q1, q2",
    "vadd.i8 q0, q1, q2",
    "vadd.i32 q0, q1, q2",
    "vand q0, q1, q2",
    "vbic q0, q1, q2",
    "vorr q0, q1, q2",
    "vorn q0, q1, q2",
    "veor q0, q1, q2",
    "vadd.f32 q0, q1, q2",
    "vsub.f32 q0, q1, q2",
    "vmul.f32 q0, q1, q2",
    "vabd.f32 q0, q1, q2",
    "vmaxnm.f32 q0, q1, q2",
    "vminnm.f32 q0, q1, q2",
    "vfma.f32 q0, q1, q2",
    "vfms.f32 q0, q1, q2",
    "vadd.f16 q0, q1, q2",
    "vadd.i16 q0, q1, r2",
    "vsub.i16 q0, q1, r2",
    "vmul.i16 q0, q1, r2",
    "vhadd.s16 q0, q1, r2",
    "vhadd.u16 q0, q1, r2",
    "vhsub.s16 q0, q1, r2",
    "vhsub.u16 q0, q1, r2",
    "vqadd.s16 q0, q1, r2",
    "vqadd.u16 q0, q1, r2",
    "vqsub.s16 q0, q1, r2",
    "vqsub.u16 q0, q1, r2",
    "vqdmulh.s16 q0, q1, r2",
    "vqrdmulh.s16 q0, q1, r2",
    "vmla.i16 q0, q1, r2",
    "vmlas.i16 q0, q1, r2",
    "vqdmlah.s16 q0, q1, r2",
    "vqrdmlah.s16 q0, q1, r2",
    "vqdmlash.s16 q0, q1, r2",
    "vqrdmlash.s16 q0, q1, r2",
    "vadd.f32 q0, q1, r2",
    "vsub.f32 q0, q1, r2",
    "vmul.f32 q0, q1, r2",
    "vfma.f32 q0, q1, r2",
    "vfmas.f32 q0, q1, r2",
    "vdup.8 q0, r1",
    "vdup.16 q0, r1",
    "vdup.32 q0, r1",
    "vshr.s16 q0, q1, #3",
    "vshr.u16 q0, q1, #3",
    "vrshr.s16 q0, q1, #3",
    "vrshr.u16 q0, q1, #3",
    "vsri.16 q0, q1, #3",
    "vshl.i16 q0, q1, #3",
    "vsli.16 q0, q1, #3",
    "vqshl.s16 q0, q1, #3",
    "vqshl.u16 q0, q1, #3",
    "vqshlu.s16 q0, q1, #3",
    "vmov.i32 q0, #0x55",
    "vorr.i32 q0, #0x55",
    "vmov.i32 q0, #0x5500",
    "vorr.i32 q0, #0x5500",
    "vmov.i32 q0, #0x550000",
    "vorr.i32 q0, #0x550000",
    "vmov.i32 q0, #0x55000000",
    "vorr.i32 q0, #0x55000000",
    "vmov.i16 q0, #0x55",
    "vorr.i16 q0, #0x55",
    "vmov.i16 q0, #0x5500",
    "vorr.i16 q0, #0x5500",
    "vmov.i32 q0, #0x55ff",
    "vmov.i32 q0, #0x55ffff",
    "vmov.i8 q0, #0x55",
    "vmov.f32 q0, #0.328125",
    "vmvn.i32 q0, #0x1",
    "vmov.i64 q0, #0xffffffffffffffff",
    "vrev64.8 q0, q1",
    "vrev32.8 q0, q1",
    "vrev16.8 q0, q1",
    "vcls.s8 q0, q1",
    "vclz.i8 q0, q1",
    "vabs.s8 q0, q1",
    "vneg.s8 q0, q1",
    "vqabs.s8 q0, q1",
    "vqneg.s8 q0, q1",
    "vabs.f32 q0, q1",
    "vneg.f32 q0, q1",
    "vmaxa.s16 q0, q1",
    "vmina.s16 q0, q1",
    "vmaxnma.f32 q0, q1",
    "vminnma.f32 q0, q1",
    "vmvn q0, q1",
    "vldrb.u8 q0, [r1, #8]",
    "vldrh.u16 q0, [r1, #8]",
    "vldrw.u32 q0, [r1, #8]",
    "vstrw.32 q0, [r1, #16]!",
    "vldrh.u16 q0, [r1], #-8",
    "vldrw.s32 q0, [r1, q2, uxtw #2]",
    "vstrh.16 q0, [r1, q2]",
    "vldrw.u32 q0, [q1, #4]",
    "vstrd.64 q0, [q1, #8]!",
    "vld20.8 {q0, q1}, [r1]",
    "vst40.32 {q0, q1, q2, q3}, [r1]!",
    "vctp.8 r0",
    "vctp.16 r0",
    "vctp.32 r0",
    "vctp.64 r0",
    "vaddv.s16 r0, q1",
    "vaddv.u16 r0, q1",
    "vaddva.s16 r0, q1",
    "vaddva.u16 r0, q1",
    "vminv.s16 r0, q1",
    "vminv.u16 r0, q1",
    "vmaxv.s16 r0, q1",
    "vmaxv.u16 r0, q1",
    "vminav.s16 r0, q1",
    "vmaxav.s16 r0, q1",
    "vabav.s16 r0, q1, q2",
    "vabav.u16 r0, q1, q2",
    "vmladav.s16 r0, q1, q2",
    "vmlsdavax.s8 r0, q1, q2",
    "vmlaldav.s32 r0, r1, q2, q3",
    "vmlsldav.s32 r0, r1, q2, q3",
    "vrmlaldavh.s32 r0, r1, q2, q3",
    "vrmlsldavh.s32 r0, r1, q2, q3",
    "vmaxnmv.f32 r0, q1",
    "vminnmv.f32 r0, q1",
    "vmaxnmav.f32 r0, q1",
    "vminnmav.f32 r0, q1",
    "vmov q0[0], q0[0], r1, r2",
    "vmov r1, r2, q0[1], q0[0]",
    "vrintn.f32 q0, q1",
    "vrinta.f32 q0, q1",
    "vrintz.f32 q0, q1",
    "vrintm.f32 q0, q1",
    "vrintp.f32 q0, q1",
    "vrintx.f32 q0, q1",
    "vcvt.s32.f32 q0, q1",
    "vcvt.f16.u16 q0, q1",
    "vcvt.s32.f32 q0, q1, #4",
    "vcvt.f16.u16 q0, q1, #2",
    "vcvtb.f32.f16 q0, q1",
    "vcvtt.f16.f32 q0, q1",
    "vcvta.s32.f32 q0, q1",
    "vcvtn.s32.f32 q0, q1",
    "vcvtp.s32.f32 q0, q1",
    "vcvtm.s32.f32 q0, q1",
    "vcvtn.u16.f16 q0, q1",
    "vshrnb.i32 q0, q1, #3",
    "vrshrnb.i32 q0, q1, #3",
    "vqshrnb.s32 q0, q1, #3",
    "vqrshrnb.s32 q0, q1, #3",
    "vqshrunb.s32 q0, q1, #3",
    "vqrshrunb.s32 q0, q1, #3",
    "vqshrnt.u32 q0, q1, #5",
    "vmovlb.s8 q0, q1",
    "vmovlt.u16 q0, q1",
    "vmovnb.i16 q0, q1",
    "vmovnt.i32 q0, q1",
    "vqmovnb.s16 q0, q1",
    "vqmovnt.u32 q0, q1",
    "vqmovunb.s16 q0, q1",
    "vmullb.s8 q0, q1, q2",
    "vmullt.u16 q0, q1, q2",
    "vmullb.p8 q0, q1, q2",
    "vmulh.s16 q0, q1, q2",
    "vrmulh.u8 q0, q1, q2",
    "vqdmullb.s16 q0, q1, q2",
    "vqdmullt.s32 q0, q1, q2",
    "vqdmullb.s32 q0, q1, r2",
    "vqdmladh.s16 q0, q1, q2",
    "vqrdmlsdhx.s8 q0, q1, q2",
    "vshl.s16 q0, q1, q2",
    "vqrshl.u8 q0, q1, q2",
    "vshl.s16 q0, r1",
    "vqrshl.u8 q0, r1",
    "vshllb.s8 q0, q1, #3",
    "vshllt.u16 q0, q1, #8",
    "vaddlv.s32 r0, r1, q2",
    "vaddlva.u32 r0, r1, q2",
    "vcadd.i16 q0, q1, q2, #90",
    "vhcadd.s16 q0, q1, q2, #270",
    "vcadd.f32 q0, q1, q2, #90",
    "vcmul.f32 q0, q1, q2, #0",
    "vcmul.f32 q0, q1, q2, #90",
    "vcmul.f32 q0, q1, q2, #180",
    "vcmul.f32 q0, q1, q2, #270",
    "vcmla.f16 q0, q1, q2, #0",
    "vcmla.f16 q0, q1, q2, #90",
    "vcmla.f16 q0, q1, q2, #180",
    "vcmla.f16 q0, q1, q2, #270",
    "vpsel q0, q1, q2",
    "vpnot",
    "vadc.i32 q0, q1, q2",
    "vsbci.i32 q0, q1, q2",
    "vshlc q0, r1, #5",
    "vidup.u16 q0, r0, #2",
    "vdwdup.u8 q0, r0, r3, #4",
    "vbrsr.16 q0, q1, r2",
    "vcmp.i16 eq, q0, q1",
    "vcmp.i16 ne, q0, q1",
    "vcmp.u16 cs, q0, q1",
    "vcmp.u16 hi, q0, q1",
    "vcmp.s16 ge, q0, q1",
    "vcmp.s16 lt, q0, q1",
    "vcmp.s16 gt, q0, q1",
    "vcmp.s16 le, q0, q1",
    "vcmp.i32 eq, q0, r1",
    "vcmp.f32 ge, q0, q1",
    "vcmp.f16 gt, q0, r1",
    "vpst",
    "vpste",
    "vpstet",
    "vpstete",
    "vpt.i16 eq, q0, q1",
    "vpte.i8 ne, q0, r1",
    "vpt.f32 ge, q0, q1",
    "vptet.f16 lt, q0, r1",
    "wls lr, r0, #256",
    "dls lr, r0",
    "wlstp.8 lr, r0, #256",
    "dlstp.16 lr, r0",
    "le lr, #64",
    "letp lr, #64",
    "lctp",
];

#[test]
fn emit__t32_mve_forms_gnu() {
    let cases = t32_mve_cases();
    assert_eq!(
        cases.len(),
        EXPECTED_T32_MVE_GNU.len(),
        "case/expected table length mismatch"
    );
    for (instruction, expected) in cases.iter().zip(EXPECTED_T32_MVE_GNU) {
        assert_eq!(
            &instruction.to_assembly_string(GNU),
            expected,
            "T32 MVE emit mismatch for {instruction:?}"
        );
    }
}

/// Non-MVE T32 forms whose emit arms the 16-bit spot-checks in `emit_tests.rs` don't reach: the 32-bit
/// wide integer / load-store / DSP families, the full scalar VFP surface, and the ARMv8.1-M scalar and
/// security additions. Order matches `EXPECTED_T32_CORE_GNU`.
fn t32_core_cases() -> Vec<ArmT32Instruction> {
    use ArmT32Instruction::*;
    // ---- 16-bit T1 data-processing / shifts not spot-checked elsewhere ----
    let mut v = vec![Adc_Register_T1(L::R0, L::R1)];
    v.push(And_Register_T1(L::R2, L::R3));
    v.push(Asr_Immediate_T1(L::R0, L::R1, 5));
    v.push(Asr_Register_T1(L::R2, L::R3));
    v.push(Bic_Register_T1(L::R4, L::R5));
    v.push(Eor_Register_T1(L::R6, L::R7));
    v.push(Lsl_Register_T1(L::R0, L::R1));
    v.push(Lsr_Immediate_T1(L::R2, L::R3, 4));
    v.push(Lsr_Register_T1(L::R4, L::R5));
    v.push(Mov_Register_T1(R::R8, R::R9));
    v.push(Mov_Register_T2(L::R0, L::R1));
    v.push(Mul_T1(L::R0, L::R1));
    v.push(Mvn_Register_T1(L::R2, L::R3));
    v.push(Orr_Register_T1(L::R4, L::R5));
    v.push(Ror_Register_T1(L::R6, L::R7));
    v.push(Rsb_Immediate_T1(L::R0, L::R1));
    v.push(Sbc_Register_T1(L::R2, L::R3));
    v.push(Sub_Immediate_T1(L::R0, L::R1, 3));
    v.push(Sub_Immediate_T2(L::R4, 200));
    v.push(Sub_Register_T1(L::R0, L::R1, L::R2));
    v.push(Cmn_Register_T1(L::R0, L::R1));
    v.push(Tst_Register_T1(L::R2, L::R3));
    v.push(Add_Register_T1(L::R0, L::R1, L::R2));
    v.push(Add_SpPlusRegister_T2(R::R9));
    v.push(Adr_T1(L::R0, 16));
    v.push(Cbz_T1(L::R0, 8));
    v.push(Cbnz_T1(L::R1, 20));
    v.push(It_T1(Cc::Equal, 0b1000));
    v.push(Blx_Register_T1(R::R3));
    v.push(Bx_T1(R::R14));
    v.push(Rev_T1(L::R0, L::R1));
    v.push(Rev16_T1(L::R2, L::R3));
    v.push(Revsh_T1(L::R4, L::R5));
    v.push(Sxtb_T1(L::R0, L::R1));
    v.push(Sxth_T1(L::R2, L::R3));
    v.push(Uxtb_T1(L::R4, L::R5));
    v.push(Uxth_T1(L::R6, L::R7));
    v.push(Pop_T1(vec![R::R4, R::R5, R::R15]));
    v.push(Stm_T1(L::R0, vec![L::R1, L::R2]));
    v.push(Ldr_Register_T1(L::R0, L::R1, L::R2));
    v.push(Ldrb_Immediate_T1(L::R0, L::R1, 3));
    v.push(Ldrb_Register_T1(L::R2, L::R3, L::R4));
    v.push(Ldrh_Immediate_T1(L::R0, L::R1, 4));
    v.push(Ldrh_Register_T1(L::R2, L::R3, L::R4));
    v.push(Ldrsb_Register_T1(L::R0, L::R1, L::R2));
    v.push(Ldrsh_Register_T1(L::R3, L::R4, L::R5));
    v.push(Strb_Immediate_T1(L::R0, L::R1, 2));
    v.push(Strb_Register_T1(L::R2, L::R3, L::R4));
    v.push(Strh_Immediate_T1(L::R0, L::R1, 6));
    v.push(Strh_Register_T1(L::R2, L::R3, L::R4));
    v.push(Str_Immediate_T1(L::R0, L::R1, 8));
    v.push(Ldr_Immediate_T2(L::R0, 40));
    v.push(Str_Immediate_T2(L::R1, 60));
    v.push(Ldr_Literal_T1(L::R0, 40));
    v.push(Dmb_T1(crate::enums::ArmT32MemoryBarrierOption::System));
    v.push(Dsb_T1(crate::enums::ArmT32MemoryBarrierOption::System));
    v.push(Isb_T1(crate::enums::ArmT32MemoryBarrierOption::System));
    v.push(Bkpt_T1(0xAB));
    v.push(Svc_T1(0x10));
    v.push(Udf_T1(0xFE));
    v.push(Udf_T2(0xDEAD));
    v.push(Sev_T1);
    v.push(Wfe_T1);
    v.push(Wfi_T1);
    v.push(Yield_T1);

    // ---- 32-bit wide integer: modified-immediate data processing ----
    v.push(Mov_Immediate_T2(R::R0, 0xFF, false));
    v.push(Mvn_Immediate_T1(R::R1, 0xFF00, true));
    v.push(And_Immediate_T1(R::R0, R::R1, 0xFF, true));
    v.push(Bic_Immediate_T1(R::R2, R::R3, 0xF000_00F0, false));
    v.push(Orr_Immediate_T1(R::R4, R::R5, 0xAB, false));
    v.push(Eor_Immediate_T1(R::R6, R::R7, 0xFF00, false));
    v.push(Add_Immediate_T3(R::R8, R::R9, 0x1FE, true));
    v.push(Sub_Immediate_T3(R::R10, R::R11, 0x100, false));
    v.push(Adc_Immediate_T1(R::R0, R::R1, 1, false));
    v.push(Sbc_Immediate_T1(R::R2, R::R3, 2, true));
    v.push(Rsb_Immediate_T2(R::R4, R::R5, 0xFF, false));
    v.push(Orn_Immediate_T1(R::R6, R::R7, 0xFF, false));
    v.push(Tst_Immediate_T1(R::R0, 0x80000000));
    v.push(Teq_Immediate_T1(R::R1, 0xFF));
    v.push(Cmn_Immediate_T1(R::R2, 1));
    v.push(Cmp_Immediate_T2(R::R3, 0x100));

    // ---- 32-bit wide integer: shifted-register data processing ----
    v.push(Add_Register_T3(R::R0, R::R1, R::R2, Sh::Lsl(4), true));
    v.push(Sub_Register_T2(R::R3, R::R4, R::R5, Sh::Asr(1), false));
    v.push(And_Register_T2(R::R6, R::R7, R::R8, Sh::Lsr(2), false));
    v.push(Orr_Register_T2(R::R9, R::R10, R::R11, Sh::Ror(3), false));
    v.push(Eor_Register_T2(R::R0, R::R1, R::R2, Sh::Rrx, true));
    v.push(Bic_Register_T2(R::R3, R::R4, R::R5, Sh::Lsl(0), false));
    v.push(Adc_Register_T2(R::R6, R::R7, R::R8, Sh::Lsl(1), false));
    v.push(Sbc_Register_T2(R::R9, R::R10, R::R11, Sh::Asr(5), true));
    v.push(Rsb_Register_T1(R::R0, R::R1, R::R2, Sh::Lsl(7), false));
    v.push(Orn_Register_T1(R::R3, R::R4, R::R5, Sh::Lsl(0), false));
    v.push(Mov_Register_T3(R::R0, R::R1, Sh::Lsl(0), false));
    v.push(Mvn_Register_T2(R::R2, R::R3, Sh::Ror(1), true));
    v.push(Tst_Register_T2(R::R0, R::R1, Sh::Lsl(2)));
    v.push(Teq_Register_T1(R::R2, R::R3, Sh::Rrx));
    v.push(Cmn_Register_T2(R::R4, R::R5, Sh::Asr(4)));
    v.push(Cmp_Register_T3(R::R6, R::R7, Sh::Lsl(1)));

    // ---- 32-bit multiply / divide / bitfield ----
    v.push(Mls_T1(R::R0, R::R1, R::R2, R::R3));
    v.push(Udiv_T1(R::R0, R::R1, R::R2));
    v.push(Rbit_T1(R::R0, R::R1));
    v.push(Ubfx_T1(R::R0, R::R1, 4, 8));
    v.push(Sbfx_T1(R::R2, R::R3, 0, 16));
    v.push(Bfi_T1(R::R4, R::R5, 8, 8));
    v.push(Bfc_T1(R::R6, 0, 32));
    v.push(Smull_T1(R::R0, R::R1, R::R2, R::R3));
    v.push(Umull_T1(R::R4, R::R5, R::R6, R::R7));
    v.push(Smlal_T1(R::R0, R::R1, R::R2, R::R3));
    v.push(Umlal_T1(R::R4, R::R5, R::R6, R::R7));
    v.push(Umaal_T1(R::R8, R::R9, R::R10, R::R11));

    // ---- 32-bit DSP: saturating / extend / pack / parallel / signed-multiply ----
    v.push(Qadd_T1(R::R0, R::R1, R::R2));
    v.push(Qsub_T1(R::R3, R::R4, R::R5));
    v.push(Qdadd_T1(R::R6, R::R7, R::R8));
    v.push(Qdsub_T1(R::R9, R::R10, R::R11));
    v.push(Sxtab_T1(R::R0, R::R1, R::R2, 0));
    v.push(Uxtab_T1(R::R3, R::R4, R::R5, 8));
    v.push(Sxtah_T1(R::R6, R::R7, R::R8, 16));
    v.push(Uxtah_T1(R::R9, R::R10, R::R11, 24));
    v.push(Sxtab16_T1(R::R0, R::R1, R::R2, 0));
    v.push(Uxtab16_T1(R::R3, R::R4, R::R5, 8));
    v.push(Sxtb16_T1(R::R0, R::R1, 0));
    v.push(Uxtb16_T1(R::R2, R::R3, 16));
    v.push(Sxtb_T2(R::R0, R::R1, 0));
    v.push(Uxtb_T2(R::R2, R::R3, 8));
    v.push(Sxth_T2(R::R4, R::R5, 16));
    v.push(Uxth_T2(R::R6, R::R7, 24));
    v.push(Rev_T2(R::R0, R::R1));
    v.push(Rev16_T2(R::R2, R::R3));
    v.push(Revsh_T2(R::R4, R::R5));
    v.push(Ssat_T1(R::R0, 16, R::R1, Sh::Lsl(0)));
    v.push(Usat_T1(R::R2, 15, R::R3, Sh::Asr(4)));
    v.push(Ssat16_T1(R::R0, 5, R::R1));
    v.push(Usat16_T1(R::R2, 7, R::R3));
    v.push(Pkhbt_T1(R::R0, R::R1, R::R2, 4));
    v.push(Pkhtb_T1(R::R3, R::R4, R::R5, 8));
    v.push(Sel_T1(R::R0, R::R1, R::R2));
    v.push(Usad8_T1(R::R3, R::R4, R::R5));
    v.push(Usada8_T1(R::R6, R::R7, R::R8, R::R9));
    v.push(ParallelAddSub_T1(
        crate::enums::ArmT32ParallelOperation::Add16,
        crate::enums::ArmT32ParallelPrefix::Signed,
        R::R0,
        R::R1,
        R::R2,
    ));
    v.push(ParallelAddSub_T1(
        crate::enums::ArmT32ParallelOperation::Sub8,
        crate::enums::ArmT32ParallelPrefix::UnsignedSaturating,
        R::R3,
        R::R4,
        R::R5,
    ));
    v.push(Smul_T1(R::R0, R::R1, R::R2, false, false));
    v.push(Smul_T1(R::R0, R::R1, R::R2, true, true));
    v.push(Smulw_T1(R::R3, R::R4, R::R5, false));
    v.push(Smla_T1(R::R0, R::R1, R::R2, R::R3, false, true));
    v.push(Smlaw_T1(R::R4, R::R5, R::R6, R::R7, true));
    v.push(Smlal_Halfword_T1(R::R0, R::R1, R::R2, R::R3, false, false));
    v.push(Smuad_T1(R::R0, R::R1, R::R2, true));
    v.push(Smusd_T1(R::R3, R::R4, R::R5, false));
    v.push(Smlad_T1(R::R0, R::R1, R::R2, R::R3, false));
    v.push(Smlsd_T1(R::R4, R::R5, R::R6, R::R7, true));
    v.push(Smlald_T1(R::R0, R::R1, R::R2, R::R3, false));
    v.push(Smlsld_T1(R::R4, R::R5, R::R6, R::R7, true));
    v.push(Smmul_T1(R::R0, R::R1, R::R2, true));
    v.push(Smmla_T1(R::R0, R::R1, R::R2, R::R3, false));
    v.push(Smmls_T1(R::R4, R::R5, R::R6, R::R7, true));

    // ---- 32-bit load/store: immediate T3/T4, register T2, literal, dual, exclusive, multiple ----
    v.push(Ldr_Immediate_T3(R::R0, R::R1, 40));
    v.push(Str_Immediate_T3(R::R2, R::R3, 60));
    v.push(Ldrb_Immediate_T2(R::R0, R::R1, 100));
    v.push(Strb_Immediate_T2(R::R2, R::R3, 200));
    v.push(Ldrh_Immediate_T2(R::R0, R::R1, 40));
    v.push(Strh_Immediate_T2(R::R2, R::R3, 60));
    v.push(Ldrsb_Immediate_T1(R::R0, R::R1, 8));
    v.push(Ldrsh_Immediate_T1(R::R2, R::R3, 16));
    v.push(Ldr_Immediate_T4(R::R0, R::R1, 4, Idx::PostIndex));
    v.push(Str_Immediate_T4(R::R2, R::R3, 8, Idx::PreIndex));
    v.push(Ldrb_Immediate_T3(R::R0, R::R1, 4, Idx::PostIndex));
    v.push(Strb_Immediate_T3(R::R2, R::R3, 8, Idx::PreIndex));
    v.push(Ldrh_Immediate_T3(R::R0, R::R1, 4, Idx::PostIndex));
    v.push(Strh_Immediate_T3(R::R2, R::R3, 8, Idx::PreIndex));
    v.push(Ldrsb_Immediate_T2(R::R0, R::R1, 4, Idx::PostIndex));
    v.push(Ldrsh_Immediate_T2(R::R2, R::R3, 8, Idx::PreIndex));
    v.push(Ldr_Register_T2(R::R0, R::R1, R::R2, 2));
    v.push(Str_Register_T2(R::R3, R::R4, R::R5, 0));
    v.push(Ldrb_Register_T2(R::R0, R::R1, R::R2, 1));
    v.push(Strb_Register_T2(R::R3, R::R4, R::R5, 3));
    v.push(Ldrh_Register_T2(R::R0, R::R1, R::R2, 0));
    v.push(Strh_Register_T2(R::R3, R::R4, R::R5, 2));
    v.push(Ldrsb_Register_T2(R::R0, R::R1, R::R2, 0));
    v.push(Ldrsh_Register_T2(R::R3, R::R4, R::R5, 1));
    v.push(Ldr_Literal_T2(R::R0, 100));
    v.push(Ldrb_Literal_T1(R::R1, -8));
    v.push(Ldrh_Literal_T1(R::R2, 16));
    v.push(Ldrsb_Literal_T1(R::R3, 4));
    v.push(Ldrsh_Literal_T1(R::R4, -20));
    v.push(Pld_Immediate_T1(R::R0, 8));
    v.push(Pli_Immediate_T1(R::R1, 16));
    v.push(Ldrd_Immediate_T1(R::R0, R::R1, R::R2, 8, Idx::Offset));
    v.push(Strd_Immediate_T1(R::R3, R::R4, R::R5, 16, Idx::PreIndex));
    v.push(Ldmia_T2(R::R0, false, vec![R::R1, R::R2, R::R3]));
    v.push(Stmia_T2(R::R4, true, vec![R::R5, R::R6]));
    v.push(Ldmdb_T1(R::R0, true, vec![R::R1, R::R14]));
    v.push(Stmdb_T1(R::R13, true, vec![R::R4, R::R5, R::R14]));
    v.push(Ldrex_T1(R::R0, R::R1, 4));
    v.push(Strex_T1(R::R0, R::R1, R::R2, 8));
    v.push(Ldrexb_T1(R::R0, R::R1));
    v.push(Strexb_T1(R::R0, R::R1, R::R2));
    v.push(Ldrexh_T1(R::R0, R::R1));
    v.push(Strexh_T1(R::R0, R::R1, R::R2));
    v.push(Clrex_T1);
    v.push(Tbb_T1(R::R0, R::R1));
    v.push(Tbh_T1(R::R2, R::R3));

    // ---- scalar VFP (T32) ----
    v.push(Vldr_Single_T2(s(0), R::R1, 4));
    v.push(Vstr_Single_T2(s(31), R::R2, -8));
    v.push(Vldr_Double_T1(d(0), R::R0, 0));
    v.push(Vstr_Double_T1(d(15), R::R4, -256));
    v.push(Vldm_Single_T2(R::R0, false, false, s(0), 4));
    v.push(Vstm_Single_T2(R::R13, true, true, s(0), 4));
    v.push(Vldm_Double_T1(R::R0, false, false, d(0), 2));
    v.push(Vstm_Double_T1(R::R3, true, true, d(5), 3));
    for op in [
        F3::Vmla, F3::Vmls, F3::Vnmla, F3::Vnmls, F3::Vmul, F3::Vnmul, F3::Vadd, F3::Vsub, F3::Vdiv,
        F3::Vfnma, F3::Vfnms, F3::Vfma, F3::Vfms,
    ] {
        v.push(FpDataProcess3_Single(op, s(0), s(1), s(2)));
        v.push(FpDataProcess3_Double(op, d(3), d(4), d(5)));
    }
    for op in [F2::Vmov, F2::Vabs, F2::Vneg, F2::Vsqrt] {
        v.push(FpDataProcess2_Single(op, s(6), s(7)));
        v.push(FpDataProcess2_Double(op, d(8), d(9)));
    }
    v.push(Vcmp_Single_T1(s(0), s(1), false));
    v.push(Vcmp_Single_T1(s(2), s(3), true));
    v.push(Vcmp_Double_T1(d(0), d(1), false));
    v.push(Vcmp_Zero_Single_T2(s(4), true));
    v.push(Vcmp_Zero_Double_T2(d(2), false));
    v.push(Vmrs_T1(R::R0));
    v.push(Vmrs_Apsr_Nzcv_T1);
    v.push(Vmsr_T1(R::R1));
    v.push(Vmov_Core_To_Single_T1(s(0), R::R1));
    v.push(Vmov_Single_To_Core_T1(R::R2, s(3)));
    v.push(Vmov_Immediate_Single_T1(s(0), 0x70));
    v.push(Vmov_Immediate_Double_T1(d(0), 0x70));
    v.push(Vmov_Double_To_CorePair_T1(R::R0, R::R1, d(2)));
    v.push(Vmov_CorePair_To_Double_T1(d(3), R::R4, R::R5));
    v.push(Vmov_Singles_To_CorePair_T1(R::R6, R::R7, s(8)));
    v.push(Vmov_CorePair_To_Singles_T1(s(10), R::R2, R::R3));
    v.push(Vcvt_FloatToInt_FromSingle_T1(s(0), s(1), true, true));
    v.push(Vcvt_FloatToInt_FromDouble_T1(s(2), d(3), false, false));
    v.push(Vcvt_IntToFloat_ToSingle_T1(s(4), s(5), true));
    v.push(Vcvt_IntToFloat_ToDouble_T1(d(0), s(1), false));
    v.push(Vcvt_Single_To_Double_T1(d(0), s(1)));
    v.push(Vcvt_Double_To_Single_T1(s(0), d(1)));
    v.push(Vcvt_HalfToSingle_T1(s(0), s(1), false));
    v.push(Vcvt_SingleToHalf_T1(s(4), s(5), true));
    v.push(Vcvt_FloatToFixed_Single_T1(s(0), true, false, 1));
    v.push(Vcvt_FloatToFixed_Double_T1(d(3), false, true, 4));
    v.push(Vcvt_FixedToFloat_Single_T1(s(3), true, false, 3));
    v.push(Vcvt_FixedToFloat_Double_T1(d(2), false, true, 8));

    // ---- ARMv8-M scalar FP: VSEL / VMAXNM / VMINNM / VRINT / directed VCVT / VJCVT / VMOVX ----
    v.push(Vsel_Single_T1(0, s(0), s(1), s(2)));
    v.push(Vsel_Double_T1(2, d(0), d(1), d(2)));
    v.push(Vmaxnm_Single_T1(s(0), s(1), s(2)));
    v.push(Vmaxnm_Double_T1(d(0), d(1), d(2)));
    v.push(Vminnm_Single_T1(s(3), s(4), s(5)));
    v.push(Vminnm_Double_T1(d(3), d(4), d(5)));
    v.push(Vrintr_Single_T1(s(0), s(1)));
    v.push(Vrintr_Double_T1(d(0), d(1)));
    v.push(Vrintz_Single_T1(s(2), s(3)));
    v.push(Vrintz_Double_T1(d(2), d(3)));
    v.push(Vrintx_Single_T1(s(4), s(5)));
    v.push(Vrintx_Double_T1(d(4), d(5)));
    v.push(Vrint_Directed_Single_T1(DRnd::A, s(0), s(1)));
    v.push(Vrint_Directed_Double_T1(DRnd::M, d(2), d(3)));
    v.push(Vcvt_Directed_FromSingle_T1(DRnd::P, s(0), s(1), true));
    v.push(Vcvt_Directed_FromDouble_T1(DRnd::N, s(2), d(3), false));
    v.push(Vjcvt_T1(s(0), d(1)));
    v.push(Vmovx_T1(false, s(0), s(1)));
    v.push(Vmovx_T1(true, s(2), s(3)));

    // ---- ARMv8.1-M scalar (CSEL family, long/saturating shifts) ----
    for op in 0..4u8 {
        v.push(Csel_T1(op, R::R0, R::R1, R::R2, Cc::Equal));
    }
    for op in 0..3u8 {
        v.push(LongShiftImm_T1(op, R::R0, R::R1, 5));
        v.push(LongShiftReg_T1(op, R::R2, R::R3, R::R4));
    }
    for op in 0..4u8 {
        v.push(SatShiftImm_T1(op, R::R0, 3));
    }
    v.push(SatShiftLongImm_T1(0, R::R0, R::R1, 5));
    v.push(SatShiftReg_T1(true, R::R0, R::R1));
    v.push(SatShiftReg_T1(false, R::R2, R::R3));
    v.push(SatShiftLongReg_T1(true, R::R0, R::R1, R::R2, false));
    v.push(SatShiftLongReg_T1(false, R::R4, R::R5, R::R6, true));

    // ---- system / hint / security / v8-M additions ----
    v.push(Mrs_T1(R::R0, Spec::Primask));
    v.push(Msr_Register_T1(Spec::Control, R::R1));
    v.push(Dbg_T1(5));
    v.push(Esb_T1);
    v.push(Ssbb_T1);
    v.push(Pssbb_T1);
    v.push(Sb_T1);
    v.push(Csdb_T1);
    v.push(Sg_T1);
    v.push(Bxns_T1(R::R3));
    v.push(Blxns_T1(R::R4));
    v.push(Tt_T1(R::R0, R::R1, false, false));
    v.push(Tt_T1(R::R2, R::R3, true, true));
    v.push(Vlstm_T1(R::R0));
    v.push(Vlldm_T1(R::R1));
    v.push(Clrm_T1(0b0000_0000_0011_0110));

    v
}


/// GNU-flavor UAL text for each entry of `t32_core_cases()`, in the same order. Captured from the emitter
/// and cross-checked against `arm-none-eabi-as`/`objdump` (the differential oracle).
const EXPECTED_T32_CORE_GNU: &[&str] = &[
    "adcs r0, r1",
    "ands r2, r3",
    "asrs r0, r1, #5",
    "asrs r2, r3",
    "bics r4, r5",
    "eors r6, r7",
    "lsls r0, r1",
    "lsrs r2, r3, #4",
    "lsrs r4, r5",
    "mov r8, r9",
    "movs r0, r1",
    "muls r0, r1, r0",
    "mvns r2, r3",
    "orrs r4, r5",
    "rors r6, r7",
    "rsbs r0, r1, #0",
    "sbcs r2, r3",
    "subs r0, r1, #3",
    "subs r4, #200",
    "subs r0, r1, r2",
    "cmn r0, r1",
    "tst r2, r3",
    "adds r0, r1, r2",
    "add sp, r9",
    "adr r0, #16",
    "cbz r0, #8",
    "cbnz r1, #20",
    "it eq",
    "blx r3",
    "bx lr",
    "rev r0, r1",
    "rev16 r2, r3",
    "revsh r4, r5",
    "sxtb r0, r1",
    "sxth r2, r3",
    "uxtb r4, r5",
    "uxth r6, r7",
    "pop {r4, r5, pc}",
    "stmia r0!, {r1, r2}",
    "ldr r0, [r1, r2]",
    "ldrb r0, [r1, #3]",
    "ldrb r2, [r3, r4]",
    "ldrh r0, [r1, #4]",
    "ldrh r2, [r3, r4]",
    "ldrsb r0, [r1, r2]",
    "ldrsh r3, [r4, r5]",
    "strb r0, [r1, #2]",
    "strb r2, [r3, r4]",
    "strh r0, [r1, #6]",
    "strh r2, [r3, r4]",
    "str r0, [r1, #8]",
    "ldr r0, [sp, #40]",
    "str r1, [sp, #60]",
    "ldr r0, [pc, #40]",
    "dmb sy",
    "dsb sy",
    "isb sy",
    "bkpt #171",
    "svc #16",
    "udf #254",
    "udf.w #57005",
    "sev",
    "wfe",
    "wfi",
    "yield",
    "mov.w r0, #255",
    "mvns.w r1, #65280",
    "ands.w r0, r1, #255",
    "bic.w r2, r3, #4026532080",
    "orr.w r4, r5, #171",
    "eor.w r6, r7, #65280",
    "adds.w r8, r9, #510",
    "sub.w r10, r11, #256",
    "adc r0, r1, #1",
    "sbcs r2, r3, #2",
    "rsb r4, r5, #255",
    "orn r6, r7, #255",
    "tst.w r0, #2147483648",
    "teq.w r1, #255",
    "cmn.w r2, #1",
    "cmp.w r3, #256",
    "adds.w r0, r1, r2, lsl #4",
    "sub.w r3, r4, r5, asr #1",
    "and.w r6, r7, r8, lsr #2",
    "orr.w r9, r10, r11, ror #3",
    "eors.w r0, r1, r2, rrx",
    "bic.w r3, r4, r5",
    "adc.w r6, r7, r8, lsl #1",
    "sbcs.w r9, r10, r11, asr #5",
    "rsb.w r0, r1, r2, lsl #7",
    "orn.w r3, r4, r5",
    "mov.w r0, r1",
    "mvns.w r2, r3, ror #1",
    "tst.w r0, r1, lsl #2",
    "teq.w r2, r3, rrx",
    "cmn.w r4, r5, asr #4",
    "cmp.w r6, r7, lsl #1",
    "mls r0, r1, r2, r3",
    "udiv r0, r1, r2",
    "rbit r0, r1",
    "ubfx r0, r1, #4, #8",
    "sbfx r2, r3, #0, #16",
    "bfi r4, r5, #8, #8",
    "bfc r6, #0, #32",
    "smull r0, r1, r2, r3",
    "umull r4, r5, r6, r7",
    "smlal r0, r1, r2, r3",
    "umlal r4, r5, r6, r7",
    "umaal r8, r9, r10, r11",
    "qadd r0, r1, r2",
    "qsub r3, r4, r5",
    "qdadd r6, r7, r8",
    "qdsub r9, r10, r11",
    "sxtab r0, r1, r2",
    "uxtab r3, r4, r5, ror #8",
    "sxtah r6, r7, r8, ror #16",
    "uxtah r9, r10, r11, ror #24",
    "sxtab16 r0, r1, r2",
    "uxtab16 r3, r4, r5, ror #8",
    "sxtb16 r0, r1",
    "uxtb16 r2, r3, ror #16",
    "sxtb.w r0, r1",
    "uxtb.w r2, r3, ror #8",
    "sxth.w r4, r5, ror #16",
    "uxth.w r6, r7, ror #24",
    "rev.w r0, r1",
    "rev16.w r2, r3",
    "revsh.w r4, r5",
    "ssat r0, #16, r1",
    "usat r2, #15, r3, asr #4",
    "ssat16 r0, #5, r1",
    "usat16 r2, #7, r3",
    "pkhbt r0, r1, r2, lsl #4",
    "pkhtb r3, r4, r5, asr #8",
    "sel r0, r1, r2",
    "usad8 r3, r4, r5",
    "usada8 r6, r7, r8, r9",
    "sadd16 r0, r1, r2",
    "uqsub8 r3, r4, r5",
    "smulbb r0, r1, r2",
    "smultt r0, r1, r2",
    "smulwb r3, r4, r5",
    "smlabt r0, r1, r2, r3",
    "smlawt r4, r5, r6, r7",
    "smlalbb r0, r1, r2, r3",
    "smuadx r0, r1, r2",
    "smusd r3, r4, r5",
    "smlad r0, r1, r2, r3",
    "smlsdx r4, r5, r6, r7",
    "smlald r0, r1, r2, r3",
    "smlsldx r4, r5, r6, r7",
    "smmulr r0, r1, r2",
    "smmla r0, r1, r2, r3",
    "smmlsr r4, r5, r6, r7",
    "ldr.w r0, [r1, #40]",
    "str.w r2, [r3, #60]",
    "ldrb.w r0, [r1, #100]",
    "strb.w r2, [r3, #200]",
    "ldrh.w r0, [r1, #40]",
    "strh.w r2, [r3, #60]",
    "ldrsb.w r0, [r1, #8]",
    "ldrsh.w r2, [r3, #16]",
    "ldr r0, [r1], #4",
    "str r2, [r3, #8]!",
    "ldrb r0, [r1], #4",
    "strb r2, [r3, #8]!",
    "ldrh r0, [r1], #4",
    "strh r2, [r3, #8]!",
    "ldrsb r0, [r1], #4",
    "ldrsh r2, [r3, #8]!",
    "ldr.w r0, [r1, r2, lsl #2]",
    "str.w r3, [r4, r5]",
    "ldrb.w r0, [r1, r2, lsl #1]",
    "strb.w r3, [r4, r5, lsl #3]",
    "ldrh.w r0, [r1, r2]",
    "strh.w r3, [r4, r5, lsl #2]",
    "ldrsb.w r0, [r1, r2]",
    "ldrsh.w r3, [r4, r5, lsl #1]",
    "ldr.w r0, [pc, #100]",
    "ldrb.w r1, [pc, #-8]",
    "ldrh.w r2, [pc, #16]",
    "ldrsb.w r3, [pc, #4]",
    "ldrsh.w r4, [pc, #-20]",
    "pld [r0, #8]",
    "pli [r1, #16]",
    "ldrd r0, r1, [r2, #8]",
    "strd r3, r4, [r5, #16]!",
    "ldm.w r0, {r1, r2, r3}",
    "stm.w r4!, {r5, r6}",
    "ldmdb r0!, {r1, lr}",
    "push.w {r4, r5, lr}",
    "ldrex r0, [r1, #4]",
    "strex r0, r1, [r2, #8]",
    "ldrexb r0, [r1]",
    "strexb r0, r1, [r2]",
    "ldrexh r0, [r1]",
    "strexh r0, r1, [r2]",
    "clrex",
    "tbb [r0, r1]",
    "tbh [r2, r3, lsl #1]",
    "vldr s0, [r1, #4]",
    "vstr s31, [r2, #-8]",
    "vldr d0, [r0]",
    "vstr d15, [r4, #-256]",
    "vldmia r0, {s0-s3}",
    "vpush {s0-s3}",
    "vldmia r0, {d0-d1}",
    "vstmdb r3!, {d5-d7}",
    "vmla.f32 s0, s1, s2",
    "vmla.f64 d3, d4, d5",
    "vmls.f32 s0, s1, s2",
    "vmls.f64 d3, d4, d5",
    "vnmla.f32 s0, s1, s2",
    "vnmla.f64 d3, d4, d5",
    "vnmls.f32 s0, s1, s2",
    "vnmls.f64 d3, d4, d5",
    "vmul.f32 s0, s1, s2",
    "vmul.f64 d3, d4, d5",
    "vnmul.f32 s0, s1, s2",
    "vnmul.f64 d3, d4, d5",
    "vadd.f32 s0, s1, s2",
    "vadd.f64 d3, d4, d5",
    "vsub.f32 s0, s1, s2",
    "vsub.f64 d3, d4, d5",
    "vdiv.f32 s0, s1, s2",
    "vdiv.f64 d3, d4, d5",
    "vfnma.f32 s0, s1, s2",
    "vfnma.f64 d3, d4, d5",
    "vfnms.f32 s0, s1, s2",
    "vfnms.f64 d3, d4, d5",
    "vfma.f32 s0, s1, s2",
    "vfma.f64 d3, d4, d5",
    "vfms.f32 s0, s1, s2",
    "vfms.f64 d3, d4, d5",
    "vmov.f32 s6, s7",
    "vmov.f64 d8, d9",
    "vabs.f32 s6, s7",
    "vabs.f64 d8, d9",
    "vneg.f32 s6, s7",
    "vneg.f64 d8, d9",
    "vsqrt.f32 s6, s7",
    "vsqrt.f64 d8, d9",
    "vcmp.f32 s0, s1",
    "vcmpe.f32 s2, s3",
    "vcmp.f64 d0, d1",
    "vcmpe.f32 s4, #0.0",
    "vcmp.f64 d2, #0.0",
    "vmrs r0, fpscr",
    "vmrs APSR_nzcv, fpscr",
    "vmsr fpscr, r1",
    "vmov s0, r1",
    "vmov r2, s3",
    "vmov.f32 s0, #1.0",
    "vmov.f64 d0, #1.0",
    "vmov r0, r1, d2",
    "vmov d3, r4, r5",
    "vmov r6, r7, s8, s9",
    "vmov s10, s11, r2, r3",
    "vcvt.s32.f32 s0, s1",
    "vcvtr.u32.f64 s2, d3",
    "vcvt.f32.s32 s4, s5",
    "vcvt.f64.u32 d0, s1",
    "vcvt.f64.f32 d0, s1",
    "vcvt.f32.f64 s0, d1",
    "vcvtb.f32.f16 s0, s1",
    "vcvtt.f16.f32 s4, s5",
    "vcvt.s16.f32 s0, s0, #1",
    "vcvt.u32.f64 d3, d3, #4",
    "vcvt.f32.s16 s3, s3, #3",
    "vcvt.f64.u32 d2, d2, #8",
    "vseleq.f32 s0, s1, s2",
    "vselge.f64 d0, d1, d2",
    "vmaxnm.f32 s0, s1, s2",
    "vmaxnm.f64 d0, d1, d2",
    "vminnm.f32 s3, s4, s5",
    "vminnm.f64 d3, d4, d5",
    "vrintr.f32 s0, s1",
    "vrintr.f64 d0, d1",
    "vrintz.f32 s2, s3",
    "vrintz.f64 d2, d3",
    "vrintx.f32 s4, s5",
    "vrintx.f64 d4, d5",
    "vrinta.f32 s0, s1",
    "vrintm.f64 d2, d3",
    "vcvtp.s32.f32 s0, s1",
    "vcvtn.u32.f64 s2, d3",
    "vjcvt.s32.f64 s0, d1",
    "vmovx.f16 s0, s1",
    "vins.f16 s2, s3",
    "csel r0, r1, r2, eq",
    "csinc r0, r1, r2, eq",
    "csinv r0, r1, r2, eq",
    "csneg r0, r1, r2, eq",
    "lsll r0, r1, #5",
    "lsll r2, r3, r4",
    "lsrl r0, r1, #5",
    "lsrl r2, r3, r4",
    "asrl r0, r1, #5",
    "asrl r2, r3, r4",
    "uqshl r0, #3",
    "urshr r0, #3",
    "srshr r0, #3",
    "sqshl r0, #3",
    "uqshll r0, r1, #5",
    "sqrshr r0, r1",
    "uqrshl r2, r3",
    "sqrshrl r0, r1, #64, r2",
    "uqrshll r4, r5, #48, r6",
    "mrs r0, PRIMASK",
    "msr CONTROL, r1",
    "dbg #5",
    "esb",
    "ssbb",
    "pssbb",
    "sb",
    "csdb",
    "sg",
    "bxns r3",
    "blxns r4",
    "tt r0, r1",
    "ttat r2, r3",
    "vlstm r0",
    "vlldm r1",
    "clrm {r1, r2, r4, r5}",
];

#[test]
fn emit__t32_core_forms_gnu() {
    let cases = t32_core_cases();
    assert_eq!(
        cases.len(),
        EXPECTED_T32_CORE_GNU.len(),
        "case/expected table length mismatch"
    );
    for (instruction, expected) in cases.iter().zip(EXPECTED_T32_CORE_GNU) {
        assert_eq!(
            &instruction.to_assembly_string(GNU),
            expected,
            "T32 core emit mismatch for {instruction:?}"
        );
    }
}
