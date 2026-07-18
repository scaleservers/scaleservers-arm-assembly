// Copyright (c) Scaleservers LLC

use crate::enums::Arm64VectorElement;

/// The SME2 multi-vector **ZA dot-product** operation: a signed/unsigned/mixed dot product of a `Zn` vector group with
/// a `Zm` multiplier, accumulated INTO a ZA single-vector group. Three source/accumulator shapes exist -- 4-way from
/// `.b` sources into a `.s` group (`SDOT`/`UDOT`/`USDOT`/`SUDOT`), 2-way from `.h` sources into a `.s` group
/// (`SDOT`/`UDOT`), and 4-way from `.h` sources into a `.d` group (`SDOT`/`UDOT`, FEAT_SME_I16I64). The op is the
/// `([22], [4], [3])` discriminant triple of the shared `ZA`-dot frame; all eight combinations are allocated, so
/// [`Self::from_bits`] is total. GNU+LLVM verified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64Sme2ZaDotOp {
    /// `SDOT` 4-way, signed `.b` sources into a `.s` ZA group (FEAT_SME2).
    SdotByteToS,
    /// `UDOT` 4-way, unsigned `.b` sources into a `.s` ZA group (FEAT_SME2).
    UdotByteToS,
    /// `USDOT` 4-way, unsigned-by-signed `.b` sources into a `.s` ZA group (FEAT_SME2).
    UsdotByteToS,
    /// `SUDOT` 4-way, signed-by-unsigned `.b` sources into a `.s` ZA group (FEAT_SME2).
    SudotByteToS,
    /// `SDOT` 2-way, signed `.h` sources into a `.s` ZA group (FEAT_SME2).
    SdotHalfToS,
    /// `UDOT` 2-way, unsigned `.h` sources into a `.s` ZA group (FEAT_SME2).
    UdotHalfToS,
    /// `SDOT` 4-way, signed `.h` sources into a `.d` ZA group (FEAT_SME2 + FEAT_SME_I16I64).
    SdotHalfToD,
    /// `UDOT` 4-way, unsigned `.h` sources into a `.d` ZA group (FEAT_SME2 + FEAT_SME_I16I64).
    UdotHalfToD,
}

impl Arm64Sme2ZaDotOp {
    /// The op-discriminant bits OR-ed into the base: source-`.h` at `[22]`, plus the `[4]`/`[3]` op bits.
    pub const fn discriminant(self) -> u32 {
        let (b22, b4, b3): (u32, u32, u32) = match self {
            Self::SdotByteToS => (0, 0, 0),
            Self::UdotByteToS => (0, 1, 0),
            Self::UsdotByteToS => (0, 0, 1),
            Self::SudotByteToS => (0, 1, 1),
            Self::SdotHalfToD => (1, 0, 0),
            Self::UdotHalfToD => (1, 1, 0),
            Self::SdotHalfToS => (1, 0, 1),
            Self::UdotHalfToS => (1, 1, 1),
        };
        (b22 << 22) | (b4 << 4) | (b3 << 3)
    }

    /// The full base word (zeroed operands) of the INDEXED ZA-dot form (`Zm.<Ts>[index]`). Unlike the single/multi
    /// forms (which share one base + the `[22]/[4]/[3]` discriminant), the indexed encoding muxes the op across
    /// `[23]`/`[22]`/`[12]`/`[5:3]` per shape, so each op carries its own base. GNU-oracle verified.
    pub const fn indexed_base(self) -> u32 {
        match self {
            Self::SdotByteToS => 0xC150_1020,
            Self::UdotByteToS => 0xC150_1030,
            Self::UsdotByteToS => 0xC150_1028,
            Self::SudotByteToS => 0xC150_1038,
            Self::SdotHalfToS => 0xC150_1000,
            Self::UdotHalfToS => 0xC150_1010,
            Self::SdotHalfToD => 0xC1D0_0008,
            Self::UdotHalfToD => 0xC1D0_0018,
        }
    }

    /// Recover the op from the `([22], [4], [3])` discriminant triple. Total -- all eight combinations are allocated.
    pub const fn from_bits(bit22: u32, bit4: u32, bit3: u32) -> Self {
        match (bit22 & 1, bit4 & 1, bit3 & 1) {
            (0, 0, 0) => Self::SdotByteToS,
            (0, 1, 0) => Self::UdotByteToS,
            (0, 0, 1) => Self::UsdotByteToS,
            (0, 1, 1) => Self::SudotByteToS,
            (1, 0, 0) => Self::SdotHalfToD,
            (1, 1, 0) => Self::UdotHalfToD,
            (1, 0, 1) => Self::SdotHalfToS,
            _ => Self::UdotHalfToS,
        }
    }

    /// The UAL mnemonic.
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::SdotByteToS | Self::SdotHalfToS | Self::SdotHalfToD => "sdot",
            Self::UdotByteToS | Self::UdotHalfToS | Self::UdotHalfToD => "udot",
            Self::UsdotByteToS => "usdot",
            Self::SudotByteToS => "sudot",
        }
    }

    /// The source-vector element (`.b` or `.h`).
    pub const fn source_element(self) -> Arm64VectorElement {
        match self {
            Self::SdotByteToS | Self::UdotByteToS | Self::UsdotByteToS | Self::SudotByteToS => {
                Arm64VectorElement::B
            }
            _ => Arm64VectorElement::H,
        }
    }

    /// The ZA-group accumulator element (`.s` or `.d`).
    pub const fn za_element(self) -> Arm64VectorElement {
        match self {
            Self::SdotHalfToD | Self::UdotHalfToD => Arm64VectorElement::D,
            _ => Arm64VectorElement::S,
        }
    }

    /// Whether this form additionally needs FEAT_SME_I16I64 (the `.d`-accumulator forms).
    pub const fn needs_i16i64(self) -> bool {
        matches!(self, Self::SdotHalfToD | Self::UdotHalfToD)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 8] = [
        Self::SdotByteToS,
        Self::UdotByteToS,
        Self::UsdotByteToS,
        Self::SudotByteToS,
        Self::SdotHalfToS,
        Self::UdotHalfToS,
        Self::SdotHalfToD,
        Self::UdotHalfToD,
    ];
}
