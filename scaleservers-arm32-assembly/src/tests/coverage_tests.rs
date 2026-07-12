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

// The MVE operation taxonomies (`enums/arm32_mve_operations.rs`) are large bit-coded field enums: each op
// reports a `base_word`, a `mnemonic`, sometimes a `type_prefix`, and recovers from its signature. The
// hand-written per-instruction MVE tests reach only a couple of variants per family, leaving most of the
// `match self` arms (and the standalone bit-codec helpers) unexecuted. Following the coverage convention's
// recipe (one exhaustive round-trip walk per field enum), this exercises EVERY variant's accessor arms and
// the signature round-trip, plus every standalone size/predicate/loop helper across its valid domain -- so the
// rarely-reached arms are covered without chasing printer/operand-combination branches elsewhere.
#[test]
fn mve_field_enums_walk_every_variant() {
    // --- size enums: bit field round-trip + width digits in both directions ---
    for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
        assert_eq!(Arm32MveSize::from_size_bits(size.size_bits()), Some(size));
        assert!(!size.width_digits().is_empty());
    }
    assert_eq!(Arm32MveSize::from_size_bits(0b11), None); // reserved
    for fsize in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
        assert_eq!(Arm32MveFloatSize::from_size_bit(fsize.size_bit()), fsize);
        assert!(!fsize.width_digits().is_empty());
    }

    // --- 3-reg integer / bitwise / float arithmetic: base_word + mnemonic + signature round-trip ---
    // `from_signature` matches against `base_word()` directly, and `base_word()` is already the canonical
    // signature (operand + size fields zeroed), so it is the exact input that must round-trip.
    for op in Arm32MveIntArithOp::ALL {
        assert!(!op.mnemonic().is_empty());
        let _ = op.type_prefix();
        assert_eq!(Arm32MveIntArithOp::from_signature(op.base_word()), Some(op));
    }
    for op in Arm32MveBitwiseOp::ALL {
        assert!(!op.mnemonic().is_empty());
        assert_eq!(Arm32MveBitwiseOp::from_signature(op.base_word()), Some(op));
    }
    for op in Arm32MveFloatArithOp::ALL {
        assert!(!op.mnemonic().is_empty());
        assert_eq!(
            Arm32MveFloatArithOp::from_signature(op.base_word()),
            Some(op)
        );
    }

    // --- vector-by-scalar integer / float ---
    for op in Arm32MveVecScalarIntOp::ALL {
        assert!(!op.mnemonic().is_empty());
        let _ = op.type_prefix();
        assert_eq!(
            Arm32MveVecScalarIntOp::from_signature(op.base_word()),
            Some(op)
        );
    }
    for op in Arm32MveVecScalarFloatOp::ALL {
        assert!(!op.mnemonic().is_empty());
        assert_eq!(
            Arm32MveVecScalarFloatOp::from_signature(op.base_word()),
            Some(op)
        );
    }

    // --- shift-by-immediate: base_word, is_left_shift, type_prefix (incl. the None bit-insert arm) ---
    for op in Arm32MveShiftImmOp::ALL {
        assert!(!op.mnemonic().is_empty());
        let _ = op.is_left_shift();
        let _ = op.type_prefix();
        assert_eq!(Arm32MveShiftImmOp::from_signature(op.base_word()), Some(op));
    }

    // --- 2-reg misc (sized int + float) ---
    for op in Arm32MveMisc2Op::ALL {
        assert!(!op.mnemonic().is_empty());
        let _ = op.type_prefix();
        assert_eq!(Arm32MveMisc2Op::from_signature(op.base_word()), Some(op));
    }
    for op in Arm32MveMisc2FloatOp::ALL {
        assert!(!op.mnemonic().is_empty());
        assert_eq!(
            Arm32MveMisc2FloatOp::from_signature(op.base_word()),
            Some(op)
        );
    }

    // --- cross-lane reductions (int + float) ---
    for op in Arm32MveReduceOp::ALL {
        assert!(!op.mnemonic().is_empty());
        let _ = op.type_prefix();
        assert_eq!(Arm32MveReduceOp::from_signature(op.base_word()), Some(op));
    }
    for op in Arm32MveFloatReduceOp::ALL {
        assert!(!op.mnemonic().is_empty());
        assert_eq!(
            Arm32MveFloatReduceOp::from_signature(op.base_word()),
            Some(op)
        );
    }

    // --- VRINT modes ---
    for op in Arm32MveVrintOp::ALL {
        assert!(!op.mnemonic().is_empty());
        assert_eq!(Arm32MveVrintOp::from_signature(op.base_word()), Some(op));
    }

    // --- non-long dual-MAC size/sign codec: every (subtract, unsigned, size) round-trips through the bits ---
    for subtract in [false, true] {
        for unsigned in [false, true] {
            for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
                let bits = mve_dualmac_size_bits(subtract, unsigned, size);
                let (decoded_unsigned, decoded_size) = mve_dualmac_decode_size(subtract, bits)
                    .expect("dual-MAC size bits must decode");
                assert_eq!(decoded_size, size);
                // The subtract form is signed-only (it repurposes bit28), so `unsigned` only round-trips on add.
                if !subtract {
                    assert_eq!(decoded_unsigned, unsigned);
                }
            }
        }
    }

    // --- long dual-MAC: every op + (unsigned, size in {16,32}) round-trips, and from_flags is exercised ---
    for op in [
        Arm32MveLongMacOp::Vmlaldav,
        Arm32MveLongMacOp::Vmlsldav,
        Arm32MveLongMacOp::Vrmlaldavh,
        Arm32MveLongMacOp::Vrmlsldavh,
    ] {
        assert!(!op.mnemonic().is_empty());
        let _ = op.subtract();
        let _ = op.rounding_high();
        assert_eq!(
            Arm32MveLongMacOp::from_flags(op.subtract(), op.rounding_high()),
            op
        );
        // rounding-high forms are 32-bit only; the plain forms support .16 and .32.
        let sizes: &[Arm32MveSize] = if op.rounding_high() {
            &[Arm32MveSize::I32]
        } else {
            &[Arm32MveSize::I16, Arm32MveSize::I32]
        };
        for &size in sizes {
            for unsigned in [false, true] {
                // `mve_long_dualmac_bits` already sets bit0 = subtract, so `bits` is the full opcode word.
                let bits = mve_long_dualmac_bits(op, unsigned, size);
                let (decoded_op, decoded_unsigned, decoded_size) =
                    mve_long_dualmac_decode(bits).expect("long dual-MAC bits must decode");
                assert_eq!(decoded_op, op);
                assert_eq!(decoded_size, size);
                if !op.subtract() {
                    assert_eq!(decoded_unsigned, unsigned);
                }
            }
        }
    }

    // --- shift-and-narrow: opcode_bits <-> from_word for every op and both signedness selectors ---
    for op in [
        Arm32MveShiftNarrowOp::Vshrn,
        Arm32MveShiftNarrowOp::Vrshrn,
        Arm32MveShiftNarrowOp::Vqshrn,
        Arm32MveShiftNarrowOp::Vqrshrn,
        Arm32MveShiftNarrowOp::Vqshrun,
        Arm32MveShiftNarrowOp::Vqrshrun,
    ] {
        assert!(!op.mnemonic().is_empty());
        for unsigned in [false, true] {
            let (bit28, bit76, bit0) = op.opcode_bits(unsigned);
            let (decoded_op, decoded_unsigned) =
                Arm32MveShiftNarrowOp::from_word(bit28, bit76, bit0)
                    .expect("shift-narrow opcode bits must decode");
            assert_eq!(decoded_op, op);
            // only VQSHRN/VQRSHRN carry a real signedness bit; the others fix bit28 as the rounding selector.
            if matches!(
                op,
                Arm32MveShiftNarrowOp::Vqshrn | Arm32MveShiftNarrowOp::Vqrshrn
            ) {
                assert_eq!(decoded_unsigned, unsigned);
            }
        }
    }

    // --- VCMP conditions: fc round-trip, mnemonic, type_prefix, and the from_mnemonic alias path ---
    for cond in Arm32MveVcmpCondition::ALL {
        assert_eq!(Arm32MveVcmpCondition::from_fc(cond.fc()), cond);
        let mnemonic = cond.mnemonic();
        assert!(!mnemonic.is_empty());
        let _ = cond.type_prefix();
        assert_eq!(Arm32MveVcmpCondition::from_mnemonic(mnemonic), Some(cond));
        // the fc-bit scatter differs between register and scalar forms; both must round-trip.
        for scalar in [false, true] {
            assert_eq!(
                mve_vcmp_fc_from_word(mve_vcmp_fc_bits(cond, scalar), scalar),
                cond.fc()
            );
        }
    }
    assert_eq!(
        Arm32MveVcmpCondition::from_mnemonic("hs"),
        Some(Arm32MveVcmpCondition::Cs)
    ); // alias
    assert_eq!(Arm32MveVcmpCondition::from_mnemonic("zz"), None);

    // --- predicate-mask suffix codec: all 16 non-zero masks round-trip through the t/e suffix and the bits ---
    for mask in 1u8..=15 {
        let suffix = mve_predicate_mask_suffix(mask);
        assert_eq!(
            mve_predicate_mask_from_suffix(suffix),
            Some(mask),
            "mask {mask:#06b} suffix {suffix:?}"
        );
        assert_eq!(
            mve_predicate_mask_from_word(mve_predicate_mask_bits(mask)),
            mask
        );
    }
    assert_eq!(mve_predicate_mask_from_suffix("nonsense"), None);

    // --- low-overhead-loop size field: plain + every tail-predicate element size ---
    for tp in [None, Some(8u8), Some(16), Some(32), Some(64)] {
        assert_eq!(lob_size_from_field(lob_size_field(tp)), Some(tp));
    }
    assert_eq!(lob_size_from_field(0b101), None); // reserved size field
    // loop-branch offset codec (both directions): a few representative forward/backward magnitudes.
    for offset in [0i32, 4, 100, -4, -200, 2046] {
        let hw1 = lob_branch_hw1(offset);
        assert_eq!(lob_branch_offset(hw1, false), offset.abs());
        assert_eq!(lob_branch_offset(hw1, true), -offset.abs());
    }

    // --- VDUP {B,E} size pair, memory-access size log, shift element size, 2-reg-misc float size ---
    for size in [Arm32MveSize::I8, Arm32MveSize::I16, Arm32MveSize::I32] {
        let (b, e) = mve_vdup_size_bits(size);
        assert_eq!(mve_vdup_size_from_bits(b, e), Some(size));
        assert!(mve_shift_esize(size) >= 8);
    }
    assert_eq!(mve_vdup_size_from_bits(1, 1), None); // {1,1} is not a valid size
    for bytes in [8u8, 16, 32, 64] {
        assert_eq!(mve_mem_size_from_log(mve_mem_size_log(bytes)), bytes);
    }
    // shift element size recovered from imm6 (the highest set bit selects the width); imm6 < 8 -> None.
    assert_eq!(mve_shift_size_from_imm6(0b001000), Some(Arm32MveSize::I8));
    assert_eq!(mve_shift_size_from_imm6(0b010000), Some(Arm32MveSize::I16));
    assert_eq!(mve_shift_size_from_imm6(0b100000), Some(Arm32MveSize::I32));
    assert_eq!(mve_shift_size_from_imm6(0b000100), None);
    for fsize in [Arm32MveFloatSize::F16, Arm32MveFloatSize::F32] {
        assert_eq!(
            mve_misc2_float_size_from_bits(mve_misc2_float_size_bits(fsize)),
            Some(fsize)
        );
    }
    assert_eq!(mve_misc2_float_size_from_bits(0b00), None);
}
