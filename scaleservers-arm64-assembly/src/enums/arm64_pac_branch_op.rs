// Copyright (c) Scaleservers LLC

/// A **pointer-authenticated indirect branch** op (DDI0487 C6, FEAT_PAuth) -- `BRAA`/`BRAB`/`BLRAA`/`BLRAB`. Each
/// authenticates the target `Xn` with a modifier `Xm` (or a zero modifier, the `*Z` forms) before branching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64PacBranchOp {
    /// `BRAA`/`BRAAZ` -- branch to register, key A.
    Braa,
    /// `BRAB`/`BRABZ` -- branch to register, key B.
    Brab,
    /// `BLRAA`/`BLRAAZ` -- branch-with-link to register, key A.
    Blraa,
    /// `BLRAB`/`BLRABZ` -- branch-with-link to register, key B.
    Blrab,
}

impl Arm64PacBranchOp {
    /// The base word of the modifier form (`Xm` present, bit24=1), with `Rn`/`Rm` zero. The zero-modifier form
    /// clears bit24. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Braa => 0xD71F_0800,
            Self::Brab => 0xD71F_0C00,
            Self::Blraa => 0xD73F_0800,
            Self::Blrab => 0xD73F_0C00,
        }
    }

    /// The mnemonic; `zero` selects the zero-modifier `*Z` spelling.
    pub fn name(self, zero: bool) -> &'static str {
        match (self, zero) {
            (Self::Braa, false) => "braa",
            (Self::Braa, true) => "braaz",
            (Self::Brab, false) => "brab",
            (Self::Brab, true) => "brabz",
            (Self::Blraa, false) => "blraa",
            (Self::Blraa, true) => "blraaz",
            (Self::Blrab, false) => "blrab",
            (Self::Blrab, true) => "blrabz",
        }
    }

    /// Recover `(op, zero)` from a masked base (`word & 0xFFFF_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<(Self, bool)> {
        for op in Self::ALL {
            if op.base() == base {
                return Some((op, false));
            }
            if op.base() & !0x0100_0000 == base {
                return Some((op, true));
            }
        }
        None
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Braa, Self::Brab, Self::Blraa, Self::Blrab];
}

/// A **pointer-authenticated return** op (DDI0487 C6, FEAT_PAuth) -- `RETAA`/`RETAB` (return, authenticating LR
/// with SP) and `ERETAA`/`ERETAB` (exception return). Operand-free fixed words.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64PacReturnOp {
    /// `RETAA` -- return, key A.
    Retaa,
    /// `RETAB` -- return, key B.
    Retab,
    /// `ERETAA` -- exception return, key A.
    Eretaa,
    /// `ERETAB` -- exception return, key B.
    Eretab,
}

impl Arm64PacReturnOp {
    /// The fixed instruction word. GNU+LLVM dual-oracle verified.
    pub fn word(self) -> u32 {
        match self {
            Self::Retaa => 0xD65F_0BFF,
            Self::Retab => 0xD65F_0FFF,
            Self::Eretaa => 0xD69F_0BFF,
            Self::Eretab => 0xD69F_0FFF,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Retaa => "retaa",
            Self::Retab => "retab",
            Self::Eretaa => "eretaa",
            Self::Eretab => "eretab",
        }
    }

    /// Recover the op from a full instruction word; `None` if it is not one of these.
    pub fn from_word(word: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.word() == word)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Retaa, Self::Retab, Self::Eretaa, Self::Eretab];
}
