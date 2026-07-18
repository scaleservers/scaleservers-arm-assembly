// Copyright (c) Scaleservers LLC

use crate::enums::Arm64RegisterWidth;

/// The access width of an AArch64 single-register load/store: which of the four memory operand sizes
/// (`B`yte / `H`alf / `W`ord / `D`ouble) the instruction transfers.
///
/// ## Why a dedicated size enum (vs. reusing `Arm64RegisterWidth`)
/// The single-register load/store unsigned-immediate group encodes its access size in the 2-bit `size`
/// field `[31:30]` -- FOUR sizes (8/16/32/64-bit), not the two of the data-processing `sf` bit. The same
/// field also fixes two derived facts: the offset is **scaled by the access size** (`byte_offset = imm12 <<
/// size`), and the transfer register `Rt` is a `W` view for the 8/16/32-bit accesses but an `X` view for the
/// 64-bit access. We therefore carry the access size as its own enum (mapping to the 2-bit field) rather
/// than overloading [`Arm64RegisterWidth`] (which only models the `sf` W/X split); [`Self::rt_width`] derives
/// the `Rt` view from the size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64LoadStoreSize {
    /// 8-bit access (`LDRB`/`STRB`) -- `size = 0b00`, offset scale 1, `Wt` transfer register.
    Byte,
    /// 16-bit access (`LDRH`/`STRH`) -- `size = 0b01`, offset scale 2, `Wt` transfer register.
    Half,
    /// 32-bit access (`LDR`/`STR` of a `W` register) -- `size = 0b10`, offset scale 4, `Wt` transfer register.
    Word,
    /// 64-bit access (`LDR`/`STR` of an `X` register) -- `size = 0b11`, offset scale 8, `Xt` transfer register.
    Double,
}

impl Arm64LoadStoreSize {
    /// The 2-bit `size` field value (`[31:30]` of the load/store register encoding): 0/1/2/3 for
    /// Byte/Half/Word/Double. The encoder shifts this into bits `[31:30]`.
    pub fn size_bits(self) -> u32 {
        match self {
            Self::Byte => 0,
            Self::Half => 1,
            Self::Word => 2,
            Self::Double => 3,
        }
    }

    /// The byte-offset scale `1 << size` of this access (Byte 1, Half 2, Word 4, Double 8). The unsigned
    /// immediate offset must be a non-negative multiple of this, and `imm12 = byte_offset / scale`.
    pub fn scale(self) -> u32 {
        1 << self.size_bits()
    }

    /// The operand [`Arm64RegisterWidth`] of the transfer register `Rt`: [`Arm64RegisterWidth::X`] for the
    /// 64-bit [`Self::Double`] access, [`Arm64RegisterWidth::W`] for the 8/16/32-bit accesses (the narrower
    /// loads/stores write/read the `W` view). Drives both the emitter's register naming and the unused-`sf`
    /// note: unlike data-processing, the `size` field -- not a separate `sf` bit -- sets this.
    pub fn rt_width(self) -> Arm64RegisterWidth {
        match self {
            Self::Double => Arm64RegisterWidth::X,
            _ => Arm64RegisterWidth::W,
        }
    }

    /// The mnemonic size suffix used by the byte/halfword load/store and atomic forms: `"b"` (byte), `"h"`
    /// (halfword), `""` (word/doubleword -- the width is carried by the register name instead).
    pub fn mnemonic_suffix(self) -> &'static str {
        match self {
            Self::Byte => "b",
            Self::Half => "h",
            Self::Word | Self::Double => "",
        }
    }

    /// Recover the access size from the 2-bit `size` field of a decoded word (the inverse of
    /// [`Self::size_bits`]). TOTAL over the two low bits, so it never panics on untrusted input.
    pub fn from_size_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0 => Self::Byte,
            1 => Self::Half,
            2 => Self::Word,
            // `& 0b11` leaves 3 as the only remaining value.
            _ => Self::Double,
        }
    }
}
