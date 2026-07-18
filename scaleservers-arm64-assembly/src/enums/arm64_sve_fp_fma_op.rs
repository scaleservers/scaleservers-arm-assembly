// Copyright (c) Scaleservers LLC

/// An SVE **predicated floating-point fused multiply-add** op (DDI0487 part C). All take the operand shape
/// `<op> Zda.<T>, Pg/M, Zn.<T>, Zm.<T>` (the first register is the destination/accumulator). `FMLA`/`FMLS`/`FNMLA`/
/// `FNMLS` accumulate into `Zda`; `FMAD`/`FMSB`/`FNMAD`/`FNMSB` are the multiplicand-destructive forms. Base
/// `0x6520_0000 | size<<22 | Zm<<16 | opcode<<13 | Pg<<10 | Zn<<5 | Zda`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpFmaOp {
    /// `FMLA` -- `Zda += Zn * Zm`.
    Fmla,
    /// `FMLS` -- `Zda += -Zn * Zm`.
    Fmls,
    /// `FNMLA` -- `Zda = -Zda + -Zn * Zm`.
    Fnmla,
    /// `FNMLS` -- `Zda = -Zda + Zn * Zm`.
    Fnmls,
    /// `FMAD` -- `Zdn = Za + Zn * Zdn` (multiplicand-destructive).
    Fmad,
    /// `FMSB` -- `Zdn = Za + -Zn * Zdn`.
    Fmsb,
    /// `FNMAD` -- `Zdn = -Za + -Zn * Zdn`.
    Fnmad,
    /// `FNMSB` -- `Zdn = -Za + Zn * Zdn`.
    Fnmsb,
}

impl Arm64SveFpFmaOp {
    /// The 3-bit `opcode` field (`[15:13]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Fmla => 0,
            Self::Fmls => 1,
            Self::Fnmla => 2,
            Self::Fnmls => 3,
            Self::Fmad => 4,
            Self::Fmsb => 5,
            Self::Fnmad => 6,
            Self::Fnmsb => 7,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fmla => "fmla",
            Self::Fmls => "fmls",
            Self::Fnmla => "fnmla",
            Self::Fnmls => "fnmls",
            Self::Fmad => "fmad",
            Self::Fmsb => "fmsb",
            Self::Fnmad => "fnmad",
            Self::Fnmsb => "fnmsb",
        }
    }

    /// Recover the op from its `opcode`.
    pub fn from_opcode(opcode: u32) -> Self {
        Self::ALL[(opcode & 0x7) as usize]
    }

    /// Every op, indexed by `opcode`.
    pub const ALL: [Self; 8] = [
        Self::Fmla,
        Self::Fmls,
        Self::Fnmla,
        Self::Fnmls,
        Self::Fmad,
        Self::Fmsb,
        Self::Fnmad,
        Self::Fnmsb,
    ];
}
