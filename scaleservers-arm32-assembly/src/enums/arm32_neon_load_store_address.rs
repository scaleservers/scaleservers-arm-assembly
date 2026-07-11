// Copyright (c) Scaleservers LLC

use crate::enums::Arm32GeneralPurposeRegister;

// The post-addressing mode shared by every NEON element/structure load/store (VLD1-4 / VST1-4). The Rm field
// is overloaded: 0b1111 means "[Rn] with no base update", 0b1101 means "[Rn]! -- advance the base by the
// access size", and any other value names a register whose contents are added to the base afterwards
// ([Rn], Rm). The alignment qualifier (`:64` / `:128` / ...) is encoded elsewhere, not here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32NeonLoadStoreAddress {
    Offset,                                   // [Rn]      (Rm = 0b1111)
    IncrementWriteback,                       // [Rn]!     (Rm = 0b1101)
    PostIndexed(Arm32GeneralPurposeRegister), // [Rn], Rm  (Rm = the register; must not be SP/PC)
}
impl Arm32NeonLoadStoreAddress {
    pub fn rm_bits(self) -> u32 {
        match self {
            Self::Offset => 0b1111,
            Self::IncrementWriteback => 0b1101,
            Self::PostIndexed(rm) => rm.as_operand_bits() as u32,
        }
    }
    pub fn from_rm_bits(rm: u32) -> Self {
        match rm & 0b1111 {
            0b1111 => Self::Offset,
            0b1101 => Self::IncrementWriteback,
            other => Self::PostIndexed(Arm32GeneralPurposeRegister::from_operand_bits(other as u8)),
        }
    }
}
