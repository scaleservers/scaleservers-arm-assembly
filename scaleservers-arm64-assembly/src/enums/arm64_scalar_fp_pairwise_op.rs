// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **floating-point pairwise reduce** op (DDI0487 C7, the `01 1 11110 E sz 11000 opcode 10
/// Rn Rd` encoding) -- reduces the two lanes of `Vn.<2s|2d>` to a single `s`/`d` result. `double` selects `.2d`
/// (else `.2s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarFpPairwiseOp {
    /// `FADDP` -- pairwise add.
    Faddp,
    /// `FMAXP` -- pairwise maximum.
    Fmaxp,
    /// `FMINP` -- pairwise minimum.
    Fminp,
    /// `FMAXNMP` -- pairwise maximum-number.
    Fmaxnmp,
    /// `FMINNMP` -- pairwise minimum-number.
    Fminnmp,
}

impl Arm64ScalarFpPairwiseOp {
    /// The base word (the `.2s`->`s` form, `sz`/`Rn`/`Rd` zero). The encoder ORs `sz<<22` for the `.2d`->`d` form.
    /// GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Faddp => 0x7E30_D800,
            Self::Fmaxp => 0x7E30_F800,
            Self::Fminp => 0x7EB0_F800,
            Self::Fmaxnmp => 0x7E30_C800,
            Self::Fminnmp => 0x7EB0_C800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Faddp => "faddp",
            Self::Fmaxp => "fmaxp",
            Self::Fminp => "fminp",
            Self::Fmaxnmp => "fmaxnmp",
            Self::Fminnmp => "fminnmp",
        }
    }

    /// Recover the op from a masked base (`word & 0xFFBF_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 5] = [
        Self::Faddp,
        Self::Fmaxp,
        Self::Fminp,
        Self::Fmaxnmp,
        Self::Fminnmp,
    ];
}
