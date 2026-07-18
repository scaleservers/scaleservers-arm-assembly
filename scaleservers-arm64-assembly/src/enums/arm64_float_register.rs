// Copyright (c) Scaleservers LLC

use alloc::string::String;

use crate::enums::Arm64FloatPrecision;

/// A SIMD&FP register operand, named `V0`..`V31` -- the 5-bit register field of the AArch64 floating-point /
/// vector register file.
///
/// AArch64 has 32 SIMD&FP registers. For SCALAR floating-point they are accessed as `Sn` (the low 32 bits, an
/// f32) or `Dn` (the low 64 bits, an f64) -- two *views* of one register `Vn`, exactly as `Wn`/`Xn` are two
/// views of one general-purpose register. So this enum keeps a single register NUMBER (`V0`..`V31`, `0..=31`)
/// and the access precision is carried separately as an [`Arm64FloatPrecision`] on each instruction variant.
/// Unlike the general-purpose file there is no zero-register / stack-pointer duality at field `31` -- `31` is
/// simply `V31`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64FloatRegister {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
}

impl Arm64FloatRegister {
    /// The 5-bit register field value (`0..=31`).
    pub fn as_operand_bits(&self) -> u8 {
        *self as u8
    }

    /// Map a 5-bit register field to its register. TOTAL: only the low five bits are significant (every value
    /// `0..=31` is covered), so this never panics on untrusted instruction words. Higher bits are ignored.
    pub fn from_operand_bits(bits: u8) -> Self {
        [
            Self::V0,
            Self::V1,
            Self::V2,
            Self::V3,
            Self::V4,
            Self::V5,
            Self::V6,
            Self::V7,
            Self::V8,
            Self::V9,
            Self::V10,
            Self::V11,
            Self::V12,
            Self::V13,
            Self::V14,
            Self::V15,
            Self::V16,
            Self::V17,
            Self::V18,
            Self::V19,
            Self::V20,
            Self::V21,
            Self::V22,
            Self::V23,
            Self::V24,
            Self::V25,
            Self::V26,
            Self::V27,
            Self::V28,
            Self::V29,
            Self::V30,
            Self::V31,
        ][(bits & 0b1_1111) as usize]
    }

    /// The lowercase UAL register name at the given [`Arm64FloatPrecision`]: `s0`..`s31` (single, the `Sn`/f32
    /// view), `d0`..`d31` (double, `Dn`/f64), or `h0`..`h31` (half, `Hn`/f16).
    pub fn name_for_precision(&self, precision: Arm64FloatPrecision) -> String {
        let number = self.as_operand_bits();
        match precision {
            Arm64FloatPrecision::Single => format!("s{number}"),
            Arm64FloatPrecision::Double => format!("d{number}"),
            Arm64FloatPrecision::Half => format!("h{number}"),
        }
    }
}
