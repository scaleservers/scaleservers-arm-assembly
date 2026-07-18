// Copyright (c) Scaleservers LLC

/// A FEAT_CSSC scalar **unary** data-processing op (`ABS`/`CNT`/`CTZ Wd|Xd, Wn|Xn`). These are additional opcodes in
/// the data-processing (one source) group (the same group as `RBIT`/`CLZ`), gated on FEAT_CSSC. GNU+LLVM verified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64CsscUnaryOp {
    /// `ABS` -- absolute value (of the signed integer).
    Abs,
    /// `CNT` -- count set bits (population count).
    Cnt,
    /// `CTZ` -- count trailing zeros.
    Ctz,
}

impl Arm64CsscUnaryOp {
    /// The 32-bit encoding base (the `W` form; the `X` form sets bit 31). GNU+LLVM verified.
    pub const fn base(self) -> u32 {
        match self {
            Self::Abs => 0x5AC0_2000,
            Self::Cnt => 0x5AC0_1C00,
            Self::Ctz => 0x5AC0_1800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Abs => "abs",
            Self::Cnt => "cnt",
            Self::Ctz => "ctz",
        }
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 3] = [Self::Abs, Self::Cnt, Self::Ctz];
}
