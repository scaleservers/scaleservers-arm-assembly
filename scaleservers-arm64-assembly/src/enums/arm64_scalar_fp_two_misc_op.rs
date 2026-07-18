// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **two-register-misc floating-point** op (DDI0487 C7, the `01 U 11110 E sz 10000 opcode
/// 10 Rn Rd` encoding) -- the scalar (`s`/`d` register) FP-to-integer converts, integer-to-FP converts, the
/// reciprocal estimates, and the compare-against-zero forms. Every op is valid for both precisions (`double`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarFpTwoMiscOp {
    /// `FCVTNS` -- FP convert to signed integer, round to nearest, ties to even.
    Fcvtns,
    /// `FCVTNU` -- FP convert to unsigned integer, round to nearest, ties to even.
    Fcvtnu,
    /// `FCVTMS` -- FP convert to signed integer, round toward minus infinity.
    Fcvtms,
    /// `FCVTMU` -- FP convert to unsigned integer, round toward minus infinity.
    Fcvtmu,
    /// `FCVTAS` -- FP convert to signed integer, round to nearest, ties away.
    Fcvtas,
    /// `FCVTAU` -- FP convert to unsigned integer, round to nearest, ties away.
    Fcvtau,
    /// `FCVTPS` -- FP convert to signed integer, round toward plus infinity.
    Fcvtps,
    /// `FCVTPU` -- FP convert to unsigned integer, round toward plus infinity.
    Fcvtpu,
    /// `FCVTZS` -- FP convert to signed integer, round toward zero.
    Fcvtzs,
    /// `FCVTZU` -- FP convert to unsigned integer, round toward zero.
    Fcvtzu,
    /// `SCVTF` -- signed integer convert to FP.
    Scvtf,
    /// `UCVTF` -- unsigned integer convert to FP.
    Ucvtf,
    /// `FRECPE` -- FP reciprocal estimate.
    Frecpe,
    /// `FRSQRTE` -- FP reciprocal square-root estimate.
    Frsqrte,
    /// `FRECPX` -- FP reciprocal exponent.
    Frecpx,
    /// `FCMEQ #0.0` -- FP compare equal to zero.
    FcmeqZero,
    /// `FCMGE #0.0` -- FP compare greater-or-equal zero.
    FcmgeZero,
    /// `FCMGT #0.0` -- FP compare greater-than zero.
    FcmgtZero,
    /// `FCMLE #0.0` -- FP compare less-or-equal zero.
    FcmleZero,
    /// `FCMLT #0.0` -- FP compare less-than zero.
    FcmltZero,
}

impl Arm64ScalarFpTwoMiscOp {
    /// The base word (the single-precision form, `sz`/`Rn`/`Rd` zero). The encoder ORs `sz<<22` for the
    /// double-precision form. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Fcvtns => 0x5E21_A800,
            Self::Fcvtnu => 0x7E21_A800,
            Self::Fcvtms => 0x5E21_B800,
            Self::Fcvtmu => 0x7E21_B800,
            Self::Fcvtas => 0x5E21_C800,
            Self::Fcvtau => 0x7E21_C800,
            Self::Fcvtps => 0x5EA1_A800,
            Self::Fcvtpu => 0x7EA1_A800,
            Self::Fcvtzs => 0x5EA1_B800,
            Self::Fcvtzu => 0x7EA1_B800,
            Self::Scvtf => 0x5E21_D800,
            Self::Ucvtf => 0x7E21_D800,
            Self::Frecpe => 0x5EA1_D800,
            Self::Frsqrte => 0x7EA1_D800,
            Self::Frecpx => 0x5EA1_F800,
            Self::FcmeqZero => 0x5EA0_D800,
            Self::FcmgeZero => 0x7EA0_C800,
            Self::FcmgtZero => 0x5EA0_C800,
            Self::FcmleZero => 0x7EA0_D800,
            Self::FcmltZero => 0x5EA0_E800,
        }
    }

    /// The half-precision (FP16, FEAT_FP16) base word -- the scalar `h`-register form (a separate opcode space
    /// from [`Self::base`]). GNU+LLVM dual-oracle verified.
    pub fn fp16_base(self) -> u32 {
        match self {
            Self::Fcvtns => 0x5E79_A800,
            Self::Fcvtnu => 0x7E79_A800,
            Self::Fcvtms => 0x5E79_B800,
            Self::Fcvtmu => 0x7E79_B800,
            Self::Fcvtas => 0x5E79_C800,
            Self::Fcvtau => 0x7E79_C800,
            Self::Fcvtps => 0x5EF9_A800,
            Self::Fcvtpu => 0x7EF9_A800,
            Self::Fcvtzs => 0x5EF9_B800,
            Self::Fcvtzu => 0x7EF9_B800,
            Self::Scvtf => 0x5E79_D800,
            Self::Ucvtf => 0x7E79_D800,
            Self::Frecpe => 0x5EF9_D800,
            Self::Frsqrte => 0x7EF9_D800,
            Self::Frecpx => 0x5EF9_F800,
            Self::FcmeqZero => 0x5EF8_D800,
            Self::FcmgeZero => 0x7EF8_C800,
            Self::FcmgtZero => 0x5EF8_C800,
            Self::FcmleZero => 0x7EF8_D800,
            Self::FcmltZero => 0x5EF8_E800,
        }
    }

    /// Recover the op from a half-precision masked base (`word & 0xFFFF_FC00`); `None` if not one of these.
    pub fn from_fp16_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.fp16_base() == base)
    }

    /// The lowercase UAL mnemonic (the compare-against-zero ops print a trailing `, #0.0`, handled by the emitter).
    pub fn name(self) -> &'static str {
        match self {
            Self::Fcvtns => "fcvtns",
            Self::Fcvtnu => "fcvtnu",
            Self::Fcvtms => "fcvtms",
            Self::Fcvtmu => "fcvtmu",
            Self::Fcvtas => "fcvtas",
            Self::Fcvtau => "fcvtau",
            Self::Fcvtps => "fcvtps",
            Self::Fcvtpu => "fcvtpu",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtzu => "fcvtzu",
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",
            Self::Frecpe => "frecpe",
            Self::Frsqrte => "frsqrte",
            Self::Frecpx => "frecpx",
            Self::FcmeqZero => "fcmeq",
            Self::FcmgeZero => "fcmge",
            Self::FcmgtZero => "fcmgt",
            Self::FcmleZero => "fcmle",
            Self::FcmltZero => "fcmlt",
        }
    }

    /// Whether this op prints a trailing `, #0.0` (the compare-against-zero forms).
    pub fn is_compare_zero(self) -> bool {
        matches!(
            self,
            Self::FcmeqZero | Self::FcmgeZero | Self::FcmgtZero | Self::FcmleZero | Self::FcmltZero
        )
    }

    /// Recover the op from a masked base (`word & 0xFFBF_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 20] = [
        Self::Fcvtns,
        Self::Fcvtnu,
        Self::Fcvtms,
        Self::Fcvtmu,
        Self::Fcvtas,
        Self::Fcvtau,
        Self::Fcvtps,
        Self::Fcvtpu,
        Self::Fcvtzs,
        Self::Fcvtzu,
        Self::Scvtf,
        Self::Ucvtf,
        Self::Frecpe,
        Self::Frsqrte,
        Self::Frecpx,
        Self::FcmeqZero,
        Self::FcmgeZero,
        Self::FcmgtZero,
        Self::FcmleZero,
        Self::FcmltZero,
    ];
}
