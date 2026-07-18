// Copyright (c) Scaleservers LLC

/// FP8 (FEAT_FP8) convert-long operations -- widen a vector of 8-bit floating-point lanes (`Vn.8b` lower half, or
/// `Vn.16b` upper half for the `2`-suffix form) to eight half-precision (`F1CVTL`/`F2CVTL`) or BFloat16
/// (`BF1CVTL`/`BF2CVTL`) lanes in `Vd.8h`. The `1`/`2` digit selects which of the two FP8 source interpretations
/// (the OFP8 E5M2 vs E4M3 formats, governed by `FPMR`) the lanes are read as. The operation is carried in the
/// two-register-misc `size<23:22>` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Fp8ConvertLongOp {
    /// `F1CVTL` -- FP8 (first format) widened to FP16. `size = 00`.
    F1cvtl,
    /// `F2CVTL` -- FP8 (second format) widened to FP16. `size = 01`.
    F2cvtl,
    /// `BF1CVTL` -- FP8 (first format) widened to BFloat16. `size = 10`.
    Bf1cvtl,
    /// `BF2CVTL` -- FP8 (second format) widened to BFloat16. `size = 11`.
    Bf2cvtl,
}

impl Arm64Fp8ConvertLongOp {
    /// The `size<23:22>` field value that selects this operation.
    pub fn size_bits(self) -> u32 {
        match self {
            Self::F1cvtl => 0b00,
            Self::F2cvtl => 0b01,
            Self::Bf1cvtl => 0b10,
            Self::Bf2cvtl => 0b11,
        }
    }

    /// Recover the operation from the `size<23:22>` field (total over all four values).
    pub fn from_size_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0b00 => Self::F1cvtl,
            0b01 => Self::F2cvtl,
            0b10 => Self::Bf1cvtl,
            _ => Self::Bf2cvtl,
        }
    }

    /// The GNU mnemonic stem, without the `2` upper-half suffix (which the instruction adds for `Vn.16b`).
    pub fn name(self) -> &'static str {
        match self {
            Self::F1cvtl => "f1cvtl",
            Self::F2cvtl => "f2cvtl",
            Self::Bf1cvtl => "bf1cvtl",
            Self::Bf2cvtl => "bf2cvtl",
        }
    }

    /// All four operations, for exhaustive round-trip testing.
    pub const ALL: [Self; 4] = [Self::F1cvtl, Self::F2cvtl, Self::Bf1cvtl, Self::Bf2cvtl];
}
