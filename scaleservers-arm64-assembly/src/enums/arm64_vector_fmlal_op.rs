// Copyright (c) Scaleservers LLC

/// A NEON **FP16 fused-multiply-long** op (FEAT_FHM, DDI0487 C7): `FMLAL`/`FMLAL2`/`FMLSL`/`FMLSL2` -- widen each
/// half-precision (`.2h`/`.4h`) source lane and multiply-accumulate into the single-precision (`.2s`/`.4s`)
/// destination. The `2` forms read the upper half-precision lanes; `FMLSL*` subtract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorFmlalOp {
    /// `FMLAL` -- multiply-accumulate long (lower lanes).
    Fmlal,
    /// `FMLAL2` -- multiply-accumulate long (upper lanes).
    Fmlal2,
    /// `FMLSL` -- multiply-subtract long (lower lanes).
    Fmlsl,
    /// `FMLSL2` -- multiply-subtract long (upper lanes).
    Fmlsl2,
}

impl Arm64VectorFmlalOp {
    /// The base word with `Q = 0` and zero registers; the encoder ORs `Q<<30`, `Rm<<16`, `Rn<<5`, `Rd`.
    /// GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Fmlal => 0x0E20_EC00,
            Self::Fmlal2 => 0x2E20_CC00,
            Self::Fmlsl => 0x0EA0_EC00,
            Self::Fmlsl2 => 0x2EA0_CC00,
        }
    }

    /// The BY-ELEMENT base word (`Q = 0`, zero registers/index): the indexed-element encoding (op0 `01111`, `size = 10`),
    /// distinct from the three-same [`Self::base`]. The encoder ORs `Q<<30`, the L/M/Rm/H index fold, `Rn<<5`, `Rd`.
    /// The `2` forms carry U=1 *and* opcode-bit-15; `FMLSL*` set opcode bit 14. GNU+LLVM dual-oracle verified.
    pub fn by_element_base(self) -> u32 {
        match self {
            Self::Fmlal => 0x0F80_0000,
            Self::Fmlal2 => 0x2F80_8000,
            Self::Fmlsl => 0x0F80_4000,
            Self::Fmlsl2 => 0x2F80_C000,
        }
    }

    /// Select the op from the `sub` (FMLSL vs FMLAL) and `second` (the `2`, upper-lanes) flags.
    pub fn from_sub_and_second(sub: bool, second: bool) -> Self {
        match (sub, second) {
            (false, false) => Self::Fmlal,
            (false, true) => Self::Fmlal2,
            (true, false) => Self::Fmlsl,
            (true, true) => Self::Fmlsl2,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fmlal => "fmlal",
            Self::Fmlal2 => "fmlal2",
            Self::Fmlsl => "fmlsl",
            Self::Fmlsl2 => "fmlsl2",
        }
    }

    /// Recover the op from a masked base (`word & 0xBFE0_FC00`); `None` if not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 4] = [Self::Fmlal, Self::Fmlal2, Self::Fmlsl, Self::Fmlsl2];
}
