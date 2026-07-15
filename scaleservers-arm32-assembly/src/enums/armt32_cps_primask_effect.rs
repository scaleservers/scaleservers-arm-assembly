// Copyright (c) Scaleservers LLC

#[derive(Debug, PartialEq)]
pub enum ArmT32CpsPrimaskEffect {
    InterruptEnable,  // IE (CPSIE i)
    InterruptDisable, // ID (CPSID i)
}
//
impl ArmT32CpsPrimaskEffect {
    pub fn as_operand_bits(&self) -> u8 {
        match self {
            Self::InterruptEnable => 0b0,
            Self::InterruptDisable => 0b1,
        }
    }

    /// TOTAL: only the low bit is significant (0 = enable, 1 = disable), so this never panics.
    pub fn from_operand_bits(bits: u8) -> Self {
        match bits & 1 {
            0b0 => Self::InterruptEnable,
            _ => Self::InterruptDisable,
        }
    }
}
