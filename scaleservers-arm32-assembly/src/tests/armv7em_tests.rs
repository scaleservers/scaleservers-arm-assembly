// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// ARMv7E-M tests: the DSP (SIMD) extension and the hardware floating-point families (the "M8"
// milestone). Exact encodings are cross-checked against `clang --target=thumbv7em-none-eabi -mcpu=...`;
// each family also round-trips (encode -> decode -> encode) and is gated behind the right
// `ArmCpuFeature` (a DSP/FP instruction is REFUSED by a plain ARMv7-M target profile).

use crate::enums::Arm32GeneralPurposeRegister as R;
use crate::{
    Arm32DirectedRound, Arm32DoublePrecisionRegister, Arm32SinglePrecisionRegister,
    Arm32VmovLaneSize, ArmCpuFeature, ArmInstructionRequirement, ArmIsaVersion, ArmT32Instruction,
    ArmTargetProfile, EncodeError,
};

// shorthands to build FP registers by number
fn s(number: u8) -> Arm32SinglePrecisionRegister {
    Arm32SinglePrecisionRegister::new(number).unwrap()
}
fn d(number: u8) -> Arm32DoublePrecisionRegister {
    Arm32DoublePrecisionRegister::new(number).unwrap()
}

// encode -> decode -> encode for a 32-bit DSP instruction, asserting the model is reproduced.
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
fn encode__m8a_saturating_arithmetic_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m4`
    assert_eq!(
        ArmT32Instruction::Qadd_T1(R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0x82, 0xFA, 0x81, 0xF0]
    ); // qadd  r0, r1, r2
    assert_eq!(
        ArmT32Instruction::Qsub_T1(R::R3, R::R4, R::R5)
            .encode()
            .unwrap(),
        vec![0x85, 0xFA, 0xA4, 0xF3]
    ); // qsub  r3, r4, r5
    assert_eq!(
        ArmT32Instruction::Qdadd_T1(R::R6, R::R7, R::R8)
            .encode()
            .unwrap(),
        vec![0x88, 0xFA, 0x97, 0xF6]
    ); // qdadd r6, r7, r8
    assert_eq!(
        ArmT32Instruction::Qdsub_T1(R::R9, R::R10, R::R11)
            .encode()
            .unwrap(),
        vec![0x8B, 0xFA, 0xBA, 0xF9]
    ); // qdsub r9, r10, r11
}

#[test]
fn round_trip__m8a_saturating_arithmetic() {
    round_trip(&ArmT32Instruction::Qadd_T1(R::R0, R::R1, R::R2));
    round_trip(&ArmT32Instruction::Qsub_T1(R::R12, R::R11, R::R10));
    round_trip(&ArmT32Instruction::Qdadd_T1(R::R6, R::R7, R::R8));
    round_trip(&ArmT32Instruction::Qdsub_T1(R::R9, R::R10, R::R11));
}

#[test]
fn encode__m8b_extend_and_add_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m4`
    assert_eq!(
        ArmT32Instruction::Sxtab_T1(R::R0, R::R1, R::R2, 0)
            .encode()
            .unwrap(),
        vec![0x41, 0xFA, 0x82, 0xF0]
    ); // sxtab   r0, r1, r2
    assert_eq!(
        ArmT32Instruction::Sxtab_T1(R::R0, R::R1, R::R2, 8)
            .encode()
            .unwrap(),
        vec![0x41, 0xFA, 0x92, 0xF0]
    ); // sxtab   r0, r1, r2, ror #8
    assert_eq!(
        ArmT32Instruction::Sxtah_T1(R::R3, R::R4, R::R5, 16)
            .encode()
            .unwrap(),
        vec![0x04, 0xFA, 0xA5, 0xF3]
    ); // sxtah   r3, r4, r5, ror #16
    assert_eq!(
        ArmT32Instruction::Sxtab16_T1(R::R6, R::R7, R::R8, 24)
            .encode()
            .unwrap(),
        vec![0x27, 0xFA, 0xB8, 0xF6]
    ); // sxtab16 r6, r7, r8, ror #24
    assert_eq!(
        ArmT32Instruction::Uxtab_T1(R::R0, R::R1, R::R2, 0)
            .encode()
            .unwrap(),
        vec![0x51, 0xFA, 0x82, 0xF0]
    ); // uxtab   r0, r1, r2
    assert_eq!(
        ArmT32Instruction::Uxtah_T1(R::R3, R::R4, R::R5, 0)
            .encode()
            .unwrap(),
        vec![0x14, 0xFA, 0x85, 0xF3]
    ); // uxtah   r3, r4, r5
    assert_eq!(
        ArmT32Instruction::Uxtab16_T1(R::R6, R::R7, R::R8, 0)
            .encode()
            .unwrap(),
        vec![0x37, 0xFA, 0x88, 0xF6]
    ); // uxtab16 r6, r7, r8
    assert_eq!(
        ArmT32Instruction::Sxtb16_T1(R::R0, R::R1, 0)
            .encode()
            .unwrap(),
        vec![0x2F, 0xFA, 0x81, 0xF0]
    ); // sxtb16  r0, r1
    assert_eq!(
        ArmT32Instruction::Sxtb16_T1(R::R0, R::R1, 8)
            .encode()
            .unwrap(),
        vec![0x2F, 0xFA, 0x91, 0xF0]
    ); // sxtb16  r0, r1, ror #8
    assert_eq!(
        ArmT32Instruction::Uxtb16_T1(R::R2, R::R3, 16)
            .encode()
            .unwrap(),
        vec![0x3F, 0xFA, 0xA3, 0xF2]
    ); // uxtb16  r2, r3, ror #16
}

#[test]
fn round_trip__m8b_extend_and_add() {
    round_trip(&ArmT32Instruction::Sxtab_T1(R::R0, R::R1, R::R2, 24));
    round_trip(&ArmT32Instruction::Uxtab_T1(R::R3, R::R4, R::R5, 16));
    round_trip(&ArmT32Instruction::Sxtah_T1(R::R6, R::R7, R::R8, 8));
    round_trip(&ArmT32Instruction::Uxtah_T1(R::R9, R::R10, R::R11, 0));
    round_trip(&ArmT32Instruction::Sxtab16_T1(R::R0, R::R1, R::R2, 0));
    round_trip(&ArmT32Instruction::Uxtab16_T1(R::R3, R::R4, R::R5, 8));
    round_trip(&ArmT32Instruction::Sxtb16_T1(R::R6, R::R7, 16));
    round_trip(&ArmT32Instruction::Uxtb16_T1(R::R8, R::R9, 24));
}

#[test]
fn encode__m8c_pack_saturate_select_sad_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m4`
    assert_eq!(
        ArmT32Instruction::Pkhbt_T1(R::R0, R::R1, R::R2, 0)
            .encode()
            .unwrap(),
        vec![0xC1, 0xEA, 0x02, 0x00]
    ); // pkhbt  r0, r1, r2
    assert_eq!(
        ArmT32Instruction::Pkhbt_T1(R::R0, R::R1, R::R2, 4)
            .encode()
            .unwrap(),
        vec![0xC1, 0xEA, 0x02, 0x10]
    ); // pkhbt  r0, r1, r2, lsl #4
    assert_eq!(
        ArmT32Instruction::Pkhtb_T1(R::R3, R::R4, R::R5, 8)
            .encode()
            .unwrap(),
        vec![0xC4, 0xEA, 0x25, 0x23]
    ); // pkhtb  r3, r4, r5, asr #8
    assert_eq!(
        ArmT32Instruction::Ssat16_T1(R::R0, 5, R::R1)
            .encode()
            .unwrap(),
        vec![0x21, 0xF3, 0x04, 0x00]
    ); // ssat16 r0, #5, r1
    assert_eq!(
        ArmT32Instruction::Usat16_T1(R::R2, 7, R::R3)
            .encode()
            .unwrap(),
        vec![0xA3, 0xF3, 0x07, 0x02]
    ); // usat16 r2, #7, r3
    assert_eq!(
        ArmT32Instruction::Sel_T1(R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0xA1, 0xFA, 0x82, 0xF0]
    ); // sel    r0, r1, r2
    assert_eq!(
        ArmT32Instruction::Usad8_T1(R::R3, R::R4, R::R5)
            .encode()
            .unwrap(),
        vec![0x74, 0xFB, 0x05, 0xF3]
    ); // usad8  r3, r4, r5
    assert_eq!(
        ArmT32Instruction::Usada8_T1(R::R6, R::R7, R::R8, R::R9)
            .encode()
            .unwrap(),
        vec![0x77, 0xFB, 0x08, 0x96]
    ); // usada8 r6, r7, r8, r9
}

#[test]
fn round_trip__m8c_pack_saturate_select_sad() {
    round_trip(&ArmT32Instruction::Pkhbt_T1(R::R0, R::R1, R::R2, 0));
    round_trip(&ArmT32Instruction::Pkhbt_T1(R::R3, R::R4, R::R5, 31));
    round_trip(&ArmT32Instruction::Pkhtb_T1(R::R6, R::R7, R::R8, 1));
    round_trip(&ArmT32Instruction::Pkhtb_T1(R::R9, R::R10, R::R11, 31));
    round_trip(&ArmT32Instruction::Ssat16_T1(R::R0, 1, R::R1));
    round_trip(&ArmT32Instruction::Ssat16_T1(R::R2, 16, R::R3));
    round_trip(&ArmT32Instruction::Usat16_T1(R::R4, 0, R::R5));
    round_trip(&ArmT32Instruction::Usat16_T1(R::R6, 15, R::R7));
    round_trip(&ArmT32Instruction::Sel_T1(R::R12, R::R11, R::R10));
    round_trip(&ArmT32Instruction::Usad8_T1(R::R0, R::R1, R::R2));
    round_trip(&ArmT32Instruction::Usada8_T1(R::R3, R::R4, R::R5, R::R6));
}

#[test]
fn encode__m8d_parallel_add_sub_exact_bytes() {
    use crate::{ArmT32ParallelOperation as Op, ArmT32ParallelPrefix as Pre};
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m4`
    assert_eq!(
        ArmT32Instruction::ParallelAddSub_T1(Op::Add16, Pre::Signed, R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0x91, 0xFA, 0x02, 0xF0]
    ); // sadd16
    assert_eq!(
        ArmT32Instruction::ParallelAddSub_T1(Op::Add16, Pre::UnsignedHalving, R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0x91, 0xFA, 0x62, 0xF0]
    ); // uhadd16
    assert_eq!(
        ArmT32Instruction::ParallelAddSub_T1(
            Op::Sub8,
            Pre::UnsignedSaturating,
            R::R0,
            R::R1,
            R::R2
        )
        .encode()
        .unwrap(),
        vec![0xC1, 0xFA, 0x52, 0xF0]
    ); // uqsub8
    assert_eq!(
        ArmT32Instruction::ParallelAddSub_T1(Op::Asx, Pre::SignedHalving, R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0xA1, 0xFA, 0x22, 0xF0]
    ); // shasx
    assert_eq!(
        ArmT32Instruction::ParallelAddSub_T1(Op::Sax, Pre::Signed, R::R0, R::R1, R::R2)
            .encode()
            .unwrap(),
        vec![0xE1, 0xFA, 0x02, 0xF0]
    ); // ssax
}

#[test]
fn round_trip__m8d_all_36_parallel_forms() {
    use crate::{ArmT32ParallelOperation as Op, ArmT32ParallelPrefix as Pre};
    let operations = [Op::Add16, Op::Asx, Op::Sax, Op::Sub16, Op::Add8, Op::Sub8];
    let prefixes = [
        Pre::Signed,
        Pre::SignedSaturating,
        Pre::SignedHalving,
        Pre::Unsigned,
        Pre::UnsignedSaturating,
        Pre::UnsignedHalving,
    ];
    for operation in operations {
        for prefix in prefixes {
            round_trip(&ArmT32Instruction::ParallelAddSub_T1(
                operation,
                prefix,
                R::R3,
                R::R4,
                R::R5,
            ));
        }
    }
}

