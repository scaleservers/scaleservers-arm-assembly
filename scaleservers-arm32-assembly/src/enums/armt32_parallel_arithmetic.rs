// Copyright (c) Scaleservers LLC

// The ARMv7E-M "parallel addition and subtraction" (packed SIMD) instructions are a regular family of 36:
// six operations x six signed/unsigned prefixes, all `Rd, Rn, Rm`. Rather than 36 near-identical model
// variants, `ArmT32Instruction::ParallelAddSub_T1` carries the operation and prefix as these two enums.
// Encoding: `operation.base() | Rn<<16 | Rd<<8 | (prefix.bits() << 4) | Rm`.

// Which packed add/subtract the instruction performs (and the halfword/byte element size).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmT32ParallelOperation {
    Add16, // ...ADD16
    Asx,   // ...ASX   (add high / subtract low, with halfword exchange)
    Sax,   // ...SAX   (subtract high / add low, with halfword exchange)
    Sub16, // ...SUB16
    Add8,  // ...ADD8
    Sub8,  // ...SUB8
}
impl ArmT32ParallelOperation {
    // base word: the high halfword opcode (operation in bits 22:20) with the fixed `1111` in bits 15:12.
    pub fn base(&self) -> u32 {
        match self {
            Self::Add8 => 0xFA80_F000,
            Self::Add16 => 0xFA90_F000,
            Self::Asx => 0xFAA0_F000,
            Self::Sub8 => 0xFAC0_F000,
            Self::Sub16 => 0xFAD0_F000,
            Self::Sax => 0xFAE0_F000,
        }
    }

    // the operation/size part of the mnemonic (after the signed/unsigned prefix)
    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::Add16 => "add16",
            Self::Asx => "asx",
            Self::Sax => "sax",
            Self::Sub16 => "sub16",
            Self::Add8 => "add8",
            Self::Sub8 => "sub8",
        }
    }

    // recover the operation from the 3-bit field at bits 22:20; None for the unallocated 011 / 111.
    pub fn from_op_bits(bits: u32) -> Option<Self> {
        // mask to the 3-bit field, so stray high bits a caller passes are ignored and the fallback is
        // exactly the unallocated 011 / 111 (a field decoder stays total over untrusted bytes).
        Some(match bits & 0b111 {
            0b000 => Self::Add8,
            0b001 => Self::Add16,
            0b010 => Self::Asx,
            0b100 => Self::Sub8,
            0b101 => Self::Sub16,
            0b110 => Self::Sax,
            _ => return None,
        })
    }
}

// The signed/unsigned (and saturating / halving) prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmT32ParallelPrefix {
    Signed,             // S
    SignedSaturating,   // Q
    SignedHalving,      // SH
    Unsigned,           // U
    UnsignedSaturating, // UQ
    UnsignedHalving,    // UH
}
impl ArmT32ParallelPrefix {
    // the 3-bit selector at bits 6:4 of the second halfword
    pub fn bits(&self) -> u32 {
        match self {
            Self::Signed => 0b000,
            Self::SignedSaturating => 0b001,
            Self::SignedHalving => 0b010,
            Self::Unsigned => 0b100,
            Self::UnsignedSaturating => 0b101,
            Self::UnsignedHalving => 0b110,
        }
    }

    pub fn mnemonic(&self) -> &'static str {
        match self {
            Self::Signed => "s",
            Self::SignedSaturating => "q",
            Self::SignedHalving => "sh",
            Self::Unsigned => "u",
            Self::UnsignedSaturating => "uq",
            Self::UnsignedHalving => "uh",
        }
    }

    pub fn from_prefix_bits(bits: u32) -> Option<Self> {
        // mask to the 3-bit field, so stray high bits are ignored and the fallback is exactly 011 / 111.
        Some(match bits & 0b111 {
            0b000 => Self::Signed,
            0b001 => Self::SignedSaturating,
            0b010 => Self::SignedHalving,
            0b100 => Self::Unsigned,
            0b101 => Self::UnsignedSaturating,
            0b110 => Self::UnsignedHalving,
            _ => return None,
        })
    }
}
