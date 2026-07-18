// Copyright (c) Scaleservers LLC

/// SVE FP8 narrowing convert op (FEAT_FP8): narrow a two-vector source list to a single `Zd.b` of 8-bit
/// floating-point lanes. `FCVTN`/`BFCVTN` read a `.h` pair (FP16/BFloat16 -> FP8); `FCVTNB` reads a `.s` pair.
/// The op is the `[11:10]` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFp8NarrowOp {
    /// `FCVTN Zd.b, {Zn.h-Zn+1.h}` -- FP16 -> FP8. `[11:10] = 00`.
    Fcvtn,
    /// `FCVTNB Zd.b, {Zn.s-Zn+1.s}` -- FP32 -> FP8 (bottom). `[11:10] = 01`.
    Fcvtnb,
    /// `BFCVTN Zd.b, {Zn.h-Zn+1.h}` -- BFloat16 -> FP8. `[11:10] = 10`.
    Bfcvtn,
}

impl Arm64SveFp8NarrowOp {
    /// The `[11:10]` op field.
    pub fn op_bits(self) -> u32 {
        match self {
            Self::Fcvtn => 0b00,
            Self::Fcvtnb => 0b01,
            Self::Bfcvtn => 0b10,
        }
    }

    /// Recover the op from the `[11:10]` field, or `None` for the unallocated `11`.
    pub fn from_op_bits(bits: u32) -> Option<Self> {
        Some(match bits & 0b11 {
            0b00 => Self::Fcvtn,
            0b01 => Self::Fcvtnb,
            0b10 => Self::Bfcvtn,
            _ => return None,
        })
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fcvtn => "fcvtn",
            Self::Fcvtnb => "fcvtnb",
            Self::Bfcvtn => "bfcvtn",
        }
    }

    /// The source-list element letter: `.s` for `FCVTNB`, `.h` for `FCVTN`/`BFCVTN`.
    pub fn source_letter(self) -> &'static str {
        match self {
            Self::Fcvtnb => "s",
            Self::Fcvtn | Self::Bfcvtn => "h",
        }
    }

    /// All three ops, for exhaustive round-trip testing.
    pub const ALL: [Self; 3] = [Self::Fcvtn, Self::Fcvtnb, Self::Bfcvtn];
}
