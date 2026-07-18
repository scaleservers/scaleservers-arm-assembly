// Copyright (c) Scaleservers LLC

// Feature-gated `no_std`: the default build is `std`, but `--no-default-features` builds against
// `core` + `alloc` only. The `extern crate alloc;` below (an item) must follow ALL inner `#![...]`
// attributes, so it sits just after the `#![allow(...)]` block.
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
// AArch64 instruction encodings are bit-fields, so literals group by field width (e.g. 0xFF80_0000, masks
// like 0xFFFF_FC1F) rather than by uniform nibbles -- clippy's uniform-grouping heuristic is noise here.
#![allow(clippy::unusual_byte_groupings)]
// Encode helpers take one argument per instruction field; >7 args is natural for an ISA codec, not a smell.
#![allow(clippy::too_many_arguments)]

// `alloc` supplies the heap collections (`Vec` / `String`) the codec returns under `no_std`; `#[macro_use]`
// brings the `vec!` / `format!` macros into scope crate-wide. (`extern crate` is an item, so it follows the
// inner attributes.)
#[macro_use]
extern crate alloc;

mod enums;
// Re-export the whole enums surface. Public `Arm64Instruction` variants carry these operand enums (op
// selectors, arrangements, sizes, ...) as fields, so a consumer must be able to name every one of them to
// construct or pattern-match those variants.
pub use enums::*;

mod errors;
pub use errors::{DecodeError, EncodeError};

// The 8-bit FP immediate codec used by `FMOV (immediate)` -- convert a native float <-> the encoded `imm8`.
mod float_immediate;
pub use float_immediate::{
    fp8_decode_double, fp8_decode_single, fp8_encode_double, fp8_encode_single,
};

// The logical (bitmask) immediate codec used by `AND`/`ORR`/`EOR`/`ANDS` (immediate) -- convert a mask value <->
// the encoded `(N, immr, imms)` fields.
mod bitmask_immediate;
pub use bitmask_immediate::{decode_bitmask, encode_bitmask};

// Target-architecture gating: restrict the emittable set to a processor profile. AArch64 is a single linear
// lineage, so the gate is a rank compare on the ISA version plus a feature-set check.
pub mod targets;
pub use targets::{
    Arm64CpuFeature, Arm64InstructionRequirement, Arm64IsaVersion, Arm64TargetProfile,
};
