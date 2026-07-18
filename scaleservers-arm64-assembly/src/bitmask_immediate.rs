// Copyright (c) Scaleservers LLC

//! The AArch64 **logical (bitmask) immediate** codec -- the value <-> `(N, immr, imms)` transform shared by the
//! `AND`/`ORR`/`EOR`/`ANDS` immediate forms (DDI0487 C4.1.93 "Bitmask immediate", pseudocode `DecodeBitMasks`).
//!
//! A logical immediate is not an arbitrary constant: it must be a register-width tiling of a `2/4/8/16/32/64`-bit
//! element, where the element is a single rotated run of ones. Only ~5334 distinct 64-bit values (and a subset
//! for 32-bit) are representable; `0` and all-ones are NOT (those are `MOVZ`/`MOVN`/`MOV` territory). The
//! encoder ([`encode_bitmask`]) returns `None` for any non-representable value rather than emitting garbage.
//!
//! The 32-bit (`W`) form requires `N == 0` and a value that fits in 32 bits; the 64-bit (`X`) form uses the full
//! `N:imms` range. The two directions are exact inverses over the representable set (exhaustively tested below).

/// Encode a logical-immediate `value` for a `reg_size`-bit register (`32` or `64`) into its `(N, immr, imms)`
/// fields, or `None` if `value` is not a representable bitmask immediate. Mirrors LLVM's
/// `AArch64_AM::processLogicalImmediate` and the ARM `DecodeBitMasks` inverse.
pub fn encode_bitmask(value: u64, reg_size: u32) -> Option<(u32, u32, u32)> {
    debug_assert!(reg_size == 32 || reg_size == 64);

    // 0 and all-ones are unrepresentable; for the 32-bit form the value must fit in 32 bits and must not be the
    // 32-bit all-ones (that is `MOV`, not a logical immediate).
    if value == 0 || value == u64::MAX {
        return None;
    }
    if reg_size != 64 && (value >> reg_size != 0 || value == (u64::MAX >> (64 - reg_size))) {
        return None;
    }

    // 1) Element size: the largest `size` whose lower half equals its upper half all the way down -- i.e. the
    //    period of the bit pattern.
    let mut size = reg_size;
    loop {
        size /= 2;
        let mask = (1u64 << size) - 1;
        if (value & mask) != ((value >> size) & mask) {
            size *= 2;
            break;
        }
        if size <= 2 {
            break;
        }
    }

    // 2) Rotation that brings one element to the canonical `0...0 1...1` (a run of ones at the bottom).
    let mask = u64::MAX >> (64 - size);
    let mut element = value & mask;

    let (rotation_index, trailing_ones) = if is_shifted_mask(element) {
        let i = element.trailing_zeros();
        let cto = (element >> i).trailing_ones();
        (i, cto)
    } else {
        // The run of ones wraps the element boundary: invert and require the complement be a shifted mask.
        element |= !mask;
        if !is_shifted_mask(!element) {
            return None;
        }
        let clo = element.leading_ones();
        let i = 64 - clo;
        let cto = clo + element.trailing_ones() - (64 - size);
        (i, cto)
    };

    // `immr` = number of right-rotations; `imms` packs the element size (as a NOT-prefix) and ones-count.
    let immr = (size - rotation_index) & (size - 1);
    let mut n_imms = (!(size - 1)) << 1;
    n_imms |= trailing_ones - 1;
    let n = ((n_imms >> 6) & 1) ^ 1;

    Some((n, immr, n_imms & 0x3f))
}

/// Decode a logical-immediate `(n, immr, imms)` triple for a `reg_size`-bit register back to its `reg_size`-bit
/// value (high bits zero for the 32-bit form), or `None` if the fields are a reserved/unallocated combination.
/// Exact inverse of [`encode_bitmask`].
pub fn decode_bitmask(n: u32, immr: u32, imms: u32, reg_size: u32) -> Option<u64> {
    debug_assert!(reg_size == 32 || reg_size == 64);

    // `len` = highest set bit of `N : NOT(imms)` (7-bit field). len < 1, or an element wider than the register,
    // is unallocated. The 32-bit form therefore rejects N == 1 (which would force a 64-bit element).
    let combined = (n << 6) | ((!imms) & 0x3f);
    if combined == 0 {
        return None;
    }
    let len = 31 - combined.leading_zeros();
    if len < 1 {
        return None;
    }
    let esize = 1u32 << len;
    if esize > reg_size {
        return None;
    }

    let levels = esize - 1;
    let s = imms & levels;
    let r = immr & levels;
    if s == levels {
        return None; // a full run of ones within the element is reserved
    }

    let element = rotate_right_in_size(ones(s + 1), r, esize);
    Some(replicate(element, esize, reg_size))
}

// A value that is a single contiguous run of ones, possibly shifted up from bit 0 (`0...0 1...1 0...0`). Zero is not
// a shifted mask.
fn is_shifted_mask(value: u64) -> bool {
    value != 0 && is_mask((value - 1) | value)
}

// A value that is a contiguous run of ones anchored at bit 0 (`0...0 1...1`).
fn is_mask(value: u64) -> bool {
    value != 0 && value.wrapping_add(1) & value == 0
}

// `count` ones in the low bits (`count` in `0..=64`).
fn ones(count: u32) -> u64 {
    if count >= 64 {
        u64::MAX
    } else {
        (1u64 << count) - 1
    }
}

// Rotate the low `size` bits of `value` right by `amount` (`amount` in `0..size`, `size` in `2..=64`).
fn rotate_right_in_size(value: u64, amount: u32, size: u32) -> u64 {
    if amount == 0 {
        return value;
    }
    let mask = if size >= 64 {
        u64::MAX
    } else {
        (1u64 << size) - 1
    };
    ((value >> amount) | (value << (size - amount))) & mask
}

// Tile the low `esize` bits of `element` across `reg_size` bits (`esize` divides `reg_size`).
fn replicate(element: u64, esize: u32, reg_size: u32) -> u64 {
    let element_mask = if esize >= 64 {
        u64::MAX
    } else {
        (1u64 << esize) - 1
    };
    let unit = element & element_mask;
    let mut result = 0u64;
    let mut shift = 0;
    while shift < reg_size {
        result |= unit << shift;
        shift += esize;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_values_match_the_assembler() {
        // (value, reg_size) -> (N, immr, imms), confirmed against GNU `aarch64-linux-gnu-as` + LLVM `llvm-mc`.
        assert_eq!(encode_bitmask(0xff, 64), Some((1, 0, 7)));
        assert_eq!(encode_bitmask(0xffff, 64), Some((1, 0, 15)));
        assert_eq!(encode_bitmask(0xffff_ffff, 64), Some((1, 0, 31)));
        assert_eq!(encode_bitmask(0x1, 64), Some((1, 0, 0)));
        assert_eq!(encode_bitmask(0x5555_5555_5555_5555, 64), Some((0, 0, 60)));
        assert_eq!(encode_bitmask(0xfff, 64), Some((1, 0, 11)));
        assert_eq!(encode_bitmask(0x8000_0000_0000_0001, 64), Some((1, 1, 1))); // wrapping run of two ones (and x,x,#0x8000000000000001 = 0x92410420)
        // 32-bit forms always carry N == 0.
        assert_eq!(encode_bitmask(0xff, 32), Some((0, 0, 7)));
        assert_eq!(encode_bitmask(0xffff, 32), Some((0, 0, 15)));
        assert_eq!(encode_bitmask(0xf, 32), Some((0, 0, 3)));
        assert_eq!(encode_bitmask(0xaaaa_aaaa, 32), Some((0, 1, 60)));
    }

    #[test]
    fn unrepresentable_values_reject() {
        assert_eq!(encode_bitmask(0, 64), None);
        assert_eq!(encode_bitmask(u64::MAX, 64), None);
        assert_eq!(encode_bitmask(0xffff_ffff, 32), None); // 32-bit all-ones is MOV, not a logical immediate
        assert_eq!(encode_bitmask(0x1_0000_0000, 32), None); // does not fit 32 bits
        assert_eq!(encode_bitmask(0x0102_0304_0506_0708, 64), None); // not a rotated run
        assert_eq!(encode_bitmask(0b1011, 64), None); // two separate runs of ones
    }

    #[test]
    fn decode_inverts_encode_for_every_64bit_field() {
        // Every (N, immr, imms) the encoder can emit must round-trip through decode back to the same value, and
        // every value the encoder accepts must be reproducible by decode. Sweep the whole field space.
        for n in 0..2u32 {
            for immr in 0..64u32 {
                for imms in 0..64u32 {
                    if let Some(value) = decode_bitmask(n, immr, imms, 64) {
                        let (n2, immr2, imms2) =
                            encode_bitmask(value, 64).expect("decoded value must re-encode");
                        let value2 = decode_bitmask(n2, immr2, imms2, 64).unwrap();
                        assert_eq!(
                            value, value2,
                            "value round-trip failed for N={n} immr={immr} imms={imms}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn decode_inverts_encode_for_every_32bit_field() {
        for immr in 0..64u32 {
            for imms in 0..64u32 {
                if let Some(value) = decode_bitmask(0, immr, imms, 32) {
                    assert_eq!(value >> 32, 0, "32-bit decode must stay in 32 bits");
                    let (n2, immr2, imms2) =
                        encode_bitmask(value, 32).expect("decoded W value must re-encode");
                    assert_eq!(n2, 0, "32-bit form must have N == 0");
                    let value2 = decode_bitmask(n2, immr2, imms2, 32).unwrap();
                    assert_eq!(value, value2);
                }
            }
        }
        // N == 1 is unallocated for the 32-bit form (would imply a 64-bit element).
        assert_eq!(decode_bitmask(1, 0, 7, 32), None);
    }
}
