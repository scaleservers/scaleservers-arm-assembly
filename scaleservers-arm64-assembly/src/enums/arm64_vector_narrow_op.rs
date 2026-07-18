// Copyright (c) Scaleservers LLC

/// A **vector narrowing extract** op of the Advanced SIMD two-register-miscellaneous group (DDI0487 C7, encoding
/// `0 Q U 01110 size 10000 opcode 10 Rn Rd`). The destination holds elements one size narrower than the source:
/// the encoded `size` is the destination element, and `Q` selects the `XTN`/`XTN2` (write the lower / upper half
/// of the 128-bit destination) variant. The source is the next-wider element at the full 128-bit width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorNarrowOp {
    /// `XTN`/`XTN2` -- extract narrow (truncate each element to the next-narrower size).
    Xtn,
    /// `SQXTN`/`SQXTN2` -- signed saturating extract narrow.
    Sqxtn,
    /// `SQXTUN`/`SQXTUN2` -- signed saturating extract unsigned narrow.
    Sqxtun,
    /// `UQXTN`/`UQXTN2` -- unsigned saturating extract narrow.
    Uqxtn,
}

impl Arm64VectorNarrowOp {
    /// The base word (`Q`/`size`/`Rn`/`Rd` zero): `(U<<29) | 0x0E20_0800 | (opcode<<12)`. The encoder ORs the
    /// destination arrangement's `Q<<30` and `size<<22`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Xtn => (0, 0b10010),
            Self::Sqxtn => (0, 0b10100),
            Self::Sqxtun => (1, 0b10010),
            Self::Uqxtn => (1, 0b10100),
        };
        (u << 29) | 0x0E20_0800 | (opcode << 12)
    }

    /// The lowercase UAL mnemonic (without the `2` upper-half suffix, which the emitter adds from `Q`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Xtn => "xtn",
            Self::Sqxtn => "sqxtn",
            Self::Sqxtun => "sqxtun",
            Self::Uqxtn => "uqxtn",
        }
    }

    /// Recover the op from a masked base (`word & 0xAF3F_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 4] = [Self::Xtn, Self::Sqxtn, Self::Sqxtun, Self::Uqxtn];
}
