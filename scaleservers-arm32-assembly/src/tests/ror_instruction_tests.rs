// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__ror_register_t1__register_to_register() {
    // RORS r0, r1
    let instruction = ArmT32Instruction::Ror_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xC8, 0x41]);
}

#[test]
fn decode__ror_register_t1__register_to_register() {
    // RORS r0, r1
    let encoded_instruction = [0xC8, 0x41];
    let verify_instruction = ArmT32Instruction::Ror_Register_T1(
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
