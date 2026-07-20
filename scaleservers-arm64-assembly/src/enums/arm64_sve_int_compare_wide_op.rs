// Copyright (c) Scaleservers LLC

/// An SVE **integer compare (wide elements)** op (DDI0487 part C). Each compares `Zn.<T>` (`.b`/`.h`/`.s`)
/// against the doubleword lanes of `Zm.D` under a governing predicate and writes a predicate result `Pd.<T>`.
/// Unlike the same-size register compares, the `LT`/`LE`/`LO`/`LS` forms are real encodings here (not
/// operand-swapped aliases). The op is a 3-bit field at `[15:13]` plus a low bit at `[4]`; the ten rows share
/// the `0x2400_0000` block with [`Arm64SveIntCompareOp`](crate::enums::Arm64SveIntCompareOp) and partition the
/// remaining `[15:13]` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveIntCompareWideOp {
    /// `CMPEQ` -- equal.
    Cmpeq,
    /// `CMPNE` -- not equal.
    Cmpne,
    /// `CMPGE` -- signed greater-or-equal.
    Cmpge,
    /// `CMPGT` -- signed greater-than.
    Cmpgt,
    /// `CMPLT` -- signed less-than.
    Cmplt,
    /// `CMPLE` -- signed less-or-equal.
    Cmple,
    /// `CMPHS` -- unsigned higher-or-same.
    Cmphs,
    /// `CMPHI` -- unsigned higher.
    Cmphi,
    /// `CMPLO` -- unsigned lower.
    Cmplo,
    /// `CMPLS` -- unsigned lower-or-same.
    Cmpls,
}

impl Arm64SveIntCompareWideOp {
    /// The 3-bit high field (`[15:13]`).
    pub fn op_high(self) -> u32 {
        match self {
            Self::Cmpeq | Self::Cmpne => 0b001,
            Self::Cmpge | Self::Cmpgt => 0b010,
            Self::Cmplt | Self::Cmple => 0b011,
            Self::Cmphs | Self::Cmphi => 0b110,
            Self::Cmplo | Self::Cmpls => 0b111,
        }
    }

    /// The low bit (`[4]`).
    pub fn op_low(self) -> u32 {
        match self {
            Self::Cmpeq | Self::Cmpge | Self::Cmplt | Self::Cmphs | Self::Cmplo => 0,
            Self::Cmpne | Self::Cmpgt | Self::Cmple | Self::Cmphi | Self::Cmpls => 1,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Cmpeq => "cmpeq",
            Self::Cmpne => "cmpne",
            Self::Cmpge => "cmpge",
            Self::Cmpgt => "cmpgt",
            Self::Cmplt => "cmplt",
            Self::Cmple => "cmple",
            Self::Cmphs => "cmphs",
            Self::Cmphi => "cmphi",
            Self::Cmplo => "cmplo",
            Self::Cmpls => "cmpls",
        }
    }

    /// Recover the op from its `(op_high, op_low)` bits, if a wide compare row.
    pub fn from_bits(op_high: u32, op_low: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.op_high() == op_high & 0b111 && op.op_low() == op_low & 1)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 10] = [
        Self::Cmpeq,
        Self::Cmpne,
        Self::Cmpge,
        Self::Cmpgt,
        Self::Cmplt,
        Self::Cmple,
        Self::Cmphs,
        Self::Cmphi,
        Self::Cmplo,
        Self::Cmpls,
    ];
}
