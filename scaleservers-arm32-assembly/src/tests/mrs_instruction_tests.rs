// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::{Arm32GeneralPurposeRegister, ArmT32SpecialRegister};

#[test]
fn encode__mrs_t1__special_register_to_register() {
    // MRS R0, CONTROL
    let instruction = ArmT32Instruction::Mrs_T1(
        Arm32GeneralPurposeRegister::R0,
        ArmT32SpecialRegister::Control,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xEF, 0xF3, 0x14, 0x80]);
}

#[test]
fn decode__mrs_t1__special_register_to_register() {
    // MRS R0, CONTROL
    let encoded_instruction = [0xEF, 0xF3, 0x14, 0x80];
    let verify_instruction = ArmT32Instruction::Mrs_T1(
        Arm32GeneralPurposeRegister::R0,
        ArmT32SpecialRegister::Control,
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
