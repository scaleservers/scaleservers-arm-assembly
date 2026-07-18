// Copyright (c) Scaleservers LLC

/// A FEAT_GCS **guarded-control-stack register operation** -- the `GCSPUSHM`/`GCSPOPM`/`GCSSS1`/`GCSSS2` named
/// aliases that live in the `SYS`/`SYSL` space at `op1=3`, `CRn=7`, `CRm=7` (DDI0487 C6.2). Each acts on a single
/// `Xt` operand; `op2` selects the operation, and the SYS-vs-SYSL choice (whether `Xt` is a source or a result)
/// is implied by the operation. The full word is `<SYS|SYSL base> | (3<<16) | (7<<12) | (7<<8) | (op2<<5) | Rt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64GcsRegisterOp {
    /// `GCSPUSHM Xt` -- push `Xt` onto the current guarded control stack (`SYS #3, C7, C7, #0, Xt`).
    Pushm,
    /// `GCSPOPM Xt` -- pop the top of the guarded control stack into `Xt` (`SYSL Xt, #3, C7, C7, #1`).
    Popm,
    /// `GCSSS1 Xt` -- guarded-control-stack stack-switch, step 1 (`SYS #3, C7, C7, #2, Xt`).
    Ss1,
    /// `GCSSS2 Xt` -- guarded-control-stack stack-switch, step 2 (`SYSL Xt, #3, C7, C7, #3`).
    Ss2,
}

impl Arm64GcsRegisterOp {
    /// `true` for the `SYSL` (result-into-`Xt`) forms `GCSPOPM`/`GCSSS2`; `false` for the `SYS` (source-`Xt`)
    /// forms `GCSPUSHM`/`GCSSS1`.
    pub fn is_result(self) -> bool {
        matches!(self, Self::Popm | Self::Ss2)
    }

    /// The 3-bit `op2` selector (`[7:5]`).
    pub fn op2(self) -> u32 {
        match self {
            Self::Pushm => 0,
            Self::Popm => 1,
            Self::Ss1 => 2,
            Self::Ss2 => 3,
        }
    }

    /// The full 32-bit encoding with `Rt = 0` (the operand register is ORed in by the caller). `SYS` base is
    /// `0xD508_0000`, `SYSL` base `0xD528_0000`; both carry `op1=3, CRn=7, CRm=7` here.
    pub fn base(self) -> u32 {
        let frame = if self.is_result() {
            0xD528_0000
        } else {
            0xD508_0000
        };
        frame | (3 << 16) | (7 << 12) | (7 << 8) | (self.op2() << 5)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Pushm => "gcspushm",
            Self::Popm => "gcspopm",
            Self::Ss1 => "gcsss1",
            Self::Ss2 => "gcsss2",
        }
    }

    /// Recover the operation from a full instruction word with its `Rt[4:0]` field already cleared, if it names
    /// one of these GCS register ops.
    pub fn from_word_without_rt(word_without_rt: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.base() == word_without_rt)
    }

    /// Every modeled operation, for tests and table-driven decode.
    pub const ALL: [Self; 4] = [Self::Pushm, Self::Popm, Self::Ss1, Self::Ss2];
}
