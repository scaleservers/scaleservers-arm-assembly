// Copyright (c) Scaleservers LLC

/// The element/access size of an SME `ZA` tile-slice load/store (`LD1`/`ST1` to `ZA`): byte/halfword/word/doubleword
/// and the 128-bit quadword `Q` (which the ordinary [`crate::enums::Arm64VectorElement`] does not model). Each
/// carries its `[24:22]` `msz` code and the slice-offset field width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SmeTileSize {
    /// `B` -- byte (`ZA0.B`, the single byte tile).
    B,
    /// `H` -- halfword (`ZA0..ZA1.H`).
    H,
    /// `W` -- word, the `.s` tile (`ZA0..ZA3.S`).
    W,
    /// `D` -- doubleword (`ZA0..ZA7.D`).
    D,
    /// `Q` -- quadword (`ZA0..ZA15.Q`).
    Q,
}

impl Arm64SmeTileSize {
    /// The 3-bit `[24:22]` `msz` code (B=0, H=1, W=2, D=3, Q=7).
    pub const fn msz_code(self) -> u32 {
        match self {
            Self::B => 0,
            Self::H => 1,
            Self::W => 2,
            Self::D => 3,
            Self::Q => 7,
        }
    }

    /// The number of bits the slice OFFSET takes in the 4-bit tile:offset field (B=4, H=3, W=2, D=1, Q=0). The tile
    /// number takes the remaining high bits.
    pub const fn offset_bits(self) -> u32 {
        match self {
            Self::B => 4,
            Self::H => 3,
            Self::W => 2,
            Self::D => 1,
            Self::Q => 0,
        }
    }

    /// The `log2` of the access size in bytes -- the `LSL #amount` applied to the index register.
    pub const fn shift(self) -> u32 {
        match self {
            Self::B => 0,
            Self::H => 1,
            Self::W => 2,
            Self::D => 3,
            Self::Q => 4,
        }
    }

    /// The tile-element suffix letter (`b`/`h`/`s`/`d`/`q`).
    pub const fn element_letter(self) -> &'static str {
        match self {
            Self::B => "b",
            Self::H => "h",
            Self::W => "s",
            Self::D => "d",
            Self::Q => "q",
        }
    }

    /// The mnemonic size letter (`LD1B`/`LD1H`/`LD1W`/`LD1D`/`LD1Q`).
    pub const fn mnemonic_letter(self) -> &'static str {
        match self {
            Self::B => "b",
            Self::H => "h",
            Self::W => "w",
            Self::D => "d",
            Self::Q => "q",
        }
    }

    /// Recover the size from its `[24:22]` `msz` code, or `None` for an unallocated value.
    pub const fn from_msz(msz: u32) -> Option<Self> {
        match msz & 0b111 {
            0 => Some(Self::B),
            1 => Some(Self::H),
            2 => Some(Self::W),
            3 => Some(Self::D),
            7 => Some(Self::Q),
            _ => None,
        }
    }

    /// Every size, for tests.
    pub const ALL: [Self; 5] = [Self::B, Self::H, Self::W, Self::D, Self::Q];
}
