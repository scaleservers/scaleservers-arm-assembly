// Copyright (c) Scaleservers LLC

mod general_purpose_registers {
    mod arm32_general_purpose_register;
    pub use arm32_general_purpose_register::Arm32GeneralPurposeRegister;

    mod arm32_low_general_purpose_register;
    pub use arm32_low_general_purpose_register::Arm32LowGeneralPurposeRegister;
}
pub use general_purpose_registers::{
    Arm32GeneralPurposeRegister,
    Arm32LowGeneralPurposeRegister,
};

mod floating_point_registers;
pub use floating_point_registers::{Arm32SinglePrecisionRegister, Arm32DoublePrecisionRegister, Arm32QuadwordRegister};

mod floating_point_data_operations;
pub use floating_point_data_operations::{ArmT32FpDataOperation3, ArmT32FpDataOperation2};

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
    Arm32NeonSize, Arm32NeonIntegerOp, Arm32NeonFloatOp, Arm32NeonBitwiseOp,
    Arm32NeonMisc2SizedOp, Arm32NeonMisc2FixedOp, Arm32NeonNarrowOp,
    Arm32NeonDiffLongOp, Arm32NeonDiffWideOp, Arm32NeonDiffNarrowOp,
    Arm32NeonScalarOp, Arm32NeonScalarLongOp,
    Arm32NeonShiftOp, Arm32NeonShiftNarrowOp,
    Arm32NeonAesOp, Arm32NeonSha3Op, Arm32NeonSha2Op,
};

mod arm32_neon_load_store_address;
pub use arm32_neon_load_store_address::Arm32NeonLoadStoreAddress;

mod arm32_mve_registers;
pub use arm32_mve_registers::Arm32MveVectorRegister;

mod arm32_vmov_lane_size;
pub use arm32_vmov_lane_size::Arm32VmovLaneSize;

mod arm32_mve_operations;
pub use arm32_mve_operations::{
    Arm32MveSize, Arm32MveFloatSize,
    Arm32MveIntArithOp, Arm32MveBitwiseOp, Arm32MveFloatArithOp,
    MVE_INT_SIGNATURE_MASK, MVE_BITWISE_SIGNATURE_MASK, MVE_FLOAT_SIGNATURE_MASK,
    Arm32MveVecScalarIntOp,
    MVE_VBS_INT_SIGNATURE_MASK, MVE_VBS_FLOAT_SIGNATURE_MASK,
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
pub use floating_point_data_operations::ArmT32FpDataOperation3 as Arm32FpDataOperation3;
pub use floating_point_data_operations::ArmT32FpDataOperation2 as Arm32FpDataOperation2;
// offset/pre-index/post-index is the same taxonomy in both sets (the A32 P/W bit mapping differs and is
// handled in the A32 encoder)
pub use armt32_index_mode::ArmT32IndexMode as Arm32IndexMode;
