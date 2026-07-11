// Copyright (c) Scaleservers LLC

pub fn sign_extend_int_to_i16(value: i16, bit_length: usize) -> i16 {
    if !(1..=16).contains(&bit_length) {
        panic!("Invalid value for argument 'max_bit_length'");
    }

    // scenario 1: nothing to extend
    if bit_length == 16 {
        return value;
    }

    // scenario 2: whole number value
    if value & (1 << (bit_length - 1)) == 0 {
        return value;
    }

    // scenario 3: negative integer value
    let value_as_u16 = value as u16;
    let result_as_u16 = value_as_u16 | (0xFFFF << bit_length);
    let result = result_as_u16 as i16;
    //
    result
}

pub fn sign_extend_int_to_i32(value: i32, bit_length: usize) -> i32 {
    if !(1..=32).contains(&bit_length) {
        panic!("Invalid value for argument 'max_bit_length'");
    }

    // scenario 1: nothing to extend
    if bit_length == 32 {
        return value;
    }

    // scenario 2: whole number value
    if value & (1 << (bit_length - 1)) == 0 {
        return value;
    }

    // scenario 3: negative integer value
    let value_as_u32 = value as u32;
    let result_as_u32 = value_as_u32 | (0xFFFFFFFF << bit_length);
    let result = result_as_u32 as i32;
    //
    result
}
