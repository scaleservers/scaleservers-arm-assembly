// Copyright (c) Scaleservers LLC

/// An AArch64 Advanced SIMD (NEON) **vector fixed-point convert** op (DDI0487 C7) -- the shift-by-immediate-
/// encoded per-lane conversions between fixed-point and floating-point, in the same
/// `0 Q U 011110 immh immb opcode 1 Rn Rd` class as the vector shifts (opcodes `11100`/`11111`, disjoint from
/// every shift op). The arrangement supplies the element size (`.4h`/`.8h` need FEAT_FP16; there is no byte
/// element and no single-lane `.1d` form); the fractional-bit count folds via `immh:immb = 2*esize - fbits`
/// (`fbits` in `1..esize`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorFixedConvertOp {
    /// `SCVTF` -- signed fixed-point convert to floating-point (`U = 0`, opcode `11100`).
    Scvtf,
    /// `UCVTF` -- unsigned fixed-point convert to floating-point (`U = 1`, opcode `11100`).
    Ucvtf,
    /// `FCVTZS` -- floating-point convert to signed fixed-point, round toward zero (`U = 0`, opcode `11111`).
    Fcvtzs,
    /// `FCVTZU` -- floating-point convert to unsigned fixed-point, round toward zero (`U = 1`, opcode `11111`).
    Fcvtzu,
}

impl Arm64VectorFixedConvertOp {
    /// The base word with `Q = 0` and `immh:immb = 0` (`U` and opcode baked in): `0x0F00_0400 | (U<<29) |
    /// (opcode<<11)`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Scvtf => (0, 0b11100),
            Self::Ucvtf => (1, 0b11100),
            Self::Fcvtzs => (0, 0b11111),
            Self::Fcvtzu => (1, 0b11111),
        };
        0x0F00_0400 | (u << 29) | (opcode << 11)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtzu => "fcvtzu",
        }
    }

    /// Recover the op from a masked base (`word & VEC_SHIFT_IMM_MASK`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Scvtf, Self::Ucvtf, Self::Fcvtzs, Self::Fcvtzu];
}
