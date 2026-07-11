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

// Floating-point vector-by-scalar operations. These sit at bits[21:20] = 0b11 (the size value reserved by
// the integer forms), with the element size carried in the single bit 28 (.f32 = 0, .f16 = 1).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveVecScalarFloatOp {
    Vadd, Vsub, Vmul, Vfma, Vfmas,
}
impl Arm32MveVecScalarFloatOp {
    // base word with Qd / Qn / Rm and the size bit (28) zeroed (i.e. the .f32 form)
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vadd => 0xEE30_0F40,
            Self::Vsub => 0xEE30_1F40,
            Self::Vmul => 0xEE31_0E60,
            Self::Vfma => 0xEE31_0E40,
            Self::Vfmas => 0xEE31_1E40,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vadd => "vadd",
            Self::Vsub => "vsub",
            Self::Vmul => "vmul",
            Self::Vfma => "vfma",
            Self::Vfmas => "vfmas",
        }
    }
    pub const ALL: [Self; 5] = [Self::Vadd, Self::Vsub, Self::Vmul, Self::Vfma, Self::Vfmas];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// VDUP (`Qd, Rt`): broadcast a GPR into every lane. The element size is encoded as the scattered pair
// {B = bit 22, E = bit 5}: .32 = {0,0}, .16 = {0,1}, .8 = {1,0}. Qd sits at bits[19:17], Rt at bits[15:12].
pub const MVE_VDUP_MASK: u32 = 0xFFB1_0FDF; // the fixed opcode bits (clears Qd, Rt, B and E)
pub const MVE_VDUP_BASE: u32 = 0xEEA0_0B10; // size .32, Qd0, Rt0

// VIDUP/VDDUP/VIWDUP/VDWDUP -- increment/decrement-and-duplicate index generators (`Qd, Rn, #imm` and, for the
// wrapping forms, `Qd, Rn, Rm, #imm`). 0xEE0x-0xEE2x space (unsigned, bit28=0). size[21:20]; Rn[19:17]=Rn>>1
// (Rn EVEN); Qd[15:13]; decrement(VDDUP/VDWDUP)=bit12; the step in {1,2,4,8} is log2 split across {bit7,bit0};
// the wrap register Rm[3:1]=Rm>>1 (Rm ODD) -- and Rm = PC ([3:1]=111) marks the NON-wrapping forms (VIDUP/VDDUP).
pub const MVE_VIDDUP_BASE: u32 = 0xEE01_0F60;
pub const MVE_VIDDUP_MASK: u32 = 0xFFC1_0F70;

// VBRSR -- Vector Bit Reverse and Shift Right by a GPR amount broadcast to all lanes (`Qd, Qn, Rm`). It is a
// vector-by-scalar shape: Qd[15:13], Qn[19:17], Rm[3:0], element size[21:20] (8/16/32; sign-agnostic, so the
// suffix is plain .8/.16/.32). bit16=1 and bit12=1 are fixed opcode (distinguishing it from the arithmetic
// vector-by-scalar ops, which have bit12=0). base 0xFE01_1E60.
pub const MVE_VBRSR_BASE: u32 = 0xFE01_1E60;
pub const MVE_VBRSR_MASK: u32 = 0xFFC1_1FF0;

// MVE gather/scatter, scalar base + VECTOR offset: VLDR{B,H,W,D} / VSTR{B,H,W,D} `Qd, [Rn, Qm]{, uxtw #n}`.
// base 0xEC80_0E00, gate `word & 0xEFE0_1E20 == 0xEC80_0E00` (top byte 0xEC/0xFC; the fixed bit12=0 and
// [11:9]=111 keep it disjoint from the contiguous vector load/store (bit12=1) and the scalar VFP ([11:9]=101)).
// L(load)=bit20; U(signedness of a widening load)=bit28; Rn[19:16]; Qd[15:13]; destination element size
// esize[8:7]; memory access size msize = {bit6,bit4}; Qm[3:1]; scaled(uxtw: offset << log2(msize/8))=bit0.
// esize and msize are each a 2-bit log: 00/01/10/11 = 8/16/32/64.
pub const MVE_GATHER_SCATTER_BASE: u32 = 0xEC80_0E00;
pub const MVE_GATHER_SCATTER_MASK: u32 = 0xEFE0_1E20;
pub fn mve_mem_size_log(size_bits: u8) -> u32 { (size_bits as u32 / 8).trailing_zeros() }
pub fn mve_mem_size_from_log(log: u32) -> u8 { (8u32 << (log & 0b11)) as u8 }

// MVE gather/scatter, VECTOR base + immediate: VLDRW/VLDRD/VSTRW/VSTRD `Qd, [Qn{, #imm}]{!}` (word/dword only).
// base 0xFD00_1E00, gate `word & 0xFF41_1E80 == 0xFD00_1E00` (top byte 0xFD; disjoint from Group 1 (0xEC/0xFC)
// and the complex ops by the tight [11:9]=111 frame). L(load)=bit20, W(writeback)=bit21, U(offset sign:1=add)=
// bit23, size=bit8 (.32=0/.64=1), Qn(base)[19:17], Qd[15:13], imm7[6:0] = |offset| / (4 word | 8 dword),
// magnitude range +/-508 (word) / +/-1016 (dword). Always pre-indexed (P=1 is baked into [27:24]=1101).
pub const MVE_GATHER_VBASE_BASE: u32 = 0xFD00_1E00;
pub const MVE_GATHER_VBASE_MASK: u32 = 0xFF41_1E80;

// MVE de-interleaving/interleaving load/store: VLD2x/VLD4x/VST2x/VST4x `{Qd..}, [Rn]{!}`. base 0xFC80_1E00,
// gate `word & 0xFFC0_1E1E == 0xFC80_1E00` (top byte 0xFC; bit12=1 + [11:9]=111 keep it disjoint from the
// gather/scatter (bit12=0) and complex ops). L(load)=bit20, W(writeback)=bit21, Rn[19:16], Qd(first of the
// list)[15:13], element size[8:7] (8/16/32), pass[6:5] (VLD2x: 0..1 via bit5; VLD4x: 0..3), is-VLD4=bit0.
pub const MVE_INTERLEAVE_BASE: u32 = 0xFC80_1E00;
pub const MVE_INTERLEAVE_MASK: u32 = 0xFFC0_1E1E;

// ---- Low-overhead loops (Armv8.1-M): DLS/WLS/LE/LCTP (+ tail-predicated DLSTP/WLSTP/LETP and VCTP, which
// need MVE). All in the 0xF0xx control space. DLS/DLSTP/LCTP have hw1 = 0xE001; VCTP has hw1 = 0xE801; the
// WLS/WLSTP/LE/LETP branches have hw1[15:12] = 1100 (bit12 = 0, which keeps them disjoint from B.W/BL whose
// hw1 bit12 = 1). The loop-start size field [22:20] is 0b100 for the plain (non-predicated) DLS/WLS, else the
// tail-predicate element size (000/001/010/011 = 8/16/32/64). For the branch forms Rn[19:16] = PC (15) marks
// the LE/LETP loop-end (size-field 000 = LE, 001 = LETP) versus the WLS/WLSTP loop-start (real Rn).
pub const MVE_LCTP_WORD: u32 = 0xF00F_E001;
pub const MVE_VCTP_BASE: u32 = 0xF000_E801;
pub const MVE_VCTP_MASK: u32 = 0xFFC0_FFFF;
pub const MVE_LOB_DLS_BASE: u32 = 0xF000_E001;
pub const MVE_LOB_DLS_MASK: u32 = 0xFF80_FFFF;
/// The [22:20] loop size field: `None` (plain) = 0b100, else the tail-predicate element size.
pub fn lob_size_field(tp_size: Option<u8>) -> u32 {
    match tp_size { None => 0b100, Some(8) => 0b000, Some(16) => 0b001, Some(32) => 0b010, Some(64) => 0b011, _ => 0b100 }
}
pub fn lob_size_from_field(field: u32) -> Option<Option<u8>> {
    match field & 0b111 {
        0b100 => Some(None), 0b000 => Some(Some(8)), 0b001 => Some(Some(16)), 0b010 => Some(Some(32)), 0b011 => Some(Some(64)), _ => None,
    }
}
/// The branch half-word (hw1) for a low-overhead-loop branch: imm = |offset|/2, with imm[0] at bit11 and
/// imm[10:1] at bits[10:1]; bit0 and [15:12]=1100 are fixed.
pub fn lob_branch_hw1(offset: i32) -> u32 {
    let imm = offset.unsigned_abs() / 2;
    0xC000 | ((imm & 1) << 11) | (((imm >> 1) & 0x3FF) << 1) | 1
}
/// Recover the signed PC-relative offset (target - (instr+4)) from hw1; `backward` (LE/LETP) negates it.
pub fn lob_branch_offset(hw1: u32, backward: bool) -> i32 {
    let imm = ((hw1 >> 11) & 1) | (((hw1 >> 1) & 0x3FF) << 1);
    let magnitude = (2 * imm) as i32;
    if backward { -magnitude } else { magnitude }
}

// the {B, E} size-bit pair for a VDUP element width
pub fn mve_vdup_size_bits(size: Arm32MveSize) -> (u32 /*b @22*/, u32 /*e @5*/) {
    match size {
        Arm32MveSize::I8 => (1, 0),
        Arm32MveSize::I16 => (0, 1),
        Arm32MveSize::I32 => (0, 0),
    }
}
pub fn mve_vdup_size_from_bits(b: u32, e: u32) -> Option<Arm32MveSize> {
    match (b & 1, e & 1) {
        (1, 0) => Some(Arm32MveSize::I8),
        (0, 1) => Some(Arm32MveSize::I16),
        (0, 0) => Some(Arm32MveSize::I32),
        _ => None, // {1,1} is not a valid size
    }
}

// ---- MVE shift by immediate: `Qd, Qm, #imm` ----
// These live in the 0xEF8x / 0xFF8x space (top byte 1110_1111/1111_1111, bit23=1), sharing it with the
// one-register modified-immediate (VMOV/VMVN #imm). The two are told apart by imm6 = bits[21:16]: a shift
// always has imm6 >= 8 (a size bit set), while the modified-immediate always has imm6[21:19]=000 (imm6 < 8).
//
// imm6 encodes BOTH the element size and the shift amount (exactly as NEON): the element size is the highest
// set bit of imm6 (0b001xxx -> 8, 0b01xxxx -> 16, 0b1xxxxx -> 32). For a RIGHT shift by N: imm6 = 2*esize-N
// (N in 1..=esize). For a LEFT shift by N: imm6 = esize+N (N in 0..=esize-1). So the op stores only its base
// word (with imm6 / Qd / Qm zeroed) plus its direction; the codec derives imm6 from (size, amount).
pub const MVE_SHIFT_SIGNATURE_MASK: u32 = 0xFFC0_1FF1; // clears imm6[21:16], Qd[15:13], Qm[3:1]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveShiftImmOp {
    VshrS, VshrU, VrshrS, VrshrU, Vsri, // right shifts
    VshlI, Vsli, VqshlS, VqshlU, VqshluS, // left shifts
}
impl Arm32MveShiftImmOp {
    // base word with imm6 / Qd / Qm zeroed
    pub fn base_word(self) -> u32 {
        match self {
            Self::VshrS   => 0xEF80_0050,
            Self::VshrU   => 0xFF80_0050,
            Self::VrshrS  => 0xEF80_0250,
            Self::VrshrU  => 0xFF80_0250,
            Self::Vsri    => 0xFF80_0450,
            Self::VshlI   => 0xEF80_0550,
            Self::Vsli    => 0xFF80_0550,
            Self::VqshlS  => 0xEF80_0750,
            Self::VqshlU  => 0xFF80_0750,
            Self::VqshluS => 0xFF80_0650,
        }
    }
    // true for the left shifts (imm6 = esize + amount); false for the right shifts (imm6 = 2*esize - amount)
    pub fn is_left_shift(self) -> bool {
        matches!(self, Self::VshlI | Self::Vsli | Self::VqshlS | Self::VqshlU | Self::VqshluS)
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::VshrS | Self::VshrU => "vshr",
            Self::VrshrS | Self::VrshrU => "vrshr",
            Self::Vsri => "vsri",
            Self::VshlI => "vshl",
            Self::Vsli => "vsli",
            Self::VqshlS | Self::VqshlU => "vqshl",
            Self::VqshluS => "vqshlu",
        }
    }
    // the UAL type letter, or None for the bit-insert ops (VSLI/VSRI, which carry only the element width)
    pub fn type_prefix(self) -> Option<char> {
        match self {
            Self::VshrS | Self::VrshrS | Self::VqshlS | Self::VqshluS => Some('s'),
            Self::VshrU | Self::VrshrU | Self::VqshlU => Some('u'),
            Self::VshlI => Some('i'),
            Self::Vsri | Self::Vsli => None,
        }
    }
    pub const ALL: [Self; 10] = [
        Self::VshrS, Self::VshrU, Self::VrshrS, Self::VrshrU, Self::Vsri,
        Self::VshlI, Self::Vsli, Self::VqshlS, Self::VqshlU, Self::VqshluS,
    ];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// the element width in bits for a shift element size
pub fn mve_shift_esize(size: Arm32MveSize) -> u32 {
    match size {
        Arm32MveSize::I8 => 8,
        Arm32MveSize::I16 => 16,
        Arm32MveSize::I32 => 32,
    }
}
// recover the element size from a shift's imm6 (the highest set bit selects the width); None if imm6 < 8
pub fn mve_shift_size_from_imm6(imm6: u32) -> Option<Arm32MveSize> {
    if imm6 & 0b100000 != 0 { Some(Arm32MveSize::I32) }
    else if imm6 & 0b010000 != 0 { Some(Arm32MveSize::I16) }
    else if imm6 & 0b001000 != 0 { Some(Arm32MveSize::I8) }
    else { None }
}

// ---- MVE two-register miscellaneous: `Qd, Qm` ----
// These live in the 0xFFBx space (bits[31:20] = 0xFFB), the NEON 2-reg-misc format. The element size is
// bits[19:18]; each op's remaining signature is in hw1 bits[10:6] and hw0 bit16 (the int/float marker).
pub const MVE_MISC2_SIGNATURE_MASK: u32 = 0xFFF3_1FF1; // clears size[19:18], Qd[15:13], Qm[3:1]

// the [19:18] field value for a 2-reg-misc float element size (.f16 = 01, .f32 = 10)
pub fn mve_misc2_float_size_bits(size: Arm32MveFloatSize) -> u32 {
    match size {
        Arm32MveFloatSize::F16 => 0b01,
        Arm32MveFloatSize::F32 => 0b10,
    }
}
pub fn mve_misc2_float_size_from_bits(bits: u32) -> Option<Arm32MveFloatSize> {
    match bits & 0b11 {
        0b01 => Some(Arm32MveFloatSize::F16),
        0b10 => Some(Arm32MveFloatSize::F32),
        _ => None,
    }
}

// Sized integer 2-reg-misc operations. Element size is bits[19:18] (with per-op constraints, e.g. VREV32
// only allows .8/.16). `type_prefix` is the UAL type letter, or None for VREV (which carries only the
// element width, e.g. `vrev64.8`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveMisc2Op {
    Vrev64, Vrev32, Vrev16, Vcls, Vclz, Vabs, Vneg, Vqabs, Vqneg,
}
impl Arm32MveMisc2Op {
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vrev64 => 0xFFB0_0040,
            Self::Vrev32 => 0xFFB0_00C0,
            Self::Vrev16 => 0xFFB0_0140,
            Self::Vcls   => 0xFFB0_0440,
            Self::Vclz   => 0xFFB0_04C0,
            Self::Vabs   => 0xFFB1_0340,
            Self::Vneg   => 0xFFB1_03C0,
            Self::Vqabs  => 0xFFB0_0740,
            Self::Vqneg  => 0xFFB0_07C0,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vrev64 => "vrev64",
            Self::Vrev32 => "vrev32",
            Self::Vrev16 => "vrev16",
            Self::Vcls => "vcls",
            Self::Vclz => "vclz",
            Self::Vabs => "vabs",
            Self::Vneg => "vneg",
            Self::Vqabs => "vqabs",
            Self::Vqneg => "vqneg",
        }
    }
    pub fn type_prefix(self) -> Option<char> {
        match self {
            Self::Vrev64 | Self::Vrev32 | Self::Vrev16 => None, // carries only the element width
            Self::Vclz => Some('i'),
            Self::Vcls | Self::Vabs | Self::Vneg | Self::Vqabs | Self::Vqneg => Some('s'),
        }
    }
    pub const ALL: [Self; 9] = [
        Self::Vrev64, Self::Vrev32, Self::Vrev16, Self::Vcls, Self::Vclz,
        Self::Vabs, Self::Vneg, Self::Vqabs, Self::Vqneg,
    ];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// Floating-point 2-reg-misc operations (VABS/VNEG .f16/.f32). The int/float marker is hw0 bit16 (set here),
// which keeps these disjoint from the saturating-integer VQABS/VQNEG that share the same hw1 signature.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveMisc2FloatOp {
    Vabs, Vneg,
}
impl Arm32MveMisc2FloatOp {
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vabs => 0xFFB1_0740,
            Self::Vneg => 0xFFB1_07C0,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vabs => "vabs",
            Self::Vneg => "vneg",
        }
    }
    pub const ALL: [Self; 2] = [Self::Vabs, Self::Vneg];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// VMVN (register): bitwise-NOT a whole vector. No element size; renders as `vmvn Qd, Qm`.
pub const MVE_VMVN_REG_MASK: u32 = 0xFFFF_1FF1; // the fixed opcode bits (clears only Qd / Qm)
pub const MVE_VMVN_REG_BASE: u32 = 0xFFB0_05C0;

// ---- MVE cross-lane reductions to a GPR ----
// VADDV/VADDVA/VMINV/VMAXV/VMINAV/VMAXAV (`Rd, Qm`) live in the 0xEE../0xFE.. space: Rd[15:12], Qm[3:1],
// element size[19:18], U(signedness)=bit28. VABAV (`Rd, Qn, Qm`) is separate -- see below.
pub const MVE_REDUCE_SIGNATURE_MASK: u32 = 0xFFF3_0FF1; // clears size[19:18], Rd[15:12], Qm[3:1]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveReduceOp {
    VaddvS, VaddvU, VaddvaS, VaddvaU,
    VminvS, VminvU, VmaxvS, VmaxvU,
    Vminav, Vmaxav, // absolute min/max are signed-only
}
impl Arm32MveReduceOp {
    pub fn base_word(self) -> u32 {
        match self {
            Self::VaddvS  => 0xEEF1_0F00,
            Self::VaddvU  => 0xFEF1_0F00,
            Self::VaddvaS => 0xEEF1_0F20,
            Self::VaddvaU => 0xFEF1_0F20,
            Self::VminvS  => 0xEEE2_0F80,
            Self::VminvU  => 0xFEE2_0F80,
            Self::VmaxvS  => 0xEEE2_0F00,
            Self::VmaxvU  => 0xFEE2_0F00,
            Self::Vminav  => 0xEEE0_0F80,
            Self::Vmaxav  => 0xEEE0_0F00,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::VaddvS | Self::VaddvU => "vaddv",
            Self::VaddvaS | Self::VaddvaU => "vaddva",
            Self::VminvS | Self::VminvU => "vminv",
            Self::VmaxvS | Self::VmaxvU => "vmaxv",
            Self::Vminav => "vminav",
            Self::Vmaxav => "vmaxav",
        }
    }
    pub fn type_prefix(self) -> char {
        match self {
            Self::VaddvU | Self::VaddvaU | Self::VminvU | Self::VmaxvU => 'u',
            _ => 's',
        }
    }
    pub const ALL: [Self; 10] = [
        Self::VaddvS, Self::VaddvU, Self::VaddvaS, Self::VaddvaU,
        Self::VminvS, Self::VminvU, Self::VmaxvS, Self::VmaxvU,
        Self::Vminav, Self::Vmaxav,
    ];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// MVE floating-point min/max reductions to a GPR: VMAXNMV/VMINNMV/VMAXNMAV/VMINNMAV (`Rd, Qm`). Rd[15:12],
// Qm[3:1], element size = bit28 (.f32 = 0, .f16 = 1). They sit at [19:18] = 11 (the size value the integer
// reductions reject), which keeps them disjoint from the integer reductions.
pub const MVE_FLOAT_REDUCE_SIGNATURE_MASK: u32 = 0xEFFF_0FF1; // clears size(bit28), Rd[15:12], Qm[3:1]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveFloatReduceOp {
    Vmaxnmv, Vminnmv, Vmaxnmav, Vminnmav,
}
impl Arm32MveFloatReduceOp {
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vmaxnmv  => 0xEEEE_0F00,
            Self::Vminnmv  => 0xEEEE_0F80,
            Self::Vmaxnmav => 0xEEEC_0F00,
            Self::Vminnmav => 0xEEEC_0F80,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vmaxnmv => "vmaxnmv",
            Self::Vminnmv => "vminnmv",
            Self::Vmaxnmav => "vmaxnmav",
            Self::Vminnmav => "vminnmav",
        }
    }
    pub const ALL: [Self; 4] = [Self::Vmaxnmv, Self::Vminnmv, Self::Vmaxnmav, Self::Vminnmav];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// VABAV (`Rd, Qn, Qm`): sum of absolute differences across lanes into a GPR. Rd[15:12], Qn[19:17], Qm[3:1],
// element size[21:20] (Qn occupies the [19:18] the other reductions use for size), U=bit28. Base (s8) below.
pub const MVE_VABAV_SIGNATURE_MASK: u32 = 0xEFC1_0FF1; // clears U(bit28), size[21:20], Qn[19:17], Rd[15:12], Qm[3:1]
pub const MVE_VABAV_BASE: u32 = 0xEE80_0F01;

// VMLADAV / VMLSDAV -- non-long dual multiply-(add/subtract)-accumulate cross-lane reductions into a single
// EVEN GPR: `<op>{a}{x}.<s|u><8|16|32> Rda, Qn, Qm`. All in the 0xEEFx/0xFEFx space. Fields: Rda[15:12] (even,
// so bit12 is free for X), Qn[19:17], Qm[3:1], A(accumulate)=bit5, X(exchange)=bit12, subtract(VMLSDAV)=bit0.
// The element-size encoding is IRREGULAR between the two operations (bit16 is always the .32 selector, but the
// .8 selector moves): for the ADD form bit28 is the U(signedness) flag and size uses {bit16=.32, bit8=.8};
// for the SUBTRACT form (signed-only, so bit28 is free) bit28 itself becomes the .8 selector and bit8 stays 0.
// The mask keeps the fixed [23:20]=1111 / [11:9]=111 / [7:6]=00 / bit4=0 frame, which (combined with decoding
// AFTER the 2-operand reductions) keeps VADDV/VADDVA -- the only other [23:20]=1111 words -- from being mistaken
// for a dual-MAC. X and subtract are signed-only.
pub const MVE_DUALMAC_BASE: u32 = 0xEEF0_0E00;
pub const MVE_DUALMAC_MASK: u32 = 0xEEF0_0ED0;
/// Encodes the {bit28, bit16, bit8} size/signedness contribution. `unsigned` is honoured only for the add form
/// (the subtract form is signed-only and repurposes bit28 as its .8 selector).
pub fn mve_dualmac_size_bits(subtract: bool, unsigned: bool, size: Arm32MveSize) -> u32 {
    if subtract {
        match size {
            Arm32MveSize::I8  => 1 << 28,
            Arm32MveSize::I16 => 0,
            Arm32MveSize::I32 => 1 << 16,
        }
    } else {
        let u = (unsigned as u32) << 28;
        match size {
            Arm32MveSize::I8  => u | (1 << 8),
            Arm32MveSize::I16 => u,
            Arm32MveSize::I32 => u | (1 << 16),
        }
    }
}
/// Recovers `(unsigned, size)` from a dual-MAC word given whether it is the subtract form. Returns `None` for
/// reserved bit combinations (subtract with bit8 set, or both the .8 and .32 selectors set).
pub fn mve_dualmac_decode_size(subtract: bool, word: u32) -> Option<(bool, Arm32MveSize)> {
    let bit28 = (word >> 28) & 1 == 1;
    let bit16 = (word >> 16) & 1 == 1;
    let bit8  = (word >> 8) & 1 == 1;
    if subtract {
        if bit8 { return None; } // the subtract form never sets bit8
        match (bit28, bit16) {
            (true,  false) => Some((false, Arm32MveSize::I8)),
            (false, false) => Some((false, Arm32MveSize::I16)),
            (false, true)  => Some((false, Arm32MveSize::I32)),
            _ => None,
        }
    } else {
        match (bit16, bit8) {
            (false, true)  => Some((bit28, Arm32MveSize::I8)),
            (false, false) => Some((bit28, Arm32MveSize::I16)),
            (true,  false) => Some((bit28, Arm32MveSize::I32)),
            _ => None,
        }
    }
}

// VMLALDAV / VMLSLDAV / VRMLALDAVH / VRMLSLDAVH -- LONG dual multiply-(add/subtract)-accumulate cross-lane
// reductions into a GPR PAIR: `<op>{a}{x}.<s|u><16|32> RdaLo, RdaHi, Qn, Qm`. 0xEE8x/0xFE8x space.
// Base **0xEE80_0E00**, mask **0xEF80_0ED0**. RdaLo[15:12] (EVEN) and RdaHi via [22:20]=RdaHi>>1 (ODD) are
// INDEPENDENT (not consecutive). Qn[19:17], Qm[3:1], A(accumulate)=bit5, X(exchange)=bit12, subtract=bit0.
// The op/size/signedness bits are NON-ORTHOGONAL (see mve_long_dualmac_bits):
//   bit0 = subtract; bit16 = .32 selector for the PLAIN forms only;
//   for ADD forms bit28=U and bit8=the rounding-high marker (VRMLALDAVH);
//   for SUBTRACT forms (signed-only) bit8 stays 0 and bit28 itself is the rounding-high marker (VRMLSLDAVH).
// Decoded AFTER the cross-lane reductions and VADDLV: those bit-patterns belong to them (GNU rejects the
// long-MAC register pairs that would alias a reduction, e.g. rounding-high with RdaHi=13), so reduction-first
// ordering is exact.
pub const MVE_LONG_DUALMAC_BASE: u32 = 0xEE80_0E00;
pub const MVE_LONG_DUALMAC_MASK: u32 = 0xEF80_0ED0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveLongMacOp { Vmlaldav, Vmlsldav, Vrmlaldavh, Vrmlsldavh }
impl Arm32MveLongMacOp {
    pub fn subtract(self) -> bool { matches!(self, Self::Vmlsldav | Self::Vrmlsldavh) }
    pub fn rounding_high(self) -> bool { matches!(self, Self::Vrmlaldavh | Self::Vrmlsldavh) }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vmlaldav => "vmlaldav",
            Self::Vmlsldav => "vmlsldav",
            Self::Vrmlaldavh => "vrmlaldavh",
            Self::Vrmlsldavh => "vrmlsldavh",
        }
    }
    pub fn from_flags(subtract: bool, rounding_high: bool) -> Self {
        match (subtract, rounding_high) {
            (false, false) => Self::Vmlaldav,
            (true,  false) => Self::Vmlsldav,
            (false, true)  => Self::Vrmlaldavh,
            (true,  true)  => Self::Vrmlsldavh,
        }
    }
}
/// The {bit28, bit16, bit8, bit0} opcode/size/signedness contribution for a long dual-MAC.
pub fn mve_long_dualmac_bits(op: Arm32MveLongMacOp, unsigned: bool, size: Arm32MveSize) -> u32 {
    let (subtract, rounding_high) = (op.subtract(), op.rounding_high());
    let mut bits = subtract as u32; // bit0
    bits |= if subtract { (rounding_high as u32) << 28 } else { (unsigned as u32) << 28 };
    bits |= if subtract { 0 } else { (rounding_high as u32) << 8 };
    if !rounding_high && size == Arm32MveSize::I32 { bits |= 1 << 16; }
    bits
}
/// Recovers `(op, unsigned, size)` from a long dual-MAC word. Returns `None` for reserved bit combinations.
pub fn mve_long_dualmac_decode(word: u32) -> Option<(Arm32MveLongMacOp, bool, Arm32MveSize)> {
    let subtract = word & 1 == 1;
    let bit28 = (word >> 28) & 1 == 1;
    let bit16 = (word >> 16) & 1 == 1;
    let bit8  = (word >> 8) & 1 == 1;
    let (rounding_high, unsigned) = if subtract {
        if bit8 { return None; } // the subtract form never sets bit8
        (bit28, false)           // subtract is signed-only; bit28 is the rounding-high marker
    } else {
        (bit8, bit28)            // add form: bit8 = rounding-high marker, bit28 = U
    };
    let size = if rounding_high {
        if bit16 { return None; } // rounding-high is 32-bit only and never sets the .32 size bit
        Arm32MveSize::I32
    } else if bit16 {
        Arm32MveSize::I32
    } else {
        Arm32MveSize::I16
    };
    Some((Arm32MveLongMacOp::from_flags(subtract, rounding_high), unsigned, size))
}

// ---- MVE VRINT and VCVT (float<->int), both in the 0xFFBx 2-reg-misc space (`Qd, Qm`) ----
// Float element size is bits[19:18] (.f16 = 01, .f32 = 10, via mve_misc2_float_size_*). They share the misc2
// signature mask (clears size + regs). VRINT (hw0 = 0xFFB2) and VCVT (hw0 = 0xFFB3) differ in bit 16.

// VRINT rounding mode lives in hw1 bits[9:7]; each mode is one base word (size + regs zeroed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MveVrintOp {
    Vrintn, Vrinta, Vrintz, Vrintm, Vrintp, Vrintx,
}
impl Arm32MveVrintOp {
    pub fn base_word(self) -> u32 {
        match self {
            Self::Vrintn => 0xFFB2_0440,
            Self::Vrinta => 0xFFB2_0540,
            Self::Vrintz => 0xFFB2_05C0,
            Self::Vrintm => 0xFFB2_06C0,
            Self::Vrintp => 0xFFB2_07C0,
            Self::Vrintx => 0xFFB2_04C0,
        }
    }
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Vrintn => "vrintn",
            Self::Vrinta => "vrinta",
            Self::Vrintz => "vrintz",
            Self::Vrintm => "vrintm",
            Self::Vrintp => "vrintp",
            Self::Vrintx => "vrintx",
        }
    }
    pub const ALL: [Self; 6] = [Self::Vrintn, Self::Vrinta, Self::Vrintz, Self::Vrintm, Self::Vrintp, Self::Vrintx];
    pub fn from_signature(signature: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|op| op.base_word() == signature)
    }
}

// VCVT float<->int: base (to-float, signed, size zeroed) plus bit8 = to-integer, bit7 = unsigned. The int
// width equals the float width (f16<->[su]16, f32<->[su]32), both selected by size[19:18].
pub const MVE_VCVT_FI_BASE: u32 = 0xFFB3_0640;
// the fixed-opcode mask/pattern for detecting a VCVT float<->int (clears size[19:18], regs, bit8, bit7)
pub const MVE_VCVT_FI_FIXED_MASK: u32 = 0xFFF3_1E71;
pub const MVE_VCVT_FI_FIXED_PATTERN: u32 = 0xFFB3_0640;

// VCVTA/VCVTN/VCVTP/VCVTM (float -> int with an explicit rounding mode), also in the 0xFFBx space (`Qd, Qm`).
// Rounding mode = bits[9:8] (a=00, n=01, p=10, m=11); signedness = bit7; size[19:18] (the float/int width,
// .f16<->[su]16 / .f32<->[su]32, via mve_misc2_float_size_*).
pub const MVE_VCVTR_BASE: u32 = 0xFFB3_0040;
pub const MVE_VCVTR_MASK: u32 = 0xFFF3_1C71; // clears size[19:18], rounding[9:8], signedness bit7, Qd, Qm

// Fixed-point VCVT (float <-> fixed-point with a fractional-bit count): `vcvt.<int>.f<w> Qd, Qm, #fbits` and
// the reverse `vcvt.f<w>.<int> Qd, Qm, #fbits`, in the 0xEFAx-0xEFBx / 0xFFAx-0xFFBx space (bit23=1, bit22=0).
// imm6[21:16] = 64 - fbits (so fbits in 1..=16 for .16 -> imm6 in 48..=63, and 1..=32 for .32 -> imm6 in
// 32..=63). U(signedness of the fixed side) = bit28; element width = bit9 (.16 = 0, .32 = 1); direction =
// bit8 (1 = float->fixed, 0 = fixed->float); Qd[15:13]; Qm[3:1]. The tight [11:10]=11 / [7:4]=0101 frame keeps
// it disjoint from the shifts / modified-immediate / 2-reg-misc sharing this space. (Half-precision VCVTB/T
// is NOT here -- GNU's encoding for that vector form is buggy, see the progress note.)
pub const MVE_VCVT_FIXED_BASE: u32 = 0xEF80_0C50;
pub const MVE_VCVT_FIXED_MASK: u32 = 0xEFC0_1CF1;

// MVE VCVT between half- and single-precision (vector): `vcvtb/vcvtt.f16.f32 Qd, Qm` / `.f32.f16`. Encoding
// from the Armv8-M ARM (DDI0553) -- GNU's encoding of this vector form is buggy (it scrambles Qd/Qm), so this
// is implemented from the spec, NOT the GNU oracle. Spec: 111 op 111 0 0 D 1 1 1 1 1 1 Qd T 1 1 1 0 0 0 M 0 Qm 1.
// op(bit28): 0 = F16.F32 (single->half), 1 = F32.F16 (half->single). T(bit12): 0 = B (bottom) / 1 = T (top).
// Qd[15:13], Qm[3:1]; D=M=0 (UNDEFINED otherwise); bit0 = 1 fixed.
pub const MVE_VCVT_HALF_BASE: u32 = 0xEE3F_0E01;
pub const MVE_VCVT_HALF_MASK: u32 = 0xEFFF_0FF1;

// MVE shift-right-and-narrow: VSHRN/VRSHRN (non-saturating), VQSHRN/VQRSHRN (signed-saturating), VQSHRUN/
// VQRSHRUN (unsigned-result-saturating), each with a Bottom/Top variant. Spec (DDI0553); GNU is buggy for the
// non-rounding saturating forms (it sets the rounding bit), so the saturating ones are implemented from spec.
// Frame: 111 [bit28] 111 0 1 D(0) 0 sz imm Qd T 1 1 1 1 [7:6] M(0) 0 Qm [bit0]; imm5 = sz:imm at [20:16] =
// (16 if src .16 / 32 if src .32) - shift. The ROUNDING bit is IRREGULAR: bit28 for the [7:6]=11 forms
// (VSHRN/VRSHRN, VQSHRUN/VQRSHRUN), bit0 for the [7:6]=01 forms (VQSHRN/VQRSHRN, where bit28 = U/signedness).
pub const MVE_SHIFT_NARROW_BASE: u32 = 0xEE80_0F00;
pub const MVE_SHIFT_NARROW_MASK: u32 = 0xEFE0_0F30; // fixes [23:21]=100, [11:8]=1111, [5:4]=00; clears bit28/imm5/Qd/T/[7:6]/Qm/bit0
