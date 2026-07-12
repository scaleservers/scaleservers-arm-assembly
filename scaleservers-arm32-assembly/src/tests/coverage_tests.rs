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

// Exhaustively exercise the bit-coded field enums in BOTH directions, so the rarely-used variants the
// per-instruction tests don't reach (most MSR/MRS special registers, every register number, each shift kind)
// are still covered. Every field round-trips: `from_operand_bits(b).as_operand_bits() == b`.
#[test]
fn field_enums_round_trip_exhaustively() {
    // Special registers: all 256 field values (the named encodings + the `Reserved` catch-all).
    for bits in 0u8..=0xFF {
        assert_eq!(
            ArmT32SpecialRegister::from_operand_bits(bits).as_operand_bits(),
            bits,
            "special register {bits:#04x} did not round-trip"
        );
    }
    // Low general-purpose registers (R0..R7): both the 3-bit accessor and the widening to a full GPR.
    for bits in 0u8..8 {
        let reg = Arm32LowGeneralPurposeRegister::from_operand_bits(bits);
        assert_eq!(
            reg.as_operand_bits(),
            bits,
            "low gpr {bits} did not round-trip"
        );
        assert_eq!(
            reg.as_general_purpose_register().as_operand_bits(),
            bits,
            "low gpr {bits} widened wrong"
        );
    }
    // General-purpose registers (R0..R15): every 4-bit encoding round-trips, and the low/high classification
    // + narrowing accessors are exercised across both halves of the register file.
    for bits in 0u8..16 {
        let reg = Arm32GeneralPurposeRegister::from_operand_bits(bits);
        assert_eq!(reg.as_operand_bits(), bits, "gpr {bits} did not round-trip");
        assert_eq!(
            reg.is_low_general_purpose_register(),
            bits < 8,
            "gpr {bits} low-classification"
        );
        assert_eq!(
            reg.is_high_general_purpose_register(),
            bits >= 8,
            "gpr {bits} high-classification"
        );
        assert_eq!(
            reg.as_low_general_purpose_register().is_some(),
            bits < 8,
            "gpr {bits} narrowing"
        );
    }
    // Register-shift kinds: each reports its 2-bit type field, and the no-shift helper is recognized.
    for shift in [
        ArmT32RegisterShift::Lsl(3),
        ArmT32RegisterShift::Lsr(1),
        ArmT32RegisterShift::Asr(31),
        ArmT32RegisterShift::Ror(7),
        ArmT32RegisterShift::Rrx,
    ] {
        let _ = shift.type_bits();
    }
    assert!(ArmT32RegisterShift::none().is_none());
    assert!(!ArmT32RegisterShift::Lsr(1).is_none());
}

// Every named target profile constructs and answers the gating queries (the profiles the targeting tests
// don't all instantiate).
#[test]
fn every_target_profile_constructor_builds_and_queries() {
    let profiles = [
        ArmTargetProfile::armv6m(),
        ArmTargetProfile::armv7m(),
        ArmTargetProfile::armv7em(),
        ArmTargetProfile::armv8m_baseline(),
        ArmTargetProfile::armv8m_mainline(),
        ArmTargetProfile::armv8_1m_mve(),
        ArmTargetProfile::permissive(),
        ArmTargetProfile::armv7ar(),
        ArmTargetProfile::armv8a_aarch32(),
        ArmTargetProfile::armv4t(),
        ArmTargetProfile::armv5te(),
        ArmTargetProfile::armv6_arm(),
        ArmTargetProfile::permissive_aarch32(),
    ];
    let requirement = ArmInstructionRequirement::new(ArmIsaVersion::Armv7M, &[]);
    for profile in &profiles {
        let _ = profile.isa_version();
        let _ = profile.has_feature(ArmCpuFeature::Mve);
        let _ = profile.supports(&requirement);
    }
    // `new` + a positive feature query: an explicitly-built MVE profile reports the feature.
    let custom = ArmTargetProfile::new(ArmIsaVersion::Armv7M, &[ArmCpuFeature::Mve]);
    assert!(custom.has_feature(ArmCpuFeature::Mve));
    // and the MVE-capable named profile does too.
    assert!(ArmTargetProfile::armv8_1m_mve().has_feature(ArmCpuFeature::Mve));
}

