// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;

#[test]
fn encode__nop_t1() {
    // NOP
    let instruction = ArmT32Instruction::Nop_T1;
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x00, 0xBF]);
}

#[test]
fn decode__nop_t1() {
    // NOP
    let encoded_instruction = [0x00, 0xBF];
    let verify_instruction = ArmT32Instruction::Nop_T1;
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
