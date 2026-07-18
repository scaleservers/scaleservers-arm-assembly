// Copyright (c) Scaleservers LLC

/// An SVE **integer compare with signed immediate** op (DDI0487 part C): `<op> Pd.<T>, Pg/Z, Zn.<T>, #imm5`, where
/// `imm5` is `-16..=15`. Selected by a 3-bit field at `[15:13]` plus a low bit at `[4]`, over base `0x2500_0000`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveCmpImmSignedOp {
    /// `CMPGE` -- signed greater-or-equal.
    Cmpge,
    /// `CMPGT` -- signed greater-than.
    Cmpgt,
    /// `CMPLT` -- signed less-than.
    Cmplt,
    /// `CMPLE` -- signed less-or-equal.
    Cmple,
    /// `CMPEQ` -- equal.
    Cmpeq,
    /// `CMPNE` -- not equal.
    Cmpne,
}

impl Arm64SveCmpImmSignedOp {
    /// `(op_high[15:13], op_low[4])`.
    fn bits(self) -> (u32, u32) {
        match self {
            Self::Cmpge => (0b000, 0),
            Self::Cmpgt => (0b000, 1),
            Self::Cmplt => (0b001, 0),
            Self::Cmple => (0b001, 1),
            Self::Cmpeq => (0b100, 0),
            Self::Cmpne => (0b100, 1),
        }
    }

    /// The selector bits OR-ed into the base.
    pub fn encoding_bits(self) -> u32 {
        let (hi, lo) = self.bits();
        (hi << 13) | (lo << 4)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Cmpge => "cmpge",
            Self::Cmpgt => "cmpgt",
            Self::Cmplt => "cmplt",
            Self::Cmple => "cmple",
            Self::Cmpeq => "cmpeq",
            Self::Cmpne => "cmpne",
        }
    }

    /// Recover the op from a word's selector bits, if a modeled op.
    pub fn from_word(word: u32) -> Option<Self> {
        let key = ((word >> 13) & 0b111, (word >> 4) & 1);
        Self::ALL.into_iter().find(|op| op.bits() == key)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 6] = [
        Self::Cmpge,
        Self::Cmpgt,
        Self::Cmplt,
        Self::Cmple,
        Self::Cmpeq,
        Self::Cmpne,
    ];
}

/// An SVE **integer compare with unsigned immediate** op (DDI0487 part C): `<op> Pd.<T>, Pg/Z, Zn.<T>, #imm7`,
/// where `imm7` is `0..=127`. Selected by `lt[13]` and `ne[4]`, over base `0x2420_0000` (bit21 = 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveCmpImmUnsignedOp {
    /// `CMPHS` -- unsigned higher-or-same.
    Cmphs,
    /// `CMPHI` -- unsigned higher.
    Cmphi,
    /// `CMPLO` -- unsigned lower.
    Cmplo,
    /// `CMPLS` -- unsigned lower-or-same.
    Cmpls,
}

impl Arm64SveCmpImmUnsignedOp {
    /// `(lt[13], ne[4])`.
    fn bits(self) -> (u32, u32) {
        match self {
            Self::Cmphs => (0, 0),
            Self::Cmphi => (0, 1),
            Self::Cmplo => (1, 0),
            Self::Cmpls => (1, 1),
        }
    }

    /// The selector bits OR-ed into the base.
    pub fn encoding_bits(self) -> u32 {
        let (lt, ne) = self.bits();
        (lt << 13) | (ne << 4)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Cmphs => "cmphs",
            Self::Cmphi => "cmphi",
            Self::Cmplo => "cmplo",
            Self::Cmpls => "cmpls",
        }
    }

    /// Recover the op from a word's selector bits.
    pub fn from_word(word: u32) -> Self {
        match (((word >> 13) & 1), ((word >> 4) & 1)) {
            (0, 0) => Self::Cmphs,
            (0, _) => Self::Cmphi,
            (_, 0) => Self::Cmplo,
            (_, _) => Self::Cmpls,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Cmphs, Self::Cmphi, Self::Cmplo, Self::Cmpls];
}
