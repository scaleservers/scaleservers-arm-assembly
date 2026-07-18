// Copyright (c) Scaleservers LLC

/// An SME2 **two-vector floating-point narrowing convert** op (FEAT_SME2; DDI0487 part C). Narrows a consecutive
/// `.s` (f32) Z-register pair to a single `.h` register. The op is split across the `[22]` (to-BFloat16) and `[5]`
/// (narrow/odd) bits of the shared frame; the `(0, 0)` combo is the *widening* `FCVT` (a different operand shape, not
/// modeled here). GNU+LLVM verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sme2FpCvtNarrowOp {
    /// `FCVTN` -- f32 pair -> f16 (IEEE half) narrowing convert.
    Fcvtn,
    /// `BFCVT` -- f32 pair -> BFloat16 convert.
    Bfcvt,
    /// `BFCVTN` -- f32 pair -> BFloat16 narrowing convert (interleaved).
    Bfcvtn,
}

impl Arm64Sme2FpCvtNarrowOp {
    /// The op bits OR-ed into the base: `[22]` (to-BFloat16) and `[5]` (narrow/odd).
    pub fn op_bits(self) -> u32 {
        match self {
            Self::Fcvtn => 1 << 5,
            Self::Bfcvt => 1 << 22,
            Self::Bfcvtn => (1 << 22) | (1 << 5),
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Fcvtn => "fcvtn",
            Self::Bfcvt => "bfcvt",
            Self::Bfcvtn => "bfcvtn",
        }
    }

    /// Recover the op from the `[22]`(to-BFloat16)/`[5]`(narrow) split; `None` for the `(0, 0)` widening `FCVT`.
    pub fn from_op_bits(to_bf16: u32, narrow: u32) -> Option<Self> {
        Some(match (to_bf16 & 1, narrow & 1) {
            (0, 1) => Self::Fcvtn,
            (1, 0) => Self::Bfcvt,
            (1, 1) => Self::Bfcvtn,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Fcvtn, Self::Bfcvt, Self::Bfcvtn];
}
