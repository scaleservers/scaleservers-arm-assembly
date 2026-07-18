// Copyright (c) Scaleservers LLC

use super::Arm64VectorArrangement;

/// The operation of an AArch64 Advanced SIMD (NEON) **vector x indexed element** instruction (DDI0487 C7) -- the
/// `0 Q U 01111 size L M Rm opcode H 0 Rn Rd` class that multiplies/accumulates every lane of `Vn` against a
/// single broadcast lane `Vm.<ts>[index]`. This enum is the same-element-width ops (result width = source width);
/// the index + `Vm` register are folded into the `H:L:M` bits per element size (`.h`: `Vm` is `v0`-`v15`, index
/// `H:L:M`; `.s`: index `H:L`, `M` is `Vm` bit 4; `.d`: index `H`). The integer ops take `.4h`/`.8h`/`.2s`/`.4s`;
/// the FP ops take `.2s`/`.4s`/`.2d` (FP16 by-element is modeled separately in `Arm64VectorFp16ByElementOp`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorByElementOp {
    /// `MUL` by element (`U = 0`, opcode `1000`).
    Mul,
    /// `MLA` by element (`U = 1`, opcode `0000`).
    Mla,
    /// `MLS` by element (`U = 1`, opcode `0100`).
    Mls,
    /// `SQDMULH` by element (`U = 0`, opcode `1100`).
    Sqdmulh,
    /// `SQRDMULH` by element (`U = 0`, opcode `1101`).
    Sqrdmulh,
    /// `FMUL` by element (`U = 0`, opcode `1001`).
    Fmul,
    /// `FMLA` by element (`U = 0`, opcode `0001`).
    Fmla,
    /// `FMLS` by element (`U = 0`, opcode `0101`).
    Fmls,
    /// `FMULX` by element (`U = 1`, opcode `1001`).
    Fmulx,
}

impl Arm64VectorByElementOp {
    /// The base word with `Q = 0`, `size = 0`, and the index bits (`L`/`M`/`H`) and `Rm` cleared (`U` + opcode
    /// baked in). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Mul => 0x0F00_8000,
            Self::Mla => 0x2F00_0000,
            Self::Mls => 0x2F00_4000,
            Self::Sqdmulh => 0x0F00_C000,
            Self::Sqrdmulh => 0x0F00_D000,
            Self::Fmul => 0x0F00_9000,
            Self::Fmla => 0x0F00_1000,
            Self::Fmls => 0x0F00_5000,
            Self::Fmulx => 0x2F00_9000,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Mul => "mul",
            Self::Mla => "mla",
            Self::Mls => "mls",
            Self::Sqdmulh => "sqdmulh",
            Self::Sqrdmulh => "sqrdmulh",
            Self::Fmul => "fmul",
            Self::Fmla => "fmla",
            Self::Fmls => "fmls",
            Self::Fmulx => "fmulx",
        }
    }

    /// Whether this is a floating-point by-element op (its element width / valid arrangements differ from the
    /// integer ops).
    pub fn is_fp(self) -> bool {
        matches!(self, Self::Fmul | Self::Fmla | Self::Fmls | Self::Fmulx)
    }

    /// Whether `arr` is a valid arrangement: the integer ops take `.4h`/`.8h`/`.2s`/`.4s`; the FP ops take
    /// `.2s`/`.4s`/`.2d`.
    pub fn allows_arrangement(self, arr: Arm64VectorArrangement) -> bool {
        use Arm64VectorArrangement::{D2, H4, H8, S2, S4};
        if self.is_fp() {
            matches!(arr, S2 | S4 | D2)
        } else {
            matches!(arr, H4 | H8 | S2 | S4)
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 9] = [
        Self::Mul,
        Self::Mla,
        Self::Mls,
        Self::Sqdmulh,
        Self::Sqrdmulh,
        Self::Fmul,
        Self::Fmla,
        Self::Fmls,
        Self::Fmulx,
    ];
}
