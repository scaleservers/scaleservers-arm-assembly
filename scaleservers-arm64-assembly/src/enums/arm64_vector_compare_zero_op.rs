// Copyright (c) Scaleservers LLC

use super::Arm64VectorArrangement;

/// The operation of an AArch64 Advanced SIMD (NEON) **compare against zero** -- the two-register-misc encoding
/// `0 Q U 01110 size 10000 opcode 10 Rn Rd` with an implicit zero operand: the integer forms compare each lane
/// against `#0`, the floating-point forms against `#0.0`. The op is an orthogonal field over the shared
/// `{ arrangement, Vd, Vn }` shape; the integer forms take every arrangement except `.1d`, the FP forms take
/// `.2s`/`.4s`/`.2d` (FP16 `.4h`/`.8h` compare-against-zero is modeled separately in `Arm64VectorFp16TwoMiscOp`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorCompareZeroOp {
    /// `CMGT Vd, Vn, #0` -- per-lane signed compare greater-than zero (`U = 0`, opcode `01000`).
    Cmgt,
    /// `CMGE Vd, Vn, #0` -- per-lane signed compare greater-or-equal zero (`U = 1`, opcode `01000`).
    Cmge,
    /// `CMEQ Vd, Vn, #0` -- per-lane compare equal zero (`U = 0`, opcode `01001`).
    Cmeq,
    /// `CMLE Vd, Vn, #0` -- per-lane signed compare less-or-equal zero (`U = 1`, opcode `01001`).
    Cmle,
    /// `CMLT Vd, Vn, #0` -- per-lane signed compare less-than zero (`U = 0`, opcode `01010`).
    Cmlt,
    /// `FCMGT Vd, Vn, #0.0` -- per-lane FP compare greater-than zero (`U = 0`, size hi 1, opcode `01100`).
    Fcmgt,
    /// `FCMGE Vd, Vn, #0.0` -- per-lane FP compare greater-or-equal zero (`U = 1`, size hi 1, opcode `01100`).
    Fcmge,
    /// `FCMEQ Vd, Vn, #0.0` -- per-lane FP compare equal zero (`U = 0`, size hi 1, opcode `01101`).
    Fcmeq,
    /// `FCMLE Vd, Vn, #0.0` -- per-lane FP compare less-or-equal zero (`U = 1`, size hi 1, opcode `01101`).
    Fcmle,
    /// `FCMLT Vd, Vn, #0.0` -- per-lane FP compare less-than zero (`U = 0`, size hi 1, opcode `01110`).
    Fcmlt,
}

impl Arm64VectorCompareZeroOp {
    /// The base word with `Q = 0` and the variable `size`/`sz` cleared (`U`, the opcode, and -- for the FP ops --
    /// the `size` high bit baked in); the arrangement adds `Q<<30` and the size contribution `<<22`. GNU+LLVM
    /// dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Cmgt => 0x0E20_8800,
            Self::Cmge => 0x2E20_8800,
            Self::Cmeq => 0x0E20_9800,
            Self::Cmle => 0x2E20_9800,
            Self::Cmlt => 0x0E20_A800,
            Self::Fcmgt => 0x0EA0_C800,
            Self::Fcmge => 0x2EA0_C800,
            Self::Fcmeq => 0x0EA0_D800,
            Self::Fcmle => 0x2EA0_D800,
            Self::Fcmlt => 0x0EA0_E800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Cmgt => "cmgt",
            Self::Cmge => "cmge",
            Self::Cmeq => "cmeq",
            Self::Cmle => "cmle",
            Self::Cmlt => "cmlt",
            Self::Fcmgt => "fcmgt",
            Self::Fcmge => "fcmge",
            Self::Fcmeq => "fcmeq",
            Self::Fcmle => "fcmle",
            Self::Fcmlt => "fcmlt",
        }
    }

    /// Whether this is a floating-point compare (against `#0.0`, with the `size` high bit fixed by the op and the
    /// arrangement supplying the 1-bit `sz`).
    pub fn is_fp(self) -> bool {
        matches!(
            self,
            Self::Fcmgt | Self::Fcmge | Self::Fcmeq | Self::Fcmle | Self::Fcmlt
        )
    }

    /// Whether `arr` is a valid arrangement: the integer forms accept all but the single-lane `.1d`; the FP forms
    /// accept `.2s`/`.4s`/`.2d`.
    pub fn allows_arrangement(self, arr: Arm64VectorArrangement) -> bool {
        use Arm64VectorArrangement::{D2, S2, S4};
        if self.is_fp() {
            matches!(arr, S2 | S4 | D2)
        } else {
            arr != Arm64VectorArrangement::D1
        }
    }

    /// The `size`-field contribution the arrangement supplies (the full 2-bit lane size for integer; the 1-bit FP
    /// `sz` for the FP ops, whose `size` high bit is in the base).
    pub fn size_field(self, arr: Arm64VectorArrangement) -> u32 {
        if self.is_fp() {
            arr.fp_sz_bit()
        } else {
            arr.size_bits()
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 10] = [
        Self::Cmgt,
        Self::Cmge,
        Self::Cmeq,
        Self::Cmle,
        Self::Cmlt,
        Self::Fcmgt,
        Self::Fcmge,
        Self::Fcmeq,
        Self::Fcmle,
        Self::Fcmlt,
    ];
}
