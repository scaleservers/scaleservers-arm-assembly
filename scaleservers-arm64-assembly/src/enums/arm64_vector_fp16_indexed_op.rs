// Copyright (c) Scaleservers LLC

/// A NEON **half-precision (FP16) by-element** op (DDI0487 C7, FEAT_FP16) -- a `.4h`/`.8h` multiply against a
/// broadcast lane `Vm.h[index]`. Encoded with `size = 00` (which distinguishes it from the f32/f64 by-element
/// FMUL/FMLA/FMLS/FMULX, whose masked base is otherwise identical).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorFp16ByElementOp {
    /// `FMUL` -- multiply.
    Fmul,
    /// `FMLA` -- fused multiply-add.
    Fmla,
    /// `FMLS` -- fused multiply-subtract.
    Fmls,
    /// `FMULX` -- multiply extended.
    Fmulx,
}

impl Arm64VectorFp16ByElementOp {
    /// The base word (`Q`/index/`Rm`/`Rn`/`Rd` zero). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Fmul => 0x0F00_9000,
            Self::Fmla => 0x0F00_1000,
            Self::Fmls => 0x0F00_5000,
            Self::Fmulx => 0x2F00_9000,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fmul => "fmul",
            Self::Fmla => "fmla",
            Self::Fmls => "fmls",
            Self::Fmulx => "fmulx",
        }
    }

    /// Recover the op from a masked base (`word & 0x3F00_F400`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Fmul, Self::Fmla, Self::Fmls, Self::Fmulx];
}

/// A NEON **half-precision (FP16) across-lanes reduce** op (DDI0487 C7, FEAT_FP16) -- reduces all lanes of a
/// `.4h`/`.8h` vector to a single `h` result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorFp16AcrossOp {
    /// `FMAXV` -- maximum across lanes (NaN-propagating).
    Fmaxv,
    /// `FMINV` -- minimum across lanes (NaN-propagating).
    Fminv,
    /// `FMAXNMV` -- maximum-number across lanes.
    Fmaxnmv,
    /// `FMINNMV` -- minimum-number across lanes.
    Fminnmv,
}

impl Arm64VectorFp16AcrossOp {
    /// The base word (`Q`/`Rn`/`Rd` zero). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Fmaxv => 0x0E30_F800,
            Self::Fminv => 0x0EB0_F800,
            Self::Fmaxnmv => 0x0E30_C800,
            Self::Fminnmv => 0x0EB0_C800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fmaxv => "fmaxv",
            Self::Fminv => "fminv",
            Self::Fmaxnmv => "fmaxnmv",
            Self::Fminnmv => "fminnmv",
        }
    }

    /// Recover the op from a masked base (`word & 0xBFFF_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Fmaxv, Self::Fminv, Self::Fmaxnmv, Self::Fminnmv];
}
