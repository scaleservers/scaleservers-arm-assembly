// Copyright (c) Scaleservers LLC

/// An MTE **block-tag** load/store op (FEAT_MTE2, DDI0487 C6): `STGM`/`LDGM`/`STZGM` store, load, or store-and-zero
/// a block of allocation tags addressed by `[Xn]`, with `Xt` carrying the packed tags. Each is a fixed frame
/// `0xD9?0_0000 | (Rn<<5) | Rt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64MteBlockOp {
    /// `STGM Xt, [Xn|SP]` -- store a block of allocation tags from `Xt`.
    Stgm,
    /// `LDGM Xt, [Xn|SP]` -- load a block of allocation tags into `Xt`.
    Ldgm,
    /// `STZGM Xt, [Xn|SP]` -- store a block of allocation tags from `Xt` and zero the associated data.
    Stzgm,
}

impl Arm64MteBlockOp {
    /// The base word (`Rn`/`Rt` zero).
    pub fn base(self) -> u32 {
        match self {
            Self::Stgm => 0xD9A0_0000,
            Self::Ldgm => 0xD9E0_0000,
            Self::Stzgm => 0xD920_0000,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Stgm => "stgm",
            Self::Ldgm => "ldgm",
            Self::Stzgm => "stzgm",
        }
    }

    /// Recover the op from a masked base (`word & 0xFFE0_FC00`); `None` if not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 3] = [Self::Stgm, Self::Ldgm, Self::Stzgm];
}
