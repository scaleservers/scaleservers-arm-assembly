// Copyright (c) Scaleservers LLC

// `Vec` is not in the `no_std` prelude; pull it from `alloc`.
use alloc::vec::Vec;
use crate::{
    ArmA32Instruction,
    ArmT32Instruction,
    EncodeError,
};

/// Which ARM instruction set a code stream is in. A32 ("ARM" state) and T32 (Thumb state) are distinct
/// instruction sets with incompatible encodings; a 32-bit ARM program selects between them at runtime
/// (interworking -- a `BX`/`BLX` whose target's low bit picks the state).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmInstructionSet {
    /// "ARM" state -- fixed-width 32-bit encoding (Cortex-A / Cortex-R, classic ARM).
    A32,
    /// Thumb state -- 16/32-bit Thumb-2 encoding (all Cortex cores).
    T32,
}

/// The union of the two 32-bit ARM instruction sets. [`ArmA32Instruction`] and [`ArmT32Instruction`] are
/// kept as SEPARATE types so a compiler backend cannot accidentally mix sets within one stream (the
/// wrong-set mistake becomes a compile error, not a runtime one). This enum re-unites them for the
/// disassembler, which must follow ARM/Thumb interworking across a single binary, and for any caller that
/// genuinely handles both states.
#[derive(Debug, PartialEq)]
pub enum Arm32Instruction {
    /// An A32 ("ARM" state) instruction.
    A32(ArmA32Instruction),
    /// A T32 (Thumb state) instruction.
    T32(ArmT32Instruction),
}
impl Arm32Instruction {
    /// Which instruction set this instruction belongs to.
    pub fn instruction_set(&self) -> ArmInstructionSet {
        match self {
            Self::A32(_) => ArmInstructionSet::A32,
            Self::T32(_) => ArmInstructionSet::T32,
        }
    }

    /// Encode to machine-code bytes (little-endian): always 4 for A32; 2 or 4 for T32. Returns
    /// [`EncodeError`] if any operand field is out of range for the chosen encoding.
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {
        match self {
            Self::A32(instruction) => instruction.encode(),
            Self::T32(instruction) => instruction.encode(),
        }
    }
}
