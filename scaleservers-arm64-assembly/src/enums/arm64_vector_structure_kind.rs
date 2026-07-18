// Copyright (c) Scaleservers LLC

/// The structure pattern of an AArch64 Advanced SIMD (NEON) **load/store multiple structures** instruction
/// (DDI0487 C7) -- the `0 Q 0011000 L 000000 opcode size Rn Rt` class that transfers 1-4 consecutive vector
/// registers, optionally de-interleaving them. `LD1`/`ST1` move 1-4 registers with no de-interleave; `LD2`/`LD3`/
/// `LD4` de-interleave 2/3/4 registers. The `opcode` field encodes both the structure number and the register
/// count, so this enum enumerates the valid combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorStructureKind {
    /// `LD1`/`ST1` of one register (opcode `0111`).
    One1,
    /// `LD1`/`ST1` of two registers (opcode `1010`).
    One2,
    /// `LD1`/`ST1` of three registers (opcode `0110`).
    One3,
    /// `LD1`/`ST1` of four registers (opcode `0010`).
    One4,
    /// `LD2`/`ST2` -- de-interleave two registers (opcode `1000`).
    Two,
    /// `LD3`/`ST3` -- de-interleave three registers (opcode `0100`).
    Three,
    /// `LD4`/`ST4` -- de-interleave four registers (opcode `0000`).
    Four,
}

impl Arm64VectorStructureKind {
    /// The 4-bit `opcode` field `[15:12]`.
    pub fn opcode(self) -> u32 {
        match self {
            Self::One1 => 0b0111,
            Self::One2 => 0b1010,
            Self::One3 => 0b0110,
            Self::One4 => 0b0010,
            Self::Two => 0b1000,
            Self::Three => 0b0100,
            Self::Four => 0b0000,
        }
    }

    /// The number of registers transferred (the length of the register list).
    pub fn register_count(self) -> u8 {
        match self {
            Self::One1 => 1,
            Self::One2 | Self::Two => 2,
            Self::One3 | Self::Three => 3,
            Self::One4 | Self::Four => 4,
        }
    }

    /// The structure number `N` of the `LDN`/`STN` mnemonic (`1` for all the `One*` forms, else 2/3/4).
    pub fn structure_digit(self) -> u8 {
        match self {
            Self::One1 | Self::One2 | Self::One3 | Self::One4 => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }

    /// Recover the kind from the 4-bit `opcode` field; `None` for the unallocated opcodes (the single-structure
    /// and reserved encodings).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        match opcode & 0b1111 {
            0b0111 => Some(Self::One1),
            0b1010 => Some(Self::One2),
            0b0110 => Some(Self::One3),
            0b0010 => Some(Self::One4),
            0b1000 => Some(Self::Two),
            0b0100 => Some(Self::Three),
            0b0000 => Some(Self::Four),
            _ => None,
        }
    }

    /// Every kind, for tests / dispatch.
    pub const ALL: [Self; 7] = [
        Self::One1,
        Self::One2,
        Self::One3,
        Self::One4,
        Self::Two,
        Self::Three,
        Self::Four,
    ];
}
