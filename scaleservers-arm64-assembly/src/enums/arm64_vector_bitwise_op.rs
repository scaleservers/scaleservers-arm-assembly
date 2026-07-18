// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **bitwise / logical** "three same" instruction -- the
/// encoding class `0 Q U 01110 size 1 Rm 00011 1 Rn Rd` (DDI0487 C7) where the 5-bit `opcode` is fixed at
/// `00011` and the `size` field is *repurposed as the operation selector* (together with `U`). Unlike the
/// element-wise three-same families these ops are size-agnostic: the only operand shape is `{ Q, Vd, Vn, Vm }`
/// -- register width `.8b` (64-bit) or `.16b` (128-bit), with no element size -- so the arrangement contributes
/// only the `Q` bit and the operation is the orthogonal `(U, size-selector)` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorBitwiseOp {
    /// `AND` -- bitwise and (`U = 0`, selector `00`).
    And,
    /// `BIC` (register) -- bitwise and-with-complement, `Vn AND NOT Vm` (`U = 0`, selector `01`).
    Bic,
    /// `ORR` (register) -- bitwise or (`U = 0`, selector `10`).
    Orr,
    /// `ORN` (register) -- bitwise or-with-complement, `Vn OR NOT Vm` (`U = 0`, selector `11`).
    Orn,
    /// `EOR` -- bitwise exclusive-or (`U = 1`, selector `00`).
    Eor,
    /// `BSL` -- bitwise select: per bit, take `Vn` where `Vd` is 1 else `Vm` (`U = 1`, selector `01`).
    Bsl,
    /// `BIT` -- bitwise insert if true: copy `Vn` bits into `Vd` where `Vm` is 1 (`U = 1`, selector `10`).
    Bit,
    /// `BIF` -- bitwise insert if false: copy `Vn` bits into `Vd` where `Vm` is 0 (`U = 1`, selector `11`).
    Bif,
}

impl Arm64VectorBitwiseOp {
    /// The `Q = 0` base word (`U`, the size-selector, and the fixed `00011` opcode baked in); the arrangement
    /// supplies `Q<<30` and the registers `Vm<<16 | Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::And => 0x0E20_1C00,
            Self::Bic => 0x0E60_1C00,
            Self::Orr => 0x0EA0_1C00,
            Self::Orn => 0x0EE0_1C00,
            Self::Eor => 0x2E20_1C00,
            Self::Bsl => 0x2E60_1C00,
            Self::Bit => 0x2EA0_1C00,
            Self::Bif => 0x2EE0_1C00,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::Bic => "bic",
            Self::Orr => "orr",
            Self::Orn => "orn",
            Self::Eor => "eor",
            Self::Bsl => "bsl",
            Self::Bit => "bit",
            Self::Bif => "bif",
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 8] = [
        Self::And,
        Self::Bic,
        Self::Orr,
        Self::Orn,
        Self::Eor,
        Self::Bsl,
        Self::Bit,
        Self::Bif,
    ];
}
