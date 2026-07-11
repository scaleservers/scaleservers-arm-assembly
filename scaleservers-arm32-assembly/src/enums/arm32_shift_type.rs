// Copyright (c) Scaleservers LLC

// The shift TYPE alone (no amount), used by the A32 register-shifted-register data-processing form, where
// the shift amount is taken from a register Rs -- e.g. the `lsl r3` in `add r0, r1, r2, lsl r3`. RRX has no
// register-shifted form (it is a fixed rotate-right-with-extend by one), so it is intentionally absent
// here; the immediate-shift operand uses `Arm32RegisterShift` instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32ShiftType {
    Lsl,
    Lsr,
    Asr,
    Ror,
}
impl Arm32ShiftType {
    // the 2-bit `type` field (encoding bits 6:5)
    pub fn type_bits(self) -> u32 {
        match self {
            Self::Lsl => 0b00,
            Self::Lsr => 0b01,
            Self::Asr => 0b10,
            Self::Ror => 0b11,
        }
    }
    pub fn from_type_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0b00 => Self::Lsl,
            0b01 => Self::Lsr,
            0b10 => Self::Asr,
            _ => Self::Ror,
        }
    }
}
