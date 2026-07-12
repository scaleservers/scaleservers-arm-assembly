// Copyright (c) Scaleservers LLC

// Robustness against UNTRUSTED input: the T32 and A32 decoders face arbitrary bytes (a disassembler
// consumes untrusted binaries), and the contract is that they return an error or a clean `Ok(None)` -- they
// never panic, never loop forever, and always make forward progress. Emission of any decoded instruction
// must never panic in either assembly flavor, and any instruction that re-encodes must round-trip exactly
// (a model-level encode/decode fixed point). These sweeps are deterministic -- a fixed-seed xorshift PRNG,
// no external fuzzing crates, so the crate test suite stays self-contained and any failure reproduces
// byte-for-byte. The cargo-fuzz crate in `fuzz/` drives the same properties coverage-guided (see fuzz/).

use crate::emit::ArmAssemblySyntax;
use crate::{ArmA32Instruction, ArmT32Instruction};

// Miri runs the SAME never-panic / round-trip code paths these sweeps exercise, but it is 100-1000x slower
// and UB depends on the path, not the seed count (miri-convention.md). So under `cfg(miri)` we shrink the
// random-byte / mutation / corpus budgets to a few short cases -- the decode -> emit -> re-encode paths still get
// UB-checked, the suite just finishes in reasonable time. Stable `cargo test` keeps the full budgets.
#[cfg(miri)]
const RANDOM_SWEEP_ITERATIONS: usize = 40;
#[cfg(not(miri))]
const RANDOM_SWEEP_ITERATIONS: usize = 6_000;
#[cfg(miri)]
const MUTATION_CORPUS_SIZE: usize = 6;
#[cfg(not(miri))]
const MUTATION_CORPUS_SIZE: usize = 96;

// xorshift64* -- tiny, deterministic, and plenty random for generating hostile input.
struct Prng(u64);

impl Prng {
    fn next(&mut self) -> u64 {
        let mut value = self.0;
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        self.0 = value;
        value.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn byte(&mut self) -> u8 {
        (self.next() >> 32) as u8
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
}

// Decode every instruction in `bytes` to exhaustion. For each instruction: forward progress is mandatory,
// both emit flavors must not panic, and a successful re-encode must reproduce an equal model that consumes
// exactly its own bytes. Any decode error / clean EOF ends the sweep. A panic here fails the test.
fn sweep_t32(bytes: &[u8]) {
    let mut iter = bytes.iter();
    let mut offset = 0usize;
    loop {
        let before = offset;
        match ArmT32Instruction::decode(&mut iter, &mut offset) {
            Ok(Some(instruction)) => {
                assert!(
                    offset > before,
                    "T32 decode reported success without consuming input"
                );
                let _ = instruction.to_assembly_string(ArmAssemblySyntax::Llvm);
                let _ = instruction.to_assembly_string(ArmAssemblySyntax::Gnu);
                if let Ok(encoded) = instruction.encode() {
                    let mut inner = encoded.iter();
                    let mut consumed = 0usize;
                    let round_trip = ArmT32Instruction::decode(&mut inner, &mut consumed)
                        .expect("re-decoding our own encoded bytes must not error")
                        .expect("re-decoding our own encoded bytes must yield an instruction");
                    assert_eq!(
                        round_trip, instruction,
                        "T32 encode/decode is not a fixed point"
                    );
                    assert_eq!(consumed, encoded.len(), "T32 re-decode left trailing bytes");
                }
            }
            Ok(None) | Err(_) => break,
        }
    }
}

fn sweep_a32(bytes: &[u8]) {
    let mut iter = bytes.iter();
    let mut offset = 0usize;
    loop {
        let before = offset;
        match ArmA32Instruction::decode(&mut iter, &mut offset) {
            Ok(Some(instruction)) => {
                assert!(
                    offset > before,
                    "A32 decode reported success without consuming input"
                );
                let _ = instruction.to_assembly_string(ArmAssemblySyntax::Llvm);
                let _ = instruction.to_assembly_string(ArmAssemblySyntax::Gnu);
                if let Ok(encoded) = instruction.encode() {
                    let mut inner = encoded.iter();
                    let mut consumed = 0usize;
                    let round_trip = ArmA32Instruction::decode(&mut inner, &mut consumed)
                        .expect("re-decoding our own encoded bytes must not error")
                        .expect("re-decoding our own encoded bytes must yield an instruction");
                    assert_eq!(
                        round_trip, instruction,
                        "A32 encode/decode is not a fixed point"
                    );
                    assert_eq!(consumed, encoded.len(), "A32 re-decode left trailing bytes");
                }
            }
            Ok(None) | Err(_) => break,
        }
    }
}

#[test]
fn t32_decode_never_panics_on_random_bytes() {
    let mut prng = Prng(0x5EED_7032);
    for _ in 0..RANDOM_SWEEP_ITERATIONS {
        let length = prng.below(64);
        let bytes: Vec<u8> = (0..length).map(|_| prng.byte()).collect();
        sweep_t32(&bytes);
    }
}

#[test]
fn a32_decode_never_panics_on_random_bytes() {
    let mut prng = Prng(0x5EED_A032);
    for _ in 0..RANDOM_SWEEP_ITERATIONS {
        // A32 is fixed-width 32-bit; align lengths to whole words so the word reader is fully exercised.
        let length = 4 * prng.below(16);
        let bytes: Vec<u8> = (0..length).map(|_| prng.byte()).collect();
        sweep_a32(&bytes);
    }
}

