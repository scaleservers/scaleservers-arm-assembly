// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;

// SETPAN #imm1 -- T1 encoding 0xB610 | (pan << 3), in the 0xB6xx CPS/SETEND miscellaneous space.
// Dual-oracle confirmed: arm-none-eabi-as -mthumb -march=armv8.1-a + llvm-mc thumbv8.1a.

#[test]
fn encode__setpan_t1__immediate() {
    assert_eq!(ArmT32Instruction::Setpan_T1(false).encode().ok().unwrap(), vec![0x10, 0xB6]); // setpan #0
    assert_eq!(ArmT32Instruction::Setpan_T1(true).encode().ok().unwrap(), vec![0x18, 0xB6]); // setpan #1
}

#[test]
fn decode__setpan_t1__immediate() {
    let encoded_instruction = [0x18, 0xB6];
    let verify_instruction = ArmT32Instruction::Setpan_T1(true);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}

#[test]
fn emit__setpan_t1() {
    assert_eq!(
        ArmT32Instruction::Setpan_T1(true).to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "setpan #1"
    );
}
