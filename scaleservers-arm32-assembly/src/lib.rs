// Copyright (c) Scaleservers LLC

// Feature-gated `no_std`: the default build is `std`, but `--no-default-features` builds against
// `core` + `alloc` only. The `extern crate alloc;` below (an item) must follow ALL inner `#![...]`
// attributes, so it sits just after the `#![allow(...)]` block.
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
// ARM instruction encodings are bit-fields, so literals group by field width (e.g. 0xFFE0_0F80, masks like
// 0b1111_1000_0000_0000_1101_0000_0000_0000) rather than by uniform nibbles -- clippy's uniform-grouping
// heuristic is noise here.
#![allow(clippy::unusual_byte_groupings)]
// Encode helpers take one argument per instruction field; >7 args is natural for an ISA codec, not a smell.
#![allow(clippy::too_many_arguments)]

// `alloc` supplies the heap collections (`Vec`/`String`/`Box`) the codec returns under `no_std`; `#[macro_use]`
// brings the `vec!`/`format!` macros into scope crate-wide so per-file imports stay minimal. (Placed after the
// inner attributes / crate doc comment, since `extern crate` is an item and inner attributes must precede it.)
#[macro_use]
extern crate alloc;

mod enums;
pub use enums::{
    Arm32GeneralPurposeRegister,
    Arm32LowGeneralPurposeRegister,
    Arm32SinglePrecisionRegister,
    Arm32DoublePrecisionRegister,
    Arm32QuadwordRegister,
};
