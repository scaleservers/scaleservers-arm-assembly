// Copyright (c) Scaleservers LLC

use crate::enums::Arm64VectorElement;

/// The data precision of an SME floating-point outer product (`FMOPA`/`FMOPS`/`BFMOPA`/`BFMOPS`): the single- and
/// double-precision FP forms (accumulating into a `.s`/`.d` ZA tile from `.s`/`.d` sources) and the BFloat16 form
/// (`.h` sources widening into a `.s` ZA tile). Each carries the base encoding (without the `[4]` subtract bit) and
/// the tile/source element kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SmeFpPrecision {
    /// Single-precision FP32 (`FMOPA`/`FMOPS Zda.S` from `.s` sources, FEAT_SME).
    F32,
    /// Double-precision FP64 (`FMOPA`/`FMOPS Zda.D` from `.d` sources, FEAT_SME_F64F64).
    F64,
    /// BFloat16 widening to FP32 (`BFMOPA`/`BFMOPS Zda.S` from `.h` sources, FEAT_SME).
    Bf16,
}

impl Arm64SmeFpPrecision {
    /// The 32-bit encoding base (`[4]` subtract and all operand fields zero). GNU+LLVM verified.
    pub const fn base(self) -> u32 {
        match self {
            Self::F32 => 0x8080_0000,
            Self::F64 => 0x80C0_0000,
            Self::Bf16 => 0x8180_0000,
        }
    }

    /// The accumulator ZA-tile element (`.s` or `.d`).
    pub const fn za_element(self) -> Arm64VectorElement {
        match self {
            Self::F64 => Arm64VectorElement::D,
            _ => Arm64VectorElement::S,
        }
    }

    /// The source-vector element.
    pub const fn source_element(self) -> Arm64VectorElement {
        match self {
            Self::F32 => Arm64VectorElement::S,
            Self::F64 => Arm64VectorElement::D,
            Self::Bf16 => Arm64VectorElement::H,
        }
    }

    /// The highest valid ZA-tile number (`.s` has tiles `ZA0..ZA3`, `.d` has `ZA0..ZA7`).
    pub const fn max_tile(self) -> u8 {
        match self {
            Self::F64 => 7,
            _ => 3,
        }
    }

    /// The non-subtract (`FMOPA`/`BFMOPA`) mnemonic.
    pub const fn mnemonic_add(self) -> &'static str {
        match self {
            Self::Bf16 => "bfmopa",
            _ => "fmopa",
        }
    }

    /// The subtract (`FMOPS`/`BFMOPS`) mnemonic.
    pub const fn mnemonic_sub(self) -> &'static str {
        match self {
            Self::Bf16 => "bfmops",
            _ => "fmops",
        }
    }

    /// Recover the precision from the `[24]` bf16 bit and the `[22]` `.d` bit, or `None` for the unallocated
    /// `bf16 & .d` pair.
    pub const fn from_bits(bf16: u32, size_d: u32) -> Option<Self> {
        match (bf16 & 1, size_d & 1) {
            (0, 0) => Some(Self::F32),
            (0, 1) => Some(Self::F64),
            (1, 0) => Some(Self::Bf16),
            _ => None,
        }
    }

    /// Every precision, for tests.
    pub const ALL: [Self; 3] = [Self::F32, Self::F64, Self::Bf16];
}
