// Copyright (c) Scaleservers LLC

/// An SVE **count-active-predicate inc/dec** op (DDI0487 part C): add (`INCP`) or subtract (`DECP`) the number of
/// active elements in `Pm.<T>`, optionally saturating (`SQINCP`/`SQDECP` signed, `UQINCP`/`UQDECP` unsigned). The
/// destination is a scalar GP register or an SVE vector (two separate encodings). Note the discriminant bits move
/// between the non-saturating form (inc/dec at `[16]`) and the saturating form (`D` at `[17]`, `U` at `[16]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SvePredCountOp {
    /// `INCP` -- increment by the active count.
    Incp,
    /// `DECP` -- decrement by the active count.
    Decp,
    /// `SQINCP` -- signed saturating increment.
    Sqincp,
    /// `SQDECP` -- signed saturating decrement.
    Sqdecp,
    /// `UQINCP` -- unsigned saturating increment.
    Uqincp,
    /// `UQDECP` -- unsigned saturating decrement.
    Uqdecp,
}

impl Arm64SvePredCountOp {
    /// Whether this is a saturating form (`SQ`/`UQ`), which uses the `[17]=D`,`[16]=U` discriminant + an `sf` width
    /// bit for the scalar form; the plain `INCP`/`DECP` use `[16]=D` and are always 64-bit in the scalar form.
    pub fn is_saturating(self) -> bool {
        matches!(
            self,
            Self::Sqincp | Self::Sqdecp | Self::Uqincp | Self::Uqdecp
        )
    }

    /// The decrement bit (`DECP`/`SQDECP`/`UQDECP`).
    pub fn decrement(self) -> bool {
        matches!(self, Self::Decp | Self::Sqdecp | Self::Uqdecp)
    }

    /// The unsigned bit (`UQINCP`/`UQDECP`); only meaningful for the saturating forms.
    pub fn unsigned(self) -> bool {
        matches!(self, Self::Uqincp | Self::Uqdecp)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Incp => "incp",
            Self::Decp => "decp",
            Self::Sqincp => "sqincp",
            Self::Sqdecp => "sqdecp",
            Self::Uqincp => "uqincp",
            Self::Uqdecp => "uqdecp",
        }
    }

    /// Recover the op from `(saturating, decrement, unsigned)`.
    pub fn from_bits(saturating: bool, decrement: bool, unsigned: bool) -> Self {
        match (saturating, decrement, unsigned) {
            (false, false, _) => Self::Incp,
            (false, true, _) => Self::Decp,
            (true, false, false) => Self::Sqincp,
            (true, true, false) => Self::Sqdecp,
            (true, false, true) => Self::Uqincp,
            (true, true, true) => Self::Uqdecp,
        }
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 6] = [
        Self::Incp,
        Self::Decp,
        Self::Sqincp,
        Self::Sqdecp,
        Self::Uqincp,
        Self::Uqdecp,
    ];
}
