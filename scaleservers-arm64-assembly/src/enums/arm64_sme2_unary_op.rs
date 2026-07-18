// Copyright (c) Scaleservers LLC

/// An SME2 multi-vector (two-register) single-precision unary op (the non-destructive form
/// `<op> {Zd.s-Zd+1.s}, {Zn.s-Zn+1.s}`): the FP rounding-to-integral `FRINTN`/`FRINTM`/`FRINTP`/`FRINTA` and the
/// same-width `.s` conversions `FCVTZS`/`FCVTZU`/`SCVTF`/`UCVTF`. Each carries a full per-op `[23:16]`+`[5]` selector
/// (there is no orthogonal size axis -- this family is modeled for `.s`; the `.h`/`.d` conversion variants and the
/// type-changing widening `FCVT` form are not modeled, but the narrowing `FCVTN`/`BFCVT`/`BFCVTN` are modeled
/// separately as `Sme2FpCvtNarrow`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64Sme2UnaryOp {
    /// `FRINTN` -- round to nearest, ties to even.
    Frintn,
    /// `FRINTM` -- round toward minus infinity.
    Frintm,
    /// `FRINTP` -- round toward plus infinity.
    Frintp,
    /// `FRINTA` -- round to nearest, ties away.
    Frinta,
    /// `FCVTZS` -- FP to signed integer, round toward zero.
    Fcvtzs,
    /// `FCVTZU` -- FP to unsigned integer, round toward zero.
    Fcvtzu,
    /// `SCVTF` -- signed integer to FP.
    Scvtf,
    /// `UCVTF` -- unsigned integer to FP.
    Ucvtf,
}

impl Arm64Sme2UnaryOp {
    /// The 32-bit encoding base with both register pairs zero. The op-specific selector lives in `[23:16]` and `[5]`;
    /// `encode` then ORs `((Zn>>1)<<6)` and `((Zd>>1)<<1)`.
    pub const fn base(self) -> u32 {
        match self {
            Self::Frinta => 0xC1AC_E000,
            Self::Frintm => 0xC1AA_E000,
            Self::Frintn => 0xC1A8_E000,
            Self::Frintp => 0xC1A9_E000,
            Self::Fcvtzs => 0xC121_E000,
            Self::Fcvtzu => 0xC121_E020,
            Self::Scvtf => 0xC122_E000,
            Self::Ucvtf => 0xC122_E020,
        }
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Frintn => "frintn",
            Self::Frintm => "frintm",
            Self::Frintp => "frintp",
            Self::Frinta => "frinta",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtzu => "fcvtzu",
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",
        }
    }

    /// Recover the op from the `[23:16]` selector and the `[5]` bit; `None` for an unmodeled selector (so the decoder
    /// falls through to other forms / `InvalidOpcode`).
    pub const fn from_bits(op_hi: u32, bit5: u32) -> Option<Self> {
        Some(match (op_hi & 0xFF, bit5 & 1) {
            (0xAC, 0) => Self::Frinta,
            (0xAA, 0) => Self::Frintm,
            (0xA8, 0) => Self::Frintn,
            (0xA9, 0) => Self::Frintp,
            (0x21, 0) => Self::Fcvtzs,
            (0x21, 1) => Self::Fcvtzu,
            (0x22, 0) => Self::Scvtf,
            (0x22, 1) => Self::Ucvtf,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 8] = [
        Self::Frintn,
        Self::Frintm,
        Self::Frintp,
        Self::Frinta,
        Self::Fcvtzs,
        Self::Fcvtzu,
        Self::Scvtf,
        Self::Ucvtf,
    ];
}
