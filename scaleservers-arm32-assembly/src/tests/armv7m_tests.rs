// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// ARMv7-M (Thumb-2) batch: exact encodings (cross-checked against clang), encode<->decode round-trips,
// the SP/PC operand guard, and -- the headline of this milestone -- the target gate: a v7-M instruction is
// REFUSED by an ARMv6-M target profile yet accepted by an ARMv7-M one. `encode()` itself is always
// target-independent; only `encode_for_target` gates.

use crate::ArmT32IndexMode as Mode;
use crate::ArmT32InstructionCondition as Cond;
use crate::ArmT32RegisterShift as Shift;
use crate::enums::Arm32GeneralPurposeRegister as R;
use crate::enums::Arm32LowGeneralPurposeRegister as L;
use crate::{
    ArmInstructionRequirement, ArmIsaVersion, ArmT32Instruction, ArmTargetProfile, EncodeError,
};

#[test]
fn encode__armv7m_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    assert_eq!(
        ArmT32Instruction::Mov_Immediate_T3(R::R0, 0x1234)
            .encode()
            .unwrap(),
        vec![0x41, 0xF2, 0x34, 0x20]
    ); // movw r0, #0x1234
    assert_eq!(
        ArmT32Instruction::Mul_T2(R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0x01, 0xFB, 0x02, 0xF0]
    ); // mul  r0, r1, r2
    assert_eq!(
        ArmT32Instruction::Sdiv_T1(R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0x91, 0xFB, 0xF2, 0xF0]
    ); // sdiv r0, r1, r2
    assert_eq!(
        ArmT32Instruction::Clz_T1(R::R3, R::R4).encode().unwrap(),
        vec![0xB4, 0xFA, 0x84, 0xF3]
    ); // clz  r3, r4
}

#[test]
fn encode__armv7m_batch2_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    assert_eq!(
        ArmT32Instruction::Ubfx_T1(R::R0, R::R1, 4, 8)
            .encode()
            .unwrap(),
        vec![0xC1, 0xF3, 0x07, 0x10]
    ); // ubfx  r0, r1, #4, #8
    assert_eq!(
        ArmT32Instruction::Sbfx_T1(R::R2, R::R3, 5, 7)
            .encode()
            .unwrap(),
        vec![0x43, 0xF3, 0x46, 0x12]
    ); // sbfx  r2, r3, #5, #7
    assert_eq!(
        ArmT32Instruction::Bfi_T1(R::R4, R::R5, 2, 6)
            .encode()
            .unwrap(),
        vec![0x65, 0xF3, 0x87, 0x04]
    ); // bfi   r4, r5, #2, #6
    assert_eq!(
        ArmT32Instruction::Bfc_T1(R::R6, 3, 9).encode().unwrap(),
        vec![0x6F, 0xF3, 0xCB, 0x06]
    ); // bfc   r6, #3, #9
    assert_eq!(
        ArmT32Instruction::Rbit_T1(R::R0, R::R1).encode().unwrap(),
        vec![0x91, 0xFA, 0xA1, 0xF0]
    ); // rbit  r0, r1
    assert_eq!(
        ArmT32Instruction::Ldr_Immediate_T3(R::R0, R::R1, 100)
            .encode()
            .unwrap(),
        vec![0xD1, 0xF8, 0x64, 0x00]
    ); // ldr.w r0, [r1, #100]
    assert_eq!(
        ArmT32Instruction::Str_Immediate_T3(R::R2, R::R3, 200)
            .encode()
            .unwrap(),
        vec![0xC3, 0xF8, 0xC8, 0x20]
    ); // str.w r2, [r3, #200]
}

#[test]
fn encode__armv7m_batch3_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    assert_eq!(
        ArmT32Instruction::Ldrex_T1(R::R0, R::R1, 16)
            .encode()
            .unwrap(),
        vec![0x51, 0xE8, 0x04, 0x0F]
    ); // ldrex  r0, [r1, #16]
    assert_eq!(
        ArmT32Instruction::Strex_T1(R::R2, R::R3, R::R4, 8)
            .encode()
            .unwrap(),
        vec![0x44, 0xE8, 0x02, 0x32]
    ); // strex  r2, r3, [r4, #8]
    assert_eq!(
        ArmT32Instruction::Ldrexb_T1(R::R0, R::R1).encode().unwrap(),
        vec![0xD1, 0xE8, 0x4F, 0x0F]
    ); // ldrexb r0, [r1]
    assert_eq!(
        ArmT32Instruction::Strexb_T1(R::R2, R::R3, R::R4)
            .encode()
            .unwrap(),
        vec![0xC4, 0xE8, 0x42, 0x3F]
    ); // strexb r2, r3, [r4]
    assert_eq!(
        ArmT32Instruction::Ldrexh_T1(R::R0, R::R1).encode().unwrap(),
        vec![0xD1, 0xE8, 0x5F, 0x0F]
    ); // ldrexh r0, [r1]
    assert_eq!(
        ArmT32Instruction::Strexh_T1(R::R2, R::R3, R::R4)
            .encode()
            .unwrap(),
        vec![0xC4, 0xE8, 0x52, 0x3F]
    ); // strexh r2, r3, [r4]
    assert_eq!(
        ArmT32Instruction::Clrex_T1.encode().unwrap(),
        vec![0xBF, 0xF3, 0x2F, 0x8F]
    ); // clrex
    assert_eq!(
        ArmT32Instruction::Tbb_T1(R::R0, R::R1).encode().unwrap(),
        vec![0xD0, 0xE8, 0x01, 0xF0]
    ); // tbb [r0, r1]
    assert_eq!(
        ArmT32Instruction::Tbh_T1(R::R0, R::R1).encode().unwrap(),
        vec![0xD0, 0xE8, 0x11, 0xF0]
    ); // tbh [r0, r1, lsl #1]
}

#[test]
fn encode__armv7m_batch4_modified_immediate_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`; exercises all four ThumbExpandImm forms
    assert_eq!(
        ArmT32Instruction::Mov_Immediate_T2(R::R0, 0x00AB00AB, false)
            .encode()
            .unwrap(),
        vec![0x4F, 0xF0, 0xAB, 0x10]
    ); // mov.w  r0, #0x00ab00ab  (0x00XY00XY)
    assert_eq!(
        ArmT32Instruction::Mov_Immediate_T2(R::R1, 0xFF00FF00, true)
            .encode()
            .unwrap(),
        vec![0x5F, 0xF0, 0xFF, 0x21]
    ); // movs.w r1, #0xff00ff00  (0xXY00XY00)
    assert_eq!(
        ArmT32Instruction::Mov_Immediate_T2(R::R2, 0xAB000000, false)
            .encode()
            .unwrap(),
        vec![0x4F, 0xF0, 0x2B, 0x42]
    ); // mov.w  r2, #0xab000000  (rotation)
    assert_eq!(
        ArmT32Instruction::And_Immediate_T1(R::R0, R::R1, 0xABABABAB, false)
            .encode()
            .unwrap(),
        vec![0x01, 0xF0, 0xAB, 0x30]
    ); // and  r0, r1, #0xabababab  (0xXYXYXYXY)
    assert_eq!(
        ArmT32Instruction::Add_Immediate_T3(R::R0, R::R1, 0x100000, false)
            .encode()
            .unwrap(),
        vec![0x01, 0xF5, 0x80, 0x10]
    ); // add.w r0, r1, #0x100000
    assert_eq!(
        ArmT32Instruction::Sub_Immediate_T3(R::R4, R::R5, 0xFF000000, false)
            .encode()
            .unwrap(),
        vec![0xA5, 0xF1, 0x7F, 0x44]
    ); // sub.w r4, r5, #0xff000000
    assert_eq!(
        ArmT32Instruction::Cmp_Immediate_T2(R::R0, 0x1000)
            .encode()
            .unwrap(),
        vec![0xB0, 0xF5, 0x80, 0x5F]
    ); // cmp.w  r0, #0x1000
    assert_eq!(
        ArmT32Instruction::Tst_Immediate_T1(R::R2, 0x80000000)
            .encode()
            .unwrap(),
        vec![0x12, 0xF0, 0x00, 0x4F]
    ); // tst.w  r2, #0x80000000
}

