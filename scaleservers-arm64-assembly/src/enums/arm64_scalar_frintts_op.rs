// Copyright (c) Scaleservers LLC

/// A scalar **round-to-integer-N** op (FEAT_FRINTTS, DDI0487 C7): `FRINT32X`/`FRINT32Z`/`FRINT64X`/`FRINT64Z`,
/// in the scalar FP one-source family (`0001 1110 ftype 1 opcode[20:15] 10000 Rn Rd`). Single/double only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarFrintTsOp {
    /// `FRINT32Z` -- round to a 32-bit signed integral value, toward zero.
    Frint32z,
    /// `FRINT32X` -- round to a 32-bit signed integral value, current rounding mode.
    Frint32x,
    /// `FRINT64Z` -- round to a 64-bit signed integral value, toward zero.
    Frint64z,
    /// `FRINT64X` -- round to a 64-bit signed integral value, current rounding mode.
    Frint64x,
}

impl Arm64ScalarFrintTsOp {
    /// The base word with `ftype = 00` (Single) and zero registers; the encoder ORs `precision.ftype() << 22` and
    /// `Rn << 5 | Rd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Frint32z => 0x1E28_4000,
            Self::Frint32x => 0x1E28_C000,
            Self::Frint64z => 0x1E29_4000,
            Self::Frint64x => 0x1E29_C000,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Frint32z => "frint32z",
            Self::Frint32x => "frint32x",
            Self::Frint64z => "frint64z",
            Self::Frint64x => "frint64x",
        }
    }

    /// Recover the op from a masked base (`word & FP_DATA_ONE_SOURCE_MASK`); `None` if not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Whether this rounds to a 64-bit (vs 32-bit) integral value (`FRINT64*`).
    pub fn is_64(self) -> bool {
        matches!(self, Self::Frint64z | Self::Frint64x)
    }

    /// Whether this uses the current rounding mode (`*X`) rather than toward-zero (`*Z`).
    pub fn is_x(self) -> bool {
        matches!(self, Self::Frint32x | Self::Frint64x)
    }

    /// Build the op from the SVE-form `(is_64, is_x)` flag pair (used by the FEAT_SVE2p2 predicated forms).
    pub fn from_flags(is_64: bool, is_x: bool) -> Self {
        match (is_64, is_x) {
            (false, false) => Self::Frint32z,
            (false, true) => Self::Frint32x,
            (true, false) => Self::Frint64z,
            (true, true) => Self::Frint64x,
        }
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 4] = [
        Self::Frint32z,
        Self::Frint32x,
        Self::Frint64z,
        Self::Frint64x,
    ];
}
