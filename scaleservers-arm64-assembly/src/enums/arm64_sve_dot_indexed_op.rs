// Copyright (c) Scaleservers LLC

use crate::enums::Arm64VectorElement;

/// The SVE 4-way dot-product-by-indexed-element operations that accumulate into a `.s` lane and share the
/// `Zda.S, Zn.<Tn>, Zm.<Tn>[index]` shape with a 2-bit index (`Zm` in `Z0..Z7`): the BFloat16 `BFDOT`
/// (`.h` sources, FEAT_BF16) and the mixed-sign 8-bit `USDOT`/`SUDOT` (`.b` sources, FEAT_I8MM). The signed
/// `SDOT`/`UDOT` indexed dots are modelled separately ([`crate::Arm64Instruction::SveDotProductIndexed`], which
/// also covers the `.d` form).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SveDotIndexedOp {
    /// `BFDOT` -- BFloat16 dot product into `.s` (`.h` sources, FEAT_BF16).
    Bfdot,
    /// `USDOT` -- unsigned-by-signed 8-bit dot product into `.s` (`.b` sources, FEAT_I8MM).
    Usdot,
    /// `SUDOT` -- signed-by-unsigned 8-bit dot product into `.s` (`.b` sources, FEAT_I8MM).
    Sudot,
}

impl Arm64SveDotIndexedOp {
    /// The 32-bit encoding base (opcode in place; `index`/`Zm`/`Zn`/`Zda` zero). GNU+LLVM verified.
    pub const fn base(self) -> u32 {
        match self {
            Self::Bfdot => 0x6460_4000,
            Self::Usdot => 0x44A0_1800,
            Self::Sudot => 0x44A0_1C00,
        }
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Bfdot => "bfdot",
            Self::Usdot => "usdot",
            Self::Sudot => "sudot",
        }
    }

    /// The source element type: `.h` for the BFloat16 dot, `.b` for the 8-bit dots.
    pub const fn source_element(self) -> Arm64VectorElement {
        match self {
            Self::Bfdot => Arm64VectorElement::H,
            Self::Usdot | Self::Sudot => Arm64VectorElement::B,
        }
    }

    /// Every op, for exhaustive round-trip testing.
    pub const ALL: [Self; 3] = [Self::Bfdot, Self::Usdot, Self::Sudot];
}
