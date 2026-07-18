// Copyright (c) Scaleservers LLC

/// An SVE2.1 **across-lanes quadword integer reduction** op (FEAT_SVE2p1; DDI0487 part C). Reduces each 128-bit
/// segment of the source `Zn` to one lane of a 128-bit NEON `Vd` register under a governing predicate. Shares the
/// integer-reduction encoding group (`0x0400_2000 | size<<22 | opcode<<16 | Pg<<10 | Zn<<5 | Vd`) with `SADDV`/etc.,
/// but with a disjoint `[20:16]` opcode set. GNU+LLVM verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveQuadReduceIntOp {
    /// `ADDQV` -- add reduction (per 128-bit segment).
    AddQV,
    /// `SMAXQV` -- signed maximum reduction.
    SmaxQV,
    /// `UMAXQV` -- unsigned maximum reduction.
    UmaxQV,
    /// `SMINQV` -- signed minimum reduction.
    SminQV,
    /// `UMINQV` -- unsigned minimum reduction.
    UminQV,
    /// `ORQV` -- bitwise OR reduction.
    OrQV,
    /// `EORQV` -- bitwise exclusive-OR reduction.
    EorQV,
    /// `ANDQV` -- bitwise AND reduction.
    AndQV,
}

impl Arm64SveQuadReduceIntOp {
    /// The 5-bit `opcode` field (`[20:16]`).
    pub fn opcode(self) -> u32 {
        match self {
            Self::AddQV => 0x05,
            Self::SmaxQV => 0x0C,
            Self::UmaxQV => 0x0D,
            Self::SminQV => 0x0E,
            Self::UminQV => 0x0F,
            Self::OrQV => 0x1C,
            Self::EorQV => 0x1D,
            Self::AndQV => 0x1E,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::AddQV => "addqv",
            Self::SmaxQV => "smaxqv",
            Self::UmaxQV => "umaxqv",
            Self::SminQV => "sminqv",
            Self::UminQV => "uminqv",
            Self::OrQV => "orqv",
            Self::EorQV => "eorqv",
            Self::AndQV => "andqv",
        }
    }

    /// Recover the op from its `opcode`, if a modeled QV op (the non-QV reduction opcodes return `None`).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.opcode() == opcode & 0x1F)
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 8] = [
        Self::AddQV,
        Self::SmaxQV,
        Self::UmaxQV,
        Self::SminQV,
        Self::UminQV,
        Self::OrQV,
        Self::EorQV,
        Self::AndQV,
    ];
}
