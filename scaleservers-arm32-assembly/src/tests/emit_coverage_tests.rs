// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

// Exhaustive emit (UAL text) assertions for the A32 and T32 emitters. The differential oracle validates
// emit text against real `arm-none-eabi-as`/`objdump`, but that is an integration test (in `tests/`) and so
// is excluded from `--lib` coverage. This module pins the same emitter code paths as ordinary library unit
// tests: every instruction below is a known-valid construction (mirrored from the round-trip corpus) and its
// expected text was captured from the emitter and cross-checked against the oracle. The cases are grouped by
// instruction family; `EXPECTED_A32_GNU` is the parallel table of GNU-flavor strings.

use crate::Arm32BlockAddressMode as Blk;
use crate::Arm32Condition as Cond;
use crate::Arm32CpsMode as Cps;
use crate::Arm32DirectedRound as DRnd;
use crate::Arm32ExtendType as Ext;
use crate::Arm32FpDataOperation2 as F2;
use crate::Arm32FpDataOperation3 as F3;
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
use crate::Arm32VrintMode as VRnd;
use crate::Arm32VselCondition as Vsel;
use crate::enums::Arm32GeneralPurposeRegister as R;
use crate::{ArmA32Instruction, ArmAssemblySyntax};

const GNU: ArmAssemblySyntax = ArmAssemblySyntax::Gnu;

fn s(number: u8) -> crate::Arm32SinglePrecisionRegister {
    crate::Arm32SinglePrecisionRegister::new(number).unwrap()
}
fn d(number: u8) -> crate::Arm32DoublePrecisionRegister {
    crate::Arm32DoublePrecisionRegister::new(number).unwrap()
}
fn q(number: u8) -> crate::Arm32QuadwordRegister {
    crate::Arm32QuadwordRegister::new(number).unwrap()
}

/// Every A32 instruction whose emit text is asserted below, in the same order as `EXPECTED_A32_GNU`.
fn a32_cases() -> Vec<ArmA32Instruction> {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;
    let imm = |add, imm12| Mem::Immediate { add, imm12 };
    let reg = |add, rm, shift| Mem::Register { add, rm, shift };
    let imm8 = |add, imm8| Mem8::Immediate { add, imm8 };
    let reg8 = |add, rm| Mem8::Register { add, rm };
    vec![
        // ---- data processing: immediate ----
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
        // ---- data processing: register (with shift rendering) ----
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
        // ---- data processing: register-shifted register ----
        And_RegisterShiftedRegister_A1(al, true, R::R0, R::R1, R::R2, ShiftType::Lsl, R::R3),
        Eor_RegisterShiftedRegister_A1(al, false, R::R4, R::R5, R::R6, ShiftType::Lsr, R::R7),
        Sub_RegisterShiftedRegister_A1(al, false, R::R8, R::R9, R::R10, ShiftType::Asr, R::R11),
        Rsb_RegisterShiftedRegister_A1(al, true, R::R0, R::R1, R::R2, ShiftType::Ror, R::R3),
        Mov_RegisterShiftedRegister_A1(al, false, R::R4, R::R5, ShiftType::Asr, R::R6),
        Mvn_RegisterShiftedRegister_A1(al, true, R::R7, R::R8, ShiftType::Ror, R::R9),
        Tst_RegisterShiftedRegister_A1(al, R::R10, R::R11, ShiftType::Lsl, R::R12),
        Cmp_RegisterShiftedRegister_A1(al, R::R3, R::R4, ShiftType::Asr, R::R5),
        // ---- multiply ----
        Mul_A1(al, false, R::R0, R::R1, R::R2),
        Mul_A1(al, true, R::R8, R::R9, R::R10),
        Mla_A1(al, false, R::R6, R::R7, R::R8, R::R9),
        Mls_A1(al, R::R1, R::R2, R::R3, R::R4),
        Umull_A1(al, false, R::R0, R::R1, R::R2, R::R3),
        Umlal_A1(al, false, R::R8, R::R9, R::R10, R::R11),
        Smull_A1(al, true, R::R0, R::R1, R::R2, R::R3),
        Smlal_A1(Cond::CarrySet, false, R::R4, R::R5, R::R6, R::R7),
        Umaal_A1(al, R::R8, R::R9, R::R10, R::R11),
        // ---- saturating ----
        Qadd_A1(al, R::R0, R::R1, R::R2),
        Qsub_A1(al, R::R3, R::R4, R::R5),
        Qdadd_A1(al, R::R6, R::R7, R::R8),
        Qdsub_A1(al, R::R9, R::R10, R::R11),
        // ---- signed multiply (DSP) ----
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
        // ---- parallel (packed SIMD) add/sub + SEL ----
        ParallelAddSub_A1(al, POp::Add16, PPfx::Signed, R::R0, R::R1, R::R2),
        ParallelAddSub_A1(al, POp::Sub8, PPfx::UnsignedSaturating, R::R3, R::R4, R::R5),
        ParallelAddSub_A1(al, POp::Add8, PPfx::SignedHalving, R::R0, R::R1, R::R2),
        ParallelAddSub_A1(al, POp::Asx, PPfx::Unsigned, R::R0, R::R1, R::R2),
        ParallelAddSub_A1(al, POp::Sax, PPfx::SignedSaturating, R::R6, R::R7, R::R8),
        ParallelAddSub_A1(al, POp::Sub16, PPfx::UnsignedHalving, R::R9, R::R10, R::R11),
        Sel_A1(al, R::R0, R::R1, R::R2),
        // ---- extend / reverse / clz ----
        Extend_A1(al, Ext::Sxtb, R::R0, R::R1, 0),
        Extend_A1(al, Ext::Sxtb, R::R0, R::R1, 8),
        Extend_A1(al, Ext::Sxth, R::R0, R::R1, 16),
        Extend_A1(al, Ext::Uxtb, R::R3, R::R4, 24),
        Extend_A1(al, Ext::Uxth, R::R3, R::R4, 0),
        Extend_A1(al, Ext::Uxtb16, R::R0, R::R1, 0),
        Extend_A1(al, Ext::Sxtb16, R::R2, R::R3, 8),
        ExtendAndAdd_A1(al, Ext::Sxtb, R::R0, R::R1, R::R2, 0),
        ExtendAndAdd_A1(al, Ext::Uxth, R::R3, R::R4, R::R5, 8),
        Rev_A1(al, R::R0, R::R1),
        Rev16_A1(al, R::R2, R::R3),
        Revsh_A1(al, R::R4, R::R5),
        Rbit_A1(al, R::R6, R::R7),
        Clz_A1(al, R::R0, R::R1),
        // ---- pack / saturate / sad ----
        Pkhbt_A1(al, R::R0, R::R1, R::R2, 0),
        Pkhbt_A1(al, R::R0, R::R1, R::R2, 4),
        Pkhtb_A1(al, R::R0, R::R1, R::R2, 1),
        Ssat_A1(al, R::R0, 1, R::R1, Shift::Lsl(0)),
        Ssat_A1(al, R::R0, 32, R::R1, Shift::Asr(1)),
        Usat_A1(al, R::R0, 0, R::R1, Shift::Lsl(0)),
        Usat_A1(al, R::R3, 15, R::R4, Shift::Lsl(5)),
        Ssat16_A1(al, R::R0, 1, R::R1),
        Usat16_A1(al, R::R2, 15, R::R3),
        Usad8_A1(al, R::R0, R::R1, R::R2),
        Usada8_A1(al, R::R0, R::R1, R::R2, R::R3),
        // ---- bitfield ----
        Bfc_A1(al, R::R0, 0, 1),
        Bfi_A1(al, R::R2, R::R3, 4, 8),
        Sbfx_A1(al, R::R2, R::R3, 4, 8),
        Ubfx_A1(al, R::R0, R::R1, 0, 32),
        // ---- load/store single (word/byte) ----
        Ldr_A1(al, R::R0, R::R1, imm(true, 0), Idx::Offset),
        Ldr_A1(al, R::R0, R::R1, imm(true, 4), Idx::Offset),
        Ldr_A1(al, R::R0, R::R1, imm(false, 4), Idx::Offset),
        Str_A1(al, R::R2, R::R3, imm(true, 8), Idx::PreIndex),
        Ldr_A1(al, R::R0, R::R1, imm(true, 4), Idx::PostIndex),
        Ldrb_A1(al, R::R0, R::R1, reg(true, R::R2, Shift::Lsl(0)), Idx::Offset),
        Ldr_A1(al, R::R0, R::R1, reg(true, R::R2, Shift::Lsl(2)), Idx::Offset),
        Ldr_A1(al, R::R0, R::R1, reg(false, R::R3, Shift::Asr(31)), Idx::PreIndex),
        Ldr_A1(al, R::R0, R::R1, reg(true, R::R4, Shift::Rrx), Idx::Offset),
        Str_A1(al, R::R5, R::R6, imm(false, 100), Idx::PostIndex),
        Strb_A1(al, R::R9, R::R10, imm(true, 4095), Idx::Offset),
        Ldrt_A1(al, R::R0, R::R1, imm(true, 4)),
        Strt_A1(al, R::R2, R::R3, imm(true, 4)),
        Ldrbt_A1(al, R::R4, R::R5, imm(false, 8)),
        Strbt_A1(al, R::R6, R::R7, imm(true, 12)),
        // ---- load/store halfword / dual / signed ----
        Ldrh_A1(al, R::R0, R::R1, imm8(true, 0), Idx::Offset),
        Ldrh_A1(al, R::R0, R::R1, imm8(true, 4), Idx::Offset),
        Ldrh_A1(al, R::R0, R::R1, imm8(false, 8), Idx::Offset),
        Strh_A1(al, R::R2, R::R3, imm8(true, 16), Idx::PreIndex),
        Ldrsb_A1(al, R::R0, R::R1, reg8(true, R::R2), Idx::Offset),
        Ldrsh_A1(al, R::R4, R::R5, imm8(true, 10), Idx::Offset),
        Ldrd_A1(al, R::R0, R::R1, imm8(true, 8), Idx::Offset),
        Strd_A1(al, R::R2, R::R3, imm8(true, 16), Idx::Offset),
        Ldrh_A1(al, R::R0, R::R1, reg8(false, R::R3), Idx::PostIndex),
        Ldrht_A1(al, R::R0, R::R1, imm8(true, 4)),
        Strht_A1(al, R::R2, R::R3, imm8(true, 4)),
        Ldrsbt_A1(al, R::R4, R::R5, imm8(true, 4)),
        Ldrsht_A1(al, R::R6, R::R7, imm8(true, 4)),
        // ---- load/store multiple (incl. push/pop aliases) ----
        Ldm_A1(al, Blk::IncrementAfter, R::R0, false, false, vec![R::R1, R::R2, R::R3]),
        Ldm_A1(al, Blk::IncrementAfter, R::R0, true, false, vec![R::R1, R::R2, R::R3]),
        Stm_A1(al, Blk::DecrementBefore, R::R13, true, false, vec![R::R4, R::R5, R::R14]),
        Ldm_A1(al, Blk::IncrementAfter, R::R13, true, false, vec![R::R4, R::R5, R::R15]),
        Ldm_A1(al, Blk::IncrementBefore, R::R0, false, false, vec![R::R1]),
        Ldm_A1(al, Blk::DecrementAfter, R::R0, false, false, vec![R::R1]),
        Ldm_A1(al, Blk::IncrementAfter, R::R0, false, true, vec![R::R1]),
        Stm_A1(al, Blk::IncrementAfter, R::R9, false, false, vec![R::R0, R::R7, R::R8]),
        // ---- synchronization ----
        Ldrex_A1(al, R::R0, R::R1),
        Strex_A1(al, R::R2, R::R3, R::R4),
        Ldrexb_A1(al, R::R5, R::R6),
        Strexb_A1(al, R::R7, R::R8, R::R9),
        Ldrexh_A1(al, R::R10, R::R11),
        Strexh_A1(al, R::R0, R::R1, R::R2),
        Ldrexd_A1(al, R::R0, R::R3),
        Strexd_A1(al, R::R4, R::R5, R::R6),
        Clrex_A1,
        Swp_A1(al, R::R0, R::R1, R::R2),
        Swpb_A1(al, R::R3, R::R4, R::R5),
        // ---- branch / interwork ----
        B_A1(al, 8),
        B_A1(al, -8),
        Bl_A1(al, 0x1000),
        Blx_Immediate_A1(4),
        Blx_Immediate_A1(6),
        Bx_A1(al, R::R1),
        Blx_Register_A1(al, R::R0),
        Bxj_A1(al, R::R2),
        B_A1(Cond::NotEqual, 12),
        // ---- status / system register access ----
        Mrs_A1(al, false, R::R0),
        Mrs_A1(al, true, R::R1),
        Msr_Register_A1(al, false, 0b1000, R::R0),
        Msr_Register_A1(al, false, 0b1001, R::R1),
        Msr_Register_A1(al, true, 0b1111, R::R2),
        Msr_Immediate_A1(al, false, 0b1000, 0xF000_0000),
        Cps_A1(Cps::Enable, false, true, false, None),
        Cps_A1(Cps::Disable, false, true, true, None),
        Cps_A1(Cps::NoChange, false, false, false, Some(0x13)),
        Setend_A1(true),
        Setend_A1(false),
        // ---- coprocessor ----
        Mcr_A1(al, 15, 0, R::R0, 1, 0, 0),
        Mrc_A1(al, 15, 0, R::R1, 1, 0, 0),
        Cdp_A1(al, 7, 1, 2, 3, 4, 5),
        Mcrr_A1(al, 15, 5, R::R0, R::R1, 2),
        Mrrc_A1(al, 14, 15, R::R10, R::R11, 15),
        Ldc_A1(al, 14, false, 5, R::R0, true, 2, Idx::Offset),
        Stc_A1(al, 13, false, 7, R::R2, true, 100, Idx::PostIndex),
        Mcr2_A1(15, 0, R::R0, 1, 0, 0),
        Mrc2_A1(15, 0, R::R0, 0, 0, 0),
        Cdp2_A1(7, 1, 2, 3, 4, 5),
        Mcrr2_A1(8, 5, R::R2, R::R3, 4),
        Mrrc2_A1(9, 10, R::R4, R::R5, 6),
        Ldc2_A1(14, true, 0, R::R3, false, 4, Idx::Offset),
        Stc2_A1(15, false, 15, R::R4, true, 200, Idx::PreIndex),
        // ---- hints / barriers / exceptions ----
        Yield_A1(al),
        Wfe_A1(al),
        Wfi_A1(al),
        Sev_A1(al),
        Dbg_A1(al, 5),
        Dmb_A1(0xF),
        Dsb_A1(0xF),
        Isb_A1(0xF),
        Bkpt_A1(al, 0xABCD),
        Hvc_A1(al, 0x1234),
        Smc_A1(al, 5),
        Udf_A1(al, 0xDEAD),
        Eret_A1(al),
        Nop_A1(al),
    ]
}

/// GNU-flavor UAL text for each entry of `a32_cases()`, in the same order. Captured from the emitter and
/// cross-checked against `arm-none-eabi-as`/`objdump` (the differential oracle).
const EXPECTED_A32_GNU: &[&str] = &[
    "ands r0, r1, #255",
    "eorne r2, r3, #43776",
    "sub r4, r5, #4278190080",
    "rsbs r6, r7, #0",
    "add r8, r9, #1020",
    "adc r10, r11, #1",
    "sbcs r12, r0, #2",
    "rsc r1, r2, #3221225472",
    "orr r3, r4, #255",
    "bics r5, r6, #16128",
    "mov r0, #256",
    "mvns r1, #0",
    "tst r2, #2147483648",
    "teq r3, #255",
    "cmp r4, #256",
    "cmncs r5, #1",
    "and r0, r1, r2",
    "eors r3, r4, r5, lsl #31",
    "sub r6, r7, r8, lsr #1",
    "rsbs r9, r10, r11, lsr #32",
    "add r12, r0, r1, asr #1",
    "adcs r2, r3, r4, asr #32",
    "sbc r5, r6, r7, ror #15",
    "rscs r8, r9, r10, rrx",
    "orr r11, r12, r0",
    "bics r1, r2, r3, lsl #7",
    "asr r4, r5, #5",
    "mvns r6, r7, ror #1",
    "tst r8, r9, lsl #3",
    "teq r10, r11",
    "cmp r12, r0, rrx",
    "cmn r1, r2, asr #1",
    "movw r0, #65535",
    "movteq r9, #32768",
    "ands r0, r1, r2, lsl r3",
    "eor r4, r5, r6, lsr r7",
    "sub r8, r9, r10, asr r11",
    "rsbs r0, r1, r2, ror r3",
    "asr r4, r5, r6",
    "mvns r7, r8, ror r9",
    "tst r10, r11, lsl r12",
    "cmp r3, r4, asr r5",
    "mul r0, r1, r2",
    "muls r8, r9, r10",
    "mla r6, r7, r8, r9",
    "mls r1, r2, r3, r4",
    "umull r0, r1, r2, r3",
    "umlal r8, r9, r10, r11",
    "smulls r0, r1, r2, r3",
    "smlalcs r4, r5, r6, r7",
    "umaal r8, r9, r10, r11",
    "qadd r0, r1, r2",
    "qsub r3, r4, r5",
    "qdadd r6, r7, r8",
    "qdsub r9, r10, r11",
    "smlabb r0, r1, r2, r3",
    "smlatb r0, r1, r2, r3",
    "smlabt r0, r1, r2, r3",
    "smlatt r0, r1, r2, r3",
    "smlawt r4, r5, r6, r7",
    "smulwb r8, r9, r10",
    "smlaltb r0, r1, r2, r3",
    "smulbt r0, r1, r2",
    "smladx r0, r1, r2, r3",
    "smuad r4, r5, r6",
    "smlsdx r0, r1, r2, r3",
    "smusd r7, r8, r9",
    "smmlar r0, r1, r2, r3",
    "smmul r4, r5, r6",
    "smmlsr r0, r1, r2, r3",
    "smlaldx r0, r1, r2, r3",
    "smlsldne r0, r1, r2, r3",
    "sadd16 r0, r1, r2",
    "uqsub8 r3, r4, r5",
    "shadd8 r0, r1, r2",
    "uasx r0, r1, r2",
    "qsax r6, r7, r8",
    "uhsub16 r9, r10, r11",
    "sel r0, r1, r2",
    "sxtb r0, r1",
    "sxtb r0, r1, ror #8",
    "sxth r0, r1, ror #16",
    "uxtb r3, r4, ror #24",
    "uxth r3, r4",
    "uxtb16 r0, r1",
    "sxtb16 r2, r3, ror #8",
    "sxtab r0, r1, r2",
    "uxtah r3, r4, r5, ror #8",
    "rev r0, r1",
    "rev16 r2, r3",
    "revsh r4, r5",
    "rbit r6, r7",
    "clz r0, r1",
    "pkhbt r0, r1, r2",
    "pkhbt r0, r1, r2, lsl #4",
    "pkhtb r0, r1, r2, asr #1",
    "ssat r0, #1, r1",
    "ssat r0, #32, r1, asr #1",
    "usat r0, #0, r1",
    "usat r3, #15, r4, lsl #5",
    "ssat16 r0, #1, r1",
    "usat16 r2, #15, r3",
    "usad8 r0, r1, r2",
    "usada8 r0, r1, r2, r3",
    "bfc r0, #0, #1",
    "bfi r2, r3, #4, #8",
    "sbfx r2, r3, #4, #8",
    "ubfx r0, r1, #0, #32",
    "ldr r0, [r1]",
    "ldr r0, [r1, #4]",
    "ldr r0, [r1, #-4]",
    "str r2, [r3, #8]!",
    "ldr r0, [r1], #4",
    "ldrb r0, [r1, r2]",
    "ldr r0, [r1, r2, lsl #2]",
    "ldr r0, [r1, -r3, asr #31]!",
    "ldr r0, [r1, r4, rrx]",
    "str r5, [r6], #-100",
    "strb r9, [r10, #4095]",
    "ldrt r0, [r1], #4",
    "strt r2, [r3], #4",
    "ldrbt r4, [r5], #-8",
    "strbt r6, [r7], #12",
    "ldrh r0, [r1]",
    "ldrh r0, [r1, #4]",
    "ldrh r0, [r1, #-8]",
    "strh r2, [r3, #16]!",
    "ldrsb r0, [r1, r2]",
    "ldrsh r4, [r5, #10]",
    "ldrd r0, r1, [r1, #8]",
    "strd r2, r3, [r3, #16]",
    "ldrh r0, [r1], -r3",
    "ldrht r0, [r1], #4",
    "strht r2, [r3], #4",
    "ldrsbt r4, [r5], #4",
    "ldrsht r6, [r7], #4",
    "ldmia r0, {r1, r2, r3}",
    "ldmia r0!, {r1, r2, r3}",
    "push {r4, r5, lr}",
    "pop {r4, r5, pc}",
    "ldmib r0, {r1}",
    "ldmda r0, {r1}",
    "ldmia r0, {r1}^",
    "stmia r9, {r0, r7, r8}",
    "ldrex r0, [r1]",
    "strex r2, r3, [r4]",
    "ldrexb r5, [r6]",
    "strexb r7, r8, [r9]",
    "ldrexh r10, [r11]",
    "strexh r0, r1, [r2]",
    "ldrexd r0, r1, [r3]",
    "strexd r4, r5, r6, [r6]",
    "clrex",
    "swp r0, r1, [r2]",
    "swpb r3, r4, [r5]",
    "b #8",
    "b #-8",
    "bl #4096",
    "blx #4",
    "blx #6",
    "bx r1",
    "blx r0",
    "bxj r2",
    "bne #12",
    "mrs r0, CPSR",
    "mrs r1, SPSR",
    "msr CPSR_f, r0",
    "msr CPSR_cf, r1",
    "msr SPSR_cxsf, r2",
    "msr CPSR_f, #4026531840",
    "cpsie i",
    "cpsid if",
    "cps #19",
    "setend be",
    "setend le",
    "mcr p15, 0, r0, c1, c0, 0",
    "mrc p15, 0, r1, c1, c0, 0",
    "cdp p7, 1, c2, c3, c4, 5",
    "mcrr p15, 5, r0, r1, c2",
    "mrrc p14, 15, r10, r11, c15",
    "ldc p14, c5, [r0, #8]",
    "stc p13, c7, [r2], #400",
    "mcr2 p15, 0, r0, c1, c0, 0",
    "mrc2 p15, 0, r0, c0, c0, 0",
    "cdp2 p7, 1, c2, c3, c4, 5",
    "mcrr2 p8, 5, r2, r3, c4",
    "mrrc2 p9, 10, r4, r5, c6",
    "ldc2l p14, c0, [r3, #-16]",
    "stc2 p15, c15, [r4, #800]!",
    "yield",
    "wfe",
    "wfi",
    "sev",
    "dbg #5",
    "dmb sy",
    "dsb sy",
    "isb sy",
    "bkpt #43981",
    "hvc #4660",
    "smc #5",
    "udf #57005",
    "eret",
    "nop",
];

#[test]
fn emit__a32_forms_gnu() {
    let cases = a32_cases();
    assert_eq!(
        cases.len(),
        EXPECTED_A32_GNU.len(),
        "case/expected table length mismatch"
    );
    for (instruction, expected) in cases.iter().zip(EXPECTED_A32_GNU) {
        assert_eq!(
            &instruction.to_assembly_string(GNU),
            expected,
            "A32 emit mismatch for {instruction:?}"
        );
    }
}

/// Every A32 VFP/NEON instruction whose emit text is asserted below. Each op enum is walked once (to reach
/// every mnemonic arm of the emit helpers), with D/Q and size spreads added where the emit helper branches on
/// them. Constructions mirror the round-trip corpus; order matches `EXPECTED_A32_FP_NEON_GNU`.
fn a32_fp_neon_cases() -> Vec<ArmA32Instruction> {
    use ArmA32Instruction::*;
    let al = Cond::AlwaysUnconditional;

    // ---- VFP load/store ----
    let mut v = vec![
        Vldr_Single_A1(al, s(0), R::R0, 0),
        Vldr_Single_A1(al, s(31), R::R2, -8),
        Vstr_Single_A1(Cond::NotEqual, s(15), R::R1, 1020),
        Vldr_Double_A1(al, d(0), R::R0, 0),
        Vstr_Double_A1(al, d(15), R::R4, -256),
        Vldm_Single_A1(al, R::R0, false, false, s(0), 4),
        Vstm_Single_A1(al, R::R1, true, false, s(8), 2),
        Vldm_Single_A1(al, R::R13, true, false, s(0), 4),
        Vstm_Single_A1(al, R::R13, true, true, s(0), 4),
        Vldm_Double_A1(al, R::R0, false, false, d(0), 2),
        Vstm_Double_A1(al, R::R3, true, true, d(5), 3),
    ];

    // ---- VFP data-processing ----
    for op in [
        F3::Vmla, F3::Vmls, F3::Vnmla, F3::Vnmls, F3::Vmul, F3::Vnmul, F3::Vadd, F3::Vsub, F3::Vdiv,
        F3::Vfnma, F3::Vfnms, F3::Vfma, F3::Vfms,
    ] {
        v.push(FpDataProcess3_Single_A1(al, op, s(0), s(1), s(2)));
        v.push(FpDataProcess3_Double_A1(al, op, d(3), d(4), d(5)));
    }
    for op in [F2::Vmov, F2::Vabs, F2::Vneg, F2::Vsqrt] {
        v.push(FpDataProcess2_Single_A1(al, op, s(6), s(7)));
        v.push(FpDataProcess2_Double_A1(al, op, d(8), d(9)));
    }

    // ---- VFP transfer / compare / immediate ----
    v.push(Vcmp_Single_A1(al, s(0), s(1), false));
    v.push(Vcmp_Single_A1(al, s(2), s(3), true));
    v.push(Vcmp_Double_A1(al, d(0), d(1), false));
    v.push(Vcmp_Zero_Single_A1(al, s(4), true));
    v.push(Vcmp_Zero_Double_A1(al, d(2), false));
    v.push(Vmrs_A1(al, R::R0));
    v.push(Vmrs_Apsr_Nzcv_A1(al));
    v.push(Vmsr_A1(al, R::R1));
    v.push(Vmov_Core_To_Single_A1(al, s(0), R::R1));
    v.push(Vmov_Single_To_Core_A1(al, R::R2, s(3)));
    v.push(Vmov_Immediate_Single_A1(al, s(0), 0x70));
    v.push(Vmov_Immediate_Double_A1(al, d(0), 0x70));
    v.push(Vmov_Double_To_CorePair_A1(al, R::R0, R::R1, d(2)));
    v.push(Vmov_CorePair_To_Double_A1(al, d(3), R::R4, R::R5));
    v.push(Vmov_Singles_To_CorePair_A1(al, R::R6, R::R7, s(8)));
    v.push(Vmov_CorePair_To_Singles_A1(Cond::NotEqual, s(10), R::R2, R::R3));

    // ---- VFP conversions (VCVT) ----
    v.push(Vcvt_FloatToInt_FromSingle_A1(al, s(0), s(1), true, true));
    v.push(Vcvt_FloatToInt_FromSingle_A1(al, s(2), s(3), false, false));
    v.push(Vcvt_FloatToInt_FromDouble_A1(al, s(4), d(5), true, false));
    v.push(Vcvt_IntToFloat_ToSingle_A1(al, s(0), s(1), true));
    v.push(Vcvt_IntToFloat_ToSingle_A1(al, s(6), s(7), false));
    v.push(Vcvt_IntToFloat_ToDouble_A1(al, d(0), s(1), true));
    v.push(Vcvt_Single_To_Double_A1(al, d(0), s(1)));
    v.push(Vcvt_Double_To_Single_A1(al, s(0), d(1)));
    v.push(Vcvt_HalfToSingle_A1(al, s(0), s(1), false));
    v.push(Vcvt_HalfToSingle_A1(Cond::NotEqual, s(2), s(3), true));
    v.push(Vcvt_SingleToHalf_A1(al, s(0), s(1), true));
    v.push(Vcvt_HalfToDouble_A1(al, d(0), s(1), false));
    v.push(Vcvt_HalfToDouble_A1(Cond::NotEqual, d(5), s(20), true));
    v.push(Vcvt_DoubleToHalf_A1(al, s(0), d(1), false));
    v.push(Vcvt_DoubleToHalf_A1(Cond::SignedGreaterThan, s(20), d(5), true));
    v.push(Vcvt_FloatToFixed_Single_A1(al, s(0), true, false, 1));
    v.push(Vcvt_FloatToFixed_Single_A1(al, s(5), false, true, 31));
    v.push(Vcvt_FloatToFixed_Double_A1(al, d(3), false, true, 4));
    v.push(Vcvt_FixedToFloat_Single_A1(al, s(0), true, false, 16));
    v.push(Vcvt_FixedToFloat_Double_A1(al, d(2), false, true, 8));

    // ---- v8 FP (VSEL / VMAXNM / VMINNM / VRINT / VCVT-directed) ----
    v.push(Vsel_Single_A1(Vsel::Equal, s(0), s(1), s(2)));
    v.push(Vsel_Single_A1(Vsel::Overflow, s(6), s(7), s(8)));
    v.push(Vsel_Single_A1(Vsel::GreaterEqual, s(3), s(4), s(5)));
    v.push(Vsel_Single_A1(Vsel::GreaterThan, s(9), s(10), s(11)));
    v.push(Vsel_Double_A1(Vsel::GreaterEqual, d(0), d(1), d(2)));
    v.push(Vmaxnm_Single_A1(s(0), s(1), s(2)));
    v.push(Vmaxnm_Double_A1(d(0), d(1), d(2)));
    v.push(Vminnm_Single_A1(s(3), s(4), s(5)));
    v.push(Vminnm_Double_A1(d(3), d(4), d(5)));
    v.push(Vrint_Directed_Single_A1(DRnd::A, s(0), s(1)));
    v.push(Vrint_Directed_Single_A1(DRnd::N, s(2), s(3)));
    v.push(Vrint_Directed_Single_A1(DRnd::P, s(4), s(5)));
    v.push(Vrint_Directed_Single_A1(DRnd::M, s(6), s(7)));
    v.push(Vrint_Directed_Double_A1(DRnd::P, d(0), d(1)));
    v.push(Vrint_Cond_Single_A1(al, VRnd::R, s(0), s(1)));
    v.push(Vrint_Cond_Single_A1(Cond::NotEqual, VRnd::Z, s(2), s(3)));
    v.push(Vrint_Cond_Single_A1(al, VRnd::X, s(4), s(5)));
    v.push(Vrint_Cond_Double_A1(al, VRnd::R, d(0), d(1)));
    v.push(Vrint_Cond_Double_A1(al, VRnd::Z, d(2), d(3)));
    v.push(Vrint_Cond_Double_A1(al, VRnd::X, d(4), d(5)));
    v.push(Vcvt_Directed_FromSingle_A1(DRnd::A, s(0), s(1), true));
    v.push(Vcvt_Directed_FromSingle_A1(DRnd::N, s(2), s(3), false));
    v.push(Vcvt_Directed_FromSingle_A1(DRnd::P, s(4), s(5), false));
    v.push(Vcvt_Directed_FromSingle_A1(DRnd::M, s(6), s(7), true));
    v.push(Vcvt_Directed_FromDouble_A1(DRnd::A, s(0), d(1), true));
    v.push(Vcvt_Directed_FromDouble_A1(DRnd::M, s(2), d(3), false));

    // ---- NEON 3-reg-same ----
    for op in [
        NInt::Vadd, NInt::Vsub, NInt::Vtst, NInt::Vceq, NInt::Vmla, NInt::Vmls, NInt::Vmul,
        NInt::VmulPoly, NInt::VqaddS, NInt::VqaddU, NInt::VhaddS, NInt::VhaddU, NInt::VqsubS,
        NInt::VqsubU, NInt::VhsubS, NInt::VhsubU, NInt::VrhaddS, NInt::VrhaddU, NInt::VabdS,
        NInt::VabdU, NInt::VabaS, NInt::VabaU, NInt::VmaxS, NInt::VmaxU, NInt::VminS, NInt::VminU,
        NInt::VcgeS, NInt::VcgeU, NInt::VcgtS, NInt::VcgtU, NInt::Vpadd, NInt::VpmaxS, NInt::VpmaxU,
        NInt::VpminS, NInt::VpminU, NInt::VqdmulhS, NInt::VqrdmulhS,
    ] {
        v.push(NeonInt3Same_D_A1(op, NSz::I16, d(1), d(2), d(3)));
    }
    for op in [NInt::Vadd, NInt::VqaddS, NInt::VmaxS] {
        v.push(NeonInt3Same_Q_A1(op, NSz::I32, q(0), q(1), q(2)));
    }
    for size in [NSz::I8, NSz::I32, NSz::I64] {
        v.push(NeonInt3Same_D_A1(NInt::Vadd, size, d(1), d(2), d(3)));
    }
    for op in [
        NFlt::Vadd, NFlt::Vsub, NFlt::Vmul, NFlt::Vmla, NFlt::Vmls, NFlt::Vabd, NFlt::Vpadd,
        NFlt::Vceq, NFlt::Vcge, NFlt::Vcgt, NFlt::Vmax, NFlt::Vmin, NFlt::Vpmax, NFlt::Vpmin,
        NFlt::Vrecps, NFlt::Vrsqrts, NFlt::Vfma, NFlt::Vfms,
    ] {
        v.push(NeonFloat3Same_D_A1(op, d(0), d(1), d(2)));
    }
    v.push(NeonFloat3Same_Q_A1(NFlt::Vadd, q(0), q(1), q(2)));
    for op in [
        NBit::Vand, NBit::Vbic, NBit::Vorr, NBit::Vorn, NBit::Veor, NBit::Vbsl, NBit::Vbit,
        NBit::Vbif,
    ] {
        v.push(NeonBitwise3Same_D_A1(op, d(3), d(4), d(5)));
    }
    v.push(NeonBitwise3Same_Q_A1(NBit::Vand, q(0), q(1), q(2)));

    // ---- NEON 2-reg-misc ----
    for op in [
        NMS::Vrev64, NMS::Vrev32, NMS::Vrev16, NMS::VpaddlS, NMS::VpaddlU, NMS::VclsS, NMS::VclzI,
        NMS::VpadalS, NMS::VpadalU, NMS::VqabsS, NMS::VqnegS, NMS::VcgtZeroS, NMS::VcgeZeroS,
        NMS::VceqZeroI, NMS::VcleZeroS, NMS::VcltZeroS, NMS::VabsS, NMS::VnegS, NMS::Vtrn,
        NMS::Vuzp, NMS::Vzip,
    ] {
        v.push(NeonMisc2Sized_D_A1(op, NSz::I16, d(2), d(3)));
    }
    v.push(NeonMisc2Sized_Q_A1(NMS::Vrev64, NSz::I8, q(0), q(1)));
    for op in [
        NMF::Vmvn, NMF::Vswp, NMF::Vcnt, NMF::VcgtZeroF, NMF::VcgeZeroF, NMF::VceqZeroF,
        NMF::VcleZeroF, NMF::VcltZeroF, NMF::VabsF, NMF::VnegF, NMF::VrintN, NMF::VrintX,
        NMF::VrintA, NMF::VrintZ, NMF::VrintM, NMF::VrintP, NMF::VrecpeU, NMF::VrsqrteU,
        NMF::VrecpeF, NMF::VrsqrteF, NMF::VcvtF32FromS32, NMF::VcvtF32FromU32, NMF::VcvtS32FromF32,
        NMF::VcvtU32FromF32, NMF::VcvtaS, NMF::VcvtaU, NMF::VcvtnS, NMF::VcvtnU, NMF::VcvtpS,
        NMF::VcvtpU, NMF::VcvtmS, NMF::VcvtmU,
    ] {
        v.push(NeonMisc2Fixed_D_A1(op, d(0), d(1)));
    }
    v.push(NeonMisc2Fixed_Q_A1(NMF::Vmvn, q(0), q(1)));
    for op in [NMN::Vmovn, NMN::Vqmovun, NMN::VqmovnS, NMN::VqmovnU] {
        v.push(NeonMisc2Narrow_A1(op, NSz::I16, d(0), q(1)));
    }
    for size in [NSz::I8, NSz::I16, NSz::I32] {
        v.push(NeonShllMax_A1(size, q(2), d(9)));
    }

    // ---- NEON 3-reg-different-length ----
    for op in [
        NDL::VaddlS, NDL::VaddlU, NDL::VsublS, NDL::VsublU, NDL::VabalS, NDL::VabalU, NDL::VabdlS,
        NDL::VabdlU, NDL::VmlalS, NDL::VmlalU, NDL::VmlslS, NDL::VmlslU, NDL::VmullS, NDL::VmullU,
        NDL::VmullP, NDL::Vqdmlal, NDL::Vqdmlsl, NDL::Vqdmull,
    ] {
        v.push(NeonDiffLong_A1(op, NSz::I16, q(3), d(20), d(7)));
    }
    for op in [NDW::VaddwS, NDW::VaddwU, NDW::VsubwS, NDW::VsubwU] {
        v.push(NeonDiffWide_A1(op, NSz::I16, q(2), q(8), d(31)));
    }
    for op in [NDN::Vaddhn, NDN::Vraddhn, NDN::Vsubhn, NDN::Vrsubhn] {
        v.push(NeonDiffNarrow_A1(op, NSz::I16, d(5), q(6), q(11)));
    }

    // ---- NEON 2-reg-and-a-scalar ----
    for op in [NSc::Vmla, NSc::Vmls, NSc::Vmul, NSc::Vqdmulh, NSc::Vqrdmulh] {
        v.push(NeonScalar_D_A1(op, NSz::I16, d(1), d(20), d(6), 2));
    }
    v.push(NeonScalar_Q_A1(NSc::Vmul, NSz::I32, q(2), q(8), d(13), 1));
    for op in [NSc::VmlaF, NSc::VmlsF, NSc::VmulF] {
        v.push(NeonScalar_D_A1(op, NSz::I32, d(0), d(17), d(13), 1));
    }
    for op in [
        NScL::VmlalS, NScL::VmlalU, NScL::VmlslS, NScL::VmlslU, NScL::VmullS, NScL::VmullU,
        NScL::Vqdmlal, NScL::Vqdmlsl, NScL::Vqdmull,
    ] {
        v.push(NeonScalarLong_A1(op, NSz::I16, q(3), d(7), d(6), 2));
    }

    // ---- NEON 2-reg-and-a-shift-amount ----
    for op in [
        NSh::VshrS, NSh::VshrU, NSh::VsraS, NSh::VsraU, NSh::VrshrS, NSh::VrshrU, NSh::VrsraS,
        NSh::VrsraU, NSh::Vsri,
    ] {
        v.push(NeonShift_D_A1(op, NSz::I16, 7, d(1), d(20)));
    }
    v.push(NeonShift_Q_A1(NSh::VshrS, NSz::I32, 20, q(2), q(8)));
    for op in [NSh::Vshl, NSh::Vsli, NSh::Vqshlu, NSh::VqshlS, NSh::VqshlU] {
        v.push(NeonShift_D_A1(op, NSz::I16, 10, d(3), d(17)));
    }
    for op in [
        NShN::Vshrn, NShN::Vrshrn, NShN::Vqshrun, NShN::Vqrshrun, NShN::VqshrnS, NShN::VqrshrnS,
        NShN::VqshrnU, NShN::VqrshrnU,
    ] {
        v.push(NeonShiftNarrow_A1(op, NSz::I16, 3, d(5), q(6)));
    }
    v.push(NeonShiftLong_A1(false, NSz::I8, 3, q(4), d(9)));
    v.push(NeonShiftLong_A1(true, NSz::I16, 9, q(4), d(9)));
    v.push(NeonShiftLong_A1(false, NSz::I8, 0, q(4), d(9))); // VMOVL

    // ---- NEON extract / table / duplicate / modified-immediate ----
    v.push(NeonExt_D_A1(3, d(0), d(1), d(2)));
    v.push(NeonExt_Q_A1(5, q(2), q(8), q(15)));
    for length in 1..=4u8 {
        v.push(NeonTableLookup_A1(false, length, d(0), d(3), d(17)));
    }
    v.push(NeonTableLookup_A1(true, 2, d(0), d(3), d(17)));
    for (size, idx) in [(NSz::I8, 3u8), (NSz::I16, 2), (NSz::I32, 1)] {
        v.push(NeonVdupScalar_D_A1(size, idx, d(5), d(9)));
        v.push(NeonVdupScalar_Q_A1(size, idx, q(3), d(31)));
    }
    for size in [NSz::I8, NSz::I16, NSz::I32] {
        v.push(NeonVdupCore_D_A1(al, size, d(0), R::R1));
        v.push(NeonVdupCore_Q_A1(al, size, q(7), R::R10));
    }
    for cmode in 0..16u8 {
        v.push(NeonModifiedImmediate_D_A1(cmode, false, 0x55, d(4)));
    }
    v.push(NeonModifiedImmediate_D_A1(0, true, 1, d(4)));
    v.push(NeonModifiedImmediate_Q_A1(0, false, 0xFF, q(6)));

    // ---- NEON element/structure load & store ----
    v.push(NeonLoadStoreMultiple_A1(true, 0b0111, NSz::I8, 0, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreMultiple_A1(true, 0b1010, NSz::I16, 0, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreMultiple_A1(true, 0b0110, NSz::I32, 0, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreMultiple_A1(true, 0b0010, NSz::I8, 0, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreMultiple_A1(true, 0b1000, NSz::I8, 0, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreMultiple_A1(true, 0b0100, NSz::I16, 0, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreMultiple_A1(true, 0b0000, NSz::I32, 0, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreMultiple_A1(false, 0b0111, NSz::I8, 1, d(0), R::R0, NLsa::IncrementWriteback));
    v.push(NeonLoadStoreMultiple_A1(true, 0b0111, NSz::I8, 0, d(0), R::R0, NLsa::PostIndexed(R::R2)));
    v.push(NeonLoadStoreSingleLane_A1(true, 1, 0, 6, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreSingleLane_A1(false, 2, 1, 0, d(0), R::R0, NLsa::IncrementWriteback));
    v.push(NeonLoadStoreAllLanes_A1(1, 0, false, false, d(0), R::R0, NLsa::Offset));
    v.push(NeonLoadStoreAllLanes_A1(2, 1, true, true, d(0), R::R0, NLsa::PostIndexed(R::R2)));

    // ---- ARMv8 cryptography extension ----
    for op in [NAes::Aese, NAes::Aesd, NAes::Aesmc, NAes::Aesimc] {
        v.push(NeonAes_A1(op, q(3), q(12)));
    }
    for op in [
        NSha3::Sha1c, NSha3::Sha1p, NSha3::Sha1m, NSha3::Sha1su0, NSha3::Sha256h, NSha3::Sha256h2,
        NSha3::Sha256su1,
    ] {
        v.push(NeonSha3Reg_A1(op, q(1), q(8), q(15)));
    }
    for op in [NSha2::Sha1h, NSha2::Sha1su1, NSha2::Sha256su0] {
        v.push(NeonSha2Reg_A1(op, q(2), q(9)));
    }
    v.push(NeonDiffLong_A1(NDL::VmullP, NSz::I32, q(5), d(20), d(7)));

    // ---- banked MRS / MSR ----
    v.push(MrsBanked_A1(al, false, 5, R::R0));
    v.push(MrsBanked_A1(al, false, 16, R::R1));
    v.push(MrsBanked_A1(al, true, 30, R::R2));
    v.push(MsrBanked_A1(al, false, 5, R::R0));
    v.push(MsrBanked_A1(al, true, 30, R::R2));

    v
}

/// GNU-flavor UAL text for each entry of `a32_fp_neon_cases()`, in the same order. Captured from the emitter
/// and cross-checked against `arm-none-eabi-as`/`objdump` (the differential oracle).
const EXPECTED_A32_FP_NEON_GNU: &[&str] = &[
    "vldr s0, [r0]",
    "vldr s31, [r2, #-8]",
    "vstrne s15, [r1, #1020]",
    "vldr d0, [r0]",
    "vstr d15, [r4, #-256]",
    "vldmia r0, {s0-s3}",
    "vstmia r1!, {s8-s9}",
    "vpop {s0-s3}",
    "vpush {s0-s3}",
    "vldmia r0, {d0-d1}",
    "vstmdb r3!, {d5-d7}",
    "vmla.f32 s0, s1, s2",
    "vmla.f64 d3, d4, d5",
    "vmls.f32 s0, s1, s2",
    "vmls.f64 d3, d4, d5",
    "vnmla.f32 s0, s1, s2",
    "vnmla.f64 d3, d4, d5",
    "vnmls.f32 s0, s1, s2",
    "vnmls.f64 d3, d4, d5",
    "vmul.f32 s0, s1, s2",
    "vmul.f64 d3, d4, d5",
    "vnmul.f32 s0, s1, s2",
    "vnmul.f64 d3, d4, d5",
    "vadd.f32 s0, s1, s2",
    "vadd.f64 d3, d4, d5",
    "vsub.f32 s0, s1, s2",
    "vsub.f64 d3, d4, d5",
    "vdiv.f32 s0, s1, s2",
    "vdiv.f64 d3, d4, d5",
    "vfnma.f32 s0, s1, s2",
    "vfnma.f64 d3, d4, d5",
    "vfnms.f32 s0, s1, s2",
    "vfnms.f64 d3, d4, d5",
    "vfma.f32 s0, s1, s2",
    "vfma.f64 d3, d4, d5",
    "vfms.f32 s0, s1, s2",
    "vfms.f64 d3, d4, d5",
    "vmov.f32 s6, s7",
    "vmov.f64 d8, d9",
    "vabs.f32 s6, s7",
    "vabs.f64 d8, d9",
    "vneg.f32 s6, s7",
    "vneg.f64 d8, d9",
    "vsqrt.f32 s6, s7",
    "vsqrt.f64 d8, d9",
    "vcmp.f32 s0, s1",
    "vcmpe.f32 s2, s3",
    "vcmp.f64 d0, d1",
    "vcmpe.f32 s4, #0",
    "vcmp.f64 d2, #0",
    "vmrs r0, fpscr",
    "vmrs apsr_nzcv, fpscr",
    "vmsr fpscr, r1",
    "vmov s0, r1",
    "vmov r2, s3",
    "vmov.f32 s0, #1.0",
    "vmov.f64 d0, #1.0",
    "vmov r0, r1, d2",
    "vmov d3, r4, r5",
    "vmov r6, r7, s8, s9",
    "vmovne s10, s11, r2, r3",
    "vcvt.s32.f32 s0, s1",
    "vcvtr.u32.f32 s2, s3",
    "vcvtr.s32.f64 s4, d5",
    "vcvt.f32.s32 s0, s1",
    "vcvt.f32.u32 s6, s7",
    "vcvt.f64.s32 d0, s1",
    "vcvt.f64.f32 d0, s1",
    "vcvt.f32.f64 s0, d1",
    "vcvtb.f32.f16 s0, s1",
    "vcvttne.f32.f16 s2, s3",
    "vcvtt.f16.f32 s0, s1",
    "vcvtb.f64.f16 d0, s1",
    "vcvttne.f64.f16 d5, s20",
    "vcvtb.f16.f64 s0, d1",
    "vcvttgt.f16.f64 s20, d5",
    "vcvt.s16.f32 s0, s0, #1",
    "vcvt.u32.f32 s5, s5, #31",
    "vcvt.u32.f64 d3, d3, #4",
    "vcvt.f32.s16 s0, s0, #16",
    "vcvt.f64.u32 d2, d2, #8",
    "vseleq.f32 s0, s1, s2",
    "vselvs.f32 s6, s7, s8",
    "vselge.f32 s3, s4, s5",
    "vselgt.f32 s9, s10, s11",
    "vselge.f64 d0, d1, d2",
    "vmaxnm.f32 s0, s1, s2",
    "vmaxnm.f64 d0, d1, d2",
    "vminnm.f32 s3, s4, s5",
    "vminnm.f64 d3, d4, d5",
    "vrinta.f32 s0, s1",
    "vrintn.f32 s2, s3",
    "vrintp.f32 s4, s5",
    "vrintm.f32 s6, s7",
    "vrintp.f64 d0, d1",
    "vrintr.f32 s0, s1",
    "vrintzne.f32 s2, s3",
    "vrintx.f32 s4, s5",
    "vrintr.f64 d0, d1",
    "vrintz.f64 d2, d3",
    "vrintx.f64 d4, d5",
    "vcvta.s32.f32 s0, s1",
    "vcvtn.u32.f32 s2, s3",
    "vcvtp.u32.f32 s4, s5",
    "vcvtm.s32.f32 s6, s7",
    "vcvta.s32.f64 s0, d1",
    "vcvtm.u32.f64 s2, d3",
    "vadd.i16 d1, d2, d3",
    "vsub.i16 d1, d2, d3",
    "vtst.16 d1, d2, d3",
    "vceq.i16 d1, d2, d3",
    "vmla.i16 d1, d2, d3",
    "vmls.i16 d1, d2, d3",
    "vmul.i16 d1, d2, d3",
    "vmul.p16 d1, d2, d3",
    "vqadd.s16 d1, d2, d3",
    "vqadd.u16 d1, d2, d3",
    "vhadd.s16 d1, d2, d3",
    "vhadd.u16 d1, d2, d3",
    "vqsub.s16 d1, d2, d3",
    "vqsub.u16 d1, d2, d3",
    "vhsub.s16 d1, d2, d3",
    "vhsub.u16 d1, d2, d3",
    "vrhadd.s16 d1, d2, d3",
    "vrhadd.u16 d1, d2, d3",
    "vabd.s16 d1, d2, d3",
    "vabd.u16 d1, d2, d3",
    "vaba.s16 d1, d2, d3",
    "vaba.u16 d1, d2, d3",
    "vmax.s16 d1, d2, d3",
    "vmax.u16 d1, d2, d3",
    "vmin.s16 d1, d2, d3",
    "vmin.u16 d1, d2, d3",
    "vcge.s16 d1, d2, d3",
    "vcge.u16 d1, d2, d3",
    "vcgt.s16 d1, d2, d3",
    "vcgt.u16 d1, d2, d3",
    "vpadd.i16 d1, d2, d3",
    "vpmax.s16 d1, d2, d3",
    "vpmax.u16 d1, d2, d3",
    "vpmin.s16 d1, d2, d3",
    "vpmin.u16 d1, d2, d3",
    "vqdmulh.s16 d1, d2, d3",
    "vqrdmulh.s16 d1, d2, d3",
    "vadd.i32 q0, q1, q2",
    "vqadd.s32 q0, q1, q2",
    "vmax.s32 q0, q1, q2",
    "vadd.i8 d1, d2, d3",
    "vadd.i32 d1, d2, d3",
    "vadd.i64 d1, d2, d3",
    "vadd.f32 d0, d1, d2",
    "vsub.f32 d0, d1, d2",
    "vmul.f32 d0, d1, d2",
    "vmla.f32 d0, d1, d2",
    "vmls.f32 d0, d1, d2",
    "vabd.f32 d0, d1, d2",
    "vpadd.f32 d0, d1, d2",
    "vceq.f32 d0, d1, d2",
    "vcge.f32 d0, d1, d2",
    "vcgt.f32 d0, d1, d2",
    "vmax.f32 d0, d1, d2",
    "vmin.f32 d0, d1, d2",
    "vpmax.f32 d0, d1, d2",
    "vpmin.f32 d0, d1, d2",
    "vrecps.f32 d0, d1, d2",
    "vrsqrts.f32 d0, d1, d2",
    "vfma.f32 d0, d1, d2",
    "vfms.f32 d0, d1, d2",
    "vadd.f32 q0, q1, q2",
    "vand d3, d4, d5",
    "vbic d3, d4, d5",
    "vorr d3, d4, d5",
    "vorn d3, d4, d5",
    "veor d3, d4, d5",
    "vbsl d3, d4, d5",
    "vbit d3, d4, d5",
    "vbif d3, d4, d5",
    "vand q0, q1, q2",
    "vrev64.16 d2, d3",
    "vrev32.16 d2, d3",
    "vrev16.16 d2, d3",
    "vpaddl.s16 d2, d3",
    "vpaddl.u16 d2, d3",
    "vcls.s16 d2, d3",
    "vclz.i16 d2, d3",
    "vpadal.s16 d2, d3",
    "vpadal.u16 d2, d3",
    "vqabs.s16 d2, d3",
    "vqneg.s16 d2, d3",
    "vcgt.s16 d2, d3, #0",
    "vcge.s16 d2, d3, #0",
    "vceq.i16 d2, d3, #0",
    "vcle.s16 d2, d3, #0",
    "vclt.s16 d2, d3, #0",
    "vabs.s16 d2, d3",
    "vneg.s16 d2, d3",
    "vtrn.16 d2, d3",
    "vuzp.16 d2, d3",
    "vzip.16 d2, d3",
    "vrev64.8 q0, q1",
    "vmvn d0, d1",
    "vswp d0, d1",
    "vcnt.8 d0, d1",
    "vcgt.f32 d0, d1, #0",
    "vcge.f32 d0, d1, #0",
    "vceq.f32 d0, d1, #0",
    "vcle.f32 d0, d1, #0",
    "vclt.f32 d0, d1, #0",
    "vabs.f32 d0, d1",
    "vneg.f32 d0, d1",
    "vrintn.f32 d0, d1",
    "vrintx.f32 d0, d1",
    "vrinta.f32 d0, d1",
    "vrintz.f32 d0, d1",
    "vrintm.f32 d0, d1",
    "vrintp.f32 d0, d1",
    "vrecpe.u32 d0, d1",
    "vrsqrte.u32 d0, d1",
    "vrecpe.f32 d0, d1",
    "vrsqrte.f32 d0, d1",
    "vcvt.f32.s32 d0, d1",
    "vcvt.f32.u32 d0, d1",
    "vcvt.s32.f32 d0, d1",
    "vcvt.u32.f32 d0, d1",
    "vcvta.s32.f32 d0, d1",
    "vcvta.u32.f32 d0, d1",
    "vcvtn.s32.f32 d0, d1",
    "vcvtn.u32.f32 d0, d1",
    "vcvtp.s32.f32 d0, d1",
    "vcvtp.u32.f32 d0, d1",
    "vcvtm.s32.f32 d0, d1",
    "vcvtm.u32.f32 d0, d1",
    "vmvn q0, q1",
    "vmovn.i16 d0, q1",
    "vqmovun.s16 d0, q1",
    "vqmovn.s16 d0, q1",
    "vqmovn.u16 d0, q1",
    "vshll.i8 q2, d9, #8",
    "vshll.i16 q2, d9, #16",
    "vshll.i32 q2, d9, #32",
    "vaddl.s16 q3, d20, d7",
    "vaddl.u16 q3, d20, d7",
    "vsubl.s16 q3, d20, d7",
    "vsubl.u16 q3, d20, d7",
    "vabal.s16 q3, d20, d7",
    "vabal.u16 q3, d20, d7",
    "vabdl.s16 q3, d20, d7",
    "vabdl.u16 q3, d20, d7",
    "vmlal.s16 q3, d20, d7",
    "vmlal.u16 q3, d20, d7",
    "vmlsl.s16 q3, d20, d7",
    "vmlsl.u16 q3, d20, d7",
    "vmull.s16 q3, d20, d7",
    "vmull.u16 q3, d20, d7",
    "vmull.p16 q3, d20, d7",
    "vqdmlal.s16 q3, d20, d7",
    "vqdmlsl.s16 q3, d20, d7",
    "vqdmull.s16 q3, d20, d7",
    "vaddw.s16 q2, q8, d31",
    "vaddw.u16 q2, q8, d31",
    "vsubw.s16 q2, q8, d31",
    "vsubw.u16 q2, q8, d31",
    "vaddhn.i16 d5, q6, q11",
    "vraddhn.i16 d5, q6, q11",
    "vsubhn.i16 d5, q6, q11",
    "vrsubhn.i16 d5, q6, q11",
    "vmla.i16 d1, d20, d6[2]",
    "vmls.i16 d1, d20, d6[2]",
    "vmul.i16 d1, d20, d6[2]",
    "vqdmulh.s16 d1, d20, d6[2]",
    "vqrdmulh.s16 d1, d20, d6[2]",
    "vmul.i32 q2, q8, d13[1]",
    "vmla.f32 d0, d17, d13[1]",
    "vmls.f32 d0, d17, d13[1]",
    "vmul.f32 d0, d17, d13[1]",
    "vmlal.s16 q3, d7, d6[2]",
    "vmlal.u16 q3, d7, d6[2]",
    "vmlsl.s16 q3, d7, d6[2]",
    "vmlsl.u16 q3, d7, d6[2]",
    "vmull.s16 q3, d7, d6[2]",
    "vmull.u16 q3, d7, d6[2]",
    "vqdmlal.s16 q3, d7, d6[2]",
    "vqdmlsl.s16 q3, d7, d6[2]",
    "vqdmull.s16 q3, d7, d6[2]",
    "vshr.s16 d1, d20, #7",
    "vshr.u16 d1, d20, #7",
    "vsra.s16 d1, d20, #7",
    "vsra.u16 d1, d20, #7",
    "vrshr.s16 d1, d20, #7",
    "vrshr.u16 d1, d20, #7",
    "vrsra.s16 d1, d20, #7",
    "vrsra.u16 d1, d20, #7",
    "vsri.16 d1, d20, #7",
    "vshr.s32 q2, q8, #20",
    "vshl.i16 d3, d17, #10",
    "vsli.16 d3, d17, #10",
    "vqshlu.s16 d3, d17, #10",
    "vqshl.s16 d3, d17, #10",
    "vqshl.u16 d3, d17, #10",
    "vshrn.i16 d5, q6, #3",
    "vrshrn.i16 d5, q6, #3",
    "vqshrun.s16 d5, q6, #3",
    "vqrshrun.s16 d5, q6, #3",
    "vqshrn.s16 d5, q6, #3",
    "vqrshrn.s16 d5, q6, #3",
    "vqshrn.u16 d5, q6, #3",
    "vqrshrn.u16 d5, q6, #3",
    "vshll.s8 q4, d9, #3",
    "vshll.u16 q4, d9, #9",
    "vmovl.s8 q4, d9",
    "vext.8 d0, d1, d2, #3",
    "vext.8 q2, q8, q15, #5",
    "vtbl.8 d0, {d3}, d17",
    "vtbl.8 d0, {d3-d4}, d17",
    "vtbl.8 d0, {d3-d5}, d17",
    "vtbl.8 d0, {d3-d6}, d17",
    "vtbx.8 d0, {d3-d4}, d17",
    "vdup.8 d5, d9[3]",
    "vdup.8 q3, d31[3]",
    "vdup.16 d5, d9[2]",
    "vdup.16 q3, d31[2]",
    "vdup.32 d5, d9[1]",
    "vdup.32 q3, d31[1]",
    "vdup.8 d0, r1",
    "vdup.8 q7, r10",
    "vdup.16 d0, r1",
    "vdup.16 q7, r10",
    "vdup.32 d0, r1",
    "vdup.32 q7, r10",
    "vmov.i32 d4, #0x55",
    "vorr.i32 d4, #0x55",
    "vmov.i32 d4, #0x5500",
    "vorr.i32 d4, #0x5500",
    "vmov.i32 d4, #0x550000",
    "vorr.i32 d4, #0x550000",
    "vmov.i32 d4, #0x55000000",
    "vorr.i32 d4, #0x55000000",
    "vmov.i16 d4, #0x55",
    "vorr.i16 d4, #0x55",
    "vmov.i16 d4, #0x5500",
    "vorr.i16 d4, #0x5500",
    "vmov.i32 d4, #0x55ff",
    "vmov.i32 d4, #0x55ffff",
    "vmov.i8 d4, #0x55",
    "vmov.f32 d4, #0.328125",
    "vmvn.i32 d4, #0x1",
    "vmov.i32 q6, #0xff",
    "vld1.8 {d0}, [r0]",
    "vld1.16 {d0-d1}, [r0]",
    "vld1.32 {d0-d2}, [r0]",
    "vld1.8 {d0-d3}, [r0]",
    "vld2.8 {d0-d1}, [r0]",
    "vld3.16 {d0-d2}, [r0]",
    "vld4.32 {d0-d3}, [r0]",
    "vst1.8 {d0}, [r0:64]!",
    "vld1.8 {d0}, [r0], r2",
    "vld1.8 {d0[3]}, [r0]",
    "vst2.16 {d0[0], d1[0]}, [r0]!",
    "vld1.8 {d0[]}, [r0]",
    "vld2.16 {d0[], d2[]}, [r0], r2",
    "aese.8 q3, q12",
    "aesd.8 q3, q12",
    "aesmc.8 q3, q12",
    "aesimc.8 q3, q12",
    "sha1c.32 q1, q8, q15",
    "sha1p.32 q1, q8, q15",
    "sha1m.32 q1, q8, q15",
    "sha1su0.32 q1, q8, q15",
    "sha256h.32 q1, q8, q15",
    "sha256h2.32 q1, q8, q15",
    "sha256su1.32 q1, q8, q15",
    "sha1h.32 q2, q9",
    "sha1su1.32 q2, q9",
    "sha256su0.32 q2, q9",
    "vmull.p64 q5, d20, d7",
    "mrs r0, SP_usr",
    "mrs r1, LR_irq",
    "mrs r2, SPSR_hyp",
    "msr SP_usr, r0",
    "msr SPSR_hyp, r2",
];

#[test]
fn emit__a32_fp_neon_forms_gnu() {
    let cases = a32_fp_neon_cases();
    assert_eq!(
        cases.len(),
        EXPECTED_A32_FP_NEON_GNU.len(),
        "case/expected table length mismatch"
    );
    for (instruction, expected) in cases.iter().zip(EXPECTED_A32_FP_NEON_GNU) {
        assert_eq!(
            &instruction.to_assembly_string(GNU),
            expected,
            "A32 FP/NEON emit mismatch for {instruction:?}"
        );
    }
}
