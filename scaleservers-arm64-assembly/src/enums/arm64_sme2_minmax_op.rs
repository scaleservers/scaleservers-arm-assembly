// Copyright (c) Scaleservers LLC

/// An SME2 multi-vector min/max operation (the by-single-vector, destructive form `<op> {Zdn-list}, {Zdn-list}, Zm`):
/// the signed/unsigned integer `SMIN`/`SMAX`/`UMIN`/`UMAX` and the floating-point `FMIN`/`FMAX` + the NaN-propagating
/// `FMINNM`/`FMAXNM`. The integer forms set `[8]=0` with the min/max selector at `[5]` and the sign at `[0]`; the FP
/// forms set `[8]=1` with the min/max selector at `[0]` and the NM (`NM` = IEEE-754 minNum/maxNum) marker at `[5]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64Sme2MinMaxOp {
    /// `SMIN` -- signed minimum.
    Smin,
    /// `SMAX` -- signed maximum.
    Smax,
    /// `UMIN` -- unsigned minimum.
    Umin,
    /// `UMAX` -- unsigned maximum.
    Umax,
    /// `FMIN` -- floating-point minimum (`[5]=0`).
    Fmin,
    /// `FMAX` -- floating-point maximum (`[5]=0`).
    Fmax,
    /// `FMINNM` -- floating-point NaN-propagating minimum (`[5]=1`).
    Fminnm,
    /// `FMAXNM` -- floating-point NaN-propagating maximum (`[5]=1`).
    Fmaxnm,
    /// `BFMIN` -- BFloat16 minimum (`.h`; the `size==00` slot of the FP form, FEAT_SME_B16B16).
    Bfmin,
    /// `BFMAX` -- BFloat16 maximum (`.h`; the `size==00` slot of the FP form, FEAT_SME_B16B16).
    Bfmax,
    /// `BFMINNM` -- BFloat16 NaN-propagating minimum (`.h`; `size==00` slot of `FMINNM`, FEAT_SME_B16B16).
    Bfminnm,
    /// `BFMAXNM` -- BFloat16 NaN-propagating maximum (`.h`; `size==00` slot of `FMAXNM`, FEAT_SME_B16B16).
    Bfmaxnm,
}

impl Arm64Sme2MinMaxOp {
    /// Whether this is a floating-point op (excludes `.b`). The BF16 forms are NOT included here (they force the
    /// `size==00` slot rather than carrying a real element size), but they DO set the `[8]` marker -- see [`Self::base`].
    pub const fn is_fp(self) -> bool {
        matches!(self, Self::Fmin | Self::Fmax | Self::Fminnm | Self::Fmaxnm)
    }

    /// Whether this is a BFloat16 op (always `.h`, encoded in the `size==00` slot of the FP form).
    pub const fn is_bf16(self) -> bool {
        matches!(
            self,
            Self::Bfmin | Self::Bfmax | Self::Bfminnm | Self::Bfmaxnm
        )
    }

    /// The 32-bit encoding base: `0xC120_A000` for the integer forms, `0xC120_A100` for the FP and BF16 forms (both set
    /// the `[8]` FP marker; BF16 then forces `size==00`).
    pub const fn base(self) -> u32 {
        if self.is_fp() || self.is_bf16() {
            0xC120_A100
        } else {
            0xC120_A000
        }
    }

    /// If this is an FP op recovered with `size==00`, it is actually the BF16 form (FEAT_SME_B16B16); other ops are
    /// returned unchanged (integer `size==00` is the valid `.b` element).
    pub const fn bf16_in_size00_slot(self) -> Self {
        match self {
            Self::Fmin => Self::Bfmin,
            Self::Fmax => Self::Bfmax,
            Self::Fminnm => Self::Bfminnm,
            Self::Fmaxnm => Self::Bfmaxnm,
            other => other,
        }
    }

    /// The `[5]` opcode bit: the integer min selector (`SMIN`/`UMIN`) and the FP/BF16 NM marker (`F*NM`/`BF*NM`).
    pub const fn bit5(self) -> u32 {
        matches!(
            self,
            Self::Smin | Self::Umin | Self::Fminnm | Self::Fmaxnm | Self::Bfminnm | Self::Bfmaxnm
        ) as u32
    }

    /// The `[0]` opcode bit (the integer sign / the FP and BF16 min selector).
    pub const fn bit0(self) -> u32 {
        matches!(
            self,
            Self::Umin | Self::Umax | Self::Fmin | Self::Fminnm | Self::Bfmin | Self::Bfminnm
        ) as u32
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Smin => "smin",
            Self::Smax => "smax",
            Self::Umin => "umin",
            Self::Umax => "umax",
            Self::Fmin => "fmin",
            Self::Fmax => "fmax",
            Self::Fminnm => "fminnm",
            Self::Fmaxnm => "fmaxnm",
            Self::Bfmin => "bfmin",
            Self::Bfmax => "bfmax",
            Self::Bfminnm => "bfminnm",
            Self::Bfmaxnm => "bfmaxnm",
        }
    }

    /// Recover the op from the `[8]` FP marker, the `[5]` NM / min-selector bit, and the `[0]` bit.
    pub const fn from_bits(fp: u32, bit5: u32, bit0: u32) -> Self {
        match (fp & 1, bit5 & 1, bit0 & 1) {
            (1, 0, 1) => Self::Fmin,
            (1, 0, 0) => Self::Fmax,
            (1, 1, 1) => Self::Fminnm,
            (1, 1, 0) => Self::Fmaxnm,
            (0, 1, 0) => Self::Smin,
            (0, 0, 0) => Self::Smax,
            (0, 1, 1) => Self::Umin,
            (0, 0, 1) => Self::Umax,
            _ => Self::Smax,
        }
    }

    /// Every non-BF16 op, for tests.
    pub const ALL: [Self; 8] = [
        Self::Smin,
        Self::Smax,
        Self::Umin,
        Self::Umax,
        Self::Fmin,
        Self::Fmax,
        Self::Fminnm,
        Self::Fmaxnm,
    ];
}
