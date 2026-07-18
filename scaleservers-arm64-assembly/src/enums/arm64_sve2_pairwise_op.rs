// Copyright (c) Scaleservers LLC

/// An SVE2 **integer pairwise arithmetic (predicated)** op (DDI0487 C4.1): `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>`,
/// reducing adjacent element pairs. Selected by `opc:U` at `[18:16]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2PairwiseOp {
    /// `ADDP` -- pairwise add (`opc:U`=001).
    Addp,
    /// `SMAXP` -- pairwise signed maximum (`opc:U`=100).
    Smaxp,
    /// `UMAXP` -- pairwise unsigned maximum (`opc:U`=101).
    Umaxp,
    /// `SMINP` -- pairwise signed minimum (`opc:U`=110).
    Sminp,
    /// `UMINP` -- pairwise unsigned minimum (`opc:U`=111).
    Uminp,
}

impl Arm64Sve2PairwiseOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Addp => "addp",
            Self::Smaxp => "smaxp",
            Self::Umaxp => "umaxp",
            Self::Sminp => "sminp",
            Self::Uminp => "uminp",
        }
    }

    /// The 3-bit `opc:U` opcode (`[18:16]`).
    pub fn code(self) -> u32 {
        match self {
            Self::Addp => 0b001,
            Self::Smaxp => 0b100,
            Self::Umaxp => 0b101,
            Self::Sminp => 0b110,
            Self::Uminp => 0b111,
        }
    }

    /// Recover the op from its `[18:16]` code, or `None` for the unallocated values.
    pub fn from_code(code: u32) -> Option<Self> {
        match code & 0b111 {
            0b001 => Some(Self::Addp),
            0b100 => Some(Self::Smaxp),
            0b101 => Some(Self::Umaxp),
            0b110 => Some(Self::Sminp),
            0b111 => Some(Self::Uminp),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 5] = [
        Self::Addp,
        Self::Smaxp,
        Self::Umaxp,
        Self::Sminp,
        Self::Uminp,
    ];
}
