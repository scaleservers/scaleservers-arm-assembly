// Copyright (c) Scaleservers LLC

/// An SVE2 **saturating extract narrow** op (DDI0487 part C): `<op>{B,T} Zd.<Tn>, Zn.<T>`, saturating each wide `Zn`
/// element down into the half-width `Zd` (no `Zm`). Selected by `[12:11]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2ExtractNarrowOp {
    /// `SQXTN` -- signed saturating extract narrow (`[12:11]`=00).
    Sqxtn,
    /// `UQXTN` -- unsigned saturating extract narrow (`[12:11]`=01).
    Uqxtn,
    /// `SQXTUN` -- signed-to-unsigned saturating extract narrow (`[12:11]`=10).
    Sqxtun,
}

impl Arm64Sve2ExtractNarrowOp {
    /// The lowercase UAL mnemonic stem (without the `B`/`T` suffix).
    pub fn stem(self) -> &'static str {
        match self {
            Self::Sqxtn => "sqxtn",
            Self::Uqxtn => "uqxtn",
            Self::Sqxtun => "sqxtun",
        }
    }

    /// The 2-bit `[12:11]` opcode.
    pub fn opcode(self) -> u32 {
        match self {
            Self::Sqxtn => 0b00,
            Self::Uqxtn => 0b01,
            Self::Sqxtun => 0b10,
        }
    }

    /// Recover the op from its `[12:11]` opcode, or `None` for the unallocated `11`.
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        match opcode & 0b11 {
            0b00 => Some(Self::Sqxtn),
            0b01 => Some(Self::Uqxtn),
            0b10 => Some(Self::Sqxtun),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Sqxtn, Self::Uqxtn, Self::Sqxtun];
}
