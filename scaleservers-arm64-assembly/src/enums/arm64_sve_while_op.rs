// Copyright (c) Scaleservers LLC

/// An SVE **while** loop-predicate op (DDI0487 part C -- "SVE integer compare scalar count and limit"). Each
/// generates a predicate that is true for the leading elements where the incrementing first operand satisfies the
/// comparison against the limit: `WHILELT`/`WHILELE` (signed `<`/`<=`), `WHILELO`/`WHILELS` (unsigned `<`/`<=`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveWhileOp {
    /// `WHILELT` -- while signed less-than.
    Whilelt,
    /// `WHILELE` -- while signed less-than-or-equal.
    Whilele,
    /// `WHILELO` -- while unsigned lower (`<`).
    Whilelo,
    /// `WHILELS` -- while unsigned lower-or-same (`<=`).
    Whilels,
}

impl Arm64SveWhileOp {
    /// The `U` bit (`[11]`): 0 = signed, 1 = unsigned.
    pub fn unsigned_bit(self) -> u32 {
        match self {
            Self::Whilelt | Self::Whilele => 0,
            Self::Whilelo | Self::Whilels => 1,
        }
    }

    /// The `eq` bit (`[4]`): 0 = strict (`<`), 1 = or-equal (`<=`).
    pub fn equal_bit(self) -> u32 {
        match self {
            Self::Whilelt | Self::Whilelo => 0,
            Self::Whilele | Self::Whilels => 1,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Whilelt => "whilelt",
            Self::Whilele => "whilele",
            Self::Whilelo => "whilelo",
            Self::Whilels => "whilels",
        }
    }

    /// Recover the op from its `(U, eq)` bits.
    pub fn from_bits(unsigned: u32, equal: u32) -> Self {
        match (unsigned & 1, equal & 1) {
            (0, 0) => Self::Whilelt,
            (0, _) => Self::Whilele,
            (_, 0) => Self::Whilelo,
            (_, _) => Self::Whilels,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Whilelt, Self::Whilele, Self::Whilelo, Self::Whilels];
}
