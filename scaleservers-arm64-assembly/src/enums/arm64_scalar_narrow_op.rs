// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **saturating narrow** op (DDI0487 C7, the two-register-misc `01 U 11110 size 10000
/// opcode 10 Rn Rd` encoding with a one-size-narrower result). The source register is one element size wider than
/// the destination (`b<-h`, `h<-s`, `s<-d`); the encoded `size` is the destination element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarNarrowOp {
    /// `SQXTN` -- signed saturating extract narrow.
    Sqxtn,
    /// `SQXTUN` -- signed saturating extract unsigned narrow.
    Sqxtun,
    /// `UQXTN` -- unsigned saturating extract narrow.
    Uqxtn,
}

impl Arm64ScalarNarrowOp {
    /// The base word (`size`/`Rn`/`Rd` zero): `0x5E20_0800 | (U<<29) | (opcode<<12)`. The encoder ORs the
    /// destination `size<<22`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Sqxtn => (0, 0b10100),
            Self::Sqxtun => (1, 0b10010),
            Self::Uqxtn => (1, 0b10100),
        };
        0x5E20_0800 | (u << 29) | (opcode << 12)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sqxtn => "sqxtn",
            Self::Sqxtun => "sqxtun",
            Self::Uqxtn => "uqxtn",
        }
    }

    /// Recover the op from a masked base (`word & 0xFF3F_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Sqxtn, Self::Sqxtun, Self::Uqxtn];
}
