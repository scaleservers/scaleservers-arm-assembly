// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__ldrb_immediate_t1__immediate_and_register_to_register() {
    // LDRB r0, [r1, #21]
    let instruction = ArmT32Instruction::Ldrb_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        21,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x48, 0x7D]);
}

#[test]
fn decode__ldrb_immediate_t1__immediate_and_register_to_register() {
    // LDRB r0, [r1, #21]
    let encoded_instruction = [0x48, 0x7D];
    let verify_instruction = ArmT32Instruction::Ldrb_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        21,
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
fn encode__ldrb_register_t1__register_and_register_to_register() {
    // LDRB r0, [r1, r2]
    let instruction = ArmT32Instruction::Ldrb_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        Arm32LowGeneralPurposeRegister::R2,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x88, 0x5C]);
}

#[test]
fn decode__ldrb_register_t1__register_and_register_to_register() {
    // LDRB r0, [r1, r2]
    let encoded_instruction = [0x88, 0x5C];
    let verify_instruction = ArmT32Instruction::Ldrb_Register_T1(
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
