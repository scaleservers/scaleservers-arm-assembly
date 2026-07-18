// Copyright (c) Scaleservers LLC

use super::Arm64VectorArrangement;

/// The operation of an AArch64 Advanced SIMD (NEON) **floating-point length convert** -- the two-register-misc
/// FP ops that change the element precision: `FCVTL` widens (f16->f32 or f32->f64), `FCVTN` narrows (f32->f16 or
/// f64->f32), `FCVTXN` narrows f64->f32 with round-to-odd. The op is an orthogonal field over a shared
/// `{ wide arrangement, high, Vd, Vn }` shape: `wide` is the 128-bit FP side (`.4s` for the f16/f32 pair, `.2d`
/// for the f32/f64 pair), `high` (the `Q` bit, `2`-suffix) selects the lower/upper half of the narrow operand,
/// and [`Self::is_widen`] says whether `Vd` is the wide or the narrow side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorFpConvertLengthOp {
    /// `FCVTL` -- widen (f16->f32 / f32->f64) (`U = 0`, opcode `10111`).
    Fcvtl,
    /// `FCVTN` -- narrow (f32->f16 / f64->f32) (`U = 0`, opcode `10110`).
    Fcvtn,
    /// `FCVTXN` -- narrow f64->f32 with round-to-odd (`U = 1`, opcode `10110`; the `.2d` wide side only).
    Fcvtxn,
}

impl Arm64VectorFpConvertLengthOp {
    /// The base word with `Q = 0` and `sz = 0` (`U` + opcode baked in); the encoder adds `Q<<30` (high) and the
    /// `sz` bit `<<22` (`.4s` wide -> 0, `.2d` wide -> 1). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Fcvtl => 0x0E21_7800,
            Self::Fcvtn => 0x0E21_6800,
            Self::Fcvtxn => 0x2E21_6800,
        }
    }

    /// The lowercase UAL mnemonic (without the `2` upper-half suffix, appended by the emitter from `high`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Fcvtl => "fcvtl",
            Self::Fcvtn => "fcvtn",
            Self::Fcvtxn => "fcvtxn",
        }
    }

    /// Whether this widens (`FCVTL`: `Vd` is the wide side, `Vn` the narrow) vs narrows (`Vd` narrow, `Vn` wide).
    pub fn is_widen(self) -> bool {
        matches!(self, Self::Fcvtl)
    }

    /// Whether `wide` (the 128-bit FP side) is valid: `FCVTL`/`FCVTN` take `.4s` (f16<->f32) or `.2d` (f32<->f64);
    /// `FCVTXN` takes only `.2d` (it narrows f64->f32).
    pub fn allows_wide(self, wide: Arm64VectorArrangement) -> bool {
        use Arm64VectorArrangement::{D2, S4};
        match self {
            Self::Fcvtxn => matches!(wide, D2),
            _ => matches!(wide, S4 | D2),
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 3] = [Self::Fcvtl, Self::Fcvtn, Self::Fcvtxn];
}
