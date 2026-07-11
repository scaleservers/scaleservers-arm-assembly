// Copyright (c) Scaleservers LLC

// The rounding mode baked into the ARMv8 "anchored" floating-point rounders -- VRINT{A,N,P,M} (round to
// integral float) and VCVT{A,N,P,M} (round float to integer). Unlike VRINTR/VCVTR, these ignore the FPSCR
// rounding mode and use the fixed mode encoded in their 2-bit RM field. The suffix letters map directly:
// A = ties to Away, N = ties to even (Nearest), P = toward Plus infinity, M = toward Minus infinity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32DirectedRound {
    A, // round to nearest, ties away from zero
    N, // round to nearest, ties to even
    P, // round toward +infinity
    M, // round toward -infinity
}
impl Arm32DirectedRound {
    // the 2-bit RM field (encoding bits 17:16)
    pub fn rm_bits(self) -> u32 {
        match self {
            Self::A => 0b00,
            Self::N => 0b01,
            Self::P => 0b10,
            Self::M => 0b11,
        }
    }
    pub fn from_rm_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0b00 => Self::A,
            0b01 => Self::N,
            0b10 => Self::P,
            _ => Self::M,
        }
    }
}
