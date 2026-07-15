// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::utils::sign_extension_utils;

pub struct ArmT32InstructionDecoder {}

impl ArmT32InstructionDecoder {
    pub fn decode_instruction_halfword__0002__0305(
        instruction_halfword: u16,
    ) -> (/*u3*/ u8, /*u3*/ u8) {
        let val0 = (instruction_halfword & 0b0000_0111) as u8;
        let val1 = ((instruction_halfword >> 3) & 0b0000_0111) as u8;

        (val0, val1)
    }

    pub fn decode_instruction_halfword__0002__0305__0608(
        instruction_halfword: u16,
    ) -> (/*u3*/ u8, /*u3*/ u8, /*u3*/ u8) {
        let val0 = (instruction_halfword & 0b0000_0111) as u8;
        let val1 = ((instruction_halfword >> 3) & 0b0000_0111) as u8;
        let val2 = ((instruction_halfword >> 6) & 0b0000_0111) as u8;

        (val0, val1, val2)
    }

    pub fn decode_instruction_halfword__0002__0305__060a(
        instruction_halfword: u16,
    ) -> (/*u3*/ u8, /*u3*/ u8, /*u5*/ u8) {
        let val0 = (instruction_halfword & 0b0000_0111) as u8;
        let val1 = ((instruction_halfword >> 3) & 0b0000_0111) as u8;
        let val2 = ((instruction_halfword >> 6) & 0b0001_1111) as u8;

        (val0, val1, val2)
    }

    pub fn decode_instruction_halfword__0002__0306__0707(
        instruction_halfword: u16,
    ) -> (/*u3*/ u8, /*u4*/ u8, /*u1*/ u8) {
        let val0 = (instruction_halfword & 0b0000_0111) as u8;
        let val1 = ((instruction_halfword >> 3) & 0b0000_1111) as u8;
        let val2 = ((instruction_halfword >> 7) & 0b0000_0001) as u8;

        (val0, val1, val2)
    }

    pub fn decode_instruction_halfword__0002__0707(
        instruction_halfword: u16,
    ) -> (/*u3*/ u8, /*u1*/ u8) {
        let val0 = (instruction_halfword & 0b0000_0111) as u8;
        let val1 = ((instruction_halfword >> 7) & 0b0000_0001) as u8;

        (val0, val1)
    }

    pub fn decode_instruction_halfword__0006(instruction_halfword: u16) -> u8 {
        (instruction_halfword & 0b0111_1111) as u8
    }

    pub fn decode_instruction_halfword__0007(instruction_halfword: u16) -> u8 {
        (instruction_halfword & 0b1111_1111) as u8
    }

    pub fn decode_instruction_halfword__0007__0808(
        instruction_halfword: u16,
    ) -> (/*u8*/ u8, /*u1*/ u8) {
        let val0 = (instruction_halfword & 0b1111_1111) as u8;
        let val1 = ((instruction_halfword >> 8) & 0b0000_0001) as u8;

        (val0, val1)
    }

    pub fn decode_instruction_halfword__0007__080a(
        instruction_halfword: u16,
    ) -> (/*u8*/ u8, /*u3*/ u8) {
        let val0 = (instruction_halfword & 0b1111_1111) as u8;
        let val1 = ((instruction_halfword >> 8) & 0b0000_0111) as u8;

        (val0, val1)
    }

    pub fn decode_instruction_halfword__s0007__080b(
        instruction_halfword: u16,
    ) -> (/*i8*/ i8, /*u4*/ u8) {
        let val0_before_sign_extension = (instruction_halfword & 0b1111_1111) as i8;
        let val0 = val0_before_sign_extension; // already 8 bits in length, so there's no need to sign-extend it
        let val1 = ((instruction_halfword >> 8) & 0b0000_1111) as u8;

        (val0, val1)
    }

    pub fn decode_instruction_halfword__s000a(instruction_halfword: u16) -> i16 {
        let val0_before_sign_extension = (instruction_halfword & 0b0000_0111_1111_1111) as i16;

        sign_extension_utils::sign_extend_int_to_i16(val0_before_sign_extension, 11)
    }

    pub fn decode_instruction_halfword__0306(instruction_halfword: u16) -> u8 {
        ((instruction_halfword >> 3) & 0b0000_1111) as u8
    }

    pub fn decode_instruction_halfword__0404(instruction_halfword: u16) -> u8 {
        ((instruction_halfword >> 4) & 0b0000_0001) as u8
    }

    //

    pub fn decode_instruction_word__0003(instruction_word: u32) -> u8 {
        (instruction_word & 0b0000_1111) as u8
    }

    pub fn decode_instruction_word__0007__080b(instruction_word: u32) -> (/*u8*/ u8, /*u4*/ u8) {
        let val0 = (instruction_word & 0b1111_1111) as u8;
        let val1 = ((instruction_word >> 8) & 0b0000_1111) as u8;

        (val0, val1)
    }

    pub fn decode_instruction_word__0007__1013(instruction_word: u32) -> (/*u8*/ u8, /*u4*/ u8) {
        let val0 = (instruction_word & 0b1111_1111) as u8;
        let val1 = ((instruction_word >> 16) & 0b0000_1111) as u8;

        (val0, val1)
    }

    pub fn decode_instruction_word__000a__0b0b__0d0d__1019__1a1a(
        instruction_word: u32,
    ) -> (
        /*u11*/ u16,
        /*u1*/ u8,
        /*u1*/ u8,
        /*u10*/ u16,
        /*u1*/ u8,
    ) {
        let val0 = (instruction_word & 0b0000_0111_1111_1111) as u16;
        let val1 = ((instruction_word >> 11) & 0b0000_0001) as u8;
        let val2 = ((instruction_word >> 13) & 0b0000_0001) as u8;
        let val3 = ((instruction_word >> 16) & 0b0000_0011_1111_1111) as u16;
        let val4 = ((instruction_word >> 26) & 0b0000_0001) as u8;

        (val0, val1, val2, val3, val4)
    }

    pub fn decode_instruction_word__000b__1013(instruction_word: u32) -> (/*u12*/ u16, /*u4*/ u8) {
        let val0 = (instruction_word & 0b0000_1111_1111_1111) as u16;
        let val1 = ((instruction_word >> 16) & 0b0000_1111) as u8;

        (val0, val1)
    }
}
