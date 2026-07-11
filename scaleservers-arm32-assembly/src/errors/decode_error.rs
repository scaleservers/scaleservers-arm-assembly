// Copyright (c) Scaleservers LLC

use core::fmt;

/// Errors returned by the decode paths (`ArmT32Instruction::decode` / `ArmA32Instruction::decode`) when the
/// input bytes are not well-formed. Decoding NEVER panics on arbitrary input: every malformation is surfaced
/// as one of these variants, because a disassembler consumes untrusted binaries and must report bad bytes,
/// not crash. A clean end-of-input at an instruction boundary is `Ok(None)`, not an error.
#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The input ended in the middle of an instruction -- a 16-bit halfword or a 32-bit word was truncated.
    IncompleteInstruction,
    /// The bytes do not encode any instruction this decoder recognizes: an unallocated or undefined
    /// encoding, or a reserved field value the architecture does not permit.
    InvalidOpcode,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompleteInstruction => {
                write!(formatter, "incomplete instruction: the input ended mid-instruction (a halfword or word was truncated)")
            },
            Self::InvalidOpcode => {
                write!(formatter, "invalid opcode: the bytes do not encode a recognized instruction")
            },
        }
    }
}

impl core::error::Error for DecodeError {}
