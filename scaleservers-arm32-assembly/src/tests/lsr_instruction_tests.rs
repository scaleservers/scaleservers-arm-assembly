// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__lsr_immediate_t1__immediate_and_register_to_register() {
    // LSRS r0, r1, #3
    let instruction = ArmT32Instruction::Lsr_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        3,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xC8, 0x08]);
}

#[test]
fn decode__lsr_immediate_t1__immediate_and_register_to_register() {
    // LSRS r0, r1, #3
    let encoded_instruction = [0xC8, 0x08];
    let verify_instruction = ArmT32Instruction::Lsr_Immediate_T1(
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
fn encode__lsr_immediate_t1__immediate_and_register_to_register__max_wraparound() {
    // LSRS r0, r1, #32
    let instruction = ArmT32Instruction::Lsr_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        32,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x08, 0x08]);
}

#[test]
fn decode__lsr_immediate_t1__immediate_and_register_to_register__max_wraparound() {
    // LSRS r0, r1, #32
    let encoded_instruction = [0x08, 0x08];
    let verify_instruction = ArmT32Instruction::Lsr_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        32,
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
fn encode__lsr_register_t1__register_to_register() {
    // LSRS r0, r1
    let instruction = ArmT32Instruction::Lsr_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xC8, 0x40]);
}

#[test]
fn decode__lsr_register_t1__register_to_register() {
    // LSRS r0, r1
    let encoded_instruction = [0xC8, 0x40];
    let verify_instruction = ArmT32Instruction::Lsr_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
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
