// Copyright (c) Scaleservers LLC

/// The 4-bit AArch64 condition code, used by `B.cond` (and, in the wider ISA, by the conditional-select /
/// conditional-compare families). The encoding is identical to AArch32's: bits `[3:1]` choose the test and
/// bit `[0]` inverts it (so `EQ`/`NE`, `CS`/`CC`, ... are paired). Value `0b1110` is `AL` ("always") and
/// `0b1111` is `NV` ("never", but architecturally behaves as always); both are representable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64Condition {
    Equal,                    // EQ -- 0b0000
    NotEqual,                 // NE -- 0b0001
    CarrySet,                 // CS / HS -- 0b0010
    CarryClear,               // CC / LO -- 0b0011
    Minus,                    // MI -- 0b0100
    Plus,                     // PL -- 0b0101
    Overflow,                 // VS -- 0b0110
    NoOverflow,               // VC -- 0b0111
    UnsignedHigher,           // HI -- 0b1000
    UnsignedLowerOrSame,      // LS -- 0b1001
    SignedGreaterThanOrEqual, // GE -- 0b1010
    SignedLessThan,           // LT -- 0b1011
    SignedGreaterThan,        // GT -- 0b1100
    SignedLessThanOrEqual,    // LE -- 0b1101
    Always,                   // AL -- 0b1110
    Never,                    // NV -- 0b1111 (behaves as always)
}

impl Arm64Condition {
    /// The 4-bit condition field value.
    pub fn as_operand_bits(&self) -> u8 {
        match self {
            Self::Equal => 0b0000,
            Self::NotEqual => 0b0001,
            Self::CarrySet => 0b0010,
            Self::CarryClear => 0b0011,
            Self::Minus => 0b0100,
            Self::Plus => 0b0101,
            Self::Overflow => 0b0110,
            Self::NoOverflow => 0b0111,
            Self::UnsignedHigher => 0b1000,
            Self::UnsignedLowerOrSame => 0b1001,
            Self::SignedGreaterThanOrEqual => 0b1010,
            Self::SignedLessThan => 0b1011,
            Self::SignedGreaterThan => 0b1100,
            Self::SignedLessThanOrEqual => 0b1101,
            Self::Always => 0b1110,
            Self::Never => 0b1111,
        }
    }

    /// Map a 4-bit condition field to its condition. TOTAL (masks the low four bits), never panics -- the
    /// decoder derives the condition from untrusted instruction bytes.
    pub fn from_operand_bits(bits: u8) -> Self {
        match bits & 0b1111 {
            0b0000 => Self::Equal,
            0b0001 => Self::NotEqual,
            0b0010 => Self::CarrySet,
            0b0011 => Self::CarryClear,
            0b0100 => Self::Minus,
            0b0101 => Self::Plus,
            0b0110 => Self::Overflow,
            0b0111 => Self::NoOverflow,
            0b1000 => Self::UnsignedHigher,
            0b1001 => Self::UnsignedLowerOrSame,
            0b1010 => Self::SignedGreaterThanOrEqual,
            0b1011 => Self::SignedLessThan,
            0b1100 => Self::SignedGreaterThan,
            0b1101 => Self::SignedLessThanOrEqual,
            0b1110 => Self::Always,
            // `& 0b1111` makes 0b1111 the only remaining value; the arm is exhaustive.
            _ => Self::Never,
        }
    }

    /// The lowercase UAL condition suffix (e.g. `eq`), used after the `.` in `b.eq`. `CS`/`CC` use the
    /// `cs`/`cc` spelling (the architectural canonical forms).
    pub fn ual_suffix(&self) -> &'static str {
        match self {
            Self::Equal => "eq",
            Self::NotEqual => "ne",
            Self::CarrySet => "cs",
            Self::CarryClear => "cc",
            Self::Minus => "mi",
            Self::Plus => "pl",
            Self::Overflow => "vs",
            Self::NoOverflow => "vc",
            Self::UnsignedHigher => "hi",
            Self::UnsignedLowerOrSame => "ls",
            Self::SignedGreaterThanOrEqual => "ge",
            Self::SignedLessThan => "lt",
            Self::SignedGreaterThan => "gt",
            Self::SignedLessThanOrEqual => "le",
            Self::Always => "al",
            Self::Never => "nv",
        }
    }
}
