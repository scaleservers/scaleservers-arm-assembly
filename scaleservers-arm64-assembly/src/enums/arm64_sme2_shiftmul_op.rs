// Copyright (c) Scaleservers LLC

/// An SME2 multi-vector rounding-shift / saturating-doubling-multiply op (the destructive multi-vector forms
/// `<op> {Zdn-list}, {Zdn-list}, Zm` and `..., {Zm-list}`): `SRSHL`/`URSHL` (signed/unsigned rounding shift left) and
/// `SQDMULH` (signed saturating doubling multiply returning high half). `SRSHL`/`URSHL` share a base with the sign at
/// `[0]`; `SQDMULH` has its own base and no sign variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64Sme2ShiftMulOp {
    /// `SRSHL` -- signed rounding shift left.
    Srshl,
    /// `URSHL` -- unsigned rounding shift left.
    Urshl,
    /// `SQDMULH` -- signed saturating doubling multiply, high half.
    Sqdmulh,
}

impl Arm64Sme2ShiftMulOp {
    /// The by-single-vector encoding base (the multi-vector source form ORs in the `[12]` marker).
    /// `SRSHL`/`URSHL` -> `0xC120_A220` (`[9]=1`, `[5]=1`); `SQDMULH` -> `0xC120_A400` (`[10]=1`).
    pub const fn base(self) -> u32 {
        match self {
            Self::Srshl | Self::Urshl => 0xC120_A220,
            Self::Sqdmulh => 0xC120_A400,
        }
    }

    /// The `[0]` sign bit (`URSHL` is the only unsigned form; `SQDMULH` is inherently signed).
    pub const fn bit0(self) -> u32 {
        matches!(self, Self::Urshl) as u32
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Srshl => "srshl",
            Self::Urshl => "urshl",
            Self::Sqdmulh => "sqdmulh",
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Srshl, Self::Urshl, Self::Sqdmulh];
}
