// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32GeneralPurposeRegister;

#[test]
fn encode__pop_t1() {
    // POP {r0, r2, r4, r6, PC}
    let instruction = ArmT32Instruction::Pop_T1(vec![
        Arm32GeneralPurposeRegister::R0,
        Arm32GeneralPurposeRegister::R2,
        Arm32GeneralPurposeRegister::R4,
        Arm32GeneralPurposeRegister::R6,
        Arm32GeneralPurposeRegister::R15,
    ]);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x55, 0xBD]);
}

#[test]
fn decode__pop_t1() {
    // POP {r0, r2, r4, r6, PC}
    let encoded_instruction = [0x55, 0xBD];
    let verify_instruction = ArmT32Instruction::Pop_T1(vec![
        Arm32GeneralPurposeRegister::R0,
        Arm32GeneralPurposeRegister::R2,
        Arm32GeneralPurposeRegister::R4,
        Arm32GeneralPurposeRegister::R6,
        Arm32GeneralPurposeRegister::R15,
    ]);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
