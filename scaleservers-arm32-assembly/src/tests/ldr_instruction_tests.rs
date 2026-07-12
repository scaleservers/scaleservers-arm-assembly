// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__ldr_immediate_t1__immediate_and_register_to_register() {
    // LDR r0, [r1, #84]
    let instruction = ArmT32Instruction::Ldr_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        84,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x48, 0x6D]);
}

#[test]
fn decode__ldr_immediate_t1__immediate_and_register_to_register() {
    // LDR r0, [r1, #84]
    let encoded_instruction = [0x48, 0x6D];
    let verify_instruction = ArmT32Instruction::Ldr_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        84,
    );
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}

//

#[test]
fn encode__ldr_immediate_t2__immediate_and_sp_to_register() {
    // LDR r0, [SP, #680]
    let instruction = ArmT32Instruction::Ldr_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 680);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xAA, 0x98]);
}

#[test]
fn decode__ldr_immediate_t2__immediate_and_sp_to_register() {
    // LDR r0, [SP, #680]
    let encoded_instruction = [0xAA, 0x98];
    let verify_instruction =
        ArmT32Instruction::Ldr_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 680);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}

//

#[test]
fn encode__ldr_literal_t1__immediate_and_pc_to_register() {
    // LDR r0, [PC, #680]
    let instruction = ArmT32Instruction::Ldr_Literal_T1(Arm32LowGeneralPurposeRegister::R0, 680);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xAA, 0x48]);
}

#[test]
fn decode__ldr_literal_t1__immediate_and_pc_to_register() {
    // LDR r0, [PC, #680]
    let encoded_instruction = [0xAA, 0x48];
    let verify_instruction =
        ArmT32Instruction::Ldr_Literal_T1(Arm32LowGeneralPurposeRegister::R0, 680);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}

//

#[test]
fn encode__ldr_register_t1__register_and_register_to_register() {
    // LDR r0, [r1, r2]
    let instruction = ArmT32Instruction::Ldr_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        Arm32LowGeneralPurposeRegister::R2,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x88, 0x58]);
}

#[test]
fn decode__ldr_register_t1__register_and_register_to_register() {
    // LDR r0, [r1, r2]
    let encoded_instruction = [0x88, 0x58];
    let verify_instruction = ArmT32Instruction::Ldr_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        Arm32LowGeneralPurposeRegister::R2,
    );
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
