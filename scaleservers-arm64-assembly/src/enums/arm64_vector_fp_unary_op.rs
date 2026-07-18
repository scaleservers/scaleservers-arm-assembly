// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **floating-point** "two-register miscellaneous" unary op --
/// the encoding class `0 Q U 01110 sz_hi sz 10000 opcode 10 Rn Rd` (DDI0487 C7). The op is an orthogonal field
/// over the shared `{ arrangement, Vd, Vn }` shape; the high `size` bit (bit 23) is baked into the op (it
/// sub-selects within the FP space -- e.g. FRINTN vs FRINTP), and the arrangement supplies the `sz` low bit
/// (bit 22, single vs double), so only `.2s`/`.4s`/`.2d` are valid. The integer<->FP converts (`FCVTZS`/
/// `FCVTZU`/`SCVTF`/`UCVTF`) are the lane-wise vector forms (same arrangement in and out).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorFpUnaryOp {
    /// `FABS` -- per-lane absolute value (`U = 0`, opcode `01111`).
    FAbs,
    /// `FNEG` -- per-lane negate (`U = 1`, opcode `01111`).
    FNeg,
    /// `FSQRT` -- per-lane square root (`U = 1`, opcode `11111`).
    FSqrt,
    /// `FRINTN` -- round to nearest, ties to even (`U = 0`, size `0x`, opcode `11000`).
    FRintN,
    /// `FRINTM` -- round toward minus infinity / floor (`U = 0`, size `0x`, opcode `11001`).
    FRintM,
    /// `FRINTP` -- round toward plus infinity / ceiling (`U = 0`, size `1x`, opcode `11000`).
    FRintP,
    /// `FRINTZ` -- round toward zero / truncate (`U = 0`, size `1x`, opcode `11001`).
    FRintZ,
    /// `FRINTA` -- round to nearest, ties away from zero (`U = 1`, size `0x`, opcode `11000`).
    FRintA,
    /// `FRINTX` -- round to integral using the current mode, signalling inexact (`U = 1`, size `0x`, opcode `11001`).
    FRintX,
    /// `FRINTI` -- round to integral using the current mode (`U = 1`, size `1x`, opcode `11001`).
    FRintI,
    /// `FRECPE` -- reciprocal estimate (`U = 0`, size `1x`, opcode `11101`).
    FRecpe,
    /// `FRSQRTE` -- reciprocal square-root estimate (`U = 1`, size `1x`, opcode `11101`).
    FRsqrte,
    /// `FCVTZS` (vector, integer) -- convert to signed integer, rounding toward zero (`U = 0`, size `1x`, opcode `11011`).
    Fcvtzs,
    /// `FCVTZU` (vector, integer) -- convert to unsigned integer, rounding toward zero (`U = 1`, size `1x`, opcode `11011`).
    Fcvtzu,
    /// `SCVTF` (vector, integer) -- convert signed integer lanes to floating-point (`U = 0`, size `0x`, opcode `11101`).
    Scvtf,
    /// `UCVTF` (vector, integer) -- convert unsigned integer lanes to floating-point (`U = 1`, size `0x`, opcode `11101`).
    Ucvtf,
    /// `FCVTNS` (vector) -- convert to signed integer, rounding to nearest with ties to even (`U = 0`, size `0x`, opcode `11010`).
    Fcvtns,
    /// `FCVTNU` (vector) -- convert to unsigned integer, rounding to nearest with ties to even (`U = 1`, size `0x`, opcode `11010`).
    Fcvtnu,
    /// `FCVTMS` (vector) -- convert to signed integer, rounding toward minus infinity (`U = 0`, size `0x`, opcode `11011`).
    Fcvtms,
    /// `FCVTMU` (vector) -- convert to unsigned integer, rounding toward minus infinity (`U = 1`, size `0x`, opcode `11011`).
    Fcvtmu,
    /// `FCVTAS` (vector) -- convert to signed integer, rounding to nearest with ties away from zero (`U = 0`, size `0x`, opcode `11100`).
    Fcvtas,
    /// `FCVTAU` (vector) -- convert to unsigned integer, rounding to nearest with ties away from zero (`U = 1`, size `0x`, opcode `11100`).
    Fcvtau,
    /// `FCVTPS` (vector) -- convert to signed integer, rounding toward plus infinity (`U = 0`, size `1x`, opcode `11010`).
    Fcvtps,
    /// `FCVTPU` (vector) -- convert to unsigned integer, rounding toward plus infinity (`U = 1`, size `1x`, opcode `11010`).
    Fcvtpu,
    /// `FRINT32Z` -- round to a 32-bit signed integral value, toward zero (FEAT_FRINTTS).
    FRint32z,
    /// `FRINT32X` -- round to a 32-bit signed integral value, current mode (FEAT_FRINTTS).
    FRint32x,
    /// `FRINT64Z` -- round to a 64-bit signed integral value, toward zero (FEAT_FRINTTS).
    FRint64z,
    /// `FRINT64X` -- round to a 64-bit signed integral value, current mode (FEAT_FRINTTS).
    FRint64x,
}

impl Arm64VectorFpUnaryOp {
    /// The base word with `Q = 0` and `sz = 0` (`U`, the `size` high bit, and opcode baked in); the arrangement
    /// supplies `Q<<30` and `sz<<22`, and the registers `Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::FAbs => 0x0EA0_F800,
            Self::FNeg => 0x2EA0_F800,
            Self::FSqrt => 0x2EA1_F800,
            Self::FRintN => 0x0E21_8800,
            Self::FRintM => 0x0E21_9800,
            Self::FRintP => 0x0EA1_8800,
            Self::FRintZ => 0x0EA1_9800,
            Self::FRintA => 0x2E21_8800,
            Self::FRintX => 0x2E21_9800,
            Self::FRintI => 0x2EA1_9800,
            Self::FRecpe => 0x0EA1_D800,
            Self::FRsqrte => 0x2EA1_D800,
            Self::Fcvtzs => 0x0EA1_B800,
            Self::Fcvtzu => 0x2EA1_B800,
            Self::Scvtf => 0x0E21_D800,
            Self::Ucvtf => 0x2E21_D800,
            // FP->int rounding-mode converts (round-to-nearest-even N, toward -inf M, ties-away A, toward +inf P);
            // signed U=0 / unsigned U=1, the P forms carry the `size` high bit (bit 23). GNU-oracle verified.
            Self::Fcvtns => 0x0E21_A800,
            Self::Fcvtnu => 0x2E21_A800,
            Self::Fcvtms => 0x0E21_B800,
            Self::Fcvtmu => 0x2E21_B800,
            Self::Fcvtas => 0x0E21_C800,
            Self::Fcvtau => 0x2E21_C800,
            Self::Fcvtps => 0x0EA1_A800,
            Self::Fcvtpu => 0x2EA1_A800,
            Self::FRint32z => 0x0E21_E800,
            Self::FRint32x => 0x2E21_E800,
            Self::FRint64z => 0x0E21_F800,
            Self::FRint64x => 0x2E21_F800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::FAbs => "fabs",
            Self::FNeg => "fneg",
            Self::FSqrt => "fsqrt",
            Self::FRintN => "frintn",
            Self::FRintM => "frintm",
            Self::FRintP => "frintp",
            Self::FRintZ => "frintz",
            Self::FRintA => "frinta",
            Self::FRintX => "frintx",
            Self::FRintI => "frinti",
            Self::FRecpe => "frecpe",
            Self::FRsqrte => "frsqrte",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtzu => "fcvtzu",
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",
            Self::Fcvtns => "fcvtns",
            Self::Fcvtnu => "fcvtnu",
            Self::Fcvtms => "fcvtms",
            Self::Fcvtmu => "fcvtmu",
            Self::Fcvtas => "fcvtas",
            Self::Fcvtau => "fcvtau",
            Self::Fcvtps => "fcvtps",
            Self::Fcvtpu => "fcvtpu",
            Self::FRint32z => "frint32z",
            Self::FRint32x => "frint32x",
            Self::FRint64z => "frint64z",
            Self::FRint64x => "frint64x",
        }
    }

    /// Whether this is a FEAT_FRINTTS op (`FRINT32X`/`FRINT32Z`/`FRINT64X`/`FRINT64Z`), which gates on FEAT_FRINTTS
    /// rather than plain Advanced SIMD.
    pub fn is_frintts(self) -> bool {
        matches!(
            self,
            Self::FRint32z | Self::FRint32x | Self::FRint64z | Self::FRint64x
        )
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 28] = [
        Self::FAbs,
        Self::FNeg,
        Self::FSqrt,
        Self::FRintN,
        Self::FRintM,
        Self::FRintP,
        Self::FRintZ,
        Self::FRintA,
        Self::FRintX,
        Self::FRintI,
        Self::FRecpe,
        Self::FRsqrte,
        Self::Fcvtzs,
        Self::Fcvtzu,
        Self::Scvtf,
        Self::Ucvtf,
        Self::Fcvtns,
        Self::Fcvtnu,
        Self::Fcvtms,
        Self::Fcvtmu,
        Self::Fcvtas,
        Self::Fcvtau,
        Self::Fcvtps,
        Self::Fcvtpu,
        Self::FRint32z,
        Self::FRint32x,
        Self::FRint64z,
        Self::FRint64x,
    ];
}
