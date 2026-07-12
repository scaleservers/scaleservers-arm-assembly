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

impl ArmT32Instruction {
    /// Decode one T32 (Thumb) instruction from a little-endian byte iterator, advancing `iter_offset` past
    /// the bytes consumed. Returns `Ok(None)` at a clean end of input, `Ok(Some(_))` for a decoded
    /// instruction, or a [`DecodeError`] for malformed or unknown bytes. Never panics on arbitrary input.
    ///
    /// T32 is variable-length: when the first halfword is the prefix of a 32-bit instruction this reads a
    /// full 4 bytes. If that yields [`DecodeError::InvalidOpcode`], a caller scanning a stream may back the
    /// iterator up and retry a 16-bit decode at the original `iter_offset + 2`; `iter_offset` is left at the
    /// correct position (e.g. original + 4) in every case.
    ///
    /// This resolves the CDE-vs-generic-coprocessor ambiguity with [`ArmDecodeContext::default`]
    /// (coprocessors 0-7 are CDE). Use [`decode_with`](Self::decode_with) to choose a different context.
    pub fn decode<'a, I>(iter: &mut I, iter_offset: &mut usize) -> Result<Option<Self>, DecodeError> where I: Iterator<Item = &'a u8> {
        Self::decode_with(iter, iter_offset, &ArmDecodeContext::default())
    }

    /// Decode one T32 instruction exactly like [`decode`](Self::decode), but with an explicit
    /// [`ArmDecodeContext`] that resolves "same bytes, different meaning" ambiguities (the family-wide
    /// Rule R4). The only such case in T32 is the CDE custom-datapath (`CX*`/`VCX*`) vs. generic coprocessor
    /// (`CDP/MCR/LDC/...`) overlap on coprocessors 0-7 -- see [`ArmDecodeContext`] for the details. With
    /// [`ArmDecodeContext::default`] the result is identical to [`decode`](Self::decode).
    pub fn decode_with<'a, I>(iter: &mut I, iter_offset: &mut usize, context: &ArmDecodeContext) -> Result<Option<Self>, DecodeError> where I: Iterator<Item = &'a u8> {
        // retrieve the first two bytes (which could be either a 16-bit instruction or the first word of a 32-bit instruction)
        let halfword0 = match next_u16le_from_iter(iter, iter_offset) {
            Ok(value) => match value {
                Some(some_value) => some_value,
                None => return Ok(None) // EOF; nothing to decode
            },
            Err(error) => return Err(error), // bubble up error to our caller
        };

        // NOTE: as the full opcode information is not fully contained in the topmost 6 bits of the first halfword, we match halfwords against a mask to detect valid instructions (and do the same with a full word against 32-bit mask if no 16-bit mask matched)

        // NOTE: we will set the following flag to true if we detect the leading pattern for a 32-bit instruction during out 16-bit decoding filter
        let mut matched_first_16_bits_of_32_bit_mask = false;

        /* 16-bit instructions */

        // NOTE: as an optimization procedure, we could group the 16-bit (and possibly the 32-bit) instructions into groups based on an initial bitmask check using the following opcode categories.
        /* NOTE: 16-bit instructions contain a preliminary opcode in the 6 topmost bits of the halfword instruction, per the ARMv6-M Architecture Reference
         * 00xxxx => shift (immediate), add, subtract, move and compare
         * 010000 => data processing
         * 010001 => special data instructions and branch and exchange
         * 01001x => load from literal pool
         * 0101xx |
         * 011xxx |
         * 100xxx => load/store single data item
         * 10100x => generate pc-relative address
         * 10101x => generate sp-relative address
         * 1011xx => miscellaneous 16-bit instructions
         * 11000x => store multiple registers
         * 11001x => load multiple registers
         * 1101xx => conditional branch, and supervisor call
         * 11100x => unconditional branch
         */ 

        // mask: 0b1111_1111_1100_0000
        match halfword0 & 0b1111_1111_1100_0000 {
            opcode if opcode == ArmT32OpcodePattern_16Bit::Adc_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Adc_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::And_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::And_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Asr_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Asr_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Bic_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Bic_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Cmn_Register_T1 as u16 => {
                let (rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Cmn_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Cmp_Register_T1 as u16 => {
                let (rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Cmp_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Eor_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Eor_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Lsl_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Lsl_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Lsr_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Lsr_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            // NOTE: MOV (register) T2 is decoded by the Lsl_Immediate_T1 arm instead -- it is the `imm5 == 0` case
            // of LSL (immediate) T1 -- so there is deliberately no separate Mov_Register_T2 arm here.
            op if op == ArmT32OpcodePattern_16Bit::Mul_T1 as u16 => {
                let (rdm, rn) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Mul_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdm), Arm32LowGeneralPurposeRegister::from_operand_bits(rn))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Mvn_Register_T1 as u16 => {
                let (rd, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Mvn_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Orr_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Orr_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Rev_T1 as u16 => {
                let (rd, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Rev_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Rev16_T1 as u16 => {
                let (rd, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Rev16_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Revsh_T1 as u16 => {
                let (rd, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Revsh_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ror_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Ror_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Rsb_Immediate_T1 as u16 => {
                let (rd, rn) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Rsb_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rn))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Sbc_Register_T1 as u16 => {
                let (rdn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Sbc_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Sxtb_T1 as u16 => {
                let (rd, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Sxtb_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Sxth_T1 as u16 => {
                let (rd, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Sxth_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Tst_Register_T1 as u16 => {
                let (rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Tst_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Uxtb_T1 as u16 => {
                let (rd, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Uxtb_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Uxth_T1 as u16 => {
                let (rd, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305(halfword0);
                return Ok(Some(Self::Uxth_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            _ => (), // continue to next masked set
        }

        // mask: 0b1111_1110_0000_0000
        match halfword0 & 0b1111_1110_0000_0000 {
            opcode if opcode == ArmT32OpcodePattern_16Bit::Add_Immediate_T1 as u16 => {
                let (rd, rn, imm3) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Add_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), imm3)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Add_Register_T1 as u16 => {
                let (rd, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Add_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldr_Register_T1 as u16 => {
                let (rt, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Ldr_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldrb_Register_T1 as u16 => {
                let (rt, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Ldrb_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldrh_Register_T1 as u16 => {
                let (rt, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Ldrh_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldrsb_Register_T1 as u16 => {
                let (rt, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Ldrsb_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldrsh_Register_T1 as u16 => {
                let (rt, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Ldrsh_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Pop_T1 as u16 => {
                let (register_list, p) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__0808(halfword0);
                let registers = gpr_coding_utils::convert_gpr_register_list_u8_and_p_u1_to_registers_vector(register_list, p);
                return Ok(Some(Self::Pop_T1(registers)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Push_T1 as u16 => {
                let (register_list, m) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__0808(halfword0);
                let registers = gpr_coding_utils::convert_gpr_register_list_u8_and_m_u1_to_registers_vector(register_list, m);
                return Ok(Some(Self::Push_T1(registers)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Str_Register_T1 as u16 => {
                let (rt, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Str_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Strb_Register_T1 as u16 => {
                let (rt, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Strb_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Strh_Register_T1 as u16 => {
                let (rt, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Strh_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Sub_Immediate_T1 as u16 => {
                let (rd, rn, imm3) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Sub_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), imm3)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Sub_Register_T1 as u16 => {
                let (rd, rn, rm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__0608(halfword0);
                return Ok(Some(Self::Sub_Register_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
            },


            _ => (), // continue to next masked set
        }

        // mask: 0b1111_1000_0000_0000
        match halfword0 & 0b1111_1000_0000_0000 {
            op if op == ArmT32OpcodePattern_16Bit::Add_Immediate_T2 as u16 => {
                let (imm8, rdn) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                return Ok(Some(Self::Add_Immediate_T2(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), imm8)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Add_SpPlusImmediate_T1 as u16 => {
                let (imm8, rd) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                let const10 = (imm8 as u16) * 4;
                return Ok(Some(Self::Add_SpPlusImmediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), const10)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Adr_T1 as u16 => {
                let (imm8, rd) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                let const10 = (imm8 as u16) * 4;
                return Ok(Some(Self::Adr_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), const10)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Asr_Immediate_T1 as u16 => {
                // NOTE: "A6.4.1 Shift operations" of the ARVv6-M ISA doc indicates that an imm5 value of ZERO is interpeted as an imm5 value of 32 (because shift 'type' is '10')
                let (rd, rm, encoded_imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                let decoded_imm5 = if encoded_imm5 == 0 { 32 } else { encoded_imm5 };
                return Ok(Some(Self::Asr_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm), decoded_imm5)));
            },
            op if op == ArmT32OpcodePattern_16Bit::B_T2 as u16 => {
                let encoded_signed_imm11 = ArmT32InstructionDecoder::decode_instruction_halfword__s000a(halfword0);
                let decoded_signed_imm12 = encoded_signed_imm11 * 2;
                return Ok(Some(Self::B_T2(decoded_signed_imm12)));
            },
            op if op == ((ArmT32OpcodePattern_32Bit::Bl_T1 as u32) >> 16) as u16 => {
                matched_first_16_bits_of_32_bit_mask = true;
            },
            op if op == ArmT32OpcodePattern_16Bit::Cmp_Immediate_T1 as u16 => {
                let (imm8, rn) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                return Ok(Some(Self::Cmp_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rn), imm8)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldm_T1 as u16 => {
                let (register_list, rn) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                let registers = gpr_coding_utils::convert_gpr_register_list_u8_to_low_registers_vector(register_list);
                return Ok(Some(Self::Ldm_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rn), registers)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldr_Immediate_T1 as u16 => {
                let (rt, rn, encoded_imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                let decoded_imm7 = encoded_imm5 * 4;
                return Ok(Some(Self::Ldr_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), decoded_imm7)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldr_Immediate_T2 as u16 => {
                let (encoded_imm8, rt) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                let decoded_imm10 = (encoded_imm8 as u16) * 4;
                return Ok(Some(Self::Ldr_Immediate_T2(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), decoded_imm10)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldr_Literal_T1 as u16 => {
                let (encoded_imm8, rt) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                let decoded_imm10 = (encoded_imm8 as u16) * 4;
                return Ok(Some(Self::Ldr_Literal_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), decoded_imm10)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldrb_Immediate_T1 as u16 => {
                let (rt, rn, imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                return Ok(Some(Self::Ldrb_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), imm5)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Ldrh_Immediate_T1 as u16 => {
                let (rt, rn, encoded_imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                let decoded_imm6 = encoded_imm5 * 2;
                return Ok(Some(Self::Ldrh_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), decoded_imm6)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Lsl_Immediate_T1 as u16 => {
                let (rd, rm, imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                //
                // NOTE: if this instruction is encoded with an imm5 value of 0b00000, then it is a MovRegister_T2 instruction (and not an LslImmediate_T1 instruction); in other words...an LSL with a shift of zero is just a MOV
                if imm5 == 0 {
                    return Ok(Some(Self::Mov_Register_T2(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm))));
                } else {
                    return Ok(Some(Self::Lsl_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm), imm5)));
                }
            },
            op if op == ArmT32OpcodePattern_16Bit::Lsr_Immediate_T1 as u16 => {
                // NOTE: "A6.4.1 Shift operations" of the ARVv6-M ISA doc indicates that an imm5 value of ZERO is interpeted as an imm5 value of 32 (because shift 'type' is '01')
                let (rd, rm, encoded_imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                let decoded_imm5 = if encoded_imm5 == 0 { 32 } else { encoded_imm5 };
                return Ok(Some(Self::Lsr_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), Arm32LowGeneralPurposeRegister::from_operand_bits(rm), decoded_imm5)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Mov_Immediate_T1 as u16 => {
                let (imm8, rd) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                return Ok(Some(Self::Mov_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rd), imm8)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Stm_T1 as u16 => {
                let (register_list, rn) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                let registers = gpr_coding_utils::convert_gpr_register_list_u8_to_low_registers_vector(register_list);
                return Ok(Some(Self::Stm_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rn), registers)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Str_Immediate_T1 as u16 => {
                let (rt, rn, encoded_imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                let decoded_imm7 = encoded_imm5 * 4;
                return Ok(Some(Self::Str_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), decoded_imm7)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Str_Immediate_T2 as u16 => {
                let (encoded_imm8, rt) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                let decoded_imm10 = (encoded_imm8 as u16) * 4;
                return Ok(Some(Self::Str_Immediate_T2(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), decoded_imm10)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Strb_Immediate_T1 as u16 => {
                let (rt, rn, imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                return Ok(Some(Self::Strb_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), imm5)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Strh_Immediate_T1 as u16 => {
                let (rt, rn, encoded_imm5) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0305__060a(halfword0);
                let decoded_imm6 = encoded_imm5 * 2;
                return Ok(Some(Self::Strh_Immediate_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rt), Arm32LowGeneralPurposeRegister::from_operand_bits(rn), decoded_imm6)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Sub_Immediate_T2 as u16 => {
                let (imm8, rdn) = ArmT32InstructionDecoder::decode_instruction_halfword__0007__080a(halfword0);
                return Ok(Some(Self::Sub_Immediate_T2(Arm32LowGeneralPurposeRegister::from_operand_bits(rdn), imm8)));
            },
            _ => (), // continue to next masked set
        }

        // mask: 0b1111_1111_0000_0000
        match halfword0 & 0b1111_1111_0000_0000 {
            op if op == ArmT32OpcodePattern_16Bit::Add_Register_T2 as u16 => {
                let (rdn, rm, dn) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0306__0707(halfword0);
                //
                let n = (dn << 3) | rdn;
                let m = rm;
                //
                let n_as_gpr = Arm32GeneralPurposeRegister::from_operand_bits(n);
                let m_as_gpr = Arm32GeneralPurposeRegister::from_operand_bits(m);
                //
                // NOTE: if d is 0b1101 or m is 0b1101, it's an ::Add_SpPlusRegister_T* instruction instead
                if m_as_gpr == Arm32GeneralPurposeRegister::R13 {
                    return Ok(Some(Self::Add_SpPlusRegister_T1(n_as_gpr)));
                }
                if n_as_gpr == Arm32GeneralPurposeRegister::R13 {
                    return Ok(Some(Self::Add_SpPlusRegister_T2(m_as_gpr)));
                }
                //
                // otherwise, this is a standard ::Add_Register_T2 encoding
                return Ok(Some(Self::Add_Register_T2(n_as_gpr, m_as_gpr)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Bkpt_T1 as u16 => {
                let imm8 = ArmT32InstructionDecoder::decode_instruction_halfword__0007(halfword0);
                return Ok(Some(Self::Bkpt_T1(imm8)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Cmp_Register_T2 as u16 => {
                let (rn, rm, n) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0306__0707(halfword0);
                //
                let n = (n << 3) | rn;
                let m = rm;
                //
                let n_as_gpr = Arm32GeneralPurposeRegister::from_operand_bits(n);
                let m_as_gpr = Arm32GeneralPurposeRegister::from_operand_bits(m);
                //
                return Ok(Some(Self::Cmp_Register_T2(n_as_gpr, m_as_gpr)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Mov_Register_T1 as u16 => {
                let (rd, rm, d) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0306__0707(halfword0);
                //
                let rd_as_u4 = (d << 3) | rd;
                //
                return Ok(Some(Self::Mov_Register_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd_as_u4), Arm32GeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Svc_T1 as u16 => {
                let imm8 = ArmT32InstructionDecoder::decode_instruction_halfword__0007(halfword0);
                return Ok(Some(Self::Svc_T1(imm8)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Udf_T1 as u16 => {
                let imm8 = ArmT32InstructionDecoder::decode_instruction_halfword__0007(halfword0);
                return Ok(Some(Self::Udf_T1(imm8)));
            },
        _ => (), // continue to next masked set
        }

        // mask: 0b1111_1111_1000_0000
        match halfword0 & 0b1111_1111_1000_0000 {
            op if op == ArmT32OpcodePattern_16Bit::Add_SpPlusImmediate_T2 as u16 => {
                let imm7 = ArmT32InstructionDecoder::decode_instruction_halfword__0006(halfword0);
                let const9 = (imm7 as u16) * 4;
                return Ok(Some(Self::Add_SpPlusImmediate_T2(const9)));
            },
            op if op == ArmT32OpcodePattern_16Bit::Sub_SpMinusImmediate_T1 as u16 => {
                let imm7 = ArmT32InstructionDecoder::decode_instruction_halfword__0006(halfword0);
                let const9 = (imm7 as u16) * 4;
                return Ok(Some(Self::Sub_SpMinusImmediate_T1(const9)));
            },

            _ => (), // continue to next masked set
        }

        // mask: 0b1111_1111_0111_1000
        match halfword0 & 0b1111_1111_0111_1000 {
            op if op == ArmT32OpcodePattern_16Bit::Add_SpPlusRegister_T1 as u16 => {
                // NOTE: this encoding is the same encoding as ::Add_Register_T2 -- but with 0b1101 specified in bits 3..=6 (i.e. "rm" in ::Add_Register_T2)
                let (rdm, dm) = ArmT32InstructionDecoder::decode_instruction_halfword__0002__0707(halfword0);
                //
                let m = (dm << 3) | rdm;
                //
                let m_as_gpr = Arm32GeneralPurposeRegister::from_operand_bits(m);
                //
                return Ok(Some(Self::Add_SpPlusRegister_T1(m_as_gpr)));
            },
            _ => (), // continue to next masked set
        }

        // mask: 0b1111_1111_1000_0111
        match halfword0 & 0b1111_1111_1000_0111 {
            op if op == ArmT32OpcodePattern_16Bit::Add_SpPlusRegister_T2 as u16 => {
                let rm = ArmT32InstructionDecoder::decode_instruction_halfword__0306(halfword0);
                return Ok(Some(Self::Add_SpPlusRegister_T2(Arm32GeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Blx_Register_T1 as u16 => {
                // NOTE: ::Blx_Register_T1's mask excluding the "(#)" bits is: 0b1111_1111_1000_0000; as we expand the ARMT32 instruction set, we may want to switch this to match against the no-#-bits mask (e.g. 0b1111_1111_1000_0000)
                let rm = ArmT32InstructionDecoder::decode_instruction_halfword__0306(halfword0);
                return Ok(Some(Self::Blx_Register_T1(Arm32GeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Bx_T1 as u16 => {
                // NOTE: ::Bx_T1's mask excluding the "(#)" bits is: 0b1111_1111_1000_0000; as we expand the ARMT32 instruction set, we may want to switch this to match against the no-#-bits mask (e.g. 0b1111_1111_1000_0000)
                let rm = ArmT32InstructionDecoder::decode_instruction_halfword__0306(halfword0);
                return Ok(Some(Self::Bx_T1(Arm32GeneralPurposeRegister::from_operand_bits(rm))));
            },
            // ARMv8-M: BXNS / BLXNS (bit 2 set vs BX / BLX)
            op if op == ArmT32OpcodePattern_16Bit::Bxns_T1 as u16 => {
                let rm = ArmT32InstructionDecoder::decode_instruction_halfword__0306(halfword0);
                return Ok(Some(Self::Bxns_T1(Arm32GeneralPurposeRegister::from_operand_bits(rm))));
            },
            op if op == ArmT32OpcodePattern_16Bit::Blxns_T1 as u16 => {
                let rm = ArmT32InstructionDecoder::decode_instruction_halfword__0306(halfword0);
                return Ok(Some(Self::Blxns_T1(Arm32GeneralPurposeRegister::from_operand_bits(rm))));
            },

            _ => (), // continue to next masked set
        }

        // mask: 0b1111_0000_0000_0000
        match halfword0 & 0b1111_0000_0000_0000 {
            op if op == ArmT32OpcodePattern_16Bit::B_T1 as u16 => {
                let (encoded_signed_imm8, cond) = ArmT32InstructionDecoder::decode_instruction_halfword__s0007__080b(halfword0);
                //
                // NOTE: if this instruction is encoded with a cond value of 0b1110 or 0b1111, it is a UDF or SVC instruction respectively
                //       [we should have already parsed those instructions earlier in this function, but these are failsafes to allow us to rearrange the order of our filtering without side-effects]
                match cond {
                    0b1110 => {
                        // NOTE: imm8 is interpreted an unsinged integer (i.e. zero-extended) for the Udf_T1 instruction
                        let imm8_as_u8 = encoded_signed_imm8 as u8;
                        return Ok(Some(Self::Udf_T1(imm8_as_u8)));
                    },
                    0b1111 => {
                        // NOTE: imm8 is interpreted an unsinged integer (i.e. zero-extended) for the Svc_T1 instruction
                        let imm8_as_u8 = encoded_signed_imm8 as u8;
                        return Ok(Some(Self::Svc_T1(imm8_as_u8)));
                    },
                    _ => {
                        let decoded_signed_imm9 = (encoded_signed_imm8 as i16) * 2;

                        return Ok(Some(Self::B_T1(ArmT32InstructionCondition::from_operand_bits(cond), decoded_signed_imm9)));
                    }
                }
            },
            _ => (), // continue to next masked set
        }

        // mask: 0b1111_1111_1110_1111
        match halfword0 & 0b1111_1111_1110_1111 {
            op if op == ArmT32OpcodePattern_16Bit::Cps_T1 as u16 => {
                // NOTE: Cps_T1's true mask excluding the "(#)" bits is: 0b1111_1111_1110_0000
                let im = ArmT32InstructionDecoder::decode_instruction_halfword__0404(halfword0);
                let primask_effect = ArmT32CpsPrimaskEffect::from_operand_bits(im);
                return Ok(Some(Self::Cps_T1(primask_effect)));
            },
            _ => (), // continue to next masked set
        }

        // mask: 0b1111_1111_1111_0000
        match halfword0 & 0b1111_1111_1111_0000 {
            op if op == ((ArmT32OpcodePattern_32Bit::Msr_Register_T1 as u32) >> 16) as u16 => {
                matched_first_16_bits_of_32_bit_mask = true;
            },
            op if op == ((ArmT32OpcodePattern_32Bit::Udf_T2 as u32) >> 16) as u16 => {
                matched_first_16_bits_of_32_bit_mask = true;
            },
            _ => (), // continue to next masked set
        }

        // mask: 0b1111_1111_1111_1111
        match halfword0 {
            op if op == ArmT32OpcodePattern_16Bit::Nop_T1 as u16 => {
                return Ok(Some(Self::Nop_T1));
            },
            op if op == ArmT32OpcodePattern_16Bit::Sev_T1 as u16 => {
                return Ok(Some(Self::Sev_T1));
            },
            op if op == ArmT32OpcodePattern_16Bit::Wfe_T1 as u16 => {
                return Ok(Some(Self::Wfe_T1));
            },
            op if op == ArmT32OpcodePattern_16Bit::Wfi_T1 as u16 => {
                return Ok(Some(Self::Wfi_T1));
            },
            op if op == ArmT32OpcodePattern_16Bit::Yield_T1 as u16 => {
                return Ok(Some(Self::Yield_T1));
            },
            //
            op if op == ((ArmT32OpcodePattern_32Bit::Dmb_T1 as u32) >> 16) as u16 => {
                matched_first_16_bits_of_32_bit_mask = true;
            },
            op if op == ((ArmT32OpcodePattern_32Bit::Dsb_T1 as u32) >> 16) as u16 => {
                matched_first_16_bits_of_32_bit_mask = true;
            },
            op if op == ((ArmT32OpcodePattern_32Bit::Isb_T1 as u32) >> 16) as u16 => {
                matched_first_16_bits_of_32_bit_mask = true;
            },
            op if op == ((ArmT32OpcodePattern_32Bit::Mrs_T1 as u32) >> 16) as u16 => {
                matched_first_16_bits_of_32_bit_mask = true;
            },           
            _ => (), // continue to next masked set
        }

        // M7n IT (If-Then): 0xBF__ with a NONZERO low nibble (the mask). A zero low nibble is a hint
        // (NOP/YIELD/WFE/WFI/SEV), already matched above.
        if halfword0 & 0xFF00 == 0xBF00 && (halfword0 & 0x000F) != 0 {
            let firstcond = ((halfword0 >> 4) & 0xF) as u8;
            let mask = (halfword0 & 0xF) as u8;
            return Ok(Some(Self::It_T1(ArmT32InstructionCondition::from_operand_bits(firstcond), mask)));
        }

        // M7m CBZ / CBNZ (16-bit, ARMv7-M): forward compare-and-branch.  mask: 0b1111_1101_0000_0000
        match halfword0 & 0b1111_1101_0000_0000 {
            0xB100 => { let (rn, offset) = decode_halfword_compare_branch(halfword0); return Ok(Some(Self::Cbz_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rn), offset))); },
            0xB900 => { let (rn, offset) = decode_halfword_compare_branch(halfword0); return Ok(Some(Self::Cbnz_T1(Arm32LowGeneralPurposeRegister::from_operand_bits(rn), offset))); },
            _ => (),
        }

        // Architectural fallback: per the ARM ARM, any halfword whose top five bits are 0b11101 / 0b11110
        // / 0b11111 is the first halfword of a 32-bit Thumb instruction. The per-instruction detection
        // above covers the ARMv6-M 32-bit forms; this catches the ARMv7-M 32-bit additions (and any other
        // 32-bit lead) so they reach the 32-bit decode section below.
        if !matched_first_16_bits_of_32_bit_mask {
            let leading_five_bits = halfword0 >> 11;
            if leading_five_bits == 0b11101 || leading_five_bits == 0b11110 || leading_five_bits == 0b11111 {
                matched_first_16_bits_of_32_bit_mask = true;
            }
        }

        if matched_first_16_bits_of_32_bit_mask {
            let halfword1 = match next_u16le_from_iter(iter, iter_offset) {
                Ok(value) => match value {
                    Some(some_value) => some_value,
                    None => return Err(DecodeError::InvalidOpcode), // if we could not capture any additional bytes, let the caller know the opcode was invalid
                },
                Err(error) => return Err(error), // bubble up error to our caller
            };

            let word = combine_instruction_halfwords_into_word(&[halfword0, halfword1]);

            /* 32-bit instructions */

            // ---- VMOVX / VINS (Armv8.1-M half-precision FP, single-precision regs) -- bit28=1 keeps these out
            // of the VFP VMOV.F32 space (0xEE); insert=bit7. Decoded before the MVE blocks (0xFE space). ----
            if word & MVE_VMOVX_MASK == MVE_VMOVX_BASE {
                let sd = Arm32SinglePrecisionRegister::from_field_and_bit((word >> 12) & 0xF, (word >> 22) & 1);
                let sm = Arm32SinglePrecisionRegister::from_field_and_bit(word & 0xF, (word >> 5) & 1);
                return Ok(Some(Self::Vmovx_T1((word >> 7) & 1 == 1, sd, sm)));
            }
            // VRINTR (FP round to integral): opc2=0110, [11:8]=1010 (.f32) / 1011 (.f64). Sd=Vd:D, Sm=Vm:M.
            if word & 0xEEBF_0ED0 == 0xEEB6_0A40 {
                let (vd, d, vm, m) = ((word >> 12) & 0xF, (word >> 22) & 1, word & 0xF, (word >> 5) & 1);
                return Ok(Some(if (word >> 8) & 1 == 1 {
                    Self::Vrintr_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd, d), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m))
                } else {
                    Self::Vrintr_Single_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd, d), Arm32SinglePrecisionRegister::from_field_and_bit(vm, m))
                }));
            }
            // VRINTZ (round toward zero): opc2=0110 like VRINTR, but [7]=1.
            if word & 0xEEBF_0ED0 == 0xEEB6_0AC0 {
                let (vd, d, vm, m) = ((word >> 12) & 0xF, (word >> 22) & 1, word & 0xF, (word >> 5) & 1);
                return Ok(Some(if (word >> 8) & 1 == 1 {
                    Self::Vrintz_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd, d), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m))
                } else {
                    Self::Vrintz_Single_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd, d), Arm32SinglePrecisionRegister::from_field_and_bit(vm, m))
                }));
            }
            // VRINTX (round to integral, signalling inexact): opc2=0111, [7]=0.
            if word & 0xEEBF_0ED0 == 0xEEB7_0A40 {
                let (vd, d, vm, m) = ((word >> 12) & 0xF, (word >> 22) & 1, word & 0xF, (word >> 5) & 1);
                return Ok(Some(if (word >> 8) & 1 == 1 {
                    Self::Vrintx_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd, d), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m))
                } else {
                    Self::Vrintx_Single_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd, d), Arm32SinglePrecisionRegister::from_field_and_bit(vm, m))
                }));
            }
            // VJCVT (VJCVTZS): opc2=1001, [7:6]=11 -- JS f64 -> s32. Sd (result) = Vd:D, Dm (source) = Vm:M.
            if word & 0xFFBF_0FD0 == 0xEEB9_0BC0 {
                let sd = Arm32SinglePrecisionRegister::from_field_and_bit((word >> 12) & 0xF, (word >> 22) & 1);
                let dm = Arm32DoublePrecisionRegister::from_field_and_bit(word & 0xF, (word >> 5) & 1);
                return Ok(Some(Self::Vjcvt_T1(sd, dm)));
            }

            // ---- ARMv8-M hints/barriers, CLRM, and VSEL (decoded early: SSBB/PSSBB precede the generic DSB,
            //      CLRM precedes LDMIA, VSEL is in the 0xFE FP space). ----
            if word & 0xFFFF_FFF0 == 0xF3AF_80F0 { return Ok(Some(Self::Dbg_T1((word & 0xF) as u8))); }
            if word == 0xF3AF_8010 { return Ok(Some(Self::Esb_T1)); }
            // PACBTI hints (no operands) and data-processing (PACG/AUTG/BXAUT) -- decoded before the multiplies.
            if word == 0xF3AF_800F { return Ok(Some(Self::PacbtiHint_T1(0))); } // bti
            if word == 0xF3AF_801D { return Ok(Some(Self::PacbtiHint_T1(1))); } // pac
            if word == 0xF3AF_802D { return Ok(Some(Self::PacbtiHint_T1(2))); } // aut
            if word == 0xF3AF_800D { return Ok(Some(Self::PacbtiHint_T1(3))); } // pacbti
            {
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                let rn = g(((word >> 16) & 0xF) as u8);
                let rm = g((word & 0xF) as u8);
                if word & 0xFFF0_F0F0 == 0xFB60_F000 {   // PACG: Rd[11:8]
                    return Ok(Some(Self::PacbtiData_T1(0, g(((word >> 8) & 0xF) as u8), rn, rm)));
                }
                if word & 0xFFF0_0FF0 == 0xFB50_0F00 {   // AUTG: [11:8]=1111, Rd[15:12], [7:4]=0000
                    return Ok(Some(Self::PacbtiData_T1(1, g(((word >> 12) & 0xF) as u8), rn, rm)));
                }
                if word & 0xFFF0_0FF0 == 0xFB50_0F10 {   // BXAUT: [11:8]=1111, Rd[15:12], [7:4]=0001
                    return Ok(Some(Self::PacbtiData_T1(2, g(((word >> 12) & 0xF) as u8), rn, rm)));
                }
            }
            if word == 0xF3BF_8F40 { return Ok(Some(Self::Ssbb_T1)); }
            if word == 0xF3BF_8F44 { return Ok(Some(Self::Pssbb_T1)); }
            if word == 0xF3BF_8F70 { return Ok(Some(Self::Sb_T1)); }
            // VSCCLRM (FP secure context clear): Rn=15 marker, [11:8]=1010(single)/1011(double), Vd:D = first
            // FP reg, imm8 = count. Decoded before the generic VLDM block (which would read Rn=PC).
            if word & 0xFFBF_0E01 == 0xEC9F_0A00 {
                let double = (word >> 8) & 1 == 1;
                let (vd, d) = (((word >> 12) & 0xF) as u8, ((word >> 22) & 1) as u8);
                let first = if double { (d << 4) | vd } else { (vd << 1) | d };
                return Ok(Some(Self::Vscclrm_T1(double, first, (word & 0xFF) as u8)));
            }
            if word & 0xFFFF_2000 == 0xE89F_0000 { return Ok(Some(Self::Clrm_T1((word & 0xDFFF) as u16))); }
            if word & 0xFE80_0E50 == 0xFE00_0A00 {
                let cond = ((word >> 20) & 0b11) as u8;
                let (vd, vn, vm) = ((word >> 12) & 0xF, (word >> 16) & 0xF, word & 0xF);
                let (d, n, m) = ((word >> 22) & 1, (word >> 7) & 1, (word >> 5) & 1);
                return Ok(Some(if (word >> 8) & 1 == 1 {
                    Self::Vsel_Double_T1(cond, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d), Arm32DoublePrecisionRegister::from_field_and_bit(vn, n), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m))
                } else {
                    Self::Vsel_Single_T1(cond, Arm32SinglePrecisionRegister::from_field_and_bit(vd, d), Arm32SinglePrecisionRegister::from_field_and_bit(vn, n), Arm32SinglePrecisionRegister::from_field_and_bit(vm, m))
                }));
            }
            // VMAXNM / VMINNM (FP): [23]=1 + [21:20]=00 (vs VSEL's [23]=0 and VRINT-directed's [21:20]=11). op[6]: 0=max/1=min, size[8].
            if word & 0xFFB0_0E10 == 0xFE80_0A00 {
                let is_min = (word >> 6) & 1 == 1;
                let (vd, vn, vm) = ((word >> 12) & 0xF, (word >> 16) & 0xF, word & 0xF);
                let (d, n, m) = ((word >> 22) & 1, (word >> 7) & 1, (word >> 5) & 1);
                return Ok(Some(if (word >> 8) & 1 == 1 {
                    let (dd, dn, dm) = (Arm32DoublePrecisionRegister::from_field_and_bit(vd, d), Arm32DoublePrecisionRegister::from_field_and_bit(vn, n), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m));
                    if is_min { Self::Vminnm_Double_T1(dd, dn, dm) } else { Self::Vmaxnm_Double_T1(dd, dn, dm) }
                } else {
                    let (sd, sn, sm) = (Arm32SinglePrecisionRegister::from_field_and_bit(vd, d), Arm32SinglePrecisionRegister::from_field_and_bit(vn, n), Arm32SinglePrecisionRegister::from_field_and_bit(vm, m));
                    if is_min { Self::Vminnm_Single_T1(sd, sn, sm) } else { Self::Vmaxnm_Single_T1(sd, sn, sm) }
                }));
            }
            // VRINTA/N/P/M (directed FP round): [21:18]=1110 + [6]=1 (vs VMAXNM's [21:20]=00; [18]=0 vs VCVT-directed's [18]=1). mode[17:16], size[8].
            if word & 0xFFBC_0ED0 == 0xFEB8_0A40 {
                let mode = Arm32DirectedRound::from_rm_bits((word >> 16) & 0b11);
                let (vd, vm) = ((word >> 12) & 0xF, word & 0xF);
                let (d, m) = ((word >> 22) & 1, (word >> 5) & 1);
                return Ok(Some(if (word >> 8) & 1 == 1 {
                    Self::Vrint_Directed_Double_T1(mode, Arm32DoublePrecisionRegister::from_field_and_bit(vd, d), Arm32DoublePrecisionRegister::from_field_and_bit(vm, m))
                } else {
                    Self::Vrint_Directed_Single_T1(mode, Arm32SinglePrecisionRegister::from_field_and_bit(vd, d), Arm32SinglePrecisionRegister::from_field_and_bit(vm, m))
                }));
            }
            // VCVTA/N/P/M (directed FP->int): the [18]=1 sibling of VRINT-directed ([18]=0). [7]=signed is left
            // free by the mask; mode[17:16], size[8] (0=f32 source, 1=f64). Result is always a single-word Sd.
            if word & 0xFFBC_0E50 == 0xFEBC_0A40 {
                let mode = Arm32DirectedRound::from_rm_bits((word >> 16) & 0b11);
                let signed = (word >> 7) & 1 == 1;
                let (vd, vm) = ((word >> 12) & 0xF, word & 0xF);
                let (d, m) = ((word >> 22) & 1, (word >> 5) & 1);
                let sd = Arm32SinglePrecisionRegister::from_field_and_bit(vd, d);
                return Ok(Some(if (word >> 8) & 1 == 1 {
                    Self::Vcvt_Directed_FromDouble_T1(mode, sd, Arm32DoublePrecisionRegister::from_field_and_bit(vm, m), signed)
                } else {
                    Self::Vcvt_Directed_FromSingle_T1(mode, sd, Arm32SinglePrecisionRegister::from_field_and_bit(vm, m), signed)
                }));
            }
            // ARMv8.1-M MVE scalar-shift extension (all 0xEA5x). Decode order matters: the SHORT forms (single
            // GPR, [11:8]=1111) MUST precede the plain LONG forms (GPR pair, [11:8]=RdaHi:1 -- whose loose [8]=1
            // mask would otherwise swallow [11:8]=1111). op[5:4]; imm5=immh[14:12]:imml[7:6]; Rm[15:12]; long
            // forms use RdaLo[19:17]<<1 with [16]=1; long-register saturation-to-48 = bit7. RdaHi=111 (PC) is
            // reassigned to the short forms by the architecture, so [11:8]=1111 is unambiguously SHORT.
            if word & 0xFFF0_8F0F == 0xEA50_0F0F {           // SQSHL/UQSHL/SRSHR/URSHR (short, immediate)
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                let imm = (((word >> 12) & 0b111) << 2) | ((word >> 6) & 0b11);
                return Ok(Some(Self::SatShiftImm_T1(((word >> 4) & 0b11) as u8, g(((word >> 16) & 0xF) as u8), imm as u8)));
            }
            if word & 0xFFF0_0FDF == 0xEA50_0F0D {           // SQRSHR/UQRSHL (short, register)
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                return Ok(Some(Self::SatShiftReg_T1((word >> 5) & 1 == 1, g(((word >> 16) & 0xF) as u8), g(((word >> 12) & 0xF) as u8))));
            }
            if word & 0xFFF1_810F == 0xEA50_010F && (word >> 4) & 0b11 != 0b11 {  // LSLL/LSRL/ASRL (plain long, imm; [16]=0; [5:4]=11 reserved)
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                let rdalo = g(((word >> 16) & 0xF) as u8);
                let rdahi = g(((((word >> 9) & 0b111) << 1) | 1) as u8);
                let imm = (((word >> 12) & 0b111) << 2) | ((word >> 6) & 0b11);
                return Ok(Some(Self::LongShiftImm_T1(((word >> 4) & 0b11) as u8, rdalo, rdahi, imm as u8)));
            }
            if word & 0xFFF1_01CF == 0xEA50_010D && (word >> 4) & 0b11 != 0b11 {  // LSLL/LSRL/ASRL (plain long, reg; [16]=0; [5:4]=11 reserved)
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                let rdalo = g(((word >> 16) & 0xF) as u8);
                let rdahi = g(((((word >> 9) & 0b111) << 1) | 1) as u8);
                return Ok(Some(Self::LongShiftReg_T1(((word >> 4) & 0b11) as u8, rdalo, rdahi, g(((word >> 12) & 0xF) as u8))));
            }
            if word & 0xFFF1_810F == 0xEA51_010F {           // SQSHLL/UQSHLL/SRSHRL/URSHRL (long, immediate; [16]=1)
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                let rdalo = g((((word >> 17) & 0b111) << 1) as u8);
                let rdahi = g(((((word >> 9) & 0b111) << 1) | 1) as u8);
                let imm = (((word >> 12) & 0b111) << 2) | ((word >> 6) & 0b11);
                return Ok(Some(Self::SatShiftLongImm_T1(((word >> 4) & 0b11) as u8, rdalo, rdahi, imm as u8)));
            }
            if word & 0xFFF1_015F == 0xEA51_010D {           // SQRSHRL/UQRSHLL (long, register, #sat 64|48; [16]=1)
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                let rdalo = g((((word >> 17) & 0b111) << 1) as u8);
                let rdahi = g(((((word >> 9) & 0b111) << 1) | 1) as u8);
                return Ok(Some(Self::SatShiftLongReg_T1((word >> 5) & 1 == 1, rdalo, rdahi, g(((word >> 12) & 0xF) as u8), (word >> 7) & 1 == 1)));
            }
            // CSEL/CSINC/CSINV/CSNEG (ARMv8.1-M): [31:20]=0xEA5, Rn[19:16]; hw2 [15:14]=10, op2[13:12], Rd[11:8],
            // cond[7:4], Rm[3:0]. hw2[15]=1 separates these from ORRS.W ([15]=0). Decoded before that family.
            if word & 0xFFF0_C000 == 0xEA50_8000 {
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                return Ok(Some(Self::Csel_T1(
                    ((word >> 12) & 0b11) as u8,
                    g(((word >> 8) & 0xF) as u8), g(((word >> 16) & 0xF) as u8), g((word & 0xF) as u8),
                    ArmT32InstructionCondition::from_operand_bits(((word >> 4) & 0xF) as u8),
                )));
            }

            // ---- ARMv8-M Security Extension (decoded first: SG sits in the LDM/STM encoding space) ----
            if word == ArmT32OpcodePattern_32Bit::Csdb_T1 as u32 {
                return Ok(Some(Self::Csdb_T1));
            }
            if word == ArmT32OpcodePattern_32Bit::Sg_T1 as u32 {
                return Ok(Some(Self::Sg_T1));
            }
            // TT / TTT / TTA / TTAT : Rd[11:8], Rn[19:16], A[7], T[6]
            if word & 0b1111_1111_1111_0000_1111_0000_0011_1111 == ArmT32OpcodePattern_32Bit::Tt_T1 as u32 {
                let rn = ((word >> 16) & 0xF) as u8;
                let rd = ((word >> 8) & 0xF) as u8;
                return Ok(Some(Self::Tt_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), (word >> 7) & 1 == 1, (word >> 6) & 1 == 1)));
            }
            // VLSTM / VLLDM : Rn[19:16]
            if word & 0b1111_1111_1111_0000_1111_1111_1111_1111 == ArmT32OpcodePattern_32Bit::Vlstm_T1 as u32 {
                return Ok(Some(Self::Vlstm_T1(Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8))));
            }
            if word & 0b1111_1111_1111_0000_1111_1111_1111_1111 == ArmT32OpcodePattern_32Bit::Vlldm_T1 as u32 {
                return Ok(Some(Self::Vlldm_T1(Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8))));
            }

            // ---- ARMv8.1-M MVE "three registers of the same length" (vector-vector) ----
            // These occupy the 0xEF.. / 0xFF.. space (top byte 1110_1111 / 1111_1111), which is otherwise
            // unused by M-profile (no NEON), so intercepting it here cannot shadow any existing instruction.
            // The three sub-families are disjoint in bits[11:8]+bit4; try float, then bitwise, then integer.
            {
                let top_byte = word >> 24;
                if top_byte == 0xEF || top_byte == 0xFF {
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                    let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                    // float: element size is the single bit 20
                    if let Some(op) = Arm32MveFloatArithOp::from_signature(word & MVE_FLOAT_SIGNATURE_MASK) {
                        let size = Arm32MveFloatSize::from_size_bit((word >> 20) & 1);
                        return Ok(Some(Self::MveFloatArith(op, size, qd, qn, qm)));
                    }
                    // bitwise: not size-parametric (bits[21:20] are part of the signature)
                    if let Some(op) = Arm32MveBitwiseOp::from_signature(word & MVE_BITWISE_SIGNATURE_MASK) {
                        return Ok(Some(Self::MveBitwise(op, qd, qn, qm)));
                    }
                    // integer: element size in bits[21:20]; 0b11 is reserved (rejected by from_size_bits)
                    if let Some(op) = Arm32MveIntArithOp::from_signature(word & MVE_INT_SIGNATURE_MASK)
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            return Ok(Some(Self::MveIntArith(op, size, qd, qn, qm)));
                        }
                    // VSHL/VRSHL/VQSHL/VQRSHL by vector (Qd, Qm, Qn): rounding=bit8, saturating=bit4
                    if word & MVE_SHIFT_VEC_MASK == MVE_SHIFT_VEC_BASE
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            return Ok(Some(Self::MveShiftByVector(
                                (word >> 8) & 1 == 1, (word >> 4) & 1 == 1, (word >> 28) & 1 == 1, size, qd, qm, qn,
                            )));
                        }
                }
            }

            // ---- MVE shift by immediate (Qd, Qm, #amount), in the 0xEF8x / 0xFF8x space ----
            // imm6 = bits[21:16]; imm6 >= 8 (a size bit set) is what tells a shift apart from the one-register
            // modified-immediate (which always has imm6 < 8). The size is imm6's highest set bit; the amount
            // is derived per direction.
            {
                let top_byte = word >> 24;
                if top_byte == 0xEF || top_byte == 0xFF {
                    let imm6 = (word >> 16) & 0x3F;
                    if let Some(size) = mve_shift_size_from_imm6(imm6)
                        && let Some(op) = Arm32MveShiftImmOp::from_signature(word & MVE_SHIFT_SIGNATURE_MASK) {
                            let esize = mve_shift_esize(size);
                            let amount = if op.is_left_shift() { imm6 - esize } else { 2 * esize - imm6 };
                            let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                            let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                            return Ok(Some(Self::MveShiftImm(op, size, amount as u8, qd, qm)));
                        }
                }
            }

            // ---- MVE one-register modified immediate (VMOV/VMVN/VORR/VBIC #imm, VMOV.f32) ----
            // Same 0xEF8x/0xFF8x space as the shifts; the fixed pattern below (which requires imm6[21:19]=000)
            // is what distinguishes it from a shift. (cmode, op, imm8) are carried raw; the emitter maps them
            // to the mnemonic / element size / value via AdvSIMDExpandImm.
            if word & 0xEFF8_10D0 == 0xEF80_0050 {
                let i = (word >> 28) & 1;
                let imm3 = (word >> 16) & 0b111;
                let imm4 = word & 0xF;
                let imm8 = ((i << 7) | (imm3 << 4) | imm4) as u8;
                let cmode = ((word >> 8) & 0xF) as u8;
                let op = (word >> 5) & 1 == 1;
                let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                return Ok(Some(Self::MveModifiedImmediate(cmode, op, imm8, qd)));
            }

            // ---- MVE fixed-point VCVT (Qd, Qm, #fbits) ---- decoded before the 0xFFBx 2-reg-misc block so the
            // U=1 forms (which fall in 0xFFBx) are claimed here by the tighter signature.
            if word & MVE_VCVT_FIXED_MASK == MVE_VCVT_FIXED_BASE {
                let imm6 = (word >> 16) & 0x3F;
                let fbits = (64 - imm6) as u8;
                let size = if (word >> 9) & 1 == 1 { Arm32MveFloatSize::F32 } else { Arm32MveFloatSize::F16 };
                // .16 forms only reach fbits 1..=16 (imm6 >= 48); reject the out-of-range .16 encodings
                if matches!(size, Arm32MveFloatSize::F32) || imm6 >= 48 {
                    let to_fixed = (word >> 8) & 1 == 1;
                    let unsigned = (word >> 28) & 1 == 1;
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                    return Ok(Some(Self::MveVcvtFixed(to_fixed, unsigned, size, fbits, qd, qm)));
                }
            }

            // ---- MVE two-register miscellaneous (Qd, Qm), in the 0xFFBx space ----
            if word & 0xFFF0_0000 == 0xFFB0_0000 {
                let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                // VMVN (register): an exact opcode with no element size
                if word & MVE_VMVN_REG_MASK == MVE_VMVN_REG_BASE {
                    return Ok(Some(Self::MveMvnRegister(qd, qm)));
                }
                let size_bits = (word >> 18) & 0b11;
                // float VABS/VNEG first (hw0 bit16, the int/float marker, separates them from VQABS/VQNEG)
                if let Some(op) = Arm32MveMisc2FloatOp::from_signature(word & MVE_MISC2_SIGNATURE_MASK)
                    && let Some(size) = mve_misc2_float_size_from_bits(size_bits) {
                        return Ok(Some(Self::MveMisc2Float(op, size, qd, qm)));
                    }
                // sized integer ops
                if let Some(op) = Arm32MveMisc2Op::from_signature(word & MVE_MISC2_SIGNATURE_MASK)
                    && let Some(size) = Arm32MveSize::from_size_bits(size_bits) {
                        return Ok(Some(Self::MveMisc2(op, size, qd, qm)));
                    }
                // VCVT float<->int (hw0 0xFFB3, distinguished from VRINT's 0xFFB2 by bit16)
                if word & MVE_VCVT_FI_FIXED_MASK == MVE_VCVT_FI_FIXED_PATTERN
                    && let Some(size) = mve_misc2_float_size_from_bits(size_bits) {
                        let to_int = (word >> 8) & 1 == 1;
                        let unsigned = (word >> 7) & 1 == 1;
                        return Ok(Some(Self::MveVcvtFloatInt(to_int, unsigned, size, qd, qm)));
                    }
                // VRINT (round to integral float)
                if let Some(op) = Arm32MveVrintOp::from_signature(word & MVE_MISC2_SIGNATURE_MASK)
                    && let Some(size) = mve_misc2_float_size_from_bits(size_bits) {
                        return Ok(Some(Self::MveVrint(op, size, qd, qm)));
                    }
                // VCVTA/N/P/M (float -> int with rounding mode = bits[9:8]; signedness = bit7)
                if word & MVE_VCVTR_MASK == MVE_VCVTR_BASE
                    && let Some(size) = mve_misc2_float_size_from_bits(size_bits) {
                        let rounding = ((word >> 8) & 0b11) as u8;
                        let unsigned = (word >> 7) & 1 == 1;
                        return Ok(Some(Self::MveVcvtRound(rounding, unsigned, size, qd, qm)));
                    }
            }

            // ---- MVE VMAXA/VMINA (int) and VMAXNMA/VMINNMA (float), 2-register elementwise (Qda, Qm) in the
            // 0xEE space. [19:18]=11 escapes to the FP twins (precision [28]: f16=1/f32=0); [12] selects min.
            if word & 0xEFF3_0FF1 == 0xEE33_0E81 {
                let qda = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                let is_min = (word >> 12) & 1 == 1;
                let size_bits = (word >> 18) & 0b11;
                if size_bits == 0b11 {
                    let size = Arm32MveFloatSize::from_size_bit((word >> 28) & 1);
                    return Ok(Some(Self::MveVmaxnmaMinnma(is_min, size, qda, qm)));
                }
                if let Some(size) = Arm32MveSize::from_size_bits(size_bits) {
                    return Ok(Some(Self::MveVmaxaMina(is_min, size, qda, qm)));
                }
            }

            // ---- MVE index generators VIDUP/VDDUP/VIWDUP/VDWDUP (0xEE space) ---- claimed first via the tight
            // signature; its [11:8]=1111 / [6:4]=110 / bit16=1 frame is disjoint from the vector-by-scalar ops.
            if word & MVE_VIDDUP_MASK == MVE_VIDDUP_BASE
                && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                    let decrement = (word >> 12) & 1 == 1;
                    let rn = Arm32GeneralPurposeRegister::from_operand_bits((((word >> 17) & 0b111) << 1) as u8);
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let imm2 = (((word >> 7) & 1) << 1) | (word & 1);
                    let step = (1u32 << imm2) as u8;
                    let rm_field = (word >> 1) & 0b111;
                    let wrap_rm = if rm_field == 0b111 { None } else { Some(Arm32GeneralPurposeRegister::from_operand_bits(((rm_field << 1) | 1) as u8)) };
                    return Ok(Some(Self::MveViddup(decrement, size, qd, rn, wrap_rm, step)));
                }

            // ---- MVE VBRSR (broadcast shift by GPR; vector-by-scalar shape) ----
            if word & MVE_VBRSR_MASK == MVE_VBRSR_BASE
                && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                    let rm = Arm32GeneralPurposeRegister::from_operand_bits((word & 0xF) as u8);
                    return Ok(Some(Self::MveVbrsr(size, qd, qn, rm)));
                }

            // ---- MVE vector-by-scalar (Qd, Qn, Rm) and VDUP (Qd, Rt), in the 0xEE.. / 0xFE.. space ----
            // Disjoint from the scalar FPU (whose coprocessor field bits[11:8] are A/B, not the E/F of the
            // vector-by-scalar ops); the tight per-op signature match below claims only genuine MVE words, so
            // any real VFP/VMOV word falls through to its existing decode further down.
            {
                let top_byte = word >> 24;
                if top_byte == 0xEE || top_byte == 0xFE {
                    // VDUP first (its bits[11:8]=1011 + bit4 form is distinct from both vbs and VFP)
                    if word & MVE_VDUP_MASK == MVE_VDUP_BASE
                        && let Some(size) = mve_vdup_size_from_bits((word >> 22) & 1, (word >> 5) & 1) {
                            let qd = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                            let rt = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 12) & 0xF) as u8);
                            return Ok(Some(Self::MveVdup(size, qd, rt)));
                        }
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                    let rm = Arm32GeneralPurposeRegister::from_operand_bits((word & 0xF) as u8);
                    // float vector-by-scalar (bits[21:20] = 0b11, size in bit 28)
                    if let Some(op) = Arm32MveVecScalarFloatOp::from_signature(word & MVE_VBS_FLOAT_SIGNATURE_MASK) {
                        let size = Arm32MveFloatSize::from_size_bit((word >> 28) & 1);
                        return Ok(Some(Self::MveVecScalarFloat(op, size, qd, qn, rm)));
                    }
                    // integer vector-by-scalar (size in bits[21:20]; 0b11 is the float marker, rejected here)
                    if let Some(op) = Arm32MveVecScalarIntOp::from_signature(word & MVE_VBS_INT_SIGNATURE_MASK)
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            return Ok(Some(Self::MveVecScalarInt(op, size, qd, qn, rm)));
                        }
                }
            }

            // ---- MVE cross-lane reductions to a GPR (VADDV/VMINV/VMAXV/... and VABAV), 0xEE.. / 0xFE.. ----
            {
                let top_byte = word >> 24;
                if top_byte == 0xEE || top_byte == 0xFE {
                    // VABAV: Rd[15:12], Qn[19:17], Qm[3:1], size[21:20] (distinct fixed bit 0 = 1)
                    if word & MVE_VABAV_SIGNATURE_MASK == MVE_VABAV_BASE
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            let signed = (word >> 28) & 1 == 0;
                            let rd = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 12) & 0xF) as u8);
                            let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                            let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                            return Ok(Some(Self::MveVabav(signed, size, rd, qn, qm)));
                        }
                    // 2-operand reductions: Rd[15:12], Qm[3:1], size[19:18] (rejects [19:18]=11, leaving those
                    // encodings for the float min/max reductions below)
                    if let Some(op) = Arm32MveReduceOp::from_signature(word & MVE_REDUCE_SIGNATURE_MASK)
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 18) & 0b11) {
                            let rd = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 12) & 0xF) as u8);
                            let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                            return Ok(Some(Self::MveReduce(op, size, rd, qm)));
                        }
                    // floating-point min/max reductions: Rd[15:12], Qm[3:1], size = bit28
                    if let Some(op) = Arm32MveFloatReduceOp::from_signature(word & MVE_FLOAT_REDUCE_SIGNATURE_MASK) {
                        let size = if (word >> 28) & 1 == 1 { Arm32MveFloatSize::F16 } else { Arm32MveFloatSize::F32 };
                        let rd = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 12) & 0xF) as u8);
                        let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                        return Ok(Some(Self::MveFloatReduce(op, size, rd, qm)));
                    }
                    // VMLADAV/VMLSDAV (non-long dual MAC reduction into an even GPR). Decoded AFTER the 2-operand
                    // reductions so VADDV/VADDVA (the only other [23:20]=1111 words) are already claimed. Rda is
                    // even: bit 12 of its field is the X (exchange) flag.
                    if word & MVE_DUALMAC_MASK == MVE_DUALMAC_BASE {
                        let subtract = word & 1 == 1;
                        if let Some((unsigned, size)) = mve_dualmac_decode_size(subtract, word) {
                            let exchange = (word >> 12) & 1 == 1;
                            let accumulate = (word >> 5) & 1 == 1;
                            // exchange is signed-only (subtract already excludes unsigned via decode_size)
                            if !(unsigned && exchange) {
                                let rda = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 12) & 0xE) as u8);
                                let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                                let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                                return Ok(Some(Self::MveDualMac(subtract, exchange, accumulate, unsigned, size, rda, qn, qm)));
                            }
                        }
                    }
                }
            }

            // ---- MVE width-changing register moves (VMOVL long, VMOVN narrow, VADDLV reduction) ----
            {
                let top_byte = word >> 24;
                if top_byte == 0xEE || top_byte == 0xFE {
                    // VADDLV (64-bit reduction to a GPR pair): RdLo[15:12] (even), RdHi>>1 at [22:20] (odd,
                    // independent of RdLo), U=bit28, accumulate=bit5
                    if word & MVE_VADDLV_MASK == MVE_VADDLV_BASE {
                        let accumulate = (word >> 5) & 1 == 1;
                        let unsigned = (word >> 28) & 1 == 1;
                        let rd_lo = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 12) & 0xF) as u8);
                        let rd_hi = Arm32GeneralPurposeRegister::from_operand_bits(((((word >> 20) & 0b111) << 1) | 1) as u8);
                        let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                        return Ok(Some(Self::MveVaddlv(accumulate, unsigned, rd_lo, rd_hi, qm)));
                    }
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                    // VSHLL T1 (imm widening) shares the VMOVL family base; decode the imm5 shift and claim it
                    // only when shift >= 1 (shift == 0 IS VMOVL, handled just below).
                    if word & MVE_VSHLL_T1_MASK == MVE_VSHLL_T1_BASE {
                        let imm5 = (word >> 16) & 0x1F;
                        let top2 = imm5 >> 3; // [20:19]
                        let (size, esize) = if top2 == 0b01 { (Arm32MveSize::I8, 8) } else { (Arm32MveSize::I16, 16) };
                        if top2 != 0 && imm5 > esize {
                            let shift = (imm5 - esize) as u8;
                            return Ok(Some(Self::MveVshll((word >> 12) & 1 == 1, (word >> 28) & 1 == 1, size, shift, qd, qm)));
                        }
                    }
                    // VMOVL (long): B/T=bit12, U=bit28, source size bit19(.8)/bit20(.16)
                    if word & MVE_VMOVL_MASK == MVE_VMOVL_BASE {
                        let size = if (word >> 19) & 1 == 1 { Arm32MveSize::I8 } else { Arm32MveSize::I16 };
                        return Ok(Some(Self::MveVmovl((word >> 12) & 1 == 1, (word >> 28) & 1 == 1, size, qd, qm)));
                    }
                    // VMOVN (narrow): T=bit12, source size bit18 (.16 = 0 / .32 = 1)
                    if word & MVE_VMOVN_MASK == MVE_VMOVN_BASE {
                        let size = if (word >> 18) & 1 == 1 { Arm32MveSize::I32 } else { Arm32MveSize::I16 };
                        return Ok(Some(Self::MveVmovn((word >> 12) & 1 == 1, size, qd, qm)));
                    }
                    // VQMOVN (saturating narrow, U=bit28 selects .s/.u): T=bit12, source size bit18
                    if word & MVE_VQMOVN_MASK == MVE_VQMOVN_BASE {
                        let size = if (word >> 18) & 1 == 1 { Arm32MveSize::I32 } else { Arm32MveSize::I16 };
                        return Ok(Some(Self::MveVqmovn(Arm32MveQMovnKind::Vqmovn, (word >> 28) & 1 == 1, (word >> 12) & 1 == 1, size, qd, qm)));
                    }
                    // VQMOVUN (signed source -> unsigned saturated): T=bit12, source size bit18, no U bit
                    if word & MVE_VQMOVUN_MASK == MVE_VQMOVUN_BASE {
                        let size = if (word >> 18) & 1 == 1 { Arm32MveSize::I32 } else { Arm32MveSize::I16 };
                        return Ok(Some(Self::MveVqmovn(Arm32MveQMovnKind::Vqmovun, false, (word >> 12) & 1 == 1, size, qd, qm)));
                    }
                    // VSHLL T2 (max shift == esize): size bit18 (.s8 = 0 / .s16 = 1), T=bit12, U=bit28
                    if word & MVE_VSHLL_T2_MASK == MVE_VSHLL_T2_BASE {
                        let (size, esize) = if (word >> 18) & 1 == 1 { (Arm32MveSize::I16, 16) } else { (Arm32MveSize::I8, 8) };
                        return Ok(Some(Self::MveVshll((word >> 12) & 1 == 1, (word >> 28) & 1 == 1, size, esize, qd, qm)));
                    }
                }
            }

            // ---- MVE long & high multiplies (VMULL int+poly, VMULH/VRMULH, VQDMULL vector+scalar) ----
            // These share the bit23 = 0 sub-space with VMOVN/VQMOVN (handled just above), so they are decoded
            // AFTER that block; they are disjoint from VADDLV/long-dual-MAC (bit23 = 1). Order within: VMULH
            // ([0]=1), VQDMULL scalar/vector ([11:8]=1111), then VMULL poly (size==11) BEFORE VMULL integer.
            {
                let top_byte = word >> 24;
                if top_byte == 0xEE || top_byte == 0xFE {
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                    let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                    // VMULH/VRMULH: rounding=bit12, U=bit28, size[21:20] (0b11 reserved -> rejected)
                    if word & MVE_VMULH_MASK == MVE_VMULH_BASE
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            return Ok(Some(Self::MveVmulh((word >> 12) & 1 == 1, (word >> 28) & 1 == 1, size, qd, qn, qm)));
                        }
                    // VQDMULL scalar (T2): [6:4]=110, Rm[3:0]; sz=bit28, T=bit12
                    if word & MVE_VQDMULL_SCALAR_MASK == MVE_VQDMULL_SCALAR_BASE {
                        let rm = Arm32GeneralPurposeRegister::from_operand_bits((word & 0xF) as u8);
                        return Ok(Some(Self::MveVqdmullScalar((word >> 12) & 1 == 1, (word >> 28) & 1 == 1, qd, qn, rm)));
                    }
                    // VQDMULL vector (T1): [11:8]=1111, [16]=0
                    if word & MVE_VQDMULL_VEC_MASK == MVE_VQDMULL_VEC_BASE {
                        return Ok(Some(Self::MveVqdmull((word >> 12) & 1 == 1, (word >> 28) & 1 == 1, qd, qn, qm)));
                    }
                    // VMULL polynomial (size[21:20]=11): bit28 = P8(0)/P16(1) -- MUST precede the integer form
                    if word & MVE_VMULL_POLY_MASK == MVE_VMULL_POLY_BASE {
                        let size = if (word >> 28) & 1 == 1 { Arm32MveSize::I16 } else { Arm32MveSize::I8 };
                        return Ok(Some(Self::MveVmull(true, false, (word >> 12) & 1 == 1, size, qd, qn, qm)));
                    }
                    // VMULL integer: U=bit28, size[21:20]
                    if word & MVE_VMULL_INT_MASK == MVE_VMULL_INT_BASE
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            return Ok(Some(Self::MveVmull(false, (word >> 28) & 1 == 1, (word >> 12) & 1 == 1, size, qd, qn, qm)));
                        }
                    // VQDMLADH/VQDMLSDH (+VQRD*): bit16=0 (vs VMULL/VMULH bit16=1); subtract=bit28, round=bit0, X=bit12
                    if word & MVE_VQDMLADH_MASK == MVE_VQDMLADH_BASE
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            return Ok(Some(Self::MveVqdmladh(
                                (word >> 28) & 1 == 1, word & 1 == 1, (word >> 12) & 1 == 1, size, qd, qn, qm,
                            )));
                        }
                    // VSHL/VRSHL/VQSHL/VQRSHL by GPR scalar (Qda, Rm): rounding=bit17, saturating=bit7, size[19:18]
                    if word & MVE_SHIFT_SCALAR_MASK == MVE_SHIFT_SCALAR_BASE
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 18) & 0b11) {
                            let rm = Arm32GeneralPurposeRegister::from_operand_bits((word & 0xF) as u8);
                            return Ok(Some(Self::MveShiftByScalar(
                                (word >> 17) & 1 == 1, (word >> 7) & 1 == 1, (word >> 28) & 1 == 1, size, qd, rm,
                            )));
                        }
                }
            }

            // ---- MVE LONG dual-MAC reductions (VMLALDAV/VMLSLDAV/VRMLALDAVH/VRMLSLDAVH) into a GPR pair ----
            // Decoded AFTER the cross-lane reductions (reduction block above) AND VADDLV (the width-changing
            // block above): all of those alias this looser signature, so they must be claimed first. The
            // complex/predication ops below all have bit23 = 0, which this signature requires to be 1.
            {
                let top_byte = word >> 24;
                if (top_byte == 0xEE || top_byte == 0xFE) && word & MVE_LONG_DUALMAC_MASK == MVE_LONG_DUALMAC_BASE {
                    let rda_hi_bits = (((word >> 20) & 0b111) << 1) | 1; // RdaHi is odd
                    if rda_hi_bits != 15 { // r15 (PC) is not a valid accumulator
                        if let Some((op, unsigned, size)) = mve_long_dualmac_decode(word) {
                            let exchange = (word >> 12) & 1 == 1;
                            // rounding-high exchange is signed-only
                            if !(op.rounding_high() && exchange && unsigned) {
                                let accumulate = (word >> 5) & 1 == 1;
                                let rda_lo = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 12) & 0xE) as u8);
                                let rda_hi = Arm32GeneralPurposeRegister::from_operand_bits(rda_hi_bits as u8);
                                let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                                let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                                return Ok(Some(Self::MveLongDualMac(op, exchange, accumulate, unsigned, size, rda_lo, rda_hi, qn, qm)));
                            }
                        }
                    }
                }
            }

            // ---- MVE complex-number ops (VCADD/VHCADD/VCMUL/VCMLA) ----
            // Four mutually-exclusive gate masks; all share Qd[15:13], Qn[19:17], Qm[3:1].
            {
                let top_byte = word >> 24;
                if matches!(top_byte, 0xEE | 0xFE | 0xFC | 0xFD) {
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                    let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                    // VCVT half<->single (vector): tight signature ([21:16]=111111)
                    if word & MVE_VCVT_HALF_MASK == MVE_VCVT_HALF_BASE {
                        return Ok(Some(Self::MveVcvtHalf((word >> 12) & 1 == 1, (word >> 28) & 1 == 1, qd, qm)));
                    }
                    // shift-and-narrow: the [7:6]/bit0 pair (rejecting [7:6]=00) disambiguates it from VABAV/VADDLV
                    if word & MVE_SHIFT_NARROW_MASK == MVE_SHIFT_NARROW_BASE
                        && let Some((op, unsigned)) = Arm32MveShiftNarrowOp::from_word((word >> 28) & 1, (word >> 6) & 0b11, word & 1) {
                            let imm5 = (word >> 16) & 0x1F;
                            let src_is_32 = imm5 >= 16; // sz = imm5[4]
                            let shift = ((if src_is_32 { 32 } else { 16 }) - imm5) as u8;
                            return Ok(Some(Self::MveShiftNarrow(op, unsigned, (word >> 12) & 1 == 1, src_is_32, shift, qd, qm)));
                        }
                    // predication primitives. VPNOT and VPST share one opcode (0xFE31_0F4D | mask): mask 0 is
                    // VPNOT, mask != 0 is VPST (a predicate block). VPSEL shares the Qd/Qn/Qm layout.
                    if word & MVE_VPST_NOT_MASK == MVE_VPST_NOT_BASE {
                        let mask = mve_predicate_mask_from_word(word);
                        return Ok(Some(if mask == 0 { Self::MveVpnot } else { Self::MveVpst(mask) }));
                    }
                    if word & MVE_VPSEL_MASK == MVE_VPSEL_BASE {
                        return Ok(Some(Self::MveVpsel(qd, qn, qm)));
                    }
                    // VADC/VSBC (occupies VCADD's reserved size=0b11 slot -- claimed here first by the tight mask):
                    // subtract = bit28, init-carry = bit12
                    if word & MVE_VADC_MASK == MVE_VADC_BASE {
                        return Ok(Some(Self::MveVadc((word >> 28) & 1 == 1, (word >> 12) & 1 == 1, qd, qn, qm)));
                    }
                    // VSHLC: imm5[20:16] = shift (0 means 32), Qda[15:13], Rdm[3:0]
                    if word & MVE_VSHLC_MASK == MVE_VSHLC_BASE {
                        let imm5 = (word >> 16) & 0x1F;
                        let shift = if imm5 == 0 { 32 } else { imm5 as u8 };
                        let rdm = Arm32GeneralPurposeRegister::from_operand_bits((word & 0xF) as u8);
                        return Ok(Some(Self::MveVshlc(shift, qd, rdm)));
                    }
                    // VCADD (integer) / VHCADD: halving = bit28==0, size[21:20], rotation 90/270 = bit12
                    if word & MVE_VCADD_INT_MASK == MVE_VCADD_INT_PATTERN
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            return Ok(Some(Self::MveVcaddInt((word >> 28) & 1 == 0, size, (word >> 12) & 1 == 1, qd, qn, qm)));
                        }
                    // VCADD (float): size = bit20, rotation 90/270 = bit24
                    if word & MVE_VCADD_FLOAT_MASK == MVE_VCADD_FLOAT_PATTERN {
                        let size = if (word >> 20) & 1 == 1 { Arm32MveFloatSize::F32 } else { Arm32MveFloatSize::F16 };
                        return Ok(Some(Self::MveVcaddFloat(size, (word >> 24) & 1 == 1, qd, qn, qm)));
                    }
                    // VCMUL: size = bit28, rotation 0/90/180/270 = {bit12, bit0}
                    if word & MVE_VCMUL_MASK == MVE_VCMUL_PATTERN {
                        let size = if (word >> 28) & 1 == 1 { Arm32MveFloatSize::F32 } else { Arm32MveFloatSize::F16 };
                        let rotate = ((word & 1) | (((word >> 12) & 1) << 1)) as u8;
                        return Ok(Some(Self::MveVcmul(size, rotate, qd, qn, qm)));
                    }
                    // VCMLA: size = bit20, rotation 0/90/180/270 = {bit24, bit23}
                    if word & MVE_VCMLA_MASK == MVE_VCMLA_PATTERN {
                        let size = if (word >> 20) & 1 == 1 { Arm32MveFloatSize::F32 } else { Arm32MveFloatSize::F16 };
                        let rotate = (((word >> 23) & 1) | (((word >> 24) & 1) << 1)) as u8;
                        return Ok(Some(Self::MveVcmla(size, rotate, qd, qn, qm)));
                    }
                    // VCMP / VPT integer (size[21:20] valid) -- register or scalar (bit6); mask 0 = VCMP, else VPT
                    if word & MVE_VCMP_INT_MASK == MVE_VCMP_INT_BASE
                        && let Some(size) = Arm32MveSize::from_size_bits((word >> 20) & 0b11) {
                            let scalar = (word >> 6) & 1 == 1;
                            let cond = Arm32MveVcmpCondition::from_fc(mve_vcmp_fc_from_word(word, scalar));
                            let mask = mve_predicate_mask_from_word(word);
                            let rm = Arm32GeneralPurposeRegister::from_operand_bits((word & 0xF) as u8);
                            return Ok(Some(match (mask == 0, scalar) {
                                (true, false) => Self::MveVcmpReg(cond, size, qn, qm),
                                (true, true) => Self::MveVcmpScalar(cond, size, qn, rm),
                                (false, false) => Self::MveVptReg(cond, size, qn, qm, mask),
                                (false, true) => Self::MveVptScalar(cond, size, qn, rm, mask),
                            }));
                        }
                    // VCMP / VPT float (its [21:20] = 11 distinguishes it from the integer form; size in bit28).
                    // Float only uses eq/ne/ge/lt/gt/le; the cs/hi fc-slots (001/011) are reserved here
                    // (VPSEL, decoded earlier, occupies one of them), so don't claim them as a float VCMP/VPT.
                    if word & MVE_VCMP_FLOAT_MASK == MVE_VCMP_FLOAT_BASE {
                        let scalar = (word >> 6) & 1 == 1;
                        let fc = mve_vcmp_fc_from_word(word, scalar);
                        if fc != 0b001 && fc != 0b011 {
                            let size = if (word >> 28) & 1 == 1 { Arm32MveFloatSize::F16 } else { Arm32MveFloatSize::F32 };
                            let cond = Arm32MveVcmpCondition::from_fc(fc);
                            let mask = mve_predicate_mask_from_word(word);
                            let rm = Arm32GeneralPurposeRegister::from_operand_bits((word & 0xF) as u8);
                            return Ok(Some(match (mask == 0, scalar) {
                                (true, false) => Self::MveVcmpFloatReg(cond, size, qn, qm),
                                (true, true) => Self::MveVcmpFloatScalar(cond, size, qn, rm),
                                (false, false) => Self::MveVptFloatReg(cond, size, qn, qm, mask),
                                (false, true) => Self::MveVptFloatScalar(cond, size, qn, rm, mask),
                            }));
                        }
                    }
                }
            }

            // ---- Branch Future (Armv8.1-M Low Overhead Branch): BF/BFL/BFX/BFLX/BFCSEL. Signature is
            // hw1[15:11]=11110, hw2[0]=1, hw2[15:12]=1110 (BF/BFX/BFLX/BFCSEL) or 1100 (BFL), with a NONZERO
            // boff=hw1[10:7]. Decoded BEFORE the low-overhead loops: WLS/DLS/LE all have boff=0 (it is a fixed
            // zero field in their encoding), so requiring boff!=0 here cleanly separates the two families that
            // share this space. (BL/B differ by hw1 bit12=1; these all have it 0.) Variant key:
            // hw2[15:12]=1100 -> BFL; else hw1[6:4]=110/111 -> BFX/BFLX, hw1[6]=0 -> BFCSEL, hw1[6:5]=10 -> BF.
            // Target offset = SignExtend(immA:immB:immC:'0'): immA=hw1[4:0], immB=hw2[10:1], immC=hw2[11]
            // (for BFCSEL immA is only the single bit hw1[0]). ----
            if word & 0xF800_0001 == 0xF000_0001 {
                let boff = ((word >> 23) & 0xF) as u8;        // hw1[10:7]
                let hw2_top = (word >> 12) & 0xF;             // hw2[15:12]
                if boff != 0 && (hw2_top == 0b1100 || hw2_top == 0b1110) {
                    let g = Arm32GeneralPurposeRegister::from_operand_bits;
                    let imm_b = (word >> 1) & 0x3FF;          // hw2[10:1]
                    let imm_c = (word >> 11) & 0x1;           // hw2[11]
                    let off17 = |imm_a: u32| sign_extension_utils::sign_extend_int_to_i32(
                        ((imm_a << 12) | (imm_b << 2) | (imm_c << 1)) as i32, 17);
                    if hw2_top == 0b1100 {                    // BFL (T4)
                        return Ok(Some(Self::Bfl_T4(boff, off17((word >> 16) & 0x1F))));
                    } else if (word >> 20) & 0b111 == 0b110 { // BFX (T3): hw1[6:4]=110
                        return Ok(Some(Self::Bfx_T3(boff, g(((word >> 16) & 0xF) as u8))));
                    } else if (word >> 20) & 0b111 == 0b111 { // BFLX (T5): hw1[6:4]=111
                        return Ok(Some(Self::Bflx_T5(boff, g(((word >> 16) & 0xF) as u8))));
                    } else if (word >> 22) & 1 == 0 {         // BFCSEL (T2): hw1[6]=0
                        let cond = ((word >> 18) & 0xF) as u8;    // hw1[5:2]
                        let t = (word >> 17) & 1 == 1;            // hw1[1]
                        let off13 = sign_extension_utils::sign_extend_int_to_i32(
                            ((((word >> 16) & 0x1) << 12) | (imm_b << 2) | (imm_c << 1)) as i32, 13);
                        return Ok(Some(Self::Bfcsel_T2(boff, off13, cond, t)));
                    } else {                                   // BF (T1): hw1[6:5]=10
                        return Ok(Some(Self::Bf_T1(boff, off17((word >> 16) & 0x1F))));
                    }
                }
            }

            // ---- low-overhead loops (DLS/WLS/LE/DLSTP/WLSTP/LETP/LCTP/VCTP), all 0xF0xx ----
            // Their hw1 bit12 = 0 (0xE001/0xE801/0xC0xx), which is disjoint from B.W/BL (hw1 bit12 = 1) below.
            if word >> 24 == 0xF0 {
                if word == MVE_LCTP_WORD {
                    return Ok(Some(Self::Lctp));
                }
                if word & MVE_VCTP_MASK == MVE_VCTP_BASE {
                    let size = match (word >> 20) & 0b11 { 0 => 8u8, 1 => 16, 2 => 32, _ => 64 };
                    let rn = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8);
                    return Ok(Some(Self::MveVctp(size, rn)));
                }
                if word & MVE_LOB_DLS_MASK == MVE_LOB_DLS_BASE
                    && let Some(tp_size) = lob_size_from_field((word >> 20) & 0b111) {
                        let rn = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8);
                        return Ok(Some(Self::LobStart(false, tp_size, rn, 0)));
                    }
                if word & 0xF000 == 0xC000 { // a low-overhead-loop branch (hw1[15:12] = 1100)
                    let rn_bits = (word >> 16) & 0xF;
                    let size_field = (word >> 20) & 0b111;
                    if rn_bits == 0xF { // PC marks the LE/LETP loop-end
                        let offset = lob_branch_offset(word & 0xFFFF, true);
                        match size_field {
                            0b000 => return Ok(Some(Self::LobEnd(false, offset))),
                            0b001 => return Ok(Some(Self::LobEnd(true, offset))),
                            _ => {} // 0b010 = the no-LR `le` form, not modelled
                        }
                    } else if let Some(tp_size) = lob_size_from_field(size_field) {
                        let rn = Arm32GeneralPurposeRegister::from_operand_bits(rn_bits as u8);
                        let offset = lob_branch_offset(word & 0xFFFF, false);
                        return Ok(Some(Self::LobStart(true, tp_size, rn, offset)));
                    }
                }
            }

            // mask: 0b1111_1000_0000_0000_1101_0000_0000_0000
            match word & 0b1111_1000_0000_0000_1101_0000_0000_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Bl_T1 as u32 => {
                    let (imm11, j2, j1, imm10, s) = ArmT32InstructionDecoder::decode_instruction_word__000a__0b0b__0d0d__1019__1a1a(word);
                    //
                    let i1 = (!(j1 ^ s)) & 0b1;
                    let i2 = (!(j2 ^ s)) & 0b1;
                    let encoded_imm24_as_u32: u32 = 
                        ((s as u32) << 23) |
                        ((i1 as u32) << 22) |
                        ((i2 as u32) << 21) |
                        ((imm10 as u32) << 11) |
                        (imm11 as u32);
                    let encoded_signed_imm24 = sign_extension_utils::sign_extend_int_to_i32(encoded_imm24_as_u32 as i32, 24);
                    //
                    let decoded_signed_imm25 = encoded_signed_imm24 * 2;
                    //
                    return Ok(Some(Self::Bl_T1(decoded_signed_imm25)));
                },
                // M7m B.W (T4): same group as BL, distinguished by the second halfword's bit 12 (0x0000_9000).
                0xF000_9000 => return Ok(Some(Self::B_T4(decode_word_branch_wide_unconditional(word)))),
                // M7m B<c>.W (T3): conditions 0..=13 only; cond 14/15 are MSR/MRS/misc-control and fall through.
                0xF000_8000 => {
                    let (cond, offset) = decode_word_branch_wide_conditional(word);
                    if cond <= 13 {
                        return Ok(Some(Self::B_T3(ArmT32InstructionCondition::from_operand_bits(cond), offset)));
                    }
                },
                _ => (), // continue to next masked set
            }

            // mask: 0b1111_1111_1111_1111_1111_1111_1111_0000
            match word & 0b1111_1111_1111_1111_1111_1111_1111_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Dmb_T1 as u32 => {
                    // NOTE: the mask excluding the "(#)" bits is: 0b1111_1111_1111_0000_1101_0000_1111_0000
                    let option = ArmT32InstructionDecoder::decode_instruction_word__0003(word);
                    return Ok(Some(Self::Dmb_T1(ArmT32MemoryBarrierOption::from_operand_bits(option))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Dsb_T1 as u32 => {
                    // NOTE: the mask excluding the "(#)" bits is: 0b1111_1111_1111_0000_1101_0000_1111_0000
                    let option = ArmT32InstructionDecoder::decode_instruction_word__0003(word);
                    return Ok(Some(Self::Dsb_T1(ArmT32MemoryBarrierOption::from_operand_bits(option))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Isb_T1 as u32 => {
                    // NOTE: the mask excluding the "(#)" bits is: 0b1111_1111_1111_0000_1101_0000_1111_0000
                    let option = ArmT32InstructionDecoder::decode_instruction_word__0003(word);
                    return Ok(Some(Self::Isb_T1(ArmT32MemoryBarrierOption::from_operand_bits(option))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Clrex_T1 as u32 => {
                    return Ok(Some(Self::Clrex_T1)); // ARMv7-M (the option field in bits 3:0 is SBO)
                },
                _ => (), // continue to next masked set
            }

            // mask: 0b1111_1111_1111_1111_1111_0000_0000_0000
            match word & 0b1111_1111_1111_1111_1111_0000_0000_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Mrs_T1 as u32 => {
                    // NOTE: the mask excluding the "(#)" bits is: 0b1111_1111_1110_0000_1101_0000_0000_0000
                    let (sysm, rd) = ArmT32InstructionDecoder::decode_instruction_word__0007__080b(word);
                    let spec_reg = ArmT32SpecialRegister::from_operand_bits(sysm);
                    return Ok(Some(Self::Mrs_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), spec_reg)));
                },
                _ => (), // continue to next masked set
            }

            // mask: 0b1111_1111_1111_0000_1111_1111_0000_0000
            match word & 0b1111_1111_1111_0000_1111_1111_0000_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Msr_Register_T1 as u32 => {
                    // NOTE: the mask excluding the "(#)" bits is: 0b1111_1111_1110_0000_1101_0000_0000_0000
                    let (sysm, rn) = ArmT32InstructionDecoder::decode_instruction_word__0007__1013(word);
                    let spec_reg = ArmT32SpecialRegister::from_operand_bits(sysm);
                    return Ok(Some(Self::Msr_Register_T1(spec_reg, Arm32GeneralPurposeRegister::from_operand_bits(rn))));
                },
                _ => (), // continue to next masked set
            }

            // mask: 0b1111_1111_1111_0000_1111_0000_0000_0000
            match word & 0b1111_1111_1111_0000_1111_0000_0000_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Udf_T2 as u32 => {
                    let (imm12, imm4) = ArmT32InstructionDecoder::decode_instruction_word__000b__1013(word);
                    let imm16: u16 = ((imm4 as u16) << 12) | imm12;
                    return Ok(Some(Self::Udf_T2(imm16)));
                },
                _ => (), // continue to next masked set
            }

            /* ---- ARMv7-M (Thumb-2) additions ---- */

            // MOVW / MOVT  (imm16 = imm4:i:imm3:imm8)  mask: 0b1111_1011_1111_0000_1000_0000_0000_0000
            match word & 0b1111_1011_1111_0000_1000_0000_0000_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Mov_Immediate_T3 as u32 => {
                    return Ok(Some(Self::Mov_Immediate_T3(Arm32GeneralPurposeRegister::from_operand_bits(decode_word_rd_11_08(word)), decode_word_imm16_movw(word))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Movt_T1 as u32 => {
                    return Ok(Some(Self::Movt_T1(Arm32GeneralPurposeRegister::from_operand_bits(decode_word_rd_11_08(word)), decode_word_imm16_movw(word))));
                },
                _ => (),
            }

            // Data processing (modified immediate): `11110 i 0 op4 S Rn 0 imm3 Rd imm8` (bit 25 = 0,
            // bit 15 = 0). The op4 field selects the operation, with Rn==PC / Rd==PC selecting the
            // MOV/MVN and TST/TEQ/CMN/CMP aliases; unallocated op4 values fall through to InvalidOpcode.
            if word & 0b1111_1010_0000_0000_1000_0000_0000_0000 == 0b1111_0000_0000_0000_0000_0000_0000_0000 {
                let op4 = (word >> 21) & 0b1111;
                let set_flags = ((word >> 20) & 0b1) == 1;
                let rn = ((word >> 16) & 0b1111) as u8;
                let rd = ((word >> 8) & 0b1111) as u8;
                let constant = decode_word_modified_immediate(word);
                let rn_reg = Arm32GeneralPurposeRegister::from_operand_bits(rn);
                let rd_reg = Arm32GeneralPurposeRegister::from_operand_bits(rd);
                match op4 {
                    0b0000 => { // AND (TST if Rd==PC, S)
                        if rd == 0b1111 && set_flags { return Ok(Some(Self::Tst_Immediate_T1(rn_reg, constant))); }
                        return Ok(Some(Self::And_Immediate_T1(rd_reg, rn_reg, constant, set_flags)));
                    },
                    0b0001 => return Ok(Some(Self::Bic_Immediate_T1(rd_reg, rn_reg, constant, set_flags))),
                    0b0010 => { // ORR (MOV if Rn==PC)
                        if rn == 0b1111 { return Ok(Some(Self::Mov_Immediate_T2(rd_reg, constant, set_flags))); }
                        return Ok(Some(Self::Orr_Immediate_T1(rd_reg, rn_reg, constant, set_flags)));
                    },
                    0b0011 => { // ORN (MVN if Rn==PC)
                        if rn == 0b1111 { return Ok(Some(Self::Mvn_Immediate_T1(rd_reg, constant, set_flags))); }
                        return Ok(Some(Self::Orn_Immediate_T1(rd_reg, rn_reg, constant, set_flags)));
                    },
                    0b0100 => { // EOR (TEQ if Rd==PC, S)
                        if rd == 0b1111 && set_flags { return Ok(Some(Self::Teq_Immediate_T1(rn_reg, constant))); }
                        return Ok(Some(Self::Eor_Immediate_T1(rd_reg, rn_reg, constant, set_flags)));
                    },
                    0b1000 => { // ADD (CMN if Rd==PC, S)
                        if rd == 0b1111 && set_flags { return Ok(Some(Self::Cmn_Immediate_T1(rn_reg, constant))); }
                        return Ok(Some(Self::Add_Immediate_T3(rd_reg, rn_reg, constant, set_flags)));
                    },
                    0b1010 => return Ok(Some(Self::Adc_Immediate_T1(rd_reg, rn_reg, constant, set_flags))),
                    0b1011 => return Ok(Some(Self::Sbc_Immediate_T1(rd_reg, rn_reg, constant, set_flags))),
                    0b1101 => { // SUB (CMP if Rd==PC, S)
                        if rd == 0b1111 && set_flags { return Ok(Some(Self::Cmp_Immediate_T2(rn_reg, constant))); }
                        return Ok(Some(Self::Sub_Immediate_T3(rd_reg, rn_reg, constant, set_flags)));
                    },
                    0b1110 => return Ok(Some(Self::Rsb_Immediate_T2(rd_reg, rn_reg, constant, set_flags))),
                    _ => (), // unallocated op4 -> InvalidOpcode
                }
            }

            // Data processing (shifted register): `11101 01 op4 S Rn (0) imm3 Rd imm2 type Rm` (bit 15 = 0).
            // op4 selects the operation; Rn==PC turns ORR/ORN into MOV/MVN (register), and Rd==PC with S
            // turns AND/EOR/ADD/SUB into TST/TEQ/CMN/CMP (register).
            if word & 0b1111_1110_0000_0000_1000_0000_0000_0000 == 0b1110_1010_0000_0000_0000_0000_0000_0000 {
                let op4 = (word >> 21) & 0b1111;
                let set_flags = ((word >> 20) & 0b1) == 1;
                let rn = ((word >> 16) & 0b1111) as u8;
                let rd = ((word >> 8) & 0b1111) as u8;
                let rm = (word & 0b1111) as u8;
                let shift = decode_register_shift(word);
                let rn_reg = Arm32GeneralPurposeRegister::from_operand_bits(rn);
                let rd_reg = Arm32GeneralPurposeRegister::from_operand_bits(rd);
                let rm_reg = Arm32GeneralPurposeRegister::from_operand_bits(rm);
                let rn_is_pc = rn == 0b1111;
                let rd_pc_set = rd == 0b1111 && set_flags;
                match op4 {
                    0b0000 => return Ok(Some(if rd_pc_set { Self::Tst_Register_T2(rn_reg, rm_reg, shift) } else { Self::And_Register_T2(rd_reg, rn_reg, rm_reg, shift, set_flags) })),
                    0b0001 => return Ok(Some(Self::Bic_Register_T2(rd_reg, rn_reg, rm_reg, shift, set_flags))),
                    0b0010 => return Ok(Some(if rn_is_pc { Self::Mov_Register_T3(rd_reg, rm_reg, shift, set_flags) } else { Self::Orr_Register_T2(rd_reg, rn_reg, rm_reg, shift, set_flags) })),
                    0b0011 => return Ok(Some(if rn_is_pc { Self::Mvn_Register_T2(rd_reg, rm_reg, shift, set_flags) } else { Self::Orn_Register_T1(rd_reg, rn_reg, rm_reg, shift, set_flags) })),
                    0b0100 => return Ok(Some(if rd_pc_set { Self::Teq_Register_T1(rn_reg, rm_reg, shift) } else { Self::Eor_Register_T2(rd_reg, rn_reg, rm_reg, shift, set_flags) })),
                    0b1000 => return Ok(Some(if rd_pc_set { Self::Cmn_Register_T2(rn_reg, rm_reg, shift) } else { Self::Add_Register_T3(rd_reg, rn_reg, rm_reg, shift, set_flags) })),
                    0b1010 => return Ok(Some(Self::Adc_Register_T2(rd_reg, rn_reg, rm_reg, shift, set_flags))),
                    0b1011 => return Ok(Some(Self::Sbc_Register_T2(rd_reg, rn_reg, rm_reg, shift, set_flags))),
                    0b1101 => return Ok(Some(if rd_pc_set { Self::Cmp_Register_T3(rn_reg, rm_reg, shift) } else { Self::Sub_Register_T2(rd_reg, rn_reg, rm_reg, shift, set_flags) })),
                    0b1110 => return Ok(Some(Self::Rsb_Register_T1(rd_reg, rn_reg, rm_reg, shift, set_flags))),
                    _ => (), // unallocated op4 -> InvalidOpcode (incl. PKH op4=0110, decoded below)
                }
            }

            // M8c DSP PKHBT / PKHTB (op4=0110 in the shifted-register space).  mask keeps op + bit15=0, bit4=0.
            if word & 0b1111_1111_1110_0000_1000_0000_0001_0000 == 0xEAC0_0000 {
                let (rd, rn, rm, amount, tb) = decode_word_pack_halfword(word);
                return Ok(Some(if tb { Self::Pkhtb_T1(g(rd), g(rn), g(rm), amount) } else { Self::Pkhbt_T1(g(rd), g(rn), g(rm), amount) }));
            }

            // MUL / SDIV / UDIV / CLZ  mask: 0b1111_1111_1111_0000_1111_0000_1111_0000
            match word & 0b1111_1111_1111_0000_1111_0000_1111_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Mul_T2 as u32 => {
                    let (rn, rd, rm) = decode_word_rn_rd_rm(word);
                    return Ok(Some(Self::Mul_T2(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), Arm32GeneralPurposeRegister::from_operand_bits(rm))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Sdiv_T1 as u32 => {
                    let (rn, rd, rm) = decode_word_rn_rd_rm(word);
                    return Ok(Some(Self::Sdiv_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), Arm32GeneralPurposeRegister::from_operand_bits(rm))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Udiv_T1 as u32 => {
                    let (rn, rd, rm) = decode_word_rn_rd_rm(word);
                    return Ok(Some(Self::Udiv_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), Arm32GeneralPurposeRegister::from_operand_bits(rm))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Clz_T1 as u32 => {
                    // CLZ encodes Rm in both bits 19:16 and 3:0; read it from bits 3:0
                    let rd = decode_word_rd_11_08(word);
                    let rm = (word & 0b1111) as u8;
                    return Ok(Some(Self::Clz_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rm))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Rbit_T1 as u32 => {
                    // RBIT (like CLZ) encodes Rm in both bits 19:16 and 3:0
                    let rd = decode_word_rd_11_08(word);
                    let rm = (word & 0b1111) as u8;
                    return Ok(Some(Self::Rbit_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rm))));
                },
                // M7l: REV.W / REV16.W / REVSH.W also live in this family (Rm in both 19:16 and 3:0, op in 7:4)
                0xFA90_F080 => { let rd = decode_word_rd_11_08(word); return Ok(Some(Self::Rev_T2(g(rd), g((word & 0b1111) as u8)))); },
                0xFA90_F090 => { let rd = decode_word_rd_11_08(word); return Ok(Some(Self::Rev16_T2(g(rd), g((word & 0b1111) as u8)))); },
                0xFA90_F0B0 => { let rd = decode_word_rd_11_08(word); return Ok(Some(Self::Revsh_T2(g(rd), g((word & 0b1111) as u8)))); },
                // M8a DSP saturating arithmetic (Rn 19:16, Rd 11:8, Rm 3:0)
                0xFA80_F080 => { let (rn, rd, rm) = decode_word_rn_rd_rm(word); return Ok(Some(Self::Qadd_T1(g(rd), g(rm), g(rn)))); },
                0xFA80_F0A0 => { let (rn, rd, rm) = decode_word_rn_rd_rm(word); return Ok(Some(Self::Qsub_T1(g(rd), g(rm), g(rn)))); },
                0xFA80_F090 => { let (rn, rd, rm) = decode_word_rn_rd_rm(word); return Ok(Some(Self::Qdadd_T1(g(rd), g(rm), g(rn)))); },
                0xFA80_F0B0 => { let (rn, rd, rm) = decode_word_rn_rd_rm(word); return Ok(Some(Self::Qdsub_T1(g(rd), g(rm), g(rn)))); },
                // M8c DSP SEL (Rd, Rn, Rm)
                0xFAA0_F080 => { let (rn, rd, rm) = decode_word_rn_rd_rm(word); return Ok(Some(Self::Sel_T1(g(rd), g(rn), g(rm)))); },
                _ => (),
            }

            // M8d DSP parallel add/subtract (packed SIMD): operation in 22:20, prefix in 6:4, bit7=0.
            // mask keeps the family bits (incl. bit7=0) but lets operation, prefix, and Rn/Rd/Rm vary.
            if word & 0b1111_1111_1000_0000_1111_0000_1000_0000 == 0xFA80_F000 {
                let operation = ArmT32ParallelOperation::from_op_bits((word >> 20) & 0b111);
                let prefix = ArmT32ParallelPrefix::from_prefix_bits((word >> 4) & 0b111);
                if let (Some(operation), Some(prefix)) = (operation, prefix) {
                    let rn = ((word >> 16) & 0b1111) as u8;
                    let rd = ((word >> 8) & 0b1111) as u8;
                    let rm = (word & 0b1111) as u8;
                    return Ok(Some(Self::ParallelAddSub_T1(operation, prefix, g(rd), g(rn), g(rm))));
                }
            }

            // UBFX / SBFX / BFI / BFC  mask: 0b1111_1111_1111_0000_1000_0000_0010_0000 (lsb=imm3:imm2, widthm1/msb in bits 4:0)
            match word & 0b1111_1111_1111_0000_1000_0000_0010_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Ubfx_T1 as u32 => {
                    let (rn, rd, lsb, widthm1) = decode_word_bitfield(word);
                    return Ok(Some(Self::Ubfx_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), lsb, widthm1 + 1)));
                },
                op if op == ArmT32OpcodePattern_32Bit::Sbfx_T1 as u32 => {
                    let (rn, rd, lsb, widthm1) = decode_word_bitfield(word);
                    return Ok(Some(Self::Sbfx_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), lsb, widthm1 + 1)));
                },
                op if op == ArmT32OpcodePattern_32Bit::Bfi_T1 as u32 => {
                    let (rn, rd, lsb, msb) = decode_word_bitfield(word);
                    let width = msb.wrapping_sub(lsb).wrapping_add(1);
                    // Rn == 0b1111 is BFC (bitfield clear); otherwise BFI
                    if rn == 0b1111 {
                        return Ok(Some(Self::Bfc_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), lsb, width)));
                    } else {
                        return Ok(Some(Self::Bfi_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), lsb, width)));
                    }
                },
                _ => (),
            }

            // M7i LDRD / STRD (`Rt, Rt2, [Rn, #+/-imm]{!}` etc.). Mask keeps the opcode + bit22 + L (bit20),
            // letting P/U/W and the operands vary. STREX/LDREX share the (P=0,W=0) "hole" in this space, so
            // the helper returns None for them and we fall through to the LDREX/STREX decode below.
            match word & 0b1111_1110_0101_0000_0000_0000_0000_0000 {
                0xE850_0000 => { if let Some((rt, rt2, rn, off, mode)) = decode_word_load_store_dual(word) { return Ok(Some(Self::Ldrd_Immediate_T1(g(rt), g(rt2), g(rn), off, mode))); } },
                0xE840_0000 => { if let Some((rt, rt2, rn, off, mode)) = decode_word_load_store_dual(word) { return Ok(Some(Self::Strd_Immediate_T1(g(rt), g(rt2), g(rn), off, mode))); } },
                _ => (),
            }

            // M7j wide load/store multiple (LDM.W/STM.W/LDMDB/STMDB).  mask keeps the IA/DB + L bits but lets
            // the W (writeback) bit, Rn, and the 16-bit register list vary.
            match word & 0b1111_1111_1101_0000_0000_0000_0000_0000 {
                0xE890_0000 => { let (rn, wb, regs) = decode_word_load_store_multiple(word); return Ok(Some(Self::Ldmia_T2(g(rn), wb, regs))); },
                0xE880_0000 => { let (rn, wb, regs) = decode_word_load_store_multiple(word); return Ok(Some(Self::Stmia_T2(g(rn), wb, regs))); },
                0xE910_0000 => { let (rn, wb, regs) = decode_word_load_store_multiple(word); return Ok(Some(Self::Ldmdb_T1(g(rn), wb, regs))); },
                0xE900_0000 => { let (rn, wb, regs) = decode_word_load_store_multiple(word); return Ok(Some(Self::Stmdb_T1(g(rn), wb, regs))); },
                _ => (),
            }

            // MVE VMOV (two 32-bit lanes) -- a pair of GPRs <-> two lanes of Qd. [11:8]=1111 + [12]=0 keeps it
            // disjoint from both the contiguous load/store (bit12=1) and the gather/scatter below. Claimed first.
            if word & 0xFFE0_1FE0 == 0xEC00_0F00 {
                let to_vector = (word >> 20) & 1 == 1;
                let rt2 = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8);
                let rt = Arm32GeneralPurposeRegister::from_operand_bits((word & 0xF) as u8);
                let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                let idx1 = 2 + ((word >> 4) & 1) as u8;
                return Ok(Some(Self::MveVmovTwoLane(to_vector, idx1, qd, rt, rt2)));
            }

            // MVE contiguous vector load/store (VLDRB/H/W, VSTRB/H/W). Same 0xEC../0xED.. space as VFP
            // load/store but disjoint: the MVE forms have bits[11:9]=111 (vs VFP's 101) and the same-size
            // marker bit12=1. Decoded BEFORE the VFP block so it claims its own words first.
            if word & 0xFE00_0000 == 0xEC00_0000 && (word >> 12) & 1 == 1 && (word >> 9) & 0b111 == 0b111
                && let Some(size) = Arm32MveSize::from_size_bits((word >> 7) & 0b11) {
                    let is_load = (word >> 20) & 1 == 1;
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let rn = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8);
                    let size_bytes = (mve_shift_esize(size) / 8) as i32;
                    let magnitude = ((word & 0x7F) as i32) * size_bytes;
                    let offset = if (word >> 23) & 1 == 1 { magnitude } else { -magnitude };
                    let mode = match (((word >> 24) & 1), ((word >> 21) & 1)) {
                        (1, 0) => ArmT32IndexMode::Offset,
                        (1, 1) => ArmT32IndexMode::PreIndex,
                        _ => ArmT32IndexMode::PostIndex, // (0,1)
                    };
                    return Ok(Some(Self::MveLoadStore(is_load, size, qd, rn, offset, mode)));
                }

            // MVE gather/scatter (scalar base + vector offset). bit12=0 keeps it disjoint from the contiguous
            // load/store above (bit12=1); decoded right after it.
            if matches!(word >> 24, 0xEC | 0xFC) && word & MVE_GATHER_SCATTER_MASK == MVE_GATHER_SCATTER_BASE {
                let is_load = (word >> 20) & 1 == 1;
                let unsigned = (word >> 28) & 1 == 1;
                let esize = mve_mem_size_from_log((word >> 7) & 0b11);
                let msize = mve_mem_size_from_log((((word >> 6) & 1) << 1) | ((word >> 4) & 1));
                let scaled = word & 1 == 1;
                let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                let rn = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8);
                let qm = Arm32MveVectorRegister::from_field((word >> 1) & 0b111);
                return Ok(Some(Self::MveGatherScatter(is_load, unsigned, esize, msize, scaled, qd, rn, qm)));
            }

            // MVE gather/scatter with a vector base + immediate (top byte 0xFD).
            if word >> 24 == 0xFD && word & MVE_GATHER_VBASE_MASK == MVE_GATHER_VBASE_BASE {
                let is_load = (word >> 20) & 1 == 1;
                let is_dword = (word >> 8) & 1 == 1;
                let writeback = (word >> 21) & 1 == 1;
                let qn = Arm32MveVectorRegister::from_field((word >> 17) & 0b111);
                let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                let scale = if is_dword { 8 } else { 4 };
                let magnitude = ((word & 0x7F) * scale) as i32;
                let offset = if (word >> 23) & 1 == 1 { magnitude } else { -magnitude };
                return Ok(Some(Self::MveGatherScatterBase(is_load, is_dword, writeback, qd, qn, offset)));
            }

            // MVE de-interleaving/interleaving load/store (top byte 0xFC, bit12=1).
            if word >> 24 == 0xFC && word & MVE_INTERLEAVE_MASK == MVE_INTERLEAVE_BASE
                && let Some(size) = Arm32MveSize::from_size_bits((word >> 7) & 0b11) {
                    let is_load = (word >> 20) & 1 == 1;
                    let is_quad = word & 1 == 1;
                    let writeback = (word >> 21) & 1 == 1;
                    let pass = ((word >> 5) & if is_quad { 0b11 } else { 0b1 }) as u8;
                    let qd = Arm32MveVectorRegister::from_field((word >> 13) & 0b111);
                    let rn = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8);
                    return Ok(Some(Self::MveInterleave(is_load, is_quad, pass, size, qd, rn, writeback)));
                }

            // M8f FP load/store VLDR/VSTR (the P=1,W=0 offset form: bit21=0).  mask: op + L(bit20) + size[11:8].
            match word & 0b1111_1111_0011_0000_0000_1111_0000_0000 {
                0xED10_0A00 => { let (vf, d, rn, off) = decode_word_fp_load_store(word); return Ok(Some(Self::Vldr_Single_T2(Arm32SinglePrecisionRegister::from_field_and_bit(vf, d), g(rn), off))); },
                0xED00_0A00 => { let (vf, d, rn, off) = decode_word_fp_load_store(word); return Ok(Some(Self::Vstr_Single_T2(Arm32SinglePrecisionRegister::from_field_and_bit(vf, d), g(rn), off))); },
                0xED10_0B00 => { let (vf, d, rn, off) = decode_word_fp_load_store(word); return Ok(Some(Self::Vldr_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vf, d), g(rn), off))); },
                0xED00_0B00 => { let (vf, d, rn, off) = decode_word_fp_load_store(word); return Ok(Some(Self::Vstr_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vf, d), g(rn), off))); },
                _ => (),
            }

            // FLDMDBX/FSTMDBX (deprecated FP load/store multiple, decrement-before + writeback + the X word):
            // P=1,U=0,W=1, [11:8]=1011, and imm8 is ODD (= 2*count + 1). The odd imm8 (bit0=1) distinguishes
            // these from VLDMDB/VSTMDB (even imm8), which the generic block below would otherwise claim.
            if word & 0xFFA0_0F01 == 0xED20_0B01 {
                let first = Arm32DoublePrecisionRegister::from_field_and_bit((word >> 12) & 0xF, (word >> 22) & 1);
                let count = (((word & 0xFF) - 1) / 2) as u8;
                return Ok(Some(Self::FldmdbxFstmdbx_T1((word >> 20) & 1 == 1, g(((word >> 16) & 0xF) as u8), first, count)));
            }

            // M8g FP load/store multiple -- reached only after VLDR (P=1,W=0) above, so the remaining FP
            // load/store words are the multiple forms. Guard PU != 00 (PU==00 is the VMOV core-pair, decoded below).
            match word & 0b1111_1110_0001_0000_0000_1111_0000_0000 {
                0xEC10_0A00 if ((word >> 23) & 1) | ((word >> 24) & 1) == 1 => { let (rn, wb, db, vf, d, imm8) = decode_word_fp_load_store_multiple(word); return Ok(Some(Self::Vldm_Single_T2(g(rn), wb, db, Arm32SinglePrecisionRegister::from_field_and_bit(vf, d), imm8))); },
                0xEC00_0A00 if ((word >> 23) & 1) | ((word >> 24) & 1) == 1 => { let (rn, wb, db, vf, d, imm8) = decode_word_fp_load_store_multiple(word); return Ok(Some(Self::Vstm_Single_T2(g(rn), wb, db, Arm32SinglePrecisionRegister::from_field_and_bit(vf, d), imm8))); },
                0xEC10_0B00 if ((word >> 23) & 1) | ((word >> 24) & 1) == 1 => { let (rn, wb, db, vf, d, imm8) = decode_word_fp_load_store_multiple(word); return Ok(Some(Self::Vldm_Double_T1(g(rn), wb, db, Arm32DoublePrecisionRegister::from_field_and_bit(vf, d), imm8 / 2))); },
                0xEC00_0B00 if ((word >> 23) & 1) | ((word >> 24) & 1) == 1 => { let (rn, wb, db, vf, d, imm8) = decode_word_fp_load_store_multiple(word); return Ok(Some(Self::Vstm_Double_T1(g(rn), wb, db, Arm32DoublePrecisionRegister::from_field_and_bit(vf, d), imm8 / 2))); },
                _ => (),
            }

            // M8i VMOV between a core register pair and a double / two singles (the (P=0,U=0) hole the VLDM
            // block above skips).  op (bit20): 0 = core -> FP, 1 = FP -> core.
            match word & 0b1111_1111_1110_0000_0000_1111_1101_0000 {
                0xEC40_0A10 => {
                    let (rt2, rt) = (((word >> 16) & 0b1111) as u8, ((word >> 12) & 0b1111) as u8);
                    let sm = Arm32SinglePrecisionRegister::from_field_and_bit(word & 0b1111, (word >> 5) & 1);
                    // op (bit20): 1 = FP -> core (singles into the pair), 0 = core -> FP
                    return Ok(Some(if (word >> 20) & 1 == 1 { Self::Vmov_Singles_To_CorePair_T1(g(rt), g(rt2), sm) } else { Self::Vmov_CorePair_To_Singles_T1(sm, g(rt), g(rt2)) }));
                },
                0xEC40_0B10 => {
                    let (rt2, rt) = (((word >> 16) & 0b1111) as u8, ((word >> 12) & 0b1111) as u8);
                    let dm = Arm32DoublePrecisionRegister::from_field_and_bit(word & 0b1111, (word >> 5) & 1);
                    return Ok(Some(if (word >> 20) & 1 == 1 { Self::Vmov_Double_To_CorePair_T1(g(rt), g(rt2), dm) } else { Self::Vmov_CorePair_To_Double_T1(dm, g(rt), g(rt2)) }));
                },
                _ => (),
            }

            // M8h FP data-processing: [31:24]=EE, [11:9]=101, [4]=0. The 2-operand "other" group is
            // (bit23=1, [21:20]=11); everything else is the 3-operand group. VCMP/VCVT (M8i) sit in the
            // 2-operand group with other opc2 values and fall through here (from_bits -> None).
            if word & 0b1111_1111_0000_0000_0000_1110_0001_0000 == 0xEE00_0A00 {
                let double = (word >> 8) & 1 == 1;
                let top = (word >> 23) & 1;
                let middle = (word >> 20) & 0b11;
                let vd_field = (word >> 12) & 0b1111;
                let d_bit = (word >> 22) & 1;
                let vm_field = word & 0b1111;
                let m_bit = (word >> 5) & 1;
                if top == 1 && middle == 0b11 {
                    // bit6 == 0 in this group is VMOV (immediate): imm8 = imm4H:imm4L.
                    if (word >> 6) & 1 == 0 {
                        let imm8 = ((((word >> 16) & 0xF) << 4) | (word & 0xF)) as u8;
                        return Ok(Some(if double {
                            Self::Vmov_Immediate_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), imm8)
                        } else {
                            Self::Vmov_Immediate_Single_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), imm8)
                        }));
                    }
                    let opc2 = (word >> 16) & 0b1111;
                    let op7 = (word >> 7) & 1;
                    // half-precision conversions (opc2 0010 = f16->f32, 0011 = f32->f16; op7 = top half)
                    if opc2 == 0b0010 {
                        return Ok(Some(Self::Vcvt_HalfToSingle_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)));
                    }
                    if opc2 == 0b0011 {
                        return Ok(Some(Self::Vcvt_SingleToHalf_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)));
                    }
                    // fixed-point conversions: opc2 = 1:op:1:U (i.e. 1010/1011/1110/1111).
                    if opc2 & 0b1010 == 0b1010 && let Some((signed, bits32, frac)) = decode_word_vcvt_fixed(word) {
                        let to_fixed = (opc2 >> 2) & 1 == 1;
                        return Ok(Some(match (to_fixed, double) {
                            (true, false) => Self::Vcvt_FloatToFixed_Single_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), signed, bits32, frac),
                            (true, true) => Self::Vcvt_FloatToFixed_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), signed, bits32, frac),
                            (false, false) => Self::Vcvt_FixedToFloat_Single_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), signed, bits32, frac),
                            (false, true) => Self::Vcvt_FixedToFloat_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), signed, bits32, frac),
                        }));
                    }
                    if let Some(op) = ArmT32FpDataOperation2::from_bits(opc2, op7) {
                        return Ok(Some(if double {
                            Self::FpDataProcess2_Double(op, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                        } else {
                            Self::FpDataProcess2_Single(op, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                        }));
                    }
                    // M8i VCMP (opc2=0100, register Vm) / VCMP #0 (opc2=0101); op7 = the signalling E bit.
                    if opc2 == 0b0100 {
                        return Ok(Some(if double {
                            Self::Vcmp_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                        } else {
                            Self::Vcmp_Single_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                        }));
                    }
                    if opc2 == 0b0101 {
                        return Ok(Some(if double {
                            Self::Vcmp_Zero_Double_T2(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), op7 == 1)
                        } else {
                            Self::Vcmp_Zero_Single_T2(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), op7 == 1)
                        }));
                    }
                    // M8i VCVT: 0111 = f32<->f64, 1000 = int->float, 110x = float->int. `double` (bit8) is
                    // the SOURCE precision for float<->int and float-narrow, the DEST precision for int->float.
                    match opc2 {
                        0b0111 => return Ok(Some(if double {
                            Self::Vcvt_Double_To_Single_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                        } else {
                            Self::Vcvt_Single_To_Double_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                        })),
                        0b1000 => return Ok(Some(if double {
                            Self::Vcvt_IntToFloat_ToDouble_T1(Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                        } else {
                            Self::Vcvt_IntToFloat_ToSingle_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), op7 == 1)
                        })),
                        0b1100 | 0b1101 => return Ok(Some(if double {
                            Self::Vcvt_FloatToInt_FromDouble_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit), opc2 & 1 == 1, op7 == 1)
                        } else {
                            Self::Vcvt_FloatToInt_FromSingle_T1(Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit), opc2 & 1 == 1, op7 == 1)
                        })),
                        _ => {},
                    }
                } else if let Some(op) = ArmT32FpDataOperation3::from_bits(top, middle, (word >> 6) & 1) {
                    let vn_field = (word >> 16) & 0b1111;
                    let n_bit = (word >> 7) & 1;
                    return Ok(Some(if double {
                        Self::FpDataProcess3_Double(op, Arm32DoublePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32DoublePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    } else {
                        Self::FpDataProcess3_Single(op, Arm32SinglePrecisionRegister::from_field_and_bit(vd_field, d_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vn_field, n_bit), Arm32SinglePrecisionRegister::from_field_and_bit(vm_field, m_bit))
                    }));
                }
            }

            // M8i VMRS / VMSR (FPSCR transfer). VMRS with Rt==1111 is the APSR_nzcv (vcmp flags) form.
            match word & 0b1111_1111_1111_1111_0000_1111_1111_1111 {
                0xEEF1_0A10 => { let rt = ((word >> 12) & 0b1111) as u8; return Ok(Some(if rt == 0b1111 { Self::Vmrs_Apsr_Nzcv_T1 } else { Self::Vmrs_T1(g(rt)) })); },
                0xEEE1_0A10 => { return Ok(Some(Self::Vmsr_T1(g(((word >> 12) & 0b1111) as u8)))); },
                _ => (),
            }

            // M8i VMOV core <-> single (op bit 20: 0 = Rt->Sn, 1 = Sn->Rt).  mask excludes Vn / Rt / N.
            match word & 0b1111_1111_1111_0000_0000_1111_0111_1111 {
                0xEE00_0A10 => { let sn = Arm32SinglePrecisionRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1); return Ok(Some(Self::Vmov_Core_To_Single_T1(sn, g(((word >> 12) & 0b1111) as u8)))); },
                0xEE10_0A10 => { let sn = Arm32SinglePrecisionRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1); return Ok(Some(Self::Vmov_Single_To_Core_T1(g(((word >> 12) & 0b1111) as u8), sn))); },
                _ => (),
            }

            // VMOV core <-> scalar-lane ([11:8]=1011, vs VMOV-single's 1010). [20]=0 Rt->Dd[x]; [20]=1 Dn[x]->Rt (U[23]).
            if word & 0xFF90_0F1F == 0xEE00_0B10 {
                let (size, index) = Arm32VmovLaneSize::from_opc_fields((word >> 21) & 0b11, (word >> 5) & 0b11);
                let dd = Arm32DoublePrecisionRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1);
                return Ok(Some(Self::Vmov_Core_To_Scalar_T1(size, index, dd, g(((word >> 12) & 0b1111) as u8))));
            }
            if word & 0xFF10_0F1F == 0xEE10_0B10 {
                let (size, index) = Arm32VmovLaneSize::from_opc_fields((word >> 21) & 0b11, (word >> 5) & 0b11);
                let dn = Arm32DoublePrecisionRegister::from_field_and_bit((word >> 16) & 0b1111, (word >> 7) & 1);
                let unsigned = !matches!(size, Arm32VmovLaneSize::Word) && (word >> 23) & 1 == 1;
                return Ok(Some(Self::Vmov_Scalar_To_Core_T1(unsigned, size, index, g(((word >> 12) & 0b1111) as u8), dn)));
            }

            // M7i PC-relative literal loads (`Rt, [pc, #+/-imm12]`); the opcode has Rn==1111. Mask excludes the
            // U bit (bit23) and Rt/imm12, but requires the Rn==1111 nibble -- so it intercepts the Rn==PC case
            // before the imm12 / register / indexed blocks that would otherwise mis-read it as a base register.
            match word & 0b1111_1111_0111_1111_0000_0000_0000_0000 {
                0xF85F_0000 => { let (rt, off) = decode_word_load_literal(word); return Ok(Some(Self::Ldr_Literal_T2(g(rt), off))); },
                0xF81F_0000 => { let (rt, off) = decode_word_load_literal(word); return Ok(Some(Self::Ldrb_Literal_T1(g(rt), off))); },
                0xF83F_0000 => { let (rt, off) = decode_word_load_literal(word); return Ok(Some(Self::Ldrh_Literal_T1(g(rt), off))); },
                0xF91F_0000 => { let (rt, off) = decode_word_load_literal(word); return Ok(Some(Self::Ldrsb_Literal_T1(g(rt), off))); },
                0xF93F_0000 => { let (rt, off) = decode_word_load_literal(word); return Ok(Some(Self::Ldrsh_Literal_T1(g(rt), off))); },
                _ => (),
            }

            // M7i PLD / PLI, positive (imm12) form. The Rt field is 1111, which is how it is distinguished
            // from a byte/half load; intercept it before the imm12 load block.  mask: keep op + Rt==1111.
            match word & 0b1111_1111_1111_0000_1111_0000_0000_0000 {
                0xF890_F000 => { let rn = ((word >> 16) & 0b1111) as u8; return Ok(Some(Self::Pld_Immediate_T1(g(rn), (word & 0xFFF) as i32))); },
                0xF990_F000 => { let rn = ((word >> 16) & 0b1111) as u8; return Ok(Some(Self::Pli_Immediate_T1(g(rn), (word & 0xFFF) as i32))); },
                _ => (),
            }

            // M7i PLD / PLI, negative (imm8) form (`[Rn, #-imm8]`). Rt==1111 and bits[11:8]=1100; intercept
            // before the indexed block.  mask: keep op + Rt==1111 + the 1100 P/U/W field.
            match word & 0b1111_1111_1111_0000_1111_1111_0000_0000 {
                0xF810_FC00 => { let rn = ((word >> 16) & 0b1111) as u8; return Ok(Some(Self::Pld_Immediate_T1(g(rn), -((word & 0xFF) as i32)))); },
                0xF910_FC00 => { let rn = ((word >> 16) & 0b1111) as u8; return Ok(Some(Self::Pli_Immediate_T1(g(rn), -((word & 0xFF) as i32)))); },
                _ => (),
            }

            // LDR.W / STR.W (immediate, T3)  mask: 0b1111_1111_1111_0000_0000_0000_0000_0000 (imm12)
            match word & 0b1111_1111_1111_0000_0000_0000_0000_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Ldr_Immediate_T3 as u32 => {
                    let (rn, rt, imm12) = decode_word_rn_rt_imm12(word);
                    return Ok(Some(Self::Ldr_Immediate_T3(Arm32GeneralPurposeRegister::from_operand_bits(rt), Arm32GeneralPurposeRegister::from_operand_bits(rn), imm12)));
                },
                op if op == ArmT32OpcodePattern_32Bit::Str_Immediate_T3 as u32 => {
                    let (rn, rt, imm12) = decode_word_rn_rt_imm12(word);
                    return Ok(Some(Self::Str_Immediate_T3(Arm32GeneralPurposeRegister::from_operand_bits(rt), Arm32GeneralPurposeRegister::from_operand_bits(rn), imm12)));
                },
                // M7h byte/half immediate-offset (imm12) forms
                0xF890_0000 => { let (rn, rt, i) = decode_word_rn_rt_imm12(word); return Ok(Some(Self::Ldrb_Immediate_T2(g(rt), g(rn), i))); },
                0xF880_0000 => { let (rn, rt, i) = decode_word_rn_rt_imm12(word); return Ok(Some(Self::Strb_Immediate_T2(g(rt), g(rn), i))); },
                0xF8B0_0000 => { let (rn, rt, i) = decode_word_rn_rt_imm12(word); return Ok(Some(Self::Ldrh_Immediate_T2(g(rt), g(rn), i))); },
                0xF8A0_0000 => { let (rn, rt, i) = decode_word_rn_rt_imm12(word); return Ok(Some(Self::Strh_Immediate_T2(g(rt), g(rn), i))); },
                0xF990_0000 => { let (rn, rt, i) = decode_word_rn_rt_imm12(word); return Ok(Some(Self::Ldrsb_Immediate_T1(g(rt), g(rn), i))); },
                0xF9B0_0000 => { let (rn, rt, i) = decode_word_rn_rt_imm12(word); return Ok(Some(Self::Ldrsh_Immediate_T1(g(rt), g(rn), i))); },
                _ => (),
            }

            // M7h register-offset load/store  mask: 0b1111_1111_1111_0000_0000_1111_1100_0000 ([11:6]=0)
            match word & 0b1111_1111_1111_0000_0000_1111_1100_0000 {
                0xF850_0000 => { let (rn, rt, l, rm) = decode_word_rn_rt_lsl_rm(word); return Ok(Some(Self::Ldr_Register_T2(g(rt), g(rn), g(rm), l))); },
                0xF840_0000 => { let (rn, rt, l, rm) = decode_word_rn_rt_lsl_rm(word); return Ok(Some(Self::Str_Register_T2(g(rt), g(rn), g(rm), l))); },
                0xF810_0000 => { let (rn, rt, l, rm) = decode_word_rn_rt_lsl_rm(word); return Ok(Some(Self::Ldrb_Register_T2(g(rt), g(rn), g(rm), l))); },
                0xF800_0000 => { let (rn, rt, l, rm) = decode_word_rn_rt_lsl_rm(word); return Ok(Some(Self::Strb_Register_T2(g(rt), g(rn), g(rm), l))); },
                0xF830_0000 => { let (rn, rt, l, rm) = decode_word_rn_rt_lsl_rm(word); return Ok(Some(Self::Ldrh_Register_T2(g(rt), g(rn), g(rm), l))); },
                0xF820_0000 => { let (rn, rt, l, rm) = decode_word_rn_rt_lsl_rm(word); return Ok(Some(Self::Strh_Register_T2(g(rt), g(rn), g(rm), l))); },
                0xF910_0000 => { let (rn, rt, l, rm) = decode_word_rn_rt_lsl_rm(word); return Ok(Some(Self::Ldrsb_Register_T2(g(rt), g(rn), g(rm), l))); },
                0xF930_0000 => { let (rn, rt, l, rm) = decode_word_rn_rt_lsl_rm(word); return Ok(Some(Self::Ldrsh_Register_T2(g(rt), g(rn), g(rm), l))); },
                _ => (),
            }

            // Unprivileged load/store (LDRT/STRT family): [31:25]=1111100, [24]=S (sign-extend), [23]=0,
            // [22:21]=size (00 B / 01 H / 10 W), [20]=L, Rn[19:16]; hw2 Rt[15:12], [11:8]=1110 (the T marker),
            // imm8[7:0]. The [11:8]=1110 marker keeps these distinct from the regular indexed T4/T3 forms below.
            if word & 0xFE80_0F00 == 0xF800_0E00 {
                let signed = (word >> 24) & 1 == 1;
                let size = ((word >> 21) & 0b11) as u8;
                let load = (word >> 20) & 1 == 1;
                // valid combos only: 5 loads (signed only on byte/half) + 3 unsigned stores
                if size <= 2 && (!signed || (load && size < 2)) {
                    let rn = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 16) & 0xF) as u8);
                    let rt = Arm32GeneralPurposeRegister::from_operand_bits(((word >> 12) & 0xF) as u8);
                    return Ok(Some(Self::UnprivLoadStore_T1(load, signed, size, rt, rn, (word & 0xFF) as u8)));
                }
            }

            // M7i single-register indexed load/store (T4/T3/T2): `Rt, [Rn, #+/-imm8]{!}` / `[Rn], #+/-imm8`.
            // Bit 11 is the marker that separates these from the register-offset forms above; the helper
            // returns None for the unmodeled LDRT/STRT (P=0,W=0) and (P=1,U=1,W=0) cases (-> InvalidOpcode).
            match word & 0b1111_1111_1111_0000_0000_1000_0000_0000 {
                0xF850_0800 => { if let Some((rt, rn, off, mode)) = decode_word_load_store_indexed(word) { return Ok(Some(Self::Ldr_Immediate_T4(g(rt), g(rn), off, mode))); } },
                0xF840_0800 => { if let Some((rt, rn, off, mode)) = decode_word_load_store_indexed(word) { return Ok(Some(Self::Str_Immediate_T4(g(rt), g(rn), off, mode))); } },
                0xF810_0800 => { if let Some((rt, rn, off, mode)) = decode_word_load_store_indexed(word) { return Ok(Some(Self::Ldrb_Immediate_T3(g(rt), g(rn), off, mode))); } },
                0xF800_0800 => { if let Some((rt, rn, off, mode)) = decode_word_load_store_indexed(word) { return Ok(Some(Self::Strb_Immediate_T3(g(rt), g(rn), off, mode))); } },
                0xF830_0800 => { if let Some((rt, rn, off, mode)) = decode_word_load_store_indexed(word) { return Ok(Some(Self::Ldrh_Immediate_T3(g(rt), g(rn), off, mode))); } },
                0xF820_0800 => { if let Some((rt, rn, off, mode)) = decode_word_load_store_indexed(word) { return Ok(Some(Self::Strh_Immediate_T3(g(rt), g(rn), off, mode))); } },
                0xF910_0800 => { if let Some((rt, rn, off, mode)) = decode_word_load_store_indexed(word) { return Ok(Some(Self::Ldrsb_Immediate_T2(g(rt), g(rn), off, mode))); } },
                0xF930_0800 => { if let Some((rt, rn, off, mode)) = decode_word_load_store_indexed(word) { return Ok(Some(Self::Ldrsh_Immediate_T2(g(rt), g(rn), off, mode))); } },
                _ => (),
            }

            // LDREX  Rt,[Rn,#imm]  mask: 0b1111_1111_1111_0000_0000_1111_0000_0000 (imm8 = offset/4)
            match word & 0b1111_1111_1111_0000_0000_1111_0000_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Ldrex_T1 as u32 => {
                    let rn = ((word >> 16) & 0b1111) as u8;
                    let rt = ((word >> 12) & 0b1111) as u8;
                    let imm = ((word & 0b1111_1111) as u16) * 4;
                    return Ok(Some(Self::Ldrex_T1(Arm32GeneralPurposeRegister::from_operand_bits(rt), Arm32GeneralPurposeRegister::from_operand_bits(rn), imm)));
                },
                _ => (),
            }

            // STREX  Rd,Rt,[Rn,#imm]  mask: 0b1111_1111_1111_0000_0000_0000_0000_0000 (imm8 = offset/4)
            match word & 0b1111_1111_1111_0000_0000_0000_0000_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Strex_T1 as u32 => {
                    let rn = ((word >> 16) & 0b1111) as u8;
                    let rt = ((word >> 12) & 0b1111) as u8;
                    let rd = ((word >> 8) & 0b1111) as u8;
                    let imm = ((word & 0b1111_1111) as u16) * 4;
                    return Ok(Some(Self::Strex_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rt), Arm32GeneralPurposeRegister::from_operand_bits(rn), imm)));
                },
                _ => (),
            }

            // LDREXB / LDREXH  Rt,[Rn]  mask: 0b1111_1111_1111_0000_0000_1111_1111_1111 (Rt in 15:12 is variable)
            match word & 0b1111_1111_1111_0000_0000_1111_1111_1111 {
                op if op == ArmT32OpcodePattern_32Bit::Ldrexb_T1 as u32 => {
                    let (rn, rt) = (((word >> 16) & 0b1111) as u8, ((word >> 12) & 0b1111) as u8);
                    return Ok(Some(Self::Ldrexb_T1(Arm32GeneralPurposeRegister::from_operand_bits(rt), Arm32GeneralPurposeRegister::from_operand_bits(rn))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Ldrexh_T1 as u32 => {
                    let (rn, rt) = (((word >> 16) & 0b1111) as u8, ((word >> 12) & 0b1111) as u8);
                    return Ok(Some(Self::Ldrexh_T1(Arm32GeneralPurposeRegister::from_operand_bits(rt), Arm32GeneralPurposeRegister::from_operand_bits(rn))));
                },
                _ => (),
            }

            // STREXB / STREXH  Rd,Rt,[Rn]  mask: 0b1111_1111_1111_0000_0000_1111_1111_0000 (Rt in 15:12 is variable)
            match word & 0b1111_1111_1111_0000_0000_1111_1111_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Strexb_T1 as u32 => {
                    let rn = ((word >> 16) & 0b1111) as u8;
                    let rt = ((word >> 12) & 0b1111) as u8;
                    let rd = (word & 0b1111) as u8;
                    return Ok(Some(Self::Strexb_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rt), Arm32GeneralPurposeRegister::from_operand_bits(rn))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Strexh_T1 as u32 => {
                    let rn = ((word >> 16) & 0b1111) as u8;
                    let rt = ((word >> 12) & 0b1111) as u8;
                    let rd = (word & 0b1111) as u8;
                    return Ok(Some(Self::Strexh_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rt), Arm32GeneralPurposeRegister::from_operand_bits(rn))));
                },
                _ => (),
            }

            // LDA/STL acquire-release family: [31:24]=0xE8, [23:21]=110, [20]=L, Rn[19:16]; hw2 Rt[15:12],
            // [11:8]=1111, [7]=1, [6]=exclusive, [5:4]=size (00 B / 01 H / 10 W). [7]=1 separates these from
            // LDREX/STREX ([7]=0). Store-exclusive (L=0, exclusive=1) takes Rd in [3:0]. size==0b11 is reserved.
            if word & 0xFFE0_0F80 == 0xE8C0_0F80 && (word >> 4) & 0b11 != 0b11 {
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                let size = ((word >> 4) & 0b11) as u8;
                let exclusive = (word >> 6) & 1 == 1;
                let rn = g(((word >> 16) & 0xF) as u8);
                let rt = g(((word >> 12) & 0xF) as u8);
                if (word >> 20) & 1 == 1 {
                    return Ok(Some(Self::LoadAcquire_T1(size, exclusive, rt, rn)));
                } else if exclusive {
                    return Ok(Some(Self::StoreReleaseExclusive_T1(size, g((word & 0xF) as u8), rt, rn)));
                } else {
                    return Ok(Some(Self::StoreRelease_T1(size, rt, rn)));
                }
            }

            // TBB / TBH  [Rn,Rm]  mask: 0b1111_1111_1111_0000_1111_1111_1111_0000 (bits 15:12 fixed = 1111)
            match word & 0b1111_1111_1111_0000_1111_1111_1111_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Tbb_T1 as u32 => {
                    let (rn, rm) = (((word >> 16) & 0b1111) as u8, (word & 0b1111) as u8);
                    return Ok(Some(Self::Tbb_T1(Arm32GeneralPurposeRegister::from_operand_bits(rn), Arm32GeneralPurposeRegister::from_operand_bits(rm))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Tbh_T1 as u32 => {
                    let (rn, rm) = (((word >> 16) & 0b1111) as u8, (word & 0b1111) as u8);
                    return Ok(Some(Self::Tbh_T1(Arm32GeneralPurposeRegister::from_operand_bits(rn), Arm32GeneralPurposeRegister::from_operand_bits(rm))));
                },
                _ => (),
            }

            // MLA / MLS  (checked after MUL above, since MUL is MLA with Ra==0b1111)  mask: 0b1111_1111_1111_0000_0000_0000_1111_0000
            match word & 0b1111_1111_1111_0000_0000_0000_1111_0000 {
                op if op == ArmT32OpcodePattern_32Bit::Mla_T1 as u32 => {
                    let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word);
                    return Ok(Some(Self::Mla_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), Arm32GeneralPurposeRegister::from_operand_bits(rm), Arm32GeneralPurposeRegister::from_operand_bits(ra))));
                },
                op if op == ArmT32OpcodePattern_32Bit::Mls_T1 as u32 => {
                    let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word);
                    return Ok(Some(Self::Mls_T1(Arm32GeneralPurposeRegister::from_operand_bits(rd), Arm32GeneralPurposeRegister::from_operand_bits(rn), Arm32GeneralPurposeRegister::from_operand_bits(rm), Arm32GeneralPurposeRegister::from_operand_bits(ra))));
                },
                // M8c DSP USAD8 / USADA8 (Ra==1111 -> USAD8): Rn 19:16, Ra 15:12, Rd 11:8, Rm 3:0
                0xFB70_0000 => {
                    let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word);
                    return Ok(Some(if ra == 0b1111 { Self::Usad8_T1(g(rd), g(rn), g(rm)) } else { Self::Usada8_T1(g(rd), g(rn), g(rm), g(ra)) }));
                },
                _ => (),
            }

            // M8e DSP halfword multiply SMUL*/SMLA* (op2[7:6]=00, n=[5], m=[4]; Ra==1111 -> SMUL).
            if word & 0b1111_1111_1111_0000_0000_0000_1100_0000 == 0xFB10_0000 {
                let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word);
                let (n, m) = ((word >> 5) & 1 == 1, (word >> 4) & 1 == 1);
                return Ok(Some(if ra == 0b1111 { Self::Smul_T1(g(rd), g(rn), g(rm), n, m) } else { Self::Smla_T1(g(rd), g(rn), g(rm), g(ra), n, m) }));
            }

            // M8e DSP dual / word-by-halfword / most-significant-word multiplies (op2[7:5]=000, one bit at [4]).
            match word & 0b1111_1111_1111_0000_0000_0000_1110_0000 {
                0xFB20_0000 => { let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word); let x = (word >> 4) & 1 == 1;
                    return Ok(Some(if ra == 0b1111 { Self::Smuad_T1(g(rd), g(rn), g(rm), x) } else { Self::Smlad_T1(g(rd), g(rn), g(rm), g(ra), x) })); },
                0xFB30_0000 => { let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word); let m = (word >> 4) & 1 == 1;
                    return Ok(Some(if ra == 0b1111 { Self::Smulw_T1(g(rd), g(rn), g(rm), m) } else { Self::Smlaw_T1(g(rd), g(rn), g(rm), g(ra), m) })); },
                0xFB40_0000 => { let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word); let x = (word >> 4) & 1 == 1;
                    return Ok(Some(if ra == 0b1111 { Self::Smusd_T1(g(rd), g(rn), g(rm), x) } else { Self::Smlsd_T1(g(rd), g(rn), g(rm), g(ra), x) })); },
                0xFB50_0000 => { let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word); let round = (word >> 4) & 1 == 1;
                    return Ok(Some(if ra == 0b1111 { Self::Smmul_T1(g(rd), g(rn), g(rm), round) } else { Self::Smmla_T1(g(rd), g(rn), g(rm), g(ra), round) })); },
                0xFB60_0000 => { let (rn, ra, rd, rm) = decode_word_rn_ra_rd_rm(word); let round = (word >> 4) & 1 == 1;
                    return Ok(Some(Self::Smmls_T1(g(rd), g(rn), g(rm), g(ra), round))); },
                _ => (),
            }

            // M7k long multiply (RdLo 15:12, RdHi 11:8, Rn 19:16, Rm 3:0)  mask: 0b1111_1111_1111_0000_0000_0000_1111_0000
            match word & 0b1111_1111_1111_0000_0000_0000_1111_0000 {
                0xFB80_0000 => { let (rn, rl, rh, rm) = decode_word_long_multiply(word); return Ok(Some(Self::Smull_T1(g(rl), g(rh), g(rn), g(rm)))); },
                0xFBA0_0000 => { let (rn, rl, rh, rm) = decode_word_long_multiply(word); return Ok(Some(Self::Umull_T1(g(rl), g(rh), g(rn), g(rm)))); },
                0xFBC0_0000 => { let (rn, rl, rh, rm) = decode_word_long_multiply(word); return Ok(Some(Self::Smlal_T1(g(rl), g(rh), g(rn), g(rm)))); },
                0xFBE0_0000 => { let (rn, rl, rh, rm) = decode_word_long_multiply(word); return Ok(Some(Self::Umlal_T1(g(rl), g(rh), g(rn), g(rm)))); },
                0xFBE0_0060 => { let (rn, rl, rh, rm) = decode_word_long_multiply(word); return Ok(Some(Self::Umaal_T1(g(rl), g(rh), g(rn), g(rm)))); },
                _ => (),
            }

            // M8e DSP long signed multiplies sharing the SMLAL opcode but with a nonzero op2 nibble:
            // SMLAL<x><y> (op2=10NM), SMLALD (op2=110X), SMLSLD (0xFBD0, op2=110X).
            match word & 0b1111_1111_1111_0000_0000_0000_1111_0000 {
                0xFBC0_0080 | 0xFBC0_0090 | 0xFBC0_00A0 | 0xFBC0_00B0 => {
                    let (rn, rl, rh, rm) = decode_word_long_multiply(word);
                    let (n, m) = ((word >> 5) & 1 == 1, (word >> 4) & 1 == 1);
                    return Ok(Some(Self::Smlal_Halfword_T1(g(rl), g(rh), g(rn), g(rm), n, m)));
                },
                0xFBC0_00C0 | 0xFBC0_00D0 => { let (rn, rl, rh, rm) = decode_word_long_multiply(word); return Ok(Some(Self::Smlald_T1(g(rl), g(rh), g(rn), g(rm), (word >> 4) & 1 == 1))); },
                0xFBD0_00C0 | 0xFBD0_00D0 => { let (rn, rl, rh, rm) = decode_word_long_multiply(word); return Ok(Some(Self::Smlsld_T1(g(rl), g(rh), g(rn), g(rm), (word >> 4) & 1 == 1))); },
                _ => (),
            }

            // M7l wide extend SXTB.W/UXTB.W/SXTH.W/UXTH.W (Rn==1111 -> no add).  Rd 11:8, rotate 5:4, Rm 3:0
            // mask keeps op (incl. Rn==1111) and the fixed 1111 in 15:12 / bit7=1,bit6=0; lets Rd, rotate, Rm vary.
            match word & 0b1111_1111_1111_1111_1111_0000_1100_0000 {
                0xFA4F_F080 => { let (rd, rot, rm) = decode_word_extend(word); return Ok(Some(Self::Sxtb_T2(g(rd), g(rm), rot))); },
                0xFA5F_F080 => { let (rd, rot, rm) = decode_word_extend(word); return Ok(Some(Self::Uxtb_T2(g(rd), g(rm), rot))); },
                0xFA0F_F080 => { let (rd, rot, rm) = decode_word_extend(word); return Ok(Some(Self::Sxth_T2(g(rd), g(rm), rot))); },
                0xFA1F_F080 => { let (rd, rot, rm) = decode_word_extend(word); return Ok(Some(Self::Uxth_T2(g(rd), g(rm), rot))); },
                _ => (),
            }

            // M8b DSP extend-and-add: same opcodes as the M7l extends but with Rn != 1111 (the *16 forms
            // are new; their Rn==1111 case is SXTB16/UXTB16).  mask lets Rn / Rd / rotate / Rm vary.
            match word & 0b1111_1111_1111_0000_1111_0000_1100_0000 {
                0xFA00_F080 => { let (rd, rn, rm, rot) = decode_word_extend_and_add(word); return Ok(Some(Self::Sxtah_T1(g(rd), g(rn), g(rm), rot))); },
                0xFA10_F080 => { let (rd, rn, rm, rot) = decode_word_extend_and_add(word); return Ok(Some(Self::Uxtah_T1(g(rd), g(rn), g(rm), rot))); },
                0xFA40_F080 => { let (rd, rn, rm, rot) = decode_word_extend_and_add(word); return Ok(Some(Self::Sxtab_T1(g(rd), g(rn), g(rm), rot))); },
                0xFA50_F080 => { let (rd, rn, rm, rot) = decode_word_extend_and_add(word); return Ok(Some(Self::Uxtab_T1(g(rd), g(rn), g(rm), rot))); },
                0xFA20_F080 => { let (rd, rn, rm, rot) = decode_word_extend_and_add(word);
                    if rn == 0b1111 { return Ok(Some(Self::Sxtb16_T1(g(rd), g(rm), rot))); } else { return Ok(Some(Self::Sxtab16_T1(g(rd), g(rn), g(rm), rot))); } },
                0xFA30_F080 => { let (rd, rn, rm, rot) = decode_word_extend_and_add(word);
                    if rn == 0b1111 { return Ok(Some(Self::Uxtb16_T1(g(rd), g(rm), rot))); } else { return Ok(Some(Self::Uxtab16_T1(g(rd), g(rn), g(rm), rot))); } },
                _ => (),
            }

            // M8c DSP SSAT16 / USAT16 (no shift): keyed on [15:12]=0000 and [7:4]=0000, which separates them
            // from SSAT(ASR) -- that always has a nonzero shift (SSAT ASR has no #0 form). Decode BEFORE SSAT.
            match word & 0b1111_1111_1111_0000_1111_0000_1111_0000 {
                0xF320_0000 => { let rn = ((word >> 16) & 0b1111) as u8; let rd = ((word >> 8) & 0b1111) as u8; return Ok(Some(Self::Ssat16_T1(g(rd), ((word & 0b1111) as u8) + 1, g(rn)))); },
                0xF3A0_0000 => { let rn = ((word >> 16) & 0b1111) as u8; let rd = ((word >> 8) & 0b1111) as u8; return Ok(Some(Self::Usat16_T1(g(rd), (word & 0b1111) as u8, g(rn)))); },
                _ => (),
            }

            // M7l SSAT / USAT.  sh 21, Rn 19:16, imm3 14:12, Rd 11:8, imm2 7:6, sat 4:0
            // mask keeps the op bits but lets sh (the LSL/ASR selector) and all operand fields vary.
            match word & 0b1111_1111_1101_0000_1000_0000_0010_0000 {
                0xF300_0000 => { let (rd, sat, rn, shift) = decode_word_saturate(word, false); return Ok(Some(Self::Ssat_T1(g(rd), sat, g(rn), shift))); },
                0xF380_0000 => { let (rd, sat, rn, shift) = decode_word_saturate(word, true); return Ok(Some(Self::Usat_T1(g(rd), sat, g(rn), shift))); },
                _ => (),
            }

            // ---- CDE (Custom Datapath Extension): CX1/CX2/CX3 (+A accumulate / +D dual) in coproc 0-7. The CDE
            // coprocessor space overlaps the generic CDP/MCR space, so a coprocessor the `context` marks CDE is
            // decoded here (before the generic coprocessor below). [27:25]=111, [23:22]=00/01/10 = CX1/CX2/CX3. ----
            // CX1 = [23:22]=00, CX2 = [23:22]=01, CX3 = [23]=1 (its [22] carries imm[5], so test bit23 alone).
            if (word >> 25) & 0b111 == 0b111 && context.is_cde_coprocessor(((word >> 8) & 0xF) as u8) && (word >> 24) & 1 == 0 {
                let g = Arm32GeneralPurposeRegister::from_operand_bits;
                let (acc, dual, cp) = ((word >> 28) & 1 == 1, (word >> 6) & 1 == 1, ((word >> 8) & 0xF) as u8);
                if (word >> 23) & 1 == 1 {   // CX3
                    let imm = (((word >> 4) & 0x3) | (((word >> 7) & 1) << 2) | (((word >> 20) & 0x7) << 3)) as u8;
                    return Ok(Some(Self::Cde_Cx3_T1(acc, dual, cp, g((word & 0xF) as u8), g(((word >> 16) & 0xF) as u8), g(((word >> 12) & 0xF) as u8), imm)));
                } else if (word >> 22) & 1 == 1 {   // CX2
                    let imm = ((word & 0x3F) | (((word >> 7) & 1) << 6) | (((word >> 20) & 0x3) << 7)) as u16;
                    return Ok(Some(Self::Cde_Cx2_T1(acc, dual, cp, g(((word >> 12) & 0xF) as u8), g(((word >> 16) & 0xF) as u8), imm)));
                } else {   // CX1
                    let imm = ((word & 0x3F) | (((word >> 7) & 1) << 6) | (((word >> 16) & 0x3F) << 7)) as u16;
                    return Ok(Some(Self::Cde_Cx1_T1(acc, dual, cp, g(((word >> 12) & 0xF) as u8), imm)));
                }
            }

            // ---- CDE VCX1/VCX2/VCX3 -- the FP/vector-register cousins of CX1/2/3 (coproc 0-7). These sit in
            // the 111_0_110 (0xEC/0xED/0xFC/0xFD) generic-coprocessor space (bit25=0, vs CX's bit25=1), so like
            // CX they are decoded before the generic coprocessor below. Arity key: bit23=1 -> VCX3; else
            // bit20 selects VCX1 (0) / VCX2 (1). kind: bit6=1 -> vector (Q), else bit24=1 -> double (D), else
            // single (S). acc = bit28, coproc = [10:8]. ----
            if (word >> 29) == 0b111 && (word >> 25) & 0b111 == 0b110 && context.is_cde_coprocessor(((word >> 8) & 0xF) as u8) {
                let acc = (word >> 28) & 1 == 1;
                let kind = if (word >> 6) & 1 == 1 { 2 } else if (word >> 24) & 1 == 1 { 1 } else { 0 };
                let cp = ((word >> 8) & 7) as u8;
                if word & 0xEE80_0000 == 0xEC80_0000 {   // VCX3 (bit23=1)
                    return Ok(Some(Self::Vcx3_T1(acc, kind, cp, vcx_dec_d(kind, word), vcx_dec_hi(kind, word), vcx_dec_lo(kind, word), vcx_dec_imm3(word))));
                } else if word & 0xEEB0_0000 == 0xEC30_0000 {   // VCX2 (bit23=0, bit20=1)
                    return Ok(Some(Self::Vcx2_T1(acc, kind, cp, vcx_dec_d(kind, word), vcx_dec_lo(kind, word), vcx_dec_imm2(word))));
                } else if word & 0xEEB0_0000 == 0xEC20_0000 {   // VCX1 (bit23=0, bit20=0)
                    return Ok(Some(Self::Vcx1_T1(acc, kind, cp, vcx_dec_d(kind, word), vcx_dec_imm1(word))));
                }
            }

            // ---- Generic coprocessor (MCR/MRC/CDP/MCRR/MRRC/LDC/STC + 2/L). Decoded LAST so the FP (cp10/11)
            // and MVE decoders claim their words first; this handles the remaining generic coprocessor space. ----
            let top = word >> 24;
            let cp = ((word >> 8) & 0xF) as u8;
            // A coprocessor the context marks CDE was already handled as CX / VCX above; the GENERIC
            // coprocessor handles every OTHER coprocessor (all of 8-15, plus any of 0-7 the context says is
            // NOT CDE). Without this gate a generic LDC/STC/CDP/MCR/MCRR with a CDE coprocessor round-trip-
            // aliases a CDE instruction (the cargo-fuzz `t32_instruction_stream` target found exactly this:
            // LDC2 p0 and VCX3 both encode to 0xFD94_0000 -- see `ArmDecodeContext` / Rule R4).
            if !context.is_cde_coprocessor(cp) {
                if (top == 0xEC || top == 0xFC) && (word >> 21) & 0b111 == 0b010 {   // MCRR / MRRC
                    return Ok(Some(Self::Coproc_Mcrr_T1(top == 0xFC, (word >> 20) & 1 == 1, cp,
                        ((word >> 4) & 0xF) as u8, g(((word >> 12) & 0xF) as u8), g(((word >> 16) & 0xF) as u8), (word & 0xF) as u8)));
                }
                if (top == 0xED || top == 0xFD) && (word >> 21) & 1 == 0 {            // LDC / STC (offset form: P=1, W=0)
                    let imm = ((word & 0xFF) as i32) * 4 * (if (word >> 23) & 1 == 1 { 1 } else { -1 });
                    return Ok(Some(Self::Coproc_Ldc_T1(top == 0xFD, (word >> 22) & 1 == 1, (word >> 20) & 1 == 1, cp,
                        ((word >> 12) & 0xF) as u8, g(((word >> 16) & 0xF) as u8), imm)));
                }
                if top == 0xEE || top == 0xFE {
                    let two = top == 0xFE;
                    if (word >> 4) & 1 == 1 {                                         // MCR / MRC
                        return Ok(Some(Self::Coproc_Mcr_T1(two, (word >> 20) & 1 == 1, cp,
                            ((word >> 21) & 0b111) as u8, g(((word >> 12) & 0xF) as u8), ((word >> 16) & 0xF) as u8, (word & 0xF) as u8, ((word >> 5) & 0b111) as u8)));
                    } else {                                                          // CDP
                        return Ok(Some(Self::Coproc_Cdp_T1(two, cp, ((word >> 20) & 0xF) as u8,
                            ((word >> 12) & 0xF) as u8, ((word >> 16) & 0xF) as u8, (word & 0xF) as u8, ((word >> 5) & 0b111) as u8)));
                    }
                }
            }
        }

        // if we could not match against any known opcode, return an invalid opcode error; the iter_offset value will indicate the new index in the iterator
        Err(DecodeError::InvalidOpcode)
    }

    /// Encode this instruction to its little-endian machine-code bytes -- 2 bytes for a 16-bit Thumb
    /// encoding, 4 for a 32-bit one. Returns [`EncodeError`] if an operand field is out of range for the
    /// encoding. The bytes are target-independent; use [`encode_for_target`](Self::encode_for_target) to
    /// also check that a given profile supports the instruction.
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        let halfwords = match self {
            Self::Adc_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Adc_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Add_Immediate_T1(rd, rn, imm3) => {
                check_unsigned_maximum("imm3", *imm3 as u32, 7)?;

                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Add_Immediate_T1, rd.as_operand_bits(), rn.as_operand_bits(), *imm3);
                vec![halfword0]
            },
            Self::Add_Immediate_T2(rdn, imm8) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Add_Immediate_T2, *imm8, rdn.as_operand_bits());
                vec![halfword0]
            },
            Self::Add_Register_T1(rd, rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Add_Register_T1, rd.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Add_Register_T2(n, m) => {
                if *n == Arm32GeneralPurposeRegister::R13 {
                    return Err(EncodeError::RegisterNotEncodable { field: "n", detail: "n cannot be SP (R13); use Add_SpPlusRegister_T2 instead" });
                }
                if *m == Arm32GeneralPurposeRegister::R13 {
                    return Err(EncodeError::RegisterNotEncodable { field: "m", detail: "m cannot be SP (R13); use Add_SpPlusRegister_T1 instead" });
                }

                let rdn_as_u3 = n.as_operand_bits() & 0b0000_0111;
                let dn_as_u1 = (n.as_operand_bits() & 0b0000_1000) >> 3;
                let rm_as_u4 = m.as_operand_bits();
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0306__0707(ArmT32OpcodePattern_16Bit::Add_Register_T2, rdn_as_u3, rm_as_u4, dn_as_u1);
                vec![halfword0]
            },
            Self::Add_SpPlusImmediate_T1(rd, const10) => {
                check_multiple_of("const10", *const10 as i64, 4)?;
                check_unsigned_maximum("const10", *const10 as u32, 1020)?;

                let imm8 = (const10 / 4) as u8;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Add_SpPlusImmediate_T1, imm8, rd.as_operand_bits());
                vec![halfword0]
            },
            Self::Add_SpPlusImmediate_T2(const9) => {
                check_multiple_of("const9", *const9 as i64, 4)?;
                // The T2 field is imm7, so const9 (= imm7 * 4) maxes at 127 * 4 = 508. (The prior code
                // checked <= 1020, which would have overflowed the 7-bit field and corrupted the opcode.)
                check_unsigned_maximum("const9", *const9 as u32, 508)?;

                let imm7 = (const9 / 4) as u8;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0006(ArmT32OpcodePattern_16Bit::Add_SpPlusImmediate_T2, imm7);
                vec![halfword0]
            },
            Self::Add_SpPlusRegister_T1(m) => { 
                // NOTE: this encoding is the same encoding as ::AddRegister_T2 -- but with 0b1101 specified in bits 3..=6 (i.e. "rm" in ::AddRegister_T2)
                let rdm_as_u3 = m.as_operand_bits() & 0b0000_0111;
                let dm_as_u1 = (m.as_operand_bits() & 0b0000_1000) >> 3;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0707(ArmT32OpcodePattern_16Bit::Add_SpPlusRegister_T1, rdm_as_u3, dm_as_u1);
                vec![halfword0]
            },
            Self::Add_SpPlusRegister_T2(rm) => {
                if *rm == Arm32GeneralPurposeRegister::R13 {
                    return Err(EncodeError::RegisterNotEncodable { field: "rm", detail: "rm cannot be SP (R13); use Add_SpPlusRegister_T1 instead" });
                }

                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0306(ArmT32OpcodePattern_16Bit::Add_SpPlusRegister_T2, rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Adr_T1(rd, const10) => {
                check_multiple_of("const10", *const10 as i64, 4)?;
                check_unsigned_maximum("const10", *const10 as u32, 1020)?;

                let imm8 = (const10 / 4) as u8;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Adr_T1, imm8, rd.as_operand_bits());
                vec![halfword0]
            },
            Self::And_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::And_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Asr_Immediate_T1(rd, rm, decoded_imm5) => {
                check_signed_range("decoded_imm5", *decoded_imm5 as i32, 1, 32)?;

                // NOTE: "A6.4.1 Shift operations" of the ARVv6-M ISA doc indicates that an imm5 value of ZERO is interpeted as an imm5 value of 32 (because shift 'type' is '10')
                let encoded_imm5 = if *decoded_imm5 == 32 { 0 } else { *decoded_imm5 };
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Asr_Immediate_T1, rd.as_operand_bits(), rm.as_operand_bits(), encoded_imm5);
                vec![halfword0]
            },
            Self::Asr_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Asr_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::B_T1(cond, decoded_signed_imm9) => {
                // NOTE: condition "AL" is _not_ allowed for B_T1 in ARMv6-M; this maps to "Udf_T1" so we don't need to worry about this issue when decoding--but when encoding we do prohibit this condition
                //       [we also handle the cond 0b1111 out of an abundance of caution, even though it's not an encodable condition]
                if *cond == ArmT32InstructionCondition::AlwaysUnconditional {
                    return Err(EncodeError::ConditionNotEncodable { field: "cond", detail: "the AL (0b1110) condition is not encodable in B<c> T1; that slot is UDF" });
                }
                if *cond == ArmT32InstructionCondition::Undefined(0b1111) {
                    return Err(EncodeError::ConditionNotEncodable { field: "cond", detail: "the 0b1111 condition is not encodable in B<c> T1; that slot is SVC" });
                }

                check_multiple_of("decoded_signed_imm9", *decoded_signed_imm9 as i64, 2)?;
                check_signed_range("decoded_signed_imm9", *decoded_signed_imm9 as i32, -256, 254)?;

                let encoded_signed_imm8 = (decoded_signed_imm9 / 2) as i8;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__s080b(ArmT32OpcodePattern_16Bit::B_T1, encoded_signed_imm8, cond.as_operand_bits());
                vec![halfword0]
            },
            Self::B_T2(decoded_signed_imm12) => {
                check_multiple_of("decoded_signed_imm12", *decoded_signed_imm12 as i64, 2)?;
                check_signed_range("decoded_signed_imm12", *decoded_signed_imm12 as i32, -2048, 2046)?;

                let encoded_signed_imm11 = decoded_signed_imm12 / 2;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__s000a(ArmT32OpcodePattern_16Bit::B_T2, encoded_signed_imm11);
                vec![halfword0]
            },
            Self::Bic_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Bic_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Bkpt_T1(imm8) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007(ArmT32OpcodePattern_16Bit::Bkpt_T1, *imm8);
                vec![halfword0]
            },
            Self::Bl_T1(decoded_signed_imm25) => {
                check_multiple_of("decoded_signed_imm25", *decoded_signed_imm25 as i64, 2)?;
                check_signed_range("decoded_signed_imm25", *decoded_signed_imm25, -16_777_216, 16_777_214)?;

                let encoded_signed_imm24 = *decoded_signed_imm25 / 2;
                let s = (((encoded_signed_imm24 as u32) >> 23) & 0b0000_0001) as u8;
                let i1 = (((encoded_signed_imm24 as u32) >> 22) & 0b0000_0001) as u8;
                let i2 = (((encoded_signed_imm24 as u32) >> 21) & 0b0000_0001) as u8;
                let imm10 = (((encoded_signed_imm24 as u32) >> 11) & 0b0000_0011_1111_1111) as u16;
                let imm11 = ((encoded_signed_imm24 as u32) & 0b0000_0111_1111_1111) as u16;
                //
                let j1 = ((!i1) ^ s) & 0b1;
                let j2 = ((!i2) ^ s) & 0b1;
                //
                let word = ArmT32InstructionEncoder::encode_instruction_word__000a__0b0b__0d0d__1019__1a1a(ArmT32OpcodePattern_32Bit::Bl_T1, imm11, j2, j1, imm10, s);
                let halfwords = split_instruction_word_into_halfwords(word);
                halfwords.to_vec()
            },
            Self::Blx_Register_T1(rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0306(ArmT32OpcodePattern_16Bit::Blx_Register_T1, rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Bx_T1(rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0306(ArmT32OpcodePattern_16Bit::Bx_T1, rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Cmn_Register_T1(rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Cmn_Register_T1, rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Cmp_Immediate_T1(rn, imm8) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Cmp_Immediate_T1, *imm8, rn.as_operand_bits());
                vec![halfword0]
            },
            Self::Cmp_Register_T1(rn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Cmp_Register_T1, rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Cmp_Register_T2(rn, rm) => {
                let rn_as_u3 = rn.as_operand_bits() & 0b0000_0111;
                let n_as_u1 = (rn.as_operand_bits() & 0b0000_1000) >> 3;
                let rm_as_u4 = rm.as_operand_bits();
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0306__0707(ArmT32OpcodePattern_16Bit::Cmp_Register_T2, rn_as_u3, rm_as_u4, n_as_u1);
                vec![halfword0]
            },
            Self::Cps_T1(primask_effect) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0404(ArmT32OpcodePattern_16Bit::Cps_T1, primask_effect.as_operand_bits());
                vec![halfword0]
            },
            Self::Dmb_T1(option) => { 
                let word = ArmT32InstructionEncoder::encode_instruction_word__0003(ArmT32OpcodePattern_32Bit::Dmb_T1, option.as_operand_bits());
                let halfwords = split_instruction_word_into_halfwords(word);
                halfwords.to_vec()
            },
            Self::Dsb_T1(option) => { 
                let word = ArmT32InstructionEncoder::encode_instruction_word__0003(ArmT32OpcodePattern_32Bit::Dsb_T1, option.as_operand_bits());
                let halfwords = split_instruction_word_into_halfwords(word);
                halfwords.to_vec()
            },
            Self::Eor_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Eor_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Isb_T1(option) => { 
                let word = ArmT32InstructionEncoder::encode_instruction_word__0003(ArmT32OpcodePattern_32Bit::Isb_T1, option.as_operand_bits());
                let halfwords = split_instruction_word_into_halfwords(word);
                halfwords.to_vec()
            },
            Self::Ldm_T1(rn, registers) => {
                let register_list = gpr_coding_utils::convert_low_registers_slice_to_gpr_register_list_u8(registers)?;
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Ldm_T1, register_list, rn.as_operand_bits());
                vec![halfword0]
            },
            Self::Ldr_Immediate_T1(rt, rn, decoded_imm7) => {
                check_multiple_of("decoded_imm7", *decoded_imm7 as i64, 4)?;
                check_unsigned_maximum("decoded_imm7", *decoded_imm7 as u32, 124)?;

                let encoded_imm5 = decoded_imm7 / 4;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Ldr_Immediate_T1, rt.as_operand_bits(), rn.as_operand_bits(), encoded_imm5);
                vec![halfword0]
            },
            Self::Ldr_Immediate_T2(rt, decoded_imm10) => {
                check_multiple_of("decoded_imm10", *decoded_imm10 as i64, 4)?;
                check_unsigned_maximum("decoded_imm10", *decoded_imm10 as u32, 1020)?;

                let encoded_imm8 = (decoded_imm10 / 4) as u8;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Ldr_Immediate_T2, encoded_imm8, rt.as_operand_bits());
                vec![halfword0]
            },
            Self::Ldr_Literal_T1(rt, decoded_imm10) => {
                check_multiple_of("decoded_imm10", *decoded_imm10 as i64, 4)?;
                check_unsigned_maximum("decoded_imm10", *decoded_imm10 as u32, 1020)?;

                let encoded_imm8 = (decoded_imm10 / 4) as u8;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Ldr_Literal_T1, encoded_imm8, rt.as_operand_bits());
                vec![halfword0]
            },
            Self::Ldr_Register_T1(rt, rn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Ldr_Register_T1, rt.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Ldrb_Immediate_T1(rt, rn, imm5) => {
                check_unsigned_maximum("imm5", *imm5 as u32, 31)?;

                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Ldrb_Immediate_T1, rt.as_operand_bits(), rn.as_operand_bits(), *imm5);
                vec![halfword0]
            },
            Self::Ldrb_Register_T1(rt, rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Ldrb_Register_T1, rt.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Ldrh_Immediate_T1(rt, rn, decoded_imm6) => {
                check_multiple_of("decoded_imm6", *decoded_imm6 as i64, 2)?;
                check_unsigned_maximum("decoded_imm6", *decoded_imm6 as u32, 62)?;

                let encoded_imm5 = decoded_imm6 / 2;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Ldrh_Immediate_T1, rt.as_operand_bits(), rn.as_operand_bits(), encoded_imm5);
                vec![halfword0]
            },
            Self::Ldrh_Register_T1(rt, rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Ldrh_Register_T1, rt.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Ldrsb_Register_T1(rt, rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Ldrsb_Register_T1, rt.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Ldrsh_Register_T1(rt, rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Ldrsh_Register_T1, rt.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Lsl_Immediate_T1(rd, rm, imm5) => {
                check_unsigned_maximum("imm5", *imm5 as u32, 31)?;

                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Lsl_Immediate_T1, rd.as_operand_bits(), rm.as_operand_bits(), *imm5);
                vec![halfword0]
            },
            Self::Lsl_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Lsl_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Lsr_Immediate_T1(rd, rm, decoded_imm5) => {
                check_signed_range("decoded_imm5", *decoded_imm5 as i32, 1, 32)?;

                // NOTE: "A6.4.1 Shift operations" of the ARVv6-M ISA doc indicates that an imm5 value of ZERO is interpeted as an imm5 value of 32 (because shift 'type' is '01'); we should consider where we want to put _that_ logic in our code
                let encoded_imm5 = if *decoded_imm5 == 32 { 0 } else { *decoded_imm5 };
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Lsr_Immediate_T1, rd.as_operand_bits(), rm.as_operand_bits(), encoded_imm5);
                vec![halfword0]
            },
            Self::Lsr_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Lsr_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Mrs_T1(rd, spec_reg) => {
                let sysm = spec_reg.as_operand_bits();
                let word = ArmT32InstructionEncoder::encode_instruction_word__0007__080b(ArmT32OpcodePattern_32Bit::Mrs_T1, sysm, rd.as_operand_bits());
                let halfwords = split_instruction_word_into_halfwords(word);
                halfwords.to_vec()
            },
            Self::Msr_Register_T1(spec_reg, rn) => { 
                let sysm = spec_reg.as_operand_bits();
                let word = ArmT32InstructionEncoder::encode_instruction_word__0007__1013(ArmT32OpcodePattern_32Bit::Msr_Register_T1, sysm, rn.as_operand_bits());
                let halfwords = split_instruction_word_into_halfwords(word);
                halfwords.to_vec()
            },
            Self::Mov_Immediate_T1(rd, imm8) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Mov_Immediate_T1, *imm8, rd.as_operand_bits());
                vec![halfword0]
            },
            Self::Mov_Register_T1(rd, rm) => {
                let rd_as_u3 = rd.as_operand_bits() & 0b0000_0111;
                let d_as_u1 = (rd.as_operand_bits() & 0b0000_1000) >> 3;
                let rm_as_u4 = rm.as_operand_bits();
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0306__0707(ArmT32OpcodePattern_16Bit::Mov_Register_T1, rd_as_u3, rm_as_u4, d_as_u1);
                vec![halfword0]
            },
            Self::Mov_Register_T2(rd, rm) => { 
                // NOTE: we encode this function using the pattern for Lsl_Immediate_T1 (which creates the same encoding as Mov_Register_T2 by setting Lsl_Immediate_T1's imm5 value to 0b00000)
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Lsl_Immediate_T1, rd.as_operand_bits(), rm.as_operand_bits(), 0b00000);
                vec![halfword0]
            },
            Self::Mul_T1(rdm, rn) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Mul_T1, rdm.as_operand_bits(), rn.as_operand_bits());
                vec![halfword0]
            },
            Self::Mvn_Register_T1(rd, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Mvn_Register_T1, rd.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Nop_T1 => {
                let halfword0 = ArmT32OpcodePattern_16Bit::Nop_T1 as u16;
                vec![halfword0]
            },
            Self::Orr_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Orr_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Pop_T1(registers) => { 
                let (register_list, p) = gpr_coding_utils::convert_registers_slice_to_gpr_register_list_u8_and_p_u1(registers)?;
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__0808(ArmT32OpcodePattern_16Bit::Pop_T1, register_list, p);
                vec![halfword0]
            },
            Self::Push_T1(registers) => { 
                let (register_list, m) = gpr_coding_utils::convert_registers_slice_to_gpr_register_list_u8_and_m_u1(registers)?;
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__0808(ArmT32OpcodePattern_16Bit::Push_T1, register_list, m);
                vec![halfword0]
            },
            Self::Rev_T1(rd, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Rev_T1, rd.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Rev16_T1(rd, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Rev16_T1, rd.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Revsh_T1(rd, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Revsh_T1, rd.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Ror_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Ror_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Rsb_Immediate_T1(rd, rn) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Rsb_Immediate_T1, rd.as_operand_bits(), rn.as_operand_bits()/*, imm5: 0*/);
                vec![halfword0]
            },
            Self::Sbc_Register_T1(rdn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Sbc_Register_T1, rdn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Sev_T1 => { 
                let halfword0 = ArmT32OpcodePattern_16Bit::Sev_T1 as u16;
                vec![halfword0]
            },
            Self::Stm_T1(rn, registers) => { 
                let register_list = gpr_coding_utils::convert_low_registers_slice_to_gpr_register_list_u8(registers)?;
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Stm_T1, register_list, rn.as_operand_bits());
                vec![halfword0]
            },
            Self::Str_Immediate_T1(rt, rn, decoded_imm7) => {
                check_multiple_of("decoded_imm7", *decoded_imm7 as i64, 4)?;
                check_unsigned_maximum("decoded_imm7", *decoded_imm7 as u32, 124)?;

                let encoded_imm5 = decoded_imm7 / 4;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Str_Immediate_T1, rt.as_operand_bits(), rn.as_operand_bits(), encoded_imm5);
                vec![halfword0]
            },
            Self::Str_Immediate_T2(rt, decoded_imm10) => {
                check_multiple_of("decoded_imm10", *decoded_imm10 as i64, 4)?;
                check_unsigned_maximum("decoded_imm10", *decoded_imm10 as u32, 1020)?;

                let encoded_imm8 = (decoded_imm10 / 4) as u8;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Str_Immediate_T2, encoded_imm8, rt.as_operand_bits());
                vec![halfword0]
            },
            Self::Str_Register_T1(rt, rn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Str_Register_T1, rt.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Strb_Immediate_T1(rt, rn, imm5) => {
                check_unsigned_maximum("imm5", *imm5 as u32, 31)?;

                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Strb_Immediate_T1, rt.as_operand_bits(), rn.as_operand_bits(), *imm5);
                vec![halfword0]
            },
            Self::Strb_Register_T1(rt, rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Strb_Register_T1, rt.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Strh_Immediate_T1(rt, rn, decoded_imm6) => {
                check_multiple_of("decoded_imm6", *decoded_imm6 as i64, 2)?;
                check_unsigned_maximum("decoded_imm6", *decoded_imm6 as u32, 62)?;

                let encoded_imm5 = decoded_imm6 / 2;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__060a(ArmT32OpcodePattern_16Bit::Strh_Immediate_T1, rt.as_operand_bits(), rn.as_operand_bits(), encoded_imm5);
                vec![halfword0]
            },
            Self::Strh_Register_T1(rt, rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Strh_Register_T1, rt.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Sub_Immediate_T1(rd, rn, imm3) => {
                check_unsigned_maximum("imm3", *imm3 as u32, 7)?;

                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Sub_Immediate_T1, rd.as_operand_bits(), rn.as_operand_bits(), *imm3);
                vec![halfword0]
            },
            Self::Sub_Immediate_T2(rdn, imm8) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007__080a(ArmT32OpcodePattern_16Bit::Sub_Immediate_T2, *imm8, rdn.as_operand_bits());
                vec![halfword0]
            },
            Self::Sub_Register_T1(rd, rn, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305__0608(ArmT32OpcodePattern_16Bit::Sub_Register_T1, rd.as_operand_bits(), rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Sub_SpMinusImmediate_T1(const9) => {
                check_multiple_of("const9", *const9 as i64, 4)?;
                // imm7 field: const9 (= imm7 * 4) maxes at 508. (The prior code checked <= 1020, which
                // would have overflowed the 7-bit field.)
                check_unsigned_maximum("const9", *const9 as u32, 508)?;

                let imm7 = (const9 / 4) as u8;
                //
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0006(ArmT32OpcodePattern_16Bit::Sub_SpMinusImmediate_T1, imm7);
                vec![halfword0]
            },
            Self::Svc_T1(imm8) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007(ArmT32OpcodePattern_16Bit::Svc_T1, *imm8);
                vec![halfword0]
            },
            Self::Sxtb_T1(rd, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Sxtb_T1, rd.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Sxth_T1(rd, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Sxth_T1, rd.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Tst_Register_T1(rn, rm) => {
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Tst_Register_T1, rn.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Udf_T1(imm8) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0007(ArmT32OpcodePattern_16Bit::Udf_T1, *imm8);
                vec![halfword0]
            },
            Self::Udf_T2(imm16) => {
                let imm12 = imm16 & 0b0000_1111_1111_1111;
                let imm4 = ((imm16 & 0b1111_0000_0000_0000) >> 12) as u8;
                let word = ArmT32InstructionEncoder::encode_instruction_word__000b__1013(ArmT32OpcodePattern_32Bit::Udf_T2, imm12, imm4);
                let halfwords = split_instruction_word_into_halfwords(word);
                halfwords.to_vec()
            },
            Self::Uxtb_T1(rd, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Uxtb_T1, rd.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Uxth_T1(rd, rm) => { 
                let halfword0 = ArmT32InstructionEncoder::encode_instruction_halfword__0002__0305(ArmT32OpcodePattern_16Bit::Uxth_T1, rd.as_operand_bits(), rm.as_operand_bits());
                vec![halfword0]
            },
            Self::Wfe_T1 => { 
                let halfword0 = ArmT32OpcodePattern_16Bit::Wfe_T1 as u16;
                vec![halfword0]
            },
            Self::Wfi_T1 => { 
                let halfword0 = ArmT32OpcodePattern_16Bit::Wfi_T1 as u16;
                vec![halfword0]
            },
            Self::Yield_T1 => {
                let halfword0 = ArmT32OpcodePattern_16Bit::Yield_T1 as u16;
                vec![halfword0]
            },

            // ---- ARMv7-M (Thumb-2) additions ----
            Self::Mov_Immediate_T3(rd, imm16) => {
                check_general_register_is_encodable("rd", rd)?;
                let word = ArmT32OpcodePattern_32Bit::Mov_Immediate_T3 as u32 | movw_immediate_field_bits(*imm16) | ((rd.as_operand_bits() as u32) << 8);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Movt_T1(rd, imm16) => {
                check_general_register_is_encodable("rd", rd)?;
                let word = ArmT32OpcodePattern_32Bit::Movt_T1 as u32 | movw_immediate_field_bits(*imm16) | ((rd.as_operand_bits() as u32) << 8);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Mul_T2(rd, rn, rm) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_general_register_is_encodable("rm", rm)?;
                let word = ArmT32OpcodePattern_32Bit::Mul_T2 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Mla_T1(rd, rn, rm, ra) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_general_register_is_encodable("rm", rm)?;
                check_general_register_is_encodable("ra", ra)?;
                let word = ArmT32OpcodePattern_32Bit::Mla_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((ra.as_operand_bits() as u32) << 12) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Mls_T1(rd, rn, rm, ra) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_general_register_is_encodable("rm", rm)?;
                check_general_register_is_encodable("ra", ra)?;
                let word = ArmT32OpcodePattern_32Bit::Mls_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((ra.as_operand_bits() as u32) << 12) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Sdiv_T1(rd, rn, rm) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_general_register_is_encodable("rm", rm)?;
                let word = ArmT32OpcodePattern_32Bit::Sdiv_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Udiv_T1(rd, rn, rm) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_general_register_is_encodable("rm", rm)?;
                let word = ArmT32OpcodePattern_32Bit::Udiv_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Clz_T1(rd, rm) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rm", rm)?;
                // Rm is encoded in BOTH bits 19:16 and 3:0
                let word = ArmT32OpcodePattern_32Bit::Clz_T1 as u32 | ((rm.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv7-M batch M7b ----
            Self::Rbit_T1(rd, rm) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rm", rm)?;
                // Rm is encoded in BOTH bits 19:16 and 3:0 (like CLZ)
                let word = ArmT32OpcodePattern_32Bit::Rbit_T1 as u32 | ((rm.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Ubfx_T1(rd, rn, lsb, width) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_bitfield_lsb_width(*lsb, *width)?;
                let word = ArmT32OpcodePattern_32Bit::Ubfx_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | bitfield_lsb_field_bits(*lsb) | ((*width - 1) as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Sbfx_T1(rd, rn, lsb, width) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_bitfield_lsb_width(*lsb, *width)?;
                let word = ArmT32OpcodePattern_32Bit::Sbfx_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | bitfield_lsb_field_bits(*lsb) | ((*width - 1) as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Bfi_T1(rd, rn, lsb, width) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_bitfield_lsb_width(*lsb, *width)?;
                let msb = (*lsb + *width - 1) as u32; // BFI stores msb (= lsb + width - 1), not widthm1
                let word = ArmT32OpcodePattern_32Bit::Bfi_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | bitfield_lsb_field_bits(*lsb) | msb;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Bfc_T1(rd, lsb, width) => {
                check_general_register_is_encodable("rd", rd)?;
                check_bitfield_lsb_width(*lsb, *width)?;
                let msb = (*lsb + *width - 1) as u32;
                // BFC is BFI with Rn == 0b1111
                let word = ArmT32OpcodePattern_32Bit::Bfi_T1 as u32 | (0b1111 << 16) | ((rd.as_operand_bits() as u32) << 8) | bitfield_lsb_field_bits(*lsb) | msb;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Ldr_Immediate_T3(rt, rn, imm12) => {
                check_register_is_not_pc("rn", rn)?;
                check_unsigned_maximum("imm12", *imm12 as u32, 4095)?;
                let word = ArmT32OpcodePattern_32Bit::Ldr_Immediate_T3 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (*imm12 as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Str_Immediate_T3(rt, rn, imm12) => {
                check_register_is_not_pc("rn", rn)?;
                check_general_register_is_encodable("rt", rt)?;
                check_unsigned_maximum("imm12", *imm12 as u32, 4095)?;
                let word = ArmT32OpcodePattern_32Bit::Str_Immediate_T3 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (*imm12 as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv8-M load-acquire / store-release ----
            Self::LoadAcquire_T1(size, exclusive, rt, rn) => {
                let word = 0xE8D0_0F8F | ((*size as u32) << 4) | ((*exclusive as u32) << 6)
                    | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::StoreRelease_T1(size, rt, rn) => {
                let word = 0xE8C0_0F8F | ((*size as u32) << 4)
                    | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::StoreReleaseExclusive_T1(size, rd, rt, rn) => {
                let word = 0xE8C0_0FC0 | ((*size as u32) << 4)
                    | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (rd.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::UnprivLoadStore_T1(load, signed, size, rt, rn, imm8) => {
                let word = 0xF800_0E00 | ((*signed as u32) << 24) | ((*size as u32) << 21) | ((*load as u32) << 20)
                    | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (*imm8 as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv7-M batch M7c ----
            Self::Ldrex_T1(rt, rn, imm) => {
                check_general_register_is_encodable("rt", rt)?;
                check_register_is_not_pc("rn", rn)?;
                let imm8 = exclusive_word_offset_field("imm", *imm)?;
                let word = ArmT32OpcodePattern_32Bit::Ldrex_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | imm8;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Strex_T1(rd, rt, rn, imm) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rt", rt)?;
                check_register_is_not_pc("rn", rn)?;
                let imm8 = exclusive_word_offset_field("imm", *imm)?;
                let word = ArmT32OpcodePattern_32Bit::Strex_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | ((rd.as_operand_bits() as u32) << 8) | imm8;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Ldrexb_T1(rt, rn) => {
                check_general_register_is_encodable("rt", rt)?;
                check_register_is_not_pc("rn", rn)?;
                let word = ArmT32OpcodePattern_32Bit::Ldrexb_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Strexb_T1(rd, rt, rn) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rt", rt)?;
                check_register_is_not_pc("rn", rn)?;
                let word = ArmT32OpcodePattern_32Bit::Strexb_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (rd.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Ldrexh_T1(rt, rn) => {
                check_general_register_is_encodable("rt", rt)?;
                check_register_is_not_pc("rn", rn)?;
                let word = ArmT32OpcodePattern_32Bit::Ldrexh_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Strexh_T1(rd, rt, rn) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rt", rt)?;
                check_register_is_not_pc("rn", rn)?;
                let word = ArmT32OpcodePattern_32Bit::Strexh_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (rd.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Clrex_T1 => {
                // the option field (bits 3:0) is SBO (1111)
                split_instruction_word_into_halfwords(ArmT32OpcodePattern_32Bit::Clrex_T1 as u32 | 0b1111).to_vec()
            },
            Self::Tbb_T1(rn, rm) => {
                check_general_register_is_encodable("rm", rm)?;
                let word = ArmT32OpcodePattern_32Bit::Tbb_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Tbh_T1(rn, rm) => {
                check_general_register_is_encodable("rm", rm)?;
                let word = ArmT32OpcodePattern_32Bit::Tbh_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv7-M batch M7d: data processing (modified immediate) ----
            Self::Mov_Immediate_T2(rd, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                let word = encode_data_processing_modified_immediate(0b0010, *set_flags, 0b1111, rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Mvn_Immediate_T1(rd, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                let word = encode_data_processing_modified_immediate(0b0011, *set_flags, 0b1111, rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::And_Immediate_T1(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b0000, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Bic_Immediate_T1(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b0001, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Orr_Immediate_T1(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b0010, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Eor_Immediate_T1(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b0100, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Add_Immediate_T3(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b1000, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Sub_Immediate_T3(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b1101, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Tst_Immediate_T1(rn, constant) => {
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b0000, true, rn.as_operand_bits(), 0b1111, *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Teq_Immediate_T1(rn, constant) => {
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b0100, true, rn.as_operand_bits(), 0b1111, *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Cmn_Immediate_T1(rn, constant) => {
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b1000, true, rn.as_operand_bits(), 0b1111, *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Cmp_Immediate_T2(rn, constant) => {
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b1101, true, rn.as_operand_bits(), 0b1111, *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv7-M batch M7e ----
            Self::Adc_Immediate_T1(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b1010, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Sbc_Immediate_T1(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b1011, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Rsb_Immediate_T2(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b1110, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Orn_Immediate_T1(rd, rn, constant, set_flags) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                let word = encode_data_processing_modified_immediate(0b0011, *set_flags, rn.as_operand_bits(), rd.as_operand_bits(), *constant)?;
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv7-M batch M7f: data processing (shifted register) ----
            Self::Add_Register_T3(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b1000, *set_flags, rd, rn, rm, shift)?,
            Self::Sub_Register_T2(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b1101, *set_flags, rd, rn, rm, shift)?,
            Self::And_Register_T2(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b0000, *set_flags, rd, rn, rm, shift)?,
            Self::Orr_Register_T2(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b0010, *set_flags, rd, rn, rm, shift)?,
            Self::Eor_Register_T2(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b0100, *set_flags, rd, rn, rm, shift)?,
            Self::Bic_Register_T2(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b0001, *set_flags, rd, rn, rm, shift)?,

            // ---- ARMv7-M batch M7g: shifted-register alias forms ----
            Self::Mov_Register_T3(rd, rm, shift, set_flags) => encode_mov_mvn_register(0b0010, *set_flags, rd, rm, shift)?,
            Self::Mvn_Register_T2(rd, rm, shift, set_flags) => encode_mov_mvn_register(0b0011, *set_flags, rd, rm, shift)?,
            Self::Adc_Register_T2(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b1010, *set_flags, rd, rn, rm, shift)?,
            Self::Sbc_Register_T2(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b1011, *set_flags, rd, rn, rm, shift)?,
            Self::Rsb_Register_T1(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b1110, *set_flags, rd, rn, rm, shift)?,
            Self::Orn_Register_T1(rd, rn, rm, shift, set_flags) => encode_dp_shifted_register(0b0011, *set_flags, rd, rn, rm, shift)?,
            Self::Tst_Register_T2(rn, rm, shift) => encode_compare_register(0b0000, rn, rm, shift)?,
            Self::Teq_Register_T1(rn, rm, shift) => encode_compare_register(0b0100, rn, rm, shift)?,
            Self::Cmn_Register_T2(rn, rm, shift) => encode_compare_register(0b1000, rn, rm, shift)?,
            Self::Cmp_Register_T3(rn, rm, shift) => encode_compare_register(0b1101, rn, rm, shift)?,

            // ---- ARMv7-M batch M7h: wide byte/half load/store (imm12) + register offset ----
            Self::Ldrb_Immediate_T2(rt, rn, imm12) => encode_load_store_immediate12(0xF890_0000, rt, rn, *imm12)?,
            Self::Strb_Immediate_T2(rt, rn, imm12) => encode_load_store_immediate12(0xF880_0000, rt, rn, *imm12)?,
            Self::Ldrh_Immediate_T2(rt, rn, imm12) => encode_load_store_immediate12(0xF8B0_0000, rt, rn, *imm12)?,
            Self::Strh_Immediate_T2(rt, rn, imm12) => encode_load_store_immediate12(0xF8A0_0000, rt, rn, *imm12)?,
            Self::Ldrsb_Immediate_T1(rt, rn, imm12) => encode_load_store_immediate12(0xF990_0000, rt, rn, *imm12)?,
            Self::Ldrsh_Immediate_T1(rt, rn, imm12) => encode_load_store_immediate12(0xF9B0_0000, rt, rn, *imm12)?,
            Self::Ldr_Register_T2(rt, rn, rm, lsl) => encode_load_store_register(0xF850_0000, rt, rn, rm, *lsl)?,
            Self::Str_Register_T2(rt, rn, rm, lsl) => encode_load_store_register(0xF840_0000, rt, rn, rm, *lsl)?,
            Self::Ldrb_Register_T2(rt, rn, rm, lsl) => encode_load_store_register(0xF810_0000, rt, rn, rm, *lsl)?,
            Self::Strb_Register_T2(rt, rn, rm, lsl) => encode_load_store_register(0xF800_0000, rt, rn, rm, *lsl)?,
            Self::Ldrh_Register_T2(rt, rn, rm, lsl) => encode_load_store_register(0xF830_0000, rt, rn, rm, *lsl)?,
            Self::Strh_Register_T2(rt, rn, rm, lsl) => encode_load_store_register(0xF820_0000, rt, rn, rm, *lsl)?,
            Self::Ldrsb_Register_T2(rt, rn, rm, lsl) => encode_load_store_register(0xF910_0000, rt, rn, rm, *lsl)?,
            Self::Ldrsh_Register_T2(rt, rn, rm, lsl) => encode_load_store_register(0xF930_0000, rt, rn, rm, *lsl)?,

            // ---- ARMv7-M batch M7k: long multiply ----
            Self::Smull_T1(rdlo, rdhi, rn, rm) => encode_long_multiply(0xFB80_0000, rdlo, rdhi, rn, rm)?,
            Self::Umull_T1(rdlo, rdhi, rn, rm) => encode_long_multiply(0xFBA0_0000, rdlo, rdhi, rn, rm)?,
            Self::Smlal_T1(rdlo, rdhi, rn, rm) => encode_long_multiply(0xFBC0_0000, rdlo, rdhi, rn, rm)?,
            Self::Umlal_T1(rdlo, rdhi, rn, rm) => encode_long_multiply(0xFBE0_0000, rdlo, rdhi, rn, rm)?,
            Self::Umaal_T1(rdlo, rdhi, rn, rm) => encode_long_multiply(0xFBE0_0060, rdlo, rdhi, rn, rm)?,

            // ---- ARMv7-M batch M7l: wide extend (with ROR), wide byte-reverse, saturate ----
            Self::Sxtb_T2(rd, rm, rotation) => encode_extend(0xFA4F_F080, rd, rm, *rotation)?,
            Self::Uxtb_T2(rd, rm, rotation) => encode_extend(0xFA5F_F080, rd, rm, *rotation)?,
            Self::Sxth_T2(rd, rm, rotation) => encode_extend(0xFA0F_F080, rd, rm, *rotation)?,
            Self::Uxth_T2(rd, rm, rotation) => encode_extend(0xFA1F_F080, rd, rm, *rotation)?,
            Self::Rev_T2(rd, rm) => encode_byte_reverse(0xFA90_F080, rd, rm)?,
            Self::Rev16_T2(rd, rm) => encode_byte_reverse(0xFA90_F090, rd, rm)?,
            Self::Revsh_T2(rd, rm) => encode_byte_reverse(0xFA90_F0B0, rd, rm)?,
            Self::Ssat_T1(rd, sat_imm, rn, shift) => encode_saturate(0xF300_0000, false, rd, *sat_imm, rn, *shift)?,
            Self::Usat_T1(rd, sat_imm, rn, shift) => encode_saturate(0xF380_0000, true, rd, *sat_imm, rn, *shift)?,

            // ---- ARMv7-M batch M7i: indexed load/store, LDRD/STRD, literal loads, preload ----
            Self::Ldr_Immediate_T4(rt, rn, offset, mode) => encode_load_store_indexed(0xF850_0800, rt, rn, *offset, *mode)?,
            Self::Str_Immediate_T4(rt, rn, offset, mode) => encode_load_store_indexed(0xF840_0800, rt, rn, *offset, *mode)?,
            Self::Ldrb_Immediate_T3(rt, rn, offset, mode) => encode_load_store_indexed(0xF810_0800, rt, rn, *offset, *mode)?,
            Self::Strb_Immediate_T3(rt, rn, offset, mode) => encode_load_store_indexed(0xF800_0800, rt, rn, *offset, *mode)?,
            Self::Ldrh_Immediate_T3(rt, rn, offset, mode) => encode_load_store_indexed(0xF830_0800, rt, rn, *offset, *mode)?,
            Self::Strh_Immediate_T3(rt, rn, offset, mode) => encode_load_store_indexed(0xF820_0800, rt, rn, *offset, *mode)?,
            Self::Ldrsb_Immediate_T2(rt, rn, offset, mode) => encode_load_store_indexed(0xF910_0800, rt, rn, *offset, *mode)?,
            Self::Ldrsh_Immediate_T2(rt, rn, offset, mode) => encode_load_store_indexed(0xF930_0800, rt, rn, *offset, *mode)?,
            Self::Ldrd_Immediate_T1(rt, rt2, rn, offset, mode) => encode_load_store_dual(true, rt, rt2, rn, *offset, *mode)?,
            Self::Strd_Immediate_T1(rt, rt2, rn, offset, mode) => encode_load_store_dual(false, rt, rt2, rn, *offset, *mode)?,
            Self::Ldr_Literal_T2(rt, offset) => encode_load_literal(0xF85F_0000, rt, *offset)?,
            Self::Ldrb_Literal_T1(rt, offset) => encode_load_literal(0xF81F_0000, rt, *offset)?,
            Self::Ldrh_Literal_T1(rt, offset) => encode_load_literal(0xF83F_0000, rt, *offset)?,
            Self::Ldrsb_Literal_T1(rt, offset) => encode_load_literal(0xF91F_0000, rt, *offset)?,
            Self::Ldrsh_Literal_T1(rt, offset) => encode_load_literal(0xF93F_0000, rt, *offset)?,
            Self::Pld_Immediate_T1(rn, offset) => encode_preload(0xF890_F000, 0xF810_FC00, rn, *offset)?,
            Self::Pli_Immediate_T1(rn, offset) => encode_preload(0xF990_F000, 0xF910_FC00, rn, *offset)?,

            // ---- ARMv7-M batch M7j: wide load/store multiple ----
            Self::Ldmia_T2(rn, writeback, registers) => encode_load_store_multiple(0xE890_0000, true, rn, *writeback, registers)?,
            Self::Stmia_T2(rn, writeback, registers) => encode_load_store_multiple(0xE880_0000, false, rn, *writeback, registers)?,
            Self::Ldmdb_T1(rn, writeback, registers) => encode_load_store_multiple(0xE910_0000, true, rn, *writeback, registers)?,
            Self::Stmdb_T1(rn, writeback, registers) => encode_load_store_multiple(0xE900_0000, false, rn, *writeback, registers)?,

            // ---- ARMv7-M batch M7m: wide branches + compare-and-branch ----
            Self::B_T4(offset) => encode_branch_wide_unconditional(*offset)?,
            Self::B_T3(cond, offset) => encode_branch_wide_conditional(cond, *offset)?,
            Self::Cbz_T1(rn, offset) => encode_compare_branch(0xB100, rn, *offset)?,
            Self::Cbnz_T1(rn, offset) => encode_compare_branch(0xB900, rn, *offset)?,

            // ---- ARMv7-M batch M7n: IT ----
            Self::It_T1(firstcond, mask) => {
                let cond_bits = firstcond.as_operand_bits();
                if cond_bits >= 14 {
                    return Err(EncodeError::ConditionNotEncodable { field: "firstcond", detail: "IT does not support the AL/NV conditions" });
                }
                if *mask == 0 || *mask > 0b1111 {
                    return Err(EncodeError::ImmediateOutOfRange { field: "mask", value: *mask as i64, minimum: 1, maximum: 15 });
                }
                vec![0xBF00u16 | ((cond_bits as u16) << 4) | (*mask as u16)]
            },

            // ---- ARMv7E-M DSP M8a: saturating arithmetic ----
            Self::Qadd_T1(rd, rm, rn) => encode_saturating_arithmetic(0xFA80_F080, rd, rm, rn)?,
            Self::Qsub_T1(rd, rm, rn) => encode_saturating_arithmetic(0xFA80_F0A0, rd, rm, rn)?,
            Self::Qdadd_T1(rd, rm, rn) => encode_saturating_arithmetic(0xFA80_F090, rd, rm, rn)?,
            Self::Qdsub_T1(rd, rm, rn) => encode_saturating_arithmetic(0xFA80_F0B0, rd, rm, rn)?,

            // ---- ARMv7E-M DSP M8b: extend-and-add + 16-bit extends ----
            Self::Sxtab_T1(rd, rn, rm, rotation) => encode_extend_and_add(0xFA40_F080, rd, rn, rm, *rotation)?,
            Self::Uxtab_T1(rd, rn, rm, rotation) => encode_extend_and_add(0xFA50_F080, rd, rn, rm, *rotation)?,
            Self::Sxtah_T1(rd, rn, rm, rotation) => encode_extend_and_add(0xFA00_F080, rd, rn, rm, *rotation)?,
            Self::Uxtah_T1(rd, rn, rm, rotation) => encode_extend_and_add(0xFA10_F080, rd, rn, rm, *rotation)?,
            Self::Sxtab16_T1(rd, rn, rm, rotation) => encode_extend_and_add(0xFA20_F080, rd, rn, rm, *rotation)?,
            Self::Uxtab16_T1(rd, rn, rm, rotation) => encode_extend_and_add(0xFA30_F080, rd, rn, rm, *rotation)?,
            Self::Sxtb16_T1(rd, rm, rotation) => encode_extend(0xFA2F_F080, rd, rm, *rotation)?,
            Self::Uxtb16_T1(rd, rm, rotation) => encode_extend(0xFA3F_F080, rd, rm, *rotation)?,

            // ---- ARMv7E-M DSP M8c: pack / saturate16 / select / SAD ----
            Self::Pkhbt_T1(rd, rn, rm, lsl) => encode_pack_halfword(rd, rn, rm, *lsl, false)?,
            Self::Pkhtb_T1(rd, rn, rm, asr) => encode_pack_halfword(rd, rn, rm, *asr, true)?,
            Self::Ssat16_T1(rd, sat_imm, rn) => encode_saturate16(0xF320_0000, false, rd, *sat_imm, rn)?,
            Self::Usat16_T1(rd, sat_imm, rn) => encode_saturate16(0xF3A0_0000, true, rd, *sat_imm, rn)?,
            Self::Sel_T1(rd, rn, rm) => encode_saturating_arithmetic(0xFAA0_F080, rd, rm, rn)?,
            Self::Usad8_T1(rd, rn, rm) => encode_saturating_arithmetic(0xFB70_F000, rd, rm, rn)?,
            Self::Usada8_T1(rd, rn, rm, ra) => encode_usada8(rd, rn, rm, ra)?,

            // ---- ARMv7E-M DSP M8d: parallel add/subtract ----
            Self::ParallelAddSub_T1(operation, prefix, rd, rn, rm) => {
                check_general_register_is_encodable("rd", rd)?;
                check_general_register_is_encodable("rn", rn)?;
                check_general_register_is_encodable("rm", rm)?;
                let word = operation.base() | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (prefix.bits() << 4) | (rm.as_operand_bits() as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv7E-M DSP M8e: signed multiplies (nibble = the op2/[7:4] field) ----
            Self::Smul_T1(rd, rn, rm, n, m) => encode_signed_multiply(0xFB10_F000, rd, rn, rm, nm_nibble(*n, *m))?,
            Self::Smulw_T1(rd, rn, rm, m) => encode_signed_multiply(0xFB30_F000, rd, rn, rm, *m as u32)?,
            Self::Smla_T1(rd, rn, rm, ra, n, m) => encode_signed_multiply_accumulate(0xFB10_0000, rd, rn, rm, ra, nm_nibble(*n, *m))?,
            Self::Smlaw_T1(rd, rn, rm, ra, m) => encode_signed_multiply_accumulate(0xFB30_0000, rd, rn, rm, ra, *m as u32)?,
            Self::Smlal_Halfword_T1(rdlo, rdhi, rn, rm, n, m) => encode_signed_multiply_long(0xFBC0_0000, rdlo, rdhi, rn, rm, 0b1000 | nm_nibble(*n, *m))?,
            Self::Smuad_T1(rd, rn, rm, x) => encode_signed_multiply(0xFB20_F000, rd, rn, rm, *x as u32)?,
            Self::Smusd_T1(rd, rn, rm, x) => encode_signed_multiply(0xFB40_F000, rd, rn, rm, *x as u32)?,
            Self::Smlad_T1(rd, rn, rm, ra, x) => encode_signed_multiply_accumulate(0xFB20_0000, rd, rn, rm, ra, *x as u32)?,
            Self::Smlsd_T1(rd, rn, rm, ra, x) => encode_signed_multiply_accumulate(0xFB40_0000, rd, rn, rm, ra, *x as u32)?,
            Self::Smlald_T1(rdlo, rdhi, rn, rm, x) => encode_signed_multiply_long(0xFBC0_0000, rdlo, rdhi, rn, rm, 0b1100 | (*x as u32))?,
            Self::Smlsld_T1(rdlo, rdhi, rn, rm, x) => encode_signed_multiply_long(0xFBD0_0000, rdlo, rdhi, rn, rm, 0b1100 | (*x as u32))?,
            Self::Smmul_T1(rd, rn, rm, round) => encode_signed_multiply(0xFB50_F000, rd, rn, rm, *round as u32)?,
            Self::Smmla_T1(rd, rn, rm, ra, round) => encode_signed_multiply_accumulate(0xFB50_0000, rd, rn, rm, ra, *round as u32)?,
            Self::Smmls_T1(rd, rn, rm, ra, round) => encode_signed_multiply_accumulate(0xFB60_0000, rd, rn, rm, ra, *round as u32)?,

            // ---- ARMv7E-M FP M8f: load/store ----
            Self::Vldr_Single_T2(sd, rn, offset) => encode_fp_load_store(0xED10_0A00, sd.field(), sd.extra_bit(), rn, *offset)?,
            Self::Vstr_Single_T2(sd, rn, offset) => encode_fp_load_store(0xED00_0A00, sd.field(), sd.extra_bit(), rn, *offset)?,
            Self::Vldr_Double_T1(dd, rn, offset) => encode_fp_load_store(0xED10_0B00, dd.field(), dd.extra_bit(), rn, *offset)?,
            Self::Vstr_Double_T1(dd, rn, offset) => encode_fp_load_store(0xED00_0B00, dd.field(), dd.extra_bit(), rn, *offset)?,

            // ---- ARMv7E-M FP M8g: load/store multiple (single imm8=count, double imm8=2*count) ----
            Self::Vldm_Single_T2(rn, wb, db, first, count) => encode_fp_load_store_multiple(0x0A00, true, rn, *wb, *db, first.field(), first.extra_bit(), *count, first.number(), 31, false)?,
            Self::Vstm_Single_T2(rn, wb, db, first, count) => encode_fp_load_store_multiple(0x0A00, false, rn, *wb, *db, first.field(), first.extra_bit(), *count, first.number(), 31, false)?,
            Self::Vldm_Double_T1(rn, wb, db, first, count) => encode_fp_load_store_multiple(0x0B00, true, rn, *wb, *db, first.field(), first.extra_bit(), *count, first.number(), 15, true)?,
            Self::Vstm_Double_T1(rn, wb, db, first, count) => encode_fp_load_store_multiple(0x0B00, false, rn, *wb, *db, first.field(), first.extra_bit(), *count, first.number(), 15, true)?,
            Self::FldmdbxFstmdbx_T1(load, rn, first, count) => {
                let base = if *load { 0xED30_0B00u32 } else { 0xED20_0B00 };
                let word = base | ((rn.as_operand_bits() as u32) << 16) | (first.extra_bit() << 22) | (first.field() << 12) | (2 * (*count as u32) + 1);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Coproc_Mcr_T1(two, load, cp, opc1, rt, crn, crm, opc2) => split_instruction_word_into_halfwords(
                (if *two { 0xFE00_0010u32 } else { 0xEE00_0010 }) | ((*load as u32) << 20) | ((*opc1 as u32) << 21)
                    | ((*crn as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | ((*cp as u32) << 8) | ((*opc2 as u32) << 5) | (*crm as u32)
            ).to_vec(),
            Self::Coproc_Cdp_T1(two, cp, opc1, crd, crn, crm, opc2) => split_instruction_word_into_halfwords(
                (if *two { 0xFE00_0000u32 } else { 0xEE00_0000 }) | ((*opc1 as u32) << 20) | ((*crn as u32) << 16)
                    | ((*crd as u32) << 12) | ((*cp as u32) << 8) | ((*opc2 as u32) << 5) | (*crm as u32)
            ).to_vec(),
            Self::Coproc_Mcrr_T1(two, load, cp, opc1, rt, rt2, crm) => split_instruction_word_into_halfwords(
                (if *two { 0xFC40_0000u32 } else { 0xEC40_0000 }) | ((*load as u32) << 20) | ((rt2.as_operand_bits() as u32) << 16)
                    | ((rt.as_operand_bits() as u32) << 12) | ((*cp as u32) << 8) | ((*opc1 as u32) << 4) | (*crm as u32)
            ).to_vec(),
            Self::Coproc_Ldc_T1(two, long, load, cp, crd, rn, offset) => {
                let u = if *offset >= 0 { 1u32 } else { 0 };
                let imm8 = (offset.unsigned_abs() / 4) & 0xFF;
                split_instruction_word_into_halfwords(
                    (if *two { 0xFC00_0000u32 } else { 0xEC00_0000 }) | (1 << 24) | (u << 23) | ((*long as u32) << 22) | ((*load as u32) << 20)
                        | ((rn.as_operand_bits() as u32) << 16) | ((*crd as u32) << 12) | ((*cp as u32) << 8) | imm8
                ).to_vec()
            },
            Self::PacbtiHint_T1(kind) => split_instruction_word_into_halfwords(
                match kind { 0 => 0xF3AF_800Fu32, 1 => 0xF3AF_801D, 2 => 0xF3AF_802D, _ => 0xF3AF_800D }
            ).to_vec(),
            Self::PacbtiData_T1(op, rd, rn, rm) => {
                let (rd_v, rm_v, rn_v) = (rd.as_operand_bits() as u32, rm.as_operand_bits() as u32, (rn.as_operand_bits() as u32) << 16);
                split_instruction_word_into_halfwords(match op {
                    0 => 0xFB60_F000 | rn_v | (rd_v << 8) | rm_v,   // pacg  ([15:12]=1111, Rd[11:8])
                    1 => 0xFB50_0F00 | rn_v | (rd_v << 12) | rm_v,  // autg  ([11:8]=1111, Rd[15:12])
                    _ => 0xFB50_0F10 | rn_v | (rd_v << 12) | rm_v,  // bxaut ([11:8]=1111, [7:4]=0001, Rd[15:12])
                }).to_vec()
            },
            Self::Vscclrm_T1(double, first, count) => {
                let (vd, d) = if *double { ((*first & 0xF) as u32, (*first >> 4) as u32) } else { ((*first >> 1) as u32, (*first & 1) as u32) };
                let base = if *double { 0xEC9F_0B00u32 } else { 0xEC9F_0A00 };
                split_instruction_word_into_halfwords(base | (d << 22) | (vd << 12) | (*count as u32)).to_vec()
            },
            Self::Cde_Cx1_T1(acc, dual, coproc, rd, imm) => split_instruction_word_into_halfwords(
                0xEE00_0000 | ((*acc as u32) << 28) | ((*dual as u32) << 6) | ((rd.as_operand_bits() as u32) << 12) | ((*coproc as u32) << 8)
                    | ((*imm as u32) & 0x3F) | ((((*imm as u32) >> 6) & 1) << 7) | ((((*imm as u32) >> 7) & 0x3F) << 16)
            ).to_vec(),
            Self::Cde_Cx2_T1(acc, dual, coproc, rd, rn, imm) => split_instruction_word_into_halfwords(
                0xEE40_0000 | ((*acc as u32) << 28) | ((*dual as u32) << 6) | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 12)
                    | ((*coproc as u32) << 8) | ((*imm as u32) & 0x3F) | ((((*imm as u32) >> 6) & 1) << 7) | ((((*imm as u32) >> 7) & 0x3) << 20)
            ).to_vec(),
            Self::Cde_Cx3_T1(acc, dual, coproc, rd, rn, rm, imm) => split_instruction_word_into_halfwords(
                0xEE80_0000 | ((*acc as u32) << 28) | ((*dual as u32) << 6) | ((rn.as_operand_bits() as u32) << 16) | ((rm.as_operand_bits() as u32) << 12)
                    | ((*coproc as u32) << 8) | (rd.as_operand_bits() as u32) | (((*imm as u32) & 0x3) << 4) | ((((*imm as u32) >> 2) & 1) << 7) | ((((*imm as u32) >> 3) & 0x7) << 20)
            ).to_vec(),

            // ---- Branch Future. Offset split: immA=offset[16:12]->hw1[4:0], immB=offset[11:2]->hw2[10:1],
            // immC=offset[1]->hw2[11]; boff->hw1[10:7]. BF base 0xF040_E001 ([6:5]=10), BFL 0xF000_C001
            // (hw2=1100), BFX 0xF060_E001 ([6:4]=110)+Rn, BFLX 0xF070_E001 ([6:4]=111)+Rn, BFCSEL
            // 0xF000_E001 ([6]=0) + cond[5:2] + T[1] + immA[0]. ----
            Self::Bf_T1(boff, offset) => split_instruction_word_into_halfwords(
                0xF040_E001 | ((*boff as u32) << 23) | (((*offset as u32 >> 12) & 0x1F) << 16)
                    | (((*offset as u32 >> 1) & 0x1) << 11) | (((*offset as u32 >> 2) & 0x3FF) << 1)
            ).to_vec(),
            Self::Bfl_T4(boff, offset) => split_instruction_word_into_halfwords(
                0xF000_C001 | ((*boff as u32) << 23) | (((*offset as u32 >> 12) & 0x1F) << 16)
                    | (((*offset as u32 >> 1) & 0x1) << 11) | (((*offset as u32 >> 2) & 0x3FF) << 1)
            ).to_vec(),
            Self::Bfx_T3(boff, rn) => split_instruction_word_into_halfwords(
                0xF060_E001 | ((*boff as u32) << 23) | ((rn.as_operand_bits() as u32) << 16)
            ).to_vec(),
            Self::Bflx_T5(boff, rn) => split_instruction_word_into_halfwords(
                0xF070_E001 | ((*boff as u32) << 23) | ((rn.as_operand_bits() as u32) << 16)
            ).to_vec(),
            Self::Bfcsel_T2(boff, offset, cond, t) => split_instruction_word_into_halfwords(
                0xF000_E001 | ((*boff as u32) << 23) | ((*cond as u32) << 18) | ((*t as u32) << 17)
                    | (((*offset as u32 >> 12) & 0x1) << 16) | (((*offset as u32 >> 1) & 0x1) << 11)
                    | (((*offset as u32 >> 2) & 0x3FF) << 1)
            ).to_vec(),

            // ---- VCX1/VCX2/VCX3 (CDE FP/vector). Base 0xEC20/0xEC30/0xEC80_0000 | acc(28) | sz(24, kind==1)
            // | vector(6, kind==2) | coproc(<<8) | the register/immediate scatters (see the vcx_* helpers). ----
            Self::Vcx1_T1(acc, kind, coproc, rd, imm) => {
                let extra = ((*acc as u32) << 28) | (((*kind == 1) as u32) << 24) | (((*kind == 2) as u32) << 6) | ((*coproc as u32) << 8);
                split_instruction_word_into_halfwords(0xEC20_0000 | extra | vcx_enc_d(*kind, *rd) | vcx_enc_imm1(*imm)).to_vec()
            },
            Self::Vcx2_T1(acc, kind, coproc, rd, rn, imm) => {
                let extra = ((*acc as u32) << 28) | (((*kind == 1) as u32) << 24) | (((*kind == 2) as u32) << 6) | ((*coproc as u32) << 8);
                split_instruction_word_into_halfwords(0xEC30_0000 | extra | vcx_enc_d(*kind, *rd) | vcx_enc_lo(*kind, *rn) | vcx_enc_imm2(*imm)).to_vec()
            },
            Self::Vcx3_T1(acc, kind, coproc, rd, rn, rm, imm) => {
                let extra = ((*acc as u32) << 28) | (((*kind == 1) as u32) << 24) | (((*kind == 2) as u32) << 6) | ((*coproc as u32) << 8);
                split_instruction_word_into_halfwords(0xEC80_0000 | extra | vcx_enc_d(*kind, *rd) | vcx_enc_hi(*kind, *rn) | vcx_enc_lo(*kind, *rm) | vcx_enc_imm3(*imm)).to_vec()
            },

            // ---- ARMv7E-M FP M8h: data-processing ----
            Self::FpDataProcess3_Single(op, sd, sn, sm) => {
                let word = 0xEE00_0A00 | op.opcode_bits() | (sd.extra_bit() << 22) | (sn.field() << 16) | (sd.field() << 12) | (sn.extra_bit() << 7) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::FpDataProcess3_Double(op, dd, dn, dm) => {
                let word = 0xEE00_0B00 | op.opcode_bits() | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | (dn.extra_bit() << 7) | (dm.extra_bit() << 5) | dm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::FpDataProcess2_Single(op, sd, sm) => {
                let word = op.base() | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::FpDataProcess2_Double(op, dd, dm) => {
                let word = op.base() | (1 << 8) | (dd.extra_bit() << 22) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv7E-M FP M8i: compare / FPSCR transfer / core<->FP move ----
            Self::Vcmp_Single_T1(sd, sm, e) => {
                let word = 0xEEB4_0A40 | (sd.extra_bit() << 22) | (sd.field() << 12) | ((*e as u32) << 7) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcmp_Double_T1(dd, dm, e) => {
                let word = 0xEEB4_0B40 | (dd.extra_bit() << 22) | (dd.field() << 12) | ((*e as u32) << 7) | (dm.extra_bit() << 5) | dm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcmp_Zero_Single_T2(sd, e) => {
                let word = 0xEEB5_0A40 | (sd.extra_bit() << 22) | (sd.field() << 12) | ((*e as u32) << 7);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcmp_Zero_Double_T2(dd, e) => {
                let word = 0xEEB5_0B40 | (dd.extra_bit() << 22) | (dd.field() << 12) | ((*e as u32) << 7);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vmrs_T1(rt) => {
                check_general_register_is_encodable("rt", rt)?;
                split_instruction_word_into_halfwords(0xEEF1_0A10 | ((rt.as_operand_bits() as u32) << 12)).to_vec()
            },
            Self::Vmrs_Apsr_Nzcv_T1 => split_instruction_word_into_halfwords(0xEEF1_FA10).to_vec(),
            Self::Vmsr_T1(rt) => {
                check_general_register_is_encodable("rt", rt)?;
                split_instruction_word_into_halfwords(0xEEE1_0A10 | ((rt.as_operand_bits() as u32) << 12)).to_vec()
            },
            Self::Vmov_Core_To_Single_T1(sn, rt) => {
                check_general_register_is_encodable("rt", rt)?;
                split_instruction_word_into_halfwords(0xEE00_0A10 | (sn.field() << 16) | ((rt.as_operand_bits() as u32) << 12) | (sn.extra_bit() << 7)).to_vec()
            },
            Self::Vmov_Single_To_Core_T1(rt, sn) => {
                check_general_register_is_encodable("rt", rt)?;
                split_instruction_word_into_halfwords(0xEE10_0A10 | (sn.field() << 16) | ((rt.as_operand_bits() as u32) << 12) | (sn.extra_bit() << 7)).to_vec()
            },
            Self::Vmov_Core_To_Scalar_T1(size, index, dd, rt) => {
                check_general_register_is_encodable("rt", rt)?;
                if *index >= size.lane_count() {
                    return Err(EncodeError::ImmediateOutOfRange { field: "VMOV scalar lane index", value: *index as i64, minimum: 0, maximum: size.lane_count() as i64 - 1 });
                }
                let (opc1, opc2) = size.opc_fields(*index);
                split_instruction_word_into_halfwords(0xEE00_0B10 | (opc1 << 21) | (dd.field() << 16) | ((rt.as_operand_bits() as u32) << 12) | (dd.extra_bit() << 7) | (opc2 << 5)).to_vec()
            },
            Self::Vmov_Scalar_To_Core_T1(unsigned, size, index, rt, dn) => {
                check_general_register_is_encodable("rt", rt)?;
                if *index >= size.lane_count() {
                    return Err(EncodeError::ImmediateOutOfRange { field: "VMOV scalar lane index", value: *index as i64, minimum: 0, maximum: size.lane_count() as i64 - 1 });
                }
                let (opc1, opc2) = size.opc_fields(*index);
                let u = if matches!(size, Arm32VmovLaneSize::Word) { 0 } else { *unsigned as u32 };
                split_instruction_word_into_halfwords(0xEE10_0B10 | (u << 23) | (opc1 << 21) | (dn.field() << 16) | ((rt.as_operand_bits() as u32) << 12) | (dn.extra_bit() << 7) | (opc2 << 5)).to_vec()
            },

            // ---- ARMv7E-M FP M8i: VCVT (integer in a single-precision register) ----
            Self::Vcvt_FloatToInt_FromSingle_T1(sd, sm, signed, round) => {
                let word = 0xEEBC_0A40 | ((*signed as u32) << 16) | ((*round as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcvt_FloatToInt_FromDouble_T1(sd, dm, signed, round) => {
                let word = 0xEEBC_0B40 | ((*signed as u32) << 16) | ((*round as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcvt_IntToFloat_ToSingle_T1(sd, sm, signed) => {
                let word = 0xEEB8_0A40 | ((*signed as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcvt_IntToFloat_ToDouble_T1(dd, sm, signed) => {
                let word = 0xEEB8_0B40 | ((*signed as u32) << 7) | (dd.extra_bit() << 22) | (dd.field() << 12) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcvt_Single_To_Double_T1(dd, sm) => {
                let word = 0xEEB7_0AC0 | (dd.extra_bit() << 22) | (dd.field() << 12) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcvt_Double_To_Single_T1(sd, dm) => {
                let word = 0xEEB7_0BC0 | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },

            // ---- ARMv7E-M FP M8i (final corners) ----
            Self::Vmov_Immediate_Single_T1(sd, imm8) => {
                let word = 0xEEB0_0A00 | (((*imm8 as u32) >> 4) << 16) | (sd.extra_bit() << 22) | (sd.field() << 12) | ((*imm8 as u32) & 0xF);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vmov_Immediate_Double_T1(dd, imm8) => {
                let word = 0xEEB0_0B00 | (((*imm8 as u32) >> 4) << 16) | (dd.extra_bit() << 22) | (dd.field() << 12) | ((*imm8 as u32) & 0xF);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vmov_CorePair_To_Double_T1(dm, rt, rt2) => encode_vmov_core_pair(0xEC40_0B10, false, rt, rt2, dm.field(), dm.extra_bit())?,
            Self::Vmov_Double_To_CorePair_T1(rt, rt2, dm) => encode_vmov_core_pair(0xEC40_0B10, true, rt, rt2, dm.field(), dm.extra_bit())?,
            Self::Vmov_CorePair_To_Singles_T1(sm, rt, rt2) => encode_vmov_core_pair(0xEC40_0A10, false, rt, rt2, sm.field(), sm.extra_bit())?,
            Self::Vmov_Singles_To_CorePair_T1(rt, rt2, sm) => encode_vmov_core_pair(0xEC40_0A10, true, rt, rt2, sm.field(), sm.extra_bit())?,
            Self::Vcvt_HalfToSingle_T1(sd, sm, top) => {
                let word = 0xEEB2_0A40 | ((*top as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcvt_SingleToHalf_T1(sd, sm, top) => {
                let word = 0xEEB3_0A40 | ((*top as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field();
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vcvt_FloatToFixed_Single_T1(sd, signed, bits32, frac) => encode_vcvt_fixed(sd.field(), sd.extra_bit(), 0, true, *signed, *bits32, *frac)?,
            Self::Vcvt_FloatToFixed_Double_T1(dd, signed, bits32, frac) => encode_vcvt_fixed(dd.field(), dd.extra_bit(), 1, true, *signed, *bits32, *frac)?,
            Self::Vcvt_FixedToFloat_Single_T1(sd, signed, bits32, frac) => encode_vcvt_fixed(sd.field(), sd.extra_bit(), 0, false, *signed, *bits32, *frac)?,
            Self::Vcvt_FixedToFloat_Double_T1(dd, signed, bits32, frac) => encode_vcvt_fixed(dd.field(), dd.extra_bit(), 1, false, *signed, *bits32, *frac)?,

            // ---- ARMv8-M Security Extension ----
            Self::Csdb_T1 => split_instruction_word_into_halfwords(ArmT32OpcodePattern_32Bit::Csdb_T1 as u32).to_vec(),
            Self::Sg_T1 => split_instruction_word_into_halfwords(ArmT32OpcodePattern_32Bit::Sg_T1 as u32).to_vec(),
            Self::Bxns_T1(rm) => vec![ArmT32OpcodePattern_16Bit::Bxns_T1 as u16 | ((rm.as_operand_bits() as u16) << 3)],
            Self::Blxns_T1(rm) => vec![ArmT32OpcodePattern_16Bit::Blxns_T1 as u16 | ((rm.as_operand_bits() as u16) << 3)],
            Self::Tt_T1(rd, rn, a, t) => {
                let word = ArmT32OpcodePattern_32Bit::Tt_T1 as u32 | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | ((*a as u32) << 7) | ((*t as u32) << 6);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            Self::Vlstm_T1(rn) => split_instruction_word_into_halfwords(ArmT32OpcodePattern_32Bit::Vlstm_T1 as u32 | ((rn.as_operand_bits() as u32) << 16)).to_vec(),
            Self::Vlldm_T1(rn) => split_instruction_word_into_halfwords(ArmT32OpcodePattern_32Bit::Vlldm_T1 as u32 | ((rn.as_operand_bits() as u32) << 16)).to_vec(),

            // ---- ARMv8.1-M MVE 3-reg-same vector-vector: Qn[19:17], Qd[15:13], Qm[3:1] ----
            Self::MveIntArith(op, size, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (size.size_bits() << 20) | (qn.field() << 17) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveBitwise(op, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (qn.field() << 17) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveFloatArith(op, size, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (size.size_bit() << 20) | (qn.field() << 17) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),

            // ---- MVE vector-by-scalar (Qd[15:13], Qn[19:17], Rm[3:0]) and VDUP (Qd[19:17], Rt[15:12]) ----
            Self::MveVecScalarInt(op, size, qd, qn, rm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (size.size_bits() << 20) | (qn.field() << 17) | (qd.field() << 13) | (rm.as_operand_bits() as u32)
                ).to_vec(),
            Self::MveVecScalarFloat(op, size, qd, qn, rm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (size.size_bit() << 28) | (qn.field() << 17) | (qd.field() << 13) | (rm.as_operand_bits() as u32)
                ).to_vec(),
            Self::MveVdup(size, qd, rt) => {
                let (b, e) = mve_vdup_size_bits(*size);
                split_instruction_word_into_halfwords(
                    MVE_VDUP_BASE | (b << 22) | (e << 5) | (qd.field() << 17) | ((rt.as_operand_bits() as u32) << 12)
                ).to_vec()
            },
            Self::MveShiftImm(op, size, amount, qd, qm) => {
                let esize = mve_shift_esize(*size);
                let imm6 = if op.is_left_shift() { esize + (*amount as u32) } else { 2 * esize - (*amount as u32) };
                split_instruction_word_into_halfwords(
                    op.base_word() | (imm6 << 16) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec()
            },
            Self::MveModifiedImmediate(cmode, op, imm8, qd) =>
                split_instruction_word_into_halfwords(encode_mve_modified_imm(*cmode, *op, *imm8, qd.field())).to_vec(),

            // ---- MVE two-register miscellaneous (Qd[15:13], Qm[3:1], size[19:18]) ----
            Self::MveMisc2(op, size, qd, qm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (size.size_bits() << 18) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveMisc2Float(op, size, qd, qm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (mve_misc2_float_size_bits(*size) << 18) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVmaxaMina(is_min, size, qda, qm) =>
                split_instruction_word_into_halfwords(
                    0xEE33_0E81 | ((*is_min as u32) << 12) | (size.size_bits() << 18) | (qda.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVmaxnmaMinnma(is_min, size, qda, qm) =>
                split_instruction_word_into_halfwords(
                    0xEE3F_0E81 | ((*is_min as u32) << 12) | (matches!(size, Arm32MveFloatSize::F16) as u32) << 28 | (qda.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveMvnRegister(qd, qm) =>
                split_instruction_word_into_halfwords(MVE_VMVN_REG_BASE | (qd.field() << 13) | (qm.field() << 1)).to_vec(),

            // ---- MVE contiguous vector load/store (base 0xEC00_1E00; P[24] U[23] W[21] L[20], size[8:7]) ----
            Self::MveLoadStore(is_load, size, qd, rn, offset, mode) => {
                let size_bytes = mve_shift_esize(*size) / 8; // 1 / 2 / 4 bytes
                let imm7 = (offset.unsigned_abs() / size_bytes) & 0x7F;
                let u = if *offset >= 0 { 1u32 } else { 0 };
                let (p, w) = match mode {
                    ArmT32IndexMode::Offset => (1u32, 0u32),
                    ArmT32IndexMode::PreIndex => (1, 1),
                    ArmT32IndexMode::PostIndex => (0, 1),
                };
                let l = if *is_load { 1u32 } else { 0 };
                let word = 0xEC00_1E00 | (p << 24) | (u << 23) | (w << 21) | (l << 20)
                    | ((rn.as_operand_bits() as u32) << 16) | (qd.field() << 13) | (size.size_bits() << 7) | imm7;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            // ---- MVE gather/scatter (scalar base + vector offset): esize[8:7], msize={bit6,bit4}, scaled=bit0 ----
            Self::MveGatherScatter(is_load, unsigned, esize, msize, scaled, qd, rn, qm) => {
                let esize_log = mve_mem_size_log(*esize);
                let msize_log = mve_mem_size_log(*msize);
                let msize_bits = (((msize_log >> 1) & 1) << 6) | ((msize_log & 1) << 4);
                let word = MVE_GATHER_SCATTER_BASE | ((*unsigned as u32) << 28) | ((*is_load as u32) << 20)
                    | ((rn.as_operand_bits() as u32) << 16) | (qd.field() << 13)
                    | (esize_log << 7) | msize_bits | (qm.field() << 1) | (*scaled as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            // ---- MVE gather/scatter, vector base + immediate: U(sign)=bit23, W=bit21, size=bit8, imm7[6:0] ----
            Self::MveGatherScatterBase(is_load, is_dword, writeback, qd, qn, offset) => {
                let scale = if *is_dword { 8 } else { 4 };
                let imm7 = (offset.unsigned_abs() / scale) & 0x7F;
                let add = if *offset >= 0 { 1u32 } else { 0 };
                let word = MVE_GATHER_VBASE_BASE | ((*is_load as u32) << 20) | ((*writeback as u32) << 21)
                    | (add << 23) | ((*is_dword as u32) << 8) | (qn.field() << 17) | (qd.field() << 13) | imm7;
                split_instruction_word_into_halfwords(word).to_vec()
            },
            // ---- MVE de-interleaving/interleaving load/store: pass[6:5], size[8:7], is-VLD4=bit0 ----
            Self::MveInterleave(is_load, is_quad, pass, size, qd, rn, writeback) => {
                let word = MVE_INTERLEAVE_BASE | ((*is_load as u32) << 20) | ((*writeback as u32) << 21)
                    | ((rn.as_operand_bits() as u32) << 16) | (qd.field() << 13) | (size.size_bits() << 7)
                    | (((*pass as u32) & 0b11) << 5) | (*is_quad as u32);
                split_instruction_word_into_halfwords(word).to_vec()
            },
            // ---- low-overhead loops ----
            Self::LobStart(is_while, tp_size, rn, offset) => {
                let hw0 = 0xF000 | (lob_size_field(*tp_size) << 4) | (rn.as_operand_bits() as u32);
                let hw1 = if *is_while { lob_branch_hw1(*offset) } else { 0xE001 };
                split_instruction_word_into_halfwords((hw0 << 16) | hw1).to_vec()
            },
            Self::LobEnd(tail_predicated, offset) => {
                let hw0: u32 = if *tail_predicated { 0xF01F } else { 0xF00F };
                split_instruction_word_into_halfwords((hw0 << 16) | lob_branch_hw1(*offset)).to_vec()
            },
            Self::Lctp => split_instruction_word_into_halfwords(MVE_LCTP_WORD).to_vec(),
            Self::MveVctp(size, rn) => {
                let size_bits = match size { 8 => 0u32, 16 => 1, 32 => 2, _ => 3 };
                split_instruction_word_into_halfwords(MVE_VCTP_BASE | (size_bits << 20) | ((rn.as_operand_bits() as u32) << 16)).to_vec()
            },

            // ---- MVE cross-lane reductions to a GPR ----
            Self::MveReduce(op, size, rd, qm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (size.size_bits() << 18) | ((rd.as_operand_bits() as u32) << 12) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVabav(signed, size, rd, qn, qm) => {
                let u = if *signed { 0u32 } else { 1 };
                split_instruction_word_into_halfwords(
                    MVE_VABAV_BASE | (u << 28) | (size.size_bits() << 20) | (qn.field() << 17) | ((rd.as_operand_bits() as u32) << 12) | (qm.field() << 1)
                ).to_vec()
            },
            // VMLADAV/VMLSDAV: Rda (even) at [15:12] with X folded into the freed bit 12, A=bit5, subtract=bit0.
            // The size/signedness bits (28/16/8) are irregular between add and subtract -- see mve_dualmac_size_bits.
            Self::MveDualMac(subtract, exchange, accumulate, unsigned, size, rda, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_DUALMAC_BASE
                        | mve_dualmac_size_bits(*subtract, *unsigned, *size)
                        | ((rda.as_operand_bits() as u32) << 12)
                        | ((*exchange as u32) << 12)
                        | (qn.field() << 17)
                        | (qm.field() << 1)
                        | ((*accumulate as u32) << 5)
                        | (*subtract as u32)
                ).to_vec(),
            // VMLALDAV/VMLSLDAV/VRMLALDAVH/VRMLSLDAVH: RdaLo[15:12] even (+ X folded into bit12), RdaHi>>1 at [22:20]
            Self::MveLongDualMac(op, exchange, accumulate, unsigned, size, rda_lo, rda_hi, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_LONG_DUALMAC_BASE
                        | mve_long_dualmac_bits(*op, *unsigned, *size)
                        | ((rda_lo.as_operand_bits() as u32) << 12)
                        | ((*exchange as u32) << 12)
                        | (((rda_hi.as_operand_bits() as u32) >> 1) << 20)
                        | (qn.field() << 17)
                        | (qm.field() << 1)
                        | ((*accumulate as u32) << 5)
                ).to_vec(),

            // ---- MVE VRINT / VCVT (float<->int) in the 0xFFBx space (size[19:18], Qd[15:13], Qm[3:1]) ----
            Self::MveVrint(op, size, qd, qm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (mve_misc2_float_size_bits(*size) << 18) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVcvtFloatInt(to_int, unsigned, size, qd, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCVT_FI_BASE | ((*to_int as u32) << 8) | ((*unsigned as u32) << 7)
                        | (mve_misc2_float_size_bits(*size) << 18) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            // fixed-point VCVT: imm6[21:16] = 64 - fbits, size = bit9, direction = bit8, U = bit28
            Self::MveVcvtFixed(to_fixed, unsigned, size, fbits, qd, qm) => {
                let imm6 = 64 - (*fbits as u32);
                split_instruction_word_into_halfwords(
                    MVE_VCVT_FIXED_BASE | ((*unsigned as u32) << 28) | (imm6 << 16)
                        | ((matches!(size, Arm32MveFloatSize::F32) as u32) << 9) | ((*to_fixed as u32) << 8)
                        | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec()
            },
            // VCVT half<->single (vector): op=bit28, T=bit12, Qd[15:13], Qm[3:1] (DDI0553)
            Self::MveVcvtHalf(top, half_to_single, qd, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCVT_HALF_BASE | ((*half_to_single as u32) << 28) | ((*top as u32) << 12)
                        | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            // shift-and-narrow: imm5 = (16|32) - shift at [20:16]; op bits = bit28 / [7:6] / bit0 (DDI0553)
            Self::MveShiftNarrow(op, unsigned, top, src_is_32, shift, qd, qm) => {
                let imm5 = (if *src_is_32 { 32 } else { 16 }) - (*shift as u32);
                let (bit28, bit76, bit0) = op.opcode_bits(*unsigned);
                split_instruction_word_into_halfwords(
                    MVE_SHIFT_NARROW_BASE | (bit28 << 28) | (imm5 << 16) | (qd.field() << 13)
                        | ((*top as u32) << 12) | (bit76 << 6) | (qm.field() << 1) | bit0
                ).to_vec()
            },
            // VADC/VSBC: subtract = bit28, init-carry = bit12
            Self::MveVadc(subtract, init_carry, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VADC_BASE | ((*subtract as u32) << 28) | ((*init_carry as u32) << 12)
                        | (qd.field() << 13) | (qn.field() << 17) | (qm.field() << 1)
                ).to_vec(),
            // VSHLC: imm5[20:16] = shift (32 encoded as 0), Qda[15:13], Rdm[3:0]
            Self::MveVshlc(shift, qda, rdm) =>
                split_instruction_word_into_halfwords(
                    MVE_VSHLC_BASE | (((*shift as u32) & 0x1F) << 16) | (qda.field() << 13) | (rdm.as_operand_bits() as u32)
                ).to_vec(),
            // VIDUP/VDDUP/VIWDUP/VDWDUP: Rn>>1 at [19:17], step (log2) split across {bit7,bit0}, wrap Rm>>1 at
            // [3:1] (PC = no wrap)
            Self::MveViddup(decrement, size, qd, rn, wrap_rm, step) => {
                let imm2 = step.trailing_zeros(); // 1->0, 2->1, 4->2, 8->3
                let rm_field = match wrap_rm {
                    Some(rm) => (rm.as_operand_bits() as u32) >> 1,
                    None => 0b111, // r15 (PC) marks the non-wrapping form
                };
                split_instruction_word_into_halfwords(
                    MVE_VIDDUP_BASE | (size.size_bits() << 20) | (((rn.as_operand_bits() as u32) >> 1) << 17)
                        | (qd.field() << 13) | ((*decrement as u32) << 12)
                        | (imm2 & 1) | (((imm2 >> 1) & 1) << 7) | (rm_field << 1)
                ).to_vec()
            },
            // VBRSR: vector-by-scalar shape -- Qd[15:13], Qn[19:17], Rm[3:0], size[21:20]
            Self::MveVbrsr(size, qd, qn, rm) =>
                split_instruction_word_into_halfwords(
                    MVE_VBRSR_BASE | (size.size_bits() << 20) | (qd.field() << 13) | (qn.field() << 17) | (rm.as_operand_bits() as u32)
                ).to_vec(),

            // ---- MVE width-changing register moves (VMOVL long, VMOVN narrow, VADDLV 64-bit reduction) ----
            Self::MveVmovl(top, unsigned, size, qd, qm) => {
                let size_bit = if matches!(size, Arm32MveSize::I8) { 1u32 << 19 } else { 1u32 << 20 }; // .8 -> bit19, .16 -> bit20
                split_instruction_word_into_halfwords(
                    MVE_VMOVL_BASE | ((*unsigned as u32) << 28) | size_bit | ((*top as u32) << 12) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec()
            },
            Self::MveVmovn(top, size, qd, qm) => {
                let size_bit = if matches!(size, Arm32MveSize::I32) { 1u32 << 18 } else { 0 }; // .16 -> 0, .32 -> bit18
                split_instruction_word_into_halfwords(
                    MVE_VMOVN_BASE | size_bit | ((*top as u32) << 12) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec()
            },
            Self::MveVmovTwoLane(to_vector, idx1, qd, rt, rt2) => {
                if *idx1 != 2 && *idx1 != 3 {
                    return Err(EncodeError::ImmediateOutOfRange { field: "VMOV two-lane index", value: *idx1 as i64, minimum: 2, maximum: 3 });
                }
                split_instruction_word_into_halfwords(
                    0xEC00_0F00 | ((*to_vector as u32) << 20) | ((rt2.as_operand_bits() as u32) << 16)
                        | (qd.field() << 13) | (((*idx1 as u32) & 1) << 4) | (rt.as_operand_bits() as u32)
                ).to_vec()
            },
            Self::MveVqmovn(kind, unsigned, top, size, qd, qm) => {
                let size_bit = if matches!(size, Arm32MveSize::I32) { 1u32 << 18 } else { 0 }; // .16 -> 0, .32 -> bit18
                let base = match kind {
                    Arm32MveQMovnKind::Vqmovn => MVE_VQMOVN_BASE | ((*unsigned as u32) << 28),
                    Arm32MveQMovnKind::Vqmovun => MVE_VQMOVUN_BASE,
                };
                split_instruction_word_into_halfwords(
                    base | size_bit | ((*top as u32) << 12) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec()
            },
            Self::MveVmull(polynomial, unsigned, top, size, qd, qn, qm) => {
                let head = if *polynomial {
                    // poly: size[21:20]=11 (in base), bit28 selects P8(I8)/P16(I16)
                    MVE_VMULL_POLY_BASE | ((matches!(size, Arm32MveSize::I16) as u32) << 28)
                } else {
                    MVE_VMULL_INT_BASE | ((*unsigned as u32) << 28) | (size.size_bits() << 20)
                };
                split_instruction_word_into_halfwords(
                    head | ((*top as u32) << 12) | (qd.field() << 13) | (qn.field() << 17) | (qm.field() << 1)
                ).to_vec()
            },
            Self::MveVmulh(rounding, unsigned, size, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VMULH_BASE | ((*unsigned as u32) << 28) | (size.size_bits() << 20) | ((*rounding as u32) << 12)
                        | (qd.field() << 13) | (qn.field() << 17) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVqdmull(top, size32, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VQDMULL_VEC_BASE | ((*size32 as u32) << 28) | ((*top as u32) << 12)
                        | (qd.field() << 13) | (qn.field() << 17) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVqdmullScalar(top, size32, qd, qn, rm) =>
                split_instruction_word_into_halfwords(
                    MVE_VQDMULL_SCALAR_BASE | ((*size32 as u32) << 28) | ((*top as u32) << 12)
                        | (qd.field() << 13) | (qn.field() << 17) | (rm.as_operand_bits() as u32)
                ).to_vec(),
            Self::MveVqdmladh(subtract, rounding, exchange, size, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VQDMLADH_BASE | ((*subtract as u32) << 28) | (*rounding as u32) | ((*exchange as u32) << 12)
                        | (size.size_bits() << 20) | (qd.field() << 13) | (qn.field() << 17) | (qm.field() << 1)
                ).to_vec(),
            Self::MveShiftByVector(rounding, saturating, unsigned, size, qd, qm, qn) =>
                split_instruction_word_into_halfwords(
                    MVE_SHIFT_VEC_BASE | ((*unsigned as u32) << 28) | (size.size_bits() << 20) | ((*rounding as u32) << 8)
                        | ((*saturating as u32) << 4) | (qd.field() << 13) | (qm.field() << 1) | (qn.field() << 17)
                ).to_vec(),
            Self::MveShiftByScalar(rounding, saturating, unsigned, size, qda, rm) =>
                split_instruction_word_into_halfwords(
                    MVE_SHIFT_SCALAR_BASE | ((*unsigned as u32) << 28) | (size.size_bits() << 18) | ((*rounding as u32) << 17)
                        | ((*saturating as u32) << 7) | (qda.field() << 13) | (rm.as_operand_bits() as u32)
                ).to_vec(),
            Self::MveVshll(top, unsigned, size, shift, qd, qm) => {
                let esize: u32 = if matches!(size, Arm32MveSize::I16) { 16 } else { 8 };
                let word = if *shift as u32 == esize {
                    MVE_VSHLL_T2_BASE | ((matches!(size, Arm32MveSize::I16) as u32) << 18)
                } else {
                    MVE_VSHLL_T1_BASE | ((esize + *shift as u32) << 16)
                };
                split_instruction_word_into_halfwords(
                    word | ((*unsigned as u32) << 28) | ((*top as u32) << 12) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec()
            },
            Self::Vmovx_T1(insert, sd, sm) =>
                split_instruction_word_into_halfwords(
                    MVE_VMOVX_BASE | ((*insert as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()
                ).to_vec(),
            Self::Dbg_T1(option) => split_instruction_word_into_halfwords(0xF3AF_80F0 | (*option as u32 & 0xF)).to_vec(),
            Self::Esb_T1 => split_instruction_word_into_halfwords(0xF3AF_8010).to_vec(),
            Self::Ssbb_T1 => split_instruction_word_into_halfwords(0xF3BF_8F40).to_vec(),
            Self::Pssbb_T1 => split_instruction_word_into_halfwords(0xF3BF_8F44).to_vec(),
            Self::Sb_T1 => split_instruction_word_into_halfwords(0xF3BF_8F70).to_vec(),
            Self::Clrm_T1(list) => split_instruction_word_into_halfwords(0xE89F_0000 | (*list as u32 & 0xDFFF)).to_vec(),
            Self::Vsel_Single_T1(cond, sd, sn, sm) => split_instruction_word_into_halfwords(
                0xFE00_0A00 | ((*cond as u32) << 20) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sn.extra_bit() << 7) | (sn.field() << 16) | (sm.extra_bit() << 5) | sm.field()
            ).to_vec(),
            Self::Vsel_Double_T1(cond, dd, dn, dm) => split_instruction_word_into_halfwords(
                0xFE00_0B00 | ((*cond as u32) << 20) | (dd.extra_bit() << 22) | (dd.field() << 12) | (dn.extra_bit() << 7) | (dn.field() << 16) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::Csel_T1(op, rd, rn, rm, cond) => split_instruction_word_into_halfwords(
                0xEA50_8000 | ((rn.as_operand_bits() as u32) << 16) | ((*op as u32) << 12)
                    | ((rd.as_operand_bits() as u32) << 8) | ((cond.as_operand_bits() as u32) << 4) | (rm.as_operand_bits() as u32)
            ).to_vec(),
            Self::LongShiftImm_T1(op, rdalo, rdahi, imm) => split_instruction_word_into_halfwords(
                0xEA50_010F | ((rdalo.as_operand_bits() as u32) << 16) | (((rdahi.as_operand_bits() as u32 >> 1) & 0b111) << 9)
                    | (((*imm as u32 >> 2) & 0b111) << 12) | ((*imm as u32 & 0b11) << 6) | ((*op as u32) << 4)
            ).to_vec(),
            Self::LongShiftReg_T1(op, rdalo, rdahi, rm) => split_instruction_word_into_halfwords(
                0xEA50_010D | ((rdalo.as_operand_bits() as u32) << 16) | (((rdahi.as_operand_bits() as u32 >> 1) & 0b111) << 9)
                    | ((rm.as_operand_bits() as u32) << 12) | ((*op as u32) << 4)
            ).to_vec(),
            Self::SatShiftImm_T1(op, rda, imm) => split_instruction_word_into_halfwords(
                0xEA50_0F0F | ((rda.as_operand_bits() as u32) << 16) | (((*imm as u32 >> 2) & 0b111) << 12)
                    | ((*imm as u32 & 0b11) << 6) | ((*op as u32) << 4)
            ).to_vec(),
            Self::SatShiftLongImm_T1(op, rdalo, rdahi, imm) => split_instruction_word_into_halfwords(
                0xEA51_010F | (((rdalo.as_operand_bits() as u32 >> 1) & 0b111) << 17) | (((rdahi.as_operand_bits() as u32 >> 1) & 0b111) << 9)
                    | (((*imm as u32 >> 2) & 0b111) << 12) | ((*imm as u32 & 0b11) << 6) | ((*op as u32) << 4)
            ).to_vec(),
            Self::SatShiftReg_T1(signed, rda, rm) => split_instruction_word_into_halfwords(
                0xEA50_0F0D | ((rda.as_operand_bits() as u32) << 16) | ((rm.as_operand_bits() as u32) << 12) | ((*signed as u32) << 5)
            ).to_vec(),
            Self::SatShiftLongReg_T1(signed, rdalo, rdahi, rm, sat48) => split_instruction_word_into_halfwords(
                0xEA51_010D | (((rdalo.as_operand_bits() as u32 >> 1) & 0b111) << 17) | (((rdahi.as_operand_bits() as u32 >> 1) & 0b111) << 9)
                    | ((rm.as_operand_bits() as u32) << 12) | ((*signed as u32) << 5) | ((*sat48 as u32) << 7)
            ).to_vec(),
            Self::Vrintr_Single_T1(sd, sm) => split_instruction_word_into_halfwords(
                0xEEB6_0A40 | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()
            ).to_vec(),
            Self::Vrintr_Double_T1(dd, dm) => split_instruction_word_into_halfwords(
                0xEEB6_0B40 | (dd.extra_bit() << 22) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::Vmaxnm_Single_T1(sd, sn, sm) => split_instruction_word_into_halfwords(
                0xFE80_0A00 | (sd.extra_bit() << 22) | (sn.field() << 16) | (sd.field() << 12) | (sn.extra_bit() << 7) | (sm.extra_bit() << 5) | sm.field()
            ).to_vec(),
            Self::Vmaxnm_Double_T1(dd, dn, dm) => split_instruction_word_into_halfwords(
                0xFE80_0B00 | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | (dn.extra_bit() << 7) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::Vminnm_Single_T1(sd, sn, sm) => split_instruction_word_into_halfwords(
                0xFE80_0A40 | (sd.extra_bit() << 22) | (sn.field() << 16) | (sd.field() << 12) | (sn.extra_bit() << 7) | (sm.extra_bit() << 5) | sm.field()
            ).to_vec(),
            Self::Vminnm_Double_T1(dd, dn, dm) => split_instruction_word_into_halfwords(
                0xFE80_0B40 | (dd.extra_bit() << 22) | (dn.field() << 16) | (dd.field() << 12) | (dn.extra_bit() << 7) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::Vrint_Directed_Single_T1(mode, sd, sm) => split_instruction_word_into_halfwords(
                0xFEB8_0A40 | (mode.rm_bits() << 16) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()
            ).to_vec(),
            Self::Vrint_Directed_Double_T1(mode, dd, dm) => split_instruction_word_into_halfwords(
                0xFEB8_0B40 | (mode.rm_bits() << 16) | (dd.extra_bit() << 22) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::Vrintz_Single_T1(sd, sm) => split_instruction_word_into_halfwords(
                0xEEB6_0AC0 | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()
            ).to_vec(),
            Self::Vrintz_Double_T1(dd, dm) => split_instruction_word_into_halfwords(
                0xEEB6_0BC0 | (dd.extra_bit() << 22) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::Vrintx_Single_T1(sd, sm) => split_instruction_word_into_halfwords(
                0xEEB7_0A40 | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()
            ).to_vec(),
            Self::Vrintx_Double_T1(dd, dm) => split_instruction_word_into_halfwords(
                0xEEB7_0B40 | (dd.extra_bit() << 22) | (dd.field() << 12) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::Vcvt_Directed_FromSingle_T1(mode, sd, sm, signed) => split_instruction_word_into_halfwords(
                0xFEBC_0A40 | (mode.rm_bits() << 16) | ((*signed as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (sm.extra_bit() << 5) | sm.field()
            ).to_vec(),
            Self::Vcvt_Directed_FromDouble_T1(mode, sd, dm, signed) => split_instruction_word_into_halfwords(
                0xFEBC_0B40 | (mode.rm_bits() << 16) | ((*signed as u32) << 7) | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::Vjcvt_T1(sd, dm) => split_instruction_word_into_halfwords(
                0xEEB9_0BC0 | (sd.extra_bit() << 22) | (sd.field() << 12) | (dm.extra_bit() << 5) | dm.field()
            ).to_vec(),
            Self::MveVaddlv(accumulate, unsigned, rd_lo, rd_hi, qm) => {
                let lo = rd_lo.as_operand_bits() as u32;
                let hi = rd_hi.as_operand_bits() as u32;
                split_instruction_word_into_halfwords(
                    MVE_VADDLV_BASE | ((*unsigned as u32) << 28) | ((*accumulate as u32) << 5)
                        | (lo << 12) | ((hi >> 1) << 20) | (qm.field() << 1)
                ).to_vec()
            },

            // ---- MVE complex-number ops (Qd[15:13], Qn[19:17], Qm[3:1] throughout) ----
            Self::MveVcaddInt(halving, size, rot270, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCADD_INT_PATTERN | (if *halving { 0 } else { 1 << 28 }) | (size.size_bits() << 20)
                        | ((*rot270 as u32) << 12) | (qn.field() << 17) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVcaddFloat(size, rot270, qd, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCADD_FLOAT_PATTERN | (if matches!(size, Arm32MveFloatSize::F32) { 1 << 20 } else { 0 })
                        | ((*rot270 as u32) << 24) | (qn.field() << 17) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVcmul(size, rotate, qd, qn, qm) => {
                let r = *rotate as u32;
                split_instruction_word_into_halfwords(
                    MVE_VCMUL_PATTERN | (if matches!(size, Arm32MveFloatSize::F32) { 1 << 28 } else { 0 })
                        | (r & 1) | (((r >> 1) & 1) << 12) | (qn.field() << 17) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec()
            },
            Self::MveVcmla(size, rotate, qd, qn, qm) => {
                let r = *rotate as u32;
                split_instruction_word_into_halfwords(
                    MVE_VCMLA_PATTERN | (if matches!(size, Arm32MveFloatSize::F32) { 1 << 20 } else { 0 })
                        | ((r & 1) << 23) | (((r >> 1) & 1) << 24) | (qn.field() << 17) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec()
            },

            // ---- MVE predication primitives ----
            Self::MveVpsel(qd, qn, qm) =>
                split_instruction_word_into_halfwords(MVE_VPSEL_BASE | (qn.field() << 17) | (qd.field() << 13) | (qm.field() << 1)).to_vec(),
            Self::MveVpnot => split_instruction_word_into_halfwords(MVE_VPNOT_WORD).to_vec(),

            // ---- MVE VCMP (compare into the VPR) ----
            Self::MveVcmpReg(cond, size, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCMP_INT_BASE | (size.size_bits() << 20) | mve_vcmp_fc_bits(*cond, false) | (qn.field() << 17) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVcmpScalar(cond, size, qn, rm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCMP_INT_BASE | (1 << 6) | (size.size_bits() << 20) | mve_vcmp_fc_bits(*cond, true) | (qn.field() << 17) | (rm.as_operand_bits() as u32)
                ).to_vec(),
            Self::MveVcmpFloatReg(cond, size, qn, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCMP_FLOAT_BASE | (if matches!(size, Arm32MveFloatSize::F16) { 1 << 28 } else { 0 }) | mve_vcmp_fc_bits(*cond, false) | (qn.field() << 17) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVcmpFloatScalar(cond, size, qn, rm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCMP_FLOAT_BASE | (1 << 6) | (if matches!(size, Arm32MveFloatSize::F16) { 1 << 28 } else { 0 }) | mve_vcmp_fc_bits(*cond, true) | (qn.field() << 17) | (rm.as_operand_bits() as u32)
                ).to_vec(),
            Self::MveVpst(mask) =>
                split_instruction_word_into_halfwords(MVE_VPST_NOT_BASE | mve_predicate_mask_bits(*mask)).to_vec(),

            // ---- VPT (compare + predicate block) = VCMP | predicate-mask bits ----
            Self::MveVptReg(cond, size, qn, qm, mask) =>
                split_instruction_word_into_halfwords(
                    MVE_VCMP_INT_BASE | (size.size_bits() << 20) | mve_vcmp_fc_bits(*cond, false) | (qn.field() << 17) | (qm.field() << 1) | mve_predicate_mask_bits(*mask)
                ).to_vec(),
            Self::MveVptScalar(cond, size, qn, rm, mask) =>
                split_instruction_word_into_halfwords(
                    MVE_VCMP_INT_BASE | (1 << 6) | (size.size_bits() << 20) | mve_vcmp_fc_bits(*cond, true) | (qn.field() << 17) | (rm.as_operand_bits() as u32) | mve_predicate_mask_bits(*mask)
                ).to_vec(),
            Self::MveVptFloatReg(cond, size, qn, qm, mask) =>
                split_instruction_word_into_halfwords(
                    MVE_VCMP_FLOAT_BASE | (if matches!(size, Arm32MveFloatSize::F16) { 1 << 28 } else { 0 }) | mve_vcmp_fc_bits(*cond, false) | (qn.field() << 17) | (qm.field() << 1) | mve_predicate_mask_bits(*mask)
                ).to_vec(),
            Self::MveVptFloatScalar(cond, size, qn, rm, mask) =>
                split_instruction_word_into_halfwords(
                    MVE_VCMP_FLOAT_BASE | (1 << 6) | (if matches!(size, Arm32MveFloatSize::F16) { 1 << 28 } else { 0 }) | mve_vcmp_fc_bits(*cond, true) | (qn.field() << 17) | (rm.as_operand_bits() as u32) | mve_predicate_mask_bits(*mask)
                ).to_vec(),
            Self::MveFloatReduce(op, size, rd, qm) =>
                split_instruction_word_into_halfwords(
                    op.base_word() | (if matches!(size, Arm32MveFloatSize::F16) { 1 << 28 } else { 0 }) | ((rd.as_operand_bits() as u32) << 12) | (qm.field() << 1)
                ).to_vec(),
            Self::MveVcvtRound(rounding, unsigned, size, qd, qm) =>
                split_instruction_word_into_halfwords(
                    MVE_VCVTR_BASE | ((*rounding as u32 & 0b11) << 8) | ((*unsigned as u32) << 7)
                        | (mve_misc2_float_size_bits(*size) << 18) | (qd.field() << 13) | (qm.field() << 1)
                ).to_vec(),
        };

        Ok(convert_halfwords_to_u8_vec(&halfwords))
    }

    // What this instruction needs from the target processor to be emittable (minimum ISA version +
    // extension features). The ARMv7-M (Thumb-2) forms report `Armv7M`; everything else is the ARMv6-M
    // baseline (the `_` arm). This is what makes `encode_for_target(&ArmTargetProfile::armv6m())` reject a
    // v7-M instruction with `UnsupportedInstructionForTarget`.
    pub fn requirement(&self) -> ArmInstructionRequirement {
        match self {
            Self::Mov_Immediate_T3(..) |
            Self::Movt_T1(..) |
            Self::Mul_T2(..) |
            Self::Mla_T1(..) |
            Self::Mls_T1(..) |
            Self::Sdiv_T1(..) |
            Self::Udiv_T1(..) |
            Self::Clz_T1(..) |
            Self::Rbit_T1(..) |
            Self::Ubfx_T1(..) |
            Self::Sbfx_T1(..) |
            Self::Bfi_T1(..) |
            Self::Bfc_T1(..) |
            Self::Ldr_Immediate_T3(..) |
            Self::Str_Immediate_T3(..) |
            Self::Ldrex_T1(..) |
            Self::Strex_T1(..) |
            Self::Ldrexb_T1(..) |
            Self::Strexb_T1(..) |
            Self::Ldrexh_T1(..) |
            Self::Strexh_T1(..) |
            Self::Clrex_T1 |
            Self::Tbb_T1(..) |
            Self::Tbh_T1(..) |
            Self::Mov_Immediate_T2(..) |
            Self::Mvn_Immediate_T1(..) |
            Self::And_Immediate_T1(..) |
            Self::Bic_Immediate_T1(..) |
            Self::Orr_Immediate_T1(..) |
            Self::Eor_Immediate_T1(..) |
            Self::Add_Immediate_T3(..) |
            Self::Sub_Immediate_T3(..) |
            Self::Tst_Immediate_T1(..) |
            Self::Teq_Immediate_T1(..) |
            Self::Cmn_Immediate_T1(..) |
            Self::Cmp_Immediate_T2(..) |
            Self::Adc_Immediate_T1(..) |
            Self::Sbc_Immediate_T1(..) |
            Self::Rsb_Immediate_T2(..) |
            Self::Orn_Immediate_T1(..) |
            Self::Add_Register_T3(..) |
            Self::Sub_Register_T2(..) |
            Self::And_Register_T2(..) |
            Self::Orr_Register_T2(..) |
            Self::Eor_Register_T2(..) |
            Self::Bic_Register_T2(..) |
            Self::Mov_Register_T3(..) |
            Self::Mvn_Register_T2(..) |
            Self::Adc_Register_T2(..) |
            Self::Sbc_Register_T2(..) |
            Self::Rsb_Register_T1(..) |
            Self::Orn_Register_T1(..) |
            Self::Tst_Register_T2(..) |
            Self::Teq_Register_T1(..) |
            Self::Cmn_Register_T2(..) |
            Self::Cmp_Register_T3(..) |
            Self::Ldrb_Immediate_T2(..) |
            Self::Strb_Immediate_T2(..) |
            Self::Ldrh_Immediate_T2(..) |
            Self::Strh_Immediate_T2(..) |
            Self::Ldrsb_Immediate_T1(..) |
            Self::Ldrsh_Immediate_T1(..) |
            Self::Ldr_Register_T2(..) |
            Self::Str_Register_T2(..) |
            Self::Ldrb_Register_T2(..) |
            Self::Strb_Register_T2(..) |
            Self::Ldrh_Register_T2(..) |
            Self::Strh_Register_T2(..) |
            Self::Ldrsb_Register_T2(..) |
            Self::Ldrsh_Register_T2(..) |
            Self::Smull_T1(..) |
            Self::Umull_T1(..) |
            Self::Smlal_T1(..) |
            Self::Umlal_T1(..) |
            Self::Umaal_T1(..) |
            Self::Sxtb_T2(..) |
            Self::Uxtb_T2(..) |
            Self::Sxth_T2(..) |
            Self::Uxth_T2(..) |
            Self::Rev_T2(..) |
            Self::Rev16_T2(..) |
            Self::Revsh_T2(..) |
            Self::Ssat_T1(..) |
            Self::Usat_T1(..) |
            Self::Ldr_Immediate_T4(..) |
            Self::Str_Immediate_T4(..) |
            Self::Ldrb_Immediate_T3(..) |
            Self::Strb_Immediate_T3(..) |
            Self::Ldrh_Immediate_T3(..) |
            Self::Strh_Immediate_T3(..) |
            Self::Ldrsb_Immediate_T2(..) |
            Self::Ldrsh_Immediate_T2(..) |
            Self::Ldrd_Immediate_T1(..) |
            Self::Strd_Immediate_T1(..) |
            Self::Ldr_Literal_T2(..) |
            Self::Ldrb_Literal_T1(..) |
            Self::Ldrh_Literal_T1(..) |
            Self::Ldrsb_Literal_T1(..) |
            Self::Ldrsh_Literal_T1(..) |
            Self::Pld_Immediate_T1(..) |
            Self::Pli_Immediate_T1(..) |
            Self::Ldmia_T2(..) |
            Self::Stmia_T2(..) |
            Self::Ldmdb_T1(..) |
            Self::Stmdb_T1(..) |
            Self::B_T4(..) |
            Self::B_T3(..) |
            Self::Cbz_T1(..) |
            Self::Cbnz_T1(..) |
            Self::It_T1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[]),

            // ---- ARMv7E-M DSP extension (Armv7EM + DspExtension) ----
            Self::Qadd_T1(..) |
            Self::Qsub_T1(..) |
            Self::Qdadd_T1(..) |
            Self::Qdsub_T1(..) |
            Self::Sxtab_T1(..) |
            Self::Uxtab_T1(..) |
            Self::Sxtah_T1(..) |
            Self::Uxtah_T1(..) |
            Self::Sxtab16_T1(..) |
            Self::Uxtab16_T1(..) |
            Self::Sxtb16_T1(..) |
            Self::Uxtb16_T1(..) |
            Self::Pkhbt_T1(..) |
            Self::Pkhtb_T1(..) |
            Self::Ssat16_T1(..) |
            Self::Usat16_T1(..) |
            Self::Sel_T1(..) |
            Self::Usad8_T1(..) |
            Self::Usada8_T1(..) |
            Self::ParallelAddSub_T1(..) |
            Self::Smul_T1(..) |
            Self::Smulw_T1(..) |
            Self::Smla_T1(..) |
            Self::Smlaw_T1(..) |
            Self::Smlal_Halfword_T1(..) |
            Self::Smuad_T1(..) |
            Self::Smusd_T1(..) |
            Self::Smlad_T1(..) |
            Self::Smlsd_T1(..) |
            Self::Smlald_T1(..) |
            Self::Smlsld_T1(..) |
            Self::Smmul_T1(..) |
            Self::Smmla_T1(..) |
            Self::Smmls_T1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7EM, &[ArmCpuFeature::DspExtension]),

            // ---- ARMv7E-M hardware floating-point (FloatingPoint feature) ----
            Self::Vldr_Single_T2(..) |
            Self::Vstr_Single_T2(..) |
            Self::Vldr_Double_T1(..) |
            Self::Vstr_Double_T1(..) |
            Self::Vldm_Single_T2(..) |
            Self::Vstm_Single_T2(..) |
            Self::Vldm_Double_T1(..) |
            Self::Vstm_Double_T1(..) |
            Self::FldmdbxFstmdbx_T1(..) |
            Self::FpDataProcess3_Single(..) |
            Self::FpDataProcess3_Double(..) |
            Self::FpDataProcess2_Single(..) |
            Self::FpDataProcess2_Double(..) |
            Self::Vcmp_Single_T1(..) |
            Self::Vcmp_Double_T1(..) |
            Self::Vcmp_Zero_Single_T2(..) |
            Self::Vcmp_Zero_Double_T2(..) |
            Self::Vmrs_T1(..) |
            Self::Vmrs_Apsr_Nzcv_T1 |
            Self::Vmsr_T1(..) |
            Self::Vmov_Core_To_Single_T1(..) |
            Self::Vmov_Single_To_Core_T1(..) |
            Self::Vmov_Core_To_Scalar_T1(..) |
            Self::Vmov_Scalar_To_Core_T1(..) |
            Self::Vcvt_FloatToInt_FromSingle_T1(..) |
            Self::Vcvt_FloatToInt_FromDouble_T1(..) |
            Self::Vcvt_IntToFloat_ToSingle_T1(..) |
            Self::Vcvt_IntToFloat_ToDouble_T1(..) |
            Self::Vcvt_Single_To_Double_T1(..) |
            Self::Vcvt_Double_To_Single_T1(..) |
            Self::Vmov_Immediate_Single_T1(..) |
            Self::Vmov_Immediate_Double_T1(..) |
            Self::Vmov_CorePair_To_Double_T1(..) |
            Self::Vmov_Double_To_CorePair_T1(..) |
            Self::Vmov_CorePair_To_Singles_T1(..) |
            Self::Vmov_Singles_To_CorePair_T1(..) |
            Self::Vcvt_HalfToSingle_T1(..) |
            Self::Vcvt_SingleToHalf_T1(..) |
            Self::Vcvt_FloatToFixed_Single_T1(..) |
            Self::Vcvt_FloatToFixed_Double_T1(..) |
            Self::Vcvt_FixedToFloat_Single_T1(..) |
            Self::Vcvt_FixedToFloat_Double_T1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[ArmCpuFeature::FloatingPoint]),

            // CSDB is a 32-bit hint (Thumb-2); it executes as a NOP on cores without speculation, so it only
            // needs the ARMv7-M 32-bit encoding space.
            Self::Csdb_T1 => ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[]),
            // ARMv8-M Security Extension (TrustZone-M): the v8-M baseline plus the Security feature.
            Self::Sg_T1 | Self::Bxns_T1(..) | Self::Blxns_T1(..) | Self::Tt_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8MBaseline, &[ArmCpuFeature::Security]),
            // lazy FP state save/restore additionally needs an FPU and the Mainline profile.
            Self::Vlstm_T1(..) | Self::Vlldm_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8MMainline, &[ArmCpuFeature::Security, ArmCpuFeature::FloatingPoint]),

            // ARMv8.1-M MVE (Helium): the integer/bitwise vector forms need the MVE feature; the
            // floating-point vector forms additionally need the MVE floating-point option.
            Self::MveIntArith(..) | Self::MveBitwise(..) | Self::MveVecScalarInt(..) | Self::MveVdup(..)
            | Self::MveShiftImm(..) | Self::MveMisc2(..) | Self::MveMvnRegister(..) | Self::MveLoadStore(..) | Self::MveGatherScatter(..) | Self::MveGatherScatterBase(..) | Self::MveInterleave(..)
            | Self::MveReduce(..) | Self::MveVabav(..) | Self::MveDualMac(..) | Self::MveLongDualMac(..) | Self::MveVmovl(..) | Self::MveVmovn(..) | Self::MveVqmovn(..) | Self::MveVaddlv(..)
            | Self::MveVmull(..) | Self::MveVmulh(..) | Self::MveVqdmull(..) | Self::MveVqdmullScalar(..) | Self::MveVqdmladh(..)
            | Self::MveShiftByVector(..) | Self::MveShiftByScalar(..) | Self::MveVshll(..)
            | Self::MveVcaddInt(..) | Self::MveVpsel(..) | Self::MveVpnot | Self::MveVpst(..)
            | Self::MveVadc(..) | Self::MveVshlc(..) | Self::MveViddup(..) | Self::MveVbrsr(..) | Self::MveShiftNarrow(..)
            | Self::MveVcmpReg(..) | Self::MveVcmpScalar(..) | Self::MveVptReg(..) | Self::MveVptScalar(..)
            | Self::MveVmaxaMina(..) | Self::MveVmovTwoLane(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::Mve]),
            Self::MveVcaddFloat(..) | Self::MveVcmul(..) | Self::MveVcmla(..)
            | Self::MveVcmpFloatReg(..) | Self::MveVcmpFloatScalar(..) | Self::MveVptFloatReg(..) | Self::MveVptFloatScalar(..)
            | Self::MveFloatReduce(..) | Self::MveVcvtRound(..) | Self::MveVcvtFixed(..) | Self::MveVcvtHalf(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::Mve, ArmCpuFeature::MveFloat]),
            Self::MveMisc2Float(..) | Self::MveVmaxnmaMinnma(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::Mve, ArmCpuFeature::MveFloat]),
            Self::MveFloatArith(..) | Self::MveVecScalarFloat(..) | Self::MveVrint(..) | Self::MveVcvtFloatInt(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::Mve, ArmCpuFeature::MveFloat]),
            // the modified immediate is integer MVE, except VMOV.f32 (cmode 0b1111, op=0) which needs MVE FP.
            Self::MveModifiedImmediate(0b1111, false, _, _) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::Mve, ArmCpuFeature::MveFloat]),
            Self::MveModifiedImmediate(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::Mve]),
            // low-overhead loops: the plain DLS/WLS/LE need only the Armv8.1-M Low-Overhead-Branch extension;
            // the tail-predicated DLSTP/WLSTP/LETP and VCTP/LCTP need MVE.
            Self::LobStart(_, Some(_), _, _) | Self::LobEnd(true, _) | Self::Lctp | Self::MveVctp(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::Mve]),
            Self::LobStart(_, None, _, _) | Self::LobEnd(false, _) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[]),
            // VMOVX/VINS are the Armv8.1-M half-precision FP move-extract/insert (need an FPU, not MVE).
            Self::Vmovx_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::FloatingPoint]),
            // ARMv8-M load-acquire / store-release (LDA/STL + exclusive variants).
            Self::LoadAcquire_T1(..) | Self::StoreRelease_T1(..) | Self::StoreReleaseExclusive_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8MBaseline, &[]),
            // Unprivileged load/store (LDRT/STRT family) -- ARMv7-M 32-bit encodings.
            Self::UnprivLoadStore_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[]),
            // Generic coprocessor instructions -- ARMv7-M 32-bit encodings.
            Self::Coproc_Mcr_T1(..) | Self::Coproc_Cdp_T1(..) | Self::Coproc_Mcrr_T1(..) | Self::Coproc_Ldc_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[]),
            // PACBTI (pointer authentication / branch target identification) -- ARMv8.1-M extension.
            Self::PacbtiHint_T1(..) | Self::PacbtiData_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[]),
            // VSCCLRM -- ARMv8.1-M Security Extension + an FPU/MVE register file.
            Self::Vscclrm_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::FloatingPoint]),
            // CDE custom datapath (the CX1/CX2/CX3 GPR forms) -- ARMv8-M Custom Datapath Extension.
            Self::Cde_Cx1_T1(..) | Self::Cde_Cx2_T1(..) | Self::Cde_Cx3_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8MBaseline, &[]),
            // Branch Future (BF/BFL/BFX/BFLX/BFCSEL) -- Armv8.1-M Low Overhead Branch extension (no MVE needed).
            Self::Bf_T1(..) | Self::Bfl_T4(..) | Self::Bfx_T3(..) | Self::Bflx_T5(..) | Self::Bfcsel_T2(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[]),
            // CDE VCX1/2/3 -- the Custom Datapath Extension with an FP/vector register file. The vector (Q) form
            // (kind==2) needs MVE; the scalar S/D forms need a floating-point unit.
            Self::Vcx1_T1(_, 2, ..) | Self::Vcx2_T1(_, 2, ..) | Self::Vcx3_T1(_, 2, ..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8MBaseline, &[ArmCpuFeature::Mve]),
            Self::Vcx1_T1(..) | Self::Vcx2_T1(..) | Self::Vcx3_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8MBaseline, &[ArmCpuFeature::FloatingPoint]),
            // Batch 9 hints/barriers/CLRM/VSEL.
            Self::Dbg_T1(..) => ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[]),
            Self::Esb_T1 | Self::Ssbb_T1 | Self::Pssbb_T1 | Self::Sb_T1 | Self::Clrm_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8MBaseline, &[]),
            Self::Vsel_Single_T1(..) | Self::Vsel_Double_T1(..) | Self::Vrintr_Single_T1(..) | Self::Vrintr_Double_T1(..)
            | Self::Vmaxnm_Single_T1(..) | Self::Vmaxnm_Double_T1(..) | Self::Vminnm_Single_T1(..) | Self::Vminnm_Double_T1(..)
            | Self::Vrint_Directed_Single_T1(..) | Self::Vrint_Directed_Double_T1(..)
            | Self::Vrintz_Single_T1(..) | Self::Vrintz_Double_T1(..) | Self::Vrintx_Single_T1(..) | Self::Vrintx_Double_T1(..)
            | Self::Vcvt_Directed_FromSingle_T1(..) | Self::Vcvt_Directed_FromDouble_T1(..)
            | Self::Vjcvt_T1(..) => // VJCVT needs FEAT_JSCVT; no finer gate in the model than the v8-M FP group.
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8MBaseline, &[ArmCpuFeature::FloatingPoint]),
            // ARMv8.1-M conditional select (CSEL/CSINC/CSINV/CSNEG and the CSET/CINC/... aliases).
            Self::Csel_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[]),
            // 64-bit long shifts and the saturating/rounding scalar shifts -- MVE scalar-shift extension.
            Self::LongShiftImm_T1(..) | Self::LongShiftReg_T1(..)
            | Self::SatShiftImm_T1(..) | Self::SatShiftLongImm_T1(..) | Self::SatShiftReg_T1(..) | Self::SatShiftLongReg_T1(..) =>
                ArmInstructionRequirement::new(ArmIsaVersion::Armv8_1MMainline, &[ArmCpuFeature::Mve]),

            _ => ArmInstructionRequirement::baseline(),
        }
    }

    // For an IT instruction, the condition that applies to each of its N (1..=4) member instructions:
    // slot 1 is `firstcond`; a later slot is `firstcond` when its mask bit equals firstcond[0] (a "then"),
    // otherwise the inverse condition (an "else"). Returns None for any non-IT instruction. The
    // disassembler uses this to apply the right `<c>` suffix to the instructions that follow an IT.
    pub fn it_block_member_conditions(&self) -> Option<Vec<ArmT32InstructionCondition>> {
        let (firstcond, mask) = match self {
            Self::It_T1(firstcond, mask) => (*firstcond, *mask),
            _ => return None,
        };
        let firstcond_low_bit = firstcond.as_operand_bits() & 1;
        let inverse = ArmT32InstructionCondition::from_operand_bits(firstcond.as_operand_bits() ^ 1);
        let length = 4 - mask.trailing_zeros() as usize; // 1..=4
        let mut conditions = vec![firstcond];
        for slot in 2..=length {
            let bit = (mask >> (5 - slot)) & 1;
            conditions.push(if bit == firstcond_low_bit { firstcond } else { inverse });
        }
        Some(conditions)
    }

    /// If this instruction opens a VPT/VPST predicate block, the then/else (`t`/`e`) letter for each of the
    /// instructions that follow it, in order. The disassembler appends these to the member mnemonics, and the
    /// assembler uses them to strip a predication suffix (the predicated and plain encodings are identical).
    pub fn vpt_block_member_suffixes(&self) -> Option<Vec<char>> {
        let mask = match self {
            Self::MveVpst(mask) => *mask,
            Self::MveVptReg(.., mask) | Self::MveVptScalar(.., mask)
            | Self::MveVptFloatReg(.., mask) | Self::MveVptFloatScalar(.., mask) => *mask,
            _ => return None,
        };
        Some(mve_predicate_mask_suffix(mask).chars().collect())
    }

    /// Encode this instruction, but first verify `target_profile` actually supports it. The bytes
    /// themselves are target-independent (a Thumb encoding is fixed), so [`encode`](Self::encode) stays
    /// pure and this guarded path sits beside it for a compiler backend to use. Returns [`EncodeError`] if
    /// the target lacks the required architecture version or extension.
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
fn decode_word_imm16_movw(word: u32) -> u16 {
    let imm4 = (word >> 16) & 0b1111;
    let i = (word >> 26) & 0b1;
    let imm3 = (word >> 12) & 0b111;
    let imm8 = word & 0b1111_1111;
    ((imm4 << 12) | (i << 11) | (imm3 << 8) | imm8) as u16
}

// ---- ThumbExpandImm (data-processing "modified immediate") codec ----
//
// A 12-bit field `i:imm3:imm8` expands to a 32-bit constant (ARMv7-M ARM, ThumbExpandImm):
//   * top two bits 00 -> one of four byte forms keyed by bits 9:8 (zero-extend / 0x00XY00XY /
//     0xXY00XY00 / 0xXYXYXYXY);
//   * otherwise -> the 8-bit value (1:imm8<6:0>) rotated right by the 5-bit rotation imm12<11:7>.

pub(crate) fn decode_thumb_expand_imm(imm12: u16) -> u32 {
    let imm12 = (imm12 & 0x0FFF) as u32;
    if (imm12 >> 10) & 0b11 == 0b00 {
        let imm8 = imm12 & 0xFF;
        match (imm12 >> 8) & 0b11 {
            0b00 => imm8,
            0b01 => (imm8 << 16) | imm8,
            0b10 => (imm8 << 24) | (imm8 << 8),
            _ /* 0b11 */ => (imm8 << 24) | (imm8 << 16) | (imm8 << 8) | imm8,
        }
    } else {
        let unrotated = 0x80u32 | (imm12 & 0x7F);
        let rotation = (imm12 >> 7) & 0b1_1111;
        unrotated.rotate_right(rotation)
    }
}

// Inverse: the canonical 12-bit field for a constant, or None if it is not encodable. Mirrors the order
// GNU/LLVM assemblers pick (byte forms first, then the smallest rotation), so encode==their bytes.
pub(crate) fn encode_thumb_expand_imm(value: u32) -> Option<u16> {
    if value <= 0xFF {
        return Some(value as u16); // byte form 00 (top bits 0)
    }
    let byte0 = value & 0xFF;
    let byte1 = (value >> 8) & 0xFF;
    if byte0 != 0 && value == (byte0 | (byte0 << 16)) {
        return Some(0x100 | byte0 as u16); // 0x00XY00XY
    }
    if byte1 != 0 && value == ((byte1 << 8) | (byte1 << 24)) {
        return Some(0x200 | byte1 as u16); // 0xXY00XY00
    }
    if byte0 != 0 && value == (byte0 | (byte0 << 8) | (byte0 << 16) | (byte0 << 24)) {
        return Some(0x300 | byte0 as u16); // 0xXYXYXYXY
    }
    // rotation form: smallest rotation in 8..=31 whose left-rotate yields an 8-bit value with bit 7 set
    for rotation in 8..=31u32 {
        let candidate = value.rotate_left(rotation);
        if (0x80..=0xFF).contains(&candidate) {
            return Some(((rotation as u16) << 7) | ((candidate as u16) & 0x7F));
        }
    }
    None
}

// Place the 12-bit modified-immediate field into a data-processing word (i @ bit 26, imm3 @ 14:12,
// imm8 @ 7:0).
fn modified_immediate_field_bits(imm12: u16) -> u32 {
    let i = ((imm12 >> 11) & 0b1) as u32;
    let imm3 = ((imm12 >> 8) & 0b111) as u32;
    let imm8 = (imm12 & 0xFF) as u32;
    (i << 26) | (imm3 << 12) | imm8
}
fn decode_word_modified_immediate(word: u32) -> u32 {
    let i = (word >> 26) & 0b1;
    let imm3 = (word >> 12) & 0b111;
    let imm8 = word & 0xFF;
    decode_thumb_expand_imm(((i << 11) | (imm3 << 8) | imm8) as u16)
}

// Build a "data processing (modified immediate)" word (T1 family): `11110 i 0 op4 S Rn 0 imm3 Rd imm8`.
fn encode_data_processing_modified_immediate(op4: u32, set_flags: bool, rn: u8, rd: u8, value: u32) -> Result<u32, EncodeError> {
    let imm12 = encode_thumb_expand_imm(value).ok_or(EncodeError::ModifiedImmediateNotEncodable { field: "const", value })?;
    let word = 0b11110_0_0_0000_0_0000_0_000_0000_00000000u32
        | (op4 << 21)
        | ((set_flags as u32) << 20)
        | ((rn as u32) << 16)
        | ((rd as u32) << 8)
        | modified_immediate_field_bits(imm12);
    Ok(word)
}

// ---- data-processing (shifted register) shift codec ----

// Validate the shift and place it: the 2-bit `type` at bits 5:4 and the 5-bit amount as imm3:imm2 at
// bits 14:12 / 7:6. Amounts are the decoded UAL values (LSL 0..=31, LSR/ASR 1..=32 with 32 encoded as 0,
// ROR 1..=31).
fn encode_register_shift_field(shift: &ArmT32RegisterShift) -> Result<u32, EncodeError> {
    let field: u32 = match shift {
        ArmT32RegisterShift::Lsl(amount) => {
            if *amount > 31 { return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: *amount as i64, minimum: 0, maximum: 31 }); }
            *amount as u32
        },
        ArmT32RegisterShift::Lsr(amount) | ArmT32RegisterShift::Asr(amount) => {
            if *amount < 1 || *amount > 32 { return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: *amount as i64, minimum: 1, maximum: 32 }); }
            if *amount == 32 { 0 } else { *amount as u32 }
        },
        ArmT32RegisterShift::Ror(amount) => {
            if *amount < 1 || *amount > 31 { return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: *amount as i64, minimum: 1, maximum: 31 }); }
            *amount as u32
        },
        ArmT32RegisterShift::Rrx => 0, // ROR type with a zero amount
    };
    let imm3 = (field >> 2) & 0b111;
    let imm2 = field & 0b11;
    Ok((imm3 << 12) | (imm2 << 6) | ((shift.type_bits() as u32) << 4))
}

fn decode_register_shift(word: u32) -> ArmT32RegisterShift {
    let imm3 = (word >> 12) & 0b111;
    let imm2 = (word >> 6) & 0b11;
    let field = ((imm3 << 2) | imm2) as u8;
    match (word >> 4) & 0b11 {
        0b00 => ArmT32RegisterShift::Lsl(field),
        0b01 => ArmT32RegisterShift::Lsr(if field == 0 { 32 } else { field }),
        0b10 => ArmT32RegisterShift::Asr(if field == 0 { 32 } else { field }),
        _ /* 0b11 */ => if field == 0 { ArmT32RegisterShift::Rrx } else { ArmT32RegisterShift::Ror(field) },
    }
}

// Build a "data processing (shifted register)" word: `11101 01 op4 S Rn (0) imm3 Rd imm2 type Rm`.
fn encode_data_processing_shifted_register(op4: u32, set_flags: bool, rn: u8, rd: u8, rm: u8, shift: &ArmT32RegisterShift) -> Result<u32, EncodeError> {
    let word = 0b11101_01_0000_0_0000_0_000_0000_00_00_0000u32
        | (op4 << 21)
        | ((set_flags as u32) << 20)
        | ((rn as u32) << 16)
        | ((rd as u32) << 8)
        | (rm as u32)
        | encode_register_shift_field(shift)?;
    Ok(word)
}

// Guard the three registers and emit the halfwords for a shifted-register data-processing form.
fn encode_dp_shifted_register(op4: u32, set_flags: bool, rd: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, shift: &ArmT32RegisterShift) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    let word = encode_data_processing_shifted_register(op4, set_flags, rn.as_operand_bits(), rd.as_operand_bits(), rm.as_operand_bits(), shift)?;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// MOV/MVN (shifted register): the Rn field is the fixed PC marker; only rd and rm are user registers.
fn encode_mov_mvn_register(op4: u32, set_flags: bool, rd: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, shift: &ArmT32RegisterShift) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rm", rm)?;
    let word = encode_data_processing_shifted_register(op4, set_flags, 0b1111, rd.as_operand_bits(), rm.as_operand_bits(), shift)?;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// TST/TEQ/CMN/CMP (shifted register): the Rd field is the fixed PC marker, S is always set; rn and rm
// are user registers.
fn encode_compare_register(op4: u32, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, shift: &ArmT32RegisterShift) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    let word = encode_data_processing_shifted_register(op4, true, rn.as_operand_bits(), 0b1111, rm.as_operand_bits(), shift)?;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_rd_11_08(word: u32) -> u8 {
    ((word >> 8) & 0b1111) as u8
}
fn decode_word_rn_rd_rm(word: u32) -> (/*rn*/u8, /*rd*/u8, /*rm*/u8) {
    let rn = ((word >> 16) & 0b1111) as u8;
    let rd = ((word >> 8) & 0b1111) as u8;
    let rm = (word & 0b1111) as u8;
    (rn, rd, rm)
}
fn decode_word_rn_ra_rd_rm(word: u32) -> (/*rn*/u8, /*ra*/u8, /*rd*/u8, /*rm*/u8) {
    let rn = ((word >> 16) & 0b1111) as u8;
    let ra = ((word >> 12) & 0b1111) as u8;
    let rd = ((word >> 8) & 0b1111) as u8;
    let rm = (word & 0b1111) as u8;
    (rn, ra, rd, rm)
}

// Bitfield (UBFX/SBFX/BFI/BFC) field layout: lsb = imm3(14:12):imm2(7:6); the 5-bit field in bits 4:0 is
// widthm1 for UBFX/SBFX and msb for BFI/BFC.
fn decode_word_bitfield(word: u32) -> (/*rn*/u8, /*rd*/u8, /*lsb*/u8, /*widthm1 or msb*/u8) {
    let rn = ((word >> 16) & 0b1111) as u8;
    let rd = ((word >> 8) & 0b1111) as u8;
    let imm3 = ((word >> 12) & 0b111) as u8;
    let imm2 = ((word >> 6) & 0b11) as u8;
    let lsb = (imm3 << 2) | imm2;
    let low_five = (word & 0b1_1111) as u8;
    (rn, rd, lsb, low_five)
}

// Wide load/store (T3) immediate layout: Rn in 19:16, Rt in 15:12, imm12 in 11:0.
fn decode_word_rn_rt_imm12(word: u32) -> (/*rn*/u8, /*rt*/u8, /*imm12*/u16) {
    let rn = ((word >> 16) & 0b1111) as u8;
    let rt = ((word >> 12) & 0b1111) as u8;
    let imm12 = (word & 0b1111_1111_1111) as u16;
    (rn, rt, imm12)
}

// Shorthand for from_operand_bits, used by the dense load/store decode arms.
fn g(bits: u8) -> Arm32GeneralPurposeRegister {
    Arm32GeneralPurposeRegister::from_operand_bits(bits)
}

// Long-multiply layout: Rn 19:16, RdLo 15:12, RdHi 11:8, Rm 3:0.
fn decode_word_long_multiply(word: u32) -> (/*rn*/u8, /*rdlo*/u8, /*rdhi*/u8, /*rm*/u8) {
    let rn = ((word >> 16) & 0b1111) as u8;
    let rdlo = ((word >> 12) & 0b1111) as u8;
    let rdhi = ((word >> 8) & 0b1111) as u8;
    let rm = (word & 0b1111) as u8;
    (rn, rdlo, rdhi, rm)
}

// Register-offset load/store layout: Rn 19:16, Rt 15:12, LSL amount 5:4, Rm 3:0.
fn decode_word_rn_rt_lsl_rm(word: u32) -> (/*rn*/u8, /*rt*/u8, /*lsl*/u8, /*rm*/u8) {
    let rn = ((word >> 16) & 0b1111) as u8;
    let rt = ((word >> 12) & 0b1111) as u8;
    let lsl = ((word >> 4) & 0b11) as u8;
    let rm = (word & 0b1111) as u8;
    (rn, rt, lsl, rm)
}

// Wide load/store with a 12-bit immediate offset (`Rt, [Rn, #imm12]`): base | Rn<<16 | Rt<<12 | imm12.
fn encode_load_store_immediate12(base: u32, rt: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, imm12: u16) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rt", rt)?;
    check_register_is_not_pc("rn", rn)?;
    check_unsigned_maximum("imm12", imm12 as u32, 4095)?;
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (imm12 as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// Long multiply (`RdLo, RdHi, Rn, Rm`): base | Rn<<16 | RdLo<<12 | RdHi<<8 | Rm.
fn encode_long_multiply(base: u32, rdlo: &Arm32GeneralPurposeRegister, rdhi: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rdlo", rdlo)?;
    check_general_register_is_encodable("rdhi", rdhi)?;
    check_general_register_is_encodable("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rdlo.as_operand_bits() as u32) << 12) | ((rdhi.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// ---- M7l: wide extend, wide byte-reverse, saturate ----

// Wide extend SXTB.W/UXTB.W/SXTH.W/UXTH.W (`Rd, Rm{, ROR #rotation}`): base | Rd<<8 | (rotation/8)<<4 | Rm.
// `rotation` is the decoded amount (0/8/16/24); only those four rotates have an encoding.
fn encode_extend(base: u32, rd: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, rotation: u8) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rm", rm)?;
    if !rotation.is_multiple_of(8) {
        return Err(EncodeError::ImmediateNotAligned { field: "rotation", value: rotation as i64, required_multiple: 8 });
    }
    if rotation > 24 {
        return Err(EncodeError::ImmediateOutOfRange { field: "rotation", value: rotation as i64, minimum: 0, maximum: 24 });
    }
    let rotate = (rotation / 8) as u32;
    let word = base | ((rd.as_operand_bits() as u32) << 8) | (rotate << 4) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_extend(word: u32) -> (/*rd*/u8, /*rotation*/u8, /*rm*/u8) {
    let rd = ((word >> 8) & 0b1111) as u8;
    let rotation = (((word >> 4) & 0b11) as u8) * 8;
    let rm = (word & 0b1111) as u8;
    (rd, rotation, rm)
}

// Wide byte-reverse REV.W/REV16.W/REVSH.W (`Rd, Rm`): like CLZ/RBIT, Rm occupies BOTH bits 19:16 and 3:0.
fn encode_byte_reverse(base: u32, rd: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rm", rm)?;
    let word = base | ((rm.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// SSAT/USAT (`Rd, #sat, Rn{, shift}`): base | sh<<21 | Rn<<16 | imm3<<12 | Rd<<8 | imm2<<6 | sat_field.
// sat_field is (sat_imm-1) for SSAT (saturates to sat_imm bits, 1..=32) and sat_imm for USAT (0..=31).
// The shift on Rn is LSL #0..=31 (sh=0) or ASR #1..=31 (sh=1); its amount fills imm3:imm2.
fn encode_saturate(base: u32, is_usat: bool, rd: &Arm32GeneralPurposeRegister, sat_imm: u8, rn: &Arm32GeneralPurposeRegister, shift: ArmT32RegisterShift) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rn", rn)?;
    let sat_field = if is_usat {
        if sat_imm > 31 {
            return Err(EncodeError::ImmediateOutOfRange { field: "sat_imm", value: sat_imm as i64, minimum: 0, maximum: 31 });
        }
        sat_imm as u32
    } else {
        if !(1..=32).contains(&sat_imm) {
            return Err(EncodeError::ImmediateOutOfRange { field: "sat_imm", value: sat_imm as i64, minimum: 1, maximum: 32 });
        }
        (sat_imm - 1) as u32
    };
    let (sh, amount) = match shift {
        ArmT32RegisterShift::Lsl(a) => {
            if a > 31 {
                return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: a as i64, minimum: 0, maximum: 31 });
            }
            (0u32, a as u32)
        },
        ArmT32RegisterShift::Asr(a) => {
            if !(1..=31).contains(&a) {
                return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: a as i64, minimum: 1, maximum: 31 });
            }
            (1u32, a as u32)
        },
        _ => return Err(EncodeError::ShiftNotEncodable { field: "shift", detail: "SSAT/USAT allow only LSL or ASR" }),
    };
    let imm3 = (amount >> 2) & 0b111;
    let imm2 = amount & 0b11;
    let word = base | (sh << 21) | ((rn.as_operand_bits() as u32) << 16) | (imm3 << 12) | ((rd.as_operand_bits() as u32) << 8) | (imm2 << 6) | sat_field;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_saturate(word: u32, is_usat: bool) -> (/*rd*/u8, /*sat_imm*/u8, /*rn*/u8, /*shift*/ArmT32RegisterShift) {
    let sh = (word >> 21) & 0b1;
    let rn = ((word >> 16) & 0b1111) as u8;
    let imm3 = (word >> 12) & 0b111;
    let rd = ((word >> 8) & 0b1111) as u8;
    let imm2 = (word >> 6) & 0b11;
    let sat = (word & 0b11111) as u8;
    let sat_imm = if is_usat { sat } else { sat + 1 };
    let amount = ((imm3 << 2) | imm2) as u8;
    // sh selects LSL (#0..=31) vs ASR (#1..=31); LSL #0 renders as "no shift".
    let shift = if sh == 0 { ArmT32RegisterShift::Lsl(amount) } else { ArmT32RegisterShift::Asr(amount) };
    (rd, sat_imm, rn, shift)
}

// ---- M7i: indexed load/store, dual load/store, literal loads, preload ----

// Single-register indexed load/store (`Rt, [Rn, #+/-imm8]{!}` / `[Rn], #+/-imm8`). `base` already has the
// T4/T3/T2 marker (bit 11) set. Layout: base | Rn<<16 | Rt<<12 | P<<10 | U<<9 | W<<8 | imm8. In `Offset`
// mode the offset must be negative -- a non-negative offset uses the imm12 form instead, and its T4 bit
// pattern (P=1,U=1,W=0) is the unprivileged LDRT/STRT encoding, not an `[Rn, #+imm]` access.
fn encode_load_store_indexed(base: u32, rt: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, offset: i16, mode: ArmT32IndexMode) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rt", rt)?;
    check_register_is_not_pc("rn", rn)?;
    if !(-255..=255).contains(&offset) {
        return Err(EncodeError::ImmediateOutOfRange { field: "offset", value: offset as i64, minimum: -255, maximum: 255 });
    }
    if matches!(mode, ArmT32IndexMode::Offset) && offset >= 0 {
        return Err(EncodeError::ImmediateOutOfRange { field: "offset", value: offset as i64, minimum: -255, maximum: -1 });
    }
    let (p, w) = mode.p_w_bits();
    let u = if offset >= 0 { 1 } else { 0 };
    let imm8 = offset.unsigned_abs() as u32;
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (p << 10) | (u << 9) | (w << 8) | imm8;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// `None` when the (P,W) bits are the unmodeled LDRT/STRT case (P=0,W=0), or the (P=1,U=1,W=0) pattern that
// is LDRT/STRT rather than an `[Rn, #+imm]` offset access.
fn decode_word_load_store_indexed(word: u32) -> Option<(/*rt*/u8, /*rn*/u8, /*offset*/i16, ArmT32IndexMode)> {
    let rn = ((word >> 16) & 0b1111) as u8;
    let rt = ((word >> 12) & 0b1111) as u8;
    let p = (word >> 10) & 1;
    let u = (word >> 9) & 1;
    let w = (word >> 8) & 1;
    let mode = ArmT32IndexMode::from_p_w_bits(p, w)?;
    if matches!(mode, ArmT32IndexMode::Offset) && u == 1 {
        return None;
    }
    let imm8 = (word & 0xFF) as i16;
    let offset = if u == 1 { imm8 } else { -imm8 };
    Some((rt, rn, offset, mode))
}

// Dual-register load/store (`Rt, Rt2, [Rn, #+/-(imm8*4)]{!}` / `[Rn], #+/-(imm8*4)`).
// Layout: 0xE8400000 | P<<24 | U<<23 | W<<21 | L<<20 | Rn<<16 | Rt<<12 | Rt2<<8 | (offset/4).
fn encode_load_store_dual(is_load: bool, rt: &Arm32GeneralPurposeRegister, rt2: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, offset: i16, mode: ArmT32IndexMode) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rt", rt)?;
    check_general_register_is_encodable("rt2", rt2)?;
    check_register_is_not_pc("rn", rn)?;
    if offset % 4 != 0 {
        return Err(EncodeError::ImmediateNotAligned { field: "offset", value: offset as i64, required_multiple: 4 });
    }
    if !(-1020..=1020).contains(&offset) {
        return Err(EncodeError::ImmediateOutOfRange { field: "offset", value: offset as i64, minimum: -1020, maximum: 1020 });
    }
    let (p, w) = mode.p_w_bits();
    let u = if offset >= 0 { 1 } else { 0 };
    let imm8 = (offset.unsigned_abs() / 4) as u32;
    let l = if is_load { 1 } else { 0 };
    let word = 0xE840_0000 | (p << 24) | (u << 23) | (w << 21) | (l << 20) | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | ((rt2.as_operand_bits() as u32) << 8) | imm8;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_load_store_dual(word: u32) -> Option<(/*rt*/u8, /*rt2*/u8, /*rn*/u8, /*offset*/i16, ArmT32IndexMode)> {
    let p = (word >> 24) & 1;
    let u = (word >> 23) & 1;
    let w = (word >> 21) & 1;
    let mode = ArmT32IndexMode::from_p_w_bits(p, w)?;
    let rn = ((word >> 16) & 0b1111) as u8;
    let rt = ((word >> 12) & 0b1111) as u8;
    let rt2 = ((word >> 8) & 0b1111) as u8;
    let magnitude = ((word & 0xFF) as i16) * 4;
    let offset = if u == 1 { magnitude } else { -magnitude };
    Some((rt, rt2, rn, offset, mode))
}

// PC-relative literal load (`Rt, [pc, #+/-imm12]`). `base` is the U=0 form; the U bit (bit 23) carries the
// sign. Layout: base | U<<23 | Rt<<12 | imm12.
fn encode_load_literal(base: u32, rt: &Arm32GeneralPurposeRegister, offset: i32) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rt", rt)?;
    if !(-4095..=4095).contains(&offset) {
        return Err(EncodeError::ImmediateOutOfRange { field: "offset", value: offset as i64, minimum: -4095, maximum: 4095 });
    }
    let u = if offset >= 0 { 1u32 } else { 0 };
    let imm12 = offset.unsigned_abs();
    let word = base | (u << 23) | ((rt.as_operand_bits() as u32) << 12) | imm12;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_load_literal(word: u32) -> (/*rt*/u8, /*offset*/i32) {
    let u = (word >> 23) & 1;
    let rt = ((word >> 12) & 0b1111) as u8;
    let imm12 = (word & 0xFFF) as i32;
    let offset = if u == 1 { imm12 } else { -imm12 };
    (rt, offset)
}

// Preload hint (`PLD/PLI [Rn, #+/-imm]`). A non-negative offset uses the imm12 form (`pos_base | Rn<<16 |
// imm12`, with the Rt field fixed at 1111); a negative offset uses the imm8 form (`neg_base | Rn<<16 |
// imm8`, with Rt=1111 and P=1/U=0/W=0 baked into `neg_base`).
fn encode_preload(pos_base: u32, neg_base: u32, rn: &Arm32GeneralPurposeRegister, offset: i32) -> Result<Vec<u16>, EncodeError> {
    check_register_is_not_pc("rn", rn)?;
    if !(-255..=4095).contains(&offset) {
        return Err(EncodeError::ImmediateOutOfRange { field: "offset", value: offset as i64, minimum: -255, maximum: 4095 });
    }
    let word = if offset >= 0 {
        pos_base | ((rn.as_operand_bits() as u32) << 16) | (offset as u32)
    } else {
        neg_base | ((rn.as_operand_bits() as u32) << 16) | ((-offset) as u32)
    };
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// Wide load/store multiple (LDM.W/STM.W/LDMDB/STMDB): base | W<<21 | Rn<<16 | register_list16. SP is never
// permitted in the list; PC is permitted only for loads (it makes LDM an interworking return / POP {..,pc}).
fn encode_load_store_multiple(base: u32, is_load: bool, rn: &Arm32GeneralPurposeRegister, writeback: bool, registers: &[Arm32GeneralPurposeRegister]) -> Result<Vec<u16>, EncodeError> {
    check_register_is_not_pc("rn", rn)?;
    if registers.is_empty() {
        return Err(EncodeError::RegisterNotEncodable { field: "registers", detail: "the register list must not be empty" });
    }
    let list = gpr_coding_utils::convert_registers_slice_to_gpr_register_list_u16(registers)?;
    if list & (1 << 13) != 0 {
        return Err(EncodeError::RegisterNotEncodable { field: "registers", detail: "SP (R13) is not permitted in a load/store-multiple register list" });
    }
    if !is_load && (list & (1 << 15) != 0) {
        return Err(EncodeError::RegisterNotEncodable { field: "registers", detail: "PC (R15) is not permitted in a store-multiple register list" });
    }
    let w = if writeback { 1u32 } else { 0 };
    let word = base | (w << 21) | ((rn.as_operand_bits() as u32) << 16) | (list as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_load_store_multiple(word: u32) -> (/*rn*/u8, /*writeback*/bool, /*registers*/Vec<Arm32GeneralPurposeRegister>) {
    let writeback = (word >> 21) & 1 == 1;
    let rn = ((word >> 16) & 0b1111) as u8;
    let registers = gpr_coding_utils::convert_gpr_register_list_u16_to_registers_vector((word & 0xFFFF) as u16);
    (rn, writeback, registers)
}

// ---- M7m: wide branches (B.W / B<c>.W) and compare-and-branch (CBZ / CBNZ) ----

// B.W (T4): unconditional, +/-16MB. Same S/I1/I2 byte-offset scrambling as BL (PC = address + 4); only the
// opcode differs (bit14 of the second halfword is 0 for B.W vs 1 for BL).
fn encode_branch_wide_unconditional(offset: i32) -> Result<Vec<u16>, EncodeError> {
    check_multiple_of("offset", offset as i64, 2)?;
    check_signed_range("offset", offset, -16_777_216, 16_777_214)?;
    let imm24 = (offset / 2) as u32;
    let s = (imm24 >> 23) & 1;
    let i1 = (imm24 >> 22) & 1;
    let i2 = (imm24 >> 21) & 1;
    let imm10 = (imm24 >> 11) & 0x3FF;
    let imm11 = imm24 & 0x7FF;
    let j1 = ((!i1) ^ s) & 1;
    let j2 = ((!i2) ^ s) & 1;
    let word = 0xF000_9000 | (s << 26) | (imm10 << 16) | (j1 << 13) | (j2 << 11) | imm11;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_branch_wide_unconditional(word: u32) -> i32 {
    let s = (word >> 26) & 1;
    let imm10 = (word >> 16) & 0x3FF;
    let j1 = (word >> 13) & 1;
    let j2 = (word >> 11) & 1;
    let imm11 = word & 0x7FF;
    let i1 = (!(j1 ^ s)) & 1;
    let i2 = (!(j2 ^ s)) & 1;
    let imm24 = (s << 23) | (i1 << 22) | (i2 << 21) | (imm10 << 11) | imm11;
    sign_extension_utils::sign_extend_int_to_i32(imm24 as i32, 24) * 2
}

// B<c>.W (T3): conditional, +/-1MB. imm21 = SignExtend(S:J2:J1:imm6:imm11:'0'); J1/J2 are direct here (no
// XOR with S, unlike T4). The AL / 0b1111 conditions are not encodable (those slots are B.W / other ops).
fn encode_branch_wide_conditional(cond: &ArmT32InstructionCondition, offset: i32) -> Result<Vec<u16>, EncodeError> {
    if *cond == ArmT32InstructionCondition::AlwaysUnconditional {
        return Err(EncodeError::ConditionNotEncodable { field: "cond", detail: "the AL condition is not encodable in B<c>.W T3; use B.W instead" });
    }
    if *cond == ArmT32InstructionCondition::Undefined(0b1111) {
        return Err(EncodeError::ConditionNotEncodable { field: "cond", detail: "the 0b1111 condition is not encodable in B<c>.W T3" });
    }
    check_multiple_of("offset", offset as i64, 2)?;
    check_signed_range("offset", offset, -1_048_576, 1_048_574)?;
    let imm20 = (offset / 2) as u32;
    let s = (imm20 >> 19) & 1;
    let j2 = (imm20 >> 18) & 1;
    let j1 = (imm20 >> 17) & 1;
    let imm6 = (imm20 >> 11) & 0x3F;
    let imm11 = imm20 & 0x7FF;
    let word = 0xF000_8000 | (s << 26) | ((cond.as_operand_bits() as u32) << 22) | (imm6 << 16) | (j1 << 13) | (j2 << 11) | imm11;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_branch_wide_conditional(word: u32) -> (/*cond*/u8, /*offset*/i32) {
    let s = (word >> 26) & 1;
    let cond = ((word >> 22) & 0b1111) as u8;
    let imm6 = (word >> 16) & 0x3F;
    let j1 = (word >> 13) & 1;
    let j2 = (word >> 11) & 1;
    let imm11 = word & 0x7FF;
    let imm20 = (s << 19) | (j2 << 18) | (j1 << 17) | (imm6 << 11) | imm11;
    let offset = sign_extension_utils::sign_extend_int_to_i32(imm20 as i32, 20) * 2;
    (cond, offset)
}

// CBZ / CBNZ (T1, 16-bit): forward branch only, 0..=126, even. imm32 = ZeroExtend(i:imm5:'0'); Rn is low.
fn encode_compare_branch(base: u32, rn: &Arm32LowGeneralPurposeRegister, offset: u8) -> Result<Vec<u16>, EncodeError> {
    if !offset.is_multiple_of(2) {
        return Err(EncodeError::ImmediateNotAligned { field: "offset", value: offset as i64, required_multiple: 2 });
    }
    if offset > 126 {
        return Err(EncodeError::ImmediateOutOfRange { field: "offset", value: offset as i64, minimum: 0, maximum: 126 });
    }
    let i = ((offset >> 6) & 1) as u32;
    let imm5 = ((offset >> 1) & 0x1F) as u32;
    let halfword = (base | (i << 9) | (imm5 << 3) | (rn.as_operand_bits() as u32)) as u16;
    Ok(vec![halfword])
}

fn decode_halfword_compare_branch(halfword: u16) -> (/*rn*/u8, /*offset*/u8) {
    let i = ((halfword >> 9) & 1) as u8;
    let imm5 = ((halfword >> 3) & 0x1F) as u8;
    let rn = (halfword & 0b111) as u8;
    let offset = (i << 6) | (imm5 << 1);
    (rn, offset)
}

// ---- ARMv7E-M DSP M8a: saturating arithmetic (`Rd, Rm, Rn`): base | Rn<<16 | Rd<<8 | Rm. ----
fn encode_saturating_arithmetic(base: u32, rd: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rm", rm)?;
    check_general_register_is_encodable("rn", rn)?;
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// ---- ARMv7E-M DSP M8b: extend-and-add (`Rd, Rn, Rm{, ROR #r}`): base | Rn<<16 | Rd<<8 | (rotation/8)<<4 | Rm. ----
fn encode_extend_and_add(base: u32, rd: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, rotation: u8) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_register_is_not_pc("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    if !rotation.is_multiple_of(8) {
        return Err(EncodeError::ImmediateNotAligned { field: "rotation", value: rotation as i64, required_multiple: 8 });
    }
    if rotation > 24 {
        return Err(EncodeError::ImmediateOutOfRange { field: "rotation", value: rotation as i64, minimum: 0, maximum: 24 });
    }
    let rotate = (rotation / 8) as u32;
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (rotate << 4) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_extend_and_add(word: u32) -> (/*rd*/u8, /*rn*/u8, /*rm*/u8, /*rotation*/u8) {
    let rn = ((word >> 16) & 0b1111) as u8;
    let rd = ((word >> 8) & 0b1111) as u8;
    let rotation = (((word >> 4) & 0b11) as u8) * 8;
    let rm = (word & 0b1111) as u8;
    (rd, rn, rm, rotation)
}

// ---- ARMv7E-M DSP M8c ----

// PKHBT/PKHTB (`Rd, Rn, Rm{, LSL/ASR #amount}`): 0xEAC00000 | Rn<<16 | imm3<<12 | Rd<<8 | imm2<<6 | tb<<5 | Rm.
// `tb` selects PKHTB (ASR, tb=1) vs PKHBT (LSL, tb=0); the amount fills imm3:imm2.
fn encode_pack_halfword(rd: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, amount: u8, tb: bool) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    if amount > 31 {
        return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: amount as i64, minimum: 0, maximum: 31 });
    }
    let imm3 = ((amount >> 2) & 0b111) as u32;
    let imm2 = (amount & 0b11) as u32;
    let word = 0xEAC0_0000 | ((rn.as_operand_bits() as u32) << 16) | (imm3 << 12) | ((rd.as_operand_bits() as u32) << 8) | (imm2 << 6) | ((tb as u32) << 5) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_pack_halfword(word: u32) -> (/*rd*/u8, /*rn*/u8, /*rm*/u8, /*amount*/u8, /*tb*/bool) {
    let rn = ((word >> 16) & 0b1111) as u8;
    let rd = ((word >> 8) & 0b1111) as u8;
    let rm = (word & 0b1111) as u8;
    let imm3 = (word >> 12) & 0b111;
    let imm2 = (word >> 6) & 0b11;
    let amount = ((imm3 << 2) | imm2) as u8;
    let tb = (word >> 5) & 1 == 1;
    (rd, rn, rm, amount, tb)
}

// SSAT16 / USAT16 (`Rd, #sat, Rn`, no shift): base | Rn<<16 | Rd<<8 | sat_field.
fn encode_saturate16(base: u32, is_usat: bool, rd: &Arm32GeneralPurposeRegister, sat_imm: u8, rn: &Arm32GeneralPurposeRegister) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rn", rn)?;
    let sat_field = if is_usat {
        if sat_imm > 15 {
            return Err(EncodeError::ImmediateOutOfRange { field: "sat_imm", value: sat_imm as i64, minimum: 0, maximum: 15 });
        }
        sat_imm as u32
    } else {
        if !(1..=16).contains(&sat_imm) {
            return Err(EncodeError::ImmediateOutOfRange { field: "sat_imm", value: sat_imm as i64, minimum: 1, maximum: 16 });
        }
        (sat_imm - 1) as u32
    };
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | sat_field;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// ---- ARMv7E-M DSP M8e: signed multiplies. The op2 [7:4] nibble selects the BB/BT/TB/TT half (n:m), the
// cross (x), or the round (r) bit; helpers below splice it with the register fields. ----
fn nm_nibble(n: bool, m: bool) -> u32 {
    ((n as u32) << 1) | (m as u32)
}

// `Rd, Rn, Rm` with the Ra field SBO=1111 (SMUL*/SMULW/SMUAD/SMUSD/SMMUL): base | Rn<<16 | Rd<<8 | nibble<<4 | Rm.
fn encode_signed_multiply(base: u32, rd: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, nibble: u32) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rd.as_operand_bits() as u32) << 8) | (nibble << 4) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// `Rd, Rn, Rm, Ra` (SMLA*/SMLAW/SMLAD/SMLSD/SMMLA/SMMLS): base | Rn<<16 | Ra<<12 | Rd<<8 | nibble<<4 | Rm.
fn encode_signed_multiply_accumulate(base: u32, rd: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, ra: &Arm32GeneralPurposeRegister, nibble: u32) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    check_general_register_is_encodable("ra", ra)?;
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((ra.as_operand_bits() as u32) << 12) | ((rd.as_operand_bits() as u32) << 8) | (nibble << 4) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// `RdLo, RdHi, Rn, Rm` (SMLAL<x><y>/SMLALD/SMLSLD): base | Rn<<16 | RdLo<<12 | RdHi<<8 | nibble<<4 | Rm.
fn encode_signed_multiply_long(base: u32, rdlo: &Arm32GeneralPurposeRegister, rdhi: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, nibble: u32) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rdlo", rdlo)?;
    check_general_register_is_encodable("rdhi", rdhi)?;
    check_general_register_is_encodable("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rdlo.as_operand_bits() as u32) << 12) | ((rdhi.as_operand_bits() as u32) << 8) | (nibble << 4) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// ---- ARMv7E-M FP M8f: load/store (`Vd, [Rn, #+/-(imm8*4)]`): base | U<<23 | D<<22 | Rn<<16 | Vd<<12 | imm8. ----
fn encode_fp_load_store(base: u32, vd_field: u32, d_bit: u32, rn: &Arm32GeneralPurposeRegister, offset: i32) -> Result<Vec<u16>, EncodeError> {
    check_register_is_not_pc("rn", rn)?;
    if offset % 4 != 0 {
        return Err(EncodeError::ImmediateNotAligned { field: "offset", value: offset as i64, required_multiple: 4 });
    }
    if !(-1020..=1020).contains(&offset) {
        return Err(EncodeError::ImmediateOutOfRange { field: "offset", value: offset as i64, minimum: -1020, maximum: 1020 });
    }
    let u = if offset >= 0 { 1u32 } else { 0 };
    let imm8 = offset.unsigned_abs() / 4;
    let word = base | (u << 23) | (d_bit << 22) | ((rn.as_operand_bits() as u32) << 16) | (vd_field << 12) | imm8;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_fp_load_store(word: u32) -> (/*vd_field*/u32, /*d_bit*/u32, /*rn*/u8, /*offset*/i32) {
    let u = (word >> 23) & 1;
    let d_bit = (word >> 22) & 1;
    let rn = ((word >> 16) & 0b1111) as u8;
    let vd_field = (word >> 12) & 0b1111;
    let imm8 = (word & 0xFF) as i32;
    let offset = if u == 1 { imm8 * 4 } else { -imm8 * 4 };
    (vd_field, d_bit, rn, offset)
}

// FP load/store multiple (`Rn{!}, {first..first+count-1}`): 0xEC000000 | size | P<<24 | U<<23 | D<<22 |
// W<<21 | L<<20 | Rn<<16 | Vd<<12 | imm8 (count for single, 2*count for double). IA is P=0,U=1; DB is P=1,U=0.
#[allow(clippy::too_many_arguments)]
fn encode_fp_load_store_multiple(size_low: u32, is_load: bool, rn: &Arm32GeneralPurposeRegister, writeback: bool, decrement_before: bool, vd_field: u32, d_bit: u32, count: u8, first_number: u8, max_register: u8, is_double: bool) -> Result<Vec<u16>, EncodeError> {
    check_register_is_not_pc("rn", rn)?;
    if count == 0 || (first_number as u32) + (count as u32) - 1 > max_register as u32 {
        return Err(EncodeError::ImmediateOutOfRange { field: "count", value: count as i64, minimum: 1, maximum: (max_register as i64) - (first_number as i64) + 1 });
    }
    if decrement_before && !writeback {
        return Err(EncodeError::RegisterNotEncodable { field: "rn", detail: "the decrement-before (DB) form requires writeback (!)" });
    }
    let p = if decrement_before { 1u32 } else { 0 };
    let u = if decrement_before { 0u32 } else { 1 };
    let imm8 = if is_double { (count as u32) * 2 } else { count as u32 };
    let word = 0xEC00_0000 | size_low | (p << 24) | (u << 23) | (d_bit << 22) | ((writeback as u32) << 21) | ((is_load as u32) << 20) | ((rn.as_operand_bits() as u32) << 16) | (vd_field << 12) | imm8;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_fp_load_store_multiple(word: u32) -> (/*rn*/u8, /*writeback*/bool, /*decrement_before*/bool, /*vd_field*/u32, /*d_bit*/u32, /*imm8*/u8) {
    let decrement_before = (word >> 24) & 1 == 1;
    let writeback = (word >> 21) & 1 == 1;
    let rn = ((word >> 16) & 0b1111) as u8;
    let d_bit = (word >> 22) & 1;
    let vd_field = (word >> 12) & 0b1111;
    let imm8 = (word & 0xFF) as u8;
    (rn, writeback, decrement_before, vd_field, d_bit, imm8)
}

// VMOV between a core register pair and a double / two consecutive singles: base | op<<20 | Rt2<<16 |
// Rt<<12 | M<<5 | Vm. `fp_to_core` is the op bit (1 = FP -> core).
fn encode_vmov_core_pair(base: u32, fp_to_core: bool, rt: &Arm32GeneralPurposeRegister, rt2: &Arm32GeneralPurposeRegister, vm_field: u32, m_bit: u32) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rt", rt)?;
    check_general_register_is_encodable("rt2", rt2)?;
    let word = base | ((fp_to_core as u32) << 20) | ((rt2.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | (m_bit << 5) | vm_field;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// Fixed-point VCVT (Vd is also the source): 0xEEBA0A40 | op<<18 | U<<16 | D<<22 | Vd<<12 | sf<<8 | sx<<7 |
// i<<5 | imm4, where the imm5 field (imm4:i) = size - frac_bits (size = 32 if `bits32` else 16).
fn encode_vcvt_fixed(vd_field: u32, d_bit: u32, sf: u32, to_fixed: bool, signed: bool, bits32: bool, frac_bits: u8) -> Result<Vec<u16>, EncodeError> {
    let size: u32 = if bits32 { 32 } else { 16 };
    if frac_bits < 1 || frac_bits as u32 > size {
        return Err(EncodeError::ImmediateOutOfRange { field: "frac_bits", value: frac_bits as i64, minimum: 1, maximum: size as i64 });
    }
    let imm5 = size - frac_bits as u32;
    let imm4 = (imm5 >> 1) & 0xF;
    let i = imm5 & 1;
    let op = if to_fixed { 1u32 } else { 0 };
    let u = if signed { 0u32 } else { 1 };
    let word = 0xEEBA_0A40 | (op << 18) | (u << 16) | (d_bit << 22) | (vd_field << 12) | (sf << 8) | ((bits32 as u32) << 7) | (i << 5) | imm4;
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

fn decode_word_vcvt_fixed(word: u32) -> Option<(/*signed*/bool, /*bits32*/bool, /*frac_bits*/u8)> {
    let signed = (word >> 16) & 1 == 0;
    let bits32 = (word >> 7) & 1 == 1;
    let size: u32 = if bits32 { 32 } else { 16 };
    let imm5 = ((word & 0xF) << 1) | ((word >> 5) & 1);
    // frac_bits = size - imm5 must be in 1..=size; imm5 >= size is an invalid (non-positive fraction) encoding.
    if imm5 >= size { return None; }
    Some((signed, bits32, (size - imm5) as u8))
}

// USADA8 (`Rd, Rn, Rm, Ra`): 0xFB700000 | Rn<<16 | Ra<<12 | Rd<<8 | Rm. (USAD8 is the Ra==1111 case.)
fn encode_usada8(rd: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, ra: &Arm32GeneralPurposeRegister) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rd", rd)?;
    check_general_register_is_encodable("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    check_general_register_is_encodable("ra", ra)?;
    let word = 0xFB70_0000 | ((rn.as_operand_bits() as u32) << 16) | ((ra.as_operand_bits() as u32) << 12) | ((rd.as_operand_bits() as u32) << 8) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// Wide load/store with a register offset (`Rt, [Rn, Rm, LSL #lsl]`): base | Rn<<16 | Rt<<12 | lsl<<4 | Rm.
fn encode_load_store_register(base: u32, rt: &Arm32GeneralPurposeRegister, rn: &Arm32GeneralPurposeRegister, rm: &Arm32GeneralPurposeRegister, lsl: u8) -> Result<Vec<u16>, EncodeError> {
    check_general_register_is_encodable("rt", rt)?;
    check_register_is_not_pc("rn", rn)?;
    check_general_register_is_encodable("rm", rm)?;
    if lsl > 3 {
        return Err(EncodeError::ImmediateOutOfRange { field: "shift", value: lsl as i64, minimum: 0, maximum: 3 });
    }
    let word = base | ((rn.as_operand_bits() as u32) << 16) | ((rt.as_operand_bits() as u32) << 12) | ((lsl as u32) << 4) | (rm.as_operand_bits() as u32);
    Ok(split_instruction_word_into_halfwords(word).to_vec())
}

// Pack lsb (5 bits) into the imm3:imm2 field positions (bits 14:12 and 7:6).
fn bitfield_lsb_field_bits(lsb: u8) -> u32 {
    let imm3 = ((lsb >> 2) & 0b111) as u32;
    let imm2 = (lsb & 0b11) as u32;
    (imm3 << 12) | (imm2 << 6)
}

fn split_instruction_word_into_halfwords(word: u32) -> [u16; 2] {
    let halfword0 = ((word >> 16) & 0xFFFF) as u16;
    let halfword1 = (word & 0xFFFF) as u16;

    [halfword0, halfword1]
}

// ---- VCX (CDE FP/vector custom-datapath) register + immediate field helpers ----
// `kind`: 0 = single (Sx, num = Vx:X), 1 = double (Dx, num = X:Vx), 2 = vector (Qx, a 3-bit Q). The three
// placement helpers put a register number into: `d` = destination ([15:12]+D[22] or Qd[15:13]); `lo` = the
// low source ([3:0]+[5] or Q[3:1], used by VCX2's Rn and VCX3's Rm); `hi` = the high source ([19:16]+[7]
// or Q[19:17], used by VCX3's Rn). The `enc`/`dec` pairs are exact inverses.
fn vcx_enc_d(kind: u8, num: u8) -> u32 {
    let n = num as u32;
    match kind { 2 => (n & 7) << 13, 0 => ((n >> 1) << 12) | ((n & 1) << 22), _ => ((n & 0xF) << 12) | (((n >> 4) & 1) << 22) }
}
fn vcx_dec_d(kind: u8, w: u32) -> u8 {
    (match kind { 2 => (w >> 13) & 7, 0 => (((w >> 12) & 0xF) << 1) | ((w >> 22) & 1), _ => ((w >> 12) & 0xF) | (((w >> 22) & 1) << 4) }) as u8
}
fn vcx_enc_lo(kind: u8, num: u8) -> u32 {
    let n = num as u32;
    match kind { 2 => (n & 7) << 1, 0 => (n >> 1) | ((n & 1) << 5), _ => (n & 0xF) | (((n >> 4) & 1) << 5) }
}
fn vcx_dec_lo(kind: u8, w: u32) -> u8 {
    (match kind { 2 => (w >> 1) & 7, 0 => ((w & 0xF) << 1) | ((w >> 5) & 1), _ => (w & 0xF) | (((w >> 5) & 1) << 4) }) as u8
}
fn vcx_enc_hi(kind: u8, num: u8) -> u32 {
    let n = num as u32;
    match kind { 2 => (n & 7) << 17, 0 => ((n >> 1) << 16) | ((n & 1) << 7), _ => ((n & 0xF) << 16) | (((n >> 4) & 1) << 7) }
}
fn vcx_dec_hi(kind: u8, w: u32) -> u8 {
    (match kind { 2 => (w >> 17) & 7, 0 => (((w >> 16) & 0xF) << 1) | ((w >> 7) & 1), _ => ((w >> 16) & 0xF) | (((w >> 7) & 1) << 4) }) as u8
}
// VCX immediate scatter (arity-specific, identical for scalar and vector).
fn vcx_enc_imm1(imm: u16) -> u32 { let i = imm as u32; (i & 0x3F) | (((i >> 6) & 1) << 7) | (((i >> 7) & 0xF) << 16) }
fn vcx_dec_imm1(w: u32) -> u16 { ((w & 0x3F) | (((w >> 7) & 1) << 6) | (((w >> 16) & 0xF) << 7)) as u16 }
fn vcx_enc_imm2(imm: u8) -> u32 { let i = imm as u32; ((i & 1) << 4) | (((i >> 1) & 1) << 7) | (((i >> 2) & 0xF) << 16) }
fn vcx_dec_imm2(w: u32) -> u8 { (((w >> 4) & 1) | (((w >> 7) & 1) << 1) | (((w >> 16) & 0xF) << 2)) as u8 }
fn vcx_enc_imm3(imm: u8) -> u32 { let i = imm as u32; ((i & 1) << 4) | (((i >> 1) & 1) << 20) | (((i >> 2) & 1) << 21) }
fn vcx_dec_imm3(w: u32) -> u8 { (((w >> 4) & 1) | (((w >> 20) & 1) << 1) | (((w >> 21) & 1) << 2)) as u8 }

// MVE one-register modified immediate: `1110 1111 1 i 0 D 000 imm3 Qd cmode 0 0 op 1 imm4`. The imm8 seed
// is scattered as i=imm8[7] (bit28), imm3=imm8[6:4] (bits[18:16]), imm4=imm8[3:0] (bits[3:0]); the Q bit
// (6) is fixed 1 for these 128-bit forms. Base (cmode=op=imm8=0, Qd=0) is 0xEF80_0050.
fn encode_mve_modified_imm(cmode: u8, op: bool, imm8: u8, qd_field: u32) -> u32 {
    let i = ((imm8 as u32) >> 7) & 1;
    let imm3 = ((imm8 as u32) >> 4) & 0b111;
    let imm4 = (imm8 as u32) & 0xF;
    0xEF80_0050 | (i << 28) | (imm3 << 16) | (qd_field << 13) | ((cmode as u32 & 0xF) << 8) | ((op as u32) << 5) | imm4
}

