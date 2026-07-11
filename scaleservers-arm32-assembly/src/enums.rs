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
