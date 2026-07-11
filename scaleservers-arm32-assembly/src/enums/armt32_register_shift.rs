// Copyright (c) Scaleservers LLC

// The optional shift applied to the second register operand (Rm) of an ARMv7-M data-processing
// (shifted register) instruction, e.g. the `, lsl #3` in `add.w r0, r1, r2, lsl #3`. The amount is the
// DECODED (UAL) value: LSL is 0..=31 (0 means "no shift"); LSR and ASR are 1..=32; ROR is 1..=31. `Rrx`
// (rotate-right-with-extend) is ROR with a zero amount field and carries no count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmT32RegisterShift {
    Lsl(u8),
    Lsr(u8),
    Asr(u8),
    Ror(u8),
    Rrx,
}
impl ArmT32RegisterShift {
    // the "no shift" case (LSL #0), which renders with no shift suffix
    pub fn none() -> Self {
        Self::Lsl(0)
    }
    pub fn is_none(&self) -> bool {
        matches!(self, Self::Lsl(0))
    }

    // the 2-bit `type` field (encoding bits 5:4); RRX shares the ROR type
    pub fn type_bits(&self) -> u8 {
        match self {
            Self::Lsl(_) => 0b00,
            Self::Lsr(_) => 0b01,
            Self::Asr(_) => 0b10,
            Self::Ror(_) | Self::Rrx => 0b11,
        }
    }
}
