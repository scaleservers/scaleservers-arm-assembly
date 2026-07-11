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
