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

#[test]
fn encode__armv7m_batch5_exact_bytes() {
    // ADC/SBC/RSB/ORN immediate -- bytes verified against clang & GNU as
    assert_eq!(
        ArmT32Instruction::Adc_Immediate_T1(R::R0, R::R1, 0xFF, false)
            .encode()
            .unwrap(),
        vec![0x41, 0xF1, 0xFF, 0x00]
    ); // adc  r0, r1, #0xff
    assert_eq!(
        ArmT32Instruction::Adc_Immediate_T1(R::R2, R::R3, 0x100, true)
            .encode()
            .unwrap(),
        vec![0x53, 0xF5, 0x80, 0x72]
    ); // adcs r2, r3, #0x100
    assert_eq!(
        ArmT32Instruction::Sbc_Immediate_T1(R::R4, R::R5, 0xFF000000, false)
            .encode()
            .unwrap(),
        vec![0x65, 0xF1, 0x7F, 0x44]
    ); // sbc r4, r5, #0xff000000
    assert_eq!(
        ArmT32Instruction::Rsb_Immediate_T2(R::R6, R::R7, 0xAB00AB00, false)
            .encode()
            .unwrap(),
        vec![0xC7, 0xF1, 0xAB, 0x26]
    ); // rsb r6, r7, #0xab00ab00
    assert_eq!(
        ArmT32Instruction::Orn_Immediate_T1(R::R2, R::R3, 0xFF00FF00, false)
            .encode()
            .unwrap(),
        vec![0x63, 0xF0, 0xFF, 0x22]
    ); // orn r2, r3, #0xff00ff00
    assert_eq!(
        ArmT32Instruction::Orn_Immediate_T1(R::R4, R::R5, 0xFF, true)
            .encode()
            .unwrap(),
        vec![0x75, 0xF0, 0xFF, 0x04]
    ); // orns r4, r5, #0xff
}

#[test]
fn encode__armv7m_batch6_shifted_register_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    assert_eq!(
        ArmT32Instruction::Add_Register_T3(R::R0, R::R1, R::R2, Shift::Lsl(0), false)
            .encode()
            .unwrap(),
        vec![0x01, 0xEB, 0x02, 0x00]
    ); // add.w  r0, r1, r2
    assert_eq!(
        ArmT32Instruction::Add_Register_T3(R::R0, R::R1, R::R2, Shift::Lsl(3), false)
            .encode()
            .unwrap(),
        vec![0x01, 0xEB, 0xC2, 0x00]
    ); // add.w  r0, r1, r2, lsl #3
    assert_eq!(
        ArmT32Instruction::Sub_Register_T2(R::R3, R::R4, R::R5, Shift::Lsr(2), false)
            .encode()
            .unwrap(),
        vec![0xA4, 0xEB, 0x95, 0x03]
    ); // sub.w  r3, r4, r5, lsr #2
    assert_eq!(
        ArmT32Instruction::And_Register_T2(R::R6, R::R7, R::R8, Shift::Asr(1), false)
            .encode()
            .unwrap(),
        vec![0x07, 0xEA, 0x68, 0x06]
    ); // and.w  r6, r7, r8, asr #1
    assert_eq!(
        ArmT32Instruction::Orr_Register_T2(R::R0, R::R1, R::R2, Shift::Ror(4), false)
            .encode()
            .unwrap(),
        vec![0x41, 0xEA, 0x32, 0x10]
    ); // orr.w  r0, r1, r2, ror #4
    assert_eq!(
        ArmT32Instruction::Bic_Register_T2(R::R6, R::R7, R::R8, Shift::Lsl(5), false)
            .encode()
            .unwrap(),
        vec![0x27, 0xEA, 0x48, 0x16]
    ); // bic.w  r6, r7, r8, lsl #5
    assert_eq!(
        ArmT32Instruction::Add_Register_T3(R::R0, R::R8, R::R9, Shift::Lsl(0), true)
            .encode()
            .unwrap(),
        vec![0x18, 0xEB, 0x09, 0x00]
    ); // adds.w r0, r8, r9
    assert_eq!(
        ArmT32Instruction::Sub_Register_T2(R::R10, R::R11, R::R12, Shift::Asr(31), true)
            .encode()
            .unwrap(),
        vec![0xBB, 0xEB, 0xEC, 0x7A]
    ); // subs.w r10, r11, r12, asr #31
}

#[test]
fn encode__armv7m_batch7_shifted_register_aliases_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    assert_eq!(
        ArmT32Instruction::Adc_Register_T2(R::R0, R::R1, R::R2, Shift::Lsl(3), false)
            .encode()
            .unwrap(),
        vec![0x41, 0xEB, 0xC2, 0x00]
    ); // adc.w  r0, r1, r2, lsl #3
    assert_eq!(
        ArmT32Instruction::Sbc_Register_T2(R::R3, R::R4, R::R5, Shift::Asr(2), false)
            .encode()
            .unwrap(),
        vec![0x64, 0xEB, 0xA5, 0x03]
    ); // sbc.w  r3, r4, r5, asr #2
    assert_eq!(
        ArmT32Instruction::Rsb_Register_T1(R::R6, R::R7, R::R8, Shift::Lsr(1), false)
            .encode()
            .unwrap(),
        vec![0xC7, 0xEB, 0x58, 0x06]
    ); // rsb.w  r6, r7, r8, lsr #1
    assert_eq!(
        ArmT32Instruction::Orn_Register_T1(R::R0, R::R1, R::R2, Shift::Ror(4), false)
            .encode()
            .unwrap(),
        vec![0x61, 0xEA, 0x32, 0x10]
    ); // orn.w  r0, r1, r2, ror #4
    assert_eq!(
        ArmT32Instruction::Mov_Register_T3(R::R0, R::R1, Shift::Lsl(0), false)
            .encode()
            .unwrap(),
        vec![0x4F, 0xEA, 0x01, 0x00]
    ); // mov.w  r0, r1
    assert_eq!(
        ArmT32Instruction::Mov_Register_T3(R::R0, R::R1, Shift::Lsl(3), false)
            .encode()
            .unwrap(),
        vec![0x4F, 0xEA, 0xC1, 0x00]
    ); // lsl.w  r0, r1, #3
    assert_eq!(
        ArmT32Instruction::Mov_Register_T3(R::R4, R::R5, Shift::Asr(5), false)
            .encode()
            .unwrap(),
        vec![0x4F, 0xEA, 0x65, 0x14]
    ); // asr.w  r4, r5, #5
    assert_eq!(
        ArmT32Instruction::Mov_Register_T3(R::R0, R::R1, Shift::Rrx, false)
            .encode()
            .unwrap(),
        vec![0x4F, 0xEA, 0x31, 0x00]
    ); // rrx    r0, r1
    assert_eq!(
        ArmT32Instruction::Mvn_Register_T2(R::R0, R::R1, Shift::Lsl(2), true)
            .encode()
            .unwrap(),
        vec![0x7F, 0xEA, 0x81, 0x00]
    ); // mvns.w r0, r1, lsl #2
    assert_eq!(
        ArmT32Instruction::Tst_Register_T2(R::R0, R::R1, Shift::Lsl(3))
            .encode()
            .unwrap(),
        vec![0x10, 0xEA, 0xC1, 0x0F]
    ); // tst.w  r0, r1, lsl #3
    assert_eq!(
        ArmT32Instruction::Cmn_Register_T2(R::R4, R::R5, Shift::Asr(1))
            .encode()
            .unwrap(),
        vec![0x14, 0xEB, 0x65, 0x0F]
    ); // cmn.w  r4, r5, asr #1
    assert_eq!(
        ArmT32Instruction::Cmp_Register_T3(R::R6, R::R7, Shift::Lsl(2))
            .encode()
            .unwrap(),
        vec![0xB6, 0xEB, 0x87, 0x0F]
    ); // cmp.w  r6, r7, lsl #2
}

#[test]
fn encode__armv7m_batch8_wide_load_store_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    assert_eq!(
        ArmT32Instruction::Ldrb_Immediate_T2(R::R0, R::R1, 100)
            .encode()
            .unwrap(),
        vec![0x91, 0xF8, 0x64, 0x00]
    ); // ldrb.w  r0, [r1, #100]
    assert_eq!(
        ArmT32Instruction::Ldrh_Immediate_T2(R::R2, R::R3, 200)
            .encode()
            .unwrap(),
        vec![0xB3, 0xF8, 0xC8, 0x20]
    ); // ldrh.w  r2, [r3, #200]
    assert_eq!(
        ArmT32Instruction::Ldrsb_Immediate_T1(R::R4, R::R5, 4)
            .encode()
            .unwrap(),
        vec![0x95, 0xF9, 0x04, 0x40]
    ); // ldrsb.w r4, [r5, #4]
    assert_eq!(
        ArmT32Instruction::Strb_Immediate_T2(R::R0, R::R1, 100)
            .encode()
            .unwrap(),
        vec![0x81, 0xF8, 0x64, 0x00]
    ); // strb.w  r0, [r1, #100]
    assert_eq!(
        ArmT32Instruction::Ldr_Register_T2(R::R0, R::R1, R::R2, 0)
            .encode()
            .unwrap(),
        vec![0x51, 0xF8, 0x02, 0x00]
    ); // ldr.w  r0, [r1, r2]
    assert_eq!(
        ArmT32Instruction::Ldr_Register_T2(R::R0, R::R1, R::R2, 2)
            .encode()
            .unwrap(),
        vec![0x51, 0xF8, 0x22, 0x00]
    ); // ldr.w  r0, [r1, r2, lsl #2]
    assert_eq!(
        ArmT32Instruction::Ldrb_Register_T2(R::R0, R::R1, R::R2, 0)
            .encode()
            .unwrap(),
        vec![0x11, 0xF8, 0x02, 0x00]
    ); // ldrb.w r0, [r1, r2]
    assert_eq!(
        ArmT32Instruction::Ldrsh_Register_T2(R::R4, R::R5, R::R6, 3)
            .encode()
            .unwrap(),
        vec![0x35, 0xF9, 0x36, 0x40]
    ); // ldrsh.w r4, [r5, r6, lsl #3]
}

#[test]
fn encode__armv7m_batch9_long_multiply_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    assert_eq!(
        ArmT32Instruction::Smull_T1(R::R0, R::R1, R::R2, R::R3)
            .encode()
            .unwrap(),
        vec![0x82, 0xFB, 0x03, 0x01]
    ); // smull r0, r1, r2, r3
    assert_eq!(
        ArmT32Instruction::Umull_T1(R::R4, R::R5, R::R6, R::R7)
            .encode()
            .unwrap(),
        vec![0xA6, 0xFB, 0x07, 0x45]
    ); // umull r4, r5, r6, r7
    assert_eq!(
        ArmT32Instruction::Smlal_T1(R::R0, R::R1, R::R2, R::R3)
            .encode()
            .unwrap(),
        vec![0xC2, 0xFB, 0x03, 0x01]
    ); // smlal r0, r1, r2, r3
    assert_eq!(
        ArmT32Instruction::Umlal_T1(R::R4, R::R5, R::R6, R::R7)
            .encode()
            .unwrap(),
        vec![0xE6, 0xFB, 0x07, 0x45]
    ); // umlal r4, r5, r6, r7
}

#[test]
fn encode__armv7m_batch10_extend_reverse_saturate_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    assert_eq!(
        ArmT32Instruction::Sxtb_T2(R::R0, R::R1, 0)
            .encode()
            .unwrap(),
        vec![0x4F, 0xFA, 0x81, 0xF0]
    ); // sxtb.w  r0, r1
    assert_eq!(
        ArmT32Instruction::Sxtb_T2(R::R0, R::R1, 8)
            .encode()
            .unwrap(),
        vec![0x4F, 0xFA, 0x91, 0xF0]
    ); // sxtb.w  r0, r1, ror #8
    assert_eq!(
        ArmT32Instruction::Uxtb_T2(R::R2, R::R3, 16)
            .encode()
            .unwrap(),
        vec![0x5F, 0xFA, 0xA3, 0xF2]
    ); // uxtb.w  r2, r3, ror #16
    assert_eq!(
        ArmT32Instruction::Sxth_T2(R::R4, R::R5, 24)
            .encode()
            .unwrap(),
        vec![0x0F, 0xFA, 0xB5, 0xF4]
    ); // sxth.w  r4, r5, ror #24
    assert_eq!(
        ArmT32Instruction::Uxth_T2(R::R6, R::R7, 0)
            .encode()
            .unwrap(),
        vec![0x1F, 0xFA, 0x87, 0xF6]
    ); // uxth.w  r6, r7
    assert_eq!(
        ArmT32Instruction::Rev_T2(R::R0, R::R1).encode().unwrap(),
        vec![0x91, 0xFA, 0x81, 0xF0]
    ); // rev.w   r0, r1
    assert_eq!(
        ArmT32Instruction::Rev16_T2(R::R2, R::R3).encode().unwrap(),
        vec![0x93, 0xFA, 0x93, 0xF2]
    ); // rev16.w r2, r3
    assert_eq!(
        ArmT32Instruction::Revsh_T2(R::R4, R::R5).encode().unwrap(),
        vec![0x95, 0xFA, 0xB5, 0xF4]
    ); // revsh.w r4, r5
    assert_eq!(
        ArmT32Instruction::Ssat_T1(R::R0, 5, R::R1, Shift::Lsl(0))
            .encode()
            .unwrap(),
        vec![0x01, 0xF3, 0x04, 0x00]
    ); // ssat r0, #5, r1
    assert_eq!(
        ArmT32Instruction::Usat_T1(R::R2, 7, R::R3, Shift::Lsl(0))
            .encode()
            .unwrap(),
        vec![0x83, 0xF3, 0x07, 0x02]
    ); // usat r2, #7, r3
    assert_eq!(
        ArmT32Instruction::Ssat_T1(R::R0, 5, R::R1, Shift::Lsl(2))
            .encode()
            .unwrap(),
        vec![0x01, 0xF3, 0x84, 0x00]
    ); // ssat r0, #5, r1, lsl #2
    assert_eq!(
        ArmT32Instruction::Ssat_T1(R::R4, 10, R::R5, Shift::Asr(3))
            .encode()
            .unwrap(),
        vec![0x25, 0xF3, 0xC9, 0x04]
    ); // ssat r4, #10, r5, asr #3
}

#[test]
fn encode__armv7m_batch11_indexed_dual_literal_preload_exact_bytes() {
    use Mode::{Offset, PostIndex, PreIndex};
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    // single-register indexed (T4/T3/T2)
    assert_eq!(
        ArmT32Instruction::Ldr_Immediate_T4(R::R0, R::R1, -4, Offset)
            .encode()
            .unwrap(),
        vec![0x51, 0xF8, 0x04, 0x0C]
    ); // ldr   r0, [r1, #-4]
    assert_eq!(
        ArmT32Instruction::Ldr_Immediate_T4(R::R0, R::R1, 4, PreIndex)
            .encode()
            .unwrap(),
        vec![0x51, 0xF8, 0x04, 0x0F]
    ); // ldr   r0, [r1, #4]!
    assert_eq!(
        ArmT32Instruction::Ldr_Immediate_T4(R::R0, R::R1, 4, PostIndex)
            .encode()
            .unwrap(),
        vec![0x51, 0xF8, 0x04, 0x0B]
    ); // ldr   r0, [r1], #4
    assert_eq!(
        ArmT32Instruction::Str_Immediate_T4(R::R2, R::R3, -8, Offset)
            .encode()
            .unwrap(),
        vec![0x43, 0xF8, 0x08, 0x2C]
    ); // str   r2, [r3, #-8]
    assert_eq!(
        ArmT32Instruction::Ldrb_Immediate_T3(R::R0, R::R1, 4, PreIndex)
            .encode()
            .unwrap(),
        vec![0x11, 0xF8, 0x04, 0x0F]
    ); // ldrb  r0, [r1, #4]!
    assert_eq!(
        ArmT32Instruction::Ldrh_Immediate_T3(R::R0, R::R1, -2, Offset)
            .encode()
            .unwrap(),
        vec![0x31, 0xF8, 0x02, 0x0C]
    ); // ldrh  r0, [r1, #-2]
    assert_eq!(
        ArmT32Instruction::Ldrsb_Immediate_T2(R::R0, R::R1, -1, PreIndex)
            .encode()
            .unwrap(),
        vec![0x11, 0xF9, 0x01, 0x0D]
    ); // ldrsb r0, [r1, #-1]!
    assert_eq!(
        ArmT32Instruction::Ldrsh_Immediate_T2(R::R0, R::R1, 2, PostIndex)
            .encode()
            .unwrap(),
        vec![0x31, 0xF9, 0x02, 0x0B]
    ); // ldrsh r0, [r1], #2
    // dual-register
    assert_eq!(
        ArmT32Instruction::Ldrd_Immediate_T1(R::R0, R::R1, R::R2, 8, Offset)
            .encode()
            .unwrap(),
        vec![0xD2, 0xE9, 0x02, 0x01]
    ); // ldrd r0, r1, [r2, #8]
    assert_eq!(
        ArmT32Instruction::Ldrd_Immediate_T1(R::R0, R::R1, R::R2, -8, Offset)
            .encode()
            .unwrap(),
        vec![0x52, 0xE9, 0x02, 0x01]
    ); // ldrd r0, r1, [r2, #-8]
    assert_eq!(
        ArmT32Instruction::Ldrd_Immediate_T1(R::R0, R::R1, R::R2, 8, PreIndex)
            .encode()
            .unwrap(),
        vec![0xF2, 0xE9, 0x02, 0x01]
    ); // ldrd r0, r1, [r2, #8]!
    assert_eq!(
        ArmT32Instruction::Ldrd_Immediate_T1(R::R0, R::R1, R::R2, 8, PostIndex)
            .encode()
            .unwrap(),
        vec![0xF2, 0xE8, 0x02, 0x01]
    ); // ldrd r0, r1, [r2], #8
    assert_eq!(
        ArmT32Instruction::Strd_Immediate_T1(R::R4, R::R5, R::R6, 16, Offset)
            .encode()
            .unwrap(),
        vec![0xC6, 0xE9, 0x04, 0x45]
    ); // strd r4, r5, [r6, #16]
    assert_eq!(
        ArmT32Instruction::Strd_Immediate_T1(R::R4, R::R5, R::R6, -16, PreIndex)
            .encode()
            .unwrap(),
        vec![0x66, 0xE9, 0x04, 0x45]
    ); // strd r4, r5, [r6, #-16]!
    // PC-relative literal loads
    assert_eq!(
        ArmT32Instruction::Ldr_Literal_T2(R::R0, 100)
            .encode()
            .unwrap(),
        vec![0xDF, 0xF8, 0x64, 0x00]
    ); // ldr.w   r0, [pc, #100]
    assert_eq!(
        ArmT32Instruction::Ldr_Literal_T2(R::R0, -100)
            .encode()
            .unwrap(),
        vec![0x5F, 0xF8, 0x64, 0x00]
    ); // ldr.w   r0, [pc, #-100]
    assert_eq!(
        ArmT32Instruction::Ldrb_Literal_T1(R::R1, 8)
            .encode()
            .unwrap(),
        vec![0x9F, 0xF8, 0x08, 0x10]
    ); // ldrb.w  r1, [pc, #8]
    assert_eq!(
        ArmT32Instruction::Ldrh_Literal_T1(R::R2, 16)
            .encode()
            .unwrap(),
        vec![0xBF, 0xF8, 0x10, 0x20]
    ); // ldrh.w  r2, [pc, #16]
    assert_eq!(
        ArmT32Instruction::Ldrsb_Literal_T1(R::R3, 4)
            .encode()
            .unwrap(),
        vec![0x9F, 0xF9, 0x04, 0x30]
    ); // ldrsb.w r3, [pc, #4]
    // preload hints
    assert_eq!(
        ArmT32Instruction::Pld_Immediate_T1(R::R0, 4)
            .encode()
            .unwrap(),
        vec![0x90, 0xF8, 0x04, 0xF0]
    ); // pld [r0, #4]
    assert_eq!(
        ArmT32Instruction::Pld_Immediate_T1(R::R0, -4)
            .encode()
            .unwrap(),
        vec![0x10, 0xF8, 0x04, 0xFC]
    ); // pld [r0, #-4]
    assert_eq!(
        ArmT32Instruction::Pld_Immediate_T1(R::R1, 255)
            .encode()
            .unwrap(),
        vec![0x91, 0xF8, 0xFF, 0xF0]
    ); // pld [r1, #255]
    assert_eq!(
        ArmT32Instruction::Pli_Immediate_T1(R::R0, 4)
            .encode()
            .unwrap(),
        vec![0x90, 0xF9, 0x04, 0xF0]
    ); // pli [r0, #4]
    assert_eq!(
        ArmT32Instruction::Pli_Immediate_T1(R::R2, 8)
            .encode()
            .unwrap(),
        vec![0x92, 0xF9, 0x08, 0xF0]
    ); // pli [r2, #8]
}

#[test]
fn encode__armv7m_batch12_load_store_multiple_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi`
    let r123 = || vec![R::R1, R::R2, R::R3];
    let r456 = || vec![R::R4, R::R5, R::R6];
    assert_eq!(
        ArmT32Instruction::Ldmia_T2(R::R0, false, r123())
            .encode()
            .unwrap(),
        vec![0x90, 0xE8, 0x0E, 0x00]
    ); // ldm.w  r0, {r1, r2, r3}
    assert_eq!(
        ArmT32Instruction::Ldmia_T2(R::R0, true, r123())
            .encode()
            .unwrap(),
        vec![0xB0, 0xE8, 0x0E, 0x00]
    ); // ldm.w  r0!, {r1, r2, r3}
    assert_eq!(
        ArmT32Instruction::Stmia_T2(R::R0, false, r456())
            .encode()
            .unwrap(),
        vec![0x80, 0xE8, 0x70, 0x00]
    ); // stm.w  r0, {r4, r5, r6}
    assert_eq!(
        ArmT32Instruction::Stmia_T2(R::R0, true, r456())
            .encode()
            .unwrap(),
        vec![0xA0, 0xE8, 0x70, 0x00]
    ); // stm.w  r0!, {r4, r5, r6}
    assert_eq!(
        ArmT32Instruction::Ldmdb_T1(R::R0, false, r123())
            .encode()
            .unwrap(),
        vec![0x10, 0xE9, 0x0E, 0x00]
    ); // ldmdb  r0, {r1, r2, r3}
    assert_eq!(
        ArmT32Instruction::Ldmdb_T1(R::R0, true, r123())
            .encode()
            .unwrap(),
        vec![0x30, 0xE9, 0x0E, 0x00]
    ); // ldmdb  r0!, {r1, r2, r3}
    assert_eq!(
        ArmT32Instruction::Stmdb_T1(R::R0, false, r456())
            .encode()
            .unwrap(),
        vec![0x00, 0xE9, 0x70, 0x00]
    ); // stmdb  r0, {r4, r5, r6}
    assert_eq!(
        ArmT32Instruction::Stmdb_T1(R::R0, true, r456())
            .encode()
            .unwrap(),
        vec![0x20, 0xE9, 0x70, 0x00]
    ); // stmdb  r0!, {r4, r5, r6}
    // PUSH.W is STMDB sp! and POP.W is LDM sp!
    assert_eq!(
        ArmT32Instruction::Stmdb_T1(R::R13, true, vec![R::R4, R::R5, R::R14])
            .encode()
            .unwrap(),
        vec![0x2D, 0xE9, 0x30, 0x40]
    ); // push.w {r4, r5, lr}
    assert_eq!(
        ArmT32Instruction::Ldmia_T2(R::R13, true, vec![R::R4, R::R5, R::R15])
            .encode()
            .unwrap(),
        vec![0xBD, 0xE8, 0x30, 0x80]
    ); // pop.w  {r4, r5, pc}
    assert_eq!(
        ArmT32Instruction::Ldmia_T2(R::R3, false, vec![R::R4, R::R12, R::R14])
            .encode()
            .unwrap(),
        vec![0x93, 0xE8, 0x10, 0x50]
    ); // ldm.w  r3, {r4, r12, lr}
    assert_eq!(
        ArmT32Instruction::Ldmia_T2(R::R0, true, vec![R::R1, R::R15])
            .encode()
            .unwrap(),
        vec![0xB0, 0xE8, 0x02, 0x80]
    ); // ldm.w  r0!, {r1, pc}
}

#[test]
fn encode__armv7m_batch13_wide_and_compare_branches_exact_bytes() {
    // bytes verified against `clang --target=thumbv7m-none-eabi` (offsets are PC-relative, PC = addr + 4)
    assert_eq!(
        ArmT32Instruction::B_T4(2).encode().unwrap(),
        vec![0x00, 0xF0, 0x01, 0xB8]
    ); // b.w    .+2
    assert_eq!(
        ArmT32Instruction::B_T4(-4).encode().unwrap(),
        vec![0xFF, 0xF7, 0xFE, 0xBF]
    ); // b.w    .-4
    assert_eq!(
        ArmT32Instruction::B_T3(Cond::Equal, 2).encode().unwrap(),
        vec![0x00, 0xF0, 0x01, 0x80]
    ); // beq.w .+2
    assert_eq!(
        ArmT32Instruction::B_T3(Cond::NotEqual, -16)
            .encode()
            .unwrap(),
        vec![0x7F, 0xF4, 0xF8, 0xAF]
    ); // bne.w .-16
    assert_eq!(
        ArmT32Instruction::B_T3(Cond::SignedGreaterThan, 42)
            .encode()
            .unwrap(),
        vec![0x00, 0xF3, 0x15, 0x80]
    ); // bgt.w .+42
    assert_eq!(
        ArmT32Instruction::Cbz_T1(L::R0, 0).encode().unwrap(),
        vec![0x00, 0xB1]
    ); // cbz  r0, .+0
    assert_eq!(
        ArmT32Instruction::Cbz_T1(L::R3, 18).encode().unwrap(),
        vec![0x4B, 0xB1]
    ); // cbz  r3, .+18
    assert_eq!(
        ArmT32Instruction::Cbnz_T1(L::R1, 2).encode().unwrap(),
        vec![0x09, 0xB9]
    ); // cbnz r1, .+2
}

#[test]
fn round_trip__armv7m_branches() {
    // covers the offset extremes and both sizes (B.W/B<c>.W are 32-bit, CBZ/CBNZ 16-bit).
    let instructions = [
        ArmT32Instruction::B_T4(0),
        ArmT32Instruction::B_T4(2),
        ArmT32Instruction::B_T4(-4),
        ArmT32Instruction::B_T4(16_777_214),
        ArmT32Instruction::B_T4(-16_777_216),
        ArmT32Instruction::B_T3(Cond::Equal, 2),
        ArmT32Instruction::B_T3(Cond::NotEqual, -16),
        ArmT32Instruction::B_T3(Cond::SignedLessThanOrEqual, 1_048_574),
        ArmT32Instruction::B_T3(Cond::CarrySet, -1_048_576),
        ArmT32Instruction::Cbz_T1(L::R0, 0),
        ArmT32Instruction::Cbz_T1(L::R7, 126),
        ArmT32Instruction::Cbnz_T1(L::R1, 2),
        ArmT32Instruction::Cbnz_T1(L::R3, 64),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmT32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, bytes.len(), "consumed wrong byte count");
        assert_eq!(decoded, instruction, "branch round-trip mismatch");
    }
}

