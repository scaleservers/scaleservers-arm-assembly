// Copyright (c) Scaleservers LLC

/// An SVE **integer reduction** op (DDI0487 part C). Reduces a vector to a single SIMD&FP scalar under a governing
/// predicate. `SADDV`/`UADDV` accumulate into a 64-bit (`Dn`) result; the others produce an element-sized scalar.
/// Base `0x0400_2000 | size<<22 | opcode<<16 | Pg<<10 | Zn<<5 | Vd`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveIntReductionOp {
    /// `SADDV` -- signed add reduction (64-bit accumulator).
    Saddv,
    /// `UADDV` -- unsigned add reduction (64-bit accumulator).
    Uaddv,
    /// `SMAXV` -- signed maximum reduction.
    Smaxv,
    /// `UMAXV` -- unsigned maximum reduction.
    Umaxv,
    /// `SMINV` -- signed minimum reduction.
    Sminv,
    /// `UMINV` -- unsigned minimum reduction.
    Uminv,
    /// `ORV` -- bitwise OR reduction.
    Orv,
    /// `EORV` -- bitwise exclusive-OR reduction.
    Eorv,
    /// `ANDV` -- bitwise AND reduction.
    Andv,
}

impl Arm64SveIntReductionOp {
    /// The 5-bit `opcode` field (`[20:16]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Saddv => 0x00,
            Self::Uaddv => 0x01,
            Self::Smaxv => 0x08,
            Self::Umaxv => 0x09,
            Self::Sminv => 0x0A,
            Self::Uminv => 0x0B,
            Self::Orv => 0x18,
            Self::Eorv => 0x19,
            Self::Andv => 0x1A,
        }
    }

    /// `SADDV`/`UADDV` always reduce into a 64-bit (`Dn`) accumulator regardless of the element size.
    pub fn result_is_doubleword(self) -> bool {
        matches!(self, Self::Saddv | Self::Uaddv)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Saddv => "saddv",
            Self::Uaddv => "uaddv",
            Self::Smaxv => "smaxv",
            Self::Umaxv => "umaxv",
            Self::Sminv => "sminv",
            Self::Uminv => "uminv",
            Self::Orv => "orv",
            Self::Eorv => "eorv",
            Self::Andv => "andv",
        }
    }

    /// Recover the op from its `opcode`, if a modeled op.
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0x1F)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 9] = [
        Self::Saddv,
        Self::Uaddv,
        Self::Smaxv,
        Self::Umaxv,
        Self::Sminv,
        Self::Uminv,
        Self::Orv,
        Self::Eorv,
        Self::Andv,
    ];
}

/// An SVE **floating-point reduction** op (DDI0487 part C). Reduces a vector to an element-sized SIMD&FP scalar.
/// Base `0x6500_2000 | size<<22 | opcode<<16 | Pg<<10 | Zn<<5 | Vd`. Valid for `.h`/`.s`/`.d`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpReductionOp {
    /// `FADDV` -- in-order floating-point add reduction.
    Faddv,
    /// `FMAXNMV` -- maximum-number reduction.
    Fmaxnmv,
    /// `FMINNMV` -- minimum-number reduction.
    Fminnmv,
    /// `FMAXV` -- maximum reduction.
    Fmaxv,
    /// `FMINV` -- minimum reduction.
    Fminv,
}

impl Arm64SveFpReductionOp {
    /// The 5-bit `opcode` field (`[20:16]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Faddv => 0x00,
            Self::Fmaxnmv => 0x04,
            Self::Fminnmv => 0x05,
            Self::Fmaxv => 0x06,
            Self::Fminv => 0x07,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Faddv => "faddv",
            Self::Fmaxnmv => "fmaxnmv",
            Self::Fminnmv => "fminnmv",
            Self::Fmaxv => "fmaxv",
            Self::Fminv => "fminv",
        }
    }

    /// Recover the op from its `opcode`, if a modeled op.
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0x1F)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 5] = [
        Self::Faddv,
        Self::Fmaxnmv,
        Self::Fminnmv,
        Self::Fmaxv,
        Self::Fminv,
    ];
}
