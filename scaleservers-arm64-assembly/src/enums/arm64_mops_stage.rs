// Copyright (c) Scaleservers LLC

/// The stage of an FEAT_MOPS memory-copy/set operation. Each `memcpy`/`memset` is a triple -- a **prologue** that
/// preconditions the operation, a **main** body that does the bulk transfer, and an **epilogue** that finishes it --
/// and the three are encoded with a 2-bit stage selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64MopsStage {
    /// `P` -- prologue.
    Prologue,
    /// `M` -- main.
    Main,
    /// `E` -- epilogue.
    Epilogue,
}

impl Arm64MopsStage {
    /// The 2-bit stage code (`P`=00, `M`=01, `E`=10).
    pub const fn code(self) -> u32 {
        match self {
            Self::Prologue => 0,
            Self::Main => 1,
            Self::Epilogue => 2,
        }
    }

    /// The mnemonic stage letter (`p`/`m`/`e`).
    pub const fn letter(self) -> &'static str {
        match self {
            Self::Prologue => "p",
            Self::Main => "m",
            Self::Epilogue => "e",
        }
    }

    /// Recover the stage from its 2-bit code, or `None` for the unallocated `11`.
    pub const fn from_code(code: u32) -> Option<Self> {
        match code & 0b11 {
            0 => Some(Self::Prologue),
            1 => Some(Self::Main),
            2 => Some(Self::Epilogue),
            _ => None,
        }
    }

    /// Every stage, for tests.
    pub const ALL: [Self; 3] = [Self::Prologue, Self::Main, Self::Epilogue];
}
