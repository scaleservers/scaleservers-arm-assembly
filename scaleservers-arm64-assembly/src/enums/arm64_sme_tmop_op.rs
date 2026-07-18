// Copyright (c) Scaleservers LLC

/// The operation of an SME2 sparse (transposed) outer product (`*TMOPA`, FEAT_SME_TMOP and friends). These
/// accumulate a sparse outer product of a 2-vector `Zn` source list and a single `Zm`, selected by an indexed
/// `Zk` sparsity register, into a `ZA` tile.
///
/// The mnemonic + element types determine the op-specific bits `{[24], [21], [15], [3]}` (a 4-bit selector
/// within the mask `0x0120_8008`): `[24]`/`[21]` are the signedness / FP-source selectors, `[15]` marks the
/// integer forms, and `[3]` selects a `.h` (FP16) destination tile over `.s` (FP32). The remaining forms beyond
/// the six FEAT_SME_TMOP ops are the FP8 (`Zn.B`) and FP16-destination variants, gated by FEAT_SME_F8F32 /
/// FEAT_SME_F8F16 / FEAT_SME_F16F16.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SmeTmopOp {
    /// `STMOPA` -- signed-int8 x signed-int8 -> `.s` (FEAT_SME_TMOP).
    Stmopa,
    /// `UTMOPA` -- unsigned-int8 x unsigned-int8 -> `.s` (FEAT_SME_TMOP).
    Utmopa,
    /// `USTMOPA` -- unsigned-int8 x signed-int8 -> `.s` (FEAT_SME_TMOP).
    Ustmopa,
    /// `SUTMOPA` -- signed-int8 x unsigned-int8 -> `.s` (FEAT_SME_TMOP).
    Sutmopa,
    /// `BFTMOPA` -- bf16 x bf16 -> `.s` (FEAT_SME_TMOP).
    Bftmopa,
    /// `FTMOPA ZAda.S, {Zn.H..}, Zm.H, ...` -- fp16 x fp16 -> `.s` (FEAT_SME_TMOP).
    FtmopaSh,
    /// `FTMOPA ZAda.H, {Zn.H..}, Zm.H, ...` -- fp16 x fp16 -> `.h` (FEAT_SME_TMOP + FEAT_SME_F16F16).
    FtmopaHh,
    /// `FTMOPA ZAda.S, {Zn.B..}, Zm.B, ...` -- fp8 x fp8 -> `.s` (FEAT_SME_TMOP + FEAT_SME_F8F32).
    FtmopaSb,
    /// `FTMOPA ZAda.H, {Zn.B..}, Zm.B, ...` -- fp8 x fp8 -> `.h` (FEAT_SME_TMOP + FEAT_SME_F8F16).
    FtmopaHb,
}

impl Arm64SmeTmopOp {
    /// Every operation, for exhaustive round-trip tests.
    pub const ALL: [Self; 9] = [
        Self::Stmopa,
        Self::Utmopa,
        Self::Ustmopa,
        Self::Sutmopa,
        Self::Bftmopa,
        Self::FtmopaSh,
        Self::FtmopaHh,
        Self::FtmopaSb,
        Self::FtmopaHb,
    ];

    /// The op-specific bits within the selector mask `0x0120_8008` (`[24]` + `[21]` + `[15]` + `[3]`).
    pub fn op_bits(self) -> u32 {
        match self {
            Self::Stmopa => 0x0000_8000,   // [15]
            Self::Utmopa => 0x0120_8000,   // [24]+[21]+[15]
            Self::Ustmopa => 0x0100_8000,  // [24]+[15]
            Self::Sutmopa => 0x0020_8000,  // [21]+[15]
            Self::Bftmopa => 0x0100_0000,  // [24]
            Self::FtmopaSh => 0x0120_0000, // [24]+[21]
            Self::FtmopaHh => 0x0100_0008, // [24]+[3]
            Self::FtmopaSb => 0x0020_0000, // [21]
            Self::FtmopaHb => 0x0020_0008, // [21]+[3]
        }
    }

    /// Recover the op from its selector bits (`word & 0x0120_8008`); `None` for an unallocated combination.
    pub fn from_op_bits(bits: u32) -> Option<Self> {
        match bits & 0x0120_8008 {
            0x0000_8000 => Some(Self::Stmopa),
            0x0120_8000 => Some(Self::Utmopa),
            0x0100_8000 => Some(Self::Ustmopa),
            0x0020_8000 => Some(Self::Sutmopa),
            0x0100_0000 => Some(Self::Bftmopa),
            0x0120_0000 => Some(Self::FtmopaSh),
            0x0100_0008 => Some(Self::FtmopaHh),
            0x0020_0000 => Some(Self::FtmopaSb),
            0x0020_0008 => Some(Self::FtmopaHb),
            _ => None,
        }
    }

    /// The lowercase mnemonic.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Stmopa => "stmopa",
            Self::Utmopa => "utmopa",
            Self::Ustmopa => "ustmopa",
            Self::Sutmopa => "sutmopa",
            Self::Bftmopa => "bftmopa",
            Self::FtmopaSh | Self::FtmopaHh | Self::FtmopaSb | Self::FtmopaHb => "ftmopa",
        }
    }

    /// Whether the destination `ZA` tile is `.h` (FP16, tiles `za0`/`za1`) rather than `.s` (FP32, tiles `za0..za3`).
    pub fn tile_is_h(self) -> bool {
        matches!(self, Self::FtmopaHh | Self::FtmopaHb)
    }

    /// The source-vector element suffix: `"b"` (int8 / fp8) or `"h"` (bf16 / fp16).
    pub fn vector_elem(self) -> &'static str {
        match self {
            Self::Stmopa
            | Self::Utmopa
            | Self::Ustmopa
            | Self::Sutmopa
            | Self::FtmopaSb
            | Self::FtmopaHb => "b",
            Self::Bftmopa | Self::FtmopaSh | Self::FtmopaHh => "h",
        }
    }
}
