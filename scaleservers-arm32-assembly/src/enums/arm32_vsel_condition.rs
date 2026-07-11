// Copyright (c) Scaleservers LLC

// The condition selector for the ARMv8 VSEL instruction. VSEL is itself UNCONDITIONAL (its A32 encoding has
// cond=1111); the test it applies is carried in a dedicated 2-bit `cc` field, restricted to these four. The
// other twelve ARM conditions are reached by swapping the operands and/or picking the complementary form
// (e.g. VSELLT == VSELGE with the inputs exchanged), so the architecture only encodes EQ / VS / GE / GT.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32VselCondition {
    Equal,        // VSELEQ -- Z == 1
    Overflow,     // VSELVS -- V == 1
    GreaterEqual, // VSELGE -- N == V
    GreaterThan,  // VSELGT -- Z == 0 && N == V
}
impl Arm32VselCondition {
    // the 2-bit `cc` field (encoding bits 21:20)
    pub fn cc_bits(self) -> u32 {
        match self {
            Self::Equal => 0b00,
            Self::Overflow => 0b01,
            Self::GreaterEqual => 0b10,
            Self::GreaterThan => 0b11,
        }
    }
    pub fn from_cc_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0b00 => Self::Equal,
            0b01 => Self::Overflow,
            0b10 => Self::GreaterEqual,
            _ => Self::GreaterThan,
        }
    }
}
