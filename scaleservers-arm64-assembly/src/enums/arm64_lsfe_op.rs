// Copyright (c) Scaleservers LLC

/// The arithmetic of an FEAT_LSFE atomic floating-point memory op (`LDF*`/`STF*`). The op occupies `[14:12]`. **LLVM-only
/// oracle** (binutils-trunk lacks FEAT_LSFE), so it lives behind the `experimental` cargo feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64LsfeOp {
    /// `LDFADD`/`STFADD` -- atomic floating-point add.
    Add,
    /// `LDFMAX`/`STFMAX` -- atomic floating-point maximum.
    Max,
    /// `LDFMIN`/`STFMIN` -- atomic floating-point minimum.
    Min,
    /// `LDFMAXNM`/`STFMAXNM` -- atomic floating-point maximum-number.
    Maxnm,
    /// `LDFMINNM`/`STFMINNM` -- atomic floating-point minimum-number.
    Minnm,
}

impl Arm64LsfeOp {
    /// The `[14:12]` op-selector value.
    pub fn code(self) -> u32 {
        match self {
            Self::Add => 0b000,
            Self::Max => 0b100,
            Self::Min => 0b101,
            Self::Maxnm => 0b110,
            Self::Minnm => 0b111,
        }
    }

    /// The mnemonic stem (after the `ldf`/`stf` prefix).
    pub fn stem(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Max => "max",
            Self::Min => "min",
            Self::Maxnm => "maxnm",
            Self::Minnm => "minnm",
        }
    }

    /// Recover the op from the `[14:12]` selector; `None` for the three unallocated values (`001`/`010`/`011`).
    pub fn from_code(code: u32) -> Option<Self> {
        Some(match code & 0b111 {
            0b000 => Self::Add,
            0b100 => Self::Max,
            0b101 => Self::Min,
            0b110 => Self::Maxnm,
            0b111 => Self::Minnm,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 5] = [Self::Add, Self::Max, Self::Min, Self::Maxnm, Self::Minnm];
}
