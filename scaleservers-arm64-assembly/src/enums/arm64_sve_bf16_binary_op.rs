// Copyright (c) Scaleservers LLC

/// An SVE2.1 **BFloat16 predicated binary** arithmetic operation (FEAT_SVE_B16B16): the `.h` BF16 add/sub/mul and the
/// max/min pair (IEEE `..nm` number-preserving and the propagating forms). The op is the `[18:16]` selector of the
/// shared predicated frame (value `3` is unallocated). GNU+LLVM verified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SveBf16BinaryOp {
    /// `BFADD` -- BF16 add.
    Add,
    /// `BFSUB` -- BF16 subtract.
    Sub,
    /// `BFMUL` -- BF16 multiply.
    Mul,
    /// `BFMAXNM` -- BF16 max, number-preserving (IEEE maxNum).
    Maxnm,
    /// `BFMINNM` -- BF16 min, number-preserving (IEEE minNum).
    Minnm,
    /// `BFMAX` -- BF16 max (propagating).
    Max,
    /// `BFMIN` -- BF16 min (propagating).
    Min,
}

impl Arm64SveBf16BinaryOp {
    /// The raw 3-bit op-selector value (`0..=7`, with `3` unallocated). The predicated form places it at `[18:16]`,
    /// the unpredicated `bfadd`/`bfsub`/`bfmul` form at `[11:10]` (only `0..=2` valid there).
    pub const fn value(self) -> u32 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
            Self::Mul => 2,
            Self::Maxnm => 4,
            Self::Minnm => 5,
            Self::Max => 6,
            Self::Min => 7,
        }
    }

    /// The `[18:16]` op-selector bits OR-ed into the predicated base.
    pub const fn discriminant(self) -> u32 {
        self.value() << 16
    }

    /// Recover the op from the `[18:16]` selector; `None` for the unallocated value `3`.
    pub const fn from_bits(bits: u32) -> Option<Self> {
        Some(match bits & 0b111 {
            0 => Self::Add,
            1 => Self::Sub,
            2 => Self::Mul,
            4 => Self::Maxnm,
            5 => Self::Minnm,
            6 => Self::Max,
            7 => Self::Min,
            _ => return None,
        })
    }

    /// The UAL mnemonic.
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Add => "bfadd",
            Self::Sub => "bfsub",
            Self::Mul => "bfmul",
            Self::Maxnm => "bfmaxnm",
            Self::Minnm => "bfminnm",
            Self::Max => "bfmax",
            Self::Min => "bfmin",
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 7] = [
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Maxnm,
        Self::Minnm,
        Self::Max,
        Self::Min,
    ];
}
