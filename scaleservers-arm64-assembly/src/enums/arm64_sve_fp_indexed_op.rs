// Copyright (c) Scaleservers LLC

/// An SVE **floating-point by-indexed-element** op (DDI0487 part C): `<op> Zd.<T>, Zn.<T>, Zm.<T>[index]`, where one
/// lane of `Zm` (within `Z0..Z7` for `.h`/`.s`, `Z0..Z15` for `.d`) is broadcast against every lane of `Zn`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpIndexedOp {
    /// `FMLA` -- multiply-accumulate (`[15:13]`=000, `[10]`=0).
    Fmla,
    /// `FMLS` -- multiply-subtract (`[15:13]`=000, `[10]`=1).
    Fmls,
    /// `FMUL` -- multiply (`[15:13]`=001, `[10]`=0).
    Fmul,
}

impl Arm64SveFpIndexedOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fmla => "fmla",
            Self::Fmls => "fmls",
            Self::Fmul => "fmul",
        }
    }

    /// The `[15:13]` opcode group (0 for the `FMLA`/`FMLS` accumulate pair, 1 for `FMUL`).
    pub fn opc(self) -> u32 {
        match self {
            Self::Fmla | Self::Fmls => 0b000,
            Self::Fmul => 0b001,
        }
    }

    /// The `[10]` subtract bit (set only for `FMLS`).
    pub fn sub_bit(self) -> u32 {
        (matches!(self, Self::Fmls)) as u32
    }

    /// Recover the op from its `[15:13]` opcode and `[10]` subtract bit, or `None` for an unallocated combination.
    pub fn from_bits(opc: u32, sub: u32) -> Option<Self> {
        match (opc & 0b111, sub & 1) {
            (0b000, 0) => Some(Self::Fmla),
            (0b000, 1) => Some(Self::Fmls),
            (0b001, 0) => Some(Self::Fmul),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Fmla, Self::Fmls, Self::Fmul];
}
