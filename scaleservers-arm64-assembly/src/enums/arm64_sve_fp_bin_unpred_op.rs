// Copyright (c) Scaleservers LLC

/// An SVE **unpredicated floating-point binary** op over `Zd.<T>, Zn.<T>, Zm.<T>` (DDI0487 part C). Base
/// `0x6500_0000 | size<<22 | Zm<<16 | opcode<<10 | Zn<<5 | Zd`, opcode at `[12:10]`. Valid for `.h`/`.s`/`.d`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpBinUnpredOp {
    /// `FADD` -- add.
    Fadd,
    /// `FSUB` -- subtract.
    Fsub,
    /// `FMUL` -- multiply.
    Fmul,
    /// `FTSMUL` -- trigonometric starting-value multiply.
    Ftsmul,
    /// `FRECPS` -- reciprocal step (`2 - Zn*Zm`).
    Frecps,
    /// `FRSQRTS` -- reciprocal square-root step (`(3 - Zn*Zm) / 2`).
    Frsqrts,
}

impl Arm64SveFpBinUnpredOp {
    /// The 3-bit `opcode` field (`[12:10]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Fadd => 0,
            Self::Fsub => 1,
            Self::Fmul => 2,
            Self::Ftsmul => 3,
            Self::Frecps => 6,
            Self::Frsqrts => 7,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fadd => "fadd",
            Self::Fsub => "fsub",
            Self::Fmul => "fmul",
            Self::Ftsmul => "ftsmul",
            Self::Frecps => "frecps",
            Self::Frsqrts => "frsqrts",
        }
    }

    /// Recover the op from its `opcode`, if a modeled op (`4`/`5` are unallocated here).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.opcode() == opcode & 0x7)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 6] = [
        Self::Fadd,
        Self::Fsub,
        Self::Fmul,
        Self::Ftsmul,
        Self::Frecps,
        Self::Frsqrts,
    ];
}
