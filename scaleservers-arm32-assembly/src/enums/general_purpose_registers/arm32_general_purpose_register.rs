// Copyright (c) Scaleservers LLC

use crate::enums::Arm32LowGeneralPurposeRegister;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32GeneralPurposeRegister {
    // low registers
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    //
    // high registers
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
//
impl Arm32GeneralPurposeRegister {
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
            //
            // high registers
            Self::R8 => 0b1000,
            Self::R9 => 0b1001,
            Self::R10 => 0b1010,
            Self::R11 => 0b1011,
            Self::R12 => 0b1100,
            Self::R13 => 0b1101,
            Self::R14 => 0b1110,
            Self::R15 => 0b1111,
        }
    }

    /// Map a 4-bit register field to its register. TOTAL: only the low four bits are significant (R0..R15
    /// cover every value), so this never panics -- important because the decoder and the emitter derive
    /// register numbers from untrusted instruction words (and some emitters compute a pair as `Rd+1`, which
    /// can momentarily exceed 15 on a degenerate/hostile encoding). Higher bits are ignored.
    pub fn from_operand_bits(bits: u8) -> Self {
        match bits & 0b1111 {
            // low registers
            0b0000 => Self::R0,
            0b0001 => Self::R1,
            0b0010 => Self::R2,
            0b0011 => Self::R3,
            0b0100 => Self::R4,
            0b0101 => Self::R5,
            0b0110 => Self::R6,
            0b0111 => Self::R7,
            //
            // high registers
            0b1000 => Self::R8,
            0b1001 => Self::R9,
            0b1010 => Self::R10,
            0b1011 => Self::R11,
            0b1100 => Self::R12,
            0b1101 => Self::R13,
            0b1110 => Self::R14,
            // `& 0b1111` makes 0b1111 the only remaining value; the arm is exhaustive.
            _ => Self::R15,
        }
    }

    //

    pub fn is_low_general_purpose_register(&self) -> bool {
        match self {
            // low registers
            Self::R0 |
            Self::R1 |
            Self::R2 |
            Self::R3 |
            Self::R4 |
            Self::R5 |
            Self::R6 |
            Self::R7 => true,
            //
            // high registers
            Self::R8 |
            Self::R9 |
            Self::R10 |
            Self::R11 |
            Self::R12 |
            Self::R13 |
            Self::R14 |
            Self::R15 => false,
        }
    }

    pub fn is_high_general_purpose_register(&self) -> bool {
        match self {
            // low registers
            Self::R0 |
            Self::R1 |
            Self::R2 |
            Self::R3 |
            Self::R4 |
            Self::R5 |
            Self::R6 |
            Self::R7 => false,
            //
            // high registers
            Self::R8 |
            Self::R9 |
            Self::R10 |
            Self::R11 |
            Self::R12 |
            Self::R13 |
            Self::R14 |
            Self::R15 => true,
        }
    }

    pub fn as_low_general_purpose_register(&self) -> Option<Arm32LowGeneralPurposeRegister> {
        match self {
            // low registers
            Self::R0 => Some(Arm32LowGeneralPurposeRegister::R0),
            Self::R1 => Some(Arm32LowGeneralPurposeRegister::R1),
            Self::R2 => Some(Arm32LowGeneralPurposeRegister::R2),
            Self::R3 => Some(Arm32LowGeneralPurposeRegister::R3),
            Self::R4 => Some(Arm32LowGeneralPurposeRegister::R4),
            Self::R5 => Some(Arm32LowGeneralPurposeRegister::R5),
            Self::R6 => Some(Arm32LowGeneralPurposeRegister::R6),
            Self::R7 => Some(Arm32LowGeneralPurposeRegister::R7),
            //
            // high registers
            Self::R8 |
            Self::R9 |
            Self::R10 |
            Self::R11 |
            Self::R12 |
            Self::R13 |
            Self::R14 |
            Self::R15 => None,
        }
    }
}
