// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **floating-point** "three same" lane instruction -- the
/// encoding class `0 Q U 01110 E sz 1 Rm opcode 1 Rn Rd` (DDI0487 C7) covering per-lane arithmetic, the
/// NaN-propagating min/max, and the register compares. Like the integer family the op is an orthogonal field
/// over the shared `{ arrangement, Vd, Vn, Vm }` shape; all members are valid only for the `.2s`/`.4s`/`.2d`
/// arrangements, and the `sz` bit (single vs double) is the arrangement's
/// [`super::Arm64VectorArrangement::fp_sz_bit`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorFpThreeSameOp {
    /// `FADD` -- per-lane add (`U = 0`, `E = 0`, opcode `11010`).
    FAdd,
    /// `FSUB` -- per-lane subtract (`U = 0`, `E = 1`, opcode `11010`).
    FSub,
    /// `FMUL` -- per-lane multiply (`U = 1`, `E = 0`, opcode `11011`).
    FMul,
    /// `FDIV` -- per-lane divide (`U = 1`, `E = 0`, opcode `11111`).
    FDiv,
    /// `FMAX` -- per-lane maximum, NaN-propagating (`U = 0`, `E = 0`, opcode `11110`).
    FMax,
    /// `FMIN` -- per-lane minimum, NaN-propagating (`U = 0`, `E = 1`, opcode `11110`).
    FMin,
    /// `FCMEQ` (register) -- per-lane compare-equal -> lane mask (`U = 0`, `E = 0`, opcode `11100`).
    FCmEq,
    /// `FCMGE` (register) -- per-lane compare greater-or-equal -> lane mask (`U = 1`, `E = 0`, opcode `11100`).
    FCmGe,
    /// `FCMGT` (register) -- per-lane compare greater-than -> lane mask (`U = 1`, `E = 1`, opcode `11100`).
    FCmGt,
    /// `FMAXNM` -- per-lane maximum-number (NaN yields the number) (`U = 0`, `E = 0`, opcode `11000`).
    FMaxNm,
    /// `FMINNM` -- per-lane minimum-number (`U = 0`, `E = 1`, opcode `11000`).
    FMinNm,
    /// `FMAXNMP` -- pairwise maximum-number (`U = 1`, `E = 0`, opcode `11000`).
    FMaxNmp,
    /// `FMINNMP` -- pairwise minimum-number (`U = 1`, `E = 1`, opcode `11000`).
    FMinNmp,
    /// `FMLA` -- per-lane fused multiply-add (`U = 0`, `E = 0`, opcode `11001`).
    FMla,
    /// `FMLS` -- per-lane fused multiply-subtract (`U = 0`, `E = 1`, opcode `11001`).
    FMls,
    /// `FABD` -- per-lane absolute difference (`U = 1`, `E = 1`, opcode `11010`).
    FAbd,
    /// `FADDP` (vector) -- per-lane pairwise add (`U = 1`, `E = 0`, opcode `11010`).
    FAddp,
    /// `FMULX` -- per-lane multiply-extended (`U = 0`, `E = 0`, opcode `11011`).
    FMulx,
    /// `FACGE` -- per-lane absolute compare greater-or-equal (`U = 1`, `E = 0`, opcode `11101`).
    FAcGe,
    /// `FACGT` -- per-lane absolute compare greater-than (`U = 1`, `E = 1`, opcode `11101`).
    FAcGt,
    /// `FMAXP` (vector) -- per-lane pairwise maximum (`U = 1`, `E = 0`, opcode `11110`).
    FMaxp,
    /// `FMINP` (vector) -- per-lane pairwise minimum (`U = 1`, `E = 1`, opcode `11110`).
    FMinp,
    /// `FRECPS` -- per-lane reciprocal step (`U = 0`, `E = 0`, opcode `11111`).
    FRecps,
    /// `FRSQRTS` -- per-lane reciprocal square-root step (`U = 0`, `E = 1`, opcode `11111`).
    FRsqrts,
    /// `FAMAX` -- per-lane absolute maximum (FEAT_FAMINMAX).
    FAmax,
    /// `FAMIN` -- per-lane absolute minimum (FEAT_FAMINMAX).
    FAmin,
}

impl Arm64VectorFpThreeSameOp {
    /// The base word with `Q = 0` and `sz = 0` (`U`, the `E` bit23, and opcode baked in); the arrangement
    /// supplies `Q<<30` and `sz<<22`, and the registers `Vm<<16 | Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::FAdd => 0x0E20_D400,
            Self::FSub => 0x0EA0_D400,
            Self::FMul => 0x2E20_DC00,
            Self::FDiv => 0x2E20_FC00,
            Self::FMax => 0x0E20_F400,
            Self::FMin => 0x0EA0_F400,
            Self::FCmEq => 0x0E20_E400,
            Self::FCmGe => 0x2E20_E400,
            Self::FCmGt => 0x2EA0_E400,
            Self::FMaxNm => 0x0E20_C400,
            Self::FMinNm => 0x0EA0_C400,
            Self::FMaxNmp => 0x2E20_C400,
            Self::FMinNmp => 0x2EA0_C400,
            Self::FMla => 0x0E20_CC00,
            Self::FMls => 0x0EA0_CC00,
            Self::FAbd => 0x2EA0_D400,
            Self::FAddp => 0x2E20_D400,
            Self::FMulx => 0x0E20_DC00,
            Self::FAcGe => 0x2E20_EC00,
            Self::FAcGt => 0x2EA0_EC00,
            Self::FMaxp => 0x2E20_F400,
            Self::FMinp => 0x2EA0_F400,
            Self::FRecps => 0x0E20_FC00,
            Self::FRsqrts => 0x0EA0_FC00,
            Self::FAmax => 0x0EA0_DC00,
            Self::FAmin => 0x2EA0_DC00,
        }
    }

    /// Whether this is a FEAT_FAMINMAX op (`FAMAX`/`FAMIN`), which gates on FEAT_FAMINMAX rather than plain
    /// Advanced SIMD.
    pub fn is_faminmax(self) -> bool {
        matches!(self, Self::FAmax | Self::FAmin)
    }

    /// The half-precision (FP16, FEAT_FP16) base word with `Q = 0` (`U`, the `E` bit23, the fixed bit22, and the
    /// FP16 opcode baked in); the arrangement supplies `Q<<30`, and the registers `Vm<<16 | Vn<<5 | Vd`. This is a
    /// SEPARATE encoding from [`Self::base`] -- the `.4h`/`.8h` ops sit in their own opcode space (bit22=1,
    /// bit21=0). GNU+LLVM dual-oracle verified.
    pub fn fp16_base(self) -> u32 {
        match self {
            Self::FAdd => 0x0E40_1400,
            Self::FSub => 0x0EC0_1400,
            Self::FMul => 0x2E40_1C00,
            Self::FDiv => 0x2E40_3C00,
            Self::FMax => 0x0E40_3400,
            Self::FMin => 0x0EC0_3400,
            Self::FCmEq => 0x0E40_2400,
            Self::FCmGe => 0x2E40_2400,
            Self::FCmGt => 0x2EC0_2400,
            Self::FMaxNm => 0x0E40_0400,
            Self::FMinNm => 0x0EC0_0400,
            Self::FMaxNmp => 0x2E40_0400,
            Self::FMinNmp => 0x2EC0_0400,
            Self::FMla => 0x0E40_0C00,
            Self::FMls => 0x0EC0_0C00,
            Self::FAbd => 0x2EC0_1400,
            Self::FAddp => 0x2E40_1400,
            Self::FMulx => 0x0E40_1C00,
            Self::FAcGe => 0x2E40_2C00,
            Self::FAcGt => 0x2EC0_2C00,
            Self::FMaxp => 0x2E40_3400,
            Self::FMinp => 0x2EC0_3400,
            Self::FRecps => 0x0E40_3C00,
            Self::FRsqrts => 0x0EC0_3C00,
            Self::FAmax => 0x0EC0_1C00,
            Self::FAmin => 0x2EC0_1C00,
        }
    }

    /// Recover the op from a half-precision masked base (`word & 0x3FE0_FC00`); `None` if not one of these.
    pub fn from_fp16_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.fp16_base() == base)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::FAdd => "fadd",
            Self::FSub => "fsub",
            Self::FMul => "fmul",
            Self::FDiv => "fdiv",
            Self::FMax => "fmax",
            Self::FMin => "fmin",
            Self::FCmEq => "fcmeq",
            Self::FCmGe => "fcmge",
            Self::FCmGt => "fcmgt",
            Self::FMaxNm => "fmaxnm",
            Self::FMinNm => "fminnm",
            Self::FMaxNmp => "fmaxnmp",
            Self::FMinNmp => "fminnmp",
            Self::FMla => "fmla",
            Self::FMls => "fmls",
            Self::FAbd => "fabd",
            Self::FAddp => "faddp",
            Self::FMulx => "fmulx",
            Self::FAcGe => "facge",
            Self::FAcGt => "facgt",
            Self::FMaxp => "fmaxp",
            Self::FMinp => "fminp",
            Self::FRecps => "frecps",
            Self::FRsqrts => "frsqrts",
            Self::FAmax => "famax",
            Self::FAmin => "famin",
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 26] = [
        Self::FAdd,
        Self::FSub,
        Self::FMul,
        Self::FDiv,
        Self::FMax,
        Self::FMin,
        Self::FCmEq,
        Self::FCmGe,
        Self::FCmGt,
        Self::FMaxNm,
        Self::FMinNm,
        Self::FMaxNmp,
        Self::FMinNmp,
        Self::FMla,
        Self::FMls,
        Self::FAbd,
        Self::FAddp,
        Self::FMulx,
        Self::FAcGe,
        Self::FAcGt,
        Self::FMaxp,
        Self::FMinp,
        Self::FRecps,
        Self::FRsqrts,
        Self::FAmax,
        Self::FAmin,
    ];
}
