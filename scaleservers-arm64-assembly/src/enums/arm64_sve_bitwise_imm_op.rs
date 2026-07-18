// Copyright (c) Scaleservers LLC

/// An SVE **bitwise logical with immediate** op (DDI0487 part C): `<op> Zdn.<T>, Zdn.<T>, #imm`, where the
/// immediate is a 13-bit `N:immr:imms` repeating bitmask (the same encoding as the A64 logical-immediate). `DUPM`
/// copies the mask. Base `0x0500_0000 | opc<<22 | imm13<<5 | Zdn`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveBitwiseImmOp {
    /// `ORR` -- bitwise OR with the mask (opc 00).
    Orr,
    /// `EOR` -- bitwise exclusive-OR (opc 01).
    Eor,
    /// `AND` -- bitwise AND (opc 10).
    And,
    /// `DUPM` -- duplicate the mask into the register (opc 11).
    Dupm,
}

impl Arm64SveBitwiseImmOp {
    /// The 2-bit `opc` field (`[23:22]`).
    pub fn opc(self) -> u32 {
        match self {
            Self::Orr => 0b00,
            Self::Eor => 0b01,
            Self::And => 0b10,
            Self::Dupm => 0b11,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Orr => "orr",
            Self::Eor => "eor",
            Self::And => "and",
            Self::Dupm => "dupm",
        }
    }

    /// Recover the op from its `opc` field.
    pub fn from_opc(opc: u32) -> Self {
        match opc & 0b11 {
            0b00 => Self::Orr,
            0b01 => Self::Eor,
            0b10 => Self::And,
            _ => Self::Dupm,
        }
    }

    /// Whether the op writes two source operands in UAL (`AND`/`ORR`/`EOR` are `Zdn, Zdn, #imm`; `DUPM` is
    /// `Zd, #imm`).
    pub fn is_binary(self) -> bool {
        !matches!(self, Self::Dupm)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Orr, Self::Eor, Self::And, Self::Dupm];
}
