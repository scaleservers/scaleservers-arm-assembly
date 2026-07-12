// Copyright (c) Scaleservers LLC

// ArmA32Instruction -- the encoding-faithful model of the ARM **A32** ("ARM" state) instruction set: the
// fixed-width 32-bit encoding used by A/R-profile cores (Cortex-A / Cortex-R) and the classic ARM cores.
// It is the sibling of `ArmT32Instruction` (Thumb / T32). Keeping A32 a SEPARATE type from T32 is
// deliberate: which instruction set a code stream uses is a property of the stream, not the CPU
// (Cortex-A/R run both), so the *type system* -- not a runtime check -- is what stops a compiler backend
// from emitting the wrong set into a stream. The outer `Arm32Instruction` enum re-unites the two for the
// disassembler, which must follow ARM/Thumb interworking within a single binary.
//
// Every A32 instruction is exactly one little-endian 32-bit word, and bits[31:28] are the condition code.
// `encode()` builds the word and returns its 4 LE bytes; `decode()` reads one LE word and matches it
// against (mask, pattern) pairs. Authority: the ARM Architecture Reference Manual (ARMv7-A/R and ARMv8
// AArch32), the instruction pages and their encoding diagrams.

#![allow(non_camel_case_types)]

// `Vec` is not in the `no_std` prelude; pull it from `alloc`.
use alloc::vec::Vec;
use crate::DecodeError;
use crate::EncodeError;
use crate::enums::{
    Arm32BlockAddressMode,
    Arm32Condition,
    Arm32CpsMode,
    Arm32DirectedRound,
    Arm32ExtendType,
    Arm32FpDataOperation3,
    Arm32FpDataOperation2,
    Arm32GeneralPurposeRegister,
    Arm32VrintMode,
    Arm32VselCondition,
    Arm32IndexMode,
    Arm32MemoryOffset,
    Arm32MemoryOffset8,
    Arm32NeonSize,
    Arm32NeonIntegerOp,
    Arm32NeonFloatOp,
    Arm32NeonBitwiseOp,
    Arm32NeonMisc2SizedOp,
    Arm32NeonMisc2FixedOp,
    Arm32NeonNarrowOp,
    Arm32NeonDiffLongOp,
    Arm32NeonDiffWideOp,
    Arm32NeonDiffNarrowOp,
    Arm32NeonScalarOp,
    Arm32NeonScalarLongOp,
    Arm32NeonShiftOp,
    Arm32NeonShiftNarrowOp,
    Arm32NeonLoadStoreAddress,
    Arm32NeonAesOp,
    Arm32NeonSha3Op,
    Arm32NeonSha2Op,
    Arm32ParallelOperation,
    Arm32ParallelPrefix,
    Arm32RegisterShift,
    Arm32ShiftType,
    Arm32SinglePrecisionRegister,
    Arm32DoublePrecisionRegister,
    Arm32VmovLaneSize,
    Arm32QuadwordRegister,
};
use crate::targets::{
    ArmCpuFeature,
    ArmInstructionRequirement,
    ArmIsaVersion,
    ArmTargetProfile,
};

#[derive(Debug, PartialEq)]
pub enum ArmA32Instruction {
    // ======================= data processing =======================
    // The three operand forms of each data-processing opcode: immediate (ARM modified immediate),
    // register (Rm with an immediate barrel shift), and register-shifted register. MOV/MVN take no Rn;
    // the compares (TST/TEQ/CMP/CMN) take no Rd and always set flags.

    // -- ops producing Rd from (Rn, operand2) --  opcode AND=0 EOR=1 SUB=2 RSB=3 ADD=4 ADC=5 SBC=6 RSC=7 ORR=12 BIC=14
    And_Immediate_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*imm32*/ u32),
    And_Register_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*shift*/ Arm32RegisterShift),
    Eor_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Eor_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Sub_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Sub_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Rsb_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Rsb_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Add_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Add_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Adc_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Adc_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Sbc_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Sbc_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Rsc_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Rsc_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Orr_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Orr_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Bic_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u32),
    Bic_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),

    // -- ops producing Rd from operand2 only (MOV=13, MVN=15) --  the LSL/LSR/ASR/ROR/RRX register aliases are Mov_Register_A1
    Mov_Immediate_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rd*/ Arm32GeneralPurposeRegister, /*imm32*/ u32),
    Mov_Register_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rd*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*shift*/ Arm32RegisterShift),
    Mvn_Immediate_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, u32),
    Mvn_Register_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),

    // -- compares: (Rn, operand2), no Rd, always set flags (TST=8 TEQ=9 CMP=10 CMN=11) --
    Tst_Immediate_A1(/*cond*/ Arm32Condition, /*rn*/ Arm32GeneralPurposeRegister, /*imm32*/ u32),
    Tst_Register_A1(/*cond*/ Arm32Condition, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*shift*/ Arm32RegisterShift),
    Teq_Immediate_A1(Arm32Condition, Arm32GeneralPurposeRegister, u32),
    Teq_Register_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Cmp_Immediate_A1(Arm32Condition, Arm32GeneralPurposeRegister, u32),
    Cmp_Register_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),
    Cmn_Immediate_A1(Arm32Condition, Arm32GeneralPurposeRegister, u32),
    Cmn_Register_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32RegisterShift),

    // -- ops (register-shifted register): the barrel-shift amount comes from a register Rs --
    And_RegisterShiftedRegister_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*type*/ Arm32ShiftType, /*rs*/ Arm32GeneralPurposeRegister),
    Eor_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Sub_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Rsb_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Add_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Adc_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Sbc_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Rsc_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Orr_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Bic_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Mov_RegisterShiftedRegister_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rd*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*type*/ Arm32ShiftType, /*rs*/ Arm32GeneralPurposeRegister),
    Mvn_RegisterShiftedRegister_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Tst_RegisterShiftedRegister_A1(/*cond*/ Arm32Condition, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*type*/ Arm32ShiftType, /*rs*/ Arm32GeneralPurposeRegister),
    Teq_RegisterShiftedRegister_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Cmp_RegisterShiftedRegister_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),
    Cmn_RegisterShiftedRegister_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32ShiftType, Arm32GeneralPurposeRegister),

    // -- 16-bit immediate moves --
    Movw_A2(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*imm16*/ u16),
    Movt_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*imm16*/ u16),

    // ======================= multiply =======================
    // operand order follows UAL: MUL Rd, Rn, Rm ; MLA Rd, Rn, Rm, Ra ; UMULL RdLo, RdHi, Rn, Rm
    Mul_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister),
    Mla_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*ra*/ Arm32GeneralPurposeRegister),
    Mls_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*ra*/ Arm32GeneralPurposeRegister),
    Umull_A1(/*cond*/ Arm32Condition, /*S*/ bool, /*rdlo*/ Arm32GeneralPurposeRegister, /*rdhi*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister),
    Umlal_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Smull_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Smlal_A1(Arm32Condition, bool, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Umaal_A1(/*cond*/ Arm32Condition, /*rdlo*/ Arm32GeneralPurposeRegister, /*rdhi*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister),

    // ======================= saturating arithmetic (DSP) =======================
    // UAL operand order is Rd, Rm, Rn (Rn is the saturated/doubled operand)
    Qadd_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Qsub_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Qdadd_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Qdsub_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),

    // ======================= signed multiply (DSP) =======================
    // halfword multiplies: n selects the Rn half (x: false=bottom,true=top), m selects the Rm half (y)
    Smla_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*ra*/ Arm32GeneralPurposeRegister, /*n*/ bool, /*m*/ bool),
    Smlaw_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*m*/ bool),
    Smulw_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*m*/ bool),
    Smlal_Halfword_A1(/*cond*/ Arm32Condition, /*rdlo*/ Arm32GeneralPurposeRegister, /*rdhi*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*n*/ bool, /*m*/ bool),
    Smul_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*n*/ bool, /*m*/ bool),
    // dual / most-significant-word multiplies: x = exchange, round = rounding
    Smlad_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*x*/ bool),
    Smuad_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*x*/ bool),
    Smlsd_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*x*/ bool),
    Smusd_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*x*/ bool),
    Smmla_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*round*/ bool),
    Smmul_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*round*/ bool),
    Smmls_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*round*/ bool),
    Smlald_A1(/*cond*/ Arm32Condition, /*rdlo*/ Arm32GeneralPurposeRegister, /*rdhi*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*x*/ bool),
    Smlsld_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, /*x*/ bool),

    // ======================= parallel (packed SIMD) add/sub + select =======================
    // 36 instructions = 6 operations x 6 signed/unsigned prefixes, all Rd, Rn, Rm
    ParallelAddSub_A1(/*cond*/ Arm32Condition, /*op*/ Arm32ParallelOperation, /*prefix*/ Arm32ParallelPrefix, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister),
    Sel_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister),

    // ======================= extend / extend-and-add =======================
    // rotation is the DECODED amount (0 / 8 / 16 / 24). Extend uses Rn=PC internally; extend-and-add adds Rn.
    Extend_A1(/*cond*/ Arm32Condition, /*type*/ Arm32ExtendType, /*rd*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*rotation*/ u8),
    ExtendAndAdd_A1(/*cond*/ Arm32Condition, /*type*/ Arm32ExtendType, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*rotation*/ u8),

    // ======================= byte/bit reverse + count leading zeros =======================
    Rev_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Rev16_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Revsh_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Rbit_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Clz_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),

    // ======================= pack / saturate / sum-of-absolute-differences =======================
    Pkhbt_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*lsl 0..=31*/ u8),
    Pkhtb_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*asr 1..=32*/ u8),
    Ssat_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*sat 1..=32*/ u8, /*rm*/ Arm32GeneralPurposeRegister, /*shift*/ Arm32RegisterShift),
    Usat_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*sat 0..=31*/ u8, /*rm*/ Arm32GeneralPurposeRegister, /*shift*/ Arm32RegisterShift),
    Ssat16_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*sat 1..=16*/ u8, /*rm*/ Arm32GeneralPurposeRegister),
    Usat16_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*sat 0..=15*/ u8, /*rm*/ Arm32GeneralPurposeRegister),
    Usad8_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister),
    Usada8_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister, /*ra*/ Arm32GeneralPurposeRegister),

    // ======================= bitfield =======================
    Bfc_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*lsb*/ u8, /*width*/ u8),
    Bfi_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*lsb*/ u8, /*width*/ u8),
    Sbfx_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*lsb*/ u8, /*width*/ u8),
    Ubfx_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*lsb*/ u8, /*width*/ u8),

    // ======================= load/store single (word/byte) =======================
    // a PC (R15) base with an immediate offset is the literal form (`ldr rt, [pc, #imm]`)
    Ldr_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*offset*/ Arm32MemoryOffset, /*index*/ Arm32IndexMode),
    Str_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset, Arm32IndexMode),
    Ldrb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset, Arm32IndexMode),
    Strb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset, Arm32IndexMode),
    // unprivileged (P=0, W=1) -- always post-indexed
    Ldrt_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*offset*/ Arm32MemoryOffset),
    Strt_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset),
    Ldrbt_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset),
    Strbt_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset),

    // ======================= load/store halfword / dual / signed =======================
    // LDRD/STRD operate on the register pair (Rt, Rt+1); only Rt is stored (Rt2 = Rt+1 is implicit)
    Ldrh_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*offset*/ Arm32MemoryOffset8, /*index*/ Arm32IndexMode),
    Strh_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset8, Arm32IndexMode),
    Ldrsb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset8, Arm32IndexMode),
    Ldrsh_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset8, Arm32IndexMode),
    Ldrd_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*offset*/ Arm32MemoryOffset8, /*index*/ Arm32IndexMode),
    Strd_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset8, Arm32IndexMode),
    // unprivileged "T" forms (P=0, W=1) -- no dual form
    Ldrht_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*offset*/ Arm32MemoryOffset8),
    Strht_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset8),
    Ldrsbt_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset8),
    Ldrsht_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32MemoryOffset8),

    // ======================= load/store multiple =======================
    // PUSH = STMDB sp!, POP = LDMIA sp! (rendered by the emitter); user_mode is the `^` (S) bit
    Ldm_A1(/*cond*/ Arm32Condition, /*mode*/ Arm32BlockAddressMode, /*rn*/ Arm32GeneralPurposeRegister, /*writeback*/ bool, /*user_mode*/ bool, /*registers*/ Vec<Arm32GeneralPurposeRegister>),
    Stm_A1(/*cond*/ Arm32Condition, /*mode*/ Arm32BlockAddressMode, /*rn*/ Arm32GeneralPurposeRegister, /*writeback*/ bool, /*user_mode*/ bool, /*registers*/ Vec<Arm32GeneralPurposeRegister>),

    // ======================= synchronization =======================
    // exclusive access (LDREXD/STREXD operate on the pair (Rt, Rt+1) -- only Rt is stored)
    Ldrex_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Strex_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Ldrexb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Strexb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Ldrexh_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Strexh_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Ldrexd_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Strexd_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Clrex_A1, // unconditional
    // the deprecated swap
    Swp_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rt2*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Swpb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),

    // ======================= status / system register access =======================
    // spsr=false -> CPSR, true -> SPSR. The MSR field_mask is the 4-bit {f,s,x,c} at bits[19:16].
    Mrs_A1(/*cond*/ Arm32Condition, /*spsr*/ bool, /*rd*/ Arm32GeneralPurposeRegister),
    Msr_Register_A1(/*cond*/ Arm32Condition, /*spsr*/ bool, /*field_mask*/ u8, /*rn*/ Arm32GeneralPurposeRegister),
    Msr_Immediate_A1(/*cond*/ Arm32Condition, /*spsr*/ bool, /*field_mask*/ u8, /*imm32*/ u32),
    // Banked register transfer (ARMv7VE Virtualization Extensions): `spsr` selects the SPSR vs the GPR bank,
    // `sysm` is the 5-bit SYSm (m:m1) that names the banked register (e.g. SP_usr = 5, LR_irq = 16, ELR_hyp = 30).
    MrsBanked_A1(/*cond*/ Arm32Condition, /*spsr*/ bool, /*sysm*/ u8, /*rd*/ Arm32GeneralPurposeRegister),
    MsrBanked_A1(/*cond*/ Arm32Condition, /*spsr*/ bool, /*sysm*/ u8, /*rn*/ Arm32GeneralPurposeRegister),
    Cps_A1(/*mode*/ Arm32CpsMode, /*a*/ bool, /*i*/ bool, /*f*/ bool, /*new_mode*/ Option<u8>), // unconditional
    Setend_A1(/*big_endian*/ bool), // unconditional

    // ======================= coprocessor =======================
    // coproc / opc1 / opc2 / CRn / CRd / CRm are raw field values (coprocessor registers are 4-bit numbers).
    // The "2" variants are the unconditional (cond=1111) encodings, so they carry no condition.
    Mcr_A1(/*cond*/ Arm32Condition, /*coproc*/ u8, /*opc1*/ u8, /*rt*/ Arm32GeneralPurposeRegister, /*crn*/ u8, /*crm*/ u8, /*opc2*/ u8),
    Mrc_A1(/*cond*/ Arm32Condition, /*coproc*/ u8, /*opc1*/ u8, /*rt*/ Arm32GeneralPurposeRegister, /*crn*/ u8, /*crm*/ u8, /*opc2*/ u8),
    Mcr2_A1(/*coproc*/ u8, /*opc1*/ u8, /*rt*/ Arm32GeneralPurposeRegister, /*crn*/ u8, /*crm*/ u8, /*opc2*/ u8),
    Mrc2_A1(u8, u8, Arm32GeneralPurposeRegister, u8, u8, u8),
    Cdp_A1(/*cond*/ Arm32Condition, /*coproc*/ u8, /*opc1*/ u8, /*crd*/ u8, /*crn*/ u8, /*crm*/ u8, /*opc2*/ u8),
    Cdp2_A1(/*coproc*/ u8, /*opc1*/ u8, /*crd*/ u8, /*crn*/ u8, /*crm*/ u8, /*opc2*/ u8),
    Mcrr_A1(/*cond*/ Arm32Condition, /*coproc*/ u8, /*opc1*/ u8, /*rt*/ Arm32GeneralPurposeRegister, /*rt2*/ Arm32GeneralPurposeRegister, /*crm*/ u8),
    Mrrc_A1(/*cond*/ Arm32Condition, /*coproc*/ u8, /*opc1*/ u8, /*rt*/ Arm32GeneralPurposeRegister, /*rt2*/ Arm32GeneralPurposeRegister, /*crm*/ u8),
    Mcrr2_A1(/*coproc*/ u8, /*opc1*/ u8, /*rt*/ Arm32GeneralPurposeRegister, /*rt2*/ Arm32GeneralPurposeRegister, /*crm*/ u8),
    Mrrc2_A1(u8, u8, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, u8),
    // LDC/STC: long is the N bit; offset is imm8 scaled by 4, with an Arm32IndexMode + add sign
    Ldc_A1(/*cond*/ Arm32Condition, /*coproc*/ u8, /*long*/ bool, /*crd*/ u8, /*rn*/ Arm32GeneralPurposeRegister, /*add*/ bool, /*imm8*/ u8, /*index*/ Arm32IndexMode),
    Stc_A1(Arm32Condition, u8, bool, u8, Arm32GeneralPurposeRegister, bool, u8, Arm32IndexMode),
    Ldc2_A1(/*coproc*/ u8, /*long*/ bool, /*crd*/ u8, /*rn*/ Arm32GeneralPurposeRegister, /*add*/ bool, /*imm8*/ u8, /*index*/ Arm32IndexMode),
    Stc2_A1(u8, bool, u8, Arm32GeneralPurposeRegister, bool, u8, Arm32IndexMode),

    // ======================= hints =======================
    Nop_A1(/*cond*/ Arm32Condition),
    Yield_A1(/*cond*/ Arm32Condition),
    Wfe_A1(/*cond*/ Arm32Condition),
    Wfi_A1(/*cond*/ Arm32Condition),
    Sev_A1(/*cond*/ Arm32Condition),
    Dbg_A1(/*cond*/ Arm32Condition, /*option*/ u8),
    /// `CSDB` -- Consumption of Speculative Data Barrier (a NOP-compatible hint; the A32 sibling of `Csdb_T1`).
    Csdb_A1(/*cond*/ Arm32Condition),
    /// `ESB` -- Error Synchronization Barrier (FEAT_RAS; NOPs on cores without RAS). The A32 sibling of `Esb_T1`.
    Esb_A1(/*cond*/ Arm32Condition),

    // ======================= memory barriers (unconditional) =======================
    Dmb_A1(/*option (4-bit barrier type; sy=0xF)*/ u8),
    Dsb_A1(/*option*/ u8),
    Isb_A1(/*option*/ u8),
    /// SB -- Speculation Barrier (FEAT_SB, ARMv8-A). Unconditional; fixed word `0xF57F_F070`.
    Sb_A1,

    // ======================= exception generation =======================
    Bkpt_A1(/*cond*/ Arm32Condition, /*imm16*/ u16),
    Hvc_A1(/*cond*/ Arm32Condition, /*imm16*/ u16),
    Smc_A1(/*cond*/ Arm32Condition, /*imm4*/ u8),
    Udf_A1(/*cond*/ Arm32Condition, /*imm16*/ u16),
    Eret_A1(/*cond*/ Arm32Condition),
    Sevl_A1(/*cond*/ Arm32Condition), // ARMv8 hint (hint8 = 5)

    // ======================= ARMv8-A AArch32 additions: CRC32 =======================
    Crc32b_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister, /*rm*/ Arm32GeneralPurposeRegister),
    Crc32h_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Crc32w_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Crc32cb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Crc32ch_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Crc32cw_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),

    // ======================= ARMv8-A AArch32 additions: load-acquire / store-release =======================
    Lda_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Ldab_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Ldah_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Stl_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Stlb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Stlh_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Ldaex_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Ldaexb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Ldaexh_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Ldaexd_A1(/*cond*/ Arm32Condition, /*rt (rt2=rt+1)*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Stlex_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rt*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),
    Stlexb_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Stlexh_A1(Arm32Condition, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister, Arm32GeneralPurposeRegister),
    Stlexd_A1(/*cond*/ Arm32Condition, /*rd*/ Arm32GeneralPurposeRegister, /*rt (rt2=rt+1)*/ Arm32GeneralPurposeRegister, /*rn*/ Arm32GeneralPurposeRegister),

    // ======================= floating-point (VFP) load/store =======================
    // S0-S31 / D0-D15. VFP shares the coprocessor encoding (coproc=1010 single / 1011 double); the offset
    // is a byte displacement (multiple of 4, +/-1020). VPUSH/VPOP are VSTMDB/VLDMIA sp! spellings (the emitter
    // renders them). VLDM/VSTM carry a base register, writeback, decrement-before, the first FP register and
    // a count.
    Vldr_Single_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*rn*/ Arm32GeneralPurposeRegister, /*offset*/ i32),
    Vstr_Single_A1(Arm32Condition, Arm32SinglePrecisionRegister, Arm32GeneralPurposeRegister, i32),
    Vldr_Double_A1(/*cond*/ Arm32Condition, /*dd*/ Arm32DoublePrecisionRegister, /*rn*/ Arm32GeneralPurposeRegister, /*offset*/ i32),
    Vstr_Double_A1(Arm32Condition, Arm32DoublePrecisionRegister, Arm32GeneralPurposeRegister, i32),
    Vldm_Single_A1(/*cond*/ Arm32Condition, /*rn*/ Arm32GeneralPurposeRegister, /*writeback*/ bool, /*decrement_before*/ bool, /*first*/ Arm32SinglePrecisionRegister, /*count*/ u8),
    Vstm_Single_A1(Arm32Condition, Arm32GeneralPurposeRegister, bool, bool, Arm32SinglePrecisionRegister, u8),
    Vldm_Double_A1(/*cond*/ Arm32Condition, /*rn*/ Arm32GeneralPurposeRegister, /*writeback*/ bool, /*decrement_before*/ bool, /*first*/ Arm32DoublePrecisionRegister, /*count*/ u8),
    Vstm_Double_A1(Arm32Condition, Arm32GeneralPurposeRegister, bool, bool, Arm32DoublePrecisionRegister, u8),

    // ======================= floating-point (VFP) data-processing =======================
    // 3-operand (Vd, Vn, Vm) and 2-operand "other" (Vd, Vm: VMOV-reg/VABS/VNEG/VSQRT), single + double
    FpDataProcess3_Single_A1(/*cond*/ Arm32Condition, /*op*/ Arm32FpDataOperation3, /*sd*/ Arm32SinglePrecisionRegister, /*sn*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister),
    FpDataProcess3_Double_A1(/*cond*/ Arm32Condition, /*op*/ Arm32FpDataOperation3, /*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    FpDataProcess2_Single_A1(/*cond*/ Arm32Condition, /*op*/ Arm32FpDataOperation2, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister),
    FpDataProcess2_Double_A1(/*cond*/ Arm32Condition, /*op*/ Arm32FpDataOperation2, /*dd*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),

    // ======================= floating-point (VFP) compare / transfer / immediate =======================
    Vcmp_Single_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister, /*e (signalling)*/ bool),
    Vcmp_Double_A1(Arm32Condition, Arm32DoublePrecisionRegister, Arm32DoublePrecisionRegister, bool),
    Vcmp_Zero_Single_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*e*/ bool),
    Vcmp_Zero_Double_A1(Arm32Condition, Arm32DoublePrecisionRegister, bool),
    Vmrs_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister),
    Vmrs_Apsr_Nzcv_A1(/*cond*/ Arm32Condition),
    Vmsr_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister),
    Vmov_Core_To_Single_A1(/*cond*/ Arm32Condition, /*sn*/ Arm32SinglePrecisionRegister, /*rt*/ Arm32GeneralPurposeRegister),
    Vmov_Single_To_Core_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*sn*/ Arm32SinglePrecisionRegister),
    Vmov_Immediate_Single_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*imm8 (VFP modified immediate)*/ u8),
    Vmov_Immediate_Double_A1(Arm32Condition, Arm32DoublePrecisionRegister, u8),
    Vmov_Double_To_CorePair_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rt2*/ Arm32GeneralPurposeRegister, /*dm*/ Arm32DoublePrecisionRegister),
    Vmov_CorePair_To_Double_A1(/*cond*/ Arm32Condition, /*dm*/ Arm32DoublePrecisionRegister, /*rt*/ Arm32GeneralPurposeRegister, /*rt2*/ Arm32GeneralPurposeRegister),
    Vmov_Singles_To_CorePair_A1(/*cond*/ Arm32Condition, /*rt*/ Arm32GeneralPurposeRegister, /*rt2*/ Arm32GeneralPurposeRegister, /*sm (sm, sm+1)*/ Arm32SinglePrecisionRegister),
    Vmov_CorePair_To_Singles_A1(/*cond*/ Arm32Condition, /*sm*/ Arm32SinglePrecisionRegister, /*rt*/ Arm32GeneralPurposeRegister, /*rt2*/ Arm32GeneralPurposeRegister),
    /// `VMOV.<8|16|32> Dd[x], Rt` -- copy a general-purpose register into a scalar lane of a doubleword (Advanced
    /// SIMD). base `0x0E00_0B10`; the width + lane `index` pack into opc1`[22:21]`/opc2`[6:5]`. See [`Arm32VmovLaneSize`].
    Vmov_Core_To_Scalar_A1(/*cond*/ Arm32Condition, Arm32VmovLaneSize, /*index*/ u8, /*dd*/ Arm32DoublePrecisionRegister, /*rt*/ Arm32GeneralPurposeRegister),
    /// `VMOV.<dt> Rt, Dn[x]` -- copy a scalar lane into a general-purpose register, sign/zero-extending for
    /// `.8`/`.16` (`unsigned` picks `.u8`/`.u16` over `.s8`/`.s16`; `.32` ignores it). base `0x0E10_0B10`; U at `[23]`.
    Vmov_Scalar_To_Core_A1(/*cond*/ Arm32Condition, /*unsigned*/ bool, Arm32VmovLaneSize, /*index*/ u8, /*rt*/ Arm32GeneralPurposeRegister, /*dn*/ Arm32DoublePrecisionRegister),

    // ======================= floating-point (VFP) conversions (VCVT) =======================
    // float<->integer (the integer lives in a single-precision register), with optional round-to-zero (the
    // `_R` UAL suffix toggles `round`); int->float carries `signed` only (it always rounds-to-nearest).
    Vcvt_FloatToInt_FromSingle_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister, /*signed*/ bool, /*round_to_zero*/ bool),
    Vcvt_FloatToInt_FromDouble_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister, /*signed*/ bool, /*round_to_zero*/ bool),
    Vcvt_IntToFloat_ToSingle_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister, /*signed*/ bool),
    Vcvt_IntToFloat_ToDouble_A1(/*cond*/ Arm32Condition, /*dd*/ Arm32DoublePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister, /*signed*/ bool),
    // precision change f32<->f64
    Vcvt_Single_To_Double_A1(/*cond*/ Arm32Condition, /*dd*/ Arm32DoublePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister),
    Vcvt_Double_To_Single_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    // half-precision (VCVTB/VCVTT -- `top` selects the high half-word of the single)
    Vcvt_HalfToSingle_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister, /*top*/ bool),
    Vcvt_SingleToHalf_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister, /*top*/ bool),
    // half <-> double (VCVTB/VCVTT `.f64.f16` / `.f16.f64`; requires FEAT_FP16). `top` selects the half-word of
    // the single container `Sd`/`Sm`; the double operand is the full `Dd`/`Dm`.
    Vcvt_HalfToDouble_A1(/*cond*/ Arm32Condition, /*dd*/ Arm32DoublePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister, /*top*/ bool),
    Vcvt_DoubleToHalf_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister, /*top*/ bool),
    // fixed-point <-> float (frac_bits in 1..=size; bits32 picks the 32-bit container, else 16-bit)
    Vcvt_FloatToFixed_Single_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*signed*/ bool, /*bits32*/ bool, /*frac_bits*/ u8),
    Vcvt_FloatToFixed_Double_A1(/*cond*/ Arm32Condition, /*dd*/ Arm32DoublePrecisionRegister, /*signed*/ bool, /*bits32*/ bool, /*frac_bits*/ u8),
    Vcvt_FixedToFloat_Single_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*signed*/ bool, /*bits32*/ bool, /*frac_bits*/ u8),
    Vcvt_FixedToFloat_Double_A1(/*cond*/ Arm32Condition, /*dd*/ Arm32DoublePrecisionRegister, /*signed*/ bool, /*bits32*/ bool, /*frac_bits*/ u8),
    /// VJCVT (VJCVTZS) -- JavaScript-semantics convert double -> signed 32-bit int, round toward zero (FEAT_JSCVT,
    /// ARMv8.3-A). base `0x0EB9_0BC0`; `sd` is the 32-bit result, `dm` the double source.
    Vjcvt_A1(/*cond*/ Arm32Condition, /*sd*/ Arm32SinglePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),

    // ======================= ARMv8-A floating-point additions =======================
    // These are UNCONDITIONAL (A32 cond=1111) except VRINT{R,Z,X}, which carry a normal condition code.
    // VSEL: pick Sd/Dd = test ? Sn/Dn : Sm/Dm, where `test` is the 2-bit Arm32VselCondition.
    Vsel_Single_A1(/*cc*/ Arm32VselCondition, /*sd*/ Arm32SinglePrecisionRegister, /*sn*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister),
    Vsel_Double_A1(/*cc*/ Arm32VselCondition, /*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    // VMAXNM / VMINNM: IEEE 754-2008 maxNum/minNum (NaN-aware), unconditional.
    Vmaxnm_Single_A1(/*sd*/ Arm32SinglePrecisionRegister, /*sn*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister),
    Vmaxnm_Double_A1(/*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    Vminnm_Single_A1(/*sd*/ Arm32SinglePrecisionRegister, /*sn*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister),
    Vminnm_Double_A1(/*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    // VRINT{A,N,P,M}: round float to integral float, fixed (anchored) rounding mode, unconditional.
    Vrint_Directed_Single_A1(/*mode*/ Arm32DirectedRound, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister),
    Vrint_Directed_Double_A1(/*mode*/ Arm32DirectedRound, /*dd*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    // VRINT{R,Z,X}: round float to integral float, conditional (FPSCR / toward-zero / exact).
    Vrint_Cond_Single_A1(/*cond*/ Arm32Condition, /*mode*/ Arm32VrintMode, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister),
    Vrint_Cond_Double_A1(/*cond*/ Arm32Condition, /*mode*/ Arm32VrintMode, /*dd*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    // VCVT{A,N,P,M}: convert float to integer with a fixed (anchored) rounding mode, unconditional. The
    // result is always a single-precision register holding the integer; the source is single or double.
    Vcvt_Directed_FromSingle_A1(/*mode*/ Arm32DirectedRound, /*sd*/ Arm32SinglePrecisionRegister, /*sm*/ Arm32SinglePrecisionRegister, /*signed*/ bool),
    Vcvt_Directed_FromDouble_A1(/*mode*/ Arm32DirectedRound, /*sd*/ Arm32SinglePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister, /*signed*/ bool),

    // ======================= NEON (Advanced SIMD) -- three registers of the same length =======================
    // Unconditional (cond=1111). Each family has a 64-bit (D) and a 128-bit (Q) form. The integer ops carry
    // an element size; the float ops are f32; the bitwise ops bake their selector into the op enum.
    NeonInt3Same_D_A1(/*op*/ Arm32NeonIntegerOp, /*size*/ Arm32NeonSize, /*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonInt3Same_Q_A1(/*op*/ Arm32NeonIntegerOp, /*size*/ Arm32NeonSize, /*qd*/ Arm32QuadwordRegister, /*qn*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),
    NeonFloat3Same_D_A1(/*op*/ Arm32NeonFloatOp, /*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonFloat3Same_Q_A1(/*op*/ Arm32NeonFloatOp, /*qd*/ Arm32QuadwordRegister, /*qn*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),
    NeonBitwise3Same_D_A1(/*op*/ Arm32NeonBitwiseOp, /*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonBitwise3Same_Q_A1(/*op*/ Arm32NeonBitwiseOp, /*qd*/ Arm32QuadwordRegister, /*qn*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),

    // ======================= NEON (Advanced SIMD) -- two registers, miscellaneous =======================
    // Unconditional. Same-width ops have a 64-bit (D) and 128-bit (Q) form; the element-sized variants carry
    // an Arm32NeonSize, the fixed-size ones bake it into the op. Narrowing is Qm->Dd; widening (VSHLL by the
    // element size) is Dm->Qd.
    NeonMisc2Sized_D_A1(/*op*/ Arm32NeonMisc2SizedOp, /*size*/ Arm32NeonSize, /*dd*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonMisc2Sized_Q_A1(/*op*/ Arm32NeonMisc2SizedOp, /*size*/ Arm32NeonSize, /*qd*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),
    NeonMisc2Fixed_D_A1(/*op*/ Arm32NeonMisc2FixedOp, /*dd*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonMisc2Fixed_Q_A1(/*op*/ Arm32NeonMisc2FixedOp, /*qd*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),
    NeonMisc2Narrow_A1(/*op*/ Arm32NeonNarrowOp, /*source size*/ Arm32NeonSize, /*dd*/ Arm32DoublePrecisionRegister, /*qm*/ Arm32QuadwordRegister),
    // VSHLL by the element size (the 2-reg-misc widening "maximum shift" form): Qd = Dm << element_size.
    NeonShllMax_A1(/*element size*/ Arm32NeonSize, /*qd*/ Arm32QuadwordRegister, /*dm*/ Arm32DoublePrecisionRegister),

    // ======================= NEON (Advanced SIMD) -- three registers of different lengths =======================
    // Unconditional. opc=[11:8] implies the register shape. `size` is the source element size (.s8/.u16/.s32
    // = 00/01/10 for long/wide; .i16/.i32/.i64 = 00/01/10 for the narrowing high-half ops).
    NeonDiffLong_A1(/*op*/ Arm32NeonDiffLongOp, /*size*/ Arm32NeonSize, /*qd*/ Arm32QuadwordRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonDiffWide_A1(/*op*/ Arm32NeonDiffWideOp, /*size*/ Arm32NeonSize, /*qd*/ Arm32QuadwordRegister, /*qn*/ Arm32QuadwordRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonDiffNarrow_A1(/*op*/ Arm32NeonDiffNarrowOp, /*source size*/ Arm32NeonSize, /*dd*/ Arm32DoublePrecisionRegister, /*qn*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),

    // ======================= NEON (Advanced SIMD) -- two registers and a scalar =======================
    // The multiplier is a scalar lane Dm[index]; for .i16/.f16 ops Dm is restricted to D0-7 (index 0..3),
    // for .i32/.f32 ops Dm is D0-15 (index 0..1). The float same-length members use the f32 element size.
    NeonScalar_D_A1(/*op*/ Arm32NeonScalarOp, /*size*/ Arm32NeonSize, /*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*scalar dm*/ Arm32DoublePrecisionRegister, /*index*/ u8),
    NeonScalar_Q_A1(/*op*/ Arm32NeonScalarOp, /*size*/ Arm32NeonSize, /*qd*/ Arm32QuadwordRegister, /*qn*/ Arm32QuadwordRegister, /*scalar dm*/ Arm32DoublePrecisionRegister, /*index*/ u8),
    NeonScalarLong_A1(/*op*/ Arm32NeonScalarLongOp, /*size*/ Arm32NeonSize, /*qd*/ Arm32QuadwordRegister, /*dn*/ Arm32DoublePrecisionRegister, /*scalar dm*/ Arm32DoublePrecisionRegister, /*index*/ u8),

    // ======================= NEON (Advanced SIMD) -- two registers and a shift amount =======================
    // Unconditional. The (element size, shift amount) pair is jointly encoded in L:imm6. Same-width has D and
    // Q forms; narrowing is Qm->Dd; widening (VSHLL / VMOVL) is Dm->Qd with shift 0..element_size-1.
    NeonShift_D_A1(/*op*/ Arm32NeonShiftOp, /*size*/ Arm32NeonSize, /*shift*/ u8, /*dd*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonShift_Q_A1(/*op*/ Arm32NeonShiftOp, /*size*/ Arm32NeonSize, /*shift*/ u8, /*qd*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),
    NeonShiftNarrow_A1(/*op*/ Arm32NeonShiftNarrowOp, /*source size*/ Arm32NeonSize, /*shift*/ u8, /*dd*/ Arm32DoublePrecisionRegister, /*qm*/ Arm32QuadwordRegister),
    // VSHLL / VMOVL (widening by a shift). `signed` sets U; shift 0 is the VMOVL spelling.
    NeonShiftLong_A1(/*signed*/ bool, /*source size*/ Arm32NeonSize, /*shift (0..esize-1)*/ u8, /*qd*/ Arm32QuadwordRegister, /*dm*/ Arm32DoublePrecisionRegister),

    // ======================= NEON (Advanced SIMD) -- extract / table / duplicate / immediate =======================
    // VEXT (byte extract): the immediate is the BYTE offset (0..7 for D, 0..15 for Q); .8/.16/.32 in UAL is
    // sugar that scales the element index to bytes, so the model carries the byte offset directly.
    NeonExt_D_A1(/*byte offset*/ u8, /*dd*/ Arm32DoublePrecisionRegister, /*dn*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonExt_Q_A1(/*byte offset*/ u8, /*qd*/ Arm32QuadwordRegister, /*qn*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),
    // VTBL / VTBX: table is `length` consecutive D registers starting at Dn; index vector Dm -> Dd.
    NeonTableLookup_A1(/*is_vtbx*/ bool, /*length 1..=4*/ u8, /*dd*/ Arm32DoublePrecisionRegister, /*dn (first table reg)*/ Arm32DoublePrecisionRegister, /*dm (index)*/ Arm32DoublePrecisionRegister),
    // VDUP (scalar): broadcast lane Dm[index] across Dd/Qd.
    NeonVdupScalar_D_A1(/*size*/ Arm32NeonSize, /*index*/ u8, /*dd*/ Arm32DoublePrecisionRegister, /*dm*/ Arm32DoublePrecisionRegister),
    NeonVdupScalar_Q_A1(/*size*/ Arm32NeonSize, /*index*/ u8, /*qd*/ Arm32QuadwordRegister, /*dm*/ Arm32DoublePrecisionRegister),
    // VDUP (from an ARM core register): broadcast Rt across Dd/Qd. Conditional (lives in the VFP/coproc space).
    NeonVdupCore_D_A1(/*cond*/ Arm32Condition, /*size*/ Arm32NeonSize, /*dd*/ Arm32DoublePrecisionRegister, /*rt*/ Arm32GeneralPurposeRegister),
    NeonVdupCore_Q_A1(/*cond*/ Arm32Condition, /*size*/ Arm32NeonSize, /*qd*/ Arm32QuadwordRegister, /*rt*/ Arm32GeneralPurposeRegister),
    // VMOV / VMVN / VORR / VBIC (modified immediate): carried raw as (cmode, op, imm8); the (cmode, op) pair
    // selects the mnemonic, element size, and shift, and imm8 is the 8-bit AdvSIMDExpandImm seed.
    NeonModifiedImmediate_D_A1(/*cmode*/ u8, /*op*/ bool, /*imm8*/ u8, /*dd*/ Arm32DoublePrecisionRegister),
    NeonModifiedImmediate_Q_A1(/*cmode*/ u8, /*op*/ bool, /*imm8*/ u8, /*qd*/ Arm32QuadwordRegister),

    // ======================= NEON (Advanced SIMD) -- element / structure load & store (VLD1-4 / VST1-4) =======================
    // Unconditional. The `[31:24]=0xF4` space, with three forms. The first transferred D register is `first`;
    // the rest of the register list is implied by the form's structure fields. Fields packing several
    // sub-values (the multiple-element `type`, the single-lane `index_align`) are carried as the encoded bits.
    // Multiple n-element structures: `type` ([11:8]) names the VLD1/2/3/4 variant + register count/stride.
    NeonLoadStoreMultiple_A1(/*is_load*/ bool, /*type bits [11:8]*/ u8, /*element size*/ Arm32NeonSize, /*align [5:4]*/ u8, /*first*/ Arm32DoublePrecisionRegister, /*rn*/ Arm32GeneralPurposeRegister, /*address*/ Arm32NeonLoadStoreAddress),
    // Single n-element structure to one lane. `index_align` ([7:4]) packs the lane index and the alignment.
    NeonLoadStoreSingleLane_A1(/*is_load*/ bool, /*structure count 1..=4*/ u8, /*element size 0..=2*/ u8, /*index_align [7:4]*/ u8, /*first*/ Arm32DoublePrecisionRegister, /*rn*/ Arm32GeneralPurposeRegister, /*address*/ Arm32NeonLoadStoreAddress),
    // Single n-element structure to all lanes (load only). `t` = double-spacing, `a` = alignment qualifier.
    NeonLoadStoreAllLanes_A1(/*structure count 1..=4*/ u8, /*element size 0..=2*/ u8, /*t (stride-2)*/ bool, /*a (align)*/ bool, /*first*/ Arm32DoublePrecisionRegister, /*rn*/ Arm32GeneralPurposeRegister, /*address*/ Arm32NeonLoadStoreAddress),

    // ======================= ARMv8 cryptography extension (NEON, Q registers) =======================
    // VMULL.p64 is not here -- it is the size-64 case of the 3-reg-different VMULL.p, i.e. NeonDiffLong(VmullP, I32).
    NeonAes_A1(/*op*/ Arm32NeonAesOp, /*qd*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),
    NeonSha3Reg_A1(/*op*/ Arm32NeonSha3Op, /*qd*/ Arm32QuadwordRegister, /*qn*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),
    NeonSha2Reg_A1(/*op*/ Arm32NeonSha2Op, /*qd*/ Arm32QuadwordRegister, /*qm*/ Arm32QuadwordRegister),

    // ======================= preload (unconditional) =======================
    // PLD/PLI/PLDW [Rn, #+/-imm12] or [Rn, +/-Rm{,shift}] (always offset addressing). PC base is the literal form.
    Pld_A1(/*rn*/ Arm32GeneralPurposeRegister, /*offset*/ Arm32MemoryOffset),
    Pldw_A1(/*rn*/ Arm32GeneralPurposeRegister, /*offset*/ Arm32MemoryOffset),
    Pli_A1(/*rn*/ Arm32GeneralPurposeRegister, /*offset*/ Arm32MemoryOffset),

    // ======================= exception save/return (unconditional) =======================
    Rfe_A1(/*mode*/ Arm32BlockAddressMode, /*rn*/ Arm32GeneralPurposeRegister, /*writeback*/ bool),
    Srs_A1(/*mode*/ Arm32BlockAddressMode, /*writeback*/ bool, /*mode_num (5-bit)*/ u8),

    // ======================= branch / interwork =======================
    // offsets are the DECODED byte displacement relative to the instruction's PC (= address + 8)
    B_A1(/*cond*/ Arm32Condition, /*offset*/ i32),
    Bl_A1(/*cond*/ Arm32Condition, /*offset*/ i32),
    Blx_Immediate_A1(/*offset (multiple of 2; switches to Thumb)*/ i32), // unconditional
    Bx_A1(/*cond*/ Arm32Condition, /*rm*/ Arm32GeneralPurposeRegister),
    Blx_Register_A1(/*cond*/ Arm32Condition, /*rm*/ Arm32GeneralPurposeRegister),
    Bxj_A1(/*cond*/ Arm32Condition, /*rm*/ Arm32GeneralPurposeRegister),

    // ======================= exception generation =======================
    Svc_A1(/*cond*/ Arm32Condition, /*imm24*/ u32),
}

impl ArmA32Instruction {
    /// Encode this instruction to its 4 little-endian machine-code bytes (A32 is fixed-width 32-bit).
    /// Returns [`EncodeError`] if an operand field is out of range for the encoding. Use
    /// [`encode_for_target`](Self::encode_for_target) to also check that a given profile has ARM state.
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let word = self.encode_word()?;
        Ok(word.to_le_bytes().to_vec())
    }

    fn encode_word(&self) -> Result<u32, EncodeError> {
        match self {
            // -- data processing (immediate) --  helper packs: cccc 001 opcode S Rn Rd imm12
            Self::And_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_AND, *s, reg(rn), reg(rd), *v),
            Self::Eor_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_EOR, *s, reg(rn), reg(rd), *v),
            Self::Sub_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_SUB, *s, reg(rn), reg(rd), *v),
            Self::Rsb_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_RSB, *s, reg(rn), reg(rd), *v),
            Self::Add_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_ADD, *s, reg(rn), reg(rd), *v),
            Self::Adc_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_ADC, *s, reg(rn), reg(rd), *v),
            Self::Sbc_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_SBC, *s, reg(rn), reg(rd), *v),
            Self::Rsc_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_RSC, *s, reg(rn), reg(rd), *v),
            Self::Orr_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_ORR, *s, reg(rn), reg(rd), *v),
            Self::Bic_Immediate_A1(c, s, rd, rn, v) => encode_dp_immediate(c, OP_BIC, *s, reg(rn), reg(rd), *v),
            Self::Mov_Immediate_A1(c, s, rd, v) => encode_dp_immediate(c, OP_MOV, *s, 0, reg(rd), *v),
            Self::Mvn_Immediate_A1(c, s, rd, v) => encode_dp_immediate(c, OP_MVN, *s, 0, reg(rd), *v),
            Self::Tst_Immediate_A1(c, rn, v) => encode_dp_immediate(c, OP_TST, true, reg(rn), 0, *v),
            Self::Teq_Immediate_A1(c, rn, v) => encode_dp_immediate(c, OP_TEQ, true, reg(rn), 0, *v),
            Self::Cmp_Immediate_A1(c, rn, v) => encode_dp_immediate(c, OP_CMP, true, reg(rn), 0, *v),
            Self::Cmn_Immediate_A1(c, rn, v) => encode_dp_immediate(c, OP_CMN, true, reg(rn), 0, *v),

            // -- data processing (register, immediate shift) --  helper packs: cccc 000 opcode S Rn Rd imm5 type 0 Rm
            Self::And_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_AND, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Eor_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_EOR, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Sub_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_SUB, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Rsb_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_RSB, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Add_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_ADD, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Adc_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_ADC, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Sbc_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_SBC, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Rsc_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_RSC, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Orr_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_ORR, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Bic_Register_A1(c, s, rd, rn, rm, sh) => encode_dp_register(c, OP_BIC, *s, reg(rn), reg(rd), reg(rm), sh),
            Self::Mov_Register_A1(c, s, rd, rm, sh) => encode_dp_register(c, OP_MOV, *s, 0, reg(rd), reg(rm), sh),
            Self::Mvn_Register_A1(c, s, rd, rm, sh) => encode_dp_register(c, OP_MVN, *s, 0, reg(rd), reg(rm), sh),
            Self::Tst_Register_A1(c, rn, rm, sh) => encode_dp_register(c, OP_TST, true, reg(rn), 0, reg(rm), sh),
            Self::Teq_Register_A1(c, rn, rm, sh) => encode_dp_register(c, OP_TEQ, true, reg(rn), 0, reg(rm), sh),
            Self::Cmp_Register_A1(c, rn, rm, sh) => encode_dp_register(c, OP_CMP, true, reg(rn), 0, reg(rm), sh),
            Self::Cmn_Register_A1(c, rn, rm, sh) => encode_dp_register(c, OP_CMN, true, reg(rn), 0, reg(rm), sh),

            // -- data processing (register-shifted register) --  helper packs: cccc 000 opcode S Rn Rd Rs 0 type 1 Rm
            Self::And_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_AND, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Eor_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_EOR, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Sub_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_SUB, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Rsb_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_RSB, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Add_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_ADD, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Adc_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_ADC, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Sbc_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_SBC, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Rsc_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_RSC, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Orr_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_ORR, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Bic_RegisterShiftedRegister_A1(c, s, rd, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_BIC, *s, reg(rn), reg(rd), reg(rm), *st, reg(rs))),
            Self::Mov_RegisterShiftedRegister_A1(c, s, rd, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_MOV, *s, 0, reg(rd), reg(rm), *st, reg(rs))),
            Self::Mvn_RegisterShiftedRegister_A1(c, s, rd, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_MVN, *s, 0, reg(rd), reg(rm), *st, reg(rs))),
            Self::Tst_RegisterShiftedRegister_A1(c, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_TST, true, reg(rn), 0, reg(rm), *st, reg(rs))),
            Self::Teq_RegisterShiftedRegister_A1(c, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_TEQ, true, reg(rn), 0, reg(rm), *st, reg(rs))),
            Self::Cmp_RegisterShiftedRegister_A1(c, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_CMP, true, reg(rn), 0, reg(rm), *st, reg(rs))),
            Self::Cmn_RegisterShiftedRegister_A1(c, rn, rm, st, rs) => Ok(encode_dp_register_shifted(c, OP_CMN, true, reg(rn), 0, reg(rm), *st, reg(rs))),

            // -- 16-bit immediate moves --
            Self::Movw_A2(c, rd, imm16) => Ok(encode_movw_movt(c, false, reg(rd), *imm16)),
            Self::Movt_A1(c, rd, imm16) => Ok(encode_movw_movt(c, true, reg(rd), *imm16)),

            // -- multiply --  helper packs: cccc 0000 op S high low Rm 1001 Rn
            Self::Mul_A1(c, s, rd, rn, rm) => Ok(encode_multiply(c, 0x0000_0090, *s, reg(rd), 0, reg(rm), reg(rn))),
            Self::Mla_A1(c, s, rd, rn, rm, ra) => Ok(encode_multiply(c, 0x0020_0090, *s, reg(rd), reg(ra), reg(rm), reg(rn))),
            Self::Mls_A1(c, rd, rn, rm, ra) => Ok(encode_multiply(c, 0x0060_0090, false, reg(rd), reg(ra), reg(rm), reg(rn))),
            Self::Umull_A1(c, s, rdlo, rdhi, rn, rm) => Ok(encode_multiply(c, 0x0080_0090, *s, reg(rdhi), reg(rdlo), reg(rm), reg(rn))),
            Self::Umlal_A1(c, s, rdlo, rdhi, rn, rm) => Ok(encode_multiply(c, 0x00A0_0090, *s, reg(rdhi), reg(rdlo), reg(rm), reg(rn))),
            Self::Smull_A1(c, s, rdlo, rdhi, rn, rm) => Ok(encode_multiply(c, 0x00C0_0090, *s, reg(rdhi), reg(rdlo), reg(rm), reg(rn))),
            Self::Smlal_A1(c, s, rdlo, rdhi, rn, rm) => Ok(encode_multiply(c, 0x00E0_0090, *s, reg(rdhi), reg(rdlo), reg(rm), reg(rn))),
            Self::Umaal_A1(c, rdlo, rdhi, rn, rm) => Ok(encode_multiply(c, 0x0040_0090, false, reg(rdhi), reg(rdlo), reg(rm), reg(rn))),

            // -- saturating arithmetic --  helper packs: cccc 00010 op 0 Rn Rd 0000 0101 Rm
            Self::Qadd_A1(c, rd, rm, rn) => Ok(encode_saturating(c, 0x0100_0050, reg(rd), reg(rm), reg(rn))),
            Self::Qsub_A1(c, rd, rm, rn) => Ok(encode_saturating(c, 0x0120_0050, reg(rd), reg(rm), reg(rn))),
            Self::Qdadd_A1(c, rd, rm, rn) => Ok(encode_saturating(c, 0x0140_0050, reg(rd), reg(rm), reg(rn))),
            Self::Qdsub_A1(c, rd, rm, rn) => Ok(encode_saturating(c, 0x0160_0050, reg(rd), reg(rm), reg(rn))),

            // -- signed multiply (halfword, type 1): [7]=1, m=bit6, n=bit5, [4]=0 --
            Self::Smla_A1(c, rd, rn, rm, ra, n, m) => Ok(encode_signed_mul(c, 0x0100_0080 | nm_bits(*n, *m), reg(rd), reg(ra), reg(rm), reg(rn))),
            Self::Smlaw_A1(c, rd, rn, rm, ra, m) => Ok(encode_signed_mul(c, 0x0120_0080 | ((*m as u32) << 6), reg(rd), reg(ra), reg(rm), reg(rn))),
            Self::Smulw_A1(c, rd, rn, rm, m) => Ok(encode_signed_mul(c, 0x0120_00A0 | ((*m as u32) << 6), reg(rd), 0, reg(rm), reg(rn))),
            Self::Smlal_Halfword_A1(c, rdlo, rdhi, rn, rm, n, m) => Ok(encode_signed_mul(c, 0x0140_0080 | nm_bits(*n, *m), reg(rdhi), reg(rdlo), reg(rm), reg(rn))),
            Self::Smul_A1(c, rd, rn, rm, n, m) => Ok(encode_signed_mul(c, 0x0160_0080 | nm_bits(*n, *m), reg(rd), 0, reg(rm), reg(rn))),
            // -- signed multiply (dual / most-significant word, type 2): [27:24]=0111, x/round=bit5, [4]=1 --
            Self::Smlad_A1(c, rd, rn, rm, ra, x) => Ok(encode_signed_mul(c, 0x0700_0010 | ((*x as u32) << 5), reg(rd), reg(ra), reg(rm), reg(rn))),
            Self::Smuad_A1(c, rd, rn, rm, x) => Ok(encode_signed_mul(c, 0x0700_0010 | ((*x as u32) << 5), reg(rd), 0xF, reg(rm), reg(rn))),
            Self::Smlsd_A1(c, rd, rn, rm, ra, x) => Ok(encode_signed_mul(c, 0x0700_0050 | ((*x as u32) << 5), reg(rd), reg(ra), reg(rm), reg(rn))),
            Self::Smusd_A1(c, rd, rn, rm, x) => Ok(encode_signed_mul(c, 0x0700_0050 | ((*x as u32) << 5), reg(rd), 0xF, reg(rm), reg(rn))),
            Self::Smmla_A1(c, rd, rn, rm, ra, r) => Ok(encode_signed_mul(c, 0x0750_0010 | ((*r as u32) << 5), reg(rd), reg(ra), reg(rm), reg(rn))),
            Self::Smmul_A1(c, rd, rn, rm, r) => Ok(encode_signed_mul(c, 0x0750_0010 | ((*r as u32) << 5), reg(rd), 0xF, reg(rm), reg(rn))),
            Self::Smmls_A1(c, rd, rn, rm, ra, r) => Ok(encode_signed_mul(c, 0x0750_00D0 | ((*r as u32) << 5), reg(rd), reg(ra), reg(rm), reg(rn))),
            Self::Smlald_A1(c, rdlo, rdhi, rn, rm, x) => Ok(encode_signed_mul(c, 0x0740_0010 | ((*x as u32) << 5), reg(rdhi), reg(rdlo), reg(rm), reg(rn))),
            Self::Smlsld_A1(c, rdlo, rdhi, rn, rm, x) => Ok(encode_signed_mul(c, 0x0740_0050 | ((*x as u32) << 5), reg(rdhi), reg(rdlo), reg(rm), reg(rn))),

            // -- parallel (packed SIMD) add/sub + select --  cccc 01100 prefix Rn Rd 1111 op 1 Rm ; SEL: ..01101000.. 1111 1011 ..
            Self::ParallelAddSub_A1(c, op, prefix, rd, rn, rm) => Ok(cond_bits(c) | 0x0600_0F00 | (a32_parallel_prefix_bits(*prefix) << 20) | (reg(rn) << 16) | (reg(rd) << 12) | (a32_parallel_op_bits(*op) << 4) | reg(rm)),
            Self::Sel_A1(c, rd, rn, rm) => Ok(cond_bits(c) | 0x0680_0FB0 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rm)),

            // -- extend / extend-and-add --  cccc <byte> Rn Rd rotate 00 0111 Rm  (Rn=1111 => plain extend)
            Self::Extend_A1(c, ty, rd, rm, rot) => Ok(encode_extend(c, ty.opcode_byte(), 0xF, reg(rd), reg(rm), encode_rotation(*rot)?)),
            Self::ExtendAndAdd_A1(c, ty, rd, rn, rm, rot) => Ok(encode_extend(c, ty.opcode_byte(), reg(rn), reg(rd), reg(rm), encode_rotation(*rot)?)),
            // -- byte/bit reverse + count leading zeros --
            Self::Rev_A1(c, rd, rm) => Ok(cond_bits(c) | 0x06BF_0F30 | (reg(rd) << 12) | reg(rm)),
            Self::Rev16_A1(c, rd, rm) => Ok(cond_bits(c) | 0x06BF_0FB0 | (reg(rd) << 12) | reg(rm)),
            Self::Revsh_A1(c, rd, rm) => Ok(cond_bits(c) | 0x06FF_0FB0 | (reg(rd) << 12) | reg(rm)),
            Self::Rbit_A1(c, rd, rm) => Ok(cond_bits(c) | 0x06FF_0F30 | (reg(rd) << 12) | reg(rm)),
            Self::Clz_A1(c, rd, rm) => Ok(cond_bits(c) | 0x016F_0F10 | (reg(rd) << 12) | reg(rm)),

            // -- pack halfword --  cccc 01101000 Rn Rd imm5 tb 01 Rm
            Self::Pkhbt_A1(c, rd, rn, rm, lsl) => {
                check_unsigned_maximum("lsl", *lsl as u32, 31)?;
                Ok(cond_bits(c) | 0x0680_0010 | (reg(rn) << 16) | (reg(rd) << 12) | ((*lsl as u32) << 7) | reg(rm))
            },
            Self::Pkhtb_A1(c, rd, rn, rm, asr) => {
                if *asr < 1 || *asr > 32 { return Err(EncodeError::ImmediateOutOfRange { field: "asr", value: *asr as i64, minimum: 1, maximum: 32 }); }
                let imm5 = if *asr == 32 { 0 } else { *asr as u32 };
                Ok(cond_bits(c) | 0x0680_0050 | (reg(rn) << 16) | (reg(rd) << 12) | (imm5 << 7) | reg(rm))
            },
            // -- saturate --  cccc 0110 101/111 sat_imm Rd imm5 sh 01 Rm  (SSAT/USAT) ; ...1010/1110 .. 1111 0011 .. (SSAT16/USAT16)
            Self::Ssat_A1(c, rd, sat, rm, shift) => encode_saturate(c, false, *sat, reg(rd), reg(rm), shift),
            Self::Usat_A1(c, rd, sat, rm, shift) => encode_saturate(c, true, *sat, reg(rd), reg(rm), shift),
            Self::Ssat16_A1(c, rd, sat, rm) => {
                if *sat < 1 || *sat > 16 { return Err(EncodeError::ImmediateOutOfRange { field: "sat", value: *sat as i64, minimum: 1, maximum: 16 }); }
                Ok(cond_bits(c) | 0x06A0_0F30 | (((*sat - 1) as u32) << 16) | (reg(rd) << 12) | reg(rm))
            },
            Self::Usat16_A1(c, rd, sat, rm) => {
                check_unsigned_maximum("sat", *sat as u32, 15)?;
                Ok(cond_bits(c) | 0x06E0_0F30 | ((*sat as u32) << 16) | (reg(rd) << 12) | reg(rm))
            },
            // -- sum of absolute differences --  cccc 01111000 Rd Ra Rm 0001 Rn  (Ra=1111 => USAD8)
            Self::Usad8_A1(c, rd, rn, rm) => Ok(cond_bits(c) | 0x0780_0010 | (reg(rd) << 16) | (0xF << 12) | (reg(rm) << 8) | reg(rn)),
            Self::Usada8_A1(c, rd, rn, rm, ra) => Ok(cond_bits(c) | 0x0780_0010 | (reg(rd) << 16) | (reg(ra) << 12) | (reg(rm) << 8) | reg(rn)),

            // -- bitfield --  BFC/BFI: cccc 0111110 msb Rd lsb 001 Rn(=1111 BFC) ; SBFX/UBFX: cccc 011110/111 widthm1 Rd lsb 101 Rn
            Self::Bfc_A1(c, rd, lsb, width) => {
                check_bitfield(*lsb, *width)?;
                Ok(cond_bits(c) | 0x07C0_001F | (((*lsb + *width - 1) as u32) << 16) | (reg(rd) << 12) | ((*lsb as u32) << 7))
            },
            Self::Bfi_A1(c, rd, rn, lsb, width) => {
                check_bitfield(*lsb, *width)?;
                Ok(cond_bits(c) | 0x07C0_0010 | (((*lsb + *width - 1) as u32) << 16) | (reg(rd) << 12) | ((*lsb as u32) << 7) | reg(rn))
            },
            Self::Sbfx_A1(c, rd, rn, lsb, width) => {
                check_bitfield(*lsb, *width)?;
                Ok(cond_bits(c) | 0x07A0_0050 | (((*width - 1) as u32) << 16) | (reg(rd) << 12) | ((*lsb as u32) << 7) | reg(rn))
            },
            Self::Ubfx_A1(c, rd, rn, lsb, width) => {
                check_bitfield(*lsb, *width)?;
                Ok(cond_bits(c) | 0x07E0_0050 | (((*width - 1) as u32) << 16) | (reg(rd) << 12) | ((*lsb as u32) << 7) | reg(rn))
            },

            // -- load/store single (word/byte) --  cccc 01 I P U B W L Rn Rt <offset>
            Self::Ldr_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); encode_load_store(c, false, true, p, w, reg(rn), reg(rt), off) },
            Self::Str_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); encode_load_store(c, false, false, p, w, reg(rn), reg(rt), off) },
            Self::Ldrb_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); encode_load_store(c, true, true, p, w, reg(rn), reg(rt), off) },
            Self::Strb_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); encode_load_store(c, true, false, p, w, reg(rn), reg(rt), off) },
            Self::Ldrt_A1(c, rt, rn, off) => encode_load_store(c, false, true, 0, 1, reg(rn), reg(rt), off),
            Self::Strt_A1(c, rt, rn, off) => encode_load_store(c, false, false, 0, 1, reg(rn), reg(rt), off),
            Self::Ldrbt_A1(c, rt, rn, off) => encode_load_store(c, true, true, 0, 1, reg(rn), reg(rt), off),
            Self::Strbt_A1(c, rt, rn, off) => encode_load_store(c, true, false, 0, 1, reg(rn), reg(rt), off),

            // -- load/store halfword/dual/signed --  cccc 000 P U I W L Rn Rt H4 1 S H 1 L4  (S,H select the op)
            Self::Ldrh_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); Ok(encode_extra_load_store(c, p, w, true, 0, 1, reg(rn), reg(rt), off)) },
            Self::Strh_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); Ok(encode_extra_load_store(c, p, w, false, 0, 1, reg(rn), reg(rt), off)) },
            Self::Ldrsb_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); Ok(encode_extra_load_store(c, p, w, true, 1, 0, reg(rn), reg(rt), off)) },
            Self::Ldrsh_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); Ok(encode_extra_load_store(c, p, w, true, 1, 1, reg(rn), reg(rt), off)) },
            Self::Ldrd_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); Ok(encode_extra_load_store(c, p, w, false, 1, 0, reg(rn), reg(rt), off)) },
            Self::Strd_A1(c, rt, rn, off, idx) => { let (p, w) = a32_index_p_w(*idx); Ok(encode_extra_load_store(c, p, w, false, 1, 1, reg(rn), reg(rt), off)) },
            Self::Ldrht_A1(c, rt, rn, off) => Ok(encode_extra_load_store(c, 0, 1, true, 0, 1, reg(rn), reg(rt), off)),
            Self::Strht_A1(c, rt, rn, off) => Ok(encode_extra_load_store(c, 0, 1, false, 0, 1, reg(rn), reg(rt), off)),
            Self::Ldrsbt_A1(c, rt, rn, off) => Ok(encode_extra_load_store(c, 0, 1, true, 1, 0, reg(rn), reg(rt), off)),
            Self::Ldrsht_A1(c, rt, rn, off) => Ok(encode_extra_load_store(c, 0, 1, true, 1, 1, reg(rn), reg(rt), off)),

            // -- load/store multiple --  cccc 100 P U S W L Rn register_list
            Self::Ldm_A1(c, mode, rn, wb, user, regs) => Ok(encode_load_store_multiple(c, *mode, *user, *wb, true, reg(rn), regs)),
            Self::Stm_A1(c, mode, rn, wb, user, regs) => Ok(encode_load_store_multiple(c, *mode, *user, *wb, false, reg(rn), regs)),

            // -- synchronization --  exclusive: cccc 00011 type L Rn (Rt|Rd) 1111 1001 (1111|Rt)
            Self::Ldrex_A1(c, rt, rn) => Ok(cond_bits(c) | 0x0190_0F9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Strex_A1(c, rd, rt, rn) => Ok(cond_bits(c) | 0x0180_0F90 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rt)),
            Self::Ldrexb_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01D0_0F9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Strexb_A1(c, rd, rt, rn) => Ok(cond_bits(c) | 0x01C0_0F90 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rt)),
            Self::Ldrexh_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01F0_0F9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Strexh_A1(c, rd, rt, rn) => Ok(cond_bits(c) | 0x01E0_0F90 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rt)),
            Self::Ldrexd_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01B0_0F9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Strexd_A1(c, rd, rt, rn) => Ok(cond_bits(c) | 0x01A0_0F90 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rt)),
            Self::Clrex_A1 => Ok(0xF57F_F01F),
            // SWP/SWPB (deprecated): cccc 00010 B 00 Rn Rt 0000 1001 Rt2
            Self::Swp_A1(c, rt, rt2, rn) => Ok(cond_bits(c) | 0x0100_0090 | (reg(rn) << 16) | (reg(rt) << 12) | reg(rt2)),
            Self::Swpb_A1(c, rt, rt2, rn) => Ok(cond_bits(c) | 0x0140_0090 | (reg(rn) << 16) | (reg(rt) << 12) | reg(rt2)),

            // -- status / system register access --
            Self::Mrs_A1(c, spsr, rd) => Ok(cond_bits(c) | 0x010F_0000 | ((*spsr as u32) << 22) | (reg(rd) << 12)),
            Self::Msr_Register_A1(c, spsr, mask, rn) => Ok(cond_bits(c) | 0x0120_F000 | ((*spsr as u32) << 22) | (((*mask & 0xF) as u32) << 16) | reg(rn)),
            Self::MrsBanked_A1(c, spsr, sysm, rd) => Ok(cond_bits(c) | 0x0100_0200 | ((*spsr as u32) << 22) | (((*sysm & 0xF) as u32) << 16) | (reg(rd) << 12) | ((((*sysm >> 4) & 1) as u32) << 8)),
            Self::MsrBanked_A1(c, spsr, sysm, rn) => Ok(cond_bits(c) | 0x0120_F200 | ((*spsr as u32) << 22) | (((*sysm & 0xF) as u32) << 16) | ((((*sysm >> 4) & 1) as u32) << 8) | reg(rn)),
            Self::Msr_Immediate_A1(c, spsr, mask, value) => {
                let imm12 = encode_a32_modified_immediate(*value)
                    .ok_or(EncodeError::ModifiedImmediateNotEncodable { field: "const", value: *value })?;
                Ok(cond_bits(c) | 0x0320_F000 | ((*spsr as u32) << 22) | (((*mask & 0xF) as u32) << 16) | (imm12 as u32))
            },
            Self::Cps_A1(mode, a, i, f, new_mode) => {
                // 1111 00010000 imod M 0 0000000 A I F 0 mode
                let m = new_mode.is_some() as u32;
                let mode_bits = (new_mode.unwrap_or(0) & 0x1F) as u32;
                Ok(0xF100_0000 | (mode.imod_bits() << 18) | (m << 17) | ((*a as u32) << 8) | ((*i as u32) << 7) | ((*f as u32) << 6) | mode_bits)
            },
            Self::Setend_A1(big_endian) => Ok(0xF101_0000 | ((*big_endian as u32) << 9)), // 1111 00010000 0001 0000 00 E 0 ...

            // -- coprocessor --
            Self::Mcr_A1(c, coproc, opc1, rt, crn, crm, opc2) => Ok(cond_bits(c) | 0x0E00_0010 | mcr_fields(*coproc, *opc1, *opc2, *crn, *crm, reg(rt))),
            Self::Mrc_A1(c, coproc, opc1, rt, crn, crm, opc2) => Ok(cond_bits(c) | 0x0E10_0010 | mcr_fields(*coproc, *opc1, *opc2, *crn, *crm, reg(rt))),
            Self::Mcr2_A1(coproc, opc1, rt, crn, crm, opc2) => Ok(0xFE00_0010 | mcr_fields(*coproc, *opc1, *opc2, *crn, *crm, reg(rt))),
            Self::Mrc2_A1(coproc, opc1, rt, crn, crm, opc2) => Ok(0xFE10_0010 | mcr_fields(*coproc, *opc1, *opc2, *crn, *crm, reg(rt))),
            Self::Cdp_A1(c, coproc, opc1, crd, crn, crm, opc2) => Ok(cond_bits(c) | 0x0E00_0000 | cdp_fields(*coproc, *opc1, *opc2, *crn, *crd, *crm)),
            Self::Cdp2_A1(coproc, opc1, crd, crn, crm, opc2) => Ok(0xFE00_0000 | cdp_fields(*coproc, *opc1, *opc2, *crn, *crd, *crm)),
            Self::Mcrr_A1(c, coproc, opc1, rt, rt2, crm) => Ok(cond_bits(c) | 0x0C40_0000 | mcrr_fields(*coproc, *opc1, reg(rt), reg(rt2), *crm)),
            Self::Mrrc_A1(c, coproc, opc1, rt, rt2, crm) => Ok(cond_bits(c) | 0x0C50_0000 | mcrr_fields(*coproc, *opc1, reg(rt), reg(rt2), *crm)),
            Self::Mcrr2_A1(coproc, opc1, rt, rt2, crm) => Ok(0xFC40_0000 | mcrr_fields(*coproc, *opc1, reg(rt), reg(rt2), *crm)),
            Self::Mrrc2_A1(coproc, opc1, rt, rt2, crm) => Ok(0xFC50_0000 | mcrr_fields(*coproc, *opc1, reg(rt), reg(rt2), *crm)),
            Self::Ldc_A1(c, coproc, long, crd, rn, add, imm8, idx) => { let (p, w) = ldc_index_p_w(*idx); Ok(cond_bits(c) | ldc_base(p, *add, *long, w, true) | ldc_fields(reg(rn), *crd, *coproc, *imm8)) },
            Self::Stc_A1(c, coproc, long, crd, rn, add, imm8, idx) => { let (p, w) = ldc_index_p_w(*idx); Ok(cond_bits(c) | ldc_base(p, *add, *long, w, false) | ldc_fields(reg(rn), *crd, *coproc, *imm8)) },
            Self::Ldc2_A1(coproc, long, crd, rn, add, imm8, idx) => { let (p, w) = ldc_index_p_w(*idx); Ok(0xF000_0000 | ldc_base(p, *add, *long, w, true) | ldc_fields(reg(rn), *crd, *coproc, *imm8)) },
            Self::Stc2_A1(coproc, long, crd, rn, add, imm8, idx) => { let (p, w) = ldc_index_p_w(*idx); Ok(0xF000_0000 | ldc_base(p, *add, *long, w, false) | ldc_fields(reg(rn), *crd, *coproc, *imm8)) },

            // -- hints --  cccc 0011 0010 0000 1111 0000 0000 hint8
            Self::Nop_A1(c) => Ok(cond_bits(c) | 0x0320_F000),
            Self::Yield_A1(c) => Ok(cond_bits(c) | 0x0320_F001),
            Self::Wfe_A1(c) => Ok(cond_bits(c) | 0x0320_F002),
            Self::Wfi_A1(c) => Ok(cond_bits(c) | 0x0320_F003),
            Self::Sev_A1(c) => Ok(cond_bits(c) | 0x0320_F004),
            Self::Dbg_A1(c, option) => Ok(cond_bits(c) | 0x0320_F0F0 | ((*option & 0xF) as u32)),
            Self::Csdb_A1(c) => Ok(cond_bits(c) | 0x0320_F014),
            Self::Esb_A1(c) => Ok(cond_bits(c) | 0x0320_F010),

            // -- memory barriers (unconditional) --  1111 0101 0111 1111 0000 0000 op4 option
            Self::Dmb_A1(option) => Ok(0xF57F_F050 | ((*option & 0xF) as u32)),
            Self::Dsb_A1(option) => Ok(0xF57F_F040 | ((*option & 0xF) as u32)),
            Self::Isb_A1(option) => Ok(0xF57F_F060 | ((*option & 0xF) as u32)),
            Self::Sb_A1 => Ok(0xF57F_F070),

            // -- exception generation --
            Self::Bkpt_A1(c, imm16) => Ok(cond_bits(c) | 0x0120_0070 | imm16_split(*imm16)), // cccc 00010010 imm12 0111 imm4
            Self::Hvc_A1(c, imm16) => Ok(cond_bits(c) | 0x0140_0070 | imm16_split(*imm16)),  // cccc 00010100 imm12 0111 imm4
            Self::Smc_A1(c, imm4) => Ok(cond_bits(c) | 0x0160_0070 | ((*imm4 & 0xF) as u32)), // cccc 00010110 0000 0000 0000 0111 imm4
            Self::Udf_A1(c, imm16) => Ok(cond_bits(c) | 0x07F0_00F0 | imm16_split(*imm16)),   // cccc 01111111 imm12 1111 imm4
            Self::Eret_A1(c) => Ok(cond_bits(c) | 0x0160_006E),                               // cccc 0001 0110 0000 0000 0000 0110 1110
            Self::Sevl_A1(c) => Ok(cond_bits(c) | 0x0320_F005),                                // hint8 = 5

            // -- CRC32 (ARMv8-A) --  cccc 00010 sz 0 Rn Rd 0000 0 C 00 0100 Rm
            Self::Crc32b_A1(c, rd, rn, rm) => Ok(cond_bits(c) | 0x0100_0040 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rm)),
            Self::Crc32h_A1(c, rd, rn, rm) => Ok(cond_bits(c) | 0x0120_0040 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rm)),
            Self::Crc32w_A1(c, rd, rn, rm) => Ok(cond_bits(c) | 0x0140_0040 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rm)),
            Self::Crc32cb_A1(c, rd, rn, rm) => Ok(cond_bits(c) | 0x0100_0240 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rm)),
            Self::Crc32ch_A1(c, rd, rn, rm) => Ok(cond_bits(c) | 0x0120_0240 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rm)),
            Self::Crc32cw_A1(c, rd, rn, rm) => Ok(cond_bits(c) | 0x0140_0240 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rm)),

            // -- load-acquire / store-release (ARMv8-A) --
            Self::Lda_A1(c, rt, rn) => Ok(cond_bits(c) | 0x0190_0C9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Ldab_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01D0_0C9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Ldah_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01F0_0C9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Stl_A1(c, rt, rn) => Ok(cond_bits(c) | 0x0180_FC90 | (reg(rn) << 16) | reg(rt)),
            Self::Stlb_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01C0_FC90 | (reg(rn) << 16) | reg(rt)),
            Self::Stlh_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01E0_FC90 | (reg(rn) << 16) | reg(rt)),
            Self::Ldaex_A1(c, rt, rn) => Ok(cond_bits(c) | 0x0190_0E9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Ldaexb_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01D0_0E9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Ldaexh_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01F0_0E9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Ldaexd_A1(c, rt, rn) => Ok(cond_bits(c) | 0x01B0_0E9F | (reg(rn) << 16) | (reg(rt) << 12)),
            Self::Stlex_A1(c, rd, rt, rn) => Ok(cond_bits(c) | 0x0180_0E90 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rt)),
            Self::Stlexb_A1(c, rd, rt, rn) => Ok(cond_bits(c) | 0x01C0_0E90 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rt)),
            Self::Stlexh_A1(c, rd, rt, rn) => Ok(cond_bits(c) | 0x01E0_0E90 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rt)),
            Self::Stlexd_A1(c, rd, rt, rn) => Ok(cond_bits(c) | 0x01A0_0E90 | (reg(rn) << 16) | (reg(rd) << 12) | reg(rt)),

            // -- floating-point (VFP) load/store --
            Self::Vldr_Single_A1(c, sd, rn, off) => encode_fp_load_store_a32(c, 0x0D10_0A00, sd.field(), sd.extra_bit(), reg(rn), *off),
            Self::Vstr_Single_A1(c, sd, rn, off) => encode_fp_load_store_a32(c, 0x0D00_0A00, sd.field(), sd.extra_bit(), reg(rn), *off),
            Self::Vldr_Double_A1(c, dd, rn, off) => encode_fp_load_store_a32(c, 0x0D10_0B00, dd.field(), dd.extra_bit(), reg(rn), *off),
            Self::Vstr_Double_A1(c, dd, rn, off) => encode_fp_load_store_a32(c, 0x0D00_0B00, dd.field(), dd.extra_bit(), reg(rn), *off),
            Self::Vldm_Single_A1(c, rn, wb, db, first, count) => encode_fp_load_store_multiple_a32(c, 0x0A00, true, reg(rn), *wb, *db, first.field(), first.extra_bit(), *count, first.number(), 31, false),
            Self::Vstm_Single_A1(c, rn, wb, db, first, count) => encode_fp_load_store_multiple_a32(c, 0x0A00, false, reg(rn), *wb, *db, first.field(), first.extra_bit(), *count, first.number(), 31, false),
            Self::Vldm_Double_A1(c, rn, wb, db, first, count) => encode_fp_load_store_multiple_a32(c, 0x0B00, true, reg(rn), *wb, *db, first.field(), first.extra_bit(), *count, first.number(), 15, true),
            Self::Vstm_Double_A1(c, rn, wb, db, first, count) => encode_fp_load_store_multiple_a32(c, 0x0B00, false, reg(rn), *wb, *db, first.field(), first.extra_bit(), *count, first.number(), 15, true),

            // -- floating-point (VFP) data-processing --
            Self::FpDataProcess3_Single_A1(c, op, sd, sn, sm) => Ok(cond_bits(c) | 0x0E00_0A00 | op.opcode_bits() | (sd.extra_bit() << 22) | (sn.field() << 16) | (sd.field() << 12) | (sn.extra_bit() << 7) | (sm.extra_bit() << 5) | sm.field()),
            Self::FpDataProcess3_Double_A1(c, op, dd, dn, dm) => Ok(cond_bits(c) | 0x0E00_0B00 | op.opcode_bits() | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | (dn.extra_bit() << 7) | (dm.extra_bit() << 5) | dm.field()),
            Self::FpDataProcess2_Single_A1(c, op, sd, sm) => Ok(cond_bits(c) | (op.base() & 0x0FFF_FFFF) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::FpDataProcess2_Double_A1(c, op, dd, dm) => Ok(cond_bits(c) | (op.base() & 0x0FFF_FFFF) | (1 << 8) | (dd.extra_bit() << 22) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field()),

            // -- floating-point (VFP) compare / transfer / immediate --
            Self::Vcmp_Single_A1(c, sd, sm, e) => Ok(cond_bits(c) | 0x0EB4_0A40 | (sd.extra_bit() << 22) | (sd.field() << 12) | ((*e as u32) << 7) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vcmp_Double_A1(c, dd, dm, e) => Ok(cond_bits(c) | 0x0EB4_0B40 | (dd.extra_bit() << 22) | (dd.field() << 12) | ((*e as u32) << 7) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vcmp_Zero_Single_A1(c, sd, e) => Ok(cond_bits(c) | 0x0EB5_0A40 | (sd.extra_bit() << 22) | (sd.field() << 12) | ((*e as u32) << 7)),
            Self::Vcmp_Zero_Double_A1(c, dd, e) => Ok(cond_bits(c) | 0x0EB5_0B40 | (dd.extra_bit() << 22) | (dd.field() << 12) | ((*e as u32) << 7)),
            Self::Vmrs_A1(c, rt) => Ok(cond_bits(c) | 0x0EF1_0A10 | (reg(rt) << 12)),
            Self::Vmrs_Apsr_Nzcv_A1(c) => Ok(cond_bits(c) | 0x0EF1_FA10),
            Self::Vmsr_A1(c, rt) => Ok(cond_bits(c) | 0x0EE1_0A10 | (reg(rt) << 12)),
            Self::Vmov_Core_To_Single_A1(c, sn, rt) => Ok(cond_bits(c) | 0x0E00_0A10 | (sn.field() << 16) | (reg(rt) << 12) | (sn.extra_bit() << 7)),
            Self::Vmov_Single_To_Core_A1(c, rt, sn) => Ok(cond_bits(c) | 0x0E10_0A10 | (sn.field() << 16) | (reg(rt) << 12) | (sn.extra_bit() << 7)),
            Self::Vmov_Immediate_Single_A1(c, sd, imm8) => Ok(cond_bits(c) | 0x0EB0_0A00 | (((*imm8 as u32) >> 4) << 16) | (sd.extra_bit() << 22) | (sd.field() << 12) | ((*imm8 as u32) & 0xF)),
            Self::Vmov_Immediate_Double_A1(c, dd, imm8) => Ok(cond_bits(c) | 0x0EB0_0B00 | (((*imm8 as u32) >> 4) << 16) | (dd.extra_bit() << 22) | (dd.field() << 12) | ((*imm8 as u32) & 0xF)),
            Self::Vmov_Double_To_CorePair_A1(c, rt, rt2, dm) => Ok(encode_vmov_core_pair_a32(c, 0x0C40_0B10, true, reg(rt), reg(rt2), dm.field(), dm.extra_bit())),
            Self::Vmov_CorePair_To_Double_A1(c, dm, rt, rt2) => Ok(encode_vmov_core_pair_a32(c, 0x0C40_0B10, false, reg(rt), reg(rt2), dm.field(), dm.extra_bit())),
            Self::Vmov_Singles_To_CorePair_A1(c, rt, rt2, sm) => Ok(encode_vmov_core_pair_a32(c, 0x0C40_0A10, true, reg(rt), reg(rt2), sm.field(), sm.extra_bit())),
            Self::Vmov_CorePair_To_Singles_A1(c, sm, rt, rt2) => Ok(encode_vmov_core_pair_a32(c, 0x0C40_0A10, false, reg(rt), reg(rt2), sm.field(), sm.extra_bit())),
            Self::Vmov_Core_To_Scalar_A1(c, size, index, dd, rt) => {
                if *index >= size.lane_count() {
                    return Err(EncodeError::ImmediateOutOfRange { field: "VMOV scalar lane index", value: *index as i64, minimum: 0, maximum: size.lane_count() as i64 - 1 });
                }
                let (opc1, opc2) = size.opc_fields(*index);
                Ok(cond_bits(c) | 0x0E00_0B10 | (opc1 << 21) | (dd.field() << 16) | (reg(rt) << 12) | (dd.extra_bit() << 7) | (opc2 << 5))
            },
            Self::Vmov_Scalar_To_Core_A1(c, unsigned, size, index, rt, dn) => {
                if *index >= size.lane_count() {
                    return Err(EncodeError::ImmediateOutOfRange { field: "VMOV scalar lane index", value: *index as i64, minimum: 0, maximum: size.lane_count() as i64 - 1 });
                }
                let (opc1, opc2) = size.opc_fields(*index);
                // The .32 transfer has no sign distinction -- U is 0 there; only .8/.16 sign/zero-extend.
                let u = if matches!(size, Arm32VmovLaneSize::Word) { 0 } else { *unsigned as u32 };
                Ok(cond_bits(c) | 0x0E10_0B10 | (u << 23) | (opc1 << 21) | (dn.field() << 16) | (reg(rt) << 12) | (dn.extra_bit() << 7) | (opc2 << 5))
            },

            // -- floating-point (VFP) conversions (VCVT) --
            Self::Vcvt_FloatToInt_FromSingle_A1(c, sd, sm, signed, round) => Ok(cond_bits(c) | 0x0EBC_0A40 | ((*signed as u32) << 16) | ((*round as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vcvt_FloatToInt_FromDouble_A1(c, sd, dm, signed, round) => Ok(cond_bits(c) | 0x0EBC_0B40 | ((*signed as u32) << 16) | ((*round as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vcvt_IntToFloat_ToSingle_A1(c, sd, sm, signed) => Ok(cond_bits(c) | 0x0EB8_0A40 | ((*signed as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vcvt_IntToFloat_ToDouble_A1(c, dd, sm, signed) => Ok(cond_bits(c) | 0x0EB8_0B40 | ((*signed as u32) << 7) | (dd.extra_bit() << 22) | (dd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vcvt_Single_To_Double_A1(c, dd, sm) => Ok(cond_bits(c) | 0x0EB7_0AC0 | (dd.extra_bit() << 22) | (dd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vcvt_Double_To_Single_A1(c, sd, dm) => Ok(cond_bits(c) | 0x0EB7_0BC0 | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vcvt_HalfToSingle_A1(c, sd, sm, top) => Ok(cond_bits(c) | 0x0EB2_0A40 | ((*top as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vjcvt_A1(c, sd, dm) => Ok(cond_bits(c) | 0x0EB9_0BC0 | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vcvt_HalfToDouble_A1(c, dd, sm, top) => Ok(cond_bits(c) | 0x0EB2_0B40 | ((*top as u32) << 7) | (dd.extra_bit() << 22) | (dd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vcvt_DoubleToHalf_A1(c, sd, dm, top) => Ok(cond_bits(c) | 0x0EB3_0B40 | ((*top as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vcvt_SingleToHalf_A1(c, sd, sm, top) => Ok(cond_bits(c) | 0x0EB3_0A40 | ((*top as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vcvt_FloatToFixed_Single_A1(c, sd, signed, bits32, frac) => encode_vcvt_fixed_a32(c, sd.field(), sd.extra_bit(), 0, true, *signed, *bits32, *frac),
            Self::Vcvt_FloatToFixed_Double_A1(c, dd, signed, bits32, frac) => encode_vcvt_fixed_a32(c, dd.field(), dd.extra_bit(), 1, true, *signed, *bits32, *frac),
            Self::Vcvt_FixedToFloat_Single_A1(c, sd, signed, bits32, frac) => encode_vcvt_fixed_a32(c, sd.field(), sd.extra_bit(), 0, false, *signed, *bits32, *frac),
            Self::Vcvt_FixedToFloat_Double_A1(c, dd, signed, bits32, frac) => encode_vcvt_fixed_a32(c, dd.field(), dd.extra_bit(), 1, false, *signed, *bits32, *frac),

            // -- ARMv8-A floating-point additions (VSEL / VMAXNM / VMINNM / VRINT / VCVTA-N-P-M) --
            Self::Vsel_Single_A1(cc, sd, sn, sm) => Ok(0xFE00_0A00 | (cc.cc_bits() << 20) | (sd.extra_bit() << 22) | (sn.field() << 16) | (sd.field() << 12) | (sn.extra_bit() << 7) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vsel_Double_A1(cc, dd, dn, dm) => Ok(0xFE00_0B00 | (cc.cc_bits() << 20) | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | (dn.extra_bit() << 7) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vmaxnm_Single_A1(sd, sn, sm) => Ok(0xFE80_0A00 | (sd.extra_bit() << 22) | (sn.field() << 16) | (sd.field() << 12) | (sn.extra_bit() << 7) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vmaxnm_Double_A1(dd, dn, dm) => Ok(0xFE80_0B00 | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | (dn.extra_bit() << 7) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vminnm_Single_A1(sd, sn, sm) => Ok(0xFE80_0A40 | (sd.extra_bit() << 22) | (sn.field() << 16) | (sd.field() << 12) | (sn.extra_bit() << 7) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vminnm_Double_A1(dd, dn, dm) => Ok(0xFE80_0B40 | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | (dn.extra_bit() << 7) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vrint_Directed_Single_A1(mode, sd, sm) => Ok(0xFEB8_0A40 | (mode.rm_bits() << 16) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vrint_Directed_Double_A1(mode, dd, dm) => Ok(0xFEB8_0B40 | (mode.rm_bits() << 16) | (dd.extra_bit() << 22) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field()),
            Self::Vrint_Cond_Single_A1(c, mode, sd, sm) => { let (opc2, op7) = mode.selector_bits(); Ok(cond_bits(c) | 0x0EB0_0A40 | (opc2 << 16) | (op7 << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()) },
            Self::Vrint_Cond_Double_A1(c, mode, dd, dm) => { let (opc2, op7) = mode.selector_bits(); Ok(cond_bits(c) | 0x0EB0_0B40 | (opc2 << 16) | (op7 << 7) | (dd.extra_bit() << 22) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field()) },
            Self::Vcvt_Directed_FromSingle_A1(mode, sd, sm, signed) => Ok(0xFEBC_0A40 | (mode.rm_bits() << 16) | ((*signed as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()),
            Self::Vcvt_Directed_FromDouble_A1(mode, sd, dm, signed) => Ok(0xFEBC_0B40 | (mode.rm_bits() << 16) | ((*signed as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field()),

            // -- NEON 3-reg-same-length data-processing --
            Self::NeonInt3Same_D_A1(op, size, dd, dn, dm) => { let (u, opc, o) = op.fields(); Ok(encode_neon_3same(u, size.size_bits(), opc, o, 0, dd.extra_bit(), dd.field(), dn.extra_bit(), dn.field(), dm.extra_bit(), dm.field())) },
            Self::NeonInt3Same_Q_A1(op, size, qd, qn, qm) => { let (u, opc, o) = op.fields(); Ok(encode_neon_3same(u, size.size_bits(), opc, o, 1, qd.extra_bit(), qd.field(), qn.extra_bit(), qn.field(), qm.extra_bit(), qm.field())) },
            Self::NeonFloat3Same_D_A1(op, dd, dn, dm) => { let (u, opc, o, sz) = op.fields(); Ok(encode_neon_3same(u, sz, opc, o, 0, dd.extra_bit(), dd.field(), dn.extra_bit(), dn.field(), dm.extra_bit(), dm.field())) },
            Self::NeonFloat3Same_Q_A1(op, qd, qn, qm) => { let (u, opc, o, sz) = op.fields(); Ok(encode_neon_3same(u, sz, opc, o, 1, qd.extra_bit(), qd.field(), qn.extra_bit(), qn.field(), qm.extra_bit(), qm.field())) },
            Self::NeonBitwise3Same_D_A1(op, dd, dn, dm) => { let (u, sz) = op.fields(); Ok(encode_neon_3same(u, sz, 0b0001, 1, 0, dd.extra_bit(), dd.field(), dn.extra_bit(), dn.field(), dm.extra_bit(), dm.field())) },
            Self::NeonBitwise3Same_Q_A1(op, qd, qn, qm) => { let (u, sz) = op.fields(); Ok(encode_neon_3same(u, sz, 0b0001, 1, 1, qd.extra_bit(), qd.field(), qn.extra_bit(), qn.field(), qm.extra_bit(), qm.field())) },

            // -- NEON 2-reg-misc --
            Self::NeonMisc2Sized_D_A1(op, size, dd, dm) => { let (a, opc2) = op.fields(); Ok(encode_neon_2misc(a, size.size_bits(), opc2, 0, dd.extra_bit(), dd.field(), dm.extra_bit(), dm.field())) },
            Self::NeonMisc2Sized_Q_A1(op, size, qd, qm) => { let (a, opc2) = op.fields(); Ok(encode_neon_2misc(a, size.size_bits(), opc2, 1, qd.extra_bit(), qd.field(), qm.extra_bit(), qm.field())) },
            Self::NeonMisc2Fixed_D_A1(op, dd, dm) => { let (a, opc2, sz) = op.fields(); Ok(encode_neon_2misc(a, sz, opc2, 0, dd.extra_bit(), dd.field(), dm.extra_bit(), dm.field())) },
            Self::NeonMisc2Fixed_Q_A1(op, qd, qm) => { let (a, opc2, sz) = op.fields(); Ok(encode_neon_2misc(a, sz, opc2, 1, qd.extra_bit(), qd.field(), qm.extra_bit(), qm.field())) },
            Self::NeonMisc2Narrow_A1(op, size, dd, qm) => { let (opc2, bit6) = op.fields(); Ok(encode_neon_2misc(0b10, size.size_bits().wrapping_sub(1) & 0b11, opc2, bit6, dd.extra_bit(), dd.field(), qm.extra_bit(), qm.field())) },
            Self::NeonShllMax_A1(size, qd, dm) => Ok(encode_neon_2misc(0b10, size.size_bits(), 0b00110, 0, qd.extra_bit(), qd.field(), dm.extra_bit(), dm.field())),

            // -- NEON 3-reg-different-length --
            Self::NeonDiffLong_A1(op, size, qd, dn, dm) => { let (u, opc) = op.fields(); Ok(encode_neon_3diff(u, size.size_bits(), opc, qd.extra_bit(), qd.field(), dn.extra_bit(), dn.field(), dm.extra_bit(), dm.field())) },
            Self::NeonDiffWide_A1(op, size, qd, qn, dm) => { let (u, opc) = op.fields(); Ok(encode_neon_3diff(u, size.size_bits(), opc, qd.extra_bit(), qd.field(), qn.extra_bit(), qn.field(), dm.extra_bit(), dm.field())) },
            Self::NeonDiffNarrow_A1(op, size, dd, qn, qm) => { let (u, opc) = op.fields(); Ok(encode_neon_3diff(u, size.size_bits().wrapping_sub(1) & 0b11, opc, dd.extra_bit(), dd.field(), qn.extra_bit(), qn.field(), qm.extra_bit(), qm.field())) },

            // -- NEON 2-reg-and-a-scalar --
            Self::NeonScalar_D_A1(op, size, dd, dn, dm, index) => { let (vm, m) = neon_scalar_vm(size.size_bits(), dm.number(), *index); Ok(encode_neon_scalar(0, size.size_bits(), op.opc(), dd.extra_bit(), dd.field(), dn.extra_bit(), dn.field(), m, vm)) },
            Self::NeonScalar_Q_A1(op, size, qd, qn, dm, index) => { let (vm, m) = neon_scalar_vm(size.size_bits(), dm.number(), *index); Ok(encode_neon_scalar(1, size.size_bits(), op.opc(), qd.extra_bit(), qd.field(), qn.extra_bit(), qn.field(), m, vm)) },
            Self::NeonScalarLong_A1(op, size, qd, dn, dm, index) => { let (u, opc) = op.fields(); let (vm, m) = neon_scalar_vm(size.size_bits(), dm.number(), *index); Ok(encode_neon_scalar(u, size.size_bits(), opc, qd.extra_bit(), qd.field(), dn.extra_bit(), dn.field(), m, vm)) },

            // -- NEON 2-reg-and-a-shift-amount --
            Self::NeonShift_D_A1(op, size, shift, dd, dm) => {
                let (u, opc, is_left) = op.fields();
                let esize = 8u32 << size.size_bits();
                let field7 = neon_shift_imm6(is_left, esize, *shift as u32)?;
                Ok(encode_neon_shift(u, field7 & 0x3F, opc, field7 >> 6, 0, dd.extra_bit(), dd.field(), dm.extra_bit(), dm.field()))
            },
            Self::NeonShift_Q_A1(op, size, shift, qd, qm) => {
                let (u, opc, is_left) = op.fields();
                let esize = 8u32 << size.size_bits();
                let field7 = neon_shift_imm6(is_left, esize, *shift as u32)?;
                Ok(encode_neon_shift(u, field7 & 0x3F, opc, field7 >> 6, 1, qd.extra_bit(), qd.field(), qm.extra_bit(), qm.field()))
            },
            Self::NeonShiftNarrow_A1(op, size, shift, dd, qm) => {
                let (u, opc, r) = op.fields();
                let src_esize = 8u32 << size.size_bits();
                let shift = *shift as u32;
                if shift < 1 || shift > src_esize {
                    return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: shift as i64, minimum: 1, maximum: src_esize as i64 });
                }
                Ok(encode_neon_shift(u, src_esize - shift, opc, 0, r, dd.extra_bit(), dd.field(), qm.extra_bit(), qm.field()))
            },
            Self::NeonShiftLong_A1(signed, size, shift, qd, dm) => {
                let src_esize = 8u32 << size.size_bits();
                Ok(encode_neon_shift(*signed as u32, src_esize + *shift as u32, 0b1010, 0, 0, qd.extra_bit(), qd.field(), dm.extra_bit(), dm.field()))
            },

            // -- NEON extract / table / duplicate / modified-immediate --
            Self::NeonExt_D_A1(imm4, dd, dn, dm) => Ok(0xF2B0_0000 | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | ((*imm4 as u32 & 0xF) << 8) | (dn.extra_bit() << 7) | (dm.extra_bit() << 5) | dm.field()),
            Self::NeonExt_Q_A1(imm4, qd, qn, qm) => Ok(0xF2B0_0000 | (qd.extra_bit() << 22) | (qn.field() << 16) | (qd.field() << 12) | ((*imm4 as u32 & 0xF) << 8) | (qn.extra_bit() << 7) | (1 << 6) | (qm.extra_bit() << 5) | qm.field()),
            Self::NeonTableLookup_A1(is_vtbx, length, dd, dn, dm) => {
                let len2 = (*length as u32).wrapping_sub(1) & 0b11;
                Ok(0xF3B0_0000 | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | ((0b1000 | len2) << 8) | (dn.extra_bit() << 7) | ((*is_vtbx as u32) << 6) | (dm.extra_bit() << 5) | dm.field())
            },
            Self::NeonVdupScalar_D_A1(size, index, dd, dm) => Ok(0xF3B0_0C00 | (dd.extra_bit() << 22) | (vdup_scalar_imm4(*size, *index) << 16) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field()),
            Self::NeonVdupScalar_Q_A1(size, index, qd, dm) => Ok(0xF3B0_0C00 | (qd.extra_bit() << 22) | (vdup_scalar_imm4(*size, *index) << 16) | (qd.field() << 12) | (1 << 6) | (dm.extra_bit() << 5) | dm.field()),
            Self::NeonVdupCore_D_A1(c, size, dd, rt) => { let (b, e) = vdup_core_be(*size); Ok(cond_bits(c) | 0x0E80_0B10 | (b << 22) | (dd.field() << 16) | (reg(rt) << 12) | (dd.extra_bit() << 7) | (e << 5)) },
            Self::NeonVdupCore_Q_A1(c, size, qd, rt) => { let (b, e) = vdup_core_be(*size); Ok(cond_bits(c) | 0x0E80_0B10 | (b << 22) | (1 << 21) | (qd.field() << 16) | (reg(rt) << 12) | (qd.extra_bit() << 7) | (e << 5)) },
            Self::NeonModifiedImmediate_D_A1(cmode, op, imm8, dd) => Ok(encode_neon_modified_imm(*cmode, *op, *imm8, 0, dd.extra_bit(), dd.field())),
            Self::NeonModifiedImmediate_Q_A1(cmode, op, imm8, qd) => Ok(encode_neon_modified_imm(*cmode, *op, *imm8, 1, qd.extra_bit(), qd.field())),

            // -- NEON element/structure load & store (VLD1-4 / VST1-4) --
            Self::NeonLoadStoreMultiple_A1(is_load, type_bits, size, align, first, rn, address) =>
                Ok(0xF400_0000 | (first.extra_bit() << 22) | ((*is_load as u32) << 21) | (reg(rn) << 16) | (first.field() << 12) | ((*type_bits as u32 & 0xF) << 8) | (size.size_bits() << 6) | ((*align as u32 & 0b11) << 4) | address.rm_bits()),
            Self::NeonLoadStoreSingleLane_A1(is_load, struct_count, size, index_align, first, rn, address) =>
                Ok(0xF480_0000 | (first.extra_bit() << 22) | ((*is_load as u32) << 21) | (reg(rn) << 16) | (first.field() << 12) | ((*size as u32 & 0b11) << 10) | (((*struct_count as u32).wrapping_sub(1) & 0b11) << 8) | ((*index_align as u32 & 0xF) << 4) | address.rm_bits()),
            Self::NeonLoadStoreAllLanes_A1(struct_count, size, t, a, first, rn, address) =>
                Ok(0xF480_0000 | (first.extra_bit() << 22) | (1 << 21) | (reg(rn) << 16) | (first.field() << 12) | (0b11 << 10) | (((*struct_count as u32).wrapping_sub(1) & 0b11) << 8) | ((*size as u32 & 0b11) << 6) | ((*t as u32) << 5) | ((*a as u32) << 4) | address.rm_bits()),

            // -- ARMv8 cryptography extension --
            Self::NeonAes_A1(op, qd, qm) => Ok(0xF3B0_0300 | (qd.extra_bit() << 22) | (qd.field() << 12) | (op.op_bits() << 6) | (qm.extra_bit() << 5) | qm.field()),
            Self::NeonSha3Reg_A1(op, qd, qn, qm) => { let (u, size) = op.fields(); Ok(encode_neon_3same(u, size, 0b1100, 0, 1, qd.extra_bit(), qd.field(), qn.extra_bit(), qn.field(), qm.extra_bit(), qm.field())) },
            Self::NeonSha2Reg_A1(op, qd, qm) => Ok(op.base() | (qd.extra_bit() << 22) | (qd.field() << 12) | (qm.extra_bit() << 5) | qm.field()),

            // -- preload (unconditional) --
            Self::Pld_A1(rn, offset) => encode_preload(0xF550_F000, 0xF750_F000, reg(rn), offset),
            Self::Pldw_A1(rn, offset) => encode_preload(0xF510_F000, 0xF710_F000, reg(rn), offset),
            Self::Pli_A1(rn, offset) => encode_preload(0xF450_F000, 0xF650_F000, reg(rn), offset),

            // -- exception save/return (unconditional) --
            Self::Rfe_A1(mode, rn, wb) => { let (p, u) = mode.p_u_bits(); Ok(0xF810_0A00 | (p << 24) | (u << 23) | ((*wb as u32) << 21) | (reg(rn) << 16)) }, // 1111 100 P U 0 W 1 Rn 0000 1010 0000 0000
            Self::Srs_A1(mode, wb, mode_num) => { let (p, u) = mode.p_u_bits(); Ok(0xF84D_0500 | (p << 24) | (u << 23) | ((*wb as u32) << 21) | ((*mode_num & 0x1F) as u32)) }, // 1111 100 P U 1 W 0 1101 0000 0101 mode

            // -- branch / interwork --
            Self::B_A1(c, offset) => encode_a32_branch(c, 0x0A00_0000, *offset),
            Self::Bl_A1(c, offset) => encode_a32_branch(c, 0x0B00_0000, *offset),
            Self::Blx_Immediate_A1(offset) => {
                // 1111 101 H imm24 ; offset = SignExtend(imm24:H:0, 26), a multiple of 2 (H is offset bit 1)
                check_multiple_of_2("offset", *offset)?;
                let imm = *offset >> 2;
                check_signed_24("offset", imm, *offset)?;
                let h = ((*offset >> 1) & 1) as u32;
                Ok(0xFA00_0000 | (h << 24) | ((imm as u32) & 0x00FF_FFFF))
            },
            Self::Bx_A1(c, rm) => Ok(cond_bits(c) | 0x012F_FF10 | reg(rm)), // cccc 0001 0010 1111 1111 1111 0001 Rm
            Self::Blx_Register_A1(c, rm) => Ok(cond_bits(c) | 0x012F_FF30 | reg(rm)),
            Self::Bxj_A1(c, rm) => Ok(cond_bits(c) | 0x012F_FF20 | reg(rm)),

            // -- exception generation --
            Self::Svc_A1(c, imm24) => {
                check_unsigned_maximum("imm24", *imm24, 0x00FF_FFFF)?;
                Ok(cond_bits(c) | 0x0F00_0000 | (*imm24 & 0x00FF_FFFF)) // cccc 1111 imm24
            },
        }
    }

    /// Decode one A32 (ARM-state) instruction -- a single little-endian 32-bit word -- from a byte iterator,
    /// advancing `iter_offset` by 4. Returns `Ok(None)` at a clean end of input, `Ok(Some(_))` for a
    /// decoded instruction, or a [`DecodeError`] for malformed or unknown bytes. Never panics on arbitrary
    /// input.
    pub fn decode<'a, I>(iter: &mut I, iter_offset: &mut usize) -> Result<Option<Self>, DecodeError> where I: Iterator<Item = &'a u8> {
        let word = match next_u32le_from_iter(iter, iter_offset)? {
            Some(value) => value,
            None => return Ok(None), // EOF; nothing to decode
        };

        let cond = Arm32Condition::from_operand_bits(((word >> 28) & 0b1111) as u8);

        // Specific (fixed-pattern) encodings are tested before the generic data-processing masks they sit
        // inside. The cond==0b1111 "unconditional" group (CLREX, memory barriers, PLD/PLI/PLDW,
        // BLX-immediate) and the misc-instruction space (MRS/MSR and the multiply families) are decoded by
        // the dedicated arms below.

        // CLREX (unconditional) : 1111 0101 0111 1111 0000 0000 0001 1111
        if word == 0xF57F_F01F {
            return Ok(Some(Self::Clrex_A1));
        }
        // memory barriers (unconditional) : 1111 0101 0111 1111 0000 0000 op4 option
        if word & 0xFFFF_FF00 == 0xF57F_F000 {
            let option = (word & 0xF) as u8;
            return Ok(Some(match (word >> 4) & 0xF {
                0b0100 => Self::Dsb_A1(option),
                0b0101 => Self::Dmb_A1(option),
                0b0110 => Self::Isb_A1(option),
                0b0111 => Self::Sb_A1, // SB (option field reads as 0)
                _ => return Err(DecodeError::InvalidOpcode),
            }));
        }
        // SETEND (unconditional) : 1111 00010000 0001 0000 00 E 0 0000 0000
        if word & 0xFFFF_FDFF == 0xF101_0000 {
            return Ok(Some(Self::Setend_A1((word >> 9) & 1 == 1)));
        }
        // CPS (unconditional) : 1111 00010000 imod M 0 ... A I F 0 mode
        if word & 0xFFF1_0020 == 0xF100_0000
            && let Some(mode) = Arm32CpsMode::from_imod_bits((word >> 18) & 0b11) {
                let new_mode = if (word >> 17) & 1 == 1 { Some((word & 0x1F) as u8) } else { None };
                return Ok(Some(Self::Cps_A1(mode, (word >> 8) & 1 == 1, (word >> 7) & 1 == 1, (word >> 6) & 1 == 1, new_mode)));
            }
        // ARMv8-A unconditional FP additions (VSEL / VMAXNM / VMINNM / VRINT{A,N,P,M} / VCVT{A,N,P,M}) all sit
        // in the 1111 1110 ... 101 sz ... space; decoded before the coproc group (which skips coproc 1010/1011).
        if word & 0xFF00_0E00 == 0xFE00_0A00 {
            let double = (word >> 8) & 1 == 1;
            let d_bit = (word >> 22) & 1;
            let vd_field = (word >> 12) & 0b1111;
            let vn_field = (word >> 16) & 0b1111;
            let n_bit = (word >> 7) & 1;
            let vm_field = word & 0b1111;
            let m_bit = (word >> 5) & 1;
            if (word >> 23) & 1 == 0 {
                // VSEL : cc = bits[21:20]
                let cc = Arm32VselCondition::from_cc_bits((word >> 20) & 0b11);
                return Ok(Some(if double {
                    Self::Vsel_Double_A1(cc, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                } else {
                    Self::Vsel_Single_A1(cc, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                }));
            }
            // bit23 == 1: VMAXNM/VMINNM ([21:20]=00, op6 picks min) or VRINT/VCVT directed ([21:20]=11).
            if (word >> 20) & 0b11 == 0b00 {
                let is_min = (word >> 6) & 1 == 1;
                return Ok(Some(match (is_min, double) {
                    (false, false) => Self::Vmaxnm_Single_A1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit)),
                    (false, true) => Self::Vmaxnm_Double_A1(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit)),
                    (true, false) => Self::Vminnm_Single_A1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit)),
                    (true, true) => Self::Vminnm_Double_A1(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit)),
                }));
            }
            if (word >> 20) & 0b11 == 0b11 {
                let rm = Arm32DirectedRound::from_rm_bits((word >> 16) & 0b11);
                if (word >> 18) & 1 == 0 {
                    // VRINT{A,N,P,M} (round float to integral float)
                    return Ok(Some(if double {
                        Self::Vrint_Directed_Double_A1(rm, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    } else {
                        Self::Vrint_Directed_Single_A1(rm, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    }));
                } else {
                    // VCVT{A,N,P,M} (float -> integer in a single register; bit7 = signed)
                    let signed = (word >> 7) & 1 == 1;
                    return Ok(Some(if double {
                        Self::Vcvt_Directed_FromDouble_A1(rm, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit), signed)
                    } else {
                        Self::Vcvt_Directed_FromSingle_A1(rm, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), signed)
                    }));
                }
            }
            // [21:20] = 01/10 with bit23=1 is unallocated here; fall through to the remaining decoders.
        }
        // NEON (Advanced SIMD) three-registers-of-the-same-length : 1111 001 U 0 D size Vn Vd opc N Q M op Vm.
        // Within the 1111001 space, bit[23]=0 uniquely marks this format (shifts/VEXT/misc/imm/diff/scalar all
        // set bit[23]=1). The opc routes the family: opc>=1100 float, opc==0001 & op==1 bitwise, else integer.
        if word & 0xFE80_0000 == 0xF200_0000 {
            let u = (word >> 24) & 1;
            let size = (word >> 20) & 0b11;
            let opc = (word >> 8) & 0b1111;
            let op = (word >> 4) & 1;
            let q = (word >> 6) & 1 == 1;
            let d_bit = (word >> 22) & 1;
            let n_bit = (word >> 7) & 1;
            let m_bit = (word >> 5) & 1;
            let vd_field = (word >> 12) & 0b1111;
            let vn_field = (word >> 16) & 0b1111;
            let vm_field = word & 0b1111;
            if opc >= 0b1100 {
                if let Some(op_f) = Arm32NeonFloatOp::from_fields(u, opc, op, size) {
                    return Ok(Some(if q {
                        Self::NeonFloat3Same_Q_A1(op_f, Arm32QuadwordRegister::from_field_and_bit(vd_field, d_bit), Arm32QuadwordRegister::from_field_and_bit(vn_field, n_bit), Arm32QuadwordRegister::from_field_and_bit(vm_field, m_bit))
                    } else {
                        Self::NeonFloat3Same_D_A1(op_f, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    }));
                }
            } else if opc == 0b0001 && op == 1 {
                let op_b = Arm32NeonBitwiseOp::from_fields(u, size);
                return Ok(Some(if q {
                    Self::NeonBitwise3Same_Q_A1(op_b, Arm32QuadwordRegister::from_field_and_bit(vd_field, d_bit), Arm32QuadwordRegister::from_field_and_bit(vn_field, n_bit), Arm32QuadwordRegister::from_field_and_bit(vm_field, m_bit))
                } else {
                    Self::NeonBitwise3Same_D_A1(op_b, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                }));
            } else if let Some(op_i) = Arm32NeonIntegerOp::from_fields(u, opc, op) {
                let sz = Arm32NeonSize::from_size_bits(size);
                return Ok(Some(if q {
                    Self::NeonInt3Same_Q_A1(op_i, sz, Arm32QuadwordRegister::from_field_and_bit(vd_field, d_bit), Arm32QuadwordRegister::from_field_and_bit(vn_field, n_bit), Arm32QuadwordRegister::from_field_and_bit(vm_field, m_bit))
                } else {
                    Self::NeonInt3Same_D_A1(op_i, sz, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                }));
            }
            // unrecognized 3-reg-same encoding: fall through.
        }
        // ARMv8 cryptography extension (NEON). Decoded before the generic 2-reg-misc / (after) 3-reg-same blocks
        // they nominally sit in, since those op tables intentionally exclude these opcodes.
        // AES : 1111 0011 1 D 11 00 00 Vd 0011 op M 0 Vm.
        if word & 0xFFFF_0F10 == 0xF3B0_0300 {
            let op = Arm32NeonAesOp::from_op_bits((word >> 6) & 0b11);
            let qd = Arm32QuadwordRegister::from_field_and_bit((word >> 12) & 0b1111, (word >> 22) & 1);
            let qm = Arm32QuadwordRegister::from_field_and_bit(word & 0b1111, (word >> 5) & 1);
            return Ok(Some(Self::NeonAes_A1(op, qd, qm)));
        }
        // SHA1/SHA256 two-register ops (each a fixed base word).
        if let Some(op) = Arm32NeonSha2Op::from_base(word & 0xFFFF_0FD0) {
            let qd = Arm32QuadwordRegister::from_field_and_bit((word >> 12) & 0b1111, (word >> 22) & 1);
            let qm = Arm32QuadwordRegister::from_field_and_bit(word & 0b1111, (word >> 5) & 1);
            return Ok(Some(Self::NeonSha2Reg_A1(op, qd, qm)));
        }
        // SHA1/SHA256 three-register ops : 1111 001 U 0 D size Vn Vd 1100 N 1 M 0 Vm (opc=1100, op=0, Q=1).
        if word & 0xFE80_0F50 == 0xF200_0C40
            && let Some(op) = Arm32NeonSha3Op::from_fields((word >> 24) & 1, (word >> 20) & 0b11) {
                let qd = Arm32QuadwordRegister::from_field_and_bit((word >> 12) & 0b1111, (word >> 22) & 1);
                let qn = Arm32QuadwordRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1);
                let qm = Arm32QuadwordRegister::from_field_and_bit(word & 0b1111, (word >> 5) & 1);
                return Ok(Some(Self::NeonSha3Reg_A1(op, qd, qn, qm)));
            }
        // NEON two-registers-miscellaneous : 1111 0011 1 D 11 size a Vd opc2 bit6 M 0 Vm. Identified by
        // [24:23,21:20]=(1,1,11) with size=11 and [4]=0 -- the [4]=0 keeps a shift-immediate word whose imm6
        // top bits read as [21:20]=11 (it has [4]=1) from being mis-claimed here.
        if word & 0xFFB0_0010 == 0xF3B0_0000 {
            let d_bit = (word >> 22) & 1;
            let size_field = (word >> 18) & 0b11;
            let a = (word >> 16) & 0b11;
            let vd = (word >> 12) & 0b1111;
            let opc2 = (word >> 7) & 0b11111;
            let bit6 = (word >> 6) & 1;
            let m_bit = (word >> 5) & 1;
            let vm = word & 0b1111;
            let q = bit6 == 1;
            // narrowing (a=10, opc2 00100/00101): Qm -> Dd; bit6 is a sub-opcode, not Q.
            if a == 0b10 && (opc2 == 0b00100 || opc2 == 0b00101)
                && let Some(op) = Arm32NeonNarrowOp::from_fields(opc2, bit6) {
                    let src_size = Arm32NeonSize::from_size_bits((size_field + 1) & 0b11);
                    return Ok(Some(Self::NeonMisc2Narrow_A1(op, src_size, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vm, m_bit))));
                }
            // VSHLL by element size (a=10, opc2 00110): Dm -> Qd.
            if a == 0b10 && opc2 == 0b00110 {
                let size = Arm32NeonSize::from_size_bits(size_field);
                return Ok(Some(Self::NeonShllMax_A1(size, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))));
            }
            // same-width ops: element-sized first, then fixed-size (their (a,opc2) sets are disjoint).
            if let Some(op) = Arm32NeonMisc2SizedOp::from_fields(a, opc2) {
                let size = Arm32NeonSize::from_size_bits(size_field);
                return Ok(Some(if q {
                    Self::NeonMisc2Sized_Q_A1(op, size, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vm, m_bit))
                } else {
                    Self::NeonMisc2Sized_D_A1(op, size, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))
                }));
            }
            if let Some(op) = Arm32NeonMisc2FixedOp::from_fields(a, opc2) {
                return Ok(Some(if q {
                    Self::NeonMisc2Fixed_Q_A1(op, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vm, m_bit))
                } else {
                    Self::NeonMisc2Fixed_D_A1(op, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))
                }));
            }
            // unrecognized (e.g. VDUP-scalar opc2=11xxx, handled elsewhere): fall through.
        }
        // NEON three-registers-of-different-lengths : 1111 001 U 1 D size Vn Vd opc N 0 M 0 Vm. [23]=1, [6]=0,
        // [4]=0; size != 11 (size==11 is the 2-reg-misc/VEXT space, already handled above). opc=[11:8] -> shape.
        if word & 0xFE80_0050 == 0xF280_0000 && (word >> 20) & 0b11 != 0b11 {
            let u = (word >> 24) & 1;
            let size = (word >> 20) & 0b11;
            let opc = (word >> 8) & 0b1111;
            let d_bit = (word >> 22) & 1;
            let n_bit = (word >> 7) & 1;
            let m_bit = (word >> 5) & 1;
            let vd = (word >> 12) & 0b1111;
            let vn = (word >> 16) & 0b1111;
            let vm = word & 0b1111;
            if let Some(op) = Arm32NeonDiffWideOp::from_fields(u, opc) {
                let sz = Arm32NeonSize::from_size_bits(size);
                return Ok(Some(Self::NeonDiffWide_A1(op, sz, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vn, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))));
            }
            if let Some(op) = Arm32NeonDiffNarrowOp::from_fields(u, opc) {
                let sz = Arm32NeonSize::from_size_bits((size + 1) & 0b11);
                return Ok(Some(Self::NeonDiffNarrow_A1(op, sz, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vn, n_bit), Arm32QuadwordRegister::from_field_and_bit(vm, m_bit))));
            }
            if let Some(op) = Arm32NeonDiffLongOp::from_fields(u, opc) {
                let sz = Arm32NeonSize::from_size_bits(size);
                return Ok(Some(Self::NeonDiffLong_A1(op, sz, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))));
            }
            // unrecognized opc: fall through.
        }
        // NEON two-registers-and-a-scalar : 1111 001 X 1 D size Vn Vd opc N 1 M 0 Vm. [23]=1, [6]=1, [4]=0,
        // size != 11. opc[1] (bit 9): 0 = same-length (bit24 = Q), 1 = long (bit24 = U). Scalar is Dm[index].
        if word & 0xFE80_0050 == 0xF280_0040 && (word >> 20) & 0b11 != 0b11 {
            let bit24 = (word >> 24) & 1;
            let size = (word >> 20) & 0b11;
            let opc = (word >> 8) & 0b1111;
            let d_bit = (word >> 22) & 1;
            let n_bit = (word >> 7) & 1;
            let m_bit = (word >> 5) & 1;
            let vd = (word >> 12) & 0b1111;
            let vn = (word >> 16) & 0b1111;
            let vm = word & 0b1111;
            let (dm_num, index) = neon_scalar_decode(size, vm, m_bit);
            // dm_num is <= 15 (vm & 0xF); use the total constructor (no unwrap in the decode path).
            let dm = Arm32DoublePrecisionRegister::from_field_and_bit(dm_num as u32, 0);
            let sz = Arm32NeonSize::from_size_bits(size);
            if (opc >> 1) & 1 == 0 {
                // same-length (opc[1]=0): bit24 = Q
                if let Some(op) = Arm32NeonScalarOp::from_opc(opc) {
                    return Ok(Some(if bit24 == 1 {
                        Self::NeonScalar_Q_A1(op, sz, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vn, n_bit), dm, index)
                    } else {
                        Self::NeonScalar_D_A1(op, sz, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn, n_bit), dm, index)
                    }));
                }
            } else if let Some(op) = Arm32NeonScalarLongOp::from_fields(bit24, opc) {
                // long (opc[1]=1): bit24 = U
                return Ok(Some(Self::NeonScalarLong_A1(op, sz, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn, n_bit), dm, index)));
            }
            // unrecognized opc: fall through.
        }
        // NEON two-registers-and-a-shift-amount : 1111 001 U 1 D imm6 Vd opc L Q M 1 Vm. [23]=1, [4]=1. The
        // L:imm6 field is < 8 only for the modified-immediate format (its [21:19]=000, L=0) -- guard on that.
        if word & 0xFE80_0010 == 0xF280_0010 {
            let u = (word >> 24) & 1;
            let imm6 = (word >> 16) & 0x3F;
            let opc = (word >> 8) & 0b1111;
            let l = (word >> 7) & 1;
            let bit6 = (word >> 6) & 1;
            let d_bit = (word >> 22) & 1;
            let m_bit = (word >> 5) & 1;
            let vd = (word >> 12) & 0b1111;
            let vm = word & 0b1111;
            if ((l << 6) | imm6) >= 8 {
                if opc <= 0b0111 {
                    // same-width (bit6 = Q)
                    if let Some(op) = Arm32NeonShiftOp::from_fields(u, opc)
                        && let Some((size, shift)) = decode_neon_shift_amount(l, imm6, op.is_left()) {
                            return Ok(Some(if bit6 == 1 {
                                Self::NeonShift_Q_A1(op, size, shift, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vm, m_bit))
                            } else {
                                Self::NeonShift_D_A1(op, size, shift, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))
                            }));
                        }
                } else if opc == 0b1000 || opc == 0b1001 {
                    // narrowing (bit6 = rounding); Qm -> Dd
                    if let Some(op) = Arm32NeonShiftNarrowOp::from_fields(u, opc, bit6)
                        && let Some((size, shift)) = decode_neon_narrow_shift(imm6) {
                        return Ok(Some(Self::NeonShiftNarrow_A1(op, size, shift, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vm, m_bit))));
                    }
                } else if opc == 0b1010 && let Some((size, shift)) = decode_neon_widen_shift(imm6) {
                    // widening VSHLL / VMOVL; Dm -> Qd (imm6 < esize is rejected by the helper)
                    return Ok(Some(Self::NeonShiftLong_A1(u == 1, size, shift, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))));
                }
            }
            // L:imm6 < 8 (modified-immediate) or unrecognized opc: fall through.
        }
        // NEON VEXT (byte extract) : 1111 0010 1 D 11 Vn Vd imm4 N Q M 0 Vm (U=0, [21:20]=11, [4]=0).
        if word & 0xFFB0_0010 == 0xF2B0_0000 {
            let imm4 = ((word >> 8) & 0b1111) as u8;
            let d_bit = (word >> 22) & 1;
            let n_bit = (word >> 7) & 1;
            let m_bit = (word >> 5) & 1;
            let vd = (word >> 12) & 0b1111;
            let vn = (word >> 16) & 0b1111;
            let vm = word & 0b1111;
            return Ok(Some(if (word >> 6) & 1 == 1 {
                Self::NeonExt_Q_A1(imm4, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), Arm32QuadwordRegister::from_field_and_bit(vn, n_bit), Arm32QuadwordRegister::from_field_and_bit(vm, m_bit))
            } else {
                Self::NeonExt_D_A1(imm4, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))
            }));
        }
        // NEON VTBL / VTBX : 1111 0011 1 D 11 Vn Vd 10 len N op M 0 Vm ([11:10]=10).
        if word & 0xFFB0_0C10 == 0xF3B0_0800 {
            let length = (((word >> 8) & 0b11) + 1) as u8;
            let is_vtbx = (word >> 6) & 1 == 1;
            let d_bit = (word >> 22) & 1;
            let n_bit = (word >> 7) & 1;
            let m_bit = (word >> 5) & 1;
            let vd = (word >> 12) & 0b1111;
            let vn = (word >> 16) & 0b1111;
            let vm = word & 0b1111;
            return Ok(Some(Self::NeonTableLookup_A1(is_vtbx, length, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit))));
        }
        // NEON VDUP (scalar) : 1111 0011 1 D 11 imm4 Vd 11000 Q M 0 Vm ([11:7]=11000).
        if word & 0xFFF0_0F90 == 0xF3B0_0C00
            && let Some((size, index)) = decode_vdup_scalar_imm4((word >> 16) & 0b1111) {
                let d_bit = (word >> 22) & 1;
                let m_bit = (word >> 5) & 1;
                let vd = (word >> 12) & 0b1111;
                let vm = word & 0b1111;
                let dm = Arm32DoublePrecisionRegister::from_field_and_bit(vm, m_bit);
                return Ok(Some(if (word >> 6) & 1 == 1 {
                    Self::NeonVdupScalar_Q_A1(size, index, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), dm)
                } else {
                    Self::NeonVdupScalar_D_A1(size, index, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), dm)
                }));
            }
        // NEON VMOV/VMVN/VORR/VBIC (modified immediate) : 1111 001 i 1 D 000 imm3 Vd cmode 0 Q op 1 imm4.
        // [21:19]=000 and [7]=0 keep this disjoint from the shift-immediate format above (which has L:imm6 >= 8).
        if word & 0xFEB8_0090 == 0xF280_0010 {
            let imm8 = ((((word >> 24) & 1) << 7) | (((word >> 16) & 0b111) << 4) | (word & 0xF)) as u8;
            let cmode = ((word >> 8) & 0b1111) as u8;
            let op = (word >> 5) & 1 == 1;
            let d_bit = (word >> 22) & 1;
            let vd = (word >> 12) & 0b1111;
            return Ok(Some(if (word >> 6) & 1 == 1 {
                Self::NeonModifiedImmediate_Q_A1(cmode, op, imm8, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit))
            } else {
                Self::NeonModifiedImmediate_D_A1(cmode, op, imm8, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit))
            }));
        }
        // NEON element/structure load & store (VLD1-4 / VST1-4) : 1111 0100 ... with [20]=0. [23]=0 multiple,
        // [23]=1 single-element (then [11:10]=11 is all-lanes, else single-lane). Disjoint from PLD/PLI ([20]=1).
        if word & 0xFF10_0000 == 0xF400_0000 {
            let d_bit = (word >> 22) & 1;
            let is_load = (word >> 21) & 1 == 1;
            let rn = gpr((word >> 16) & 0b1111);
            let first = Arm32DoublePrecisionRegister::from_field_and_bit((word >> 12) & 0b1111, d_bit);
            let address = Arm32NeonLoadStoreAddress::from_rm_bits(word & 0b1111);
            if (word >> 23) & 1 == 0 {
                let type_bits = ((word >> 8) & 0b1111) as u8;
                let size = Arm32NeonSize::from_size_bits((word >> 6) & 0b11);
                let align = ((word >> 4) & 0b11) as u8;
                return Ok(Some(Self::NeonLoadStoreMultiple_A1(is_load, type_bits, size, align, first, rn, address)));
            } else if (word >> 10) & 0b11 == 0b11 {
                let struct_count = (((word >> 8) & 0b11) + 1) as u8;
                let size = ((word >> 6) & 0b11) as u8;
                return Ok(Some(Self::NeonLoadStoreAllLanes_A1(struct_count, size, (word >> 5) & 1 == 1, (word >> 4) & 1 == 1, first, rn, address)));
            } else {
                let struct_count = (((word >> 8) & 0b11) + 1) as u8;
                let size = ((word >> 10) & 0b11) as u8;
                let index_align = ((word >> 4) & 0b1111) as u8;
                return Ok(Some(Self::NeonLoadStoreSingleLane_A1(is_load, struct_count, size, index_align, first, rn, address)));
            }
        }
        // preload + exception save/return are unconditional (cond=1111) and must precede the cond-agnostic blocks
        // PLI (immediate / register) : 1111 0100/0110 U 101 Rn 1111 ...
        if word & 0xFF70_F000 == 0xF450_F000 {
            return Ok(Some(Self::Pli_A1(gpr((word >> 16) & 0b1111), preload_imm_offset(word))));
        }
        if word & 0xFF70_F010 == 0xF650_F000 {
            return Ok(Some(Self::Pli_A1(gpr((word >> 16) & 0b1111), preload_reg_offset(word))));
        }
        // PLD / PLDW (immediate / register) : 1111 0101/0111 U R 01 Rn 1111 ...  (R=1 PLD, R=0 PLDW)
        if word & 0xFF30_F000 == 0xF510_F000 {
            let rn = gpr((word >> 16) & 0b1111);
            let offset = preload_imm_offset(word);
            return Ok(Some(if (word >> 22) & 1 == 1 { Self::Pld_A1(rn, offset) } else { Self::Pldw_A1(rn, offset) }));
        }
        if word & 0xFF30_F010 == 0xF710_F000 {
            let rn = gpr((word >> 16) & 0b1111);
            let offset = preload_reg_offset(word);
            return Ok(Some(if (word >> 22) & 1 == 1 { Self::Pld_A1(rn, offset) } else { Self::Pldw_A1(rn, offset) }));
        }
        // RFE : 1111 100 P U 0 W 1 Rn 0000 1010 0000 0000
        if word & 0xFE50_FFFF == 0xF810_0A00 {
            let mode = Arm32BlockAddressMode::from_p_u_bits((word >> 24) & 1, (word >> 23) & 1);
            return Ok(Some(Self::Rfe_A1(mode, gpr((word >> 16) & 0b1111), (word >> 21) & 1 == 1)));
        }
        // SRS : 1111 100 P U 1 W 0 1101 0000 0101 mode
        if word & 0xFE4F_FFE0 == 0xF84D_0500 {
            let mode = Arm32BlockAddressMode::from_p_u_bits((word >> 24) & 1, (word >> 23) & 1);
            return Ok(Some(Self::Srs_A1(mode, (word >> 21) & 1 == 1, (word & 0x1F) as u8)));
        }
        // A32 reserves bits[31:28]==0b1111 for the UNCONDITIONAL-instruction space -- it is not a condition.
        // Every modeled unconditional encoding above has been tried; the decoders below are conditional and
        // their masks deliberately do not pin the condition field, so a leftover 0b1111 word would be
        // mis-claimed as a bogus `Undefined(0b1111)`-conditioned instruction (which then re-encodes into a
        // different, genuinely-unconditional word -- a decode/encode asymmetry the robustness sweep flags).
        // Stop here for any such word EXCEPT the two unconditional forms still decoded below: BLX (immediate)
        // at bits[27:25]==0b101, and the coprocessor `...2` encodings at bits[27:26]==0b11 (whose decoders
        // read the condition field themselves and emit the correct unconditional variant).
        if (word >> 28) == 0b1111 && (word >> 26) & 0b11 != 0b11 && (word >> 25) & 0b111 != 0b101 {
            return Err(DecodeError::InvalidOpcode);
        }
        // hints : cccc 0011 0010 0000 1111 0000 0000 hint8
        if word & 0x0FFF_FF00 == 0x0320_F000 {
            return Ok(Some(match word & 0xFF {
                0x00 => Self::Nop_A1(cond),
                0x01 => Self::Yield_A1(cond),
                0x02 => Self::Wfe_A1(cond),
                0x03 => Self::Wfi_A1(cond),
                0x04 => Self::Sev_A1(cond),
                0x05 => Self::Sevl_A1(cond),
                0x10 => Self::Esb_A1(cond),
                0x14 => Self::Csdb_A1(cond),
                hint if hint & 0xF0 == 0xF0 => Self::Dbg_A1(cond, (hint & 0xF) as u8),
                _ => return Err(DecodeError::InvalidOpcode),
            }));
        }
        // BX / BLX (register) / BXJ : cccc 0001 0010 1111 1111 1111 0001/0011/0010 Rm
        if word & 0x0FFF_FFF0 == 0x012F_FF10 {
            return Ok(Some(Self::Bx_A1(cond, gpr(word & 0b1111))));
        }
        if word & 0x0FFF_FFF0 == 0x012F_FF30 {
            return Ok(Some(Self::Blx_Register_A1(cond, gpr(word & 0b1111))));
        }
        if word & 0x0FFF_FFF0 == 0x012F_FF20 {
            return Ok(Some(Self::Bxj_A1(cond, gpr(word & 0b1111))));
        }
        // MOVW : cccc 0011 0000 imm4 Rd imm12      MOVT : cccc 0011 0100 imm4 Rd imm12
        if word & 0x0FF0_0000 == 0x0300_0000 {
            return Ok(Some(Self::Movw_A2(cond, gpr((word >> 12) & 0b1111), decode_movw_imm16(word))));
        }
        if word & 0x0FF0_0000 == 0x0340_0000 {
            return Ok(Some(Self::Movt_A1(cond, gpr((word >> 12) & 0b1111), decode_movw_imm16(word))));
        }
        // SVC : cccc 1111 imm24
        if word & 0x0F00_0000 == 0x0F00_0000 {
            return Ok(Some(Self::Svc_A1(cond, word & 0x00FF_FFFF)));
        }

        // saturating add/sub : cccc 00010 op 0 Rn Rd 0000 0101 Rm   (must precede the generic DP blocks: the
        // op bits alias the AND/SUB/ADD/SBC opcodes and bit4=1,bit7=0 would otherwise match register-shifted)
        if word & 0x0F90_00F0 == 0x0100_0050 {
            let rn = gpr((word >> 16) & 0b1111);
            let rd = gpr((word >> 12) & 0b1111);
            let rm = gpr(word & 0b1111);
            return Ok(Some(match (word >> 21) & 0b11 {
                0b00 => Self::Qadd_A1(cond, rd, rm, rn),
                0b01 => Self::Qsub_A1(cond, rd, rm, rn),
                0b10 => Self::Qdadd_A1(cond, rd, rm, rn),
                _ => Self::Qdsub_A1(cond, rd, rm, rn),
            }));
        }
        // signed multiply (halfword, type 1) : cccc 00010 op 0 .. 1 M N 0 ..   ([27:23]=00010, [7]=1, [4]=0)
        if word & 0x0F90_0090 == 0x0100_0080 {
            let op = (word >> 21) & 0b11;
            let high = gpr((word >> 16) & 0b1111);
            let low = gpr((word >> 12) & 0b1111);
            let rm = gpr((word >> 8) & 0b1111);
            let rn = gpr(word & 0b1111);
            let m = (word >> 6) & 1 == 1;
            let n = (word >> 5) & 1 == 1;
            return Ok(Some(match op {
                0b00 => Self::Smla_A1(cond, high, rn, rm, low, n, m),
                0b01 => if (word >> 5) & 1 == 0 { Self::Smlaw_A1(cond, high, rn, rm, low, m) } else { Self::Smulw_A1(cond, high, rn, rm, m) },
                0b10 => Self::Smlal_Halfword_A1(cond, low, high, rn, rm, n, m),
                _ => Self::Smul_A1(cond, high, rn, rm, n, m),
            }));
        }
        // signed multiply (dual / most-significant word, type 2) : cccc 0111 0xx .. .. .. R/x 1 ..   ([27:24]=0111, bit23=0, [4]=1)
        if word & 0x0F80_0010 == 0x0700_0010 {
            let op1 = (word >> 20) & 0b111;
            let high = gpr((word >> 16) & 0b1111);
            let accum_bits = (word >> 12) & 0b1111;
            let accum = gpr(accum_bits);
            let rm = gpr((word >> 8) & 0b1111);
            let rn = gpr(word & 0b1111);
            let bit5 = (word >> 5) & 1 == 1; // x / round
            let bit6 = (word >> 6) & 1;
            let bit7 = (word >> 7) & 1;
            match op1 {
                0b000 if bit7 == 0 => {
                    return Ok(Some(if bit6 == 0 {
                        if accum_bits == 0b1111 { Self::Smuad_A1(cond, high, rn, rm, bit5) } else { Self::Smlad_A1(cond, high, rn, rm, accum, bit5) }
                    } else {
                        if accum_bits == 0b1111 { Self::Smusd_A1(cond, high, rn, rm, bit5) } else { Self::Smlsd_A1(cond, high, rn, rm, accum, bit5) }
                    }));
                },
                0b100 if bit7 == 0 => {
                    return Ok(Some(if bit6 == 0 { Self::Smlald_A1(cond, accum, high, rn, rm, bit5) } else { Self::Smlsld_A1(cond, accum, high, rn, rm, bit5) }));
                },
                0b101 => {
                    if bit7 == 0 && bit6 == 0 {
                        return Ok(Some(if accum_bits == 0b1111 { Self::Smmul_A1(cond, high, rn, rm, bit5) } else { Self::Smmla_A1(cond, high, rn, rm, accum, bit5) }));
                    } else if bit7 == 1 && bit6 == 1 {
                        return Ok(Some(Self::Smmls_A1(cond, high, rn, rm, accum, bit5)));
                    }
                },
                _ => {}
            }
        }
        // SEL : cccc 01101000 Rn Rd 1111 1011 Rm
        if word & 0x0FF0_0FF0 == 0x0680_0FB0 {
            return Ok(Some(Self::Sel_A1(cond, gpr((word >> 12) & 0b1111), gpr((word >> 16) & 0b1111), gpr(word & 0b1111))));
        }
        // parallel (packed SIMD) add/sub : cccc 01100 prefix Rn Rd 1111 op 1 Rm   ([27:23]=01100, [11:8]=1111, [4]=1)
        if word & 0x0F80_0F10 == 0x0600_0F10
            && let (Some(prefix), Some(op)) = (a32_parallel_prefix_from_bits((word >> 20) & 0b111), a32_parallel_op_from_bits((word >> 4) & 0b1111)) {
                return Ok(Some(Self::ParallelAddSub_A1(cond, op, prefix, gpr((word >> 12) & 0b1111), gpr((word >> 16) & 0b1111), gpr(word & 0b1111))));
            }
        // CLZ : cccc 00010110 1111 Rd 1111 0001 Rm
        if word & 0x0FFF_0FF0 == 0x016F_0F10 {
            return Ok(Some(Self::Clz_A1(cond, gpr((word >> 12) & 0b1111), gpr(word & 0b1111))));
        }
        // REV / REV16 / RBIT / REVSH : cccc 0110 10/11 11 1111 Rd 1111 op Rm
        if word & 0x0FFF_0FF0 == 0x06BF_0F30 { return Ok(Some(Self::Rev_A1(cond, gpr((word >> 12) & 0b1111), gpr(word & 0b1111)))); }
        if word & 0x0FFF_0FF0 == 0x06BF_0FB0 { return Ok(Some(Self::Rev16_A1(cond, gpr((word >> 12) & 0b1111), gpr(word & 0b1111)))); }
        if word & 0x0FFF_0FF0 == 0x06FF_0F30 { return Ok(Some(Self::Rbit_A1(cond, gpr((word >> 12) & 0b1111), gpr(word & 0b1111)))); }
        if word & 0x0FFF_0FF0 == 0x06FF_0FB0 { return Ok(Some(Self::Revsh_A1(cond, gpr((word >> 12) & 0b1111), gpr(word & 0b1111)))); }
        // extend / extend-and-add : cccc 0110 1xxx Rn Rd rotate 00 0111 Rm  (Rn=1111 => plain extend)
        if word & 0x0F00_03F0 == 0x0600_0070
            && let Some(extend_type) = Arm32ExtendType::from_opcode_byte((word >> 20) & 0xFF) {
                let rotation = decode_rotation((word >> 10) & 0b11);
                let rd = gpr((word >> 12) & 0b1111);
                let rm = gpr(word & 0b1111);
                let rn_bits = (word >> 16) & 0b1111;
                return Ok(Some(if rn_bits == 0b1111 {
                    Self::Extend_A1(cond, extend_type, rd, rm, rotation)
                } else {
                    Self::ExtendAndAdd_A1(cond, extend_type, rd, gpr(rn_bits), rm, rotation)
                }));
            }
        // PKHBT / PKHTB : cccc 01101000 Rn Rd imm5 tb 01 Rm  (only [5:4]=01 is fixed in the low nibble;
        // bit7 is the LSB of imm5 and bit6 is tb, so they must NOT be masked)
        if word & 0x0FF0_0030 == 0x0680_0010 {
            let imm5 = ((word >> 7) & 0b1_1111) as u8;
            let rn = gpr((word >> 16) & 0b1111);
            let rd = gpr((word >> 12) & 0b1111);
            let rm = gpr(word & 0b1111);
            return Ok(Some(if (word >> 6) & 1 == 0 {
                Self::Pkhbt_A1(cond, rd, rn, rm, imm5)
            } else {
                Self::Pkhtb_A1(cond, rd, rn, rm, if imm5 == 0 { 32 } else { imm5 })
            }));
        }
        // SSAT16 / USAT16 : cccc 0110 1010/1110 sat_imm Rd 1111 0011 Rm
        if word & 0x0FB0_0FF0 == 0x06A0_0F30 {
            let is_usat = (word >> 22) & 1 == 1;
            let sat_imm = ((word >> 16) & 0b1111) as u8;
            let rd = gpr((word >> 12) & 0b1111);
            let rm = gpr(word & 0b1111);
            return Ok(Some(if is_usat { Self::Usat16_A1(cond, rd, sat_imm, rm) } else { Self::Ssat16_A1(cond, rd, sat_imm + 1, rm) }));
        }
        // SSAT / USAT : cccc 0110 101/111 sat_imm Rd imm5 sh 01 Rm
        if word & 0x0FA0_0030 == 0x06A0_0010 {
            let is_usat = (word >> 22) & 1 == 1;
            let sat_imm = ((word >> 16) & 0b1_1111) as u8;
            let rd = gpr((word >> 12) & 0b1111);
            let imm5 = ((word >> 7) & 0b1_1111) as u8;
            let shift = if (word >> 6) & 1 == 1 { Arm32RegisterShift::Asr(if imm5 == 0 { 32 } else { imm5 }) } else { Arm32RegisterShift::Lsl(imm5) };
            let rm = gpr(word & 0b1111);
            return Ok(Some(if is_usat { Self::Usat_A1(cond, rd, sat_imm, rm, shift) } else { Self::Ssat_A1(cond, rd, sat_imm + 1, rm, shift) }));
        }
        // USAD8 / USADA8 : cccc 01111000 Rd Ra Rm 0001 Rn  (Ra=1111 => USAD8)
        if word & 0x0FF0_00F0 == 0x0780_0010 {
            let rd = gpr((word >> 16) & 0b1111);
            let ra_bits = (word >> 12) & 0b1111;
            let rm = gpr((word >> 8) & 0b1111);
            let rn = gpr(word & 0b1111);
            return Ok(Some(if ra_bits == 0b1111 { Self::Usad8_A1(cond, rd, rn, rm) } else { Self::Usada8_A1(cond, rd, rn, rm, gpr(ra_bits)) }));
        }
        // BFC / BFI : cccc 0111110 msb Rd lsb 001 Rn  (Rn=1111 => BFC)
        if word & 0x0FE0_0070 == 0x07C0_0010 {
            let msb = ((word >> 16) & 0b1_1111) as u8;
            let lsb = ((word >> 7) & 0b1_1111) as u8;
            let rd = gpr((word >> 12) & 0b1111);
            let width = msb.saturating_sub(lsb) + 1;
            let rn_bits = word & 0b1111;
            return Ok(Some(if rn_bits == 0b1111 { Self::Bfc_A1(cond, rd, lsb, width) } else { Self::Bfi_A1(cond, rd, gpr(rn_bits), lsb, width) }));
        }
        // SBFX : cccc 0111101 widthm1 Rd lsb 101 Rn
        if word & 0x0FE0_0070 == 0x07A0_0050 {
            let widthm1 = ((word >> 16) & 0b1_1111) as u8;
            let lsb = ((word >> 7) & 0b1_1111) as u8;
            return Ok(Some(Self::Sbfx_A1(cond, gpr((word >> 12) & 0b1111), gpr(word & 0b1111), lsb, widthm1 + 1)));
        }
        // UBFX : cccc 0111111 widthm1 Rd lsb 101 Rn
        if word & 0x0FE0_0070 == 0x07E0_0050 {
            let widthm1 = ((word >> 16) & 0b1_1111) as u8;
            let lsb = ((word >> 7) & 0b1_1111) as u8;
            return Ok(Some(Self::Ubfx_A1(cond, gpr((word >> 12) & 0b1111), gpr(word & 0b1111), lsb, widthm1 + 1)));
        }
        // MRS (banked) : cccc 00010 R 00 m1 Rd 001 m 0 0000  ([11:9]=001 distinguishes it from the plain MRS below)
        if word & 0x0FB0_0EFF == 0x0100_0200 {
            let sysm = (((word >> 8) & 1) << 4) | ((word >> 16) & 0xF);
            return Ok(Some(Self::MrsBanked_A1(cond, (word >> 22) & 1 == 1, sysm as u8, gpr((word >> 12) & 0b1111))));
        }
        // MSR (banked) : cccc 00010 R 10 m1 1111 001 m 0000 Rn
        if word & 0x0FB0_FEF0 == 0x0120_F200 {
            let sysm = (((word >> 8) & 1) << 4) | ((word >> 16) & 0xF);
            return Ok(Some(Self::MsrBanked_A1(cond, (word >> 22) & 1 == 1, sysm as u8, gpr(word & 0b1111))));
        }
        // MRS : cccc 00010 R 00 1111 Rd 0000 0000 0000
        if word & 0x0FBF_0FFF == 0x010F_0000 {
            return Ok(Some(Self::Mrs_A1(cond, (word >> 22) & 1 == 1, gpr((word >> 12) & 0b1111))));
        }
        // MSR (register) : cccc 00010 R 10 mask 1111 0000 0000 Rn
        if word & 0x0FB0_FFF0 == 0x0120_F000 {
            return Ok(Some(Self::Msr_Register_A1(cond, (word >> 22) & 1 == 1, ((word >> 16) & 0xF) as u8, gpr(word & 0b1111))));
        }
        // MSR (immediate) : cccc 00110 R 10 mask 1111 imm12  (mask != 0000; mask==0 is the hints space)
        if word & 0x0FB0_F000 == 0x0320_F000 && (word >> 16) & 0xF != 0 {
            let value = decode_a32_modified_immediate((word & 0x0FFF) as u16);
            return Ok(Some(Self::Msr_Immediate_A1(cond, (word >> 22) & 1 == 1, ((word >> 16) & 0xF) as u8, value)));
        }
        // BKPT / HVC : cccc 0001 0010/0100 imm12 0111 imm4
        if word & 0x0FF0_00F0 == 0x0120_0070 {
            return Ok(Some(Self::Bkpt_A1(cond, imm16_join(word))));
        }
        if word & 0x0FF0_00F0 == 0x0140_0070 {
            return Ok(Some(Self::Hvc_A1(cond, imm16_join(word))));
        }
        // ERET : cccc 0001 0110 0000 0000 0000 0110 1110  (precedes SMC: both [27:20]=00010110)
        if word & 0x0FFF_FFFF == 0x0160_006E {
            return Ok(Some(Self::Eret_A1(cond)));
        }
        // SMC : cccc 0001 0110 0000 0000 0000 0111 imm4
        if word & 0x0FF0_00F0 == 0x0160_0070 {
            return Ok(Some(Self::Smc_A1(cond, (word & 0xF) as u8)));
        }
        // UDF : cccc 0111 1111 imm12 1111 imm4
        if word & 0x0FF0_00F0 == 0x07F0_00F0 {
            return Ok(Some(Self::Udf_A1(cond, imm16_join(word))));
        }
        // CRC32 (ARMv8-A) : cccc 00010 sz 0 Rn Rd 0000 0 C 00 0100 Rm
        if word & 0x0F90_0DF0 == 0x0100_0040 {
            let rd = gpr((word >> 12) & 0b1111);
            let rn = gpr((word >> 16) & 0b1111);
            let rm = gpr(word & 0b1111);
            return Ok(Some(match ((word >> 21) & 0b11, (word >> 9) & 1 == 1) {
                (0b00, false) => Self::Crc32b_A1(cond, rd, rn, rm),
                (0b01, false) => Self::Crc32h_A1(cond, rd, rn, rm),
                (0b10, false) => Self::Crc32w_A1(cond, rd, rn, rm),
                (0b00, true) => Self::Crc32cb_A1(cond, rd, rn, rm),
                (0b01, true) => Self::Crc32ch_A1(cond, rd, rn, rm),
                (0b10, true) => Self::Crc32cw_A1(cond, rd, rn, rm),
                _ => return Err(DecodeError::InvalidOpcode), // sz=11 is unallocated
            }));
        }
        // load-acquire / store-release (ARMv8-A), non-exclusive : cccc 00011 sz L Rn (Rt) 1100 1001 (1111|Rt)
        if word & 0x0F80_0FF0 == 0x0180_0C90 {
            let l = (word >> 20) & 1 == 1;
            let rn = gpr((word >> 16) & 0b1111);
            return Ok(Some(match ((word >> 21) & 0b11, l) {
                (0b00, true) => Self::Lda_A1(cond, gpr((word >> 12) & 0b1111), rn),
                (0b10, true) => Self::Ldab_A1(cond, gpr((word >> 12) & 0b1111), rn),
                (0b11, true) => Self::Ldah_A1(cond, gpr((word >> 12) & 0b1111), rn),
                (0b00, false) => Self::Stl_A1(cond, gpr(word & 0b1111), rn),
                (0b10, false) => Self::Stlb_A1(cond, gpr(word & 0b1111), rn),
                (0b11, false) => Self::Stlh_A1(cond, gpr(word & 0b1111), rn),
                _ => return Err(DecodeError::InvalidOpcode),
            }));
        }
        // load-acquire-exclusive / store-release-exclusive (ARMv8-A) : cccc 00011 sz L Rn (Rt|Rd) 1110 1001 (1111|Rt)
        if word & 0x0F80_0FF0 == 0x0180_0E90 {
            let l = (word >> 20) & 1 == 1;
            let rn = gpr((word >> 16) & 0b1111);
            let high = gpr((word >> 12) & 0b1111);
            let low = gpr(word & 0b1111);
            return Ok(Some(match ((word >> 21) & 0b11, l) {
                (0b00, true) => Self::Ldaex_A1(cond, high, rn),
                (0b01, true) => Self::Ldaexd_A1(cond, high, rn),
                (0b10, true) => Self::Ldaexb_A1(cond, high, rn),
                (0b11, true) => Self::Ldaexh_A1(cond, high, rn),
                (0b00, false) => Self::Stlex_A1(cond, high, low, rn),
                (0b01, false) => Self::Stlexd_A1(cond, high, low, rn),
                (0b10, false) => Self::Stlexb_A1(cond, high, low, rn),
                (0b11, false) => Self::Stlexh_A1(cond, high, low, rn),
                _ => return Err(DecodeError::InvalidOpcode),
            }));
        }
        // exclusive access : cccc 00011 type L Rn (Rt|Rd) 1111 1001 (1111|Rt)
        if word & 0x0F80_0FF0 == 0x0180_0F90 {
            let is_load = (word >> 20) & 1 == 1;
            let rn = gpr((word >> 16) & 0b1111);
            let high = gpr((word >> 12) & 0b1111); // Rt (load) or Rd (store)
            let rt_store = gpr(word & 0b1111);
            return Ok(Some(match ((word >> 21) & 0b11, is_load) {
                (0b00, true) => Self::Ldrex_A1(cond, high, rn),
                (0b00, false) => Self::Strex_A1(cond, high, rt_store, rn),
                (0b01, true) => Self::Ldrexd_A1(cond, high, rn),
                (0b01, false) => Self::Strexd_A1(cond, high, rt_store, rn),
                (0b10, true) => Self::Ldrexb_A1(cond, high, rn),
                (0b10, false) => Self::Strexb_A1(cond, high, rt_store, rn),
                (_, true) => Self::Ldrexh_A1(cond, high, rn),
                (_, false) => Self::Strexh_A1(cond, high, rt_store, rn),
            }));
        }
        // SWP / SWPB (deprecated) : cccc 00010 B 00 Rn Rt 0000 1001 Rt2
        if word & 0x0FB0_0FF0 == 0x0100_0090 {
            let rn = gpr((word >> 16) & 0b1111);
            let rt = gpr((word >> 12) & 0b1111);
            let rt2 = gpr(word & 0b1111);
            return Ok(Some(if (word >> 22) & 1 == 1 { Self::Swpb_A1(cond, rt, rt2, rn) } else { Self::Swp_A1(cond, rt, rt2, rn) }));
        }
        // multiply / long multiply : cccc 0000 op 1001 ....   ([27:24]=0000, [7:4]=1001)
        if word & 0x0F00_00F0 == 0x0000_0090 {
            let op3 = (word >> 21) & 0b111;
            let set_flags = (word >> 20) & 1 == 1;
            let high = gpr((word >> 16) & 0b1111); // Rd (MUL/MLA/MLS) or RdHi (long)
            let low = gpr((word >> 12) & 0b1111);  // Ra (MUL/MLA/MLS) or RdLo (long)
            let rm = gpr((word >> 8) & 0b1111);
            let rn = gpr(word & 0b1111);
            match op3 {
                0b000 => return Ok(Some(Self::Mul_A1(cond, set_flags, high, rn, rm))),
                0b001 => return Ok(Some(Self::Mla_A1(cond, set_flags, high, rn, rm, low))),
                0b010 if !set_flags => return Ok(Some(Self::Umaal_A1(cond, low, high, rn, rm))),
                0b011 if !set_flags => return Ok(Some(Self::Mls_A1(cond, high, rn, rm, low))),
                0b100 => return Ok(Some(Self::Umull_A1(cond, set_flags, low, high, rn, rm))),
                0b101 => return Ok(Some(Self::Umlal_A1(cond, set_flags, low, high, rn, rm))),
                0b110 => return Ok(Some(Self::Smull_A1(cond, set_flags, low, high, rn, rm))),
                0b111 => return Ok(Some(Self::Smlal_A1(cond, set_flags, low, high, rn, rm))),
                _ => {} // op3 010/011 with S=1 is unallocated -- fall through
            }
        }
        // extra load/store (halfword / signed byte / signed halfword / dual) : cccc 000 P U I W L Rn Rt H4 1 S H 1 L4
        // ([27:25]=000, [7]=1, [4]=1; (S,H)=(0,0) is the multiply / sync space and is excluded)
        if word & 0x0E00_0090 == 0x0000_0090 {
            let s = (word >> 6) & 1;
            let h = (word >> 5) & 1;
            if !(s == 0 && h == 0) {
                let p = (word >> 24) & 1;
                let add = (word >> 23) & 1 == 1;
                let i_bit = (word >> 22) & 1;
                let w = (word >> 21) & 1;
                let is_load = (word >> 20) & 1 == 1;
                let rn = gpr((word >> 16) & 0b1111);
                let rt = gpr((word >> 12) & 0b1111);
                let offset = if i_bit == 1 {
                    let imm8 = ((((word >> 8) & 0b1111) << 4) | (word & 0b1111)) as u8;
                    Arm32MemoryOffset8::Immediate { add, imm8 }
                } else {
                    Arm32MemoryOffset8::Register { add, rm: gpr(word & 0b1111) }
                };
                let index = index_from_p_w(p, w);
                let is_t = p == 0 && w == 1;
                return Ok(Some(match (s, h, is_load) {
                    (0, 1, true)  => if is_t { Self::Ldrht_A1(cond, rt, rn, offset) } else { Self::Ldrh_A1(cond, rt, rn, offset, index) },
                    (0, 1, false) => if is_t { Self::Strht_A1(cond, rt, rn, offset) } else { Self::Strh_A1(cond, rt, rn, offset, index) },
                    (1, 0, true)  => if is_t { Self::Ldrsbt_A1(cond, rt, rn, offset) } else { Self::Ldrsb_A1(cond, rt, rn, offset, index) },
                    (1, 1, true)  => if is_t { Self::Ldrsht_A1(cond, rt, rn, offset) } else { Self::Ldrsh_A1(cond, rt, rn, offset, index) },
                    (1, 0, false) => Self::Ldrd_A1(cond, rt, rn, offset, index), // LDRD (no T form)
                    _             => Self::Strd_A1(cond, rt, rn, offset, index), // (1,1,false) STRD
                }));
            }
        }
        // data processing (register, immediate shift) : cccc 000 opcode S Rn Rd imm5 type 0 Rm   (bit25=0, bit4=0)
        if word & 0x0E00_0010 == 0x0000_0000 {
            let opcode = (word >> 21) & 0b1111;
            let set_flags = (word >> 20) & 1 == 1;
            let rn = (word >> 16) & 0b1111;
            let rd = (word >> 12) & 0b1111;
            let rm = word & 0b1111;
            let shift = decode_a32_shift(((word >> 7) & 0b1_1111) as u8, ((word >> 5) & 0b11) as u8);
            if let Some(instruction) = decode_dp_register(cond, opcode, set_flags, rn, rd, rm, shift) {
                return Ok(Some(instruction));
            }
        }
        // data processing (register-shifted register) : cccc 000 opcode S Rn Rd Rs 0 type 1 Rm  (bit25=0, bit7=0, bit4=1)
        if word & 0x0E00_0090 == 0x0000_0010 {
            let opcode = (word >> 21) & 0b1111;
            let set_flags = (word >> 20) & 1 == 1;
            let rn = (word >> 16) & 0b1111;
            let rd = (word >> 12) & 0b1111;
            let rs = (word >> 8) & 0b1111;
            let rm = word & 0b1111;
            let shift_type = Arm32ShiftType::from_type_bits((word >> 5) & 0b11);
            if let Some(instruction) = decode_dp_register_shifted(cond, opcode, set_flags, rn, rd, rm, shift_type, rs) {
                return Ok(Some(instruction));
            }
        }
        // data processing (immediate) : cccc 001 opcode S Rn Rd imm12   (bit25=1)
        if word & 0x0E00_0000 == 0x0200_0000 {
            let opcode = (word >> 21) & 0b1111;
            let set_flags = (word >> 20) & 1 == 1;
            let rn = (word >> 16) & 0b1111;
            let rd = (word >> 12) & 0b1111;
            let value = decode_a32_modified_immediate((word & 0x0FFF) as u16);
            if let Some(instruction) = decode_dp_immediate(cond, opcode, set_flags, rn, rd, value) {
                return Ok(Some(instruction));
            }
        }
        // load/store single (word/byte) : cccc 01 I P U B W L Rn Rt <offset>
        if word & 0x0C00_0000 == 0x0400_0000 {
            let is_register = (word >> 25) & 1 == 1;
            // a register offset with bit4=1 is the media-instruction space (handled earlier), not a load/store
            if !(is_register && (word >> 4) & 1 == 1) {
                let p = (word >> 24) & 1;
                let add = (word >> 23) & 1 == 1;
                let is_byte = (word >> 22) & 1 == 1;
                let w = (word >> 21) & 1;
                let is_load = (word >> 20) & 1 == 1;
                let rn = gpr((word >> 16) & 0b1111);
                let rt = gpr((word >> 12) & 0b1111);
                let offset = if is_register {
                    let shift = decode_a32_shift(((word >> 7) & 0b1_1111) as u8, ((word >> 5) & 0b11) as u8);
                    Arm32MemoryOffset::Register { add, rm: gpr(word & 0b1111), shift }
                } else {
                    Arm32MemoryOffset::Immediate { add, imm12: (word & 0x0FFF) as u16 }
                };
                if p == 0 && w == 1 {
                    // unprivileged "T" forms
                    return Ok(Some(match (is_load, is_byte) {
                        (true, false) => Self::Ldrt_A1(cond, rt, rn, offset),
                        (false, false) => Self::Strt_A1(cond, rt, rn, offset),
                        (true, true) => Self::Ldrbt_A1(cond, rt, rn, offset),
                        (false, true) => Self::Strbt_A1(cond, rt, rn, offset),
                    }));
                }
                let index = match (p, w) {
                    (1, 0) => Arm32IndexMode::Offset,
                    (1, 1) => Arm32IndexMode::PreIndex,
                    _ => Arm32IndexMode::PostIndex, // (0, 0)
                };
                return Ok(Some(match (is_load, is_byte) {
                    (true, false) => Self::Ldr_A1(cond, rt, rn, offset, index),
                    (false, false) => Self::Str_A1(cond, rt, rn, offset, index),
                    (true, true) => Self::Ldrb_A1(cond, rt, rn, offset, index),
                    (false, true) => Self::Strb_A1(cond, rt, rn, offset, index),
                }));
            }
        }
        // load/store multiple : cccc 100 P U S W L Rn register_list  (cond=1111 in this space is RFE/SRS, handled above)
        if word & 0x0E00_0000 == 0x0800_0000 && (word >> 28) != 0xF {
            let p = (word >> 24) & 1;
            let u = (word >> 23) & 1;
            let user_mode = (word >> 22) & 1 == 1;
            let writeback = (word >> 21) & 1 == 1;
            let is_load = (word >> 20) & 1 == 1;
            let rn = gpr((word >> 16) & 0b1111);
            let registers = decode_register_list((word & 0xFFFF) as u16);
            let mode = Arm32BlockAddressMode::from_p_u_bits(p, u);
            return Ok(Some(if is_load {
                Self::Ldm_A1(cond, mode, rn, writeback, user_mode, registers)
            } else {
                Self::Stm_A1(cond, mode, rn, writeback, user_mode, registers)
            }));
        }
        // BLX (immediate, unconditional) : 1111 101 H imm24  (precedes B/BL -- it shares [27:25]=101 with cond=1111)
        if word & 0xFE00_0000 == 0xFA00_0000 {
            let h = ((word >> 24) & 1) as i32;
            let offset = (sign_extend_24(word & 0x00FF_FFFF) << 2) | (h << 1);
            return Ok(Some(Self::Blx_Immediate_A1(offset)));
        }
        // B / BL : cccc 1010/1011 imm24
        if word & 0x0F00_0000 == 0x0A00_0000 {
            return Ok(Some(Self::B_A1(cond, sign_extend_24(word & 0x00FF_FFFF) << 2)));
        }
        if word & 0x0F00_0000 == 0x0B00_0000 {
            return Ok(Some(Self::Bl_A1(cond, sign_extend_24(word & 0x00FF_FFFF) << 2)));
        }
        // VFP data-processing : cccc 1110 ... 101 sz ... 0 ...  (3-operand + the 2-operand "other" group)
        if word & 0x0F00_0E10 == 0x0E00_0A00 {
            let double = (word >> 8) & 1 == 1;
            let top = (word >> 23) & 1;
            let middle = (word >> 20) & 0b11;
            let vd_field = (word >> 12) & 0b1111;
            let d_bit = (word >> 22) & 1;
            let vm_field = word & 0b1111;
            let m_bit = (word >> 5) & 1;
            if top == 1 && middle == 0b11 {
                let opc2 = (word >> 16) & 0b1111;
                let op7 = (word >> 7) & 1;
                // the 2-operand "other" group. bit6=0 is VMOV (immediate); bit6=1 selects by opc2.
                if (word >> 6) & 1 == 0 {
                    let imm8 = ((((word >> 16) & 0xF) << 4) | (word & 0xF)) as u8;
                    return Ok(Some(if double {
                        Self::Vmov_Immediate_Double_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), imm8)
                    } else {
                        Self::Vmov_Immediate_Single_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), imm8)
                    }));
                }
                if let Some(op) = Arm32FpDataOperation2::from_bits(opc2, op7) {
                    return Ok(Some(if double {
                        Self::FpDataProcess2_Double_A1(cond, op, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    } else {
                        Self::FpDataProcess2_Single_A1(cond, op, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    }));
                }
                if opc2 == 0b0100 {
                    return Ok(Some(if double {
                        Self::Vcmp_Double_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                    } else {
                        Self::Vcmp_Single_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                    }));
                }
                if opc2 == 0b0101 {
                    return Ok(Some(if double {
                        Self::Vcmp_Zero_Double_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), op7 == 1)
                    } else {
                        Self::Vcmp_Zero_Single_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), op7 == 1)
                    }));
                }
                // VCVT half-precision: opc2 0010 = f16->wide, 0011 = wide->f16 (op7 = which half-word). The
                // wide operand is single when `double`=0 and double (FEAT_FP16) when `double`=1.
                if opc2 == 0b0010 {
                    return Ok(Some(if double {
                        Self::Vcvt_HalfToDouble_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                    } else {
                        Self::Vcvt_HalfToSingle_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                    }));
                }
                if opc2 == 0b0011 {
                    return Ok(Some(if double {
                        Self::Vcvt_DoubleToHalf_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                    } else {
                        Self::Vcvt_SingleToHalf_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                    }));
                }
                // VJCVT (VJCVTZS): opc2=1001, op7=1 -- JS double -> s32. Sd (result) at Vd:D, Dm (source) at Vm:M.
                if opc2 == 0b1001 && op7 == 1 {
                    return Ok(Some(Self::Vjcvt_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))));
                }
                // VCVT fixed-point: opc2 = 1:op:1:U (1010/1011/1110/1111); op[18] = to_fixed, op[16] = unsigned.
                if opc2 & 0b1010 == 0b1010 && let Some((signed, bits32, frac)) = decode_a32_vcvt_fixed(word) {
                    let to_fixed = (opc2 >> 2) & 1 == 1;
                    return Ok(Some(match (to_fixed, double) {
                        (true, false) => Self::Vcvt_FloatToFixed_Single_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), signed, bits32, frac),
                        (true, true) => Self::Vcvt_FloatToFixed_Double_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), signed, bits32, frac),
                        (false, false) => Self::Vcvt_FixedToFloat_Single_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), signed, bits32, frac),
                        (false, true) => Self::Vcvt_FixedToFloat_Double_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), signed, bits32, frac),
                    }));
                }
                // Conditional VRINT{R,Z,X} (ARMv8): opc2=0110 is VRINTR (op7=0) / VRINTZ (op7=1); opc2=0111
                // with op7=0 is VRINTX. opc2=0111 with op7=1 is the VCVT f32<->f64 below, so guard on op7.
                if opc2 == 0b0110 {
                    let mode = if op7 == 0 { Arm32VrintMode::R } else { Arm32VrintMode::Z };
                    return Ok(Some(if double {
                        Self::Vrint_Cond_Double_A1(cond, mode, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    } else {
                        Self::Vrint_Cond_Single_A1(cond, mode, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    }));
                }
                if opc2 == 0b0111 && op7 == 0 {
                    return Ok(Some(if double {
                        Self::Vrint_Cond_Double_A1(cond, Arm32VrintMode::X, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    } else {
                        Self::Vrint_Cond_Single_A1(cond, Arm32VrintMode::X, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    }));
                }
                // VCVT: 0111 = f32<->f64 (op7=1), 1000 = int->float, 110x = float->int. `double` (bit8) is the
                // SOURCE precision for float<->int and float-narrow, the DEST precision for int->float.
                match opc2 {
                    0b0111 => return Ok(Some(if double {
                        Self::Vcvt_Double_To_Single_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    } else {
                        Self::Vcvt_Single_To_Double_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    })),
                    0b1000 => return Ok(Some(if double {
                        Self::Vcvt_IntToFloat_ToDouble_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                    } else {
                        Self::Vcvt_IntToFloat_ToSingle_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                    })),
                    0b1100 | 0b1101 => return Ok(Some(if double {
                        Self::Vcvt_FloatToInt_FromDouble_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit), opc2 & 1 == 1, op7 == 1)
                    } else {
                        Self::Vcvt_FloatToInt_FromSingle_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), opc2 & 1 == 1, op7 == 1)
                    })),
                    _ => {},
                }
            } else if let Some(op) = Arm32FpDataOperation3::from_bits(top, middle, (word >> 6) & 1) {
                let vn_field = (word >> 16) & 0b1111;
                let n_bit = (word >> 7) & 1;
                return Ok(Some(if double {
                    Self::FpDataProcess3_Double_A1(cond, op, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                } else {
                    Self::FpDataProcess3_Single_A1(cond, op, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                }));
            }
        }
        // VMRS / VMSR (FPSCR transfer); VMRS with Rt=1111 is the APSR_nzcv (vcmp-flags) form
        if word & 0x0FFF_0FFF == 0x0EF1_0A10 {
            let rt_bits = (word >> 12) & 0b1111;
            return Ok(Some(if rt_bits == 0b1111 { Self::Vmrs_Apsr_Nzcv_A1(cond) } else { Self::Vmrs_A1(cond, gpr(rt_bits)) }));
        }
        if word & 0x0FFF_0FFF == 0x0EE1_0A10 {
            return Ok(Some(Self::Vmsr_A1(cond, gpr((word >> 12) & 0b1111))));
        }
        // VMOV core <-> single (op[20]: 0 = Rt->Sn, 1 = Sn->Rt)
        if word & 0x0FF0_0F7F == 0x0E00_0A10 {
            let sn = Arm32SinglePrecisionRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1);
            return Ok(Some(Self::Vmov_Core_To_Single_A1(cond, sn, gpr((word >> 12) & 0b1111))));
        }
        if word & 0x0FF0_0F7F == 0x0E10_0A10 {
            let sn = Arm32SinglePrecisionRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1);
            return Ok(Some(Self::Vmov_Single_To_Core_A1(cond, gpr((word >> 12) & 0b1111), sn)));
        }
        // VMOV (general-purpose register to scalar): Rt -> Dd[x]. [11:8]=1011 distinguishes it from VMOV-single
        // ([11:8]=1010); [20]=0 to-scalar, [23]=0; size+lane pack into opc1[22:21]/opc2[6:5].
        if word & 0x0F90_0F1F == 0x0E00_0B10 {
            let (size, index) = Arm32VmovLaneSize::from_opc_fields((word >> 21) & 0b11, (word >> 5) & 0b11);
            let dd = Arm32DoublePrecisionRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1);
            return Ok(Some(Self::Vmov_Core_To_Scalar_A1(cond, size, index, dd, gpr((word >> 12) & 0b1111))));
        }
        // VMOV (scalar to general-purpose register): Dn[x] -> Rt. [20]=1; U=[23] (sign-extend for .8/.16).
        if word & 0x0F10_0F1F == 0x0E10_0B10 {
            let (size, index) = Arm32VmovLaneSize::from_opc_fields((word >> 21) & 0b11, (word >> 5) & 0b11);
            let dn = Arm32DoublePrecisionRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1);
            // .32 carries no sign bit (U is 0); only .8/.16 use U[23] as signed/unsigned.
            let unsigned = !matches!(size, Arm32VmovLaneSize::Word) && (word >> 23) & 1 == 1;
            return Ok(Some(Self::Vmov_Scalar_To_Core_A1(cond, unsigned, size, index, gpr((word >> 12) & 0b1111), dn)));
        }
        // VMOV core-pair <-> double / two-singles : cccc 1100 010 op Rt2 Rt 101 sz M 1 Vm
        if word & 0x0FE0_0ED0 == 0x0C40_0A10 {
            let fp_to_core = (word >> 20) & 1 == 1;
            let is_double = (word >> 8) & 1 == 1;
            let rt2 = gpr((word >> 16) & 0b1111);
            let rt = gpr((word >> 12) & 0b1111);
            let vm_field = word & 0b1111;
            let m_bit = (word >> 5) & 1;
            return Ok(Some(if is_double {
                let dm = Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit);
                if fp_to_core { Self::Vmov_Double_To_CorePair_A1(cond, rt, rt2, dm) } else { Self::Vmov_CorePair_To_Double_A1(cond, dm, rt, rt2) }
            } else {
                let sm = Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit);
                if fp_to_core { Self::Vmov_Singles_To_CorePair_A1(cond, rt, rt2, sm) } else { Self::Vmov_CorePair_To_Singles_A1(cond, sm, rt, rt2) }
            }));
        }
        // VFP scalar load/store : cccc 110 P U D W L Rn Vd 101 sz imm8  (coproc = 1010 single / 1011 double).
        // VLDR/VSTR are conditional-only; a cond=0b1111 word in this space is not VLDR (it is the LDC2/STC2
        // or an undefined encoding), so it must not be claimed here with a bogus `Undefined` condition.
        if (word >> 28) != 0b1111 && word & 0x0E00_0E00 == 0x0C00_0A00 {
            let p = (word >> 24) & 1;
            let w = (word >> 21) & 1;
            let is_load = (word >> 20) & 1 == 1;
            let is_double = (word >> 8) & 1 == 1;
            let d_bit = (word >> 22) & 1;
            let rn = gpr((word >> 16) & 0b1111);
            let vd_field = (word >> 12) & 0b1111;
            if p == 1 && w == 0 {
                // VLDR / VSTR (single transfer)
                let imm8 = (word & 0xFF) as i32;
                let offset = if (word >> 23) & 1 == 1 { imm8 * 4 } else { -imm8 * 4 };
                return Ok(Some(match (is_load, is_double) {
                    (true, false) => Self::Vldr_Single_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), rn, offset),
                    (false, false) => Self::Vstr_Single_A1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), rn, offset),
                    (true, true) => Self::Vldr_Double_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), rn, offset),
                    (false, true) => Self::Vstr_Double_A1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), rn, offset),
                }));
            } else {
                // VLDM / VSTM
                let writeback = w == 1;
                let decrement_before = p == 1;
                let imm8 = (word & 0xFF) as u8;
                return Ok(Some(if is_double {
                    let first = Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit);
                    if is_load { Self::Vldm_Double_A1(cond, rn, writeback, decrement_before, first, imm8 / 2) } else { Self::Vstm_Double_A1(cond, rn, writeback, decrement_before, first, imm8 / 2) }
                } else {
                    let first = Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit);
                    if is_load { Self::Vldm_Single_A1(cond, rn, writeback, decrement_before, first, imm8) } else { Self::Vstm_Single_A1(cond, rn, writeback, decrement_before, first, imm8) }
                }));
            }
        }
        // NEON VDUP (from an ARM core register) : cccc 1110 1 B Q 0 Vd Rt 1011 D 0 E 1 0000. Conditional, and
        // it lives in the coproc-1011 space, so it is decoded before the (coproc-1011-skipping) MCR/CDP group.
        if word & 0x0F90_0F5F == 0x0E80_0B10 {
            let b = (word >> 22) & 1;
            let e = (word >> 5) & 1;
            let size = match (b, e) {
                (1, 0) => Arm32NeonSize::I8,
                (0, 1) => Arm32NeonSize::I16,
                _ => Arm32NeonSize::I32, // (0,0); (1,1) is unallocated
            };
            let vd = (word >> 16) & 0b1111;
            let d_bit = (word >> 7) & 1;
            let rt = gpr((word >> 12) & 0b1111);
            return Ok(Some(if (word >> 21) & 1 == 1 {
                Self::NeonVdupCore_Q_A1(cond, size, Arm32QuadwordRegister::from_field_and_bit(vd, d_bit), rt)
            } else {
                Self::NeonVdupCore_D_A1(cond, size, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d_bit), rt)
            }));
        }
        // coprocessor MCRR / MRRC : cccc 1100 010 L Rt2 Rt coproc opc1 CRm  (decode before LDC/STC -- both [27:25]=110)
        if word & 0x0FE0_0000 == 0x0C40_0000 && !is_fp_coproc(word) {
            let uncond = (word >> 28) == 0xF;
            let is_load = (word >> 20) & 1 == 1;
            let coproc = ((word >> 8) & 0xF) as u8;
            let opc1 = ((word >> 4) & 0xF) as u8;
            let crm = (word & 0xF) as u8;
            let rt2 = gpr((word >> 16) & 0b1111);
            let rt = gpr((word >> 12) & 0b1111);
            return Ok(Some(match (is_load, uncond) {
                (false, false) => Self::Mcrr_A1(cond, coproc, opc1, rt, rt2, crm),
                (true, false) => Self::Mrrc_A1(cond, coproc, opc1, rt, rt2, crm),
                (false, true) => Self::Mcrr2_A1(coproc, opc1, rt, rt2, crm),
                (true, true) => Self::Mrrc2_A1(coproc, opc1, rt, rt2, crm),
            }));
        }
        // MCR / MRC : cccc 1110 opc1 L CRn Rt coproc opc2 1 CRm
        if word & 0x0F00_0010 == 0x0E00_0010 && !is_fp_coproc(word) {
            let uncond = (word >> 28) == 0xF;
            let is_load = (word >> 20) & 1 == 1;
            let opc1 = ((word >> 21) & 0b111) as u8;
            let crn = ((word >> 16) & 0xF) as u8;
            let rt = gpr((word >> 12) & 0b1111);
            let coproc = ((word >> 8) & 0xF) as u8;
            let opc2 = ((word >> 5) & 0b111) as u8;
            let crm = (word & 0xF) as u8;
            return Ok(Some(match (is_load, uncond) {
                (false, false) => Self::Mcr_A1(cond, coproc, opc1, rt, crn, crm, opc2),
                (true, false) => Self::Mrc_A1(cond, coproc, opc1, rt, crn, crm, opc2),
                (false, true) => Self::Mcr2_A1(coproc, opc1, rt, crn, crm, opc2),
                (true, true) => Self::Mrc2_A1(coproc, opc1, rt, crn, crm, opc2),
            }));
        }
        // CDP : cccc 1110 opc1 CRn CRd coproc opc2 0 CRm
        if word & 0x0F00_0010 == 0x0E00_0000 && !is_fp_coproc(word) {
            let uncond = (word >> 28) == 0xF;
            let opc1 = ((word >> 20) & 0xF) as u8;
            let crn = ((word >> 16) & 0xF) as u8;
            let crd = ((word >> 12) & 0xF) as u8;
            let coproc = ((word >> 8) & 0xF) as u8;
            let opc2 = ((word >> 5) & 0b111) as u8;
            let crm = (word & 0xF) as u8;
            return Ok(Some(if uncond {
                Self::Cdp2_A1(coproc, opc1, crd, crn, crm, opc2)
            } else {
                Self::Cdp_A1(cond, coproc, opc1, crd, crn, crm, opc2)
            }));
        }
        // LDC / STC : cccc 110 P U N W L Rn CRd coproc imm8
        if word & 0x0E00_0000 == 0x0C00_0000 && !is_fp_coproc(word) {
            let uncond = (word >> 28) == 0xF;
            let p = (word >> 24) & 1;
            let add = (word >> 23) & 1 == 1;
            let long = (word >> 22) & 1 == 1;
            let w = (word >> 21) & 1;
            let is_load = (word >> 20) & 1 == 1;
            let rn = gpr((word >> 16) & 0b1111);
            let crd = ((word >> 12) & 0xF) as u8;
            let coproc = ((word >> 8) & 0xF) as u8;
            let imm8 = (word & 0xFF) as u8;
            let index = ldc_index_from_p_w(p, w);
            return Ok(Some(match (is_load, uncond) {
                (true, false) => Self::Ldc_A1(cond, coproc, long, crd, rn, add, imm8, index),
                (false, false) => Self::Stc_A1(cond, coproc, long, crd, rn, add, imm8, index),
                (true, true) => Self::Ldc2_A1(coproc, long, crd, rn, add, imm8, index),
                (false, true) => Self::Stc2_A1(coproc, long, crd, rn, add, imm8, index),
            }));
        }

        Err(DecodeError::InvalidOpcode)
    }

    // ---- target gating ----
    pub fn requirement(&self) -> ArmInstructionRequirement {
        match self {
            // MOVW/MOVT are ARMv6T2/ARMv7 (we floor them at the A/R v7 baseline).
            Self::Movw_A2(..) | Self::Movt_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            // the architectural NOP hint (distinct from `mov r0, r0`) is ARMv6K / ARMv6T2 onwards.
            Self::Nop_A1(..) | Self::Yield_A1(..) | Self::Wfe_A1(..) | Self::Wfi_A1(..) | Self::Sev_A1(..) | Self::Dbg_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            Self::Csdb_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            Self::Esb_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv8A, &[]),
            Self::Sb_A1 => ArmInstructionRequirement::new(ArmIsaVersion::Armv8A, &[]), // FEAT_SB
            // memory barriers are ARMv7; BKPT is ARMv5T; SMC/UDF are ARMv6; HVC/ERET are ARMv7VE.
            Self::Dmb_A1(..) | Self::Dsb_A1(..) | Self::Isb_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            Self::Bkpt_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv5TE, &[]),
            Self::Smc_A1(..) | Self::Udf_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            Self::Hvc_A1(..) | Self::Eret_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            // PLD is ARMv5TE; PLI and PLDW are ARMv7; RFE/SRS are ARMv6.
            Self::Pld_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv5TE, &[]),
            Self::Pli_A1(..) | Self::Pldw_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            Self::Rfe_A1(..) | Self::Srs_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            // ARMv8-A AArch32 additions: CRC32, the load-acquire/store-release family, and SEVL.
            Self::Sevl_A1(..)
            | Self::Crc32b_A1(..) | Self::Crc32h_A1(..) | Self::Crc32w_A1(..)
            | Self::Crc32cb_A1(..) | Self::Crc32ch_A1(..) | Self::Crc32cw_A1(..)
            | Self::Lda_A1(..) | Self::Ldab_A1(..) | Self::Ldah_A1(..)
            | Self::Stl_A1(..) | Self::Stlb_A1(..) | Self::Stlh_A1(..)
            | Self::Ldaex_A1(..) | Self::Ldaexb_A1(..) | Self::Ldaexh_A1(..) | Self::Ldaexd_A1(..)
            | Self::Stlex_A1(..) | Self::Stlexb_A1(..) | Self::Stlexh_A1(..) | Self::Stlexd_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv8A, &[]),
            // VFP (hardware floating-point) requires the FloatingPoint extension feature.
            Self::Vldr_Single_A1(..) | Self::Vstr_Single_A1(..) | Self::Vldr_Double_A1(..) | Self::Vstr_Double_A1(..)
            | Self::Vldm_Single_A1(..) | Self::Vstm_Single_A1(..) | Self::Vldm_Double_A1(..) | Self::Vstm_Double_A1(..)
            | Self::FpDataProcess3_Single_A1(..) | Self::FpDataProcess3_Double_A1(..)
            | Self::FpDataProcess2_Single_A1(..) | Self::FpDataProcess2_Double_A1(..)
            | Self::Vcmp_Single_A1(..) | Self::Vcmp_Double_A1(..) | Self::Vcmp_Zero_Single_A1(..) | Self::Vcmp_Zero_Double_A1(..)
            | Self::Vmrs_A1(..) | Self::Vmrs_Apsr_Nzcv_A1(..) | Self::Vmsr_A1(..)
            | Self::Vmov_Core_To_Single_A1(..) | Self::Vmov_Single_To_Core_A1(..)
            | Self::Vmov_Immediate_Single_A1(..) | Self::Vmov_Immediate_Double_A1(..)
            | Self::Vmov_Double_To_CorePair_A1(..) | Self::Vmov_CorePair_To_Double_A1(..)
            | Self::Vmov_Singles_To_CorePair_A1(..) | Self::Vmov_CorePair_To_Singles_A1(..)
            | Self::Vcvt_FloatToInt_FromSingle_A1(..) | Self::Vcvt_FloatToInt_FromDouble_A1(..)
            | Self::Vcvt_IntToFloat_ToSingle_A1(..) | Self::Vcvt_IntToFloat_ToDouble_A1(..)
            | Self::Vcvt_Single_To_Double_A1(..) | Self::Vcvt_Double_To_Single_A1(..)
            | Self::Vcvt_HalfToSingle_A1(..) | Self::Vcvt_SingleToHalf_A1(..)
            | Self::Vcvt_FloatToFixed_Single_A1(..) | Self::Vcvt_FloatToFixed_Double_A1(..)
            | Self::Vcvt_FixedToFloat_Single_A1(..) | Self::Vcvt_FixedToFloat_Double_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[ArmCpuFeature::FloatingPoint]),

            // ARMv8-A floating-point additions require the v8 baseline (plus an FPU).
            Self::Vsel_Single_A1(..) | Self::Vsel_Double_A1(..)
            | Self::Vmaxnm_Single_A1(..) | Self::Vmaxnm_Double_A1(..)
            | Self::Vminnm_Single_A1(..) | Self::Vminnm_Double_A1(..)
            | Self::Vrint_Directed_Single_A1(..) | Self::Vrint_Directed_Double_A1(..)
            | Self::Vrint_Cond_Single_A1(..) | Self::Vrint_Cond_Double_A1(..)
            | Self::Vcvt_Directed_FromSingle_A1(..) | Self::Vcvt_Directed_FromDouble_A1(..)
            // half <-> double needs FEAT_FP16 (Armv8.2-A); VJCVT needs FEAT_JSCVT (Armv8.3-A); the model has no
            // finer gate than the v8-A FP group.
            | Self::Vcvt_HalfToDouble_A1(..) | Self::Vcvt_DoubleToHalf_A1(..)
            | Self::Vjcvt_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv8A, &[ArmCpuFeature::FloatingPoint]),

            // NEON Advanced SIMD -- available from ARMv7-A/R with the Advanced SIMD extension.
            Self::NeonInt3Same_D_A1(..) | Self::NeonInt3Same_Q_A1(..)
            | Self::NeonFloat3Same_D_A1(..) | Self::NeonFloat3Same_Q_A1(..)
            | Self::NeonBitwise3Same_D_A1(..) | Self::NeonBitwise3Same_Q_A1(..)
            | Self::NeonMisc2Sized_D_A1(..) | Self::NeonMisc2Sized_Q_A1(..)
            | Self::NeonMisc2Narrow_A1(..) | Self::NeonShllMax_A1(..)
            | Self::NeonDiffLong_A1(..) | Self::NeonDiffWide_A1(..) | Self::NeonDiffNarrow_A1(..)
            | Self::NeonScalar_D_A1(..) | Self::NeonScalar_Q_A1(..) | Self::NeonScalarLong_A1(..)
            | Self::NeonShift_D_A1(..) | Self::NeonShift_Q_A1(..) | Self::NeonShiftNarrow_A1(..) | Self::NeonShiftLong_A1(..)
            | Self::NeonExt_D_A1(..) | Self::NeonExt_Q_A1(..) | Self::NeonTableLookup_A1(..)
            | Self::NeonVdupScalar_D_A1(..) | Self::NeonVdupScalar_Q_A1(..)
            | Self::NeonVdupCore_D_A1(..) | Self::NeonVdupCore_Q_A1(..)
            | Self::NeonModifiedImmediate_D_A1(..) | Self::NeonModifiedImmediate_Q_A1(..)
            | Self::NeonLoadStoreMultiple_A1(..) | Self::NeonLoadStoreSingleLane_A1(..) | Self::NeonLoadStoreAllLanes_A1(..)
            | Self::Vmov_Core_To_Scalar_A1(..) | Self::Vmov_Scalar_To_Core_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[ArmCpuFeature::AdvancedSimd]),

            // ARMv8 cryptography extension: the v8 baseline plus NEON plus the crypto feature.
            Self::NeonAes_A1(..) | Self::NeonSha3Reg_A1(..) | Self::NeonSha2Reg_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv8A, &[ArmCpuFeature::AdvancedSimd, ArmCpuFeature::Crypto]),
            // the fixed-size 2-reg-misc group mixes v7 ops with the v8 rounding/anchored-convert members.
            Self::NeonMisc2Fixed_D_A1(op, ..) | Self::NeonMisc2Fixed_Q_A1(op, ..) => {
                let isa = if op.is_armv8() { ArmIsaVersion::Armv8A } else { ArmIsaVersion::Armv7AR };
                ArmInstructionRequirement::new(isa, &[ArmCpuFeature::AdvancedSimd])
            },
            // multiply: MUL/MLA and the 32x32->64 long multiplies are the A32 baseline; UMAAL is ARMv6; MLS is ARMv6T2/v7.
            Self::Mul_A1(..) | Self::Mla_A1(..)
            | Self::Umull_A1(..) | Self::Umlal_A1(..) | Self::Smull_A1(..) | Self::Smlal_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            Self::Umaal_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            Self::Mls_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            // saturating arithmetic is the ARMv5TE "E" (DSP) extension (baseline from ARMv6).
            Self::Qadd_A1(..) | Self::Qsub_A1(..) | Self::Qdadd_A1(..) | Self::Qdsub_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv5TE, &[]),
            // signed-multiply halfword forms are ARMv5TE; the dual / most-significant-word forms are ARMv6.
            Self::Smla_A1(..) | Self::Smlaw_A1(..) | Self::Smulw_A1(..) | Self::Smlal_Halfword_A1(..) | Self::Smul_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv5TE, &[]),
            Self::Smlad_A1(..) | Self::Smuad_A1(..) | Self::Smlsd_A1(..) | Self::Smusd_A1(..)
            | Self::Smmla_A1(..) | Self::Smmul_A1(..) | Self::Smmls_A1(..)
            | Self::Smlald_A1(..) | Self::Smlsld_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            // the packed-SIMD (parallel) add/subtracts and SEL are the ARMv6 media instructions.
            Self::ParallelAddSub_A1(..) | Self::Sel_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            // extend, extend-and-add and the byte-reverses are ARMv6; RBIT is ARMv6T2/v7; CLZ is ARMv5T.
            Self::Extend_A1(..) | Self::ExtendAndAdd_A1(..)
            | Self::Rev_A1(..) | Self::Rev16_A1(..) | Self::Revsh_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            Self::Rbit_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            Self::Clz_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv5TE, &[]),
            // pack-halfword, the saturates and the sum-of-absolute-differences are ARMv6 media instructions.
            Self::Pkhbt_A1(..) | Self::Pkhtb_A1(..)
            | Self::Ssat_A1(..) | Self::Usat_A1(..) | Self::Ssat16_A1(..) | Self::Usat16_A1(..)
            | Self::Usad8_A1(..) | Self::Usada8_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            // the bitfield instructions are ARMv6T2 / ARMv7.
            Self::Bfc_A1(..) | Self::Bfi_A1(..) | Self::Sbfx_A1(..) | Self::Ubfx_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            // single word/byte load/store (and the unprivileged T forms) are the A32 baseline.
            Self::Ldr_A1(..) | Self::Str_A1(..) | Self::Ldrb_A1(..) | Self::Strb_A1(..)
            | Self::Ldrt_A1(..) | Self::Strt_A1(..) | Self::Ldrbt_A1(..) | Self::Strbt_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            // halfword/signed load/store are ARMv4T; LDRD/STRD are ARMv5TE; the H/SB/SH unprivileged T forms are ARMv6T2/v7.
            Self::Ldrh_A1(..) | Self::Strh_A1(..) | Self::Ldrsb_A1(..) | Self::Ldrsh_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            Self::Ldrd_A1(..) | Self::Strd_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv5TE, &[]),
            Self::Ldrht_A1(..) | Self::Strht_A1(..) | Self::Ldrsbt_A1(..) | Self::Ldrsht_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            // load/store-multiple (incl. PUSH/POP spellings and the user-mode `^` forms) are the A32 baseline.
            Self::Ldm_A1(..) | Self::Stm_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            // SWP/SWPB are the (deprecated) ARMv4T swap; the exclusive-access family and CLREX are ARMv6/ARMv6K.
            Self::Swp_A1(..) | Self::Swpb_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            Self::Ldrex_A1(..) | Self::Strex_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            Self::Ldrexb_A1(..) | Self::Strexb_A1(..) | Self::Ldrexh_A1(..) | Self::Strexh_A1(..)
            | Self::Ldrexd_A1(..) | Self::Strexd_A1(..) | Self::Clrex_A1 => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            // everything else here (the data-processing family, BX, SVC) is the A32 baseline.
            Self::And_Immediate_A1(..) | Self::And_Register_A1(..)
            | Self::Eor_Immediate_A1(..) | Self::Eor_Register_A1(..)
            | Self::Sub_Immediate_A1(..) | Self::Sub_Register_A1(..)
            | Self::Rsb_Immediate_A1(..) | Self::Rsb_Register_A1(..)
            | Self::Add_Immediate_A1(..) | Self::Add_Register_A1(..)
            | Self::Adc_Immediate_A1(..) | Self::Adc_Register_A1(..)
            | Self::Sbc_Immediate_A1(..) | Self::Sbc_Register_A1(..)
            | Self::Rsc_Immediate_A1(..) | Self::Rsc_Register_A1(..)
            | Self::Orr_Immediate_A1(..) | Self::Orr_Register_A1(..)
            | Self::Bic_Immediate_A1(..) | Self::Bic_Register_A1(..)
            | Self::Mov_Immediate_A1(..) | Self::Mov_Register_A1(..)
            | Self::Mvn_Immediate_A1(..) | Self::Mvn_Register_A1(..)
            | Self::Tst_Immediate_A1(..) | Self::Tst_Register_A1(..)
            | Self::Teq_Immediate_A1(..) | Self::Teq_Register_A1(..)
            | Self::Cmp_Immediate_A1(..) | Self::Cmp_Register_A1(..)
            | Self::Cmn_Immediate_A1(..) | Self::Cmn_Register_A1(..)
            | Self::And_RegisterShiftedRegister_A1(..) | Self::Eor_RegisterShiftedRegister_A1(..)
            | Self::Sub_RegisterShiftedRegister_A1(..) | Self::Rsb_RegisterShiftedRegister_A1(..)
            | Self::Add_RegisterShiftedRegister_A1(..) | Self::Adc_RegisterShiftedRegister_A1(..)
            | Self::Sbc_RegisterShiftedRegister_A1(..) | Self::Rsc_RegisterShiftedRegister_A1(..)
            | Self::Orr_RegisterShiftedRegister_A1(..) | Self::Bic_RegisterShiftedRegister_A1(..)
            | Self::Mov_RegisterShiftedRegister_A1(..) | Self::Mvn_RegisterShiftedRegister_A1(..)
            | Self::Tst_RegisterShiftedRegister_A1(..) | Self::Teq_RegisterShiftedRegister_A1(..)
            | Self::Cmp_RegisterShiftedRegister_A1(..) | Self::Cmn_RegisterShiftedRegister_A1(..)
            | Self::Bx_A1(..)
            | Self::Svc_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            // B/BL are the A32 baseline; BLX (imm/reg) is ARMv5T; BXJ is ARMv5TEJ/ARMv6.
            Self::B_A1(..) | Self::Bl_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            Self::Blx_Immediate_A1(..) | Self::Blx_Register_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv5TE, &[]),
            Self::Bxj_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            // MRS/MSR (CPSR/SPSR access) are the A32 baseline; CPS and SETEND are ARMv6.
            Self::Mrs_A1(..) | Self::Msr_Register_A1(..) | Self::Msr_Immediate_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            // banked register transfer is an ARMv7VE (Virtualization) addition -- gated at the A/R baseline here.
            Self::MrsBanked_A1(..) | Self::MsrBanked_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7AR, &[]),
            Self::Cps_A1(..) | Self::Setend_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
            // coprocessor: CDP/LDC/STC/MCR/MRC are the A32 baseline; MCRR/MRRC + the "2" forms are ARMv5TE; MCRR2/MRRC2 ARMv6.
            Self::Mcr_A1(..) | Self::Mrc_A1(..) | Self::Cdp_A1(..) | Self::Ldc_A1(..) | Self::Stc_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv4T, &[]),
            Self::Mcr2_A1(..) | Self::Mrc2_A1(..) | Self::Cdp2_A1(..) | Self::Ldc2_A1(..) | Self::Stc2_A1(..)
            | Self::Mcrr_A1(..) | Self::Mrrc_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv5TE, &[]),
            Self::Mcrr2_A1(..) | Self::Mrrc2_A1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv6, &[]),
        }
    }

    /// Encode this instruction only if `target_profile` supports it. A Thumb-only M-profile target has no
    /// ARM state, so its profile rejects every A32 form (A32 requires the A/R lineage). Returns
    /// [`EncodeError`] if the target lacks ARM state or the required extension.
    pub fn encode_for_target(&self, target_profile: &ArmTargetProfile) -> Result<Vec<u8>, EncodeError> {
        let requirement = self.requirement();
        if !target_profile.supports(&requirement) {
            return Err(EncodeError::UnsupportedInstructionForTarget {
                required: requirement,
                target_isa_version: target_profile.isa_version(),
            });
        }
        self.encode()
    }
}

// ===================== data-processing opcode nibbles (bits 24:21) =====================
const OP_AND: u32 = 0b0000;
const OP_EOR: u32 = 0b0001;
const OP_SUB: u32 = 0b0010;
const OP_RSB: u32 = 0b0011;
const OP_ADD: u32 = 0b0100;
const OP_ADC: u32 = 0b0101;
const OP_SBC: u32 = 0b0110;
const OP_RSC: u32 = 0b0111;
const OP_TST: u32 = 0b1000;
const OP_TEQ: u32 = 0b1001;
const OP_CMP: u32 = 0b1010;
const OP_CMN: u32 = 0b1011;
const OP_ORR: u32 = 0b1100;
const OP_MOV: u32 = 0b1101;
const OP_BIC: u32 = 0b1110;
const OP_MVN: u32 = 0b1111;

// ===================== encode helpers =====================

// the 4-bit condition code in bits[31:28]
fn cond_bits(cond: &Arm32Condition) -> u32 {
    (cond.as_operand_bits() as u32) << 28
}

// a 4-bit register number (consumers shift it into place)
fn reg(register: &Arm32GeneralPurposeRegister) -> u32 {
    register.as_operand_bits() as u32
}

fn gpr(bits: u32) -> Arm32GeneralPurposeRegister {
    Arm32GeneralPurposeRegister::from_operand_bits((bits & 0b1111) as u8)
}

// the S (set-flags) bit, bit[20]
fn s_bit(set_flags: bool) -> u32 {
    if set_flags { 1 << 20 } else { 0 }
}

fn check_unsigned_maximum(field: &'static str, value: u32, maximum: u32) -> Result<(), EncodeError> {
    if value > maximum {
        return Err(EncodeError::ImmediateOutOfRange { field, value: value as i64, minimum: 0, maximum: maximum as i64 });
    }
    Ok(())
}

// branch / interwork: cccc <base> imm24, where imm24 = (decoded byte offset) / 4
fn encode_a32_branch(cond: &Arm32Condition, base: u32, offset: i32) -> Result<u32, EncodeError> {
    if offset % 4 != 0 {
        return Err(EncodeError::ImmediateNotAligned { field: "offset", value: offset as i64, required_multiple: 4 });
    }
    let imm = offset >> 2;
    check_signed_24("offset", imm, offset)?;
    Ok(cond_bits(cond) | base | ((imm as u32) & 0x00FF_FFFF))
}
fn check_multiple_of_2(field: &'static str, offset: i32) -> Result<(), EncodeError> {
    if offset % 2 != 0 {
        return Err(EncodeError::ImmediateNotAligned { field, value: offset as i64, required_multiple: 2 });
    }
    Ok(())
}
fn check_signed_24(field: &'static str, imm: i32, offset: i32) -> Result<(), EncodeError> {
    if !(-(1 << 23)..(1 << 23)).contains(&imm) {
        return Err(EncodeError::ImmediateOutOfRange { field, value: offset as i64, minimum: -(1 << 25), maximum: (1 << 25) - 4 });
    }
    Ok(())
}
// sign-extend a 24-bit value to i32
fn sign_extend_24(value: u32) -> i32 {
    ((value << 8) as i32) >> 8
}

// BKPT/HVC/UDF split a 16-bit immediate as imm12 (bits[19:8]) : imm4 (bits[3:0])
fn imm16_split(imm16: u16) -> u32 {
    ((((imm16 >> 4) & 0x0FFF) as u32) << 8) | ((imm16 & 0xF) as u32)
}
fn imm16_join(word: u32) -> u16 {
    ((((word >> 8) & 0x0FFF) << 4) | (word & 0xF)) as u16
}

// bitfield lsb/width: 0 <= lsb <= 31, 1 <= width, lsb + width <= 32
fn check_bitfield(lsb: u8, width: u8) -> Result<(), EncodeError> {
    if lsb > 31 {
        return Err(EncodeError::ImmediateOutOfRange { field: "lsb", value: lsb as i64, minimum: 0, maximum: 31 });
    }
    if !(1..=32).contains(&width) {
        return Err(EncodeError::ImmediateOutOfRange { field: "width", value: width as i64, minimum: 1, maximum: 32 });
    }
    if lsb as u16 + width as u16 > 32 {
        return Err(EncodeError::ImmediateOutOfRange { field: "width", value: width as i64, minimum: 1, maximum: (32 - lsb) as i64 });
    }
    Ok(())
}

// data processing (immediate): cccc 001 opcode S Rn Rd imm12
fn encode_dp_immediate(cond: &Arm32Condition, opcode: u32, set_flags: bool, rn: u32, rd: u32, value: u32) -> Result<u32, EncodeError> {
    let imm12 = encode_a32_modified_immediate(value)
        .ok_or(EncodeError::ModifiedImmediateNotEncodable { field: "const", value })?;
    Ok(cond_bits(cond) | 0x0200_0000 | (opcode << 21) | s_bit(set_flags) | (rn << 16) | (rd << 12) | (imm12 as u32))
}

// data processing (register, immediate shift): cccc 000 opcode S Rn Rd imm5 type 0 Rm
fn encode_dp_register(cond: &Arm32Condition, opcode: u32, set_flags: bool, rn: u32, rd: u32, rm: u32, shift: &Arm32RegisterShift) -> Result<u32, EncodeError> {
    let (imm5, shift_type) = encode_a32_shift_imm5_type(shift)?;
    Ok(cond_bits(cond) | (opcode << 21) | s_bit(set_flags) | (rn << 16) | (rd << 12) | (imm5 << 7) | (shift_type << 5) | rm)
}

// data processing (register-shifted register): cccc 000 opcode S Rn Rd Rs 0 type 1 Rm
fn encode_dp_register_shifted(cond: &Arm32Condition, opcode: u32, set_flags: bool, rn: u32, rd: u32, rm: u32, shift_type: Arm32ShiftType, rs: u32) -> u32 {
    cond_bits(cond) | (opcode << 21) | s_bit(set_flags) | (rn << 16) | (rd << 12) | (rs << 8) | (shift_type.type_bits() << 5) | (1 << 4) | rm
}

// MOVW (is_movt=false): cccc 0011 0000 imm4 Rd imm12 ; MOVT (is_movt=true): cccc 0011 0100 imm4 Rd imm12
fn encode_movw_movt(cond: &Arm32Condition, is_movt: bool, rd: u32, imm16: u16) -> u32 {
    let imm4 = ((imm16 >> 12) & 0b1111) as u32;
    let imm12 = (imm16 & 0x0FFF) as u32;
    let base = if is_movt { 0x0340_0000 } else { 0x0300_0000 };
    cond_bits(cond) | base | (imm4 << 16) | (rd << 12) | imm12
}

// multiply / long multiply: cccc 0000 op S high low Rm 1001 Rn  (base_with_90 carries the op bits + 1001)
fn encode_multiply(cond: &Arm32Condition, base_with_90: u32, set_flags: bool, high: u32, low: u32, rm: u32, rn: u32) -> u32 {
    cond_bits(cond) | base_with_90 | s_bit(set_flags) | (high << 16) | (low << 12) | (rm << 8) | rn
}

// saturating add/sub: cccc 00010 op 0 Rn Rd 0000 0101 Rm  (base carries the op bits + the 0101)
fn encode_saturating(cond: &Arm32Condition, base: u32, rd: u32, rm: u32, rn: u32) -> u32 {
    cond_bits(cond) | base | (rn << 16) | (rd << 12) | rm
}

// the M (Rm-half) and N (Rn-half) selector bits of a halfword multiply: m -> bit6, n -> bit5
fn nm_bits(n: bool, m: bool) -> u32 {
    ((m as u32) << 6) | ((n as u32) << 5)
}

// signed multiply word: cccc <base> high accum Rm Rn  (base carries the op + op2 bits; high/accum are
// Rd/Ra or RdHi/RdLo, Rm at 11:8, Rn at 3:0)
fn encode_signed_mul(cond: &Arm32Condition, base: u32, high: u32, accum: u32, rm: u32, rn: u32) -> u32 {
    cond_bits(cond) | base | (high << 16) | (accum << 12) | (rm << 8) | rn
}

// ---- parallel (packed SIMD) add/sub field codecs (A32 field positions) ----
// prefix selector at bits[22:20]
fn a32_parallel_prefix_bits(prefix: Arm32ParallelPrefix) -> u32 {
    match prefix {
        Arm32ParallelPrefix::Signed => 0b001,
        Arm32ParallelPrefix::SignedSaturating => 0b010,
        Arm32ParallelPrefix::SignedHalving => 0b011,
        Arm32ParallelPrefix::Unsigned => 0b101,
        Arm32ParallelPrefix::UnsignedSaturating => 0b110,
        Arm32ParallelPrefix::UnsignedHalving => 0b111,
    }
}
fn a32_parallel_prefix_from_bits(bits: u32) -> Option<Arm32ParallelPrefix> {
    Some(match bits {
        0b001 => Arm32ParallelPrefix::Signed,
        0b010 => Arm32ParallelPrefix::SignedSaturating,
        0b011 => Arm32ParallelPrefix::SignedHalving,
        0b101 => Arm32ParallelPrefix::Unsigned,
        0b110 => Arm32ParallelPrefix::UnsignedSaturating,
        0b111 => Arm32ParallelPrefix::UnsignedHalving,
        _ => return None,
    })
}
// operation selector at bits[7:4]
fn a32_parallel_op_bits(op: Arm32ParallelOperation) -> u32 {
    match op {
        Arm32ParallelOperation::Add16 => 0b0001,
        Arm32ParallelOperation::Asx => 0b0011,
        Arm32ParallelOperation::Sax => 0b0101,
        Arm32ParallelOperation::Sub16 => 0b0111,
        Arm32ParallelOperation::Add8 => 0b1001,
        Arm32ParallelOperation::Sub8 => 0b1111,
    }
}
fn a32_parallel_op_from_bits(bits: u32) -> Option<Arm32ParallelOperation> {
    Some(match bits {
        0b0001 => Arm32ParallelOperation::Add16,
        0b0011 => Arm32ParallelOperation::Asx,
        0b0101 => Arm32ParallelOperation::Sax,
        0b0111 => Arm32ParallelOperation::Sub16,
        0b1001 => Arm32ParallelOperation::Add8,
        0b1111 => Arm32ParallelOperation::Sub8,
        _ => return None,
    })
}

// extend / extend-and-add: cccc <byte> Rn Rd rotate 00 0111 Rm
fn encode_extend(cond: &Arm32Condition, opcode_byte: u32, rn: u32, rd: u32, rm: u32, rotate_field: u32) -> u32 {
    cond_bits(cond) | (opcode_byte << 20) | (rn << 16) | (rd << 12) | (rotate_field << 10) | 0x70 | rm
}
// the rotation operand is the decoded amount 0/8/16/24; the field at bits[11:10] is amount/8
fn encode_rotation(rotation: u8) -> Result<u32, EncodeError> {
    match rotation {
        0 => Ok(0),
        8 => Ok(1),
        16 => Ok(2),
        24 => Ok(3),
        _ => Err(EncodeError::ImmediateOutOfRange { field: "rotation", value: rotation as i64, minimum: 0, maximum: 24 }),
    }
}
fn decode_rotation(field: u32) -> u8 {
    (field as u8) * 8
}

// load/store single (word/byte): cccc 01 I P U B W L Rn Rt <offset> ; I=1 (register) for the shifted-Rm form
fn encode_load_store(cond: &Arm32Condition, is_byte: bool, is_load: bool, p: u32, w: u32, rn: u32, rt: u32, offset: &Arm32MemoryOffset) -> Result<u32, EncodeError> {
    let (base, add, tail) = match offset {
        Arm32MemoryOffset::Immediate { add, imm12 } => {
            check_unsigned_maximum("imm12", *imm12 as u32, 4095)?;
            (0x0400_0000, *add as u32, *imm12 as u32)
        },
        Arm32MemoryOffset::Register { add, rm, shift } => {
            let (imm5, shift_type) = encode_a32_shift_imm5_type(shift)?;
            (0x0600_0000, *add as u32, (imm5 << 7) | (shift_type << 5) | reg(rm))
        },
    };
    Ok(cond_bits(cond)
        | base
        | (p << 24)
        | (add << 23)
        | ((is_byte as u32) << 22)
        | (w << 21)
        | ((is_load as u32) << 20)
        | (rn << 16)
        | (rt << 12)
        | tail)
}

// preload (PLD/PLI/PLDW): imm_base/reg_base carry [27:20] + the fixed 1111 in [15:12]; offset adds U + Rn + the operand
fn encode_preload(imm_base: u32, reg_base: u32, rn: u32, offset: &Arm32MemoryOffset) -> Result<u32, EncodeError> {
    match offset {
        Arm32MemoryOffset::Immediate { add, imm12 } => {
            check_unsigned_maximum("imm12", *imm12 as u32, 4095)?;
            Ok(imm_base | ((*add as u32) << 23) | (rn << 16) | (*imm12 as u32))
        },
        Arm32MemoryOffset::Register { add, rm, shift } => {
            let (imm5, shift_type) = encode_a32_shift_imm5_type(shift)?;
            Ok(reg_base | ((*add as u32) << 23) | (rn << 16) | (imm5 << 7) | (shift_type << 5) | reg(rm))
        },
    }
}
fn preload_imm_offset(word: u32) -> Arm32MemoryOffset {
    Arm32MemoryOffset::Immediate { add: (word >> 23) & 1 == 1, imm12: (word & 0x0FFF) as u16 }
}
fn preload_reg_offset(word: u32) -> Arm32MemoryOffset {
    Arm32MemoryOffset::Register {
        add: (word >> 23) & 1 == 1,
        rm: gpr(word & 0b1111),
        shift: decode_a32_shift(((word >> 7) & 0b1_1111) as u8, ((word >> 5) & 0b11) as u8),
    }
}

// the coprocessor numbers 1010 / 1011 are the VFP / Advanced-SIMD space, NOT a real coprocessor -- the
// generic LDC/STC/MCR/MRC/CDP/MCRR decoders must skip them so the VFP decoders can claim them.
fn is_fp_coproc(word: u32) -> bool {
    matches!((word >> 8) & 0xF, 0b1010 | 0b1011)
}

// VFP scalar load/store: cccc 110 P U D W L Rn Vd 101 sz imm8 (offset is a byte displacement, multiple of 4)
fn encode_fp_load_store_a32(cond: &Arm32Condition, base: u32, vd_field: u32, d_bit: u32, rn: u32, offset: i32) -> Result<u32, EncodeError> {
    if offset % 4 != 0 {
        return Err(EncodeError::ImmediateNotAligned { field: "offset", value: offset as i64, required_multiple: 4 });
    }
    if !(-1020..=1020).contains(&offset) {
        return Err(EncodeError::ImmediateOutOfRange { field: "offset", value: offset as i64, minimum: -1020, maximum: 1020 });
    }
    let u = if offset >= 0 { 1u32 } else { 0 };
    let imm8 = offset.unsigned_abs() / 4;
    Ok(cond_bits(cond) | base | (u << 23) | (d_bit << 22) | (rn << 16) | (vd_field << 12) | imm8)
}

// VFP load/store-multiple: cccc 110 P U D W L Rn Vd 101 sz imm8 (imm8 = count for single, 2*count for double)
#[allow(clippy::too_many_arguments)]
fn encode_fp_load_store_multiple_a32(cond: &Arm32Condition, size_low: u32, is_load: bool, rn: u32, writeback: bool, decrement_before: bool, vd_field: u32, d_bit: u32, count: u8, first_number: u8, max_register: u8, is_double: bool) -> Result<u32, EncodeError> {
    if count == 0 || (first_number as u32) + (count as u32) - 1 > max_register as u32 {
        return Err(EncodeError::ImmediateOutOfRange { field: "count", value: count as i64, minimum: 1, maximum: (max_register as i64) - (first_number as i64) + 1 });
    }
    if decrement_before && !writeback {
        return Err(EncodeError::RegisterNotEncodable { field: "rn", detail: "the decrement-before (DB) form requires writeback (!)" });
    }
    let p = if decrement_before { 1u32 } else { 0 };
    let u = if decrement_before { 0u32 } else { 1 };
    let imm8 = if is_double { (count as u32) * 2 } else { count as u32 };
    Ok(cond_bits(cond) | 0x0C00_0000 | size_low | (p << 24) | (u << 23) | (d_bit << 22) | ((writeback as u32) << 21) | ((is_load as u32) << 20) | (rn << 16) | (vd_field << 12) | imm8)
}

// VMOV between a core register pair and a double / two consecutive singles: base | op<<20 | Rt2<<16 | Rt<<12 | M<<5 | Vm
fn encode_vmov_core_pair_a32(cond: &Arm32Condition, base: u32, fp_to_core: bool, rt: u32, rt2: u32, vm_field: u32, m_bit: u32) -> u32 {
    cond_bits(cond) | base | ((fp_to_core as u32) << 20) | (rt2 << 16) | (rt << 12) | (m_bit << 5) | vm_field
}

// NEON three-registers-of-the-same-length word: 1111 001 U 0 D size Vn Vd opc N Q M op Vm.
#[allow(clippy::too_many_arguments)]
fn encode_neon_3same(u: u32, size: u32, opc: u32, op: u32, q: u32, d_bit: u32, vd: u32, n_bit: u32, vn: u32, m_bit: u32, vm: u32) -> u32 {
    0xF200_0000 | (u << 24) | (d_bit << 22) | (size << 20) | (vn << 16) | (vd << 12) | (opc << 8) | (n_bit << 7) | (q << 6) | (m_bit << 5) | (op << 4) | vm
}

// NEON two-registers-miscellaneous word: 1111 0011 1 D 11 size a Vd opc2 bit6 M 0 Vm. bit6 is Q for the
// same-width ops and a sub-opcode for the narrowing ops; either way it lands in bit 6.
#[allow(clippy::too_many_arguments)]
fn encode_neon_2misc(a: u32, size: u32, opc2: u32, bit6: u32, d_bit: u32, vd: u32, m_bit: u32, vm: u32) -> u32 {
    0xF3B0_0000 | (d_bit << 22) | (size << 18) | (a << 16) | (vd << 12) | (opc2 << 7) | (bit6 << 6) | (m_bit << 5) | vm
}

// NEON three-registers-of-different-lengths word: 1111 001 U 1 D size Vn Vd opc N 0 M 0 Vm.
#[allow(clippy::too_many_arguments)]
fn encode_neon_3diff(u: u32, size: u32, opc: u32, d_bit: u32, vd: u32, n_bit: u32, vn: u32, m_bit: u32, vm: u32) -> u32 {
    0xF280_0000 | (u << 24) | (d_bit << 22) | (size << 20) | (vn << 16) | (vd << 12) | (opc << 8) | (n_bit << 7) | (m_bit << 5) | vm
}

// NEON two-registers-and-a-scalar word: 1111 001 X 1 D size Vn Vd opc N 1 M 0 Vm (bit24 = Q or U, bit6 = 1).
#[allow(clippy::too_many_arguments)]
fn encode_neon_scalar(bit24: u32, size: u32, opc: u32, d_bit: u32, vd: u32, n_bit: u32, vn: u32, m_bit: u32, vm: u32) -> u32 {
    0xF280_0000 | (bit24 << 24) | (d_bit << 22) | (size << 20) | (vn << 16) | (vd << 12) | (opc << 8) | (n_bit << 7) | (1 << 6) | (m_bit << 5) | vm
}

// The scalar lane Dm[index] packs differently by element size: a 16-bit lane uses Dm in D0-7 with the index
// in M:Vm[3]; a 32-bit lane uses Dm in D0-15 with the index in M. These are inverses for round-tripping.
fn neon_scalar_vm(size: u32, dm: u8, index: u8) -> (/*vm*/ u32, /*m*/ u32) {
    if size == 0b01 {
        (((dm as u32) & 0x7) | (((index as u32) & 1) << 3), ((index as u32) >> 1) & 1)
    } else {
        ((dm as u32) & 0xF, (index as u32) & 1)
    }
}
fn neon_scalar_decode(size: u32, vm: u32, m: u32) -> (/*dm*/ u8, /*index*/ u8) {
    if size == 0b01 {
        ((vm & 0x7) as u8, (((m << 1) | ((vm >> 3) & 1)) & 0b11) as u8)
    } else {
        ((vm & 0xF) as u8, (m & 1) as u8)
    }
}

// NEON two-registers-and-a-shift-amount word: 1111 001 U 1 D imm6 Vd opc L Q M 1 Vm. bit6 = Q for same-width,
// the rounding bit for narrowing, 0 for widening.
#[allow(clippy::too_many_arguments)]
fn encode_neon_shift(u: u32, imm6: u32, opc: u32, l: u32, bit6: u32, d_bit: u32, vd: u32, m_bit: u32, vm: u32) -> u32 {
    0xF280_0010 | (u << 24) | (d_bit << 22) | (imm6 << 16) | (vd << 12) | (opc << 8) | (l << 7) | (bit6 << 6) | (m_bit << 5) | vm
}

// The 7-bit (L:imm6) shift field for a NEON same-width shift-by-immediate: `esize + shift` for a left shift
// (valid shift `0..esize-1`), `2*esize - shift` for a right shift (valid shift `1..esize`). An out-of-range
// shift -- which a parsed or hand-built model can carry -- is REJECTED here rather than left to under/overflow,
// so `encode` stays total (it returns an error, never panics; found by the `arm32_asm_parse` fuzz target).
fn neon_shift_imm6(is_left: bool, esize: u32, shift: u32) -> Result<u32, EncodeError> {
    if is_left {
        if shift >= esize {
            return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: shift as i64, minimum: 0, maximum: (esize - 1) as i64 });
        }
        Ok(esize + shift)
    } else {
        if shift < 1 || shift > esize {
            return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: shift as i64, minimum: 1, maximum: esize as i64 });
        }
        Ok(2 * esize - shift)
    }
}

// Recover (element size, shift amount) from the L:imm6 field of a same-width shift. The element size is the
// position of the leading 1 (L=1 -> 64-bit); a right shift is 2*esize - field, a left shift is field - esize.
// Returns None if no size bit is set (that encoding belongs to the modified-immediate format, not a shift).
fn decode_neon_shift_amount(l: u32, imm6: u32, is_left: bool) -> Option<(Arm32NeonSize, u8)> {
    let field7 = (l << 6) | imm6;
    let (size, esize) = if l == 1 {
        (Arm32NeonSize::I64, 64u32)
    } else if imm6 & 0b100000 != 0 {
        (Arm32NeonSize::I32, 32)
    } else if imm6 & 0b010000 != 0 {
        (Arm32NeonSize::I16, 16)
    } else if imm6 & 0b001000 != 0 {
        (Arm32NeonSize::I8, 8)
    } else {
        return None;
    };
    let shift = if is_left { field7 - esize } else { 2 * esize - field7 };
    Some((size, shift as u8))
}

// Narrowing shift: imm6's leading bit gives the RESULT element size (8/16/32); the carried source size is
// twice that (.i16/.i32/.i64), and the shift amount is 2*result_esize - imm6.
fn decode_neon_narrow_shift(imm6: u32) -> Option<(Arm32NeonSize, u8)> {
    let (size, result_esize) = if imm6 & 0b100000 != 0 {
        (Arm32NeonSize::I64, 32u32)
    } else if imm6 & 0b010000 != 0 {
        (Arm32NeonSize::I32, 16)
    } else {
        (Arm32NeonSize::I16, 8)
    };
    // shift = 2*result_esize - imm6 must be in 1..=result_esize. imm6 < result_esize is only reached when the
    // L bit pushed L:imm6 past the >=8 gate; it yields an out-of-range shift that does not round-trip -- reject.
    if imm6 < result_esize { return None; }
    Some((size, (2 * result_esize - imm6) as u8))
}

// Widening (VSHLL / VMOVL): imm6's leading bit gives the SOURCE element size (8/16/32); the shift amount is
// imm6 - source_esize (0 = VMOVL).
fn decode_neon_widen_shift(imm6: u32) -> Option<(Arm32NeonSize, u8)> {
    let (size, esize) = if imm6 & 0b100000 != 0 {
        (Arm32NeonSize::I32, 32u32)
    } else if imm6 & 0b010000 != 0 {
        (Arm32NeonSize::I16, 16)
    } else {
        (Arm32NeonSize::I8, 8)
    };
    // shift amount = imm6 - source_esize (0 = VMOVL); imm6 < esize is an invalid encoding (reject, don't underflow).
    if imm6 < esize { return None; }
    Some((size, (imm6 - esize) as u8))
}

// VDUP (scalar): the imm4 field jointly encodes the element size and the source lane index.
fn vdup_scalar_imm4(size: Arm32NeonSize, index: u8) -> u32 {
    match size {
        Arm32NeonSize::I8 => ((index as u32) << 1) | 0b1,
        Arm32NeonSize::I16 => ((index as u32) << 2) | 0b10,
        _ => ((index as u32) << 3) | 0b100, // I32 (I64 not valid for VDUP-scalar)
    }
}
fn decode_vdup_scalar_imm4(imm4: u32) -> Option<(Arm32NeonSize, u8)> {
    if imm4 & 0b1 != 0 {
        Some((Arm32NeonSize::I8, (imm4 >> 1) as u8))
    } else if imm4 & 0b10 != 0 {
        Some((Arm32NeonSize::I16, (imm4 >> 2) as u8))
    } else if imm4 & 0b100 != 0 {
        Some((Arm32NeonSize::I32, (imm4 >> 3) as u8))
    } else {
        None
    }
}

// VDUP (from an ARM core register): the (B, E) bit pair encodes the element size (8 -> 10, 16 -> 01, 32 -> 00).
fn vdup_core_be(size: Arm32NeonSize) -> (/*B*/ u32, /*E*/ u32) {
    match size {
        Arm32NeonSize::I8 => (1, 0),
        Arm32NeonSize::I16 => (0, 1),
        _ => (0, 0), // I32
    }
}

// VMOV/VMVN/VORR/VBIC (modified immediate): 1111 001 i 1 D 000 imm3 Vd cmode 0 Q op 1 imm4, imm8 = i:imm3:imm4.
fn encode_neon_modified_imm(cmode: u8, op: bool, imm8: u8, q: u32, d_bit: u32, vd: u32) -> u32 {
    let i = ((imm8 as u32) >> 7) & 1;
    let imm3 = ((imm8 as u32) >> 4) & 0b111;
    let imm4 = (imm8 as u32) & 0xF;
    0xF280_0010 | (i << 24) | (d_bit << 22) | (imm3 << 16) | (vd << 12) | ((cmode as u32 & 0xF) << 8) | (q << 6) | ((op as u32) << 5) | imm4
}

// VCVT (floating-point <-> fixed-point). frac_bits in 1..=size; the field stored is imm5 = size - frac_bits,
// split as imm4 (bits[3:0]) and i (bit[5]). op[18]=to_fixed, U[16]=unsigned, sf[8]=double, sx[7]=bits32.
fn encode_vcvt_fixed_a32(cond: &Arm32Condition, vd_field: u32, d_bit: u32, sf: u32, to_fixed: bool, signed: bool, bits32: bool, frac_bits: u8) -> Result<u32, EncodeError> {
    let size: u32 = if bits32 { 32 } else { 16 };
    if frac_bits < 1 || frac_bits as u32 > size {
        return Err(EncodeError::ImmediateOutOfRange { field: "frac_bits", value: frac_bits as i64, minimum: 1, maximum: size as i64 });
    }
    let imm5 = size - frac_bits as u32;
    let imm4 = (imm5 >> 1) & 0xF;
    let i = imm5 & 1;
    let op = if to_fixed { 1u32 } else { 0 };
    let u = if signed { 0u32 } else { 1 };
    Ok(cond_bits(cond) | 0x0EBA_0A40 | (op << 18) | (u << 16) | (d_bit << 22) | (vd_field << 12) | (sf << 8) | ((bits32 as u32) << 7) | (i << 5) | imm4)
}

// Inverse of `encode_vcvt_fixed_a32` for the operand fields (signed / container width / frac_bits).
fn decode_a32_vcvt_fixed(word: u32) -> Option<(/*signed*/ bool, /*bits32*/ bool, /*frac_bits*/ u8)> {
    let signed = (word >> 16) & 1 == 0;
    let bits32 = (word >> 7) & 1 == 1;
    let size: u32 = if bits32 { 32 } else { 16 };
    let imm5 = ((word & 0xF) << 1) | ((word >> 5) & 1);
    // frac_bits = size - imm5 must be in 1..=size; imm5 >= size is an invalid (non-positive fraction) encoding.
    if imm5 >= size { return None; }
    Some((signed, bits32, (size - imm5) as u8))
}

// A32 (P, W) encoding bits for an index mode (NB: A32 post-index is W=0, unlike T32)
fn a32_index_p_w(index: Arm32IndexMode) -> (u32, u32) {
    match index {
        Arm32IndexMode::Offset => (1, 0),
        Arm32IndexMode::PreIndex => (1, 1),
        Arm32IndexMode::PostIndex => (0, 0),
    }
}
fn index_from_p_w(p: u32, w: u32) -> Arm32IndexMode {
    match (p, w) {
        (1, 0) => Arm32IndexMode::Offset,
        (1, 1) => Arm32IndexMode::PreIndex,
        _ => Arm32IndexMode::PostIndex, // (0, 0)
    }
}

// LDC/STC use a DIFFERENT (P, W) mapping from the integer load/stores: post-index is W=1 (W=0 with P=0 is
// the unindexed/option form, which is not modeled -- it decodes back as the offset form).
fn ldc_index_p_w(index: Arm32IndexMode) -> (u32, u32) {
    match index {
        Arm32IndexMode::Offset => (1, 0),
        Arm32IndexMode::PreIndex => (1, 1),
        Arm32IndexMode::PostIndex => (0, 1),
    }
}
fn ldc_index_from_p_w(p: u32, w: u32) -> Arm32IndexMode {
    match (p, w) {
        (1, 1) => Arm32IndexMode::PreIndex,
        (0, 1) => Arm32IndexMode::PostIndex,
        _ => Arm32IndexMode::Offset, // (1,0) offset; (0,0) unindexed renders as offset
    }
}

// extra load/store (halfword/dual/signed): cccc 000 P U I W L Rn Rt imm4H 1 S H 1 imm4L  (I=1 immediate)
fn encode_extra_load_store(cond: &Arm32Condition, p: u32, w: u32, is_load: bool, s: u32, h: u32, rn: u32, rt: u32, offset: &Arm32MemoryOffset8) -> u32 {
    let op74 = 0x90 | (s << 6) | (h << 5); // 1 S H 1 at bits [7:4]
    let (i_bit, add, imm4h, low4) = match offset {
        Arm32MemoryOffset8::Immediate { add, imm8 } => (1u32, *add as u32, ((*imm8 as u32) >> 4) & 0xF, (*imm8 as u32) & 0xF),
        Arm32MemoryOffset8::Register { add, rm } => (0u32, *add as u32, 0u32, reg(rm)),
    };
    cond_bits(cond)
        | (p << 24)
        | (add << 23)
        | (i_bit << 22)
        | (w << 21)
        | ((is_load as u32) << 20)
        | (rn << 16)
        | (rt << 12)
        | (imm4h << 8)
        | op74
        | low4
}

// load/store multiple: cccc 100 P U S W L Rn register_list16
fn encode_load_store_multiple(cond: &Arm32Condition, mode: Arm32BlockAddressMode, user_mode: bool, writeback: bool, is_load: bool, rn: u32, registers: &[Arm32GeneralPurposeRegister]) -> u32 {
    let (p, u) = mode.p_u_bits();
    let mut list = 0u32;
    for register in registers {
        list |= 1 << register.as_operand_bits();
    }
    cond_bits(cond)
        | 0x0800_0000
        | (p << 24)
        | (u << 23)
        | ((user_mode as u32) << 22)
        | ((writeback as u32) << 21)
        | ((is_load as u32) << 20)
        | (rn << 16)
        | list
}

// recover a register list (ascending) from the 16-bit bitmap
fn decode_register_list(bits: u16) -> Vec<Arm32GeneralPurposeRegister> {
    let mut registers = Vec::new();
    for i in 0..16u8 {
        if (bits >> i) & 1 == 1 {
            registers.push(Arm32GeneralPurposeRegister::from_operand_bits(i));
        }
    }
    registers
}

// ---- coprocessor field packers ----
// MCR/MRC: opc1(3)@21 CRn@16 Rt@12 coproc@8 opc2(3)@5 (bit4=1 is in the base) CRm@0
fn mcr_fields(coproc: u8, opc1: u8, opc2: u8, crn: u8, crm: u8, rt: u32) -> u32 {
    (((opc1 & 0b111) as u32) << 21) | (((crn & 0xF) as u32) << 16) | (rt << 12) | (((coproc & 0xF) as u32) << 8) | (((opc2 & 0b111) as u32) << 5) | ((crm & 0xF) as u32)
}
// CDP: opc1(4)@20 CRn@16 CRd@12 coproc@8 opc2(3)@5 CRm@0
fn cdp_fields(coproc: u8, opc1: u8, opc2: u8, crn: u8, crd: u8, crm: u8) -> u32 {
    (((opc1 & 0xF) as u32) << 20) | (((crn & 0xF) as u32) << 16) | (((crd & 0xF) as u32) << 12) | (((coproc & 0xF) as u32) << 8) | (((opc2 & 0b111) as u32) << 5) | ((crm & 0xF) as u32)
}
// MCRR/MRRC: Rt2@16 Rt@12 coproc@8 opc1(4)@4 CRm@0
fn mcrr_fields(coproc: u8, opc1: u8, rt: u32, rt2: u32, crm: u8) -> u32 {
    (rt2 << 16) | (rt << 12) | (((coproc & 0xF) as u32) << 8) | (((opc1 & 0xF) as u32) << 4) | ((crm & 0xF) as u32)
}
// LDC/STC: the [27:20] opcode bits (P U N W L over the 0x0C000000 base)
fn ldc_base(p: u32, add: bool, long: bool, w: u32, is_load: bool) -> u32 {
    0x0C00_0000 | (p << 24) | ((add as u32) << 23) | ((long as u32) << 22) | (w << 21) | ((is_load as u32) << 20)
}
fn ldc_fields(rn: u32, crd: u8, coproc: u8, imm8: u8) -> u32 {
    (rn << 16) | (((crd & 0xF) as u32) << 12) | (((coproc & 0xF) as u32) << 8) | (imm8 as u32)
}

// SSAT / USAT: cccc 0110 101/111 sat_imm Rd imm5 sh 01 Rm  (sh: 0=LSL, 1=ASR; only LSL/ASR are encodable)
fn encode_saturate(cond: &Arm32Condition, is_usat: bool, sat: u8, rd: u32, rm: u32, shift: &Arm32RegisterShift) -> Result<u32, EncodeError> {
    let (base, sat_imm) = if is_usat {
        check_unsigned_maximum("sat", sat as u32, 31)?;
        (0x06E0_0010, sat as u32)
    } else {
        if !(1..=32).contains(&sat) {
            return Err(EncodeError::ImmediateOutOfRange { field: "sat", value: sat as i64, minimum: 1, maximum: 32 });
        }
        (0x06A0_0010, (sat - 1) as u32)
    };
    let (sh, imm5) = match shift {
        Arm32RegisterShift::Lsl(amount) => {
            check_unsigned_maximum("shift", *amount as u32, 31)?;
            (0u32, *amount as u32)
        },
        Arm32RegisterShift::Asr(amount) => {
            if *amount < 1 || *amount > 32 {
                return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: *amount as i64, minimum: 1, maximum: 32 });
            }
            (1u32, if *amount == 32 { 0 } else { *amount as u32 })
        },
        _ => return Err(EncodeError::ShiftNotEncodable { field: "shift", detail: "SSAT/USAT allow only LSL or ASR" }),
    };
    Ok(cond_bits(cond) | base | (sat_imm << 16) | (rd << 12) | (imm5 << 7) | (sh << 6) | rm)
}

// The ARM "modified immediate" (ARMExpandImm) codec: a 32-bit constant <-> the 12-bit rotation:imm8 field
// (`value = ROR(imm8, 2*rotation)`). Encoding picks the canonical SMALLEST rotation (the GNU `as` choice).
pub(crate) fn encode_a32_modified_immediate(value: u32) -> Option<u16> {
    if value <= 0xFF {
        return Some(value as u16); // rotation 0
    }
    for rotation in 1u32..16 {
        let imm8 = value.rotate_left(2 * rotation); // imm8 = ROL(value, 2r), since value = ROR(imm8, 2r)
        if imm8 <= 0xFF {
            return Some(((rotation << 8) | imm8) as u16);
        }
    }
    None
}
pub(crate) fn decode_a32_modified_immediate(imm12: u16) -> u32 {
    let rotation = ((imm12 >> 8) & 0b1111) as u32;
    let imm8 = (imm12 & 0xFF) as u32;
    imm8.rotate_right(2 * rotation)
}

// A32 immediate-shift operand: imm5 at bits[11:7], the 2-bit type at bits[6:5], bit[4]=0. Decoded amounts
// follow UAL: LSL #0..=31, LSR/ASR #1..=32 (32 encodes as imm5 = 0), ROR #1..=31, and RRX (type ROR,
// imm5 = 0).
fn encode_a32_shift_imm5_type(shift: &Arm32RegisterShift) -> Result<(/*imm5*/ u32, /*type*/ u32), EncodeError> {
    let result = match shift {
        Arm32RegisterShift::Lsl(amount) => {
            if *amount > 31 { return Err(shift_out_of_range(*amount as i64, 0, 31)); }
            (*amount as u32, 0b00)
        },
        Arm32RegisterShift::Lsr(amount) => {
            if *amount < 1 || *amount > 32 { return Err(shift_out_of_range(*amount as i64, 1, 32)); }
            ((*amount as u32) & 0b1_1111, 0b01) // 32 -> imm5 = 0
        },
        Arm32RegisterShift::Asr(amount) => {
            if *amount < 1 || *amount > 32 { return Err(shift_out_of_range(*amount as i64, 1, 32)); }
            ((*amount as u32) & 0b1_1111, 0b10) // 32 -> imm5 = 0
        },
        Arm32RegisterShift::Ror(amount) => {
            if *amount < 1 || *amount > 31 { return Err(shift_out_of_range(*amount as i64, 1, 31)); }
            (*amount as u32, 0b11)
        },
        Arm32RegisterShift::Rrx => (0, 0b11),
    };
    Ok(result)
}

fn decode_a32_shift(imm5: u8, shift_type: u8) -> Arm32RegisterShift {
    match shift_type {
        0b00 => Arm32RegisterShift::Lsl(imm5),
        0b01 => Arm32RegisterShift::Lsr(if imm5 == 0 { 32 } else { imm5 }),
        0b10 => Arm32RegisterShift::Asr(if imm5 == 0 { 32 } else { imm5 }),
        _ => if imm5 == 0 { Arm32RegisterShift::Rrx } else { Arm32RegisterShift::Ror(imm5) },
    }
}

fn shift_out_of_range(value: i64, minimum: i64, maximum: i64) -> EncodeError {
    EncodeError::ImmediateOutOfRange { field: "shift_amount", value, minimum, maximum }
}

fn decode_movw_imm16(word: u32) -> u16 {
    let imm4 = (word >> 16) & 0b1111;
    let imm12 = word & 0x0FFF;
    ((imm4 << 12) | imm12) as u16
}

// ===================== decode helpers (data processing) =====================

// data processing (register form). Returns None for the opcodes/holes that are NOT a data-processing
// instruction at this point: the compare slots (TST/TEQ/CMP/CMN, opcode 8..=11) are only compares when
// S=1 -- with S=0 that space is the miscellaneous instructions (MRS/MSR, BX, CLZ, ...), handled elsewhere.
fn decode_dp_register(cond: Arm32Condition, opcode: u32, set_flags: bool, rn: u32, rd: u32, rm: u32, shift: Arm32RegisterShift) -> Option<ArmA32Instruction> {
    let (rdr, rnr, rmr) = (gpr(rd), gpr(rn), gpr(rm));
    Some(match opcode {
        OP_AND => ArmA32Instruction::And_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_EOR => ArmA32Instruction::Eor_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_SUB => ArmA32Instruction::Sub_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_RSB => ArmA32Instruction::Rsb_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_ADD => ArmA32Instruction::Add_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_ADC => ArmA32Instruction::Adc_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_SBC => ArmA32Instruction::Sbc_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_RSC => ArmA32Instruction::Rsc_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_ORR => ArmA32Instruction::Orr_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_BIC => ArmA32Instruction::Bic_Register_A1(cond, set_flags, rdr, rnr, rmr, shift),
        OP_MOV => ArmA32Instruction::Mov_Register_A1(cond, set_flags, rdr, rmr, shift), // Rn is SBZ
        OP_MVN => ArmA32Instruction::Mvn_Register_A1(cond, set_flags, rdr, rmr, shift),
        OP_TST if set_flags => ArmA32Instruction::Tst_Register_A1(cond, rnr, rmr, shift),
        OP_TEQ if set_flags => ArmA32Instruction::Teq_Register_A1(cond, rnr, rmr, shift),
        OP_CMP if set_flags => ArmA32Instruction::Cmp_Register_A1(cond, rnr, rmr, shift),
        OP_CMN if set_flags => ArmA32Instruction::Cmn_Register_A1(cond, rnr, rmr, shift),
        _ => return None,
    })
}

// data processing (register-shifted register) opcode dispatch (compare slots only when S=1; MOV/MVN ignore Rn)
fn decode_dp_register_shifted(cond: Arm32Condition, opcode: u32, set_flags: bool, rn: u32, rd: u32, rm: u32, shift_type: Arm32ShiftType, rs: u32) -> Option<ArmA32Instruction> {
    let (rdr, rnr, rmr, rsr) = (gpr(rd), gpr(rn), gpr(rm), gpr(rs));
    Some(match opcode {
        OP_AND => ArmA32Instruction::And_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_EOR => ArmA32Instruction::Eor_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_SUB => ArmA32Instruction::Sub_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_RSB => ArmA32Instruction::Rsb_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_ADD => ArmA32Instruction::Add_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_ADC => ArmA32Instruction::Adc_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_SBC => ArmA32Instruction::Sbc_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_RSC => ArmA32Instruction::Rsc_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_ORR => ArmA32Instruction::Orr_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_BIC => ArmA32Instruction::Bic_RegisterShiftedRegister_A1(cond, set_flags, rdr, rnr, rmr, shift_type, rsr),
        OP_MOV => ArmA32Instruction::Mov_RegisterShiftedRegister_A1(cond, set_flags, rdr, rmr, shift_type, rsr),
        OP_MVN => ArmA32Instruction::Mvn_RegisterShiftedRegister_A1(cond, set_flags, rdr, rmr, shift_type, rsr),
        OP_TST if set_flags => ArmA32Instruction::Tst_RegisterShiftedRegister_A1(cond, rnr, rmr, shift_type, rsr),
        OP_TEQ if set_flags => ArmA32Instruction::Teq_RegisterShiftedRegister_A1(cond, rnr, rmr, shift_type, rsr),
        OP_CMP if set_flags => ArmA32Instruction::Cmp_RegisterShiftedRegister_A1(cond, rnr, rmr, shift_type, rsr),
        OP_CMN if set_flags => ArmA32Instruction::Cmn_RegisterShiftedRegister_A1(cond, rnr, rmr, shift_type, rsr),
        _ => return None,
    })
}

fn decode_dp_immediate(cond: Arm32Condition, opcode: u32, set_flags: bool, rn: u32, rd: u32, value: u32) -> Option<ArmA32Instruction> {
    let (rdr, rnr) = (gpr(rd), gpr(rn));
    Some(match opcode {
        OP_AND => ArmA32Instruction::And_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_EOR => ArmA32Instruction::Eor_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_SUB => ArmA32Instruction::Sub_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_RSB => ArmA32Instruction::Rsb_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_ADD => ArmA32Instruction::Add_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_ADC => ArmA32Instruction::Adc_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_SBC => ArmA32Instruction::Sbc_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_RSC => ArmA32Instruction::Rsc_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_ORR => ArmA32Instruction::Orr_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_BIC => ArmA32Instruction::Bic_Immediate_A1(cond, set_flags, rdr, rnr, value),
        OP_MOV => ArmA32Instruction::Mov_Immediate_A1(cond, set_flags, rdr, value), // Rn is SBZ
        OP_MVN => ArmA32Instruction::Mvn_Immediate_A1(cond, set_flags, rdr, value),
        OP_TST if set_flags => ArmA32Instruction::Tst_Immediate_A1(cond, rnr, value),
        OP_TEQ if set_flags => ArmA32Instruction::Teq_Immediate_A1(cond, rnr, value),
        OP_CMP if set_flags => ArmA32Instruction::Cmp_Immediate_A1(cond, rnr, value),
        OP_CMN if set_flags => ArmA32Instruction::Cmn_Immediate_A1(cond, rnr, value),
        // opcode 8..=11 with S=0 is MOVW/MOVT/MSR-immediate/hints (handled by specific patterns), not a compare
        _ => return None,
    })
}

fn next_u32le_from_iter<'a, I>(iter: &mut I, iter_offset: &mut usize) -> Result<Option<u32>, DecodeError> where I: Iterator<Item = &'a u8> {
    let byte0 = match iter.next() {
        Some(value) => *value,
        None => return Ok(None), // EOF; nothing to decode
    };
    *iter_offset += 1;
    //
    let mut bytes = [byte0, 0, 0, 0];
    for slot in bytes.iter_mut().skip(1) {
        match iter.next() {
            Some(value) => { *slot = *value; *iter_offset += 1; },
            None => return Err(DecodeError::IncompleteInstruction), // ran out mid-word
        }
    }

    Ok(Some(u32::from_le_bytes(bytes)))
}
