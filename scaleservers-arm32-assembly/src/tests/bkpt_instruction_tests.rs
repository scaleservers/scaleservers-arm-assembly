// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;

#[test]
fn encode__bkpt_t1__immediate() {
    // BKPT #170
    let instruction = ArmT32Instruction::Bkpt_T1(170);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xAA, 0xBE]);
}

#[test]
fn decode__svc_t1__immediate() {
    // BKPT #170
    let encoded_instruction = [0xAA, 0xBE];
    let verify_instruction = ArmT32Instruction::Bkpt_T1(170);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
