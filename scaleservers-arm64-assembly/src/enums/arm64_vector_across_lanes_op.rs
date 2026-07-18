// Copyright (c) Scaleservers LLC

use super::Arm64VectorArrangement;

/// The operation of an AArch64 Advanced SIMD (NEON) **across lanes** reduction (DDI0487 C7) -- the encoding
/// class `0 Q U 01110 size 11000 opcode 10 Rn Rd`, which reduces every lane of a vector `Vn.<arr>` to a single
/// scalar `Vd` (named with the result's size letter `b`/`h`/`s`/`d`). The op is an orthogonal field over a shared
/// `{ arrangement, Vd, Vn }` shape; the result width is the lane width (`ADDV`/min/max), twice it (`SADDLV`/
/// `UADDLV` long), or the FP single (`FMAXV`/`FMINV`/`FMAXNMV`/`FMINNMV`). (FP16 `.8h` across-lanes reduce is
/// modeled separately in `Arm64VectorFp16AcrossOp`.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorAcrossLanesOp {
    /// `ADDV` -- add across all lanes (`U = 0`, opcode `11011`).
    Addv,
    /// `SMAXV` -- signed maximum across all lanes (`U = 0`, opcode `01010`).
    Smaxv,
    /// `SMINV` -- signed minimum across all lanes (`U = 0`, opcode `11010`).
    Sminv,
    /// `UMAXV` -- unsigned maximum across all lanes (`U = 1`, opcode `01010`).
    Umaxv,
    /// `UMINV` -- unsigned minimum across all lanes (`U = 1`, opcode `11010`).
    Uminv,
    /// `SADDLV` -- signed add long across all lanes (result twice the lane width) (`U = 0`, opcode `00011`).
    Saddlv,
    /// `UADDLV` -- unsigned add long across all lanes (`U = 1`, opcode `00011`).
    Uaddlv,
    /// `FMAXV` -- FP maximum (NaN-propagating) across all lanes (`U = 1`, size hi 0, opcode `01111`).
    Fmaxv,
    /// `FMINV` -- FP minimum (NaN-propagating) across all lanes (`U = 1`, size hi 1, opcode `01111`).
    Fminv,
    /// `FMAXNMV` -- FP maximum-number across all lanes (`U = 1`, size hi 0, opcode `01100`).
    Fmaxnmv,
    /// `FMINNMV` -- FP minimum-number across all lanes (`U = 1`, size hi 1, opcode `01100`).
    Fminnmv,
}

impl Arm64VectorAcrossLanesOp {
    /// The base word with `Q = 0` and the variable `size`/`sz` cleared (`U`, the opcode, and -- for the FP ops --
    /// the `size` high bit baked in); the encoder adds `Q<<30`, the size contribution `<<22`, and `Vn<<5 | Vd`.
    /// GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Addv => 0x0E31_B800,
            Self::Smaxv => 0x0E30_A800,
            Self::Sminv => 0x0E31_A800,
            Self::Umaxv => 0x2E30_A800,
            Self::Uminv => 0x2E31_A800,
            Self::Saddlv => 0x0E30_3800,
            Self::Uaddlv => 0x2E30_3800,
            Self::Fmaxv => 0x2E30_F800,
            Self::Fminv => 0x2EB0_F800,
            Self::Fmaxnmv => 0x2E30_C800,
            Self::Fminnmv => 0x2EB0_C800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Addv => "addv",
            Self::Smaxv => "smaxv",
            Self::Sminv => "sminv",
            Self::Umaxv => "umaxv",
            Self::Uminv => "uminv",
            Self::Saddlv => "saddlv",
            Self::Uaddlv => "uaddlv",
            Self::Fmaxv => "fmaxv",
            Self::Fminv => "fminv",
            Self::Fmaxnmv => "fmaxnmv",
            Self::Fminnmv => "fminnmv",
        }
    }

    /// Whether this is one of the floating-point reductions (which encode the element size as the 1-bit `sz` with
    /// the `size` high bit fixed by the op, and are valid only for `.4s` here).
    pub fn is_fp(self) -> bool {
        matches!(
            self,
            Self::Fmaxv | Self::Fminv | Self::Fmaxnmv | Self::Fminnmv
        )
    }

    /// Whether this is a *long* reduction (`SADDLV`/`UADDLV`), whose scalar result is twice the lane width.
    pub fn is_long(self) -> bool {
        matches!(self, Self::Saddlv | Self::Uaddlv)
    }

    /// Whether `arr` is a valid source arrangement. The integer reductions take `.8b`/`.16b`/`.4h`/`.8h`/`.4s`
    /// (no `.2s`/`.1d`/`.2d`); the FP reductions take `.4s` only here (FP16 `.8h` lives in `Arm64VectorFp16AcrossOp`).
    pub fn allows_arrangement(self, arr: Arm64VectorArrangement) -> bool {
        use Arm64VectorArrangement::{B8, B16, H4, H8, S4};
        if self.is_fp() {
            matches!(arr, S4)
        } else {
            matches!(arr, B8 | B16 | H4 | H8 | S4)
        }
    }

    /// The 2-bit `size` view of the scalar result (`0`=`b`, `1`=`h`, `2`=`s`, `3`=`d`): the lane width for the
    /// min/max/ADDV ops, twice it for the long reductions, and the FP single (`s`) for the FP reductions.
    pub fn result_size(self, arr: Arm64VectorArrangement) -> u32 {
        if self.is_long() {
            arr.size_bits() + 1
        } else if self.is_fp() {
            2 - arr.fp_sz_bit() // .4s (sz 0) -> s (2); .8h (sz 1) -> h (1)
        } else {
            arr.size_bits()
        }
    }

    /// The `size`-field contribution the arrangement supplies to the encoded word (shifted into `[23:22]`): the
    /// full 2-bit lane size for the integer ops, the 1-bit FP `sz` for the FP ops (their `size` high bit is in
    /// the base).
    pub fn size_field(self, arr: Arm64VectorArrangement) -> u32 {
        if self.is_fp() {
            arr.fp_sz_bit()
        } else {
            arr.size_bits()
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 11] = [
        Self::Addv,
        Self::Smaxv,
        Self::Sminv,
        Self::Umaxv,
        Self::Uminv,
        Self::Saddlv,
        Self::Uaddlv,
        Self::Fmaxv,
        Self::Fminv,
        Self::Fmaxnmv,
        Self::Fminnmv,
    ];
}
