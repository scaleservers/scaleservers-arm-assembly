// Copyright (c) Scaleservers LLC

/// The SME2 multi-vector clamp operations `<op> { Zd-list }, Zn.<T>, Zm.<T>`, which clamp each destination element to
/// the inclusive range `[Zn, Zm]`: the signed/unsigned integer `SCLAMP`/`UCLAMP`, the floating-point `FCLAMP`, and the
/// BFloat16 `BFCLAMP` (the `size==00` slot of `FCLAMP`, FEAT_SME_B16B16).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64Sme2ClampKind {
    /// `SCLAMP` -- signed integer clamp (`.b`/`.h`/`.s`/`.d`).
    Sclamp,
    /// `UCLAMP` -- unsigned integer clamp (`.b`/`.h`/`.s`/`.d`).
    Uclamp,
    /// `FCLAMP` -- floating-point clamp (`.h`/`.s`/`.d`).
    Fclamp,
    /// `BFCLAMP` -- BFloat16 clamp (`.h` only; the `size==00` slot of `FCLAMP`, FEAT_SME_B16B16).
    Bfclamp,
}

impl Arm64Sme2ClampKind {
    /// The 32-bit encoding base (with the dest-list and register fields zero). GNU+LLVM verified.
    pub const fn base(self) -> u32 {
        match self {
            // SCLAMP/UCLAMP share [12:10]=001 (base 0xC120_C400) and differ in the [0] sign bit; FCLAMP is [12:10]=000.
            Self::Sclamp | Self::Uclamp => 0xC120_C400,
            // BFCLAMP reuses the FCLAMP base; the encoder forces size==00 (the BF16 slot) and the decoder maps it back.
            Self::Fclamp | Self::Bfclamp => 0xC120_C000,
        }
    }

    /// Whether this is the BFloat16 clamp (always `.h`, encoded in the `size==00` slot of `FCLAMP`).
    pub const fn is_bf16(self) -> bool {
        matches!(self, Self::Bfclamp)
    }

    /// The `[0]` bit (the signed/unsigned selector; `UCLAMP` sets it).
    pub const fn low_bit(self) -> u32 {
        matches!(self, Self::Uclamp) as u32
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sclamp => "sclamp",
            Self::Uclamp => "uclamp",
            Self::Fclamp => "fclamp",
            Self::Bfclamp => "bfclamp",
        }
    }

    /// Whether this is the floating-point clamp (which excludes the `.b` element).
    pub const fn is_fp(self) -> bool {
        matches!(self, Self::Fclamp)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Sclamp, Self::Uclamp, Self::Fclamp, Self::Bfclamp];
}
