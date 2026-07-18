// Copyright (c) Scaleservers LLC

//! The AArch64 8-bit floating-point immediate ("VFP/AdvSIMD modified immediate"), used by `FMOV (immediate)`.
//!
//! The field `abcdefgh` encodes the restricted value set `+/-(16 + efgh)/16 x 2^exp` for `exp in -3..=4` -- a
//! 4-bit mantissa (`efgh`) and one of 8 exponents (so magnitudes `0.125 ..= 31.0`). These helpers convert
//! between a native float and the `imm8`: the **float -> imm8** direction returns `None` when the value is not
//! representable (most values aren't -- the codegen then materializes the constant via a general register
//! instead), and **imm8 -> float** is the exact inverse (`VFPExpandImm`), used to render the disassembly.

/// Encode an `f64` as the 8-bit FP immediate, or `None` if it is not representable.
pub fn fp8_encode_double(value: f64) -> Option<u8> {
    let bits = value.to_bits();
    let sign = ((bits >> 63) & 1) as u8;
    let exponent = (bits >> 52) & 0x7FF; // 11-bit biased exponent
    let mantissa = bits & 0x000F_FFFF_FFFF_FFFF; // 52-bit
    // only the top 4 mantissa bits (efgh) may be set
    if mantissa & 0x0000_FFFF_FFFF_FFFF != 0 {
        return None;
    }
    // the biased exponent must be 1020..=1027 (unbiased -3..=4) -- exactly the `NOT(b):bx8:cd` bit pattern
    if !(1020..=1027).contains(&exponent) {
        return None;
    }
    let efgh = ((mantissa >> 48) & 0xF) as u8;
    let b = 1 - ((exponent >> 10) & 1) as u8; // exponent bit 10 is NOT(b)
    let cd = (exponent & 0x3) as u8;
    Some((sign << 7) | (b << 6) | (cd << 4) | efgh)
}

/// Decode the 8-bit FP immediate to the `f64` it represents -- `VFPExpandImm` for double precision, the exact
/// inverse of [`fp8_encode_double`] on the representable set.
pub fn fp8_decode_double(imm8: u8) -> f64 {
    let sign = ((imm8 >> 7) & 1) as u64;
    let b = ((imm8 >> 6) & 1) as u64;
    let cd = ((imm8 >> 4) & 0x3) as u64;
    let efgh = (imm8 & 0xF) as u64;
    // exponent (11 bits) = NOT(b) : Replicate(b, 8) : cd
    let exponent = ((1 - b) << 10) | ((b * 0xFF) << 2) | cd;
    f64::from_bits((sign << 63) | (exponent << 52) | (efgh << 48))
}

/// Encode an `f32` as the 8-bit FP immediate, or `None` if it is not representable.
pub fn fp8_encode_single(value: f32) -> Option<u8> {
    let bits = value.to_bits();
    let sign = ((bits >> 31) & 1) as u8;
    let exponent = (bits >> 23) & 0xFF; // 8-bit biased exponent
    let mantissa = bits & 0x007F_FFFF; // 23-bit
    if mantissa & 0x0007_FFFF != 0 {
        return None; // only the top 4 mantissa bits may be set
    }
    if !(124..=131).contains(&exponent) {
        return None; // unbiased -3..=4 (bias 127)
    }
    let efgh = ((mantissa >> 19) & 0xF) as u8;
    let b = 1 - ((exponent >> 7) & 1) as u8;
    let cd = (exponent & 0x3) as u8;
    Some((sign << 7) | (b << 6) | (cd << 4) | efgh)
}

/// Decode the 8-bit FP immediate to the `f32` it represents -- `VFPExpandImm` for single precision.
pub fn fp8_decode_single(imm8: u8) -> f32 {
    let sign = ((imm8 >> 7) & 1) as u32;
    let b = ((imm8 >> 6) & 1) as u32;
    let cd = ((imm8 >> 4) & 0x3) as u32;
    let efgh = (imm8 & 0xF) as u32;
    // exponent (8 bits) = NOT(b) : Replicate(b, 5) : cd
    let exponent = ((1 - b) << 7) | ((b * 0x1F) << 2) | cd;
    f32::from_bits((sign << 31) | (exponent << 23) | (efgh << 19))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_round_trips_every_imm8() {
        // every imm8 decodes to a representable value that re-encodes to the same imm8 (exhaustive).
        for imm8 in 0..=u8::MAX {
            let value = fp8_decode_double(imm8);
            assert_eq!(
                fp8_encode_double(value),
                Some(imm8),
                "imm8 0x{imm8:02x} -> {value} -> mismatch"
            );
        }
    }

    #[test]
    fn single_round_trips_every_imm8() {
        for imm8 in 0..=u8::MAX {
            let value = fp8_decode_single(imm8);
            assert_eq!(
                fp8_encode_single(value),
                Some(imm8),
                "imm8 0x{imm8:02x} -> {value} -> mismatch"
            );
        }
    }

    #[test]
    fn known_double_values() {
        assert_eq!(fp8_encode_double(1.0), Some(0x70));
        assert_eq!(fp8_decode_double(0x70), 1.0);
        assert_eq!(fp8_encode_double(2.0), Some(0x00));
        assert_eq!(fp8_encode_double(0.5), Some(0x60));
        assert_eq!(fp8_encode_double(-1.0), Some(0xF0));
        assert_eq!(fp8_encode_double(1.5), Some(0x78)); // 1 + 8/16
        assert_eq!(fp8_encode_single(1.0), Some(0x70));
    }

    #[test]
    fn non_representable_values_are_rejected() {
        assert_eq!(fp8_encode_double(0.1), None); // not a short dyadic -- mantissa needs > 4 bits
        assert_eq!(fp8_encode_double(100.0), None); // exponent too large
        assert_eq!(fp8_encode_double(0.0), None); // zero is not in the +/-(16+m)/16 x 2^e set
        assert_eq!(fp8_encode_double(1.01), None);
        assert_eq!(fp8_encode_single(0.1), None);
    }
}
