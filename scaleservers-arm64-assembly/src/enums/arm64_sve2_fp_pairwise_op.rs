// Copyright (c) Scaleservers LLC

/// An SVE2 **floating-point pairwise** op (DDI0487): `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>`, reducing adjacent
/// floating-point element pairs of the `Zdn:Zm` concatenation. Selected by `opc` at `[18:16]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2FpPairwiseOp {
    /// `FADDP` -- pairwise add (`opc`=000).
    Faddp,
    /// `FMAXNMP` -- pairwise maximum-number (`opc`=100).
    Fmaxnmp,
    /// `FMINNMP` -- pairwise minimum-number (`opc`=101).
    Fminnmp,
    /// `FMAXP` -- pairwise maximum (`opc`=110).
    Fmaxp,
    /// `FMINP` -- pairwise minimum (`opc`=111).
    Fminp,
}

impl Arm64Sve2FpPairwiseOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Faddp => "faddp",
            Self::Fmaxnmp => "fmaxnmp",
            Self::Fminnmp => "fminnmp",
            Self::Fmaxp => "fmaxp",
            Self::Fminp => "fminp",
        }
    }

    /// The 3-bit `opc` opcode (`[18:16]`).
    pub fn opc(self) -> u32 {
        match self {
            Self::Faddp => 0b000,
            Self::Fmaxnmp => 0b100,
            Self::Fminnmp => 0b101,
            Self::Fmaxp => 0b110,
            Self::Fminp => 0b111,
        }
    }

    /// Recover the op from its `[18:16]` opcode, or `None` for an unallocated value.
    pub fn from_opc(opc: u32) -> Option<Self> {
        match opc & 0b111 {
            0b000 => Some(Self::Faddp),
            0b100 => Some(Self::Fmaxnmp),
            0b101 => Some(Self::Fminnmp),
            0b110 => Some(Self::Fmaxp),
            0b111 => Some(Self::Fminp),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 5] = [
        Self::Faddp,
        Self::Fmaxnmp,
        Self::Fminnmp,
        Self::Fmaxp,
        Self::Fminp,
    ];
}
