// Copyright (c) Scaleservers LLC

// 16-bit opcodes (and their bit patterns and masks)
#[allow(non_camel_case_types)]
#[allow(clippy::unusual_byte_groupings)] // patterns are grouped into 4-bit instruction fields for readability
#[repr(u16)]
pub enum ArmT32OpcodePattern_16Bit {
    Adc_Register_T1          = 0b0100_0001_0100_0000, // mask: 0b1111_1111_1100_0000
    Add_Immediate_T1         = 0b0001_1100_0000_0000, // mask: 0b1111_1110_0000_0000
    Add_Immediate_T2         = 0b0011_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Add_Register_T1          = 0b0001_1000_0000_0000, // mask: 0b1111_1110_0000_0000
    Add_Register_T2          = 0b0100_0100_0000_0000, // mask: 0b1111_1111_0000_0000
    Add_SpPlusImmediate_T1   = 0b1010_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Add_SpPlusImmediate_T2   = 0b1011_0000_0000_0000, // mask: 0b1111_1111_1000_0000
    Add_SpPlusRegister_T1    = 0b0100_0100_0110_1000, // mask: 0b1111_1111_0111_1000
    Add_SpPlusRegister_T2    = 0b0100_0100_1000_0101, // mask: 0b1111_1111_1000_0111
    Adr_T1                   = 0b1010_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    And_Register_T1          = 0b0100_0000_0000_0000, // mask: 0b1111_1111_1100_0000
    Asr_Immediate_T1         = 0b0001_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Asr_Register_T1          = 0b0100_0001_0000_0000, // mask: 0b1111_1111_1100_0000
    B_T1                     = 0b1101_0000_0000_0000, // mask: 0b1111_0000_0000_0000
    B_T2                     = 0b1110_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Bic_Register_T1          = 0b0100_0011_1000_0000, // mask: 0b1111_1111_1100_0000
    Bkpt_T1                  = 0b1011_1110_0000_0000, // mask: 0b1111_1111_0000_0000
    Blx_Register_T1          = 0b0100_0111_1000_0000, // mask: 0b1111_1111_1000_0111 // NOTE: the mask excluding the "(#)" bits is: 0b1111_1111_1000_0000
    Bx_T1                    = 0b0100_0111_0000_0000, // mask: 0b1111_1111_1000_0111 // NOTE: the mask excluding the "(#)" bits is: 0b1111_1111_1000_0000
    // ---- ARMv8-M Security Extension (TrustZone-M) ----
    Bxns_T1                  = 0b0100_0111_0000_0100, // BXNS Rm; mask: 0b1111_1111_1000_0111 (bit2=1 distinguishes from BX)
    Blxns_T1                 = 0b0100_0111_1000_0100, // BLXNS Rm; mask: 0b1111_1111_1000_0111 (bit2=1 distinguishes from BLX)
    Cmn_Register_T1          = 0b0100_0010_1100_0000, // mask: 0b1111_1111_1100_0000
    Cmp_Immediate_T1         = 0b0010_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Cmp_Register_T1          = 0b0100_0010_1000_0000, // mask: 0b1111_1111_1100_0000
    Cmp_Register_T2          = 0b0100_0101_0000_0000, // mask: 0b1111_1111_0000_0000
    Cps_T1                   = 0b1011_0110_0110_0010, // mask: 0b1111_1111_1110_1111 // NOTE: the mask excluding the "(#)" bits is: 0b1111_1111_1110_0000
    Eor_Register_T1          = 0b0100_0000_0100_0000, // mask: 0b1111_1111_1100_0000
    Ldm_T1                   = 0b1100_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Ldr_Immediate_T1         = 0b0110_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Ldr_Immediate_T2         = 0b1001_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Ldr_Literal_T1           = 0b0100_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Ldr_Register_T1          = 0b0101_1000_0000_0000, // mask: 0b1111_1110_0000_0000
    Ldrb_Immediate_T1        = 0b0111_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Ldrb_Register_T1         = 0b0101_1100_0000_0000, // mask: 0b1111_1110_0000_0000
    Ldrh_Immediate_T1        = 0b1000_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Ldrh_Register_T1         = 0b0101_1010_0000_0000, // mask: 0b1111_1110_0000_0000
    Ldrsb_Register_T1        = 0b0101_0110_0000_0000, // mask: 0b1111_1110_0000_0000
    Ldrsh_Register_T1        = 0b0101_1110_0000_0000, // mask: 0b1111_1110_0000_0000
    Lsl_Immediate_T1         = 0b0000_0000_0000_0000, // mask: 0b1111_1000_0000_0000 // NOTE: if bits 6..=10 are zeroes (i.e. imm5 == 0), this represents a MOV_Register_T1 instruction instead
    Lsl_Register_T1          = 0b0100_0000_1000_0000, // mask: 0b1111_1111_1100_0000
    Lsr_Immediate_T1         = 0b0000_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Lsr_Register_T1          = 0b0100_0000_1100_0000, // mask: 0b1111_1111_1100_0000
    Mov_Immediate_T1         = 0b0010_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Mov_Register_T1          = 0b0100_0110_0000_0000, // mask: 0b1111_1111_0000_0000
    // Mov_Register_T2          = 0b0000_0000_0000_0000, // mask: 0b1111_1111_1100_0000 // NOTE: this is identical to Lsl_Immediate_T1 with imm5 == 0 (they do the same thing; the value of b00000 means "just move, don't shift")--so we use Lsl_Immediate_T1 as the opcode pattern for both
    Mul_T1                   = 0b0100_0011_0100_0000, // mask: 0b1111_1111_1100_0000
    Mvn_Register_T1          = 0b0100_0011_1100_0000, // mask: 0b1111_1111_1100_0000
    Nop_T1                   = 0b1011_1111_0000_0000, // mask: 0b1111_1111_1111_1111
    Orr_Register_T1          = 0b0100_0011_0000_0000, // mask: 0b1111_1111_1100_0000
    Pop_T1                   = 0b1011_1100_0000_0000, // mask: 0b1111_1110_0000_0000
    Push_T1                  = 0b1011_0100_0000_0000, // mask: 0b1111_1110_0000_0000
    Rev_T1                   = 0b1011_1010_0000_0000, // mask: 0b1111_1111_1100_0000
    Rev16_T1                 = 0b1011_1010_0100_0000, // mask: 0b1111_1111_1100_0000
    Revsh_T1                 = 0b1011_1010_1100_0000, // mask: 0b1111_1111_1100_0000
    Ror_Register_T1          = 0b0100_0001_1100_0000, // mask: 0b1111_1111_1100_0000
    Rsb_Immediate_T1         = 0b0100_0010_0100_0000, // mask: 0b1111_1111_1100_0000
    Sbc_Register_T1          = 0b0100_0001_1000_0000, // mask: 0b1111_1111_1100_0000
    Sev_T1                   = 0b1011_1111_0100_0000, // mask: 0b1111_1111_1111_1111
    Stm_T1                   = 0b1100_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Str_Immediate_T1         = 0b0110_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Str_Immediate_T2         = 0b1001_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Str_Register_T1          = 0b0101_0000_0000_0000, // mask: 0b1111_1110_0000_0000
    Strb_Immediate_T1        = 0b0111_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Strb_Register_T1         = 0b0101_0100_0000_0000, // mask: 0b1111_1110_0000_0000
    Strh_Immediate_T1        = 0b1000_0000_0000_0000, // mask: 0b1111_1000_0000_0000
    Strh_Register_T1         = 0b0101_0010_0000_0000, // mask: 0b1111_1110_0000_0000
    Sub_Immediate_T1         = 0b0001_1110_0000_0000, // mask: 0b1111_1110_0000_0000
    Sub_Immediate_T2         = 0b0011_1000_0000_0000, // mask: 0b1111_1000_0000_0000
    Sub_Register_T1          = 0b0001_1010_0000_0000, // mask: 0b1111_1110_0000_0000
    Sub_SpMinusImmediate_T1  = 0b1011_0000_1000_0000, // mask: 0b1111_1111_1000_0000
    Svc_T1                   = 0b1101_1111_0000_0000, // mask: 0b1111_1111_0000_0000
    Sxtb_T1                  = 0b1011_0010_0100_0000, // mask: 0b1111_1111_1100_0000
    Sxth_T1                  = 0b1011_0010_0000_0000, // mask: 0b1111_1111_1100_0000
    Tst_Register_T1          = 0b0100_0010_0000_0000, // mask: 0b1111_1111_1100_0000
    Udf_T1                   = 0b1101_1110_0000_0000, // mask: 0b1111_1111_0000_0000
    Uxtb_T1                  = 0b1011_0010_1100_0000, // mask: 0b1111_1111_1100_0000
    Uxth_T1                  = 0b1011_0010_1000_0000, // mask: 0b1111_1111_1100_0000
    Wfe_T1                   = 0b1011_1111_0010_0000, // mask: 0b1111_1111_1111_1111
    Wfi_T1                   = 0b1011_1111_0011_0000, // mask: 0b1111_1111_1111_1111
    Yield_T1                 = 0b1011_1111_0001_0000, // mask: 0b1111_1111_1111_1111
}
