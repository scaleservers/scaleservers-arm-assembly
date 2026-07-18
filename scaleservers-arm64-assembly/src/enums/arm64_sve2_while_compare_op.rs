// Copyright (c) Scaleservers LLC

/// An SVE2 **while greater** loop-predicate op (DDI0487 part C): the SVE2 counterparts of the SVE1 `WHILELT` family,
/// generating a predicate true for the leading elements while the decrementing operand satisfies a `>`/`>=`
/// comparison. `WHILEGE`/`WHILEGT` are signed; `WHILEHS`/`WHILEHI` are unsigned. These share the `WHILE` frame but
/// with `[10]`=0 (vs 1 for the `WHILELT` family).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2WhileCompareOp {
    /// `WHILEGE` -- while signed greater-than-or-equal.
    Whilege,
    /// `WHILEGT` -- while signed greater-than.
    Whilegt,
    /// `WHILEHS` -- while unsigned higher-or-same (`>=`).
    Whilehs,
    /// `WHILEHI` -- while unsigned higher (`>`).
    Whilehi,
}

impl Arm64Sve2WhileCompareOp {
    /// The `U` bit (`[11]`): 0 = signed, 1 = unsigned.
    pub fn unsigned_bit(self) -> u32 {
        match self {
            Self::Whilege | Self::Whilegt => 0,
            Self::Whilehs | Self::Whilehi => 1,
        }
    }

    /// The `[4]` bit: 1 for the strict comparisons (`WHILEGT`/`WHILEHI`).
    pub fn strict_bit(self) -> u32 {
        match self {
            Self::Whilege | Self::Whilehs => 0,
            Self::Whilegt | Self::Whilehi => 1,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Whilege => "whilege",
            Self::Whilegt => "whilegt",
            Self::Whilehs => "whilehs",
            Self::Whilehi => "whilehi",
        }
    }

    /// Recover the op from its `(U, strict)` bits.
    pub fn from_bits(unsigned: u32, strict: u32) -> Self {
        match (unsigned & 1, strict & 1) {
            (0, 0) => Self::Whilege,
            (0, _) => Self::Whilegt,
            (_, 0) => Self::Whilehs,
            (_, _) => Self::Whilehi,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Whilege, Self::Whilegt, Self::Whilehs, Self::Whilehi];
}
