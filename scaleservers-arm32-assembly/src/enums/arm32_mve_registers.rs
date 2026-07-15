// Copyright (c) Scaleservers LLC

// The ARMv8.1-M MVE ("Helium", the M-profile Vector Extension) vector register file Q0..Q7. MVE shares the
// floating-point register bank with the scalar FPU (the 128-bit Qn aliases the double pair D2n:D2n+1, i.e.
// the four singles S4n..S4n+3), but only the low EIGHT quad registers exist on M-profile -- there is no
// Q8..Q15 (the M-profile FP bank stops at D15 / S31).
//
// Unlike the A-profile NEON quad register (which is encoded as the split D(2n) field + a 1-bit "extra"
// scattered elsewhere in the word -- see `Arm32QuadwordRegister`), an MVE vector register is encoded as a
// plain CONTIGUOUS 3-bit field. The three operand slots sit at fixed positions in the 32-bit word:
//   * Qd : bits [15:13]
//   * Qn : bits [19:17]
//   * Qm : bits  [3:1]
// (bit 22 / bit 7 / bit 5 -- the slots the NEON "extra" bit would occupy -- are part of the fixed opcode for
// MVE, so we never set them here.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arm32MveVectorRegister(u8);
impl Arm32MveVectorRegister {
    pub fn new(number: u8) -> Option<Self> {
        if number <= 7 {
            Some(Self(number))
        } else {
            None
        }
    }
    pub fn number(&self) -> u8 {
        self.0
    }
    // the contiguous 3-bit register field (identical for the Qd/Qn/Qm slots; the caller shifts it into place)
    pub fn field(&self) -> u32 {
        self.0 as u32
    }
    // recover a register from a 3-bit field (the low 3 bits are the register number)
    pub fn from_field(field: u32) -> Self {
        Self((field & 0b111) as u8)
    }
}
