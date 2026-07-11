// Copyright (c) Scaleservers LLC

use crate::Arm32GeneralPurposeRegister;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32LowGeneralPurposeRegister {
    // low registers
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}
//
impl Arm32LowGeneralPurposeRegister {
    pub fn as_operand_bits(&self) -> u8 {
        match self {
            // low registers
            Self::R0 => 0b0000,
            Self::R1 => 0b0001,
            Self::R2 => 0b0010,
            Self::R3 => 0b0011,
            Self::R4 => 0b0100,
            Self::R5 => 0b0101,
            Self::R6 => 0b0110,
            Self::R7 => 0b0111,
        }
    }

    /// Map a 3-bit low-register field to its register. TOTAL: only the low three bits are significant
    /// (R0..R7 cover every value), so this never panics -- the decoder derives it from untrusted bytes.
    pub fn from_operand_bits(bits: u8) -> Self {
        match bits & 0b111 {
            // low registers
            0b000 => Self::R0,
            0b001 => Self::R1,
            0b010 => Self::R2,
            0b011 => Self::R3,
            0b100 => Self::R4,
            0b101 => Self::R5,
            0b110 => Self::R6,
            // `& 0b111` leaves 0b111 as the only remaining value; the arm is exhaustive.
            _ => Self::R7,
        }
    }

    pub fn as_general_purpose_register(&self) -> Arm32GeneralPurposeRegister {
        match self {
            // low registers
            Self::R0 => Arm32GeneralPurposeRegister::R0,
            Self::R1 => Arm32GeneralPurposeRegister::R1,
            Self::R2 => Arm32GeneralPurposeRegister::R2,
            Self::R3 => Arm32GeneralPurposeRegister::R3,
            Self::R4 => Arm32GeneralPurposeRegister::R4,
            Self::R5 => Arm32GeneralPurposeRegister::R5,
            Self::R6 => Arm32GeneralPurposeRegister::R6,
            Self::R7 => Arm32GeneralPurposeRegister::R7,
        }
    }
}
