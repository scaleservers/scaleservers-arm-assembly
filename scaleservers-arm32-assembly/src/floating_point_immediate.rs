// Copyright (c) Scaleservers LLC

// The VFP "modified immediate" used by VMOV (immediate): an 8-bit field a:b:cdefgh that expands to one of
// 256 floating-point values. The expansion builds a float whose sign is `a`, whose exponent is
// `NOT(b):Replicate(b):cd...` and whose mantissa top bits are the rest of cdefgh. The assembler uses the
// inverse (`vfp_encode_f64_to_imm8`) to encode a `#<float>` literal, refusing values outside the 256.

pub fn vfp_expand_imm8_to_f32(imm8: u8) -> f32 {
    let a = (imm8 >> 7) & 1;
    let b = (imm8 >> 6) & 1;
    let cdefgh = (imm8 & 0x3F) as u32;
    let exponent_fill = if b == 1 { 0b11111u32 } else { 0 };
    let pattern =
        ((a as u32) << 31) | (((b ^ 1) as u32) << 30) | (exponent_fill << 25) | (cdefgh << 19);
    f32::from_bits(pattern)
}

pub fn vfp_expand_imm8_to_f64(imm8: u8) -> f64 {
    let a = (imm8 >> 7) & 1;
    let b = (imm8 >> 6) & 1;
    let cdefgh = (imm8 & 0x3F) as u64;
    let exponent_fill = if b == 1 { 0xFFu64 } else { 0 };
    let pattern =
        ((a as u64) << 63) | (((b ^ 1) as u64) << 62) | (exponent_fill << 54) | (cdefgh << 48);
    f64::from_bits(pattern)
}

// The imm8 whose expansion equals `value`, or None if `value` is not one of the 256 VFP immediates. The 256
// values are the same logical set for single and double precision, so single-precision matching suffices.
pub fn vfp_encode_f64_to_imm8(value: f64) -> Option<u8> {
    (0..=255u8).find(|&imm8| vfp_expand_imm8_to_f32(imm8) as f64 == value)
}
