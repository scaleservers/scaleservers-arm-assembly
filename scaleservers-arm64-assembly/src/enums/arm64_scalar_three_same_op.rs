// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **three-same** integer op (DDI0487 C7, the `01 U 11110 size 1 Rm opcode 1 Rn Rd`
/// encoding) -- the scalar (single `b`/`h`/`s`/`d` register) counterparts of the vector three-same ops. The op
/// is orthogonal over the element size; each op allows only some sizes ([`Self::allows_size`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarThreeSameOp {
    /// `SQADD` -- signed saturating add (all sizes).
    Sqadd,
    /// `UQADD` -- unsigned saturating add (all sizes).
    Uqadd,
    /// `SQSUB` -- signed saturating subtract (all sizes).
    Sqsub,
    /// `UQSUB` -- unsigned saturating subtract (all sizes).
    Uqsub,
    /// `CMGT` -- compare signed greater-than (`.d` only).
    Cmgt,
    /// `CMHI` -- compare unsigned higher (`.d` only).
    Cmhi,
    /// `CMGE` -- compare signed greater-or-equal (`.d` only).
    Cmge,
    /// `CMHS` -- compare unsigned higher-or-same (`.d` only).
    Cmhs,
    /// `SSHL` -- signed shift left (register) (`.d` only).
    Sshl,
    /// `USHL` -- unsigned shift left (register) (`.d` only).
    Ushl,
    /// `SQSHL` -- signed saturating shift left (register) (all sizes).
    Sqshl,
    /// `UQSHL` -- unsigned saturating shift left (register) (all sizes).
    Uqshl,
    /// `SRSHL` -- signed rounding shift left (register) (`.d` only).
    Srshl,
    /// `URSHL` -- unsigned rounding shift left (register) (`.d` only).
    Urshl,
    /// `SQRSHL` -- signed saturating rounding shift left (register) (all sizes).
    Sqrshl,
    /// `UQRSHL` -- unsigned saturating rounding shift left (register) (all sizes).
    Uqrshl,
    /// `ADD` -- add (`.d` only).
    Add,
    /// `SUB` -- subtract (`.d` only).
    Sub,
    /// `CMTST` -- compare bitwise test (`.d` only).
    Cmtst,
    /// `CMEQ` -- compare bitwise equal (`.d` only).
    Cmeq,
    /// `SQDMULH` -- signed saturating doubling multiply returning high half (`.h`/`.s`).
    Sqdmulh,
    /// `SQRDMULH` -- signed saturating rounding doubling multiply high (`.h`/`.s`).
    Sqrdmulh,
}

impl Arm64ScalarThreeSameOp {
    /// The base word (`size`/`Rm`/`Rn`/`Rd` zero): `0x5E20_0400 | (U<<29) | (opcode<<11)`. GNU+LLVM dual-oracle
    /// verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Sqadd => (0, 0b00001),
            Self::Uqadd => (1, 0b00001),
            Self::Sqsub => (0, 0b00101),
            Self::Uqsub => (1, 0b00101),
            Self::Cmgt => (0, 0b00110),
            Self::Cmhi => (1, 0b00110),
            Self::Cmge => (0, 0b00111),
            Self::Cmhs => (1, 0b00111),
            Self::Sshl => (0, 0b01000),
            Self::Ushl => (1, 0b01000),
            Self::Sqshl => (0, 0b01001),
            Self::Uqshl => (1, 0b01001),
            Self::Srshl => (0, 0b01010),
            Self::Urshl => (1, 0b01010),
            Self::Sqrshl => (0, 0b01011),
            Self::Uqrshl => (1, 0b01011),
            Self::Add => (0, 0b10000),
            Self::Sub => (1, 0b10000),
            Self::Cmtst => (0, 0b10001),
            Self::Cmeq => (1, 0b10001),
            Self::Sqdmulh => (0, 0b10110),
            Self::Sqrdmulh => (1, 0b10110),
        };
        0x5E20_0400 | (u << 29) | (opcode << 11)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sqadd => "sqadd",
            Self::Uqadd => "uqadd",
            Self::Sqsub => "sqsub",
            Self::Uqsub => "uqsub",
            Self::Cmgt => "cmgt",
            Self::Cmhi => "cmhi",
            Self::Cmge => "cmge",
            Self::Cmhs => "cmhs",
            Self::Sshl => "sshl",
            Self::Ushl => "ushl",
            Self::Sqshl => "sqshl",
            Self::Uqshl => "uqshl",
            Self::Srshl => "srshl",
            Self::Urshl => "urshl",
            Self::Sqrshl => "sqrshl",
            Self::Uqrshl => "uqrshl",
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Cmtst => "cmtst",
            Self::Cmeq => "cmeq",
            Self::Sqdmulh => "sqdmulh",
            Self::Sqrdmulh => "sqrdmulh",
        }
    }

    /// Whether this op allocates the given 2-bit element size: the saturating add/sub + saturating shifts allow
    /// every size; `SQDMULH`/`SQRDMULH` allow only `.h`/`.s` (1/2); everything else is `.d`-only (3).
    pub fn allows_size(self, size: u32) -> bool {
        match self {
            Self::Sqadd
            | Self::Uqadd
            | Self::Sqsub
            | Self::Uqsub
            | Self::Sqshl
            | Self::Uqshl
            | Self::Sqrshl
            | Self::Uqrshl => size <= 0b11,
            Self::Sqdmulh | Self::Sqrdmulh => size == 0b01 || size == 0b10,
            _ => size == 0b11,
        }
    }

    /// Recover the op from a masked base (`word & 0xFF20_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 22] = [
        Self::Sqadd,
        Self::Uqadd,
        Self::Sqsub,
        Self::Uqsub,
        Self::Cmgt,
        Self::Cmhi,
        Self::Cmge,
        Self::Cmhs,
        Self::Sshl,
        Self::Ushl,
        Self::Sqshl,
        Self::Uqshl,
        Self::Srshl,
        Self::Urshl,
        Self::Sqrshl,
        Self::Uqrshl,
        Self::Add,
        Self::Sub,
        Self::Cmtst,
        Self::Cmeq,
        Self::Sqdmulh,
        Self::Sqrdmulh,
    ];
}
