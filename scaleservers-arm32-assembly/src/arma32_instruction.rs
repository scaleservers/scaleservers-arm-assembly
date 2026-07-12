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
    // register (Rm with an immediate barrel shift), and -- added in a later batch -- register-shifted
    // register. MOV/MVN take no Rn; the compares (TST/TEQ/CMP/CMN) take no Rd and always set flags.

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
