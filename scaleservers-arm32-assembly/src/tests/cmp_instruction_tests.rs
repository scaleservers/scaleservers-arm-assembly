// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::{Arm32GeneralPurposeRegister, Arm32LowGeneralPurposeRegister};

#[test]
fn encode__cmp_immediate_t1__immediate_to_register() {
    // CMP r0, #85
    let instruction = ArmT32Instruction::Cmp_Immediate_T1(Arm32LowGeneralPurposeRegister::R0, 85);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x55, 0x28]);
}

#[test]
fn decode__cmp_immediate_t1__immediate_to_register() {
    // CMP r0, #85
    let encoded_instruction = [0x55, 0x28];
    let verify_instruction =
        ArmT32Instruction::Cmp_Immediate_T1(Arm32LowGeneralPurposeRegister::R0, 85);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}

//

#[test]
fn encode__cmp_register_t1__register_and_register() {
    // CMP r0, r1
    let instruction = ArmT32Instruction::Cmp_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x88, 0x42]);
}

#[test]
fn decode__cmp_register_t1__register_and_register() {
    // CMP r0, r1
    let encoded_instruction = [0x88, 0x42];
    let verify_instruction = ArmT32Instruction::Cmp_Register_T1(
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

//

#[test]
fn encode__cmp_register_t2__register_and_register() {
    // CMP r8, r9
    let instruction = ArmT32Instruction::Cmp_Register_T2(
        Arm32GeneralPurposeRegister::R8,
        Arm32GeneralPurposeRegister::R9,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xC8, 0x45]);
}

#[test]
fn decode__cmp_register_t2__register_and_register() {
    // CMP r8, r9
    let encoded_instruction = [0xC8, 0x45];
    let verify_instruction = ArmT32Instruction::Cmp_Register_T2(
        Arm32GeneralPurposeRegister::R8,
        Arm32GeneralPurposeRegister::R9,
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
