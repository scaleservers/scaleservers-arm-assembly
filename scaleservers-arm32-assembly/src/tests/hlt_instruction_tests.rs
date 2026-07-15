// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;

// HLT #imm6 -- T1 encoding 0xBA80 | imm6 (the 0xBA80 slot in the REV family).
// Dual-oracle confirmed: arm-none-eabi-as -mthumb -march=armv8.1-a + llvm-mc thumbv8.1a.

#[test]
fn encode__hlt_t1__immediate() {
    assert_eq!(ArmT32Instruction::Hlt_T1(0).encode().ok().unwrap(), vec![0x80, 0xBA]); // hlt #0
    assert_eq!(ArmT32Instruction::Hlt_T1(63).encode().ok().unwrap(), vec![0xBF, 0xBA]); // hlt #63 (max imm6)
}

#[test]
fn decode__hlt_t1__immediate() {
    let encoded_instruction = [0xBF, 0xBA];
    let verify_instruction = ArmT32Instruction::Hlt_T1(63);
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
fn emit__hlt_t1() {
    assert_eq!(
        ArmT32Instruction::Hlt_T1(7).to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "hlt #7"
    );
}
