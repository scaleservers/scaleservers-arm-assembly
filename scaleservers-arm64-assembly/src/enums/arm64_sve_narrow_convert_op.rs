// Copyright (c) Scaleservers LLC

/// An SVE2.1 **two-vector saturating narrowing convert** op (FEAT_SVE2p1; DDI0487 part C). Narrows a consecutive
/// `.s` Z-register pair to a single `.h` Z register with signed/unsigned saturation. The op is the `[12:11]` selector
/// of the shared frame. GNU+LLVM verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveNarrowConvertOp {
    /// `SQCVTN` -- signed saturating extract narrow.
    Sqcvtn,
    /// `UQCVTN` -- unsigned saturating extract narrow.
    Uqcvtn,
    /// `SQCVTUN` -- signed-to-unsigned saturating extract narrow.
    Sqcvtun,
}

impl Arm64SveNarrowConvertOp {
    /// The `[12:11]` op-selector value.
    pub fn code(self) -> u32 {
        match self {
            Self::Sqcvtn => 0b00,
            Self::Uqcvtn => 0b01,
            Self::Sqcvtun => 0b10,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Sqcvtn => "sqcvtn",
            Self::Uqcvtn => "uqcvtn",
            Self::Sqcvtun => "sqcvtun",
        }
    }

    /// Recover the op from the `[12:11]` selector; `None` for the unallocated value `3`.
    pub fn from_code(code: u32) -> Option<Self> {
        Some(match code & 0b11 {
            0b00 => Self::Sqcvtn,
            0b01 => Self::Uqcvtn,
            0b10 => Self::Sqcvtun,
            _ => return None,
        })
    }

    /// The lowercase UAL mnemonic of the matching SME2 NON-interleaving (no-`N`) form (`SQCVT`/`UQCVT`/`SQCVTU`).
    pub fn no_n_mnemonic(self) -> &'static str {
        match self {
            Self::Sqcvtn => "sqcvt",
            Self::Uqcvtn => "uqcvt",
            Self::Sqcvtun => "sqcvtu",
        }
    }

    /// The SME2 four-vector narrowing-convert op-bit split: `[22]` (to-unsigned) OR-ed with `[5]` (unsigned source).
    /// (The two-vector form instead uses the contiguous `[12:11]` [`Self::code`].)
    pub fn sme2_op_bits(self) -> u32 {
        match self {
            Self::Sqcvtn => 0,
            Self::Uqcvtn => 1 << 5,
            Self::Sqcvtun => 1 << 22,
        }
    }

    /// Recover the op from the SME2 four-vector `[22]`(to-unsigned)/`[5]`(unsigned-source) split; `None` for the
    /// unallocated `(1, 1)` combo.
    pub fn from_sme2_op_bits(to_unsigned: u32, unsigned_src: u32) -> Option<Self> {
        Some(match (to_unsigned & 1, unsigned_src & 1) {
            (0, 0) => Self::Sqcvtn,
            (0, 1) => Self::Uqcvtn,
            (1, 0) => Self::Sqcvtun,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Sqcvtn, Self::Uqcvtn, Self::Sqcvtun];
}
