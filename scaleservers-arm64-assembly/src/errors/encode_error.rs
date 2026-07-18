// Copyright (c) Scaleservers LLC

use core::fmt;

use crate::targets::{Arm64InstructionRequirement, Arm64IsaVersion};

/// Errors returned by [`crate::Arm64Instruction::encode`] (and `encode_for_target`) when an instruction
/// value cannot be represented in the target encoding. Encoding never panics on a constructible
/// instruction: every operand whose width / range / alignment is constrained by the encoding is validated
/// here and surfaced as one of these variants instead of aborting the process. This matters because a
/// console assembler builds instructions from user text and must report bad operands, not crash.
#[derive(Debug, PartialEq)]
pub enum EncodeError {
    /// An immediate / offset operand fell outside the range the encoding allows.
    /// `field` is the model field name (e.g. "imm12"); `minimum`/`maximum` are inclusive.
    ImmediateOutOfRange {
        field: &'static str,
        value: i64,
        minimum: i64,
        maximum: i64,
    },

    /// An immediate / offset operand was not the multiple the encoding requires (e.g. a 64-bit load's
    /// scaled byte offset that must be a multiple of 8, or a branch target that must be 4-byte aligned).
    /// `required_multiple` is that divisor.
    ImmediateNotAligned {
        field: &'static str,
        value: i64,
        required_multiple: u32,
    },

    /// A register is not permitted in this operand position (e.g. the stack pointer `SP` in a slot whose
    /// `31` encoding means the zero register `XZR`, or vice versa). `detail` names what is allowed.
    RegisterNotEncodable {
        field: &'static str,
        detail: &'static str,
    },

    /// Two operands form a combination the encoding cannot represent even though each is individually valid
    /// (e.g. an `FCVT` whose source and destination precision are the same). `detail` describes the rule.
    InvalidOperandCombination { detail: &'static str },

    /// A logical-immediate operand (`AND`/`ORR`/`EOR`/`ANDS` immediate) is not a representable bitmask: it is
    /// `0`, all-ones, or not a register-width tiling of a single rotated run of ones (only ~5334 distinct 64-bit
    /// values qualify). `value` is the requested constant; such constants must instead be materialized with
    /// `MOVZ`/`MOVK`/`MOVN`. `reg_size_bits` is the operand width the value was checked against (32 or 64).
    UnrepresentableBitmaskImmediate {
        field: &'static str,
        value: u64,
        reg_size_bits: u32,
    },

    /// An instruction was requested for a target processor profile that does not support it (its
    /// `requirement()` exceeds the profile's ISA version / features). Produced only by
    /// `Arm64Instruction::encode_for_target`; the plain `encode` path is target-independent.
    UnsupportedInstructionForTarget {
        required: Arm64InstructionRequirement,
        target_isa_version: Arm64IsaVersion,
    },
}

impl fmt::Display for EncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ImmediateOutOfRange {
                field,
                value,
                minimum,
                maximum,
            } => {
                write!(
                    formatter,
                    "operand `{field}` value {value} is out of range {minimum}..={maximum} for this encoding"
                )
            }
            Self::ImmediateNotAligned {
                field,
                value,
                required_multiple,
            } => {
                write!(
                    formatter,
                    "operand `{field}` value {value} is not a multiple of {required_multiple} (required by this encoding)"
                )
            }
            Self::RegisterNotEncodable { field, detail } => {
                write!(
                    formatter,
                    "register in operand `{field}` is not encodable here: {detail}"
                )
            }
            Self::InvalidOperandCombination { detail } => {
                write!(formatter, "invalid operand combination: {detail}")
            }
            Self::UnrepresentableBitmaskImmediate {
                field,
                value,
                reg_size_bits,
            } => {
                write!(
                    formatter,
                    "operand `{field}` value {value:#x} is not a representable {reg_size_bits}-bit logical (bitmask) immediate"
                )
            }
            Self::UnsupportedInstructionForTarget {
                required,
                target_isa_version,
            } => {
                write!(
                    formatter,
                    "instruction needs {required:?} but the target profile is {target_isa_version:?}"
                )
            }
        }
    }
}

impl core::error::Error for EncodeError {}
