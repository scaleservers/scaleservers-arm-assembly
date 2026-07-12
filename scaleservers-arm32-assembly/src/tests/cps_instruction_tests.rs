// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::ArmT32CpsPrimaskEffect;

#[test]
fn encode__cps_t1() {
    // CPSID i
    let instruction = ArmT32Instruction::Cps_T1(ArmT32CpsPrimaskEffect::InterruptDisable);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0x72, 0xB6]);
}

#[test]
fn decode__cps_t1() {
    // CPSID i
    let encoded_instruction = [0x72, 0xB6];
    let verify_instruction = ArmT32Instruction::Cps_T1(ArmT32CpsPrimaskEffect::InterruptDisable);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
