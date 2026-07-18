// Copyright (c) Scaleservers LLC

/// An SVE **unpredicated bitwise logical** op over the whole register: `<op> Zd.D, Zn.D, Zm.D` (DDI0487 part C).
/// The operands are named `.d` but the operation is bitwise over the entire vector. Base
/// `0x0420_3000 | opc<<22 | Zm<<16 | Zn<<5 | Zd`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveBitwiseLogicalOp {
    /// `AND` -- `Zn & Zm`.
    And,
    /// `ORR` -- `Zn | Zm`.
    Orr,
    /// `EOR` -- `Zn ^ Zm`.
    Eor,
    /// `BIC` -- `Zn & ~Zm`.
    Bic,
}

impl Arm64SveBitwiseLogicalOp {
    /// The 2-bit `opc` field (`[23:22]`).
    pub fn opc(self) -> u32 {
        match self {
            Self::And => 0b00,
            Self::Orr => 0b01,
            Self::Eor => 0b10,
            Self::Bic => 0b11,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Orr => "orr",
            Self::Eor => "eor",
            Self::Bic => "bic",
        }
    }

    /// Recover the op from its `opc` field.
    pub fn from_opc(opc: u32) -> Self {
        match opc & 0b11 {
            0b00 => Self::And,
            0b01 => Self::Orr,
            0b10 => Self::Eor,
            _ => Self::Bic,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::And, Self::Orr, Self::Eor, Self::Bic];
}
