// Copyright (c) Scaleservers LLC

/// A NEON **rounding double multiply accumulate** op (DDI0487 C7, FEAT_RDM): `SQRDMLAH` accumulates, `SQRDMLSH`
/// subtracts, the signed saturating rounding doubling multiply-high. Available in a vector form
/// (`Vd.<arr>, Vn.<arr>, Vm.<arr>`) and a by-element form (`..., Vm.<ts>[index]`); the accumulator is `.4h`/`.8h`/
/// `.2s`/`.4s` in both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorRdmOp {
    /// `SQRDMLAH` -- signed saturating rounding doubling multiply-high, accumulating.
    Sqrdmlah,
    /// `SQRDMLSH` -- signed saturating rounding doubling multiply-high, subtracting.
    Sqrdmlsh,
}

impl Arm64VectorRdmOp {
    /// The vector-form base (three-same-extra, `[21]=0`); the arrangement supplies `Q<<30` and `size<<22`, the
    /// registers `Vm<<16 | Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn vector_base(self) -> u32 {
        match self {
            Self::Sqrdmlah => 0x2E00_8400,
            Self::Sqrdmlsh => 0x2E00_8C00,
        }
    }

    /// The by-element base (the by-element encoding, opcodes `1101`/`1111` with `U = 1`); the arrangement
    /// supplies `Q<<30` and `size<<22`, the index+`Vm` the `L`/`M`/`Rm`/`H` fields.
    pub fn by_element_base(self) -> u32 {
        match self {
            Self::Sqrdmlah => 0x2F00_D000,
            Self::Sqrdmlsh => 0x2F00_F000,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sqrdmlah => "sqrdmlah",
            Self::Sqrdmlsh => "sqrdmlsh",
        }
    }

    /// Both operations, for decode dispatch.
    pub const ALL: [Self; 2] = [Self::Sqrdmlah, Self::Sqrdmlsh];
}
