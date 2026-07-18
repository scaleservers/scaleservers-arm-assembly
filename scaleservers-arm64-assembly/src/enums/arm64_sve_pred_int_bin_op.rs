// Copyright (c) Scaleservers LLC

/// An SVE **predicated integer binary** op over the destructive form `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>`
/// (DDI0487 part C -- "SVE integer binary arithmetic (predicated)" + the predicated bitwise-logical group, which
/// shares the encoding). `Zdn` is both destination and first source; `Pg` (P0..P7) governs which elements update.
/// All share the base `0x0400_0000 | size<<22 | opcode<<16 | Pg<<10 | Zm<<5 | Zdn`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SvePredIntBinOp {
    /// `ADD` -- add.
    Add,
    /// `SUB` -- subtract.
    Sub,
    /// `SUBR` -- reversed subtract (`Zm - Zdn`).
    Subr,
    /// `SMAX` -- signed maximum.
    Smax,
    /// `UMAX` -- unsigned maximum.
    Umax,
    /// `SMIN` -- signed minimum.
    Smin,
    /// `UMIN` -- unsigned minimum.
    Umin,
    /// `SABD` -- signed absolute difference.
    Sabd,
    /// `UABD` -- unsigned absolute difference.
    Uabd,
    /// `MUL` -- multiply (low half).
    Mul,
    /// `SMULH` -- signed multiply, high half.
    Smulh,
    /// `UMULH` -- unsigned multiply, high half.
    Umulh,
    /// `SDIV` -- signed divide (`.s`/`.d` only).
    Sdiv,
    /// `UDIV` -- unsigned divide (`.s`/`.d` only).
    Udiv,
    /// `SDIVR` -- reversed signed divide.
    Sdivr,
    /// `UDIVR` -- reversed unsigned divide.
    Udivr,
    /// `ORR` -- bitwise OR.
    Orr,
    /// `EOR` -- bitwise exclusive-OR.
    Eor,
    /// `AND` -- bitwise AND.
    And,
    /// `BIC` -- bitwise AND-NOT (`Zdn & ~Zm`).
    Bic,
}

impl Arm64SvePredIntBinOp {
    /// The 5-bit `opcode` field (`[20:16]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Add => 0,
            Self::Sub => 1,
            Self::Subr => 3,
            Self::Smax => 8,
            Self::Umax => 9,
            Self::Smin => 10,
            Self::Umin => 11,
            Self::Sabd => 12,
            Self::Uabd => 13,
            Self::Mul => 16,
            Self::Smulh => 18,
            Self::Umulh => 19,
            Self::Sdiv => 20,
            Self::Udiv => 21,
            Self::Sdivr => 22,
            Self::Udivr => 23,
            Self::Orr => 24,
            Self::Eor => 25,
            Self::And => 26,
            Self::Bic => 27,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Subr => "subr",
            Self::Smax => "smax",
            Self::Umax => "umax",
            Self::Smin => "smin",
            Self::Umin => "umin",
            Self::Sabd => "sabd",
            Self::Uabd => "uabd",
            Self::Mul => "mul",
            Self::Smulh => "smulh",
            Self::Umulh => "umulh",
            Self::Sdiv => "sdiv",
            Self::Udiv => "udiv",
            Self::Sdivr => "sdivr",
            Self::Udivr => "udivr",
            Self::Orr => "orr",
            Self::Eor => "eor",
            Self::And => "and",
            Self::Bic => "bic",
        }
    }

    /// Recover the op from its `opcode` field, if it is one of the modeled ops.
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0x1F)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 20] = [
        Self::Add,
        Self::Sub,
        Self::Subr,
        Self::Smax,
        Self::Umax,
        Self::Smin,
        Self::Umin,
        Self::Sabd,
        Self::Uabd,
        Self::Mul,
        Self::Smulh,
        Self::Umulh,
        Self::Sdiv,
        Self::Udiv,
        Self::Sdivr,
        Self::Udivr,
        Self::Orr,
        Self::Eor,
        Self::And,
        Self::Bic,
    ];
}
