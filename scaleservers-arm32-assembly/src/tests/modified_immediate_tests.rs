// Copyright (c) Scaleservers LLC

// Exhaustive modified-immediate codec tests -- the classic ARM-assembler bug
// source: a 12-bit field expands to a 32-bit constant, only some constants are representable, and some
// constants have multiple field encodings. Covered for BOTH the T32 `ThumbExpandImm` codec and the A32
// modified-immediate codec, in both directions:
//   (1) every one of the 4096 fields decodes to a constant that re-encodes to a field decoding to the SAME
//       constant -- a canonical fixed point over the whole space (catches any decode/encode asymmetry);
//   (2) every representable constant round-trips, and an unrepresentable constant is REJECTED (no silent
//       mis-encode).

use crate::arma32_instruction::{decode_a32_modified_immediate, encode_a32_modified_immediate};
use crate::armt32_instruction::{decode_thumb_expand_imm, encode_thumb_expand_imm};

// Under Miri (100-1000x slower; UB depends on the code PATH, not the iteration count) we walk the field
// space with a stride instead of every value -- the encode/decode paths still run.
#[cfg(miri)]
const MODIMM_FIELD_STRIDE: u16 = 97; // coprime-ish with 4096 so the sample spreads across all field shapes
#[cfg(not(miri))]
const MODIMM_FIELD_STRIDE: u16 = 1;
#[cfg(miri)]
const MODIMM_VALUE_STRIDE: u32 = 1031; // sample the 0..=0xFFFF constant space sparsely under Miri
#[cfg(not(miri))]
const MODIMM_VALUE_STRIDE: u32 = 1;

#[test]
fn thumb_expand_imm_is_a_canonical_fixed_point_over_all_4096_fields() {
    for imm12 in (0u16..4096).step_by(MODIMM_FIELD_STRIDE as usize) {
        let constant = decode_thumb_expand_imm(imm12);
        // The constant came from a real field, so it MUST re-encode, and the re-encoded field must decode to
        // the same constant (a canonical form -- not necessarily the same field bits).
        let re = encode_thumb_expand_imm(constant).unwrap_or_else(|| {
            panic!("ThumbExpandImm {imm12:#05x} -> {constant:#010x} did not re-encode")
        });
        assert_eq!(
            decode_thumb_expand_imm(re),
            constant,
            "ThumbExpandImm is not a fixed point: field {imm12:#05x} -> {constant:#010x} -> field {re:#05x} -> {:#010x}",
            decode_thumb_expand_imm(re)
        );
    }
}

#[test]
fn thumb_expand_imm_round_trips_representable_and_rejects_unrepresentable() {
    let mut representable = 0u32;
    for value in (0u32..=0xFFFF).step_by(MODIMM_VALUE_STRIDE as usize) {
        if let Some(imm12) = encode_thumb_expand_imm(value) {
            assert_eq!(
                decode_thumb_expand_imm(imm12),
                value,
                "ThumbExpandImm encode/decode mismatch for {value:#x}"
            );
            representable += 1;
        }
    }
    // The representable-count floor only holds when we walk the full space (stride 1); under Miri's sparse
    // stride we just need at least one to prove the path runs.
    #[cfg(not(miri))]
    assert!(
        representable > 256,
        "expected many representable constants, got {representable}"
    );
    #[cfg(miri)]
    assert!(
        representable > 0,
        "expected at least one representable constant under the Miri stride"
    );
    // byte-rotated 8-bit patterns (the ARM modified-immediate family) must round-trip when representable.
    for byte in [0x01u32, 0x7F, 0x80, 0xAB, 0xFF] {
        for rotation in 0..32u32 {
            let value = byte.rotate_right(rotation);
            if let Some(imm12) = encode_thumb_expand_imm(value) {
                assert_eq!(decode_thumb_expand_imm(imm12), value);
            }
        }
    }
    // a constant with no valid ThumbExpandImm form must be REJECTED, not silently mis-encoded.
    assert!(
        encode_thumb_expand_imm(0x1234_5678).is_none(),
        "0x12345678 is not a ThumbExpandImm"
    );
    assert!(
        encode_thumb_expand_imm(0x00FF_FF00).is_none(),
        "0x00FFFF00 is not a ThumbExpandImm"
    );
}

#[test]
fn a32_modified_immediate_is_a_canonical_fixed_point_over_all_4096_fields() {
    for imm12 in (0u16..4096).step_by(MODIMM_FIELD_STRIDE as usize) {
        let constant = decode_a32_modified_immediate(imm12);
        let re = encode_a32_modified_immediate(constant).unwrap_or_else(|| {
            panic!("A32 modimm {imm12:#05x} -> {constant:#010x} did not re-encode")
        });
        assert_eq!(
            decode_a32_modified_immediate(re),
            constant,
            "A32 modified immediate is not a fixed point: field {imm12:#05x} -> {constant:#010x}"
        );
    }
}

#[test]
fn a32_modified_immediate_round_trips_representable_and_rejects_unrepresentable() {
    for value in (0u32..=0xFFFF).step_by(MODIMM_VALUE_STRIDE as usize) {
        if let Some(imm12) = encode_a32_modified_immediate(value) {
            assert_eq!(
                decode_a32_modified_immediate(imm12),
                value,
                "A32 modimm encode/decode mismatch for {value:#x}"
            );
        }
    }
    // A32 modified immediates are an 8-bit value rotated right by an EVEN amount; this isn't one.
    assert!(
        encode_a32_modified_immediate(0x1234_5678).is_none(),
        "0x12345678 is not an A32 modified immediate"
    );
    assert!(
        encode_a32_modified_immediate(0x0000_01FE).is_none(),
        "0x000001FE (9 significant bits) is not encodable"
    );
}
