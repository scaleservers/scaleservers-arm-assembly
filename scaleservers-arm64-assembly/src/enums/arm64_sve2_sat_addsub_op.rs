// Copyright (c) Scaleservers LLC

/// An SVE2 **saturating add/subtract (predicated)** op (DDI0487 C4.1): `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>`,
/// destructive and saturating. Selected by the 3-bit `op:S:U` field at `[18:16]`. (These are the predicated
/// counterparts of the SVE1 unpredicated `SQADD`/`UQADD`/... ; `SUQADD`/`USQADD` are the mixed-sign forms.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2SatAddSubOp {
    Sqadd,
    Uqadd,
    Sqsub,
    Uqsub,
    Suqadd,
    Usqadd,
    Sqsubr,
    Uqsubr,
}

impl Arm64Sve2SatAddSubOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sqadd => "sqadd",
            Self::Uqadd => "uqadd",
            Self::Sqsub => "sqsub",
            Self::Uqsub => "uqsub",
            Self::Suqadd => "suqadd",
            Self::Usqadd => "usqadd",
            Self::Sqsubr => "sqsubr",
            Self::Uqsubr => "uqsubr",
        }
    }

    /// The 3-bit `op:S:U` opcode (`[18:16]`).
    pub fn code(self) -> u32 {
        match self {
            Self::Sqadd => 0b000,
            Self::Uqadd => 0b001,
            Self::Sqsub => 0b010,
            Self::Uqsub => 0b011,
            Self::Suqadd => 0b100,
            Self::Usqadd => 0b101,
            Self::Sqsubr => 0b110,
            Self::Uqsubr => 0b111,
        }
    }

    /// Recover the op from its `[18:16]` code.
    pub fn from_code(code: u32) -> Self {
        Self::ALL[(code & 0b111) as usize]
    }

    /// Every op, indexed by code.
    pub const ALL: [Self; 8] = [
        Self::Sqadd,
        Self::Uqadd,
        Self::Sqsub,
        Self::Uqsub,
        Self::Suqadd,
        Self::Usqadd,
        Self::Sqsubr,
        Self::Uqsubr,
    ];
}
