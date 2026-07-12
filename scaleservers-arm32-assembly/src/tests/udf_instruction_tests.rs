// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::ArmT32Instruction;

#[test]
fn encode__udf_t1__immediate() {
    // UDF #170
    let instruction = ArmT32Instruction::Udf_T1(170);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xAA, 0xDE]);
}

#[test]
fn decode__udf_t1__immediate() {
    // UDF #170
    let encoded_instruction = [0xAA, 0xDE];
    let verify_instruction = ArmT32Instruction::Udf_T1(170);
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
fn encode__udf_t2__immediate() {
    // UDF.W #42405
    let instruction = ArmT32Instruction::Udf_T2(42405);
    //
    let encoded_instruction = instruction.encode().ok().unwrap();
    assert_eq!(encoded_instruction, vec![0xFA, 0xF7, 0xA5, 0xA5]);
}

#[test]
fn decode__udf_t2__immediate() {
    // UDF.W #42405
    let encoded_instruction = [0xFA, 0xF7, 0xA5, 0xA5];
    let verify_instruction = ArmT32Instruction::Udf_T2(42405);
    //
    let mut iter_offset = 0;
    let instruction = ArmT32Instruction::decode(&mut encoded_instruction.iter(), &mut iter_offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(instruction, verify_instruction);
    assert_eq!(iter_offset, encoded_instruction.len());
}
