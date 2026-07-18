// Copyright (c) Scaleservers LLC

/// An AArch64 **CRC32** checksum op (DDI0487 C6, FEAT_CRC32) -- the data-processing (2-source) `sf 0011010110 Rm
/// 0100 sz C Rn Rd` encoding. `Rd`/`Rn` are always the 32-bit accumulator (`Wd`/`Wn`); the data input `Rm` is
/// `Wm` except for the `*x` ops (64-bit data, `Xm`). The `C` variants use the Castagnoli polynomial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Crc32Op {
    /// `CRC32B` -- CRC-32 over a byte.
    Crc32b,
    /// `CRC32H` -- CRC-32 over a halfword.
    Crc32h,
    /// `CRC32W` -- CRC-32 over a word.
    Crc32w,
    /// `CRC32X` -- CRC-32 over a doubleword (64-bit `Xm`).
    Crc32x,
    /// `CRC32CB` -- CRC-32C over a byte.
    Crc32cb,
    /// `CRC32CH` -- CRC-32C over a halfword.
    Crc32ch,
    /// `CRC32CW` -- CRC-32C over a word.
    Crc32cw,
    /// `CRC32CX` -- CRC-32C over a doubleword (64-bit `Xm`).
    Crc32cx,
}

impl Arm64Crc32Op {
    /// The base word (`Rm`/`Rn`/`Rd` zero): `0x1AC0_4000 | (sf<<31) | (variant<<10)`. The `sf` bit is set for the
    /// `*x` ops (their data input is 64-bit). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        let (sf, opcode): (u32, u32) = match self {
            Self::Crc32b => (0, 0b010000),
            Self::Crc32h => (0, 0b010001),
            Self::Crc32w => (0, 0b010010),
            Self::Crc32x => (1, 0b010011),
            Self::Crc32cb => (0, 0b010100),
            Self::Crc32ch => (0, 0b010101),
            Self::Crc32cw => (0, 0b010110),
            Self::Crc32cx => (1, 0b010111),
        };
        0x1AC0_0000 | (sf << 31) | (opcode << 10)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Crc32b => "crc32b",
            Self::Crc32h => "crc32h",
            Self::Crc32w => "crc32w",
            Self::Crc32x => "crc32x",
            Self::Crc32cb => "crc32cb",
            Self::Crc32ch => "crc32ch",
            Self::Crc32cw => "crc32cw",
            Self::Crc32cx => "crc32cx",
        }
    }

    /// Whether the data input `Rm` is a 64-bit `Xm` (the `*x` ops) rather than a 32-bit `Wm`.
    pub fn data_is_64bit(self) -> bool {
        matches!(self, Self::Crc32x | Self::Crc32cx)
    }

    /// Recover the op from a masked base (`word & 0xFFE0_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 8] = [
        Self::Crc32b,
        Self::Crc32h,
        Self::Crc32w,
        Self::Crc32x,
        Self::Crc32cb,
        Self::Crc32ch,
        Self::Crc32cw,
        Self::Crc32cx,
    ];
}
