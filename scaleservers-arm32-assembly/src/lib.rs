// Copyright (c) Scaleservers LLC

// Feature-gated `no_std`: the default build is `std`, but `--no-default-features` builds against
// `core` + `alloc` only. The `extern crate alloc;` below (an item) must follow ALL inner `#![...]`
// attributes, so it sits just after the `#![allow(...)]` block.
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
// ARM instruction encodings are bit-fields, so literals group by field width (e.g. 0xFFE0_0F80, masks like
// 0b1111_1000_0000_0000_1101_0000_0000_0000) rather than by uniform nibbles -- clippy's uniform-grouping
// heuristic is noise here.
#![allow(clippy::unusual_byte_groupings)]
// Encode helpers take one argument per instruction field; >7 args is natural for an ISA codec, not a smell.
#![allow(clippy::too_many_arguments)]

// `alloc` supplies the heap collections (`Vec`/`String`/`Box`) the codec returns under `no_std`; `#[macro_use]`
// brings the `vec!`/`format!` macros into scope crate-wide so per-file imports stay minimal. (Placed after the
// inner attributes / crate doc comment, since `extern crate` is an item and inner attributes must precede it.)
#[macro_use]
extern crate alloc;

mod enums;
pub use enums::{
    Arm32GeneralPurposeRegister,
    Arm32LowGeneralPurposeRegister,
    Arm32SinglePrecisionRegister,
    Arm32DoublePrecisionRegister,
    Arm32QuadwordRegister,
    ArmT32CpsPrimaskEffect,
    ArmT32FpDataOperation3,
    ArmT32FpDataOperation2,
    ArmT32IndexMode,
    ArmT32InstructionCondition,
    ArmT32MemoryBarrierOption,
    ArmT32ParallelOperation,
    ArmT32ParallelPrefix,
    ArmT32RegisterShift,
    ArmT32SpecialRegister,
    // neutral ARM-wide aliases (the 4-bit condition code and barrel-shift operand are shared by A32 + T32)
    Arm32Condition,
    Arm32RegisterShift,
    Arm32ShiftType,
    Arm32ExtendType,
    Arm32ParallelOperation,
    Arm32ParallelPrefix,
    Arm32IndexMode,
    Arm32MemoryOffset,
    Arm32MemoryOffset8,
    Arm32BlockAddressMode,
    Arm32CpsMode,
    Arm32FpDataOperation3,
    Arm32FpDataOperation2,
    Arm32VselCondition,
    Arm32DirectedRound,
    Arm32VrintMode,
    Arm32NeonSize,
    Arm32NeonIntegerOp,
    Arm32NeonFloatOp,
    Arm32NeonBitwiseOp,
    Arm32NeonMisc2SizedOp,
    Arm32NeonMisc2FixedOp,
    Arm32NeonNarrowOp,
    Arm32NeonDiffLongOp,
    Arm32NeonDiffWideOp,
    Arm32NeonDiffNarrowOp,
    Arm32NeonScalarOp,
    Arm32NeonScalarLongOp,
    Arm32NeonShiftOp,
    Arm32NeonShiftNarrowOp,
    Arm32NeonLoadStoreAddress,
    Arm32NeonAesOp,
    Arm32NeonSha3Op,
    Arm32NeonSha2Op,
    Arm32MveVectorRegister,
    Arm32VmovLaneSize,
    Arm32MveSize,
    Arm32MveFloatSize,
    Arm32MveIntArithOp,
    Arm32MveBitwiseOp,
    Arm32MveFloatArithOp,
    Arm32MveVecScalarIntOp,
    Arm32MveVecScalarFloatOp,
    Arm32MveShiftImmOp,
    Arm32MveMisc2Op,
    Arm32MveMisc2FloatOp,
    Arm32MveReduceOp,
    Arm32MveLongMacOp,
    Arm32MveShiftNarrowOp,
    Arm32MveQMovnKind,
    Arm32MveFloatReduceOp,
    Arm32MveVrintOp,
    Arm32MveVcmpCondition,
    mve_predicate_mask_from_suffix,
};

mod errors;
pub use errors::{
    DecodeError,
    EncodeError,
};

pub mod targets;
pub use targets::{
    ArmCpuFeature,
    ArmInstructionRequirement,
    ArmIsaVersion,
    ArmTargetProfile,
};

// Decode-time context for the family-wide Rule R4 ("same bytes, different meaning" -> disambiguate by an
// explicit decode context). Used by `ArmT32Instruction::decode_with` to choose between a CDE custom-datapath
// instruction and a generic coprocessor instruction on coprocessors 0-7.
mod arm32_decode_context;
pub use arm32_decode_context::ArmDecodeContext;

// The VFP modified-immediate codec for VMOV (immediate).
pub mod floating_point_immediate;
pub use floating_point_immediate::{vfp_expand_imm8_to_f32, vfp_expand_imm8_to_f64, vfp_encode_f64_to_imm8};
