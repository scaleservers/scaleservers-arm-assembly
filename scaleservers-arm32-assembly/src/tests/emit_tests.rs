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

#[test]
fn emit__system_forms() {
    assert_eq!(
        ArmT32Instruction::Mrs_T1(
            Arm32GeneralPurposeRegister::R0,
            ArmT32SpecialRegister::Primask
        )
        .to_assembly_string(GNU),
        "mrs r0, PRIMASK"
    );
    assert_eq!(
        ArmT32Instruction::Msr_Register_T1(
            ArmT32SpecialRegister::Control,
            Arm32GeneralPurposeRegister::R1
        )
        .to_assembly_string(GNU),
        "msr CONTROL, r1"
    );
    assert_eq!(ArmT32Instruction::Nop_T1.to_assembly_string(GNU), "nop");
}

#[test]
fn emit__armv7m_additions() {
    assert_eq!(
        ArmT32Instruction::Mov_Immediate_T3(Arm32GeneralPurposeRegister::R0, 0x1234)
            .to_assembly_string(GNU),
        "movw r0, #4660"
    );
    assert_eq!(
        ArmT32Instruction::Mov_Immediate_T3(Arm32GeneralPurposeRegister::R0, 0x1234)
            .to_assembly_string(LLVM),
        "movw r0, #0x1234"
    );
    assert_eq!(
        ArmT32Instruction::Movt_T1(Arm32GeneralPurposeRegister::R1, 0xABCD)
            .to_assembly_string(LLVM),
        "movt r1, #0xabcd"
    );
    assert_eq!(
        ArmT32Instruction::Mul_T2(
            Arm32GeneralPurposeRegister::R0,
            Arm32GeneralPurposeRegister::R1,
            Arm32GeneralPurposeRegister::R2
        )
        .to_assembly_string(GNU),
        "mul r0, r1, r2"
    );
    assert_eq!(
        ArmT32Instruction::Mla_T1(
            Arm32GeneralPurposeRegister::R0,
            Arm32GeneralPurposeRegister::R1,
            Arm32GeneralPurposeRegister::R2,
            Arm32GeneralPurposeRegister::R3
        )
        .to_assembly_string(GNU),
        "mla r0, r1, r2, r3"
    );
    assert_eq!(
        ArmT32Instruction::Sdiv_T1(
            Arm32GeneralPurposeRegister::R0,
            Arm32GeneralPurposeRegister::R1,
            Arm32GeneralPurposeRegister::R2
        )
        .to_assembly_string(GNU),
        "sdiv r0, r1, r2"
    );
    assert_eq!(
        ArmT32Instruction::Clz_T1(
            Arm32GeneralPurposeRegister::R3,
            Arm32GeneralPurposeRegister::R4
        )
        .to_assembly_string(GNU),
        "clz r3, r4"
    );
}

#[test]
fn emit__branch_raw_vs_address_resolved() {
    let conditional = ArmT32Instruction::B_T1(ArmT32InstructionCondition::Equal, 8);
    assert_eq!(conditional.to_assembly_string(GNU), "beq #8");
    assert_eq!(
        conditional.to_assembly_string_at(0x0000_0100, GNU),
        "beq 0x0000010c"
    );
}

#[test]
fn emit__decode_then_emit_at() {
    let bytes = [0xC8, 0x1C];
    let mut offset = 0;
    let instruction = ArmT32Instruction::decode(&mut bytes.iter(), &mut offset)
        .ok()
        .unwrap()
        .unwrap();
    assert_eq!(
        instruction.to_assembly_string_at(0x0000_2000, GNU),
        "adds r0, r1, #3"
    );
}
