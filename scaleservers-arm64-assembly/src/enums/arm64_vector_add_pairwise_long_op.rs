// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **add long pairwise** instruction (DDI0487 C7) -- the
/// two-register-misc encoding `0 Q U 01110 size 10000 opcode 10 Rn Rd` that adds adjacent element pairs of `Vn`
/// into a destination whose elements are twice as wide (`SADDLP`/`UADDLP`), optionally accumulating into `Vd`
/// (`SADALP`/`UADALP`). The op is an orthogonal field over a shared `{ narrow arrangement, Vd, Vn }` shape: `Vn`
/// is the narrow source (`.8b`/`.16b`/`.4h`/`.8h`/`.2s`/`.4s`), `Vd` is twice as wide with half the lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorAddPairwiseLongOp {
    /// `SADDLP` -- signed add long pairwise (`U = 0`, opcode `00010`).
    Saddlp,
    /// `UADDLP` -- unsigned add long pairwise (`U = 1`, opcode `00010`).
    Uaddlp,
    /// `SADALP` -- signed add long pairwise and accumulate (`U = 0`, opcode `00110`).
    Sadalp,
    /// `UADALP` -- unsigned add long pairwise and accumulate (`U = 1`, opcode `00110`).
    Uadalp,
}

impl Arm64VectorAddPairwiseLongOp {
    /// The base word with `Q = 0` and `size = 0` (`U` + opcode baked in); the arrangement supplies `Q<<30` and
    /// the *source* `size<<22`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Saddlp => 0x0E20_2800,
            Self::Uaddlp => 0x2E20_2800,
            Self::Sadalp => 0x0E20_6800,
            Self::Uadalp => 0x2E20_6800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Saddlp => "saddlp",
            Self::Uaddlp => "uaddlp",
            Self::Sadalp => "sadalp",
            Self::Uadalp => "uadalp",
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 4] = [Self::Saddlp, Self::Uaddlp, Self::Sadalp, Self::Uadalp];
}
