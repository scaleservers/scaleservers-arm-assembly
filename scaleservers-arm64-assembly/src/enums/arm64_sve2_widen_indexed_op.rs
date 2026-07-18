// Copyright (c) Scaleservers LLC

/// An SVE2 **widening by-indexed-element** op (DDI0487 "SVE Multiply - Indexed"): `<op> Zd.<Tw>, Zn.<Tn>,
/// Zm.<Tn>[index]`, where the wide result (`.s` from `.h`, or `.d` from `.s`) accumulates a product against one
/// indexed lane of `Zm`. The op is selected by the `[15:12]` field; `B`/`T` (the `[10]` bit) read the even/odd lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2WidenIndexedOp {
    Smlalb,
    Smlalt,
    Umlalb,
    Umlalt,
    Smlslb,
    Smlslt,
    Umlslb,
    Umlslt,
    Smullb,
    Smullt,
    Umullb,
    Umullt,
    Sqdmullb,
    Sqdmullt,
}

impl Arm64Sve2WidenIndexedOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        use Arm64Sve2WidenIndexedOp::*;
        match self {
            Smlalb => "smlalb",
            Smlalt => "smlalt",
            Umlalb => "umlalb",
            Umlalt => "umlalt",
            Smlslb => "smlslb",
            Smlslt => "smlslt",
            Umlslb => "umlslb",
            Umlslt => "umlslt",
            Smullb => "smullb",
            Smullt => "smullt",
            Umullb => "umullb",
            Umullt => "umullt",
            Sqdmullb => "sqdmullb",
            Sqdmullt => "sqdmullt",
        }
    }

    /// The 4-bit `[15:12]` discriminator.
    pub fn discriminator(self) -> u32 {
        use Arm64Sve2WidenIndexedOp::*;
        match self {
            Smlalb | Smlalt => 0b1000,
            Umlalb | Umlalt => 0b1001,
            Smlslb | Smlslt => 0b1010,
            Umlslb | Umlslt => 0b1011,
            Smullb | Smullt => 0b1100,
            Umullb | Umullt => 0b1101,
            Sqdmullb | Sqdmullt => 0b1110,
        }
    }

    /// Whether this is a `T` (odd-lane) variant (the `[10]` bit).
    pub fn top(self) -> bool {
        use Arm64Sve2WidenIndexedOp::*;
        matches!(
            self,
            Smlalt | Umlalt | Smlslt | Umlslt | Smullt | Umullt | Sqdmullt
        )
    }

    /// Recover the op from its `[15:12]` discriminator and `[10]` top bit, or `None` if unallocated.
    pub fn from_bits(discriminator: u32, top: bool) -> Option<Self> {
        use Arm64Sve2WidenIndexedOp::*;
        let (b, t) = match discriminator & 0xF {
            0b1000 => (Smlalb, Smlalt),
            0b1001 => (Umlalb, Umlalt),
            0b1010 => (Smlslb, Smlslt),
            0b1011 => (Umlslb, Umlslt),
            0b1100 => (Smullb, Smullt),
            0b1101 => (Umullb, Umullt),
            0b1110 => (Sqdmullb, Sqdmullt),
            _ => return None,
        };
        Some(if top { t } else { b })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 14] = {
        use Arm64Sve2WidenIndexedOp::*;
        [
            Smlalb, Smlalt, Umlalb, Umlalt, Smlslb, Smlslt, Umlslb, Umlslt, Smullb, Smullt, Umullb,
            Umullt, Sqdmullb, Sqdmullt,
        ]
    };
}
