// Copyright (c) Scaleservers LLC

use core::fmt;

/// Errors returned by [`crate::Arm64Instruction::decode`] when the input bytes are not well-formed.
/// Decoding NEVER panics on arbitrary input: every malformation is surfaced as one of these variants,
/// because a disassembler consumes untrusted binaries and must report bad bytes, not crash. A clean
/// end-of-input at an instruction boundary is `Ok(None)`, not an error.
///
/// A64 is a single fixed-width 32-bit instruction set, so "incomplete" always means fewer than four bytes
/// remained when a word was expected.
#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// The input ended in the middle of an instruction -- fewer than four bytes remained for a 32-bit word.
    IncompleteInstruction,
    /// The 32-bit word does not encode any instruction this decoder recognizes: an unallocated or undefined
    /// encoding, or a reserved field value the architecture does not permit. (A word the library does not
    /// model -- unallocated, or a not-yet-covered form -- also yields this.)
    InvalidOpcode,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IncompleteInstruction => {
                write!(
                    formatter,
                    "incomplete instruction: the input ended mid-instruction (fewer than 4 bytes remained for a word)"
                )
            }
            Self::InvalidOpcode => {
                write!(
                    formatter,
                    "invalid opcode: the bytes do not encode a recognized instruction"
                )
            }
        }
    }
}

impl core::error::Error for DecodeError {}
