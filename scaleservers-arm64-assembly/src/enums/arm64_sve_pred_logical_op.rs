// Copyright (c) Scaleservers LLC

/// An SVE **predicate logical** op over `Pd.B, Pg/Z, Pn.B, Pm.B` (DDI0487 part C). The predicates are always
/// byte-element. The op is selected by four encoding bits -- `op[23]`, `S[22]` (flag-setting), `[9]`, `[4]` -- over
/// the shared base `0x2500_4000`. `SEL` is the odd one out: it is a merging select (`Pg`, no `/z`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SvePredLogicalOp {
    /// `AND` -- `Pn & Pm`.
    And,
    /// `BIC` -- `Pn & ~Pm`.
    Bic,
    /// `EOR` -- `Pn ^ Pm`.
    Eor,
    /// `SEL` -- merging select (`Pg ? Pn : Pm`); uses `Pg` (not `/z`).
    Sel,
    /// `ORR` -- `Pn | Pm`.
    Orr,
    /// `ORN` -- `Pn | ~Pm`.
    Orn,
    /// `NOR` -- `~(Pn | Pm)`.
    Nor,
    /// `NAND` -- `~(Pn & Pm)`.
    Nand,
    /// `ANDS` -- `AND`, setting the condition flags.
    Ands,
    /// `BICS` -- `BIC`, setting the condition flags.
    Bics,
    /// `EORS` -- `EOR`, setting the condition flags.
    Eors,
    /// `ORRS` -- `ORR`, setting the condition flags.
    Orrs,
    /// `ORNS` -- `ORN`, setting the condition flags.
    Orns,
    /// `NORS` -- `NOR`, setting the condition flags.
    Nors,
    /// `NANDS` -- `NAND`, setting the condition flags.
    Nands,
}

impl Arm64SvePredLogicalOp {
    /// The four selector bits `(op[23], S[22], bit[9], bit[4])`.
    fn bits(self) -> (u32, u32, u32, u32) {
        match self {
            Self::And => (0, 0, 0, 0),
            Self::Bic => (0, 0, 0, 1),
            Self::Eor => (0, 0, 1, 0),
            Self::Sel => (0, 0, 1, 1),
            Self::Orr => (1, 0, 0, 0),
            Self::Orn => (1, 0, 0, 1),
            Self::Nor => (1, 0, 1, 0),
            Self::Nand => (1, 0, 1, 1),
            Self::Ands => (0, 1, 0, 0),
            Self::Bics => (0, 1, 0, 1),
            Self::Eors => (0, 1, 1, 0),
            Self::Orrs => (1, 1, 0, 0),
            Self::Orns => (1, 1, 0, 1),
            Self::Nors => (1, 1, 1, 0),
            Self::Nands => (1, 1, 1, 1),
        }
    }

    /// The selector bits OR-ed into the base word.
    pub fn encoding_bits(self) -> u32 {
        let (op, s, b9, b4) = self.bits();
        (op << 23) | (s << 22) | (b9 << 9) | (b4 << 4)
    }

    /// Whether the op uses the zeroing qualifier (`Pg/Z`). Only `SEL` does not (it merges).
    pub fn uses_zeroing(self) -> bool {
        !matches!(self, Self::Sel)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Bic => "bic",
            Self::Eor => "eor",
            Self::Sel => "sel",
            Self::Orr => "orr",
            Self::Orn => "orn",
            Self::Nor => "nor",
            Self::Nand => "nand",
            Self::Ands => "ands",
            Self::Bics => "bics",
            Self::Eors => "eors",
            Self::Orrs => "orrs",
            Self::Orns => "orns",
            Self::Nors => "nors",
            Self::Nands => "nands",
        }
    }

    /// Recover the op from a word's selector bits, if a modeled op.
    pub fn from_word(word: u32) -> Option<Self> {
        let key = (
            (word >> 23) & 1,
            (word >> 22) & 1,
            (word >> 9) & 1,
            (word >> 4) & 1,
        );
        Self::ALL.into_iter().find(|op| op.bits() == key)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 15] = [
        Self::And,
        Self::Bic,
        Self::Eor,
        Self::Sel,
        Self::Orr,
        Self::Orn,
        Self::Nor,
        Self::Nand,
        Self::Ands,
        Self::Bics,
        Self::Eors,
        Self::Orrs,
        Self::Orns,
        Self::Nors,
        Self::Nands,
    ];
}
