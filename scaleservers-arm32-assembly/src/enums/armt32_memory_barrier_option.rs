// Copyright (c) Scaleservers LLC

#[derive(Debug, PartialEq)]
pub enum ArmT32MemoryBarrierOption {
    System, // SY
    Undefined(/*u4*/u8)
}
impl ArmT32MemoryBarrierOption {
    pub fn as_operand_bits(&self) -> u8 {
        match self {
            Self::System => 0b1111,
            Self::Undefined(bits) => {
                assert!(*bits <= 0b1111, "Member Undefined's field 'value' is out of the valid range (0b0000..=0b1111)");

                *bits
            }
        }
    }

    /// Map a 4-bit barrier-option field to its option. TOTAL: only the low four bits are significant
    /// (0b1111 = SY; every other value is an `Undefined` option), so this never panics -- the decoder derives
    /// the option from untrusted instruction bytes (a DMB/DSB/ISB with any option 0-14, not just 8-14).
    pub fn from_operand_bits(bits: u8) -> Self {
        match bits & 0b1111 {
            0b1111 => Self::System,
            other => Self::Undefined(other),
        }
    }
}
