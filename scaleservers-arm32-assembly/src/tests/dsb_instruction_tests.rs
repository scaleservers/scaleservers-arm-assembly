// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;
use crate::enums::ArmT32MemoryBarrierOption;

#[test]
fn encode__dsb_t1__option() {
    // DSB SY
    let instruction = ArmT32Instruction::Dsb_T1(ArmT32MemoryBarrierOption::System);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xBF, 0xF3, 0x4F, 0x8F]);
}

#[test]
fn decode__dsb_t1__option() {
    // DSB SY
    let encoded_instruction = [0xBF, 0xF3, 0x4F, 0x8F];
    let verify_instruction = ArmT32Instruction::Dsb_T1(ArmT32MemoryBarrierOption::System);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
