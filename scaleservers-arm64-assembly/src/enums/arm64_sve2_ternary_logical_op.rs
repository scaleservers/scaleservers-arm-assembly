// Copyright (c) Scaleservers LLC

/// An SVE2 **bitwise ternary** op (DDI0487 part C): a three-source 64-bit logical operation `<op> Zdn.D, Zdn.D,
/// Zm.D, Zk.D` (FEAT_SVE2). The op is selected by `opc` at `[23:22]` and the `[10]` group bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2TernaryLogicalOp {
    /// `EOR3` -- three-way exclusive-OR (`opc`=00, `[10]`=0).
    Eor3,
    /// `BCAX` -- bit-clear and exclusive-OR (`opc`=01, `[10]`=0).
    Bcax,
    /// `BSL` -- bitwise select (`opc`=00, `[10]`=1).
    Bsl,
    /// `BSL1N` -- bitwise select with first input inverted (`opc`=01, `[10]`=1).
    Bsl1n,
    /// `BSL2N` -- bitwise select with second input inverted (`opc`=10, `[10]`=1).
    Bsl2n,
    /// `NBSL` -- bitwise select then invert (`opc`=11, `[10]`=1).
    Nbsl,
}

impl Arm64Sve2TernaryLogicalOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Eor3 => "eor3",
            Self::Bcax => "bcax",
            Self::Bsl => "bsl",
            Self::Bsl1n => "bsl1n",
            Self::Bsl2n => "bsl2n",
            Self::Nbsl => "nbsl",
        }
    }

    /// The 2-bit `opc` field (`[23:22]`).
    pub fn opc(self) -> u32 {
        match self {
            Self::Eor3 | Self::Bsl => 0b00,
            Self::Bcax | Self::Bsl1n => 0b01,
            Self::Bsl2n => 0b10,
            Self::Nbsl => 0b11,
        }
    }

    /// The `[10]` group bit (0 for the `EOR3`/`BCAX` pair, 1 for the `BSL` family).
    pub fn group_bit(self) -> u32 {
        match self {
            Self::Eor3 | Self::Bcax => 0,
            Self::Bsl | Self::Bsl1n | Self::Bsl2n | Self::Nbsl => 1,
        }
    }

    /// Recover the op from its `opc` (`[23:22]`) and group bit (`[10]`), or `None` for an unallocated combination
    /// (the `EOR3`/`BCAX` group only defines `opc` 00/01).
    pub fn from_bits(opc: u32, group_bit: u32) -> Option<Self> {
        match (opc & 0b11, group_bit & 1) {
            (0b00, 0) => Some(Self::Eor3),
            (0b01, 0) => Some(Self::Bcax),
            (0b00, 1) => Some(Self::Bsl),
            (0b01, 1) => Some(Self::Bsl1n),
            (0b10, 1) => Some(Self::Bsl2n),
            (0b11, 1) => Some(Self::Nbsl),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 6] = [
        Self::Eor3,
        Self::Bcax,
        Self::Bsl,
        Self::Bsl1n,
        Self::Bsl2n,
        Self::Nbsl,
    ];
}
