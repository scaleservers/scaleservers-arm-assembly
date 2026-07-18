// Copyright (c) Scaleservers LLC

/// A FEAT_CSSC scalar **integer min/max** op (`SMAX`/`SMIN`/`UMAX`/`UMIN`), in both the register form (an additional
/// opcode in the data-processing two-source group) and the 8-bit-immediate form (its own encoding group). Gated on
/// FEAT_CSSC. GNU+LLVM verified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64CsscMinMaxOp {
    /// `SMAX` -- signed maximum.
    Smax,
    /// `UMAX` -- unsigned maximum.
    Umax,
    /// `SMIN` -- signed minimum.
    Smin,
    /// `UMIN` -- unsigned minimum.
    Umin,
}

impl Arm64CsscMinMaxOp {
    /// The register-form base (the `W` two-source encoding; the `X` form sets bit 31). GNU+LLVM verified.
    pub const fn reg_base(self) -> u32 {
        match self {
            Self::Smax => 0x1AC0_6000,
            Self::Umax => 0x1AC0_6400,
            Self::Smin => 0x1AC0_6800,
            Self::Umin => 0x1AC0_6C00,
        }
    }

    /// The immediate-form `[19:18]` opcode selector.
    pub const fn imm_opcode(self) -> u32 {
        match self {
            Self::Smax => 0b00,
            Self::Umax => 0b01,
            Self::Smin => 0b10,
            Self::Umin => 0b11,
        }
    }

    /// Whether the immediate is signed (`SMAX`/`SMIN`: `-128..=127`) vs unsigned (`UMAX`/`UMIN`: `0..=255`).
    pub const fn imm_is_signed(self) -> bool {
        matches!(self, Self::Smax | Self::Smin)
    }

    /// The lowercase UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Smax => "smax",
            Self::Umax => "umax",
            Self::Smin => "smin",
            Self::Umin => "umin",
        }
    }

    /// Recover the op from the immediate-form `[19:18]` opcode.
    pub const fn from_imm_opcode(opcode: u32) -> Self {
        match opcode & 0b11 {
            0b00 => Self::Smax,
            0b01 => Self::Umax,
            0b10 => Self::Smin,
            _ => Self::Umin,
        }
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 4] = [Self::Smax, Self::Umax, Self::Smin, Self::Umin];
}
