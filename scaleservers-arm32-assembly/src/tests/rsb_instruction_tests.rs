// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__rsb_immediate_t1__register_and_zero_to_register() {
    // RSBS r0, r1, #0
    let instruction = ArmT32Instruction::Rsb_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1, /*, imm5: 0*/
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x48, 0x42]);
}

#[test]
fn decode__rsb_immediate_t1__register_and_zero_to_register() {
    // RSBS r0, r1, #0
    let encoded_instruction = [0x48, 0x42];
    let verify_instruction = ArmT32Instruction::Rsb_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1, /*, imm5: 0*/
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
