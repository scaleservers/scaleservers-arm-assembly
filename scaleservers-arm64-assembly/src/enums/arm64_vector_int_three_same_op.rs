// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **integer** "three same" lane instruction -- the encoding
/// class `0 Q U 01110 size 1 Rm opcode 1 Rn Rd` (DDI0487 C7) that covers per-lane arithmetic, min/max, the
/// unsigned rounding halving add, the register compares, and the variable shifts. Every member shares the one
/// operand shape `{ arrangement, Vd, Vn, Vm }`; the operation is an orthogonal field over it (`U` + the 5-bit
/// `opcode`). This is the ratified family shape for combinatorial vector families -- one `op` sub-enum + typed
/// operand fields -- which collapses what would otherwise be hundreds of per-(mnemonic x arrangement) variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorIntThreeSameOp {
    /// `ADD` -- per-lane add (`U = 0`, opcode `10000`).
    Add,
    /// `SUB` -- per-lane subtract (`U = 1`, opcode `10000`).
    Sub,
    /// `MUL` -- per-lane multiply (`U = 0`, opcode `10011`; no 64-bit `.1d`/`.2d` form).
    Mul,
    /// `SMAX` -- per-lane signed maximum (`U = 0`, opcode `01100`; no 64-bit form).
    Smax,
    /// `SMIN` -- per-lane signed minimum (`U = 0`, opcode `01101`; no 64-bit form).
    Smin,
    /// `UMAX` -- per-lane unsigned maximum (`U = 1`, opcode `01100`; no 64-bit form).
    Umax,
    /// `UMIN` -- per-lane unsigned minimum (`U = 1`, opcode `01101`; no 64-bit form).
    Umin,
    /// `URHADD` -- per-lane unsigned rounding halving add (`U = 1`, opcode `00010`; no 64-bit form).
    Urhadd,
    /// `CMEQ` (register) -- per-lane compare-equal yielding an all-ones/all-zero lane mask (`U = 1`, opcode `10001`).
    CmEq,
    /// `CMGT` (register) -- per-lane signed compare greater-than (`U = 0`, opcode `00110`).
    CmGt,
    /// `CMGE` (register) -- per-lane signed compare greater-or-equal (`U = 0`, opcode `00111`).
    CmGe,
    /// `CMHI` (register) -- per-lane unsigned compare higher (`U = 1`, opcode `00110`).
    CmHi,
    /// `CMHS` (register) -- per-lane unsigned compare higher-or-same (`U = 1`, opcode `00111`).
    CmHs,
    /// `SSHL` (register) -- per-lane signed variable shift-left (negative count shifts right) (`U = 0`, opcode `01000`).
    Sshl,
    /// `USHL` (register) -- per-lane unsigned variable shift-left (`U = 1`, opcode `01000`).
    Ushl,
    /// `SQADD` -- per-lane signed saturating add (`U = 0`, opcode `00001`).
    Sqadd,
    /// `UQADD` -- per-lane unsigned saturating add (`U = 1`, opcode `00001`).
    Uqadd,
    /// `SQSUB` -- per-lane signed saturating subtract (`U = 0`, opcode `00101`).
    Sqsub,
    /// `UQSUB` -- per-lane unsigned saturating subtract (`U = 1`, opcode `00101`).
    Uqsub,
    /// `SHADD` -- per-lane signed halving add (`U = 0`, opcode `00000`; no 64-bit form).
    Shadd,
    /// `UHADD` -- per-lane unsigned halving add (`U = 1`, opcode `00000`; no 64-bit form).
    Uhadd,
    /// `SHSUB` -- per-lane signed halving subtract (`U = 0`, opcode `00100`; no 64-bit form).
    Shsub,
    /// `UHSUB` -- per-lane unsigned halving subtract (`U = 1`, opcode `00100`; no 64-bit form).
    Uhsub,
    /// `SRHADD` -- per-lane signed rounding halving add (`U = 0`, opcode `00010`; no 64-bit form).
    Srhadd,
    /// `SQSHL` (register) -- per-lane signed saturating variable shift-left (`U = 0`, opcode `01001`).
    Sqshl,
    /// `UQSHL` (register) -- per-lane unsigned saturating variable shift-left (`U = 1`, opcode `01001`).
    Uqshl,
    /// `SRSHL` (register) -- per-lane signed rounding variable shift-left (`U = 0`, opcode `01010`).
    Srshl,
    /// `URSHL` (register) -- per-lane unsigned rounding variable shift-left (`U = 1`, opcode `01010`).
    Urshl,
    /// `SQRSHL` (register) -- per-lane signed saturating rounding shift-left (`U = 0`, opcode `01011`).
    Sqrshl,
    /// `UQRSHL` (register) -- per-lane unsigned saturating rounding shift-left (`U = 1`, opcode `01011`).
    Uqrshl,
    /// `SABD` -- per-lane signed absolute difference (`U = 0`, opcode `01110`; no 64-bit form).
    Sabd,
    /// `UABD` -- per-lane unsigned absolute difference (`U = 1`, opcode `01110`; no 64-bit form).
    Uabd,
    /// `SABA` -- per-lane signed absolute-difference accumulate (`U = 0`, opcode `01111`; no 64-bit form).
    Saba,
    /// `UABA` -- per-lane unsigned absolute-difference accumulate (`U = 1`, opcode `01111`; no 64-bit form).
    Uaba,
    /// `CMTST` (register) -- per-lane test-bits (AND-nonzero) yielding a lane mask (`U = 0`, opcode `10001`).
    CmTst,
    /// `MLA` (vector) -- per-lane multiply-accumulate (`U = 0`, opcode `10010`; no 64-bit form).
    Mla,
    /// `MLS` (vector) -- per-lane multiply-subtract (`U = 1`, opcode `10010`; no 64-bit form).
    Mls,
    /// `SMAXP` -- per-lane signed pairwise maximum (`U = 0`, opcode `10100`; no 64-bit form).
    Smaxp,
    /// `UMAXP` -- per-lane unsigned pairwise maximum (`U = 1`, opcode `10100`; no 64-bit form).
    Umaxp,
    /// `SMINP` -- per-lane signed pairwise minimum (`U = 0`, opcode `10101`; no 64-bit form).
    Sminp,
    /// `UMINP` -- per-lane unsigned pairwise minimum (`U = 1`, opcode `10101`; no 64-bit form).
    Uminp,
    /// `ADDP` (vector) -- per-lane pairwise add (`U = 0`, opcode `10111`).
    Addp,
    /// `PMUL` -- per-lane polynomial multiply (`U = 1`, opcode `10011`; byte `.8b`/`.16b` only).
    Pmul,
    /// `SQDMULH` -- per-lane signed saturating doubling multiply, high half (`U = 0`, opcode `10110`;
    /// half/word `.4h`/`.8h`/`.2s`/`.4s` only).
    Sqdmulh,
    /// `SQRDMULH` -- per-lane signed saturating rounding doubling multiply, high half (`U = 1`, opcode `10110`;
    /// half/word only).
    Sqrdmulh,
}

impl Arm64VectorIntThreeSameOp {
    /// The `size = 0`, `Q = 0` base word (`U` and the 5-bit opcode baked in); the arrangement supplies `Q<<30`
    /// and `size<<22`, and the registers `Vm<<16 | Vn<<5 | Vd`. All bases are GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Add => 0x0E20_8400,
            Self::Sub => 0x2E20_8400,
            Self::Mul => 0x0E20_9C00,
            Self::Smax => 0x0E20_6400,
            Self::Smin => 0x0E20_6C00,
            Self::Umax => 0x2E20_6400,
            Self::Umin => 0x2E20_6C00,
            Self::Urhadd => 0x2E20_1400,
            Self::CmEq => 0x2E20_8C00,
            Self::CmGt => 0x0E20_3400,
            Self::CmGe => 0x0E20_3C00,
            Self::CmHi => 0x2E20_3400,
            Self::CmHs => 0x2E20_3C00,
            Self::Sshl => 0x0E20_4400,
            Self::Ushl => 0x2E20_4400,
            Self::Sqadd => 0x0E20_0C00,
            Self::Uqadd => 0x2E20_0C00,
            Self::Sqsub => 0x0E20_2C00,
            Self::Uqsub => 0x2E20_2C00,
            Self::Shadd => 0x0E20_0400,
            Self::Uhadd => 0x2E20_0400,
            Self::Shsub => 0x0E20_2400,
            Self::Uhsub => 0x2E20_2400,
            Self::Srhadd => 0x0E20_1400,
            Self::Sqshl => 0x0E20_4C00,
            Self::Uqshl => 0x2E20_4C00,
            Self::Srshl => 0x0E20_5400,
            Self::Urshl => 0x2E20_5400,
            Self::Sqrshl => 0x0E20_5C00,
            Self::Uqrshl => 0x2E20_5C00,
            Self::Sabd => 0x0E20_7400,
            Self::Uabd => 0x2E20_7400,
            Self::Saba => 0x0E20_7C00,
            Self::Uaba => 0x2E20_7C00,
            Self::CmTst => 0x0E20_8C00,
            Self::Mla => 0x0E20_9400,
            Self::Mls => 0x2E20_9400,
            Self::Smaxp => 0x0E20_A400,
            Self::Umaxp => 0x2E20_A400,
            Self::Sminp => 0x0E20_AC00,
            Self::Uminp => 0x2E20_AC00,
            Self::Addp => 0x0E20_BC00,
            Self::Pmul => 0x2E20_9C00,
            Self::Sqdmulh => 0x0E20_B400,
            Self::Sqrdmulh => 0x2E20_B400,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Sub => "sub",
            Self::Mul => "mul",
            Self::Smax => "smax",
            Self::Smin => "smin",
            Self::Umax => "umax",
            Self::Umin => "umin",
            Self::Urhadd => "urhadd",
            Self::CmEq => "cmeq",
            Self::CmGt => "cmgt",
            Self::CmGe => "cmge",
            Self::CmHi => "cmhi",
            Self::CmHs => "cmhs",
            Self::Sshl => "sshl",
            Self::Ushl => "ushl",
            Self::Sqadd => "sqadd",
            Self::Uqadd => "uqadd",
            Self::Sqsub => "sqsub",
            Self::Uqsub => "uqsub",
            Self::Shadd => "shadd",
            Self::Uhadd => "uhadd",
            Self::Shsub => "shsub",
            Self::Uhsub => "uhsub",
            Self::Srhadd => "srhadd",
            Self::Sqshl => "sqshl",
            Self::Uqshl => "uqshl",
            Self::Srshl => "srshl",
            Self::Urshl => "urshl",
            Self::Sqrshl => "sqrshl",
            Self::Uqrshl => "uqrshl",
            Self::Sabd => "sabd",
            Self::Uabd => "uabd",
            Self::Saba => "saba",
            Self::Uaba => "uaba",
            Self::CmTst => "cmtst",
            Self::Mla => "mla",
            Self::Mls => "mls",
            Self::Smaxp => "smaxp",
            Self::Umaxp => "umaxp",
            Self::Sminp => "sminp",
            Self::Uminp => "uminp",
            Self::Addp => "addp",
            Self::Pmul => "pmul",
            Self::Sqdmulh => "sqdmulh",
            Self::Sqrdmulh => "sqrdmulh",
        }
    }

    /// Whether this operation has a valid encoding for the 2-bit element `size` field (`00`=byte, `01`=half,
    /// `10`=word, `11`=doubleword). Most ops cover byte/half/word but reserve `size = 11`; Add/Sub, the
    /// saturating add/sub, the compares, every variable shift, `CMTST`, and `ADDP` add the doubleword form;
    /// `PMUL` is byte-only; `SQDMULH`/`SQRDMULH` are half/word-only (no byte, no doubleword).
    pub fn allows_size(self, size_bits: u32) -> bool {
        match self {
            Self::Pmul => size_bits == 0b00,
            Self::Sqdmulh | Self::Sqrdmulh => matches!(size_bits, 0b01 | 0b10),
            Self::Add
            | Self::Sub
            | Self::CmEq
            | Self::CmGt
            | Self::CmGe
            | Self::CmHi
            | Self::CmHs
            | Self::Sshl
            | Self::Ushl
            | Self::Sqadd
            | Self::Uqadd
            | Self::Sqsub
            | Self::Uqsub
            | Self::Sqshl
            | Self::Uqshl
            | Self::Srshl
            | Self::Urshl
            | Self::Sqrshl
            | Self::Uqrshl
            | Self::CmTst
            | Self::Addp => true,
            _ => size_bits != 0b11,
        }
    }

    /// Whether this operation is valid for the 64-bit-element (`.1d`/`.2d`) arrangements (`size = 11`).
    /// A convenience over [`allows_size`](Self::allows_size).
    pub fn allows_doubleword(self) -> bool {
        self.allows_size(0b11)
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 45] = [
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Smax,
        Self::Smin,
        Self::Umax,
        Self::Umin,
        Self::Urhadd,
        Self::CmEq,
        Self::CmGt,
        Self::CmGe,
        Self::CmHi,
        Self::CmHs,
        Self::Sshl,
        Self::Ushl,
        Self::Sqadd,
        Self::Uqadd,
        Self::Sqsub,
        Self::Uqsub,
        Self::Shadd,
        Self::Uhadd,
        Self::Shsub,
        Self::Uhsub,
        Self::Srhadd,
        Self::Sqshl,
        Self::Uqshl,
        Self::Srshl,
        Self::Urshl,
        Self::Sqrshl,
        Self::Uqrshl,
        Self::Sabd,
        Self::Uabd,
        Self::Saba,
        Self::Uaba,
        Self::CmTst,
        Self::Mla,
        Self::Mls,
        Self::Smaxp,
        Self::Umaxp,
        Self::Sminp,
        Self::Uminp,
        Self::Addp,
        Self::Pmul,
        Self::Sqdmulh,
        Self::Sqrdmulh,
    ];
}
