// Copyright (c) Scaleservers LLC

/// An AArch64 **store allocation tag** op (DDI0487 C6, FEAT_MTE) -- `STG`/`STZG`/`ST2G`/`STZ2G`. They write the
/// tag of `Xt` to one (or two, the `2`-suffix) 16-byte granule(s); the `Z` variants also zero the data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64StoreTagOp {
    /// `STG` -- store tag.
    Stg,
    /// `STZG` -- store tag, zeroing the granule.
    Stzg,
    /// `ST2G` -- store tag to two granules.
    St2g,
    /// `STZ2G` -- store tag to two granules, zeroing them.
    Stz2g,
}

impl Arm64StoreTagOp {
    /// The 2-bit `opc` field (`[23:22]`): STG 00 / STZG 01 / ST2G 10 / STZ2G 11.
    pub fn opc(self) -> u32 {
        match self {
            Self::Stg => 0b00,
            Self::Stzg => 0b01,
            Self::St2g => 0b10,
            Self::Stz2g => 0b11,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Stg => "stg",
            Self::Stzg => "stzg",
            Self::St2g => "st2g",
            Self::Stz2g => "stz2g",
        }
    }

    /// Recover the op from its `opc` field.
    pub fn from_opc(opc: u32) -> Self {
        match opc & 0b11 {
            0b00 => Self::Stg,
            0b01 => Self::Stzg,
            0b10 => Self::St2g,
            _ => Self::Stz2g,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Stg, Self::Stzg, Self::St2g, Self::Stz2g];
}
