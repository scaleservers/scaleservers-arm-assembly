// Copyright (c) Scaleservers LLC

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmT32InstructionCondition {
    Equal,                      // EQ
    NotEqual,                   // NE
    CarrySet,                   // CS (HS)
    CarryClear,                 // CC (LO)
    MinusNegative,              // MI
    PlusPositiveOrZero,         // PL
    Overflow,                   // VS
    NoOverflow,                 // VC
    UnsignedHigher,             // HI
    UnsignedLowerOrSame,        // LS
    SignedGreaterThanOrEqual,   // GE
    SignedLessThan,             // LT
    SignedGreaterThan,          // GT
    SignedLessThanOrEqual,      // LE
    AlwaysUnconditional,        // AL (None)
    Undefined(u8)
}
//
impl ArmT32InstructionCondition {
    pub fn as_operand_bits(&self) -> u8 {
        match self {
            Self::Equal                     => 0b0000, // EQ
            Self::NotEqual                  => 0b0001, // NE
            Self::CarrySet                  => 0b0010, // CS (HS)
            Self::CarryClear                => 0b0011, // CC (LO)
            Self::MinusNegative             => 0b0100, // MI
            Self::PlusPositiveOrZero        => 0b0101, // PL
            Self::Overflow                  => 0b0110, // VS
            Self::NoOverflow                => 0b0111, // VC
            Self::UnsignedHigher            => 0b1000, // HI
            Self::UnsignedLowerOrSame       => 0b1001, // LS
            Self::SignedGreaterThanOrEqual  => 0b1010, // GE
            Self::SignedLessThan            => 0b1011, // LT
            Self::SignedGreaterThan         => 0b1100, // GT
            Self::SignedLessThanOrEqual     => 0b1101, // LE
            Self::AlwaysUnconditional       => 0b1110, // AL (None)
            Self::Undefined(bits)      => {
                assert!(*bits == 0b1111, "Member Undefined's field 'value' is out of the valid range (0b1111..=0b1111)");

                *bits
            }
        }
    }

    pub fn from_operand_bits(bits: u8) -> Self {
        match bits {
            0b0000 => Self::Equal,                      // EQ
            0b0001 => Self::NotEqual,                   // NE
            0b0010 => Self::CarrySet,                   // CS (HS)
            0b0011 => Self::CarryClear,                 // CC (LO)
            0b0100 => Self::MinusNegative,              // MI
            0b0101 => Self::PlusPositiveOrZero,         // PL
            0b0110 => Self::Overflow,                   // VS
            0b0111 => Self::NoOverflow,                 // VC
            0b1000 => Self::UnsignedHigher,             // HI
            0b1001 => Self::UnsignedLowerOrSame,        // LS
            0b1010 => Self::SignedGreaterThanOrEqual,   // GE
            0b1011 => Self::SignedLessThan,             // LT
            0b1100 => Self::SignedGreaterThan,          // GT
            0b1101 => Self::SignedLessThanOrEqual,      // LE
            0b1110 => Self::AlwaysUnconditional,        // AL (None)
            0b1111 => Self::Undefined(0b1111),
            // TOTAL: only the low four bits are significant; higher bits map to an Undefined condition rather
            // than panicking (the decoder derives the condition from untrusted instruction bytes).
            _ => Self::Undefined(bits & 0b1111),
        }
    }

    // The lowercase UAL condition suffix (e.g. "eq"); empty for AL. CS/CC use the cs/cc spelling.
    pub fn ual_suffix(&self) -> &'static str {
        match self {
            Self::Equal => "eq",
            Self::NotEqual => "ne",
            Self::CarrySet => "cs",
            Self::CarryClear => "cc",
            Self::MinusNegative => "mi",
            Self::PlusPositiveOrZero => "pl",
            Self::Overflow => "vs",
            Self::NoOverflow => "vc",
            Self::UnsignedHigher => "hi",
            Self::UnsignedLowerOrSame => "ls",
            Self::SignedGreaterThanOrEqual => "ge",
            Self::SignedLessThan => "lt",
            Self::SignedGreaterThan => "gt",
            Self::SignedLessThanOrEqual => "le",
            Self::AlwaysUnconditional => "",
            Self::Undefined(_) => "",
        }
    }
}
