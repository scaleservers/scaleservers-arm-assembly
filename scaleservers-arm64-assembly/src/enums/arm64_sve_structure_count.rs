// Copyright (c) Scaleservers LLC

/// The structure count of an SVE **structured load/store** (`LD2`/`LD3`/`LD4`, `ST2`/`ST3`/`ST4`): how many
/// consecutive Z registers are de-interleaved / interleaved (DDI0487 part C). Encoded in the 2-bit field `[22:21]`
/// as `count - 1` (so `LD2`=`01`, `LD3`=`10`, `LD4`=`11`); the value `00` is NOT a structured form -- it is the
/// single-register `LDNT1`/contiguous space, so [`Self::from_bits`] returns `None` there (which is what keeps the
/// structured decoder from claiming an `LDNT1` word).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveStructureCount {
    /// `LD2`/`ST2` -- two registers.
    Two,
    /// `LD3`/`ST3` -- three registers.
    Three,
    /// `LD4`/`ST4` -- four registers.
    Four,
}

impl Arm64SveStructureCount {
    /// The 2-bit `[22:21]` field value (`count - 1`).
    pub fn count_bits(self) -> u32 {
        match self {
            Self::Two => 0b01,
            Self::Three => 0b10,
            Self::Four => 0b11,
        }
    }

    /// The number of registers in the list (2, 3, or 4).
    pub fn num_regs(self) -> u32 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    /// The mnemonic digit (`"2"`/`"3"`/`"4"`).
    pub fn digit(self) -> &'static str {
        match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
        }
    }

    /// Recover the count from the 2-bit field, or `None` for `00` (the single-register `LDNT1`/contiguous space).
    pub fn from_bits(bits: u32) -> Option<Self> {
        match bits & 0b11 {
            0b01 => Some(Self::Two),
            0b10 => Some(Self::Three),
            0b11 => Some(Self::Four),
            _ => None,
        }
    }

    /// Every count, for tests and table-driven iteration.
    pub const ALL: [Self; 3] = [Self::Two, Self::Three, Self::Four];
}
