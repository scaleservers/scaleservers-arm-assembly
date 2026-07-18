// Copyright (c) Scaleservers LLC

/// A FEAT_PAuth_LR pointer-authentication operation that takes a **PC-relative label** as its modifier -- the
/// `AUTIASPPC`/`AUTIBSPPC`/`RETAASPPC`/`RETABSPPC <label>` forms (authenticate / authenticated-return on `LR`,
/// using the address of `<label>` as the discriminator). The label is encoded as a signed 16-bit PC-relative
/// branch offset (`imm16` at `[20:5]`, scaled by 4); `Rn` is the implicit `LR` (`[4:0] = 11111`).
///
/// **Experimental / single-oracle:** only LLVM-20 emits these (binutils-trunk lacks `+pauth-lr`); the encodings
/// are LLVM-20-confirmed and match the DDI0487 diagrams. Gated behind the `experimental` cargo feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64PointerAuthLrLabelOp {
    /// `AUTIASPPC <label>` -- authenticate `LR` with key A (`[23]=1`, `[21]=0`).
    Autiasppc,
    /// `AUTIBSPPC <label>` -- authenticate `LR` with key B (`[23]=1`, `[21]=1`).
    Autibsppc,
    /// `RETAASPPC <label>` -- authenticated return with key A (`[23]=0`, `[21]=0`).
    Retaasppc,
    /// `RETABSPPC <label>` -- authenticated return with key B (`[23]=0`, `[21]=1`).
    Retabsppc,
}

impl Arm64PointerAuthLrLabelOp {
    /// Every operation, for exhaustive round-trip tests.
    pub const ALL: [Self; 4] = [
        Self::Autiasppc,
        Self::Autibsppc,
        Self::Retaasppc,
        Self::Retabsppc,
    ];

    /// The base word (with a zero label offset); the caller ORs `imm16 << 5`. `Rn = LR` is the fixed `[4:0]=11111`.
    pub fn base(self) -> u32 {
        match self {
            Self::Autiasppc => 0xF380_001F,
            Self::Autibsppc => 0xF3A0_001F,
            Self::Retaasppc => 0x5500_001F,
            Self::Retabsppc => 0x5520_001F,
        }
    }

    /// Recover the operation from a masked word (`word & 0xFFE0_001F`, i.e. the base bits with the `imm16` offset
    /// cleared); `None` if it is not one of these four.
    pub fn from_base(masked: u32) -> Option<Self> {
        match masked {
            0xF380_001F => Some(Self::Autiasppc),
            0xF3A0_001F => Some(Self::Autibsppc),
            0x5500_001F => Some(Self::Retaasppc),
            0x5520_001F => Some(Self::Retabsppc),
            _ => None,
        }
    }

    /// The lowercase mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Autiasppc => "autiasppc",
            Self::Autibsppc => "autibsppc",
            Self::Retaasppc => "retaasppc",
            Self::Retabsppc => "retabsppc",
        }
    }
}
