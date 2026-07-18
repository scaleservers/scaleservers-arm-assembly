// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **by-element (indexed)** op (DDI0487 C7, the `01 U 11111 size L M Rm opcode H 0 Rn Rd`
/// encoding) -- the same-size scalar multiplies against a broadcast lane `Vm.<ts>[index]`. The integer ops are
/// `.h`/`.s`; the FP ops are `.s`/`.d`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarByElementOp {
    /// `SQDMULH` -- saturating doubling multiply returning high half (`.h`/`.s`).
    Sqdmulh,
    /// `SQRDMULH` -- saturating rounding doubling multiply high (`.h`/`.s`).
    Sqrdmulh,
    /// `SQRDMLAH` -- saturating rounding doubling multiply-accumulate high (`.h`/`.s`, FEAT_RDM).
    Sqrdmlah,
    /// `SQRDMLSH` -- saturating rounding doubling multiply-subtract high (`.h`/`.s`, FEAT_RDM).
    Sqrdmlsh,
    /// `FMUL` -- floating-point multiply (`.s`/`.d`).
    Fmul,
    /// `FMLA` -- floating-point fused multiply-add (`.s`/`.d`).
    Fmla,
    /// `FMLS` -- floating-point fused multiply-subtract (`.s`/`.d`).
    Fmls,
    /// `FMULX` -- floating-point multiply extended (`.s`/`.d`).
    Fmulx,
}

impl Arm64ScalarByElementOp {
    /// The base word (`size`/index/`Rm`/`Rn`/`Rd` zero): `0x5F00_0000 | (U<<29) | (opcode<<12)`. GNU+LLVM
    /// dual-oracle verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Fmla => (0, 0b0001),
            Self::Fmls => (0, 0b0101),
            Self::Fmul => (0, 0b1001),
            Self::Sqdmulh => (0, 0b1100),
            Self::Sqrdmulh => (0, 0b1101),
            Self::Fmulx => (1, 0b1001),
            Self::Sqrdmlah => (1, 0b1101),
            Self::Sqrdmlsh => (1, 0b1111),
        };
        0x5F00_0000 | (u << 29) | (opcode << 12)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sqdmulh => "sqdmulh",
            Self::Sqrdmulh => "sqrdmulh",
            Self::Sqrdmlah => "sqrdmlah",
            Self::Sqrdmlsh => "sqrdmlsh",
            Self::Fmul => "fmul",
            Self::Fmla => "fmla",
            Self::Fmls => "fmls",
            Self::Fmulx => "fmulx",
        }
    }

    /// Whether this op allocates the given 2-bit element size: the integer ops are `.h`/`.s` (1/2), the FP ops are
    /// `.s`/`.d` (2/3).
    pub fn allows_size(self, size: u32) -> bool {
        match self {
            Self::Sqdmulh | Self::Sqrdmulh | Self::Sqrdmlah | Self::Sqrdmlsh => {
                size == 0b01 || size == 0b10
            }
            Self::Fmul | Self::Fmla | Self::Fmls | Self::Fmulx => size == 0b10 || size == 0b11,
        }
    }

    /// Recover the op from a masked base (`word & 0xFF00_F400`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 8] = [
        Self::Sqdmulh,
        Self::Sqrdmulh,
        Self::Sqrdmlah,
        Self::Sqrdmlsh,
        Self::Fmul,
        Self::Fmla,
        Self::Fmls,
        Self::Fmulx,
    ];
}

/// A scalar Advanced SIMD **by-element long** op (DDI0487 C7) -- `SQDMULL`/`SQDMLAL`/`SQDMLSL` against a broadcast
/// lane, where the destination is one element size wider than the source (`s<-h`, `d<-s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarByElementLongOp {
    /// `SQDMULL` -- saturating doubling multiply long.
    Sqdmull,
    /// `SQDMLAL` -- saturating doubling multiply-add long.
    Sqdmlal,
    /// `SQDMLSL` -- saturating doubling multiply-subtract long.
    Sqdmlsl,
}

impl Arm64ScalarByElementLongOp {
    /// The base word (`size`/index/`Rm`/`Rn`/`Rd` zero): `0x5F00_0000 | (opcode<<12)`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        let opcode: u32 = match self {
            Self::Sqdmlal => 0b0011,
            Self::Sqdmlsl => 0b0111,
            Self::Sqdmull => 0b1011,
        };
        0x5F00_0000 | (opcode << 12)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sqdmull => "sqdmull",
            Self::Sqdmlal => "sqdmlal",
            Self::Sqdmlsl => "sqdmlsl",
        }
    }

    /// Recover the op from a masked base (`word & 0xFF00_F400`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Sqdmull, Self::Sqdmlal, Self::Sqdmlsl];
}
