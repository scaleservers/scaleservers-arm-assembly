// Copyright (c) Scaleservers LLC

// `Vec` is not in the `no_std` prelude; pull it from `alloc` (the `vec!` macro comes from the crate-level
// `#[macro_use] extern crate alloc`).
use alloc::vec::Vec;
use crate::{
    ArmDecodeContext,
    DecodeError,
    EncodeError,
};
use crate::enums::{
    ArmT32CpsPrimaskEffect,
    Arm32GeneralPurposeRegister,
    Arm32LowGeneralPurposeRegister,
    Arm32SinglePrecisionRegister,
    Arm32DoublePrecisionRegister,
    Arm32VmovLaneSize,
    Arm32DirectedRound,
    Arm32MveVectorRegister,
    Arm32MveSize,
    Arm32MveFloatSize,
    Arm32MveIntArithOp,
    Arm32MveBitwiseOp,
    Arm32MveFloatArithOp,
    Arm32MveVecScalarIntOp,
    Arm32MveVecScalarFloatOp,
    MVE_INT_SIGNATURE_MASK,
    MVE_BITWISE_SIGNATURE_MASK,
    MVE_FLOAT_SIGNATURE_MASK,
    MVE_VBS_INT_SIGNATURE_MASK,
    MVE_VBS_FLOAT_SIGNATURE_MASK,
    MVE_VDUP_MASK,
    MVE_VDUP_BASE,
    MVE_VIDDUP_BASE, MVE_VIDDUP_MASK,
    MVE_VBRSR_BASE, MVE_VBRSR_MASK,
    MVE_GATHER_SCATTER_BASE, MVE_GATHER_SCATTER_MASK, mve_mem_size_log, mve_mem_size_from_log,
    MVE_GATHER_VBASE_BASE, MVE_GATHER_VBASE_MASK,
    MVE_INTERLEAVE_BASE, MVE_INTERLEAVE_MASK,
    MVE_LCTP_WORD, MVE_VCTP_BASE, MVE_VCTP_MASK, MVE_LOB_DLS_BASE, MVE_LOB_DLS_MASK,
    lob_size_field, lob_size_from_field, lob_branch_hw1, lob_branch_offset,
    mve_vdup_size_bits,
    mve_vdup_size_from_bits,
    Arm32MveShiftImmOp,
    MVE_SHIFT_SIGNATURE_MASK,
    mve_shift_esize,
    mve_shift_size_from_imm6,
    Arm32MveMisc2Op,
    Arm32MveMisc2FloatOp,
    MVE_MISC2_SIGNATURE_MASK,
    MVE_VMVN_REG_MASK,
    MVE_VMVN_REG_BASE,
    mve_misc2_float_size_bits,
    mve_misc2_float_size_from_bits,
    Arm32MveReduceOp,
    MVE_REDUCE_SIGNATURE_MASK,
    Arm32MveFloatReduceOp,
    MVE_FLOAT_REDUCE_SIGNATURE_MASK,
    MVE_VABAV_SIGNATURE_MASK,
    MVE_VABAV_BASE,
    MVE_DUALMAC_BASE, MVE_DUALMAC_MASK, mve_dualmac_size_bits, mve_dualmac_decode_size,
    Arm32MveLongMacOp, MVE_LONG_DUALMAC_BASE, MVE_LONG_DUALMAC_MASK, mve_long_dualmac_bits, mve_long_dualmac_decode,
    Arm32MveVrintOp,
    MVE_VCVT_FI_BASE,
    MVE_VCVT_FI_FIXED_MASK,
    MVE_VCVT_FI_FIXED_PATTERN,
    MVE_VCVTR_BASE,
    MVE_VCVTR_MASK,
    MVE_VCVT_FIXED_BASE,
    MVE_VCVT_FIXED_MASK,
    MVE_VCVT_HALF_BASE, MVE_VCVT_HALF_MASK, MVE_SHIFT_NARROW_BASE, MVE_SHIFT_NARROW_MASK,
    Arm32MveShiftNarrowOp,
    MVE_VMOVL_BASE,
    MVE_VMOVL_MASK,
    MVE_VMOVN_BASE,
    MVE_VMOVN_MASK,
    MVE_VADDLV_BASE,
    MVE_VADDLV_MASK,
    MVE_VQMOVN_BASE, MVE_VQMOVN_MASK, MVE_VQMOVUN_BASE, MVE_VQMOVUN_MASK, Arm32MveQMovnKind,
    MVE_VMULL_INT_BASE, MVE_VMULL_INT_MASK, MVE_VMULL_POLY_BASE, MVE_VMULL_POLY_MASK,
    MVE_VMULH_BASE, MVE_VMULH_MASK,
    MVE_VQDMULL_VEC_BASE, MVE_VQDMULL_VEC_MASK, MVE_VQDMULL_SCALAR_BASE, MVE_VQDMULL_SCALAR_MASK,
    MVE_VQDMLADH_BASE, MVE_VQDMLADH_MASK,
    MVE_SHIFT_VEC_BASE, MVE_SHIFT_VEC_MASK, MVE_SHIFT_SCALAR_BASE, MVE_SHIFT_SCALAR_MASK,
    MVE_VSHLL_T1_BASE, MVE_VSHLL_T1_MASK, MVE_VSHLL_T2_BASE, MVE_VSHLL_T2_MASK,
    MVE_VMOVX_BASE, MVE_VMOVX_MASK,
    MVE_VCADD_INT_MASK,
    MVE_VCADD_INT_PATTERN,
    MVE_VCADD_FLOAT_MASK,
    MVE_VCADD_FLOAT_PATTERN,
    MVE_VCMUL_MASK,
    MVE_VCMUL_PATTERN,
    MVE_VCMLA_MASK,
    MVE_VCMLA_PATTERN,
    MVE_VPSEL_BASE,
    MVE_VPSEL_MASK,
    MVE_VPNOT_WORD,
    MVE_VADC_BASE, MVE_VADC_MASK, MVE_VSHLC_BASE, MVE_VSHLC_MASK,
    MVE_VPST_NOT_BASE,
    MVE_VPST_NOT_MASK,
    mve_predicate_mask_bits,
    mve_predicate_mask_from_word,
    mve_predicate_mask_suffix,
    Arm32MveVcmpCondition,
    MVE_VCMP_INT_BASE,
    MVE_VCMP_INT_MASK,
    MVE_VCMP_FLOAT_BASE,
    MVE_VCMP_FLOAT_MASK,
    mve_vcmp_fc_bits,
    mve_vcmp_fc_from_word,
    ArmT32FpDataOperation3,
    ArmT32FpDataOperation2,
    ArmT32IndexMode,
    ArmT32InstructionCondition,
    ArmT32MemoryBarrierOption,
    ArmT32OpcodePattern_16Bit,
    ArmT32OpcodePattern_32Bit,
    ArmT32ParallelOperation,
    ArmT32ParallelPrefix,
    ArmT32RegisterShift,
    ArmT32SpecialRegister,
};
use crate::targets::{
    ArmCpuFeature,
    ArmInstructionRequirement,
    ArmIsaVersion,
    ArmTargetProfile,
};
use crate::utils::{
    ArmT32InstructionDecoder,
    ArmT32InstructionEncoder,
};
use crate::utils::gpr_coding_utils;
use crate::utils::sign_extension_utils;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum ArmT32Instruction {
    Adc_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Add_Immediate_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*imm3: u3*/u8),
    Add_Immediate_T2(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*imm8: u8*/u8),
    Add_Register_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Add_Register_T2(/*rdn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),
    Add_SpPlusImmediate_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*const10: u10*/u16),
    Add_SpPlusImmediate_T2(/*const9: u9*/u16),
    Add_SpPlusRegister_T1(/*rdm: u4*/Arm32GeneralPurposeRegister),
    Add_SpPlusRegister_T2(/*rm: u4*/Arm32GeneralPurposeRegister),
    Adr_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*const10: u10*/u16),
    And_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Asr_Immediate_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm5: u8*/u8),
    Asr_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    B_T1(/*cond: u4*/ArmT32InstructionCondition, /*decoded_signed_imm9: i9*/i16),
    B_T2(/*decoded_signed_imm12: i12*/i16),
    Bic_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Bl_T1(/*decoded_signed_imm25: i25*/i32),
    Bkpt_T1(/*imm8: u8*/u8),
    Blx_Register_T1(/*rm: u4*/Arm32GeneralPurposeRegister),
    Bx_T1(/*rm: u4*/Arm32GeneralPurposeRegister),
    Cmn_Register_T1(/*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Cmp_Immediate_T1(/*rn: u3*/Arm32LowGeneralPurposeRegister, /*imm8: u8*/u8),
    Cmp_Register_T1(/*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Cmp_Register_T2(/*rn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),
    Cps_T1(/*primask_effect: u1*/ArmT32CpsPrimaskEffect),
    Dmb_T1(/*option: u4*/ArmT32MemoryBarrierOption),
    Dsb_T1(/*option: u4*/ArmT32MemoryBarrierOption),
    Eor_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Isb_T1(/*option: u4*/ArmT32MemoryBarrierOption),
    Ldm_T1(/*rn: u3*/Arm32LowGeneralPurposeRegister, /*registers: */Vec</*u3*/Arm32LowGeneralPurposeRegister>),
    Ldr_Immediate_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm7: u7*/u8),
    Ldr_Immediate_T2(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm10: u10*/u16),
    Ldr_Literal_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm10: u10*/u16),
    Ldr_Register_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Ldrb_Immediate_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*imm5: u5*/u8),
    Ldrb_Register_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Ldrh_Immediate_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm6: u6*/u8),
    Ldrh_Register_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Ldrsb_Register_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Ldrsh_Register_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Lsl_Immediate_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister, /*imm5: u5*/u8),
    Lsl_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Lsr_Immediate_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm5: u8*/u8),
    Lsr_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Mov_Immediate_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*imm8: u8*/u8),
    Mov_Register_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),
    Mov_Register_T2(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Mrs_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*spec_reg: u8*/ArmT32SpecialRegister),
    Msr_Register_T1(/*spec_reg: u8*/ArmT32SpecialRegister, /*rn: u4*/Arm32GeneralPurposeRegister),
    Mul_T1(/*rdm: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister),
    Mvn_Register_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Nop_T1,
    Orr_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Pop_T1(/*registers: */Vec</*u4*/Arm32GeneralPurposeRegister>),
    Push_T1(/*registers: */Vec</*u4*/Arm32GeneralPurposeRegister>),
    Rev_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Rev16_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Revsh_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Ror_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Rsb_Immediate_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister/*, imm5: u5 = 0*/),
    Sbc_Register_T1(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Sev_T1,
    Stm_T1(/*rn: u3*/Arm32LowGeneralPurposeRegister, /*registers: */Vec</*u3*/Arm32LowGeneralPurposeRegister>),
    Str_Immediate_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm7: u7*/u8),
    Str_Immediate_T2(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm10: u10*/u16),
    Str_Register_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Strb_Immediate_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*imm5: u5*/u8),
    Strb_Register_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Strh_Immediate_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm6: u6*/u8),
    Strh_Register_T1(/*rt: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Sub_Immediate_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*imm3: u3*/u8),
    Sub_Immediate_T2(/*rdn: u3*/Arm32LowGeneralPurposeRegister, /*imm8: u8*/u8),
    Sub_Register_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Sub_SpMinusImmediate_T1(/*const9: u9*/u16),
    Svc_T1(/*imm8: u8*/u8),
    Sxtb_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Sxth_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Tst_Register_T1(/*rn: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Udf_T1(/*imm8: u8*/u8),
    Udf_T2(/*imm16: u16*/u16),
    Uxtb_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Uxth_T1(/*rd: u3*/Arm32LowGeneralPurposeRegister, /*rm: u3*/Arm32LowGeneralPurposeRegister),
    Wfe_T1,
    Wfi_T1,
    Yield_T1,

    // ---- ARMv7-M (Thumb-2) additions (gated to ArmIsaVersion::Armv7M via requirement()) ----
    Mov_Immediate_T3(/*rd: u4*/Arm32GeneralPurposeRegister, /*imm16: u16*/u16), // MOVW (move wide, 16-bit immediate)
    Movt_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*imm16: u16*/u16),          // MOVT (move top)
    Mul_T2(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),
    Mla_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister, /*ra: u4*/Arm32GeneralPurposeRegister),
    Mls_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister, /*ra: u4*/Arm32GeneralPurposeRegister),
    Sdiv_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),
    Udiv_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),
    Clz_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),

    // ---- ARMv7-M batch M7b: bitfield + RBIT + wide load/store ----
    Ubfx_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*lsb: u5*/u8, /*width: 1..=32*/u8),
    Sbfx_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*lsb: u5*/u8, /*width: 1..=32*/u8),
    Bfi_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*lsb: u5*/u8, /*width: 1..=32*/u8),
    Bfc_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*lsb: u5*/u8, /*width: 1..=32*/u8),
    Rbit_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),
    Ldr_Immediate_T3(/*rt: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*imm12: u12*/u16), // LDR.W
    Str_Immediate_T3(/*rt: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*imm12: u12*/u16), // STR.W

    // ---- ARMv7-M batch M7c: synchronization (exclusive access) + table branch ----
    Ldrex_T1(/*rt: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*imm: 0..=1020, mult 4*/u16),
    Strex_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rt: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*imm: 0..=1020, mult 4*/u16),
    /// LDA/LDAB/LDAH (`exclusive`=false) and LDAEX/LDAEXB/LDAEXH (`exclusive`=true) -- ARMv8-M load-acquire
    /// (`Rt, [Rn]`). `size`: 0 = byte, 1 = halfword, 2 = word.
    LoadAcquire_T1(/*size*/u8, /*exclusive*/bool, /*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister),
    /// STL/STLB/STLH -- ARMv8-M store-release (`Rt, [Rn]`). `size`: 0 = byte, 1 = halfword, 2 = word.
    StoreRelease_T1(/*size*/u8, /*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister),
    /// STLEX/STLEXB/STLEXH -- ARMv8-M store-release exclusive (`Rd, Rt, [Rn]`); `Rd` receives the status.
    StoreReleaseExclusive_T1(/*size*/u8, /*rd*/Arm32GeneralPurposeRegister, /*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister),
    /// Unprivileged load/store (`Rt, [Rn, #imm8]`): LDRT/LDRBT/LDRHT/LDRSBT/LDRSHT (load) and STRT/STRBT/STRHT
    /// (store). `signed` = sign-extending load (only valid for byte/half loads); `size`: 0 = byte, 1 = half, 2 = word.
    UnprivLoadStore_T1(/*load*/bool, /*signed*/bool, /*size*/u8, /*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*imm8*/u8),
    Ldrexb_T1(/*rt: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister),
    Strexb_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rt: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister),
    Ldrexh_T1(/*rt: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister),
    Strexh_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rt: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister),
    Clrex_T1,
    Tbb_T1(/*rn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),
    Tbh_T1(/*rn: u4*/Arm32GeneralPurposeRegister, /*rm: u4*/Arm32GeneralPurposeRegister),

    // ---- ARMv7-M batch M7d: data processing (modified immediate / ThumbExpandImm), the `.W` forms ----
    // The trailing `u32` is the LOGICAL 32-bit constant; encode() packs it via ThumbExpandImm (and errors
    // with ModifiedImmediateNotEncodable if it is not representable). `set_flags` is the `S` bit.
    Mov_Immediate_T2(/*rd: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),  // MOV{S}.W (ORR, Rn=PC)
    Mvn_Immediate_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),  // MVN{S}   (ORN, Rn=PC)
    And_Immediate_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Bic_Immediate_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Orr_Immediate_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Eor_Immediate_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Add_Immediate_T3(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Sub_Immediate_T3(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Tst_Immediate_T1(/*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32),  // AND, Rd=PC, S=1
    Teq_Immediate_T1(/*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32),  // EOR, Rd=PC, S=1
    Cmn_Immediate_T1(/*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32),  // ADD, Rd=PC, S=1
    Cmp_Immediate_T2(/*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32),  // SUB, Rd=PC, S=1

    // ---- ARMv7-M batch M7e: the rest of the modified-immediate family (no narrow form -> emitted without `.w`) ----
    Adc_Immediate_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Sbc_Immediate_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Rsb_Immediate_T2(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),
    Orn_Immediate_T1(/*rd: u4*/Arm32GeneralPurposeRegister, /*rn: u4*/Arm32GeneralPurposeRegister, /*const*/u32, /*set_flags*/bool),

    // ---- ARMv7-M batch M7f: data processing (shifted register), the `.W` forms (Rm may carry a shift) ----
    Add_Register_T3(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Sub_Register_T2(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    And_Register_T2(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Orr_Register_T2(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Eor_Register_T2(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Bic_Register_T2(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),

    // ---- ARMv7-M batch M7g: shifted-register ALIAS forms (Rn==PC and Rd==PC,S cases) + ADC/SBC/RSB/ORN reg ----
    // MOV (register) and the shift mnemonics (LSL/LSR/ASR/ROR register-by-immediate) and RRX are ONE
    // encoding (ORR, Rn=PC), distinguished by the `shift`; emit picks the canonical mnemonic.
    Mov_Register_T3(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Mvn_Register_T2(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool), // ORN, Rn=PC
    Adc_Register_T2(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Sbc_Register_T2(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Rsb_Register_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Orn_Register_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift, /*set_flags*/bool),
    Tst_Register_T2(/*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift), // AND, Rd=PC, S=1
    Teq_Register_T1(/*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift), // EOR, Rd=PC, S=1
    Cmn_Register_T2(/*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift), // ADD, Rd=PC, S=1
    Cmp_Register_T3(/*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift), // SUB, Rd=PC, S=1

    // ---- ARMv7-M batch M7h: wide byte/halfword load/store (imm12) + register-offset load/store ----
    // immediate offset (imm12): Rt, [Rn, #imm12]
    Ldrb_Immediate_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*imm12*/u16),
    Strb_Immediate_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*imm12*/u16),
    Ldrh_Immediate_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*imm12*/u16),
    Strh_Immediate_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*imm12*/u16),
    Ldrsb_Immediate_T1(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*imm12*/u16),
    Ldrsh_Immediate_T1(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*imm12*/u16),
    // register offset (LSL #imm2, 0..=3): Rt, [Rn, Rm{, lsl #shift}]
    Ldr_Register_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=3*/u8),
    Str_Register_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=3*/u8),
    Ldrb_Register_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=3*/u8),
    Strb_Register_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=3*/u8),
    Ldrh_Register_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=3*/u8),
    Strh_Register_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=3*/u8),
    Ldrsb_Register_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=3*/u8),
    Ldrsh_Register_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=3*/u8),

    // ---- ARMv7-M batch M7k: long multiply (RdLo, RdHi, Rn, Rm) ----
    Smull_T1(/*rdlo*/Arm32GeneralPurposeRegister, /*rdhi*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    Umull_T1(/*rdlo*/Arm32GeneralPurposeRegister, /*rdhi*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    Smlal_T1(/*rdlo*/Arm32GeneralPurposeRegister, /*rdhi*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    Umlal_T1(/*rdlo*/Arm32GeneralPurposeRegister, /*rdhi*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    /// UMAAL -- unsigned multiply accumulate accumulate long: RdHi:RdLo = Rn*Rm + RdLo + RdHi.
    Umaal_T1(/*rdlo*/Arm32GeneralPurposeRegister, /*rdhi*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),

    // ---- ARMv7-M batch M7l: wide extend (with ROR), wide byte-reverse, and saturate ----
    Sxtb_T2(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation: 0/8/16/24*/u8),
    Uxtb_T2(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation: 0/8/16/24*/u8),
    Sxth_T2(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation: 0/8/16/24*/u8),
    Uxth_T2(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation: 0/8/16/24*/u8),
    Rev_T2(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    Rev16_T2(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    Revsh_T2(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    // SSAT Rd, #sat(1..=32), Rn{, shift}; USAT Rd, #sat(0..=31), Rn{, shift}. Shift is LSL (0..=31) or ASR (1..=31).
    Ssat_T1(/*rd*/Arm32GeneralPurposeRegister, /*sat_imm*/u8, /*rn*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift),
    Usat_T1(/*rd*/Arm32GeneralPurposeRegister, /*sat_imm*/u8, /*rn*/Arm32GeneralPurposeRegister, /*shift*/ArmT32RegisterShift),

    // ---- ARMv7-M batch M7i: indexed (T4/T3/T2) load/store, LDRD/STRD, wide literal loads, preload ----
    // Single-register indexed forms: `Rt, [Rn, #+/-imm8]` (offset) / `[Rn, #+/-imm8]!` (pre) / `[Rn], #+/-imm8` (post).
    // The positive-offset, no-writeback case is the imm12 T3/T2 form already modeled; this 8-bit form carries
    // the signed offset (-255..=255) and the P/W index mode. (Offset mode here is the negative-offset case.)
    Ldr_Immediate_T4(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    Str_Immediate_T4(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    Ldrb_Immediate_T3(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    Strb_Immediate_T3(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    Ldrh_Immediate_T3(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    Strh_Immediate_T3(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    Ldrsb_Immediate_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    Ldrsh_Immediate_T2(/*rt*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    // Dual-register: `Rt, Rt2, [Rn, #+/-(imm8*4)]` / `[Rn, #+/-(imm8*4)]!` / `[Rn], #+/-(imm8*4)`. Offset is a
    // multiple of 4 in -1020..=1020.
    Ldrd_Immediate_T1(/*rt*/Arm32GeneralPurposeRegister, /*rt2*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    Strd_Immediate_T1(/*rt*/Arm32GeneralPurposeRegister, /*rt2*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i16, ArmT32IndexMode),
    // PC-relative literal loads: `Rt, [pc, #+/-imm12]`. Offset is -4095..=4095 (the U bit carries the sign).
    Ldr_Literal_T2(/*rt*/Arm32GeneralPurposeRegister, /*offset*/i32),
    Ldrb_Literal_T1(/*rt*/Arm32GeneralPurposeRegister, /*offset*/i32),
    Ldrh_Literal_T1(/*rt*/Arm32GeneralPurposeRegister, /*offset*/i32),
    Ldrsb_Literal_T1(/*rt*/Arm32GeneralPurposeRegister, /*offset*/i32),
    Ldrsh_Literal_T1(/*rt*/Arm32GeneralPurposeRegister, /*offset*/i32),
    // Preload hints: `PLD/PLI [Rn, #+/-imm]`. A positive offset uses the imm12 form, a negative offset the imm8
    // form, so the modeled range is -255..=4095.
    Pld_Immediate_T1(/*rn*/Arm32GeneralPurposeRegister, /*offset*/i32),
    Pli_Immediate_T1(/*rn*/Arm32GeneralPurposeRegister, /*offset*/i32),

    // ---- ARMv7-M batch M7j: wide load/store multiple (LDM.W/STM.W/LDMDB/STMDB; PUSH.W/POP.W are the
    //      SP-with-writeback spellings). `writeback` is the `!` after Rn; the list is a 16-bit set R0..PC. ----
    Ldmia_T2(/*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool, /*registers*/Vec<Arm32GeneralPurposeRegister>),
    Stmia_T2(/*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool, /*registers*/Vec<Arm32GeneralPurposeRegister>),
    Ldmdb_T1(/*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool, /*registers*/Vec<Arm32GeneralPurposeRegister>),
    Stmdb_T1(/*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool, /*registers*/Vec<Arm32GeneralPurposeRegister>),

    // ---- ARMv7-M batch M7m: wide branches (B.W +/-16MB, B<c>.W +/-1MB) and compare-and-branch (forward 0..126).
    //      Offsets are the DECODED byte displacement from the branch (PC = address + 4), as with Bl_T1. ----
    B_T4(/*decoded_signed_imm25*/i32),
    B_T3(/*cond*/ArmT32InstructionCondition, /*decoded_signed_imm21*/i32),
    Cbz_T1(/*rn: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm7: 0..=126, even*/u8),
    Cbnz_T1(/*rn: u3*/Arm32LowGeneralPurposeRegister, /*decoded_imm7: 0..=126, even*/u8),

    // ---- ARMv7-M batch M7n: IT (If-Then). Makes the next 1..=4 instructions conditional. `mask` is the
    //      raw 4-bit field (1..=15): its lowest set bit marks the block length, the bits above encode the
    //      then/else pattern of slots 2..N relative to firstcond[0] (see the emitter / assembler). ----
    It_T1(/*firstcond*/ArmT32InstructionCondition, /*mask: u4, nonzero*/u8),

    // ==== ARMv7E-M DSP extension (gated Armv7EM + DspExtension) ====
    // ---- M8a: saturating arithmetic. UAL operand order is `Rd, Rm, Rn`. ----
    Qadd_T1(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister),
    Qsub_T1(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister),
    Qdadd_T1(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister),
    Qdsub_T1(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister),

    // ---- M8b: extend-and-add (`Rd, Rn, Rm{, ROR #r}`) and the 16-bit byte extends ----
    Sxtab_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation*/u8),
    Uxtab_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation*/u8),
    Sxtah_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation*/u8),
    Uxtah_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation*/u8),
    Sxtab16_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation*/u8),
    Uxtab16_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation*/u8),
    Sxtb16_T1(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation*/u8),
    Uxtb16_T1(/*rd*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*rotation*/u8),

    // ---- M8c: pack halfword, 16-bit saturate, select, sum-of-absolute-differences ----
    Pkhbt_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*lsl: 0..=31*/u8),
    Pkhtb_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*asr: 1..=31*/u8),
    Ssat16_T1(/*rd*/Arm32GeneralPurposeRegister, /*sat_imm: 1..=16*/u8, /*rn*/Arm32GeneralPurposeRegister),
    Usat16_T1(/*rd*/Arm32GeneralPurposeRegister, /*sat_imm: 0..=15*/u8, /*rn*/Arm32GeneralPurposeRegister),
    Sel_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    Usad8_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    Usada8_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*ra*/Arm32GeneralPurposeRegister),

    // ---- M8d: parallel (packed SIMD) add/subtract -- 36 forms via operation x prefix (`Rd, Rn, Rm`) ----
    ParallelAddSub_T1(ArmT32ParallelOperation, ArmT32ParallelPrefix, /*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),

    // ---- M8e: signed multiplies. The BB/BT/TB/TT halves are `n`(top of Rn) / `m`(top of Rm) booleans;
    //      the dual forms carry `x`(cross) and the most-significant-word forms carry `round`. ----
    Smul_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*n*/bool, /*m*/bool),
    Smulw_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*m*/bool),
    Smla_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*ra*/Arm32GeneralPurposeRegister, /*n*/bool, /*m*/bool),
    Smlaw_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*ra*/Arm32GeneralPurposeRegister, /*m*/bool),
    Smlal_Halfword_T1(/*rdlo*/Arm32GeneralPurposeRegister, /*rdhi*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*n*/bool, /*m*/bool),
    Smuad_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*x*/bool),
    Smusd_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*x*/bool),
    Smlad_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*ra*/Arm32GeneralPurposeRegister, /*x*/bool),
    Smlsd_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*ra*/Arm32GeneralPurposeRegister, /*x*/bool),
    Smlald_T1(/*rdlo*/Arm32GeneralPurposeRegister, /*rdhi*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*x*/bool),
    Smlsld_T1(/*rdlo*/Arm32GeneralPurposeRegister, /*rdhi*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*x*/bool),
    Smmul_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*round*/bool),
    Smmla_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*ra*/Arm32GeneralPurposeRegister, /*round*/bool),
    Smmls_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*ra*/Arm32GeneralPurposeRegister, /*round*/bool),

    // ==== ARMv7E-M floating-point (FPv4-SP / FPv5; gated FloatingPoint feature) ====
    // ---- M8f: FP load/store (`Sd/Dd, [Rn, #+/-(imm8*4)]`). Offset is a multiple of 4 in -1020..=1020. ----
    Vldr_Single_T2(/*sd*/Arm32SinglePrecisionRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i32),
    Vstr_Single_T2(/*sd*/Arm32SinglePrecisionRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i32),
    Vldr_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i32),
    Vstr_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i32),

    // ---- M8g: FP load/store multiple. A contiguous register range (`first`..`first+count-1`), IA
    //      (decrement_before=false) or DB. VPUSH/VPOP are the SP-with-writeback DB-store / IA-load forms. ----
    Vldm_Single_T2(/*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool, /*decrement_before*/bool, /*first*/Arm32SinglePrecisionRegister, /*count*/u8),
    Vstm_Single_T2(/*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool, /*decrement_before*/bool, /*first*/Arm32SinglePrecisionRegister, /*count*/u8),
    Vldm_Double_T1(/*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool, /*decrement_before*/bool, /*first*/Arm32DoublePrecisionRegister, /*count*/u8),
    Vstm_Double_T1(/*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool, /*decrement_before*/bool, /*first*/Arm32DoublePrecisionRegister, /*count*/u8),
    /// FLDMDBX / FSTMDBX -- deprecated FP load/store multiple, decrement-before with writeback and the extra
    /// (X) word (`imm8 = 2*count + 1`). `load` selects FLDMDBX. `Rn!, {Dd-...}`.
    FldmdbxFstmdbx_T1(/*load*/bool, /*rn*/Arm32GeneralPurposeRegister, /*first*/Arm32DoublePrecisionRegister, /*count*/u8),
    /// MCR/MCR2/MRC/MRC2 -- move between a core register and a coprocessor. `two` = the `2` variant, `load` = MRC.
    Coproc_Mcr_T1(/*two*/bool, /*load*/bool, /*coproc*/u8, /*opc1*/u8, /*rt*/Arm32GeneralPurposeRegister, /*crn*/u8, /*crm*/u8, /*opc2*/u8),
    /// CDP/CDP2 -- coprocessor data processing.
    Coproc_Cdp_T1(/*two*/bool, /*coproc*/u8, /*opc1*/u8, /*crd*/u8, /*crn*/u8, /*crm*/u8, /*opc2*/u8),
    /// MCRR/MCRR2/MRRC/MRRC2 -- move between a core register pair and a coprocessor. `load` = MRRC.
    Coproc_Mcrr_T1(/*two*/bool, /*load*/bool, /*coproc*/u8, /*opc1*/u8, /*rt*/Arm32GeneralPurposeRegister, /*rt2*/Arm32GeneralPurposeRegister, /*crm*/u8),
    /// LDC/LDC2/STC/STC2 (+ `L` long) -- coprocessor load/store, `CRd, [Rn, #+/-imm]` (offset form, P=1/W=0).
    Coproc_Ldc_T1(/*two*/bool, /*long*/bool, /*load*/bool, /*coproc*/u8, /*crd*/u8, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i32),
    /// PACBTI hint-space instructions (no operands -- the architectural defaults R12/LR/SP are implicit).
    /// `kind`: 0 = BTI, 1 = PAC, 2 = AUT, 3 = PACBTI.
    PacbtiHint_T1(/*kind*/u8),
    /// PACG/AUTG/BXAUT -- PACBTI data-processing (`Rd, Rn, Rm`). `op`: 0 = PACG, 1 = AUTG, 2 = BXAUT.
    PacbtiData_T1(/*op*/u8, /*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    /// VSCCLRM -- FP secure context clear multiple: zeros VPR and a contiguous FP register range. `double`
    /// selects the D-register list; `first` is the first register number; `count` is the raw imm8 (0 = `{vpr}`
    /// only; for the double list it is 2xthe number of D registers).
    Vscclrm_T1(/*double*/bool, /*first*/u8, /*count*/u8),
    /// CDE (Custom Datapath Extension) CX1/CX1A/CX1D/CX1DA -- `p<coproc>, Rd{, Ra}, #imm`. `acc` = the A
    /// (accumulate) form, `dual` = the D (64-bit Rd:Rd+1) form. `imm` is the 13-bit custom immediate.
    Cde_Cx1_T1(/*acc*/bool, /*dual*/bool, /*coproc*/u8, /*rd*/Arm32GeneralPurposeRegister, /*imm*/u16),
    /// CDE CX2/CX2A/CX2D/CX2DA -- `p<coproc>, Rd{, Ra}, Rn, #imm` (9-bit immediate).
    Cde_Cx2_T1(/*acc*/bool, /*dual*/bool, /*coproc*/u8, /*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*imm*/u16),
    /// CDE CX3/CX3A/CX3D/CX3DA -- `p<coproc>, Rd{, Ra}, Rn, Rm, #imm` (6-bit immediate).
    Cde_Cx3_T1(/*acc*/bool, /*dual*/bool, /*coproc*/u8, /*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*imm*/u8),

    // ---- Branch Future (Armv8.1-M Low Overhead Branch Extension): BF/BFL/BFX/BFLX/BFCSEL. These set up a
    // future branch from <b_label> (the branch point) to <label>. `boff` is the raw 4-bit b_label field
    // (1..=15): b_label = bf_addr + 4 + 2*boff. `offset` is the decoded signed byte displacement of the
    // target from PC (= bf_addr + 4), exactly as for B_T4/Bl_T1. ----
    /// BF (T1): `bf <b_label>, <label>`. Plain branch future. 17-bit (+/-64KiB) target range.
    Bf_T1(/*boff*/u8, /*offset*/i32),
    /// BFL (T4): `bfl <b_label>, <label>`. Branch future with link.
    Bfl_T4(/*boff*/u8, /*offset*/i32),
    /// BFX (T3): `bfx <b_label>, Rn`. Branch future and exchange to a register target.
    Bfx_T3(/*boff*/u8, /*rn*/Arm32GeneralPurposeRegister),
    /// BFLX (T5): `bflx <b_label>, Rn`. Branch future with link and exchange.
    Bflx_T5(/*boff*/u8, /*rn*/Arm32GeneralPurposeRegister),
    /// BFCSEL (T2): `bfcsel <b_label>, <label>, <ba_label>, <cond>`. Conditional-select branch future:
    /// targets `<label>` if `cond` passes, else the fall-through `<ba_label>` selected by `t`. 13-bit target
    /// range. `cond` is the 4-bit condition; `t` is the T bit (selects the ba_label fall-through distance).
    Bfcsel_T2(/*boff*/u8, /*offset*/i32, /*cond*/u8, /*t*/bool),

    // ---- VCX1/VCX2/VCX3 -- CDE custom-datapath with Floating-point/vector registers. `acc` = the A
    // (accumulate) form; `kind` selects the register file: 0 = single (Sx), 1 = double (Dx), 2 = vector
    // (Qx, MVE). `rd`/`rn`/`rm` are raw register numbers. The immediate is 11/6/3 bits for VCX1/2/3. ----
    /// VCX1/VCX1A: `p<coproc>, <Sd|Dd|Qd>, #imm`.
    Vcx1_T1(/*acc*/bool, /*kind*/u8, /*coproc*/u8, /*rd*/u8, /*imm*/u16),
    /// VCX2/VCX2A: `p<coproc>, <Sd|Dd|Qd>, <Sn|Dn|Qn>, #imm`.
    Vcx2_T1(/*acc*/bool, /*kind*/u8, /*coproc*/u8, /*rd*/u8, /*rn*/u8, /*imm*/u8),
    /// VCX3/VCX3A: `p<coproc>, <Sd|Dd|Qd>, <Sn|Dn|Qn>, <Sm|Dm|Qm>, #imm`.
    Vcx3_T1(/*acc*/bool, /*kind*/u8, /*coproc*/u8, /*rd*/u8, /*rn*/u8, /*rm*/u8, /*imm*/u8),

    // ---- M8h: FP data-processing (`Vd, Vn, Vm` for 3-operand; `Vd, Vm` for the 2-operand "other" group) ----
    FpDataProcess3_Single(ArmT32FpDataOperation3, /*sd*/Arm32SinglePrecisionRegister, /*sn*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    FpDataProcess3_Double(ArmT32FpDataOperation3, /*dd*/Arm32DoublePrecisionRegister, /*dn*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    FpDataProcess2_Single(ArmT32FpDataOperation2, /*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    FpDataProcess2_Double(ArmT32FpDataOperation2, /*dd*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),

    // ---- M8i: FP compare, FPSCR transfer, and core<->FP register moves. `e` = the signalling (E) compare. ----
    Vcmp_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister, /*e*/bool),
    Vcmp_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister, /*e*/bool),
    Vcmp_Zero_Single_T2(/*sd*/Arm32SinglePrecisionRegister, /*e*/bool),
    Vcmp_Zero_Double_T2(/*dd*/Arm32DoublePrecisionRegister, /*e*/bool),
    Vmrs_T1(/*rt*/Arm32GeneralPurposeRegister),  // VMRS Rt, FPSCR
    Vmrs_Apsr_Nzcv_T1,                            // VMRS APSR_nzcv, FPSCR
    Vmsr_T1(/*rt*/Arm32GeneralPurposeRegister),  // VMSR FPSCR, Rt
    Vmov_Core_To_Single_T1(/*sn*/Arm32SinglePrecisionRegister, /*rt*/Arm32GeneralPurposeRegister),  // VMOV Sn, Rt
    Vmov_Single_To_Core_T1(/*rt*/Arm32GeneralPurposeRegister, /*sn*/Arm32SinglePrecisionRegister),  // VMOV Rt, Sn
    /// `VMOV.<8|16|32> Dd[x], Rt` -- copy a general-purpose register into a scalar lane of a doubleword. base
    /// `0xEE00_0B10`; width + lane `index` pack into opc1`[22:21]`/opc2`[6:5]`. See [`Arm32VmovLaneSize`].
    Vmov_Core_To_Scalar_T1(Arm32VmovLaneSize, /*index*/u8, /*dd*/Arm32DoublePrecisionRegister, /*rt*/Arm32GeneralPurposeRegister),
    /// `VMOV.<dt> Rt, Dn[x]` -- copy a scalar lane into a general-purpose register; `unsigned` picks `.u8`/`.u16`
    /// over `.s8`/`.s16` (`.32` ignores it). base `0xEE10_0B10`; U at `[23]`.
    Vmov_Scalar_To_Core_T1(/*unsigned*/bool, Arm32VmovLaneSize, /*index*/u8, /*rt*/Arm32GeneralPurposeRegister, /*dn*/Arm32DoublePrecisionRegister),

    // ---- M8i VCVT: the standard conversions. `signed` selects S32 vs U32; `round_to_zero` is VCVT (true)
    //      vs VCVTR (false, round per FPSCR). The integer always lives in a single-precision register. ----
    Vcvt_FloatToInt_FromSingle_T1(/*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister, /*signed*/bool, /*round_to_zero*/bool),
    Vcvt_FloatToInt_FromDouble_T1(/*sd*/Arm32SinglePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister, /*signed*/bool, /*round_to_zero*/bool),
    Vcvt_IntToFloat_ToSingle_T1(/*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister, /*signed*/bool),
    Vcvt_IntToFloat_ToDouble_T1(/*dd*/Arm32DoublePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister, /*signed*/bool),
    Vcvt_Single_To_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    Vcvt_Double_To_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),

    // ---- M8i (final corners): VMOV immediate, VMOV core-pair moves, half-precision and fixed-point VCVT ----
    Vmov_Immediate_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*imm8*/u8),
    Vmov_Immediate_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*imm8*/u8),
    // VMOV Dm, Rt, Rt2  /  VMOV Rt, Rt2, Dm  (a double moved to/from a pair of core registers)
    Vmov_CorePair_To_Double_T1(/*dm*/Arm32DoublePrecisionRegister, /*rt*/Arm32GeneralPurposeRegister, /*rt2*/Arm32GeneralPurposeRegister),
    Vmov_Double_To_CorePair_T1(/*rt*/Arm32GeneralPurposeRegister, /*rt2*/Arm32GeneralPurposeRegister, /*dm*/Arm32DoublePrecisionRegister),
    // VMOV Sm, Sm+1, Rt, Rt2  /  VMOV Rt, Rt2, Sm, Sm+1  (two consecutive singles to/from a core pair)
    Vmov_CorePair_To_Singles_T1(/*sm*/Arm32SinglePrecisionRegister, /*rt*/Arm32GeneralPurposeRegister, /*rt2*/Arm32GeneralPurposeRegister),
    Vmov_Singles_To_CorePair_T1(/*rt*/Arm32GeneralPurposeRegister, /*rt2*/Arm32GeneralPurposeRegister, /*sm*/Arm32SinglePrecisionRegister),
    // half-precision conversions: `top` picks the top (T) vs bottom (B) 16 bits of the single register
    Vcvt_HalfToSingle_T1(/*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister, /*top*/bool),
    Vcvt_SingleToHalf_T1(/*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister, /*top*/bool),
    // fixed-point conversions (Vd is its own source): `signed`, `bits32` (vs 16), and the fraction-bit count
    Vcvt_FloatToFixed_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*signed*/bool, /*bits32*/bool, /*frac_bits*/u8),
    Vcvt_FloatToFixed_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*signed*/bool, /*bits32*/bool, /*frac_bits*/u8),
    Vcvt_FixedToFloat_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*signed*/bool, /*bits32*/bool, /*frac_bits*/u8),
    Vcvt_FixedToFloat_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*signed*/bool, /*bits32*/bool, /*frac_bits*/u8),

    // ======================= ARMv8-M Security Extension (TrustZone-M) =======================
    Csdb_T1, // Consumption of Speculative Data Barrier (a 32-bit hint; NOPs on cores without speculation)
    Sg_T1, // Secure Gateway (a fixed 32-bit marker word)
    Bxns_T1(/*rm*/Arm32GeneralPurposeRegister),  // Branch and Exchange to Non-secure state
    Blxns_T1(/*rm*/Arm32GeneralPurposeRegister), // Branch with Link and Exchange to Non-secure state
    // Test Target: TT(a=0,t=0) / TTT(a=0,t=1, unprivileged) / TTA(a=1,t=0, alternate domain) / TTAT(a=1,t=1)
    Tt_T1(/*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*alternate (A)*/bool, /*unprivileged (T)*/bool),
    // lazy FP context save/restore (Security Extension, requires an FPU)
    Vlstm_T1(/*rn*/Arm32GeneralPurposeRegister),
    Vlldm_T1(/*rn*/Arm32GeneralPurposeRegister),

    // ======================= ARMv8.1-M MVE (Helium) =======================
    // The "three registers of the same length" vector data-processing format (vector-vector, `Qd, Qn, Qm`).
    // Three sub-families share the 0xEF../0xFF.. space; see `enums::arm32_mve_operations`.
    MveIntArith(Arm32MveIntArithOp, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveBitwise(Arm32MveBitwiseOp, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveFloatArith(Arm32MveFloatArithOp, Arm32MveFloatSize, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    // vector-by-scalar: a vector and a GPR scalar (`Qd, Qn, Rm`); and VDUP (`Qd, Rt`).
    MveVecScalarInt(Arm32MveVecScalarIntOp, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister),
    MveVecScalarFloat(Arm32MveVecScalarFloatOp, Arm32MveFloatSize, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister),
    MveVdup(Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*rt*/Arm32GeneralPurposeRegister),
    // shift by immediate: `Qd, Qm, #amount` (imm6 encodes element size + amount)
    MveShiftImm(Arm32MveShiftImmOp, Arm32MveSize, /*amount*/u8, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    // one-register modified immediate (VMOV/VMVN/VORR/VBIC #imm, VMOV.f32): carried raw as (cmode, op, imm8),
    // exactly as the A32 NEON form. (cmode, op) select the mnemonic/element size/shift; imm8 is the
    // AdvSIMDExpandImm seed. Disjoint from the shift family by imm6[21:19] == 000 (here always 0).
    MveModifiedImmediate(/*cmode*/u8, /*op*/bool, /*imm8*/u8, /*qd*/Arm32MveVectorRegister),
    // two-register miscellaneous (`Qd, Qm`): VREV/VCLS/VCLZ/VABS/VNEG/VQABS/VQNEG (sized int), VABS/VNEG
    // (float), and VMVN (register).
    MveMisc2(Arm32MveMisc2Op, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveMisc2Float(Arm32MveMisc2FloatOp, Arm32MveFloatSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VMAXA / VMINA -- 2-register elementwise max/min of `Qda` against the absolute value of `Qm`, result
    /// accumulated into `Qda` (signed-only int). base `0xEE33_0E81`; VMINA sets `[12]`, size `[19:18]`.
    MveVmaxaMina(/*is_min*/bool, Arm32MveSize, /*qda*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VMAXNMA / VMINNMA -- the floating-point twins of [`Self::MveVmaxaMina`] (`[19:18]=11`, precision at
    /// `[28]`: f16=1/f32=0). VMINNMA sets `[12]`. base `0xEE3F_0E81` (f32).
    MveVmaxnmaMinnma(/*is_min*/bool, Arm32MveFloatSize, /*qda*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveMvnRegister(/*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    // contiguous vector load/store (same element & access size): VLDRB/H/W and VSTRB/H/W, with the
    // `[Rn, #+/-off]` / `[Rn, #+/-off]!` / `[Rn], #+/-off` addressing forms. The offset is a multiple of the
    // access size (byte x1 / half x2 / word x4), in the imm7 range. `is_load` selects VLDR vs VSTR.
    MveLoadStore(/*is_load*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i32, ArmT32IndexMode),
    /// MVE gather load / scatter store with a scalar base and a vector of offsets (`Qd, [Rn, Qm]{, uxtw}`).
    /// `esize`/`msize` are element/access widths in bits (8/16/32/64); `unsigned` is the signedness of a
    /// widening load; `scaled` selects the `uxtw` (offset << log2(msize/8)) form.
    MveGatherScatter(/*is_load*/bool, /*unsigned*/bool, /*esize*/u8, /*msize*/u8, /*scaled*/bool, /*qd*/Arm32MveVectorRegister, /*rn*/Arm32GeneralPurposeRegister, /*qm*/Arm32MveVectorRegister),
    /// MVE gather/scatter with a VECTOR base and a scaled immediate (`Qd, [Qn{, #imm}]{!}`); word (`is_dword`
    /// = false) or doubleword only.
    MveGatherScatterBase(/*is_load*/bool, /*is_dword*/bool, /*writeback*/bool, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*offset*/i32),
    /// VLD2x/VLD4x/VST2x/VST4x -- de-interleaving/interleaving load/store of a list of 2 (`is_quad`=false) or 4
    /// consecutive vectors starting at `qd`, base `Rn`, optional writeback. `pass` is the interleave pass
    /// (0..1 for the 2-vector forms, 0..3 for the 4-vector forms).
    MveInterleave(/*is_load*/bool, /*is_quad*/bool, /*pass*/u8, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*rn*/Arm32GeneralPurposeRegister, /*writeback*/bool),
    /// Low-overhead loop start -- DLS/DLSTP (`is_while`=false) and WLS/WLSTP (`is_while`=true, forward branch).
    /// `tp_size` = `Some(size)` for the tail-predicated forms (needs MVE), `None` for the plain forms. `LR` is
    /// implicit. `offset` is the PC-relative forward branch (only used when `is_while`).
    LobStart(/*is_while*/bool, /*tp_size*/Option<u8>, /*rn*/Arm32GeneralPurposeRegister, /*offset*/i32),
    /// Low-overhead loop end -- LE / LETP (`tail_predicated`), a backward branch to the loop top. `LR` implicit.
    LobEnd(/*tail_predicated*/bool, /*offset*/i32),
    /// LCTP -- loop clear with tail predication.
    Lctp,
    /// VCTP -- create a tail predicate in the VPR from the element count in `rn`. `size` in {8,16,32,64}.
    MveVctp(/*size*/u8, /*rn*/Arm32GeneralPurposeRegister),
    // cross-lane reductions to a GPR: VADDV/VADDVA/VMINV/VMAXV/VMINAV/VMAXAV (`Rd, Qm`); VABAV (`Rd, Qn, Qm`).
    MveReduce(Arm32MveReduceOp, Arm32MveSize, /*rd*/Arm32GeneralPurposeRegister, /*qm*/Arm32MveVectorRegister),
    MveVabav(/*signed*/bool, Arm32MveSize, /*rd*/Arm32GeneralPurposeRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VMLADAV / VMLSDAV (non-long dual multiply-accumulate cross-lane reduction into an even GPR). `Rda` must
    /// be even; `exchange` and `subtract` are signed-only (`unsigned` excludes them).
    MveDualMac(/*subtract*/bool, /*exchange*/bool, /*accumulate*/bool, /*unsigned*/bool, Arm32MveSize, /*rda*/Arm32GeneralPurposeRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VMLALDAV / VMLSLDAV / VRMLALDAVH / VRMLSLDAVH (LONG dual MAC reduction into a GPR pair). `RdaLo` even and
    /// `RdaHi` odd are independent. Validity (enforced by the parser): subtract -> signed-only; rounding-high ->
    /// 32-bit only and (with exchange) signed-only and RdaHi != 13.
    MveLongDualMac(Arm32MveLongMacOp, /*exchange*/bool, /*accumulate*/bool, /*unsigned*/bool, Arm32MveSize, /*rda_lo*/Arm32GeneralPurposeRegister, /*rda_hi*/Arm32GeneralPurposeRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    // vector round-to-integral-float (VRINT) and float<->int convert (VCVT), both 2-register (`Qd, Qm`).
    MveVrint(Arm32MveVrintOp, Arm32MveFloatSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveVcvtFloatInt(/*to_int*/bool, /*unsigned*/bool, Arm32MveFloatSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// Fixed-point VCVT (`vcvt.<int>.f<w> Qd, Qm, #fbits` and the reverse). `to_fixed` = float->fixed (else
    /// fixed->float); `unsigned` is the fixed side's signedness; size is the shared 16/32 width; `fbits` in
    /// 1..=16 (F16) or 1..=32 (F32).
    MveVcvtFixed(/*to_fixed*/bool, /*unsigned*/bool, Arm32MveFloatSize, /*fbits*/u8, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// MVE VCVT between half- and single-precision (`vcvtb/vcvtt.f16.f32` / `.f32.f16`). `top` = T variant,
    /// `half_to_single` = `.f32.f16` (op=1). Implemented from DDI0553 (GNU's encoding of this is buggy).
    MveVcvtHalf(/*top*/bool, /*half_to_single*/bool, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// MVE shift-right-and-narrow (VSHRN/VRSHRN/VQSHRN/VQRSHRN/VQSHRUN/VQRSHRUN). `unsigned` applies only to
    /// VQSHRN/VQRSHRN; `top` = T variant; `src_is_32` selects the 32->16 narrowing (else 16->8); `shift` in
    /// 1..=(src/2). The saturating non-rounding forms are implemented from DDI0553 (GNU is buggy there).
    MveShiftNarrow(Arm32MveShiftNarrowOp, /*unsigned*/bool, /*top*/bool, /*src_is_32*/bool, /*shift*/u8, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VADC/VADCI/VSBC/VSBCI -- 32-bit vector add/subtract with carry (`Qd, Qn, Qm`). `init_carry` selects the
    /// I-variant (seeds the carry instead of reading FPSCR.C).
    MveVadc(/*subtract*/bool, /*init_carry*/bool, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VSHLC -- whole-vector left shift with carry through a GPR (`Qda, Rdm, #shift`), shift in 1..=32.
    MveVshlc(/*shift*/u8, /*qda*/Arm32MveVectorRegister, /*rdm*/Arm32GeneralPurposeRegister),
    /// VIDUP/VDDUP (no wrap) and VIWDUP/VDWDUP (with a wrap register) -- index generators. `rn` is even; `wrap_rm`
    /// (odd) is `Some` for the wrapping forms; `step` in {1,2,4,8}.
    MveViddup(/*decrement*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*rn*/Arm32GeneralPurposeRegister, /*wrap_rm*/Option<Arm32GeneralPurposeRegister>, /*step*/u8),
    /// VBRSR -- bit-reverse and shift right by a GPR amount broadcast to all lanes (`Qd, Qn, Rm`).
    MveVbrsr(Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister),
    // width-changing register moves: VMOVL (long, source size I8/I16), VMOVN (narrow, source size I16/I32),
    // and VADDLV (64-bit reduction to a GPR pair RdLo/RdLo+1).
    MveVmovl(/*top*/bool, /*unsigned*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveVmovn(/*top*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VMOV (two 32-bit lanes) -- transfers a pair of general-purpose registers to/from two 32-bit lanes of a
    /// vector register `Qd` (lanes `{idx1, idx1-2}`, `idx1` in {2,3}). `to_vector` = GPRs -> lanes. base
    /// `0xEC00_0F00`; dir `[20]`, Rt2 `[19:16]`, Qd `[15:13]`, `idx1`'s low bit `[4]`, Rt `[3:0]`. (MVE.)
    MveVmovTwoLane(/*to_vector*/bool, /*idx1*/u8, /*qd*/Arm32MveVectorRegister, /*rt*/Arm32GeneralPurposeRegister, /*rt2*/Arm32GeneralPurposeRegister),
    /// VQMOVN/VQMOVUN -- saturating narrowing register move. `unsigned` only applies to VQMOVN (selects
    /// `.s`/`.u`); VQMOVUN is always signed-source. `size` is the (wider) source element size I16/I32.
    MveVqmovn(Arm32MveQMovnKind, /*unsigned*/bool, /*top*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VMULL -- vector multiply long (widening), bottom/top. `polynomial` selects the poly form (size = P8/P16
    /// carried as I8/I16); otherwise integer with `unsigned` picking `.s`/`.u`. `size` is the source size.
    MveVmull(/*polynomial*/bool, /*unsigned*/bool, /*top*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VMULH/VRMULH -- vector multiply returning the high half. `rounding` selects VRMULH.
    MveVmulh(/*rounding*/bool, /*unsigned*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VQDMULL (vector) -- saturating doubling multiply long, bottom/top. `size32` selects `.s32` (else `.s16`).
    MveVqdmull(/*top*/bool, /*size32*/bool, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VQDMULL (scalar) -- saturating doubling multiply long with a GPR-broadcast second multiplicand.
    MveVqdmullScalar(/*top*/bool, /*size32*/bool, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister),
    /// VQDMLADH/VQDMLSDH (+ rounding VQRD*) -- saturating doubling multiply add/subtract dual, high half,
    /// accumulating into Qd. `subtract` selects VQDMLSDH, `rounding` selects the VQRD* form, `exchange` the X variant.
    MveVqdmladh(/*subtract*/bool, /*rounding*/bool, /*exchange*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VSHL/VRSHL/VQSHL/VQRSHL by VECTOR -- shift Qm left per-lane by the signed amounts in Qn (negative = right).
    /// `rounding` selects the VR*/VQR* form, `saturating` the VQ* form. Operand order `Qd, Qm, Qn`.
    MveShiftByVector(/*rounding*/bool, /*saturating*/bool, /*unsigned*/bool, Arm32MveSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister),
    /// VSHL/VRSHL/VQSHL/VQRSHL by GPR SCALAR -- shift every lane of `Qda` (read-modify-write) by `Rm`.
    MveShiftByScalar(/*rounding*/bool, /*saturating*/bool, /*unsigned*/bool, Arm32MveSize, /*qda*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister),
    /// VSHLL -- vector shift left long (widening), bottom/top. `size` is the (narrower) source element size
    /// I8/I16; `shift` is 1..=esize (esize = 8 or 16). shift == esize uses the T2 max encoding.
    MveVshll(/*top*/bool, /*unsigned*/bool, Arm32MveSize, /*shift*/u8, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    /// VMOVX / VINS -- Armv8.1-M half-precision FP move-extract (VMOVX) / insert (VINS) on single-precision
    /// registers. `insert` selects VINS. `.f16 Sd, Sm`.
    Vmovx_T1(/*insert*/bool, /*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    /// DBG -- debug hint (`dbg #<option>`, option 0..15).
    Dbg_T1(/*option*/u8),
    /// ESB -- error synchronization barrier (RAS); a NOP-compatible hint.
    Esb_T1,
    /// SSBB / PSSBB -- speculative store bypass barriers (DSB #0 / #4 in encoding).
    Ssbb_T1,
    Pssbb_T1,
    /// SB -- Speculation Barrier (FEAT_SB). Fixed word `0xF3BF_8F70` (the `[7:4]=0111` sibling of DSB/DMB/ISB).
    Sb_T1,
    /// CLRM -- ARMv8-M clear multiple registers (`clrm {<list>}`); the bitmap covers R0..R12, then APSR(15)
    /// and LR(14). Bit 13 (SP) must be zero.
    Clrm_T1(/*register_list*/u16),
    /// VSEL -- ARMv8-M FP conditional select (`vsel<cc>.f32/.f64`). `cond`: 0 = EQ, 1 = VS, 2 = GE, 3 = GT.
    Vsel_Single_T1(/*cond*/u8, /*sd*/Arm32SinglePrecisionRegister, /*sn*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    Vsel_Double_T1(/*cond*/u8, /*dd*/Arm32DoublePrecisionRegister, /*dn*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    /// CSEL/CSINC/CSINV/CSNEG -- ARMv8.1-M conditional select (`op`: 0 = CSEL, 1 = CSINC, 2 = CSINV, 3 = CSNEG).
    /// The CSET/CSETM/CINC/CINV/CNEG aliases assemble to these (Rn==Rm and the inverted condition).
    Csel_T1(/*op*/u8, /*rd*/Arm32GeneralPurposeRegister, /*rn*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, ArmT32InstructionCondition),
    /// LSLL/LSRL/ASRL -- ARMv8.1-M 64-bit shift of the `RdaHi:RdaLo` GPR pair (`op`: 0 = LSLL, 1 = LSRL, 2 = ASRL).
    /// `RdaLo` is even, `RdaHi` = RdaLo+1. Immediate form: shift 1..=31.
    LongShiftImm_T1(/*op*/u8, /*rdalo*/Arm32GeneralPurposeRegister, /*rdahi*/Arm32GeneralPurposeRegister, /*imm*/u8),
    /// LSLL/LSRL/ASRL by register (the shift amount is in `Rm`).
    LongShiftReg_T1(/*op*/u8, /*rdalo*/Arm32GeneralPurposeRegister, /*rdahi*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    /// UQSHL/URSHR/SRSHR/SQSHL -- ARMv8.1-M saturating/rounding shift of a single GPR by immediate. `op`:
    /// 0 = UQSHL, 1 = URSHR, 2 = SRSHR, 3 = SQSHL.
    SatShiftImm_T1(/*op*/u8, /*rda*/Arm32GeneralPurposeRegister, /*imm*/u8),
    /// UQSHLL/URSHRL/SRSHRL/SQSHLL -- the 64-bit (GPR-pair) saturating/rounding shift by immediate.
    SatShiftLongImm_T1(/*op*/u8, /*rdalo*/Arm32GeneralPurposeRegister, /*rdahi*/Arm32GeneralPurposeRegister, /*imm*/u8),
    /// SQRSHR/UQRSHL -- saturating rounding shift of a single GPR by register (`signed` selects SQRSHR).
    SatShiftReg_T1(/*signed*/bool, /*rda*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister),
    /// SQRSHRL/UQRSHLL -- the 64-bit saturating rounding shift by register; `sat48` selects #48 (else #64).
    SatShiftLongReg_T1(/*signed*/bool, /*rdalo*/Arm32GeneralPurposeRegister, /*rdahi*/Arm32GeneralPurposeRegister, /*rm*/Arm32GeneralPurposeRegister, /*sat48*/bool),
    /// VRINTR -- FP round to integral (using the FPSCR rounding mode). `.f32 Sd, Sm`.
    Vrintr_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    /// VRINTR `.f64 Dd, Dm`.
    Vrintr_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    /// VMAXNM / VMINNM -- FP maximum/minimum with IEEE-754 default NaN handling (ARMv8-M FP). base `0xFE80_0A00`
    /// (max) / `0xFE80_0A40` (min); `.f32 Sd, Sn, Sm`.
    Vmaxnm_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*sn*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    /// VMAXNM `.f64 Dd, Dn, Dm` (base `0xFE80_0B00`).
    Vmaxnm_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*dn*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    /// VMINNM `.f32 Sd, Sn, Sm`.
    Vminnm_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*sn*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    /// VMINNM `.f64 Dd, Dn, Dm` (base `0xFE80_0B40`).
    Vminnm_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*dn*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    /// VRINTA/N/P/M -- FP round to integral with an explicit directed rounding mode (ARMv8-M FP). base
    /// `0xFEB8_0A40`; the mode is `rm_bits() @ [17:16]`. `.f32 Sd, Sm`. See [`Arm32DirectedRound`].
    Vrint_Directed_Single_T1(/*mode*/Arm32DirectedRound, /*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    /// VRINTA/N/P/M `.f64 Dd, Dm` (base `0xFEB8_0B40`).
    Vrint_Directed_Double_T1(/*mode*/Arm32DirectedRound, /*dd*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    /// VRINTZ -- FP round to integral toward zero (base `0xEEB6_0AC0`, the `[7]=1` sibling of VRINTR). `.f32 Sd, Sm`.
    Vrintz_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    /// VRINTZ `.f64 Dd, Dm` (base `0xEEB6_0BC0`).
    Vrintz_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    /// VRINTX -- FP round to integral, signalling inexact (base `0xEEB7_0A40`, `opc2=0111`). `.f32 Sd, Sm`.
    Vrintx_Single_T1(/*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister),
    /// VRINTX `.f64 Dd, Dm` (base `0xEEB7_0B40`).
    Vrintx_Double_T1(/*dd*/Arm32DoublePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    /// VCVTA/N/P/M -- FP-to-integer convert with an explicit directed rounding mode, result in a single-word
    /// register (ARMv8-M FP). base `0xFEBC_0A40`; the mode is `rm_bits() @ [17:16]`, `signed @ [7]`. Source is
    /// f32: `.s32.f32`/`.u32.f32 Sd, Sm`. See [`Arm32DirectedRound`].
    Vcvt_Directed_FromSingle_T1(/*mode*/Arm32DirectedRound, /*sd*/Arm32SinglePrecisionRegister, /*sm*/Arm32SinglePrecisionRegister, /*signed*/bool),
    /// VCVTA/N/P/M from an f64 source (base `0xFEBC_0B40`): `.s32.f64`/`.u32.f64 Sd, Dm`.
    Vcvt_Directed_FromDouble_T1(/*mode*/Arm32DirectedRound, /*sd*/Arm32SinglePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister, /*signed*/bool),
    /// VJCVT (VJCVTZS) -- JavaScript-semantics convert f64 -> s32, round toward zero (FEAT_JSCVT). base
    /// `0xEEB9_0BC0`; `sd` is the 32-bit result, `dm` the double source.
    Vjcvt_T1(/*sd*/Arm32SinglePrecisionRegister, /*dm*/Arm32DoublePrecisionRegister),
    /// VADDLV/VADDLVA (64-bit cross-lane reduction into a GPR pair). `rd_lo` even and `rd_hi` odd are
    /// independent (RdHi>>1 is encoded at `[22:20]` separately from RdLo at `[15:12]`).
    MveVaddlv(/*accumulate*/bool, /*unsigned*/bool, /*rd_lo*/Arm32GeneralPurposeRegister, /*rd_hi*/Arm32GeneralPurposeRegister, /*qm*/Arm32MveVectorRegister),
    // complex-number ops (`Qd, Qn, Qm, #rotate`). VCADD/VHCADD allow rotate 90/270 (rot270 flag); VCMUL/VCMLA
    // allow 0/90/180/270 (rot = 0..3).
    MveVcaddInt(/*halving*/bool, Arm32MveSize, /*rot270*/bool, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveVcaddFloat(Arm32MveFloatSize, /*rot270*/bool, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveVcmul(Arm32MveFloatSize, /*rotate*/u8, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveVcmla(Arm32MveFloatSize, /*rotate*/u8, /*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    // predication primitives: VPSEL (per-lane select by the VPR) and VPNOT (invert the VPR).
    MveVpsel(/*qd*/Arm32MveVectorRegister, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveVpnot,
    // VCMP (vector compare into the VPR): register `<cond>, Qn, Qm` and scalar `<cond>, Qn, Rm`; int + float.
    MveVcmpReg(Arm32MveVcmpCondition, Arm32MveSize, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveVcmpScalar(Arm32MveVcmpCondition, Arm32MveSize, /*qn*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister),
    MveVcmpFloatReg(Arm32MveVcmpCondition, Arm32MveFloatSize, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
    MveVcmpFloatScalar(Arm32MveVcmpCondition, Arm32MveFloatSize, /*qn*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister),
    // VPST: set up a 1-4 instruction predicate block from the VPR. `mask` is the 4-bit then/else pattern
    // (1..=15; the following N = 4 - mask.trailing_zeros() instructions are predicated).
    MveVpst(/*mask*/u8),
    // VPT: compare AND set up a predicate block (VCMP with a nonzero predicate mask). Same forms as VCMP.
    MveVptReg(Arm32MveVcmpCondition, Arm32MveSize, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister, /*mask*/u8),
    MveVptScalar(Arm32MveVcmpCondition, Arm32MveSize, /*qn*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister, /*mask*/u8),
    MveVptFloatReg(Arm32MveVcmpCondition, Arm32MveFloatSize, /*qn*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister, /*mask*/u8),
    MveVptFloatScalar(Arm32MveVcmpCondition, Arm32MveFloatSize, /*qn*/Arm32MveVectorRegister, /*rm*/Arm32GeneralPurposeRegister, /*mask*/u8),
    // floating-point min/max cross-lane reduction to a GPR (VMAXNMV/VMINNMV/VMAXNMAV/VMINNMAV)
    MveFloatReduce(Arm32MveFloatReduceOp, Arm32MveFloatSize, /*rd*/Arm32GeneralPurposeRegister, /*qm*/Arm32MveVectorRegister),
    // VCVTA/N/P/M: convert float -> int with an explicit rounding mode (0=A, 1=N, 2=P, 3=M).
    MveVcvtRound(/*rounding*/u8, /*unsigned*/bool, Arm32MveFloatSize, /*qd*/Arm32MveVectorRegister, /*qm*/Arm32MveVectorRegister),
}

//

// ---- operand validation helpers (return EncodeError instead of panicking) ----
//
// Each encode arm runs the relevant checks before handing field values to the bit-packing helpers in
// ArmT32InstructionEncoder. With these in place every constrained operand is validated here, so the
// `panic_on_invalid_bit_length_*` checks inside that encoder become an unreachable defensive backstop
// rather than a path a caller can trip.

fn check_unsigned_maximum(field: &'static str, value: u32, maximum: u32) -> Result<(), EncodeError> {
    if value > maximum {
        return Err(EncodeError::ImmediateOutOfRange { field, value: value as i64, minimum: 0, maximum: maximum as i64 });
    }
    Ok(())
}

fn check_signed_range(field: &'static str, value: i32, minimum: i32, maximum: i32) -> Result<(), EncodeError> {
    if value < minimum || value > maximum {
        return Err(EncodeError::ImmediateOutOfRange { field, value: value as i64, minimum: minimum as i64, maximum: maximum as i64 });
    }
    Ok(())
}

fn check_multiple_of(field: &'static str, value: i64, required_multiple: u32) -> Result<(), EncodeError> {
    if (value % (required_multiple as i64)) != 0 {
        return Err(EncodeError::ImmediateNotAligned { field, value, required_multiple });
    }
    Ok(())
}

//

fn next_u16le_from_iter<'a, I>(iter: &mut I, iter_offset: &mut usize) -> Result<Option<u16>, DecodeError> where I: Iterator<Item = &'a u8> {
    let byte0 = match iter.next() {
        Some(value) => value,
        None => { return Ok(None); } // EOF; nothing to decode
    };
    *iter_offset += 1;
    //
    let byte1 = match iter.next() {
        Some(value) => value,
        None => { return Err(DecodeError::IncompleteInstruction); } // could not capture second byte (out of 2) of word
    };
    *iter_offset += 1;

    let result = u16::from_le_bytes([*byte0, *byte1]);

    Ok(Some(result))
}

//

fn convert_halfwords_to_u8_vec(halfwords: &[u16]) -> Vec<u8> {
    let mut result = Vec::<u8>::with_capacity(halfwords.len() * 2);
    
    for halfword in halfwords.iter() {
        result.extend_from_slice(&halfword.to_le_bytes());
    }

    result
}

fn combine_instruction_halfwords_into_word(halfwords: &[u16; 2]) -> u32 {
    ((halfwords[0] as u32) << 16) | (halfwords[1] as u32)
}

// ---- ARMv7-M 32-bit field helpers (shared by decode; the encode arms pack fields inline) ----

// ARMv7-M data-processing operands cannot be SP (R13) or PC (R15) -- those are UNPREDICTABLE.
fn check_general_register_is_encodable(field: &'static str, register: &Arm32GeneralPurposeRegister) -> Result<(), EncodeError> {
    match register {
        Arm32GeneralPurposeRegister::R13 => Err(EncodeError::RegisterNotEncodable { field, detail: "SP (R13) is not permitted in this ARMv7-M operand" }),
        Arm32GeneralPurposeRegister::R15 => Err(EncodeError::RegisterNotEncodable { field, detail: "PC (R15) is not permitted in this ARMv7-M operand" }),
        _ => Ok(()),
    }
}

// Load/store wide base register cannot be PC (that selects the literal/T2 form instead).
fn check_register_is_not_pc(field: &'static str, register: &Arm32GeneralPurposeRegister) -> Result<(), EncodeError> {
    if *register == Arm32GeneralPurposeRegister::R15 {
        Err(EncodeError::RegisterNotEncodable { field, detail: "PC (R15) base selects the literal form; use a register base here" })
    } else {
        Ok(())
    }
}

// Bitfield lsb/width must satisfy 0 <= lsb <= 31, 1 <= width, and lsb + width <= 32.
fn check_bitfield_lsb_width(lsb: u8, width: u8) -> Result<(), EncodeError> {
    if lsb > 31 {
        return Err(EncodeError::ImmediateOutOfRange { field: "lsb", value: lsb as i64, minimum: 0, maximum: 31 });
    }
    if !(1..=32).contains(&width) {
        return Err(EncodeError::ImmediateOutOfRange { field: "width", value: width as i64, minimum: 1, maximum: 32 });
    }
    if (lsb as u16) + (width as u16) > 32 {
        return Err(EncodeError::ImmediateOutOfRange { field: "width", value: width as i64, minimum: 1, maximum: (32 - lsb) as i64 });
    }
    Ok(())
}

// Exclusive word-access (LDREX/STREX) offset: a byte offset that must be a multiple of 4 in 0..=1020;
// the encoded field is offset/4 (imm8).
fn exclusive_word_offset_field(field: &'static str, offset: u16) -> Result<u32, EncodeError> {
    if !offset.is_multiple_of(4) {
        return Err(EncodeError::ImmediateNotAligned { field, value: offset as i64, required_multiple: 4 });
    }
    if offset > 1020 {
        return Err(EncodeError::ImmediateOutOfRange { field, value: offset as i64, minimum: 0, maximum: 1020 });
    }
    Ok((offset / 4) as u32)
}

// MOVW/MOVT pack imm16 as imm4:i:imm3:imm8 across bits 19:16 / 26 / 14:12 / 7:0.
fn movw_immediate_field_bits(imm16: u16) -> u32 {
    let imm4 = ((imm16 >> 12) & 0b1111) as u32;
    let i = ((imm16 >> 11) & 0b1) as u32;
    let imm3 = ((imm16 >> 8) & 0b111) as u32;
    let imm8 = (imm16 & 0b1111_1111) as u32;
    (imm4 << 16) | (i << 26) | (imm3 << 12) | imm8
}
