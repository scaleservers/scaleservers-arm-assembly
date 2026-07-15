// Copyright (c) Scaleservers LLC

mod general_purpose_registers {
    mod arm32_general_purpose_register;
    pub use arm32_general_purpose_register::Arm32GeneralPurposeRegister;

    mod arm32_low_general_purpose_register;
    pub use arm32_low_general_purpose_register::Arm32LowGeneralPurposeRegister;
}
pub use general_purpose_registers::{Arm32GeneralPurposeRegister, Arm32LowGeneralPurposeRegister};

mod floating_point_registers;
pub use floating_point_registers::{
    Arm32DoublePrecisionRegister, Arm32QuadwordRegister, Arm32SinglePrecisionRegister,
};

mod floating_point_data_operations;
pub use floating_point_data_operations::{ArmT32FpDataOperation2, ArmT32FpDataOperation3};

mod armt32_cps_primask_effect;
pub use armt32_cps_primask_effect::ArmT32CpsPrimaskEffect;

mod armt32_index_mode;
pub use armt32_index_mode::ArmT32IndexMode;

mod armt32_instruction_condition;
pub use armt32_instruction_condition::ArmT32InstructionCondition;

mod armt32_memory_barrier_option;
pub use armt32_memory_barrier_option::ArmT32MemoryBarrierOption;

mod armt32_parallel_arithmetic;
pub use armt32_parallel_arithmetic::{ArmT32ParallelOperation, ArmT32ParallelPrefix};

mod armt32_opcode_pattern_16bit;
pub use armt32_opcode_pattern_16bit::ArmT32OpcodePattern_16Bit;

mod armt32_opcode_pattern_32bit;
pub use armt32_opcode_pattern_32bit::ArmT32OpcodePattern_32Bit;

mod armt32_register_shift;
pub use armt32_register_shift::ArmT32RegisterShift;

mod armt32_special_register;
pub use armt32_special_register::ArmT32SpecialRegister;

mod arm32_shift_type;
pub use arm32_shift_type::Arm32ShiftType;

mod arm32_extend_type;
pub use arm32_extend_type::Arm32ExtendType;

mod arm32_memory_offset;
pub use arm32_memory_offset::{Arm32MemoryOffset, Arm32MemoryOffset8};

mod arm32_block_address_mode;
pub use arm32_block_address_mode::Arm32BlockAddressMode;

mod arm32_cps_mode;
pub use arm32_cps_mode::Arm32CpsMode;

mod arm32_vsel_condition;
pub use arm32_vsel_condition::Arm32VselCondition;

mod arm32_directed_round;
pub use arm32_directed_round::Arm32DirectedRound;

mod arm32_vrint_mode;
pub use arm32_vrint_mode::Arm32VrintMode;

mod arm32_neon_operations;
pub use arm32_neon_operations::{
    Arm32NeonAesOp, Arm32NeonBitwiseOp, Arm32NeonDiffLongOp, Arm32NeonDiffNarrowOp,
    Arm32NeonDiffWideOp, Arm32NeonFloatOp, Arm32NeonIntegerOp, Arm32NeonMisc2FixedOp,
    Arm32NeonMisc2SizedOp, Arm32NeonNarrowOp, Arm32NeonScalarLongOp, Arm32NeonScalarOp,
    Arm32NeonSha2Op, Arm32NeonSha3Op, Arm32NeonShiftNarrowOp, Arm32NeonShiftOp, Arm32NeonSize,
};

mod arm32_neon_load_store_address;
pub use arm32_neon_load_store_address::Arm32NeonLoadStoreAddress;

mod arm32_mve_registers;
pub use arm32_mve_registers::Arm32MveVectorRegister;

mod arm32_vmov_lane_size;
pub use arm32_vmov_lane_size::Arm32VmovLaneSize;

mod arm32_mve_operations;
pub use arm32_mve_operations::{
    Arm32MveBitwiseOp, Arm32MveFloatArithOp, Arm32MveFloatReduceOp, Arm32MveFloatSize,
    Arm32MveIntArithOp, Arm32MveLongMacOp, Arm32MveMisc2FloatOp, Arm32MveMisc2Op,
    Arm32MveQMovnKind, Arm32MveReduceOp, Arm32MveShiftImmOp, Arm32MveShiftNarrowOp, Arm32MveSize,
    Arm32MveVcmpCondition, Arm32MveVecScalarFloatOp, Arm32MveVecScalarIntOp, Arm32MveVrintOp,
    MVE_BITWISE_SIGNATURE_MASK, MVE_DUALMAC_BASE, MVE_DUALMAC_MASK,
    MVE_FLOAT_REDUCE_SIGNATURE_MASK, MVE_FLOAT_SIGNATURE_MASK, MVE_GATHER_SCATTER_BASE,
    MVE_GATHER_SCATTER_MASK, MVE_GATHER_VBASE_BASE, MVE_GATHER_VBASE_MASK, MVE_INT_SIGNATURE_MASK,
    MVE_INTERLEAVE_BASE, MVE_INTERLEAVE_MASK, MVE_LCTP_WORD, MVE_LOB_DLS_BASE, MVE_LOB_DLS_MASK,
    MVE_LONG_DUALMAC_BASE, MVE_LONG_DUALMAC_MASK, MVE_MISC2_SIGNATURE_MASK,
    MVE_REDUCE_SIGNATURE_MASK, MVE_SHIFT_NARROW_BASE, MVE_SHIFT_NARROW_MASK, MVE_SHIFT_SCALAR_BASE,
    MVE_SHIFT_SCALAR_MASK, MVE_SHIFT_SIGNATURE_MASK, MVE_SHIFT_VEC_BASE, MVE_SHIFT_VEC_MASK,
    MVE_VABAV_BASE, MVE_VABAV_SIGNATURE_MASK, MVE_VADC_BASE, MVE_VADC_MASK, MVE_VADDLV_BASE,
    MVE_VADDLV_MASK, MVE_VBRSR_BASE, MVE_VBRSR_MASK, MVE_VBS_FLOAT_SIGNATURE_MASK,
    MVE_VBS_INT_SIGNATURE_MASK, MVE_VCADD_FLOAT_MASK, MVE_VCADD_FLOAT_PATTERN, MVE_VCADD_INT_MASK,
    MVE_VCADD_INT_PATTERN, MVE_VCMLA_MASK, MVE_VCMLA_PATTERN, MVE_VCMP_FLOAT_BASE,
    MVE_VCMP_FLOAT_MASK, MVE_VCMP_INT_BASE, MVE_VCMP_INT_MASK, MVE_VCMUL_MASK, MVE_VCMUL_PATTERN,
    MVE_VCTP_BASE, MVE_VCTP_MASK, MVE_VCVT_FI_BASE, MVE_VCVT_FI_FIXED_MASK,
    MVE_VCVT_FI_FIXED_PATTERN, MVE_VCVT_FIXED_BASE, MVE_VCVT_FIXED_MASK, MVE_VCVT_HALF_BASE,
    MVE_VCVT_HALF_MASK, MVE_VCVTR_BASE, MVE_VCVTR_MASK, MVE_VDUP_BASE, MVE_VDUP_MASK,
    MVE_VIDDUP_BASE, MVE_VIDDUP_MASK, MVE_VMOVL_BASE, MVE_VMOVL_MASK, MVE_VMOVN_BASE,
    MVE_VMOVN_MASK, MVE_VMOVX_BASE, MVE_VMOVX_MASK, MVE_VMULH_BASE, MVE_VMULH_MASK,
    MVE_VMULL_INT_BASE, MVE_VMULL_INT_MASK, MVE_VMULL_POLY_BASE, MVE_VMULL_POLY_MASK,
    MVE_VMVN_REG_BASE, MVE_VMVN_REG_MASK, MVE_VPNOT_WORD, MVE_VPSEL_BASE, MVE_VPSEL_MASK,
    MVE_VPST_NOT_BASE, MVE_VPST_NOT_MASK, MVE_VQDMLADH_BASE, MVE_VQDMLADH_MASK,
    MVE_VQDMULL_SCALAR_BASE, MVE_VQDMULL_SCALAR_MASK, MVE_VQDMULL_VEC_BASE, MVE_VQDMULL_VEC_MASK,
    MVE_VQMOVN_BASE, MVE_VQMOVN_MASK, MVE_VQMOVUN_BASE, MVE_VQMOVUN_MASK, MVE_VSHLC_BASE,
    MVE_VSHLC_MASK, MVE_VSHLL_T1_BASE, MVE_VSHLL_T1_MASK, MVE_VSHLL_T2_BASE, MVE_VSHLL_T2_MASK,
    lob_branch_hw1, lob_branch_offset, lob_size_field, lob_size_from_field,
    mve_dualmac_decode_size, mve_dualmac_size_bits, mve_long_dualmac_bits, mve_long_dualmac_decode,
    mve_mem_size_from_log, mve_mem_size_log, mve_misc2_float_size_bits,
    mve_misc2_float_size_from_bits, mve_predicate_mask_bits, mve_predicate_mask_from_suffix,
    mve_predicate_mask_from_word, mve_predicate_mask_suffix, mve_shift_esize,
    mve_shift_size_from_imm6, mve_vcmp_fc_bits, mve_vcmp_fc_from_word, mve_vdup_size_bits,
    mve_vdup_size_from_bits,
};

// ---- neutral ARM-wide aliases ----
// The 4-bit condition code and the barrel-shift operand are identical in the A32 and T32 instruction sets,
// so they are also exposed under neutral `Arm32*` names for use by `ArmA32Instruction`. The historical
// `ArmT32*` names are retained for the Thumb side. (`Arm32GeneralPurposeRegister` is already neutral.)
pub use armt32_instruction_condition::ArmT32InstructionCondition as Arm32Condition;
pub use armt32_register_shift::ArmT32RegisterShift as Arm32RegisterShift;
// the packed-SIMD operation/prefix taxonomy is identical in A32 and T32 (only the field positions differ)
pub use armt32_parallel_arithmetic::ArmT32ParallelOperation as Arm32ParallelOperation;
pub use armt32_parallel_arithmetic::ArmT32ParallelPrefix as Arm32ParallelPrefix;
// the VFP data-processing operation taxonomy is identical in A32 and T32
pub use floating_point_data_operations::ArmT32FpDataOperation2 as Arm32FpDataOperation2;
pub use floating_point_data_operations::ArmT32FpDataOperation3 as Arm32FpDataOperation3;
// offset/pre-index/post-index is the same taxonomy in both sets (the A32 P/W bit mapping differs and is
// handled in the A32 encoder)
pub use armt32_index_mode::ArmT32IndexMode as Arm32IndexMode;
