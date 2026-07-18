// Copyright (c) Scaleservers LLC

/// An SVE **predicated integer multiply-accumulate** op (DDI0487 part C). `MLA`/`MLS` accumulate into `Zda`
/// (`Zda +/-= Zn*Zm`); `MAD`/`MSB` are multiplicand-destructive (`Zdn = Za +/- Zn*Zdn`). The encoding fields are
/// uniform -- a register at `[20:16]`, one at `[9:5]`, and the destination at `[4:0]` -- but the UAL operand order
/// differs between the two forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveIntMacOp {
    /// `MLA` -- `Zda += Zn * Zm`.
    Mla,
    /// `MLS` -- `Zda -= Zn * Zm`.
    Mls,
    /// `MAD` -- `Zdn = Za + Zn * Zdn`.
    Mad,
    /// `MSB` -- `Zdn = Za - Zn * Zdn`.
    Msb,
}

impl Arm64SveIntMacOp {
    /// The fixed frame (size/regs/Pg all zero): MLA `0x0400_4000`, MLS `0x0400_6000`, MAD `0x0400_C000`,
    /// MSB `0x0400_E000` (the op occupies `[15:13]` = 010/011/110/111).
    pub fn base(self) -> u32 {
        match self {
            Self::Mla => 0x0400_4000,
            Self::Mls => 0x0400_6000,
            Self::Mad => 0x0400_C000,
            Self::Msb => 0x0400_E000,
        }
    }

    /// The `MAD`/`MSB` forms print `Zdn, Pg/M, Zm([20:16]), Za([9:5])`; `MLA`/`MLS` print `Zda, Pg/M, Zn([9:5]),
    /// Zm([20:16])`. This selects which encoding field prints first.
    pub fn is_multiplicand_destructive(self) -> bool {
        matches!(self, Self::Mad | Self::Msb)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Mla => "mla",
            Self::Mls => "mls",
            Self::Mad => "mad",
            Self::Msb => "msb",
        }
    }

    /// Recover the op from a word's `[15:13]` field, if one of the modeled MAC ops.
    pub fn from_word(word: u32) -> Option<Self> {
        match (word >> 13) & 0b111 {
            0b010 => Some(Self::Mla),
            0b011 => Some(Self::Mls),
            0b110 => Some(Self::Mad),
            0b111 => Some(Self::Msb),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Mla, Self::Mls, Self::Mad, Self::Msb];
}
