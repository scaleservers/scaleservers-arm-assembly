// Copyright (c) Scaleservers LLC

/// An AArch64 operand-free **hint** in the reserved-hint (`NOP`) space (DDI0487 C6 -- `HINT #<imm>`). Each is a
/// 7-bit hint number `CRm:op2` placed at `[11:5]`; the word is `0xD503_201F | (hint_number << 5)`. `NOP` (hint 0)
/// and the pointer-auth / `BTI` hints are modeled separately, so they are not repeated here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SystemHintOp {
    /// `YIELD` -- hint that the thread is in a spin-wait (hint 1).
    Yield,
    /// `WFE` -- wait for event (hint 2).
    Wfe,
    /// `WFI` -- wait for interrupt (hint 3).
    Wfi,
    /// `SEV` -- send event (hint 4).
    Sev,
    /// `SEVL` -- send event local (hint 5).
    Sevl,
    /// `DGH` -- data gathering hint (FEAT_DGH, hint 6).
    Dgh,
    /// `ESB` -- error synchronization barrier (FEAT_RAS, hint 16).
    Esb,
    /// `PSB CSYNC` -- profiling synchronization barrier (FEAT_SPE, hint 17).
    PsbCsync,
    /// `TSB CSYNC` -- trace synchronization barrier (FEAT_TRF, hint 18).
    TsbCsync,
    /// `GCSB DSYNC` -- guarded-control-stack data synchronization barrier (FEAT_GCS, hint 19).
    GcsbDsync,
    /// `CSDB` -- consumption of speculative data barrier (hint 20).
    Csdb,
    /// `CLRBHB` -- clear branch history (FEAT_CLRBHB, hint 22).
    Clrbhb,
    /// `CHKFEAT X16` -- query enabled features (FEAT_CHK, hint 40). `X16` is an implicit, fixed operand: the bit
    /// mask of features to check is read from / the result written back to `X16`, so it is not a separate field.
    ChkfeatX16,
}

impl Arm64SystemHintOp {
    /// The 7-bit hint number (`CRm:op2`).
    pub fn hint_number(self) -> u32 {
        match self {
            Self::Yield => 1,
            Self::Wfe => 2,
            Self::Wfi => 3,
            Self::Sev => 4,
            Self::Sevl => 5,
            Self::Dgh => 6,
            Self::Esb => 16,
            Self::PsbCsync => 17,
            Self::TsbCsync => 18,
            Self::GcsbDsync => 19,
            Self::Csdb => 20,
            Self::Clrbhb => 22,
            Self::ChkfeatX16 => 40,
        }
    }

    /// The full 32-bit encoding (`0xD503_201F | hint_number << 5`).
    pub fn word(self) -> u32 {
        0xD503_201F | (self.hint_number() << 5)
    }

    /// The lowercase UAL mnemonic (the multi-token hints render their full text).
    pub fn name(self) -> &'static str {
        match self {
            Self::Yield => "yield",
            Self::Wfe => "wfe",
            Self::Wfi => "wfi",
            Self::Sev => "sev",
            Self::Sevl => "sevl",
            Self::Dgh => "dgh",
            Self::Esb => "esb",
            Self::PsbCsync => "psb csync",
            Self::TsbCsync => "tsb csync",
            Self::GcsbDsync => "gcsb dsync",
            Self::Csdb => "csdb",
            Self::Clrbhb => "clrbhb",
            Self::ChkfeatX16 => "chkfeat x16",
        }
    }

    /// Recover the hint from a full instruction word, if it is one of the modeled hints.
    pub fn from_word(word: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.word() == word)
    }

    /// Every modeled hint, for tests and table-driven decode.
    pub const ALL: [Self; 13] = [
        Self::Yield,
        Self::Wfe,
        Self::Wfi,
        Self::Sev,
        Self::Sevl,
        Self::Dgh,
        Self::Esb,
        Self::PsbCsync,
        Self::TsbCsync,
        Self::GcsbDsync,
        Self::Csdb,
        Self::Clrbhb,
        Self::ChkfeatX16,
    ];
}
