// Copyright (c) Scaleservers LLC

/// An SVE2 **shift right narrow by immediate** op (DDI0487 C8 "SVE2 bitwise shift right narrow"): `<op>{B,T}
/// Zd.<Tn>, Zn.<T>, #<shift>`, shifting each wide source element right and writing the narrow (half-width) result.
/// The op is selected by the 3-bit `op:U:R` field at `[13:11]`; `R` rounds, the saturating variants clamp.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2NarrowShiftOp {
    /// `SQSHRUN` -- signed saturating shift right unsigned narrow (`[13:11]`=000).
    Sqshrun,
    /// `SQRSHRUN` -- signed saturating rounding shift right unsigned narrow (`[13:11]`=001).
    Sqrshrun,
    /// `SHRN` -- shift right narrow (`[13:11]`=010).
    Shrn,
    /// `RSHRN` -- rounding shift right narrow (`[13:11]`=011).
    Rshrn,
    /// `SQSHRN` -- signed saturating shift right narrow (`[13:11]`=100).
    Sqshrn,
    /// `SQRSHRN` -- signed saturating rounding shift right narrow (`[13:11]`=101).
    Sqrshrn,
    /// `UQSHRN` -- unsigned saturating shift right narrow (`[13:11]`=110).
    Uqshrn,
    /// `UQRSHRN` -- unsigned saturating rounding shift right narrow (`[13:11]`=111).
    Uqrshrn,
}

impl Arm64Sve2NarrowShiftOp {
    /// The lowercase UAL mnemonic stem (without the `B`/`T` suffix).
    pub fn stem(self) -> &'static str {
        match self {
            Self::Sqshrun => "sqshrun",
            Self::Sqrshrun => "sqrshrun",
            Self::Shrn => "shrn",
            Self::Rshrn => "rshrn",
            Self::Sqshrn => "sqshrn",
            Self::Sqrshrn => "sqrshrn",
            Self::Uqshrn => "uqshrn",
            Self::Uqrshrn => "uqrshrn",
        }
    }

    /// The 3-bit `op:U:R` opcode (`[13:11]`).
    pub fn code(self) -> u32 {
        match self {
            Self::Sqshrun => 0b000,
            Self::Sqrshrun => 0b001,
            Self::Shrn => 0b010,
            Self::Rshrn => 0b011,
            Self::Sqshrn => 0b100,
            Self::Sqrshrn => 0b101,
            Self::Uqshrn => 0b110,
            Self::Uqrshrn => 0b111,
        }
    }

    /// Recover the op from its `[13:11]` code.
    pub fn from_code(code: u32) -> Self {
        Self::ALL[(code & 0b111) as usize]
    }

    /// Every op, indexed by code.
    pub const ALL: [Self; 8] = [
        Self::Sqshrun,
        Self::Sqrshrun,
        Self::Shrn,
        Self::Rshrn,
        Self::Sqshrn,
        Self::Sqrshrn,
        Self::Uqshrn,
        Self::Uqrshrn,
    ];
}
