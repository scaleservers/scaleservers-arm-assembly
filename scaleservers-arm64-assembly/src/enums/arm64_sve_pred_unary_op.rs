// Copyright (c) Scaleservers LLC

/// An SVE **predicated integer unary** op over `Zd.<T>, Pg/M, Zn.<T>` (DDI0487 part C -- "SVE integer unary
/// operations (predicated)"; the sign/zero-extends and `FABS`/`FNEG` share the group). All share the base
/// `0x0400_A000 | size<<22 | opcode<<16 | Pg<<10 | Zn<<5 | Zd`, with the op at `[21:16]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SvePredUnaryOp {
    /// `SXTB` -- sign-extend byte (`.h`/`.s`/`.d`).
    Sxtb,
    /// `UXTB` -- zero-extend byte.
    Uxtb,
    /// `SXTH` -- sign-extend halfword (`.s`/`.d`).
    Sxth,
    /// `UXTH` -- zero-extend halfword.
    Uxth,
    /// `SXTW` -- sign-extend word (`.d`).
    Sxtw,
    /// `UXTW` -- zero-extend word.
    Uxtw,
    /// `ABS` -- absolute value.
    Abs,
    /// `NEG` -- negate.
    Neg,
    /// `CLS` -- count leading sign bits.
    Cls,
    /// `CLZ` -- count leading zero bits.
    Clz,
    /// `CNT` -- count set bits per element.
    Cnt,
    /// `CNOT` -- logical NOT of each element (`0` -> `1`, nonzero -> `0`).
    Cnot,
    /// `FABS` -- floating-point absolute value.
    Fabs,
    /// `FNEG` -- floating-point negate.
    Fneg,
    /// `NOT` -- bitwise NOT.
    Not,
}

impl Arm64SvePredUnaryOp {
    /// The 6-bit `opcode` field (`[21:16]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Sxtb => 0x10,
            Self::Uxtb => 0x11,
            Self::Sxth => 0x12,
            Self::Uxth => 0x13,
            Self::Sxtw => 0x14,
            Self::Uxtw => 0x15,
            Self::Abs => 0x16,
            Self::Neg => 0x17,
            Self::Cls => 0x18,
            Self::Clz => 0x19,
            Self::Cnt => 0x1A,
            Self::Cnot => 0x1B,
            Self::Fabs => 0x1C,
            Self::Fneg => 0x1D,
            Self::Not => 0x1E,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sxtb => "sxtb",
            Self::Uxtb => "uxtb",
            Self::Sxth => "sxth",
            Self::Uxth => "uxth",
            Self::Sxtw => "sxtw",
            Self::Uxtw => "uxtw",
            Self::Abs => "abs",
            Self::Neg => "neg",
            Self::Cls => "cls",
            Self::Clz => "clz",
            Self::Cnt => "cnt",
            Self::Cnot => "cnot",
            Self::Fabs => "fabs",
            Self::Fneg => "fneg",
            Self::Not => "not",
        }
    }

    /// Recover the op from its `opcode`, if a modeled op (`0x1F` is reserved).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0x3F)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 15] = [
        Self::Sxtb,
        Self::Uxtb,
        Self::Sxth,
        Self::Uxth,
        Self::Sxtw,
        Self::Uxtw,
        Self::Abs,
        Self::Neg,
        Self::Cls,
        Self::Clz,
        Self::Cnt,
        Self::Cnot,
        Self::Fabs,
        Self::Fneg,
        Self::Not,
    ];
}
