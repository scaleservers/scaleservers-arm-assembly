// Copyright (c) Scaleservers LLC

// The addressing mode of an A32 load/store-multiple (LDM/STM): how the block of registers is laid out
// relative to the base register Rn, encoded as the (P, U) pair. PUSH is STMDB sp! and POP is LDMIA sp!.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32BlockAddressMode {
    IncrementAfter, // IA / IncrementAfter  (P=0, U=1) -- the bare LDM/STM and the POP side
    IncrementBefore, // IB / IncrementBefore (P=1, U=1)
    DecrementAfter, // DA / DecrementAfter  (P=0, U=0)
    DecrementBefore, // DB / DecrementBefore (P=1, U=0) -- the PUSH side
}
impl Arm32BlockAddressMode {
    pub fn p_u_bits(self) -> (u32, u32) {
        match self {
            Self::IncrementAfter => (0, 1),
            Self::IncrementBefore => (1, 1),
            Self::DecrementAfter => (0, 0),
            Self::DecrementBefore => (1, 0),
        }
    }
    pub fn from_p_u_bits(p: u32, u: u32) -> Self {
        // p and u are 1-bit fields; mask so stray high bits are ignored and the fallback is exactly (1, 0).
        match (p & 1, u & 1) {
            (0, 1) => Self::IncrementAfter,
            (1, 1) => Self::IncrementBefore,
            (0, 0) => Self::DecrementAfter,
            _ => Self::DecrementBefore, // (1, 0) -- the only combo `& 1` leaves
        }
    }
}
