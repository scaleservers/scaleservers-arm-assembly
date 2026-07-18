// Copyright (c) Scaleservers LLC

/// An SVE **unpredicated integer binary** op over `Zd.<T>, Zn.<T>, Zm.<T>` (DDI0487 part C -- "SVE integer add/
/// subtract vectors (unpredicated)"). All share the base `0x0420_0000 | size<<22 | Zm<<16 | opc<<10 | Zn<<5 | Zd`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveIntBinUnpredOp {
    /// `ADD` -- vector add.
    Add,
    /// `SUB` -- vector subtract.
    Sub,
    /// `SQADD` -- signed saturating add.
    Sqadd,
    /// `UQADD` -- unsigned saturating add.
    Uqadd,
    /// `SQSUB` -- signed saturating subtract.
    Sqsub,
    /// `UQSUB` -- unsigned saturating subtract.
    Uqsub,
}

impl Arm64SveIntBinUnpredOp {
    /// The 3-bit `opcode` field (`[12:10]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Add => 0b000,
            Self::Sub => 0b001,
            Self::Sqadd => 0b100,
            Self::Uqadd => 0b101,
            Self::Sqsub => 0b110,
            Self::Uqsub => 0b111,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Sqadd => "sqadd",
            Self::Uqadd => "uqadd",
            Self::Sqsub => "sqsub",
            Self::Uqsub => "uqsub",
        }
    }

    /// Recover the op from its `opcode` field, if allocated (`010`/`011` are unallocated here).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0b111)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 6] = [
        Self::Add,
        Self::Sub,
        Self::Sqadd,
        Self::Uqadd,
        Self::Sqsub,
        Self::Uqsub,
    ];
}
