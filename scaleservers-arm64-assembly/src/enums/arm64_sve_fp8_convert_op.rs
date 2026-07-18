// Copyright (c) Scaleservers LLC

/// SVE FP8 widening convert op (FEAT_FP8): widen the 8-bit floating-point lanes of `Zn.b` to half-precision /
/// BFloat16 in `Zd.h`. The numeric format (`F1`/`F2` = the two OFP8 interpretations, `BF1`/`BF2` likewise) is the
/// `[11:10]` field; the instruction carries a separate `top` flag (the `LT` suffix, `[16]`) selecting the odd
/// (top) source lanes over the even (bottom) ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFp8ConvertOp {
    /// `F1CVT(LT)` -- FP8 (first format) to FP16. `[11:10] = 00`.
    F1cvt,
    /// `F2CVT(LT)` -- FP8 (second format) to FP16. `[11:10] = 01`.
    F2cvt,
    /// `BF1CVT(LT)` -- FP8 (first format) to BFloat16. `[11:10] = 10`.
    Bf1cvt,
    /// `BF2CVT(LT)` -- FP8 (second format) to BFloat16. `[11:10] = 11`.
    Bf2cvt,
}

impl Arm64SveFp8ConvertOp {
    /// The `[11:10]` format field.
    pub fn format_bits(self) -> u32 {
        match self {
            Self::F1cvt => 0b00,
            Self::F2cvt => 0b01,
            Self::Bf1cvt => 0b10,
            Self::Bf2cvt => 0b11,
        }
    }

    /// Recover the op from the `[11:10]` format field (total over all four values).
    pub fn from_format_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0b00 => Self::F1cvt,
            0b01 => Self::F2cvt,
            0b10 => Self::Bf1cvt,
            _ => Self::Bf2cvt,
        }
    }

    /// The mnemonic stem, without the `lt` top-lane suffix (which the instruction adds when `top`).
    pub fn name(self) -> &'static str {
        match self {
            Self::F1cvt => "f1cvt",
            Self::F2cvt => "f2cvt",
            Self::Bf1cvt => "bf1cvt",
            Self::Bf2cvt => "bf2cvt",
        }
    }

    /// All four formats, for exhaustive round-trip testing.
    pub const ALL: [Self; 4] = [Self::F1cvt, Self::F2cvt, Self::Bf1cvt, Self::Bf2cvt];
}
