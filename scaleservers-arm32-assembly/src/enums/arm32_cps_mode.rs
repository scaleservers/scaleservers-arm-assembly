// Copyright (c) Scaleservers LLC

// The interrupt-masking effect of an A32 CPS (Change Processor State) instruction -- the `imod` field.
// `NoChange` is the bare `CPS #mode` (change mode only); `Disable`/`Enable` are CPSID/CPSIE, which set or
// clear the A/I/F interrupt-mask bits selected by the iflags. CPS is always unconditional (cond = 1111).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32CpsMode {
    NoChange, // imod = 00 (CPS #mode)
    Disable,  // imod = 10 (CPSID)
    Enable,   // imod = 11 (CPSIE)
}
impl Arm32CpsMode {
    pub fn imod_bits(self) -> u32 {
        match self {
            Self::NoChange => 0b00,
            Self::Enable => 0b10,  // CPSIE (verified against GNU `as`)
            Self::Disable => 0b11, // CPSID
        }
    }
    pub fn from_imod_bits(bits: u32) -> Option<Self> {
        // `imod` is a 2-bit field, so mask off any higher bits a caller might pass -- matching the sibling
        // field decoders (`from_operand_bits`, `from_type_bits`), which stay total over untrusted instruction
        // bytes (decode must never panic). After the mask the only value the fallback can see is 0b01.
        match bits & 0b11 {
            0b00 => Some(Self::NoChange),
            0b10 => Some(Self::Enable),
            0b11 => Some(Self::Disable),
            _ => None, // 0b01 (reserved) -- the only value `& 0b11` leaves for this arm
        }
    }
}
