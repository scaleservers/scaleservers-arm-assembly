// Copyright (c) Scaleservers LLC

use crate::targets::{Arm64CpuFeature, Arm64InstructionRequirement, Arm64IsaVersion};
use alloc::collections::BTreeSet;

/// The machine we are emitting for: a baseline ISA version plus a set of architecture-extension features.
/// Modeling features as a set (rather than only a named core) lets us express targets like a base ARMv8-A
/// core WITHOUT the crypto extension. Named constructors cover common profiles. Passed to the
/// `encode_for_target` method to gate instructions a profile does not support.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arm64TargetProfile {
    isa_version: Arm64IsaVersion,
    features: BTreeSet<Arm64CpuFeature>,
}

impl Arm64TargetProfile {
    /// Build a profile from a baseline ISA version and an explicit set of CPU features.
    pub fn new(isa_version: Arm64IsaVersion, features: &[Arm64CpuFeature]) -> Self {
        Self {
            isa_version,
            features: features.iter().copied().collect(),
        }
    }

    pub fn isa_version(&self) -> Arm64IsaVersion {
        self.isa_version
    }

    pub fn has_feature(&self, feature: Arm64CpuFeature) -> bool {
        self.features.contains(&feature)
    }

    /// Does this target satisfy an instruction's requirement? AArch64 is one linear lineage, so this is a
    /// rank compare on the ISA version plus a check that every required feature is present.
    pub fn supports(&self, requirement: &Arm64InstructionRequirement) -> bool {
        if !self.isa_version.satisfies(requirement.min_isa_version) {
            return false;
        }
        requirement
            .required_features
            .iter()
            .all(|feature| self.features.contains(feature))
    }

    /* named convenience profiles */

    /// Base ARMv8.0-A with the standard FP + Advanced SIMD units (the common application-core baseline).
    pub fn armv8a() -> Self {
        Self::new(
            Arm64IsaVersion::Armv8A,
            &[
                Arm64CpuFeature::FloatingPoint,
                Arm64CpuFeature::AdvancedSimd,
            ],
        )
    }

    /// Base ARMv8.0-A integer core, no optional units at all -- the strictest profile (used to prove the
    /// gate refuses FP/SIMD/crypto forms).
    pub fn armv8a_integer_only() -> Self {
        Self::new(Arm64IsaVersion::Armv8A, &[])
    }

    /// A maximally-permissive profile (newest ISA, all features) -- used by the test/oracle harness, where
    /// we want raw bytes regardless of any particular target's limits.
    pub fn permissive() -> Self {
        Self::new(
            Arm64IsaVersion::Armv9A,
            &[
                Arm64CpuFeature::FloatingPoint,
                Arm64CpuFeature::AdvancedSimd,
                Arm64CpuFeature::Crypto,
                Arm64CpuFeature::Lse,
                Arm64CpuFeature::Pauth,
                Arm64CpuFeature::Sve,
            ],
        )
    }
}
