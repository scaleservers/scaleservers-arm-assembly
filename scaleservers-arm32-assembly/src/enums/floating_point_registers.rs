// Copyright (c) Scaleservers LLC

// The VFP / Advanced-SIMD (NEON) register files. M-profile FPUs provide 32 single-precision registers
// S0..S31 and 16 double-precision registers D0..D15 (the single registers alias the low half of the
// doubles). VFPv3-D32 and NEON widen the double file to D0..D31, and NEON adds 16 quadword registers
// Q0..Q15 (Qn aliases the D-pair D2n:D2n+1). Each is a small newtype over the register NUMBER.
//
// VFP/NEON encodings never store the register number contiguously: a register splits into a 4-bit field and
// a 1-bit "extra" bit that lives elsewhere in the word (the D/N/M bit for the Vd/Vn/Vm operand slots). The
// split differs by width:
//   * single Sn : field = n >> 1, extra = n & 1
//   * double Dn : field = n & 0xF, extra = (n >> 4) & 1     (extra is 0 for D0..D15)
//   * quad   Qn : encoded as the double register D(2n), i.e. field = (2n) & 0xF, extra = (2n >> 4) & 1

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arm32SinglePrecisionRegister(u8);
impl Arm32SinglePrecisionRegister {
    pub fn new(number: u8) -> Option<Self> {
        if number <= 31 { Some(Self(number)) } else { None }
    }
    pub fn number(&self) -> u8 {
        self.0
    }
    // the 4-bit Vd/Vn/Vm field for this register
    pub fn field(&self) -> u32 {
        (self.0 >> 1) as u32
    }
    // the 1-bit D/N/M "extra" bit for this register
    pub fn extra_bit(&self) -> u32 {
        (self.0 & 1) as u32
    }
    // recover a single register from a (field, extra) pair (the inverse of the split above)
    pub fn from_field_and_bit(field: u32, extra_bit: u32) -> Self {
        Self((((field << 1) | (extra_bit & 1)) & 0x1F) as u8)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arm32DoublePrecisionRegister(u8);
impl Arm32DoublePrecisionRegister {
    // D0..D31 (VFPv3-D32 / NEON). A VFPv2 / VFPv3-D16 target only has D0..D15; that restriction is enforced
    // at the emit gate (via the instruction's feature requirement), not here.
    pub fn new(number: u8) -> Option<Self> {
        if number <= 31 { Some(Self(number)) } else { None }
    }
    pub fn number(&self) -> u8 {
        self.0
    }
    pub fn field(&self) -> u32 {
        (self.0 & 0xF) as u32
    }
    pub fn extra_bit(&self) -> u32 {
        ((self.0 >> 4) & 1) as u32
    }
    pub fn from_field_and_bit(field: u32, extra_bit: u32) -> Self {
        Self(((field & 0xF) | ((extra_bit & 1) << 4)) as u8)
    }
}

// The NEON quadword register file Q0..Q15. A quad register is encoded exactly like the double register D(2n),
// so `field`/`extra_bit` yield the D(2n) split; the surrounding instruction's Q bit marks it as 128-bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Arm32QuadwordRegister(u8);
impl Arm32QuadwordRegister {
    pub fn new(number: u8) -> Option<Self> {
        if number <= 15 { Some(Self(number)) } else { None }
    }
    pub fn number(&self) -> u8 {
        self.0
    }
    // Qn occupies the encoding slot of the double register D(2n).
    pub fn field(&self) -> u32 {
        ((self.0 << 1) & 0xF) as u32
    }
    pub fn extra_bit(&self) -> u32 {
        ((self.0 >> 3) & 1) as u32
    }
    // recover Qn from the (field, extra) of its underlying D(2n) (the low bit of 2n is always 0)
    pub fn from_field_and_bit(field: u32, extra_bit: u32) -> Self {
        let double_number = (field & 0xF) | ((extra_bit & 1) << 4);
        Self((double_number >> 1) as u8)
    }
}
