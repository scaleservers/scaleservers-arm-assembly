// Copyright (c) Scaleservers LLC

use crate::enums::Arm64VectorElement;

/// The SVE/SVE2 matrix-multiply-accumulate and BFloat16 dot-product operations that share the unpredicated
/// `Zda, Zn, Zm` three-register shape (no governing predicate, no immediate): the 8-bit integer matrix
/// multiplies `SMMLA`/`USMMLA`/`UMMLA` (FEAT_I8MM), the floating-point matrix multiplies `FMMLA` single and
/// double (FEAT_F32MM / FEAT_F64MM) and `BFMMLA` (FEAT_BF16), and the BFloat16 dot product `BFDOT` (FEAT_BF16).
/// Each carries its full 32-bit encoding base with the size/sign discriminant already in place; the encoder
/// ORs in `Zm`/`Zn`/`Zda`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SveMatmulOp {
    /// Signed 8-bit integer matrix multiply-accumulate to 32-bit (`SMMLA Zda.S, Zn.B, Zm.B`).
    Smmla,
    /// Unsigned-by-signed 8-bit integer matrix multiply-accumulate to 32-bit (`USMMLA Zda.S, Zn.B, Zm.B`).
    Usmmla,
    /// Unsigned 8-bit integer matrix multiply-accumulate to 32-bit (`UMMLA Zda.S, Zn.B, Zm.B`).
    Ummla,
    /// Single-precision floating-point matrix multiply-accumulate (`FMMLA Zda.S, Zn.S, Zm.S`, FEAT_F32MM).
    FmmlaS,
    /// Double-precision floating-point matrix multiply-accumulate (`FMMLA Zda.D, Zn.D, Zm.D`, FEAT_F64MM).
    FmmlaD,
    /// BFloat16 matrix multiply-accumulate to single-precision (`BFMMLA Zda.S, Zn.H, Zm.H`, FEAT_BF16).
    Bfmmla,
    /// BFloat16 dot product to single-precision (`BFDOT Zda.S, Zn.H, Zm.H`, FEAT_BF16).
    Bfdot,
}

impl Arm64SveMatmulOp {
    /// The full 32-bit encoding base: top byte, size/sign discriminant and fixed opcode bits in place, with
    /// `Zm`/`Zn`/`Zda` zero. GNU+LLVM verified.
    pub const fn word(self) -> u32 {
        match self {
            Self::Smmla => 0x4500_9800,
            Self::Usmmla => 0x4580_9800,
            Self::Ummla => 0x45C0_9800,
            Self::FmmlaS => 0x64A0_E400,
            Self::FmmlaD => 0x64E0_E400,
            Self::Bfmmla => 0x6460_E400,
            Self::Bfdot => 0x6460_8000,
        }
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Smmla => "smmla",
            Self::Usmmla => "usmmla",
            Self::Ummla => "ummla",
            Self::FmmlaS | Self::FmmlaD => "fmmla",
            Self::Bfmmla => "bfmmla",
            Self::Bfdot => "bfdot",
        }
    }

    /// The destination (accumulator) element type: `.s` for every form except double-precision `FMMLA`.
    pub const fn dest_element(self) -> Arm64VectorElement {
        match self {
            Self::FmmlaD => Arm64VectorElement::D,
            _ => Arm64VectorElement::S,
        }
    }

    /// The source element type: `.b` for the 8-bit integer matmuls, `.h` for the BFloat16 forms, and the same
    /// as the destination for `FMMLA`.
    pub const fn source_element(self) -> Arm64VectorElement {
        match self {
            Self::Smmla | Self::Usmmla | Self::Ummla => Arm64VectorElement::B,
            Self::FmmlaS => Arm64VectorElement::S,
            Self::FmmlaD => Arm64VectorElement::D,
            Self::Bfmmla | Self::Bfdot => Arm64VectorElement::H,
        }
    }

    /// Every operation, for exhaustive round-trip testing.
    pub const ALL: [Self; 7] = [
        Self::Smmla,
        Self::Usmmla,
        Self::Ummla,
        Self::FmmlaS,
        Self::FmmlaD,
        Self::Bfmmla,
        Self::Bfdot,
    ];
}
