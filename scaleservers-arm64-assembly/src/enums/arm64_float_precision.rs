// Copyright (c) Scaleservers LLC

/// The precision (and operand size) of an AArch64 scalar floating-point access: **Single** (32-bit, `Sn`, an
/// f32), **Double** (64-bit, `Dn`, an f64), or **Half** (16-bit, `Hn`, an f16).
///
/// Like the general-purpose [`Arm64RegisterWidth`](super::Arm64RegisterWidth), the precision is NOT part of the
/// register number -- the same `Vn` register is accessed as `Hn`/`Sn`/`Dn` -- so it is carried as a separate
/// per-instruction-variant field, mapping to the encoding's 2-bit `ftype` field `[23:22]`: `S = 00`, `D = 01`,
/// `H = 11` (`10` is reserved). Because precision is an orthogonal 2-bit encoding field, it is modeled as a
/// shared per-variant field rather than baked into the mnemonic.
///
/// **Scope note:** [`Self::Half`] is used by the FP16 convert family (`FCVT` to/from `H`) AND the
/// precision-carrying scalar FP data-processing forms (the 2-source FADD/FSUB/FMUL/FDIV/FMAX/FMIN, the 1-source
/// and 3-source forms, FCMP, and FCSEL all accept `Half`, ftype = 11), under `FEAT_FP16`. Only [`Self::from_word`]
/// is Single/Double-only (it cannot express Half); the half-capable forms decode via [`Self::from_ftype_bits`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64FloatPrecision {
    /// 32-bit single precision (`Sn`, f32) -- `ftype = 00`.
    Single,
    /// 64-bit double precision (`Dn`, f64) -- `ftype = 01`.
    Double,
    /// 16-bit half precision (`Hn`, f16) -- `ftype = 11`. Used by the FP16 convert and data-processing forms; needs `FEAT_FP16`.
    Half,
}

impl Arm64FloatPrecision {
    /// The 2-bit `ftype` field value: `0b00` Single, `0b01` Double, `0b11` Half. The encoder shifts this into
    /// bits `[23:22]` (the source-precision `ftype`) or `[16:15]` (the `FCVT` destination `opc`, same code).
    pub fn ftype(self) -> u32 {
        match self {
            Self::Single => 0b00,
            Self::Double => 0b01,
            Self::Half => 0b11,
        }
    }

    /// Recover a precision from a 2-bit `ftype`/`opc` field, or `None` for the reserved `0b10`. Used by the
    /// convert (`FCVT`) decoder, which reads source and destination precisions independently.
    pub fn from_ftype_bits(bits: u32) -> Option<Self> {
        match bits & 0b11 {
            0b00 => Some(Self::Single),
            0b01 => Some(Self::Double),
            0b11 => Some(Self::Half),
            _ => None, // 0b10 is reserved
        }
    }

    /// Whether this is the half-precision form (which gates on `FEAT_FP16`).
    pub fn is_half(self) -> bool {
        matches!(self, Self::Half)
    }

    /// Recover the **single/double** precision from an instruction word's `ftype` bit (bit 22): [`Self::Double`] if
    /// set, else [`Self::Single`]. Used by the scalar-FP forms whose decode mask pins bit 23 = 0 (so only ftype
    /// 00/01 reach here -- the S/D-only FRINTTS and the FP<->int converts). The half-capable data-processing forms
    /// open bit 23 and decode their precision via [`Self::from_ftype_bits`] (which also rejects the reserved 10).
    pub fn from_word(word: u32) -> Self {
        if (word >> 22) & 1 == 1 {
            Self::Double
        } else {
            Self::Single
        }
    }
}
