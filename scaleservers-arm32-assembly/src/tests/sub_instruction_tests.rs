// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__sub_immediate_t1__immediate_and_register_to_register() {
    // SUBS r0, r1, #3
    let instruction = ArmT32Instruction::Sub_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        3,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xC8, 0x1E]);
}

#[test]
fn decode__sub_immediate_t1__immediate_and_register_to_register() {
    // SUBS r0, r1, #3
    let encoded_instruction = [0xC8, 0x1E];
    let verify_instruction = ArmT32Instruction::Sub_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        3,
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
fn encode__sub_immediate_t2__immediate_to_register() {
    // SUBS r0, #85
    let instruction = ArmT32Instruction::Sub_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 85);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x55, 0x38]);
}

#[test]
fn decode__sub_immediate_t2__immediate_to_register() {
    // SUBS r0, #85
    let encoded_instruction = [0x55, 0x38];
    let verify_instruction =
        ArmT32Instruction::Sub_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 85);
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
fn encode__sub_register_t1__register_and_register_to_register() {
    // SUBS r0, r1, r2
    let instruction = ArmT32Instruction::Sub_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        Arm32LowGeneralPurposeRegister::R2,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x88, 0x1A]);
}

#[test]
fn decode__sub_register_t1__register_and_register_to_register() {
    // SUBS r0, r1, r2
    let encoded_instruction = [0x88, 0x1A];
    let verify_instruction = ArmT32Instruction::Sub_Register_T1(
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

//

#[test]
fn encode__sub_sp_minus_immediate_t1__immediate_to_sp() {
    // SUB SP, SP, #340
    let instruction = ArmT32Instruction::Sub_SpMinusImmediate_T1(340);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xD5, 0xB0]);
}

#[test]
fn decode__sub_sp_minus_immediate_t1__immediate_to_sp() {
    // SUB SP, SP, #340
    let encoded_instruction = [0xD5, 0xB0];
    let verify_instruction = ArmT32Instruction::Sub_SpMinusImmediate_T1(340);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
