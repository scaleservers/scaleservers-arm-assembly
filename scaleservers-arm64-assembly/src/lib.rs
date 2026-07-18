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
