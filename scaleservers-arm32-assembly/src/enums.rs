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
