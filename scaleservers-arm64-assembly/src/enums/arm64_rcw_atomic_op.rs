// Copyright (c) Scaleservers LLC

/// The operation of a FEAT_THE **read-check-write atomic** in the atomic-memory-op form (DDI0487 C6): `RCWCLR`
/// (bit-clear), `RCWSET` (bit-set), or `RCWSWP` (swap). Each has a single-register base (the atomic-op group,
/// `0x38..`) and a 128-bit register-pair base (the RCW pair group, `0x19..`); the `RCWS` secure variants and the
/// A/L ordering are orthogonal bits added by the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64RcwAtomicOp {
    /// `RCWCLR` -- atomic bit-clear (`[Xn] &= ~Xs`).
    Clr,
    /// `RCWSET` -- atomic bit-set (`[Xn] |= Xs`).
    Set,
    /// `RCWSWP` -- atomic swap.
    Swp,
}

impl Arm64RcwAtomicOp {
    /// The single-register base word (the atomic-memory-op group); the caller ORs `S<<30`, `A<<23`, `L<<22`,
    /// `Rs<<16`, `Rn<<5`, `Rt`.
    pub fn single_base(self) -> u32 {
        match self {
            Self::Clr => 0x3820_9000,
            Self::Set => 0x3820_B000,
            Self::Swp => 0x3820_A000,
        }
    }

    /// The 128-bit register-pair base word (the RCW pair group); the caller ORs `S<<30`, `A<<23`, `L<<22`,
    /// `Rt2<<16`, `Rn<<5`, `Rt1`.
    pub fn pair_base(self) -> u32 {
        match self {
            Self::Clr => 0x1920_9000,
            Self::Set => 0x1920_B000,
            Self::Swp => 0x1920_A000,
        }
    }

    /// The lowercase op mnemonic stem (`clr`/`set`/`swp`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Clr => "clr",
            Self::Set => "set",
            Self::Swp => "swp",
        }
    }

    /// Recover the op from a masked single-register base (`word & RCW_MASK`), if one of these.
    pub fn from_single_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.single_base() == base)
    }

    /// Recover the op from a masked pair base (`word & RCW_MASK`), if one of these.
    pub fn from_pair_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.pair_base() == base)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 3] = [Self::Clr, Self::Set, Self::Swp];
}
