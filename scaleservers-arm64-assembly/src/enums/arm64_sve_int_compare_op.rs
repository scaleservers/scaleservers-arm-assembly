// Copyright (c) Scaleservers LLC

/// An SVE **integer compare (vectors)** op (DDI0487 part C). Each compares `Zn.<T>` against `Zm.<T>` under a
/// governing predicate and writes a predicate result `Pd.<T>`. The op is selected by a 3-bit field at `[15:13]`
/// and a low bit at `[4]`: signed `CMPGE`/`CMPGT`, unsigned `CMPHS`/`CMPHI`, and `CMPEQ`/`CMPNE`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveIntCompareOp {
    /// `CMPHS` -- unsigned higher-or-same (`>=`).
    Cmphs,
    /// `CMPHI` -- unsigned higher (`>`).
    Cmphi,
    /// `CMPGE` -- signed greater-or-equal.
    Cmpge,
    /// `CMPGT` -- signed greater-than.
    Cmpgt,
    /// `CMPEQ` -- equal.
    Cmpeq,
    /// `CMPNE` -- not equal.
    Cmpne,
}

impl Arm64SveIntCompareOp {
    /// The 3-bit high field (`[15:13]`).
    pub fn op_high(self) -> u32 {
        match self {
            Self::Cmphs | Self::Cmphi => 0b000,
            Self::Cmpge | Self::Cmpgt => 0b100,
            Self::Cmpeq | Self::Cmpne => 0b101,
        }
    }

    /// The low bit (`[4]`): 0 = the `hs`/`ge`/`eq` form, 1 = the `hi`/`gt`/`ne` form.
    pub fn op_low(self) -> u32 {
        match self {
            Self::Cmphs | Self::Cmpge | Self::Cmpeq => 0,
            Self::Cmphi | Self::Cmpgt | Self::Cmpne => 1,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Cmphs => "cmphs",
            Self::Cmphi => "cmphi",
            Self::Cmpge => "cmpge",
            Self::Cmpgt => "cmpgt",
            Self::Cmpeq => "cmpeq",
            Self::Cmpne => "cmpne",
        }
    }

    /// Recover the op from its `(op_high, op_low)` bits, if a modeled compare.
    pub fn from_bits(op_high: u32, op_low: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.op_high() == op_high & 0b111 && op.op_low() == op_low & 1)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 6] = [
        Self::Cmphs,
        Self::Cmphi,
        Self::Cmpge,
        Self::Cmpgt,
        Self::Cmpeq,
        Self::Cmpne,
    ];
}
