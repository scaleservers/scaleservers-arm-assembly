// Copyright (c) Scaleservers LLC

/// An SVE **predicated floating-point binary** op over the destructive form `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>`
/// (DDI0487 part C -- "SVE floating-point arithmetic (predicated)"). `Zdn` is both destination and first source.
/// All share the base `0x6500_8000 | size<<22 | opcode<<16 | Pg<<10 | Zm<<5 | Zdn`. Valid for `.h`/`.s`/`.d`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpPredBinOp {
    /// `FADD` -- add.
    Fadd,
    /// `FSUB` -- subtract.
    Fsub,
    /// `FMUL` -- multiply.
    Fmul,
    /// `FSUBR` -- reversed subtract (`Zm - Zdn`).
    Fsubr,
    /// `FMAXNM` -- maximum-number (NaN-quieting).
    Fmaxnm,
    /// `FMINNM` -- minimum-number.
    Fminnm,
    /// `FMAX` -- maximum.
    Fmax,
    /// `FMIN` -- minimum.
    Fmin,
    /// `FABD` -- absolute difference.
    Fabd,
    /// `FSCALE` -- scale by 2^(int in Zm).
    Fscale,
    /// `FMULX` -- multiply extended.
    Fmulx,
    /// `FDIVR` -- reversed divide (`Zm / Zdn`).
    Fdivr,
    /// `FDIV` -- divide.
    Fdiv,
    /// `FAMAX` -- absolute maximum (FEAT_FAMINMAX).
    Famax,
    /// `FAMIN` -- absolute minimum (FEAT_FAMINMAX).
    Famin,
}

impl Arm64SveFpPredBinOp {
    /// The `opcode` field (`[20:16]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Fadd => 0,
            Self::Fsub => 1,
            Self::Fmul => 2,
            Self::Fsubr => 3,
            Self::Fmaxnm => 4,
            Self::Fminnm => 5,
            Self::Fmax => 6,
            Self::Fmin => 7,
            Self::Fabd => 8,
            Self::Fscale => 9,
            Self::Fmulx => 10,
            Self::Fdivr => 12,
            Self::Fdiv => 13,
            Self::Famax => 14,
            Self::Famin => 15,
        }
    }

    /// Whether this is a FEAT_FAMINMAX op (`FAMAX`/`FAMIN`), which gates on FEAT_FAMINMAX rather than plain SVE.
    pub fn is_faminmax(self) -> bool {
        matches!(self, Self::Famax | Self::Famin)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fadd => "fadd",
            Self::Fsub => "fsub",
            Self::Fmul => "fmul",
            Self::Fsubr => "fsubr",
            Self::Fmaxnm => "fmaxnm",
            Self::Fminnm => "fminnm",
            Self::Fmax => "fmax",
            Self::Fmin => "fmin",
            Self::Fabd => "fabd",
            Self::Fscale => "fscale",
            Self::Fmulx => "fmulx",
            Self::Fdivr => "fdivr",
            Self::Fdiv => "fdiv",
            Self::Famax => "famax",
            Self::Famin => "famin",
        }
    }

    /// Recover the op from its `opcode` field, if a modeled op (`11`/`14`/`15` are unallocated here).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0x1F)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 15] = [
        Self::Fadd,
        Self::Fsub,
        Self::Fmul,
        Self::Fsubr,
        Self::Fmaxnm,
        Self::Fminnm,
        Self::Fmax,
        Self::Fmin,
        Self::Fabd,
        Self::Fscale,
        Self::Fmulx,
        Self::Fdivr,
        Self::Fdiv,
        Self::Famax,
        Self::Famin,
    ];
}
