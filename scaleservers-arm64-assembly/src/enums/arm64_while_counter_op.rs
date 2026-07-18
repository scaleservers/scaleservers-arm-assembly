// Copyright (c) Scaleservers LLC

/// The comparison of an SVE2.1 `WHILE`-to-predicate-as-counter instruction (`WHILEGE`/`WHILEGT`/`WHILELT`/`WHILELE` --
/// signed; `WHILEHS`/`WHILEHI`/`WHILELO`/`WHILELS` -- unsigned). The op is encoded as the `[11:10]` category plus the
/// `[3]` strict/equal bit of the shared frame; GNU+LLVM verified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64WhileCounterOp {
    /// `WHILEGE` -- signed greater-than-or-equal.
    Ge,
    /// `WHILEGT` -- signed greater-than.
    Gt,
    /// `WHILELT` -- signed less-than.
    Lt,
    /// `WHILELE` -- signed less-than-or-equal.
    Le,
    /// `WHILEHS` -- unsigned higher-or-same.
    Hs,
    /// `WHILEHI` -- unsigned higher.
    Hi,
    /// `WHILELO` -- unsigned lower.
    Lo,
    /// `WHILELS` -- unsigned lower-or-same.
    Ls,
}

impl Arm64WhileCounterOp {
    /// The `[11:10]` comparison category (signed-ge/gt=0, signed-lt/le=1, unsigned-hs/hi=2, unsigned-lo/ls=3).
    pub const fn category(self) -> u32 {
        match self {
            Self::Ge | Self::Gt => 0b00,
            Self::Lt | Self::Le => 0b01,
            Self::Hs | Self::Hi => 0b10,
            Self::Lo | Self::Ls => 0b11,
        }
    }

    /// The strict/equal bit (set for the `..gt`/`..le`/`..hi`/`..ls` variant of each category).
    pub const fn eq_bit(self) -> u32 {
        matches!(self, Self::Gt | Self::Le | Self::Hi | Self::Ls) as u32
    }

    /// The op-discriminant bits for the predicate-as-counter form: the `[11:10]` category and the `[3]` strict bit.
    pub const fn discriminant(self) -> u32 {
        (self.category() << 10) | (self.eq_bit() << 3)
    }

    /// Recover the op from the `[11:10]` category and the `[3]` bit. Total -- all eight combinations are allocated.
    pub const fn from_bits(cat: u32, eq: u32) -> Self {
        match (cat & 0b11, eq & 1) {
            (0b00, 0) => Self::Ge,
            (0b00, 1) => Self::Gt,
            (0b01, 0) => Self::Lt,
            (0b01, 1) => Self::Le,
            (0b10, 0) => Self::Hs,
            (0b10, 1) => Self::Hi,
            (0b11, 0) => Self::Lo,
            _ => Self::Ls,
        }
    }

    /// The UAL mnemonic.
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Ge => "whilege",
            Self::Gt => "whilegt",
            Self::Lt => "whilelt",
            Self::Le => "whilele",
            Self::Hs => "whilehs",
            Self::Hi => "whilehi",
            Self::Lo => "whilelo",
            Self::Ls => "whilels",
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 8] = [
        Self::Ge,
        Self::Gt,
        Self::Lt,
        Self::Le,
        Self::Hs,
        Self::Hi,
        Self::Lo,
        Self::Ls,
    ];
}
