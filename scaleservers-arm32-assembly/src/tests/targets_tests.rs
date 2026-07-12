// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// Exercises the target-gating machinery. The whole implemented set is ARMv6-M, so `encode_for_target`
// against an ARMv6-M profile never rejects today; these tests pin the supports() ordering/feature logic
// (which the ARMv7-M milestone will rely on) and the happy path of the guarded encode.

use crate::ArmT32Instruction;
use crate::enums::Arm32LowGeneralPurposeRegister;
use crate::targets::{ArmCpuFeature, ArmInstructionRequirement, ArmIsaVersion, ArmTargetProfile};

#[test]
fn requirement__current_set_is_armv6m_baseline() {
    let instruction = ArmT32Instruction::Add_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 85);
    assert_eq!(
        instruction.requirement(),
        ArmInstructionRequirement::baseline()
    );
    assert_eq!(
        instruction.requirement().min_isa_version,
        ArmIsaVersion::Armv6M
    );
}

#[test]
fn encode_for_target__armv6m_profile_matches_plain_encode() {
    let instruction = ArmT32Instruction::Add_Immediate_T2(Arm32LowGeneralPurposeRegister::R0, 85);
    //
    let gated = instruction.encode_for_target(&ArmTargetProfile::armv6m());
    assert_eq!(gated, instruction.encode());
    assert!(gated.is_ok());
}

#[test]
fn supports__newer_isa_satisfies_older_requirement_but_not_vice_versa() {
    let armv6m = ArmTargetProfile::armv6m();
    let armv7m = ArmTargetProfile::armv7m();

    let needs_v6m = ArmInstructionRequirement::new(ArmIsaVersion::Armv6M, &[]);
    let needs_v7m = ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[]);

    // an ARMv7-M core runs ARMv6-M instructions...
    assert!(armv7m.supports(&needs_v6m));
    // ...but an ARMv6-M core cannot run an ARMv7-M-only instruction
    assert!(armv6m.supports(&needs_v6m));
    assert!(!armv6m.supports(&needs_v7m));
    assert!(armv7m.supports(&needs_v7m));
}

#[test]
fn supports__required_feature_must_be_present() {
    let needs_dsp =
        ArmInstructionRequirement::new(ArmIsaVersion::Armv7EM, &[ArmCpuFeature::DspExtension]);

    // armv7em() bundles the DSP extension; a bare ARMv7-M profile lacks it
    assert!(ArmTargetProfile::armv7em().supports(&needs_dsp));
    assert!(!ArmTargetProfile::armv7m().supports(&needs_dsp));
    // the permissive profile (used by the oracle harness) supports everything
    assert!(ArmTargetProfile::permissive().supports(&needs_dsp));
}
