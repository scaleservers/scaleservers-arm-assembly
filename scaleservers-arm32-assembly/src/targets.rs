// Copyright (c) Scaleservers LLC

// Target-architecture gating (processor/profile targeting). An `ArmTargetProfile` describes the machine we
// are emitting for (a baseline ISA version plus a set of architecture-extension features); each
// `ArmT32Instruction` reports an `ArmInstructionRequirement`; `ArmT32Instruction::encode_for_target`
// refuses to emit anything the profile does not support. The implemented set spans the whole ARM M-profile
// -- ARMv6-M, ARMv7-M, and the ARMv7E-M DSP/FP extensions -- so requirements range from the baseline up to
// `Armv7EM` + `DspExtension`/`FloatingPoint`, and the gate is exercised across all of them.

mod arm_cpu_feature;
pub use arm_cpu_feature::ArmCpuFeature;

mod arm_instruction_requirement;
pub use arm_instruction_requirement::ArmInstructionRequirement;

mod arm_isa_version;
pub use arm_isa_version::{ArmIsaLineage, ArmIsaVersion};

mod arm_target_profile;
pub use arm_target_profile::ArmTargetProfile;
