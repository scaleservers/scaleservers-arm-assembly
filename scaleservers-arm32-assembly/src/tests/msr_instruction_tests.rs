// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::{Arm32GeneralPurposeRegister, ArmT32SpecialRegister};

#[test]
fn encode__msr_register_t1__register_to_special_register() {
    // MSR CONTROL, R0
    let instruction = ArmT32Instruction::Msr_Register_T1(
        ArmT32SpecialRegister::Control,
        Arm32GeneralPurposeRegister::R0,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x80, 0xF3, 0x14, 0x88]);
}

#[test]
fn decode__msr_register_t1__register_to_special_register() {
    // MSR CONTROL, R0
    let encoded_instruction = [0x80, 0xF3, 0x14, 0x88];
    let verify_instruction = ArmT32Instruction::Msr_Register_T1(
        ArmT32SpecialRegister::Control,
        Arm32GeneralPurposeRegister::R0,
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
