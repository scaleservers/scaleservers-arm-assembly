// Copyright (c) Scaleservers LLC

/// A NEON **half-precision (FP16) two-register-misc** op (DDI0487 C7, FEAT_FP16) -- the `.4h`/`.8h` per-lane FP
/// rounding, the FP<->integer converts, the reciprocal estimates, the unary ops, and the compare-against-zero
/// forms. Their own opcode space (separate from the f32/f64 two-register-misc), so this carries explicit bases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorFp16TwoMiscOp {
    /// `FRINTN` -- round to nearest, ties to even.
    Frintn,
    /// `FRINTM` -- round toward minus infinity.
    Frintm,
    /// `FRINTP` -- round toward plus infinity.
    Frintp,
    /// `FRINTZ` -- round toward zero.
    Frintz,
    /// `FRINTA` -- round to nearest, ties away.
    Frinta,
    /// `FRINTX` -- round to integral, exact (signals inexact).
    Frintx,
    /// `FRINTI` -- round to integral using the current mode.
    Frinti,
    /// `FCVTNS` -- convert to signed integer, round to nearest ties even.
    Fcvtns,
    /// `FCVTMS` -- convert to signed integer, round toward minus infinity.
    Fcvtms,
    /// `FCVTAS` -- convert to signed integer, round to nearest ties away.
    Fcvtas,
    /// `FCVTPS` -- convert to signed integer, round toward plus infinity.
    Fcvtps,
    /// `FCVTZS` -- convert to signed integer, round toward zero.
    Fcvtzs,
    /// `FCVTNU` -- convert to unsigned integer, round to nearest ties even.
    Fcvtnu,
    /// `FCVTMU` -- convert to unsigned integer, round toward minus infinity.
    Fcvtmu,
    /// `FCVTAU` -- convert to unsigned integer, round to nearest ties away.
    Fcvtau,
    /// `FCVTPU` -- convert to unsigned integer, round toward plus infinity.
    Fcvtpu,
    /// `FCVTZU` -- convert to unsigned integer, round toward zero.
    Fcvtzu,
    /// `SCVTF` -- signed integer convert to FP.
    Scvtf,
    /// `UCVTF` -- unsigned integer convert to FP.
    Ucvtf,
    /// `FRECPE` -- reciprocal estimate.
    Frecpe,
    /// `FRSQRTE` -- reciprocal square-root estimate.
    Frsqrte,
    /// `FABS` -- absolute value.
    Fabs,
    /// `FNEG` -- negate.
    Fneg,
    /// `FSQRT` -- square root.
    Fsqrt,
    /// `FCMEQ #0.0` -- compare equal to zero.
    FcmeqZero,
    /// `FCMGE #0.0` -- compare greater-or-equal zero.
    FcmgeZero,
    /// `FCMGT #0.0` -- compare greater-than zero.
    FcmgtZero,
    /// `FCMLE #0.0` -- compare less-or-equal zero.
    FcmleZero,
    /// `FCMLT #0.0` -- compare less-than zero.
    FcmltZero,
}

impl Arm64VectorFp16TwoMiscOp {
    /// The base word for the `.4h` form (`Q = 0`); the encoder ORs `Q<<30` for `.8h`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Frintn => 0x0E79_8800,
            Self::Frintm => 0x0E79_9800,
            Self::Frinta => 0x2E79_8800,
            Self::Frintx => 0x2E79_9800,
            Self::Frintp => 0x0EF9_8800,
            Self::Frintz => 0x0EF9_9800,
            Self::Frinti => 0x2EF9_9800,
            Self::Fcvtns => 0x0E79_A800,
            Self::Fcvtms => 0x0E79_B800,
            Self::Fcvtas => 0x0E79_C800,
            Self::Fcvtps => 0x0EF9_A800,
            Self::Fcvtzs => 0x0EF9_B800,
            Self::Fcvtnu => 0x2E79_A800,
            Self::Fcvtmu => 0x2E79_B800,
            Self::Fcvtau => 0x2E79_C800,
            Self::Fcvtpu => 0x2EF9_A800,
            Self::Fcvtzu => 0x2EF9_B800,
            Self::Scvtf => 0x0E79_D800,
            Self::Ucvtf => 0x2E79_D800,
            Self::Frecpe => 0x0EF9_D800,
            Self::Frsqrte => 0x2EF9_D800,
            Self::Fabs => 0x0EF8_F800,
            Self::Fneg => 0x2EF8_F800,
            Self::Fsqrt => 0x2EF9_F800,
            Self::FcmeqZero => 0x0EF8_D800,
            Self::FcmgeZero => 0x2EF8_C800,
            Self::FcmgtZero => 0x0EF8_C800,
            Self::FcmleZero => 0x2EF8_D800,
            Self::FcmltZero => 0x0EF8_E800,
        }
    }

    /// The lowercase UAL mnemonic (the compare-against-zero ops print a trailing `, #0.0`, handled by the emitter).
    pub fn name(self) -> &'static str {
        match self {
            Self::Frintn => "frintn",
            Self::Frintm => "frintm",
            Self::Frinta => "frinta",
            Self::Frintx => "frintx",
            Self::Frintp => "frintp",
            Self::Frintz => "frintz",
            Self::Frinti => "frinti",
            Self::Fcvtns => "fcvtns",
            Self::Fcvtms => "fcvtms",
            Self::Fcvtas => "fcvtas",
            Self::Fcvtps => "fcvtps",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtnu => "fcvtnu",
            Self::Fcvtmu => "fcvtmu",
            Self::Fcvtau => "fcvtau",
            Self::Fcvtpu => "fcvtpu",
            Self::Fcvtzu => "fcvtzu",
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",
            Self::Frecpe => "frecpe",
            Self::Frsqrte => "frsqrte",
            Self::Fabs => "fabs",
            Self::Fneg => "fneg",
            Self::Fsqrt => "fsqrt",
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

    /// Recover the op from a masked base (`word & 0xBFFF_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 29] = [
        Self::Frintn,
        Self::Frintm,
        Self::Frintp,
        Self::Frintz,
        Self::Frinta,
        Self::Frintx,
        Self::Frinti,
        Self::Fcvtns,
        Self::Fcvtms,
        Self::Fcvtas,
        Self::Fcvtps,
        Self::Fcvtzs,
        Self::Fcvtnu,
        Self::Fcvtmu,
        Self::Fcvtau,
        Self::Fcvtpu,
        Self::Fcvtzu,
        Self::Scvtf,
        Self::Ucvtf,
        Self::Frecpe,
        Self::Frsqrte,
        Self::Fabs,
        Self::Fneg,
        Self::Fsqrt,
        Self::FcmeqZero,
        Self::FcmgeZero,
        Self::FcmgtZero,
        Self::FcmleZero,
        Self::FcmltZero,
    ];
}
