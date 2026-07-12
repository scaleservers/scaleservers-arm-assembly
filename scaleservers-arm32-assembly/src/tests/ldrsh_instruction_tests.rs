// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__ldrsh_register_t1__register_and_register_to_register() {
    // LDRSH r0, [r1, r2]
    let instruction = ArmT32Instruction::Ldrsh_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        Arm32LowGeneralPurposeRegister::R2,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x88, 0x5E]);
}

#[test]
fn decode__ldrsh_register_t1__register_and_register_to_register() {
    // LDRSH r0, [r1, r2]
    let encoded_instruction = [0x88, 0x5E];
    let verify_instruction = ArmT32Instruction::Ldrsh_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        Arm32LowGeneralPurposeRegister::R2,
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
