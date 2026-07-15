// Copyright (c) Scaleservers LLC

// The VFP data-processing operations, factored out of `ArmT32Instruction` so that each comes in a single
// pair of variants (single / double precision) rather than one per operation x precision. The opcode of a
// 3-operand FP instruction lives in bits [23], [21:20] and the op bit [6]; the 2-operand "other" group is
// selected by [19:16] and bit [7]. See the encode/decode in armt32_instruction.rs.

// Three-operand FP data-processing (`Vd, Vn, Vm`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmT32FpDataOperation3 {
    Vmla,
    Vmls,
    Vnmla,
    Vnmls,
    Vmul,
    Vnmul,
    Vadd,
    Vsub,
    Vdiv,
    Vfnma,
    Vfnms,
    Vfma,
    Vfms,
}
impl ArmT32FpDataOperation3 {
    // the opcode bits to OR into 0xEE000A00: ([23] << 23) | ([21:20] << 20) | (op[6] << 6).
    pub fn opcode_bits(&self) -> u32 {
        let (top, middle, op) = match self {
            Self::Vmla => (0, 0b00, 0),
            Self::Vmls => (0, 0b00, 1),
            Self::Vnmls => (0, 0b01, 0),
            Self::Vnmla => (0, 0b01, 1),
            Self::Vmul => (0, 0b10, 0),
            Self::Vnmul => (0, 0b10, 1),
            Self::Vadd => (0, 0b11, 0),
            Self::Vsub => (0, 0b11, 1),
            Self::Vdiv => (1, 0b00, 0),
            Self::Vfnms => (1, 0b01, 0),
            Self::Vfnma => (1, 0b01, 1),
            Self::Vfma => (1, 0b10, 0),
            Self::Vfms => (1, 0b10, 1),
        };
        (top << 23) | (middle << 20) | (op << 6)
    }

    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::Vmla => "vmla",
            Self::Vmls => "vmls",
            Self::Vnmla => "vnmla",
            Self::Vnmls => "vnmls",
            Self::Vmul => "vmul",
            Self::Vnmul => "vnmul",
            Self::Vadd => "vadd",
            Self::Vsub => "vsub",
            Self::Vdiv => "vdiv",
            Self::Vfnma => "vfnma",
            Self::Vfnms => "vfnms",
            Self::Vfma => "vfma",
            Self::Vfms => "vfms",
        }
    }

    // recover the operation from the (top=[23], middle=[21:20], op=[6]) bits.
    pub fn from_bits(top: u32, middle: u32, op: u32) -> Option<Self> {
        // mask each sub-field to its width (top=[23]=1, middle=[21:20]=2, op=[6]=1 bits) so stray high bits
        // are ignored; the fallback then catches only genuinely-unallocated combinations, not out-of-range input.
        Some(match (top & 1, middle & 0b11, op & 1) {
            (0, 0b00, 0) => Self::Vmla,
            (0, 0b00, 1) => Self::Vmls,
            (0, 0b01, 0) => Self::Vnmls,
            (0, 0b01, 1) => Self::Vnmla,
            (0, 0b10, 0) => Self::Vmul,
            (0, 0b10, 1) => Self::Vnmul,
            (0, 0b11, 0) => Self::Vadd,
            (0, 0b11, 1) => Self::Vsub,
            (1, 0b00, 0) => Self::Vdiv,
            (1, 0b01, 0) => Self::Vfnms,
            (1, 0b01, 1) => Self::Vfnma,
            (1, 0b10, 0) => Self::Vfma,
            (1, 0b10, 1) => Self::Vfms,
            _ => return None,
        })
    }
}

// Two-operand FP "other" data-processing (`Vd, Vm`): register move, absolute, negate, square root.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmT32FpDataOperation2 {
    Vmov,
    Vabs,
    Vneg,
    Vsqrt,
}
impl ArmT32FpDataOperation2 {
    // the single-precision base word (the opcode with all register fields zero).
    pub fn base(&self) -> u32 {
        match self {
            Self::Vmov => 0xEEB0_0A40,
            Self::Vabs => 0xEEB0_0AC0,
            Self::Vneg => 0xEEB1_0A40,
            Self::Vsqrt => 0xEEB1_0AC0,
        }
    }

    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::Vmov => "vmov",
            Self::Vabs => "vabs",
            Self::Vneg => "vneg",
            Self::Vsqrt => "vsqrt",
        }
    }

    // recover from opc2=[19:16] and op=[7].
    pub fn from_bits(opc2: u32, op: u32) -> Option<Self> {
        // mask each sub-field to its width (opc2=[19:16]=4, op=[7]=1 bits) so stray high bits are ignored.
        Some(match (opc2 & 0b1111, op & 1) {
            (0b0000, 0) => Self::Vmov,
            (0b0000, 1) => Self::Vabs,
            (0b0001, 0) => Self::Vneg,
            (0b0001, 1) => Self::Vsqrt,
            _ => return None,
        })
    }
}
