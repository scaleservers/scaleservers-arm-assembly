// Copyright (c) Scaleservers LLC

// Assembly-string emission (model -> text), the analogue of the 32-bit library's `emit` module. ARM has a
// single Unified Assembly Language (UAL) that both the GNU assembler and LLVM consume, so the *grammar* is
// shared; what differs between toolchains is *disassembly presentation* (immediate radix, etc.), which
// `ArmAssemblySyntax` selects. A disassembler renders each decoded instruction through this layer.
//
// Entry point added to `Arm64Instruction`:
//   * `to_assembly_string(syntax)` -- raw form; PC-relative operands print as signed byte offsets.

mod arm_assembly_syntax;
pub use arm_assembly_syntax::ArmAssemblySyntax;

mod arm64_assembly_emitter;
