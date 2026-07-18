// Copyright (c) Scaleservers LLC

/// The lane **arrangement** of an AArch64 Advanced SIMD (NEON) vector operand -- how the 64- or 128-bit register
/// is divided into equal lanes (DDI0487 C7). It encodes two orthogonal facts the "three same" and most other
/// vector encodings carry as separate bits: the **register width** via `Q` (bit 30 -- `0` = the 64-bit `Dn` half,
/// `1` = the full 128-bit `Qn`) and the **element size** via `size` (bits `[23:22]` -- `00` byte / `01` halfword /
/// `10` word / `11` doubleword). The mnemonic suffix is `<lanes>.<element>`, e.g. `.4s` = four 32-bit lanes.
///
/// For the floating-point vector forms the encoding uses only a 1-bit `sz` (single vs double) at bit 22; that is
/// the low bit of [`Self::size_bits`] ([`Self::fp_sz_bit`]), and only `.2s`/`.4s`/`.2d` are FP-valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorArrangement {
    /// `.8b` -- eight 8-bit lanes (Q=0, size=00).
    B8,
    /// `.16b` -- sixteen 8-bit lanes (Q=1, size=00).
    B16,
    /// `.4h` -- four 16-bit lanes (Q=0, size=01).
    H4,
    /// `.8h` -- eight 16-bit lanes (Q=1, size=01).
    H8,
    /// `.2s` -- two 32-bit lanes (Q=0, size=10).
    S2,
    /// `.4s` -- four 32-bit lanes (Q=1, size=10).
    S4,
    /// `.1d` -- one 64-bit lane (Q=0, size=11).
    D1,
    /// `.2d` -- two 64-bit lanes (Q=1, size=11).
    D2,
}

impl Arm64VectorArrangement {
    /// The `Q` bit (bit 30): `1` for the 128-bit arrangements, `0` for the 64-bit ones.
    pub fn q_bit(self) -> u32 {
        match self {
            Self::B16 | Self::H8 | Self::S4 | Self::D2 => 1,
            Self::B8 | Self::H4 | Self::S2 | Self::D1 => 0,
        }
    }

    /// The 2-bit `size` field (bits `[23:22]`): byte 00, halfword 01, word 10, doubleword 11.
    pub fn size_bits(self) -> u32 {
        match self {
            Self::B8 | Self::B16 => 0b00,
            Self::H4 | Self::H8 => 0b01,
            Self::S2 | Self::S4 => 0b10,
            Self::D1 | Self::D2 => 0b11,
        }
    }

    /// The 1-bit `sz` the FP vector forms use (bit 22): `0` single (`.2s`/`.4s`), `1` double (`.2d`). It is the
    /// low bit of [`Self::size_bits`].
    pub fn fp_sz_bit(self) -> u32 {
        self.size_bits() & 1
    }

    /// The lowercase mnemonic suffix (`8b`, `16b`, `4h`, ..., `2d`) -- rendered after the register as `v0.<suffix>`.
    pub fn name(self) -> &'static str {
        match self {
            Self::B8 => "8b",
            Self::B16 => "16b",
            Self::H4 => "4h",
            Self::H8 => "8h",
            Self::S2 => "2s",
            Self::S4 => "4s",
            Self::D1 => "1d",
            Self::D2 => "2d",
        }
    }

    /// Recover the arrangement from a decoded `Q` bit and 2-bit `size` field (total over all eight combinations).
    pub fn from_q_and_size(q: u32, size: u32) -> Self {
        match (q & 1, size & 0b11) {
            (0, 0b00) => Self::B8,
            (1, 0b00) => Self::B16,
            (0, 0b01) => Self::H4,
            (1, 0b01) => Self::H8,
            (0, 0b10) => Self::S2,
            (1, 0b10) => Self::S4,
            (0, 0b11) => Self::D1,
            _ => Self::D2,
        }
    }
}
