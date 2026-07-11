// Copyright (c) Scaleservers LLC

// The six signed/unsigned byte/halfword extend operations shared by the A32 extend (SXTB / SXTH /
// SXTB16 / UXTB / UXTH / UXTB16) and extend-and-add (SXTAB / SXTAH / SXTAB16 / UXTAB / UXTAH / UXTAB16)
// instruction families -- the two differ only by whether Rn is the no-add marker (1111) or a real
// accumulator. The signed/unsigned choice and element size are carried by the [27:20] opcode byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32ExtendType {
    Sxtb16,
    Sxtb,
    Sxth,
    Uxtb16,
    Uxtb,
    Uxth,
}
impl Arm32ExtendType {
    // the [27:20] opcode byte (its bit3 -- i.e. [23] -- is always 1; the no-add vs add choice is Rn, not here)
    pub fn opcode_byte(self) -> u32 {
        match self {
            Self::Sxtb16 => 0x68,
            Self::Sxtb => 0x6A,
            Self::Sxth => 0x6B,
            Self::Uxtb16 => 0x6C,
            Self::Uxtb => 0x6E,
            Self::Uxth => 0x6F,
        }
    }
    pub fn from_opcode_byte(byte: u32) -> Option<Self> {
        Some(match byte {
            0x68 => Self::Sxtb16,
            0x6A => Self::Sxtb,
            0x6B => Self::Sxth,
            0x6C => Self::Uxtb16,
            0x6E => Self::Uxtb,
            0x6F => Self::Uxth,
            _ => return None,
        })
    }
}
