// Copyright (c) Scaleservers LLC

/// A FEAT_GCS **guarded-control-stack exception operation** -- the operand-free `GCSPUSHX`/`GCSPOPX`/`GCSPOPCX`
/// named aliases in the `SYS` space (`op1=3, CRn=7, CRm=7`; `Rt = 11111` fixed). Used by exception entry/exit to
/// push/pop a GCS exception record. Each is a fixed 32-bit word (unlike [`super::Arm64GcsRegisterOp`], these take
/// no register operand).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64GcsExceptionOp {
    /// `GCSPUSHX` -- push a GCS exception record (`SYS #3, C7, C7, #4`); `0xD508_779F`.
    Pushx,
    /// `GCSPOPCX` -- pop and compare a GCS exception record (`SYS #3, C7, C7, #5`); `0xD508_77BF`.
    Popcx,
    /// `GCSPOPX` -- pop a GCS exception record (`SYS #3, C7, C7, #6`); `0xD508_77DF`.
    Popx,
}

impl Arm64GcsExceptionOp {
    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Pushx, Self::Popcx, Self::Popx];

    /// The fixed instruction word. GNU+LLVM dual-oracle verified.
    pub fn word(self) -> u32 {
        match self {
            Self::Pushx => 0xD508_779F,
            Self::Popcx => 0xD508_77BF,
            Self::Popx => 0xD508_77DF,
        }
    }

    /// Recover the op from a full instruction word, if it names one of these.
    pub fn from_word(word: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.word() == word)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Pushx => "gcspushx",
            Self::Popcx => "gcspopcx",
            Self::Popx => "gcspopx",
        }
    }
}
