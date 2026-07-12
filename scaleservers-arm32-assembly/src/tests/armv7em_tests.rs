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

#[test]
fn encode__m8e_signed_multiplies_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m4`
    assert_eq!(
        ArmT32Instruction::Smul_T1(R::R0, R::R1, R::R2, false, false)
            .encode()
            .unwrap(),
        vec![0x11, 0xFB, 0x02, 0xF0]
    ); // smulbb
    assert_eq!(
        ArmT32Instruction::Smul_T1(R::R0, R::R1, R::R2, true, true)
            .encode()
            .unwrap(),
        vec![0x11, 0xFB, 0x32, 0xF0]
    ); // smultt
    assert_eq!(
        ArmT32Instruction::Smulw_T1(R::R0, R::R1, R::R2, false)
            .encode()
            .unwrap(),
        vec![0x31, 0xFB, 0x02, 0xF0]
    ); // smulwb
    assert_eq!(
        ArmT32Instruction::Smla_T1(R::R0, R::R1, R::R2, R::R3, false, false)
            .encode()
            .unwrap(),
        vec![0x11, 0xFB, 0x02, 0x30]
    ); // smlabb
    assert_eq!(
        ArmT32Instruction::Smlal_Halfword_T1(R::R0, R::R1, R::R2, R::R3, false, false)
            .encode()
            .unwrap(),
        vec![0xC2, 0xFB, 0x83, 0x01]
    ); // smlalbb
    assert_eq!(
        ArmT32Instruction::Smuad_T1(R::R0, R::R1, R::R2, false)
            .encode()
            .unwrap(),
        vec![0x21, 0xFB, 0x02, 0xF0]
    ); // smuad
    assert_eq!(
        ArmT32Instruction::Smuad_T1(R::R0, R::R1, R::R2, true)
            .encode()
            .unwrap(),
        vec![0x21, 0xFB, 0x12, 0xF0]
    ); // smuadx
    assert_eq!(
        ArmT32Instruction::Smlad_T1(R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        vec![0x21, 0xFB, 0x02, 0x30]
    ); // smlad
    assert_eq!(
        ArmT32Instruction::Smlald_T1(R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        vec![0xC2, 0xFB, 0xC3, 0x01]
    ); // smlald
    assert_eq!(
        ArmT32Instruction::Smlsld_T1(R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        vec![0xD2, 0xFB, 0xC3, 0x01]
    ); // smlsld
    assert_eq!(
        ArmT32Instruction::Smmul_T1(R::R0, R::R1, R::R2, false)
            .encode()
            .unwrap(),
        vec![0x51, 0xFB, 0x02, 0xF0]
    ); // smmul
    assert_eq!(
        ArmT32Instruction::Smmla_T1(R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        vec![0x51, 0xFB, 0x02, 0x30]
    ); // smmla
    assert_eq!(
        ArmT32Instruction::Smmls_T1(R::R0, R::R1, R::R2, R::R3, false)
            .encode()
            .unwrap(),
        vec![0x61, 0xFB, 0x02, 0x30]
    ); // smmls
}

#[test]
fn round_trip__m8e_signed_multiplies() {
    for (n, m) in [(false, false), (false, true), (true, false), (true, true)] {
        round_trip(&ArmT32Instruction::Smul_T1(R::R0, R::R1, R::R2, n, m));
        round_trip(&ArmT32Instruction::Smla_T1(
            R::R3,
            R::R4,
            R::R5,
            R::R6,
            n,
            m,
        ));
        round_trip(&ArmT32Instruction::Smlal_Halfword_T1(
            R::R7,
            R::R8,
            R::R9,
            R::R10,
            n,
            m,
        ));
    }
    for m in [false, true] {
        round_trip(&ArmT32Instruction::Smulw_T1(R::R0, R::R1, R::R2, m));
        round_trip(&ArmT32Instruction::Smlaw_T1(R::R3, R::R4, R::R5, R::R6, m));
    }
    for x in [false, true] {
        round_trip(&ArmT32Instruction::Smuad_T1(R::R0, R::R1, R::R2, x));
        round_trip(&ArmT32Instruction::Smusd_T1(R::R3, R::R4, R::R5, x));
        round_trip(&ArmT32Instruction::Smlad_T1(R::R0, R::R1, R::R2, R::R3, x));
        round_trip(&ArmT32Instruction::Smlsd_T1(R::R4, R::R5, R::R6, R::R7, x));
        round_trip(&ArmT32Instruction::Smlald_T1(R::R0, R::R1, R::R2, R::R3, x));
        round_trip(&ArmT32Instruction::Smlsld_T1(R::R4, R::R5, R::R6, R::R7, x));
    }
    for r in [false, true] {
        round_trip(&ArmT32Instruction::Smmul_T1(R::R0, R::R1, R::R2, r));
        round_trip(&ArmT32Instruction::Smmla_T1(R::R3, R::R4, R::R5, R::R6, r));
        round_trip(&ArmT32Instruction::Smmls_T1(R::R7, R::R8, R::R9, R::R10, r));
    }
}

#[test]
fn encode__m8f_fp_load_store_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m7 -mfpu=fpv5-d16`
    assert_eq!(
        ArmT32Instruction::Vldr_Single_T2(s(0), R::R0, 0)
            .encode()
            .unwrap(),
        vec![0x90, 0xED, 0x00, 0x0A]
    ); // vldr s0, [r0]
    assert_eq!(
        ArmT32Instruction::Vldr_Single_T2(s(1), R::R0, 0)
            .encode()
            .unwrap(),
        vec![0xD0, 0xED, 0x00, 0x0A]
    ); // vldr s1, [r0]
    assert_eq!(
        ArmT32Instruction::Vldr_Single_T2(s(15), R::R1, 4)
            .encode()
            .unwrap(),
        vec![0xD1, 0xED, 0x01, 0x7A]
    ); // vldr s15, [r1, #4]
    assert_eq!(
        ArmT32Instruction::Vldr_Single_T2(s(31), R::R2, -8)
            .encode()
            .unwrap(),
        vec![0x52, 0xED, 0x02, 0xFA]
    ); // vldr s31, [r2, #-8]
    assert_eq!(
        ArmT32Instruction::Vstr_Single_T2(s(0), R::R0, 1020)
            .encode()
            .unwrap(),
        vec![0x80, 0xED, 0xFF, 0x0A]
    ); // vstr s0, [r0, #1020]
    assert_eq!(
        ArmT32Instruction::Vldr_Double_T1(d(0), R::R0, 0)
            .encode()
            .unwrap(),
        vec![0x90, 0xED, 0x00, 0x0B]
    ); // vldr d0, [r0]
    assert_eq!(
        ArmT32Instruction::Vldr_Double_T1(d(15), R::R3, 16)
            .encode()
            .unwrap(),
        vec![0x93, 0xED, 0x04, 0xFB]
    ); // vldr d15, [r3, #16]
    assert_eq!(
        ArmT32Instruction::Vstr_Double_T1(d(5), R::R4, -256)
            .encode()
            .unwrap(),
        vec![0x04, 0xED, 0x40, 0x5B]
    ); // vstr d5, [r4, #-256]
}

#[test]
fn round_trip__m8f_fp_load_store() {
    for number in [0u8, 1, 2, 15, 16, 30, 31] {
        round_trip(&ArmT32Instruction::Vldr_Single_T2(s(number), R::R1, 0));
        round_trip(&ArmT32Instruction::Vstr_Single_T2(s(number), R::R2, -1020));
    }
    for number in 0u8..=15 {
        round_trip(&ArmT32Instruction::Vldr_Double_T1(d(number), R::R3, 1020));
        round_trip(&ArmT32Instruction::Vstr_Double_T1(d(number), R::R4, 8));
    }
}

#[test]
fn encode__m8g_fp_load_store_multiple_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m7 -mfpu=fpv5-d16`
    assert_eq!(
        ArmT32Instruction::Vldm_Single_T2(R::R0, false, false, s(0), 4)
            .encode()
            .unwrap(),
        vec![0x90, 0xEC, 0x04, 0x0A]
    ); // vldmia r0, {s0-s3}
    assert_eq!(
        ArmT32Instruction::Vldm_Single_T2(R::R0, true, false, s(4), 4)
            .encode()
            .unwrap(),
        vec![0xB0, 0xEC, 0x04, 0x2A]
    ); // vldmia r0!, {s4-s7}
    assert_eq!(
        ArmT32Instruction::Vstm_Single_T2(R::R1, true, false, s(0), 1)
            .encode()
            .unwrap(),
        vec![0xA1, 0xEC, 0x01, 0x0A]
    ); // vstmia r1!, {s0}
    assert_eq!(
        ArmT32Instruction::Vldm_Single_T2(R::R2, true, true, s(8), 2)
            .encode()
            .unwrap(),
        vec![0x32, 0xED, 0x02, 0x4A]
    ); // vldmdb r2!, {s8-s9}
    assert_eq!(
        ArmT32Instruction::Vstm_Single_T2(R::R13, true, true, s(0), 4)
            .encode()
            .unwrap(),
        vec![0x2D, 0xED, 0x04, 0x0A]
    ); // vpush  {s0-s3}
    assert_eq!(
        ArmT32Instruction::Vldm_Single_T2(R::R13, true, false, s(0), 4)
            .encode()
            .unwrap(),
        vec![0xBD, 0xEC, 0x04, 0x0A]
    ); // vpop   {s0-s3}
    assert_eq!(
        ArmT32Instruction::Vldm_Double_T1(R::R0, false, false, d(0), 2)
            .encode()
            .unwrap(),
        vec![0x90, 0xEC, 0x04, 0x0B]
    ); // vldmia r0, {d0-d1}
    assert_eq!(
        ArmT32Instruction::Vstm_Double_T1(R::R3, true, true, d(5), 3)
            .encode()
            .unwrap(),
        vec![0x23, 0xED, 0x06, 0x5B]
    ); // vstmdb r3!, {d5-d7}
    assert_eq!(
        ArmT32Instruction::Vstm_Double_T1(R::R13, true, true, d(0), 4)
            .encode()
            .unwrap(),
        vec![0x2D, 0xED, 0x08, 0x0B]
    ); // vpush  {d0-d3}
}

#[test]
fn round_trip__m8g_fp_load_store_multiple() {
    round_trip(&ArmT32Instruction::Vldm_Single_T2(
        R::R0,
        false,
        false,
        s(0),
        4,
    ));
    round_trip(&ArmT32Instruction::Vstm_Single_T2(
        R::R1,
        true,
        false,
        s(4),
        8,
    ));
    round_trip(&ArmT32Instruction::Vldm_Single_T2(
        R::R2,
        true,
        true,
        s(16),
        16,
    ));
    round_trip(&ArmT32Instruction::Vstm_Single_T2(
        R::R13,
        true,
        true,
        s(0),
        1,
    ));
    round_trip(&ArmT32Instruction::Vldm_Double_T1(
        R::R0,
        false,
        false,
        d(0),
        2,
    ));
    round_trip(&ArmT32Instruction::Vstm_Double_T1(
        R::R3,
        true,
        true,
        d(8),
        8,
    ));
    round_trip(&ArmT32Instruction::Vldm_Double_T1(
        R::R13,
        true,
        false,
        d(0),
        4,
    )); // vpop {d0-d3}
}

#[test]
fn encode__m8h_fp_data_processing_exact_bytes() {
    use crate::{ArmT32FpDataOperation2 as Op2, ArmT32FpDataOperation3 as Op3};
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m7 -mfpu=fpv5-d16`
    assert_eq!(
        ArmT32Instruction::FpDataProcess3_Single(Op3::Vadd, s(0), s(1), s(2))
            .encode()
            .unwrap(),
        vec![0x30, 0xEE, 0x81, 0x0A]
    ); // vadd.f32 s0, s1, s2
    assert_eq!(
        ArmT32Instruction::FpDataProcess3_Single(Op3::Vsub, s(3), s(4), s(5))
            .encode()
            .unwrap(),
        vec![0x72, 0xEE, 0x62, 0x1A]
    ); // vsub.f32 s3, s4, s5
    assert_eq!(
        ArmT32Instruction::FpDataProcess3_Single(Op3::Vmul, s(6), s(7), s(8))
            .encode()
            .unwrap(),
        vec![0x23, 0xEE, 0x84, 0x3A]
    ); // vmul.f32 s6, s7, s8
    assert_eq!(
        ArmT32Instruction::FpDataProcess3_Single(Op3::Vdiv, s(9), s(10), s(11))
            .encode()
            .unwrap(),
        vec![0xC5, 0xEE, 0x25, 0x4A]
    ); // vdiv.f32 s9, s10, s11
    assert_eq!(
        ArmT32Instruction::FpDataProcess3_Single(Op3::Vfma, s(0), s(1), s(2))
            .encode()
            .unwrap(),
        vec![0xA0, 0xEE, 0x81, 0x0A]
    ); // vfma.f32 s0, s1, s2
    assert_eq!(
        ArmT32Instruction::FpDataProcess2_Single(Op2::Vabs, s(0), s(1))
            .encode()
            .unwrap(),
        vec![0xB0, 0xEE, 0xE0, 0x0A]
    ); // vabs.f32 s0, s1
    assert_eq!(
        ArmT32Instruction::FpDataProcess2_Single(Op2::Vneg, s(3), s(4))
            .encode()
            .unwrap(),
        vec![0xF1, 0xEE, 0x42, 0x1A]
    ); // vneg.f32 s3, s4
    assert_eq!(
        ArmT32Instruction::FpDataProcess2_Single(Op2::Vmov, s(10), s(11))
            .encode()
            .unwrap(),
        vec![0xB0, 0xEE, 0x65, 0x5A]
    ); // vmov.f32 s10, s11
    assert_eq!(
        ArmT32Instruction::FpDataProcess3_Double(Op3::Vadd, d(0), d(1), d(2))
            .encode()
            .unwrap(),
        vec![0x31, 0xEE, 0x02, 0x0B]
    ); // vadd.f64 d0, d1, d2
    assert_eq!(
        ArmT32Instruction::FpDataProcess2_Double(Op2::Vabs, d(6), d(7))
            .encode()
            .unwrap(),
        vec![0xB0, 0xEE, 0xC7, 0x6B]
    ); // vabs.f64 d6, d7
    assert_eq!(
        ArmT32Instruction::FpDataProcess2_Double(Op2::Vmov, d(10), d(11))
            .encode()
            .unwrap(),
        vec![0xB0, 0xEE, 0x4B, 0xAB]
    ); // vmov.f64 d10, d11
}

#[test]
fn round_trip__m8h_fp_data_processing() {
    use crate::{ArmT32FpDataOperation2 as Op2, ArmT32FpDataOperation3 as Op3};
    let ops3 = [
        Op3::Vmla,
        Op3::Vmls,
        Op3::Vnmla,
        Op3::Vnmls,
        Op3::Vmul,
        Op3::Vnmul,
        Op3::Vadd,
        Op3::Vsub,
        Op3::Vdiv,
        Op3::Vfnma,
        Op3::Vfnms,
        Op3::Vfma,
        Op3::Vfms,
    ];
    for op in ops3 {
        round_trip(&ArmT32Instruction::FpDataProcess3_Single(
            op,
            s(1),
            s(20),
            s(31),
        ));
        round_trip(&ArmT32Instruction::FpDataProcess3_Double(
            op,
            d(1),
            d(10),
            d(15),
        ));
    }
    for op in [Op2::Vmov, Op2::Vabs, Op2::Vneg, Op2::Vsqrt] {
        round_trip(&ArmT32Instruction::FpDataProcess2_Single(op, s(5), s(30)));
        round_trip(&ArmT32Instruction::FpDataProcess2_Double(op, d(5), d(14)));
    }
}

#[test]
fn encode__m8i_fp_compare_move_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m7 -mfpu=fpv5-d16`
    assert_eq!(
        ArmT32Instruction::Vcmp_Single_T1(s(0), s(1), false)
            .encode()
            .unwrap(),
        vec![0xB4, 0xEE, 0x60, 0x0A]
    ); // vcmp.f32  s0, s1
    assert_eq!(
        ArmT32Instruction::Vcmp_Single_T1(s(2), s(3), true)
            .encode()
            .unwrap(),
        vec![0xB4, 0xEE, 0xE1, 0x1A]
    ); // vcmpe.f32 s2, s3
    assert_eq!(
        ArmT32Instruction::Vcmp_Zero_Single_T2(s(4), false)
            .encode()
            .unwrap(),
        vec![0xB5, 0xEE, 0x40, 0x2A]
    ); // vcmp.f32  s4, #0
    assert_eq!(
        ArmT32Instruction::Vcmp_Double_T1(d(0), d(1), false)
            .encode()
            .unwrap(),
        vec![0xB4, 0xEE, 0x41, 0x0B]
    ); // vcmp.f64  d0, d1
    assert_eq!(
        ArmT32Instruction::Vcmp_Zero_Double_T2(d(2), false)
            .encode()
            .unwrap(),
        vec![0xB5, 0xEE, 0x40, 0x2B]
    ); // vcmp.f64  d2, #0
    assert_eq!(
        ArmT32Instruction::Vmrs_Apsr_Nzcv_T1.encode().unwrap(),
        vec![0xF1, 0xEE, 0x10, 0xFA]
    ); // vmrs APSR_nzcv, fpscr
    assert_eq!(
        ArmT32Instruction::Vmrs_T1(R::R0).encode().unwrap(),
        vec![0xF1, 0xEE, 0x10, 0x0A]
    ); // vmrs r0, fpscr
    assert_eq!(
        ArmT32Instruction::Vmsr_T1(R::R1).encode().unwrap(),
        vec![0xE1, 0xEE, 0x10, 0x1A]
    ); // vmsr fpscr, r1
    assert_eq!(
        ArmT32Instruction::Vmov_Core_To_Single_T1(s(0), R::R1)
            .encode()
            .unwrap(),
        vec![0x00, 0xEE, 0x10, 0x1A]
    ); // vmov s0, r1
    assert_eq!(
        ArmT32Instruction::Vmov_Single_To_Core_T1(R::R2, s(3))
            .encode()
            .unwrap(),
        vec![0x11, 0xEE, 0x90, 0x2A]
    ); // vmov r2, s3
}

#[test]
fn round_trip__m8i_fp_compare_move() {
    for e in [false, true] {
        round_trip(&ArmT32Instruction::Vcmp_Single_T1(s(10), s(20), e));
        round_trip(&ArmT32Instruction::Vcmp_Double_T1(d(5), d(10), e));
        round_trip(&ArmT32Instruction::Vcmp_Zero_Single_T2(s(31), e));
        round_trip(&ArmT32Instruction::Vcmp_Zero_Double_T2(d(15), e));
    }
    round_trip(&ArmT32Instruction::Vmrs_T1(R::R7));
    round_trip(&ArmT32Instruction::Vmrs_Apsr_Nzcv_T1);
    round_trip(&ArmT32Instruction::Vmsr_T1(R::R12));
    for number in [0u8, 1, 15, 30, 31] {
        round_trip(&ArmT32Instruction::Vmov_Core_To_Single_T1(s(number), R::R3));
        round_trip(&ArmT32Instruction::Vmov_Single_To_Core_T1(R::R4, s(number)));
    }
}

#[test]
fn encode__m8i_vcvt_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m7 -mfpu=fpv5-d16`
    assert_eq!(
        ArmT32Instruction::Vcvt_FloatToInt_FromSingle_T1(s(0), s(1), true, true)
            .encode()
            .unwrap(),
        vec![0xBD, 0xEE, 0xE0, 0x0A]
    ); // vcvt.s32.f32
    assert_eq!(
        ArmT32Instruction::Vcvt_FloatToInt_FromSingle_T1(s(2), s(3), false, true)
            .encode()
            .unwrap(),
        vec![0xBC, 0xEE, 0xE1, 0x1A]
    ); // vcvt.u32.f32
    assert_eq!(
        ArmT32Instruction::Vcvt_FloatToInt_FromSingle_T1(s(0), s(1), true, false)
            .encode()
            .unwrap(),
        vec![0xBD, 0xEE, 0x60, 0x0A]
    ); // vcvtr.s32.f32
    assert_eq!(
        ArmT32Instruction::Vcvt_IntToFloat_ToSingle_T1(s(4), s(5), true)
            .encode()
            .unwrap(),
        vec![0xB8, 0xEE, 0xE2, 0x2A]
    ); // vcvt.f32.s32
    assert_eq!(
        ArmT32Instruction::Vcvt_IntToFloat_ToSingle_T1(s(6), s(7), false)
            .encode()
            .unwrap(),
        vec![0xB8, 0xEE, 0x63, 0x3A]
    ); // vcvt.f32.u32
    assert_eq!(
        ArmT32Instruction::Vcvt_Single_To_Double_T1(d(0), s(2))
            .encode()
            .unwrap(),
        vec![0xB7, 0xEE, 0xC1, 0x0A]
    ); // vcvt.f64.f32
    assert_eq!(
        ArmT32Instruction::Vcvt_Double_To_Single_T1(s(3), d(4))
            .encode()
            .unwrap(),
        vec![0xF7, 0xEE, 0xC4, 0x1B]
    ); // vcvt.f32.f64
    assert_eq!(
        ArmT32Instruction::Vcvt_FloatToInt_FromDouble_T1(s(0), d(1), true, true)
            .encode()
            .unwrap(),
        vec![0xBD, 0xEE, 0xC1, 0x0B]
    ); // vcvt.s32.f64
    assert_eq!(
        ArmT32Instruction::Vcvt_IntToFloat_ToDouble_T1(d(2), s(3), true)
            .encode()
            .unwrap(),
        vec![0xB8, 0xEE, 0xE1, 0x2B]
    ); // vcvt.f64.s32
}

#[test]
fn round_trip__m8i_vcvt() {
    for signed in [false, true] {
        for round in [false, true] {
            round_trip(&ArmT32Instruction::Vcvt_FloatToInt_FromSingle_T1(
                s(10),
                s(20),
                signed,
                round,
            ));
            round_trip(&ArmT32Instruction::Vcvt_FloatToInt_FromDouble_T1(
                s(5),
                d(10),
                signed,
                round,
            ));
        }
        round_trip(&ArmT32Instruction::Vcvt_IntToFloat_ToSingle_T1(
            s(0),
            s(31),
            signed,
        ));
        round_trip(&ArmT32Instruction::Vcvt_IntToFloat_ToDouble_T1(
            d(7),
            s(15),
            signed,
        ));
    }
    round_trip(&ArmT32Instruction::Vcvt_Single_To_Double_T1(d(0), s(1)));
    round_trip(&ArmT32Instruction::Vcvt_Double_To_Single_T1(s(2), d(3)));
}

#[test]
fn encode__m8i_corners_exact_bytes() {
    // bytes verified against `clang --target=thumbv7em-none-eabi -mcpu=cortex-m7 -mfpu=fpv5-d16`
    // VMOV immediate (imm8 0x70 == 1.0)
    assert_eq!(
        ArmT32Instruction::Vmov_Immediate_Single_T1(s(0), 0x70)
            .encode()
            .unwrap(),
        vec![0xB7, 0xEE, 0x00, 0x0A]
    ); // vmov.f32 s0, #1.0
    assert_eq!(
        ArmT32Instruction::Vmov_Immediate_Double_T1(d(0), 0x70)
            .encode()
            .unwrap(),
        vec![0xB7, 0xEE, 0x00, 0x0B]
    ); // vmov.f64 d0, #1.0
    // VMOV core pairs
    assert_eq!(
        ArmT32Instruction::Vmov_Double_To_CorePair_T1(R::R0, R::R1, d(2))
            .encode()
            .unwrap(),
        vec![0x51, 0xEC, 0x12, 0x0B]
    ); // vmov r0, r1, d2
    assert_eq!(
        ArmT32Instruction::Vmov_CorePair_To_Double_T1(d(3), R::R4, R::R5)
            .encode()
            .unwrap(),
        vec![0x45, 0xEC, 0x13, 0x4B]
    ); // vmov d3, r4, r5
    assert_eq!(
        ArmT32Instruction::Vmov_Singles_To_CorePair_T1(R::R6, R::R7, s(8))
            .encode()
            .unwrap(),
        vec![0x57, 0xEC, 0x14, 0x6A]
    ); // vmov r6, r7, s8, s9
    assert_eq!(
        ArmT32Instruction::Vmov_CorePair_To_Singles_T1(s(10), R::R2, R::R3)
            .encode()
            .unwrap(),
        vec![0x43, 0xEC, 0x15, 0x2A]
    ); // vmov s10, s11, r2, r3
    // half-precision conversions
    assert_eq!(
        ArmT32Instruction::Vcvt_HalfToSingle_T1(s(0), s(1), false)
            .encode()
            .unwrap(),
        vec![0xB2, 0xEE, 0x60, 0x0A]
    ); // vcvtb.f32.f16 s0, s1
    assert_eq!(
        ArmT32Instruction::Vcvt_SingleToHalf_T1(s(4), s(5), false)
            .encode()
            .unwrap(),
        vec![0xB3, 0xEE, 0x62, 0x2A]
    ); // vcvtb.f16.f32 s4, s5
    // fixed-point conversions
    assert_eq!(
        ArmT32Instruction::Vcvt_FloatToFixed_Single_T1(s(0), true, false, 1)
            .encode()
            .unwrap(),
        vec![0xBE, 0xEE, 0x67, 0x0A]
    ); // vcvt.s16.f32 s0, s0, #1
    assert_eq!(
        ArmT32Instruction::Vcvt_FixedToFloat_Single_T1(s(3), true, false, 3)
            .encode()
            .unwrap(),
        vec![0xFA, 0xEE, 0x66, 0x1A]
    ); // vcvt.f32.s16 s3, s3, #3
}

#[test]
fn round_trip__m8i_corners() {
    for imm8 in [0u8, 0x70, 0x00, 0x60, 0xF0, 0xFF] {
        round_trip(&ArmT32Instruction::Vmov_Immediate_Single_T1(s(15), imm8));
        round_trip(&ArmT32Instruction::Vmov_Immediate_Double_T1(d(7), imm8));
    }
    round_trip(&ArmT32Instruction::Vmov_Double_To_CorePair_T1(
        R::R0,
        R::R1,
        d(15),
    ));
    round_trip(&ArmT32Instruction::Vmov_CorePair_To_Double_T1(
        d(8),
        R::R2,
        R::R3,
    ));
    round_trip(&ArmT32Instruction::Vmov_Singles_To_CorePair_T1(
        R::R4,
        R::R5,
        s(30),
    ));
    round_trip(&ArmT32Instruction::Vmov_CorePair_To_Singles_T1(
        s(0),
        R::R6,
        R::R7,
    ));
    for top in [false, true] {
        round_trip(&ArmT32Instruction::Vcvt_HalfToSingle_T1(s(10), s(20), top));
        round_trip(&ArmT32Instruction::Vcvt_SingleToHalf_T1(s(31), s(0), top));
    }
    for signed in [false, true] {
        round_trip(&ArmT32Instruction::Vcvt_FloatToFixed_Single_T1(
            s(5),
            signed,
            false,
            7,
        ));
        round_trip(&ArmT32Instruction::Vcvt_FloatToFixed_Double_T1(
            d(5),
            signed,
            true,
            20,
        ));
        round_trip(&ArmT32Instruction::Vcvt_FixedToFloat_Single_T1(
            s(6),
            signed,
            true,
            31,
        ));
        round_trip(&ArmT32Instruction::Vcvt_FixedToFloat_Double_T1(
            d(6),
            signed,
            false,
            16,
        ));
    }
}

#[test]
fn vfp_immediate_codec__round_trips_representable_values() {
    use crate::{vfp_encode_f64_to_imm8, vfp_expand_imm8_to_f32};
    assert_eq!(vfp_expand_imm8_to_f32(0x70), 1.0);
    assert_eq!(vfp_expand_imm8_to_f32(0x00), 2.0);
    assert_eq!(vfp_expand_imm8_to_f32(0x60), 0.5);
    assert_eq!(vfp_expand_imm8_to_f32(0xF0), -1.0);
    assert_eq!(vfp_encode_f64_to_imm8(1.0), Some(0x70));
    assert_eq!(vfp_encode_f64_to_imm8(0.5), Some(0x60));
    assert_eq!(vfp_encode_f64_to_imm8(-1.0), Some(0xF0));
    assert_eq!(vfp_encode_f64_to_imm8(0.123456), None); // not representable
}

#[test]
fn gating__fp_instruction_requires_floating_point_feature() {
    let instruction = ArmT32Instruction::Vldr_Single_T2(s(0), R::R0, 0);
    assert_eq!(
        instruction.requirement(),
        ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[ArmCpuFeature::FloatingPoint]),
    );
    // a plain ARMv7-M target (no FPU) refuses it
    assert!(
        instruction
            .encode_for_target(&ArmTargetProfile::armv7m())
            .is_err()
    );
    // the permissive profile (which has the FloatingPoint feature) accepts it
    assert_eq!(
        instruction.encode_for_target(&ArmTargetProfile::permissive()),
        instruction.encode()
    );
}

#[test]
fn gating__dsp_instruction_requires_dsp_extension() {
    let instruction = ArmT32Instruction::Qadd_T1(R::R0, R::R1, R::R2);

    // a DSP instruction reports the DSP requirement
    assert_eq!(
        instruction.requirement(),
        ArmInstructionRequirement::new(ArmIsaVersion::Armv7EM, &[ArmCpuFeature::DspExtension]),
    );

    // a plain ARMv7-M target (no DSP) refuses it
    assert_eq!(
        instruction.encode_for_target(&ArmTargetProfile::armv7m()),
        Err(EncodeError::UnsupportedInstructionForTarget {
            required: ArmInstructionRequirement::new(
                ArmIsaVersion::Armv7EM,
                &[ArmCpuFeature::DspExtension]
            ),
            target_isa_version: ArmIsaVersion::Armv7M,
        }),
    );

    // an ARMv7E-M (Cortex-M4) target accepts it, and the bytes equal the target-independent encode()
    assert_eq!(
        instruction.encode_for_target(&ArmTargetProfile::armv7em()),
        instruction.encode()
    );
}

// VMOV core <-> scalar-lane (T32): VMOV.<size> Dd[x], Rt and VMOV.<dt> Rt, Dn[x]. The T32 word equals the A32
// word with cond = AL. Every byte confirmed byte-identical against BOTH arm-none-eabi-as (-mthumb) AND llvm-mc
// (-triple=thumbv8a). 4 bytes per instruction (two LE halfwords).
#[test]
fn t32_vmov_core_scalar_lane() {
    use Arm32VmovLaneSize::{Byte, Half, Word};
    // exact bytes
    assert_eq!(
        ArmT32Instruction::Vmov_Core_To_Scalar_T1(Word, 0, d(0), R::R1)
            .encode()
            .unwrap(),
        vec![0x00, 0xee, 0x10, 0x1b]
    ); // vmov.32 d0[0], r1
    assert_eq!(
        ArmT32Instruction::Vmov_Core_To_Scalar_T1(Word, 1, d(0), R::R1)
            .encode()
            .unwrap(),
        vec![0x20, 0xee, 0x10, 0x1b]
    ); // vmov.32 d0[1], r1
    assert_eq!(
        ArmT32Instruction::Vmov_Core_To_Scalar_T1(Byte, 7, d(0), R::R1)
            .encode()
            .unwrap(),
        vec![0x60, 0xee, 0x70, 0x1b]
    ); // vmov.8  d0[7], r1
    assert_eq!(
        ArmT32Instruction::Vmov_Core_To_Scalar_T1(Half, 3, d(0), R::R1)
            .encode()
            .unwrap(),
        vec![0x20, 0xee, 0x70, 0x1b]
    ); // vmov.16 d0[3], r1
    assert_eq!(
        ArmT32Instruction::Vmov_Scalar_To_Core_T1(false, Word, 0, R::R2, d(3))
            .encode()
            .unwrap(),
        vec![0x13, 0xee, 0x10, 0x2b]
    ); // vmov.32 r2, d3[0]
    assert_eq!(
        ArmT32Instruction::Vmov_Scalar_To_Core_T1(false, Byte, 5, R::R2, d(3))
            .encode()
            .unwrap(),
        vec![0x73, 0xee, 0x30, 0x2b]
    ); // vmov.s8  r2, d3[5]
    assert_eq!(
        ArmT32Instruction::Vmov_Scalar_To_Core_T1(true, Half, 2, R::R2, d(3))
            .encode()
            .unwrap(),
        vec![0xb3, 0xee, 0x30, 0x2b]
    ); // vmov.u16 r2, d3[2]
    // emit (no condition in T32)
    assert_eq!(
        ArmT32Instruction::Vmov_Core_To_Scalar_T1(Word, 0, d(0), R::R1)
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vmov.32 d0[0], r1"
    );
    assert_eq!(
        ArmT32Instruction::Vmov_Scalar_To_Core_T1(false, Byte, 5, R::R2, d(3))
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vmov.s8 r2, d3[5]"
    );
    // requirement: M-profile FP extension
    assert_eq!(
        ArmT32Instruction::Vmov_Core_To_Scalar_T1(Word, 0, d(0), R::R1).requirement(),
        ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[ArmCpuFeature::FloatingPoint])
    );
    // reject out-of-range lane
    assert!(matches!(
        ArmT32Instruction::Vmov_Core_To_Scalar_T1(Word, 2, d(0), R::R1).encode(),
        Err(EncodeError::ImmediateOutOfRange { .. })
    ));
    // round-trip every width x lane x direction
    for (size, lanes) in [(Byte, 8u8), (Half, 4), (Word, 2)] {
        for index in 0..lanes {
            round_trip(&ArmT32Instruction::Vmov_Core_To_Scalar_T1(
                size,
                index,
                d(5),
                R::R9,
            ));
            let signs: &[bool] = if matches!(size, Word) {
                &[false]
            } else {
                &[false, true]
            };
            for &u in signs {
                round_trip(&ArmT32Instruction::Vmov_Scalar_To_Core_T1(
                    u,
                    size,
                    index,
                    R::R9,
                    d(5),
                ));
            }
        }
    }
}

// VMAXNM / VMINNM (T32) -- ARMv8-M scalar FP max/min. Bytes dual-oracle confirmed vs arm-none-eabi-as
// (-march=armv8.1-m.main+fp.dp) AND llvm-mc (-triple=thumbv8a).
#[test]
fn t32_vmaxnm_vminnm() {
    use ArmT32Instruction::*;
    assert_eq!(
        Vmaxnm_Single_T1(s(0), s(1), s(2)).encode().unwrap(),
        vec![0x80, 0xfe, 0x81, 0x0a]
    ); // vmaxnm.f32 s0, s1, s2
    assert_eq!(
        Vmaxnm_Double_T1(d(0), d(1), d(2)).encode().unwrap(),
        vec![0x81, 0xfe, 0x02, 0x0b]
    ); // vmaxnm.f64 d0, d1, d2
    assert_eq!(
        Vminnm_Single_T1(s(0), s(1), s(2)).encode().unwrap(),
        vec![0x80, 0xfe, 0xc1, 0x0a]
    ); // vminnm.f32 s0, s1, s2
    assert_eq!(
        Vminnm_Double_T1(d(0), d(1), d(2)).encode().unwrap(),
        vec![0x81, 0xfe, 0x42, 0x0b]
    ); // vminnm.f64 d0, d1, d2
    assert_eq!(
        Vmaxnm_Single_T1(s(0), s(1), s(2)).to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vmaxnm.f32 s0, s1, s2"
    );
    assert_eq!(
        Vminnm_Double_T1(d(3), d(4), d(5)).to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vminnm.f64 d3, d4, d5"
    );
    assert_eq!(
        Vmaxnm_Single_T1(s(0), s(1), s(2)).requirement(),
        ArmInstructionRequirement::new(
            ArmIsaVersion::Armv8MBaseline,
            &[ArmCpuFeature::FloatingPoint]
        )
    );
    // round-trip over a spread of registers (exercises the D/N/M extra bits + max/min + single/double decode)
    for (sd, sn, sm) in [(0u8, 1, 2), (5, 20, 31), (31, 0, 15)] {
        round_trip(&Vmaxnm_Single_T1(s(sd), s(sn), s(sm)));
        round_trip(&Vminnm_Single_T1(s(sd), s(sn), s(sm)));
        round_trip(&Vmaxnm_Double_T1(d(sd), d(sn), d(sm)));
        round_trip(&Vminnm_Double_T1(d(sd), d(sn), d(sm)));
    }
}

// VRINTA/N/P/M (T32) -- ARMv8-M directed FP round to integral. Dual-oracle confirmed (arm-none-eabi-as
// -march=armv8.1-m.main+fp.dp + llvm-mc -triple=thumbv8a).
#[test]
fn t32_vrint_directed() {
    use Arm32DirectedRound::{A, M, N, P};
    use ArmT32Instruction::*;
    assert_eq!(
        Vrint_Directed_Single_T1(A, s(0), s(1)).encode().unwrap(),
        vec![0xb8, 0xfe, 0x60, 0x0a]
    ); // vrinta.f32 s0, s1
    assert_eq!(
        Vrint_Directed_Single_T1(N, s(0), s(1)).encode().unwrap(),
        vec![0xb9, 0xfe, 0x60, 0x0a]
    ); // vrintn.f32
    assert_eq!(
        Vrint_Directed_Single_T1(P, s(0), s(1)).encode().unwrap(),
        vec![0xba, 0xfe, 0x60, 0x0a]
    ); // vrintp.f32
    assert_eq!(
        Vrint_Directed_Single_T1(M, s(0), s(1)).encode().unwrap(),
        vec![0xbb, 0xfe, 0x60, 0x0a]
    ); // vrintm.f32
    assert_eq!(
        Vrint_Directed_Double_T1(A, d(0), d(1)).encode().unwrap(),
        vec![0xb8, 0xfe, 0x41, 0x0b]
    ); // vrinta.f64 d0, d1
    assert_eq!(
        Vrint_Directed_Single_T1(A, s(0), s(1))
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vrinta.f32 s0, s1"
    );
    assert_eq!(
        Vrint_Directed_Double_T1(M, d(3), d(4))
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vrintm.f64 d3, d4"
    );
    assert_eq!(
        Vrint_Directed_Single_T1(A, s(0), s(1)).requirement(),
        ArmInstructionRequirement::new(
            ArmIsaVersion::Armv8MBaseline,
            &[ArmCpuFeature::FloatingPoint]
        )
    );
    for mode in [A, N, P, M] {
        for (vd, vm) in [(0u8, 1), (5, 20), (31, 15)] {
            round_trip(&Vrint_Directed_Single_T1(mode, s(vd), s(vm)));
            round_trip(&Vrint_Directed_Double_T1(mode, d(vd), d(vm)));
        }
    }
}

#[test]
fn t32_vrintz_vrintx() {
    use ArmT32Instruction::*;
    // Dual-oracle (arm-none-eabi-as -march=armv8.1-m.main+fp.dp -mthumb + llvm-mc -triple=thumbv8a):
    assert_eq!(
        Vrintz_Single_T1(s(3), s(4)).encode().unwrap(),
        vec![0xf6, 0xee, 0xc2, 0x1a]
    ); // vrintz.f32 s3, s4
    assert_eq!(
        Vrintz_Double_T1(d(3), d(4)).encode().unwrap(),
        vec![0xb6, 0xee, 0xc4, 0x3b]
    ); // vrintz.f64 d3, d4
    assert_eq!(
        Vrintx_Single_T1(s(3), s(4)).encode().unwrap(),
        vec![0xf7, 0xee, 0x42, 0x1a]
    ); // vrintx.f32 s3, s4
    assert_eq!(
        Vrintx_Double_T1(d(3), d(4)).encode().unwrap(),
        vec![0xb7, 0xee, 0x44, 0x3b]
    ); // vrintx.f64 d3, d4
    assert_eq!(
        Vrintz_Single_T1(s(3), s(4)).to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vrintz.f32 s3, s4"
    );
    assert_eq!(
        Vrintx_Double_T1(d(3), d(4)).to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vrintx.f64 d3, d4"
    );
    assert_eq!(
        Vrintz_Single_T1(s(3), s(4)).requirement(),
        ArmInstructionRequirement::new(
            ArmIsaVersion::Armv8MBaseline,
            &[ArmCpuFeature::FloatingPoint]
        )
    );
    for (vd, vm) in [(0u8, 1), (5, 20), (31, 15)] {
        round_trip(&Vrintz_Single_T1(s(vd), s(vm)));
        round_trip(&Vrintz_Double_T1(d(vd), d(vm)));
        round_trip(&Vrintx_Single_T1(s(vd), s(vm)));
        round_trip(&Vrintx_Double_T1(d(vd), d(vm)));
    }
}

#[test]
fn t32_vcvt_directed_fp_to_int() {
    use Arm32DirectedRound::{A, M, N, P};
    use ArmT32Instruction::*;
    // Dual-oracle (arm-none-eabi-as -march=armv8.1-m.main+fp.dp -mthumb + llvm-mc -triple=thumbv8a):
    assert_eq!(
        Vcvt_Directed_FromSingle_T1(A, s(0), s(1), true)
            .encode()
            .unwrap(),
        vec![0xbc, 0xfe, 0xe0, 0x0a]
    ); // vcvta.s32.f32 s0, s1
    assert_eq!(
        Vcvt_Directed_FromSingle_T1(A, s(0), s(1), false)
            .encode()
            .unwrap(),
        vec![0xbc, 0xfe, 0x60, 0x0a]
    ); // vcvta.u32.f32 s0, s1
    assert_eq!(
        Vcvt_Directed_FromDouble_T1(A, s(0), d(1), true)
            .encode()
            .unwrap(),
        vec![0xbc, 0xfe, 0xc1, 0x0b]
    ); // vcvta.s32.f64 s0, d1
    assert_eq!(
        Vcvt_Directed_FromSingle_T1(N, s(2), s(3), true)
            .encode()
            .unwrap(),
        vec![0xbd, 0xfe, 0xe1, 0x1a]
    ); // vcvtn.s32.f32 s2, s3
    assert_eq!(
        Vcvt_Directed_FromSingle_T1(P, s(4), s(5), false)
            .encode()
            .unwrap(),
        vec![0xbe, 0xfe, 0x62, 0x2a]
    ); // vcvtp.u32.f32 s4, s5
    assert_eq!(
        Vcvt_Directed_FromDouble_T1(M, s(6), d(7), true)
            .encode()
            .unwrap(),
        vec![0xbf, 0xfe, 0xc7, 0x3b]
    ); // vcvtm.s32.f64 s6, d7
    assert_eq!(
        Vcvt_Directed_FromDouble_T1(M, s(8), d(9), false)
            .encode()
            .unwrap(),
        vec![0xbf, 0xfe, 0x49, 0x4b]
    ); // vcvtm.u32.f64 s8, d9
    assert_eq!(
        Vcvt_Directed_FromSingle_T1(A, s(0), s(1), true)
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vcvta.s32.f32 s0, s1"
    );
    assert_eq!(
        Vcvt_Directed_FromDouble_T1(M, s(8), d(9), false)
            .to_assembly_string(crate::emit::ArmAssemblySyntax::Gnu),
        "vcvtm.u32.f64 s8, d9"
    );
    assert_eq!(
        Vcvt_Directed_FromSingle_T1(A, s(0), s(1), true).requirement(),
        ArmInstructionRequirement::new(
            ArmIsaVersion::Armv8MBaseline,
            &[ArmCpuFeature::FloatingPoint]
        )
    );
    for mode in [A, N, P, M] {
        for signed in [true, false] {
            for (vd, vm) in [(0u8, 1), (5, 20), (31, 15)] {
                round_trip(&Vcvt_Directed_FromSingle_T1(mode, s(vd), s(vm), signed));
                round_trip(&Vcvt_Directed_FromDouble_T1(mode, s(vd), d(vm), signed));
            }
        }
    }
}
