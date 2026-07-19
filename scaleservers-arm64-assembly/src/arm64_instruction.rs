// Copyright (c) Scaleservers LLC

#![allow(non_camel_case_types)]

// `Vec` is not in the `no_std` prelude; pull it from `alloc` (the `vec!` macro comes from the crate-level
// `#[macro_use] extern crate alloc`).
use alloc::vec::Vec;

#[cfg(feature = "experimental")]
use crate::enums::Arm64LsfeOp;
#[cfg(feature = "experimental")]
use crate::enums::Arm64PointerAuthLrLabelOp;
use crate::enums::{
    Arm64AtomicOp, Arm64AtomicOrdering, Arm64BarrierOption, Arm64BranchRecordBufferOp,
    Arm64BtiTarget, Arm64CmpBranchCond, Arm64CmpBranchImmCond, Arm64ComplexRotation,
    Arm64Condition, Arm64Crc32Op, Arm64CryptoFamily, Arm64CsscMinMaxOp, Arm64CsscUnaryOp,
    Arm64ExtendOption, Arm64FloatPrecision, Arm64FloatRegister, Arm64Fp8ConvertLongOp,
    Arm64FpToIntRoundOp, Arm64FprcvtOp, Arm64GcsExceptionOp, Arm64GcsRegisterOp,
    Arm64GeneralPurposeRegister, Arm64Imm9Mode, Arm64LoadStoreIndex, Arm64LoadStoreSize,
    Arm64Lse128Op, Arm64LsuiAtomicOp, Arm64LsuiPairIndex, Arm64MemoryExtend, Arm64MopsStage,
    Arm64MteBlockOp, Arm64MteDataOp, Arm64PacBranchOp, Arm64PacHintOp, Arm64PacOp,
    Arm64PacReturnOp, Arm64PointerAuthLrOp, Arm64PredicateAsCounter, Arm64PredicateRegister,
    Arm64PstateField, Arm64RcpcUnscaledOp, Arm64RcwAtomicOp, Arm64RegisterWidth,
    Arm64ScalableVectorRegister, Arm64ScalarByElementLongOp, Arm64ScalarByElementOp,
    Arm64ScalarFixedConvertOp, Arm64ScalarFpPairwiseOp, Arm64ScalarFpThreeSameOp,
    Arm64ScalarFpTwoMiscOp, Arm64ScalarFrintTsOp, Arm64ScalarNarrowOp, Arm64ScalarShiftImmOp,
    Arm64ScalarShiftNarrowOp, Arm64ScalarThreeSameOp, Arm64ScalarTwoMiscOp, Arm64Sme2ClampKind,
    Arm64Sme2FpCvtNarrowOp, Arm64Sme2MinMaxOp, Arm64Sme2ShiftMulOp, Arm64Sme2UnaryOp,
    Arm64Sme2ZaDotOp, Arm64Sme2ZaMlalOp, Arm64Sme2ZaMlalWiden, Arm64Sme2ZaVdotOp,
    Arm64SmeFpPrecision, Arm64SmeMop4DoubleKind, Arm64SmeMop4Kind, Arm64SmeStateTarget,
    Arm64SmeTileSize, Arm64SmeTmopOp, Arm64StoreTagOp, Arm64Sve2BitPermuteOp,
    Arm64Sve2ExtractNarrowOp, Arm64Sve2FpPairwiseOp, Arm64Sve2FpUpdownOp, Arm64Sve2HalvingOp,
    Arm64Sve2MulOp, Arm64Sve2NarrowHighOp, Arm64Sve2NarrowShiftOp, Arm64Sve2PairwiseOp,
    Arm64Sve2SatAddSubOp, Arm64Sve2ShiftLeftPredOp, Arm64Sve2TernaryLogicalOp,
    Arm64Sve2UnaryPredOp, Arm64Sve2WhileCompareOp, Arm64Sve2WidenIndexedOp, Arm64Sve2WideningOp,
    Arm64SveAdrMode, Arm64SveBf16BinaryOp, Arm64SveBitwiseImmOp, Arm64SveBitwiseLogicalOp,
    Arm64SveClampOp, Arm64SveCmpImmSignedOp, Arm64SveCmpImmUnsignedOp, Arm64SveContiguousLoadType,
    Arm64SveCryptoBinaryOp, Arm64SveCryptoDestructiveOp, Arm64SveDotIndexedOp,
    Arm64SveFp8ConvertOp, Arm64SveFp8NarrowOp, Arm64SveFpBinUnpredOp, Arm64SveFpCompareOp,
    Arm64SveFpConvertKind, Arm64SveFpFmaOp, Arm64SveFpIndexedOp, Arm64SveFpPredBinOp,
    Arm64SveFpReductionOp, Arm64SveFpUnaryOp, Arm64SveIndexOperand, Arm64SveIntBinUnpredOp,
    Arm64SveIntCompareOp, Arm64SveIntIndexedOp, Arm64SveIntMacOp, Arm64SveIntReductionOp,
    Arm64SveMatmulOp, Arm64SveNarrowConvertOp, Arm64SveOffsetMode, Arm64SvePredCountOp,
    Arm64SvePredIntBinOp, Arm64SvePredLogicalOp, Arm64SvePredShiftVectorOp, Arm64SvePredUnaryOp,
    Arm64SveQuadPermuteOp, Arm64SveQuadReduceFpOp, Arm64SveQuadReduceIntOp, Arm64SveReverseWidth,
    Arm64SveShiftImmOp, Arm64SveShiftNarrowOp, Arm64SveStructureCount, Arm64SveWhileOp,
    Arm64SystemHintOp, Arm64SystemRegister, Arm64VectorAcrossLanesOp, Arm64VectorAddPairwiseLongOp,
    Arm64VectorAesOp, Arm64VectorArrangement, Arm64VectorBitwiseOp, Arm64VectorByElementLongOp,
    Arm64VectorByElementOp, Arm64VectorCompareZeroOp, Arm64VectorCrypto2Op, Arm64VectorCrypto3Op,
    Arm64VectorCrypto4Op, Arm64VectorElement, Arm64VectorFmlalOp, Arm64VectorFp16AcrossOp,
    Arm64VectorFp16ByElementOp, Arm64VectorFp16TwoMiscOp, Arm64VectorFpConvertLengthOp,
    Arm64VectorFpThreeSameOp, Arm64VectorFpUnaryOp, Arm64VectorImmediateShift,
    Arm64VectorIntThreeSameOp, Arm64VectorIntUnaryOp, Arm64VectorLoadStoreSize,
    Arm64VectorMatMulOp, Arm64VectorMixedDotOp, Arm64VectorModifiedImmediateOp,
    Arm64VectorNarrowOp, Arm64VectorPermuteOp, Arm64VectorRdmOp, Arm64VectorSha2Op,
    Arm64VectorSha3Op, Arm64VectorShiftImmOp, Arm64VectorShiftLongNarrowOp, Arm64VectorSm3TtOp,
    Arm64VectorStructureKind, Arm64VectorThreeDifferentOp, Arm64WhileCounterOp,
};
use crate::targets::{Arm64InstructionRequirement, Arm64TargetProfile};
use crate::{DecodeError, EncodeError, decode_bitmask, encode_bitmask};

/// The AArch64 (A64) instruction model -- the single central type of this crate.
///
/// AArch64, unlike the 32-bit architecture, has ONE fixed-width instruction set (there is no Thumb/ARM
/// split), so this is a single enum (contrast `scaleservers-arm32-assembly`, which keeps `ArmT32Instruction`
/// and `ArmA32Instruction` as separate types). Every variant encodes to **exactly four little-endian
/// bytes**.
///
/// Each variant offers the family signatures: [`Self::encode`] (model -> machine bytes), [`Self::decode`]
/// (bytes -> model, the exact inverse of `encode`), [`Self::encode_for_target`] (target/ISA gating), and
/// `to_assembly_string` (model -> UAL text, in LLVM or GNU flavor; defined in the `emit` module).
///
/// This models a near-complete A64 surface: the scalar integer, floating-point and system instructions, the
/// whole Advanced SIMD (NEON) set, and the SVE/SVE2, SME/SME2, FP8 and crypto extension families, across
/// ARMv8.0 through ARMv9.6. See the crate README for the coverage summary and the documented gaps.
///
/// Variant naming follows the 32-bit library: `Mnemonic_Form`. A64 has far fewer "encoding numbers" per
/// mnemonic than Thumb, so the form is usually the operand class (`Immediate` / `Register`) rather than a
/// `T1`/`A1` suffix.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Arm64Instruction {
    /// `NOP` -- no operation (the canonical hint `0xD503201F`).
    Nop,

    /// `RET {Xn}` -- return from subroutine, branching to the address in `Xn` (defaults to X30/LR in asm).
    /// Operand: the branch-target register `Xn`.
    Ret(/* xn */ Arm64GeneralPurposeRegister),

    /// `ADD Wd|Xd, Wn|Xn, #imm{, LSL #(0|12)}` -- add of a 12-bit unsigned immediate, optionally shifted left
    /// by 12. The leading `width` selects the 32-bit (`W`) or 64-bit (`X`) form (the `sf` bit). `Rd`/`Rn` use
    /// the SP encoding at 31. Operands: `width`, `xd`, `xn`, `imm12` (0..=4095), `shift12` (true => the
    /// immediate is `<< 12`).
    AddImmediate(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* imm12 */ u16,
        /* shift12 */ bool,
    ),

    /// `SUB Wd|Xd, Wn|Xn, #imm{, LSL #(0|12)}` -- subtract of a 12-bit unsigned immediate, optionally shifted
    /// left by 12. Operands as [`Self::AddImmediate`].
    SubImmediate(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* imm12 */ u16,
        /* shift12 */ bool,
    ),

    /// `ADDS Wd|Xd, Wn|Xn, #imm{, LSL #(0|12)}` -- add of a 12-bit immediate, setting the NZCV flags. `CMN` is
    /// the `Rd == ZR` alias. `Rd` uses ZR at 31, `Rn` uses SP at 31. Operands as [`Self::AddImmediate`].
    AddsImmediate(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* imm12 */ u16,
        /* shift12 */ bool,
    ),

    /// `SUBS Wd|Xd, Wn|Xn, #imm{, LSL #(0|12)}` -- subtract of a 12-bit immediate, setting the NZCV flags. `CMP`
    /// is the `Rd == ZR` alias. Operands as [`Self::AddsImmediate`].
    SubsImmediate(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* imm12 */ u16,
        /* shift12 */ bool,
    ),

    /// `ADDS Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- add of an LSL-shifted register, setting the NZCV flags.
    /// `CMN` is the `Rd == ZR` alias. All three registers use ZR at 31. Operands: `width`, `xd`, `xn`, `xm`,
    /// `amount` (LSL shift, 0..=31 for `W` / 0..=63 for `X`).
    AddsRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* amount */ u8,
    ),

    /// `SUBS Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- subtract of an LSL-shifted register, setting the NZCV flags.
    /// `CMP` is the `Rd == ZR` alias. Operands as [`Self::AddsRegister`].
    SubsRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* amount */ u8,
    ),

    /// `ADD` (extended register): `Rd = Rn + (extend(Rm, option) << amount)`. The **stack-pointer-capable** add
    /// form -- `Rd`/`Rn` name the stack pointer at field `31` (`Rm` names the zero register); `option` selects the
    /// [`Arm64ExtendOption`] and `amount` is the `0..=4` left shift. Operand order: width, Rd, Rn, Rm, option,
    /// amount. The `Rm` register width follows the option ([`Arm64ExtendOption::source_width`]).
    AddExtended(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* rm */ Arm64GeneralPurposeRegister,
        /* option */ Arm64ExtendOption,
        /* amount */ u8,
    ),
    /// `SUB` (extended register). Operands as [`Self::AddExtended`].
    SubExtended(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* rm */ Arm64GeneralPurposeRegister,
        /* option */ Arm64ExtendOption,
        /* amount */ u8,
    ),
    /// `ADDS` (extended register), flag-setting; `CMN` is the `Rd == ZR` alias. `Rd` names the zero register at
    /// field `31`, `Rn` the stack pointer. Operands as [`Self::AddExtended`].
    AddsExtended(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* rm */ Arm64GeneralPurposeRegister,
        /* option */ Arm64ExtendOption,
        /* amount */ u8,
    ),
    /// `SUBS` (extended register), flag-setting; `CMP` is the `Rd == ZR` alias. Operands as
    /// [`Self::AddsExtended`].
    SubsExtended(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* rm */ Arm64GeneralPurposeRegister,
        /* option */ Arm64ExtendOption,
        /* amount */ u8,
    ),

    /// `AND` (immediate): `Rd = Rn AND bitmask(imm)`. `Rd` is **stack-pointer-capable** (field `31` = SP); `Rn`
    /// is the zero-register view. `imm` is the logical-immediate VALUE -- encoded to `(N, immr, imms)` at encode
    /// time via [`crate::encode_bitmask`] and rejected with [`EncodeError::UnrepresentableBitmaskImmediate`] if
    /// it is not a representable bitmask (for `W`, it must also fit in 32 bits). Operand order: width, Rd, Rn,
    /// imm.
    AndImmediate(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* imm */ u64,
    ),
    /// `ORR` (immediate); `MOV` (bitmask immediate) is the `ORR Rd, ZR, #imm` alias. Operands as
    /// [`Self::AndImmediate`].
    OrrImmediate(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* imm */ u64,
    ),
    /// `EOR` (immediate). Operands as [`Self::AndImmediate`].
    EorImmediate(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* imm */ u64,
    ),
    /// `ANDS` (immediate), flag-setting; `TST` is the `Rd == ZR` alias. Unlike AND/ORR/EOR, `Rd` here is the
    /// zero-register view (not SP-capable). Operands as [`Self::AndImmediate`].
    AndsImmediate(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* imm */ u64,
    ),

    /// `MOVZ Wd|Xd, #imm16{, LSL #shift}` -- move a 16-bit immediate into `Rd` (zeroing the rest), placed at
    /// halfword position `hw`. The leading `width` selects the 32-bit (`W`) or 64-bit (`X`) form; for `W`,
    /// `hw` is 0..=1 (shift 0/16), for `X` it is 0..=3 (shift 0/16/32/48). Operands: `width`, `xd`, `imm16`,
    /// `hw`.
    Movz(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* imm16 */ u16,
        /* hw */ u8,
    ),

    /// `MOVK Wd|Xd, #imm16{, LSL #shift}` -- move a 16-bit immediate into the `hw` halfword of `Rd`, keeping
    /// the other bits. Operands as [`Self::Movz`] (`hw` 0..=1 for `W`, 0..=3 for `X`).
    Movk(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* imm16 */ u16,
        /* hw */ u8,
    ),

    /// `MOVN Wd|Xd, #imm16{, LSL #shift}` -- move the bitwise-inverse of a shifted 16-bit immediate into `Rd`:
    /// `Rd = !(imm16 << (hw*16))` (so every bit outside the placed halfword becomes 1). The canonical way to
    /// materialize small negative constants. The leading `width` selects the 32-bit (`W`) or 64-bit (`X`)
    /// form; `hw` ranges as for [`Self::Movz`]. Operands: `width`, `xd`, `imm16`, `hw`.
    Movn(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* imm16 */ u16,
        /* hw */ u8,
    ),

    /// `ADR Xd, label` -- PC-relative byte address: `Xd = PC + offset`. The operand is the signed **byte** offset
    /// from this instruction to `label`, any byte in `+/-1 MiB` (the 21-bit imm is split immlo`[1:0]`@`[30:29]` :
    /// immhi`[20:2]`@`[23:5]`). Always writes a 64-bit `Xd` (no W-form). Operands: `xd`, `offset_bytes`.
    Adr(
        /* xd */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ i32,
    ),

    /// `ADRP Xd, label` -- PC-relative **page** address: `Xd = (PC & !0xFFF) + (offset_bytes)`, where the encoded
    /// 21-bit imm is `offset_bytes >> 12`. The operand is the signed byte offset to the target page, a multiple
    /// of 4096 in `+/-4 GiB` (so `i64`, to hold the full range). Always writes a 64-bit `Xd`. Operands: `xd`,
    /// `page_offset_bytes`.
    Adrp(
        /* xd */ Arm64GeneralPurposeRegister,
        /* page_offset_bytes */ i64,
    ),

    /// `ADD Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- add of a (LSL-)shifted register. The leading `width`
    /// selects the 32-bit (`W`) or 64-bit (`X`) form (the `sf` bit); for `W` the `shift_amount` is 0..=31, for
    /// `X` it is 0..=63. `Rd`/`Rn`/`Rm` use the ZR encoding at 31 (no SP here). Operands: `width`, `xd`, `xn`,
    /// `xm`, `shift_amount`.
    AddRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* shift_amount */ u8,
    ),

    /// `SUB Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- subtract of a (LSL-)shifted register. Operands as
    /// [`Self::AddRegister`].
    SubRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* shift_amount */ u8,
    ),

    /// `ORR Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- bitwise OR of a (LSL-)shifted register. The `MOV (register)`
    /// alias is exactly `ORR Rd, ZR, Rm` (Rn = ZR, amount = 0). Operands as [`Self::AddRegister`].
    OrrRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* shift_amount */ u8,
    ),

    /// `AND Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- bitwise AND of a (LSL-)shifted register. Operands as
    /// [`Self::AddRegister`].
    AndRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
    ),
    /// `BIC Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- AND with the (LSL-)shifted register's bitwise NOT.
    BicRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
    ),
    /// `ORN Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- OR with the shifted register's bitwise NOT (the
    /// `MVN (register)` alias is `ORN Rd, ZR, Rm`).
    OrnRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
    ),
    /// `EOR Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- bitwise exclusive-OR of a (LSL-)shifted register.
    EorRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
    ),
    /// `EON Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- exclusive-OR with the shifted register's bitwise NOT.
    EonRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
    ),
    /// `ANDS Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- AND of a shifted register, setting the NZCV flags (the
    /// `TST (register)` alias is `ANDS ZR, Rn, Rm`).
    AndsRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
    ),
    /// `BICS Wd|Xd, Wn|Xn, Wm|Xm{, LSL #amount}` -- AND with the shifted register's NOT, setting the NZCV flags.
    BicsRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
    ),

    // ---- data-processing (2 source), W/X forms ----
    //
    // Layout (DDI0487 C4.1 "Data-processing (2 source)"): `sf 0 S 11010110 Rm[20:16] opcode[15:10] Rn[9:5]
    // Rd[4:0]`; the leading `width` supplies sf (S=0), and the 6-bit `opcode` field selects the operation. All
    // three registers use the ZR encoding at 31 (no SP), like the shifted-register family above. None of these
    // forms take a shift/operand modifier, so width is the only addition.
    /// `UDIV Wd|Xd, Wn|Xn, Wm|Xm` -- unsigned divide (`Rn / Rm`, truncating toward zero; divide-by-zero yields
    /// 0). The leading `width` selects the 32-bit (`W`) or 64-bit (`X`) form. Operands: `width`, `xd`, `xn`,
    /// `xm`.
    UdivRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
    ),
    /// `SDIV Wd|Xd, Wn|Xn, Wm|Xm` -- signed divide. Operands as [`Self::UdivRegister`].
    SdivRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
    ),
    /// `LSLV Wd|Xd, Wn|Xn, Wm|Xm` -- logical shift left by the value in `Rm` (mod 32/64). `LSL (register)` is an
    /// alias of this (the emitter renders it as `lsl`). Operands as [`Self::UdivRegister`].
    LslvRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
    ),
    /// `LSRV Wd|Xd, Wn|Xn, Wm|Xm` -- logical shift right by `Rm` (mod 32/64); `LSR (register)` is an alias.
    LsrvRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
    ),
    /// `ASRV Wd|Xd, Wn|Xn, Wm|Xm` -- arithmetic shift right by `Rm` (mod 32/64); `ASR (register)` is an alias.
    AsrvRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
    ),
    /// `RORV Wd|Xd, Wn|Xn, Wm|Xm` -- rotate right by `Rm` (mod 32/64); `ROR (register)` is an alias.
    RorvRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
    ),

    // ---- data-processing (3 source) forms ----
    //
    // Layout (DDI0487 C4.1 "Data-processing (3 source)"): `sf 00 11011 op31[23:21] Rm[20:16] o0[15]
    // Ra[14:10] Rn[9:5] Rd[4:0]`; op31 + o0 select the operation. All four registers use the ZR encoding at 31
    // (no SP). MADD/MSUB (`Rd, Rn, Rm, Ra`) are W-capable, so they carry a leading `width` supplying sf.
    // SMULH/UMULH (`Xd, Xn, Xm`) are 64-bit ONLY (sf is fixed to 1 by op31 in their base), so they do NOT carry
    // a width field.
    //
    // The long forms SMADDL/UMADDL/SMSUBL/UMSUBL (and the SMULL/UMULL aliases) mix W and X registers
    // (`Xd, Wn, Wm, Xa`); rather than a per-operand width on this group, they are modeled as the dedicated
    // no-width variants below.
    /// `MADD Wd|Xd, Wn|Xn, Wm|Xm, Wa|Xa` -- multiply-add: `Rd = Ra + Rn * Rm`. The leading `width` selects the
    /// 32-bit (`W`) or 64-bit (`X`) form. `MUL Rd, Rn, Rm` is the alias with `Ra = ZR` (the emitter renders it as
    /// `mul`). Operands: `width`, `xd`, `xn`, `xm`, `xa`.
    MaddRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* xa */ Arm64GeneralPurposeRegister,
    ),
    /// `MSUB Wd|Xd, Wn|Xn, Wm|Xm, Wa|Xa` -- multiply-subtract: `Rd = Ra - Rn * Rm`. `MNEG Rd, Rn, Rm` is the
    /// alias with `Ra = ZR` (the emitter renders it as `mneg`). Operands as [`Self::MaddRegister`].
    MsubRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* xa */ Arm64GeneralPurposeRegister,
    ),
    /// `SMULH Xd, Xn, Xm` -- signed multiply-high: `Xd = (SInt(Xn) * SInt(Xm))[127:64]`. **64-bit only** (the
    /// `sf` bit is fixed to 1 by the encoding), so there is no `W` form and no width field. The encoding's `Ra`
    /// field is fixed to `11111` (no addend operand). Operands: `xd`, `xn`, `xm`.
    SmulhRegister(
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
    ),
    /// `UMULH Xd, Xn, Xm` -- unsigned multiply-high: `Xd = (UInt(Xn) * UInt(Xm))[127:64]`. As
    /// [`Self::SmulhRegister`], it is **64-bit only** and the `Ra` field is fixed to `11111`.
    UmulhRegister(
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
    ),

    // ---- 3-source long multiply (SMADDL / UMADDL / SMSUBL / UMSUBL), W/X-mixed (sf=1 fixed) ----
    //
    // Layout (DDI0487 C6.2, the op31=001/101 members of "Data-processing (3 source)"): `1 00 11011 op31 Rm o0 Ra
    // Rn Rd`, sf=1 ALWAYS (the result + accumulator are 64-bit; the two multiplicands are 32-bit). So these have
    // NO width field -- `Rd`/`Ra` are `Xd`/`Xa`, `Rn`/`Rm` are `Wn`/`Wm`. SMULL/UMULL are the `Xa=XZR` aliases
    // (the emitter renders them as smull/umull). All operands use the ZR encoding at 31.
    /// `SMADDL Xd, Wn, Wm, Xa` -- signed multiply-add long: `Xd = Xa + SInt(Wn) * SInt(Wm)`. `SMULL Xd, Wn, Wm` is
    /// the `Xa=XZR` alias. Operands: `xd`, `wn`, `wm`, `xa`.
    SmaddlRegister(
        /* xd */ Arm64GeneralPurposeRegister,
        /* wn */ Arm64GeneralPurposeRegister,
        /* wm */ Arm64GeneralPurposeRegister,
        /* xa */ Arm64GeneralPurposeRegister,
    ),
    /// `UMADDL Xd, Wn, Wm, Xa` -- unsigned multiply-add long (`UMULL` = the `Xa=XZR` alias). Operands as [`Self::SmaddlRegister`].
    UmaddlRegister(
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
    ),
    /// `SMSUBL Xd, Wn, Wm, Xa` -- signed multiply-subtract long: `Xd = Xa - SInt(Wn) * SInt(Wm)`. Operands as [`Self::SmaddlRegister`].
    SmsublRegister(
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
    ),
    /// `UMSUBL Xd, Wn, Wm, Xa` -- unsigned multiply-subtract long. Operands as [`Self::SmaddlRegister`].
    UmsublRegister(
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
    ),

    /// `ADDPT`/`SUBPT Xd, Xn, Xm` -- FEAT_CPA checked pointer add/subtract (64-bit). `sub` selects `SUBPT`.
    AddSubCheckedPointer {
        sub: bool,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `MADDPT`/`MSUBPT Xd, Xn, Xm, Xa` -- FEAT_CPA checked pointer multiply-add/subtract (`Xd = Xa +/- Xn*Xm`, the
    /// 3-source `op31=011` slot). `sub` selects `MSUBPT`.
    MaddSubCheckedPointer {
        sub: bool,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
        ra: Arm64GeneralPurposeRegister,
    },

    /// `ADDPT`/`SUBPT Zdn.D, Pg/M, Zdn.D, Zm.D` -- FEAT_CPA SVE predicated checked pointer add/subtract (`.d` only).
    /// base ADDPT `0x04C4_0000` / SUBPT `0x04C5_0000` (sub`[16]`): Pg`[12:10]`, Zm`[9:5]`, Zdn`[4:0]`.
    SveCheckedPointerAddSubPred {
        sub: bool,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `ADDPT`/`SUBPT Zd.D, Zn.D, Zm.D` -- FEAT_CPA SVE unpredicated checked pointer add/subtract (`.d` only). base
    /// `0x04E0_0800` / SUBPT adds `[10]`: Zm`[20:16]`, Zn`[9:5]`, Zd`[4:0]`.
    SveCheckedPointerAddSub {
        sub: bool,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `MADPT`/`MLAPT Zda.D, <op_a>.D, <op_b>.D` -- FEAT_CPA SVE checked pointer multiply-add (`.d` only). `madpt`
    /// (`mlapt:false`) base `0x44C0_D800` places op_a (2nd operand) at `[20:16]` and op_b (3rd) at `[9:5]`; `mlapt`
    /// (`mlapt:true`) base `0x44C0_D000` swaps them (op_b`[20:16]`, op_a`[9:5]`). bit`[11]` selects madpt(1)/mlapt(0). Zda`[4:0]`.
    SveCheckedPointerMulAdd {
        mlapt: bool,
        zda: Arm64ScalableVectorRegister,
        op_a: Arm64ScalableVectorRegister,
        op_b: Arm64ScalableVectorRegister,
    },

    // ---- conditional select, W/X forms ----
    //
    // Layout (DDI0487 C6.2 "Conditional select"): `sf op S 11010100 Rm[20:16] cond[15:12] o2 0[11:10] Rn[9:5]
    // Rd[4:0]`; the leading `width` supplies sf (S=0); op(bit30) + o2(bit10) select the operation. All three
    // registers use the ZR encoding at 31 (no SP). The condition is an [`Arm64Condition`]. The common aliases
    // (CSET/CSETM = CSINC/CSINV with Rn=Rm=ZR + inverted cond; CINC/CINV/CNEG = same with Rn=Rm) are
    // rendered by the emitter as cset/csetm/cinc/cinv/cneg; the raw csel/csinc/csinv/csneg is kept where
    // no alias applies. These forms have no immediate, so width is the only addition.
    /// `CSEL Wd|Xd, Wn|Xn, Wm|Xm, cond` -- `Rd = if cond then Rn else Rm`. The leading `width` selects the
    /// 32-bit (`W`) or 64-bit (`X`) form. Operands: `width`, `xd`, `xn`, `xm`, [`Arm64Condition`].
    CselRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* cond */ Arm64Condition,
    ),
    /// `CSINC Wd|Xd, Wn|Xn, Wm|Xm, cond` -- `Rd = if cond then Rn else Rm+1` (the `CSET`/`CINC` aliases build on
    /// this). Operands as [`Self::CselRegister`].
    CsincRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64Condition,
    ),
    /// `CSINV Wd|Xd, Wn|Xn, Wm|Xm, cond` -- `Rd = if cond then Rn else NOT(Rm)` (the `CSETM`/`CINV` aliases build
    /// on this). Operands as [`Self::CselRegister`].
    CsinvRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64Condition,
    ),
    /// `CSNEG Wd|Xd, Wn|Xn, Wm|Xm, cond` -- `Rd = if cond then Rn else -Rm` (the `CNEG` alias builds on this).
    /// Operands as [`Self::CselRegister`].
    CsnegRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        Arm64Condition,
    ),

    // ---- conditional compare (register + immediate), W/X ----
    //
    // Layout (DDI0487 C6.2 "Conditional compare (register|immediate)"): `sf op[30] S[29]=1 11010010
    // Rm|imm5[20:16] cond[15:12] reg/imm[11] o2[10]=0 Rn[9:5] o3[4]=0 nzcv[3:0]`. `width` supplies sf; op =
    // CCMN(0) / CCMP(1); bit 11 = 0 register / 1 immediate. If `cond` is FALSE the flags are set to `#nzcv`
    // (0..=15); else Rn is compared with Rm (register) or `#imm5` (0..=31). Rn/Rm use the ZR encoding at 31.
    /// `CCMP Wn|Xn, Wm|Xm, #nzcv, cond` -- conditional compare (register): if `cond` holds, `Rn - Rm` sets NZCV;
    /// else NZCV = `#nzcv`. Operands: `width`, `xn`, `xm`, `nzcv` (0..=15), [`Arm64Condition`].
    CcmpRegister(
        /* width */ Arm64RegisterWidth,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* nzcv */ u8,
        /* cond */ Arm64Condition,
    ),
    /// `CCMN Wn|Xn, Wm|Xm, #nzcv, cond` -- conditional compare negative (register): the `Rn + Rm` form. Operands as [`Self::CcmpRegister`].
    CcmnRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
        Arm64Condition,
    ),
    /// `CCMP Wn|Xn, #imm5, #nzcv, cond` -- conditional compare (immediate): `Rn - #imm5`. Operands: `width`,
    /// `xn`, `imm5` (0..=31), `nzcv` (0..=15), [`Arm64Condition`].
    CcmpImmediate(
        /* width */ Arm64RegisterWidth,
        /* xn */ Arm64GeneralPurposeRegister,
        /* imm5 */ u8,
        /* nzcv */ u8,
        /* cond */ Arm64Condition,
    ),
    /// `CCMN Wn|Xn, #imm5, #nzcv, cond` -- conditional compare negative (immediate): `Rn + #imm5`. Operands as [`Self::CcmpImmediate`].
    CcmnImmediate(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        u8,
        u8,
        Arm64Condition,
    ),

    // ---- bitfield move (SBFM / BFM / UBFM), W/X ----
    //
    // Layout (DDI0487 C6.2 "Bitfield"): `sf opc[30:29] 100110 N[22] immr[21:16] imms[15:10] Rn[9:5] Rd[4:0]`. opc
    // = SBFM(00) / BFM(01) / UBFM(10); `width` supplies sf, and **N MUST equal sf** (1 for X, 0 for W) -- a word
    // with N != sf is reserved. immr/imms are 6-bit: range 0..=31 for W, 0..=63 for X. The LSL/LSR/ASR-immediate,
    // UXTB/SXTH, BFI/BFXIL, SBFX/UBFX aliases all lower to these, and the emitter renders each as its preferred alias.
    // Rn/Rd use the ZR encoding at 31.
    /// `SBFM Wd|Xd, Wn|Xn, #immr, #imms` -- signed bitfield move. Operands: `width`, `xd`, `xn`, `immr`, `imms`.
    SbfmRegister(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* immr */ u8,
        /* imms */ u8,
    ),
    /// `BFM Wd|Xd, Wn|Xn, #immr, #imms` -- bitfield move (insert/copy). Operands as [`Self::SbfmRegister`].
    BfmRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
        u8,
    ),
    /// `UBFM Wd|Xd, Wn|Xn, #immr, #imms` -- unsigned bitfield move. Operands as [`Self::SbfmRegister`].
    UbfmRegister(
        Arm64RegisterWidth,
        Arm64GeneralPurposeRegister,
        Arm64GeneralPurposeRegister,
        u8,
        u8,
    ),

    /// `B label` -- unconditional branch. The operand is the **byte** offset from this instruction to the
    /// target, a signed multiple of 4 in `+/-128 MiB` (the encoded `imm26` is `offset/4`).
    B(/* offset_bytes */ i32),

    /// `BL label` -- branch with link (call); sets X30 to the return address. Operand as [`Self::B`].
    Bl(/* offset_bytes */ i32),

    /// `B.cond label` -- conditional branch. Operands: the [`Arm64Condition`] and the signed byte offset, a
    /// multiple of 4 in `+/-1 MiB` (the encoded `imm19` is `offset/4`).
    BCond(/* cond */ Arm64Condition, /* offset_bytes */ i32),

    /// `BC.<cond> label` -- FEAT_HBC hinted conditional branch: same as `B.cond` but with the `[4]=1` consistent-branch
    /// hint. `offset_bytes` is the PC-relative byte offset (multiple of 4, +/-1 MiB). base `0x5400_0010`.
    BcCond(/* cond */ Arm64Condition, /* offset_bytes */ i32),

    /// `CBZ Wt|Xt, label` -- compare and branch if `Rt` is zero (no flags read/written). The leading `width`
    /// selects the 32-bit (`W`) or 64-bit (`X`) register; the operand is the signed **byte** offset, a multiple
    /// of 4 in `+/-1 MiB` (the encoded `imm19` is `offset/4`). `Rt` uses ZR at 31.
    Cbz(
        /* width */ Arm64RegisterWidth,
        /* xt */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ i32,
    ),

    /// `CBNZ Wt|Xt, label` -- compare and branch if `Rt` is non-zero. Operands as [`Self::Cbz`].
    Cbnz(
        /* width */ Arm64RegisterWidth,
        /* xt */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ i32,
    ),

    /// `CB<cc> <Rn>, <Rm>, label` / `CBB<cc> Wn, Wm, label` / `CBH<cc> Wn, Wm, label` -- FEAT_CMPBR compare two
    /// registers and branch on the condition. `size` selects the comparison width: `Word`/`Double` are the `CB`
    /// register form (`Wn`/`Xn`, via `sf`), `Byte`/`Half` the `CBB`/`CBH` 8/16-bit forms (always `W`). `cond` is the
    /// architectural condition (`GT`/`GE`/`HI`/`HS`/`EQ`/`NE`; `LT`/`LE`/`LO`/`LS` are swapped-operand aliases).
    /// `offset_bytes` is the signed PC-relative byte offset, a multiple of 4 in `+/-1 KiB` (the 9-bit `imm9`). base
    /// `0x7400_0000`: sf`[31]`, cc`[23:21]`, Rm`[20:16]`, size`[15:14]` (00 reg / 10 byte / 11 half), imm9`[13:5]`, Rn`[4:0]`.
    CompareBranchRegister {
        cond: Arm64CmpBranchCond,
        size: Arm64LoadStoreSize,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `CB<cc> <Rn>, #<imm6>, label` -- FEAT_CMPBR compare a register against a 6-bit unsigned immediate (`0..=63`)
    /// and branch on the condition. `width` selects `Wn`/`Xn`. `cond` is the architectural condition (`GT`/`LT`/`HI`/
    /// `LO`/`EQ`/`NE`; `GE`/`LE`/`HS`/`LS` are immediate-adjust aliases). `offset_bytes` as [`Self::CompareBranchRegister`].
    /// base `0x7500_0000`: sf`[31]`, cc`[23:21]`, imm6`[5:1]` at `[20:16]` + imm6`[0]` at `[15]`, imm9`[13:5]`, Rn`[4:0]`.
    CompareBranchImmediate {
        cond: Arm64CmpBranchImmCond,
        width: Arm64RegisterWidth,
        rn: Arm64GeneralPurposeRegister,
        imm6: u8,
        offset_bytes: i32,
    },

    /// `TBZ Rt, #<bit>, label` -- test bit `<bit>` of `Rt` and branch if it is zero. The bit position `0..=63` is
    /// split across the encoding (`b5` at bit 31, `b40` at `[23:19]`); `Rt` renders as `W` when `bit < 32`, else
    /// `X`. `offset_bytes` is the signed PC-relative byte offset, a multiple of 4 in `+/-32 KiB` (`imm14`).
    /// Operands: `xt`, `bit`, `offset_bytes`.
    TestBitZero(
        /* xt */ Arm64GeneralPurposeRegister,
        /* bit */ u8,
        /* offset_bytes */ i32,
    ),

    /// `TBNZ Rt, #<bit>, label` -- test bit `<bit>` of `Rt` and branch if it is non-zero. Operands as
    /// [`Self::TestBitZero`].
    TestBitNonzero(
        /* xt */ Arm64GeneralPurposeRegister,
        /* bit */ u8,
        /* offset_bytes */ i32,
    ),

    /// `BR Xn` -- unconditional branch to the address in `Xn`. Operand: `xn`.
    Br(/* xn */ Arm64GeneralPurposeRegister),

    /// `BLR Xn` -- branch with link to the address in `Xn` (indirect call). Operand: `xn`.
    Blr(/* xn */ Arm64GeneralPurposeRegister),

    // ---- load/store register, unsigned-immediate OFFSET, all four sizes ----
    //
    // Layout (DDI0487 C6.2 "Load/store register (unsigned immediate)"): `size[31:30] 111 V[26]=0 01[25:24]
    // opc[23:22] imm12[21:10] Rn[9:5] Rt[4:0]`. The leading `size` (an [`Arm64LoadStoreSize`]) supplies the
    // 2-bit size field; `opc` bit 22 (L) selects load vs store (these plain integer forms keep opc bit 23 = 0).
    // `Rt` is a `Wt` for the 8/16/32-bit accesses and an `Xt` for the 64-bit access (the size sets the view, not
    // an `sf` bit), using the ZR encoding at 31; `Rn` is the base, using the SP encoding at 31. The byte offset
    // is SCALED by the access size -- `byte_offset = imm12 << size`, so `imm12 = offset_bytes >> size` with
    // `offset_bytes` a non-negative multiple of `1 << size` and `imm12 in 0..=4095`. This GENERALIZES the former
    // 64-bit-only `LdrImmediate`/`StrImmediate` (the size = Double case).
    //
    // The register pre-index `[Xn, #imm]!` and post-index `[Xn], #imm` single-register forms, the register-offset
    // form `[Xn, Xm{, LSL #amount}]`, the signed loads (LDRSB/LDRSH/LDRSW), the unscaled LDUR/STUR, the
    // exclusive/acquire-release forms, and the SIMD/FP load/store forms are modeled by their own variants below.
    /// `LDRB Wt, [Xn{, #imm}]` / `LDRH Wt, [Xn{, #imm}]` / `LDR Wt|Xt, [Xn{, #imm}]` -- load register,
    /// unsigned-immediate offset form, in the access width given by `size`. The model field `offset_bytes` is the
    /// **byte** offset (a non-negative multiple of `1 << size`, up to `4095 << size`); the encoded `imm12` is
    /// `offset_bytes >> size`. `Rt` is a `W` register for [`Arm64LoadStoreSize::Byte`]/`Half`/`Word` and an `X`
    /// register for `Double`; `Xn` uses the SP encoding at 31. Operands: `size`, `xt`, `xn`, `offset_bytes`.
    LoadRegister(
        /* size */ Arm64LoadStoreSize,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ u32,
    ),

    /// `STRB Wt, [Xn{, #imm}]` / `STRH Wt, [Xn{, #imm}]` / `STR Wt|Xt, [Xn{, #imm}]` -- store register,
    /// unsigned-immediate offset form. Operands / `Rt`-width / scaling as [`Self::LoadRegister`].
    StoreRegister(
        /* size */ Arm64LoadStoreSize,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ u32,
    ),

    /// `LDRSB Wt|Xt, [Xn{, #imm}]` -- load a byte and **sign-extend** it into the 32-bit (`W`) or 64-bit (`X`)
    /// register selected by `dest_width`. `offset_bytes` is unscaled (a multiple of 1, up to 4095). `Xn` uses
    /// SP at 31. Operands: `dest_width`, `xt`, `xn`, `offset_bytes`.
    LoadSignedByte(
        /* dest_width */ Arm64RegisterWidth,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ u32,
    ),

    /// `LDRSH Wt|Xt, [Xn{, #imm}]` -- load a halfword and sign-extend it into the `W` or `X` register selected
    /// by `dest_width`. `offset_bytes` is a multiple of 2 (up to `4095 << 1`). Operands as [`Self::LoadSignedByte`].
    LoadSignedHalf(
        /* dest_width */ Arm64RegisterWidth,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ u32,
    ),

    /// `LDRSW Xt, [Xn{, #imm}]` -- load a 32-bit word and sign-extend it into the 64-bit register (there is no
    /// 32-bit-dest form, so no `dest_width`). `offset_bytes` is a multiple of 4 (up to `4095 << 2`). Operands:
    /// `xt`, `xn`, `offset_bytes`.
    LoadSignedWord(
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ u32,
    ),

    // ---- load/store PAIR (LDP/STP), W and X, all three index modes ----
    //
    // Layout (DDI0487 C6.2 "Load/store register pair"): `opc[31:30] 101 V[26]=0 0[25] idx[24:23] L[22]
    // imm7[21:15] Rt2[14:10] Rn[9:5] Rt[4:0]`. The leading `width` ([`Arm64RegisterWidth`]) supplies opc (00 =>
    // W via opc=00, X via opc=10, i.e. opc = sf << 1); `L` bit 22 selects load (1) vs store (0); the
    // [`Arm64LoadStoreIndex`] supplies idx[24:23] (offset 10, pre-index 11, post-index 01). `Rt`/`Rt2` are the
    // two transfer registers (`W` or `X` per `width`, ZR encoding at 31); `Rn` is the base (SP encoding at 31).
    // The offset is SIGNED and scaled by the access size -- scale 4 for W, 8 for X -- so `imm7 in -64..=63` and
    // `offset_bytes` is a multiple of the scale in `[-64*scale, 63*scale]`. (The `idx = 00` no-allocate
    // LDNP/STNP slot is modeled separately as `LoadStorePairNonTemporal`.)
    /// `LDP Wt, Wt2, [Xn ...]` / `LDP Xt, Xt2, [Xn ...]` -- load pair of registers, in the given `width` and index
    /// mode. The three [`Arm64LoadStoreIndex`] modes render `[Xn, #imm]`, `[Xn, #imm]!` (pre-index), and
    /// `[Xn], #imm` (post-index). `offset_bytes` is the **signed** byte offset, a multiple of the access size
    /// (4 for `W`, 8 for `X`) in `[-64*scale, 63*scale]`; the encoded `imm7` is `offset_bytes / scale`.
    /// `Rt`/`Rt2` use the ZR encoding at 31, `Xn` the SP encoding at 31. Operands: `width`, `index`, `xt`,
    /// `xt2`, `xn`, `offset_bytes`.
    LoadPair(
        /* width */ Arm64RegisterWidth,
        /* index */ Arm64LoadStoreIndex,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xt2 */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ i32,
    ),

    /// `STP Wt, Wt2, [Xn ...]` / `STP Xt, Xt2, [Xn ...]` -- store pair of registers. The prologue/epilogue
    /// workhorse (`stp x29, x30, [sp, #-16]!`). Operands / index modes / scaling as [`Self::LoadPair`].
    StorePair(
        /* width */ Arm64RegisterWidth,
        /* index */ Arm64LoadStoreIndex,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xt2 */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ i32,
    ),

    /// `LDPSW Xt, Xt2, [Xn ...]` -- load pair of signed words (each 32-bit word sign-extended into a 64-bit `Xt`).
    /// Same three index modes as [`Self::LoadPair`]; the offset is signed, scaled by 4 (the word access size).
    LoadPairSignedWord(
        /* index */ Arm64LoadStoreIndex,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xt2 */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ i32,
    ),

    /// `STTP`/`LDTP Xt, Xt2, [Xn|SP ...]` -- FEAT_LSUI unprivileged load/store PAIR (the `opc=11` `T` view of
    /// `STP`/`LDP`, **64-bit only**, scale 8). `load` selects LDTP vs STTP; `index` the offset / pre / post form.
    /// `offset_bytes` is signed, a multiple of 8, in `[-512, 504]`. bases STTP off `0xE900_0000` / pre `0xE980_0000`
    /// / post `0xE880_0000`; `L[22]` selects load.
    LsuiPair {
        load: bool,
        index: Arm64LoadStoreIndex,
        rt: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `LDTNP`/`STTNP Xt, Xt2, [Xn|SP{, #imm}]` -- FEAT_LSUI unprivileged load/store pair, NON-TEMPORAL (the
    /// `opc=11`, `idx=00` `T` view of `LDNP`/`STNP`, **64-bit only**, scale 8, no writeback). `offset_bytes` is signed,
    /// a multiple of 8, in `[-512, 504]`. bases STTNP `0xE800_0000` / LDTNP `0xE840_0000` (`L[22]`).
    LsuiPairNonTemporal {
        load: bool,
        rt: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `LDTP`/`STTP`/`LDTNP`/`STTNP Qt, Qt2, [Xn|SP ...]` -- FEAT_LSUI unprivileged SIMD&FP load/store pair (the
    /// `opc=11`, `V=1` `T` view of the SIMD pair, **128-bit `Q` only**, scale 16). `index` selects non-temporal /
    /// post / offset / pre; `offset_bytes` is signed, a multiple of 16, in `[-1024, 1008]`. bases STTNP `0xEC00_0000`
    /// / STTP post `0xEC80_0000` / off `0xED00_0000` / pre `0xED80_0000`; `L[22]` selects load. Needs the FP unit.
    LsuiVecPair {
        load: bool,
        index: Arm64LsuiPairIndex,
        vt: Arm64FloatRegister,
        vt2: Arm64FloatRegister,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `LDNP`/`STNP Wt|Xt, Wt2|Xt2, [Xn{, #imm}]` -- load/store pair, NON-TEMPORAL (a no-allocate cache hint). GP
    /// form (W/X). The offset is signed, scaled by the access size (4 for W, 8 for X), offset-only (no pre/post).
    LoadStorePairNonTemporal {
        load: bool,
        width: Arm64RegisterWidth,
        rt: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        xn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `LDNP`/`STNP St|Dt|Qt, ..., [Xn{, #imm}]` -- the SIMD&FP non-temporal pair. `size` is `S`/`D`/`Q` (scale
    /// 4/8/16). Needs the FP unit.
    VecLoadStorePairNonTemporal {
        load: bool,
        size: Arm64VectorLoadStoreSize,
        vt: Arm64FloatRegister,
        vt2: Arm64FloatRegister,
        xn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    // ---- load/store register-offset `[Xn, Rm{, <ext> #amount}]` (DDI0487 C6.2 "register offset") ----
    //
    // Layout: `size[31:30] 111 V[26]=0 00 opc[23:22] 1[21] Rm[20:16] option[15:13] S[12] 10[11:10] Rn[9:5]
    // Rt[4:0]`. The index `Rm` is extended per [`Arm64MemoryExtend`] (`option`) and, when `S = 1`, shifted left
    // by `log2(access_size)`. `Rt` uses the ZR encoding at 31, `Rn` the SP encoding at 31, `Rm` the ZR encoding.
    // This is the array-indexing form (`ldr x0, [base, index, lsl #3]`).
    /// `LDR Wt|Xt, [Xn, Rm{, <ext> #amount}]` -- load register, register-offset addressing. `size` picks the
    /// access width (and `Rt`'s view), `extend` the index operation ([`Arm64MemoryExtend`]; `Lsl` for the plain
    /// `[Xn, Xm]` form), and `scaled` the `S` bit (`true` => shift the index left by `log2(size)`). Operands:
    /// `size`, `xt`, `xn`, `xm`, `extend`, `scaled`.
    LoadRegisterOffset(
        /* size */ Arm64LoadStoreSize,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* extend */ Arm64MemoryExtend,
        /* scaled */ bool,
    ),

    /// `STR Wt|Xt, [Xn, Rm{, <ext> #amount}]` -- store register, register-offset addressing. Operands as
    /// [`Self::LoadRegisterOffset`].
    StoreRegisterOffset(
        /* size */ Arm64LoadStoreSize,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* extend */ Arm64MemoryExtend,
        /* scaled */ bool,
    ),

    /// `LDRSB Wt|Xt, [Xn, Rm{, <ext> #amount}]` -- load byte and sign-extend into the `W` or `X` register chosen
    /// by `dest_width`, register-offset addressing. Operands: `dest_width`, `xt`, `xn`, `xm`, `extend`, `scaled`.
    LoadSignedByteOffset(
        /* dest_width */ Arm64RegisterWidth,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* extend */ Arm64MemoryExtend,
        /* scaled */ bool,
    ),

    /// `LDRSH Wt|Xt, [Xn, Rm{, <ext> #amount}]` -- load halfword and sign-extend into `W`/`X`, register-offset
    /// addressing. Operands as [`Self::LoadSignedByteOffset`].
    LoadSignedHalfOffset(
        /* dest_width */ Arm64RegisterWidth,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* extend */ Arm64MemoryExtend,
        /* scaled */ bool,
    ),

    /// `LDRSW Xt, [Xn, Rm{, <ext> #amount}]` -- load word and sign-extend into the 64-bit register,
    /// register-offset addressing (no 32-bit-dest form). Operands: `xt`, `xn`, `xm`, `extend`, `scaled`.
    LoadSignedWordOffset(
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* extend */ Arm64MemoryExtend,
        /* scaled */ bool,
    ),

    /// `PRFM <prfop>, [Xn{, #imm}]` -- prefetch memory, unsigned scaled offset. `prfop` is the 5-bit prefetch
    /// operation (`<type><target><policy>`, e.g. `pldl1keep` = 0); `offset_bytes` is a multiple of 8 in
    /// `0..=32760`.
    PrefetchImmediate {
        prfop: u8,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: u32,
    },

    /// `PRFUM <prfop>, [Xn{, #imm}]` -- prefetch memory, unscaled 9-bit signed offset (`-256..=255`).
    PrefetchUnscaled {
        prfop: u8,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `PRFM <prfop>, [Xn, Rm{, <ext> #amount}]` -- prefetch memory, register offset. `extend`/`scaled` are the
    /// index extension and shift, exactly as for [`Self::LoadRegisterOffset`].
    PrefetchRegister {
        prfop: u8,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
        extend: Arm64MemoryExtend,
        scaled: bool,
    },

    /// `PRFM <prfop>, <label>` -- prefetch memory, PC-relative literal. `offset_bytes` is a multiple of 4 in
    /// `-1048576..=1048572`.
    PrefetchLiteral { prfop: u8, offset_bytes: i32 },

    // ---- load/store single register, 9-bit unscaled immediate (LDUR/STUR + single-reg pre/post-index) ----
    //
    // Layout: `size[31:30] 111 V[26]=0 00 opc[23:22] 0[21] imm9[20:12] idx[11:10] Rn[9:5] Rt[4:0]`. The signed
    // `imm9` is a raw byte offset `-256..=255` (NOT scaled). [`Arm64Imm9Mode`] supplies `idx` (00 unscaled
    // `LDUR`/`STUR`, 01 post-index, 11 pre-index). `Rt` uses the ZR encoding at 31, `Rn` the SP encoding at 31.
    /// `LDUR Wt|Xt, [Xn, #imm]` / `LDR Wt|Xt, [Xn], #imm` / `LDR Wt|Xt, [Xn, #imm]!` -- load register with the
    /// 9-bit unscaled signed offset, in the given [`Arm64Imm9Mode`] (unscaled / post- / pre-index). `size` picks
    /// the access width; `offset` is the raw signed byte offset `-256..=255`. Operands: `size`, `mode`, `xt`,
    /// `xn`, `offset`.
    LoadRegisterImm9(
        /* size */ Arm64LoadStoreSize,
        /* mode */ Arm64Imm9Mode,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    /// `STUR Wt|Xt, [Xn, #imm]` / `STR Wt|Xt, [Xn], #imm` / `STR Wt|Xt, [Xn, #imm]!` -- store register, 9-bit
    /// unscaled signed offset. Operands as [`Self::LoadRegisterImm9`].
    StoreRegisterImm9(
        /* size */ Arm64LoadStoreSize,
        /* mode */ Arm64Imm9Mode,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    /// `LDURSB`/`LDRSB` (post/pre) `Wt|Xt, [Xn ...]` -- load byte and sign-extend into the `W`/`X` register chosen
    /// by `dest_width`, 9-bit unscaled signed offset in the given mode. Operands: `dest_width`, `mode`, `xt`,
    /// `xn`, `offset`.
    LoadSignedByteImm9(
        /* dest_width */ Arm64RegisterWidth,
        /* mode */ Arm64Imm9Mode,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    /// `LDURSH`/`LDRSH` (post/pre) `Wt|Xt, [Xn ...]` -- load halfword and sign-extend into `W`/`X`, 9-bit unscaled
    /// signed offset. Operands as [`Self::LoadSignedByteImm9`].
    LoadSignedHalfImm9(
        /* dest_width */ Arm64RegisterWidth,
        /* mode */ Arm64Imm9Mode,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    /// `LDURSW`/`LDRSW` (post/pre) `Xt, [Xn ...]` -- load word and sign-extend into the 64-bit register, 9-bit
    /// unscaled signed offset (no 32-bit-dest form). Operands: `mode`, `xt`, `xn`, `offset`.
    LoadSignedWordImm9(
        /* mode */ Arm64Imm9Mode,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    // ---- load register (PC-relative literal): `opc[31:30] 011 V=0 00 imm19[23:5] Rt[4:0]` ----
    /// `LDR Wt|Xt, <label>` -- load a register from a PC-relative literal. `offset` is the **signed byte** offset
    /// from the instruction to the literal, a multiple of 4 in `+/-1 MiB` (encoded `imm19 = offset / 4`); a
    /// code-generator resolves a label to this offset (and emits a relocation). Operands: `width`, `xt`, `offset`.
    LoadLiteral(
        /* width */ Arm64RegisterWidth,
        /* xt */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    /// `LDRSW Xt, <label>` -- load a 32-bit word from a PC-relative literal and sign-extend into the 64-bit
    /// register. `offset` as [`Self::LoadLiteral`]. Operands: `xt`, `offset`.
    LoadSignedWordLiteral(
        /* xt */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    // ---- atomics: load/store-exclusive, acquire/release, and the ARMv8.1 LSE single-op atomics ----
    //
    // The exclusive + acquire/release forms are the ARMv8.0 "load/store exclusive" group (`size 001000 ...`);
    // `Rt`/`Rs` use the access-size view (`W` for B/H/W, `X` for D), `Rn` the SP encoding. The LSE forms
    // (`LDADD`/`SWP`/`CAS`/...) are the ARMv8.1 "atomic memory operations" group and require [`Self::requirement`]
    // == `lse()`. All take a bare `[Xn]` base (no offset).
    /// `LDXR`/`LDAXR Wt|Xt, [Xn]` -- load-exclusive (acquire variant when `acquire`). `size` selects the access
    /// width and `Rt`'s view. Operands: `size`, `acquire`, `xt`, `xn`.
    LoadExclusive(
        /* size */ Arm64LoadStoreSize,
        /* acquire */ bool,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `STXR`/`STLXR Ws, Wt|Xt, [Xn]` -- store-exclusive (release variant when `release`); the 32-bit status
    /// register `Ws` receives 0 on success / 1 on failure. Operands: `size`, `release`, `ws`, `xt`, `xn`.
    StoreExclusive(
        /* size */ Arm64LoadStoreSize,
        /* release */ bool,
        /* ws */ Arm64GeneralPurposeRegister,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `LDTXR`/`LDATXR Wt|Xt, [Xn|SP]` -- unprivileged load-exclusive (FEAT_LSUI; acquire variant when `acquire`). The
    /// `[24]=1` (`T`) variant of [`Self::LoadExclusive`]; `.s`(W) / `.d`(X) sizes only.
    LoadExclusiveUnpriv(
        /* size */ Arm64LoadStoreSize,
        /* acquire */ bool,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `STTXR`/`STLTXR Ws, Wt|Xt, [Xn|SP]` -- unprivileged store-exclusive (FEAT_LSUI; release variant when `release`).
    /// The `[24]=1` (`T`) variant of [`Self::StoreExclusive`]; `.s`(W) / `.d`(X) sizes only.
    StoreExclusiveUnpriv(
        /* size */ Arm64LoadStoreSize,
        /* release */ bool,
        /* ws */ Arm64GeneralPurposeRegister,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `CAST`/`CASAT`/`CASLT`/`CASALT Xs, Xt, [Xn|SP]` -- FEAT_LSUI unprivileged compare-and-swap (the `T` sibling
    /// of `CAS`, **64-bit only**). `ordering` selects the acquire (`L[22]`) / release (`o0[15]`) suffix. base
    /// `0xC980_7C00` (the `[24]=1`, `o2=1`, `o1=0`, `[31:30]=11` view; `Rt2[14:10]` fixed `11111`).
    CompareAndSwapUnpriv {
        ordering: Arm64AtomicOrdering,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `CASPT`/`CASPAT`/`CASPLT`/`CASPALT Xs, Xs+1, Xt, Xt+1, [Xn|SP]` -- FEAT_LSUI unprivileged compare-and-swap
    /// PAIR (the `T` sibling of `CASP`, **64-bit element only**). `rs`/`rt` are the even first registers of the
    /// operand pairs. base `0x4980_7C00` (the pair view, `[31:30]=01`).
    CompareAndSwapPairUnpriv {
        ordering: Arm64AtomicOrdering,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `LDRAA`/`LDRAB Xt, [Xn{, #imm}]{!}` -- load (64-bit) from an address authenticated with key A (`key_b =
    /// false`) or key B (FEAT_PAuth). `pre_index` selects the writeback form. The offset is signed, scaled by 8,
    /// in `[-4096, 4088]` (a 10-bit `simm10`).
    LoadPac {
        key_b: bool,
        pre_index: bool,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `PACGA Xd, Xn, Xm` -- compute a generic pointer-auth code (FEAT_PAuth). Operands are general-purpose.
    Pacga {
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `BRAA`/`BRAB`/`BLRAA`/`BLRAB Xn, Xm` (or the `*Z Xn` zero-modifier form) -- pointer-authenticated indirect
    /// branch (FEAT_PAuth). `modifier = None` is the `*Z` form (zero modifier).
    PacBranch {
        op: Arm64PacBranchOp,
        rn: Arm64GeneralPurposeRegister,
        modifier: Option<Arm64GeneralPurposeRegister>,
    },

    /// `RETAA`/`RETAB`/`ERETAA`/`ERETAB` -- pointer-authenticated return (FEAT_PAuth). Operand-free.
    PacReturn(Arm64PacReturnOp),

    /// `PACIASP`/`AUTIASP`/`XPACLRI`/... -- an operand-free pointer-auth hint (FEAT_PAuth) acting on LR. A fixed
    /// word in the hint (NOP) space.
    PointerAuthHint(Arm64PacHintOp),

    /// `BTI {c|j|jc}` -- branch target identification (FEAT_BTI), a hint marking valid indirect-branch landing pads.
    Bti(Arm64BtiTarget),

    /// `<op> Xd, Xn` / `<op> Xd` -- pointer authentication (FEAT_PAuth). The `PAC*`/`AUT*` ops sign/authenticate
    /// `Xd` with the modifier `Xn`; the `*Z*` and `XPAC*` ops use a fixed (zero/none) modifier and take only `Xd`.
    PointerAuth {
        op: Arm64PacOp,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `<op> Xt, [Xn{, #imm}]{!}` -- store allocation tag (`STG`/`STZG`/`ST2G`/`STZ2G`, FEAT_MTE). The offset is
    /// signed, scaled by 16 (the tag granule); `index` selects offset / pre / post.
    StoreTag {
        op: Arm64StoreTagOp,
        index: Arm64LoadStoreIndex,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `LDG Xt, [Xn{, #imm}]` -- load the allocation tag of an address into `Xt` (FEAT_MTE). Offset-only, signed,
    /// scaled by 16.
    LoadTag {
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `STGP Xt, Xt2, [Xn{, #imm}]{!}` -- store an allocation tag and a pair of registers (FEAT_MTE). The offset is
    /// signed, scaled by 16; `index` selects offset / pre / post.
    StoreTagPair {
        index: Arm64LoadStoreIndex,
        rt: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `ADDG`/`SUBG Xd, Xn, #imm, #tag` -- add/subtract a (16-byte-scaled) immediate to a tagged pointer and offset
    /// its tag (FEAT_MTE). `offset` is `0..=1008` (a multiple of 16); `tag` is `0..=15`.
    MteAddSubImmTag {
        sub: bool,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        offset: u32,
        tag: u8,
    },

    /// `<op> Xd, Xn, Xm` -- memory-tagging data-processing op (`IRG`/`GMI`/`SUBP`/`SUBPS`, FEAT_MTE).
    MteDataProc {
        op: Arm64MteDataOp,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `<op> Wd, Wn, Wm|Xm` -- CRC32 checksum (FEAT_CRC32). `Wd`/`Wn` are the 32-bit accumulator; the data input is
    /// `Wm` (or `Xm` for the `*x` ops). Operands carried as general-purpose registers.
    Crc32 {
        op: Arm64Crc32Op,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LDXP`/`LDAXP Wt1|Xt1, Wt2|Xt2, [Xn]` -- load-exclusive PAIR (acquire variant when `acquire`). `width` is
    /// the W/X element width. Operands: two destination registers + the base.
    LoadExclusivePair {
        width: Arm64RegisterWidth,
        acquire: bool,
        rt: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        xn: Arm64GeneralPurposeRegister,
    },

    /// `STXP`/`STLXP Ws, Wt1|Xt1, Wt2|Xt2, [Xn]` -- store-exclusive PAIR (release variant when `release`); the
    /// 32-bit status register `Ws` (`rs`) receives 0 on success / 1 on failure. `width` is the W/X element width.
    StoreExclusivePair {
        width: Arm64RegisterWidth,
        release: bool,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        xn: Arm64GeneralPurposeRegister,
    },

    /// `LDAR Wt|Xt, [Xn]` -- load-acquire (ordinary, non-exclusive). Operands: `size`, `xt`, `xn`.
    LoadAcquire(
        /* size */ Arm64LoadStoreSize,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `STLR Wt|Xt, [Xn]` -- store-release (ordinary, non-exclusive). Operands: `size`, `xt`, `xn`.
    StoreRelease(
        /* size */ Arm64LoadStoreSize,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `LDLAR Wt|Xt, [Xn]` -- load-acquire in a Limited Ordering Region (FEAT_LOR; the `o0 = 0` variant of LDAR).
    /// Operands: `size`, `xt`, `xn`.
    LoadLOAcquire(
        /* size */ Arm64LoadStoreSize,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `STLLR Wt|Xt, [Xn]` -- store-release in a Limited Ordering Region (FEAT_LOR; the `o0 = 0` variant of STLR).
    /// Operands: `size`, `xt`, `xn`.
    StoreLORelease(
        /* size */ Arm64LoadStoreSize,
        /* xt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `CAS{A}{L} Ws, Wt, [Xn]` -- compare-and-swap (LSE): if `[Xn] == Rs`, store `Rt`; the prior value of
    /// `[Xn]` is returned in `Rs`. `ordering` selects the acquire/release suffix. Operands: `size`, `ordering`,
    /// `rs`, `rt`, `xn`. Requires LSE.
    CompareAndSwap(
        /* size */ Arm64LoadStoreSize,
        /* ordering */ Arm64AtomicOrdering,
        /* rs */ Arm64GeneralPurposeRegister,
        /* rt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `LDAPR{B|H} Wt|Xt, [Xn]` -- load-acquire RCpc register (FEAT_LRCPC); like `LDAR` but with the weaker
    /// release-consistent (processor-consistent) ordering. `size` picks the access width; `Rt` follows it.
    LoadAcquireRcpc(
        /* size */ Arm64LoadStoreSize,
        /* rt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `STLUR`/`LDAPUR`/`LDAPURS{B,H,W} <Rt>, [Xn|SP{, #simm9}]` -- RCpc load/store with a 9-bit unscaled signed
    /// offset (FEAT_LRCPC2). [`Arm64RcpcUnscaledOp`] selects the size/direction/dest-width; `offset` is `-256..=255`.
    RcpcUnscaled {
        op: Arm64RcpcUnscaledOp,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        offset: i32,
    },

    /// `CASP{A}{L} <Ws>, <W(s+1)>, <Wt>, <W(t+1)>, [Xn]` -- compare-and-swap PAIR (LSE). Operates on the register
    /// pairs `rs`/`rs+1` and `rt`/`rt+1`, so `rs` and `rt` must be even. `width` selects the W (32-bit) or X
    /// (64-bit) element. Operands: `width`, `ordering`, `rs`, `rt`, `xn`. Requires LSE.
    CompareAndSwapPair {
        width: Arm64RegisterWidth,
        ordering: Arm64AtomicOrdering,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        xn: Arm64GeneralPurposeRegister,
    },

    /// `LD<op>{A}{L} Ws, Wt, [Xn]` -- LSE atomic read-modify-write: apply `op` to `[Xn]` with source `Rs`,
    /// returning the prior value of `[Xn]` in `Rt`. Operands: `op`, `size`, `ordering`, `rs`, `rt`, `xn`.
    /// Requires LSE.
    AtomicRmw(
        /* op */ Arm64AtomicOp,
        /* size */ Arm64LoadStoreSize,
        /* ordering */ Arm64AtomicOrdering,
        /* rs */ Arm64GeneralPurposeRegister,
        /* rt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `SWPP`/`LDCLRP`/`LDSETP{A}{L} Xt1, Xt2, [Xn|SP]` -- FEAT_LSE128 128-bit atomic on a 64-bit register pair.
    /// `acquire`/`release` select the ordering (`A`/`L`/`AL`). base `0x1920_0000`: A`[23]`, L`[22]`, Xt2`[20:16]`, op`[15:12]`,
    /// Xn`[9:5]`, Xt1`[4:0]`. See [`Arm64Lse128Op`].
    Lse128Atomic {
        op: Arm64Lse128Op,
        acquire: bool,
        release: bool,
        rt: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `LDIAPP`/`STILP Wt1|Xt1, Wt2|Xt2, [Xn|SP]{, ...}` -- FEAT_LRCPC3 release-consistent ordered load/store pair.
    /// `load` selects `LDIAPP` (load-acquire-RCpc) vs `STILP` (store-release-RCpc). `writeback` selects the writeback
    /// form (`LDIAPP [Xn], #N` post-index / `STILP [Xn, #-N]!` pre-index, `N`=8 for W / 16 for X) vs the plain `[Xn]`.
    /// base `0x9900_0800`: size`[30]`, load`[22]`, !writeback`[12]`, Xt2`[20:16]`, Xn`[9:5]`, Xt1`[4:0]`.
    Rcpc3OrderedPair {
        load: bool,
        width: Arm64RegisterWidth,
        writeback: bool,
        rt: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `LDAPUR`/`STLUR Bt|Ht|St|Dt|Qt, [Xn|SP{, #imm9}]` -- FEAT_LRCPC3 RCpc-ordered SIMD&FP unscaled load-acquire /
    /// store-release. `load` selects `LDAPUR` vs `STLUR`; `imm9` is the signed `-256..=255` unscaled byte offset. base
    /// `0x1D00_0800`: size`[31:30]`, opc1(Q)`[23]`, L`[22]`, imm9`[20:12]`, Xn`[9:5]`, Vt`[4:0]`.
    Rcpc3SimdLoadStore {
        load: bool,
        size: Arm64VectorLoadStoreSize,
        ft: Arm64FloatRegister,
        rn: Arm64GeneralPurposeRegister,
        imm9: i32,
    },

    /// `GCSSTR`/`GCSSTTR Xt, [Xn|SP]` -- FEAT_GCS store a 64-bit value to the guarded control stack at `[Xn]`.
    /// `unprivileged` selects `GCSSTTR` (the unprivileged `[12]=1` form) vs `GCSSTR`. base `0x_D91F_0C00`: unpriv`[12]`,
    /// Xn`[9:5]`, Xt`[4:0]`.
    GcsStore {
        unprivileged: bool,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `LD64B`/`ST64B Xt, [Xn|SP]` -- FEAT_LS64 atomic 64-byte load/store. `rt` is the first of the eight
    /// consecutive 64-bit registers (`Xt`..`Xt+7`) holding the 64-byte value. `store` selects `ST64B`.
    Ls64 {
        store: bool,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `ST64BV`/`ST64BV0 Xs, Xt, [Xn|SP]` -- FEAT_LS64 atomic 64-byte store with status: `rs` receives the
    /// completion status; `rt` is the first of the eight source registers. `zero_data` selects `ST64BV0`.
    Ls64StoreStatus {
        zero_data: bool,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `STGM`/`LDGM`/`STZGM Xt, [Xn|SP]` -- FEAT_MTE2 block allocation-tag store/load/store-zero. See
    /// [`Arm64MteBlockOp`].
    MteBlockTag {
        op: Arm64MteBlockOp,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `RCWCAS{A}{L}`/`RCWSCAS{A}{L} Xs, Xt, [Xn|SP]` -- FEAT_THE read-check-write compare-and-swap. `secure`
    /// selects the `RCWS` variant; `ordering` the A/L acquire/release bits.
    RcwCompareAndSwap {
        secure: bool,
        ordering: Arm64AtomicOrdering,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `RCWCASP{A}{L}`/`RCWSCASP{A}{L} Xs, Xs+1, Xt, Xt+1, [Xn|SP]` -- FEAT_THE+FEAT_D128 128-bit read-check-write
    /// compare-and-swap. `rs`/`rt` are the even first registers of the operand pairs.
    RcwCompareAndSwapPair {
        secure: bool,
        ordering: Arm64AtomicOrdering,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `RCW<op>{A}{L}`/`RCWS<op>{A}{L} Xs, Xt, [Xn|SP]` -- FEAT_THE read-check-write atomic (`RCWCLR`/`RCWSET`/
    /// `RCWSWP`). See [`Arm64RcwAtomicOp`].
    RcwAtomic {
        op: Arm64RcwAtomicOp,
        secure: bool,
        ordering: Arm64AtomicOrdering,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `RCW<op>P{A}{L}`/`RCWS<op>P{A}{L} Xt1, Xt2, [Xn|SP]` -- FEAT_THE+FEAT_D128 128-bit read-check-write atomic
    /// (`RCWCLRP`/`RCWSETP`/`RCWSWPP`). See [`Arm64RcwAtomicOp`].
    RcwAtomicPair {
        op: Arm64RcwAtomicOp,
        secure: bool,
        ordering: Arm64AtomicOrdering,
        rt1: Arm64GeneralPurposeRegister,
        rt2: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `GCSPUSHM Xt` / `GCSPOPM Xt` / `GCSSS1 Xt` / `GCSSS2 Xt` -- FEAT_GCS guarded-control-stack register
    /// operations, named aliases in the `SYS`/`SYSL` space (`op1=3`, `CRn=7`, `CRm=7`; `op2` selects the
    /// operation). `rt` is the pushed source / popped result. See [`Arm64GcsRegisterOp`].
    GcsRegister {
        op: Arm64GcsRegisterOp,
        rt: Arm64GeneralPurposeRegister,
    },

    /// `GCSPUSHX` / `GCSPOPX` / `GCSPOPCX` -- FEAT_GCS guarded-control-stack exception push/pop, operand-free named
    /// aliases in the `SYS` space (`op1=3, CRn=7, CRm=7`, `Rt=11111`). A fixed word. See [`Arm64GcsExceptionOp`].
    GcsException(Arm64GcsExceptionOp),

    /// `BRB IALL` / `BRB INJ` -- FEAT_BRBE branch-record-buffer maintenance, operand-free named aliases in the
    /// `SYS` space (`op1=1`, `CRn=7`, `CRm=2`). See [`Arm64BranchRecordBufferOp`].
    BranchRecordBuffer(Arm64BranchRecordBufferOp),

    /// `TRCIT Xt` -- FEAT_ITE trace-instrumentation, a named alias in the `SYS` space (`SYS #3, C7, C2, #7, Xt`).
    /// `rt` carries the instrumentation value written to the trace unit.
    TraceInstrumentation(/* rt */ Arm64GeneralPurposeRegister),

    /// `LDAP1`/`STL1 {Vt.D}[index], [Xn|SP]` -- FEAT_LRCPC3 RCpc-ordered single `.d` element load-acquire /
    /// store-release. `index` is 0 or 1. base `0x0D01_8400`: index(Q)`[30]`, L`[22]`, Xn`[9:5]`, Vt`[4:0]`.
    Rcpc3VectorElement {
        load: bool,
        index: u8,
        vt: Arm64FloatRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `RPRFM <prfop|#imm6>, Xm, [Xn|SP]` -- FEAT_RPRFM range-prefetch-memory hint. `prfop` is the 6-bit prefetch
    /// operation (`0`=PLDKEEP, `1`=PSTKEEP, `4`=PLDSTRM, `5`=PSTSTRM; others render as `#imm`). base `0xf8a0_4818`:
    /// Xm`[20:16]`, Xn`[9:5]`, and `prfop` split as `[15]`=p5, `[13:12]`=p4:p3, `[2:0]`=p2:p0.
    RangePrefetch {
        prfop: u8,
        rm: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `SWP{A}{L} Ws, Wt, [Xn]` -- LSE atomic swap: store `Rs` to `[Xn]`, returning the prior value in `Rt`.
    /// Operands: `size`, `ordering`, `rs`, `rt`, `xn`. Requires LSE.
    Swap(
        /* size */ Arm64LoadStoreSize,
        /* ordering */ Arm64AtomicOrdering,
        /* rs */ Arm64GeneralPurposeRegister,
        /* rt */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `LDTADD`/`LDTCLR`/`LDTSET`/`SWPT{A}{L} Ws|Xs, Wt|Xt, [Xn|SP]` -- FEAT_LSUI unprivileged LSE atomic
    /// (the `T` siblings of `LDADD`/`LDCLR`/`LDSET`/`SWP`, restricted to add/clr/set/swap at `W`/`X` only).
    /// `width` selects the 32/64-bit access; `ordering` the A/L suffix. base `0x1920_0400` (`[21]`=1, `[11:10]`=01):
    /// size`[30]`, A`[23]`, R`[22]`, Rs`[20:16]`, `o3:opc``[15:12]`, Rn`[9:5]`, Rt`[4:0]`. See [`Arm64LsuiAtomicOp`].
    LsuiAtomic {
        op: Arm64LsuiAtomicOp,
        width: Arm64RegisterWidth,
        ordering: Arm64AtomicOrdering,
        rs: Arm64GeneralPurposeRegister,
        rt: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    // ---- system: memory barriers, exception-generating, and system-register move ----
    /// `DMB <option>` -- data memory barrier with the given [`Arm64BarrierOption`] shareability/access scope.
    DataMemoryBarrier(/* option */ Arm64BarrierOption),

    /// `DSB <option>` -- data synchronization barrier (a stronger barrier that also waits for completion).
    DataSyncBarrier(/* option */ Arm64BarrierOption),

    /// `ISB` -- instruction synchronization barrier (flushes the pipeline; always the full-system `SY` form).
    InstructionSyncBarrier,

    /// `SB` -- speculation barrier (FEAT_SB): a fixed word in the barrier space (`opc2=111`) that bounds
    /// speculative execution more strongly than the `CSDB` hint. A single operand-free encoding.
    SpeculationBarrier,

    /// `SSBB` / `PSSBB` -- speculative store-bypass barrier (`physical = false`/`true`). These are the otherwise-unnamed
    /// `DSB #0` / `DSB #4` (`CRm = 0`/`4`) encodings; fixed words `0xD503_309F` / `0xD503_349F`. Operand-free, base.
    SpeculativeStoreBypassBarrier { physical: bool },

    /// `BRK #imm16` -- software breakpoint (a self-hosted debug trap; a compiler emits it for `unreachable`).
    Brk(/* imm16 */ u16),

    /// `UDF #imm16` -- permanently undefined: raises an Undefined Instruction exception. The whole word is
    /// `0x0000_0000 | imm16` (top 16 bits zero); compilers emit it as a trap / poison value.
    Udf(/* imm16 */ u16),

    /// `SVC #imm16` -- supervisor call (system call into EL1).
    Svc(/* imm16 */ u16),

    /// `HVC #imm16` -- hypervisor call (a call into EL2).
    Hvc(/* imm16 */ u16),

    /// `SMC #imm16` -- secure monitor call (a call into EL3).
    Smc(/* imm16 */ u16),

    /// `HLT #imm16` -- halting-mode software breakpoint (external debug).
    Hlt(/* imm16 */ u16),

    /// `DCPS1`/`DCPS2`/`DCPS3 {#imm16}` -- debug change PE state to ELx (used by an external debugger). `level` is
    /// `1`/`2`/`3`; `imm16` is conventionally `0`.
    Dcps { level: u8, imm16: u16 },

    /// `ERET` -- exception return (restore PC/PSTATE from ELR/SPSR). A fixed word.
    Eret,

    /// `DRPS` -- debug restore PE state. A fixed word.
    Drps,

    /// `RMIF Xn, #<shift>, #<mask>` -- rotate `Xn` right by `shift` (0..=63) and insert the selected bits into the
    /// NZCV flags per the 4-bit `mask` (FEAT_FlagM).
    Rmif {
        xn: Arm64GeneralPurposeRegister,
        shift: u8,
        mask: u8,
    },

    /// `SETF8 Wn` -- set the NZV flags from the low 8 bits of `Wn` (FEAT_FlagM).
    Setf8(/* wn */ Arm64GeneralPurposeRegister),

    /// `SETF16 Wn` -- set the NZV flags from the low 16 bits of `Wn` (FEAT_FlagM).
    Setf16(/* wn */ Arm64GeneralPurposeRegister),

    /// `CFINV` -- invert the carry flag (FEAT_FlagM). A fixed word.
    Cfinv,

    /// `XAFLAG` -- convert the NZCV flags from Arm to the alternative (external) format (FEAT_FlagM2). Fixed word.
    Xaflag,

    /// `AXFLAG` -- convert the NZCV flags from the alternative format back to Arm (FEAT_FlagM2). Fixed word.
    Axflag,

    /// `SYS #<op1>, C<crn>, C<crm>, #<op2>{, Xt}` -- system instruction (the generic form behind the `DC`/`IC`/
    /// `AT`/`TLBI` cache/TLB/address-translation maintenance aliases). `op1`/`op2` are 3-bit, `crn`/`crm` 4-bit.
    Sys {
        op1: u8,
        crn: u8,
        crm: u8,
        op2: u8,
        rt: Arm64GeneralPurposeRegister,
    },

    /// `SYSL Xt, #<op1>, C<crn>, C<crm>, #<op2>` -- system instruction with result (reads a system value into `Xt`).
    Sysl {
        rt: Arm64GeneralPurposeRegister,
        op1: u8,
        crn: u8,
        crm: u8,
        op2: u8,
    },

    /// `MRS Xt, <sysreg>` -- read a system register into a general register. Operands: `sysreg`, `xt`.
    Mrs(
        /* sysreg */ Arm64SystemRegister,
        /* xt */ Arm64GeneralPurposeRegister,
    ),

    /// `MSR <sysreg>, Xt` -- write a general register into a system register. Operands: `sysreg`, `xt`.
    Msr(
        /* sysreg */ Arm64SystemRegister,
        /* xt */ Arm64GeneralPurposeRegister,
    ),

    /// `MRRS Xt, Xt+1, <sysreg>` / `MSRR <sysreg>, Xt, Xt+1` -- FEAT_SYSREG128 128-bit system-register access into a
    /// 64-bit register pair (`rt` is the even first register). `read` selects `MRRS` (read) vs `MSRR` (write).
    /// `sysreg` is the raw 15-bit `o0:op1:CRn:CRm:op2` specifier (the `[19:5]` field), so any system register encodes;
    /// it emits the generic `S<op0>_<op1>_c<CRn>_c<CRm>_<op2>` form. base `0xD570_0000` read / `0xD550_0000` write.
    SystemRegisterPair {
        read: bool,
        sysreg: u16,
        rt: Arm64GeneralPurposeRegister,
    },

    /// `WFI`/`WFE`/`SEV`/`SEVL`/`YIELD`/`DGH`/`ESB`/`PSB CSYNC`/`TSB CSYNC`/`GCSB DSYNC`/`CSDB`/`CLRBHB`/
    /// `CHKFEAT X16` -- an operand-free hint in the reserved-hint (`NOP`) space. See [`Arm64SystemHintOp`].
    SystemHint(Arm64SystemHintOp),

    /// `PACIASPPC`/`PACIBSPPC`/`PACNBIASPPC`/`PACNBIBSPPC`/`PACM` -- the operand-free pointer-authentication
    /// link-register operations (FEAT_PAuth_LR). Each is a fixed 32-bit word. See [`Arm64PointerAuthLrOp`].
    PointerAuthLr(Arm64PointerAuthLrOp),

    /// `AUTIASPPC`/`AUTIBSPPC`/`RETAASPPC`/`RETABSPPC <label>` -- FEAT_PAuth_LR pointer-auth on `LR` using a
    /// PC-relative label as the discriminator. `offset_bytes` is the signed byte offset to the label (a multiple
    /// of 4, `+/-128 KiB`; the encoded `imm16` is `offset/4`). **Experimental** (LLVM-20-only oracle). See
    /// [`Arm64PointerAuthLrLabelOp`]. Only present with the `experimental` cargo feature.
    #[cfg(feature = "experimental")]
    PointerAuthLrLabel {
        op: Arm64PointerAuthLrLabelOp,
        offset_bytes: i32,
    },

    /// `STSHH keep` / `STSHH strm` -- FEAT_PCDPHINT producer/consumer data-placement (cache-stash) hint. `strm`
    /// selects the streaming (`STRM`) variant over the retained (`KEEP`) variant. A fixed word in the hint space
    /// (`0xD501_961F` / `0xD501_963F`). **Experimental** (LLVM-20-only oracle). Only present with the
    /// `experimental` cargo feature.
    #[cfg(feature = "experimental")]
    Stshh { strm: bool },

    /// `LDF<op>{A}{L} <Vs>, <Vt>, [Xn|SP]` / `STF<op>{L} <Vs>, [Xn|SP]` -- FEAT_LSFE atomic floating-point memory op:
    /// atomically loads, applies `op` with the value in `Vs`, and stores back; the load form returns the old value in
    /// `Vt`, the store form (`rt = None`) discards it. `acquire`/`release` are the ordering bits (`[23]`/`[22]`); `precision`
    /// is the access size. base `0x3C20_0000`: size`[31:30]` (h=01/s=10/d=11), A`[23]`, R`[22]`, store`[15]`, op`[14:12]`,
    /// Vs`[20:16]`, Rn`[9:5]`, Vt`[4:0]`. **Experimental** (LLVM-20-only oracle). Only present with the `experimental` feature.
    #[cfg(feature = "experimental")]
    LsfeAtomicFloat {
        op: Arm64LsfeOp,
        precision: Arm64FloatPrecision,
        acquire: bool,
        release: bool,
        rs: Arm64FloatRegister,
        rt: Option<Arm64FloatRegister>,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `WFET Xt` -- wait for event with timeout (FEAT_WFxT); like `WFE` but bounded by the timeout in `Xt`.
    WaitForEventTimeout(/* xt */ Arm64GeneralPurposeRegister),

    /// `WFIT Xt` -- wait for interrupt with timeout (FEAT_WFxT); like `WFI` but bounded by the timeout in `Xt`.
    WaitForInterruptTimeout(/* xt */ Arm64GeneralPurposeRegister),

    /// `CLREX #<imm>` -- clear the local exclusive monitor. The 4-bit `CRm` operand is conventionally `15` (the
    /// `clrex` form with no immediate); other values assemble but have no architectural effect.
    ClearExclusive(/* imm4 */ u8),

    /// `MSR <pstatefield>, #<imm>` -- write a PSTATE field directly (the immediate `MSR` form). `imm` is the 4-bit
    /// `CRm` value (a 0/1 toggle for the single-bit fields, a 4-bit mask for `DAIFSet`/`DAIFClr`). See
    /// [`Arm64PstateField`].
    MsrImmediate(/* field */ Arm64PstateField, /* imm */ u8),

    // ---- scalar floating-point (f32/f64), tier-2: a separate register file (Vn) + precision (ftype) ----
    /// `FADD Sd|Dd, Sn|Dn, Sm|Dm` -- floating-point add. The leading `precision` selects the 32-bit single
    /// (`S`) or 64-bit double (`D`) form (the `ftype` field). Operands: `precision`, `fd`, `fn`, `fm` -- all
    /// [`Arm64FloatRegister`] at that precision.
    FAdd(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),

    /// `FSUB Sd|Dd, Sn|Dn, Sm|Dm` -- floating-point subtract. Operands as [`Self::FAdd`].
    FSub(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),

    /// `FMUL Sd|Dd, Sn|Dn, Sm|Dm` -- floating-point multiply. Operands as [`Self::FAdd`].
    FMul(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),

    /// `FDIV Sd|Dd, Sn|Dn, Sm|Dm` -- floating-point divide. Operands as [`Self::FAdd`].
    FDiv(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),

    /// `FNEG Sd|Dd, Sn|Dn` -- floating-point negate. Operands: `precision`, `fd`, `fn` ([`Arm64FloatRegister`]).
    FNeg(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FABS Sd|Dd, Sn|Dn` -- floating-point absolute value. Operands as [`Self::FNeg`].
    FAbs(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FSQRT Sd|Dd, Sn|Dn` -- floating-point square root. Operands as [`Self::FNeg`].
    FSqrt(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FMOV Sd|Dd, Sn|Dn` -- floating-point register move (copy `Fn` to `Fd` at the same precision). Operands
    /// as [`Self::FNeg`]. (The FP<->general-purpose and immediate `FMOV` forms are separate variants.)
    FMov(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),

    /// `FCVT <dest>d, <src>n` -- convert a scalar floating-point value between precisions (`H`/`S`/`D`); one
    /// variant covers all six pairs (`dest` and `src` must differ), modeled as a `(destination, source)`
    /// pair of the shared [`Arm64FloatPrecision`] enum. Any pair involving [`Arm64FloatPrecision::Half`] needs `FEAT_FP16`; single<->double is
    /// baseline FP. Operands: `dest`, `src`, `fd` (dest reg), `fn` (source reg).
    FcvtFloat(
        /* dest */ Arm64FloatPrecision,
        /* src */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),

    /// `FCMP Sn, Sm` / `FCMP Dn, Dm` -- floating-point compare, setting the NZCV flags. Operands: `precision`,
    /// `fn`, `fm` ([`Arm64FloatRegister`]). There is no destination (the result is the flags).
    FCmp(
        /* precision */ Arm64FloatPrecision,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),
    /// `FCMP Sn, #0.0` / `FCMP Dn, #0.0` -- floating-point compare with zero, setting NZCV. Operands:
    /// `precision`, `fn`.
    FCmpZero(
        /* precision */ Arm64FloatPrecision,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FCMPE Sn, Sm` / `FCMPE Dn, Dm` -- the SIGNALING floating-point compare (raises Invalid on any NaN, not
    /// just signalling NaNs); the ordered-comparison lowering. Sets NZCV. Operands as [`Self::FCmp`].
    FCmpE(
        /* precision */ Arm64FloatPrecision,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),
    /// `FCMPE Sn, #0.0` / `FCMPE Dn, #0.0` -- signaling compare with zero. Operands as [`Self::FCmpZero`].
    FCmpEZero(
        /* precision */ Arm64FloatPrecision,
        /* fn */ Arm64FloatRegister,
    ),

    /// `FCCMP`/`FCCMPE Sn|Dn|Hn, Sm|Dm|Hm, #nzcv, cond` -- scalar floating-point conditional compare: if `cond`
    /// holds, compare `fn` with `fm` (quiet `FCCMP` or signaling `FCCMPE`) and set NZCV; else set NZCV = `#nzcv`
    /// (0..=15). `precision` is the `ftype` (Half needs FEAT_FP16). `signaling` selects `FCCMPE`.
    FccmpScalar {
        precision: Arm64FloatPrecision,
        signaling: bool,
        fn_: Arm64FloatRegister,
        fm: Arm64FloatRegister,
        nzcv: u8,
        cond: Arm64Condition,
    },

    /// `SCVTF Sd|Dd, Wn|Xn` -- convert a SIGNED integer (general-purpose `rn`, width `gp_width`) to
    /// floating-point (`fd`, `fp_precision`), round to nearest. The GP width and FP precision are independent
    /// (e.g. `scvtf d0, w1` is i32->f64). Operands: `fp_precision`, `gp_width`, `fd` (FP dest), `rn` (GP source).
    Scvtf(
        /* fp_precision */ Arm64FloatPrecision,
        /* gp_width */ Arm64RegisterWidth,
        /* fd */ Arm64FloatRegister,
        /* rn */ Arm64GeneralPurposeRegister,
    ),
    /// `UCVTF Sd|Dd, Wn|Xn` -- convert an UNSIGNED integer to floating-point. Operands as [`Self::Scvtf`].
    Ucvtf(
        /* fp_precision */ Arm64FloatPrecision,
        /* gp_width */ Arm64RegisterWidth,
        /* fd */ Arm64FloatRegister,
        /* rn */ Arm64GeneralPurposeRegister,
    ),
    /// `FCVTZS Wd|Xd, Sn|Dn` -- convert floating-point (`fn`, `fp_precision`) to a SIGNED integer (`rd`,
    /// `gp_width`), rounding toward zero (truncate). Operands: `gp_width`, `fp_precision`, `rd` (GP dest),
    /// `fn` (FP source).
    Fcvtzs(
        /* gp_width */ Arm64RegisterWidth,
        /* fp_precision */ Arm64FloatPrecision,
        /* rd */ Arm64GeneralPurposeRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FCVTZU Wd|Xd, Sn|Dn` -- convert floating-point to an UNSIGNED integer, toward zero. Operands as
    /// [`Self::Fcvtzs`].
    Fcvtzu(
        /* gp_width */ Arm64RegisterWidth,
        /* fp_precision */ Arm64FloatPrecision,
        /* rd */ Arm64GeneralPurposeRegister,
        /* fn */ Arm64FloatRegister,
    ),

    /// `FCVT{N,A,P,M}{S,U} Wd|Xd, Hn|Sn|Dn` -- convert floating-point to a general-purpose integer with an explicit
    /// rounding mode (the non-toward-zero modes; see [`Arm64FpToIntRoundOp`]). `gp_width` is the integer dest size,
    /// `fp_precision` the source (half needs FEAT_FP16). The toward-zero `FCVTZS`/`FCVTZU` are [`Self::Fcvtzs`]/[`Self::Fcvtzu`].
    FcvtFpToIntRound {
        op: Arm64FpToIntRoundOp,
        gp_width: Arm64RegisterWidth,
        fp_precision: Arm64FloatPrecision,
        rd: Arm64GeneralPurposeRegister,
        fn_: Arm64FloatRegister,
    },

    /// `SCVTF Sd|Dd, Wn|Xn, #fbits` -- convert a SIGNED integer to a FIXED-POINT value with `fbits` fractional
    /// bits (`1..=32` for `W`, `1..=64` for `X`; encoded `scale = 64 - fbits`). Operands: `fp_precision`,
    /// `gp_width`, `fd`, `rn`, `fbits`.
    ScvtfFixed(
        /* fp_precision */ Arm64FloatPrecision,
        /* gp_width */ Arm64RegisterWidth,
        /* fd */ Arm64FloatRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* fbits */ u8,
    ),
    /// `UCVTF Sd|Dd, Wn|Xn, #fbits` -- UNSIGNED integer -> fixed-point. Operands as [`Self::ScvtfFixed`].
    UcvtfFixed(
        /* fp_precision */ Arm64FloatPrecision,
        /* gp_width */ Arm64RegisterWidth,
        /* fd */ Arm64FloatRegister,
        /* rn */ Arm64GeneralPurposeRegister,
        /* fbits */ u8,
    ),
    /// `FCVTZS Wd|Xd, Sn|Dn, #fbits` -- convert a FIXED-POINT value (`fbits` fractional bits) to a SIGNED integer,
    /// toward zero. Operands: `gp_width`, `fp_precision`, `rd`, `fn`, `fbits`.
    FcvtzsFixed(
        /* gp_width */ Arm64RegisterWidth,
        /* fp_precision */ Arm64FloatPrecision,
        /* rd */ Arm64GeneralPurposeRegister,
        /* fn */ Arm64FloatRegister,
        /* fbits */ u8,
    ),
    /// `FCVTZU Wd|Xd, Sn|Dn, #fbits` -- fixed-point -> UNSIGNED integer, toward zero. Operands as
    /// [`Self::FcvtzsFixed`].
    FcvtzuFixed(
        /* gp_width */ Arm64RegisterWidth,
        /* fp_precision */ Arm64FloatPrecision,
        /* rd */ Arm64GeneralPurposeRegister,
        /* fn */ Arm64FloatRegister,
        /* fbits */ u8,
    ),
    /// `FJCVTZS Wd, Dn` -- JavaScript `double`->`int32` convert (round toward zero with JS overflow/NaN
    /// semantics), setting the Z flag on exactness. Needs `FEAT_JSCVT`. Operands: `rd` (`W` dest), `fn` (`D` src).
    Fjcvtzs(
        /* rd */ Arm64GeneralPurposeRegister,
        /* fn */ Arm64FloatRegister,
    ),

    // ---- Advanced SIMD (NEON) -- the FIRST families in the ratified named-field-struct + op-sub-enum shape ----
    /// `<op> Vd.<arr>, Vn.<arr>, Vm.<arr>` -- NEON integer "three same" lane operation: arithmetic (`ADD`/`SUB`/
    /// `MUL`), min/max (`SMAX`/`SMIN`/`UMAX`/`UMIN`), the unsigned rounding halving add (`URHADD`), the register
    /// compares (`CMEQ`/`CMGT`/`CMGE`/`CMHI`/`CMHS`), and the variable shifts (`SSHL`/`USHL`). The op is an
    /// orthogonal field over the shared `(arrangement, Vd, Vn, Vm)` shape. Needs Advanced SIMD. (`MUL`, min/max,
    /// and `URHADD` have no 64-bit `.1d`/`.2d` element form.)
    VecInt3Same {
        op: Arm64VectorIntThreeSameOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<arr>, Vm.<arr>` -- NEON floating-point "three same" lane operation: arithmetic
    /// (`FADD`/`FSUB`/`FMUL`/`FDIV`), NaN-propagating min/max (`FMAX`/`FMIN`), and the register compares
    /// (`FCMEQ`/`FCMGE`/`FCMGT`). Valid only for the `.2s`/`.4s`/`.2d` arrangements. Needs Advanced SIMD.
    VecFp3Same {
        op: Arm64VectorFpThreeSameOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<arr>, Vm.<arr>` -- NEON bitwise/logical "three same" op (`AND`/`BIC`/`ORR`/`ORN`/
    /// `EOR`/`BSL`/`BIT`/`BIF`). Size-agnostic: valid only for the `.8b`/`.16b` arrangements (the op is the
    /// orthogonal `(U, size-selector)` field; the arrangement contributes only `Q`). Needs Advanced SIMD.
    VecBitwise {
        op: Arm64VectorBitwiseOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<arr>` -- NEON integer two-register-misc unary (`ABS`/`NEG`), per lane. The
    /// arrangement supplies the element size; valid for every arrangement except the single-lane `.1d`. Needs
    /// Advanced SIMD.
    VecIntUnary {
        op: Arm64VectorIntUnaryOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<arr>` -- NEON floating-point two-register-misc unary (`FABS`/`FNEG`/`FSQRT`), per
    /// lane. Valid only for the `.2s`/`.4s`/`.2d` arrangements. Needs Advanced SIMD.
    VecFpUnary {
        op: Arm64VectorFpUnaryOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op>{2} Vd.<dst>, Vn.<src>` -- NEON vector narrowing extract two-register-misc (`XTN`/`SQXTN`/`SQXTUN`/
    /// `UQXTN`). `dst` is the narrower destination arrangement (`.8b`..`.4s`); the source is the next-wider element
    /// at full 128-bit width, and `dst`'s `Q` selects the `XTN`/`XTN2` (lower / upper destination half) form. Needs
    /// Advanced SIMD. See [`Arm64VectorNarrowOp`].
    VecNarrow {
        op: Arm64VectorNarrowOp,
        dst: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `FMLAL`/`FMLAL2`/`FMLSL`/`FMLSL2 Vd.<2s|4s>, Vn.<2h|4h>, Vm.<2h|4h>` -- NEON FP16 fused-multiply-long
    /// (FEAT_FHM): widen the half-precision sources and multiply-accumulate into the single-precision destination.
    /// `q` selects the `.4s`/`.4h` (128-bit) vs `.2s`/`.2h` (64-bit) form. See [`Arm64VectorFmlalOp`].
    VecFpMulAddLong {
        op: Arm64VectorFmlalOp,
        q: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `FMLAL`/`FMLAL2`/`FMLSL`/`FMLSL2 Vd.<2s|4s>, Vn.<2h|4h>, Vm.h[index]` -- NEON FP16 fused-multiply-long by
    /// INDEXED element (FEAT_FHM): the by-element counterpart of [`Self::VecFpMulAddLong`]. `Vm` is v0-v15, `index`
    /// 0..=7 (the `.h` halfword fold). `q` selects the `.4s`/`.4h` (128-bit) form.
    VecFpMulAddLongByElement {
        op: Arm64VectorFmlalOp,
        q: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `NOT Vd.<arr>, Vn.<arr>` -- NEON bitwise NOT (assembler alias `MVN`), byte-wise; valid only for the
    /// `.8b`/`.16b` arrangements. Needs Advanced SIMD.
    VecNot {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<arr>, #<shift>` -- NEON shift by immediate (`SHL` left; `SSHR`/`USHR` right). The
    /// arrangement gives the element size and `Q`; `shift` is `0..element_bits-1` (left) or `1..element_bits`
    /// (right), folded with the element size into `immh:immb`. `.1d` is invalid. Needs Advanced SIMD.
    VecShiftImm {
        op: Arm64VectorShiftImmOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        shift: u8,
    },

    /// `LDR <Bt|Ht|St|Dt|Qt>, [Xn{, #imm}]` -- SIMD&FP single-register load, unsigned-immediate offset form.
    /// `size` selects the access width (8/16/32/64/128-bit -> `Bt`/`Ht`/`St`/`Dt`/`Qt`); the offset is a
    /// non-negative multiple of the access size, scaled into `imm12` (`byte_offset >> size.scale()`, range
    /// `0..=4095`). `Xn` uses the SP encoding at field 31. The 128-bit `Qt` form is the v128 load. Needs FP/
    /// Advanced SIMD. Operands: `size`, `vt`, `xn`, `offset_bytes`.
    VecLoadRegister(
        /* size */ Arm64VectorLoadStoreSize,
        /* vt */ Arm64FloatRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ u32,
    ),

    /// `STR <Bt|Ht|St|Dt|Qt>, [Xn{, #imm}]` -- SIMD&FP single-register store, unsigned-immediate offset form.
    /// Operands / scaling / `Xn` SP encoding as [`Self::VecLoadRegister`]. The 128-bit `Qt` form is the v128
    /// store.
    VecStoreRegister(
        /* size */ Arm64VectorLoadStoreSize,
        /* vt */ Arm64FloatRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset_bytes */ u32,
    ),

    /// `LDP`/`STP <St|Dt|Qt>, <St2|Dt2|Qt2>, <addr>` -- SIMD&FP load/store PAIR, offset/pre/post-index (DDI0487
    /// C7). `size` is `S`/`D`/`Q` (scale 4/8/16; `B`/`H` are invalid for a pair); `index` picks the addressing
    /// mode; the signed byte offset is scaled by the access size (`imm7` in `-64..=63`). `load` selects LDP vs
    /// STP. Needs FEAT_FP. The direct sibling of [`Self::VecLoadStorePairNonTemporal`] (same `0x2C` pair family,
    /// `idx != 00`); the classic FP prologue `stp d8, d9, [sp, #-16]!` / epilogue `ldp d8, d9, [sp], #16`.
    VecLoadStorePair {
        load: bool,
        index: Arm64LoadStoreIndex,
        size: Arm64VectorLoadStoreSize,
        vt: Arm64FloatRegister,
        vt2: Arm64FloatRegister,
        xn: Arm64GeneralPurposeRegister,
        offset_bytes: i32,
    },

    /// `LDR <St|Dt|Qt>, <label>` -- SIMD&FP load from a PC-relative literal (DDI0487 C7). `size` is `S`/`D`/`Q`
    /// (`opc` 0/1/2); `offset` is the signed byte displacement to the literal, a multiple of 4 in +/-1 MiB (scaled
    /// into `imm19`). Load-only (there is no SIMD&FP store-literal). Needs FEAT_FP. Operands: `size`, `vt`, `offset`.
    VecLoadLiteral(
        /* size */ Arm64VectorLoadStoreSize,
        /* vt */ Arm64FloatRegister,
        /* offset */ i32,
    ),

    /// `LDR <Bt|Ht|St|Dt|Qt>, [Xn, Rm{, <ext> #amount}]` -- SIMD&FP load, register-offset addressing (DDI0487
    /// C7). `size` picks the access width (and `Vt`'s size letter), `extend` the index operation
    /// ([`Arm64MemoryExtend`]; `Lsl` for the plain `[Xn, Xm]` form), and `scaled` the `S` bit (`true` => shift the
    /// index left by `log2(size)`). Needs FEAT_FP. Operands: `size`, `vt`, `xn`, `xm`, `extend`, `scaled`.
    VecLoadRegisterOffset(
        /* size */ Arm64VectorLoadStoreSize,
        /* vt */ Arm64FloatRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* extend */ Arm64MemoryExtend,
        /* scaled */ bool,
    ),

    /// `STR <Bt|Ht|St|Dt|Qt>, [Xn, Rm{, <ext> #amount}]` -- SIMD&FP store, register-offset addressing. Operands
    /// as [`Self::VecLoadRegisterOffset`].
    VecStoreRegisterOffset(
        /* size */ Arm64VectorLoadStoreSize,
        /* vt */ Arm64FloatRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* extend */ Arm64MemoryExtend,
        /* scaled */ bool,
    ),

    /// `LDUR <Bt|Ht|St|Dt|Qt>, [Xn{, #imm}]` / `LDR <..>, [Xn, #imm]!` / `LDR <..>, [Xn], #imm` -- SIMD&FP load,
    /// 9-bit unscaled immediate (DDI0487 C7): `mode` is Unscaled (LDUR) / PreIndex / PostIndex -- the
    /// [`Arm64Imm9Mode::Unprivileged`] (LDTR/STTR) mode is invalid for SIMD&FP. `offset` is a raw signed byte
    /// value in `-256..=255`. Needs FEAT_FP. Operands: `size`, `mode`, `vt`, `xn`, `offset`.
    VecLoadRegisterImm9(
        /* size */ Arm64VectorLoadStoreSize,
        /* mode */ Arm64Imm9Mode,
        /* vt */ Arm64FloatRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    /// `STUR <Bt|Ht|St|Dt|Qt>, [Xn{, #imm}]` / `STR <..>, [Xn, #imm]!` / `STR <..>, [Xn], #imm` -- SIMD&FP store,
    /// 9-bit unscaled immediate. Operands / mode constraints as [`Self::VecLoadRegisterImm9`].
    VecStoreRegisterImm9(
        /* size */ Arm64VectorLoadStoreSize,
        /* mode */ Arm64Imm9Mode,
        /* vt */ Arm64FloatRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* offset */ i32,
    ),

    /// `<op> Vd.<arr>, Vn.<arr>, Vm.<arr>` -- NEON permute (`ZIP1`/`ZIP2` interleave, `UZP1`/`UZP2`
    /// de-interleave, `TRN1`/`TRN2` transpose); all three registers share the arrangement. `.1d` is invalid.
    /// Needs Advanced SIMD.
    VecPermute {
        op: Arm64VectorPermuteOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op>{2} Vd.<Ta>, Vn.<Tb>, Vm.<Tc>` -- NEON "three different" widening/narrowing op (`SMULL`/`SADDL`/
    /// `SMLAL`/`SQDMULL` long, `SADDW`/`SSUBW` wide, `ADDHN`/`SUBHN` narrow, and their unsigned/rounding kin).
    /// `wide` is the 128-bit side (`.8h`/`.4s`/`.2d`); `high` selects the upper-half (`2`-suffix, `Q=1`) form of
    /// the 64/128-bit narrow operand. The op's [`Arm64VectorThreeDifferentOp::shape`] fixes which operands are
    /// wide vs narrow. Needs Advanced SIMD.
    VecThreeDifferent {
        op: Arm64VectorThreeDifferentOp,
        wide: Arm64VectorArrangement,
        high: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op>{2} Vd.<Ta>, Vn.<Tb>, #<shift>` -- NEON shift-by-immediate with a size change: the *long* widening
    /// left shift (`SSHLL`/`USHLL`; `SXTL`/`UXTL` = `#0`) or the *narrowing* right shifts (`SHRN`/`SQSHRN`/...).
    /// `narrow` is the narrow side (`.8b`/`.16b`/`.4h`/`.8h`/`.2s`/`.4s`) -- its element size + the `Q` upper-half
    /// (`2`-suffix) bit; the wide side is twice as wide. `shift` is `0..narrow_bits-1` (long) or `1..narrow_bits`
    /// (narrow). Needs Advanced SIMD.
    VecShiftLongNarrow {
        op: Arm64VectorShiftLongNarrowOp,
        narrow: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        shift: u8,
    },

    /// `SHLL{2} Vd.<Ta>, Vn.<Tb>, #<shift>` -- NEON shift left long by the element size: widen each `.b`/`.h`/`.s`
    /// source lane into the double-width `.8h`/`.4s`/`.2d` destination, shifted left by `8`/`16`/`32` (= the source
    /// element bits). Unlike [`Self::VecShiftLongNarrow`] this is a two-register-misc encoding with a FIXED shift.
    /// `size` is the source element (`B`/`H`/`S`); `high` selects `SHLL2` (the `.16b`/`.8h`/`.4s` upper-half source).
    /// base `0x2E21_3800`: `Q(high)[30]`, `size[23:22]`. Needs Advanced SIMD.
    VecShll {
        high: bool,
        size: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> <V>d, Vn.<arr>` -- NEON across-lanes reduction: `ADDV`/`SMAXV`/`SMINV`/`UMAXV`/`UMINV` reduce to a
    /// same-width scalar, `SADDLV`/`UADDLV` to a double-width scalar, `FMAXV`/`FMINV`/`FMAXNMV`/`FMINNMV` to an FP
    /// single. `Vd` is the scalar result (named `b`/`h`/`s`/`d` by its width). Integer sources are
    /// `.8b`/`.16b`/`.4h`/`.8h`/`.4s`; FP sources are `.4s`. Needs Advanced SIMD.
    VecAcrossLanes {
        op: Arm64VectorAcrossLanesOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<arr>, #0` (integer) / `#0.0` (FP) -- NEON compare each lane against zero
    /// (`CMGT`/`CMGE`/`CMEQ`/`CMLE`/`CMLT`, `FCMGT`/`FCMGE`/`FCMEQ`/`FCMLE`/`FCMLT`). Integer arrangements are
    /// every one but `.1d`; FP arrangements are `.2s`/`.4s`/`.2d`. Needs Advanced SIMD.
    VecCompareZero {
        op: Arm64VectorCompareZeroOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `EXT Vd.<arr>, Vn.<arr>, Vm.<arr>, #<index>` -- NEON extract: take the `index`-byte-aligned window spanning
    /// the concatenation `Vn:Vm`. Valid only for `.8b` (`index` 0..=7) and `.16b` (`index` 0..=15). Needs Advanced
    /// SIMD. Operands: `arrangement`, `rd`, `rn`, `rm`, `index`.
    VecExt {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> Vd.<arr>, #imm8{, <shift>}` -- NEON modified immediate (`MOVI`/`MVNI`/`ORR`/`BIC` vector constant).
    /// The `arrangement` fixes the element width (8-bit MOVI is `.8b`/`.16b`; 16-bit and 32-bit forms add shifts;
    /// 64-bit `MOVI` is `.2d`); `shift` is the LSL/MSL applied to `imm8` before replication (valid combinations
    /// depend on the element width). Needs Advanced SIMD. Operands: `op`, `arrangement`,
    /// `imm8`, `shift`, `rd`.
    VecModifiedImmediate {
        op: Arm64VectorModifiedImmediateOp,
        arrangement: Arm64VectorArrangement,
        imm8: u8,
        shift: Arm64VectorImmediateShift,
        rd: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<arr>, Vm.<ts>[index]` -- NEON vector by indexed element (same element width): every
    /// lane of `Vn` is multiplied/accumulated against the single broadcast lane `Vm.<ts>[index]`. The integer
    /// ops (`MUL`/`MLA`/`MLS`/`SQDMULH`/`SQRDMULH`) take `.4h`/`.8h`/`.2s`/`.4s`; the FP ops (`FMUL`/`FMLA`/`FMLS`/
    /// `FMULX`) take `.2s`/`.4s`/`.2d`. For `.h` the index is `0..=7` and `Vm` is `v0`-`v15`; `.s` index `0..=3`;
    /// `.d` index `0..=1`. Needs Advanced SIMD.
    VecByElement {
        op: Arm64VectorByElementOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op>{2} Vd.<wide>, Vn.<narrow>, Vm.<ts>[index]` -- NEON long vector by indexed element (result twice the
    /// source width): `SMULL`/`UMULL`/`SMLAL`/`UMLAL`/`SMLSL`/`UMLSL`/`SQDMULL`/`SQDMLAL`/`SQDMLSL`. `wide` is the
    /// 128-bit destination (`.4s` or `.2d`); `high` selects the upper-half (`2`-suffix) of `Vn`; the index + `Vm`
    /// fold into `H:L:M` per the narrow element size (`.h` for `.4s`, `.s` for `.2d`). For `.h`, `Vm` is `v0`-`v15`
    /// and index `0..=7`; for `.s`, index `0..=3`. Needs Advanced SIMD.
    VecByElementLong {
        op: Arm64VectorByElementLongOp,
        wide: Arm64VectorArrangement,
        high: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `DUP Vd.<arr>, Vn.<ts>[index]` -- NEON duplicate (broadcast) one lane of `Vn` across every lane of `Vd`.
    /// The arrangement gives the element size + lane count (`.1d` invalid); `index` selects the source lane
    /// (`< 16 / element_bytes`). Needs Advanced SIMD.
    VecDupElement {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        index: u8,
    },

    /// `INS Vd.<ts>[dst_index], Vn.<ts>[src_index]` -- NEON insert (copy) one lane of `Vn` into one lane of `Vd`
    /// (the other lanes of `Vd` are preserved). Both lanes share the `element` size; this is a 128-bit operation.
    /// Needs Advanced SIMD.` alias disassembly.)
    VecInsElement {
        element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        dst_index: u8,
        rn: Arm64FloatRegister,
        src_index: u8,
    },

    /// `<op>{2} Vd.<...>, Vn.<...>` -- NEON FP length convert: `FCVTL` widens (f16->f32 / f32->f64), `FCVTN`
    /// narrows (f32->f16 / f64->f32), `FCVTXN` narrows f64->f32 (round-to-odd). `wide` is the 128-bit FP side
    /// (`.4s` or `.2d`); `high` selects the upper-half (`2`-suffix). For `FCVTL`, `Vd` is wide and `Vn` narrow;
    /// for `FCVTN`/`FCVTXN`, `Vd` is narrow and `Vn` wide. Needs Advanced SIMD.
    VecFpConvertLength {
        op: Arm64VectorFpConvertLengthOp,
        wide: Arm64VectorArrangement,
        high: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `FCMLA Vd.<arr>, Vn.<arr>, Vm.<arr>, #<rotation>` -- NEON complex floating-point multiply-accumulate with a
    /// `0`/`90`/`180`/`270` rotation. `arrangement` is `.2s`/`.4s`/`.2d`. Needs FEAT_FCMA. (For complex-math libraries.)
    VecFcmla {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
        rotation: Arm64ComplexRotation,
    },

    /// `FCADD Vd.<arr>, Vn.<arr>, Vm.<arr>, #<rotation>` -- NEON complex floating-point add with a `90`/`270`
    /// rotation. `arrangement` is `.2s`/`.4s`/`.2d`. Needs FEAT_FCMA.
    VecFcadd {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
        rotation: Arm64ComplexRotation,
    },

    /// `FCMLA Vd.<arr>, Vn.<arr>, Vm.<ts>[index], #<rotation>` -- NEON complex FP multiply-accumulate by an INDEXED
    /// element (FEAT_FCMA; the `.4h`/`.8h` forms also need FEAT_FP16). `arrangement` is `.4h`/`.8h`/`.4s`; `vm` is
    /// `v0..v31` (encoded `M[20]:Rm[19:16]`); `index` selects the complex-pair element (`.4h`/`.8h` 0..=3 via
    /// `L[21]:H[11]`, `.4s` 0..=1 via `H[11]`). base `0x2F00_1000`: `Q[30]`, `size[23:22]`, `rotation[14:13]`.
    VecFcmlaByElement {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
        rotation: Arm64ComplexRotation,
    },

    /// `<op> Vd.<arr>, Vn.<arr>, Vm.<arr>` -- NEON rounding doubling multiply-accumulate (`SQRDMLAH`/`SQRDMLSH`),
    /// vector form. `arrangement` is `.4h`/`.8h`/`.2s`/`.4s`. Needs FEAT_RDM.
    VecRdm {
        op: Arm64VectorRdmOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<arr>, Vm.<ts>[index]` -- NEON rounding doubling multiply-accumulate by indexed element
    /// (`SQRDMLAH`/`SQRDMLSH`). `arrangement` is `.4h`/`.8h`/`.2s`/`.4s`. Needs FEAT_RDM.
    VecRdmByElement {
        op: Arm64VectorRdmOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> Vd.16b, Vn.16b` -- AES cryptographic round step (`AESE`/`AESD`/`AESMC`/`AESIMC`). Always `.16b`.
    /// Needs the cryptography extension (FEAT_AES).
    VecAes {
        op: Arm64VectorAesOp,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> <Vd>, <Vn>, Vm.4s` -- three-register SHA1/SHA256 acceleration. The `Vd`/`Vn` operand views (`Qd, Sn`
    /// for SHA1C/P/M; `Qd, Qn` for SHA256H/H2; `.4s` for the schedule-update ops) come from the op. Needs the
    /// cryptography extension.
    VecSha3 {
        op: Arm64VectorSha3Op,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> <Vd>, <Vn>` -- two-register SHA1/SHA256 acceleration (`SHA1H` scalar `Sd, Sn`; `SHA1SU1`/`SHA256SU0`
    /// `.4s`). Needs the cryptography extension.
    VecSha2 {
        op: Arm64VectorSha2Op,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `SDOT`/`UDOT Vd.<arr>, Vn.<bt>, Vm.<bt>` -- NEON 4-way 8-bit dot product into 32-bit lanes (`unsigned`
    /// picks UDOT). `arrangement` is `.2s` or `.4s` (the accumulator); `Vn`/`Vm` are the `.8b`/`.16b` byte
    /// sources. Needs FEAT_DotProd.
    VecDotProduct {
        unsigned: bool,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `SDOT`/`UDOT Vd.<arr>, Vn.<bt>, Vm.4b[index]` -- NEON dot product against a broadcast 4-byte group of
    /// `Vm` (`index` selects which 32-bit group, `0..=3`). `arrangement` is `.2s`/`.4s`. Needs FEAT_DotProd.
    VecDotProductByElement {
        unsigned: bool,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> Vd.4s, Vn.16b, Vm.16b` -- NEON integer matrix multiply-accumulate (`SMMLA`/`UMMLA`/`USMMLA`): an
    /// 8x8 -> 2x2 32-bit matrix product into the fixed `.4s`/`.16b` shape. Needs FEAT_I8MM. (Machine-learning int8 GEMM.)
    VecMatrixMultiply {
        op: Arm64VectorMatMulOp,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `USDOT Vd.<arr>, Vn.<bt>, Vm.<bt>` -- NEON mixed-sign 4-way 8-bit dot product (unsigned `Vn` x signed
    /// `Vm`), accumulated into `Vd.<2s|4s>`. Needs FEAT_I8MM. (The vector form exists only for `USDOT`.)
    VecUsdot {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> Vd.<arr>, Vn.<bt>, Vm.4b[index]` -- NEON mixed-sign dot product against a broadcast 4-byte group of
    /// `Vm` (`USDOT` = unsigned x signed, `SUDOT` = signed x unsigned). `arrangement` is `.2s`/`.4s`; `index` is
    /// `0..=3`. Needs FEAT_I8MM.
    VecMixedDotByElement {
        op: Arm64VectorMixedDotOp,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> <V>d, <V>n, <V>m` -- scalar Advanced SIMD three-same integer op (`SQADD`/`SQSUB`/`ADD`/`CMGT`/the
    /// scalar shifts/`SQDMULH`/...), operating on a single `b`/`h`/`s`/`d` register (`element`). Needs Advanced SIMD.
    ScalarThreeSame {
        op: Arm64ScalarThreeSameOp,
        element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> <s|d>d, <s|d>n, <s|d>m` -- scalar Advanced SIMD three-same floating-point op (`FABD`/`FMULX`/the FP
    /// compares/`FRECPS`/`FRSQRTS`). `double` selects `d` (else `s`). Needs Advanced SIMD.
    ScalarFpThreeSame {
        op: Arm64ScalarFpThreeSameOp,
        double: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> <V>d, <V>n[, #0]` -- scalar Advanced SIMD two-register-misc integer op (`SQABS`/`SQNEG`/`SUQADD`/
    /// `USQADD`/`ABS`/`NEG`/the compare-against-zero forms), on a single `b`/`h`/`s`/`d` register. Needs Advanced SIMD.
    ScalarTwoMisc {
        op: Arm64ScalarTwoMiscOp,
        element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> <s|d>d, <s|d>n[, #0.0]` -- scalar Advanced SIMD two-register-misc floating-point op (FP-to-int and
    /// int-to-FP converts, `FRECPE`/`FRSQRTE`/`FRECPX`, the FP compare-against-zero forms). Needs Advanced SIMD.
    ScalarFpTwoMisc {
        op: Arm64ScalarFpTwoMiscOp,
        double: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op>{2} Vd.8h, Vn.<8b|16b>` -- FP8 (FEAT_FP8) convert-long: widen the 8-bit floating-point lanes of the lower
    /// `.8b` (`upper = false`) or upper `.16b` (`upper = true`, the `2`-suffix) half of `Vn` to eight FP16 / BFloat16
    /// lanes in `Vd.8h`. The `op` selects the numeric format and destination type ([`Arm64Fp8ConvertLongOp`]).
    Fp8ConvertLong {
        op: Arm64Fp8ConvertLongOp,
        upper: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `FCVTN Vd.<8b|16b>, Vn.<4h|8h|4s>, Vm.<...>` -- NEON FP8 narrowing convert (FEAT_FP8): narrow two FP16/FP32 source
    /// vectors into 8-bit floating-point lanes. `fp32` sources are `.4s` (`q` must be `false`, `Vd.8b`); FP16 sources are
    /// `.4h` (`q=false`, `Vd.8b`) or `.8h` (`q=true`, `Vd.16b`). base `0x0E00_F400`: Q`[30]`, fp16`[22]`, Vm`[20:16]`, Vn`[9:5]`, Vd`[4:0]`.
    VecFp8Narrow {
        q: bool,
        fp32: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `LUTI2`/`LUTI4 Vd.<16b|8h>, { Vn.<16b|8h>(-...) }, Vm[index]` -- NEON table-vector lookup (FEAT_LUT). `lut4` picks
    /// the 4-bit `LUTI4` (else 2-bit `LUTI2`); `half` picks the `.8h` element (else `.16b`). The element-index width is
    /// `(lut4?1:2) + (half?1:0)`, left-aligned into `[14:12]` with a trailing 1-marker. base `0x4E00_0000`: luti2`[23]`,
    /// half`[22]`, Vm`[20:16]`, Vn`[9:5]`, Vd`[4:0]`. `LUTI4 .8h` reads a two-register table list `{Vn, Vn+1}`.
    VecLuti {
        lut4: bool,
        half: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
        index: u8,
    },

    /// `FMMLA Vd.<4s|8h>, Vn.16b, Vm.16b` -- NEON FP8 matrix multiply-accumulate (FEAT_F8F32MM for `.4s`, FEAT_F8F16MM
    /// for `.8h`). `half` picks the `.8h` (FP8 -> FP16) accumulator. base `0x6E00_EC00` (`[15:10]`=111011 like BFMMLA but
    /// `[22]`=0): the `.4s` form sets `[23]`. Vm`[20:16]`, Vn`[9:5]`, Vd`[4:0]`.
    VecFp8Matmul {
        half: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `FCVT*`/`SCVTF`/`UCVTF <Sd|Dd>, <Dn|Sn>` -- FEAT_FPRCVT scalar convert between a floating-point value and an
    /// integer held in the **other-size** FP register. `wide_dest` selects the 64-bit (`Dd`) destination. base
    /// `0x1E00_0000` (scalar 1-source frame, `[15:10]`=000000): the integer operand's 64-bitness is `[31]` and the FP
    /// operand's size is `[22]` (both derived from the op direction + `wide_dest`); opcode at `[21:16]`. See [`Arm64FprcvtOp`].
    Fprcvt {
        op: Arm64FprcvtOp,
        wide_dest: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `FDOT Vd.<4h|2s|8h|4s>, Vn.<8b|16b>, Vm.<8b|16b>` -- FP8 (FEAT_FP8DOT2/4) dot product: a 2-way (`half`, FP8 ->
    /// FP16, `.4h`/`.8h`) or 4-way (`!half`, FP8 -> FP32, `.2s`/`.4s`) dot product of 8-bit floating-point lanes. `wide`
    /// selects the 128-bit (`.16b` source) form.
    Fp8Dot {
        half: bool,
        wide: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `FMLAL{B|T} Vd.8h, Vn.16b, Vm.16b` -- FP8 (FEAT_FP8FMA) widening fused multiply-accumulate (FP8 -> FP16): `top`
    /// selects the `FMLALT` (odd lanes) over `FMLALB` (even lanes) form.
    Fp8MlalLong {
        top: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `FMLALL{BB|BT|TB|TT} Vd.4s, Vn.16b, Vm.16b` -- FP8 (FEAT_FP8FMA) 4-way widening fused multiply-accumulate
    /// (FP8 -> FP32). The two `top` flags pick the lane group: `first_top` is the first letter (B/T), `second_top` the
    /// second.
    Fp8MlalLongLong {
        first_top: bool,
        second_top: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `FDOT Vd.<4h|2s|8h|4s>, Vn.<8b|16b>, Vm.<2b|4b>[index]` -- FP8 (FEAT_FP8DOT2/4) dot product by indexed element.
    /// `half` selects the 2-way FP8->FP16 form (`Vm.2b[0..=7]`, `Vm` is v0-v15) over the 4-way FP8->FP32 form
    /// (`Vm.4b[0..=3]`, `Vm` is v0-v31); `wide` selects the 128-bit (`.16b` source) variant.
    Fp8DotByElement {
        half: bool,
        wide: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `FMLAL{B|T} Vd.8h, Vn.16b, Vm.b[index]` -- FP8 (FEAT_FP8FMA) widening fused multiply-accumulate (FP8 -> FP16)
    /// by indexed element. `top` selects `FMLALT`. `Vm` is v0-v7, `index` 0..=15.
    Fp8MlalLongByElement {
        top: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `FMLALL{BB|BT|TB|TT} Vd.4s, Vn.16b, Vm.b[index]` -- FP8 (FEAT_FP8FMA) 4-way widening fused multiply-accumulate
    /// (FP8 -> FP32) by indexed element. `first_top`/`second_top` pick the two lane groups. `Vm` is v0-v7, `index` 0..=15.
    Fp8MlalLongLongByElement {
        first_top: bool,
        second_top: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> Vd.<4h|8h>, Vn.<4h|8h>, Vm.<4h|8h>` -- NEON half-precision (FP16) three-same op (`FADD`/`FMUL`/the FP
    /// compares/`FMLA`/...), the `.4h`/`.8h` counterpart of [`Self::VecFp3Same`]. `wide` selects `.8h`. Needs FEAT_FP16.
    VecFp16ThreeSame {
        op: Arm64VectorFpThreeSameOp,
        wide: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> Vd.<4h|8h>, Vn.<4h|8h>[, #0.0]` -- NEON half-precision (FP16) two-register-misc op (FP rounding, the
    /// FP<->int converts, `FRECPE`/`FRSQRTE`, `FABS`/`FNEG`/`FSQRT`, the compare-against-zero forms). `wide` selects
    /// `.8h`. Needs FEAT_FP16.
    VecFp16TwoMisc {
        op: Arm64VectorFp16TwoMiscOp,
        wide: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> Vd.<4h|8h>, Vn.<4h|8h>, Vm.h[index]` -- NEON half-precision (FP16) by-element multiply (`FMUL`/`FMLA`/
    /// `FMLS`/`FMULX`). `wide` selects `.8h`; `index` is `0..=7`. Needs FEAT_FP16.
    VecFp16ByElement {
        op: Arm64VectorFp16ByElementOp,
        wide: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> Hd, Vn.<4h|8h>` -- NEON half-precision (FP16) across-lanes reduce (`FMAXV`/`FMINV`/`FMAXNMV`/`FMINNMV`)
    /// to a single `h` result. `wide` selects `.8h`. Needs FEAT_FP16.
    VecFp16Across {
        op: Arm64VectorFp16AcrossOp,
        wide: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `FMOV Vd.<arr>, #<fp>` -- NEON move floating-point immediate (the modified-immediate `cmode=1111` form).
    /// `arrangement` is `.2s`/`.4s`/`.2d` (Advanced SIMD) or `.4h`/`.8h` (FEAT_FP16); `imm8` is the 8-bit
    /// minifloat field (use [`crate::fp8_decode_single`] to recover the value).
    VecFmovImmediate {
        arrangement: Arm64VectorArrangement,
        imm8: u8,
        rd: Arm64FloatRegister,
    },

    /// `<op> Hd, Hn, Hm` -- scalar half-precision (FP16) three-same op (`FABD`/`FMULX`/the FP compares/`FRECPS`/
    /// `FRSQRTS`) on the `h` register. Needs FEAT_FP16.
    ScalarFp16ThreeSame {
        op: Arm64ScalarFpThreeSameOp,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `<op> Hd, Hn[, #0.0]` -- scalar half-precision (FP16) two-register-misc op (the FP<->int converts,
    /// `FRECPE`/`FRSQRTE`/`FRECPX`, the compare-against-zero forms) on the `h` register. Needs FEAT_FP16.
    ScalarFp16TwoMisc {
        op: Arm64ScalarFpTwoMiscOp,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> Hd, Hn, Vm.h[index]` -- scalar half-precision (FP16) by-element multiply (`FMUL`/`FMLA`/`FMLS`/
    /// `FMULX`). `index` is `0..=7`. Needs FEAT_FP16.
    ScalarFp16ByElement {
        op: Arm64ScalarByElementOp,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `ADDP Dd, Vn.2d` -- scalar Advanced SIMD pairwise add (reduce the two doubleword lanes of `Vn` to `Dd`).
    /// Needs Advanced SIMD.
    ScalarAddp {
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> <s|d>d, Vn.<2s|2d>` -- scalar Advanced SIMD floating-point pairwise reduce (`FADDP`/`FMAXP`/`FMINP`/
    /// `FMAXNMP`/`FMINNMP`). `double` selects `.2d`->`d` (else `.2s`->`s`). Needs Advanced SIMD.
    ScalarFpPairwise {
        op: Arm64ScalarFpPairwiseOp,
        double: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `DUP <V>d, Vn.<ts>[index]` -- scalar Advanced SIMD duplicate a vector lane into a scalar register. `element`
    /// is the lane size (`b`/`h`/`s`/`d`). (GNU/LLVM print the `mov` alias.) Needs Advanced SIMD.
    ScalarDup {
        element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> <V>d, <V>n, Vm.<ts>[index]` -- scalar Advanced SIMD by-element (indexed) multiply (`SQDMULH`/`FMUL`/
    /// `FMLA`/...), on a single `h`/`s`/`d` register. Int ops are `.h`/`.s`, FP ops `.s`/`.d`. Needs Advanced SIMD
    /// (the `SQRDMLAH`/`SQRDMLSH` forms need FEAT_RDM).
    ScalarByElement {
        op: Arm64ScalarByElementOp,
        element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> <V>d, <V>n, Vm.<ts>[index]` -- scalar Advanced SIMD by-element long multiply (`SQDMULL`/`SQDMLAL`/
    /// `SQDMLSL`): the destination is one size wider than `src_element` (`s<-h`, `d<-s`). Needs Advanced SIMD.
    ScalarByElementLong {
        op: Arm64ScalarByElementLongOp,
        src_element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `<op> <V>d, <V>n, #shift` -- scalar Advanced SIMD shift-by-immediate (`SSHR`/`SHL`/`SQSHL`/...), on a single
    /// `b`/`h`/`s`/`d` register. `shift` is `1..element_bits` (right) or `0..element_bits-1` (left). Needs Advanced SIMD.
    ScalarShiftImm {
        op: Arm64ScalarShiftImmOp,
        element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        shift: u8,
    },

    /// `<op> <V>d, <V>n, #shift` -- scalar Advanced SIMD narrowing shift-right (`SQSHRN`/`SQRSHRN`/`SQSHRUN`/
    /// `SQRSHRUN`/`UQSHRN`/`UQRSHRN`): `narrow_element` (`b`/`h`/`s`) is the destination; the source is one size
    /// wider. `shift` is `1..narrow_bits`. Needs Advanced SIMD.
    ScalarShiftNarrow {
        op: Arm64ScalarShiftNarrowOp,
        narrow_element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        shift: u8,
    },

    /// `<op> <s|d>d, <s|d>n, #fbits` -- scalar Advanced SIMD fixed-point convert (`SCVTF`/`UCVTF`/`FCVTZS`/
    /// `FCVTZU`). `element` is `s`/`d`; `fbits` (the fractional bit count) is `1..esize`. Needs Advanced SIMD.
    ScalarFixedConvert {
        op: Arm64ScalarFixedConvertOp,
        element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        fbits: u8,
    },

    /// `<op> <V>d, <V>n` -- scalar Advanced SIMD saturating narrow (`SQXTN`/`SQXTUN`/`UQXTN`): the destination is
    /// `dst_element` (`b`/`h`/`s`) and the source is one size wider. Needs Advanced SIMD.
    ScalarNarrow {
        op: Arm64ScalarNarrowOp,
        dst_element: Arm64VectorElement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `FCVTXN Sd, Dn` -- scalar FP convert to narrower precision, rounding to odd (the only scalar form: `s` from
    /// `d`). Needs Advanced SIMD.
    ScalarFcvtxn {
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// A three-register SHA512/SM3/SM4 crypto op (`SHA512H`/`SHA512H2`/`SHA512SU1`/`RAX1`/`SM3PARTW1`/`SM3PARTW2`/
    /// `SM4EKEY`). The `op` carries the base + display views + extension. Needs the matching FEAT_SHA512/SHA3/SM3/SM4.
    VecCrypto3 {
        op: Arm64VectorCrypto3Op,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// A two-register SHA512/SM4 crypto op (`SHA512SU0`/`SM4E`). Needs FEAT_SHA512 / FEAT_SM4.
    VecCrypto2 {
        op: Arm64VectorCrypto2Op,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// A four-register SHA3/SM3 crypto op (`EOR3`/`BCAX`/`SM3SS1`) with three source registers `Vn`/`Vm`/`Va`.
    /// Needs FEAT_SHA3 / FEAT_SM3.
    VecCrypto4 {
        op: Arm64VectorCrypto4Op,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
        ra: Arm64FloatRegister,
    },

    /// `XAR Vd.2d, Vn.2d, Vm.2d, #rotate` -- SHA-3 XOR-and-rotate (FEAT_SHA3). `rotate` is `0..=63`.
    VecXar {
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
        rotate: u8,
    },

    /// `<op> Vd.4s, Vn.4s, Vm.s[index]` -- indexed SM3 "TT" op (`SM3TT1A`/`SM3TT1B`/`SM3TT2A`/`SM3TT2B`, FEAT_SM3).
    /// `index` is `0..=3`.
    VecSm3Tt {
        op: Arm64VectorSm3TtOp,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `PMULL`/`PMULL2 Vd.<8h|1q>, Vn.<bt>, Vm.<bt>` -- NEON polynomial multiply long. `poly64=false` is the
    /// `.8b`/`.16b` -> `.8h` form (needs Advanced SIMD); `poly64=true` is the `.1d`/`.2d` -> `.1q` form (needs
    /// FEAT_PMULL, the AES crypto level). `high` selects the `2`-suffix (upper-half source).
    VecPmull {
        high: bool,
        poly64: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `BFDOT Vd.<arr>, Vn.<bt>, Vm.<bt>` -- NEON BFloat16 2-way dot product into an f32 accumulator. `arrangement`
    /// is `.2s`/`.4s` (the f32 accumulator); `Vn`/`Vm` are `.4h`/`.8h` bf16 pairs. Needs FEAT_BF16.
    VecBfdot {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `BFDOT Vd.<arr>, Vn.<bt>, Vm.2h[index]` -- NEON BFloat16 dot product against a broadcast 2-halfword group of
    /// `Vm` (`index` `0..=3`). `arrangement` is `.2s`/`.4s`. Needs FEAT_BF16.
    VecBfdotByElement {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `BFMMLA Vd.4s, Vn.8h, Vm.8h` -- NEON BFloat16 matrix multiply-accumulate (a 2x4 x 4x2 -> 2x2 f32 product)
    /// in the fixed `.4s`/`.8h` shape. Needs FEAT_BF16.
    VecBfmmla {
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `BFMLALB`/`BFMLALT Vd.4s, Vn.8h, Vm.8h` -- NEON BFloat16 multiply-accumulate long: the bottom (`top=false`)
    /// or top (`top=true`) bf16 element of each f32 lane, widened into `Vd.4s`. Needs FEAT_BF16.
    VecBfmlal {
        top: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        rm: Arm64FloatRegister,
    },

    /// `BFMLALB`/`BFMLALT Vd.4s, Vn.8h, Vm.h[index]` -- NEON BFloat16 multiply-accumulate long against a broadcast
    /// halfword lane of `Vm` (`index` `0..=7`). `top` selects bottom/top. Needs FEAT_BF16.
    VecBfmlalByElement {
        top: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
        vm: Arm64FloatRegister,
        index: u8,
    },

    /// `BFCVTN`/`BFCVTN2 Vd.<4h|8h>, Vn.4s` -- NEON convert f32 to BFloat16, narrowing into the lower (`top=false`,
    /// `.4h`) or upper (`top=true`, `.8h`) half of `Vd`. Needs FEAT_BF16.
    VecBfcvtn {
        top: bool,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `BFCVT Hd, Sn` -- scalar convert a single-precision value to BFloat16 (held in the `H` view). Needs
    /// FEAT_BF16.
    BfConvertScalar {
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `<op> Vd.<wide>, Vn.<narrow>` -- NEON add long pairwise (`SADDLP`/`UADDLP`/`SADALP`/`UADALP`): sum adjacent
    /// element pairs of `Vn` into a `Vd` with twice the element width and half the lanes. `narrow` is the source
    /// arrangement (`.8b`/`.16b`/`.4h`/`.8h`/`.2s`/`.4s`). Needs Advanced SIMD.
    VecAddPairwiseLong {
        op: Arm64VectorAddPairwiseLongOp,
        narrow: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64FloatRegister,
    },

    /// `LDN`/`STN { Vt.<arr> .. }, [Xn]` -- NEON load/store multiple structures (no-offset form): transfer the
    /// `kind`'s register count starting at `rt_first` (consecutive, wrapping mod 32) to/from `[Xn]`. `LD1`/`ST1`
    /// move 1-4 registers contiguously; `LD2`/`LD3`/`LD4` de-interleave. `load` selects load vs store. `Xn` uses
    /// the SP encoding at 31. Needs Advanced SIMD.
    VecLoadStoreMultiple {
        kind: Arm64VectorStructureKind,
        load: bool,
        arrangement: Arm64VectorArrangement,
        rt_first: Arm64FloatRegister,
        xn: Arm64GeneralPurposeRegister,
    },

    /// `LD<n>`/`ST<n> { Vt.<ts> .. }[index], [Xn]` -- NEON load/store a SINGLE lane of `count` (1-4) structures
    /// (`count` consecutive registers from `rt_first`, wrapping mod 32). `element` is the lane size (`b`/`h`/`s`/
    /// `d`); `index` is the lane (packed into Q:S:size). Needs Advanced SIMD.
    VecLoadStoreSingleLane {
        load: bool,
        count: u8,
        element: Arm64VectorElement,
        index: u8,
        rt_first: Arm64FloatRegister,
        xn: Arm64GeneralPurposeRegister,
    },

    /// `LD<n>R { Vt.<arr> .. }, [Xn]` -- NEON load single structure and REPLICATE to all lanes of `count` (1-4)
    /// registers. Load-only. `arrangement` is the whole-vector form (`.8b`..`.2d`). Needs Advanced SIMD.
    VecLoadStoreReplicate {
        count: u8,
        arrangement: Arm64VectorArrangement,
        rt_first: Arm64FloatRegister,
        xn: Arm64GeneralPurposeRegister,
    },

    /// `LD<n>`/`ST<n> { Vt.<arr> .. }, [Xn], <inc>` -- the POST-INDEX multiple-structure form. `xm = None` is the
    /// immediate form (`[Xn], #<implicit>` -- the increment equals the bytes accessed); `xm = Some(Xm)` is the
    /// register form (`[Xn], Xm`). Otherwise identical to [`Self::VecLoadStoreMultiple`]. Needs Advanced SIMD.
    VecLoadStoreMultiplePostIndex {
        kind: Arm64VectorStructureKind,
        load: bool,
        arrangement: Arm64VectorArrangement,
        rt_first: Arm64FloatRegister,
        xn: Arm64GeneralPurposeRegister,
        xm: Option<Arm64GeneralPurposeRegister>,
    },

    /// `LD<n>`/`ST<n> { Vt.<ts> .. }[index], [Xn], <inc>` -- the POST-INDEX single-lane form (`xm = None` immediate
    /// / `Some(Xm)` register). Needs Advanced SIMD.
    VecLoadStoreSingleLanePostIndex {
        load: bool,
        count: u8,
        element: Arm64VectorElement,
        index: u8,
        rt_first: Arm64FloatRegister,
        xn: Arm64GeneralPurposeRegister,
        xm: Option<Arm64GeneralPurposeRegister>,
    },

    /// `LD<n>R { Vt.<arr> .. }, [Xn], <inc>` -- the POST-INDEX load-and-replicate form (`xm = None` immediate /
    /// `Some(Xm)` register). Load-only. Needs Advanced SIMD.
    VecLoadStoreReplicatePostIndex {
        count: u8,
        arrangement: Arm64VectorArrangement,
        rt_first: Arm64FloatRegister,
        xn: Arm64GeneralPurposeRegister,
        xm: Option<Arm64GeneralPurposeRegister>,
    },

    /// `TBL`/`TBX Vd.<arr>, { Vn.16b .. }, Vm.<arr>` -- NEON table lookup: each byte of `Vm` indexes into the
    /// `num_tables`-register table starting at `rn_first` (consecutive, wrapping mod 32, always `.16b`); `TBL`
    /// writes 0 for out-of-range indices, `TBX` leaves `Vd` unchanged. `arrangement` is `.8b` or `.16b` (the
    /// result/index width); `num_tables` is `1..=4`. Needs Advanced SIMD.
    VecTableLookup {
        tbx: bool,
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn_first: Arm64FloatRegister,
        num_tables: u8,
        rm: Arm64FloatRegister,
    },

    /// `DUP Vd.<arr>, <R>n` -- NEON duplicate (splat) a general-purpose register across every lane of the
    /// destination vector. The arrangement gives the lane size + `Q`; the source width follows it (`.d` -> Xn,
    /// else Wn). `.1d` is invalid. Needs Advanced SIMD.
    VecDupGeneral {
        arrangement: Arm64VectorArrangement,
        rd: Arm64FloatRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `UMOV <R>d, Vn.<ts>[index]` -- NEON move one vector lane into a general-purpose register, zero-extended.
    /// The destination width follows the element (`.b`/`.h`/`.s` -> Wd, `.d` -> Xd). Needs Advanced SIMD.
    VecUmov {
        element: Arm64VectorElement,
        index: u8,
        rd: Arm64GeneralPurposeRegister,
        vn: Arm64FloatRegister,
    },

    /// `SMOV <R>d, Vn.<ts>[index]` -- NEON move one vector lane into a general-purpose register, sign-extended
    /// to the explicit `width` (`W` <- `.b`/`.h`; `X` <- `.b`/`.h`/`.s`). Needs Advanced SIMD.
    VecSmov {
        width: Arm64RegisterWidth,
        element: Arm64VectorElement,
        index: u8,
        rd: Arm64GeneralPurposeRegister,
        vn: Arm64FloatRegister,
    },

    /// `INS Vd.<ts>[index], <R>n` -- NEON insert a general-purpose register into one lane of the destination
    /// vector (a 128-bit operation; the other lanes are preserved). Needs Advanced SIMD.
    VecInsGeneral {
        element: Arm64VectorElement,
        index: u8,
        vd: Arm64FloatRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `FMOV Sd, Wn` / `FMOV Dd, Xn` -- bitwise move (reinterpret) from a general-purpose register to an FP
    /// register: `W<->S` (32-bit) or `X<->D` (64-bit), selected by the single `size` (the GP width and FP
    /// precision are locked equal). Operands: `size`, `fd` (FP dest), `rn` (GP source).
    FmovGeneralToFp(
        /* size */ Arm64RegisterWidth,
        /* fd */ Arm64FloatRegister,
        /* rn */ Arm64GeneralPurposeRegister,
    ),
    /// `FMOV Wd, Sn` / `FMOV Xd, Dn` -- bitwise move (reinterpret) from an FP register to a general-purpose
    /// register. Operands: `size`, `rd` (GP dest), `fn` (FP source).
    FmovFpToGeneral(
        /* size */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* fn */ Arm64FloatRegister,
    ),

    /// `FMOV Wd, Hn` / `FMOV Xd, Hn` -- bitwise move (zero-extended reinterpret) from a half-precision `Hn` to a
    /// general-purpose register (FEAT_FP16). Unlike the S/D forms, the GP width (`width`) is independent of the FP
    /// precision (always half). Operands: `width` (`W`/`X`), `rd` (GP dest), `fn` (`Hn` source).
    FmovHalfToGeneral(
        /* width */ Arm64RegisterWidth,
        /* rd */ Arm64GeneralPurposeRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FMOV Hd, Wn` / `FMOV Hd, Xn` -- bitwise move from a general-purpose register to a half-precision `Hd`
    /// (FEAT_FP16). Operands: `width` (`W`/`X` source), `fd` (`Hd` dest), `rn` (GP source).
    FmovGeneralToHalf(
        /* width */ Arm64RegisterWidth,
        /* fd */ Arm64FloatRegister,
        /* rn */ Arm64GeneralPurposeRegister,
    ),
    /// `FMOV Xd, Vn.D[1]` -- bitwise move from the HIGH 64 bits of a 128-bit vector register to `Xd` (base FEAT_FP;
    /// the top-half transfer, `X`-only). Operands: `rd` (GP dest), `vn` (vector source, `.D[1]`).
    FmovTopHalfToGeneral(
        /* rd */ Arm64GeneralPurposeRegister,
        /* vn */ Arm64FloatRegister,
    ),
    /// `FMOV Vd.D[1], Xn` -- bitwise move from `Xn` into the HIGH 64 bits of a 128-bit vector register, preserving
    /// the low half (base FEAT_FP; `X`-only). Operands: `vd` (vector dest, `.D[1]`), `rn` (GP source).
    FmovGeneralToTopHalf(
        /* vd */ Arm64FloatRegister,
        /* rn */ Arm64GeneralPurposeRegister,
    ),

    /// `FMOV Sd, #imm` / `FMOV Dd, #imm` -- move an 8-bit-encoded FP immediate into `Fd`. `imm8` is the raw
    /// VFP/AdvSIMD modified-immediate field; obtain it from a value with [`crate::fp8_encode_single`] /
    /// [`crate::fp8_encode_double`] (which return `None` when the value is not representable). Operands:
    /// `precision`, `fd`, `imm8`.
    FMovImmediate(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* imm8 */ u8,
    ),

    /// `FMAX Sd|Dd, Sn|Dn, Sm|Dm` -- floating-point maximum (NaN-propagating). Operands as [`Self::FAdd`].
    FMax(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),
    /// `FMIN Sd|Dd, Sn|Dn, Sm|Dm` -- floating-point minimum (NaN-propagating). Operands as [`Self::FAdd`].
    FMin(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),
    /// `FMAXNM Sd|Dd, Sn|Dn, Sm|Dm` -- FP maximum-number (IEEE `maxNum`: returns the number operand if one is
    /// NaN). Operands as [`Self::FAdd`].
    FMaxnm(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),
    /// `FMINNM Sd|Dd, Sn|Dn, Sm|Dm` -- FP minimum-number. Operands as [`Self::FAdd`].
    FMinnm(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),
    /// `FNMUL Sd|Dd, Sn|Dn, Sm|Dm` -- negated floating-point multiply (`-(Fn x Fm)`). Operands as [`Self::FAdd`].
    FNmul(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
    ),

    /// `FRINTN Sd|Dd, Sn|Dn` -- round to integral, to nearest with ties to even. Operands
    /// as [`Self::FNeg`].
    FRintN(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FRINTP Sd|Dd, Sn|Dn` -- round to integral, toward +inf. Operands as [`Self::FNeg`].
    FRintP(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FRINTM Sd|Dd, Sn|Dn` -- round to integral, toward -inf. Operands as [`Self::FNeg`].
    FRintM(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FRINTZ Sd|Dd, Sn|Dn` -- round to integral, toward zero. Operands as [`Self::FNeg`].
    FRintZ(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FRINTA Sd|Dd, Sn|Dn` -- round to integral, to nearest with ties away from zero. Operands as [`Self::FNeg`].
    FRintA(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FRINTX Sd|Dd, Sn|Dn` -- round to integral using the current rounding mode, signalling Inexact (the C
    /// `rint`/`nearbyint` lowering). Operands as [`Self::FNeg`].
    FRintX(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),
    /// `FRINTI Sd|Dd, Sn|Dn` -- round to integral using the current rounding mode (no Inexact signal). Operands
    /// as [`Self::FNeg`].
    FRintI(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
    ),

    /// `FRINT32X`/`FRINT32Z`/`FRINT64X`/`FRINT64Z Sd|Dd, Sn|Dn` -- scalar round to a 32/64-bit signed integral
    /// value (FEAT_FRINTTS); single/double only. See [`Arm64ScalarFrintTsOp`].
    FRoundIntScalar {
        op: Arm64ScalarFrintTsOp,
        precision: Arm64FloatPrecision,
        fd: Arm64FloatRegister,
        fn_: Arm64FloatRegister,
    },

    /// `FMADD Sd|Dd, Sn|Dn, Sm|Dm, Sa|Da` -- fused multiply-add: `Fd = Fa + Fn x Fm`. Operands: `precision`,
    /// `fd`, `fn`, `fm`, `fa`.
    FMadd(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
        /* fa */ Arm64FloatRegister,
    ),
    /// `FMSUB Sd|Dd, Sn|Dn, Sm|Dm, Sa|Da` -- fused multiply-subtract: `Fd = Fa - Fn x Fm`. Operands as [`Self::FMadd`].
    FMsub(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
        /* fa */ Arm64FloatRegister,
    ),
    /// `FNMADD Sd|Dd, Sn|Dn, Sm|Dm, Sa|Da` -- negated fused multiply-add: `Fd = -Fa - Fn x Fm`. Operands as [`Self::FMadd`].
    FNmadd(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
        /* fa */ Arm64FloatRegister,
    ),
    /// `FNMSUB Sd|Dd, Sn|Dn, Sm|Dm, Sa|Da` -- negated fused multiply-subtract: `Fd = -Fa + Fn x Fm`. Operands as [`Self::FMadd`].
    FNmsub(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
        /* fa */ Arm64FloatRegister,
    ),

    /// `FCSEL Sd|Dd, Sn|Dn, Sm|Dm, cond` -- floating-point conditional select: `Fd = cond ? Fn : Fm`. Operands:
    /// `precision`, `fd`, `fn`, `fm`, `cond`.
    FCsel(
        /* precision */ Arm64FloatPrecision,
        /* fd */ Arm64FloatRegister,
        /* fn */ Arm64FloatRegister,
        /* fm */ Arm64FloatRegister,
        /* cond */ Arm64Condition,
    ),

    // ---- additional integer data-processing (compiler-typical scalar gaps) ----
    /// `RBIT Wd|Xd, Wn|Xn` -- reverse the bit order. Operands: `width`, `xd`, `xn`.
    Rbit(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),
    /// `REV16 Wd|Xd, Wn|Xn` -- reverse bytes within each 16-bit halfword. Operands as [`Self::Rbit`].
    Rev16(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),
    /// `REV Wd|Xd, Wn|Xn` -- reverse all bytes (byte-swap the whole register). Operands as [`Self::Rbit`]. (The
    /// architectural opcode differs by width -- `W` uses 000010, `X` uses 000011 -- handled in encode/decode.)
    Rev(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),
    /// `REV32 Xd, Xn` -- reverse bytes within each 32-bit word of a 64-bit register (X-only). Operands: `xd`, `xn`.
    Rev32(
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),
    /// `CLZ Wd|Xd, Wn|Xn` -- count leading zeros. Operands as [`Self::Rbit`].
    Clz(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),
    /// `CLS Wd|Xd, Wn|Xn` -- count leading sign bits. Operands as [`Self::Rbit`].
    Cls(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
    ),

    /// `ABS`/`CNT`/`CTZ Wd|Xd, Wn|Xn` -- FEAT_CSSC scalar unary data-processing (an extra opcode in the one-source
    /// group). See [`Arm64CsscUnaryOp`].
    CsscUnary {
        op: Arm64CsscUnaryOp,
        width: Arm64RegisterWidth,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `SMAX`/`SMIN`/`UMAX`/`UMIN Wd|Xd, Wn|Xn, Wm|Xm` -- FEAT_CSSC integer min/max, register form (an extra opcode in
    /// the two-source group). See [`Arm64CsscMinMaxOp`].
    CsscMinMaxReg {
        op: Arm64CsscMinMaxOp,
        width: Arm64RegisterWidth,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `SMAX`/`SMIN`/`UMAX`/`UMIN Wd|Xd, Wn|Xn, #imm` -- FEAT_CSSC integer min/max, 8-bit immediate form (signed for
    /// `SMAX`/`SMIN`, unsigned for `UMAX`/`UMIN`). base `0x_1C0_0000`: opcode`[19:18]`, imm8`[17:10]`. See
    /// [`Arm64CsscMinMaxOp`].
    CsscMinMaxImm {
        op: Arm64CsscMinMaxOp,
        width: Arm64RegisterWidth,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        imm: i32,
    },

    /// `ADC Wd|Xd, Wn|Xn, Wm|Xm` -- add with carry (`Rd = Rn + Rm + C`). Multi-word arithmetic. Operands:
    /// `width`, `xd`, `xn`, `xm`.
    Adc(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
    ),
    /// `SBC Wd|Xd, Wn|Xn, Wm|Xm` -- subtract with carry (`Rd = Rn - Rm - !C`). Operands as [`Self::Adc`].
    Sbc(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
    ),
    /// `ADCS Wd|Xd, Wn|Xn, Wm|Xm` -- add with carry, setting NZCV. Operands as [`Self::Adc`].
    Adcs(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
    ),
    /// `SBCS Wd|Xd, Wn|Xn, Wm|Xm` -- subtract with carry, setting NZCV. Operands as [`Self::Adc`].
    Sbcs(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
    ),

    /// `EXTR Wd|Xd, Wn|Xn, Wm|Xm, #lsb` -- extract a register-pair-wide field: `Rd = (Rn:Rm) >> lsb`. With
    /// `Rn == Rm` this is `ROR` by `lsb`. `lsb` is 0..=31 for `W` / 0..=63 for `X`. Operands: `width`, `xd`,
    /// `xn`, `xm`, `lsb`.
    Extr(
        /* width */ Arm64RegisterWidth,
        /* xd */ Arm64GeneralPurposeRegister,
        /* xn */ Arm64GeneralPurposeRegister,
        /* xm */ Arm64GeneralPurposeRegister,
        /* lsb */ u8,
    ),

    // ---- SVE (Scalable Vector Extension): the Z (scalable vector) + P (predicate) register files ----
    /// `PTRUE{S} Pd.<T>{, <pattern>}` -- initialise a predicate to all-true under a size pattern (FEAT_SVE). The
    /// `S` form (`sets_flags`) also sets the NZCV condition flags. `pattern` is the 5-bit pattern field (`31` =
    /// `ALL`, the default that prints with no pattern). `size` selects the element width.
    SvePtrue {
        sets_flags: bool,
        size: Arm64VectorElement,
        pattern: u8,
        pd: Arm64PredicateRegister,
    },

    /// `<op> Zd.<T>, Zn.<T>, Zm.<T>` -- SVE unpredicated integer add/subtract (FEAT_SVE). See
    /// [`Arm64SveIntBinUnpredOp`].
    SveIntBinaryUnpredicated {
        op: Arm64SveIntBinUnpredOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Pd.<T>, <R><n>, <R><m>` -- SVE while-comparison loop predicate (FEAT_SVE). `compare_64` selects the
    /// 64-bit (`Xn`/`Xm`) vs 32-bit (`Wn`/`Wm`) scalar comparison operands. See [`Arm64SveWhileOp`].
    SveWhile {
        op: Arm64SveWhileOp,
        size: Arm64VectorElement,
        compare_64: bool,
        pd: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>` -- SVE predicated integer binary (destructive; FEAT_SVE). `zdn` is
    /// both destination and first source; `pg` (P0..P7) governs active elements (merging). See
    /// [`Arm64SvePredIntBinOp`].
    SveIntBinaryPredicated {
        op: Arm64SvePredIntBinOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `LD1{S}{B,H,W,D} {Zt.<T>}, Pg/Z, [Xn|SP{, #imm, MUL VL}]` -- SVE contiguous predicated zeroing load
    /// (FEAT_SVE). `imm` is the `-8..=7` element-count offset (scaled by the vector length). See
    /// [`Arm64SveContiguousLoadType`].
    SveContiguousLoad {
        load: Arm64SveContiguousLoadType,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm4: i8,
    },

    /// `ST1{B,H,W,D} {Zt.<T>}, Pg, [Xn|SP{, #imm, MUL VL}]` -- SVE contiguous predicated store (FEAT_SVE).
    /// `msize` is the access size (`st1b`/`h`/`w`/`d`); `esize` (>= `msize`) is the source element size; `imm` is
    /// the `-8..=7` element-count offset.
    SveContiguousStore {
        msize: Arm64VectorElement,
        esize: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm4: i8,
    },

    /// `LD1{S}{B,H,W,D} {Zt.<T>}, Pg/Z, [Xn|SP, Xm{, LSL #amount}]` -- SVE contiguous predicated load with a scalar
    /// base and a scalar index register (FEAT_SVE). The index `Xm` is scaled by the access size (`LSL #log2(access)`,
    /// printed only when it shifts). `Xm` must not be `XZR`. See [`Arm64SveContiguousLoadType`].
    SveContiguousLoadScalar {
        load: Arm64SveContiguousLoadType,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `ST1{B,H,W,D} {Zt.<T>}, Pg, [Xn|SP, Xm{, LSL #amount}]` -- SVE contiguous predicated store with a scalar base
    /// and a scalar index register (FEAT_SVE). `msize` is the access size, `esize` (>= `msize`) the source element
    /// size; the index is scaled by the access size. `Xm` must not be `XZR`.
    SveContiguousStoreScalar {
        msize: Arm64VectorElement,
        esize: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LD1{S}{B,H,W,D} { Zt.<T> }, Pg/Z, [Zn.<T>{, #imm}]` -- SVE gather load with a vector base and a scalar
    /// immediate offset (FEAT_SVE). `element` is the lane size `.s` or `.d` (also the base-vector size); `msz` is
    /// the per-lane access size (`<=` the element); `signed` sign-extends. `imm5` is the unscaled `0..=31`
    /// immediate (displayed offset = `imm5 * access_size_bytes`). A full-width access (`msz == element`) is always
    /// unsigned.
    SveGatherLoadVectorImm {
        msz: Arm64VectorElement,
        element: Arm64VectorElement,
        signed: bool,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        imm5: u8,
    },

    /// `ST1{B,H,W,D} { Zt.<T> }, Pg, [Zn.<T>{, #imm}]` -- SVE scatter store with a vector base and a scalar immediate
    /// offset (FEAT_SVE). `element` is the base/data lane size `.s` or `.d`; `msz` is the per-lane access size
    /// (`<=` the element). `imm5` is the unscaled `0..=31` immediate (displayed offset = `imm5 * access_size_bytes`).
    SveScatterStoreVectorImm {
        msz: Arm64VectorElement,
        element: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        imm5: u8,
    },

    /// `ST1{B,H,W,D} { Zt.<T> }, Pg, [Xn|SP, Zm.<T>, <mode>{ #amount}]` -- SVE scatter store with a scalar base and a
    /// vector offset (FEAT_SVE). `element` is the offset/data lane size `.s` or `.d`; `msz` is the per-lane access size
    /// (`<=` the element). `mode` extends the offset (`LSL` needs a `.d` offset); `scaled` applies the access-size
    /// shift (`log2(access)`).
    SveScatterStoreScalarVector {
        msz: Arm64VectorElement,
        element: Arm64VectorElement,
        mode: Arm64SveOffsetMode,
        scaled: bool,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `LD1{S}{B,H,W,D}`/`LDFF1{S}{B,H,W,D} { Zt.<T> }, Pg/Z, [Xn|SP, Zm.<T>, <mode>{ #amount}]` -- SVE gather load
    /// with a scalar base and a vector offset (FEAT_SVE). `element` is the offset/data lane size `.s` or `.d`; `msz`
    /// is the per-lane access size; `signed` picks the sign-extending `LD1S*` (requires `msz < element`); `first_fault`
    /// picks `LDFF1`. `mode` extends the offset (`LSL` needs `.d`); `scaled` applies the access-size shift (not for `B`).
    SveGatherLoadScalarVector {
        msz: Arm64VectorElement,
        element: Arm64VectorElement,
        signed: bool,
        first_fault: bool,
        mode: Arm64SveOffsetMode,
        scaled: bool,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `LDNT1{S}{B,H,W,D} { Zt.<T> }, Pg/Z, [Zn.<T>, Xm]` -- SVE2 gather non-temporal load with a vector base and a
    /// scalar offset (FEAT_SVE2). `element` is the base-vector lane (`.s`/`.d`); `msz` is the access size; `signed`
    /// picks the sign-extending `LDNT1S*` (requires `msz < element`).
    Sve2GatherNonTemporalLoad {
        msz: Arm64VectorElement,
        element: Arm64VectorElement,
        signed: bool,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `STNT1{B,H,W,D} { Zt.<T> }, Pg, [Zn.<T>, Xm]` -- SVE2 scatter non-temporal store with a vector base and a
    /// scalar offset (FEAT_SVE2). `element` is the base-vector lane (`.s`/`.d`); `msz` is the access size (`<= element`).
    Sve2ScatterNonTemporalStore {
        msz: Arm64VectorElement,
        element: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LD1Q { Zt.Q }, Pg/Z, [Zn.D{, Xm}]` / `ST1Q { Zt.Q }, Pg, [Zn.D{, Xm}]` -- SVE2.1 quadword gather load / scatter
    /// store with a `.d` vector base and an optional scalar offset (`Xm`, `XZR` = none) (FEAT_SVE2p1). `store` picks
    /// `ST1Q`. load base `0xC400_A000` / store base `0xE420_2000`: Rm`[20:16]`, Pg`[12:10]`, Zn`[9:5]`, Zt`[4:0]`.
    SveQuadwordGatherScatter {
        store: bool,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LD2Q`/`LD3Q`/`LD4Q`/`ST*Q { Zt1.Q-... }, Pg{/Z}, [Xn|SP{, #imm, MUL VL}]` -- SVE2.1 structured quadword
    /// load/store, scalar base + scalar-immediate offset (FEAT_SVE2p1). `count` is `2..=4`, `imm` is the encoded signed
    /// `[19:16]` (the assembly `#` value is `imm * count`). load base `0xA410_E000` (count-1 at `[24:23]`) / store base
    /// `0xE400_0000` (count-1 at `[23:22]`): imm`[19:16]`, Pg`[12:10]`, Rn`[9:5]`, Zt-list`[4:0]`.
    SveStructuredQuadwordImm {
        store: bool,
        count: u8,
        pg: Arm64PredicateRegister,
        zt_base: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm: i8,
    },

    /// `LD2Q`/.../`ST*Q { Zt1.Q-... }, Pg{/Z}, [Xn|SP, Xm, LSL #4]` -- SVE2.1 structured quadword load/store, scalar base
    /// + scalar (shift-4) offset. load base `0xA420_8000` / store base `0xE420_0000`: Rm`[20:16]`, Pg`[12:10]`, Rn`[9:5]`, Zt`[4:0]`.
    SveStructuredQuadwordScalar {
        store: bool,
        count: u8,
        pg: Arm64PredicateRegister,
        zt_base: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `SMSTART`/`SMSTOP {<target>}` -- enter/exit SME streaming mode and/or enable the `ZA` array (FEAT_SME). An
    /// MSR to the `SVCR` PSTATE field. `start` picks `SMSTART`; `target` is `SM`/`ZA`/both. See [`Arm64SmeStateTarget`].
    SmeStartStop {
        start: bool,
        target: Arm64SmeStateTarget,
    },

    /// `RDSVL Xd, #imm` -- read the streaming vector length (the SVE vector length in streaming mode), multiplied by
    /// `imm` (`-32..=31`), into `Xd` (FEAT_SME). Like `RDVL` but for the streaming VL.
    SmeReadStreamingVectorLength {
        rd: Arm64GeneralPurposeRegister,
        imm6: i8,
    },

    /// `ADDSVL Xd, Xn, #imm` -- add `imm` (`-32..=31`) times the streaming vector length (in bytes) to `Xn`, into `Xd`
    /// (FEAT_SME). `Xd`/`Xn` may be `SP`. Like `ADDVL` but for the streaming VL.
    SmeAddStreamingVectorLength {
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        imm6: i8,
    },

    /// `ADDSPL Xd, Xn, #imm` -- add `imm` (`-32..=31`) times the streaming predicate-register length (in bytes) to
    /// `Xn`, into `Xd` (FEAT_SME). `Xd`/`Xn` may be `SP`. Like `ADDPL` but for the streaming VL.
    SmeAddStreamingPredicateLength {
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        imm6: i8,
    },

    /// `FMOPA`/`FMOPS`/`BFMOPA`/`BFMOPS ZAda.<T>, Pn/M, Pm/M, Zn.<Tn>, Zm.<Tn>` -- SME floating-point outer product
    /// accumulate/subtract into a `ZA` tile (FEAT_SME; `.d` needs FEAT_SME_F64F64). `subtract` picks the `*MOPS`
    /// form; `za_tile` is the destination tile number (`0..=3` for `.s`, `0..=7` for `.d`). See [`Arm64SmeFpPrecision`].
    SmeFpOuterProduct {
        precision: Arm64SmeFpPrecision,
        subtract: bool,
        za_tile: u8,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMOPA`/`FMOPS`/`BFMOPA`/`BFMOPS ZAda.H, Pn/M, Pm/M, Zn.H, Zm.H` -- SME2 half-precision outer product
    /// accumulating into a **`.h`** ZA tile, distinct from the FEAT_SME `*MOPA Zda.S`. `bf16` selects BFloat16
    /// (`BFMOPA`, FEAT_SME_B16B16, `[21]=1`) vs IEEE FP16 (`FMOPA`, FEAT_SME_F16F16, `[21]=0`). base `0x8180_0008`
    /// (`[3]`=1 `.h` marker): bf16`[21]`, op(sub)`[4]`, Zm`[20:16]`, Pm`[15:13]`, Pn`[12:10]`, Zn`[9:5]`, tile`[0]` (`ZA0.H`/`ZA1.H`).
    Sme16BitOuterProductH {
        bf16: bool,
        subtract: bool,
        za_tile: u8,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SMOPA`/`UMOPA`/`SUMOPA`/`USMOPA` (and the `*MOPS` subtract forms) `ZAda.<T>, Pn/M, Pm/M, Zn.<Tn>, Zm.<Tn>` --
    /// SME integer outer product into a `ZA` tile (FEAT_SME; the `.d` form needs FEAT_SME_I16I64). `unsigned_first`/
    /// `unsigned_second` select each source's signedness (the mnemonic prefix), `subtract` picks the `*MOPS` form.
    /// `size` is the accumulator/tile element (`.s` from `.b` sources, `.d` from `.h`); `za_tile` is `0..=3`/`0..=7`.
    SmeIntOuterProduct {
        unsigned_first: bool,
        unsigned_second: bool,
        subtract: bool,
        size: Arm64VectorElement,
        za_tile: u8,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BMOPA`/`BMOPS ZAda.S, Pn/M, Pm/M, Zn.S, Zm.S` -- SME2 bitwise (population-count) outer product into a `.s` ZA
    /// tile (FEAT_SME2; `.s` only, `ZA0-3`). Shares the FP `*MOPA` frame but sets the `[3]=1` marker (base `0x8080_0008`,
    /// which `SME_FP_MOP_MASK`'s `[3]=0` excludes): `subtract` picks `BMOPS` at `[4]`, Zm`[20:16]`, Pm`[15:13]`, Pn`[12:10]`,
    /// Zn`[9:5]`, tile`[1:0]`.
    SmeBitwiseOuterProduct {
        subtract: bool,
        za_tile: u8,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMOPA ZAda.<T>, Pn/M, Pm/M, Zn.B, Zm.B` -- SME2 FP8 outer-product accumulate into a `ZA` tile from FP8 (`.b`)
    /// sources. There is NO `FMOPS` subtract form for FP8 (both oracles reject it). `single` selects the `.s` (FP32)
    /// accumulator (FEAT_SME_F8F32, `[3]=0`, tile `0..=3`) over the `.h` (FP16) accumulator (FEAT_SME_F8F16, `[3]=1`,
    /// tile `0..=1`). base `0x80A0_0000` (`[21]`=1 separates it from the FP32/FP16/BF16 outer products): Zm`[20:16]`,
    /// Pm`[15:13]`, Pn`[12:10]`, Zn`[9:5]`, `.h` marker`[3]`, tile`[1:0]` (`.s`) / `[0]` (`.h`).
    SmeFp8OuterProduct {
        single: bool,
        za_tile: u8,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> ZAda.<T>, {Zn1.<Ts>-Zn2.<Ts>}, Zm.<Ts>, Zk[index]` -- SME2 sparse (transposed) outer-product accumulate
    /// (FEAT_SME_TMOP and friends). Accumulates a sparse outer product of the 2-vector `Zn` list and the single `Zm`,
    /// with sparsity selected by `Zk[index]`, into the `ZA` tile `za_tile`. `op` ([`Arm64SmeTmopOp`]) carries the
    /// element types + destination size. base `0x8040_0000`: Zm`[20:16]`, Zk-z20`[11:10]`, Zn>>1`[9:6]`, index`[5:4]`,
    /// za_tile`[1:0]` (`.s`) / `[0]` (`.h`); op bits at `{[24],[21],[15],[3]}`. `zn` is the even list base (`z0`..`z30`);
    /// `zk` is `z20`..`z23`; `index` is `0..=3`.
    SmeTmop {
        op: Arm64SmeTmopOp,
        za_tile: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        zk: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `<op>4{A|S} ZAda.S, <Zn>, <Zm>` -- SME2 quarter-tile outer-product accumulate/subtract (FEAT_SME_MOP4). `kind`
    /// ([`Arm64SmeMop4Kind`]) selects the source element types/signedness; `subtract` the `S` (subtract) over `A`
    /// (accumulate) form. `Zn`/`Zm` are each either a single even vector or a 2-vector list: `zn`/`zm` carry the even
    /// base register (`Zn` in `z0`..`z14`, `Zm` in `z16`..`z30`) and `zn_list`/`zm_list` select the `{Zb, Zb+1}` form.
    /// `.s` accumulator only (tiles `za0`..`za3`). base `0x8000_0000`: op`[24]`/`[21]`/`[15]`, Zm-list`[20]`+(Zm-16)>>1`[19:17]`,
    /// Zn-list`[9]`+Zn>>1`[8:6]`, subtract`[4]`, za_tile`[1:0]`.
    SmeMop4 {
        kind: Arm64SmeMop4Kind,
        subtract: bool,
        za_tile: u8,
        zn: Arm64ScalableVectorRegister,
        zn_list: bool,
        zm: Arm64ScalableVectorRegister,
        zm_list: bool,
    },

    /// `<op>4{A|S} ZAda.D, <Zn>, <Zm>` -- SME2 quarter-tile outer product into a **`.d` (64-bit) `ZA` tile**: the
    /// `.h`-source integer i16->i64 forms (FEAT_SME_I16I64) and the `.d`-source f64 form (FEAT_SME_F64F64). A separate
    /// sub-encoding from [`Self::SmeMop4`] (`[23:22]`=11, `[3]`=1; integer sets `[29]`). Operands as `SmeMop4` (single even /
    /// 2-vector list). `.d` accumulator -> tiles `za0`..`za7`. base `0x80C0_0008`: int`[29]`, op`[24]`/`[21]`, Zm-list`[20]`+
    /// (Zm-16)>>1`[19:17]`, Zn-list`[9]`+Zn>>1`[8:6]`, subtract`[4]`, za_tile`[2:0]`.
    SmeMop4Double {
        kind: Arm64SmeMop4DoubleKind,
        subtract: bool,
        za_tile: u8,
        zn: Arm64ScalableVectorRegister,
        zn_list: bool,
        zm: Arm64ScalableVectorRegister,
        zm_list: bool,
    },

    /// `FMOP4{A|S} ZAda.H, <Zn.H>, <Zm.H>` / `BFMOP4{A|S} ZAda.H, <Zn.H>, <Zm.H>` -- SME2 quarter-tile FP outer product
    /// into a **`.h` (16-bit) `ZA` tile**: f16 (`bf16 = false`, FEAT_SME_F16F16) or bf16 (`bf16 = true`, FEAT_SME_B16B16).
    /// The `[3]=1` (`.h` tile) sibling of the FP [`Self::SmeMop4`] forms. Operands as `SmeMop4`; tiles `za0`/`za1` only.
    /// base `0x8100_0008` (`[24]`=1, `[3]`=1): bf16`[21]`, Zm-list`[20]`+(Zm-16)>>1`[19:17]`, Zn-list`[9]`+Zn>>1`[8:6]`, subtract`[4]`,
    /// za_tile`[0]`.
    SmeMop4Half {
        bf16: bool,
        subtract: bool,
        za_tile: u8,
        zn: Arm64ScalableVectorRegister,
        zn_list: bool,
        zm: Arm64ScalableVectorRegister,
        zm_list: bool,
    },

    /// `MOVA Zd.<T>, Pg/M, ZA<t><H|V>.<T>[Wv, off]` / `MOVA ZA<t><H|V>.<T>[Wv, off], Pg/M, Zn.<T>` -- SME move between
    /// an SVE vector and a horizontal/vertical `ZA` tile slice (FEAT_SME; disassembles as the `MOV` alias).
    /// `to_vector` reads the tile slice into `z`; else it writes `z` into the slice. `vertical` picks the `V` slice;
    /// `za_tile`/`slice_offset` index within the `size` element's tile grid; `slice_reg` is `0..=3` for `W12..W15`.
    SmeMova {
        to_vector: bool,
        size: Arm64VectorElement,
        vertical: bool,
        za_tile: u8,
        slice_reg: u8,
        slice_offset: u8,
        pg: Arm64PredicateRegister,
        z: Arm64ScalableVectorRegister,
    },

    /// `LD1{B,H,W,D,Q}`/`ST1{B,H,W,D,Q} { ZA<t><H|V>.<T>[Wv, off] }, Pg{/Z}, [Xn|SP{, Xm{, LSL #amount}}]` -- SME
    /// load/store a horizontal/vertical `ZA` tile slice from/to memory (FEAT_SME). `store` picks `ST1`; `size` is the
    /// access/tile element; the slice is addressed by `za_tile`/`slice_offset`/`slice_reg` (`W12..W15`); the memory
    /// index `Xm` (default `XZR`) is scaled by the access size. See [`Arm64SmeTileSize`].
    SmeTileLoadStore {
        store: bool,
        size: Arm64SmeTileSize,
        vertical: bool,
        za_tile: u8,
        slice_reg: u8,
        slice_offset: u8,
        pg: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `ZERO { <tiles> }` -- SME zero a set of `ZA` tiles (FEAT_SME). `mask` is the raw 8-bit tile-select immediate
    /// (one bit per `.d`-granule slot); `0xFF` is the whole array (`{za}`), `0x00` is `{}`. The disassembly renders
    /// the minimal tile list (greedily `.h`/`.s`/`.d`).
    SmeZero { mask: u8 },

    /// `ZERO { ZT0 }` -- SME2 zero the 512-bit `ZT0` lookup-table register (FEAT_SME2). A fixed-encoding instruction
    /// (`0xC048_0001`) with no operands.
    SmeZeroZt0,

    /// `ADDHA`/`ADDVA ZAda.<T>, Pn/M, Pm/M, Zn.<T>` -- SME add each horizontal (`ADDHA`) or vertical (`ADDVA`) `Zn`
    /// element-group into the `ZA` tile (FEAT_SME; the `.d` form needs FEAT_SME_I16I64). `vertical` picks `ADDVA`;
    /// `size` is the tile element (`.s`/`.d`); `za_tile` is `0..=3`/`0..=7`.
    SmeAddHorizVert {
        vertical: bool,
        size: Arm64VectorElement,
        za_tile: u8,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `LDR ZA[Wv, off], [Xn|SP{, #off, MUL VL}]` / `STR ZA[Wv, off], ...` -- SME load/store one `ZA` array vector
    /// (an unpredicated spill/fill; FEAT_SME). `store` picks `STR`; `slice_reg` is `0..=3` for `W12..W15`; `offset`
    /// (`0..=15`) is both the ZA array-vector select offset and the `MUL VL` memory offset.
    SmeArrayLoadStore {
        store: bool,
        slice_reg: u8,
        offset: u8,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `CPYF<stage>{<opts>} [Xd]!, [Xs]!, Xn!` / `CPY<stage>{<opts>} ...` -- FEAT_MOPS memory copy (one of the
    /// prologue/main/epilogue triple). `forward` picks the forward-only `CPYF*`; `read_nt`/`write_nt` apply the
    /// non-temporal hint to the source/destination accesses. `rd`/`rs` are the dest/source pointers (writeback),
    /// `rn` the byte count (writeback). See [`Arm64MopsStage`].
    MopsCopy {
        forward: bool,
        stage: Arm64MopsStage,
        read_nt: bool,
        write_nt: bool,
        read_unpriv: bool,
        write_unpriv: bool,
        rd: Arm64GeneralPurposeRegister,
        rs: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `ZIP`/`UZP { Zd.<T>-Zd+1.<T> }, Zn.<T>, Zm.<T>` -- SME2 multi-vector (two-register) interleave/de-interleave of
    /// `Zn`/`Zm` into a `Zd` register pair (FEAT_SME2). `uzp` picks the de-interleave; `zd_base` is the even first
    /// register of the destination pair. See [`Arm64SmeStateTarget`]-style multi-vector lists.
    Sme2MultiVecZipUzp {
        uzp: bool,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SCLAMP`/`UCLAMP`/`FCLAMP { Zd.<T>-Zd+1.<T> }, Zn.<T>, Zm.<T>` -- SME2 multi-vector (two-register) clamp of the
    /// `Zd` pair to `[Zn, Zm]` (FEAT_SME2). `kind` picks signed/unsigned/FP; `zd_base` is the even pair base. The FP
    /// form excludes `.b`. See [`Arm64Sme2ClampKind`].
    Sme2MultiVecClamp {
        kind: Arm64Sme2ClampKind,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SCLAMP`/`UCLAMP`/`FCLAMP`/`BFCLAMP { Zd.<T> x4 }, Zn.<T>, Zm.<T>` -- the **four-vector** (vgx4) SME2 clamp
    /// (FEAT_SME2; `BFCLAMP` is FEAT_SME_B16B16). Like [`Self::Sme2MultiVecClamp`] but the destination is a
    /// multiple-of-4 quad list and the `[11]` four-register marker is set. `zd_base` must be a multiple of 4.
    Sme2Vgx4Clamp {
        kind: Arm64Sme2ClampKind,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SMIN`/`SMAX`/`UMIN`/`UMAX`/`FMIN`/`FMAX { Zdn.<T>-Zdn+1.<T> }, { Zdn.<T>-Zdn+1.<T> }, Zm.<T>` -- SME2
    /// multi-vector (two-register) min/max by a single vector, destructive (the destination pair is also the first
    /// source) (FEAT_SME2). `op` picks signed/unsigned integer or FP min/max; `zd_base` is the even pair base; `zm`
    /// the single source vector. The FP forms exclude `.b`. See [`Arm64Sme2MinMaxOp`].
    Sme2MultiVecMinMax {
        op: Arm64Sme2MinMaxOp,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SMIN`/`SMAX`/`UMIN`/`UMAX`/`FMIN`/`FMAX { Zdn.<T>-Zdn+1.<T> }, { Zdn.<T>-Zdn+1.<T> }, { Zm.<T>-Zm+1.<T> }` --
    /// SME2 multi-vector (two-register) min/max by a two-register source list, destructive (FEAT_SME2). The same op
    /// scheme as the by-single-vector form but with the multi-vector marker `[12]=1` and `zm_base` an even source pair
    /// base. The FP forms exclude `.b`. See [`Arm64Sme2MinMaxOp`].
    Sme2MultiVecMinMaxMulti {
        op: Arm64Sme2MinMaxOp,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `SRSHL`/`URSHL`/`SQDMULH { Zdn.<T>-Zdn+1.<T> }, { Zdn.<T>-Zdn+1.<T> }, Zm.<T>` -- SME2 multi-vector
    /// (two-register) rounding-shift / sat-doubling-multiply by a single vector, destructive (FEAT_SME2). `op` picks
    /// the operation; `zd_base` is the even pair base; `zm` is restricted to `Z0..Z15`. See [`Arm64Sme2ShiftMulOp`].
    Sme2MultiVecShiftMul {
        op: Arm64Sme2ShiftMulOp,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SRSHL`/`URSHL`/`SQDMULH { Zdn.<T>-Zdn+1.<T> }, { Zdn.<T>-Zdn+1.<T> }, { Zm.<T>-Zm+1.<T> }` -- the two-register
    /// source-list form of [`Self::Sme2MultiVecShiftMul`] (sets the `[12]` multi-vector marker; `zm_base` is an even
    /// source pair) (FEAT_SME2). See [`Arm64Sme2ShiftMulOp`].
    Sme2MultiVecShiftMulMulti {
        op: Arm64Sme2ShiftMulOp,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `FRINTN`/`FRINTM`/`FRINTP`/`FRINTA`/`FCVTZS`/`FCVTZU`/`SCVTF`/`UCVTF { Zd.s-Zd+1.s }, { Zn.s-Zn+1.s }` -- SME2
    /// multi-vector (two-register) single-precision unary round/convert (FEAT_SME2). Non-destructive: `zd_base` and
    /// `zn_base` are independent even pair bases. `.s` only in this model. See [`Arm64Sme2UnaryOp`].
    Sme2MultiVecUnary {
        op: Arm64Sme2UnaryOp,
        zd_base: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `SMIN`/`SMAX`/`UMIN`/`UMAX`/`FMIN`/`FMAX { Zdn.<T>-Zdn+3.<T> }, { Zdn.<T>-Zdn+3.<T> }, Zm.<T>` -- the
    /// four-register (vgx4) form of [`Self::Sme2MultiVecMinMax`] (sets the `[11]` list-size bit; the destination
    /// quad base `zd_base` is a multiple of 4 encoded `Zd>>2` at `[4:2]`; `zm` is `Z0..Z15`) (FEAT_SME2).
    Sme2Vgx4MinMax {
        op: Arm64Sme2MinMaxOp,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SMIN`/... `{ Zdn-quad }, { Zdn-quad }, { Zm.<T>-Zm+3.<T> }` -- the four-register (vgx4) min/max by a
    /// four-register source list (sets both the `[12]` multi-vector and `[11]` list-size bits; `zm_base` is a
    /// multiple of 4) (FEAT_SME2). See [`Arm64Sme2MinMaxOp`].
    Sme2Vgx4MinMaxMulti {
        op: Arm64Sme2MinMaxOp,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `FMUL { Zd-list }, { Zn-list }, Zm.<T>` or `..., { Zm-list }` -- SME2.2 multi-vector floating-point multiply
    /// (FEAT_SME2p2; `.h`/`.s`/`.d`). NON-destructive: the destination list is independent of the sources. `vgx4`
    /// picks the four-register list (else two); `multi_src` picks the `{ Zm-list }` form (else a single `Zm` in
    /// `Z0..Z15`). Bases `0xC120_E800`/`E400` (vgx2 single/multi) and `0xC121_E800`/`E400` (vgx4); `zm` is the single
    /// register or the source-list base.
    Sme2MultiVecFmul {
        size: Arm64VectorElement,
        vgx4: bool,
        multi_src: bool,
        zd_base: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SRSHL`/`URSHL`/`SQDMULH { Zdn-quad }, { Zdn-quad }, Zm.<T>` -- the four-register (vgx4) form of
    /// [`Self::Sme2MultiVecShiftMul`] (sets the `[11]` list-size bit; dest quad base `Zd>>2` at `[4:2]`; `zm` is
    /// `Z0..Z15`) (FEAT_SME2). See [`Arm64Sme2ShiftMulOp`].
    Sme2Vgx4ShiftMul {
        op: Arm64Sme2ShiftMulOp,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SRSHL`/`URSHL`/`SQDMULH { Zdn-quad }, { Zdn-quad }, { Zm-quad }` -- the four-register (vgx4) shift/multiply by a
    /// four-register source list (sets the `[12]` multi-vector and `[11]` list-size bits) (FEAT_SME2).
    Sme2Vgx4ShiftMulMulti {
        op: Arm64Sme2ShiftMulOp,
        size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `FRINT*`/`FCVTZS`/`FCVTZU`/`SCVTF`/`UCVTF { Zd.s-Zd+3.s }, { Zn.s-Zn+3.s }` -- the four-register (vgx4) form of
    /// [`Self::Sme2MultiVecUnary`] (sets the `[20]` list-size bit in the selector; both quad bases are multiples of 4)
    /// (FEAT_SME2; `.s` only in this model). See [`Arm64Sme2UnaryOp`].
    Sme2Vgx4Unary {
        op: Arm64Sme2UnaryOp,
        zd_base: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `ADD`/`SUB ZA.<T>[Wv, off{, vgx2|vgx4}], { Zn.<T>-... }` -- SME2 accumulate a two- or four-register `Zn` vector
    /// group INTO a ZA single-vector group (FEAT_SME2; the `.d` form needs FEAT_SME_I16I64). `sub` picks `SUB`; `size`
    /// is `.s` or `.d`; `four` selects the four-register (vgx4) group; `wv` selects the slice-index register `W8..W11`
    /// (`0..=3`); `off` is the `0..=7` slice offset; `zn_base` is the source list base (a multiple of 2 for vgx2 or 4
    /// for vgx4). This is the first of the SME2 ZA-vector-group target family.
    Sme2ZaAddSub {
        sub: bool,
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `FADD`/`FSUB ZA.<T>[Wv, off{, vgx2|vgx4}], { Zn.<T>-... }` -- SME2 floating-point accumulate a two- or
    /// four-register `Zn` vector group INTO a ZA single-vector group. Like [`Self::Sme2ZaAddSub`] but FP and with bit`[4]`=0:
    /// base `0xC1A0_1C00`, `.s` is base SME2, `.d` adds `[22]` (needs FEAT_SME_F64F64), `.h` adds `[18]` (FEAT_SME_F16F16);
    /// `sub` sets `[3]`, `four` sets `[16]`, `wv` is `[14:13]`, `zn_base>>1` is `[9:6]`, `off` is `[2:0]`.
    Sme2ZaFpAddSub {
        sub: bool,
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `BFADD`/`BFSUB ZA.H[Wv, off{, vgx2|vgx4}], { Zn.H-... }` -- SME2 BFloat16 accumulate a `Zn` group INTO a `.h`
    /// ZA single-vector group (FEAT_SME_B16B16). The `.h` FP form [`Self::Sme2ZaFpAddSub`] with the BF16 marker `[22]`=1:
    /// base `0xC1E4_1C00`, `sub` `[3]`, `four` `[16]`, `wv` `[14:13]`, `zn_base>>1` `[9:6]` (vgx2) / `>>2` `[9:7]` (vgx4), off `[2:0]`.
    Sme2ZaBfAddSub {
        sub: bool,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `BFMLA`/`BFMLS ZA.H[Wv, off{, vgx2|vgx4}], { Zn.H-... }, Zm.H` -- SME2 BF16 multiply-accumulate a `Zn` group by a
    /// SINGLE `Zm` (`z0..z15`) INTO a `.h` ZA group (FEAT_SME_B16B16). The `.h` FMLA-single form with the BF16 marker
    /// `[22]`=1: base `0xC160_1C00`, vgx4 `[20]`, Zm `[19:16]`, Wv-8 `[14:13]`, Zn>>1 `[9:6]`, `sub` `[3]`, off `[2:0]`.
    Sme2ZaBfmlaSingle {
        sub: bool,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BFMLA`/`BFMLS ZA.H[Wv, off{, vgx2|vgx4}], { Zn.H-... }, { Zm.H-... }` -- SME2 BF16 multiply-accumulate a `Zn`
    /// group by a same-size `Zm` group INTO a `.h` ZA group (FEAT_SME_B16B16). The `.h` FMLA-multi form (`[12:10]`=100,
    /// `[3]`=1) with the BF16 marker `[22]`=1: base `0xC1E0_1008`, Zm>>1 `[20:17]`, vgx4 `[16]`, Wv-8 `[14:13]`, Zn `[9:6]`,
    /// `sub` `[4]`, off `[2:0]`.
    Sme2ZaBfmlaMulti {
        sub: bool,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `BFMLA`/`BFMLS ZA.H[Wv, off{, vgx2|vgx4}], { Zn.H-... }, Zm.H[index]` -- SME2 BF16 multiply-accumulate a `Zn` group
    /// by an INDEXED `Zm` (`z0..z15`) element INTO a `.h` ZA group (FEAT_SME_B16B16). The `.h` FMLA-indexed form with the
    /// BF16 marker `[5]`=1: base `0xC110_1020`, Zm `[19:16]`, vgx4 `[15]`, Wv-8 `[14:13]`, index (0..=7) high `[11:10]` +
    /// low `[3]`, Zn `[9:6]`, `sub` `[4]`, off `[2:0]`.
    Sme2ZaBfmlaIndexed {
        sub: bool,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FMLA`/`FMLS ZA.<T>[Wv, off{, vgx2|vgx4}], { Zn.<T>-... }, Zm.<T>` -- SME2 multiply-accumulate a two- or
    /// four-register `Zn` vector group by a SINGLE vector `Zm` (`z0..z15`) INTO a ZA single-vector group (FEAT_SME2;
    /// the `.d` form needs FEAT_SME_F64F64). `sub` picks `FMLS`; `size` is `.s` or `.d`; `four` selects the
    /// four-register (vgx4) group; `wv` selects the slice register `W8..W11` (`0..=3`); `off` is the `0..=7` slice
    /// offset; `zn_base` is the source list base (a multiple of 2 for vgx2 / 4 for vgx4); `zm` is the single
    /// multiplier vector. base `0xC120_1800`: size`[22]`, vgx`[20]`, Zm`[19:16]`, Wv-8`[14:13]`, Zn>>1`[9:6]`, op`[3]`, off`[2:0]`.
    /// (The FP16 `.h` form is FEAT_SME_F16F16, a separate `[10]=1` encoding; `size = .h` selects it.)
    Sme2ZaFmlaSingle {
        sub: bool,
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMLA`/`FMLS ZA.<T>[Wv, off{, vgx2|vgx4}], { Zn.<T>-... }, { Zm.<T>-... }` -- SME2 multiply-accumulate a
    /// two- or four-register `Zn` group by a same-size `Zm` group INTO a ZA single-vector group (FEAT_SME2; `.d`
    /// needs FEAT_SME_F64F64). Like [`Self::Sme2ZaFmlaSingle`] but the multiplier is a vector list: base
    /// `0xC1A0_1800` (`[23]`=1 multi marker), Zm>>1`[20:17]`, vgx`[16]`; `zm_base` is the multiplier list base.
    Sme2ZaFmlaMulti {
        sub: bool,
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `FMLA`/`FMLS ZA.<T>[Wv, off{, vgx2|vgx4}], { Zn.<T>-... }, Zm.<T>[index]` -- SME2 multiply-accumulate a
    /// two-/four-register `Zn` group by an INDEXED element of a single `Zm` (`z0..z15`) INTO a ZA single-vector
    /// group (FEAT_SME2; `.h` = FEAT_SME_F16F16, `.d` = FEAT_SME_F64F64). Like [`Self::Sme2ZaFmlaSingle`] but the
    /// multiplier element is `Zm.<T>[index]`. The `.s`/`.d` base is `0xC150_0000` (`[23]`=`.d`, `[12]`=0, `[5]`=0,
    /// `[3]`=0): Zm`[19:16]`, vgx4`[15]`, index`[11:10]` (`.s` 0..=3 / `.d` 0..=1), op`[4]`. The `.h` base is
    /// `0xC110_1000` (`[12]`=1) with the index SPLIT: high two bits `[11:10]`, low bit `[3]` (`.h` 0..=7). GNU-verified.
    Sme2ZaFmlaIndexed {
        sub: bool,
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `SDOT`/`UDOT`/`USDOT`/`SUDOT ZA.<Td>[Wv, off{, vgx2|vgx4}], { Zn.<Ts>-... }, Zm.<Ts>` -- SME2 multi-vector
    /// dot-product of a two-/four-register `Zn` group with a SINGLE multiplier `Zm` (`z0..z15`) accumulated INTO a ZA
    /// single-vector group. `op` is the source/accumulator/sign shape (`.b`->`.s` 4-way, `.h`->`.s` 2-way, `.h`->`.d`
    /// 4-way; the `.d` forms need FEAT_SME_I16I64); `four` selects vgx4; `wv`=`W8..W11` (`0..=3`); `off`=`0..=7`;
    /// `zn_base` is the source list base (a multiple of 2/4); `zm` is the single multiplier. base `0xC120_1400`:
    /// op`[22]`/`[4]`/`[3]`, vgx`[20]`, Zm`[19:16]`, Wv-8`[14:13]`, Zn>>1`[9:6]`, off`[2:0]`. See [`Arm64Sme2ZaDotOp`].
    Sme2ZaDotSingle {
        op: Arm64Sme2ZaDotOp,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SDOT`/`UDOT`/`USDOT`/`SUDOT ZA.<Td>[Wv, off{, vgx2|vgx4}], { Zn.<Ts>-... }, { Zm.<Ts>-... }` -- like
    /// [`Self::Sme2ZaDotSingle`] but the multiplier is a same-size vector list: base `0xC1A0_1400` (`[23]`=1 multi
    /// marker), Zm>>1`[20:17]`, vgx`[16]`; `zm_base` is the multiplier list base.
    Sme2ZaDotMulti {
        op: Arm64Sme2ZaDotOp,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `SDOT`/`UDOT`/`USDOT`/`SUDOT ZA.<Td>[Wv, off{, vgx2|vgx4}], { Zn.<Ts>-... }, Zm.<Ts>[index]` -- SME2 multi-
    /// vector dot product of a `Zn` group with an INDEXED single `Zm` (`z0..z15`) accumulated INTO a ZA single-vector
    /// group (the `.d`-accumulator forms need FEAT_SME_I16I64). Like [`Self::Sme2ZaDotSingle`] but the multiplier is
    /// `Zm.<Ts>[index]` (index `0..=3` for a `.s` accumulator, `0..=1` for `.d`). The op is encoded per-form (see
    /// [`Arm64Sme2ZaDotOp::indexed_base`]); Zm`[19:16]`, vgx4`[15]`, Wv-8`[14:13]`, index`[11:10]`, Zn-list, off`[2:0]`.
    Sme2ZaDotIndexed {
        op: Arm64Sme2ZaDotOp,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `SVDOT`/`UVDOT`/`SUVDOT`/`USVDOT`/`BFVDOT`/`FVDOT ZA.<Td>[Wv, off{, vgx2|vgx4}], { Zn.<Ts>-... }, Zm.<Ts>[index]`
    /// -- SME2 vertical dot product, indexed, into a ZA single-vector group (FEAT_SME2; the `.d` forms need
    /// FEAT_SME_I16I64). base `0xC150_0000`: op `[23]`+`[15]`+`[5:3]` (see [`Arm64Sme2ZaVdotOp`]), Zm`[19:16]` (`z0..z15`),
    /// Wv-8`[14:13]`, index`[11:10]`, Zn-list (`>>1``[9:6]` vgx2 / `>>2``[9:7]` vgx4), off`[2:0]`.
    Sme2ZaVdot {
        op: Arm64Sme2ZaVdotOp,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `SMLAL`/`UMLAL`/.../`SMLALL`/... `ZA.<Td>[Wv, slice], Zn.<Ts>, Zm.<Ts>` -- SME2 integer multiply-long accumulate
    /// into a ZA slice group, single `Zn` x single `Zm`. `widen` picks `.h`->`.s` (2-way pair), `.b`->`.s` or `.h`->`.d`
    /// (4-way quad). base `0xC120_0400 | h_src<<22 | two_way<<11`: Zm`[20:16]`, Wv-8`[14:13]`, Zn`[9:5]`, op`[4:2]`, slice/step`[2:0]`.
    Sme2ZaMlalSingle {
        op: Arm64Sme2ZaMlalOp,
        widen: Arm64Sme2ZaMlalWiden,
        wv: u8,
        slice: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SMLAL`/... `ZA.<Td>[Wv, slice{, vgx2|vgx4}], { Zn.<Ts>-... }, Zm.<Ts>` -- like [`Self::Sme2ZaMlalSingle`] but the
    /// first source is a 2-/4-register `Zn` group and `Zm` is a single `z0..z15`. base `0xC120_0000 | h_src<<22 |
    /// two_way<<11`: Zm `[19:16]`, vgx4 `[20]`, Zn-list (`>>1` `[9:6]` / `>>2` `[9:7]`).
    Sme2ZaMlalMulti {
        op: Arm64Sme2ZaMlalOp,
        widen: Arm64Sme2ZaMlalWiden,
        four: bool,
        wv: u8,
        slice: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SMLAL`/... `ZA.<Td>[Wv, slice{, vgx2|vgx4}], { Zn.<Ts>-... }, { Zm.<Ts>-... }` -- like [`Self::Sme2ZaMlalMulti`]
    /// but `Zm` is also a vector group. base `0xC1A0_0000 | h_src<<22 | two_way<<11`: Zm>>1`[20:17]`, vgx4`[16]`, Zn-list.
    Sme2ZaMlalMultiZm {
        op: Arm64Sme2ZaMlalOp,
        widen: Arm64Sme2ZaMlalWiden,
        four: bool,
        wv: u8,
        slice: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `SMLAL`/... `ZA.<Td>[Wv, slice], Zn.<Ts>, Zm.<Ts>[index]` -- SME2 integer MLAL/MLALL indexed (single `Zn` x an
    /// indexed `Zm` `z0..z15`). The irregular base `0xC100_0000` places h_src at `[23]` and two_way at `[22]`+`[12]`;
    /// `Zm``[19:16]`, `Zn``[9:5]`; the element index is `[15]`+`[11:10]` (`.h`, `0..=7`) or `[15]`+`[12]`+`[11:10]`
    /// (`.b`, `0..=15`). `SUMLALL` exists only in this form. op`[4:2]`/slice as in the other forms.
    Sme2ZaMlalIndexed {
        op: Arm64Sme2ZaMlalOp,
        widen: Arm64Sme2ZaMlalWiden,
        wv: u8,
        slice: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FMLAL`/`FMLSL ZA.S[Wv, slice], Zn.H, Zm.H` -- SME2 FP16 multiply-long accumulate into a `.s` ZA slice pair
    /// (FEAT_SME_F16F16). Structurally the integer MLAL `.h`->`.s` form with `[22]=0`: base `0xC120_0C00`, `subtract`
    /// picks `FMLSL` at `[3]`; Zm`[19:16]` (`z0..z15`), Wv-8`[14:13]`, Zn`[9:5]`, slice/2`[1:0]`.
    Sme2ZaFmlalSingle {
        subtract: bool,
        wv: u8,
        slice: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMLAL`/`FMLSL ZA.S[Wv, slice{, vgx2|vgx4}], { Zn.H-... }, Zm.H` -- like [`Self::Sme2ZaFmlalSingle`] but `Zn` is a
    /// 2-/4-register group. base `0xC120_0800`: Zm`[19:16]`, vgx4`[20]`, Zn-list (`>>1``[9:6]` / `>>2``[9:7]`).
    Sme2ZaFmlalMulti {
        subtract: bool,
        four: bool,
        wv: u8,
        slice: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMLAL`/`FMLSL ZA.S[Wv, slice], Zn.H, Zm.H[index]` -- like [`Self::Sme2ZaFmlalSingle`] but `Zm` (`z0..z15`) is
    /// indexed (`0..=7`). base `0xC180_0000` (`[23]=1`, `[12]=1`): index at `[15]`+`[11:10]`.
    Sme2ZaFmlalIndexed {
        subtract: bool,
        wv: u8,
        slice: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FMLAL`/`FMLSL ZA.S[Wv, slice{, vgx2|vgx4}], { Zn.H-... }, { Zm.H-... }` -- like [`Self::Sme2ZaFmlalMulti`] but `Zm`
    /// is also a group. base `0xC1A0_0800`: Zm>>1`[20:17]`, vgx4`[16]`, Zn-list.
    Sme2ZaFmlalMultiZm {
        subtract: bool,
        four: bool,
        wv: u8,
        slice: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `FDOT`/`BFDOT ZA.S[Wv, off{, vgx2|vgx4}], { Zn.H-... }, Zm.H` -- SME2 FP16/BF16 2-way dot product of a `Zn` group
    /// by a single `Zm` (`z0..z15`) into a `.s` ZA single-vector group (FEAT_SME2). `bf16` picks `BFDOT` at `[4]`. base
    /// `0xC120_1000`: vgx4`[20]`, Zm`[19:16]`, Wv-8`[14:13]`, Zn-list (`>>1``[9:6]` / `>>2``[9:7]`), off`[2:0]`.
    Sme2ZaFdotSingle {
        bf16: bool,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FDOT`/`BFDOT ZA.S[Wv, off{, vgx2|vgx4}], { Zn.H-... }, { Zm.H-... }` -- like [`Self::Sme2ZaFdotSingle`] but `Zm` is
    /// a vector group. base `0xC1A0_1000`: Zm>>1`[20:17]`, vgx4`[16]`.
    Sme2ZaFdotMulti {
        bf16: bool,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `FDOT`/`BFDOT ZA.S[Wv, off{, vgx2|vgx4}], { Zn.H-... }, Zm.H[index]` -- like [`Self::Sme2ZaFdotSingle`] but `Zm`
    /// (`z0..z15`) is indexed (`0..=3`). base `0xC150_1008` (`[22]=1`,`[12]=1`,`[3]=1`): vgx4`[15]`, index`[11:10]`.
    Sme2ZaFdotIndexed {
        bf16: bool,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FDOT ZA.<T>[Wv, off{, vgx2|vgx4}], { Zn.B-... }, Zm.B` -- SME2 FP8 floating-point dot product of a two-/four-
    /// register `Zn` group (FP8 `.b` sources) with a SINGLE multiplier `Zm` (`z0..z15`) accumulated INTO a ZA
    /// single-vector group. `size` is the accumulator element (`.h` = FEAT_SME_F8F16, `.s` = FEAT_SME_F8F32); `four`
    /// selects vgx4; `wv`=`W8..W11` (`0..=3`); `off`=`0..=7`; `zn_base` is the source list base (a multiple of 2/4).
    /// base `0xC120_1008` (`.h`): `.s` marker`[4]`, vgx`[20]`, Zm`[19:16]`, Wv-8`[14:13]`, Zn>>1`[9:6]`, off`[2:0]`; `[12:10]`=100,
    /// `[3]`=1, `[5]`=0 fixed.
    Sme2ZaFp8DotSingle {
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FDOT ZA.<T>[Wv, off{, vgx2|vgx4}], { Zn.B-... }, { Zm.B-... }` -- like [`Self::Sme2ZaFp8DotSingle`] but the
    /// multiplier is a same-size FP8 vector list. base `0xC1A0_1020` (`[23]`=1 multi marker, `[5]`=1, `[3]`=0): Zm>>1`[20:17]`,
    /// vgx`[16]`; `zm_base` is the multiplier list base.
    Sme2ZaFp8DotMulti {
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `FDOT ZA.<T>[Wv, off{, vgx2|vgx4}], { Zn.B-... }, Zm.B[index]` -- SME2 FP8 FDOT into ZA by an indexed element of
    /// `Zm` (`z0..z7`). `size` is the accumulator (`.h` = FEAT_SME_F8F16, `index` 0..=7; `.s` = FEAT_SME_F8F32, `index`
    /// 0..=3); `four` selects vgx4. The vgx2 and vgx4 forms are DISTINCT frames: vgx2 packs `Zn>>1` at `[9:6]`, vgx4 packs
    /// `Zn>>2` at `[9:7]`. Bases: `.h` vgx2 `0xC1D00020` / vgx4 `0xC1109040`; `.s` vgx2 `0xC1500038` / vgx4 `0xC1508008`.
    /// index = bit11:bit10:bit3 (`.h`) / bit11:bit10 (`.s`); Zm`[18:16]`, Wv-8`[14:13]`, off`[2:0]`.
    Sme2ZaFp8DotIndexed {
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FVDOT ZA.H[Wv, off{, vgx2}], { Zn.B-Zn+1.B }, Zm.B[index]` -- SME2 FP8 vertical dot product into a `.h` ZA group
    /// (FEAT_SME_F8F16). Like [`Self::Sme2ZaFp8DotIndexed`] `.h` vgx2 but vertical (`[12]=1`); `.h`/vgx2 ONLY. `index`
    /// 0..=7; `zm` is `z0..z7`. base `0xC1D0_1020`: Zm`[18:16]`, Wv-8`[14:13]`, Zn>>1`[9:6]`, off`[2:0]`, index = bit11:bit10:bit3.
    Sme2ZaFp8VerticalDot {
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FVDOTB`/`FVDOTT ZA.S[Wv, off{, vgx4}], { Zn.B-Zn+1.B }, Zm.B[index]` -- SME2 FP8 vertical dot product (bottom/top)
    /// into a `.s` ZA vgx4 group (FEAT_SME_F8F32). Note the ZA group is vgx4 but the `Zn` source is a TWO-register list.
    /// `top` picks `FVDOTT`; `index` 0..=3; `zm` is `z0..z7`. base `0xC1D0_0800` (`[11]`=1, `[5]`=0): top`[4]`, Zm`[18:16]`,
    /// Wv-8`[14:13]`, Zn>>1`[9:6]`, off`[2:0]`, index = bit10:bit3.
    Sme2ZaFp8VerticalDotBottomTop {
        top: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FMLAL ZA.H[Wv, slice:slice+1], Zn.B, Zm.B` / `FMLALL ZA.S[Wv, slice:slice+3], Zn.B, Zm.B` -- SME2 FP8 widening
    /// multiply-add-long into a ZA slice RANGE (a distinct operand model from the vgx2/vgx4 group form). `FMLAL`
    /// (`size`=`.h`, FEAT_SME_F8F16, 2-slice range, `slice` in {0,2,4,6}) widens FP8->FP16; `FMLALL` (`size`=`.s`,
    /// FEAT_SME_F8F32, 4-slice range, `slice` in {0,4}) is the 4-way long-long. `zn` is `z0..z31`, `zm` is `z0..z15`.
    /// base `.h` `0xC130_0C00` / `.s` `0xC130_0400` (`[11:10]` = 11 / 01): Zm`[19:16]`, Wv-8`[14:13]`, Zn`[9:5]`, slice-off (`.h`
    /// slice/2 at `[1:0]`; `.s` slice/4 at `[0]`).
    Sme2ZaFp8MlalSingle {
        size: Arm64VectorElement,
        wv: u8,
        slice: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMLAL ZA.H[Wv, slice:slice+1{, vgx2|vgx4}], { Zn.B-... }, Zm.B` / `FMLALL ZA.S[Wv, slice:slice+3{, ...}], ...`
    /// -- SME2 FP8 widening multiply-add-long into a ZA slice range from a two-/four-register `Zn` group by a SINGLE
    /// multiplier `Zm` (`z0..z15`). `size`=`.h` (FMLAL, FEAT_SME_F8F16) / `.s` (FMLALL, FEAT_SME_F8F32); `four` picks
    /// vgx4. base `.h` `0xC120_0804` / `.s` `0xC120_0002` (vgx`[20]`): Zm`[19:16]`, Wv-8`[14:13]`, Zn>>1`[9:6]` (vgx2) /
    /// Zn>>2`[9:7]` (vgx4), slice-off (`.h` slice/2 at `[1:0]`; `.s` slice/4 at `[0]`).
    Sme2ZaFp8MlalMulti {
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        slice: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMLAL ZA.H[Wv, slice:slice+1], Zn.B, Zm.B[index]` / `FMLALL ZA.S[Wv, slice:slice+3], Zn.B, Zm.B[index]` -- SME2
    /// FP8 widening multiply-add-long into a ZA slice range by an INDEXED element of `Zm` (`z0..z7`, `index` 0..=15).
    /// `size`=`.h` (FMLAL, FEAT_SME_F8F16) / `.s` (FMLALL, FEAT_SME_F8F32); `zn` is `z0..z31`. base `.h` `0xC1C0_0000`
    /// (index = bit15:bit11:bit10:bit3) / `.s` `0xC140_0000` (index = bit15:bit12:bit11:bit10): Zm`[18:16]`, Wv-8`[14:13]`,
    /// Zn`[9:5]`, slice-off (`.h` slice/2 at `[1:0]`; `.s` slice/4 at `[0]`).
    Sme2ZaFp8MlalIndexed {
        size: Arm64VectorElement,
        wv: u8,
        slice: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FMLAL ZA.H[Wv, slice:slice+1{, vgx2|vgx4}], { Zn.B-... }, { Zm.B-... }` / `FMLALL ZA.S[...]` -- SME2 FP8
    /// widening multiply-add-long into a ZA slice range from a `Zn` group by a same-size `Zm` GROUP. `size`=`.h`
    /// (FMLAL, FEAT_SME_F8F16) / `.s` (FMLALL, FEAT_SME_F8F32); `four` picks vgx4 (a 4-register `Zm` list). base `.h`
    /// `0xC1A0_0820` (`[23]`=1, `[11]`=1, `[5]`=1) / `.s` `0xC1A0_0020`: Zm>>1`[20:17]`, vgx`[16]`, Wv-8`[14:13]`, Zn>>1`[9:6]` (vgx2) /
    /// Zn>>2`[9:7]` (vgx4), slice-off (`.h` slice/2 at `[1:0]`; `.s` slice/4 at `[0]`).
    Sme2ZaFp8MlalMultiZm {
        size: Arm64VectorElement,
        four: bool,
        wv: u8,
        slice: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `BFMLAL`/`BFMLSL ZA.S[Wv, slice:slice+1], Zn.H, Zm.H` -- SME2 BFloat16 widening multiply-add/subtract-long into
    /// a ZA slice range (BF16 `.h` sources -> FP32 `.s` accumulator, FEAT_SME2). Like the FP8 [`Self::Sme2ZaFp8MlalSingle`]
    /// `.h` form but a `.h`(BF16)-source `[4]=1` encoding; `subtract` picks `BFMLSL` at `[3]`. `slice` in {0,2,4,6}; `zn`
    /// is `z0..z31`, `zm` is `z0..z15`. base `0xC120_0C10`: Zm`[19:16]`, Wv-8`[14:13]`, Zn`[9:5]`, slice/2 at `[1:0]`.
    Sme2ZaBfmlalSingle {
        subtract: bool,
        wv: u8,
        slice: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BFMLAL`/`BFMLSL ZA.S[Wv, slice:slice+1{, vgx2|vgx4}], { Zn.H-... }, Zm.H` -- the multi-vector (two-/four-register
    /// `Zn` group, single `Zm`) BFMLAL/BFMLSL into a ZA slice range (FEAT_SME2). base `0xC120_0810` (vgx`[20]`; `subtract`
    /// `[3]`): Zm`[19:16]`, Wv-8`[14:13]`, Zn>>1`[9:6]` (vgx2) / Zn>>2`[9:7]` (vgx4), slice/2 at `[1:0]`.
    Sme2ZaBfmlalMulti {
        subtract: bool,
        four: bool,
        wv: u8,
        slice: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BFMLAL`/`BFMLSL ZA.S[Wv, slice:slice+1], Zn.H, Zm.H[index]` -- the indexed-by-element BFMLAL/BFMLSL into a ZA
    /// slice range (FEAT_SME2). `zm` is `z0..z7`, `index` 0..=7. base `0xC180_1010` (`subtract` `[3]`): index =
    /// bit15:bit11:bit10, Zm`[18:16]`, Wv-8`[14:13]`, Zn`[9:5]`, slice/2 at `[1:0]`.
    Sme2ZaBfmlalIndexed {
        subtract: bool,
        wv: u8,
        slice: u8,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `BFMLAL`/`BFMLSL ZA.S[Wv, slice:slice+1{, vgx2|vgx4}], { Zn.H-... }, { Zm.H-... }` -- the by-`Zm`-group
    /// BFMLAL/BFMLSL into a ZA slice range (FEAT_SME2). base `0xC1A0_0810` (`[23]`=1 multi-Zm; `subtract` `[3]`):
    /// Zm>>1`[20:17]`, vgx`[16]`, Wv-8`[14:13]`, Zn>>1`[9:6]` (vgx2) / Zn>>2`[9:7]` (vgx4), slice/2 at `[1:0]`.
    Sme2ZaBfmlalMultiZm {
        subtract: bool,
        four: bool,
        wv: u8,
        slice: u8,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `MOVA { Zd.D-... }, ZA.D[Wv, off{, vgx2|vgx4}]` -- SME2 multi-vector move of a ZA array vector group INTO a
    /// two-/four-register `Zd` group (FEAT_SME2). This whole-vector array form has NO element-size field (the bytes are
    /// identical for `.b`/`.s`/`.d`; canonical disassembly is `.d`). `four` selects vgx4; `wv`=`W8..W11` (`0..=3`);
    /// `off`=`0..=7`; `zd_base` is the destination list base (a multiple of 2/4). base `0xC006_0800`: Wv-8`[14:13]`,
    /// vgx`[10]`, off`[7:5]`, Zd>>1`[4:1]`. Emits the preferred `mov` alias (matching [`Self::SmeMova`]).
    Sme2MovaArrayToVec {
        four: bool,
        wv: u8,
        off: u8,
        zd_base: Arm64ScalableVectorRegister,
    },

    /// `MOVA ZA.D[Wv, off{, vgx2|vgx4}], { Zn.D-... }` -- SME2 multi-vector move of a two-/four-register `Zn` group INTO
    /// a ZA array vector group (FEAT_SME2). Like [`Self::Sme2MovaArrayToVec`] reversed: base `0xC004_0800` (`[17]`=0),
    /// off`[2:0]`, Zn>>1`[9:6]`. `zn_base` is the source list base.
    Sme2MovaVecToArray {
        four: bool,
        wv: u8,
        off: u8,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `MOVA { Zd.<T>-... }, ZA<tile><H|V>.<T>[Wv, o0:o1]` (and the reverse) -- SME2 multi-vector move between a
    /// two-/four-register `Z` group and a horizontal/vertical ZA TILE slice group (FEAT_SME2; the 2/4-vector analog of
    /// the single-vector [`Self::SmeMova`], unpredicated). `to_vector` picks the ZA->Z direction; `size` is the tile
    /// element (`.b`/`.h`/`.s`/`.d`); `vertical` picks the V slice; `four` selects vgx4; `za_tile` is the tile number
    /// (`0..2^size`); `slice_reg`=`W12..W15` (`0..=3`); `slice_offset` is the slice-GROUP index (the printed range is
    /// `slice_offset*VG : +VG-1`); `z_base` is the Z list base. base `0xC004_0000` (`[17]`=to_vector): size`[23:22]`,
    /// vgx`[10]`, the combined `(za_tile<<goff)|groff` field at `[7:5]` (to-vector) / `[2:0]` (to-tile), Z>>1 at `[4:1]` /
    /// `[9:6]`. `[11]=0` separates it from the array MOVA ([`Self::Sme2MovaArrayToVec`], `[11]=1`).
    Sme2MovaTileSlice {
        to_vector: bool,
        size: Arm64VectorElement,
        vertical: bool,
        four: bool,
        za_tile: u8,
        slice_reg: u8,
        slice_offset: u8,
        z_base: Arm64ScalableVectorRegister,
    },

    /// `MOVAZ { Zd.D-... }, ZA.D[Wv, off{, vgx2|vgx4}]` -- SME2.1 move-and-zero of a ZA array vector group INTO a
    /// two-/four-register `Zd` group, zeroing the source (FEAT_SME2p1). This is the array MOVA-to-vector
    /// ([`Self::Sme2MovaArrayToVec`]) with the `[9]=1` zero marker; element-agnostic (`.d` canonical). base
    /// `0xC006_0A00`: Wv-8`[14:13]`, vgx`[10]`, off`[7:5]`, Zd>>1`[4:1]`.
    Sme2MovazArray {
        four: bool,
        wv: u8,
        off: u8,
        zd_base: Arm64ScalableVectorRegister,
    },

    /// `MOVAZ Zd.<T>, ZA<t><H|V>.<T>[Wv, off]` -- SME2.1 move-and-zero of a single ZA tile slice INTO a vector,
    /// zeroing the slice (FEAT_SME2p1). Unpredicated (unlike [`Self::SmeMova`]). `slice_reg` is `0..=3` for `W12..W15`.
    /// base `0xC002_0200` (`[17]`=1, `[9]`=1): size`[23:22]`, V(vertical)`[15]`, Wv-12`[14:13]`, combined (za_tile<<(4-size) |
    /// off) at `[8:5]`, Zd`[4:0]`.
    Sme2MovazTileSingle {
        size: Arm64VectorElement,
        vertical: bool,
        za_tile: u8,
        slice_offset: u8,
        slice_reg: u8,
        z: Arm64ScalableVectorRegister,
    },

    /// `MOVAZ { Zd.<T>-... }, ZA<t><H|V>.<T>[Wv, off{, vgx2|vgx4}]` -- SME2.1 move-and-zero of a ZA tile slice group
    /// INTO a two-/four-register `Zd` group (FEAT_SME2p1). The multi-vector tile MOVA-to-vector
    /// ([`Self::Sme2MovaTileSlice`], `to_vector`) with the `[9]=1` zero marker. `slice_reg` is `0..=3` for `W12..W15`.
    /// base `0xC006_0200`: size`[23:22]`, V`[15]`, Wv-12`[14:13]`, vgx`[10]`, combined<<5, Zd>>1`[4:1]`.
    Sme2MovazTileMulti {
        size: Arm64VectorElement,
        vertical: bool,
        four: bool,
        za_tile: u8,
        slice_offset: u8,
        slice_reg: u8,
        z_base: Arm64ScalableVectorRegister,
    },

    /// `MOVT ZT0[off, MUL VL], Zt` -- SME2 move a vector into the `ZT0` lookup-table register (FEAT_SME_LUTv2). `off`
    /// is the `MUL VL` table offset (`0..=3`); `zt` is the source `z0..z31`. base `0xC04F_03E0`: off`[13:12]`, Zt`[4:0]`.
    /// (The `MOVT ZT0, Zt` shorthand is `off`=0.)
    Sme2MovtTable {
        offset: u8,
        zt: Arm64ScalableVectorRegister,
    },

    /// `LD1<B|H|W|D>`/`ST1<B|H|W|D> { Zt.<T>-... }, PNg{/z}, [Xn{, #imm, mul vl}]` -- SME2/SVE2.1 multi-vector
    /// contiguous load/store of a two-/four-register `Z` group, governed by a PN predicate-as-counter, scalar+immediate
    /// addressing (FEAT_SME2). `store` picks ST1; `msz` is the access size (`.b`/`.h`/`.s`=`w`/`.d`); `four` selects
    /// vgx4; `png` is the governing `PN8..PN15` (loads print `/z`); `zt_base` is the Z list base (a multiple of 2/4);
    /// `rn` is the base register; `imm` is the signed vector offset (a multiple of VG, range `-8*VG..=7*VG`).
    /// `non_temporal` picks the `LDNT1`/`STNT1` non-temporal hint form (`[0]=1`). base `0xA040_0000` (`[22]`=1):
    /// store`[21]`, imm/VG`[19:16]`, vgx`[15]`, msz`[14:13]`, (PN-8)`[12:10]`, Rn`[9:5]`, Zt>>1`[4:1]`, NT`[0]`.
    Sme2MultiVecContiguousImm {
        store: bool,
        non_temporal: bool,
        msz: Arm64VectorElement,
        four: bool,
        png: Arm64PredicateAsCounter,
        zt_base: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm: i8,
    },

    /// `LD1<B|H|W|D>`/`ST1<B|H|W|D>`/`LDNT1*`/`STNT1* { Zt.<T>-... }, PNg{/z}, [Xn, Xm{, lsl #msz}]` -- like
    /// [`Self::Sme2MultiVecContiguousImm`] with a scalar+scalar (`[Xn, Xm]`) index (the index is implicitly scaled by
    /// the access size). base `0xA000_0000` (`[22]`=0): store`[21]`, Rm`[20:16]`, vgx`[15]`, msz`[14:13]`, (PN-8)`[12:10]`,
    /// Rn`[9:5]`, Zt>>1`[4:1]`, NT`[0]`.
    Sme2MultiVecContiguousScalar {
        store: bool,
        non_temporal: bool,
        msz: Arm64VectorElement,
        four: bool,
        png: Arm64PredicateAsCounter,
        zt_base: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LD1*`/`ST1*`/`LDNT1*`/`STNT1* { Zn, Zn+8 }`/`{ Zn, Zn+4, Zn+8, Zn+12 }, PNg{/z}, [Xn{, #imm, mul vl}]` -- SME2
    /// STRIDED multi-vector load/store, scalar+imm (FEAT_SME2): like [`Self::Sme2MultiVecContiguousImm`] but the Z group
    /// is strided (`[24]`=1) -- two registers 8 apart (vgx2, base `0..7`/`16..23`) or four 4 apart (vgx4, base
    /// `0..3`/`16..19`). base `0xA140_0000`: store`[21]`, imm/VG`[19:16]`, vgx`[15]`, msz`[14:13]`, (PN-8)`[12:10]`, Rn`[9:5]`,
    /// strided-base N`[4:0]` (bit3=0), **NT`[3]`** (reuses the always-zero bit3 of N). `strided_base` is N.
    Sme2MultiVecStridedImm {
        store: bool,
        non_temporal: bool,
        msz: Arm64VectorElement,
        four: bool,
        png: Arm64PredicateAsCounter,
        strided_base: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm: i8,
    },

    /// `LD1*`/`ST1*`/`LDNT1*`/`STNT1* { Zn, Zn+8 }`/`{...}, PNg{/z}, [Xn, Xm{, lsl #msz}]` -- the scalar+scalar form of
    /// [`Self::Sme2MultiVecStridedImm`]. base `0xA100_0000` (`[22]`=0): Rm`[20:16]` replaces the immediate.
    Sme2MultiVecStridedScalar {
        store: bool,
        non_temporal: bool,
        msz: Arm64VectorElement,
        four: bool,
        png: Arm64PredicateAsCounter,
        strided_base: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LUTI2`/`LUTI4 { Zd.<T>-... }, ZT0, Zm[index]` -- SME2 lookup-table read: gather `Zm`-indexed segments from the
    /// `ZT0` table register into a two-/four-register `Zd` group (FEAT_SME2). `lut4` picks LUTI4 (4-bit table indices)
    /// over LUTI2 (2-bit); `four` selects the four-register destination; `half` picks the `.h` element over `.b`; `zm`
    /// is the index vector; `index` is the table-segment selector. The valid forms and the index packing are irregular
    /// (LUTI4 `.b` has no four-register form; the index is 2 bits for LUTI2-2vec, else fewer), so a form table drives
    /// the base/shift/range. base e.g. `0xC08C_4000` (luti2 .b 2-vec): luti2/4`[18:17]`, index<<(15 2vec / 16 4vec),
    /// 2vec`[14]`/4vec`[15]`, `.h``[12]`, Zm`[9:5]`, Zd`[4:0]`.
    Sme2Luti {
        lut4: bool,
        four: bool,
        half: bool,
        zd_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `WHILEGE`/`WHILEGT`/`WHILELT`/`WHILELE`/`WHILEHS`/`WHILEHI`/`WHILELO`/`WHILELS PNd.<T>, Xn, Xm, vlx2|vlx4` --
    /// SVE2.1 compare-and-generate a predicate-as-counter governing a `vlx2`/`vlx4` multi-vector group (FEAT_SVE2p1).
    /// `op` is the comparison; `size` is the element (`.b`/`.h`/`.s`/`.d`); `four` selects `vlx4`; `pn` is the
    /// `PN8..PN15` result; `rn`/`rm` are the 64-bit operands. base `0x2520_4010`: size`[23:22]`, vlx`[13]`, op`[11:10]`+`[3]`,
    /// Rm`[20:16]`, Rn`[9:5]`, `[4]`=1 (the PN-counter destination marker, vs a normal predicate's `[4]=0`), PN-8`[2:0]`.
    WhileToPredicateCounter {
        op: Arm64WhileCounterOp,
        size: Arm64VectorElement,
        four: bool,
        pn: Arm64PredicateAsCounter,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `WHILE<cc> { Pd.<T>, Pd+1.<T> }, Xn, Xm` -- SVE2.1 compare-and-generate a predicate PAIR (FEAT_SVE2p1). Like
    /// [`Self::WhileToPredicateCounter`] but the destination is a `{Pd, Pd+1}` predicate pair (no `vlx` multiplier).
    /// base `0x2520_5010` (`[12]`=1, vs the PN-counter form's `[12]`=0): size`[23:22]`, op`[11:10]`+**`[0]`** (the strict bit
    /// moves to `[0]` here), Rm`[20:16]`, Rn`[9:5]`, `[4]`=1, Pd>>1`[3:1]`. `pd_base` is the even base of the pair.
    WhileToPredicatePair {
        op: Arm64WhileCounterOp,
        size: Arm64VectorElement,
        pd_base: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `ZIPQ1`/`ZIPQ2`/`UZPQ1`/`UZPQ2 Zd.<T>, Zn.<T>, Zm.<T>` -- SVE2.1 quadword (128-bit-segment) zip/unzip permute
    /// (FEAT_SVE2p1). `op` is the permute; `size` is the element (`.b`/`.h`/`.s`/`.d`). base `0x4400_E000`:
    /// size`[23:22]`, Zm`[20:16]`, op`[11:10]`, Zn`[9:5]`, Zd`[4:0]`. See [`Arm64SveQuadPermuteOp`].
    SveQuadPermute {
        op: Arm64SveQuadPermuteOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `TBLQ Zd.<T>, { Zn.<T> }, Zm.<T>` -- SVE2.1 quadword (128-bit-segment) table lookup (FEAT_SVE2p1). `zn` is the
    /// table; `zm` indexes within each 128-bit segment. base `0x4400_F800`: size`[23:22]`, Zm`[20:16]`, Zn`[9:5]`, Zd`[4:0]`.
    SveQuadTableLookup {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `TBXQ Zd.<T>, Zn.<T>, Zm.<T>` -- SVE2.1 quadword table lookup with the prior `Zd` merged where the index is out
    /// of range (FEAT_SVE2p1). base `0x0500_3400`: size`[23:22]`, Zm`[20:16]`, Zn`[9:5]`, Zd`[4:0]`.
    SveQuadTableExtend {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `DUPQ Zd.<T>, Zn.<T>[index]` -- SVE2.1 broadcast the indexed element of each 128-bit segment (FEAT_SVE2p1).
    /// `index` is `0..16/elem-bytes` (`.b` 0-15 / `.h` 0-7 / `.s` 0-3 / `.d` 0-1). base `0x0520_2400`: the size+index
    /// fold into the `tsz` field `[20:16]` = `(1<<size) | (index<<(size+1))`; Zn`[9:5]`, Zd`[4:0]`.
    SveQuadDupIndexed {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `EXTQ Zd.B, Zd.B, Zn.B, #imm` -- SVE2.1 extract a byte-aligned window spanning the destructive `Zd` and `Zn`
    /// within each 128-bit segment (FEAT_SVE2p1; `.b` only, destructive). `imm` is `0..=15`. base `0x0560_2400`
    /// (`[22]`=1 vs DUPQ's 0): imm`[19:16]`, Zn`[9:5]`, Zd`[4:0]`.
    SveQuadExtract {
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        imm: u8,
    },

    /// `BFADD`/`BFSUB`/`BFMUL`/`BFMAXNM`/`BFMINNM`/`BFMAX`/`BFMIN Zdn.H, Pg/M, Zdn.H, Zm.H` -- SVE2.1 BFloat16
    /// predicated, destructive binary arithmetic (FEAT_SVE_B16B16). `op` is the operation; `pg` is the governing
    /// predicate; `zdn` is the destructive accumulator; `zm` is the second source. base `0x6500_8000`: op`[18:16]`,
    /// Pg`[12:10]`, Zm`[9:5]`, Zdn`[4:0]`. See [`Arm64SveBf16BinaryOp`].
    SveBf16PredicatedBinary {
        op: Arm64SveBf16BinaryOp,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BFADD`/`BFSUB`/`BFMUL Zd.H, Zn.H, Zm.H` -- SVE2.1 BFloat16 unpredicated (non-destructive) three-same binary
    /// arithmetic (FEAT_SVE_B16B16). Only `op` in {`Add`,`Sub`,`Mul`} is valid here. base `0x6500_0000`: op`[11:10]`,
    /// Zm`[20:16]`, Zn`[9:5]`, Zd`[4:0]`. See [`Arm64SveBf16BinaryOp`].
    SveBf16UnpredicatedBinary {
        op: Arm64SveBf16BinaryOp,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BFCLAMP Zd.H, Zn.H, Zm.H` -- SVE2.1 BFloat16 clamp: `Zd = min(max(Zd, Zn), Zm)` (FEAT_SVE_B16B16). Destructive
    /// (`Zd` is also a source). base `0x6420_2400` (the `size==00` BF16 slot of the `FCLAMP` family): Zm`[20:16]`,
    /// Zn`[9:5]`, Zd`[4:0]`.
    SveBf16Clamp {
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BFMLA`/`BFMLS Zda.H, Pg/M, Zn.H, Zm.H` -- SVE2.1 BFloat16 predicated fused multiply-add / multiply-subtract
    /// into the destructive accumulator `Zda` (FEAT_SVE_B16B16). base `0x6520_0000`: op(mls)`[13]`, Zm`[20:16]`,
    /// Pg`[12:10]`, Zn`[9:5]`, Zda`[4:0]`. `sub` selects `BFMLS` over `BFMLA`.
    SveBf16MulAdd {
        sub: bool,
        pg: Arm64PredicateRegister,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BFMUL Zd.H, Zn.H, Zm.H[index]` -- SVE2.1 BFloat16 multiply by an indexed element (FEAT_SVE_B16B16). `zm` is
    /// restricted to `Z0..Z7` (the 3-bit `[18:16]` field); `index` is `0..=7`. base `0x6420_2800`: index`[22]`+`[20:19]`,
    /// Zm`[18:16]`, Zn`[9:5]`, Zd`[4:0]`.
    SveBf16MulIndexed {
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `BFMLA`/`BFMLS Zda.H, Zn.H, Zm.H[index]` -- SVE2.1 BFloat16 multiply-add / multiply-subtract by an indexed
    /// element into the destructive accumulator `Zda` (FEAT_SVE_B16B16). `zm` is `Z0..Z7`; `index` is `0..=7`. base
    /// `0x6420_0800`: op(mls)`[10]`, index`[22]`+`[20:19]`, Zm`[18:16]`, Zn`[9:5]`, Zda`[4:0]`. `sub` selects `BFMLS`.
    SveBf16MulAddIndexed {
        sub: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `ADDQV`/`SMAXQV`/`UMAXQV`/`SMINQV`/`UMINQV`/`ORQV`/`EORQV`/`ANDQV Vd.<T>, Pg, Zn.<T>` -- SVE2.1 across-lanes
    /// quadword integer reduction (FEAT_SVE2p1): reduces each 128-bit segment of `Zn` to one lane of the NEON `Vd`
    /// (a full-128 arrangement). Shares the integer-reduction group with `SADDV`/etc. but a disjoint `[20:16]` opcode.
    /// base `0x0400_2000`: size`[23:22]`, opcode`[20:16]`, Pg`[12:10]`, Zn`[9:5]`, Vd`[4:0]`. See [`Arm64SveQuadReduceIntOp`].
    SveQuadReduceInt {
        op: Arm64SveQuadReduceIntOp,
        size: Arm64VectorElement,
        vd: Arm64FloatRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `FMAXNMQV`/`FMINNMQV`/`FMAXQV`/`FMINQV Vd.<T>, Pg, Zn.<T>` -- SVE2.1 across-lanes quadword floating-point
    /// reduction (FEAT_SVE2p1): reduces each 128-bit segment of `Zn` to one lane of the NEON `Vd`. `size` is `.h`/`.s`/
    /// `.d` only (`.b` is rejected). base `0x6400_A000`: size`[23:22]`, opcode`[20:16]`, Pg`[12:10]`, Zn`[9:5]`, Vd`[4:0]`.
    /// See [`Arm64SveQuadReduceFpOp`].
    SveQuadReduceFp {
        op: Arm64SveQuadReduceFpOp,
        size: Arm64VectorElement,
        vd: Arm64FloatRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `SQCVTN`/`UQCVTN`/`SQCVTUN Zd.H, {Zn.S-Zn+1.S}` -- SVE2.1 two-vector saturating narrowing convert (FEAT_SVE2p1):
    /// narrows the consecutive `.s` Z pair starting at `zn_base` (which must be even) to a single `.h` `zd`. base
    /// `0x4531_4000`: op`[12:11]`, Zn-pair`[9:6]` (= base>>1), Zd`[4:0]`. See [`Arm64SveNarrowConvertOp`].
    SveNarrowConvert {
        op: Arm64SveNarrowConvertOp,
        zd: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `SQRSHRN`/`UQRSHRN`/`SQRSHRUN Zd.H, {Zn.S-Zn+1.S}, #shift` -- SVE2.1 two-vector saturating rounding shift-right
    /// narrowing (FEAT_SVE2p1): shifts the even `.s` pair right by `shift` (`1..=16`), rounds, saturates, narrows to
    /// `.h`. base `0x45A0_0800`: op`[13:12]`, (32-shift)`[20:16]`, Zn-pair`[9:6]` (= base>>1), Zd`[4:0]`. See
    /// [`Arm64SveShiftNarrowOp`].
    SveShiftNarrow {
        op: Arm64SveShiftNarrowOp,
        zd: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
        shift: u8,
    },

    /// `SQRSHRN`/`UQRSHRN`/`SQRSHRUN Zd.<Td>, {Zn.<Ts>-Zn+3.<Ts>}, #shift` -- SME2 four-vector saturating rounding
    /// shift-right narrowing (FEAT_SME2). `dest` is `B` (`.s`->`.b`, shift `1..=32`) or `H` (`.d`->`.h`, shift
    /// `1..=64`). base `0xc100_dc00`: size+shift byte`[23:16]`, Zn-quad`[9:7]` (= base>>2), op`[6:5]`, Zd`[4:0]`. The shift
    /// byte V is `128-shift` for `.b`, and `256-shift`(shift<=32) / `224-shift`(shift>32) for `.h`. See
    /// [`Arm64SveShiftNarrowOp`].
    Sme2QuadShiftNarrow {
        op: Arm64SveShiftNarrowOp,
        dest: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
        shift: u8,
    },

    /// `SQCVTN`/`UQCVTN`/`SQCVTUN Zd.<Td>, {Zn.<Ts>-Zn+3.<Ts>}` -- SME2 four-vector saturating narrowing convert
    /// (FEAT_SME2): narrows a multiple-of-4 source group (`.s`->`.b` when `dest` is `B`, `.d`->`.h` when `dest` is `H`)
    /// to a single `zd`. base `0xc133_e040`: dest-size`[23]`, to-unsigned op`[22]`, unsigned-source op`[5]`, Zn-quad`[9:7]`
    /// (= base>>2), Zd`[4:0]`. Reuses [`Arm64SveNarrowConvertOp`].
    Sme2NarrowConvert {
        op: Arm64SveNarrowConvertOp,
        dest: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `SQCVT`/`UQCVT`/`SQCVTU Zd.<Td>, {Zn.<Ts>-...}` -- SME2 multi-vector saturating narrowing convert WITHOUT the `N`
    /// (non-interleaving) form (FEAT_SME2). `four=false`: a `.s` pair -> `.h` (base `0xC123_E000`, Zn-pair`[9:6]`). `four=true`:
    /// a four-vector group, `.s`->`.b` (`dest` `B`) or `.d`->`.h` (`dest` `H`, adds `[23]`) (base `0xC133_E000`, Zn-quad`[9:7]`).
    /// Op via to-unsigned`[22]`+unsigned-source`[5]`; the `[6]=0` separates the 4-vec from the `N` form [`Self::Sme2NarrowConvert`].
    Sme2NarrowConvertNoN {
        op: Arm64SveNarrowConvertOp,
        four: bool,
        dest: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `SQRSHR`/`UQRSHR`/`SQRSHRU Zd.<Td>, {Zn.<Ts>-...}, #shift` -- SME2 multi-vector saturating rounding shift-right
    /// narrowing WITHOUT the `N` (non-interleaving) form (FEAT_SME2). `four=false`: a `.s` pair -> `.h` (shift `1..=16`,
    /// base `0xC1E0_D400`, `(16-shift)``[19:16]`, to-unsigned`[20]`, unsigned-src`[5]`, Zn-pair`[9:6]`). `four=true`: a four-vector
    /// group `.s`->`.b` (shift `1..=32`) or `.d`->`.h` (shift `1..=64`), encoded like [`Self::Sme2QuadShiftNarrow`] but the
    /// `[10]=0` (vs the `N` form's `[10]=1`) at base `0xC100_D800`, op at `[6:5]`.
    Sme2ShiftNarrowNoN {
        op: Arm64SveShiftNarrowOp,
        four: bool,
        dest: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
        shift: u8,
    },

    /// `SUNPK`/`UUNPK {Zd.<Tw>-...}, {Zn.<Ts>-...}` -- SME2 multi-vector signed/unsigned unpack (widening sign/zero
    /// extend, FEAT_SME2). `four=false`: one source vector -> a `.<Ts*2>` 2-vector dest; `four=true`: a 2-vector source ->
    /// a `.<Ts*2>` 4-vector dest. `src_size` is the source element (`.b`->`.h`, `.h`->`.s`, `.s`->`.d`). base
    /// `0xC125_E000`: size+1`[23:22]`, vgx4`[20]`, Zn`[9:5]`, Zd-base`[4:1]`, unsigned`[0]`.
    Sme2UnpackWiden {
        unsigned: bool,
        four: bool,
        src_size: Arm64VectorElement,
        zd_base: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `SEL {Zd.<T>-...}, PNg, {Zn.<T>-...}, {Zm.<T>-...}` -- SME2 multi-vector select governed by a predicate-as-counter
    /// (FEAT_SME2). Selects, per element, from the `Zn` or `Zm` two-/four-register group into the `Zd` group. base
    /// `0xC120_8000`: size`[23:22]`, Zm>>1`[20:17]`, vgx4`[16]`, PNg-8`[12:10]`, Zn>>1`[9:6]`, Zd`[4:0]` (a list base, even for vgx2
    /// or a multiple of 4 for vgx4).
    Sme2VectorSelect {
        size: Arm64VectorElement,
        four: bool,
        png: Arm64PredicateAsCounter,
        zd_base: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
        zm_base: Arm64ScalableVectorRegister,
    },

    /// `FCVTN`/`BFCVT`/`BFCVTN Zd.H, {Zn.S-Zn+1.S}` -- SME2 two-vector floating-point narrowing convert (FEAT_SME2):
    /// narrows an even `.s` (f32) pair to `.h` (`FCVTN` -> f16, `BFCVT`/`BFCVTN` -> BFloat16). base `0xc120_e000`:
    /// op`[22]`+`[5]`, Zn-pair`[9:6]` (= base>>1), Zd`[4:0]`. See [`Arm64Sme2FpCvtNarrowOp`].
    Sme2FpCvtNarrow {
        op: Arm64Sme2FpCvtNarrowOp,
        zd: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `PTRUE PNd.<T>` -- SVE2.1 initialise an all-true predicate-as-counter (FEAT_SVE2p1). `size` is the element;
    /// `pn` is the `PN8..PN15` destination. base `0x2520_7810`: size`[23:22]`, PN-8`[2:0]`, `[4]`=1 PN-counter marker.
    PTruePredicateCounter {
        size: Arm64VectorElement,
        pn: Arm64PredicateAsCounter,
    },

    /// `CNTP Xd, PNn.<T>, vlx2|vlx4` -- SVE2.1 count the active elements of a predicate-as-counter (FEAT_SVE2p1).
    /// `size` is the element; `four` selects `vlx4`; `pn` is the `PN8..PN15` source; `rd` is the 64-bit result. base
    /// `0x2520_8300`: size`[23:22]`, vlx`[10]`, PN-8`[7:5]`, Rd`[4:0]`.
    CountPredicateCounter {
        size: Arm64VectorElement,
        four: bool,
        pn: Arm64PredicateAsCounter,
        rd: Arm64GeneralPurposeRegister,
    },

    /// `PEXT Pd.<T>, PNn[index]` -- SVE2.1 extract a single predicate from a predicate-as-counter (FEAT_SVE2p1).
    /// `size` is the element; `pd` is the `P0..P15` result; `pn` is the `PN8..PN15` source; `index` is `0..=3`. base
    /// `0x2520_7010`: size`[23:22]`, index`[9:8]`, PN-8`[7:5]`, `[4]`=1, Pd`[3:0]`.
    PredicateExtractSingle {
        size: Arm64VectorElement,
        pd: Arm64PredicateRegister,
        pn: Arm64PredicateAsCounter,
        index: u8,
    },

    /// `PEXT { Pd.<T>, Pd+1.<T> }, PNn[index]` -- SVE2.1 extract a predicate PAIR from a predicate-as-counter
    /// (FEAT_SVE2p1). `pd_base` is the even base of the `{Pd, Pd+1}` pair; `index` is `0..=1`. base `0x2520_7410`
    /// (`[10]`=1 pair marker): index`[8]`, Pd>>1`[3:1]`.
    PredicateExtractPair {
        size: Arm64VectorElement,
        pd_base: Arm64PredicateRegister,
        pn: Arm64PredicateAsCounter,
        index: u8,
    },

    /// `PSEL Pd, Pn, Pm.<T>[Wv, off]` -- SME2 predicate select: `Pd` = `Pn` if the indexed `Pm` element is true, else
    /// all-false (FEAT_SME2). The `Pm.<T>[Wv, off]` slice indexes by element `size`, slice register `slice_reg`
    /// (`0..=3` for `W12..W15`), and `slice_offset` (range by size: `.b` `0..15` ... `.d` `0..1`).
    SmePredicateSelect {
        size: Arm64VectorElement,
        pd: Arm64PredicateRegister,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
        slice_reg: u8,
        slice_offset: u8,
    },

    /// `SET<stage>{n}`/`SETG<stage>{n} [Xd]!, Xn!, Xm` -- FEAT_MOPS memory set, or the tagged `SETG*` set-with-tag
    /// (`tagged`, which additionally needs FEAT_MTE). `non_temporal` applies the non-temporal hint. `rd` is the dest
    /// pointer (writeback), `rn` the byte count (writeback), `rm` the byte value. See [`Arm64MopsStage`].
    MopsSet {
        tagged: bool,
        stage: Arm64MopsStage,
        non_temporal: bool,
        unpriv: bool,
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LD1R{S}{B,H,W,D} { Zt.<T> }, Pg/Z, [Xn|SP{, #imm}]` -- SVE load one element and broadcast it to every lane
    /// (FEAT_SVE). The 4-bit dtype is the same packing as [`Arm64SveContiguousLoadType`]. `imm6` is the unscaled
    /// `0..=63` immediate; the displayed byte offset is `imm6 * access_size_bytes` (a `0` offset prints `[Xn]`).
    SveLoadReplicate {
        load: Arm64SveContiguousLoadType,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm6: u8,
    },

    /// `LD1RQ{B,H,W,D} { Zt.<T> }, Pg/Z, [Xn|SP, Xm{, LSL #amount}]` -- SVE load and replicate a 128-bit quadword to
    /// every 128-bit segment, scalar base + scalar index (FEAT_SVE). `size` is the element/access size; the index is
    /// scaled by the access size. `Xm` must not be `XZR`.
    SveLoadReplicateQuadScalar {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LD1RQ{B,H,W,D} { Zt.<T> }, Pg/Z, [Xn|SP{, #imm}]` -- SVE load and replicate a 128-bit quadword, scalar base +
    /// immediate (FEAT_SVE). `imm4` is the `-8..=7` quadword-count offset (displayed byte offset = `imm4 * 16`).
    SveLoadReplicateQuadImm {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm4: i8,
    },

    /// `LDNT1{B,H,W,D}`/`STNT1{B,H,W,D} {Zt.<T>}, Pg{/Z}, [Xn|SP, Xm{, LSL #amount}]` -- SVE contiguous non-temporal
    /// load/store, scalar base + scalar index (FEAT_SVE). `store` picks `STNT1`; `size` is the element/access size;
    /// the index is scaled by the access size. `Xm` must not be `XZR`.
    SveNonTemporalScalar {
        store: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LDNT1{B,H,W,D}`/`STNT1{B,H,W,D} {Zt.<T>}, Pg{/Z}, [Xn|SP{, #imm, MUL VL}]` -- SVE contiguous non-temporal
    /// load/store, scalar base + immediate (FEAT_SVE). `store` picks `STNT1`; `imm4` is the `-8..=7` element-count
    /// offset (scaled by the vector length).
    SveNonTemporalImm {
        store: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm4: i8,
    },

    /// `LD2{B,H,W,D}`/`LD3`/`LD4` and `ST2`/`ST3`/`ST4` `{Zt.<T>, ...}, Pg{/Z}, [Xn|SP{, #imm, MUL VL}]` -- SVE
    /// structured (de)interleaving load/store, scalar base + immediate (FEAT_SVE). `count` is the 2/3/4-register
    /// structure; `size` is the element (`.b`/`.h`/`.s`/`.d`, with mnemonic suffix `b`/`h`/`w`/`d`); `zt` is the
    /// first register of the list. `imm4` is the `-8..=7` structure-count offset (displayed = `imm4 * count`,
    /// MUL VL). `store` selects `ST`. Loads take `Pg/Z`, stores take a bare `Pg`.
    SveStructuredLoadStoreImm {
        store: bool,
        count: Arm64SveStructureCount,
        size: Arm64VectorElement,
        zt: Arm64ScalableVectorRegister,
        pg: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        imm4: i8,
    },

    /// `LD2{B,H,W,D}`/`LD3`/`LD4` and `ST2`/`ST3`/`ST4` `{Zt.<T>, ...}, Pg{/Z}, [Xn|SP, Xm{, LSL #amount}]` -- SVE
    /// structured load/store, scalar base + scalar index (FEAT_SVE). The index is scaled by the access size; `Xm`
    /// must not be `XZR`. See [`Self::SveStructuredLoadStoreImm`].
    SveStructuredLoadStoreScalar {
        store: bool,
        count: Arm64SveStructureCount,
        size: Arm64VectorElement,
        zt: Arm64ScalableVectorRegister,
        pg: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LDFF1{S}{B,H,W,D} {Zt.<T>}, Pg/Z, [Xn|SP{, Xm{, LSL #amount}}]` -- SVE contiguous first-fault load with a scalar
    /// base and a scalar index (FEAT_SVE). Unlike the ordinary scalar+scalar loads, `Xm` MAY be `XZR` (meaning no
    /// offset; it prints `[Xn, xzr]`). The index is scaled by the access size. See [`Arm64SveContiguousLoadType`].
    SveLoadFirstFault {
        load: Arm64SveContiguousLoadType,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LDNF1{S}{B,H,W,D} {Zt.<T>}, Pg/Z, [Xn|SP{, #imm, MUL VL}]` -- SVE contiguous non-fault load, scalar base +
    /// immediate (FEAT_SVE). `imm4` is the `-8..=7` element-count offset (scaled by the vector length).
    SveLoadNonFault {
        load: Arm64SveContiguousLoadType,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm4: i8,
    },

    /// `LD1RO{B,H,W,D} { Zt.<T> }, Pg/Z, [Xn|SP, Xm{, LSL #amount}]` -- SVE load and replicate a 256-bit octword to
    /// every 256-bit segment, scalar base + scalar index (FEAT_F64MM). The index is scaled by the access size.
    /// `Xm` must not be `XZR`.
    SveLoadReplicateOctScalar {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `LD1RO{B,H,W,D} { Zt.<T> }, Pg/Z, [Xn|SP{, #imm}]` -- SVE load and replicate a 256-bit octword, scalar base +
    /// immediate (FEAT_F64MM). `imm4` is the `-8..=7` octword-count offset (displayed byte offset = `imm4 * 32`).
    SveLoadReplicateOctImm {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm4: i8,
    },

    /// `PRF{B,H,W,D} <prfop>, Pg, [Xn|SP{, #imm, MUL VL}]` -- SVE contiguous prefetch, scalar base + immediate
    /// (FEAT_SVE). `msz` is the prefetch granule (`B`/`H`/word=`S`/`D`); `prfop` is the 4-bit prefetch operation
    /// (`<pld|pst><l1|l2|l3><keep|strm>`, e.g. `pldl1keep` = 0). `imm6` is the `-32..=31` element-count offset.
    SvePrefetchImm {
        msz: Arm64VectorElement,
        prfop: u8,
        pg: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        imm6: i8,
    },

    /// `PRF{B,H,W,D} <prfop>, Pg, [Xn|SP, Xm{, LSL #amount}]` -- SVE contiguous prefetch, scalar base + scalar index
    /// (FEAT_SVE). The index is scaled by the granule size. See [`Self::SvePrefetchImm`] for `prfop`.
    SvePrefetchScalar {
        msz: Arm64VectorElement,
        prfop: u8,
        pg: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `PRF{B,H,W,D} <prfop>, Pg, [Zn.<T>{, #imm}]` -- SVE gather prefetch with a vector base and a scalar immediate
    /// offset (FEAT_SVE). `element` is the base-vector lane (`.s`/`.d`); `msz` is the prefetch granule. `imm5` is the
    /// unscaled `0..=31` immediate (displayed offset = `imm5 * granule_bytes`). See [`Self::SvePrefetchImm`] for `prfop`.
    SveGatherPrefetchVectorImm {
        msz: Arm64VectorElement,
        element: Arm64VectorElement,
        prfop: u8,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        imm5: u8,
    },

    /// `PRF{B,H,W,D} <prfop>, Pg, [Xn|SP, Zm.<T>, <mode>{ #amount}]` -- SVE gather prefetch with a scalar base and a
    /// vector offset (FEAT_SVE). `element` is the offset-vector lane (`.s`/`.d`); `msz` is the prefetch granule;
    /// `mode` extends the offset (`LSL` needs `.d`). The offset is always scaled by the granule. See `prfop` above.
    SveGatherPrefetchScalarVector {
        msz: Arm64VectorElement,
        element: Arm64VectorElement,
        mode: Arm64SveOffsetMode,
        prfop: u8,
        pg: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Pd.<T>, Pg/Z, Zn.<T>, Zm.<T>` -- SVE integer compare of two vectors into a predicate result
    /// (FEAT_SVE). `pg` (P0..P7) governs the comparison. See [`Arm64SveIntCompareOp`].
    SveIntCompareVectors {
        op: Arm64SveIntCompareOp,
        size: Arm64VectorElement,
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>` -- SVE predicated floating-point binary (destructive; FEAT_SVE).
    /// `Zdn` is both destination and first source; `pg` (P0..P7) governs. Valid for `.h`/`.s`/`.d`. See
    /// [`Arm64SveFpPredBinOp`].
    SveFpBinaryPredicated {
        op: Arm64SveFpPredBinOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BFSCALE Zdn.H, Pg/M, Zdn.H, Zm.H` -- SVE BFloat16 scale-by-`2^(int in Zm)` (FEAT_SVE_BFSCALE). This is `FSCALE`
    /// (opcode 9) in the BF16 `size==00` slot of the SVE predicated FP-arithmetic group. base `0x6509_8000`: Pg`[12:10]`,
    /// Zm`[9:5]`, Zdn`[4:0]`.
    SveBfscale {
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FIRSTP`/`LASTP Xd, Pg, Pn.<T>` -- SVE2.2 extract the index of the first/last active element of `Pn` (governed by
    /// `Pg`) into a general-purpose register (FEAT_SVE2p2). `last` picks `LASTP`. base `FIRSTP` `0x2521_8000` / `LASTP`
    /// `0x2522_8000`: size`[23:22]`, Pg`[12:10]`, Pn`[8:5]`, Xd`[4:0]`.
    SvePredExtractIndex {
        last: bool,
        size: Arm64VectorElement,
        rd: Arm64GeneralPurposeRegister,
        pg: Arm64PredicateRegister,
        pn: Arm64PredicateRegister,
    },

    /// `EXPAND Zd.<T>, Pg, Zn.<T>` -- SVE2.2 expand: pack the `Pg`-active elements of `Zn` into the low lanes of `Zd`
    /// (FEAT_SVE2p2). base `0x0531_8000`: size`[23:22]`, Pg`[12:10]`, Zn`[9:5]`, Zd`[4:0]`.
    SveExpand {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Pd.<T>, Pg/Z, Zn.<T>, Zm.<T>` -- SVE floating-point compare of two vectors into a predicate result
    /// (FEAT_SVE). `pg` (P0..P7) governs. Valid for `.h`/`.s`/`.d`. See [`Arm64SveFpCompareOp`].
    SveFpCompareVectors {
        op: Arm64SveFpCompareOp,
        size: Arm64VectorElement,
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FADDA <V>dn, Pg, <V>dn, Zm.<T>` -- SVE floating-point strictly-ordered accumulating add reduction
    /// (FEAT_SVE): fold every active `Zm` lane, in order, into the scalar FP accumulator `vdn` (which is both the
    /// source and the destination). `.h`/`.s`/`.d`.
    SveFpAddStrictReduction {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        vdn: Arm64FloatRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FTMAD Zdn.<T>, Zdn.<T>, Zm.<T>, #imm` -- SVE floating-point trigonometric multiply-add (FEAT_SVE); `imm3`
    /// (0..=7) selects the series coefficient. Destructive (`Zdn` is source and destination). `.h`/`.s`/`.d`.
    SveFpTrigMulAdd {
        size: Arm64VectorElement,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        imm3: u8,
    },

    /// `PUNPKHI`/`PUNPKLO Pd.H, Pn.B` -- SVE unpack and widen the high (`high=true`) or low half of a predicate
    /// into halfword-element form (FEAT_SVE).
    SvePredicateUnpack {
        high: bool,
        pd: Arm64PredicateRegister,
        pn: Arm64PredicateRegister,
    },

    /// `RDFFRS Pd.B, Pg/Z` -- read the first-fault register, predicated, and set the condition flags (the
    /// flag-setting form of [`Self::SveRdffrPredicated`], FEAT_SVE).
    SveRdffrSetFlags {
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
    },

    /// `EORBT`/`EORTB Zd.<T>, Zn.<T>, Zm.<T>` -- SVE2 interleaving exclusive-OR (FEAT_SVE2). `top` selects the
    /// `EORTB` (`tb=1`) interleave vs `EORBT` (`tb=0`). `.b`/`.h`/`.s`/`.d`.
    Sve2InterleavingEor {
        top: bool,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `INDEX Zd.<T>, <base>, <step>` -- SVE create an arithmetic sequence (FEAT_SVE): element `i` gets
    /// `base + i*step`. `base` and `step` are each an immediate (`-16..=15`) or a scalar register (see
    /// [`Arm64SveIndexOperand`]); register operands are `Wn` for `.b`/`.h`/`.s` and `Xn` for `.d`.
    SveIndex {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        base: Arm64SveIndexOperand,
        step: Arm64SveIndexOperand,
    },

    /// `CNT{B,H,W,D} Xd{, <pattern>{, MUL #imm}}` -- SVE element count into a 64-bit register (FEAT_SVE): the
    /// number of `element`-sized lanes selected by `pattern`, times `mul` (`1..=16`). `pattern` is the 5-bit
    /// predicate-count pattern (`31` = `ALL`, the default).
    SveElementCount {
        element: Arm64VectorElement,
        rd: Arm64GeneralPurposeRegister,
        pattern: u8,
        mul: u8,
    },

    /// `{INC,DEC}{B,H,W,D} Xdn{, <pattern>{, MUL #imm}}` -- SVE increment/decrement a 64-bit register by the
    /// element count (FEAT_SVE; `decrement` picks `DEC`). `pattern`/`mul` as for [`Self::SveElementCount`].
    SveIncDecScalar {
        element: Arm64VectorElement,
        decrement: bool,
        rdn: Arm64GeneralPurposeRegister,
        pattern: u8,
        mul: u8,
    },

    /// `{SQ,UQ}{INC,DEC}{B,H,W,D} <Xdn>|<Wdn>{, <pattern>{, MUL #imm}}` -- SVE saturating increment/decrement of a
    /// general-purpose register by the element count (FEAT_SVE). `unsigned` picks `UQ` (else `SQ`); `decrement`
    /// picks `DEC`; `wide` picks the 64-bit `Xdn` result (else a 32-bit saturating result -- the signed 32-bit
    /// form renders `Xdn, Wdn`, the unsigned `Wdn`). `pattern`/`mul` as for [`Self::SveElementCount`].
    SveSaturatingIncDecScalar {
        element: Arm64VectorElement,
        decrement: bool,
        unsigned: bool,
        wide: bool,
        rdn: Arm64GeneralPurposeRegister,
        pattern: u8,
        mul: u8,
    },

    /// `{SQ,UQ}{INC,DEC}{H,W,D} Zdn.<T>{, <pattern>{, MUL #imm}}` -- SVE saturating increment/decrement of every
    /// vector element by the element count (FEAT_SVE; `.b` has no vector form). Flags as for
    /// [`Self::SveSaturatingIncDecScalar`].
    SveSaturatingIncDecVector {
        element: Arm64VectorElement,
        decrement: bool,
        unsigned: bool,
        zd: Arm64ScalableVectorRegister,
        pattern: u8,
        mul: u8,
    },

    /// `DUP Zd.<T>, <R><n|SP>` -- broadcast a general-purpose scalar across every lane (FEAT_SVE). `rn` renders
    /// `Wn`/`WSP` for `.b`/`.h`/`.s` and `Xn`/`SP` for `.d`.
    SveDupScalar {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `DUP Zd.<T>, #<imm>{, LSL #8}` -- broadcast a signed 8-bit immediate across every lane (FEAT_SVE). `shift`
    /// applies the optional `LSL #8`.
    SveDupImmediate {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        imm8: i8,
        shift: bool,
    },

    /// `DUP Zd.<T>, Zn.<T>[index]` -- broadcast one indexed element across every lane (FEAT_SVE). The element size
    /// and `index` pack into the `imm2:tsz` lane field.
    SveDupIndexed {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        index: u32,
    },

    /// `<op> Pd.B, Pg/Z, Pn.B, Pm.B` (or `SEL Pd.B, Pg, Pn.B, Pm.B`) -- SVE predicate logical operation
    /// (FEAT_SVE). All predicates are byte-element. See [`Arm64SvePredLogicalOp`].
    SvePredicateLogical {
        op: Arm64SvePredLogicalOp,
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
    },

    /// `<op> Zd.<T>, Pg/M, Zn.<T>` -- SVE predicated integer unary op (FEAT_SVE; the sign/zero-extends and
    /// `FABS`/`FNEG` share this group). `pg` (P0..P7) governs (merging). See [`Arm64SvePredUnaryOp`].
    SveIntUnaryPredicated {
        op: Arm64SvePredUnaryOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Pg/Z, Zn.<T>` -- the **zeroing**-predicate form of [`Self::SveIntUnaryPredicated`]
    /// (FEAT_SVE2p2). Identical to the merging form but with inactive `Zd` elements zeroed; encoded in the same
    /// `0x0400_A000` frame with bit `[20]` cleared (the merging opcodes `0x10..0x1E` clear to `0x00..0x0E`).
    SveIntUnaryZeroing {
        op: Arm64SvePredUnaryOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> <V>d, Pg, Zn.<T>` -- SVE integer reduction to a SIMD&FP scalar (FEAT_SVE). `vd` is `Dn` for
    /// `SADDV`/`UADDV`, else the element-sized scalar. `pg` (P0..P7) governs. See [`Arm64SveIntReductionOp`].
    SveIntReduction {
        op: Arm64SveIntReductionOp,
        size: Arm64VectorElement,
        vd: Arm64FloatRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> <V>d, Pg, Zn.<T>` -- SVE floating-point reduction to an element-sized SIMD&FP scalar (FEAT_SVE).
    /// Valid for `.h`/`.s`/`.d`. `pg` (P0..P7) governs. See [`Arm64SveFpReductionOp`].
    SveFpReduction {
        op: Arm64SveFpReductionOp,
        size: Arm64VectorElement,
        vd: Arm64FloatRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, #<shift>` -- SVE predicated shift by immediate (destructive; FEAT_SVE). The
    /// `shift` range is `1..=esize` for `ASR`/`LSR` and `0..=esize-1` for `LSL`. `pg` (P0..P7) governs. See
    /// [`Arm64SveShiftImmOp`].
    SveShiftImmediatePredicated {
        op: Arm64SveShiftImmOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        shift: u8,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>` -- SVE predicated shift by vector (`ASR`/`LSR`/`LSL` and the reversed
    /// `ASRR`/`LSRR`/`LSLR`), FEAT_SVE. Destructive (`Zdn` is source and destination). See [`Arm64SvePredShiftVectorOp`].
    SveShiftVectorPredicated {
        op: Arm64SvePredShiftVectorOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> <Xdn>{, <Wdn>}, Pm.<T>` -- SVE inc/dec a scalar GP register by the active-element count of `Pm`
    /// (`INCP`/`DECP` plain, always 64-bit; `SQINCP`/`SQDECP`/`UQINCP`/`UQDECP` saturating, `width` = `W`/`X`).
    /// FEAT_SVE. See [`Arm64SvePredCountOp`].
    SvePredicateCountScalar {
        op: Arm64SvePredCountOp,
        size: Arm64VectorElement,
        width: Arm64RegisterWidth,
        pm: Arm64PredicateRegister,
        rdn: Arm64GeneralPurposeRegister,
    },

    /// `<op> Zdn.<T>, Pm.<T>` -- SVE inc/dec each element of an SVE vector by the active-element count of `Pm`
    /// (`INCP`/`DECP`/`SQINCP`/`SQDECP`/`UQINCP`/`UQDECP`), FEAT_SVE. See [`Arm64SvePredCountOp`].
    SvePredicateCountVector {
        op: Arm64SvePredCountOp,
        size: Arm64VectorElement,
        pm: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
    },

    /// `LDR Zt, [Xn|SP{, #imm, MUL VL}]` / `STR Zt, ...` -- fill/spill a whole scalable vector register, unpredicated
    /// (FEAT_SVE). `store` selects `STR`; `imm9` is the `-256..=255` `MUL VL` element offset.
    SveFillSpillVector {
        store: bool,
        zt: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
        imm9: i32,
    },

    /// `LDR Pt, [Xn|SP{, #imm, MUL VL}]` / `STR Pt, ...` -- fill/spill a whole predicate register (FEAT_SVE).
    SveFillSpillPredicate {
        store: bool,
        pt: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        imm9: i32,
    },

    /// `<op> Zda.<T>, Pg/M, Zn.<T>, Zm.<T>` -- SVE predicated floating-point fused multiply-add (FEAT_SVE). `zda` is
    /// the destination/accumulator; `pg` (P0..P7) governs. Valid for `.h`/`.s`/`.d`. See [`Arm64SveFpFmaOp`].
    SveFpFma {
        op: Arm64SveFpFmaOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.D, Zn.D, Zm.D` -- SVE unpredicated bitwise logical over the whole register (FEAT_SVE). See
    /// [`Arm64SveBitwiseLogicalOp`].
    SveBitwiseLogicalUnpredicated {
        op: Arm64SveBitwiseLogicalOp,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// SVE predicated integer multiply-accumulate (`MLA`/`MLS`/`MAD`/`MSB`; FEAT_SVE). The three register fields are
    /// `dst` (`[4:0]`, the destination), `reg16` (`[20:16]`), and `reg5` (`[9:5]`); the op's UAL operand order is
    /// rendered by the emitter. `pg` (P0..P7) governs. See [`Arm64SveIntMacOp`].
    SveIntMac {
        op: Arm64SveIntMacOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        dst: Arm64ScalableVectorRegister,
        reg16: Arm64ScalableVectorRegister,
        reg5: Arm64ScalableVectorRegister,
    },

    /// `<op> Pd.<T>, Pg/Z, Zn.<T>, #imm5` -- SVE integer compare against a signed immediate (`-16..=15`) into a
    /// predicate (FEAT_SVE). `pg` (P0..P7) governs. See [`Arm64SveCmpImmSignedOp`].
    SveIntCompareImmSigned {
        op: Arm64SveCmpImmSignedOp,
        size: Arm64VectorElement,
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        imm5: i8,
    },

    /// `<op> Pd.<T>, Pg/Z, Zn.<T>, #imm7` -- SVE integer compare against an unsigned immediate (`0..=127`) into a
    /// predicate (FEAT_SVE). `pg` (P0..P7) governs. See [`Arm64SveCmpImmUnsignedOp`].
    SveIntCompareImmUnsigned {
        op: Arm64SveCmpImmUnsignedOp,
        size: Arm64VectorElement,
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        imm7: u8,
    },

    /// `MOVPRFX Zd, Zn` -- SVE unpredicated move-prefix (FEAT_SVE): a constructive prefix that lets the following
    /// destructive instruction write `Zd` while reading `Zn`.
    SveMovprfxUnpredicated {
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `MOVPRFX Zd.<T>, Pg/M|Pg/Z, Zn.<T>` -- SVE predicated move-prefix (FEAT_SVE). `merge` selects `/M` (merging)
    /// vs `/Z` (zeroing); `pg` (P0..P7) governs.
    SveMovprfxPredicated {
        merge: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Zn.<T>, Zm.<T>` -- SVE permute vectors (`ZIP1`/`ZIP2`/`UZP1`/`UZP2`/`TRN1`/`TRN2`; FEAT_SVE).
    /// See [`Arm64VectorPermuteOp`].
    SvePermute {
        op: Arm64VectorPermuteOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Zn.<T>, Zm.<T>` -- SVE unpredicated floating-point binary (FEAT_SVE). Valid for `.h`/`.s`/`.d`.
    /// See [`Arm64SveFpBinUnpredOp`].
    SveFpBinaryUnpredicated {
        op: Arm64SveFpBinUnpredOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `CPY Zd.<T>, Pg/M|Pg/Z, #<imm>{, LSL #8}` -- SVE copy a signed 8-bit immediate into the active lanes
    /// (FEAT_SVE). `merge` selects `/M` vs `/Z`; `pg` may be `P0..P15`. (GNU/LLVM disassemble this as `mov`.)
    SveCopyImmediate {
        merge: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        imm8: i8,
        shift: bool,
    },

    /// `CPY Zd.<T>, Pg/M, <R><n|SP>` -- SVE copy a general-purpose scalar into the active lanes, merging (FEAT_SVE).
    /// `pg` is `P0..P7`; `rn` renders `Wn`/`WSP` for `.b`/`.h`/`.s` and `Xn`/`SP` for `.d`.
    SveCopyScalar {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `FDUP Zd.<T>, #<const>` (disassembled `FMOV Zd.<T>, #<const>`) -- SVE broadcast an 8-bit-encoded FP
    /// immediate across every lane, unpredicated (FEAT_SVE). `size` is `.h`/`.s`/`.d` (`.b` is invalid); `imm8` is
    /// the raw VFP/AdvSIMD modified-immediate field (obtain it with [`crate::fp8_encode_single`] /
    /// [`crate::fp8_encode_double`]). The FP-immediate analogue of [`Self::SveDupImmediate`].
    SveFdup {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        imm8: u8,
    },

    /// `FCPY Zd.<T>, Pg/M, #<const>` (disassembled `FMOV Zd.<T>, Pg/M, #<const>`) -- SVE copy an 8-bit-encoded FP
    /// immediate into the active (merging) lanes (FEAT_SVE). `size` is `.h`/`.s`/`.d`; `pg` may be `P0..P15`;
    /// `imm8` is the raw modified-immediate field. The FP-immediate, merge-only analogue of [`Self::SveCopyImmediate`].
    SveFcpy {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        imm8: u8,
    },

    /// `REV Zd.<T>, Zn.<T>` -- SVE reverse all elements in a vector (unpredicated; FEAT_SVE).
    SveReverseElements {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `REVB`/`REVH`/`REVW Zd.<T>, Pg/M, Zn.<T>` -- SVE reverse bytes/halfwords/words within each element
    /// (predicated; FEAT_SVE). `pg` is `P0..P7`. See [`Arm64SveReverseWidth`].
    SveReverseBytes {
        width: Arm64SveReverseWidth,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `REVB`/`REVH`/`REVW Zd.<T>, Pg/Z, Zn.<T>` -- the **zeroing**-predicate form of [`Self::SveReverseBytes`]
    /// (FEAT_SVE2p2). Same `0x0524_8000` frame with bit `[13]` set (`[15:13]` `100` -> `101`).
    SveReverseBytesZeroing {
        width: Arm64SveReverseWidth,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `REVD Zd.Q, Pg/M, Zn.Q` -- SVE reverse 64-bit doublewords within each 128-bit quadword (predicated;
    /// FEAT_SVE2p1). `pg` is `P0..P7`. base `0x052E_8000`.
    SveReverseDoublewords {
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `TBL Zd.<T>, {Zn.<T>}, Zm.<T>` -- SVE table lookup: gather elements of `Zn` indexed by `Zm` (FEAT_SVE,
    /// single-table form).
    SveTableLookup {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Pg/M, Zn.<T>` -- SVE predicated floating-point unary (round / `FRECPX` / `FSQRT`; FEAT_SVE).
    /// `pg` is `P0..P7`. Valid for `.h`/`.s`/`.d`. See [`Arm64SveFpUnaryOp`].
    SveFpUnaryPredicated {
        op: Arm64SveFpUnaryOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Pg/Z, Zn.<T>` -- the **zeroing**-predicate form of [`Self::SveFpUnaryPredicated`]
    /// (FEAT_SVE2p2). Encoded in the relocated `0x6400_8000` frame with the op spread over `[21:16]` and `[14:13]`.
    SveFpUnaryZeroing {
        op: Arm64SveFpUnaryOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `PFALSE Pd.B` -- set an entire predicate to false (FEAT_SVE).
    SvePfalse(Arm64PredicateRegister),

    /// `PTEST Pg, Pn.B` -- set the condition flags from a predicate under a governing predicate (FEAT_SVE).
    SvePtest {
        pg: Arm64PredicateRegister,
        pn: Arm64PredicateRegister,
    },

    /// `RDFFR Pd.B` -- read the first-fault register into a predicate (FEAT_SVE).
    SveRdffr(Arm64PredicateRegister),

    /// `RDFFR Pd.B, Pg/Z` -- read the first-fault register, predicated (FEAT_SVE).
    SveRdffrPredicated {
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
    },

    /// `WRFFR Pn.B` -- write a predicate into the first-fault register (FEAT_SVE).
    SveWrffr(Arm64PredicateRegister),

    /// `SETFFR` -- set the first-fault register to all-true (FEAT_SVE).
    SveSetffr,

    /// `PFIRST Pdn.B, Pg, Pdn.B` -- set the first active element of `Pdn` (FEAT_SVE). Destructive.
    SvePfirst {
        pdn: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
    },

    /// `PNEXT Pdn.<T>, Pg, Pdn.<T>` -- advance to the next active element (FEAT_SVE). Destructive.
    SvePnext {
        size: Arm64VectorElement,
        pdn: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
    },

    /// `INSR Zdn.<T>, <R><n>` -- shift the vector up by one element and insert a general-purpose scalar in the low
    /// lane (FEAT_SVE). `rn` renders `Wn` for `.b`/`.h`/`.s` and `Xn` for `.d`.
    SveInsr {
        size: Arm64VectorElement,
        zdn: Arm64ScalableVectorRegister,
        rn: Arm64GeneralPurposeRegister,
    },

    /// `LASTA`/`LASTB <R>d, Pg, Zn.<T>` -- extract the element after / at the last active lane into a
    /// general-purpose register (FEAT_SVE). `last_b` selects `LASTB`; `rd` is `Wd` for `.b`/`.h`/`.s`, `Xd` for `.d`.
    SveExtractLast {
        last_b: bool,
        size: Arm64VectorElement,
        rd: Arm64GeneralPurposeRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `LASTA`/`LASTB <V>d, Pg, Zn.<T>` -- extract the element after / at the last active lane into a SIMD&FP scalar
    /// register (FEAT_SVE). `vd` names `Bd`/`Hd`/`Sd`/`Dd` per `size`. `last_b` selects `LASTB`.
    SveExtractLastSimd {
        last_b: bool,
        size: Arm64VectorElement,
        vd: Arm64FloatRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `CLASTA`/`CLASTB Zdn.<T>, Pg, Zdn.<T>, Zm.<T>` -- conditionally extract the element after / at the last
    /// active lane of `Zm` into every lane of the destructive `Zdn`, or leave `Zdn` unchanged when no lane is
    /// active (FEAT_SVE). `last_b` selects `CLASTB`.
    SveConditionalExtractVector {
        last_b: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `CLASTA`/`CLASTB <R>dn, Pg, <R>dn, Zm.<T>` -- conditional extract into a general-purpose register (FEAT_SVE).
    /// `rdn` is `Wdn` for `.b`/`.h`/`.s`, `Xdn` for `.d`. `last_b` selects `CLASTB`.
    SveConditionalExtractGpr {
        last_b: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        rdn: Arm64GeneralPurposeRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `CLASTA`/`CLASTB <V>dn, Pg, <V>dn, Zm.<T>` -- conditional extract into a SIMD&FP scalar register (FEAT_SVE).
    /// `vd` names `Bdn`/`Hdn`/`Sdn`/`Ddn` per `size`. `last_b` selects `CLASTB`.
    SveConditionalExtractSimd {
        last_b: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        vd: Arm64FloatRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `ADR Zd.<T>, [Zn.<T>, Zm.<T>{, <mod> #<shift>}]` -- SVE compute a vector of addresses, base `Zn` plus each
    /// `Zm` offset shifted left by `shift` (`0..=3`) and optionally sign/zero-extended (FEAT_SVE). The element size
    /// and extend come from `mode`. See [`Arm64SveAdrMode`].
    SveAddressGeneration {
        mode: Arm64SveAdrMode,
        shift: u8,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FRECPE`/`FRSQRTE Zd.<T>, Zn.<T>` -- SVE unpredicated floating-point reciprocal / reciprocal-square-root
    /// estimate (FEAT_SVE). `sqrt` picks `FRSQRTE`. Valid for `.h`/`.s`/`.d`.
    SveFpReciprocalEstimate {
        sqrt: bool,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `FEXPA Zd.<T>, Zn.<T>` -- SVE floating-point exponential accelerator (FEAT_SVE); expands the lookup-table
    /// index in `Zn` into a coefficient. Valid for `.h`/`.s`/`.d`.
    SveFexpa {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `FTSSEL Zd.<T>, Zn.<T>, Zm.<T>` -- SVE floating-point trigonometric select coefficient (FEAT_SVE); selects
    /// 1.0 or the value in `Zn` per the low bits of `Zm`. Valid for `.h`/`.s`/`.d`.
    SveFtssel {
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `BRKA`/`BRKB Pd.B, Pg/<Z|M>, Pn.B` -- SVE break after / before the first active+true lane: set lanes up to
    /// (BRKA: including / BRKB: excluding) that lane true, the rest false (FEAT_SVE). `before` picks `BRKB`;
    /// `set_flags` picks the flag-setting `BRKAS`/`BRKBS` (zeroing only); `merging` picks `/M` (else `/Z`).
    SveBreak {
        before: bool,
        set_flags: bool,
        merging: bool,
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        pn: Arm64PredicateRegister,
    },

    /// `<op> Zd.<T>, Zn.<T>, Zm.<T>` -- SVE2 unpredicated integer multiply (`MUL`/`PMUL`/`SMULH`/`UMULH`)
    /// (FEAT_SVE2). `PMUL` is `.b` only; the rest take any element. See [`Arm64Sve2MulOp`].
    Sve2MultiplyUnpredicated {
        op: Arm64Sve2MulOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.D, Zdn.D, Zm.D, Zk.D` -- SVE2 bitwise ternary (three-source 64-bit logical) op (FEAT_SVE2). All
    /// operands are `.d`. See [`Arm64Sve2TernaryLogicalOp`].
    Sve2TernaryLogical {
        op: Arm64Sve2TernaryLogicalOp,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        zk: Arm64ScalableVectorRegister,
    },

    /// `WHILEGE`/`WHILEGT`/`WHILEHS`/`WHILEHI Pd.<T>, <R>n, <R>m` -- SVE2 while-greater loop predicates (FEAT_SVE2).
    /// `compare_64` picks the 64-bit `Xn,Xm` operands (else `Wn,Wm`). See [`Arm64Sve2WhileCompareOp`].
    Sve2WhileGreater {
        op: Arm64Sve2WhileCompareOp,
        size: Arm64VectorElement,
        compare_64: bool,
        pd: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `WHILERW`/`WHILEWR Pd.<T>, Xn, Xm` -- SVE2 pointer-hazard predicates (FEAT_SVE2): true while the `size`
    /// element ranges from `Xn`/`Xm` do not overlap as a read-after-write (`WHILERW`) / write-after-read
    /// (`WHILEWR`) hazard. Operands are always 64-bit. `write_after_read` picks `WHILEWR`.
    Sve2WhilePointerHazard {
        write_after_read: bool,
        size: Arm64VectorElement,
        pd: Arm64PredicateRegister,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `SABA`/`UABA Zd.<T>, Zn.<T>, Zm.<T>` -- SVE2 accumulate the absolute difference of `Zn` and `Zm` into `Zd`
    /// (FEAT_SVE2; `unsigned` picks `UABA`). Valid for `.b`/`.h`/`.s`/`.d`.
    Sve2AbsDiffAccumulate {
        unsigned: bool,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Pg/M, Zn.<T>` -- SVE2 predicated (merging) integer unary op (FEAT_SVE2): `SQABS`/`SQNEG`
    /// (saturating abs/negate) or `URECPE`/`URSQRTE` (unsigned reciprocal estimates, `.s` only). See
    /// [`Arm64Sve2UnaryPredOp`].
    Sve2UnaryPredicated {
        op: Arm64Sve2UnaryPredOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Pg/Z, Zn.<T>` -- the **zeroing**-predicate form of [`Self::Sve2UnaryPredicated`]
    /// (FEAT_SVE2p2). Same `0x4400_A000` frame with bit `[17]` set (the `[19:16]` code gains `0b0010`).
    Sve2UnaryZeroing {
        op: Arm64Sve2UnaryPredOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>` -- SVE2 predicated saturating/rounding bitwise shift left (FEAT_SVE2):
    /// shift `Zdn` by the signed per-element amount in `Zm`. Destructive. See [`Arm64Sve2ShiftLeftPredOp`].
    Sve2ShiftLeftPredicated {
        op: Arm64Sve2ShiftLeftPredOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FLOGB Zd.<T>, Pg/M, Zn.<T>` -- SVE2 predicated floating-point base-2 logarithm as a signed integer of the
    /// same width (FEAT_SVE2). Valid for `.h`/`.s`/`.d`.
    Sve2FpLogb {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `FLOGB Zd.<T>, Pg/Z, Zn.<T>` -- the **zeroing**-predicate form of [`Self::Sve2FpLogb`] (FEAT_SVE2p2).
    /// Relocated to `0x641E_8000` with the element size in `[14:13]`. Valid for `.h`/`.s`/`.d`.
    Sve2FpLogbZeroing {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `FRINT32Z`/`FRINT32X`/`FRINT64Z`/`FRINT64X Zd.<T>, Pg/{M,Z}, Zn.<T>` -- SVE round-to-integral-N (FEAT_SVE2p2;
    /// the SVE forms of the scalar/NEON FRINTTS ops). `.s`/`.d` only. `zeroing` selects `/Z` (relocated `0x64..`
    /// frame) vs `/M` (`0x65..` frame). See [`Arm64ScalarFrintTsOp`].
    SveFrintTs {
        op: Arm64ScalarFrintTsOp,
        zeroing: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `FMLALB`/`FMLALT`/`FMLSLB`/`FMLSLT`/`BFMLALB`/`BFMLALT Zda.S, Zn.H, Zm.H` -- SVE2 FP16/BFloat16 widening
    /// multiply-add long: accumulate the products of the even/odd (`top`) `.h` lanes into the `.s` accumulator
    /// (FEAT_SVE2; `bf16` picks the BFloat16 forms, which need FEAT_BF16 and have no subtract). `subtract` picks
    /// the `FMLSL*` forms.
    Sve2FpWidenMulAddLong {
        bf16: bool,
        subtract: bool,
        top: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zda.S, Zn.H, Zm.H[index]` -- SVE2 FP16/BFloat16 widening multiply-add long by indexed element:
    /// `FMLALB`/`FMLALT`/`FMLSLB`/`FMLSLT` (FEAT_SVE2) and `BFMLALB`/`BFMLALT` (`bf16`, FEAT_BF16; no subtract form).
    /// `index` is 0..7 and `Zm` is restricted to `Z0..Z7`. The 3-bit index is split across `[20]:[19]:[11]`.
    Sve2FpWidenMulAddLongIndexed {
        bf16: bool,
        subtract: bool,
        top: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `<op> Zda.<Tda>, Zn.<Tn>, Zm.<Tn>` -- SVE/SVE2 matrix-multiply-accumulate and BFloat16 dot product (the
    /// unpredicated three-register matmul/dot shape): `SMMLA`/`USMMLA`/`UMMLA` (FEAT_I8MM), `FMMLA` single/double
    /// (FEAT_F32MM / FEAT_F64MM), `BFMMLA`/`BFDOT` (FEAT_BF16). The destination/source element types come from `op`.
    /// See [`Arm64SveMatmulOp`].
    SveMatrixMul {
        op: Arm64SveMatmulOp,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMMLA Zda.H, Zn.H, Zm.H` -- SVE half-precision floating-point matrix multiply-accumulate (FEAT_F16MM,
    /// SVE2p2). base `0x64A0_E000` (`[23]`=1 separates it from the FP8 FMMLA). **Experimental** (binutils-trunk-only
    /// oracle: LLVM-20 has no FP16-matmul feature). Only present with the `experimental` cargo feature.
    #[cfg(feature = "experimental")]
    SveFp16Matmul {
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<Td>, Pg/M, Zn.<Tn>` -- SVE2 predicated floating-point precision up/down convert (top/bottom)
    /// (FEAT_SVE2; `BFCVT`/`BFCVTNT` need FEAT_BF16). The destination/source element sizes come from `op`. See
    /// [`Arm64Sve2FpUpdownOp`].
    Sve2FpConvertUpdown {
        op: Arm64Sve2FpUpdownOp,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<Td>, Pg/Z, Zn.<Tn>` -- the **zeroing**-predicate form of [`Self::Sve2FpConvertUpdown`]
    /// (FEAT_SVE2p2): FCVTLT/FCVTNT/FCVTX/FCVTXNT/BFCVT/BFCVTNT. Relocated to the `0x64..` frame; each op is a
    /// fully-fixed `[31:13]` base (see [`Arm64Sve2FpUpdownOp::zeroing_base`]).
    Sve2FpConvertUpdownZeroing {
        op: Arm64Sve2FpUpdownOp,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>` -- SVE2 predicated floating-point pairwise reduction (FEAT_SVE2):
    /// `FADDP`/`FMAXNMP`/`FMINNMP`/`FMAXP`/`FMINP`. Destructive. See [`Arm64Sve2FpPairwiseOp`].
    Sve2FpPairwise {
        op: Arm64Sve2FpPairwiseOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `USDOT Zda.S, Zn.B, Zm.B` -- SVE unsigned-by-signed 8-bit integer dot product to 32-bit (FEAT_SVE+I8MM):
    /// four unsigned `Zn` bytes by four signed `Zm` bytes, accumulated into each `.s` lane of `Zda`.
    Sve2UnsignedSignedDot {
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `CDOT Zda.<T>, Zn.<Tb>, Zm.<Tb>, #<rot>` -- SVE2 complex integer dot product (FEAT_SVE2): accumulate the
    /// real/imaginary dot product (per `rotation`) of complex byte/halfword pairs into `Zda` (`.s` from `.b`, or
    /// `.d` from `.h`). `rotation` is one of `#0`/`#90`/`#180`/`#270`.
    Sve2ComplexDot {
        size: Arm64VectorElement,
        rotation: Arm64ComplexRotation,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SQDMULH`/`SQRDMULH Zd.<T>, Zn.<T>, Zm.<T>` -- SVE2 signed saturating doubling multiply, returning the high
    /// half (FEAT_SVE2; unpredicated). `rounding` picks `SQRDMULH`. Valid for `.b`/`.h`/`.s`/`.d`.
    Sve2SaturatingDoublingMulHigh {
        rounding: bool,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SQRDMLAH`/`SQRDMLSH Zda.<T>, Zn.<T>, Zm.<T>` -- SVE2 signed saturating rounding doubling multiply-add /
    /// -subtract, high half, accumulating into `Zda` (FEAT_SVE2). `subtract` picks `SQRDMLSH`.
    Sve2SaturatingDoublingMulAddHigh {
        subtract: bool,
        size: Arm64VectorElement,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `TBL Zd.<T>, { Zn.<T>, Zn+1.<T> }, Zm.<T>` (two-register table) or `TBX Zd.<T>, Zn.<T>, Zm.<T>` (single-table
    /// merge) -- SVE2 programmable table lookup (FEAT_SVE2). `extend` picks `TBX` (the merging single-table form);
    /// otherwise `TBL` reads the two-register table `{Zn, Zn+1}` (modulo 32).
    Sve2TableLookup {
        extend: bool,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `MATCH`/`NMATCH Pd.<T>, Pg/Z, Zn.<T>, Zm.<T>` -- SVE2 detect (non-)matching elements into a predicate and
    /// set the condition flags (FEAT_SVE2). `half` picks the `.h` element (else `.b`); `negate` picks `NMATCH`.
    Sve2Match {
        negate: bool,
        half: bool,
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<Tw>, Zn.<Tn>, Zm.<Tn>[index]` -- SVE2 widening multiply(-add) by an indexed element (FEAT_SVE2).
    /// `size` is the wide result `.s` (from `.h`, `Zm` in `Z0..Z7`, `index` `0..7`) or `.d` (from `.s`, `Zm` in
    /// `Z0..Z15`, `index` `0..3`). See [`Arm64Sve2WidenIndexedOp`].
    Sve2WideningIndexed {
        op: Arm64Sve2WidenIndexedOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `BEXT`/`BDEP`/`BGRP Zd.<T>, Zn.<T>, Zm.<T>` -- SVE bitwise permute (FEAT_SVE_BitPerm): gather/scatter/group
    /// the bits of `Zn` under the `Zm` mask. See [`Arm64Sve2BitPermuteOp`].
    Sve2BitwisePermute {
        op: Arm64Sve2BitPermuteOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `ADCLB`/`ADCLT`/`SBCLB`/`SBCLT Zda.<T>, Zn.<T>, Zm.<T>` -- SVE2 integer add/subtract long with carry
    /// (FEAT_SVE2): a long add (`ADCL`) or subtract (`SBCL` -- `subtract`) propagating the carry from the even/odd
    /// (`top`) lanes. `element_double` selects `.d` (else `.s`).
    Sve2AddSubLongCarry {
        subtract: bool,
        top: bool,
        element_double: bool,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SSRA`/`USRA`/`SRSRA`/`URSRA Zda.<T>, Zn.<T>, #<shift>` -- SVE2 bitwise shift right (by `1..=esize`) and
    /// accumulate into `Zda` (FEAT_SVE2). `rounding` picks the `R`-prefixed forms; `unsigned` the `U` ones.
    Sve2ShiftRightAccumulate {
        rounding: bool,
        unsigned: bool,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        shift: u8,
    },

    /// `SRI`/`SLI Zd.<T>, Zn.<T>, #<shift>` -- SVE2 bitwise shift and insert (FEAT_SVE2): shift right (`SRI`,
    /// `shift` `1..=esize`) or left (`SLI`, `shift` `0..=esize-1`) and merge into the unshifted bits of `Zd`.
    /// `left` picks `SLI`.
    Sve2ShiftInsert {
        left: bool,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        shift: u8,
    },

    /// `SABALB`/`SABALT`/`UABALB`/`UABALT Zda.<Tw>, Zn.<Tn>, Zm.<Tn>` -- SVE2 absolute-difference and accumulate
    /// long (FEAT_SVE2): the abs-difference of the narrow even/odd source lanes accumulated into the wide `Zda`.
    /// `size` is the wide result (`.h`/`.s`/`.d`); the sources are one narrower. `unsigned` picks the `U` forms,
    /// `top` the `T` forms.
    Sve2AbsDiffAccLong {
        unsigned: bool,
        top: bool,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SADALP`/`UADALP Zda.<T>, Pg/M, Zn.<Tb>` -- SVE2 predicated pairwise add and accumulate long (FEAT_SVE2):
    /// add adjacent narrow-source pairs and accumulate into the wide `Zda`. `size` is the wide accumulator element
    /// (`.h`/`.s`/`.d`); the source is one narrower. `unsigned` picks `UADALP`.
    Sve2PairwiseAddAccLong {
        unsigned: bool,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>` -- SVE2 predicated saturating add/subtract (FEAT_SVE2). Destructive;
    /// `SUQADD`/`USQADD` are the mixed-sign accumulate forms. See [`Arm64Sve2SatAddSubOp`].
    Sve2SaturatingAddSub {
        op: Arm64Sve2SatAddSubOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>` -- SVE2 predicated integer halving add/subtract (FEAT_SVE2): the
    /// half-sum / half-difference of `Zdn` and `Zm`, optionally rounded. Destructive. See [`Arm64Sve2HalvingOp`].
    Sve2HalvingAddSub {
        op: Arm64Sve2HalvingOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>` -- SVE2 predicated integer pairwise arithmetic (FEAT_SVE2): reduce
    /// adjacent element pairs of the concatenation of `Zdn` and `Zm`. Destructive. See [`Arm64Sve2PairwiseOp`].
    Sve2PairwiseArith {
        op: Arm64Sve2PairwiseOp,
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op>{B,T} Zd.<Tn>, Zn.<T>, #<shift>` -- SVE2 shift right narrow by immediate (FEAT_SVE2): each wide source
    /// element is shifted right by `shift` (`1..=esize` of the narrow result) and written narrow. `result_size` is
    /// the narrow result (`.b`/`.h`/`.s`); the source is one wider. `top` writes the odd lanes. See
    /// [`Arm64Sve2NarrowShiftOp`].
    Sve2NarrowingShiftRight {
        op: Arm64Sve2NarrowShiftOp,
        top: bool,
        result_size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        shift: u8,
    },

    /// `SSHLL`/`USHLL{B,T} Zd.<Tw>, Zn.<Tn>, #<shift>` -- SVE2 shift left long (FEAT_SVE2): each narrow `Zn` element
    /// is zero-/sign-extended, shifted left by `shift` (`0..=esize-1`), into the wider result. `src_size` is the
    /// narrow source (`.b`/`.h`/`.s`); the result is one wider. `unsigned` picks `USHLL`, `top` the odd lanes.
    Sve2WideningShiftLeft {
        unsigned: bool,
        top: bool,
        src_size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        shift: u8,
    },

    /// `<op>{B,T} Zd.<Tn>, Zn.<T>` -- SVE2 saturating extract narrow (FEAT_SVE2): each wide `Zn` element is
    /// saturated down into the half-width result. `result_size` is the narrow result element (`.b`/`.h`/`.s`);
    /// the source is one wider. `top` writes the odd result lanes. See [`Arm64Sve2ExtractNarrowOp`].
    Sve2SaturatingExtractNarrow {
        op: Arm64Sve2ExtractNarrowOp,
        top: bool,
        result_size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<Tn>, Zn.<T>, Zm.<T>` -- SVE2 add/subtract narrow high (FEAT_SVE2): the high half of each `size`
    /// add/subtract is written into the half-width result. `size` is the wide source element (`.h`/`.s`/`.d`); the
    /// result is one narrower. See [`Arm64Sve2NarrowHighOp`].
    Sve2NarrowHigh {
        op: Arm64Sve2NarrowHighOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `CADD`/`SQCADD Zdn.<T>, Zdn.<T>, Zm.<T>, #<rot>` -- SVE2 complex integer add (FEAT_SVE2; `saturating` picks
    /// `SQCADD`). `rotation` is `#90` or `#270`. Valid for `.b`/`.h`/`.s`/`.d`.
    Sve2ComplexAdd {
        saturating: bool,
        size: Arm64VectorElement,
        rotation: Arm64ComplexRotation,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `CMLA`/`SQRDCMLAH Zda.<T>, Zn.<T>, Zm.<T>, #<rot>` -- SVE2 complex integer multiply-accumulate (FEAT_SVE2;
    /// `rounding` picks the saturating-rounding `SQRDCMLAH`). `rotation` is one of `#0`/`#90`/`#180`/`#270`. Valid
    /// for `.b`/`.h`/`.s`/`.d`.
    Sve2ComplexMulAdd {
        rounding: bool,
        size: Arm64VectorElement,
        rotation: Arm64ComplexRotation,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `XAR Zdn.<T>, Zdn.<T>, Zm.<T>, #<rotate>` -- SVE2 bitwise exclusive-OR of `Zdn` and `Zm` then rotate each
    /// `size`-element right by `rotate` (`1..=esize`) (FEAT_SVE2; destructive). Valid for `.b`/`.h`/`.s`/`.d`.
    Sve2Xar {
        size: Arm64VectorElement,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        rotate: u8,
    },

    /// `<op> Zd.<Tw>, Zn.<Tn>, Zm.<Tn>` -- SVE2 widening integer op (FEAT_SVE2): the result `size` (`.h`/`.s`/`.d`)
    /// is one element wider than the narrow sources (`.b`/`.h`/`.s`). `B`/`T` variants take even/odd source lanes;
    /// the `ADDW`/`SUBW` family keep `Zn` at the wide size; the `MLAL`/`MLSL` family accumulate into `Zd`. See
    /// [`Arm64Sve2WideningOp`].
    Sve2Widening {
        op: Arm64Sve2WideningOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Zn.<T>, Zm.<T>[index]` -- SVE2 integer multiply / multiply-add / multiply-sub by an indexed
    /// element (FEAT_SVE2). Same per-element index/`Zm` limits as the FP indexed group (`.h`/`.s`: `Zm` in
    /// `Z0..Z7`; `.d`: `Z0..Z15`). Valid for `.h`/`.s`/`.d`. See [`Arm64SveIntIndexedOp`].
    SveIntMulAddIndexed {
        op: Arm64SveIntIndexedOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FDOT Zda.<S|H>, Zn.B, Zm.B` -- SVE FP8 dot product (FEAT_SSVE_FP8DOT2/4): a 4-way FP8->FP32 (`half = false`,
    /// `.s` accumulator) or 2-way FP8->FP16 (`half = true`, `.h`) dot product of 8-bit floating-point lanes.
    SveFp8Dot {
        half: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FDOT Zda.<S|H>, Zn.B, Zm.B[index]` -- SVE FP8 dot product by indexed element. `.s` (`half = false`): `index`
    /// `0..3`; `.h` (`half = true`): `index` `0..7`. `Zm` is `Z0..Z7`.
    SveFp8DotByElement {
        half: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FMLAL{B|T} Zda.H, Zn.B, Zm.B` -- SVE FP8 widening fused multiply-add (FEAT_SSVE_FP8FMA, FP8 -> FP16). `top`
    /// selects `FMLALT` (odd lanes) over `FMLALB`.
    SveFp8MlalLong {
        top: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMMLA Zda.<H|S>, Zn.B, Zm.B` -- SVE FP8 matrix multiply-accumulate (FEAT_F8F16MM for the `.h` accumulator,
    /// FEAT_F8F32MM for `.s`). `half` picks the `.h` (FP8 -> FP16) form. base `0x6420_E000` (`[15:10]`=111000, distinct
    /// from the F32MM/F64MM/BF16 FMMLA at 111001): size`[22]`=half, Zm`[20:16]`, Zn`[9:5]`, Zda`[4:0]`.
    SveFp8Matmul {
        half: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMLALL{BB|BT|TB|TT} Zda.S, Zn.B, Zm.B` -- SVE FP8 4-way widening fused multiply-add (FP8 -> FP32). The two
    /// `top` flags pick the lane group (`first_top` = first letter B/T, `second_top` = second).
    SveFp8MlalLongLong {
        first_top: bool,
        second_top: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FMLAL{B|T} Zda.H, Zn.B, Zm.B[index]` -- SVE FP8 widening MAC (FP8 -> FP16) by indexed element. `Zm` is
    /// `Z0..Z7`, `index` `0..15`. `top` selects `FMLALT`.
    SveFp8MlalLongByElement {
        top: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FMLALL{BB|BT|TB|TT} Zda.S, Zn.B, Zm.B[index]` -- SVE FP8 4-way widening MAC (FP8 -> FP32) by indexed element.
    /// `Zm` is `Z0..Z7`, `index` `0..15`. `first_top`/`second_top` pick the lane group.
    SveFp8MlalLongLongByElement {
        first_top: bool,
        second_top: bool,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `F1CVT`/`F2CVT`/`BF1CVT`/`BF2CVT`(`LT`) `Zd.H, Zn.B` -- SVE FP8 widening convert (FEAT_FP8): widen the 8-bit
    /// floating-point lanes of `Zn.b` to FP16/BFloat16 in `Zd.h`. `top` selects the `LT` (odd/top source lanes) form.
    SveFp8Convert {
        op: Arm64SveFp8ConvertOp,
        top: bool,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `FCVTN`/`FCVTNB`/`BFCVTN Zd.B, {Zn.<h|s>-Zn+1.<h|s>}` -- SVE FP8 narrowing convert (FEAT_FP8): narrow a
    /// two-vector source list (`zn_base` even) to 8-bit floating-point lanes in `Zd.b`. See [`Arm64SveFp8NarrowOp`].
    SveFp8Narrow {
        op: Arm64SveFp8NarrowOp,
        zd: Arm64ScalableVectorRegister,
        zn_base: Arm64ScalableVectorRegister,
    },

    /// `SDOT`/`UDOT Zda.<T>, Zn.<Tb>, Zm.<Tb>` -- SVE 4-way integer dot product accumulating into `Zda` (FEAT_SVE).
    /// `size` is the accumulator element: `.s` (from `.b` sources) or `.d` (from `.h`). `unsigned` picks `UDOT`.
    SveDotProduct {
        unsigned: bool,
        size: Arm64VectorElement,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SDOT`/`UDOT Zda.<T>, Zn.<Tb>, Zm.<Tb>[index]` -- SVE 4-way integer dot product by an indexed 32-/64-bit
    /// group (FEAT_SVE). `size` `.s`: `Zm` in `Z0..Z7`, `index` `0..3`; `.d`: `Zm` in `Z0..Z15`, `index` `0..1`.
    SveDotProductIndexed {
        unsigned: bool,
        size: Arm64VectorElement,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `BFDOT`/`USDOT`/`SUDOT Zda.S, Zn.<Tn>, Zm.<Tn>[index]` -- SVE 4-way dot product by indexed element into a
    /// `.s` accumulator, with a 2-bit `index` (`0..3`) and `Zm` in `Z0..Z7`: `BFDOT` (bf16 `.h` sources, FEAT_BF16),
    /// `USDOT`/`SUDOT` (mixed-sign `.b` sources, FEAT_I8MM). See [`Arm64SveDotIndexedOp`].
    SveDotIndexedMixed {
        op: Arm64SveDotIndexedOp,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `AESE`/`AESD Zdn.B, Zdn.B, Zm.B` / `SM4E Zdn.S, Zdn.S, Zm.S` -- SVE2 destructive cryptographic round
    /// (FEAT_SVE_AES / FEAT_SVE_SM4). The first source is also the destination `Zdn`. See [`Arm64SveCryptoDestructiveOp`].
    Sve2CryptoDestructive {
        op: Arm64SveCryptoDestructiveOp,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `PMOV Pd.<T>, Zn[index]` / `PMOV Zd[index], Pn.<T>` -- SVE2.1 move between a predicate and the per-element
    /// predicate-as-bits view of a vector (FEAT_SVE2p1). `to_vector` picks the `Zd, Pn` direction. `size` is the element
    /// (`.b` has no index; `.h`/`.s`/`.d` index `0..=1/3/7`). The element+index use the interleaved SVE `tsz` field
    /// `[23:17]`: direction`[16]`, to-pred Zn`[9:5]`+Pd`[3:0]`, to-vector Pn`[8:5]`+Zd`[4:0]`. base `0x0500_3800`.
    SvePredVectorMove {
        to_vector: bool,
        size: Arm64VectorElement,
        reg: Arm64ScalableVectorRegister,
        pred: Arm64PredicateRegister,
        index: u8,
    },

    /// `AESEMC`/`AESDIMC { Zdn.B-... }, { Zdn.B-... }, Zm.Q[index]` -- SVE2.1 multi-vector AES single-round encrypt/decrypt
    /// plus (inverse) mix-columns (FEAT_SVE_AES2). Destructive on the two-/four-register `Zdn` group; `Zm` (`z0` to `z7`)
    /// is element-indexed. base `0x4523_E800`: decrypt`[10]`, vgx4`[18]`, index`[20:19]`, Zm`[7:5]`, Zdn`[4:0]`.
    SveAes2MultiVec {
        decrypt: bool,
        four: bool,
        zdn_base: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `AESMC`/`AESIMC Zdn.B, Zdn.B` -- SVE2 AES (inverse) mix-columns, applied in place to `Zdn` (FEAT_SVE_AES).
    /// `inverse` picks `AESIMC`.
    Sve2AesMixColumns {
        inverse: bool,
        zdn: Arm64ScalableVectorRegister,
    },

    /// `SM4EKEY Zd.S, Zn.S, Zm.S` / `RAX1 Zd.D, Zn.D, Zm.D` -- SVE2 constructive cryptographic op (SM4 key
    /// expansion, FEAT_SVE_SM4; or SHA-3 rotate-and-XOR, FEAT_SVE_SHA3). See [`Arm64SveCryptoBinaryOp`].
    Sve2CryptoBinary {
        op: Arm64SveCryptoBinaryOp,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `HISTCNT Zd.<T>, Pg/Z, Zn.<T>, Zm.<T>` -- SVE2 predicated histogram count: for each `.s`/`.d` element, count
    /// how many earlier active elements of `Zn` equal `Zm` (FEAT_SVE2). `pg` is a zeroing governing predicate.
    Sve2HistCnt {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `HISTSEG Zd.B, Zn.B, Zm.B` -- SVE2 histogram of matches in a 16-byte segment (FEAT_SVE2; unpredicated, `.b`).
    Sve2HistSeg {
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `PMULLB`/`PMULLT Zd.Q, Zn.D, Zm.D` -- SVE2 128-bit-result polynomial multiply long (FEAT_SVE_AES): the
    /// `.q`-from-`.d` widening polynomial multiply of the even/odd (`top`) doublewords. (The `.h`-from-`.b` widening
    /// `PMULLB`/`PMULLT` are the regular FEAT_SVE2 forms in [`Self::Sve2Widening`].)
    Sve2PolyMultiply128 {
        top: bool,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SCLAMP`/`UCLAMP`/`FCLAMP Zd.<T>, Zn.<T>, Zm.<T>` -- SVE2.1 clamp each `Zd` element to the inclusive range
    /// `[Zn, Zm]` (FEAT_SVE2p1). `SCLAMP`/`UCLAMP` take `.b`/`.h`/`.s`/`.d`; `FCLAMP` takes `.h`/`.s`/`.d`. See
    /// [`Arm64SveClampOp`].
    SveClamp {
        op: Arm64SveClampOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<T>, Zn.<T>, Zm.<T>[index]` -- SVE floating-point multiply(-accumulate/-subtract) by an indexed
    /// element (FEAT_SVE). `op` is `FMLA`/`FMLS`/`FMUL`. The indexed register `zm` is limited to `Z0..Z7` for
    /// `.h`/`.s` and `Z0..Z15` for `.d`; `index` ranges `0..7` (`.h`), `0..3` (`.s`), `0..1` (`.d`). Valid for
    /// `.h`/`.s`/`.d`. See [`Arm64SveFpIndexedOp`].
    SveFpMulAddIndexed {
        op: Arm64SveFpIndexedOp,
        size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `FCADD Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>, #<rot>` -- SVE predicated complex floating-point add, rotating `Zm`'s
    /// elements by `rotation` (only `#90`/`#270`) before adding (FEAT_SVE; destructive). Valid for `.h`/`.s`/`.d`.
    SveFpComplexAdd {
        size: Arm64VectorElement,
        rotation: Arm64ComplexRotation,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `FCMLA Zda.<T>, Pg/M, Zn.<T>, Zm.<T>, #<rot>` -- SVE predicated complex floating-point multiply-accumulate,
    /// with `rotation` one of `#0`/`#90`/`#180`/`#270` (FEAT_SVE; `Zda` accumulates). Valid for `.h`/`.s`/`.d`.
    SveFpComplexMulAdd {
        size: Arm64VectorElement,
        rotation: Arm64ComplexRotation,
        pg: Arm64PredicateRegister,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `CMLA`/`FCMLA Zda.<T>, Zn.<T>, Zm.<T>[index], #<rot>` -- SVE complex multiply-accumulate by indexed element,
    /// with `rotation` one of `#0`/`#90`/`#180`/`#270`. `fp` picks the floating-point `FCMLA` (FEAT_SVE); the integer
    /// `CMLA` is FEAT_SVE2. Only `.h` (`Zm` in `Z0..Z7`, `index` `0..3`) and `.s` (`Zm` in `Z0..Z15`, `index` `0..1`).
    SveComplexMulAddIndexed {
        fp: bool,
        size: Arm64VectorElement,
        rotation: Arm64ComplexRotation,
        zda: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        index: u8,
    },

    /// `CTERMEQ`/`CTERMNE <R>n, <R>m` -- SVE compare and terminate a scalarized loop: set the condition flags so a
    /// following conditional branch exits when the scalar pair is equal (`CTERMEQ`) / not equal (`CTERMNE`)
    /// (FEAT_SVE). `wide` picks the 64-bit `Xn,Xm` form (else `Wn,Wm`); `ne` picks `CTERMNE`.
    SveCompareTerminate {
        ne: bool,
        wide: bool,
        rn: Arm64GeneralPurposeRegister,
        rm: Arm64GeneralPurposeRegister,
    },

    /// `BRKN Pdm.B, Pg/Z, Pn.B, Pdm.B` -- SVE break propagate: if any `Pg`-active lane of `Pn` is true, leave
    /// `Pdm` unchanged, else zero it (FEAT_SVE; destructive). `set_flags` picks `BRKNS`.
    SveBreakPropagate {
        set_flags: bool,
        pdm: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        pn: Arm64PredicateRegister,
    },

    /// `BRKPA`/`BRKPB Pd.B, Pg/Z, Pn.B, Pm.B` -- SVE break after / before, on a pair: propagate from `Pn` into
    /// `Pm`'s first lane (FEAT_SVE). `before` picks `BRKPB`; `set_flags` picks `BRKPAS`/`BRKPBS`.
    SveBreakPair {
        before: bool,
        set_flags: bool,
        pd: Arm64PredicateRegister,
        pg: Arm64PredicateRegister,
        pn: Arm64PredicateRegister,
        pm: Arm64PredicateRegister,
    },

    /// `RDVL Xd, #imm` -- SVE read the streaming/scalable vector length in bytes times `imm` (`-32..=31`) into a
    /// general-purpose register (FEAT_SVE).
    SveReadVectorLength {
        rd: Arm64GeneralPurposeRegister,
        imm6: i8,
    },

    /// `ADDVL Xd|SP, Xn|SP, #imm` -- SVE add the vector length (in bytes) times `imm` (`-32..=31`) to a
    /// general-purpose / stack-pointer register (FEAT_SVE). Used to allocate scalable stack frames.
    SveAddVectorLength {
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        imm6: i8,
    },

    /// `ADDPL Xd|SP, Xn|SP, #imm` -- SVE add the predicate length (vector length / 8, in bytes) times `imm`
    /// (`-32..=31`) to a general-purpose / stack-pointer register (FEAT_SVE).
    SveAddPredicateLength {
        rd: Arm64GeneralPurposeRegister,
        rn: Arm64GeneralPurposeRegister,
        imm6: i8,
    },

    /// `EXT Zdn.B, Zdn.B, Zm.B, #imm` -- extract a byte-window from the concatenation `Zdn:Zm` at offset `imm`
    /// (`0..=255`), destructive (FEAT_SVE).
    SveExt {
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
        imm8: u8,
    },

    /// `SPLICE Zdn.<T>, Pg, Zdn.<T>, Zm.<T>` -- splice the active elements of `Zdn` then fill from `Zm`, destructive
    /// (FEAT_SVE). `pg` is `P0..P7`.
    SveSplice {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zdn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `COMPACT Zd.<T>, Pg, Zn.<T>` -- pack the active elements of `Zn` into the low lanes of `Zd` (FEAT_SVE,
    /// `.s`/`.d` only). `pg` is `P0..P7`.
    SveCompact {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `RBIT Zd.<T>, Pg/M, Zn.<T>` -- reverse the bits in each element, predicated (FEAT_SVE). `pg` is `P0..P7`.
    SveRbit {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `RBIT Zd.<T>, Pg/Z, Zn.<T>` -- the **zeroing**-predicate form of [`Self::SveRbit`] (FEAT_SVE2p2). Same
    /// `0x0527_8000` frame with bit `[13]` set (`[15:13]` `100` -> `101`).
    SveRbitZeroing {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `SEL Zd.<T>, Pg, Zn.<T>, Zm.<T>` -- SVE element-wise select: `Zd[i] = Pg[i] ? Zn[i] : Zm[i]` (FEAT_SVE).
    /// `pg` may be `P0..P15` (a 4-bit field).
    SveSelect {
        size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
        zm: Arm64ScalableVectorRegister,
    },

    /// `SUNPK`/`UUNPK {LO,HI} Zd.<Tw>, Zn.<Tn>` -- SVE unpack the low/high half of `Zn` into the wider `Zd`,
    /// sign-/zero-extending (FEAT_SVE). `dest_size` is the wide element (`.h`/`.s`/`.d`); the source is one size
    /// narrower.
    SveUnpack {
        signed: bool,
        high: bool,
        dest_size: Arm64VectorElement,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zdn.<T>, Zdn.<T>, #imm` (or `DUPM Zd.<T>, #imm`) -- SVE bitwise logical with a repeating-bitmask
    /// immediate (FEAT_SVE). `imm` is the full 64-bit repeating pattern; the element size is derived from it. See
    /// [`Arm64SveBitwiseImmOp`].
    SveBitwiseImmediate {
        op: Arm64SveBitwiseImmOp,
        zdn: Arm64ScalableVectorRegister,
        imm: u64,
    },

    /// `<op> Zd.<Td>, Pg/M, Zn.<Tn>` -- SVE predicated floating-point convert (FEAT_SVE): change of FP precision
    /// (`FCVT`), FP-to-integer (`FCVTZS`/`FCVTZU`), or integer-to-FP (`SCVTF`/`UCVTF`). `pg` is `P0..P7`. Only the
    /// element-size pairs listed in the architecture are legal; an illegal `(kind, dest, src)` triple fails to
    /// encode. See [`Arm64SveFpConvertKind`].
    SveFpConvert {
        kind: Arm64SveFpConvertKind,
        dest_size: Arm64VectorElement,
        src_size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },

    /// `<op> Zd.<Td>, Pg/Z, Zn.<Tn>` -- the **zeroing**-predicate form of [`Self::SveFpConvert`] (FEAT_SVE2p2):
    /// `FCVT`/`FCVTZS`/`FCVTZU`/`SCVTF`/`UCVTF`. Encoded in the relocated `0x64..` frame; each `(kind, dest, src)`
    /// triple is a fully-fixed `[31:13]` discriminant (see `SVE_FP_CONVERTS_ZEROING`).
    SveFpConvertZeroing {
        kind: Arm64SveFpConvertKind,
        dest_size: Arm64VectorElement,
        src_size: Arm64VectorElement,
        pg: Arm64PredicateRegister,
        zd: Arm64ScalableVectorRegister,
        zn: Arm64ScalableVectorRegister,
    },
}
