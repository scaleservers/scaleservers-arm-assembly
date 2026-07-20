// Copyright (c) Scaleservers LLC

/// An SVE2 **integer by-indexed-element** op (DDI0487 part C): `<op> Zd.<T>, Zn.<T>, Zm.<T>[index]`. `MUL` writes a
/// product; `MLA`/`MLS` accumulate into `Zd`; `SQDMULH` is a saturating doubling multiply returning the high half,
/// and `SQRDMLAH`/`SQRDMLSH` are the rounding saturating doubling multiply-add/subtract-high accumulators. The
/// opcode sits at `[15:10]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveIntIndexedOp {
    /// `MUL` -- multiply (`[15:10]` = 111110).
    Mul,
    /// `MLA` -- multiply-accumulate (`[15:10]` = 000010).
    Mla,
    /// `MLS` -- multiply-subtract (`[15:10]` = 000011).
    Mls,
    /// `SQDMULH` -- saturating doubling multiply returning high half (`[15:10]` = 111100).
    Sqdmulh,
    /// `SQRDMULH` -- saturating rounding doubling multiply returning high half (`[15:10]` = 111101).
    Sqrdmulh,
    /// `SQRDMLAH` -- saturating rounding doubling multiply-add high (`[15:10]` = 000100).
    Sqrdmlah,
    /// `SQRDMLSH` -- saturating rounding doubling multiply-subtract high (`[15:10]` = 000101).
    Sqrdmlsh,
}

impl Arm64SveIntIndexedOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Mul => "mul",
            Self::Mla => "mla",
            Self::Mls => "mls",
            Self::Sqdmulh => "sqdmulh",
            Self::Sqrdmulh => "sqrdmulh",
            Self::Sqrdmlah => "sqrdmlah",
            Self::Sqrdmlsh => "sqrdmlsh",
        }
    }

    /// The 6-bit `[15:10]` opcode.
    pub fn opcode(self) -> u32 {
        match self {
            Self::Mul => 0b111110,
            Self::Mla => 0b000010,
            Self::Mls => 0b000011,
            Self::Sqdmulh => 0b111100,
            Self::Sqrdmulh => 0b111101,
            Self::Sqrdmlah => 0b000100,
            Self::Sqrdmlsh => 0b000101,
        }
    }

    /// Recover the op from its `[15:10]` opcode, or `None` for an unallocated value.
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        match opcode & 0b111111 {
            0b111110 => Some(Self::Mul),
            0b000010 => Some(Self::Mla),
            0b000011 => Some(Self::Mls),
            0b111100 => Some(Self::Sqdmulh),
            0b111101 => Some(Self::Sqrdmulh),
            0b000100 => Some(Self::Sqrdmlah),
            0b000101 => Some(Self::Sqrdmlsh),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 7] = [
        Self::Mul,
        Self::Mla,
        Self::Mls,
        Self::Sqdmulh,
        Self::Sqrdmulh,
        Self::Sqrdmlah,
        Self::Sqrdmlsh,
    ];
}
