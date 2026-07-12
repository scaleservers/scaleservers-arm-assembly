// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// ARMv8-M tests: the Security Extension (TrustZone-M) instructions and the lazy FP state save/restore.
// Exact encodings are cross-checked against `arm-none-eabi-as -march=armv8-m.main+fp` (Thumb mode); each
// family round-trips (encode -> decode -> encode) and is gated behind the v8-M ISA version + the Security
// feature (so a plain ARMv7-M target REFUSES them).

use crate::enums::Arm32GeneralPurposeRegister as R;
use crate::{
    ArmCpuFeature, ArmInstructionRequirement, ArmIsaVersion, ArmT32Instruction, ArmTargetProfile,
};

fn round_trip(instruction: &ArmT32Instruction) {
    let bytes = instruction.encode().unwrap();
    let mut offset = 0;
    let decoded = ArmT32Instruction::decode(&mut bytes.iter(), &mut offset)
        .unwrap()
        .unwrap();
    assert_eq!(offset, bytes.len(), "consumed wrong byte count");
    assert_eq!(
        &decoded, instruction,
        "decode did not reproduce the encoded instruction"
    );
}

#[test]
fn encode__armv8m_security_exact_bytes() {
    // bytes verified against `arm-none-eabi-as -march=armv8-m.main+fp` (Thumb)
    assert_eq!(
        ArmT32Instruction::Sg_T1.encode().unwrap(),
        vec![0x7f, 0xe9, 0x7f, 0xe9]
    ); // sg
    assert_eq!(
        ArmT32Instruction::Bxns_T1(R::R3).encode().unwrap(),
        vec![0x1c, 0x47]
    ); // bxns r3
    assert_eq!(
        ArmT32Instruction::Blxns_T1(R::R5).encode().unwrap(),
        vec![0xac, 0x47]
    ); // blxns r5
    assert_eq!(
        ArmT32Instruction::Tt_T1(R::R0, R::R1, false, false)
            .encode()
            .unwrap(),
        vec![0x41, 0xe8, 0x00, 0xf0]
    ); // tt   r0, r1
    assert_eq!(
        ArmT32Instruction::Tt_T1(R::R0, R::R1, false, true)
            .encode()
            .unwrap(),
        vec![0x41, 0xe8, 0x40, 0xf0]
    ); // ttt  r0, r1
    assert_eq!(
        ArmT32Instruction::Tt_T1(R::R2, R::R3, true, false)
            .encode()
            .unwrap(),
        vec![0x43, 0xe8, 0x80, 0xf2]
    ); // tta  r2, r3
    assert_eq!(
        ArmT32Instruction::Tt_T1(R::R2, R::R3, true, true)
            .encode()
            .unwrap(),
        vec![0x43, 0xe8, 0xc0, 0xf2]
    ); // ttat r2, r3
    assert_eq!(
        ArmT32Instruction::Vlstm_T1(R::R4).encode().unwrap(),
        vec![0x24, 0xec, 0x00, 0x0a]
    ); // vlstm r4
    assert_eq!(
        ArmT32Instruction::Vlldm_T1(R::R4).encode().unwrap(),
        vec![0x34, 0xec, 0x00, 0x0a]
    ); // vlldm r4
    assert_eq!(
        ArmT32Instruction::Csdb_T1.encode().unwrap(),
        vec![0xaf, 0xf3, 0x14, 0x80]
    ); // csdb
}

#[test]
fn round_trip__armv8m_security() {
    let instructions = [
        ArmT32Instruction::Sg_T1,
        ArmT32Instruction::Bxns_T1(R::R3),
        ArmT32Instruction::Blxns_T1(R::R9),
        ArmT32Instruction::Tt_T1(R::R0, R::R1, false, false),
        ArmT32Instruction::Tt_T1(R::R5, R::R6, false, true),
        ArmT32Instruction::Tt_T1(R::R2, R::R3, true, false),
        ArmT32Instruction::Tt_T1(R::R7, R::R8, true, true),
        ArmT32Instruction::Vlstm_T1(R::R4),
        ArmT32Instruction::Vlldm_T1(R::R12),
        ArmT32Instruction::Csdb_T1,
    ];
    for instruction in &instructions {
        round_trip(instruction);
    }
}

#[test]
fn gating__security_instruction_requires_v8m_and_security() {
    let sg = ArmT32Instruction::Sg_T1;
    assert_eq!(
        sg.requirement(),
        ArmInstructionRequirement::new(ArmIsaVersion::Armv8MBaseline, &[ArmCpuFeature::Security])
    );
    // a plain ARMv7-M (Cortex-M3) target has neither v8-M nor the Security feature -> refused.
    assert!(sg.encode_for_target(&ArmTargetProfile::armv7m()).is_err());
    // an ARMv8-M Baseline (Cortex-M23) target with the Security Extension accepts it.
    assert_eq!(
        sg.encode_for_target(&ArmTargetProfile::armv8m_baseline()),
        sg.encode()
    );
    assert_eq!(
        sg.encode_for_target(&ArmTargetProfile::armv8m_mainline()),
        sg.encode()
    );

    // VLSTM additionally needs the Mainline profile (FP) -- Baseline (no FP) refuses it.
    let vlstm = ArmT32Instruction::Vlstm_T1(R::R4);
    assert!(
        vlstm
            .encode_for_target(&ArmTargetProfile::armv8m_baseline())
            .is_err()
    );
    assert_eq!(
        vlstm.encode_for_target(&ArmTargetProfile::armv8m_mainline()),
        vlstm.encode()
    );
}
