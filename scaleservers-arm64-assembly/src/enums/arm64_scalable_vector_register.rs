// Copyright (c) Scaleservers LLC

use alloc::format;
use alloc::string::String;

use crate::enums::Arm64VectorElement;

/// An SVE **scalable vector register** operand, named `Z0`..`Z31` -- the 5-bit register field of the AArch64 SVE
/// register file (DDI0487 part C / the SVE supplement).
///
/// SVE has 32 scalable vector registers whose width is an IMPLEMENTATION-DEFINED multiple of 128 bits (128..2048).
/// The low 128 bits of `Zn` alias the Advanced SIMD register `Vn`, but in SVE encodings the register is just a
/// 5-bit NUMBER; the element size (`.b`/`.h`/`.s`/`.d`) is carried separately as an [`Arm64VectorElement`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arm64ScalableVectorRegister {
    Z0,
    Z1,
    Z2,
    Z3,
    Z4,
    Z5,
    Z6,
    Z7,
    Z8,
    Z9,
    Z10,
    Z11,
    Z12,
    Z13,
    Z14,
    Z15,
    Z16,
    Z17,
    Z18,
    Z19,
    Z20,
    Z21,
    Z22,
    Z23,
    Z24,
    Z25,
    Z26,
    Z27,
    Z28,
    Z29,
    Z30,
    Z31,
}

impl Arm64ScalableVectorRegister {
    /// The 5-bit register field value (`0..=31`).
    pub fn as_operand_bits(&self) -> u8 {
        *self as u8
    }

    /// Map a 5-bit register field to its register. TOTAL: only the low five bits are significant, so this never
    /// panics on untrusted instruction words. Higher bits are ignored.
    pub fn from_operand_bits(bits: u8) -> Self {
        [
            Self::Z0,
            Self::Z1,
            Self::Z2,
            Self::Z3,
            Self::Z4,
            Self::Z5,
            Self::Z6,
            Self::Z7,
            Self::Z8,
            Self::Z9,
            Self::Z10,
            Self::Z11,
            Self::Z12,
            Self::Z13,
            Self::Z14,
            Self::Z15,
            Self::Z16,
            Self::Z17,
            Self::Z18,
            Self::Z19,
            Self::Z20,
            Self::Z21,
            Self::Z22,
            Self::Z23,
            Self::Z24,
            Self::Z25,
            Self::Z26,
            Self::Z27,
            Self::Z28,
            Self::Z29,
            Self::Z30,
            Self::Z31,
        ][(bits & 0b1_1111) as usize]
    }

    /// The lowercase UAL name with the given element-size suffix: `z0.s`, `z31.b`, ...
    pub fn name_with_element(&self, element: Arm64VectorElement) -> String {
        format!("z{}.{}", self.as_operand_bits(), element.name())
    }

    /// The lowercase UAL name with no element suffix (`z0`), used where the instruction names the whole register.
    pub fn bare_name(&self) -> String {
        format!("z{}", self.as_operand_bits())
    }
}
