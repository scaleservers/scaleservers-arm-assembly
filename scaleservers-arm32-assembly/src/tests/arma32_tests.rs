// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// A32 ("ARM" state) scaffolding smoke test: exact little-endian encodings (cross-checked against the ARM
// ARM and the well-known classic ARM-mode word values), encode<->decode round-trips, the
// requirement()/target gate, the separate-type delineation (an A32 instruction is REFUSED by a Thumb-only
// M-profile target yet accepted by an A/R-profile one), and the `Arm32Instruction` interworking union.

use crate::Arm32Condition as Cond;
use crate::Arm32DirectedRound as DRnd;
use crate::Arm32ExtendType as Ext;
use crate::Arm32IndexMode as Idx;
use crate::Arm32MemoryOffset as Mem;
use crate::Arm32MemoryOffset8 as Mem8;
use crate::Arm32NeonAesOp as NAes;
use crate::Arm32NeonBitwiseOp as NBit;
use crate::Arm32NeonDiffLongOp as NDL;
use crate::Arm32NeonDiffNarrowOp as NDN;
use crate::Arm32NeonDiffWideOp as NDW;
use crate::Arm32NeonFloatOp as NFlt;
use crate::Arm32NeonIntegerOp as NInt;
use crate::Arm32NeonLoadStoreAddress as NLsa;
use crate::Arm32NeonMisc2FixedOp as NMF;
use crate::Arm32NeonMisc2SizedOp as NMS;
use crate::Arm32NeonNarrowOp as NMN;
use crate::Arm32NeonScalarLongOp as NScL;
use crate::Arm32NeonScalarOp as NSc;
use crate::Arm32NeonSha2Op as NSha2;
use crate::Arm32NeonSha3Op as NSha3;
use crate::Arm32NeonShiftNarrowOp as NShN;
use crate::Arm32NeonShiftOp as NSh;
use crate::Arm32NeonSize as NSz;
use crate::Arm32ParallelOperation as POp;
use crate::Arm32ParallelPrefix as PPfx;
use crate::Arm32RegisterShift as Shift;
use crate::Arm32ShiftType as ShiftType;
use crate::Arm32VmovLaneSize as VLS;
use crate::Arm32VrintMode as VRnd;
use crate::Arm32VselCondition as Vsel;
use crate::enums::Arm32GeneralPurposeRegister as R;
use crate::{
    Arm32Instruction, ArmA32Instruction, ArmInstructionRequirement, ArmInstructionSet,
    ArmIsaVersion, ArmT32Instruction, ArmTargetProfile, EncodeError,
};

#[test]
fn encode__a32_smoke_exact_bytes() {
    // exact A32 words (little-endian byte order), cross-checked against the ARM Architecture Reference Manual
    // nop                          -> 0xE320F000
    assert_eq!(
        ArmA32Instruction::Nop_A1(Cond::AlwaysUnconditional)
            .encode()
            .unwrap(),
        vec![0x00, 0xF0, 0x20, 0xE3]
    );
    // mov r0, r0                   -> 0xE1A00000  (the classic ARM-mode no-op)
    assert_eq!(
        ArmA32Instruction::Mov_Register_A1(
            Cond::AlwaysUnconditional,
            false,
            R::R0,
            R::R0,
            Shift::none()
        )
        .encode()
        .unwrap(),
        vec![0x00, 0x00, 0xA0, 0xE1]
    );
    // movs r1, r2, lsl #3          -> 0xE1B01182
    assert_eq!(
        ArmA32Instruction::Mov_Register_A1(
            Cond::AlwaysUnconditional,
            true,
            R::R1,
            R::R2,
            Shift::Lsl(3)
        )
        .encode()
        .unwrap(),
        vec![0x82, 0x11, 0xB0, 0xE1]
    );
    // add r0, r1, r2               -> 0xE0810002
    assert_eq!(
        ArmA32Instruction::Add_Register_A1(
            Cond::AlwaysUnconditional,
            false,
            R::R0,
            R::R1,
            R::R2,
            Shift::none()
        )
        .encode()
        .unwrap(),
        vec![0x02, 0x00, 0x81, 0xE0]
    );
    // addeq r3, r4, r5             -> 0x00843005  (exercises a non-AL condition nibble)
    assert_eq!(
        ArmA32Instruction::Add_Register_A1(Cond::Equal, false, R::R3, R::R4, R::R5, Shift::none())
            .encode()
            .unwrap(),
        vec![0x05, 0x30, 0x84, 0x00]
    );
    // bx lr                        -> 0xE12FFF1E
    assert_eq!(
        ArmA32Instruction::Bx_A1(Cond::AlwaysUnconditional, R::R14)
            .encode()
            .unwrap(),
        vec![0x1E, 0xFF, 0x2F, 0xE1]
    );
    // svc #0x123456                -> 0xEF123456
    assert_eq!(
        ArmA32Instruction::Svc_A1(Cond::AlwaysUnconditional, 0x123456)
            .encode()
            .unwrap(),
        vec![0x56, 0x34, 0x12, 0xEF]
    );
}

#[test]
fn round_trip__a32_smoke() {
    let instructions = [
        ArmA32Instruction::Nop_A1(Cond::AlwaysUnconditional),
        ArmA32Instruction::Mov_Register_A1(Cond::NotEqual, true, R::R1, R::R2, Shift::Lsl(0)),
        ArmA32Instruction::Mov_Register_A1(
            Cond::AlwaysUnconditional,
            false,
            R::R3,
            R::R4,
            Shift::Asr(5),
        ),
        ArmA32Instruction::Mov_Register_A1(
            Cond::AlwaysUnconditional,
            false,
            R::R0,
            R::R1,
            Shift::Lsr(32),
        ),
        ArmA32Instruction::Mov_Register_A1(
            Cond::AlwaysUnconditional,
            false,
            R::R0,
            R::R1,
            Shift::Rrx,
        ),
        ArmA32Instruction::Add_Register_A1(Cond::Equal, false, R::R3, R::R4, R::R5, Shift::none()),
        ArmA32Instruction::Add_Register_A1(
            Cond::AlwaysUnconditional,
            true,
            R::R8,
            R::R9,
            R::R10,
            Shift::Ror(7),
        ),
        ArmA32Instruction::Bx_A1(Cond::AlwaysUnconditional, R::R14),
        ArmA32Instruction::Svc_A1(Cond::CarrySet, 0x000123),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        assert_eq!(bytes.len(), 4, "every A32 instruction is one 32-bit word");
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(decoded, instruction, "A32 round-trip mismatch");
    }
}

#[test]
fn requirement__a32_smoke() {
    assert_eq!(
        ArmA32Instruction::Bx_A1(Cond::AlwaysUnconditional, R::R0).requirement(),
        ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[])
    );
}

#[test]
fn encode_for_target__a32_refused_by_m_profile_accepted_by_ar() {
    let instruction = ArmA32Instruction::Add_Register_A1(
        Cond::AlwaysUnconditional,
        false,
        R::R0,
        R::R1,
        R::R2,
        Shift::none(),
    );

    // a Thumb-only M-profile core has no ARM state, so it refuses every A32 instruction
    assert_eq!(
        instruction.encode_for_target(&ArmTargetProfile::armv6m()),
        Err(EncodeError::UnsupportedInstructionForTarget {
            required: ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            target_isa_version: ArmIsaVersion::Armv6M,
        })
    );
    // an A/R-profile core accepts it, and the gated bytes equal the plain encode()
    assert_eq!(
        instruction.encode_for_target(&ArmTargetProfile::armv7ar()),
        instruction.encode()
    );
    assert_eq!(
        instruction.encode_for_target(&ArmTargetProfile::permissive_aarch32()),
        instruction.encode()
    );
}

#[test]
fn outer_enum__delineates_and_encodes_both_sets() {
    let a32 = Arm32Instruction::A32(ArmA32Instruction::Bx_A1(Cond::AlwaysUnconditional, R::R14));
    let t32 = Arm32Instruction::T32(ArmT32Instruction::Nop_T1);

    assert_eq!(a32.instruction_set(), ArmInstructionSet::A32);
    assert_eq!(t32.instruction_set(), ArmInstructionSet::T32);

    assert_eq!(a32.encode().unwrap(), vec![0x1E, 0xFF, 0x2F, 0xE1]); // bx lr   (A32, 4 bytes)
    assert_eq!(t32.encode().unwrap(), vec![0x00, 0xBF]); // nop     (Thumb, 2 bytes)
}

// ---- A1: data processing ----

// little-endian-byte form of a 32-bit A32 word
fn le(word: u32) -> Vec<u8> {
    word.to_le_bytes().to_vec()
}

fn s(number: u8) -> crate::Arm32SinglePrecisionRegister {
    crate::Arm32SinglePrecisionRegister::new(number).unwrap()
}
fn d(number: u8) -> crate::Arm32DoublePrecisionRegister {
    crate::Arm32DoublePrecisionRegister::new(number).unwrap()
}
fn q(number: u8) -> crate::Arm32QuadwordRegister {
    crate::Arm32QuadwordRegister::new(number).unwrap()
}

#[test]
fn encode__a32_data_processing_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;

    // register forms
    assert_eq!(
        And_Register_A1(al, false, R::R0, R::R1, R::R2, Shift::none())
            .encode()
            .unwrap(),
        le(0xE001_0002)
    ); // and  r0, r1, r2
    assert_eq!(
        Eor_Register_A1(al, false, R::R0, R::R1, R::R2, Shift::Lsr(4))
            .encode()
            .unwrap(),
        le(0xE021_0222)
    ); // eor  r0, r1, r2, lsr #4
    assert_eq!(
        Orr_Register_A1(al, true, R::R5, R::R6, R::R7, Shift::Ror(8))
            .encode()
            .unwrap(),
        le(0xE196_5467)
    ); // orrs r5, r6, r7, ror #8
    assert_eq!(
        Mvn_Register_A1(al, false, R::R0, R::R1, Shift::none())
            .encode()
            .unwrap(),
        le(0xE1E0_0001)
    ); // mvn  r0, r1

    // immediate forms
    assert_eq!(
        Sub_Immediate_A1(al, false, R::R3, R::R4, 5)
            .encode()
            .unwrap(),
        le(0xE244_3005)
    ); // sub  r3, r4, #5
    assert_eq!(
        Add_Immediate_A1(al, false, R::R0, R::R0, 1)
            .encode()
            .unwrap(),
        le(0xE280_0001)
    ); // add  r0, r0, #1
    assert_eq!(
        Mov_Immediate_A1(al, false, R::R0, 0).encode().unwrap(),
        le(0xE3A0_0000)
    ); // mov  r0, #0
    assert_eq!(
        Mov_Immediate_A1(al, false, R::R0, 0x100).encode().unwrap(),
        le(0xE3A0_0C01)
    ); // mov  r0, #0x100  (rotation: imm8=1 ror 24)
    assert_eq!(
        Add_Immediate_A1(al, false, R::R0, R::R1, 0xFF00_0000)
            .encode()
            .unwrap(),
        le(0xE281_04FF)
    ); // add r0, r1, #0xff000000

    // compares (S implied)
    assert_eq!(
        Cmp_Immediate_A1(al, R::R0, 0).encode().unwrap(),
        le(0xE350_0000)
    ); // cmp  r0, #0
    assert_eq!(
        Tst_Immediate_A1(al, R::R1, 1).encode().unwrap(),
        le(0xE311_0001)
    ); // tst  r1, #1
    assert_eq!(
        Cmn_Register_A1(al, R::R4, R::R5, Shift::none())
            .encode()
            .unwrap(),
        le(0xE174_0005)
    ); // cmn  r4, r5

    // 16-bit immediate moves
    assert_eq!(
        Movw_A2(al, R::R0, 0x1234).encode().unwrap(),
        le(0xE301_0234)
    ); // movw r0, #0x1234
    assert_eq!(
        Movt_A1(al, R::R1, 0xABCD).encode().unwrap(),
        le(0xE34A_1BCD)
    ); // movt r1, #0xabcd

    // a non-AL condition on a data-processing instruction
    assert_eq!(
        Add_Register_A1(
            Cond::SignedGreaterThan,
            false,
            R::R0,
            R::R1,
            R::R2,
            Shift::none()
        )
        .encode()
        .unwrap(),
        le(0xC081_0002)
    ); // addgt r0, r1, r2
}

#[test]
fn encode__a32_modified_immediate_not_representable_errors() {
    // 0x12345678 is not an ARM modified immediate (no single 8-bit value rotated by an even amount yields it)
    assert_eq!(
        ArmA32Instruction::Mov_Immediate_A1(Cond::AlwaysUnconditional, false, R::R0, 0x12345678)
            .encode(),
        Err(EncodeError::ModifiedImmediateNotEncodable {
            field: "const",
            value: 0x12345678
        })
    );
    // a representable boundary still succeeds
    assert!(
        ArmA32Instruction::Mov_Immediate_A1(Cond::AlwaysUnconditional, false, R::R0, 0xFF00_0000)
            .encode()
            .is_ok()
    );
}

#[test]
fn round_trip__a32_data_processing() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        And_Immediate_A1(al, true, R::R0, R::R1, 0xFF),
        Eor_Immediate_A1(Cond::NotEqual, false, R::R2, R::R3, 0xAB00),
        Sub_Immediate_A1(al, false, R::R4, R::R5, 0xFF000000),
        Rsb_Immediate_A1(al, true, R::R6, R::R7, 0),
        Add_Immediate_A1(al, false, R::R8, R::R9, 0x3FC),
        Adc_Immediate_A1(al, false, R::R10, R::R11, 1),
        Sbc_Immediate_A1(al, true, R::R12, R::R0, 2),
        Rsc_Immediate_A1(al, false, R::R1, R::R2, 0xC0000000),
        Orr_Immediate_A1(al, false, R::R3, R::R4, 0xFF),
        Bic_Immediate_A1(al, true, R::R5, R::R6, 0x3F00),
        Mov_Immediate_A1(al, false, R::R0, 0x100),
        Mvn_Immediate_A1(al, true, R::R1, 0),
        Tst_Immediate_A1(al, R::R2, 0x80000000),
        Teq_Immediate_A1(al, R::R3, 0xFF),
        Cmp_Immediate_A1(al, R::R4, 0x100),
        Cmn_Immediate_A1(Cond::CarrySet, R::R5, 1),
        And_Register_A1(al, false, R::R0, R::R1, R::R2, Shift::Lsl(0)),
        Eor_Register_A1(al, true, R::R3, R::R4, R::R5, Shift::Lsl(31)),
        Sub_Register_A1(al, false, R::R6, R::R7, R::R8, Shift::Lsr(1)),
        Rsb_Register_A1(al, true, R::R9, R::R10, R::R11, Shift::Lsr(32)),
        Add_Register_A1(al, false, R::R12, R::R0, R::R1, Shift::Asr(1)),
        Adc_Register_A1(al, true, R::R2, R::R3, R::R4, Shift::Asr(32)),
        Sbc_Register_A1(al, false, R::R5, R::R6, R::R7, Shift::Ror(15)),
        Rsc_Register_A1(al, true, R::R8, R::R9, R::R10, Shift::Rrx),
        Orr_Register_A1(al, false, R::R11, R::R12, R::R0, Shift::none()),
        Bic_Register_A1(al, true, R::R1, R::R2, R::R3, Shift::Lsl(7)),
        Mov_Register_A1(al, false, R::R4, R::R5, Shift::Asr(5)),
        Mvn_Register_A1(al, true, R::R6, R::R7, Shift::Ror(1)),
        Tst_Register_A1(al, R::R8, R::R9, Shift::Lsl(3)),
        Teq_Register_A1(al, R::R10, R::R11, Shift::none()),
        Cmp_Register_A1(al, R::R12, R::R0, Shift::Rrx),
        Cmn_Register_A1(al, R::R1, R::R2, Shift::Asr(1)),
        Movw_A2(al, R::R0, 0xFFFF),
        Movt_A1(Cond::Equal, R::R9, 0x8000),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 data-processing round-trip mismatch"
        );
    }
}

#[test]
fn encode__a32_register_shifted_register_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    // shift amount taken from Rs
    assert_eq!(
        Add_RegisterShiftedRegister_A1(al, false, R::R0, R::R1, R::R2, ShiftType::Lsl, R::R3)
            .encode()
            .unwrap(),
        le(0xE081_0312)
    ); // add  r0, r1, r2, lsl r3
    assert_eq!(
        Mov_RegisterShiftedRegister_A1(al, false, R::R0, R::R1, ShiftType::Asr, R::R2)
            .encode()
            .unwrap(),
        le(0xE1A0_0251)
    ); // mov  r0, r1, asr r2   (= asr r0, r1, r2)
    assert_eq!(
        Mov_RegisterShiftedRegister_A1(al, true, R::R4, R::R5, ShiftType::Lsr, R::R6)
            .encode()
            .unwrap(),
        le(0xE1B0_4635)
    ); // movs r4, r5, lsr r6   (= lsrs r4, r5, r6)
    assert_eq!(
        Cmp_RegisterShiftedRegister_A1(al, R::R0, R::R1, ShiftType::Ror, R::R2)
            .encode()
            .unwrap(),
        le(0xE150_0271)
    ); // cmp  r0, r1, ror r2
}

#[test]
fn round_trip__a32_register_shifted_register() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        And_RegisterShiftedRegister_A1(al, true, R::R0, R::R1, R::R2, ShiftType::Lsl, R::R3),
        Eor_RegisterShiftedRegister_A1(al, false, R::R4, R::R5, R::R6, ShiftType::Lsr, R::R7),
        Sub_RegisterShiftedRegister_A1(al, false, R::R8, R::R9, R::R10, ShiftType::Asr, R::R11),
        Rsb_RegisterShiftedRegister_A1(al, true, R::R0, R::R1, R::R2, ShiftType::Ror, R::R3),
        Add_RegisterShiftedRegister_A1(
            Cond::NotEqual,
            false,
            R::R4,
            R::R5,
            R::R6,
            ShiftType::Lsl,
            R::R7,
        ),
        Adc_RegisterShiftedRegister_A1(al, true, R::R8, R::R9, R::R10, ShiftType::Lsr, R::R11),
        Sbc_RegisterShiftedRegister_A1(al, false, R::R0, R::R1, R::R2, ShiftType::Asr, R::R3),
        Rsc_RegisterShiftedRegister_A1(al, true, R::R4, R::R5, R::R6, ShiftType::Ror, R::R7),
        Orr_RegisterShiftedRegister_A1(al, false, R::R8, R::R9, R::R10, ShiftType::Lsl, R::R11),
        Bic_RegisterShiftedRegister_A1(al, true, R::R0, R::R1, R::R2, ShiftType::Lsr, R::R3),
        Mov_RegisterShiftedRegister_A1(al, false, R::R4, R::R5, ShiftType::Asr, R::R6),
        Mvn_RegisterShiftedRegister_A1(al, true, R::R7, R::R8, ShiftType::Ror, R::R9),
        Tst_RegisterShiftedRegister_A1(al, R::R10, R::R11, ShiftType::Lsl, R::R12),
        Teq_RegisterShiftedRegister_A1(al, R::R0, R::R1, ShiftType::Lsr, R::R2),
        Cmp_RegisterShiftedRegister_A1(al, R::R3, R::R4, ShiftType::Asr, R::R5),
        Cmn_RegisterShiftedRegister_A1(Cond::CarrySet, R::R6, R::R7, ShiftType::Ror, R::R8),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 register-shifted-register round-trip mismatch"
        );
    }
}

// ---- A2: multiply ----

#[test]
fn encode__a32_multiply_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(
        Mul_A1(al, false, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE000_0291)
    ); // mul    r0, r1, r2
    assert_eq!(
        Mla_A1(al, false, R::R0, R::R1, R::R2, R::R3)
            .encode()
            .unwrap(),
        le(0xE020_3291)
    ); // mla    r0, r1, r2, r3
    assert_eq!(
        Mls_A1(al, R::R0, R::R1, R::R2, R::R3).encode().unwrap(),
        le(0xE060_3291)
    ); // mls    r0, r1, r2, r3
    assert_eq!(
        Umull_A1(al, false, R::R0, R::R1, R::R2, R::R3)
            .encode()
            .unwrap(),
        le(0xE081_0392)
    ); // umull  r0, r1, r2, r3
    assert_eq!(
        Umlal_A1(al, false, R::R0, R::R1, R::R2, R::R3)
            .encode()
            .unwrap(),
        le(0xE0A1_0392)
    ); // umlal  r0, r1, r2, r3
    assert_eq!(
        Smull_A1(al, true, R::R4, R::R5, R::R6, R::R7)
            .encode()
            .unwrap(),
        le(0xE0D5_4796)
    ); // smulls r4, r5, r6, r7
    assert_eq!(
        Smlal_A1(al, false, R::R0, R::R1, R::R2, R::R3)
            .encode()
            .unwrap(),
        le(0xE0E1_0392)
    ); // smlal  r0, r1, r2, r3
    assert_eq!(
        Umaal_A1(al, R::R0, R::R1, R::R2, R::R3).encode().unwrap(),
        le(0xE041_0392)
    ); // umaal  r0, r1, r2, r3
    assert_eq!(
        Mul_A1(al, true, R::R8, R::R9, R::R10).encode().unwrap(),
        le(0xE018_0A99)
    ); // muls   r8, r9, r10
}

#[test]
fn round_trip__a32_multiply() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        Mul_A1(al, false, R::R0, R::R1, R::R2),
        Mul_A1(Cond::NotEqual, true, R::R3, R::R4, R::R5),
        Mla_A1(al, false, R::R6, R::R7, R::R8, R::R9),
        Mla_A1(al, true, R::R10, R::R11, R::R12, R::R0),
        Mls_A1(al, R::R1, R::R2, R::R3, R::R4),
        Umull_A1(al, false, R::R0, R::R1, R::R2, R::R3),
        Umull_A1(al, true, R::R4, R::R5, R::R6, R::R7),
        Umlal_A1(al, false, R::R8, R::R9, R::R10, R::R11),
        Smull_A1(al, true, R::R0, R::R1, R::R2, R::R3),
        Smlal_A1(Cond::CarrySet, false, R::R4, R::R5, R::R6, R::R7),
        Umaal_A1(al, R::R8, R::R9, R::R10, R::R11),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(decoded, instruction, "A32 multiply round-trip mismatch");
    }
}

// ---- A3: saturating arithmetic ----

#[test]
fn encode__a32_saturating_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(
        Qadd_A1(al, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE102_0051)
    ); // qadd  r0, r1, r2
    assert_eq!(
        Qsub_A1(al, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE122_0051)
    ); // qsub  r0, r1, r2
    assert_eq!(
        Qdadd_A1(al, R::R3, R::R4, R::R5).encode().unwrap(),
        le(0xE145_3054)
    ); // qdadd r3, r4, r5
    assert_eq!(
        Qdsub_A1(al, R::R6, R::R7, R::R8).encode().unwrap(),
        le(0xE168_6057)
    ); // qdsub r6, r7, r8
}

#[test]
fn round_trip__a32_saturating() {
    use ArmA32Instruction::*;
    let instructions = [
        Qadd_A1(Cond::AlwaysUnconditional, R::R0, R::R1, R::R2),
        Qsub_A1(Cond::NotEqual, R::R3, R::R4, R::R5),
        Qdadd_A1(Cond::AlwaysUnconditional, R::R6, R::R7, R::R8),
        Qdsub_A1(Cond::AlwaysUnconditional, R::R9, R::R10, R::R11),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(decoded, instruction, "A32 saturating round-trip mismatch");
    }
}

// ---- A3: signed multiply (DSP) ----

#[test]
fn encode__a32_signed_multiply_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    // halfword multiplies
    assert_eq!(
        Smla_A1(al, R::R0, R::R1, R::R2, R::R3, false, false)
            .encode()
            .unwrap(),
        le(0xE100_3281)
    ); // smlabb  r0, r1, r2, r3
    assert_eq!(
        Smla_A1(al, R::R4, R::R5, R::R6, R::R7, true, true)
            .encode()
            .unwrap(),
        le(0xE104_76E5)
    ); // smlatt  r4, r5, r6, r7
    assert_eq!(
        Smul_A1(al, R::R0, R::R1, R::R2, true, true)
            .encode()
            .unwrap(),
        le(0xE160_02E1)
    ); // smultt  r0, r1, r2
    assert_eq!(
        Smlaw_A1(al, R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        le(0xE120_3281)
    ); // smlawb  r0, r1, r2, r3
    assert_eq!(
        Smulw_A1(al, R::R0, R::R1, R::R2, true).encode().unwrap(),
        le(0xE120_02E1)
    ); // smulwt  r0, r1, r2
    assert_eq!(
        Smlal_Halfword_A1(al, R::R0, R::R1, R::R2, R::R3, false, false)
            .encode()
            .unwrap(),
        le(0xE141_0382)
    ); // smlalbb r0, r1, r2, r3
    // dual / most-significant-word multiplies
    assert_eq!(
        Smlad_A1(al, R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        le(0xE700_3211)
    ); // smlad   r0, r1, r2, r3
    assert_eq!(
        Smuad_A1(al, R::R0, R::R1, R::R2, false).encode().unwrap(),
        le(0xE700_F211)
    ); // smuad   r0, r1, r2
    assert_eq!(
        Smusd_A1(al, R::R0, R::R1, R::R2, false).encode().unwrap(),
        le(0xE700_F251)
    ); // smusd   r0, r1, r2
    assert_eq!(
        Smmul_A1(al, R::R0, R::R1, R::R2, false).encode().unwrap(),
        le(0xE750_F211)
    ); // smmul   r0, r1, r2
    assert_eq!(
        Smmla_A1(al, R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        le(0xE750_3211)
    ); // smmla   r0, r1, r2, r3
    assert_eq!(
        Smmls_A1(al, R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        le(0xE750_32D1)
    ); // smmls   r0, r1, r2, r3
    assert_eq!(
        Smlald_A1(al, R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        le(0xE741_0312)
    ); // smlald  r0, r1, r2, r3
}

#[test]
fn round_trip__a32_signed_multiply() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        Smla_A1(al, R::R0, R::R1, R::R2, R::R3, false, false),
        Smla_A1(al, R::R0, R::R1, R::R2, R::R3, true, false),
        Smla_A1(al, R::R0, R::R1, R::R2, R::R3, false, true),
        Smla_A1(al, R::R0, R::R1, R::R2, R::R3, true, true),
        Smlaw_A1(al, R::R4, R::R5, R::R6, R::R7, true),
        Smulw_A1(al, R::R8, R::R9, R::R10, false),
        Smlal_Halfword_A1(al, R::R0, R::R1, R::R2, R::R3, true, false),
        Smul_A1(al, R::R0, R::R1, R::R2, false, true),
        Smlad_A1(al, R::R0, R::R1, R::R2, R::R3, true),
        Smuad_A1(al, R::R4, R::R5, R::R6, false),
        Smlsd_A1(al, R::R0, R::R1, R::R2, R::R3, true),
        Smusd_A1(al, R::R7, R::R8, R::R9, false),
        Smmla_A1(al, R::R0, R::R1, R::R2, R::R3, true),
        Smmul_A1(al, R::R4, R::R5, R::R6, false),
        Smmls_A1(al, R::R0, R::R1, R::R2, R::R3, true),
        Smlald_A1(al, R::R0, R::R1, R::R2, R::R3, true),
        Smlsld_A1(Cond::NotEqual, R::R0, R::R1, R::R2, R::R3, false),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 signed-multiply round-trip mismatch"
        );
    }
}

// ---- A4: parallel (packed SIMD) add/sub + SEL ----

#[test]
fn encode__a32_parallel_and_sel_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(
        ParallelAddSub_A1(al, POp::Add16, PPfx::Signed, R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        le(0xE611_0F12)
    ); // sadd16  r0, r1, r2
    assert_eq!(
        ParallelAddSub_A1(al, POp::Sub8, PPfx::UnsignedSaturating, R::R3, R::R4, R::R5)
            .encode()
            .unwrap(),
        le(0xE664_3FF5)
    ); // uqsub8  r3, r4, r5
    assert_eq!(
        ParallelAddSub_A1(al, POp::Add8, PPfx::SignedHalving, R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        le(0xE631_0F92)
    ); // shadd8  r0, r1, r2
    assert_eq!(
        ParallelAddSub_A1(al, POp::Asx, PPfx::Unsigned, R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        le(0xE651_0F32)
    ); // uasx    r0, r1, r2
    assert_eq!(
        Sel_A1(al, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE681_0FB2)
    ); // sel     r0, r1, r2
}

#[test]
fn round_trip__a32_parallel_and_sel() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let ops = [
        POp::Add16,
        POp::Asx,
        POp::Sax,
        POp::Sub16,
        POp::Add8,
        POp::Sub8,
    ];
    let prefixes = [
        PPfx::Signed,
        PPfx::SignedSaturating,
        PPfx::SignedHalving,
        PPfx::Unsigned,
        PPfx::UnsignedSaturating,
        PPfx::UnsignedHalving,
    ];
    for op in ops {
        for prefix in prefixes {
            let instruction = ParallelAddSub_A1(al, op, prefix, R::R0, R::R1, R::R2);
            let bytes = instruction.encode().unwrap();
            let mut offset = 0;
            let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
                .unwrap()
                .unwrap();
            assert_eq!(offset, 4, "decode consumed wrong byte count");
            assert_eq!(
                decoded, instruction,
                "A32 parallel round-trip mismatch ({:?} {:?})",
                op, prefix
            );
        }
    }
    let sel = Sel_A1(Cond::NotEqual, R::R3, R::R4, R::R5);
    let bytes = sel.encode().unwrap();
    let mut offset = 0;
    assert_eq!(
        ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap(),
        sel,
        "SEL round-trip mismatch"
    );
}

// ---- A5: extend / reverse / CLZ ----

#[test]
fn encode__a32_extend_reverse_clz_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(
        Extend_A1(al, Ext::Sxtb, R::R0, R::R1, 0).encode().unwrap(),
        le(0xE6AF_0071)
    ); // sxtb   r0, r1
    assert_eq!(
        Extend_A1(al, Ext::Sxtb, R::R0, R::R1, 8).encode().unwrap(),
        le(0xE6AF_0471)
    ); // sxtb   r0, r1, ror #8
    assert_eq!(
        Extend_A1(al, Ext::Sxth, R::R0, R::R1, 0).encode().unwrap(),
        le(0xE6BF_0071)
    ); // sxth   r0, r1
    assert_eq!(
        Extend_A1(al, Ext::Uxth, R::R3, R::R4, 0).encode().unwrap(),
        le(0xE6FF_3074)
    ); // uxth   r3, r4
    assert_eq!(
        Extend_A1(al, Ext::Uxtb16, R::R0, R::R1, 0)
            .encode()
            .unwrap(),
        le(0xE6CF_0071)
    ); // uxtb16 r0, r1
    assert_eq!(
        ExtendAndAdd_A1(al, Ext::Sxtb, R::R0, R::R1, R::R2, 0)
            .encode()
            .unwrap(),
        le(0xE6A1_0072)
    ); // sxtab r0, r1, r2
    assert_eq!(Rev_A1(al, R::R0, R::R1).encode().unwrap(), le(0xE6BF_0F31)); // rev    r0, r1
    assert_eq!(
        Rev16_A1(al, R::R2, R::R3).encode().unwrap(),
        le(0xE6BF_2FB3)
    ); // rev16  r2, r3
    assert_eq!(
        Revsh_A1(al, R::R4, R::R5).encode().unwrap(),
        le(0xE6FF_4FB5)
    ); // revsh  r4, r5
    assert_eq!(Rbit_A1(al, R::R6, R::R7).encode().unwrap(), le(0xE6FF_6F37)); // rbit   r6, r7
    assert_eq!(Clz_A1(al, R::R0, R::R1).encode().unwrap(), le(0xE16F_0F11)); // clz    r0, r1
}

#[test]
fn round_trip__a32_extend_reverse_clz() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let types = [
        Ext::Sxtb16,
        Ext::Sxtb,
        Ext::Sxth,
        Ext::Uxtb16,
        Ext::Uxtb,
        Ext::Uxth,
    ];
    let mut instructions = Vec::new();
    for ty in types {
        for rot in [0u8, 8, 16, 24] {
            instructions.push(Extend_A1(al, ty, R::R0, R::R1, rot));
            instructions.push(ExtendAndAdd_A1(al, ty, R::R2, R::R3, R::R4, rot));
        }
    }
    instructions.push(Rev_A1(al, R::R0, R::R1));
    instructions.push(Rev16_A1(Cond::NotEqual, R::R2, R::R3));
    instructions.push(Revsh_A1(al, R::R4, R::R5));
    instructions.push(Rbit_A1(al, R::R6, R::R7));
    instructions.push(Clz_A1(al, R::R8, R::R9));
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 extend/reverse/clz round-trip mismatch"
        );
    }
}

// ---- A5: pack / saturate / sum-of-absolute-differences ----

#[test]
fn encode__a32_pack_saturate_sad_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(
        Pkhbt_A1(al, R::R0, R::R1, R::R2, 0).encode().unwrap(),
        le(0xE681_0012)
    ); // pkhbt  r0, r1, r2
    assert_eq!(
        Pkhbt_A1(al, R::R0, R::R1, R::R2, 4).encode().unwrap(),
        le(0xE681_0212)
    ); // pkhbt  r0, r1, r2, lsl #4
    assert_eq!(
        Pkhtb_A1(al, R::R0, R::R1, R::R2, 1).encode().unwrap(),
        le(0xE681_00D2)
    ); // pkhtb  r0, r1, r2, asr #1
    assert_eq!(
        Ssat_A1(al, R::R0, 1, R::R1, Shift::Lsl(0))
            .encode()
            .unwrap(),
        le(0xE6A0_0011)
    ); // ssat   r0, #1, r1
    assert_eq!(
        Ssat_A1(al, R::R0, 32, R::R1, Shift::Asr(1))
            .encode()
            .unwrap(),
        le(0xE6BF_00D1)
    ); // ssat   r0, #32, r1, asr #1
    assert_eq!(
        Usat_A1(al, R::R0, 0, R::R1, Shift::Lsl(0))
            .encode()
            .unwrap(),
        le(0xE6E0_0011)
    ); // usat   r0, #0, r1
    assert_eq!(
        Usat_A1(al, R::R3, 15, R::R4, Shift::Lsl(5))
            .encode()
            .unwrap(),
        le(0xE6EF_3294)
    ); // usat   r3, #15, r4, lsl #5
    assert_eq!(
        Ssat16_A1(al, R::R0, 1, R::R1).encode().unwrap(),
        le(0xE6A0_0F31)
    ); // ssat16 r0, #1, r1
    assert_eq!(
        Usat16_A1(al, R::R2, 15, R::R3).encode().unwrap(),
        le(0xE6EF_2F33)
    ); // usat16 r2, #15, r3
    assert_eq!(
        Usad8_A1(al, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE780_F211)
    ); // usad8  r0, r1, r2
    assert_eq!(
        Usada8_A1(al, R::R0, R::R1, R::R2, R::R3).encode().unwrap(),
        le(0xE780_3211)
    ); // usada8 r0, r1, r2, r3
}

#[test]
fn round_trip__a32_pack_saturate_sad() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        Pkhbt_A1(al, R::R0, R::R1, R::R2, 0),
        Pkhbt_A1(al, R::R3, R::R4, R::R5, 31),
        Pkhtb_A1(al, R::R6, R::R7, R::R8, 1),
        Pkhtb_A1(al, R::R9, R::R10, R::R11, 32),
        Ssat_A1(al, R::R0, 1, R::R1, Shift::Lsl(0)),
        Ssat_A1(al, R::R2, 32, R::R3, Shift::Lsl(31)),
        Ssat_A1(al, R::R4, 16, R::R5, Shift::Asr(1)),
        Ssat_A1(al, R::R6, 8, R::R7, Shift::Asr(32)),
        Usat_A1(al, R::R0, 0, R::R1, Shift::Lsl(0)),
        Usat_A1(al, R::R2, 31, R::R3, Shift::Asr(31)),
        Ssat16_A1(al, R::R0, 1, R::R1),
        Ssat16_A1(al, R::R2, 16, R::R3),
        Usat16_A1(al, R::R4, 0, R::R5),
        Usat16_A1(al, R::R6, 15, R::R7),
        Usad8_A1(Cond::NotEqual, R::R0, R::R1, R::R2),
        Usada8_A1(al, R::R3, R::R4, R::R5, R::R6),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 pack/saturate/sad round-trip mismatch"
        );
    }
}

// ---- A6: bitfield ----

#[test]
fn encode__a32_bitfield_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(Bfc_A1(al, R::R0, 0, 1).encode().unwrap(), le(0xE7C0_001F)); // bfc  r0, #0, #1
    assert_eq!(
        Bfi_A1(al, R::R2, R::R3, 4, 8).encode().unwrap(),
        le(0xE7CB_2213)
    ); // bfi  r2, r3, #4, #8
    assert_eq!(
        Sbfx_A1(al, R::R2, R::R3, 4, 8).encode().unwrap(),
        le(0xE7A7_2253)
    ); // sbfx r2, r3, #4, #8
    assert_eq!(
        Ubfx_A1(al, R::R0, R::R1, 0, 32).encode().unwrap(),
        le(0xE7FF_0051)
    ); // ubfx r0, r1, #0, #32
}

#[test]
fn round_trip__a32_bitfield() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        Bfc_A1(al, R::R0, 0, 32),
        Bfc_A1(al, R::R1, 31, 1),
        Bfi_A1(al, R::R2, R::R3, 4, 8),
        Bfi_A1(Cond::NotEqual, R::R5, R::R6, 0, 32),
        Sbfx_A1(al, R::R0, R::R1, 0, 1),
        Sbfx_A1(al, R::R2, R::R3, 31, 1),
        Ubfx_A1(al, R::R4, R::R5, 8, 16),
        Ubfx_A1(al, R::R6, R::R7, 0, 32),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(decoded, instruction, "A32 bitfield round-trip mismatch");
    }
}

// ---- A7: load/store single (word/byte) ----

#[test]
fn encode__a32_load_store_word_byte_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let imm = |add, imm12| Mem::Immediate { add, imm12 };
    assert_eq!(
        Ldr_A1(al, R::R0, R::R1, imm(true, 0), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE591_0000)
    ); // ldr  r0, [r1]
    assert_eq!(
        Ldr_A1(al, R::R0, R::R1, imm(true, 4), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE591_0004)
    ); // ldr  r0, [r1, #4]
    assert_eq!(
        Ldr_A1(al, R::R0, R::R1, imm(false, 4), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE511_0004)
    ); // ldr  r0, [r1, #-4]
    assert_eq!(
        Str_A1(al, R::R2, R::R3, imm(true, 8), Idx::PreIndex)
            .encode()
            .unwrap(),
        le(0xE5A3_2008)
    ); // str  r2, [r3, #8]!
    assert_eq!(
        Ldr_A1(al, R::R0, R::R1, imm(true, 4), Idx::PostIndex)
            .encode()
            .unwrap(),
        le(0xE491_0004)
    ); // ldr  r0, [r1], #4
    assert_eq!(
        Ldrb_A1(
            al,
            R::R0,
            R::R1,
            Mem::Register {
                add: true,
                rm: R::R2,
                shift: Shift::Lsl(0)
            },
            Idx::Offset
        )
        .encode()
        .unwrap(),
        le(0xE7D1_0002)
    ); // ldrb r0, [r1, r2]
    assert_eq!(
        Ldr_A1(
            al,
            R::R0,
            R::R1,
            Mem::Register {
                add: true,
                rm: R::R2,
                shift: Shift::Lsl(2)
            },
            Idx::Offset
        )
        .encode()
        .unwrap(),
        le(0xE791_0102)
    ); // ldr  r0, [r1, r2, lsl #2]
    assert_eq!(
        Ldr_A1(al, R::R0, R::R15, imm(true, 8), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE59F_0008)
    ); // ldr  r0, [pc, #8]  (literal)
    assert_eq!(
        Ldrt_A1(al, R::R0, R::R1, imm(true, 4)).encode().unwrap(),
        le(0xE4B1_0004)
    ); // ldrt r0, [r1], #4
}

#[test]
fn round_trip__a32_load_store_word_byte() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let imm_offsets = [
        Mem::Immediate {
            add: true,
            imm12: 0,
        },
        Mem::Immediate {
            add: true,
            imm12: 4095,
        },
        Mem::Immediate {
            add: false,
            imm12: 100,
        },
    ];
    let reg_offsets = [
        Mem::Register {
            add: true,
            rm: R::R2,
            shift: Shift::Lsl(0),
        },
        Mem::Register {
            add: false,
            rm: R::R3,
            shift: Shift::Asr(31),
        },
        Mem::Register {
            add: true,
            rm: R::R4,
            shift: Shift::Rrx,
        },
    ];
    let modes = [Idx::Offset, Idx::PreIndex, Idx::PostIndex];
    let mut instructions = Vec::new();
    for off in imm_offsets.iter().chain(reg_offsets.iter()) {
        for mode in modes {
            instructions.push(Ldr_A1(al, R::R0, R::R1, *off, mode));
            instructions.push(Str_A1(al, R::R5, R::R6, *off, mode));
            instructions.push(Ldrb_A1(al, R::R7, R::R8, *off, mode));
            instructions.push(Strb_A1(al, R::R9, R::R10, *off, mode));
        }
        instructions.push(Ldrt_A1(al, R::R0, R::R1, *off));
        instructions.push(Strt_A1(al, R::R2, R::R3, *off));
        instructions.push(Ldrbt_A1(al, R::R4, R::R5, *off));
        instructions.push(Strbt_A1(al, R::R6, R::R7, *off));
    }
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 load/store word/byte round-trip mismatch"
        );
    }
}

// ---- A8: load/store halfword / dual / signed ----

#[test]
fn encode__a32_load_store_halfword_dual_signed_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let imm = |add, imm8| Mem8::Immediate { add, imm8 };
    assert_eq!(
        Ldrh_A1(al, R::R0, R::R1, imm(true, 0), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE1D1_00B0)
    ); // ldrh  r0, [r1]
    assert_eq!(
        Ldrh_A1(al, R::R0, R::R1, imm(true, 4), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE1D1_00B4)
    ); // ldrh  r0, [r1, #4]
    assert_eq!(
        Ldrh_A1(al, R::R0, R::R1, imm(false, 8), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE151_00B8)
    ); // ldrh  r0, [r1, #-8]
    assert_eq!(
        Strh_A1(al, R::R2, R::R3, imm(true, 16), Idx::PreIndex)
            .encode()
            .unwrap(),
        le(0xE1E3_21B0)
    ); // strh  r2, [r3, #16]!
    assert_eq!(
        Ldrsb_A1(
            al,
            R::R0,
            R::R1,
            Mem8::Register {
                add: true,
                rm: R::R2
            },
            Idx::Offset
        )
        .encode()
        .unwrap(),
        le(0xE191_00D2)
    ); // ldrsb r0, [r1, r2]
    assert_eq!(
        Ldrsh_A1(al, R::R4, R::R5, imm(true, 10), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE1D5_40FA)
    ); // ldrsh r4, [r5, #10]
    assert_eq!(
        Ldrd_A1(al, R::R0, R::R1, imm(true, 8), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE1C1_00D8)
    ); // ldrd  r0, r1, [r1, #8]
    assert_eq!(
        Strd_A1(al, R::R2, R::R3, imm(true, 16), Idx::Offset)
            .encode()
            .unwrap(),
        le(0xE1C3_21F0)
    ); // strd  r2, r3, [r3, #16]
    assert_eq!(
        Ldrht_A1(al, R::R0, R::R1, imm(true, 4)).encode().unwrap(),
        le(0xE0F1_00B4)
    ); // ldrht r0, [r1], #4
}

#[test]
fn round_trip__a32_load_store_halfword_dual_signed() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let imm_offsets = [
        Mem8::Immediate { add: true, imm8: 0 },
        Mem8::Immediate {
            add: true,
            imm8: 255,
        },
        Mem8::Immediate {
            add: false,
            imm8: 50,
        },
    ];
    let reg_offsets = [
        Mem8::Register {
            add: true,
            rm: R::R2,
        },
        Mem8::Register {
            add: false,
            rm: R::R3,
        },
    ];
    let modes = [Idx::Offset, Idx::PreIndex, Idx::PostIndex];
    let mut instructions = Vec::new();
    for off in imm_offsets.iter().chain(reg_offsets.iter()) {
        for mode in modes {
            instructions.push(Ldrh_A1(al, R::R0, R::R1, *off, mode));
            instructions.push(Strh_A1(al, R::R4, R::R5, *off, mode));
            instructions.push(Ldrsb_A1(al, R::R6, R::R7, *off, mode));
            instructions.push(Ldrsh_A1(al, R::R8, R::R9, *off, mode));
            instructions.push(Ldrd_A1(al, R::R0, R::R10, *off, mode));
            instructions.push(Strd_A1(al, R::R2, R::R11, *off, mode));
        }
        instructions.push(Ldrht_A1(al, R::R0, R::R1, *off));
        instructions.push(Strht_A1(al, R::R2, R::R3, *off));
        instructions.push(Ldrsbt_A1(al, R::R4, R::R5, *off));
        instructions.push(Ldrsht_A1(al, R::R6, R::R7, *off));
    }
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 ls halfword/dual/signed round-trip mismatch"
        );
    }
}

// ---- A9: load/store multiple ----

#[test]
fn encode__a32_load_store_multiple_exact_bytes() {
    use crate::Arm32BlockAddressMode::*;
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(
        Ldm_A1(
            al,
            IncrementAfter,
            R::R0,
            false,
            false,
            vec![R::R1, R::R2, R::R3]
        )
        .encode()
        .unwrap(),
        le(0xE890_000E)
    ); // ldm   r0, {r1, r2, r3}
    assert_eq!(
        Ldm_A1(
            al,
            IncrementAfter,
            R::R0,
            true,
            false,
            vec![R::R1, R::R2, R::R3]
        )
        .encode()
        .unwrap(),
        le(0xE8B0_000E)
    ); // ldm   r0!, {r1, r2, r3}
    assert_eq!(
        Stm_A1(
            al,
            DecrementBefore,
            R::R13,
            true,
            false,
            vec![R::R4, R::R5, R::R14]
        )
        .encode()
        .unwrap(),
        le(0xE92D_4030)
    ); // push  {r4, r5, lr}
    assert_eq!(
        Ldm_A1(
            al,
            IncrementAfter,
            R::R13,
            true,
            false,
            vec![R::R4, R::R5, R::R15]
        )
        .encode()
        .unwrap(),
        le(0xE8BD_8030)
    ); // pop   {r4, r5, pc}
    assert_eq!(
        Ldm_A1(al, IncrementBefore, R::R0, false, false, vec![R::R1])
            .encode()
            .unwrap(),
        le(0xE990_0002)
    ); // ldmib r0, {r1}
    assert_eq!(
        Ldm_A1(al, DecrementAfter, R::R0, false, false, vec![R::R1])
            .encode()
            .unwrap(),
        le(0xE810_0002)
    ); // ldmda r0, {r1}
    assert_eq!(
        Ldm_A1(al, IncrementAfter, R::R0, false, true, vec![R::R1])
            .encode()
            .unwrap(),
        le(0xE8D0_0002)
    ); // ldm   r0, {r1}^
}

#[test]
fn round_trip__a32_load_store_multiple() {
    use crate::Arm32BlockAddressMode::*;
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let modes = [
        IncrementAfter,
        IncrementBefore,
        DecrementAfter,
        DecrementBefore,
    ];
    let lists = [
        vec![R::R0],
        vec![R::R1, R::R2, R::R3],
        vec![R::R0, R::R7, R::R8, R::R15],
        vec![R::R4, R::R5, R::R13, R::R14],
    ];
    let mut instructions = Vec::new();
    for mode in modes {
        for list in lists.iter() {
            for wb in [false, true] {
                for user in [false, true] {
                    instructions.push(Ldm_A1(al, mode, R::R6, wb, user, list.clone()));
                    instructions.push(Stm_A1(al, mode, R::R9, wb, user, list.clone()));
                }
            }
        }
    }
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 load/store-multiple round-trip mismatch"
        );
    }
}

// ---- A10: synchronization ----

#[test]
fn encode__a32_synchronization_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(
        Ldrex_A1(al, R::R0, R::R1).encode().unwrap(),
        le(0xE191_0F9F)
    ); // ldrex   r0, [r1]
    assert_eq!(
        Strex_A1(al, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE182_0F91)
    ); // strex   r0, r1, [r2]
    assert_eq!(
        Ldrexb_A1(al, R::R0, R::R1).encode().unwrap(),
        le(0xE1D1_0F9F)
    ); // ldrexb  r0, [r1]
    assert_eq!(
        Strexb_A1(al, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE1C2_0F91)
    ); // strexb r0, r1, [r2]
    assert_eq!(
        Ldrexd_A1(al, R::R0, R::R1).encode().unwrap(),
        le(0xE1B1_0F9F)
    ); // ldrexd  r0, r1, [r1]
    assert_eq!(
        Ldrexh_A1(al, R::R0, R::R1).encode().unwrap(),
        le(0xE1F1_0F9F)
    ); // ldrexh  r0, [r1]
    assert_eq!(
        Swp_A1(al, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE102_0091)
    ); // swp     r0, r1, [r2]
    assert_eq!(
        Swpb_A1(al, R::R0, R::R1, R::R2).encode().unwrap(),
        le(0xE142_0091)
    ); // swpb    r0, r1, [r2]
    assert_eq!(Clrex_A1.encode().unwrap(), le(0xF57F_F01F)); // clrex
}

#[test]
fn round_trip__a32_synchronization() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        Ldrex_A1(al, R::R0, R::R1),
        Strex_A1(Cond::NotEqual, R::R2, R::R3, R::R4),
        Ldrexb_A1(al, R::R5, R::R6),
        Strexb_A1(al, R::R7, R::R8, R::R9),
        Ldrexh_A1(al, R::R10, R::R11),
        Strexh_A1(al, R::R0, R::R1, R::R2),
        Ldrexd_A1(al, R::R0, R::R3),
        Strexd_A1(al, R::R4, R::R5, R::R6),
        Clrex_A1,
        Swp_A1(al, R::R0, R::R1, R::R2),
        Swpb_A1(Cond::CarrySet, R::R3, R::R4, R::R5),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 synchronization round-trip mismatch"
        );
    }
}

// ---- A11: branch / interwork ----

#[test]
fn encode__a32_branch_exact_bytes() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(B_A1(al, 8).encode().unwrap(), le(0xEA00_0002)); // b   .+8
    assert_eq!(B_A1(al, -8).encode().unwrap(), le(0xEAFF_FFFE)); // b   .-8
    assert_eq!(Bl_A1(al, 0).encode().unwrap(), le(0xEB00_0000)); // bl  .+0
    assert_eq!(Bl_A1(al, 0x1000).encode().unwrap(), le(0xEB00_0400)); // bl  .+0x1000
    assert_eq!(Blx_Immediate_A1(4).encode().unwrap(), le(0xFA00_0001)); // blx .+4
    assert_eq!(Blx_Immediate_A1(6).encode().unwrap(), le(0xFB00_0001)); // blx .+6  (H=1)
    assert_eq!(
        Blx_Register_A1(al, R::R0).encode().unwrap(),
        le(0xE12F_FF30)
    ); // blx r0
    assert_eq!(Bxj_A1(al, R::R2).encode().unwrap(), le(0xE12F_FF22)); // bxj r2
    assert_eq!(B_A1(Cond::NotEqual, 12).encode().unwrap(), le(0x1A00_0003)); // bne .+12
}

#[test]
fn round_trip__a32_branch() {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        B_A1(al, 0),
        B_A1(al, 8),
        B_A1(al, -8),
        B_A1(al, (1 << 25) - 4),
        B_A1(al, -(1 << 25)),
        Bl_A1(Cond::Equal, 0x100),
        Bl_A1(al, -0x100),
        Blx_Immediate_A1(0),
        Blx_Immediate_A1(6),
        Blx_Immediate_A1(-6),
        Blx_Immediate_A1((1 << 25) - 2),
        Bx_A1(al, R::R1),
        Blx_Register_A1(al, R::R3),
        Bxj_A1(al, R::R5),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(decoded, instruction, "A32 branch round-trip mismatch");
    }
}

// ---- A12: status / system register access ----

#[test]
fn encode__a32_status_system_exact_bytes() {
    use crate::Arm32CpsMode::*;
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    assert_eq!(Mrs_A1(al, false, R::R0).encode().unwrap(), le(0xE10F_0000)); // mrs  r0, cpsr
    assert_eq!(Mrs_A1(al, true, R::R1).encode().unwrap(), le(0xE14F_1000)); // mrs  r1, spsr
    assert_eq!(
        Msr_Register_A1(al, false, 0b1000, R::R0).encode().unwrap(),
        le(0xE128_F000)
    ); // msr cpsr_f, r0
    assert_eq!(
        Msr_Register_A1(al, false, 0b1001, R::R1).encode().unwrap(),
        le(0xE129_F001)
    ); // msr cpsr_fc, r1
    assert_eq!(
        Msr_Register_A1(al, true, 0b1111, R::R2).encode().unwrap(),
        le(0xE16F_F002)
    ); // msr spsr_fsxc, r2
    assert_eq!(
        Msr_Immediate_A1(al, false, 0b1000, 0xF000_0000)
            .encode()
            .unwrap(),
        le(0xE328_F20F)
    ); // msr cpsr_f, #0xf0000000
    assert_eq!(
        Cps_A1(Enable, false, true, false, None).encode().unwrap(),
        le(0xF108_0080)
    ); // cpsie i
    assert_eq!(
        Cps_A1(Disable, false, true, true, None).encode().unwrap(),
        le(0xF10C_00C0)
    ); // cpsid if
    assert_eq!(
        Cps_A1(NoChange, false, false, false, Some(0x13))
            .encode()
            .unwrap(),
        le(0xF102_0013)
    ); // cps #0x13
    assert_eq!(Setend_A1(true).encode().unwrap(), le(0xF101_0200)); // setend be
    assert_eq!(Setend_A1(false).encode().unwrap(), le(0xF101_0000)); // setend le
}

#[test]
fn from_imod_bits__round_trips_masks_high_bits_and_rejects_reserved() {
    use crate::Arm32CpsMode::{self, Disable, Enable, NoChange};
    // the three valid imod values round-trip
    for mode in [NoChange, Enable, Disable] {
        assert_eq!(Arm32CpsMode::from_imod_bits(mode.imod_bits()), Some(mode));
    }
    // 0b01 is the reserved imod value -> None
    assert_eq!(Arm32CpsMode::from_imod_bits(0b01), None);
    // higher bits a caller might pass are masked off (`& 0b11`); only the low two bits decide:
    assert_eq!(Arm32CpsMode::from_imod_bits(0b100), Some(NoChange)); // 0b100 & 0b11 = 0b00
    assert_eq!(Arm32CpsMode::from_imod_bits(0b110), Some(Enable)); // 0b110 & 0b11 = 0b10
    assert_eq!(Arm32CpsMode::from_imod_bits(0b101), None); // 0b101 & 0b11 = 0b01 (reserved)
    assert_eq!(Arm32CpsMode::from_imod_bits(u32::MAX), Some(Disable)); // ...11 & 0b11 = 0b11
}

#[test]
fn round_trip__a32_status_system() {
    use crate::Arm32CpsMode::*;
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let instructions = [
        Mrs_A1(al, false, R::R0),
        Mrs_A1(Cond::NotEqual, true, R::R5),
        Msr_Register_A1(al, false, 0b1000, R::R0),
        Msr_Register_A1(al, true, 0b1111, R::R3),
        Msr_Register_A1(al, false, 0b0001, R::R7),
        Msr_Immediate_A1(al, false, 0b1000, 0xF000_0000),
        Msr_Immediate_A1(al, true, 0b1001, 0xFF),
        Cps_A1(Enable, true, true, true, None),
        Cps_A1(Disable, false, true, false, None),
        Cps_A1(Enable, true, false, false, Some(0x1F)),
        Cps_A1(NoChange, false, false, false, Some(0x10)),
        Setend_A1(true),
        Setend_A1(false),
    ];
    for instruction in instructions {
        let bytes = instruction.encode().unwrap();
        let mut offset = 0;
        let decoded = ArmA32Instruction::decode(&mut bytes.iter(), &mut offset)
            .unwrap()
            .unwrap();
        assert_eq!(offset, 4, "decode consumed wrong byte count");
        assert_eq!(
            decoded, instruction,
            "A32 status/system round-trip mismatch"
        );
    }
}

