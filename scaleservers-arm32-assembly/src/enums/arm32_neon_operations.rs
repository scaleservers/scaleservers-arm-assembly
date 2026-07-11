// Copyright (c) Scaleservers LLC

// Operation taxonomies for the NEON (Advanced SIMD) "three registers of the same length" data-processing
// format `1111 001 U 0 D size Vn Vd opc N Q M op Vm`. The (U, opc, op, size) tuple selects the mnemonic.
// Three disjoint sub-families share this format; the decoder routes by opc: opc>=1100 is floating-point,
// opc==0001 & op==1 is bitwise, everything else is integer.
//   * integer   -- `size` is the element size; signed/unsigned (where it matters) is folded into the op via U.
//   * float     -- always f32 here; `size[1]` (bit 21) is a per-mnemonic sub-selector, so it is baked into op.
//   * bitwise   -- opc/op fixed (0001/1); (U, size) pick the boolean function, so both are baked into op.

// Element size for the size-parametric integer operations (the 2-bit `size` field, bits 21:20).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32NeonSize {
    I8,
    I16,
    I32,
    I64,
}
impl Arm32NeonSize {
    pub fn size_bits(self) -> u32 {
        match self {
            Self::I8 => 0b00,
            Self::I16 => 0b01,
            Self::I32 => 0b10,
            Self::I64 => 0b11,
        }
    }
    pub fn from_size_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0b00 => Self::I8,
            0b01 => Self::I16,
            0b10 => Self::I32,
            _ => Self::I64,
        }
    }
}

// Integer 3-reg-same operations. Each maps to a fixed (U, opc, op) triple. Signedness, where it changes the
// instruction, is part of the mnemonic (...S/...U) and is carried here because it sets the U bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32NeonIntegerOp {
    Vadd, Vsub, Vtst, Vceq,            // opc 1000
    Vmla, Vmls, Vmul, VmulPoly,        // opc 1001
    VqaddS, VqaddU, VhaddS, VhaddU,    // opc 0000
    VqsubS, VqsubU, VhsubS, VhsubU,    // opc 0010
    VrhaddS, VrhaddU,                  // opc 0001 (op 0)
    VabdS, VabdU, VabaS, VabaU,        // opc 0111
    VmaxS, VmaxU, VminS, VminU,        // opc 0110
    VcgeS, VcgeU, VcgtS, VcgtU,        // opc 0011
    Vpadd,                             // opc 1011 (op 1)
    VpmaxS, VpmaxU, VpminS, VpminU,    // opc 1010
    VqdmulhS, VqrdmulhS,               // opc 1011 (op 0)
}
impl Arm32NeonIntegerOp {
    // (U, opc, op)
    pub fn fields(self) -> (u32, u32, u32) {
        match self {
            Self::Vadd => (0, 0b1000, 0),
            Self::Vsub => (1, 0b1000, 0),
            Self::Vtst => (0, 0b1000, 1),
            Self::Vceq => (1, 0b1000, 1),
            Self::Vmla => (0, 0b1001, 0),
            Self::Vmls => (1, 0b1001, 0),
            Self::Vmul => (0, 0b1001, 1),
            Self::VmulPoly => (1, 0b1001, 1),
            Self::VqaddS => (0, 0b0000, 1),
            Self::VqaddU => (1, 0b0000, 1),
            Self::VhaddS => (0, 0b0000, 0),
            Self::VhaddU => (1, 0b0000, 0),
            Self::VqsubS => (0, 0b0010, 1),
            Self::VqsubU => (1, 0b0010, 1),
            Self::VhsubS => (0, 0b0010, 0),
            Self::VhsubU => (1, 0b0010, 0),
            Self::VrhaddS => (0, 0b0001, 0),
            Self::VrhaddU => (1, 0b0001, 0),
            Self::VabdS => (0, 0b0111, 0),
            Self::VabdU => (1, 0b0111, 0),
            Self::VabaS => (0, 0b0111, 1),
            Self::VabaU => (1, 0b0111, 1),
            Self::VmaxS => (0, 0b0110, 0),
            Self::VmaxU => (1, 0b0110, 0),
            Self::VminS => (0, 0b0110, 1),
            Self::VminU => (1, 0b0110, 1),
            Self::VcgeS => (0, 0b0011, 1),
            Self::VcgeU => (1, 0b0011, 1),
            Self::VcgtS => (0, 0b0011, 0),
            Self::VcgtU => (1, 0b0011, 0),
            Self::Vpadd => (0, 0b1011, 1),
            Self::VpmaxS => (0, 0b1010, 0),
            Self::VpmaxU => (1, 0b1010, 0),
            Self::VpminS => (0, 0b1010, 1),
            Self::VpminU => (1, 0b1010, 1),
            Self::VqdmulhS => (0, 0b1011, 0),
            Self::VqrdmulhS => (1, 0b1011, 0),
        }
    }
    pub fn from_fields(u: u32, opc: u32, op: u32) -> Option<Self> {
        Some(match (opc, op, u) {
            (0b1000, 0, 0) => Self::Vadd,
            (0b1000, 0, 1) => Self::Vsub,
            (0b1000, 1, 0) => Self::Vtst,
            (0b1000, 1, 1) => Self::Vceq,
            (0b1001, 0, 0) => Self::Vmla,
            (0b1001, 0, 1) => Self::Vmls,
            (0b1001, 1, 0) => Self::Vmul,
            (0b1001, 1, 1) => Self::VmulPoly,
            (0b0000, 1, 0) => Self::VqaddS,
            (0b0000, 1, 1) => Self::VqaddU,
            (0b0000, 0, 0) => Self::VhaddS,
            (0b0000, 0, 1) => Self::VhaddU,
            (0b0010, 1, 0) => Self::VqsubS,
            (0b0010, 1, 1) => Self::VqsubU,
            (0b0010, 0, 0) => Self::VhsubS,
            (0b0010, 0, 1) => Self::VhsubU,
            (0b0001, 0, 0) => Self::VrhaddS,
            (0b0001, 0, 1) => Self::VrhaddU,
            (0b0111, 0, 0) => Self::VabdS,
            (0b0111, 0, 1) => Self::VabdU,
            (0b0111, 1, 0) => Self::VabaS,
            (0b0111, 1, 1) => Self::VabaU,
            (0b0110, 0, 0) => Self::VmaxS,
            (0b0110, 0, 1) => Self::VmaxU,
            (0b0110, 1, 0) => Self::VminS,
            (0b0110, 1, 1) => Self::VminU,
            (0b0011, 1, 0) => Self::VcgeS,
            (0b0011, 1, 1) => Self::VcgeU,
            (0b0011, 0, 0) => Self::VcgtS,
            (0b0011, 0, 1) => Self::VcgtU,
            (0b1011, 1, 0) => Self::Vpadd,
            (0b1010, 0, 0) => Self::VpmaxS,
            (0b1010, 0, 1) => Self::VpmaxU,
            (0b1010, 1, 0) => Self::VpminS,
            (0b1010, 1, 1) => Self::VpminU,
            (0b1011, 0, 0) => Self::VqdmulhS,
            (0b1011, 0, 1) => Self::VqrdmulhS,
            _ => return None,
        })
    }
}

// Floating-point (f32) 3-reg-same operations. Each maps to a fixed (U, opc, op, size) tuple; only size[1]
// (bit 21) is ever set, as a sub-selector, so it is part of the op identity rather than an operand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32NeonFloatOp {
    Vadd, Vsub, Vmul, Vmla, Vmls, Vabd, Vpadd,          // opc 1100/1101
    Vceq, Vcge, Vcgt,                                   // opc 1110
    Vmax, Vmin, Vpmax, Vpmin, Vrecps, Vrsqrts,          // opc 1111
    Vfma, Vfms,                                         // opc 1100
}
impl Arm32NeonFloatOp {
    // (U, opc, op, size)
    pub fn fields(self) -> (u32, u32, u32, u32) {
        match self {
            Self::Vadd => (0, 0b1101, 0, 0b00),
            Self::Vsub => (0, 0b1101, 0, 0b10),
            Self::Vmla => (0, 0b1101, 1, 0b00),
            Self::Vmls => (0, 0b1101, 1, 0b10),
            Self::Vmul => (1, 0b1101, 1, 0b00),
            Self::Vabd => (1, 0b1101, 0, 0b10),
            Self::Vpadd => (1, 0b1101, 0, 0b00),
            Self::Vceq => (0, 0b1110, 0, 0b00),
            Self::Vcge => (1, 0b1110, 0, 0b00),
            Self::Vcgt => (1, 0b1110, 0, 0b10),
            Self::Vmax => (0, 0b1111, 0, 0b00),
            Self::Vmin => (0, 0b1111, 0, 0b10),
            Self::Vpmax => (1, 0b1111, 0, 0b00),
            Self::Vpmin => (1, 0b1111, 0, 0b10),
            Self::Vrecps => (0, 0b1111, 1, 0b00),
            Self::Vrsqrts => (0, 0b1111, 1, 0b10),
            Self::Vfma => (0, 0b1100, 1, 0b00),
            Self::Vfms => (0, 0b1100, 1, 0b10),
        }
    }
    pub fn from_fields(u: u32, opc: u32, op: u32, size: u32) -> Option<Self> {
        let bit21 = (size >> 1) & 1;
        Some(match (opc, u, op, bit21) {
            (0b1100, 0, 1, 0) => Self::Vfma,
            (0b1100, 0, 1, 1) => Self::Vfms,
            (0b1101, 0, 0, 0) => Self::Vadd,
            (0b1101, 0, 0, 1) => Self::Vsub,
            (0b1101, 0, 1, 0) => Self::Vmla,
            (0b1101, 0, 1, 1) => Self::Vmls,
            (0b1101, 1, 1, 0) => Self::Vmul,
            (0b1101, 1, 0, 0) => Self::Vpadd,
            (0b1101, 1, 0, 1) => Self::Vabd,
            (0b1110, 0, 0, 0) => Self::Vceq,
            (0b1110, 1, 0, 0) => Self::Vcge,
            (0b1110, 1, 0, 1) => Self::Vcgt,
            (0b1111, 0, 0, 0) => Self::Vmax,
            (0b1111, 0, 0, 1) => Self::Vmin,
            (0b1111, 1, 0, 0) => Self::Vpmax,
            (0b1111, 1, 0, 1) => Self::Vpmin,
            (0b1111, 0, 1, 0) => Self::Vrecps,
            (0b1111, 0, 1, 1) => Self::Vrsqrts,
            _ => return None,
        })
    }
}

// Bitwise 3-reg-same operations (opc 0001, op 1). The boolean function is selected by (U, size).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32NeonBitwiseOp {
    Vand, Vbic, Vorr, Vorn, // U 0, size 00/01/10/11
    Veor, Vbsl, Vbit, Vbif, // U 1, size 00/01/10/11
}
impl Arm32NeonBitwiseOp {
    // (U, size)
    pub fn fields(self) -> (u32, u32) {
        match self {
            Self::Vand => (0, 0b00),
            Self::Vbic => (0, 0b01),
            Self::Vorr => (0, 0b10),
            Self::Vorn => (0, 0b11),
            Self::Veor => (1, 0b00),
            Self::Vbsl => (1, 0b01),
            Self::Vbit => (1, 0b10),
            Self::Vbif => (1, 0b11),
        }
    }
    pub fn from_fields(u: u32, size: u32) -> Self {
        match (u, size & 0b11) {
            (0, 0b00) => Self::Vand,
            (0, 0b01) => Self::Vbic,
            (0, 0b10) => Self::Vorr,
            (0, 0b11) => Self::Vorn,
            (_, 0b00) => Self::Veor,
            (_, 0b01) => Self::Vbsl,
            (_, 0b10) => Self::Vbit,
            (_, _) => Self::Vbif,
        }
    }
}

// ---- NEON "two registers, miscellaneous" format: 1111 0011 1 D 11 size a Vd opc2 Q M 0 Vm ----
// `a` = bits[17:16] picks one of four sub-groups; `opc2` = bits[11:7] picks the op within it; bit6 = Q for
// the same-width ops (a separate operand bit), but for the narrowing ops it is part of the op selector.
// Operations split by register shape: same-width D->D / Q->Q (element-sized OR fixed-size), narrowing
// Q->D, and widening D->Q (VSHLL by element size). Element-sized and fixed-size are separate enums so the
// fixed ones don't carry a meaningless size operand.

// Same-width 2-reg-misc ops whose `size` field is the operand element size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32NeonMisc2SizedOp {
    Vrev64, Vrev32, Vrev16,                  // a=00
    VpaddlS, VpaddlU, VclsS, VclzI,          // a=00
    VpadalS, VpadalU, VqabsS, VqnegS,        // a=00
    VcgtZeroS, VcgeZeroS, VceqZeroI, VcleZeroS, VcltZeroS, VabsS, VnegS, // a=01 (integer)
    Vtrn, Vuzp, Vzip,                        // a=10
}
impl Arm32NeonMisc2SizedOp {
    // (a, opc2)
    pub fn fields(self) -> (u32, u32) {
        match self {
            Self::Vrev64 => (0b00, 0b00000),
            Self::Vrev32 => (0b00, 0b00001),
            Self::Vrev16 => (0b00, 0b00010),
            Self::VpaddlS => (0b00, 0b00100),
            Self::VpaddlU => (0b00, 0b00101),
            Self::VclsS => (0b00, 0b01000),
            Self::VclzI => (0b00, 0b01001),
            Self::VpadalS => (0b00, 0b01100),
            Self::VpadalU => (0b00, 0b01101),
            Self::VqabsS => (0b00, 0b01110),
            Self::VqnegS => (0b00, 0b01111),
            Self::VcgtZeroS => (0b01, 0b00000),
            Self::VcgeZeroS => (0b01, 0b00001),
            Self::VceqZeroI => (0b01, 0b00010),
            Self::VcleZeroS => (0b01, 0b00011),
            Self::VcltZeroS => (0b01, 0b00100),
            Self::VabsS => (0b01, 0b00110),
            Self::VnegS => (0b01, 0b00111),
            Self::Vtrn => (0b10, 0b00001),
            Self::Vuzp => (0b10, 0b00010),
            Self::Vzip => (0b10, 0b00011),
        }
    }
    pub fn from_fields(a: u32, opc2: u32) -> Option<Self> {
        Some(match (a, opc2) {
            (0b00, 0b00000) => Self::Vrev64,
            (0b00, 0b00001) => Self::Vrev32,
            (0b00, 0b00010) => Self::Vrev16,
            (0b00, 0b00100) => Self::VpaddlS,
            (0b00, 0b00101) => Self::VpaddlU,
            (0b00, 0b01000) => Self::VclsS,
            (0b00, 0b01001) => Self::VclzI,
            (0b00, 0b01100) => Self::VpadalS,
            (0b00, 0b01101) => Self::VpadalU,
            (0b00, 0b01110) => Self::VqabsS,
            (0b00, 0b01111) => Self::VqnegS,
            (0b01, 0b00000) => Self::VcgtZeroS,
            (0b01, 0b00001) => Self::VcgeZeroS,
            (0b01, 0b00010) => Self::VceqZeroI,
            (0b01, 0b00011) => Self::VcleZeroS,
            (0b01, 0b00100) => Self::VcltZeroS,
            (0b01, 0b00110) => Self::VabsS,
            (0b01, 0b00111) => Self::VnegS,
            (0b10, 0b00001) => Self::Vtrn,
            (0b10, 0b00010) => Self::Vuzp,
            (0b10, 0b00011) => Self::Vzip,
            _ => return None,
        })
    }
}

// Same-width 2-reg-misc ops with a fixed `size` field (no element-size operand): bitwise/permute corners,
// the floating-point compares-with-zero and abs/neg, the v8 round-to-integral (VRINT*), reciprocal
// estimates, and the vector float<->int / anchored conversions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32NeonMisc2FixedOp {
    Vmvn, Vswp, Vcnt,                                            // a=00 / a=10 (size 00)
    VcgtZeroF, VcgeZeroF, VceqZeroF, VcleZeroF, VcltZeroF, VabsF, VnegF, // a=01 (size 10)
    VrintN, VrintX, VrintA, VrintZ, VrintM, VrintP,             // a=10 (size 10)
    VrecpeU, VrsqrteU, VrecpeF, VrsqrteF,                       // a=11 (size 10)
    VcvtF32FromS32, VcvtF32FromU32, VcvtS32FromF32, VcvtU32FromF32, // a=11 (size 10)
    VcvtaS, VcvtaU, VcvtnS, VcvtnU, VcvtpS, VcvtpU, VcvtmS, VcvtmU,  // a=11 (size 10)
}
impl Arm32NeonMisc2FixedOp {
    // (a, opc2, size)
    pub fn fields(self) -> (u32, u32, u32) {
        match self {
            Self::Vcnt => (0b00, 0b01010, 0b00),
            Self::Vmvn => (0b00, 0b01011, 0b00),
            Self::Vswp => (0b10, 0b00000, 0b00),
            Self::VcgtZeroF => (0b01, 0b01000, 0b10),
            Self::VcgeZeroF => (0b01, 0b01001, 0b10),
            Self::VceqZeroF => (0b01, 0b01010, 0b10),
            Self::VcleZeroF => (0b01, 0b01011, 0b10),
            Self::VcltZeroF => (0b01, 0b01100, 0b10),
            Self::VabsF => (0b01, 0b01110, 0b10),
            Self::VnegF => (0b01, 0b01111, 0b10),
            Self::VrintN => (0b10, 0b01000, 0b10),
            Self::VrintX => (0b10, 0b01001, 0b10),
            Self::VrintA => (0b10, 0b01010, 0b10),
            Self::VrintZ => (0b10, 0b01011, 0b10),
            Self::VrintM => (0b10, 0b01101, 0b10),
            Self::VrintP => (0b10, 0b01111, 0b10),
            Self::VrecpeU => (0b11, 0b01000, 0b10),
            Self::VrsqrteU => (0b11, 0b01001, 0b10),
            Self::VrecpeF => (0b11, 0b01010, 0b10),
            Self::VrsqrteF => (0b11, 0b01011, 0b10),
            Self::VcvtF32FromS32 => (0b11, 0b01100, 0b10),
            Self::VcvtF32FromU32 => (0b11, 0b01101, 0b10),
            Self::VcvtS32FromF32 => (0b11, 0b01110, 0b10),
            Self::VcvtU32FromF32 => (0b11, 0b01111, 0b10),
            Self::VcvtaS => (0b11, 0b00000, 0b10),
            Self::VcvtaU => (0b11, 0b00001, 0b10),
            Self::VcvtnS => (0b11, 0b00010, 0b10),
            Self::VcvtnU => (0b11, 0b00011, 0b10),
            Self::VcvtpS => (0b11, 0b00100, 0b10),
            Self::VcvtpU => (0b11, 0b00101, 0b10),
            Self::VcvtmS => (0b11, 0b00110, 0b10),
            Self::VcvtmU => (0b11, 0b00111, 0b10),
        }
    }
    // the v8-only rounding/anchored-convert members (VRINT{N,X,A,Z,M,P}, VCVT{A,N,P,M}{S,U}) need ARMv8;
    // every other member is available from ARMv7 with the Advanced SIMD extension.
    pub fn is_armv8(self) -> bool {
        matches!(self,
            Self::VrintN | Self::VrintX | Self::VrintA | Self::VrintZ | Self::VrintM | Self::VrintP
            | Self::VcvtaS | Self::VcvtaU | Self::VcvtnS | Self::VcvtnU
            | Self::VcvtpS | Self::VcvtpU | Self::VcvtmS | Self::VcvtmU)
    }
    pub fn from_fields(a: u32, opc2: u32) -> Option<Self> {
        Some(match (a, opc2) {
            (0b00, 0b01010) => Self::Vcnt,
            (0b00, 0b01011) => Self::Vmvn,
            (0b10, 0b00000) => Self::Vswp,
            (0b01, 0b01000) => Self::VcgtZeroF,
            (0b01, 0b01001) => Self::VcgeZeroF,
            (0b01, 0b01010) => Self::VceqZeroF,
            (0b01, 0b01011) => Self::VcleZeroF,
            (0b01, 0b01100) => Self::VcltZeroF,
            (0b01, 0b01110) => Self::VabsF,
            (0b01, 0b01111) => Self::VnegF,
            (0b10, 0b01000) => Self::VrintN,
            (0b10, 0b01001) => Self::VrintX,
            (0b10, 0b01010) => Self::VrintA,
            (0b10, 0b01011) => Self::VrintZ,
            (0b10, 0b01101) => Self::VrintM,
            (0b10, 0b01111) => Self::VrintP,
            (0b11, 0b01000) => Self::VrecpeU,
            (0b11, 0b01001) => Self::VrsqrteU,
            (0b11, 0b01010) => Self::VrecpeF,
            (0b11, 0b01011) => Self::VrsqrteF,
            (0b11, 0b01100) => Self::VcvtF32FromS32,
            (0b11, 0b01101) => Self::VcvtF32FromU32,
            (0b11, 0b01110) => Self::VcvtS32FromF32,
            (0b11, 0b01111) => Self::VcvtU32FromF32,
            (0b11, 0b00000) => Self::VcvtaS,
            (0b11, 0b00001) => Self::VcvtaU,
            (0b11, 0b00010) => Self::VcvtnS,
            (0b11, 0b00011) => Self::VcvtnU,
            (0b11, 0b00100) => Self::VcvtpS,
            (0b11, 0b00101) => Self::VcvtpU,
            (0b11, 0b00110) => Self::VcvtmS,
            (0b11, 0b00111) => Self::VcvtmU,
            _ => return None,
        })
    }
}

// Narrowing 2-reg-misc ops (a=10): Qm -> Dd. opc2 + bit6 together select the op; the `size` field names the
// SOURCE element size as 16/32/64 (field 00/01/10 = one less than the Arm32NeonSize bits).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32NeonNarrowOp {
    Vmovn, Vqmovun, VqmovnS, VqmovnU,
}
impl Arm32NeonNarrowOp {
    // (opc2, bit6)
    pub fn fields(self) -> (u32, u32) {
        match self {
            Self::Vmovn => (0b00100, 0),
            Self::Vqmovun => (0b00100, 1),
            Self::VqmovnS => (0b00101, 0),
            Self::VqmovnU => (0b00101, 1),
        }
    }
    pub fn from_fields(opc2: u32, bit6: u32) -> Option<Self> {
        Some(match (opc2, bit6) {
            (0b00100, 0) => Self::Vmovn,
            (0b00100, 1) => Self::Vqmovun,
            (0b00101, 0) => Self::VqmovnS,
            (0b00101, 1) => Self::VqmovnU,
            _ => return None,
        })
    }
}
