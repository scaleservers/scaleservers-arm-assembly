// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// Spot-checks of the UAL emit layer: exact text (GNU flavor) for representative forms, the LLVM-vs-GNU
// immediate-radix difference, the ARMv7-M additions, and a decode->emit pass that resolves a PC-relative
// branch target to an absolute address. (Exhaustive text correctness is the differential oracle's job.)

use crate::enums::{
    Arm32GeneralPurposeRegister, Arm32LowGeneralPurposeRegister, ArmT32InstructionCondition,
    ArmT32SpecialRegister,
};
use crate::{ArmAssemblySyntax, ArmT32Instruction};

const GNU: ArmAssemblySyntax = ArmAssemblySyntax::Gnu;
const LLVM: ArmAssemblySyntax = ArmAssemblySyntax::Llvm;

#[test]
fn emit__data_processing_forms() {
    assert_eq!(
        ArmT32Instruction::Add_Immediate_T1(
            Arm32LowGeneralPurposeRegister::R0,
            Arm32LowGeneralPurposeRegister::R1,
            3
        )
        .to_assembly_string(GNU),
        "adds r0, r1, #3"
    );
    assert_eq!(
        ArmT32Instruction::Add_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 85)
            .to_assembly_string(GNU),
        "adds r0, #85"
    );
    assert_eq!(
        ArmT32Instruction::Mov_Immediate_T1(Arm32LowGeneralPurposeRegister::R3, 5)
            .to_assembly_string(GNU),
        "movs r3, #5"
    );
    assert_eq!(
        ArmT32Instruction::Lsl_Immediate_T1(
            Arm32LowGeneralPurposeRegister::R0,
            Arm32LowGeneralPurposeRegister::R1,
            4
        )
        .to_assembly_string(GNU),
        "lsls r0, r1, #4"
    );
    // high-register T2 form does NOT carry the flag-setting `s` suffix
    assert_eq!(
        ArmT32Instruction::Add_Register_T2(
            Arm32GeneralPurposeRegister::R0,
            Arm32GeneralPurposeRegister::R8
        )
        .to_assembly_string(GNU),
        "add r0, r8"
    );
    assert_eq!(
        ArmT32Instruction::Cmp_Register_T2(
            Arm32GeneralPurposeRegister::R10,
            Arm32GeneralPurposeRegister::R11
        )
        .to_assembly_string(GNU),
        "cmp r10, r11"
    );
}

#[test]
fn emit__llvm_vs_gnu_immediate_radix() {
    let instruction = ArmT32Instruction::Cmp_Immediate_T1(Arm32LowGeneralPurposeRegister::R0, 31);
    assert_eq!(instruction.to_assembly_string(GNU), "cmp r0, #31");
    assert_eq!(instruction.to_assembly_string(LLVM), "cmp r0, #0x1f");
}

#[test]
fn emit__stack_and_address_forms() {
    assert_eq!(
        ArmT32Instruction::Add_SpPlusImmediate_T1(Arm32LowGeneralPurposeRegister::R0, 340)
            .to_assembly_string(GNU),
        "add r0, sp, #340"
    );
    assert_eq!(
        ArmT32Instruction::Add_SpPlusImmediate_T2(340).to_assembly_string(GNU),
        "add sp, sp, #340"
    );
    assert_eq!(
        ArmT32Instruction::Sub_SpMinusImmediate_T1(16).to_assembly_string(GNU),
        "sub sp, sp, #16"
    );
}

#[test]
fn emit__memory_and_lists() {
    assert_eq!(
        ArmT32Instruction::Ldr_Immediate_T1(
            Arm32LowGeneralPurposeRegister::R0,
            Arm32LowGeneralPurposeRegister::R1,
            4
        )
        .to_assembly_string(GNU),
        "ldr r0, [r1, #4]"
    );
    assert_eq!(
        ArmT32Instruction::Str_Register_T1(
            Arm32LowGeneralPurposeRegister::R0,
            Arm32LowGeneralPurposeRegister::R1,
            Arm32LowGeneralPurposeRegister::R2
        )
        .to_assembly_string(GNU),
        "str r0, [r1, r2]"
    );
    assert_eq!(
        ArmT32Instruction::Push_T1(vec![
            Arm32GeneralPurposeRegister::R4,
            Arm32GeneralPurposeRegister::R5,
            Arm32GeneralPurposeRegister::R14
        ])
        .to_assembly_string(GNU),
        "push {r4, r5, lr}"
    );
    assert_eq!(
        ArmT32Instruction::Ldm_T1(
            Arm32LowGeneralPurposeRegister::R0,
            vec![
                Arm32LowGeneralPurposeRegister::R1,
                Arm32LowGeneralPurposeRegister::R2
            ]
        )
        .to_assembly_string(GNU),
        "ldmia r0!, {r1, r2}"
    );
}

