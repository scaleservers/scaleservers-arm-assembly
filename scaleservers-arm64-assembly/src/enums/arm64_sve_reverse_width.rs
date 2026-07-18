// Copyright (c) Scaleservers LLC

/// The reversal granularity of an SVE predicated byte/halfword/word reverse-within-element op (DDI0487 part C):
/// `REVB`/`REVH`/`REVW`. The op occupies `[17:16]` of the encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveReverseWidth {
    /// `REVB` -- reverse bytes within each element.
    Byte,
    /// `REVH` -- reverse halfwords within each element.
    Half,
    /// `REVW` -- reverse words within each element.
    Word,
}

impl Arm64SveReverseWidth {
    /// The 2-bit `opc` field (`[17:16]`).
    pub fn opc(self) -> u32 {
        match self {
            Self::Byte => 0b00,
            Self::Half => 0b01,
            Self::Word => 0b10,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Byte => "revb",
            Self::Half => "revh",
            Self::Word => "revw",
        }
    }

    /// Recover the op from its `opc`, if a modeled width (`11` is unallocated).
    pub fn from_opc(opc: u32) -> Option<Self> {
        match opc & 0b11 {
            0b00 => Some(Self::Byte),
            0b01 => Some(Self::Half),
            0b10 => Some(Self::Word),
            _ => None,
        }
    }

    /// Every width, for tests.
    pub const ALL: [Self; 3] = [Self::Byte, Self::Half, Self::Word];
}
