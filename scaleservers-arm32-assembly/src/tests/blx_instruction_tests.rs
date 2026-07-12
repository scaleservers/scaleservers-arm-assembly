// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32GeneralPurposeRegister;

#[test]
fn encode__blx_register_t1__register() {
    // BLX r8
    let instruction = ArmT32Instruction::Blx_Register_T1(Arm32GeneralPurposeRegister::R8);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xC0, 0x47]);
}

#[test]
fn decode__blx_register_t1__register() {
    // BLX r8
    let encoded_instruction = [0xC0, 0x47];
    let verify_instruction = ArmT32Instruction::Blx_Register_T1(Arm32GeneralPurposeRegister::R8);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
