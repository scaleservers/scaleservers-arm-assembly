# scaleservers-arm32-assembly

ARM 32-bit (Thumb/T32 + ARM/A32) instruction **encode / decode** library in Rust -- the 32-bit sibling of
`scaleservers-arm64-assembly`.

**Zero runtime dependencies. `no_std`-capable.** Every instruction offers the family signatures:

| Method | Direction |
|--------|-----------|
| `encode` | model -> machine bytes |
| `decode` | machine bytes -> model (the exact inverse of `encode`) |
| `encode_for_target(&ArmTargetProfile)` | gated encode -- refuses forms the chosen CPU cannot run |
| `to_assembly_string(syntax)` | model -> UAL text (GNU or LLVM flavor) |

## Design: one typed layer (no raw/logical split)

Some assemblers expose two layers -- a *raw* layer of literal bit-fields and a *logical* layer of
semantic operands -- with an explicit mapping between them. This crate intentionally uses a **single typed
layer**: `ArmA32Instruction` / `ArmT32Instruction` are the model, each variant carries semantic operands
(typed registers, sized immediates, condition/rounding enums), and the bit-fields live only inside `encode`
/`decode`. The two directions are exact inverses, so the round-trip *is* the raw/logical correspondence --
there is no separate raw struct that could drift out of sync with the logical one. Correctness is pinned
instead by the dual-oracle differential tests (assemble/disassemble against real GNU `as`/`objdump` and LLVM
`llvm-mc`) plus exhaustive round-trip and never-panic sweeps. This matches how the AArch64 sibling is
structured and is a deliberate choice, not a missing layer.

## Coverage

The **ARM 32-bit instruction surface** -- the complete T32 and A32 instruction set across the profiles below,
validated byte-for-byte against `arm-none-eabi-as` and cross-checked against the official Arm ARMs (DDI0553 /
DDI0487).

- **T32 (Thumb)** -- the full M-profile set: ARMv6-M, ARMv7-M, ARMv7E-M (DSP + hardware FP), the ARMv8-M Security
  Extension, the complete **ARMv8.1-M MVE / "Helium"** vector extension, the v8.1-M scalar additions (CSEL,
  long/saturating shifts, branch-future, VSCCLRM, low-overhead loops), and the optional extensions (generic
  coprocessor, **CDE**, PACBTI).
- **A32 (ARM state)** -- the A/R-profile set: integer/DSP, system, coprocessor, ARMv8-A AArch32 additions, full
  VFP, full NEON + crypto.

T32 and A32 are **separate types** (`ArmT32Instruction` / `ArmA32Instruction`) so a code generator can't mix the
two instruction sets in one stream; `Arm32Instruction` is the interworking union.

## Example

```rust
use scaleservers_arm32_assembly::{ArmT32Instruction, ArmAssemblySyntax};

let nop = ArmT32Instruction::Nop_T1;
assert_eq!(nop.encode().unwrap(), [0x00, 0xbf]);                  // model -> bytes
assert_eq!(nop.to_assembly_string(ArmAssemblySyntax::Gnu), "nop");

let bytes = [0x00u8, 0xbf];                                       // bytes -> model (inverse of encode)
let mut offset = 0;
let decoded = ArmT32Instruction::decode(&mut bytes.iter(), &mut offset).unwrap().unwrap();
assert_eq!(decoded, nop);
```

**Decoding never panics on arbitrary input** -- malformed bytes return a `DecodeError`, never a crash (a
disassembler consumes untrusted binaries). Enforced by deterministic robustness sweeps and a coverage-guided
cargo-fuzz campaign.

## `no_std`

Default build is `std`; `--no-default-features` builds against `core` + `alloc` only (for bare-metal Cortex-M):

```sh
cargo build --lib --no-default-features --target thumbv7em-none-eabi
```

## Testing

- **Spec-grounded unit tests** -- exact encode/decode/emit/gating per instruction, byte values from the spec.
- **Exhaustive round-trips** -- `model -> bytes -> model` fixed points.
- **External differential oracle** -- the suite shells out to `arm-none-eabi-as` (and `clang`, when present) and
  compares bytes; absent the toolchain these tests **skip loudly** rather than fail. `apt install
  gcc-arm-none-eabi` for full confidence.
- **Robustness + fuzzing** -- `src/tests/robustness_tests.rs` runs deterministic never-panic / fixed-point sweeps
  in normal `cargo test`; `fuzz/` is a coverage-guided cargo-fuzz harness (see `fuzz/README.md`). Every crash it
  has found is pinned as a committed regression test.

The console tools (`armasm` assembler, `armdasm` disassembler) build on this library.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
