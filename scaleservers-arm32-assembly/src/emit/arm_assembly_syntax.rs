// Copyright (c) Scaleservers LLC

/// Which disassembler's formatting conventions to emit. Both flavors are valid UAL that the GNU and LLVM
/// assemblers accept; they differ in *disassembly* presentation (most visibly the immediate radix), so a
/// listing can be made to match either toolchain's `objdump`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmAssemblySyntax {
    /// LLVM (`llvm-objdump` / `llvm-mc`) conventions -- e.g. hexadecimal immediates (`#0x1f`).
    Llvm,
    /// GNU binutils (`arm-none-eabi-objdump` / `as`) conventions -- e.g. decimal immediates (`#31`).
    Gnu,
}
