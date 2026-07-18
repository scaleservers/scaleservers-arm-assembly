// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **modified immediate** instruction (DDI0487 C7) -- the
/// `0 Q op 0111100000 abc cmode 01 defgh Rd` class that builds a vector constant from an 8-bit immediate. The
/// `cmode` field (with `op`) selects both the operation and the element size / shift; this enum is the
/// operation, [`super::Arm64VectorImmediateShift`] the shift. (`FMOV` vector-immediate, the `cmode = 1111` case,
/// is modeled separately as the dedicated `VecFmovImmediate` variant.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorModifiedImmediateOp {
    /// `MOVI` -- move the (optionally shifted) immediate into every lane (8/16/32/64-bit forms).
    Movi,
    /// `MVNI` -- move the bitwise NOT of the shifted immediate (16/32-bit forms).
    Mvni,
    /// `ORR` (vector, immediate) -- OR the shifted immediate into every lane (16/32-bit forms).
    Orr,
    /// `BIC` (vector, immediate) -- AND the NOT of the shifted immediate into every lane (16/32-bit forms).
    Bic,
}

impl Arm64VectorModifiedImmediateOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Movi => "movi",
            Self::Mvni => "mvni",
            Self::Orr => "orr",
            Self::Bic => "bic",
        }
    }
}

/// The shift applied to a NEON modified-immediate's 8-bit value before it is replicated across the element. The
/// 16-bit forms allow `LSL #0`/`#8`; the 32-bit forms allow `LSL #0`/`#8`/`#16`/`#24` and the "move shift left
/// ones" `MSL #8`/`#16`; the 8/64-bit forms allow no shift. [`Self::None`] is `LSL #0` (no suffix printed).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorImmediateShift {
    /// No shift (`LSL #0`).
    None,
    /// `LSL #8`.
    Lsl8,
    /// `LSL #16`.
    Lsl16,
    /// `LSL #24`.
    Lsl24,
    /// `MSL #8` -- shift left filling the vacated low bits with ones.
    Msl8,
    /// `MSL #16`.
    Msl16,
}

impl Arm64VectorImmediateShift {
    /// The assembly suffix this shift prints after the immediate (`""` for [`Self::None`], else `, lsl #N` /
    /// `, msl #N`).
    pub fn suffix(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Lsl8 => ", lsl #8",
            Self::Lsl16 => ", lsl #16",
            Self::Lsl24 => ", lsl #24",
            Self::Msl8 => ", msl #8",
            Self::Msl16 => ", msl #16",
        }
    }
}
