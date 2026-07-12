// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::{Arm32GeneralPurposeRegister, Arm32LowGeneralPurposeRegister};

#[test]
fn encode__add_immediate_t1__immediate_and_register_to_register() {
    // ADDS r0, r1, #3
    let instruction = ArmT32Instruction::Add_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        3,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xC8, 0x1C]);
}

#[test]
fn decode__add_immediate_t1__immediate_and_register_to_register() {
    // ADDS r0, r1, #3
    let encoded_instruction = [0xC8, 0x1C];
    let verify_instruction = ArmT32Instruction::Add_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        3,
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
fn encode__add_immediate_t2__immediate_to_register() {
    // ADDS r0, #85
    let instruction = ArmT32Instruction::Add_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 85);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x55, 0x30]);
}

#[test]
fn decode__add_immediate_t2__immediate_to_register() {
    // ADDS r0, #85
    let encoded_instruction = [0x55, 0x30];
    let verify_instruction =
        ArmT32Instruction::Add_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 85);
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
fn encode__add_register_t1__register_and_register_to_register() {
    // ADDS r0, r1, r2
    let instruction = ArmT32Instruction::Add_Register_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        Arm32LowGeneralPurposeRegister::R2,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x88, 0x18]);
}

#[test]
fn decode__add_register_t1__register_and_register_to_register() {
    // ADDS r0, r1, r2
    let encoded_instruction = [0x88, 0x18];
    let verify_instruction = ArmT32Instruction::Add_Register_T1(
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

//

#[test]
fn encode__add_register_t2__register_to_register() {
    // ADD r0, r8
    let instruction = ArmT32Instruction::Add_Register_T2(
        Arm32GeneralPurposeRegister::R0,
        Arm32GeneralPurposeRegister::R8,
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x40, 0x44]);
}

#[test]
fn decode__add_register_t2__register_to_register() {
    // ADD r0, r8
    let encoded_instruction = [0x40, 0x44];
    let verify_instruction = ArmT32Instruction::Add_Register_T2(
        Arm32GeneralPurposeRegister::R0,
        Arm32GeneralPurposeRegister::R8,
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

