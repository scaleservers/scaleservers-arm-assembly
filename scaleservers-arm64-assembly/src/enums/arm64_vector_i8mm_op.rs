// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD **integer matrix multiply-accumulate** instruction (DDI0487 C7,
/// FEAT_I8MM) -- an 8x8 -> 2x2 32-bit matrix product accumulated into `Vd.4s`, with fixed `.16b` sources. The
/// signedness of the two source matrices is the only degree of freedom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorMatMulOp {
    /// `SMMLA` -- signed x signed.
    Smmla,
    /// `UMMLA` -- unsigned x unsigned.
    Ummla,
    /// `USMMLA` -- unsigned (Vn) x signed (Vm).
    Usmmla,
}

impl Arm64VectorMatMulOp {
    /// Every matrix-multiply op.
    pub const ALL: [Self; 3] = [Self::Smmla, Self::Ummla, Self::Usmmla];

    /// The fixed instruction word (operands `.4s`/`.16b` only, so `Q`/`size` are baked in): only `Rm`/`Rn`/`Rd`
    /// are added by the encoder.
    pub fn base(self) -> u32 {
        match self {
            Self::Smmla => 0x4E80_A400,
            Self::Ummla => 0x6E80_A400,
            Self::Usmmla => 0x4E80_AC00,
        }
    }

    /// The GNU/UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Smmla => "smmla",
            Self::Ummla => "ummla",
            Self::Usmmla => "usmmla",
        }
    }
}

/// The operation of an AArch64 Advanced SIMD **mixed-sign dot product, by element** instruction (DDI0487 C7,
/// FEAT_I8MM) -- a 4-way 8-bit dot product against a broadcast `Vm.4b[index]` lane, accumulated into `Vd.<2s|4s>`,
/// where the two sources carry opposite signedness. The vector (non-indexed) form exists only for `USDOT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorMixedDotOp {
    /// `USDOT` -- unsigned (Vn) x signed (Vm).
    Usdot,
    /// `SUDOT` -- signed (Vn) x unsigned (Vm). Indexed form only.
    Sudot,
}

impl Arm64VectorMixedDotOp {
    /// Both mixed-sign dot ops.
    pub const ALL: [Self; 2] = [Self::Usdot, Self::Sudot];

    /// The by-element base word: the two ops are distinguished by the `size` field (`USDOT`=10, `SUDOT`=00), which
    /// is baked in here -- the encoder adds only `Q`, the index (`L`/`M`/`H`), `Rm`, `Rn`, `Rd`.
    pub fn by_element_base(self) -> u32 {
        match self {
            Self::Usdot => 0x0F80_F000,
            Self::Sudot => 0x0F00_F000,
        }
    }

    /// The GNU/UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Usdot => "usdot",
            Self::Sudot => "sudot",
        }
    }
}
