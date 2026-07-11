// Copyright (c) Scaleservers LLC

// Operation taxonomies for the ARMv8.1-M MVE ("Helium") "three registers of the same length" vector
// data-processing format (the vector-vector form `Qd, Qn, Qm`). All three live in the 0xEF.. / 0xFF..
// encoding space (top byte 1110_1111 / 1111_1111), disjoint from the scalar-FPU 0xEE.. space, and split
// into three sub-families that this module models as separate op-enums:
//
//   * integer   (`Arm32MveIntArithOp`)   -- element size in bits[21:20] (.i/.s/.u 8|16|32). The U bit (28)
//                                           folds add/sub or signedness depending on the mnemonic, so each
//                                           mnemonic carries its full base word.
//   * bitwise   (`Arm32MveBitwiseOp`)    -- NOT size-parametric; bits[21:20] are the boolean-function select.
//   * float     (`Arm32MveFloatArithOp`) -- element size is the single bit 20 (.f32 = 0, .f16 = 1).
//
// Each op stores its BASE WORD: the 32-bit encoding with the Qd[15:13] / Qn[19:17] / Qm[3:1] operand fields
// and the size field(s) all zeroed. Encoding ORs the operands and size back in; decoding masks them out
// (the family's signature mask) and matches the remainder against the table. The three families never
// collide because bits[11:8]+bit4 are disjoint: integer in {0,1,2,6,7,9}, bitwise = 0001 with bit4 = 1,
// float in {0xC,0xD,0xF}. All base words below are transcribed from `arm-none-eabi-as
// -march=armv8.1-m.main+mve.fp` output.

// signature masks: a word AND'd with the family mask drops its operand (and size) fields, leaving the
// opcode signature to match against the table's base words.
pub const MVE_INT_SIGNATURE_MASK: u32 = 0xFFC1_1FF1; // clears Qd/Qn/Qm and size[21:20]
pub const MVE_BITWISE_SIGNATURE_MASK: u32 = 0xFFF1_1FF1; // clears Qd/Qn/Qm only (keeps the [21:20] selector)
pub const MVE_FLOAT_SIGNATURE_MASK: u32 = 0xFFE1_1FF1; // clears Qd/Qn/Qm and the single size bit 20

// Element size for the size-parametric integer operations (the 2-bit `size` field, bits 21:20). MVE vector
// arithmetic operates on 8/16/32-bit lanes; 0b11 is not a valid element size here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveSize {
    I8,
    I16,
    I32,
}
impl Arm32MveSize {
    pub fn size_bits(self) -> u32 {
        match self {
            Self::I8 => 0b00,
            Self::I16 => 0b01,
            Self::I32 => 0b10,
        }
    }
    pub fn from_size_bits(bits: u32) -> Option<Self> {
        match bits & 0b11 {
            0b00 => Some(Self::I8),
            0b01 => Some(Self::I16),
            0b10 => Some(Self::I32),
            _ => None, // 0b11 is reserved for these forms
        }
    }
    // the element-width digits used in the UAL type suffix (e.g. the "8" in ".i8")
    pub fn width_digits(self) -> &'static str {
        match self {
            Self::I8 => "8",
            Self::I16 => "16",
            Self::I32 => "32",
        }
    }
}

// Element size for the floating-point operations: a single bit (bit 20).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveFloatSize {
    F16,
    F32,
}
impl Arm32MveFloatSize {
    // the value of bit 20 for this size
    pub fn size_bit(self) -> u32 {
        match self {
            Self::F32 => 0,
            Self::F16 => 1,
        }
    }
    pub fn from_size_bit(bit: u32) -> Self {
        if bit & 1 == 1 { Self::F16 } else { Self::F32 }
    }
    pub fn width_digits(self) -> &'static str {
        match self {
            Self::F16 => "16",
            Self::F32 => "32",
        }
    }
}

// Integer 3-reg-same vector-vector operations. `type_prefix` is the UAL type letter: VADD/VSUB/VMUL are
// signedness-agnostic (".i"), the rest carry signedness in the mnemonic (".s"/".u", which is what sets the
// U bit, bit 28).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveIntArithOp {
    Vadd, Vsub, Vmul,
    VqaddS, VqaddU, VqsubS, VqsubU,
    VhaddS, VhaddU, VhsubS, VhsubU,
    VrhaddS, VrhaddU,
    VabdS, VabdU,
    VmaxS, VmaxU, VminS, VminU,
    // saturating (rounding) doubling multiply high -- signed-only; VQRDMULH is the [28]=1 rounding twin.
    VqdmulhS, VqrdmulhS,
}
impl Arm32MveIntArithOp {
    // base word with Qd / Qn / Qm and size[21:20] zeroed
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vadd    => 0xEF00_0840,
            Self::Vsub    => 0xFF00_0840,
            Self::Vmul    => 0xEF00_0950,
            Self::VqaddS  => 0xEF00_0050,
            Self::VqaddU  => 0xFF00_0050,
            Self::VqsubS  => 0xEF00_0250,
            Self::VqsubU  => 0xFF00_0250,
            Self::VhaddS  => 0xEF00_0040,
            Self::VhaddU  => 0xFF00_0040,
            Self::VhsubS  => 0xEF00_0240,
            Self::VhsubU  => 0xFF00_0240,
            Self::VrhaddS => 0xEF00_0140,
            Self::VrhaddU => 0xFF00_0140,
            Self::VabdS   => 0xEF00_0740,
            Self::VabdU   => 0xFF00_0740,
            Self::VmaxS   => 0xEF00_0640,
            Self::VmaxU   => 0xFF00_0640,
            Self::VminS   => 0xEF00_0650,
            Self::VminU   => 0xFF00_0650,
            Self::VqdmulhS  => 0xEF00_0B40,
            Self::VqrdmulhS => 0xFF00_0B40,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vadd => "vadd",
            Self::Vsub => "vsub",
            Self::Vmul => "vmul",
            Self::VqaddS | Self::VqaddU => "vqadd",
            Self::VqsubS | Self::VqsubU => "vqsub",
            Self::VhaddS | Self::VhaddU => "vhadd",
            Self::VhsubS | Self::VhsubU => "vhsub",
            Self::VrhaddS | Self::VrhaddU => "vrhadd",
            Self::VabdS | Self::VabdU => "vabd",
            Self::VmaxS | Self::VmaxU => "vmax",
            Self::VminS | Self::VminU => "vmin",
            Self::VqdmulhS => "vqdmulh",
            Self::VqrdmulhS => "vqrdmulh",
        }
    }
    // the UAL type letter: 'i' (signedness-agnostic), 's' (signed) or 'u' (unsigned)
    pub fn type_prefix(self) -> char {
        match self {
            Self::Vadd | Self::Vsub | Self::Vmul => 'i',
            Self::VqaddS | Self::VqsubS | Self::VhaddS | Self::VhsubS | Self::VrhaddS
            | Self::VabdS | Self::VmaxS | Self::VminS | Self::VqdmulhS | Self::VqrdmulhS => 's',
            Self::VqaddU | Self::VqsubU | Self::VhaddU | Self::VhsubU | Self::VrhaddU
            | Self::VabdU | Self::VmaxU | Self::VminU => 'u',
        }
    }
    pub const ALL: [Self; 21] = [
        Self::Vadd, Self::Vsub, Self::Vmul,
        Self::VqaddS, Self::VqaddU, Self::VqsubS, Self::VqsubU,
        Self::VhaddS, Self::VhaddU, Self::VhsubS, Self::VhsubU,
        Self::VrhaddS, Self::VrhaddU,
        Self::VabdS, Self::VabdU,
        Self::VmaxS, Self::VmaxU, Self::VminS, Self::VminU,
        Self::VqdmulhS, Self::VqrdmulhS,
    ];
    // recover the op from a signature (`word & MVE_INT_SIGNATURE_MASK`)
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// Bitwise 3-reg-same operations. Not size-parametric: bits[21:20] (and the U bit for VEOR) select the
// boolean function, so they are baked into each base word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveBitwiseOp {
    Vand, Vbic, Vorr, Vorn, Veor,
}
impl Arm32MveBitwiseOp {
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vand => 0xEF00_0150,
            Self::Vbic => 0xEF10_0150,
            Self::Vorr => 0xEF20_0150,
            Self::Vorn => 0xEF30_0150,
            Self::Veor => 0xFF00_0150,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vand => "vand",
            Self::Vbic => "vbic",
            Self::Vorr => "vorr",
            Self::Vorn => "vorn",
            Self::Veor => "veor",
        }
    }
    pub const ALL: [Self; 5] = [Self::Vand, Self::Vbic, Self::Vorr, Self::Vorn, Self::Veor];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// Floating-point 3-reg-same vector-vector operations. Element size is the single bit 20 (.f32 / .f16).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveFloatArithOp {
    Vadd, Vsub, Vmul, Vabd, Vmaxnm, Vminnm, Vfma, Vfms,
}
impl Arm32MveFloatArithOp {
    // base word with Qd / Qn / Qm and the size bit (20) zeroed (i.e. the .f32 form)
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vadd   => 0xEF00_0D40,
            Self::Vsub   => 0xEF20_0D40,
            Self::Vmul   => 0xFF00_0D50,
            Self::Vabd   => 0xFF20_0D40,
            Self::Vmaxnm => 0xFF00_0F50,
            Self::Vminnm => 0xFF20_0F50,
            Self::Vfma   => 0xEF00_0C50,
            Self::Vfms   => 0xEF20_0C50,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vadd => "vadd",
            Self::Vsub => "vsub",
            Self::Vmul => "vmul",
            Self::Vabd => "vabd",
            Self::Vmaxnm => "vmaxnm",
            Self::Vminnm => "vminnm",
            Self::Vfma => "vfma",
            Self::Vfms => "vfms",
        }
    }
    pub const ALL: [Self; 8] = [
        Self::Vadd, Self::Vsub, Self::Vmul, Self::Vabd,
        Self::Vmaxnm, Self::Vminnm, Self::Vfma, Self::Vfms,
    ];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// ---- MVE "vector by scalar" forms: `Qd, Qn, Rm` (a vector operand and a GPR scalar) ----
// These live in the 0xEE.. / 0xFE.. space (top byte 1110_1110 / 1111_1110), shared with the scalar FPU, but
// are disjoint from it: the scalar-FP coprocessor field bits[11:8] are 0b1010/0b1011 (A/B), while the MVE
// vector-by-scalar ops use 0b1110/0b1111 (E/F). Register fields: Qd[15:13], Qn[19:17], and the GPR Rm[3:0].

pub const MVE_VBS_INT_SIGNATURE_MASK: u32 = 0xFFC1_1FF0; // clears Qd/Qn, size[21:20], and Rm[3:0]
pub const MVE_VBS_FLOAT_SIGNATURE_MASK: u32 = 0xEFF1_1FF0; // clears Qd/Qn, Rm, and the size bit 28 (keeps [21:20]=11)

// Integer vector-by-scalar operations. Element size is bits[21:20] (I8/I16/I32; 0b11 is the float marker,
// so it is reserved here). Signedness, where it applies, sets the U bit (28) and is part of the mnemonic.
// EXCEPTION: VMLA/VMLAS (vector by scalar) are signedness-agnostic -- DDI0553 C2.4.380/C2.4.384 fix bit 28
// to (0) and give <dt> = I8/I16/I32 only (no S/U). GNU wrongly models a U bit here; see mve_tests.rs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveVecScalarIntOp {
    Vadd, Vsub, Vmul,
    VhaddS, VhaddU, VhsubS, VhsubU,
    VqaddS, VqaddU, VqsubS, VqsubU,
    VqdmulhS, VqrdmulhS,
    // multiply-accumulate forms (the destination Qda is also an accumulator source); Vmla/Vmlas are I-typed
    Vmla, Vmlas,
    VqdmlahS, VqrdmlahS, VqdmlashS, VqrdmlashS,
}
impl Arm32MveVecScalarIntOp {
    // base word with Qd / Qn / Rm and size[21:20] zeroed
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vadd      => 0xEE01_0F40,
            Self::Vsub      => 0xEE01_1F40,
            Self::Vmul      => 0xEE01_1E60,
            Self::VhaddS    => 0xEE00_0F40,
            Self::VhaddU    => 0xFE00_0F40,
            Self::VhsubS    => 0xEE00_1F40,
            Self::VhsubU    => 0xFE00_1F40,
            Self::VqaddS    => 0xEE00_0F60,
            Self::VqaddU    => 0xFE00_0F60,
            Self::VqsubS    => 0xEE00_1F60,
            Self::VqsubU    => 0xFE00_1F60,
            Self::VqdmulhS  => 0xEE01_0E60,
            Self::VqrdmulhS => 0xFE01_0E60,
            Self::Vmla      => 0xEE01_0E40, // bit 28 = (0) fixed per DDI0553 C2.4.380 (NOT a U bit)
            Self::Vmlas     => 0xEE01_1E40, //  "      "     "       "     C2.4.384
            Self::VqdmlahS  => 0xEE00_0E60,
            Self::VqrdmlahS => 0xEE00_0E40,
            Self::VqdmlashS => 0xEE00_1E60,
            Self::VqrdmlashS => 0xEE00_1E40,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vadd => "vadd",
            Self::Vsub => "vsub",
            Self::Vmul => "vmul",
            Self::VhaddS | Self::VhaddU => "vhadd",
            Self::VhsubS | Self::VhsubU => "vhsub",
            Self::VqaddS | Self::VqaddU => "vqadd",
            Self::VqsubS | Self::VqsubU => "vqsub",
            Self::VqdmulhS => "vqdmulh",
            Self::VqrdmulhS => "vqrdmulh",
            Self::Vmla => "vmla",
            Self::Vmlas => "vmlas",
            Self::VqdmlahS => "vqdmlah",
            Self::VqrdmlahS => "vqrdmlah",
            Self::VqdmlashS => "vqdmlash",
            Self::VqrdmlashS => "vqrdmlash",
        }
    }
    pub fn type_prefix(self) -> char {
        match self {
            Self::Vadd | Self::Vsub | Self::Vmul | Self::Vmla | Self::Vmlas => 'i',
            Self::VhaddS | Self::VhsubS | Self::VqaddS | Self::VqsubS | Self::VqdmulhS | Self::VqrdmulhS
            | Self::VqdmlahS | Self::VqrdmlahS | Self::VqdmlashS | Self::VqrdmlashS => 's',
            Self::VhaddU | Self::VhsubU | Self::VqaddU | Self::VqsubU => 'u',
        }
    }
    pub const ALL: [Self; 19] = [
        Self::Vadd, Self::Vsub, Self::Vmul,
        Self::VhaddS, Self::VhaddU, Self::VhsubS, Self::VhsubU,
        Self::VqaddS, Self::VqaddU, Self::VqsubS, Self::VqsubU,
        Self::VqdmulhS, Self::VqrdmulhS,
        Self::Vmla, Self::Vmlas,
        Self::VqdmlahS, Self::VqrdmlahS, Self::VqdmlashS, Self::VqrdmlashS,
    ];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}
