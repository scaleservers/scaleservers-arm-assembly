// Copyright (c) Scaleservers LLC

/// An SVE2 **integer halving add/subtract (predicated)** op (DDI0487 C4.1): `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>`,
/// computing `(a +/- b) >> 1` (optionally rounding). Selected by the 3-bit `R:S:U` field at `[18:16]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2HalvingOp {
    Shadd,
    Uhadd,
    Shsub,
    Uhsub,
    Srhadd,
    Urhadd,
    Shsubr,
    Uhsubr,
}

impl Arm64Sve2HalvingOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Shadd => "shadd",
            Self::Uhadd => "uhadd",
            Self::Shsub => "shsub",
            Self::Uhsub => "uhsub",
            Self::Srhadd => "srhadd",
            Self::Urhadd => "urhadd",
            Self::Shsubr => "shsubr",
            Self::Uhsubr => "uhsubr",
        }
    }

    /// The 3-bit `R:S:U` opcode (`[18:16]`).
    pub fn code(self) -> u32 {
        match self {
            Self::Shadd => 0b000,
            Self::Uhadd => 0b001,
            Self::Shsub => 0b010,
            Self::Uhsub => 0b011,
            Self::Srhadd => 0b100,
            Self::Urhadd => 0b101,
            Self::Shsubr => 0b110,
            Self::Uhsubr => 0b111,
        }
    }

    /// Recover the op from its `[18:16]` code.
    pub fn from_code(code: u32) -> Self {
        Self::ALL[(code & 0b111) as usize]
    }

    /// Every op, indexed by code.
    pub const ALL: [Self; 8] = [
        Self::Shadd,
        Self::Uhadd,
        Self::Shsub,
        Self::Uhsub,
        Self::Srhadd,
        Self::Urhadd,
        Self::Shsubr,
        Self::Uhsubr,
    ];
}
