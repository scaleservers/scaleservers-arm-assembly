// Copyright (c) Scaleservers LLC

/// An SVE2.1 **two-vector saturating rounding shift-right narrowing** op (FEAT_SVE2p1; DDI0487 part C). Shifts a
/// consecutive `.s` Z-register pair right by `#1..16`, rounds, saturates, and narrows to a single `.h` Z register.
/// The op is the `[13:12]` selector of the shared frame. GNU+LLVM verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveShiftNarrowOp {
    /// `SQRSHRUN` -- signed-to-unsigned saturating rounding shift-right narrow.
    Sqrshrun,
    /// `SQRSHRN` -- signed saturating rounding shift-right narrow.
    Sqrshrn,
    /// `UQRSHRN` -- unsigned saturating rounding shift-right narrow.
    Uqrshrn,
}

impl Arm64SveShiftNarrowOp {
    /// The `[13:12]` op-selector value.
    pub fn code(self) -> u32 {
        match self {
            Self::Sqrshrun => 0b00,
            Self::Sqrshrn => 0b10,
            Self::Uqrshrn => 0b11,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Sqrshrun => "sqrshrun",
            Self::Sqrshrn => "sqrshrn",
            Self::Uqrshrn => "uqrshrn",
        }
    }

    /// The lowercase UAL mnemonic of the matching SME2 NON-interleaving (no-`N`) form (`SQRSHR`/`UQRSHR`/`SQRSHRU`).
    pub fn no_n_mnemonic(self) -> &'static str {
        match self {
            Self::Sqrshrun => "sqrshru",
            Self::Sqrshrn => "sqrshr",
            Self::Uqrshrn => "uqrshr",
        }
    }

    /// The SME2 **four-vector** narrowing-shift `[6:5]` op selector (a different mapping than the two-vector
    /// [`Self::code`] at `[13:12]`).
    pub fn quad_code(self) -> u32 {
        match self {
            Self::Sqrshrn => 0b00,
            Self::Uqrshrn => 0b01,
            Self::Sqrshrun => 0b10,
        }
    }

    /// Recover the op from the four-vector `[6:5]` selector; `None` for the unallocated value `0b11`.
    pub fn from_quad_code(code: u32) -> Option<Self> {
        Some(match code & 0b11 {
            0b00 => Self::Sqrshrn,
            0b01 => Self::Uqrshrn,
            0b10 => Self::Sqrshrun,
            _ => return None,
        })
    }

    /// Recover the op from the `[13:12]` selector; `None` for the unallocated value `0b01`.
    pub fn from_code(code: u32) -> Option<Self> {
        Some(match code & 0b11 {
            0b00 => Self::Sqrshrun,
            0b10 => Self::Sqrshrn,
            0b11 => Self::Uqrshrn,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Sqrshrun, Self::Sqrshrn, Self::Uqrshrn];
}
