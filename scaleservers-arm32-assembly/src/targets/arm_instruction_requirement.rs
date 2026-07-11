// Copyright (c) Scaleservers LLC

use crate::targets::{
    ArmCpuFeature,
    ArmIsaVersion,
};

// What an instruction (or one of its forms) needs from the target to be emittable: a minimum ISA
// version and zero or more architecture-extension features. `required_features` is a `&'static` slice
// so requirement values are allocation-free (call sites pass promoted constant slices, e.g.
// `&[ArmCpuFeature::DspExtension]`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArmInstructionRequirement {
    pub min_isa_version: ArmIsaVersion,
    pub required_features: &'static [ArmCpuFeature],
}
impl ArmInstructionRequirement {
    pub const fn new(min_isa_version: ArmIsaVersion, required_features: &'static [ArmCpuFeature]) -> Self {
        Self { min_isa_version, required_features }
    }

    // the current universally-available baseline: ARMv6-M, no extension features
    pub const fn baseline() -> Self {
        Self { min_isa_version: ArmIsaVersion::Armv6M, required_features: &[] }
    }
}
