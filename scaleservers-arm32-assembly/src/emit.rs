// Copyright (c) Scaleservers LLC

// Assembly-string emission (model -> text). ARM has a single Unified Assembly Language (UAL) that both the
// GNU assembler and LLVM consume, so the *grammar* is shared; what differs between toolchains is
// *disassembly presentation* (immediate radix, etc.), which `ArmAssemblySyntax` selects. The disassembler
// (arm32dasm) renders each decoded instruction through this layer.
//
// Entry points added to `ArmT32Instruction`:
//   * `to_assembly_string(syntax)`           -- raw form; PC-relative operands print as offsets.
//   * `to_assembly_string_at(address, syntax)` -- resolves PC-relative targets to absolute addresses.

mod arm_assembly_syntax;
pub use arm_assembly_syntax::ArmAssemblySyntax;

mod arm_assembly_emitter;
pub use arm_assembly_emitter::apply_it_block_condition;
pub use arm_assembly_emitter::apply_vpt_block_suffix;

// A32 ("ARM" state) UAL emitter: adds `to_assembly_string` / `to_assembly_string_at` to `ArmA32Instruction`.
mod arm_a32_assembly_emitter;
