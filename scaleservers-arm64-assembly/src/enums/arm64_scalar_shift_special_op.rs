// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **narrowing shift-by-immediate** op (DDI0487 C7) -- the saturating shift-right-narrow
/// family. The encoded `immh` is the *narrow* (destination) element size (`b`/`h`/`s`); the source is one size
/// wider. `immh:immb = 2*narrow_bits - shift` (a right shift, `shift` in `1..narrow_bits`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarShiftNarrowOp {
    /// `SQSHRUN` -- signed saturating shift right unsigned narrow.
    Sqshrun,
    /// `SQRSHRUN` -- signed saturating rounding shift right unsigned narrow.
    Sqrshrun,
    /// `SQSHRN` -- signed saturating shift right narrow.
    Sqshrn,
    /// `SQRSHRN` -- signed saturating rounding shift right narrow.
    Sqrshrn,
    /// `UQSHRN` -- unsigned saturating shift right narrow.
    Uqshrn,
    /// `UQRSHRN` -- unsigned saturating rounding shift right narrow.
    Uqrshrn,
}

impl Arm64ScalarShiftNarrowOp {
    /// The base word (`immh:immb`/`Rn`/`Rd` zero): `0x5F00_0400 | (U<<29) | (opcode<<11)`. GNU+LLVM dual-oracle
    /// verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Sqshrun => (1, 0b10000),
            Self::Sqrshrun => (1, 0b10001),
            Self::Sqshrn => (0, 0b10010),
            Self::Sqrshrn => (0, 0b10011),
            Self::Uqshrn => (1, 0b10010),
            Self::Uqrshrn => (1, 0b10011),
        };
        0x5F00_0400 | (u << 29) | (opcode << 11)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sqshrun => "sqshrun",
            Self::Sqrshrun => "sqrshrun",
            Self::Sqshrn => "sqshrn",
            Self::Sqrshrn => "sqrshrn",
            Self::Uqshrn => "uqshrn",
            Self::Uqrshrn => "uqrshrn",
        }
    }

    /// Recover the op from a masked base (`word & 0xFF80_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 6] = [
        Self::Sqshrun,
        Self::Sqrshrun,
        Self::Sqshrn,
        Self::Sqrshrn,
        Self::Uqshrn,
        Self::Uqrshrn,
    ];
}

/// A scalar Advanced SIMD **fixed-point convert** op (DDI0487 C7) -- the shift-by-immediate-encoded conversions
/// between a fixed-point value and floating-point. The element (`s`/`d`) is the register size; the number of
/// fractional bits folds via `immh:immb = 2*esize - fbits` (`fbits` in `1..esize`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarFixedConvertOp {
    /// `SCVTF` -- signed fixed-point convert to floating-point.
    Scvtf,
    /// `UCVTF` -- unsigned fixed-point convert to floating-point.
    Ucvtf,
    /// `FCVTZS` -- floating-point convert to signed fixed-point, round toward zero.
    Fcvtzs,
    /// `FCVTZU` -- floating-point convert to unsigned fixed-point, round toward zero.
    Fcvtzu,
}

impl Arm64ScalarFixedConvertOp {
    /// The base word (`immh:immb`/`Rn`/`Rd` zero): `0x5F00_0400 | (U<<29) | (opcode<<11)`. GNU+LLVM dual-oracle
    /// verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Scvtf => (0, 0b11100),
            Self::Ucvtf => (1, 0b11100),
            Self::Fcvtzs => (0, 0b11111),
            Self::Fcvtzu => (1, 0b11111),
        };
        0x5F00_0400 | (u << 29) | (opcode << 11)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtzu => "fcvtzu",
        }
    }

    /// Recover the op from a masked base (`word & 0xFF80_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Scvtf, Self::Ucvtf, Self::Fcvtzs, Self::Fcvtzu];
}
