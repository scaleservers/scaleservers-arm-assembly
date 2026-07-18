// Copyright (c) Scaleservers LLC

/// Orthogonal AArch64 architecture-extension flags. These gate instructions whose availability is not
/// captured by the coarse ISA version alone (mirroring how the 32-bit library models `ArmCpuFeature`). The integer base set
/// requires none of these.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Arm64CpuFeature {
    FloatingPoint, // the scalar FP unit (FP/half/single/double) -- H0-31 / S0-31 / D0-31
    Fp16, // ARMv8.2 half-precision (FEAT_FP16) -- the f16 scalar data-processing/convert forms
    AdvancedSimd, // the Advanced SIMD (NEON) vector unit -- V0-31
    DotProd, // ARMv8.2 dot product (FEAT_DotProd) -- SDOT / UDOT (vector + by-element)
    Rdm,  // ARMv8.1 rounding double multiply accumulate (FEAT_RDM) -- SQRDMLAH / SQRDMLSH
    Fcma, // ARMv8.3 floating-point complex (FEAT_FCMA) -- FCMLA / FCADD
    Crc,  // ARMv8.0 CRC32 (FEAT_CRC32, mandatory from ARMv8.1) -- CRC32B/H/W/X + CRC32C*
    Mte,  // ARMv8.5 memory tagging (FEAT_MTE2) -- ADDG/SUBG/IRG/GMI/SUBP/STG/LDG/...
    I8mm, // ARMv8.6 int8 matrix multiply (FEAT_I8MM) -- SMMLA / UMMLA / USMMLA / USDOT / SUDOT
    Bf16, // ARMv8.6 BFloat16 (FEAT_BF16) -- BFDOT / BFMMLA / BFMLALB / BFMLALT / BFCVT(N)
    Crypto, // the ARMv8 cryptography extension (AES / SHA1 / SHA256)
    Sha512, // ARMv8.2 SHA-512 acceleration (FEAT_SHA512) -- SHA512H/H2/SU0/SU1
    Sha3, // ARMv8.2 SHA-3 acceleration (FEAT_SHA3) -- EOR3 / BCAX / RAX1 / XAR
    Sm3,  // ARMv8.2 SM3 acceleration (FEAT_SM3) -- SM3SS1 / SM3TT* / SM3PARTW*
    Sm4,  // ARMv8.2 SM4 acceleration (FEAT_SM4) -- SM4E / SM4EKEY
    Lse,  // ARMv8.1 Large System Extensions (atomic LDADD/SWP/CAS/...)
    Jscvt, // ARMv8.3 JavaScript conversion (FEAT_JSCVT) -- FJCVTZS
    Pauth, // ARMv8.3 pointer authentication (PACIA/AUTIA/...)
    FlagM, // ARMv8.4 flag manipulation (FEAT_FlagM) -- RMIF / SETF8 / SETF16 / CFINV
    FlagM2, // ARMv8.5 flag manipulation 2 (FEAT_FlagM2) -- XAFLAG / AXFLAG
    Rcpc, // ARMv8.3 release-consistent processor consistent (FEAT_LRCPC) -- LDAPR{B,H}
    Rcpc2, // ARMv8.4 RCpc with immediate offset (FEAT_LRCPC2) -- LDAPUR* / STLUR*
    Sve,  // the Scalable Vector Extension (Z0-31 / P0-15)
    Sve2, // SVE2 (FEAT_SVE2) -- the second-generation SVE integer/DSP surface
    SveBitPerm, // SVE bit-permute (FEAT_SVE_BitPerm) -- BEXT / BDEP / BGRP
    F32mm, // ARMv8.6 32-bit FP matrix multiply (FEAT_F32MM) -- the SVE FMMLA single-precision form
    F64mm, // ARMv8.6 64-bit FP matrix multiply (FEAT_F64MM) -- the SVE FMMLA double-precision form
    SveAes, // SVE AES (FEAT_SVE_AES) -- the SVE2 AESE/AESD/AESMC/AESIMC and PMULLB/PMULLT-128 forms
    SveAes2, // SVE AES2 (FEAT_SVE_AES2) -- the SVE2.1 multi-vector AESEMC/AESDIMC forms
    SveSm4, // SVE SM4 (FEAT_SVE_SM4) -- the SVE2 SM4E/SM4EKEY forms
    SveSha3, // SVE SHA-3 (FEAT_SVE_SHA3) -- the SVE2 RAX1 form
    Sve2p1, // SVE2.1 (FEAT_SVE2p1) -- the SVE2.1 additions (SCLAMP/UCLAMP/FCLAMP, ...); also enabled by FEAT_SME2
    Sme, // the Scalable Matrix Extension (FEAT_SME) -- the ZA tile register, streaming mode, outer products
    SmeF64F64, // SME double-precision outer product (FEAT_SME_F64F64) -- FMOPA/FMOPS into a .d ZA tile
    SmeI16I64, // SME 16-bit-to-64-bit integer outer product (FEAT_SME_I16I64) -- the .d-accumulator integer MOPA/MOPS
    Mops, // ARMv8.8/v9.3 memory-copy/set operations (FEAT_MOPS) -- CPYF*/CPY*/SET* memcpy/memset
    Sme2, // SME2 (FEAT_SME2) -- the multi-vector ZA surface (multi-vector MOVA/loads, ZA dot/MLA, LUTI, ...)
    Sme2p2, // SME2.2 (FEAT_SME2p2, Armv9.6) -- the multi-vector FMUL (and the SVE2p2 predicated forms in streaming mode)
    SveB16B16, // SVE BF16 arithmetic (FEAT_SVE_B16B16) -- BFADD/BFSUB/BFMUL/BFMAX/BFMIN/BFCLAMP/BFMLA on .h BF16
    SmeB16B16, // SME2 BF16 arithmetic (FEAT_SME_B16B16) -- the multi-vector BFCLAMP/BFMAX/BFMIN and BFMOPA on .h BF16
    Cssc, // ARMv8.9/9.4 common short sequence compression (FEAT_CSSC) -- ABS/CNT/CTZ + SMAX/SMIN/UMAX/UMIN (reg+imm)
    Hbc,  // ARMv8.8/9.3 hinted conditional branch (FEAT_HBC) -- BC.cond
    Lse128, // ARMv9.4 128-bit atomics (FEAT_LSE128) -- SWPP/LDCLRP/LDSETP (+ A/L/AL ordering)
    Rcpc3, // ARMv8.9/9.4 release-consistent processor-consistent v3 (FEAT_LRCPC3) -- LDIAPP/STILP ordered pair
    Rprfm, // ARMv8.9/9.4 range prefetch memory (FEAT_RPRFM) -- RPRFM
    Sysreg128, // ARMv9.4 128-bit system registers (FEAT_SYSREG128 / FEAT_D128) -- MRRS/MSRR
    SmeF16F16, // SME2 FP16 arithmetic (FEAT_SME_F16F16) -- FMOPA/FMOPS into a .h ZA tile
    Gcs, // ARMv9.4 guarded control stack (FEAT_GCS) -- GCSSTR/GCSSTTR/GCSPUSHM/GCSPOPM/GCSSS1/GCSSS2/GCSB
    Sb,  // ARMv8.0 speculation barrier (FEAT_SB, mandatory from ARMv8.5) -- the SB barrier
    Brbe, // ARMv9.0 branch record buffer (FEAT_BRBE) -- BRB IALL / BRB INJ
    Ite, // ARMv9.0 instrumentation trace extension (FEAT_ITE) -- TRCIT
    Frintts, // ARMv8.5 round-to-integer-N (FEAT_FRINTTS) -- FRINT32X/Z, FRINT64X/Z (scalar + vector)
    Fhm, // ARMv8.2 FP16 fused-multiply-long (FEAT_FHM) -- the NEON FMLAL/FMLAL2/FMLSL/FMLSL2 .4s<-.4h forms
    Ls64, // ARMv8.7 atomic 64-byte access (FEAT_LS64/LS64_V) -- LD64B/ST64B/ST64BV/ST64BV0
    The, // ARMv8.8/9.3 translation hardening (FEAT_THE) -- the RCW/RCWS read-check-write atomics
    D128, // ARMv9.4 128-bit translation/atomics (FEAT_D128) -- the 128-bit RCW pair forms (RCWCASP/...P)
    Cpa,  // ARMv9.5 checked pointer arithmetic (FEAT_CPA) -- ADDPT/SUBPT/MADDPT/MSUBPT
    FaMinMax, // ARMv9.5 floating-point absolute min/max (FEAT_FAMINMAX) -- FAMAX/FAMIN (NEON + SVE)
    Fp8, // ARMv9.2 8-bit floating-point (FEAT_FP8) -- the FP8 convert-long widen (F1CVTL/F2CVTL/BF1CVTL/BF2CVTL)
    Fp8Fma, // ARMv9.2 FP8 fused multiply-add (FEAT_FP8FMA) -- the widening FMLALB/FMLALT + 4-way FMLALLBB/BT/TB/TT
    Fp8Dot, // ARMv9.2 FP8 dot product (FEAT_FP8DOT2/FP8DOT4) -- the FP8->FP16/FP32 FDOT (2-way / 4-way)
    SsveFp8Dot, // ARMv9.2 SVE/streaming FP8 dot product (FEAT_SSVE_FP8DOT2/FP8DOT4) -- the SVE `FDOT Z.s/.h, Z.b, Z.b`
    SsveFp8Fma, // ARMv9.2 SVE/streaming FP8 fused multiply-add (FEAT_SSVE_FP8FMA) -- SVE FMLALB/T + FMLALLBB/BT/TB/TT into Z
    F8f16mm,    // FP8-to-FP16 matrix multiply (FEAT_F8F16MM) -- the SVE FMMLA Z.h, Z.b, Z.b
    F8f32mm,    // FP8-to-FP32 matrix multiply (FEAT_F8F32MM) -- the SVE FMMLA Z.s, Z.b, Z.b
    F16mm,      // FP16 matrix multiply (FEAT_F16MM, SVE2p2) -- the SVE FMMLA Z.h, Z.h, Z.h
    SmeF8F16, // SME2 FP8-to-FP16 (FEAT_SME_F8F16) -- the FP8 FMOPA/FDOT/FVDOT/FMLAL accumulating into a .h ZA tile
    SmeF8F32, // SME2 FP8-to-FP32 (FEAT_SME_F8F32) -- the FP8 FMOPA/FDOT accumulating into a .s ZA tile
    Sme2p1, // SME2.1 (FEAT_SME2p1) -- the SME2.1 additions (MOVAZ move-and-zero, ZERO of a ZA vector group, ...)
    SmeLutv2, // SME lookup table v2 (FEAT_SME_LUTv2) -- MOVT (vector -> ZT0) + the LUTI4 4-register strided forms
    PauthLr, // ARMv9.5 pointer authentication, link-register variant (FEAT_PAuth_LR) -- PACIASPPC/PACIBSPPC/PACM/...
    SveBfscale, // ARMv9.6 SVE BFloat16 scale (FEAT_SVE_BFSCALE) -- the BFSCALE predicated BF16 scale-by-2^int
    Sve2p2, // ARMv9.6 SVE2.2 (FEAT_SVE2p2) -- the SVE2.2 additions (FIRSTP/LASTP predicate-extract, EXPAND, ...)
    Lsui, // ARMv9.6 unprivileged LSE (FEAT_LSUI) -- LDTXR/STTXR + unprivileged atomics/CAS (SWPT/LDTADD/CAST/...)
    SmeTmop, // ARMv9.6 SME2 sparse outer product (FEAT_SME_TMOP) -- STMOPA/UTMOPA/USTMOPA/SUTMOPA/BFTMOPA/FTMOPA
    SmeMop4, // ARMv9.6 SME2 quarter-tile outer product (FEAT_SME_MOP4) -- SMOP4A/UMOP4A/SUMOP4A/USMOP4A/FMOP4A/BFMOP4A (+S)
    Cmpbr, // ARMv9.6 compare and branch (FEAT_CMPBR) -- CB<cc>/CBB<cc>/CBH<cc> register + CB<cc> immediate compare-and-branch
    Pcdphint, // ARMv9.6 producer/consumer data-placement hint (FEAT_PCDPHINT) -- STSHH (experimental: LLVM-20-only oracle)
    Lsfe, // ARMv9.6 large-system float extension (FEAT_LSFE) -- atomic float LDF*/STF* (experimental: LLVM-20-only oracle)
    Lut,  // ARMv9.6 lookup-table (FEAT_LUT) -- the NEON LUTI2/LUTI4 table-vector lookups
    Fprcvt, // ARMv9.6 FP convert to/from int in an FP register (FEAT_FPRCVT) -- the cross-size FCVT*/SCVTF/UCVTF
}
