// Copyright (c) Scaleservers LLC

/// An SVE2 **bitwise permute** op (DDI0487; FEAT_SVE_BitPerm): `<op> Zd.<T>, Zn.<T>, Zm.<T>`, gathering/scattering
/// the bits of `Zn` under the mask `Zm`. Selected by `opc` at `[11:10]`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2BitPermuteOp {
    /// `BEXT` -- gather the `Zn` bits selected by the `Zm` mask into the low bits (`opc`=00).
    Bext,
    /// `BDEP` -- scatter the low `Zn` bits into the `Zm` mask positions (`opc`=01).
    Bdep,
    /// `BGRP` -- group the masked / unmasked `Zn` bits to the low / high halves (`opc`=10).
    Bgrp,
}

impl Arm64Sve2BitPermuteOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Bext => "bext",
            Self::Bdep => "bdep",
            Self::Bgrp => "bgrp",
        }
    }

    /// The 2-bit `opc` field (`[11:10]`).
    pub fn opc(self) -> u32 {
        match self {
            Self::Bext => 0b00,
            Self::Bdep => 0b01,
            Self::Bgrp => 0b10,
        }
    }

    /// Recover the op from its `[11:10]` opcode, or `None` for the unallocated `11`.
    pub fn from_opc(opc: u32) -> Option<Self> {
        match opc & 0b11 {
            0b00 => Some(Self::Bext),
            0b01 => Some(Self::Bdep),
            0b10 => Some(Self::Bgrp),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Bext, Self::Bdep, Self::Bgrp];
}
