// Copyright (c) Scaleservers LLC

/// An SVE **predicated floating-point unary** op over `Zd.<T>, Pg/M, Zn.<T>` (DDI0487 part C -- the same-element-size
/// rounding / reciprocal / square-root forms of the SVE FP unary group). Base
/// `0x6500_A000 | size<<22 | opcode<<16 | Pg<<10 | Zn<<5 | Zd`, opcode at `[21:16]`. Valid for `.h`/`.s`/`.d`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpUnaryOp {
    /// `FRINTN` -- round to nearest, ties to even.
    Frintn,
    /// `FRINTP` -- round toward +inf.
    Frintp,
    /// `FRINTM` -- round toward -inf.
    Frintm,
    /// `FRINTZ` -- round toward zero.
    Frintz,
    /// `FRINTA` -- round to nearest, ties away.
    Frinta,
    /// `FRINTX` -- round to integral, signalling inexact.
    Frintx,
    /// `FRINTI` -- round using the current mode.
    Frinti,
    /// `FRECPX` -- reciprocal exponent.
    Frecpx,
    /// `FSQRT` -- square root.
    Fsqrt,
}

impl Arm64SveFpUnaryOp {
    /// The 6-bit `opcode` field (`[21:16]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Frintn => 0x00,
            Self::Frintp => 0x01,
            Self::Frintm => 0x02,
            Self::Frintz => 0x03,
            Self::Frinta => 0x04,
            Self::Frintx => 0x06,
            Self::Frinti => 0x07,
            Self::Frecpx => 0x0C,
            Self::Fsqrt => 0x0D,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Frintn => "frintn",
            Self::Frintp => "frintp",
            Self::Frintm => "frintm",
            Self::Frintz => "frintz",
            Self::Frinta => "frinta",
            Self::Frintx => "frintx",
            Self::Frinti => "frinti",
            Self::Frecpx => "frecpx",
            Self::Fsqrt => "fsqrt",
        }
    }

    /// Recover the op from its `opcode`, if a modeled same-size unary (the convert opcodes are not in this set).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0x3F)
    }

    /// The FEAT_SVE2p2 **zeroing** (`/z`) encoding fields `([21:16] opcode6, [14:13] sub2)` in the relocated
    /// `0x6400_8000` frame. The rounding-mode `FRINT*` ops pack n/p/m/z (and a/x/i) into `sub2`.
    pub fn zeroing_fields(self) -> (u32, u32) {
        match self {
            Self::Frintn => (0x18, 0b00),
            Self::Frintp => (0x18, 0b01),
            Self::Frintm => (0x18, 0b10),
            Self::Frintz => (0x18, 0b11),
            Self::Frinta => (0x19, 0b00),
            Self::Frintx => (0x19, 0b10),
            Self::Frinti => (0x19, 0b11),
            Self::Frecpx => (0x1B, 0b00),
            Self::Fsqrt => (0x1B, 0b01),
        }
    }

    /// Recover the op from its zeroing `([21:16], [14:13])` fields, or `None` for an unallocated pair (or a
    /// convert opcode, which is handled by the FP-convert group).
    pub fn from_zeroing_fields(opcode6: u32, sub2: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.zeroing_fields() == (opcode6 & 0x3F, sub2 & 0b11))
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 9] = [
        Self::Frintn,
        Self::Frintp,
        Self::Frintm,
        Self::Frintz,
        Self::Frinta,
        Self::Frintx,
        Self::Frinti,
        Self::Frecpx,
        Self::Fsqrt,
    ];
}
