// Copyright (c) Scaleservers LLC

use core::fmt;

use crate::targets::{
    ArmInstructionRequirement,
    ArmIsaVersion,
};

// Errors returned by `ArmT32Instruction::encode` (and `encode_for_target`) when an instruction
// value cannot be represented in the target encoding. Encoding never panics on a constructible
// instruction: every operand whose width/range/alignment is constrained by the encoding is validated
// here and surfaced as one of these variants instead of aborting the process. This matters because the
// console assembler builds instructions from user text and must report bad operands, not crash.
#[derive(Debug, PartialEq)]
pub enum EncodeError {
    /// An immediate / offset operand fell outside the range the encoding allows.
    /// `field` is the model field name (e.g. "decoded_imm7"); `minimum`/`maximum` are inclusive.
    ImmediateOutOfRange { field: &'static str, value: i64, minimum: i64, maximum: i64 },

    /// An immediate / offset operand was not the multiple the encoding requires (e.g. a word-aligned
    /// offset that must be a multiple of 4). `required_multiple` is that divisor.
    ImmediateNotAligned { field: &'static str, value: i64, required_multiple: u32 },

    /// A register is not permitted in this operand position because a dedicated encoding exists for it
    /// (e.g. SP / R13 in an ADD (register) slot, which must use the SP-relative form instead).
    /// `detail` names the form to use instead.
    RegisterNotEncodable { field: &'static str, detail: &'static str },

    /// A condition code is not permitted in this encoding (e.g. AL -- "always" -- in a conditional-branch
    /// T1 form, whose `1110`/`1111` condition slots are repurposed for UDF / SVC).
    ConditionNotEncodable { field: &'static str, detail: &'static str },

    /// A shift type is not permitted in this encoding (e.g. SSAT / USAT allow only LSL or ASR on Rn --
    /// LSR, ROR and RRX have no encoding there). `detail` names what is allowed.
    ShiftNotEncodable { field: &'static str, detail: &'static str },

    /// A 32-bit constant cannot be represented as a Thumb "modified immediate" (ThumbExpandImm): only a
    /// byte zero-extended, certain byte-replicated patterns, or an 8-bit value (bit 7 set) rotated right
    /// are encodable. `value` is the constant that could not be encoded.
    ModifiedImmediateNotEncodable { field: &'static str, value: u32 },

    /// An instruction was requested for a target processor profile that does not support it (its
    /// `requirement()` exceeds the profile's ISA version / features). Produced only by
    /// `ArmT32Instruction::encode_for_target`; the plain `encode` path is target-independent.
    UnsupportedInstructionForTarget { required: ArmInstructionRequirement, target_isa_version: ArmIsaVersion },
}

impl fmt::Display for EncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImmediateOutOfRange { field, value, minimum, maximum } => {
                write!(formatter, "operand `{field}` value {value} is out of range {minimum}..={maximum} for this encoding")
            },
            Self::ImmediateNotAligned { field, value, required_multiple } => {
                write!(formatter, "operand `{field}` value {value} is not a multiple of {required_multiple} (required by this encoding)")
            },
            Self::RegisterNotEncodable { field, detail } => {
                write!(formatter, "register in operand `{field}` is not encodable here: {detail}")
            },
            Self::ConditionNotEncodable { field, detail } => {
                write!(formatter, "condition in operand `{field}` is not encodable here: {detail}")
            },
            Self::ShiftNotEncodable { field, detail } => {
                write!(formatter, "shift in operand `{field}` is not encodable here: {detail}")
            },
            Self::ModifiedImmediateNotEncodable { field, value } => {
                write!(formatter, "operand `{field}` constant 0x{value:08X} cannot be expressed as a Thumb modified immediate")
            },
            Self::UnsupportedInstructionForTarget { required, target_isa_version } => {
                write!(formatter, "instruction needs {required:?} but the target profile is {target_isa_version:?}")
            },
        }
    }
}

impl core::error::Error for EncodeError {}
