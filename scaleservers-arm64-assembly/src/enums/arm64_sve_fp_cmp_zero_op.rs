// Copyright (c) Scaleservers LLC

/// An SVE **floating-point compare with zero** op (DDI0487 part C). Each compares `Zn.<T>` against `#0.0` under a
/// governing predicate and writes a predicate result `Pd.<T>`; `FCMLT`/`FCMLE` exist ONLY in this
/// compare-against-zero form. Selected by a 3-bit `eq` field at `[18:16]` plus a low bit at `[4]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpCmpZeroOp {
    /// `FCMGE` -- greater-or-equal to zero.
    Fcmge,
    /// `FCMGT` -- greater-than zero.
    Fcmgt,
    /// `FCMLT` -- less-than zero.
    Fcmlt,
    /// `FCMLE` -- less-or-equal to zero.
    Fcmle,
    /// `FCMEQ` -- equal to zero.
    Fcmeq,
    /// `FCMNE` -- not equal to zero.
    Fcmne,
}

impl Arm64SveFpCmpZeroOp {
    /// The 3-bit `eq` field (`[18:16]`).
    pub fn op_high(self) -> u32 {
        match self {
            Self::Fcmge | Self::Fcmgt => 0b000,
            Self::Fcmlt | Self::Fcmle => 0b001,
            Self::Fcmeq => 0b010,
            Self::Fcmne => 0b011,
        }
    }

    /// The low bit (`[4]`): 0 = `ge`/`lt`/`eq`/`ne`, 1 = `gt`/`le`.
    pub fn op_low(self) -> u32 {
        match self {
            Self::Fcmge | Self::Fcmlt | Self::Fcmeq | Self::Fcmne => 0,
            Self::Fcmgt | Self::Fcmle => 1,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fcmge => "fcmge",
            Self::Fcmgt => "fcmgt",
            Self::Fcmlt => "fcmlt",
            Self::Fcmle => "fcmle",
            Self::Fcmeq => "fcmeq",
            Self::Fcmne => "fcmne",
        }
    }

    /// Recover the op from its `(op_high, op_low)` bits, or `None` for the unallocated pairs.
    pub fn from_bits(op_high: u32, op_low: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.op_high() == op_high & 0b111 && op.op_low() == op_low & 1)
    }

    /// Every op, for tests and table-driven iteration.
    pub const ALL: [Self; 6] = [
        Self::Fcmge,
        Self::Fcmgt,
        Self::Fcmlt,
        Self::Fcmle,
        Self::Fcmeq,
        Self::Fcmne,
    ];
}
