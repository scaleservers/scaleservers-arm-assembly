// Copyright (c) Scaleservers LLC

use crate::targets::{
    ArmCpuFeature,
    ArmInstructionRequirement,
    ArmIsaVersion,
};
// A `BTreeSet` (from `alloc`, available under `no_std`) rather than a `std` `HashSet`: this is a CPU-feature
// *membership* set queried only with `.contains()` (see `has_feature` / `supports`), so the tree's ordering
// is irrelevant to behavior and never reaches an emitted byte. Requires `ArmCpuFeature: Ord` (derived).
use alloc::collections::BTreeSet;

/// The machine we are emitting for: a baseline ISA version plus a set of architecture-extension features.
/// Modeling features as a set (rather than only a named core) lets us express targets like a Cortex-M4
/// core WITHOUT its optional FPU. Named constructors cover the common profiles; the initial
/// compiler-backend target is `armv6m()`. Passed to the `encode_for_target` methods to gate
/// instructions a profile does not support.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArmTargetProfile {
    isa_version: ArmIsaVersion,
    features: BTreeSet<ArmCpuFeature>,
}
impl ArmTargetProfile {
    /// Build a profile from a baseline ISA version and an explicit set of CPU features.
    pub fn new(isa_version: ArmIsaVersion, features: &[ArmCpuFeature]) -> Self {
        Self {
            isa_version,
            features: features.iter().copied().collect(),
        }
    }

    pub fn isa_version(&self) -> ArmIsaVersion {
        self.isa_version
    }

    pub fn has_feature(&self, feature: ArmCpuFeature) -> bool {
        self.features.contains(&feature)
    }

    // does this target satisfy an instruction's requirement?
    pub fn supports(&self, requirement: &ArmInstructionRequirement) -> bool {
        // lineage-aware: an A32 (A/R-profile) requirement is never satisfied by a Thumb-only M-profile
        // target, and vice versa (see ArmIsaVersion::satisfies). Within one lineage it is a rank compare.
        if !self.isa_version.satisfies(requirement.min_isa_version) {
            return false;
        }

        requirement.required_features.iter().all(|feature| self.features.contains(feature))
    }

    /* named convenience profiles */

    // the initial compiler-backend target: ARMv6-M (Cortex-M0 / M0+ / M1), no extensions
    pub fn armv6m() -> Self {
        Self::new(ArmIsaVersion::Armv6M, &[])
    }

    // ARMv7-M (Cortex-M3): the bulk of Thumb-2, no DSP/FP.
    pub fn armv7m() -> Self {
        Self::new(ArmIsaVersion::Armv7M, &[])
    }

    // ARMv7E-M (Cortex-M4): adds the DSP extension.
    pub fn armv7em() -> Self {
        Self::new(ArmIsaVersion::Armv7EM, &[ArmCpuFeature::DspExtension])
    }

    // ARMv8-M Baseline (Cortex-M23): the v6-M superset with the Security Extension (TrustZone-M).
    pub fn armv8m_baseline() -> Self {
        Self::new(ArmIsaVersion::Armv8MBaseline, &[ArmCpuFeature::Security])
    }

    // ARMv8-M Mainline (Cortex-M33): adds DSP, hardware FP, and the Security Extension.
    pub fn armv8m_mainline() -> Self {
        Self::new(ArmIsaVersion::Armv8MMainline, &[ArmCpuFeature::DspExtension, ArmCpuFeature::FloatingPoint, ArmCpuFeature::Security])
    }

    // ARMv8.1-M Mainline with MVE (Cortex-M55 / M85): the v8-M Mainline superset plus the MVE "Helium"
    // vector extension (integer + floating-point) on top of DSP, hardware FP and the Security Extension.
    pub fn armv8_1m_mve() -> Self {
        Self::new(
            ArmIsaVersion::Armv8_1MMainline,
            &[
                ArmCpuFeature::DspExtension,
                ArmCpuFeature::FloatingPoint,
                ArmCpuFeature::Security,
                ArmCpuFeature::Mve,
                ArmCpuFeature::MveFloat,
            ],
        )
    }

    // a maximally-permissive profile (newest ISA, all features) -- used by the test/oracle harness, where
    // we want raw bytes regardless of any particular target's limits
    pub fn permissive() -> Self {
        Self::new(
            ArmIsaVersion::Armv8_1MMainline,
            &[
                ArmCpuFeature::DspExtension,
                ArmCpuFeature::FloatingPoint,
                ArmCpuFeature::Security,
                ArmCpuFeature::Mve,
                ArmCpuFeature::MveFloat,
            ],
        )
    }

    /* A/R-profile (A32) named convenience profiles */

    // ARMv7-A / ARMv7-R (Cortex-A / Cortex-R): the full A32 base ISA, no optional extensions.
    pub fn armv7ar() -> Self {
        Self::new(ArmIsaVersion::Armv7AR, &[])
    }

    // ARMv8-A in AArch32 (32-bit) state: the v8 A32 base. Optional extensions (CRC, crypto, NEON, FP) are
    // added as features by the caller.
    pub fn armv8a_aarch32() -> Self {
        Self::new(ArmIsaVersion::Armv8A, &[])
    }

    // classic pre-Cortex A32 cores
    pub fn armv4t() -> Self {
        Self::new(ArmIsaVersion::Armv4T, &[])
    }
    pub fn armv5te() -> Self {
        Self::new(ArmIsaVersion::Armv5TE, &[])
    }
    pub fn armv6_arm() -> Self {
        Self::new(ArmIsaVersion::Armv6, &[])
    }

    // a maximally-permissive A/R-profile profile (newest A32 ISA + all A/R features) -- the A32 analogue of
    // `permissive()`, used by the A32 test/oracle harness so raw bytes are emitted regardless of any one
    // core's limits.
    pub fn permissive_aarch32() -> Self {
        Self::new(
            ArmIsaVersion::Armv8A,
            &[
                ArmCpuFeature::DspExtension,
                ArmCpuFeature::FloatingPoint,
                ArmCpuFeature::AdvancedSimd,
                ArmCpuFeature::Crypto,
            ],
        )
    }
}
