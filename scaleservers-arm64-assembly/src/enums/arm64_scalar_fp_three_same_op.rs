// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **three-same floating-point** op (DDI0487 C7, the `01 U 11110 E sz 1 Rm opcode 1 Rn Rd`
/// encoding) -- the scalar (`s`/`d` register) FP compares and multiply extras. Every op is valid for both single
/// (`double=false`) and double (`double=true`) precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarFpThreeSameOp {
    /// `FMULX` -- floating-point multiply extended.
    Fmulx,
    /// `FCMEQ` -- floating-point compare equal.
    Fcmeq,
    /// `FRECPS` -- floating-point reciprocal step.
    Frecps,
    /// `FRSQRTS` -- floating-point reciprocal square-root step.
    Frsqrts,
    /// `FCMGE` -- floating-point compare greater-or-equal.
    Fcmge,
    /// `FACGE` -- floating-point absolute compare greater-or-equal.
    Facge,
    /// `FABD` -- floating-point absolute difference.
    Fabd,
    /// `FCMGT` -- floating-point compare greater-than.
    Fcmgt,
    /// `FACGT` -- floating-point absolute compare greater-than.
    Facgt,
}

impl Arm64ScalarFpThreeSameOp {
    /// The base word (the single-precision form, `sz`/`Rm`/`Rn`/`Rd` zero). The encoder ORs `sz<<22` for the
    /// double-precision form. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Fmulx => 0x5E20_DC00,
            Self::Fcmeq => 0x5E20_E400,
            Self::Frecps => 0x5E20_FC00,
            Self::Frsqrts => 0x5EA0_FC00,
            Self::Fcmge => 0x7E20_E400,
            Self::Facge => 0x7E20_EC00,
            Self::Fabd => 0x7EA0_D400,
            Self::Fcmgt => 0x7EA0_E400,
            Self::Facgt => 0x7EA0_EC00,
        }
    }

    /// The half-precision (FP16, FEAT_FP16) base word -- the scalar `h`-register form (a separate opcode space
    /// from [`Self::base`]). GNU+LLVM dual-oracle verified.
    pub fn fp16_base(self) -> u32 {
        match self {
            Self::Fmulx => 0x5E40_1C00,
            Self::Fcmeq => 0x5E40_2400,
            Self::Frecps => 0x5E40_3C00,
            Self::Frsqrts => 0x5EC0_3C00,
            Self::Fcmge => 0x7E40_2400,
            Self::Facge => 0x7E40_2C00,
            Self::Fabd => 0x7EC0_1400,
            Self::Fcmgt => 0x7EC0_2400,
            Self::Facgt => 0x7EC0_2C00,
        }
    }

    /// Recover the op from a half-precision masked base (`word & 0xFFE0_FC00`); `None` if not one of these.
    pub fn from_fp16_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.fp16_base() == base)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fmulx => "fmulx",
            Self::Fcmeq => "fcmeq",
            Self::Frecps => "frecps",
            Self::Frsqrts => "frsqrts",
            Self::Fcmge => "fcmge",
            Self::Facge => "facge",
            Self::Fabd => "fabd",
            Self::Fcmgt => "fcmgt",
            Self::Facgt => "facgt",
        }
    }

    /// Recover the op from a masked base (`word & 0xFFA0_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 9] = [
        Self::Fmulx,
        Self::Fcmeq,
        Self::Frecps,
        Self::Frsqrts,
        Self::Fcmge,
        Self::Facge,
        Self::Fabd,
        Self::Fcmgt,
        Self::Facgt,
    ];
}
