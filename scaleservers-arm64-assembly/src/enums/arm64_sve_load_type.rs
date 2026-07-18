// Copyright (c) Scaleservers LLC

use crate::enums::Arm64VectorElement;

/// The kind of an SVE **contiguous load** (`LD1{S}{B,H,W,D}`, DDI0487 part C). The 4-bit `dtype` field (`[24:21]`)
/// jointly selects the memory access size, sign- vs zero-extension, and the destination element size. Unsigned
/// loads (`LD1B`/`LD1H`/`LD1W`/`LD1D`) zero-extend; signed loads (`LD1SB`/`LD1SH`/`LD1SW`) sign-extend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveContiguousLoadType {
    Ld1bB,
    Ld1bH,
    Ld1bS,
    Ld1bD,
    Ld1swD,
    Ld1hH,
    Ld1hS,
    Ld1hD,
    Ld1shD,
    Ld1shS,
    Ld1wS,
    Ld1wD,
    Ld1sbD,
    Ld1sbS,
    Ld1sbH,
    Ld1d,
}

impl Arm64SveContiguousLoadType {
    /// The 4-bit `dtype` value (`[24:21]`).
    pub fn dtype(self) -> u32 {
        match self {
            Self::Ld1bB => 0,
            Self::Ld1bH => 1,
            Self::Ld1bS => 2,
            Self::Ld1bD => 3,
            Self::Ld1swD => 4,
            Self::Ld1hH => 5,
            Self::Ld1hS => 6,
            Self::Ld1hD => 7,
            Self::Ld1shD => 8,
            Self::Ld1shS => 9,
            Self::Ld1wS => 10,
            Self::Ld1wD => 11,
            Self::Ld1sbD => 12,
            Self::Ld1sbS => 13,
            Self::Ld1sbH => 14,
            Self::Ld1d => 15,
        }
    }

    /// The lowercase load mnemonic (`ld1b`/`ld1sb`/...).
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Ld1bB | Self::Ld1bH | Self::Ld1bS | Self::Ld1bD => "ld1b",
            Self::Ld1hH | Self::Ld1hS | Self::Ld1hD => "ld1h",
            Self::Ld1wS | Self::Ld1wD => "ld1w",
            Self::Ld1d => "ld1d",
            Self::Ld1sbD | Self::Ld1sbS | Self::Ld1sbH => "ld1sb",
            Self::Ld1shD | Self::Ld1shS => "ld1sh",
            Self::Ld1swD => "ld1sw",
        }
    }

    /// The destination element size suffix.
    pub fn element(self) -> Arm64VectorElement {
        match self {
            Self::Ld1bB => Arm64VectorElement::B,
            Self::Ld1bH | Self::Ld1hH => Arm64VectorElement::H,
            Self::Ld1bS | Self::Ld1hS | Self::Ld1wS | Self::Ld1sbS | Self::Ld1shS => {
                Arm64VectorElement::S
            }
            Self::Ld1bD
            | Self::Ld1hD
            | Self::Ld1wD
            | Self::Ld1d
            | Self::Ld1swD
            | Self::Ld1shD
            | Self::Ld1sbD => Arm64VectorElement::D,
            Self::Ld1sbH => Arm64VectorElement::H,
        }
    }

    /// The memory access size in bytes (`ld1b`/`ld1sb` -> 1, `ld1h`/`ld1sh` -> 2, `ld1w`/`ld1sw` -> 4,
    /// `ld1d` -> 8). This is the granule actually fetched, independent of the (wider) destination element.
    pub fn access_size_bytes(self) -> u32 {
        match self {
            Self::Ld1bB
            | Self::Ld1bH
            | Self::Ld1bS
            | Self::Ld1bD
            | Self::Ld1sbD
            | Self::Ld1sbS
            | Self::Ld1sbH => 1,
            Self::Ld1hH | Self::Ld1hS | Self::Ld1hD | Self::Ld1shD | Self::Ld1shS => 2,
            Self::Ld1wS | Self::Ld1wD | Self::Ld1swD => 4,
            Self::Ld1d => 8,
        }
    }

    /// Recover the load type from its 4-bit `dtype`.
    pub fn from_dtype(dtype: u32) -> Self {
        Self::ALL[(dtype & 0xF) as usize]
    }

    /// Every load type, indexed by `dtype`.
    pub const ALL: [Self; 16] = [
        Self::Ld1bB,
        Self::Ld1bH,
        Self::Ld1bS,
        Self::Ld1bD,
        Self::Ld1swD,
        Self::Ld1hH,
        Self::Ld1hS,
        Self::Ld1hD,
        Self::Ld1shD,
        Self::Ld1shS,
        Self::Ld1wS,
        Self::Ld1wD,
        Self::Ld1sbD,
        Self::Ld1sbS,
        Self::Ld1sbH,
        Self::Ld1d,
    ];
}
