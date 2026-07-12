// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// These tests lock in the robustness contract: encoding a constructible-but-invalid instruction returns
// a structured EncodeError instead of panicking.

use crate::ArmT32Instruction;
use crate::EncodeError;
use crate::enums::{Arm32GeneralPurposeRegister, Arm32LowGeneralPurposeRegister};

#[test]
fn encode__immediate_out_of_range__returns_error() {
    // ADD r0, SP, #1024  -- const10 max is 1020
    let instruction =
        ArmT32Instruction::Add_SpPlusImmediate_T1(Arm32LowGeneralPurposeRegister::R0, 1024);
    //
    let result = instruction.encode();
    assert_eq!(
        result,
        Err(EncodeError::ImmediateOutOfRange {
            field: "const10",
            value: 1024,
            minimum: 0,
            maximum: 1020
        })
    );
}

#[test]
fn encode__sp_immediate_t2_above_508__returns_error() {
    // ADD SP, SP, #512 -- the imm7 field caps const9 at 508 (regression guard for the old <=1020 bug)
    let instruction = ArmT32Instruction::Add_SpPlusImmediate_T2(512);
    //
    let result = instruction.encode();
    assert_eq!(
        result,
        Err(EncodeError::ImmediateOutOfRange {
            field: "const9",
            value: 512,
            minimum: 0,
            maximum: 508
        })
    );
}

#[test]
fn encode__immediate_not_aligned__returns_error() {
    // LDR r0, [r1, #3]  -- decoded_imm7 must be a multiple of 4
    let instruction = ArmT32Instruction::Ldr_Immediate_T1(
        Arm32LowGeneralPurposeRegister::R0,
        Arm32LowGeneralPurposeRegister::R1,
        3,
    );
    //
    let result = instruction.encode();
    assert_eq!(
        result,
        Err(EncodeError::ImmediateNotAligned {
            field: "decoded_imm7",
            value: 3,
            required_multiple: 4
        })
    );
}

#[test]
fn encode__sp_register_in_add_register_slot__returns_error() {
    // ADD <Rn>, SP must use the SP-relative form, not Add_Register_T2
    let instruction = ArmT32Instruction::Add_Register_T2(
        Arm32GeneralPurposeRegister::R0,
        Arm32GeneralPurposeRegister::R13,
    );
    //
    let result = instruction.encode();
    assert_eq!(
        result,
        Err(EncodeError::RegisterNotEncodable {
            field: "m",
            detail: "m cannot be SP (R13); use Add_SpPlusRegister_T1 instead"
        })
    );
}

#[test]
fn encode__al_condition_in_conditional_branch__returns_error() {
    // B<c> T1 cannot encode the AL condition (that slot is UDF)
    let instruction = ArmT32Instruction::B_T1(
        crate::enums::ArmT32InstructionCondition::AlwaysUnconditional,
        8,
    );
    //
    let result = instruction.encode();
    assert_eq!(
        result,
        Err(EncodeError::ConditionNotEncodable {
            field: "cond",
            detail: "the AL (0b1110) condition is not encodable in B<c> T1; that slot is UDF"
        })
    );
}

#[test]
fn encode__high_register_in_push_list__returns_error() {
    // PUSH may only list R0-R7 and LR (R14); R8 is not encodable
    let instruction = ArmT32Instruction::Push_T1(vec![Arm32GeneralPurposeRegister::R8]);
    //
    let result = instruction.encode();
    assert_eq!(
        result,
        Err(EncodeError::RegisterNotEncodable {
            field: "registers",
            detail: "a PUSH register list may only contain R0-R7 and LR (R14)"
        })
    );
}

#[test]
fn encode__valid_boundary_values_still_succeed() {
    // the inclusive maxima must still encode cleanly
    assert!(
        ArmT32Instruction::Add_SpPlusImmediate_T1(Arm32LowGeneralPurposeRegister::R0, 1020)
            .encode()
            .is_ok()
    );
    assert!(
        ArmT32Instruction::Add_SpPlusImmediate_T2(508)
            .encode()
            .is_ok()
    );
    assert!(
        ArmT32Instruction::Ldr_Immediate_T1(
            Arm32LowGeneralPurposeRegister::R0,
            Arm32LowGeneralPurposeRegister::R1,
            124
        )
        .encode()
        .is_ok()
    );
}
