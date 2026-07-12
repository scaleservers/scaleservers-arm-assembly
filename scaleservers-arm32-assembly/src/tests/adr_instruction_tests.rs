// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__adr_t1__register() {
    // ADR r0, #340
    let instruction = ArmT32Instruction::Adr_T1(Arm32LowGeneralPurposeRegister::R0, 340);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x55, 0xA0]);
}

#[test]
fn decode__adr_t1__register() {
    // ADR r0, #340
    let encoded_instruction = [0x55, 0xA0];
    let verify_instruction = ArmT32Instruction::Adr_T1(Arm32LowGeneralPurposeRegister::R0, 340);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
