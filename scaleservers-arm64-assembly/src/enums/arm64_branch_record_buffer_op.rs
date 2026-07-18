// Copyright (c) Scaleservers LLC

/// A FEAT_BRBE **branch-record-buffer maintenance operation** -- the `BRB IALL` / `BRB INJ` named aliases in the
/// `SYS` space at `op1=1`, `CRn=7`, `CRm=2` (DDI0487 C6.2). Both are operand-free (`Rt` is fixed `0b11111`); `op2`
/// selects the operation. The full word is `SYS_BASE | (1<<16) | (7<<12) | (2<<8) | (op2<<5) | 0b11111`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64BranchRecordBufferOp {
    /// `BRB IALL` -- invalidate all branch records (`SYS #1, C7, C2, #4`).
    Iall,
    /// `BRB INJ` -- inject a branch record from the BRBINF/BRBSRC/BRBTGT system registers (`SYS #1, C7, C2, #5`).
    Inj,
}

impl Arm64BranchRecordBufferOp {
    /// The full 32-bit encoding (operand-free: `Rt = 0b11111`).
    pub fn word(self) -> u32 {
        let op2 = match self {
            Self::Iall => 4,
            Self::Inj => 5,
        };
        0xD508_0000 | (1 << 16) | (7 << 12) | (2 << 8) | (op2 << 5) | 0b11111
    }

    /// The lowercase second token of the UAL mnemonic (the family mnemonic is `brb`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Iall => "iall",
            Self::Inj => "inj",
        }
    }

    /// Recover the operation from a full instruction word, if it is one of these `BRB` aliases.
    pub fn from_word(word: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.word() == word)
    }

    /// Every modeled operation, for tests and table-driven decode.
    pub const ALL: [Self; 2] = [Self::Iall, Self::Inj];
}
