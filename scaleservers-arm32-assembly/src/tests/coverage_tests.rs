// Copyright (c) Scaleservers LLC

// Targeted tests closing coverage gaps the robustness/fuzz work surfaced: every error variant renders via
// Display AND Debug (and is a real `std::error::Error`), the interworking union type encodes through to its
// inner set, and both assembly flavors emit.

use crate::enums::{
    Arm32MveBitwiseOp, Arm32MveFloatArithOp, Arm32MveFloatReduceOp, Arm32MveFloatSize,
    Arm32MveIntArithOp, Arm32MveLongMacOp, Arm32MveMisc2FloatOp, Arm32MveMisc2Op, Arm32MveReduceOp,
    Arm32MveShiftImmOp, Arm32MveShiftNarrowOp, Arm32MveSize, Arm32MveVcmpCondition,
    Arm32MveVecScalarFloatOp, Arm32MveVecScalarIntOp, Arm32MveVrintOp, lob_branch_hw1,
    lob_branch_offset, lob_size_field, lob_size_from_field, mve_dualmac_decode_size,
    mve_dualmac_size_bits, mve_long_dualmac_bits, mve_long_dualmac_decode, mve_mem_size_from_log,
    mve_mem_size_log, mve_misc2_float_size_bits, mve_misc2_float_size_from_bits,
    mve_predicate_mask_bits, mve_predicate_mask_from_suffix, mve_predicate_mask_from_word,
    mve_predicate_mask_suffix, mve_shift_esize, mve_shift_size_from_imm6, mve_vcmp_fc_bits,
    mve_vcmp_fc_from_word, mve_vdup_size_bits, mve_vdup_size_from_bits,
};
use crate::{
    Arm32GeneralPurposeRegister, Arm32Instruction, Arm32LowGeneralPurposeRegister,
    ArmAssemblySyntax, ArmCpuFeature, ArmInstructionRequirement, ArmInstructionSet, ArmIsaVersion,
    ArmT32Instruction, ArmT32RegisterShift, ArmT32SpecialRegister, ArmTargetProfile, DecodeError,
    EncodeError,
};

#[test]
fn every_decode_error_variant_renders() {
    for error in [
        DecodeError::IncompleteInstruction,
        DecodeError::InvalidOpcode,
    ] {
        assert!(
            !error.to_string().is_empty(),
            "Display rendered empty for {error:?}"
        );
        assert!(
            !format!("{error:?}").is_empty(),
            "Debug rendered empty for {error:?}"
        );
        let _as_error: &dyn std::error::Error = &error; // the Error impl must exist
    }
}

#[test]
fn every_encode_error_variant_renders() {
    let errors = [
        EncodeError::ImmediateOutOfRange {
            field: "imm7",
            value: 999,
            minimum: 0,
            maximum: 127,
        },
        EncodeError::ImmediateNotAligned {
            field: "offset",
            value: 3,
            required_multiple: 4,
        },
        EncodeError::RegisterNotEncodable {
            field: "rd",
            detail: "use the SP-relative form",
        },
        EncodeError::ConditionNotEncodable {
            field: "cond",
            detail: "AL is not encodable here",
        },
        EncodeError::ShiftNotEncodable {
            field: "shift",
            detail: "only LSL or ASR",
        },
        EncodeError::ModifiedImmediateNotEncodable {
            field: "const",
            value: 0x1234_5678,
        },
        EncodeError::UnsupportedInstructionForTarget {
            required: ArmInstructionRequirement::new(
                ArmIsaVersion::Armv8_1MMainline,
                &[ArmCpuFeature::Mve],
            ),
            target_isa_version: ArmIsaVersion::Armv7M,
        },
    ];
    for error in &errors {
        assert!(
            !error.to_string().is_empty(),
            "Display rendered empty for {error:?}"
        );
        assert!(
            !format!("{error:?}").is_empty(),
            "Debug rendered empty for {error:?}"
        );
        let _as_error: &dyn std::error::Error = error;
    }
}

#[test]
fn interworking_union_encodes_and_both_flavors_emit() {
    // The interworking union dispatches encode/instruction_set to the inner set.
    let thumb = Arm32Instruction::T32(ArmT32Instruction::Nop_T1);
    assert!(matches!(thumb.instruction_set(), ArmInstructionSet::T32));
    assert_eq!(thumb.encode().unwrap(), [0x00, 0xbf]);

    // Both assembly flavors render (the disassembler chooses one).
    let nop = ArmT32Instruction::Nop_T1;
    assert_eq!(nop.to_assembly_string(ArmAssemblySyntax::Llvm), "nop");
    assert_eq!(nop.to_assembly_string(ArmAssemblySyntax::Gnu), "nop");
}

