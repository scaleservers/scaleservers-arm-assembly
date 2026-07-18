// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **permute** instruction (DDI0487 C7) -- the encoding class
/// `0 Q 001110 size 0 Rm 0 opcode 10 Rn Rd`, which interleaves (`ZIP`), de-interleaves (`UZP`), or transposes
/// (`TRN`) the lanes of two source vectors. The op is an orthogonal field over the shared
/// `{ arrangement, Vd, Vn, Vm }` shape; every arrangement except the single-lane `.1d` is valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorPermuteOp {
    /// `ZIP1` -- interleave the lower halves of the two source vectors (`opcode = 011`).
    Zip1,
    /// `ZIP2` -- interleave the upper halves (`opcode = 111`).
    Zip2,
    /// `UZP1` -- de-interleave, taking the even-indexed lanes of the concatenated sources (`opcode = 001`).
    Uzp1,
    /// `UZP2` -- de-interleave, taking the odd-indexed lanes (`opcode = 101`).
    Uzp2,
    /// `TRN1` -- transpose, taking the even-indexed lanes (`opcode = 010`).
    Trn1,
    /// `TRN2` -- transpose, taking the odd-indexed lanes (`opcode = 110`).
    Trn2,
}

impl Arm64VectorPermuteOp {
    /// The base word with `Q = 0` and `size = 0` (the 3-bit opcode baked in); the arrangement supplies `Q<<30`
    /// and `size<<22`, and the registers `Vm<<16 | Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Zip1 => 0x0E00_3800,
            Self::Zip2 => 0x0E00_7800,
            Self::Uzp1 => 0x0E00_1800,
            Self::Uzp2 => 0x0E00_5800,
            Self::Trn1 => 0x0E00_2800,
            Self::Trn2 => 0x0E00_6800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Zip1 => "zip1",
            Self::Zip2 => "zip2",
            Self::Uzp1 => "uzp1",
            Self::Uzp2 => "uzp2",
            Self::Trn1 => "trn1",
            Self::Trn2 => "trn2",
        }
    }

    /// The SVE `opcode` field (`[12:10]`) of the SVE permute encoding (which numbers the ops 0..5, unlike the NEON
    /// 3-bit opcode). Used by the SVE `ZIP1`/.../`TRN2` vector permutes.
    pub fn sve_opcode(self) -> u32 {
        match self {
            Self::Zip1 => 0,
            Self::Zip2 => 1,
            Self::Uzp1 => 2,
            Self::Uzp2 => 3,
            Self::Trn1 => 4,
            Self::Trn2 => 5,
        }
    }

    /// Recover the op from the SVE `opcode` field, if one of the modeled permutes (`6`/`7` are unallocated).
    pub fn from_sve_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.sve_opcode() == opcode & 0x7)
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 6] = [
        Self::Zip1,
        Self::Zip2,
        Self::Uzp1,
        Self::Uzp2,
        Self::Trn1,
        Self::Trn2,
    ];
}
