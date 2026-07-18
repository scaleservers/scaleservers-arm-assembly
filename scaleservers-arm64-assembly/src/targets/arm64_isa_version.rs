// Copyright (c) Scaleservers LLC

/// AArch64 (A64) architecture version. Unlike the 32-bit library -- which spans two non-comparable lineages
/// (M-profile Thumb-only vs A/R-profile) -- AArch64 is a single linear lineage: each ARMv8.x / ARMv9.x
/// release is a strict superset of the one before, so a plain rank comparison is sound. Newer baseline
/// instructions (e.g. the v8.1 atomics, v8.3 pointer authentication) are gated by raising `min_isa_version`;
/// orthogonal optional units (FP/SIMD, crypto, SVE, ...) are gated by [`super::Arm64CpuFeature`] instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Arm64IsaVersion {
    Armv8A,   // ARMv8.0-A -- the original AArch64 baseline
    Armv8_1A, // adds LSE atomics, LOR, RDM, ...
    Armv8_2A, // adds RAS, the half-precision FP/SIMD data-processing extensions, ...
    Armv8_3A, // adds pointer authentication (PAC), complex-number FP, JSCVT, ...
    Armv8_4A, // adds the dot-product, FlagM, more atomics, ...
    Armv8_5A, // adds BTI, FRINT, FlagM2, ...
    Armv8_6A, // adds the int8 matrix multiply (I8MM), BFloat16, the general matrix-multiply forms, ...
    Armv9A,   // ARMv9.0-A -- SVE2 baseline (built on ARMv8.5)
}

impl Arm64IsaVersion {
    /// Monotonically increasing rank: AArch64 is one linear lineage, so `a.rank() >= b.rank()` means a
    /// target at `a` can run everything a `b` baseline can.
    pub fn rank(self) -> u8 {
        match self {
            Self::Armv8A => 0,
            Self::Armv8_1A => 1,
            Self::Armv8_2A => 2,
            Self::Armv8_3A => 3,
            Self::Armv8_4A => 4,
            Self::Armv8_5A => 5,
            Self::Armv8_6A => 6,
            Self::Armv9A => 7,
        }
    }

    /// Does a target running THIS version satisfy a requirement of at least `required`? True when this
    /// version's rank is >= the requirement's (single linear lineage; no cross-lineage caveat as in the
    /// 32-bit library).
    pub fn satisfies(self, required: Arm64IsaVersion) -> bool {
        self.rank() >= required.rank()
    }
}
