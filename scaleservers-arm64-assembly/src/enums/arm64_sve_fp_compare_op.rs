// Copyright (c) Scaleservers LLC

/// An SVE **floating-point compare (vectors)** op (DDI0487 part C). Each compares `Zn.<T>` against `Zm.<T>` under a
/// governing predicate and writes a predicate result `Pd.<T>`. Selected by a 3-bit field at `[15:13]` plus a low
/// bit at `[4]`: `FCMGE`/`FCMGT`, `FCMEQ`/`FCMNE`, and the unordered `FCMUO`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpCompareOp {
    /// `FCMGE` -- greater-or-equal.
    Fcmge,
    /// `FCMGT` -- greater-than.
    Fcmgt,
    /// `FCMEQ` -- equal.
    Fcmeq,
    /// `FCMNE` -- not equal.
    Fcmne,
    /// `FCMUO` -- unordered (either operand NaN).
    Fcmuo,
    /// `FACGE` -- absolute compare greater-or-equal (`|Zn| >= |Zm|`).
    Facge,
    /// `FACGT` -- absolute compare greater-than (`|Zn| > |Zm|`).
    Facgt,
}

impl Arm64SveFpCompareOp {
    /// The 3-bit high field (`[15:13]`).
    pub fn op_high(self) -> u32 {
        match self {
            Self::Fcmge | Self::Fcmgt => 0b010,
            Self::Fcmeq | Self::Fcmne => 0b011,
            Self::Fcmuo | Self::Facge => 0b110,
            Self::Facgt => 0b111,
        }
    }

    /// The low bit (`[4]`): 0 = `ge`/`eq`/`uo`, 1 = `gt`/`ne`/`acge`/`acgt`.
    pub fn op_low(self) -> u32 {
        match self {
            Self::Fcmge | Self::Fcmeq | Self::Fcmuo => 0,
            Self::Fcmgt | Self::Fcmne | Self::Facge | Self::Facgt => 1,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fcmge => "fcmge",
            Self::Fcmgt => "fcmgt",
            Self::Fcmeq => "fcmeq",
            Self::Fcmne => "fcmne",
            Self::Fcmuo => "fcmuo",
            Self::Facge => "facge",
            Self::Facgt => "facgt",
        }
    }

    /// Recover the op from its `(op_high, op_low)` bits, if a modeled compare.
    pub fn from_bits(op_high: u32, op_low: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.op_high() == op_high & 0b111 && op.op_low() == op_low & 1)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 7] = [
        Self::Fcmge,
        Self::Fcmgt,
        Self::Fcmeq,
        Self::Fcmne,
        Self::Fcmuo,
        Self::Facge,
        Self::Facgt,
    ];
}
