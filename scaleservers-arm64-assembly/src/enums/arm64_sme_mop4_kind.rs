// Copyright (c) Scaleservers LLC

/// The data-type "kind" of an SME2 quarter-tile outer product (`*MOP4A`/`*MOP4S`, FEAT_SME_MOP4). The kind
/// selects the source element type + signedness via the op bits `[24]` (`unsigned_first` / FP-type-high),
/// `[21]` (`unsigned_second` / FP-type-low) and `[15]` (the integer marker). The `A` vs `S` (accumulate vs
/// subtract) choice is the orthogonal `subtract` (`[4]`) field, not part of the kind.
///
/// All kinds accumulate into a `.s` (FP32 / S32) `ZA` tile; the source vectors are `.b` (int8), `.s` (f32) or
/// `.h` (bf16 / f16) per [`Self::source_element`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SmeMop4Kind {
    /// `SMOP4*` -- signed-int8 x signed-int8 (`[24]=0`, `[21]=0`, `[15]=1`).
    Smop4,
    /// `UMOP4*` -- unsigned-int8 x unsigned-int8 (`[24]=1`, `[21]=1`, `[15]=1`).
    Umop4,
    /// `SUMOP4*` -- signed-int8 x unsigned-int8 (`[24]=0`, `[21]=1`, `[15]=1`).
    Sumop4,
    /// `USMOP4*` -- unsigned-int8 x signed-int8 (`[24]=1`, `[21]=0`, `[15]=1`).
    Usmop4,
    /// `FMOP4*` -- f32 x f32 (`[24]=0`, `[21]=0`, `[15]=0`); `.s` sources.
    Fmop4F32,
    /// `BFMOP4*` -- bf16 x bf16 (`[24]=1`, `[21]=0`, `[15]=0`); `.h` sources.
    Bfmop4,
    /// `FMOP4*` -- f16 x f16 (`[24]=1`, `[21]=1`, `[15]=0`); `.h` sources.
    Fmop4F16,
}

impl Arm64SmeMop4Kind {
    /// Every kind, for exhaustive round-trip tests.
    pub const ALL: [Self; 7] = [
        Self::Smop4,
        Self::Umop4,
        Self::Sumop4,
        Self::Usmop4,
        Self::Fmop4F32,
        Self::Bfmop4,
        Self::Fmop4F16,
    ];

    /// The op-selector bits (`[24]` + `[21]` + `[15]`, mask `0x0120_8000`).
    pub fn selector_bits(self) -> u32 {
        match self {
            Self::Smop4 => 0x0000_8000,    // [15]
            Self::Umop4 => 0x0120_8000,    // [24]+[21]+[15]
            Self::Sumop4 => 0x0020_8000,   // [21]+[15]
            Self::Usmop4 => 0x0100_8000,   // [24]+[15]
            Self::Fmop4F32 => 0x0000_0000, // (none)
            Self::Bfmop4 => 0x0100_0000,   // [24]
            Self::Fmop4F16 => 0x0120_0000, // [24]+[21]
        }
    }

    /// Recover the kind from its op-selector bits (`word & 0x0120_8000`); `None` for an unallocated combination
    /// (the FP `[24]=0, [21]=1, [15]=0` slot is unallocated).
    pub fn from_selector(bits: u32) -> Option<Self> {
        match bits & 0x0120_8000 {
            0x0000_8000 => Some(Self::Smop4),
            0x0120_8000 => Some(Self::Umop4),
            0x0020_8000 => Some(Self::Sumop4),
            0x0100_8000 => Some(Self::Usmop4),
            0x0000_0000 => Some(Self::Fmop4F32),
            0x0100_0000 => Some(Self::Bfmop4),
            0x0120_0000 => Some(Self::Fmop4F16),
            _ => None,
        }
    }

    /// The mnemonic stem (before the `a`/`s` accumulate/subtract suffix): `smop4`/`umop4`/.../`fmop4`/`bfmop4`.
    pub fn stem(self) -> &'static str {
        match self {
            Self::Smop4 => "smop4",
            Self::Umop4 => "umop4",
            Self::Sumop4 => "sumop4",
            Self::Usmop4 => "usmop4",
            Self::Fmop4F32 | Self::Fmop4F16 => "fmop4",
            Self::Bfmop4 => "bfmop4",
        }
    }

    /// The source-vector element suffix: `"b"` (int8), `"s"` (f32) or `"h"` (bf16 / f16).
    pub fn source_element(self) -> &'static str {
        match self {
            Self::Smop4 | Self::Umop4 | Self::Sumop4 | Self::Usmop4 => "b",
            Self::Fmop4F32 => "s",
            Self::Bfmop4 | Self::Fmop4F16 => "h",
        }
    }
}

/// The data-type "kind" of an SME2 quarter-tile outer product accumulating into a **`.d` (64-bit) `ZA` tile** --
/// the `.h`-source integer i16->i64 forms (FEAT_SME_I16I64) and the `.d`-source f64 form (FEAT_SME_F64F64). The
/// kind selects `[29]` (integer marker), `[24]` (`unsigned_first`) and `[21]` (`unsigned_second`); the `A`/`S`
/// (accumulate/subtract) choice is the orthogonal `[4]` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SmeMop4DoubleKind {
    /// `SMOP4*` -- signed-int16 x signed-int16 -> S64 (`[29]=1`, `[24]=0`, `[21]=0`).
    Smop4,
    /// `UMOP4*` -- unsigned-int16 x unsigned-int16 -> S64 (`[29]=1`, `[24]=1`, `[21]=1`).
    Umop4,
    /// `SUMOP4*` -- signed-int16 x unsigned-int16 -> S64 (`[29]=1`, `[24]=0`, `[21]=1`).
    Sumop4,
    /// `USMOP4*` -- unsigned-int16 x signed-int16 -> S64 (`[29]=1`, `[24]=1`, `[21]=0`).
    Usmop4,
    /// `FMOP4*` -- f64 x f64 -> F64 (`[29]=0`); `.d` sources.
    Fmop4,
}

impl Arm64SmeMop4DoubleKind {
    /// Every kind, for exhaustive round-trip tests.
    pub const ALL: [Self; 5] = [
        Self::Smop4,
        Self::Umop4,
        Self::Sumop4,
        Self::Usmop4,
        Self::Fmop4,
    ];

    /// The op-selector bits (`[29]` + `[24]` + `[21]`, mask `0x2120_0000`).
    pub fn selector_bits(self) -> u32 {
        match self {
            Self::Smop4 => 0x2000_0000,  // [29]
            Self::Umop4 => 0x2120_0000,  // [29]+[24]+[21]
            Self::Sumop4 => 0x2020_0000, // [29]+[21]
            Self::Usmop4 => 0x2100_0000, // [29]+[24]
            Self::Fmop4 => 0x0000_0000,  // (none -- [29]=0)
        }
    }

    /// Recover the kind from its op-selector bits (`word & 0x2120_0000`); `None` for an unallocated combination
    /// (any `[24]`/`[21]` set with `[29]=0` -- there is no signed/unsigned f64 form).
    pub fn from_selector(bits: u32) -> Option<Self> {
        match bits & 0x2120_0000 {
            0x2000_0000 => Some(Self::Smop4),
            0x2120_0000 => Some(Self::Umop4),
            0x2020_0000 => Some(Self::Sumop4),
            0x2100_0000 => Some(Self::Usmop4),
            0x0000_0000 => Some(Self::Fmop4),
            _ => None,
        }
    }

    /// The mnemonic stem (before the `a`/`s` suffix): `smop4`/`umop4`/`sumop4`/`usmop4`/`fmop4`.
    pub fn stem(self) -> &'static str {
        match self {
            Self::Smop4 => "smop4",
            Self::Umop4 => "umop4",
            Self::Sumop4 => "sumop4",
            Self::Usmop4 => "usmop4",
            Self::Fmop4 => "fmop4",
        }
    }

    /// The source-vector element suffix: `"h"` (the i16 integer forms) or `"d"` (the f64 form).
    pub fn source_element(self) -> &'static str {
        match self {
            Self::Fmop4 => "d",
            _ => "h",
        }
    }

    /// Whether this is one of the integer (i16i64) kinds (vs the f64f64 kind).
    pub fn is_integer(self) -> bool {
        !matches!(self, Self::Fmop4)
    }
}
