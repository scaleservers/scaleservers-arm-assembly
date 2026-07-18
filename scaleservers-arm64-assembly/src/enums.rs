// Copyright (c) Scaleservers LLC

mod general_purpose_registers {
    mod arm64_general_purpose_register;
    pub use arm64_general_purpose_register::Arm64GeneralPurposeRegister;

    mod arm64_general_purpose_register_32;
    pub use arm64_general_purpose_register_32::Arm64GeneralPurposeRegister32;
}
pub use general_purpose_registers::{Arm64GeneralPurposeRegister, Arm64GeneralPurposeRegister32};

mod arm64_condition;
pub use arm64_condition::Arm64Condition;

mod arm64_register_width;
pub use arm64_register_width::Arm64RegisterWidth;

mod arm64_extend_option;
pub use arm64_extend_option::Arm64ExtendOption;

mod arm64_load_store_size;
pub use arm64_load_store_size::Arm64LoadStoreSize;

mod arm64_load_store_index;
pub use arm64_load_store_index::Arm64LoadStoreIndex;

mod arm64_memory_extend;
pub use arm64_memory_extend::Arm64MemoryExtend;

mod arm64_imm9_mode;
pub use arm64_imm9_mode::Arm64Imm9Mode;

mod arm64_atomic_ordering;
pub use arm64_atomic_ordering::Arm64AtomicOrdering;

mod arm64_atomic_op;
pub use arm64_atomic_op::Arm64AtomicOp;

mod arm64_cmp_branch_cond;
pub use arm64_cmp_branch_cond::{Arm64CmpBranchCond, Arm64CmpBranchImmCond};

mod arm64_lsui_atomic_op;
pub use arm64_lsui_atomic_op::Arm64LsuiAtomicOp;

mod arm64_lsui_pair_index;
pub use arm64_lsui_pair_index::Arm64LsuiPairIndex;

mod arm64_crc32_op;
pub use arm64_crc32_op::Arm64Crc32Op;

mod arm64_mte_data_op;
pub use arm64_mte_data_op::Arm64MteDataOp;

mod arm64_store_tag_op;
pub use arm64_store_tag_op::Arm64StoreTagOp;

mod arm64_pac_op;
pub use arm64_pac_op::Arm64PacOp;

mod arm64_pac_hint_op;
pub use arm64_pac_hint_op::{Arm64BtiTarget, Arm64PacHintOp};

mod arm64_pac_branch_op;
pub use arm64_pac_branch_op::{Arm64PacBranchOp, Arm64PacReturnOp};

mod arm64_vector_arrangement;
pub use arm64_vector_arrangement::Arm64VectorArrangement;

mod arm64_vector_int_three_same_op;
pub use arm64_vector_int_three_same_op::Arm64VectorIntThreeSameOp;

mod arm64_vector_fp_three_same_op;
pub use arm64_vector_fp_three_same_op::Arm64VectorFpThreeSameOp;

mod arm64_vector_bitwise_op;
pub use arm64_vector_bitwise_op::Arm64VectorBitwiseOp;

mod arm64_vector_int_unary_op;
pub use arm64_vector_int_unary_op::Arm64VectorIntUnaryOp;

mod arm64_vector_fp_unary_op;
pub use arm64_vector_fp_unary_op::Arm64VectorFpUnaryOp;

mod arm64_vector_shift_imm_op;
pub use arm64_vector_shift_imm_op::Arm64VectorShiftImmOp;

mod arm64_vector_load_store_size;
pub use arm64_vector_load_store_size::Arm64VectorLoadStoreSize;

mod arm64_vector_permute_op;
pub use arm64_vector_permute_op::Arm64VectorPermuteOp;

mod arm64_vector_three_different_op;
pub use arm64_vector_three_different_op::{
    Arm64VectorThreeDifferentOp, Arm64VectorThreeDifferentShape,
};

mod arm64_vector_shift_long_narrow_op;
pub use arm64_vector_shift_long_narrow_op::Arm64VectorShiftLongNarrowOp;

mod arm64_vector_narrow_op;
pub use arm64_vector_narrow_op::Arm64VectorNarrowOp;

mod arm64_vector_across_lanes_op;
pub use arm64_vector_across_lanes_op::Arm64VectorAcrossLanesOp;

mod arm64_vector_compare_zero_op;
pub use arm64_vector_compare_zero_op::Arm64VectorCompareZeroOp;

mod arm64_vector_modified_immediate;
pub use arm64_vector_modified_immediate::{
    Arm64VectorImmediateShift, Arm64VectorModifiedImmediateOp,
};

mod arm64_vector_by_element_op;
pub use arm64_vector_by_element_op::Arm64VectorByElementOp;

mod arm64_vector_by_element_long_op;
pub use arm64_vector_by_element_long_op::Arm64VectorByElementLongOp;

mod arm64_vector_structure_kind;
pub use arm64_vector_structure_kind::Arm64VectorStructureKind;

mod arm64_vector_add_pairwise_long_op;
pub use arm64_vector_add_pairwise_long_op::Arm64VectorAddPairwiseLongOp;

mod arm64_vector_fp_convert_length_op;
pub use arm64_vector_fp_convert_length_op::Arm64VectorFpConvertLengthOp;

mod arm64_vector_aes_op;
pub use arm64_vector_aes_op::Arm64VectorAesOp;

mod arm64_vector_sha_op;
pub use arm64_vector_sha_op::{Arm64ShaRegView, Arm64VectorSha2Op, Arm64VectorSha3Op};

mod arm64_vector_rdm_op;
pub use arm64_vector_rdm_op::Arm64VectorRdmOp;

mod arm64_complex_rotation;
pub use arm64_complex_rotation::Arm64ComplexRotation;

mod arm64_vector_i8mm_op;
pub use arm64_vector_i8mm_op::{Arm64VectorMatMulOp, Arm64VectorMixedDotOp};

mod arm64_vector_crypto_extra_op;
pub use arm64_vector_crypto_extra_op::{
    Arm64CryptoFamily, Arm64VectorCrypto2Op, Arm64VectorCrypto3Op, Arm64VectorCrypto4Op,
    Arm64VectorSm3TtOp,
};

mod arm64_scalar_three_same_op;
pub use arm64_scalar_three_same_op::Arm64ScalarThreeSameOp;

mod arm64_scalar_fp_three_same_op;
pub use arm64_scalar_fp_three_same_op::Arm64ScalarFpThreeSameOp;

mod arm64_scalar_two_misc_op;
pub use arm64_scalar_two_misc_op::Arm64ScalarTwoMiscOp;

mod arm64_scalar_fp_two_misc_op;
pub use arm64_scalar_fp_two_misc_op::Arm64ScalarFpTwoMiscOp;

mod arm64_scalar_narrow_op;
pub use arm64_scalar_narrow_op::Arm64ScalarNarrowOp;

mod arm64_scalar_shift_imm_op;
pub use arm64_scalar_shift_imm_op::Arm64ScalarShiftImmOp;

mod arm64_scalar_by_element_op;
pub use arm64_scalar_by_element_op::{Arm64ScalarByElementLongOp, Arm64ScalarByElementOp};

mod arm64_scalar_fp_pairwise_op;
pub use arm64_scalar_fp_pairwise_op::Arm64ScalarFpPairwiseOp;

mod arm64_scalar_shift_special_op;
pub use arm64_scalar_shift_special_op::{Arm64ScalarFixedConvertOp, Arm64ScalarShiftNarrowOp};

mod arm64_vector_fp16_two_misc_op;
pub use arm64_vector_fp16_two_misc_op::Arm64VectorFp16TwoMiscOp;

mod arm64_vector_fp16_indexed_op;
pub use arm64_vector_fp16_indexed_op::{Arm64VectorFp16AcrossOp, Arm64VectorFp16ByElementOp};

mod arm64_vector_element;
pub use arm64_vector_element::Arm64VectorElement;

mod arm64_barrier_option;
pub use arm64_barrier_option::Arm64BarrierOption;

mod arm64_system_hint_op;
pub use arm64_system_hint_op::Arm64SystemHintOp;

mod arm64_gcs_register_op;
pub use arm64_gcs_register_op::Arm64GcsRegisterOp;

mod arm64_gcs_exception_op;
pub use arm64_gcs_exception_op::Arm64GcsExceptionOp;

mod arm64_branch_record_buffer_op;
pub use arm64_branch_record_buffer_op::Arm64BranchRecordBufferOp;

mod arm64_rcpc_unscaled_op;
pub use arm64_rcpc_unscaled_op::Arm64RcpcUnscaledOp;

mod arm64_pstate_field;
pub use arm64_pstate_field::Arm64PstateField;

mod arm64_system_register;
pub use arm64_system_register::Arm64SystemRegister;

mod arm64_float_precision;
pub use arm64_float_precision::Arm64FloatPrecision;

mod arm64_float_register;
pub use arm64_float_register::Arm64FloatRegister;

mod arm64_fp8_convert_long_op;
pub use arm64_fp8_convert_long_op::Arm64Fp8ConvertLongOp;

mod arm64_fp_to_int_round_op;
pub use arm64_fp_to_int_round_op::Arm64FpToIntRoundOp;

mod arm64_sve_fp8_convert_op;
pub use arm64_sve_fp8_convert_op::Arm64SveFp8ConvertOp;

mod arm64_sve_fp8_narrow_op;
pub use arm64_sve_fp8_narrow_op::Arm64SveFp8NarrowOp;

mod arm64_scalable_vector_register;
pub use arm64_scalable_vector_register::Arm64ScalableVectorRegister;

mod arm64_predicate_register;
pub use arm64_predicate_register::Arm64PredicateRegister;

mod arm64_sve_int_bin_unpred_op;
pub use arm64_sve_int_bin_unpred_op::Arm64SveIntBinUnpredOp;

mod arm64_sve_while_op;
pub use arm64_sve_while_op::Arm64SveWhileOp;

mod arm64_sve_pred_int_bin_op;
pub use arm64_sve_pred_int_bin_op::Arm64SvePredIntBinOp;

mod arm64_sve_load_type;
pub use arm64_sve_load_type::Arm64SveContiguousLoadType;

mod arm64_sve_int_compare_op;
pub use arm64_sve_int_compare_op::Arm64SveIntCompareOp;

mod arm64_sve_fp_pred_bin_op;
pub use arm64_sve_fp_pred_bin_op::Arm64SveFpPredBinOp;

mod arm64_sve_fp_compare_op;
pub use arm64_sve_fp_compare_op::Arm64SveFpCompareOp;

mod arm64_sve_fp_bin_unpred_op;
pub use arm64_sve_fp_bin_unpred_op::Arm64SveFpBinUnpredOp;

mod arm64_sve_fp_unary_op;
pub use arm64_sve_fp_unary_op::Arm64SveFpUnaryOp;

mod arm64_sve_fp_fma_op;
pub use arm64_sve_fp_fma_op::Arm64SveFpFmaOp;

mod arm64_sve_index_operand;
pub use arm64_sve_index_operand::Arm64SveIndexOperand;

mod arm64_sve_pred_logical_op;
pub use arm64_sve_pred_logical_op::Arm64SvePredLogicalOp;

mod arm64_sve_pred_unary_op;
pub use arm64_sve_pred_unary_op::Arm64SvePredUnaryOp;

mod arm64_sve_reduction_op;
pub use arm64_sve_reduction_op::{Arm64SveFpReductionOp, Arm64SveIntReductionOp};

mod arm64_sve_shift_imm_op;
pub use arm64_sve_shift_imm_op::Arm64SveShiftImmOp;

mod arm64_sve_pred_shift_vector_op;
pub use arm64_sve_pred_shift_vector_op::Arm64SvePredShiftVectorOp;

mod arm64_sve_pred_count_op;
pub use arm64_sve_pred_count_op::Arm64SvePredCountOp;

mod arm64_sve_structure_count;
pub use arm64_sve_structure_count::Arm64SveStructureCount;

mod arm64_scalar_frintts_op;
pub use arm64_scalar_frintts_op::Arm64ScalarFrintTsOp;

mod arm64_vector_fmlal_op;
pub use arm64_vector_fmlal_op::Arm64VectorFmlalOp;

mod arm64_mte_block_op;
pub use arm64_mte_block_op::Arm64MteBlockOp;

mod arm64_rcw_atomic_op;
pub use arm64_rcw_atomic_op::Arm64RcwAtomicOp;

mod arm64_sve_bitwise_logical_op;
pub use arm64_sve_bitwise_logical_op::Arm64SveBitwiseLogicalOp;

mod arm64_sve_reverse_width;
pub use arm64_sve_reverse_width::Arm64SveReverseWidth;

mod arm64_sve_bitwise_imm_op;
pub use arm64_sve_bitwise_imm_op::Arm64SveBitwiseImmOp;

mod arm64_sve_fp_convert_kind;
pub use arm64_sve_fp_convert_kind::Arm64SveFpConvertKind;

mod arm64_sve_adr_mode;
pub use arm64_sve_adr_mode::Arm64SveAdrMode;

mod arm64_sve_fp_indexed_op;
pub use arm64_sve_fp_indexed_op::Arm64SveFpIndexedOp;

mod arm64_sve_int_indexed_op;
pub use arm64_sve_int_indexed_op::Arm64SveIntIndexedOp;

mod arm64_sve2_widening_op;
pub use arm64_sve2_widening_op::Arm64Sve2WideningOp;

mod arm64_sve2_ternary_logical_op;
pub use arm64_sve2_ternary_logical_op::Arm64Sve2TernaryLogicalOp;

mod arm64_sve2_mul_op;
pub use arm64_sve2_mul_op::Arm64Sve2MulOp;

mod arm64_sve2_narrow_high_op;
pub use arm64_sve2_narrow_high_op::Arm64Sve2NarrowHighOp;

mod arm64_sve2_extract_narrow_op;
pub use arm64_sve2_extract_narrow_op::Arm64Sve2ExtractNarrowOp;

mod arm64_sve2_narrow_shift_op;
pub use arm64_sve2_narrow_shift_op::Arm64Sve2NarrowShiftOp;

mod arm64_sve2_halving_op;
pub use arm64_sve2_halving_op::Arm64Sve2HalvingOp;

mod arm64_sve2_pairwise_op;
pub use arm64_sve2_pairwise_op::Arm64Sve2PairwiseOp;

mod arm64_sve2_shift_left_pred_op;
pub use arm64_sve2_shift_left_pred_op::Arm64Sve2ShiftLeftPredOp;

mod arm64_sve2_unary_pred_op;
pub use arm64_sve2_unary_pred_op::Arm64Sve2UnaryPredOp;

mod arm64_sve2_sat_addsub_op;
pub use arm64_sve2_sat_addsub_op::Arm64Sve2SatAddSubOp;

mod arm64_sve2_bit_permute_op;
pub use arm64_sve2_bit_permute_op::Arm64Sve2BitPermuteOp;

mod arm64_sve2_widen_indexed_op;
pub use arm64_sve2_widen_indexed_op::Arm64Sve2WidenIndexedOp;

mod arm64_sve2_fp_pairwise_op;
pub use arm64_sve2_fp_pairwise_op::Arm64Sve2FpPairwiseOp;

mod arm64_sve2_fp_updown_op;
pub use arm64_sve2_fp_updown_op::Arm64Sve2FpUpdownOp;

mod arm64_sve2_while_compare_op;
pub use arm64_sve2_while_compare_op::Arm64Sve2WhileCompareOp;

mod arm64_sve_matmul_op;
pub use arm64_sve_matmul_op::Arm64SveMatmulOp;

mod arm64_sve_dot_indexed_op;
pub use arm64_sve_dot_indexed_op::Arm64SveDotIndexedOp;

mod arm64_sve_crypto_op;
pub use arm64_sve_crypto_op::{Arm64SveCryptoBinaryOp, Arm64SveCryptoDestructiveOp};

mod arm64_sve_clamp_op;
pub use arm64_sve_clamp_op::Arm64SveClampOp;

mod arm64_sve_offset_mode;
pub use arm64_sve_offset_mode::Arm64SveOffsetMode;

mod arm64_sme_state_target;
pub use arm64_sme_state_target::Arm64SmeStateTarget;

mod arm64_sme_fp_precision;
pub use arm64_sme_fp_precision::Arm64SmeFpPrecision;

mod arm64_sme_tile_size;
pub use arm64_sme_tile_size::Arm64SmeTileSize;

mod arm64_mops_stage;
pub use arm64_mops_stage::Arm64MopsStage;

mod arm64_sme2_clamp_kind;
pub use arm64_sme2_clamp_kind::Arm64Sme2ClampKind;

mod arm64_sme2_minmax_op;
pub use arm64_sme2_minmax_op::Arm64Sme2MinMaxOp;

mod arm64_sme2_shiftmul_op;
pub use arm64_sme2_shiftmul_op::Arm64Sme2ShiftMulOp;

mod arm64_sme2_unary_op;
pub use arm64_sme2_unary_op::Arm64Sme2UnaryOp;

mod arm64_sme2_za_dot_op;
pub use arm64_sme2_za_dot_op::Arm64Sme2ZaDotOp;
mod arm64_sme2_za_vdot_op;
pub use arm64_sme2_za_vdot_op::Arm64Sme2ZaVdotOp;
mod arm64_sme2_za_mlal_op;
pub use arm64_sme2_za_mlal_op::{Arm64Sme2ZaMlalOp, Arm64Sme2ZaMlalWiden};
mod arm64_fprcvt_op;
pub use arm64_fprcvt_op::Arm64FprcvtOp;

mod arm64_sme_tmop_op;
pub use arm64_sme_tmop_op::Arm64SmeTmopOp;

mod arm64_sme_mop4_kind;
pub use arm64_sme_mop4_kind::{Arm64SmeMop4DoubleKind, Arm64SmeMop4Kind};

mod arm64_pointer_auth_lr_op;
pub use arm64_pointer_auth_lr_op::Arm64PointerAuthLrOp;

#[cfg(feature = "experimental")]
mod arm64_pointer_auth_lr_label_op;
#[cfg(feature = "experimental")]
pub use arm64_pointer_auth_lr_label_op::Arm64PointerAuthLrLabelOp;
#[cfg(feature = "experimental")]
mod arm64_lsfe_op;
#[cfg(feature = "experimental")]
pub use arm64_lsfe_op::Arm64LsfeOp;

mod arm64_predicate_as_counter;
pub use arm64_predicate_as_counter::Arm64PredicateAsCounter;

mod arm64_while_counter_op;
pub use arm64_while_counter_op::Arm64WhileCounterOp;

mod arm64_sve_quad_permute_op;
pub use arm64_sve_quad_permute_op::Arm64SveQuadPermuteOp;

mod arm64_sve_bf16_binary_op;
pub use arm64_sve_bf16_binary_op::Arm64SveBf16BinaryOp;

mod arm64_sve_quad_reduce_int_op;
pub use arm64_sve_quad_reduce_int_op::Arm64SveQuadReduceIntOp;

mod arm64_sve_quad_reduce_fp_op;
pub use arm64_sve_quad_reduce_fp_op::Arm64SveQuadReduceFpOp;

mod arm64_sve_narrow_convert_op;
pub use arm64_sve_narrow_convert_op::Arm64SveNarrowConvertOp;

mod arm64_sve_shift_narrow_op;
pub use arm64_sve_shift_narrow_op::Arm64SveShiftNarrowOp;

mod arm64_sme2_fp_cvt_narrow_op;
pub use arm64_sme2_fp_cvt_narrow_op::Arm64Sme2FpCvtNarrowOp;

mod arm64_cssc_unary_op;
pub use arm64_cssc_unary_op::Arm64CsscUnaryOp;

mod arm64_cssc_min_max_op;
pub use arm64_cssc_min_max_op::Arm64CsscMinMaxOp;

mod arm64_lse128_op;
pub use arm64_lse128_op::Arm64Lse128Op;

mod arm64_sve_int_mac_op;
pub use arm64_sve_int_mac_op::Arm64SveIntMacOp;

mod arm64_sve_cmp_imm_op;
pub use arm64_sve_cmp_imm_op::{Arm64SveCmpImmSignedOp, Arm64SveCmpImmUnsignedOp};
