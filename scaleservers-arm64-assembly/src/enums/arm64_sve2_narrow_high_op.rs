// Copyright (c) Scaleservers LLC

/// An SVE2 **add/subtract narrow high** op (DDI0487 part C): `<op> Zd.<Tn>, Zn.<T>, Zm.<T>`, taking the high half of
/// each `size`-element add/subtract result into the narrower `Zd`. `B`/`T` write the even/odd result lanes;
/// `R`-prefixed variants round. Base `0x4520_6000 | size<<22 | Zm<<16 | opcode<<10 | Zn<<5 | Zd` (`[15:13]`=011).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2NarrowHighOp {
    Addhnb,
    Addhnt,
    Raddhnb,
    Raddhnt,
    Subhnb,
    Subhnt,
    Rsubhnb,
    Rsubhnt,
}

impl Arm64Sve2NarrowHighOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Addhnb => "addhnb",
            Self::Addhnt => "addhnt",
            Self::Raddhnb => "raddhnb",
            Self::Raddhnt => "raddhnt",
            Self::Subhnb => "subhnb",
            Self::Subhnt => "subhnt",
            Self::Rsubhnb => "rsubhnb",
            Self::Rsubhnt => "rsubhnt",
        }
    }

    /// The 3-bit `[12:10]` opcode.
    pub fn opcode(self) -> u32 {
        match self {
            Self::Addhnb => 0b000,
            Self::Addhnt => 0b001,
            Self::Raddhnb => 0b010,
            Self::Raddhnt => 0b011,
            Self::Subhnb => 0b100,
            Self::Subhnt => 0b101,
            Self::Rsubhnb => 0b110,
            Self::Rsubhnt => 0b111,
        }
    }

    /// Recover the op from its `[12:10]` opcode.
    pub fn from_opcode(opcode: u32) -> Self {
        Self::ALL[(opcode & 0b111) as usize]
    }

    /// Every op, indexed by opcode.
    pub const ALL: [Self; 8] = [
        Self::Addhnb,
        Self::Addhnt,
        Self::Raddhnb,
        Self::Raddhnt,
        Self::Subhnb,
        Self::Subhnt,
        Self::Rsubhnb,
        Self::Rsubhnt,
    ];
}
