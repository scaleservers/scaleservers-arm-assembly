// Copyright (c) Scaleservers LLC

/// The PSTATE field affected by `SMSTART`/`SMSTOP` (FEAT_SME): the streaming-mode bit `SM`, the `ZA` array-enable bit
/// `ZA`, or both at once (the bare `SMSTART`/`SMSTOP`). Encoded in the MSR-immediate `CRm` field as `(code << 1) | v`,
/// where `v` is 1 for `SMSTART` and 0 for `SMSTOP`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SmeStateTarget {
    /// `SM` -- the streaming-SVE-mode PSTATE bit.
    Sm,
    /// `ZA` -- the ZA-array-enable PSTATE bit.
    Za,
    /// both `SM` and `ZA` (the operand-less `SMSTART`/`SMSTOP`).
    Both,
}

impl Arm64SmeStateTarget {
    /// The 2-bit target code that forms the high bits of the `CRm` field.
    pub const fn code(self) -> u32 {
        match self {
            Self::Sm => 1,
            Self::Za => 2,
            Self::Both => 3,
        }
    }

    /// The UAL operand text (the bare `SMSTART`/`SMSTOP` print no operand).
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Za => "za",
            Self::Both => "",
        }
    }

    /// Recover the target from the 2-bit `CRm` high field, or `None` for the unallocated `0`.
    pub const fn from_code(code: u32) -> Option<Self> {
        match code & 0b11 {
            1 => Some(Self::Sm),
            2 => Some(Self::Za),
            3 => Some(Self::Both),
            _ => None,
        }
    }

    /// Every target, for tests.
    pub const ALL: [Self; 3] = [Self::Sm, Self::Za, Self::Both];
}
