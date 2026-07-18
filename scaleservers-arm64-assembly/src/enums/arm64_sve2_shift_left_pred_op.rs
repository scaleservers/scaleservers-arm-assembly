// Copyright (c) Scaleservers LLC

/// An SVE2 **saturating / rounding bitwise shift left (predicated)** op (DDI0487 C8): `<op> Zdn.<T>, Pg/M, Zdn.<T>,
/// Zm.<T>`, shifting the first source by the (signed) per-element amount in the second. Selected by the 4-bit
/// `Q:R:N:U` field at `[19:16]` -- `Q` saturates, `R` reverses the operands, `N` rounds, `U` is unsigned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2ShiftLeftPredOp {
    Srshl,
    Urshl,
    Srshlr,
    Urshlr,
    Sqshl,
    Uqshl,
    Sqrshl,
    Uqrshl,
    Sqshlr,
    Uqshlr,
    Sqrshlr,
    Uqrshlr,
}

impl Arm64Sve2ShiftLeftPredOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Srshl => "srshl",
            Self::Urshl => "urshl",
            Self::Srshlr => "srshlr",
            Self::Urshlr => "urshlr",
            Self::Sqshl => "sqshl",
            Self::Uqshl => "uqshl",
            Self::Sqrshl => "sqrshl",
            Self::Uqrshl => "uqrshl",
            Self::Sqshlr => "sqshlr",
            Self::Uqshlr => "uqshlr",
            Self::Sqrshlr => "sqrshlr",
            Self::Uqrshlr => "uqrshlr",
        }
    }

    /// The 4-bit `Q:R:N:U` opcode (`[19:16]`).
    pub fn code(self) -> u32 {
        match self {
            Self::Srshl => 0b0010,
            Self::Urshl => 0b0011,
            Self::Srshlr => 0b0110,
            Self::Urshlr => 0b0111,
            Self::Sqshl => 0b1000,
            Self::Uqshl => 0b1001,
            Self::Sqrshl => 0b1010,
            Self::Uqrshl => 0b1011,
            Self::Sqshlr => 0b1100,
            Self::Uqshlr => 0b1101,
            Self::Sqrshlr => 0b1110,
            Self::Uqrshlr => 0b1111,
        }
    }

    /// Recover the op from its `[19:16]` code, or `None` for an unallocated value (`0000`/`0001`/`0100`/`0101`).
    pub fn from_code(code: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.code() == code & 0xF)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 12] = [
        Self::Srshl,
        Self::Urshl,
        Self::Srshlr,
        Self::Urshlr,
        Self::Sqshl,
        Self::Uqshl,
        Self::Sqrshl,
        Self::Uqrshl,
        Self::Sqshlr,
        Self::Uqshlr,
        Self::Sqrshlr,
        Self::Uqrshlr,
    ];
}
