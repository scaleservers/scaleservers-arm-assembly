// Copyright (c) Scaleservers LLC

/// An SVE2.1/SME2 **predicate-as-counter** governing register `PN8`..`PN15`. These are the `P8`..`P15` predicate
/// registers reinterpreted as a packed element counter (an element index + an all-true/all-false direction), used to
/// govern the multi-vector contiguous loads/stores and other multi-vector ops. Only `PN8`..`PN15` are encodable as a
/// governing operand (the field is the 3-bit value `pn - 8`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64PredicateAsCounter {
    /// `PN8`.
    Pn8,
    /// `PN9`.
    Pn9,
    /// `PN10`.
    Pn10,
    /// `PN11`.
    Pn11,
    /// `PN12`.
    Pn12,
    /// `PN13`.
    Pn13,
    /// `PN14`.
    Pn14,
    /// `PN15`.
    Pn15,
}

impl Arm64PredicateAsCounter {
    /// The 3-bit governing field value (`pn - 8`, i.e. `0..=7`).
    pub const fn index(self) -> u32 {
        self as u32
    }

    /// Recover the register from the 3-bit `pn - 8` field.
    pub const fn from_index(bits: u32) -> Self {
        match bits & 0b111 {
            0 => Self::Pn8,
            1 => Self::Pn9,
            2 => Self::Pn10,
            3 => Self::Pn11,
            4 => Self::Pn12,
            5 => Self::Pn13,
            6 => Self::Pn14,
            _ => Self::Pn15,
        }
    }

    /// The `pnN` assembly name.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Pn8 => "pn8",
            Self::Pn9 => "pn9",
            Self::Pn10 => "pn10",
            Self::Pn11 => "pn11",
            Self::Pn12 => "pn12",
            Self::Pn13 => "pn13",
            Self::Pn14 => "pn14",
            Self::Pn15 => "pn15",
        }
    }

    /// Every governing counter, for tests.
    pub const ALL: [Self; 8] = [
        Self::Pn8,
        Self::Pn9,
        Self::Pn10,
        Self::Pn11,
        Self::Pn12,
        Self::Pn13,
        Self::Pn14,
        Self::Pn15,
    ];
}
