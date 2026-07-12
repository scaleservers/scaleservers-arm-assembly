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

