// Copyright (c) Scaleservers LLC

#![allow(non_snake_case)]

use crate::enums::{ArmT32OpcodePattern_16Bit, ArmT32OpcodePattern_32Bit};

pub struct ArmT32InstructionEncoder {}

impl ArmT32InstructionEncoder {
    pub fn encode_instruction_halfword__0002__0305(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u3*/ u8,
        arg1: /*u3*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 3);
        panic_on_invalid_bit_length_u8(arg1, 3);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1111_1100_0000;
        result |= arg0 as u16;
        result |= (arg1 as u16) << 3;

        result
    }

    pub fn encode_instruction_halfword__0002__0305__0608(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u3*/ u8,
        arg1: /*u3*/ u8,
        arg2: /*u3*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 3);
        panic_on_invalid_bit_length_u8(arg1, 3);
        panic_on_invalid_bit_length_u8(arg2, 3);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1110_0000_0000;
        result |= arg0 as u16;
        result |= (arg1 as u16) << 3;
        result |= (arg2 as u16) << 6;

        result
    }

    pub fn encode_instruction_halfword__0002__0305__060a(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u3*/ u8,
        arg1: /*u3*/ u8,
        arg2: /*u5*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 3);
        panic_on_invalid_bit_length_u8(arg1, 3);
        panic_on_invalid_bit_length_u8(arg2, 5);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1000_0000_0000;
        result |= arg0 as u16;
        result |= (arg1 as u16) << 3;
        result |= (arg2 as u16) << 6;

        result
    }

    pub fn encode_instruction_halfword__0002__0306__0707(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u3*/ u8,
        arg1: /*u4*/ u8,
        arg2: /*u1*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 3);
        panic_on_invalid_bit_length_u8(arg1, 4);
        panic_on_invalid_bit_length_u8(arg2, 1);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1111_0000_0000;
        result |= arg0 as u16;
        result |= (arg1 as u16) << 3;
        result |= (arg2 as u16) << 7;

        result
    }

    pub fn encode_instruction_halfword__0002__0707(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u3*/ u8,
        arg1: /*u1*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 3);
        panic_on_invalid_bit_length_u8(arg1, 1);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1111_0111_1000;
        result |= arg0 as u16;
        result |= (arg1 as u16) << 7;

        result
    }

    pub fn encode_instruction_halfword__0006(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u7*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 7);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1111_1000_0000;
        result |= arg0 as u16;

        result
    }

    pub fn encode_instruction_halfword__0007(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u8*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 8);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1111_0000_0000;
        result |= arg0 as u16;

        result
    }

    pub fn encode_instruction_halfword__0007__0808(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u8*/ u8,
        arg1: /*u1*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 8);
        panic_on_invalid_bit_length_u8(arg1, 1);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1110_0000_0000;
        result |= arg0 as u16;
        result |= (arg1 as u16) << 8;

        result
    }

    pub fn encode_instruction_halfword__0007__080a(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u8*/ u8,
        arg1: /*u3*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 8);
        panic_on_invalid_bit_length_u8(arg1, 3);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1000_0000_0000;
        result |= arg0 as u16;
        result |= (arg1 as u16) << 8;

        result
    }

    // NOTE: the "s" in "s080b" signifies that the argument is signed
    pub fn encode_instruction_halfword__0007__s080b(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*i8*/ i8,
        arg1: /*u4*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_i8(arg0, 8);
        panic_on_invalid_bit_length_u8(arg1, 4);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_0000_0000_0000;
        let arg0_as_u16 = (arg0 as u8) as u16;
        result |= arg0_as_u16;
        result |= (arg1 as u16) << 8;

        result
    }

    // NOTE: the "s" in "s000a" signifies that the argument is signed
    pub fn encode_instruction_halfword__s000a(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*i11*/ i16,
    ) -> u16 {
        panic_on_invalid_bit_length_i16(arg0, 11);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1000_0000_0000;
        let arg0_as_u11 = (arg0 as u16) & 0b0000_0111_1111_1111;
        result |= arg0_as_u11;

        result
    }

    pub fn encode_instruction_halfword__0306(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u4*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 4);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1111_1000_0111;
        result |= (arg0 as u16) << 3;

        result
    }

    pub fn encode_instruction_halfword__0404(
        opcode_pattern: ArmT32OpcodePattern_16Bit,
        arg0: /*u1*/ u8,
    ) -> u16 {
        panic_on_invalid_bit_length_u8(arg0, 1);

        let opcode_pattern_as_u16 = opcode_pattern as u16;

        let mut result = opcode_pattern_as_u16 & 0b1111_1111_1110_1111;
        result |= (arg0 as u16) << 4;

        result
    }

    //

    pub fn encode_instruction_word__0003(
        opcode_pattern: ArmT32OpcodePattern_32Bit,
        arg0: /*u4*/ u8,
    ) -> u32 {
        panic_on_invalid_bit_length_u8(arg0, 4);

        let opcode_pattern_as_u32 = opcode_pattern as u32;

        let mut result = opcode_pattern_as_u32 & 0b1111_1111_1111_1111_1111_1111_1111_0000;
        result |= arg0 as u32;

        result
    }

    pub fn encode_instruction_word__0007__080b(
        opcode_pattern: ArmT32OpcodePattern_32Bit,
        arg0: /*u8*/ u8,
        arg1: /*u4*/ u8,
    ) -> u32 {
        panic_on_invalid_bit_length_u8(arg0, 8);
        panic_on_invalid_bit_length_u8(arg1, 4);

        let opcode_pattern_as_u32 = opcode_pattern as u32;

        let mut result = opcode_pattern_as_u32 & 0b1111_1111_1111_1111_1111_0000_0000_0000;
        result |= arg0 as u32;
        result |= (arg1 as u32) << 8;

        result
    }

    pub fn encode_instruction_word__0007__1013(
        opcode_pattern: ArmT32OpcodePattern_32Bit,
        arg0: /*u8*/ u8,
        arg1: /*u4*/ u8,
    ) -> u32 {
        panic_on_invalid_bit_length_u8(arg0, 8);
        panic_on_invalid_bit_length_u8(arg1, 4);

        let opcode_pattern_as_u32 = opcode_pattern as u32;

        let mut result = opcode_pattern_as_u32 & 0b1111_1111_1111_0000_1111_1111_0000_0000;
        result |= arg0 as u32;
        result |= (arg1 as u32) << 16;

        result
    }

    pub fn encode_instruction_word__000a__0b0b__0d0d__1019__1a1a(
        opcode_pattern: ArmT32OpcodePattern_32Bit,
        arg0: /*u11*/ u16,
        arg1: /*u1*/ u8,
        arg2: /*u1*/ u8,
        arg3: /*u10*/ u16,
        arg4: /*u1*/ u8,
    ) -> u32 {
        panic_on_invalid_bit_length_u16(arg0, 11);
        panic_on_invalid_bit_length_u8(arg1, 1);
        panic_on_invalid_bit_length_u8(arg2, 1);
        panic_on_invalid_bit_length_u16(arg3, 10);
        panic_on_invalid_bit_length_u8(arg4, 1);

        let opcode_pattern_as_u32 = opcode_pattern as u32;

        let mut result = opcode_pattern_as_u32 & 0b1111_1000_0000_0000_1101_0000_0000_0000;
        result |= arg0 as u32;
        result |= (arg1 as u32) << 11;
        result |= (arg2 as u32) << 13;
        result |= (arg3 as u32) << 16;
        result |= (arg4 as u32) << 26;

        result
    }

    pub fn encode_instruction_word__000b__1013(
        opcode_pattern: ArmT32OpcodePattern_32Bit,
        arg0: /*u12*/ u16,
        arg1: /*u4*/ u8,
    ) -> u32 {
        panic_on_invalid_bit_length_u16(arg0, 12);
        panic_on_invalid_bit_length_u8(arg1, 4);

        let opcode_pattern_as_u32 = opcode_pattern as u32;

        let mut result = opcode_pattern_as_u32 & 0b1111_1111_1111_0000_1111_0000_0000_0000;
        result |= arg0 as u32;
        result |= (arg1 as u32) << 16;

        result
    }
}

//

fn panic_on_invalid_bit_length_u8(value: u8, max_bit_length: usize) {
    if max_bit_length > 8 {
        panic!("Invalid value for argument 'max_bit_length'");
    }

    let bitmask: u8 = (0xFFu16 << max_bit_length) as u8;

    if (value & bitmask) != 0 {
        panic!("Instruction data parameter value is out of allowed range");
    }
}

fn panic_on_invalid_bit_length_i8(value: i8, max_bit_length: usize) {
    if !(1..=8).contains(&max_bit_length) {
        panic!("Invalid value for argument 'max_bit_length'");
    }

    let bitmask: i8 = (0xFFu8 << (max_bit_length - 1)) as i8;

    if value >= 0 {
        // whole number value
        if (value & bitmask) != 0 {
            // positive number out of range
            panic!("Instruction data parameter value is out of allowed range");
        }
    } else {
        // negative integer value
        if (value & bitmask) != bitmask {
            // negative number out of range
            panic!("Instruction data parameter value is out of allowed range");
        }
    }
}

fn panic_on_invalid_bit_length_u16(value: u16, max_bit_length: usize) {
    if max_bit_length > 16 {
        panic!("Invalid value for argument 'max_bit_length'");
    }

    let bitmask: u16 = (0xFFFFu32 << max_bit_length) as u16;

    if (value & bitmask) != 0 {
        panic!("Instruction data parameter value is out of allowed range");
    }
}

fn panic_on_invalid_bit_length_i16(value: i16, max_bit_length: usize) {
    if !(1..=16).contains(&max_bit_length) {
        panic!("Invalid value for argument 'max_bit_length'");
    }

    let bitmask: i16 = (0xFFFFu16 << (max_bit_length - 1)) as i16;

    if value >= 0 {
        // whole number value
        if (value & bitmask) != 0 {
            // positive number out of range
            panic!("Instruction data parameter value is out of allowed range");
        }
    } else {
        // negative integer value
        if (value & bitmask) != bitmask {
            // negative number out of range
            panic!("Instruction data parameter value is out of allowed range");
        }
    }
}
