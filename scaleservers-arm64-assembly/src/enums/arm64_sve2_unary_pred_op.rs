// Copyright (c) Scaleservers LLC

/// An SVE2 **integer unary (predicated, merging)** op (DDI0487 C8): `<op> Zd.<T>, Pg/M, Zn.<T>`. Selected by `Q`
/// (`[19]`) and `op` (`[16]`), with the merging-predication bits fixed. `URECPE`/`URSQRTE` (unsigned reciprocal /
/// reciprocal-square-root estimate) operate on `.s` only; `SQABS`/`SQNEG` take any element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2UnaryPredOp {
    /// `URECPE` -- unsigned reciprocal estimate (`[19:16]`=0000), `.s` only.
    Urecpe,
    /// `URSQRTE` -- unsigned reciprocal square-root estimate (`[19:16]`=0001), `.s` only.
    Ursqrte,
    /// `SQABS` -- signed saturating absolute value (`[19:16]`=1000).
    Sqabs,
    /// `SQNEG` -- signed saturating negate (`[19:16]`=1001).
    Sqneg,
}

impl Arm64Sve2UnaryPredOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Urecpe => "urecpe",
            Self::Ursqrte => "ursqrte",
            Self::Sqabs => "sqabs",
            Self::Sqneg => "sqneg",
        }
    }

    /// The 4-bit `[19:16]` opcode (`Q:0:0:op`, merging).
    pub fn code(self) -> u32 {
        match self {
            Self::Urecpe => 0b0000,
            Self::Ursqrte => 0b0001,
            Self::Sqabs => 0b1000,
            Self::Sqneg => 0b1001,
        }
    }

    /// Whether the op is restricted to the `.s` element (`URECPE`/`URSQRTE`).
    pub fn is_single_only(self) -> bool {
        matches!(self, Self::Urecpe | Self::Ursqrte)
    }

    /// Recover the op from its `[19:16]` code, or `None` for an unallocated (or non-merging) value.
    pub fn from_code(code: u32) -> Option<Self> {
        match code & 0xF {
            0b0000 => Some(Self::Urecpe),
            0b0001 => Some(Self::Ursqrte),
            0b1000 => Some(Self::Sqabs),
            0b1001 => Some(Self::Sqneg),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Urecpe, Self::Ursqrte, Self::Sqabs, Self::Sqneg];
}
