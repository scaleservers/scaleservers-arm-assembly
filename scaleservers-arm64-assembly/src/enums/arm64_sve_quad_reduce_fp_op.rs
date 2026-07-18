// Copyright (c) Scaleservers LLC

/// An SVE2.1 **across-lanes quadword floating-point reduction** op (FEAT_SVE2p1; DDI0487 part C). Reduces each
/// 128-bit segment of the source `Zn` to one lane of a 128-bit NEON `Vd` register under a governing predicate.
/// base `0x6400_A000 | size<<22 | opcode<<16 | Pg<<10 | Zn<<5 | Vd`. Valid for `.h`/`.s`/`.d` only. GNU+LLVM verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveQuadReduceFpOp {
    /// `FADDQV` -- add (sum) reduction.
    FaddQV,
    /// `FMAXNMQV` -- maximum-number reduction.
    FmaxnmQV,
    /// `FMINNMQV` -- minimum-number reduction.
    FminnmQV,
    /// `FMAXQV` -- maximum reduction.
    FmaxQV,
    /// `FMINQV` -- minimum reduction.
    FminQV,
}

impl Arm64SveQuadReduceFpOp {
    /// The 5-bit `opcode` field (`[20:16]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::FaddQV => 0x10,
            Self::FmaxnmQV => 0x14,
            Self::FminnmQV => 0x15,
            Self::FmaxQV => 0x16,
            Self::FminQV => 0x17,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::FaddQV => "faddqv",
            Self::FmaxnmQV => "fmaxnmqv",
            Self::FminnmQV => "fminnmqv",
            Self::FmaxQV => "fmaxqv",
            Self::FminQV => "fminqv",
        }
    }

    /// Recover the op from its `opcode`, if a modeled FP QV op.
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0x1F)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 5] = [
        Self::FaddQV,
        Self::FmaxnmQV,
        Self::FminnmQV,
        Self::FmaxQV,
        Self::FminQV,
    ];
}
