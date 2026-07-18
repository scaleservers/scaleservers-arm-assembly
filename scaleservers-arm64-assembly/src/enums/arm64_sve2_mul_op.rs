// Copyright (c) Scaleservers LLC

/// An SVE2 **unpredicated integer multiply** op over `Zd.<T>, Zn.<T>, Zm.<T>` (DDI0487 part C). Shares the base
/// `0x0420_6000 | size<<22 | Zm<<16 | opcode<<10 | Zn<<5 | Zd` (`[15:13]`=011); `PMUL` is `.b` only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2MulOp {
    /// `MUL` -- multiply, low half (`[12:10]`=000).
    Mul,
    /// `PMUL` -- polynomial multiply (`[12:10]`=001), `.b` only.
    Pmul,
    /// `SMULH` -- signed multiply, high half (`[12:10]`=010).
    Smulh,
    /// `UMULH` -- unsigned multiply, high half (`[12:10]`=011).
    Umulh,
}

impl Arm64Sve2MulOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Mul => "mul",
            Self::Pmul => "pmul",
            Self::Smulh => "smulh",
            Self::Umulh => "umulh",
        }
    }

    /// The 3-bit `[12:10]` opcode.
    pub fn opcode(self) -> u32 {
        match self {
            Self::Mul => 0b000,
            Self::Pmul => 0b001,
            Self::Smulh => 0b010,
            Self::Umulh => 0b011,
        }
    }

    /// Recover the op from its `[12:10]` opcode, or `None` if unallocated in this group (`100`..`111`).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        match opcode & 0b111 {
            0b000 => Some(Self::Mul),
            0b001 => Some(Self::Pmul),
            0b010 => Some(Self::Smulh),
            0b011 => Some(Self::Umulh),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Mul, Self::Pmul, Self::Smulh, Self::Umulh];
}
