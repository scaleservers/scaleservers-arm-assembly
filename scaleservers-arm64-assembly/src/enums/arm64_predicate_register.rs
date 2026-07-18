// Copyright (c) Scaleservers LLC

use alloc::format;
use alloc::string::String;

use crate::enums::Arm64VectorElement;

/// An SVE **predicate register** operand, named `P0`..`P15` -- the 4-bit register field of the SVE predicate file
/// (DDI0487 part C). A predicate holds one bit per byte-lane of a scalable vector; an instruction's element size
/// determines how many of those bits are active.
///
/// Many instructions take a *governing* predicate `Pg` that selects active elements, often restricted to `P0`..`P7`
/// (a 3-bit field). The register itself is just a NUMBER here; the element-size suffix and the zeroing/merging
/// qualifier are carried separately on each instruction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Arm64PredicateRegister {
    P0,
    P1,
    P2,
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13,
    P14,
    P15,
}

impl Arm64PredicateRegister {
    /// The 4-bit register field value (`0..=15`).
    pub fn as_operand_bits(&self) -> u8 {
        *self as u8
    }

    /// Map a 4-bit register field to its register. TOTAL: only the low four bits are significant, so this never
    /// panics on untrusted instruction words. Higher bits are ignored.
    pub fn from_operand_bits(bits: u8) -> Self {
        [
            Self::P0,
            Self::P1,
            Self::P2,
            Self::P3,
            Self::P4,
            Self::P5,
            Self::P6,
            Self::P7,
            Self::P8,
            Self::P9,
            Self::P10,
            Self::P11,
            Self::P12,
            Self::P13,
            Self::P14,
            Self::P15,
        ][(bits & 0b1111) as usize]
    }

    /// The lowercase UAL name with the given element-size suffix: `p0.s`, `p15.b`, ...
    pub fn name_with_element(&self, element: Arm64VectorElement) -> String {
        format!("p{}.{}", self.as_operand_bits(), element.name())
    }

    /// The lowercase UAL name with no element suffix (`p0`), used for a governing predicate written `Pg`.
    pub fn bare_name(&self) -> String {
        format!("p{}", self.as_operand_bits())
    }
}
