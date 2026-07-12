// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;

#[test]
fn encode__ldm_t1() {
    // LDM r6, {r0, r2, r4, r6}
    let instruction = ArmT32Instruction::Ldm_T1(
        Arm32LowGeneralPurposeRegister::R6,
        vec![
            Arm32LowGeneralPurposeRegister::R0,
            Arm32LowGeneralPurposeRegister::R2,
            Arm32LowGeneralPurposeRegister::R4,
            Arm32LowGeneralPurposeRegister::R6,
        ],
    );
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x55, 0xCE]);
}

#[test]
fn decode__ldm_t1() {
    // LDM r6, {r0, r2, r4, r6}
    let encoded_instruction = [0x55, 0xCE];
    let verify_instruction = ArmT32Instruction::Ldm_T1(
        Arm32LowGeneralPurposeRegister::R6,
        vec![
            Arm32LowGeneralPurposeRegister::R0,
            Arm32LowGeneralPurposeRegister::R2,
            Arm32LowGeneralPurposeRegister::R4,
            Arm32LowGeneralPurposeRegister::R6,
        ],
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
