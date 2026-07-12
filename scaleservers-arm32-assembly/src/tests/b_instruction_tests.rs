// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::ArmT32InstructionCondition;

#[test]
fn encode__b_t1__immediate() {
    // BNE #-256
    let instruction = ArmT32Instruction::B_T1(ArmT32InstructionCondition::NotEqual, -256);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x80, 0xD1]);
}

#[test]
fn decode__b_t1__immediate() {
    // BNE #-256
    let encoded_instruction = [0x80, 0xD1];
    let verify_instruction = ArmT32Instruction::B_T1(ArmT32InstructionCondition::NotEqual, -256);
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
fn encode__b_t2__immediate() {
    // BAL #-2048
    let instruction = ArmT32Instruction::B_T2(-2048);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x00, 0xE4]);
}

#[test]
fn decode__b_t2__immediate() {
    // BAL #-2048
    let encoded_instruction = [0x00, 0xE4];
    let verify_instruction = ArmT32Instruction::B_T2(-2048);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
