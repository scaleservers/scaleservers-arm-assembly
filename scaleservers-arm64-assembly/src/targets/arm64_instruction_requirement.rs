// Copyright (c) Scaleservers LLC

use crate::targets::{Arm64CpuFeature, Arm64IsaVersion};

/// What an instruction (or one of its forms) needs from the target to be emittable: a minimum ISA version
/// and zero or more architecture-extension features. `required_features` is a `&'static` slice so
/// requirement values are allocation-free (call sites pass promoted constant slices, e.g.
/// `&[Arm64CpuFeature::FloatingPoint]`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arm64InstructionRequirement {
    pub min_isa_version: Arm64IsaVersion,
    pub required_features: &'static [Arm64CpuFeature],
}

impl Arm64InstructionRequirement {
    pub const fn new(
        min_isa_version: Arm64IsaVersion,
        required_features: &'static [Arm64CpuFeature],
    ) -> Self {
        Self {
            min_isa_version,
            required_features,
        }
    }

    /// The universally-available baseline: ARMv8.0-A, no extension features. The integer/branch/load-store
    /// base set is at this level.
    pub const fn baseline() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8A,
            required_features: &[],
        }
    }

    /// ARMv8.0-A with the scalar **floating-point** unit -- the level the scalar FP data-processing surface
    /// (FADD/FSUB/..., FNEG, FCVT, FCMP, the int<->FP conversions, FMOV) needs. A profile without
    /// [`Arm64CpuFeature::FloatingPoint`] (e.g. `armv8a_integer_only`) refuses these forms.
    pub const fn floating_point() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8A,
            required_features: &[Arm64CpuFeature::FloatingPoint],
        }
    }

    /// ARMv8.0-A with the **Advanced SIMD** (NEON) unit -- the level the vector data-processing forms need (the
    /// NEON register file is `V0`-`V31`, shared with scalar FP).
    pub const fn advanced_simd() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8A,
            required_features: &[Arm64CpuFeature::AdvancedSimd],
        }
    }

    /// ARMv8.2-A with the **dot product** extension -- the level `SDOT`/`UDOT` (4-way 8-bit dot into a 32-bit
    /// accumulator, vector and by-element) need.
    pub const fn dot_product() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_2A,
            required_features: &[Arm64CpuFeature::DotProd],
        }
    }

    /// ARMv8.1-A with the **rounding double multiply** extension -- the level `SQRDMLAH`/`SQRDMLSH` (vector and
    /// by-element) need.
    pub const fn rdm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_1A,
            required_features: &[Arm64CpuFeature::Rdm],
        }
    }

    /// ARMv8.3-A with the **floating-point complex** extension -- the level `FCMLA`/`FCADD` (complex multiply-add
    /// and add, with a rotation) need.
    pub const fn fcma() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_3A,
            required_features: &[Arm64CpuFeature::Fcma],
        }
    }

    /// ARMv8.5-A with **memory tagging** (FEAT_MTE) -- `ADDG`/`SUBG`/`IRG`/`GMI`/`SUBP`/`STG`/`LDG`/...
    pub const fn mte() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_5A,
            required_features: &[Arm64CpuFeature::Mte],
        }
    }

    /// ARMv8.3-A with **pointer authentication** (FEAT_PAuth) -- `PACIA`/`AUTIA`/`XPACI`/... and the PAC loads.
    pub const fn pauth() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_3A,
            required_features: &[Arm64CpuFeature::Pauth],
        }
    }

    /// ARMv8.0-A with the **CRC32** extension (FEAT_CRC32, mandatory from ARMv8.1) -- `CRC32B`/`H`/`W`/`X` and the
    /// `CRC32C*` (Castagnoli) variants.
    pub const fn crc() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8A,
            required_features: &[Arm64CpuFeature::Crc],
        }
    }

    /// ARMv8.6-A with the **int8 matrix multiply** extension -- the level `SMMLA`/`UMMLA`/`USMMLA` (8-bit matrix
    /// product) and `USDOT`/`SUDOT` (mixed-sign dot product) need.
    pub const fn i8mm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_6A,
            required_features: &[Arm64CpuFeature::I8mm],
        }
    }

    /// ARMv8.6-A with the **BFloat16** extension -- the level `BFDOT`/`BFMMLA`/`BFMLALB`/`BFMLALT` (bf16
    /// multiply-accumulate into f32) and `BFCVT`/`BFCVTN` (f32 -> bf16 narrowing) need.
    pub const fn bf16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_6A,
            required_features: &[Arm64CpuFeature::Bf16],
        }
    }

    /// ARMv8.0-A with the **cryptography** extension -- the level the AES (`AESE`/`AESD`/`AESMC`/`AESIMC`) and
    /// SHA1/SHA256 acceleration instructions need.
    pub const fn crypto() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8A,
            required_features: &[Arm64CpuFeature::Crypto],
        }
    }

    /// ARMv8.2-A with the **SHA-512** acceleration extension (FEAT_SHA512) -- `SHA512H`/`SHA512H2`/`SHA512SU0`/
    /// `SHA512SU1`.
    pub const fn sha512() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_2A,
            required_features: &[Arm64CpuFeature::Sha512],
        }
    }

    /// ARMv8.2-A with the **SHA-3** acceleration extension (FEAT_SHA3) -- `EOR3`/`BCAX`/`RAX1`/`XAR`.
    pub const fn sha3() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_2A,
            required_features: &[Arm64CpuFeature::Sha3],
        }
    }

    /// ARMv8.2-A with the **SM3** acceleration extension (FEAT_SM3) -- `SM3SS1`/`SM3TT*`/`SM3PARTW*`.
    pub const fn sm3() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_2A,
            required_features: &[Arm64CpuFeature::Sm3],
        }
    }

    /// ARMv8.2-A with the **SM4** acceleration extension (FEAT_SM4) -- `SM4E`/`SM4EKEY`.
    pub const fn sm4() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_2A,
            required_features: &[Arm64CpuFeature::Sm4],
        }
    }

    /// ARMv8.2-A with **half-precision** floating-point -- the level `FCVT` to/from the `H` (f16) format needs.
    /// The single<->double converts and all other scalar FP stay at [`Self::floating_point`]; only the f16
    /// convert forms require this.
    pub const fn fp16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_2A,
            required_features: &[Arm64CpuFeature::Fp16],
        }
    }

    /// ARMv8.3-A with the **JavaScript conversion** extension -- the level `FJCVTZS` (double->int32, JS rounding)
    /// needs.
    pub const fn jscvt() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_3A,
            required_features: &[Arm64CpuFeature::Jscvt],
        }
    }

    /// ARMv8.1-A with the **Large System Extensions** -- the level the single-instruction atomic
    /// read-modify-writes need (`LDADD`/`LDCLR`/`LDEOR`/`LDSET`/`LD{S,U}{MAX,MIN}`, `SWP`, `CAS`). The
    /// ARMv8.0 load/store-exclusive and acquire/release ordinary forms (`LDXR`/`STXR`/`LDAR`/`STLR`) stay at
    /// [`Self::baseline`]; only the LSE single-op atomics require this.
    pub const fn lse() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_1A,
            required_features: &[Arm64CpuFeature::Lse],
        }
    }

    /// ARMv8.4-A with the **flag-manipulation** extension (FEAT_FlagM) -- `RMIF`/`SETF8`/`SETF16`/`CFINV`.
    pub const fn flagm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_4A,
            required_features: &[Arm64CpuFeature::FlagM],
        }
    }

    /// ARMv8.5-A with the **flag-manipulation 2** extension (FEAT_FlagM2) -- `XAFLAG`/`AXFLAG`.
    pub const fn flagm2() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_5A,
            required_features: &[Arm64CpuFeature::FlagM2],
        }
    }

    /// ARMv8.3-A with **release-consistent** atomics (FEAT_LRCPC) -- the RCpc load-acquire `LDAPR`/`LDAPRB`/`LDAPRH`.
    pub const fn rcpc() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_3A,
            required_features: &[Arm64CpuFeature::Rcpc],
        }
    }

    /// ARMv8.4-A with **RCpc immediate-offset** loads/stores (FEAT_LRCPC2) -- `LDAPUR*`/`STLUR*`.
    pub const fn rcpc2() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_4A,
            required_features: &[Arm64CpuFeature::Rcpc2],
        }
    }

    /// The **Scalable Vector Extension** (FEAT_SVE) -- the Z/P register files and the SVE instruction set. Optional
    /// from ARMv8.2-A, mandatory from ARMv9.0-A.
    pub const fn sve() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_2A,
            required_features: &[Arm64CpuFeature::Sve],
        }
    }

    /// **SVE2** (FEAT_SVE2) -- the second-generation SVE integer/DSP instruction surface, baseline in ARMv9.0-A.
    pub const fn sve2() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2],
        }
    }

    /// **SVE bit-permute** (FEAT_SVE_BitPerm) -- the optional `BEXT`/`BDEP`/`BGRP` extension over SVE2.
    pub const fn sve_bitperm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2, Arm64CpuFeature::SveBitPerm],
        }
    }

    /// **SVE int8 matrix-multiply** (FEAT_SVE + FEAT_I8MM) -- the SVE `USDOT`/`SUDOT`/`USMMLA` mixed-sign dot/matmul.
    pub const fn sve_i8mm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_6A,
            required_features: &[Arm64CpuFeature::Sve, Arm64CpuFeature::I8mm],
        }
    }

    /// **SVE BFloat16** (FEAT_SVE + FEAT_BF16) -- the SVE `BFCVT`/`BFDOT`/`BFMMLA`/`BFMLAL*` BFloat16 instructions.
    pub const fn sve_bf16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_6A,
            required_features: &[Arm64CpuFeature::Sve, Arm64CpuFeature::Bf16],
        }
    }

    /// **SVE BF16 arithmetic** (FEAT_SVE2 + FEAT_SVE_B16B16) -- the SVE2.1 `BFADD`/`BFSUB`/`BFMUL`/`BFMAX`/`BFMIN`
    /// (predicated + unpredicated), `BFCLAMP`, and `BFMLA`/`BFMLS` on `.h` BFloat16 (also enabled by FEAT_SME2).
    pub const fn sve_b16b16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sve,
                Arm64CpuFeature::Sve2,
                Arm64CpuFeature::SveB16B16,
            ],
        }
    }

    /// **SME2 BF16 arithmetic** (FEAT_SME2 + FEAT_SME_B16B16) -- the SME2 multi-vector `BFCLAMP`/`BFMAX`/`BFMIN` (and
    /// `BFMOPA`) on `.h` BFloat16 (the `size==00` slot of the FP multi-vector clamp/min/max forms).
    pub const fn sme_b16b16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeB16B16,
            ],
        }
    }

    /// **Common short sequence compression** (FEAT_CSSC, ARMv8.9/9.4) -- the scalar `ABS`/`CNT`/`CTZ` and the
    /// `SMAX`/`SMIN`/`UMAX`/`UMIN` integer min/max (register and 8-bit-immediate forms).
    pub const fn cssc() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Cssc],
        }
    }

    /// **Hinted conditional branch** (FEAT_HBC, ARMv8.8/9.3) -- the `BC.cond` consistent-branch hint form of `B.cond`.
    pub const fn hbc() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Hbc],
        }
    }

    /// **128-bit atomics** (FEAT_LSE128, ARMv9.4) -- the `SWPP`/`LDCLRP`/`LDSETP` 128-bit atomic memory operations on
    /// a 64-bit register pair (with the `A`/`L`/`AL` acquire/release ordering variants).
    pub const fn lse128() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Lse128],
        }
    }

    /// **RCpc3 ordered access** (FEAT_LRCPC3, ARMv8.9/9.4) -- the `LDIAPP`/`STILP` release-consistent ordered
    /// load/store register-pair forms.
    pub const fn rcpc3() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Rcpc3],
        }
    }

    /// **Range prefetch** (FEAT_RPRFM, ARMv8.9/9.4) -- the `RPRFM` range-prefetch-memory hint.
    pub const fn rprfm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Rprfm],
        }
    }

    /// **128-bit system registers** (FEAT_SYSREG128 / FEAT_D128, ARMv9.4) -- the `MRRS`/`MSRR` 128-bit system-register
    /// read/write into a 64-bit register pair.
    pub const fn sysreg128() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sysreg128],
        }
    }

    /// **SME2 FP16 arithmetic** (FEAT_SME2 + FEAT_SME_F16F16) -- the SME2 `FMOPA`/`FMOPS` half-precision outer product
    /// accumulating into a `.h` ZA tile.
    pub const fn sme_f16f16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeF16F16,
            ],
        }
    }

    /// **SME2 FP8-to-FP16** (FEAT_SME2 + FEAT_SME_F8F16) -- the FP8 (`.b` source) `FMOPA`/`FDOT`/`FVDOT`/`FMLAL`
    /// accumulating into a `.h` ZA tile.
    pub const fn sme_f8f16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeF8F16,
            ],
        }
    }

    /// **SME2 FP8-to-FP32** (FEAT_SME2 + FEAT_SME_F8F32) -- the FP8 (`.b` source) `FMOPA`/`FDOT` accumulating into a
    /// `.s` ZA tile.
    pub const fn sme_f8f32() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeF8F32,
            ],
        }
    }

    /// **SME2.1** (FEAT_SME2p1) -- the SME2.1 additions such as `MOVAZ` (move-and-zero from `ZA`).
    pub const fn sme2p1() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::Sme2p1,
            ],
        }
    }

    /// **SME lookup table v2** (FEAT_SME_LUTv2) -- the `MOVT` vector-to-`ZT0` move and the LUTI4 4-register strided forms.
    pub const fn sme_lutv2() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeLutv2,
            ],
        }
    }

    /// **Pointer authentication, link-register variant** (FEAT_PAuth_LR, ARMv9.5) -- `PACIASPPC`/`PACIBSPPC`/`PACM`/...
    pub const fn pauth_lr() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Pauth, Arm64CpuFeature::PauthLr],
        }
    }

    /// **SVE BFloat16 scale** (FEAT_SVE_BFSCALE, ARMv9.6) -- the SVE predicated `BFSCALE` (BF16 scale-by-2^int).
    pub const fn sve_bfscale() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sve,
                Arm64CpuFeature::Sve2,
                Arm64CpuFeature::SveBfscale,
            ],
        }
    }

    /// **SVE2.2** (FEAT_SVE2p2, ARMv9.6) -- the SVE2.2 additions such as `FIRSTP`/`LASTP` (predicate-extract to a GPR)
    /// and `EXPAND`.
    pub const fn sve2p2() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sve,
                Arm64CpuFeature::Sve2,
                Arm64CpuFeature::Sve2p2,
            ],
        }
    }

    /// **Unprivileged LSE** (FEAT_LSUI, ARMv9.6) -- the unprivileged `LDTXR`/`STTXR` exclusives and the unprivileged
    /// LSE atomics/CAS (`SWPT`/`LDTADD`/`CAST`/...).
    pub const fn lsui() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Lse, Arm64CpuFeature::Lsui],
        }
    }

    /// **SME2 sparse outer product** (FEAT_SME_TMOP, ARMv9.6) -- the integer/bf16/f16 `STMOPA`/`UTMOPA`/`USTMOPA`/
    /// `SUTMOPA`/`BFTMOPA`/`FTMOPA` quarter-tile sparse outer products.
    pub const fn sme_tmop() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeTmop,
            ],
        }
    }

    /// **SME2 sparse outer product into a `.h` (FP16) tile** (FEAT_SME_TMOP + FEAT_SME_F16F16) -- `FTMOPA ZAda.H, ...`.
    pub const fn sme_tmop_f16f16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeTmop,
                Arm64CpuFeature::SmeF16F16,
            ],
        }
    }

    /// **SME2 FP8 sparse outer product into a `.s` (FP32) tile** (FEAT_SME_TMOP + FEAT_SME_F8F32) -- `FTMOPA ZAda.S, Zn.B, ...`.
    pub const fn sme_tmop_f8f32() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeTmop,
                Arm64CpuFeature::SmeF8F32,
            ],
        }
    }

    /// **SME2 FP8 sparse outer product into a `.h` (FP16) tile** (FEAT_SME_TMOP + FEAT_SME_F8F16) -- `FTMOPA ZAda.H, Zn.B, ...`.
    pub const fn sme_tmop_f8f16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeTmop,
                Arm64CpuFeature::SmeF8F16,
            ],
        }
    }

    /// **SME2 quarter-tile outer product** (FEAT_SME_MOP4, ARMv9.6) -- the `.s`-accumulator integer/f32/bf16/f16
    /// `SMOP4A`/`UMOP4A`/`SUMOP4A`/`USMOP4A`/`FMOP4A`/`BFMOP4A` (and the `*S` subtract forms), with single or
    /// 2-vector-list `Zn`/`Zm` operands.
    pub const fn sme_mop4() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeMop4,
            ],
        }
    }

    /// **Compare and branch** (FEAT_CMPBR, ARMv9.6) -- the `CB<cc>`/`CBB<cc>`/`CBH<cc>` register and `CB<cc> #imm`
    /// immediate compare-and-branch instructions.
    pub const fn cmpbr() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Cmpbr],
        }
    }

    /// **Producer/consumer data-placement hint** (FEAT_PCDPHINT, ARMv9.6) -- the `STSHH` cache-stash hint. Only the
    /// `experimental` feature surfaces the instruction (single-oracle: LLVM-20 + DDI0487).
    #[cfg(feature = "experimental")]
    pub const fn pcdphint() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Pcdphint],
        }
    }

    /// **LSFE** (FEAT_LSFE) -- atomic floating-point memory ops `LDF*`/`STF*`. Experimental (LLVM-20-only oracle).
    #[cfg(feature = "experimental")]
    pub const fn lsfe() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Lsfe],
        }
    }

    /// **SME2 quarter-tile integer outer product into a `.d` (S64) tile** (FEAT_SME_MOP4 + FEAT_SME_I16I64) -- the
    /// `.h`-source `SMOP4A`/`UMOP4A`/`SUMOP4A`/`USMOP4A` (+`*S`) accumulating into a 64-bit `ZA` tile.
    pub const fn sme_mop4_i16i64() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeMop4,
                Arm64CpuFeature::SmeI16I64,
            ],
        }
    }

    /// **SME2 quarter-tile FP outer product into a `.d` (F64) tile** (FEAT_SME_MOP4 + FEAT_SME_F64F64) -- the
    /// `.d`-source `FMOP4A`/`FMOP4S` accumulating into a 64-bit `ZA` tile.
    pub const fn sme_mop4_f64f64() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeMop4,
                Arm64CpuFeature::SmeF64F64,
            ],
        }
    }

    /// **SME2 quarter-tile FP16 outer product into a `.h` (FP16) tile** (FEAT_SME_MOP4 + FEAT_SME_F16F16) -- the
    /// f16-source `FMOP4A`/`FMOP4S ZAda.H`.
    pub const fn sme_mop4_f16f16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeMop4,
                Arm64CpuFeature::SmeF16F16,
            ],
        }
    }

    /// **SME2 quarter-tile BF16 outer product into a `.h` (BF16) tile** (FEAT_SME_MOP4 + FEAT_SME_B16B16) -- the
    /// bf16-source `BFMOP4A`/`BFMOP4S ZAda.H`.
    pub const fn sme_mop4_b16b16() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeMop4,
                Arm64CpuFeature::SmeB16B16,
            ],
        }
    }

    /// **Guarded control stack** (FEAT_GCS, ARMv9.4) -- the `GCSSTR`/`GCSSTTR` GCS stores, the `GCSPUSHM`/`GCSPOPM`/
    /// `GCSSS1`/`GCSSS2` GCS register operations, and the `GCSB DSYNC` barrier.
    pub const fn gcs() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Gcs],
        }
    }

    /// The **speculation barrier** (FEAT_SB, optional from ARMv8.0-A, mandatory from ARMv8.5-A) -- the `SB`
    /// instruction (a stronger speculation boundary than the `CSDB` hint).
    pub const fn sb() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8A,
            required_features: &[Arm64CpuFeature::Sb],
        }
    }

    /// The **branch record buffer** extension (FEAT_BRBE, ARMv9.0) -- `BRB IALL` (invalidate all) / `BRB INJ`
    /// (inject) branch-record-buffer maintenance.
    pub const fn brbe() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Brbe],
        }
    }

    /// The **instrumentation trace** extension (FEAT_ITE, ARMv9.0) -- the `TRCIT Xt` trace-instrumentation
    /// instruction.
    pub const fn ite() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Ite],
        }
    }

    /// The **round-to-integer-N** extension (FEAT_FRINTTS, ARMv8.5) -- `FRINT32X`/`FRINT32Z`/`FRINT64X`/`FRINT64Z`
    /// (scalar and vector).
    pub const fn frintts() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_5A,
            required_features: &[Arm64CpuFeature::Frintts],
        }
    }

    /// The **FP16 fused-multiply-long** extension (FEAT_FHM, ARMv8.2) -- the NEON `FMLAL`/`FMLAL2`/`FMLSL`/`FMLSL2`
    /// half-to-single widening multiply-accumulate.
    pub const fn fhm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_2A,
            required_features: &[Arm64CpuFeature::Fhm],
        }
    }

    /// The **atomic 64-byte access** extension (FEAT_LS64, ARMv8.7) -- `LD64B`/`ST64B`/`ST64BV`/`ST64BV0`.
    pub const fn ls64() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_5A,
            required_features: &[Arm64CpuFeature::Ls64],
        }
    }

    /// The **translation hardening** extension (FEAT_THE, ARMv8.8/9.3) -- the single-register `RCW`/`RCWS`
    /// read-check-write atomics (`RCWCAS`/`RCWCLR`/`RCWSET`/`RCWSWP` and the `S` variants, with A/L ordering).
    pub const fn the() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::The],
        }
    }

    /// FEAT_THE **with 128-bit data** (FEAT_THE + FEAT_D128, ARMv9.4) -- the 128-bit register-pair RCW forms
    /// (`RCWCASP`/`RCWCLRP`/`RCWSETP`/`RCWSWPP` and the `S` variants).
    pub const fn the_d128() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::The, Arm64CpuFeature::D128],
        }
    }

    /// The **checked pointer arithmetic** extension (FEAT_CPA, ARMv9.5) -- `ADDPT`/`SUBPT`/`MADDPT`/`MSUBPT`.
    /// **SVE checked pointer arithmetic** (FEAT_CPA, with SVE2) -- the SVE-vector `ADDPT`/`SUBPT`/`MADPT`/`MLAPT`.
    pub const fn sve_cpa() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sve,
                Arm64CpuFeature::Sve2,
                Arm64CpuFeature::Cpa,
            ],
        }
    }

    pub const fn cpa() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Cpa],
        }
    }

    /// The **FP absolute min/max** extension (FEAT_FAMINMAX, ARMv9.5) -- `FAMAX`/`FAMIN` (NEON and SVE).
    pub const fn faminmax() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::FaMinMax],
        }
    }

    /// **8-bit floating-point convert-long** (FEAT_FP8) -- the FP8 widening converts (`F1CVTL`/`F2CVTL`/`BF1CVTL`/
    /// `BF2CVTL`) that expand a vector of 8-bit floating-point lanes to half-precision / BFloat16.
    pub const fn fp8() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Fp8],
        }
    }

    /// **FP8 fused multiply-add** (FEAT_FP8 + FEAT_FP8FMA) -- the FP8 widening multiply-accumulate forms: the
    /// 2-way `FMLALB`/`FMLALT` (FP8 -> FP16) and the 4-way `FMLALLBB`/`FMLALLBT`/`FMLALLTB`/`FMLALLTT` (FP8 -> FP32).
    pub const fn fp8fma() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Fp8, Arm64CpuFeature::Fp8Fma],
        }
    }

    /// **FP8 dot product** (FEAT_FP8 + FEAT_FP8DOT2/FP8DOT4) -- the FP8 `FDOT` accumulating a 2-way (FP8 -> FP16)
    /// or 4-way (FP8 -> FP32) dot product of 8-bit floating-point lanes.
    pub const fn fp8dot() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Fp8, Arm64CpuFeature::Fp8Dot],
        }
    }

    /// **SVE FP8 dot product** (FEAT_FP8 + FEAT_SSVE_FP8DOT2/FP8DOT4) -- the SVE `FDOT Zda.s, Zn.b, Zm.b` (4-way,
    /// FP8 -> FP32) / `Zda.h` (2-way, FP8 -> FP16), vector and by-indexed-element.
    pub const fn ssve_fp8dot() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sve2,
                Arm64CpuFeature::Fp8,
                Arm64CpuFeature::SsveFp8Dot,
            ],
        }
    }

    /// **SVE FP8 fused multiply-add** (FEAT_FP8 + FEAT_SSVE_FP8FMA) -- the SVE widening MAC into Z: `FMLALB`/`FMLALT`
    /// (FP8 -> FP16, `Zda.h`) and the 4-way `FMLALLBB`/`FMLALLBT`/`FMLALLTB`/`FMLALLTT` (FP8 -> FP32, `Zda.s`).
    pub const fn ssve_fp8fma() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sve2,
                Arm64CpuFeature::Fp8,
                Arm64CpuFeature::SsveFp8Fma,
            ],
        }
    }

    /// **SVE FP8 convert** (FEAT_SVE2 + FEAT_FP8) -- the SVE FP8 widening converts `F1CVT(LT)`/`F2CVT(LT)`/
    /// `BF1CVT(LT)`/`BF2CVT(LT)` (`Zd.h <- Zn.b`).
    pub const fn sve_fp8() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2, Arm64CpuFeature::Fp8],
        }
    }

    /// **SVE 32-bit FP matrix-multiply** (FEAT_SVE + FEAT_F32MM) -- the single-precision SVE `FMMLA z.s` form.
    pub const fn sve_f32mm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_6A,
            required_features: &[Arm64CpuFeature::Sve, Arm64CpuFeature::F32mm],
        }
    }

    /// **SVE 64-bit FP matrix-multiply** (FEAT_SVE + FEAT_F64MM) -- the double-precision SVE `FMMLA z.d` form.
    pub const fn sve_f64mm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv8_6A,
            required_features: &[Arm64CpuFeature::Sve, Arm64CpuFeature::F64mm],
        }
    }

    /// **SVE AES** (FEAT_SVE2 + FEAT_SVE_AES) -- the SVE2 `AESE`/`AESD`/`AESMC`/`AESIMC` acceleration.
    pub const fn sve_aes() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2, Arm64CpuFeature::SveAes],
        }
    }

    /// **FPRCVT** (FEAT_FPRCVT) -- scalar `FCVT*`/`SCVTF`/`UCVTF` between an FP value and an int held in the other-size FP register.
    pub const fn fprcvt() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Fprcvt],
        }
    }

    /// **LUT** (FEAT_LUT) -- the NEON `LUTI2`/`LUTI4` table-vector lookups.
    pub const fn lut() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Lut],
        }
    }

    /// **FP8 FP16 matmul** (FEAT_SVE2 + FEAT_F8F16MM) -- the SVE `FMMLA Z.h, Z.b, Z.b`.
    pub const fn f8f16mm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2, Arm64CpuFeature::F8f16mm],
        }
    }

    /// **FP16 matmul** (FEAT_F16MM, an SVE2p2-era extension) -- the SVE `FMMLA Z.h, Z.h, Z.h`. **Experimental**
    /// (binutils-trunk-only oracle; LLVM-20 has no FP16-matmul feature).
    #[cfg(feature = "experimental")]
    pub const fn f16mm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2, Arm64CpuFeature::F16mm],
        }
    }

    /// **FP8 FP32 matmul** (FEAT_SVE2 + FEAT_F8F32MM) -- the SVE `FMMLA Z.s, Z.b, Z.b`.
    pub const fn f8f32mm() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2, Arm64CpuFeature::F8f32mm],
        }
    }

    /// **SVE AES2** (FEAT_SVE2p1 + FEAT_SVE_AES2) -- the SVE2.1 multi-vector `AESEMC`/`AESDIMC`.
    pub const fn sve_aes2() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sve2,
                Arm64CpuFeature::Sve2p1,
                Arm64CpuFeature::SveAes2,
            ],
        }
    }

    /// **SVE SM4** (FEAT_SVE2 + FEAT_SVE_SM4) -- the SVE2 `SM4E`/`SM4EKEY` acceleration.
    pub const fn sve_sm4() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2, Arm64CpuFeature::SveSm4],
        }
    }

    /// **SVE SHA-3** (FEAT_SVE2 + FEAT_SVE_SHA3) -- the SVE2 `RAX1` rotate-and-XOR.
    pub const fn sve_sha3() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2, Arm64CpuFeature::SveSha3],
        }
    }

    /// **SVE2.1** (FEAT_SVE2p1) -- the SVE2.1 additions such as `SCLAMP`/`UCLAMP`/`FCLAMP`. Also enabled by FEAT_SME2.
    pub const fn sve2p1() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sve2p1],
        }
    }

    /// The **Scalable Matrix Extension** (FEAT_SME) -- the `ZA` tile storage, streaming SVE mode (`SMSTART`/`SMSTOP`),
    /// and the matrix outer-product surface. Baseline in ARMv9.2-A.
    pub const fn sme() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sme],
        }
    }

    /// **SME double-precision outer product** (FEAT_SME + FEAT_SME_F64F64) -- `FMOPA`/`FMOPS` into a `.d` ZA tile.
    pub const fn sme_f64f64() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sme, Arm64CpuFeature::SmeF64F64],
        }
    }

    /// **SME 16-bit-to-64-bit integer outer product** (FEAT_SME + FEAT_SME_I16I64) -- the `.d`-accumulator integer
    /// `SMOPA`/`UMOPA`/`SUMOPA`/`USMOPA` (and the `S` subtract forms).
    pub const fn sme_i16i64() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sme, Arm64CpuFeature::SmeI16I64],
        }
    }

    /// The **memory-copy/set** extension (FEAT_MOPS, mandatory from ARMv9.3-A) -- the `CPYF*`/`CPY*`/`SET*` memcpy/
    /// memset accelerators.
    pub const fn mops() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Mops],
        }
    }

    /// FEAT_MOPS **with memory tagging** (FEAT_MOPS + FEAT_MTE) -- the tag-setting `SETG*` memset forms.
    pub const fn mops_mte() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Mops, Arm64CpuFeature::Mte],
        }
    }

    /// **SME2** (FEAT_SME2) -- the multi-vector `ZA` instruction surface (multi-vector MOVA/loads/stores, `ZA`
    /// dot-product/multiply-accumulate, `LUTI`, `PSEL`, ...). Built on FEAT_SME.
    pub const fn sme2() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[Arm64CpuFeature::Sme, Arm64CpuFeature::Sme2],
        }
    }

    /// **SME2.2** (FEAT_SME2p2, Armv9.6) -- the multi-vector `FMUL`. Built on FEAT_SME2.
    pub const fn sme2p2() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::Sme2p2,
            ],
        }
    }

    /// **SME2 double-precision multi-vector** (FEAT_SME2 + FEAT_SME_F64F64) -- the `.d` multi-vector `ZA` FP
    /// multiply-accumulate (`FMLA`/`FMLS` into a `.d` ZA group). Distinct from `sme_f64f64()` (the non-SME2
    /// `FMOPA` outer product), which does not require FEAT_SME2.
    pub const fn sme2_f64f64() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeF64F64,
            ],
        }
    }

    /// **SME2 16-bit-to-64-bit integer multi-vector** (FEAT_SME2 + FEAT_SME_I16I64) -- the `.d`-accumulator
    /// multi-vector `ZA` integer ops (`ADD`/`SUB`/`SDOT`/`UDOT` into a `.d` ZA group). Distinct from `sme_i16i64()`
    /// (the non-SME2 `SMOPA` outer product), which does not require FEAT_SME2.
    pub const fn sme2_i16i64() -> Self {
        Self {
            min_isa_version: Arm64IsaVersion::Armv9A,
            required_features: &[
                Arm64CpuFeature::Sme,
                Arm64CpuFeature::Sme2,
                Arm64CpuFeature::SmeI16I64,
            ],
        }
    }
}
