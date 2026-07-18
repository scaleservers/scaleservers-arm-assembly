// Copyright (c) Scaleservers LLC

/// An AArch64 **memory-tagging data-processing (2-source)** op (DDI0487 C6, FEAT_MTE) -- `IRG`/`GMI`/`SUBP`/
/// `SUBPS`. All operate on general-purpose registers `Xd, Xn, Xm`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64MteDataOp {
    /// `IRG Xd, Xn, Xm` -- insert a random logical address tag (excluding the tags in `Xm`).
    Irg,
    /// `GMI Xd, Xn, Xm` -- insert `Xn`'s tag into the tag-exclusion mask `Xm` -> `Xd`.
    Gmi,
    /// `SUBP Xd, Xn, Xm` -- subtract two tagged pointers (ignoring the tags).
    Subp,
    /// `SUBPS Xd, Xn, Xm` -- subtract two tagged pointers and set the flags.
    Subps,
}

impl Arm64MteDataOp {
    /// The base word (`Rm`/`Rn`/`Rd` zero), in the data-processing (2-source) frame. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Irg => 0x9AC0_1000,
            Self::Gmi => 0x9AC0_1400,
            Self::Subp => 0x9AC0_0000,
            Self::Subps => 0xBAC0_0000,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Irg => "irg",
            Self::Gmi => "gmi",
            Self::Subp => "subp",
            Self::Subps => "subps",
        }
    }

    /// Recover the op from a masked base (`word & 0xFFE0_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Irg, Self::Gmi, Self::Subp, Self::Subps];
}
