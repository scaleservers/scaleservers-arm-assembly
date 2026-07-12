// Copyright (c) Scaleservers LLC

#![cfg(test)]

pub mod adc_instruction_tests;
pub mod add_instruction_tests;
pub mod adr_instruction_tests;
pub mod and_instruction_tests;
pub mod asr_instruction_tests;
pub mod b_instruction_tests;
pub mod bic_instruction_tests;
pub mod bkpt_instruction_tests;
pub mod bl_instruction_tests;
pub mod blx_instruction_tests;
pub mod bx_instruction_tests;
pub mod cmn_instruction_tests;
pub mod cmp_instruction_tests;
pub mod cps_instruction_tests;
pub mod dmb_instruction_tests;
pub mod dsb_instruction_tests;
pub mod eor_instruction_tests;
pub mod isb_instruction_tests;
pub mod ldm_instruction_tests;
pub mod ldr_instruction_tests;
pub mod ldrb_instruction_tests;
pub mod ldrh_instruction_tests;
pub mod ldrsb_instruction_tests;
pub mod ldrsh_instruction_tests;
pub mod lsl_instruction_tests;
pub mod lsr_instruction_tests;
pub mod mov_instruction_tests;
pub mod mrs_instruction_tests;
pub mod msr_instruction_tests;
pub mod mul_instruction_tests;
pub mod mvn_instruction_tests;
pub mod nop_instruction_tests;
pub mod orr_instruction_tests;
pub mod pop_instruction_tests;
pub mod push_instruction_tests;
pub mod rev16_instruction_tests;
pub mod rev_instruction_tests;
pub mod revsh_instruction_tests;
pub mod ror_instruction_tests;
pub mod rsb_instruction_tests;
pub mod sbc_instruction_tests;
pub mod sev_instruction_tests;
pub mod stm_instruction_tests;
pub mod str_instruction_tests;
pub mod strb_instruction_tests;
pub mod strh_instruction_tests;
pub mod sub_instruction_tests;
pub mod svc_instruction_tests;
pub mod sxtb_instruction_tests;
pub mod sxth_instruction_tests;
pub mod tst_instruction_tests;
pub mod udf_instruction_tests;
pub mod uxtb_instruction_tests;
pub mod uxth_instruction_tests;
pub mod wfe_instruction_tests;
pub mod wfi_instruction_tests;
pub mod yield_instruction_tests;

// cross-cutting tests (not tied to a single mnemonic)
pub mod armv8m_tests;
pub mod encode_error_tests;
pub mod modified_immediate_tests;
pub mod targets_tests;
