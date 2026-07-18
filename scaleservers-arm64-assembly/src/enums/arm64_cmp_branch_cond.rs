// Copyright (c) Scaleservers LLC

/// The comparison condition of a FEAT_CMPBR **register** compare-and-branch (`CB<cc> <Rn>, <Rm>, <label>` and the
/// byte/halfword `CBB`/`CBH` forms). The 3-bit `cc` field `[23:21]` selects the test; the `LT`/`LE`/`LO`/`LS`
/// conditions are assembler aliases that swap `Rn`/`Rm`, so the architectural (decoded) set is just these six.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64CmpBranchCond {
    /// `GT` -- signed greater-than (`cc = 0b000`).
    Gt,
    /// `GE` -- signed greater-or-equal (`cc = 0b001`).
    Ge,
    /// `HI` -- unsigned higher (`cc = 0b010`).
    Hi,
    /// `HS` -- unsigned higher-or-same (`cc = 0b011`).
    Hs,
    /// `EQ` -- equal (`cc = 0b110`).
    Eq,
    /// `NE` -- not equal (`cc = 0b111`).
    Ne,
}

impl Arm64CmpBranchCond {
    /// Every condition, for exhaustive round-trip tests.
    pub const ALL: [Self; 6] = [Self::Gt, Self::Ge, Self::Hi, Self::Hs, Self::Eq, Self::Ne];

    /// The 3-bit `cc` field value (`[23:21]`).
    pub fn bits(self) -> u32 {
        match self {
            Self::Gt => 0b000,
            Self::Ge => 0b001,
            Self::Hi => 0b010,
            Self::Hs => 0b011,
            Self::Eq => 0b110,
            Self::Ne => 0b111,
        }
    }

    /// Recover the condition from its 3-bit `cc` field; `None` for the unallocated `0b100`/`0b101` values.
    pub fn from_bits(bits: u32) -> Option<Self> {
        match bits & 0b111 {
            0b000 => Some(Self::Gt),
            0b001 => Some(Self::Ge),
            0b010 => Some(Self::Hi),
            0b011 => Some(Self::Hs),
            0b110 => Some(Self::Eq),
            0b111 => Some(Self::Ne),
            _ => None,
        }
    }

    /// The lowercase mnemonic suffix (`gt`/`ge`/`hi`/`hs`/`eq`/`ne`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Gt => "gt",
            Self::Ge => "ge",
            Self::Hi => "hi",
            Self::Hs => "hs",
            Self::Eq => "eq",
            Self::Ne => "ne",
        }
    }
}

/// The comparison condition of a FEAT_CMPBR **immediate** compare-and-branch (`CB<cc> <Rn>, #<imm6>, <label>`).
/// Shares the 3-bit `cc` field positions of [`Arm64CmpBranchCond`] but, because the second operand is an
/// immediate (not swappable), the architectural set is `GT`/`LT`/`HI`/`LO`/`EQ`/`NE`; `GE`/`LE`/`HS`/`LS` are
/// assembler aliases that adjust the immediate by 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64CmpBranchImmCond {
    /// `GT` -- signed greater-than (`cc = 0b000`).
    Gt,
    /// `LT` -- signed less-than (`cc = 0b001`).
    Lt,
    /// `HI` -- unsigned higher (`cc = 0b010`).
    Hi,
    /// `LO` -- unsigned lower (`cc = 0b011`).
    Lo,
    /// `EQ` -- equal (`cc = 0b110`).
    Eq,
    /// `NE` -- not equal (`cc = 0b111`).
    Ne,
}

impl Arm64CmpBranchImmCond {
    /// Every condition, for exhaustive round-trip tests.
    pub const ALL: [Self; 6] = [Self::Gt, Self::Lt, Self::Hi, Self::Lo, Self::Eq, Self::Ne];

    /// The 3-bit `cc` field value (`[23:21]`).
    pub fn bits(self) -> u32 {
        match self {
            Self::Gt => 0b000,
            Self::Lt => 0b001,
            Self::Hi => 0b010,
            Self::Lo => 0b011,
            Self::Eq => 0b110,
            Self::Ne => 0b111,
        }
    }

    /// Recover the condition from its 3-bit `cc` field; `None` for the unallocated `0b100`/`0b101` values.
    pub fn from_bits(bits: u32) -> Option<Self> {
        match bits & 0b111 {
            0b000 => Some(Self::Gt),
            0b001 => Some(Self::Lt),
            0b010 => Some(Self::Hi),
            0b011 => Some(Self::Lo),
            0b110 => Some(Self::Eq),
            0b111 => Some(Self::Ne),
            _ => None,
        }
    }

    /// The lowercase mnemonic suffix (`gt`/`lt`/`hi`/`lo`/`eq`/`ne`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Gt => "gt",
            Self::Lt => "lt",
            Self::Hi => "hi",
            Self::Lo => "lo",
            Self::Eq => "eq",
            Self::Ne => "ne",
        }
    }
}
