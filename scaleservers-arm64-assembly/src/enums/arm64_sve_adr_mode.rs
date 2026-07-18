// Copyright (c) Scaleservers LLC

use crate::enums::Arm64VectorElement;

/// The addressing mode of an SVE vector-of-addresses `ADR` (DDI0487 part C): `ADR Zd.<T>, [Zn.<T>, Zm.<T>{, <mod>
/// #<amount>}]`. The 2-bit `[23:22]` field jointly selects the element size and how each `Zm` offset is treated:
/// packed 32-bit (`.s`), unpacked 64-bit (`.d`), or a 32-bit offset sign-/zero-extended into 64 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveAdrMode {
    /// `.d, SXTW` -- each 64-bit lane adds a sign-extended 32-bit offset (`[23:22]` = 00).
    DSxtw,
    /// `.d, UXTW` -- each 64-bit lane adds a zero-extended 32-bit offset (`[23:22]` = 01).
    DUxtw,
    /// `.s, LSL` -- packed 32-bit base and offset (`[23:22]` = 10).
    SPacked,
    /// `.d, LSL` -- unpacked 64-bit base and offset (`[23:22]` = 11).
    DUnpacked,
}

impl Arm64SveAdrMode {
    /// The 2-bit `[23:22]` selector.
    pub fn sel(self) -> u32 {
        match self {
            Self::DSxtw => 0b00,
            Self::DUxtw => 0b01,
            Self::SPacked => 0b10,
            Self::DUnpacked => 0b11,
        }
    }

    /// Recover the mode from its `[23:22]` selector.
    pub fn from_sel(sel: u32) -> Self {
        match sel & 0b11 {
            0b00 => Self::DSxtw,
            0b01 => Self::DUxtw,
            0b10 => Self::SPacked,
            _ => Self::DUnpacked,
        }
    }

    /// The element suffix of all three vector operands (`.s` only for the packed form, else `.d`).
    pub fn element(self) -> Arm64VectorElement {
        match self {
            Self::SPacked => Arm64VectorElement::S,
            _ => Arm64VectorElement::D,
        }
    }

    /// The offset modifier keyword.
    pub fn modifier(self) -> &'static str {
        match self {
            Self::DSxtw => "sxtw",
            Self::DUxtw => "uxtw",
            Self::SPacked | Self::DUnpacked => "lsl",
        }
    }

    /// Whether the modifier is `LSL` (which is omitted entirely when the shift amount is zero; `SXTW`/`UXTW`
    /// always print, only their `#amount` being dropped at zero).
    pub fn is_lsl(self) -> bool {
        matches!(self, Self::SPacked | Self::DUnpacked)
    }

    /// Every mode, for tests.
    pub const ALL: [Self; 4] = [Self::DSxtw, Self::DUxtw, Self::SPacked, Self::DUnpacked];
}
