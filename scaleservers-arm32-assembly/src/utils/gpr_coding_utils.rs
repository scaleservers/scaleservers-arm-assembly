// Copyright (c) Scaleservers LLC

// `Vec` is not in the `no_std` prelude; pull it from `alloc`.
use alloc::vec::Vec;
use crate::EncodeError;
use crate::enums::{
    Arm32LowGeneralPurposeRegister,
    Arm32GeneralPurposeRegister
};

pub fn convert_gpr_register_list_u8_and_m_u1_to_registers_vector(register_list: u8, m: /*u1*/u8) -> Vec<Arm32GeneralPurposeRegister> {
    let mut result = convert_gpr_register_list_u8_to_registers_vector(register_list);
    if m != 0 {
        result.push(Arm32GeneralPurposeRegister::R14/* R14/LR */);
    }

    result
}

pub fn convert_gpr_register_list_u8_and_p_u1_to_registers_vector(register_list: u8, p: /*u1*/u8) -> Vec<Arm32GeneralPurposeRegister> {
    let mut result = convert_gpr_register_list_u8_to_registers_vector(register_list);
    if p != 0 {
        result.push(Arm32GeneralPurposeRegister::R15/* R15/PC */);
    }

    result
}

pub fn convert_gpr_register_list_u8_to_low_registers_vector(register_list: u8) -> Vec<Arm32LowGeneralPurposeRegister> {
    let mut result = Vec::<Arm32LowGeneralPurposeRegister>::new();
    for i in 0..=7 {
        if (register_list & (1 << i)) != 0 {
            result.push(Arm32LowGeneralPurposeRegister::from_operand_bits(i));
        }
    }

    result
}

pub fn convert_gpr_register_list_u8_to_registers_vector(register_list: u8) -> Vec<Arm32GeneralPurposeRegister> {
    let mut result = Vec::<Arm32GeneralPurposeRegister>::new();
    for i in 0..=7 {
        if (register_list & (1 << i)) != 0 {
            result.push(Arm32GeneralPurposeRegister::from_operand_bits(i));
        }
    }

    result
}

// The full 16-bit register-list bitmask (R0..PC) used by the wide LDM/STM/PUSH/POP forms -> a register
// vector, ascending by register number (the order UAL prints them in).
pub fn convert_gpr_register_list_u16_to_registers_vector(register_list: u16) -> Vec<Arm32GeneralPurposeRegister> {
    let mut result = Vec::<Arm32GeneralPurposeRegister>::new();
    for i in 0..=15 {
        if (register_list & (1u16 << i)) != 0 {
            result.push(Arm32GeneralPurposeRegister::from_operand_bits(i));
        }
    }

    result
}

//


pub fn convert_registers_slice_to_gpr_register_list_u8_and_p_u1(registers: &[Arm32GeneralPurposeRegister]) -> Result<(/*register_list: */u8, /*p: u1*/u8), EncodeError> {
    for register in registers {
        let register_as_operand_bits = register.as_operand_bits();
        match register_as_operand_bits {
            0..=7 | 15 => {
                // R0-R7 and R15 (PC) are allowed
            },
            _ => {
                return Err(EncodeError::RegisterNotEncodable { field: "registers", detail: "a POP register list may only contain R0-R7 and PC (R15)" });
            }
        }
    }

    let register_list_as_u16 = convert_registers_slice_to_gpr_register_list_u16(registers)?;

    let p = ((register_list_as_u16 >> 15) & 0b0000_0001) as u8;
    let register_list = (register_list_as_u16 & 0xFF) as u8;

    Ok((register_list, p))
}

pub fn convert_registers_slice_to_gpr_register_list_u8_and_m_u1(registers: &[Arm32GeneralPurposeRegister]) -> Result<(/*register_list: */u8, /*m: u1*/u8), EncodeError> {
    for register in registers {
        let register_as_operand_bits = register.as_operand_bits();
        match register_as_operand_bits {
            0..=7 | 14 => {
                // R0-R7 and R14 (LR) are allowed
            },
            _ => {
                return Err(EncodeError::RegisterNotEncodable { field: "registers", detail: "a PUSH register list may only contain R0-R7 and LR (R14)" });
            }
        }
    }

    let register_list_as_u16 = convert_registers_slice_to_gpr_register_list_u16(registers)?;

    let m = ((register_list_as_u16 >> 14) & 0b0000_0001) as u8;
    let register_list = (register_list_as_u16 & 0xFF) as u8;

    Ok((register_list, m))
}

pub fn convert_low_registers_slice_to_gpr_register_list_u8(registers: &[Arm32LowGeneralPurposeRegister]) -> Result<u8, EncodeError> {
    let mut register_list: u8 = 0;
    for register in registers {
        let register_as_operand_bits = register.as_operand_bits();
        match register_as_operand_bits {
            0..=7 => {
                // R0-R7 are allowed
            },
            _ => {
                return Err(EncodeError::RegisterNotEncodable { field: "registers", detail: "this register list may only contain R0-R7" });
            }
        }

        register_list |= 1u8 << register_as_operand_bits;
    }

    Ok(register_list)
}

pub fn convert_registers_slice_to_gpr_register_list_u16(registers: &[Arm32GeneralPurposeRegister]) -> Result<u16, EncodeError> {
    let mut register_list: u16 = 0;
    for register in registers {
        let register_as_operand_bits = register.as_operand_bits();
        match register_as_operand_bits {
            0..=15 => {
                // R0-R15 are allowed
            },
            _ => {
                // NOTE: this code path should not be possible with Arm32 as Arm32 only has 16 general purpose registers
                return Err(EncodeError::RegisterNotEncodable { field: "registers", detail: "register out of range R0-R15" });
            }
        }

        register_list |= 1u16 << register_as_operand_bits;
    }

    Ok(register_list)
}
