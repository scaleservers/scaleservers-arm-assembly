// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **long vector x indexed element** instruction (DDI0487 C7) --
/// the `0 Q U 01111 size L M Rm opcode H 0 Rn Rd` class whose result is twice the source element width, against a
/// single broadcast lane `Vm.<ts>[index]`. The op is an orthogonal field over a shared `{ wide arrangement, high,
/// Vd, Vn, Vm, index }` shape: `wide` is the 128-bit destination (`.4s`/`.2d`), `high` (the `Q` bit, `2`-suffix)
/// selects the lower/upper half of the narrow `Vn`, and the index + `Vm` fold into `H:L:M` per the *narrow*
/// element size (`.h` for a `.4s` result, `.s` for a `.2d` result).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorByElementLongOp {
    /// `SMULL` (by element) -- signed multiply long (`U = 0`, opcode `1010`).
    Smull,
    /// `UMULL` (by element) -- unsigned multiply long (`U = 1`, opcode `1010`).
    Umull,
    /// `SMLAL` (by element) -- signed multiply-add long (`U = 0`, opcode `0010`).
    Smlal,
    /// `UMLAL` (by element) -- unsigned multiply-add long (`U = 1`, opcode `0010`).
    Umlal,
    /// `SMLSL` (by element) -- signed multiply-subtract long (`U = 0`, opcode `0110`).
    Smlsl,
    /// `UMLSL` (by element) -- unsigned multiply-subtract long (`U = 1`, opcode `0110`).
    Umlsl,
    /// `SQDMULL` (by element) -- signed saturating doubling multiply long (`U = 0`, opcode `1011`).
    Sqdmull,
    /// `SQDMLAL` (by element) -- signed saturating doubling multiply-add long (`U = 0`, opcode `0011`).
    Sqdmlal,
    /// `SQDMLSL` (by element) -- signed saturating doubling multiply-subtract long (`U = 0`, opcode `0111`).
    Sqdmlsl,
}

impl Arm64VectorByElementLongOp {
    /// The base word with `Q = 0`, `size = 0`, and the index bits + `Rm` cleared (`U` + opcode baked in).
    /// GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Smull => 0x0F00_A000,
            Self::Umull => 0x2F00_A000,
            Self::Smlal => 0x0F00_2000,
            Self::Umlal => 0x2F00_2000,
            Self::Smlsl => 0x0F00_6000,
            Self::Umlsl => 0x2F00_6000,
            Self::Sqdmull => 0x0F00_B000,
            Self::Sqdmlal => 0x0F00_3000,
            Self::Sqdmlsl => 0x0F00_7000,
        }
    }

    /// The lowercase UAL mnemonic (without the `2` upper-half suffix, which the emitter appends from `high`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Smull => "smull",
            Self::Umull => "umull",
            Self::Smlal => "smlal",
            Self::Umlal => "umlal",
            Self::Smlsl => "smlsl",
            Self::Umlsl => "umlsl",
            Self::Sqdmull => "sqdmull",
            Self::Sqdmlal => "sqdmlal",
            Self::Sqdmlsl => "sqdmlsl",
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 9] = [
        Self::Smull,
        Self::Umull,
        Self::Smlal,
        Self::Umlal,
        Self::Smlsl,
        Self::Umlsl,
        Self::Sqdmull,
        Self::Sqdmlal,
        Self::Sqdmlsl,
    ];
}
