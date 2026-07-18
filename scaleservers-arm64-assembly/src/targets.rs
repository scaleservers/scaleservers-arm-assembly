// Copyright (c) Scaleservers LLC

// Target-architecture gating (processor/profile targeting), mirroring the 32-bit library's `targets`
// module. An `Arm64TargetProfile` describes the machine we are emitting for (a baseline ISA version plus a
// set of architecture-extension features); each `Arm64Instruction` reports an `Arm64InstructionRequirement`;
// `Arm64Instruction::encode_for_target` refuses to emit anything the profile does not support. AArch64 is a
// single linear lineage (unlike the 32-bit library's two non-comparable lineages), so version satisfaction
// is a plain rank compare.

mod arm64_cpu_feature;
pub use arm64_cpu_feature::Arm64CpuFeature;

mod arm64_instruction_requirement;
pub use arm64_instruction_requirement::Arm64InstructionRequirement;

mod arm64_isa_version;
pub use arm64_isa_version::Arm64IsaVersion;

mod arm64_target_profile;
pub use arm64_target_profile::Arm64TargetProfile;
