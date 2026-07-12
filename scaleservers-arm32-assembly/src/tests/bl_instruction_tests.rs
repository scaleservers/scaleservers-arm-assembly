// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;

#[test]
fn encode__bl_t1__immediate() {
    // BL #-5592406
    let instruction = ArmT32Instruction::Bl_T1(-5592406);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xAA, 0xF6, 0x55, 0xF5]);
}

#[test]
fn decode__bl_t1__immediate() {
    // BL #-5592406
    let encoded_instruction = [0xAA, 0xF6, 0x55, 0xF5];
    let verify_instruction = ArmT32Instruction::Bl_T1(-5592406);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
