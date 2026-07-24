// Copyright (c) Scaleservers LLC

/// Which disassembler's formatting conventions to emit -- a local copy of the 32-bit library's enum of the
/// same name (this crate is independent and shares no types with `scaleservers-arm32-assembly`). Both
/// flavors are valid A64 assembly that the GNU and LLVM assemblers accept; they differ in *disassembly*
/// presentation (most visibly the immediate radix), so a listing can be made to match either toolchain's
/// `objdump`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmAssemblySyntax {
    /// LLVM (`llvm-objdump` / `llvm-mc`) conventions -- e.g. hexadecimal immediates (`#0x10`).
    Llvm,
    /// GNU binutils (`aarch64-none-elf-objdump` / `as`) conventions -- e.g. decimal immediates (`#16`).
    Gnu,
}
