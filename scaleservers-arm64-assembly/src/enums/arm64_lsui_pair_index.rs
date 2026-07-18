// Copyright (c) Scaleservers LLC

/// The addressing mode of a FEAT_LSUI unprivileged SIMD&FP load/store pair (`LDTNP`/`STTNP`/`LDTP`/`STTP` of `Q`
/// registers). Mirrors the `idx[24:23]` field of the load/store-pair encoding: `00` = non-temporal (no
/// writeback), `01` = post-index, `10` = signed offset, `11` = pre-index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64LsuiPairIndex {
    /// `LDTNP`/`STTNP [Xn{, #imm}]` -- non-temporal (no writeback), `idx = 00`.
    NonTemporal,
    /// `LDTP`/`STTP [Xn], #imm` -- post-index, `idx = 01`.
    PostIndex,
    /// `LDTP`/`STTP [Xn{, #imm}]` -- signed offset, `idx = 10`.
    Offset,
    /// `LDTP`/`STTP [Xn, #imm]!` -- pre-index, `idx = 11`.
    PreIndex,
}

impl Arm64LsuiPairIndex {
    /// Every mode, for tests.
    pub const ALL: [Self; 4] = [
        Self::NonTemporal,
        Self::PostIndex,
        Self::Offset,
        Self::PreIndex,
    ];

    /// The 2-bit `idx` field value (`[24:23]`).
    pub fn idx_bits(self) -> u32 {
        match self {
            Self::NonTemporal => 0b00,
            Self::PostIndex => 0b01,
            Self::Offset => 0b10,
            Self::PreIndex => 0b11,
        }
    }

    /// Recover the mode from the 2-bit `idx` field.
    pub fn from_idx_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0b00 => Self::NonTemporal,
            0b01 => Self::PostIndex,
            0b10 => Self::Offset,
            _ => Self::PreIndex,
        }
    }

    /// Whether this is the non-temporal (`*TNP`) mode (vs the `*TP` indexed/offset modes).
    pub fn is_non_temporal(self) -> bool {
        matches!(self, Self::NonTemporal)
    }
}
