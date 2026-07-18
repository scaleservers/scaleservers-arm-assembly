// Copyright (c) Scaleservers LLC

use super::Arm64VectorElement;

/// An SME2 **vertical dot product into ZA** op (FEAT_SME2; the `.d`-accumulator forms need FEAT_SME_I16I64; DDI0487
/// part C). VDOT multiplies a multi-vector `Zn` group by an indexed `Zm` element vertically and accumulates into a ZA
/// single-vector group. The op selector occupies `[5:3]`, and the `.d` (i16i64) forms additionally set `[23]` and
/// `[15]`. GNU+LLVM verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sme2ZaVdotOp {
    /// `FVDOT` -- FP16 `.h` pair (vgx2) -> `.s` ZA.
    Fvdot,
    /// `BFVDOT` -- BFloat16 `.h` pair (vgx2) -> `.s` ZA.
    Bfvdot,
    /// `SVDOT` -- signed `.h` pair (vgx2) -> `.s` ZA.
    SvdotHtoS,
    /// `UVDOT` -- unsigned `.h` pair (vgx2) -> `.s` ZA.
    UvdotHtoS,
    /// `USVDOT` -- unsigned-by-signed `.b` quad (vgx4) -> `.s` ZA.
    Usvdot,
    /// `SUVDOT` -- signed-by-unsigned `.b` quad (vgx4) -> `.s` ZA.
    Suvdot,
    /// `SVDOT` -- signed `.h` quad (vgx4) -> `.d` ZA (FEAT_SME_I16I64).
    SvdotHtoD,
    /// `UVDOT` -- unsigned `.h` quad (vgx4) -> `.d` ZA (FEAT_SME_I16I64).
    UvdotHtoD,
}

impl Arm64Sme2ZaVdotOp {
    /// The fixed op-bit contribution beyond the shared base: `[23]` (the `.d` accumulator) + `[15]` (= [`Self::four`], the
    /// vgx4 forms) OR-ed with the `[5:3]` selector.
    pub fn op_bits(self) -> u32 {
        let (hi, code) = match self {
            Self::Fvdot => (false, 0b001),
            Self::Bfvdot => (false, 0b011),
            Self::SvdotHtoS => (false, 0b100),
            Self::UvdotHtoS => (false, 0b110),
            Self::Usvdot => (false, 0b101),
            Self::Suvdot => (false, 0b111),
            Self::SvdotHtoD => (true, 0b001),
            Self::UvdotHtoD => (true, 0b011),
        };
        // the .d (i16i64) forms additionally pin [11]=1 and carry only a 1-bit index at [10].
        ((hi as u32) << 23) | ((hi as u32) << 11) | ((self.four() as u32) << 15) | (code << 3)
    }

    /// The maximum element index: `1` for the `.h`->`.d` forms (1-bit index at `[10]`), else `3` (2-bit index `[11:10]`).
    pub fn max_index(self) -> u8 {
        if self.needs_i16i64() { 1 } else { 3 }
    }

    /// Whether the source group is a four-register (vgx4) list (`.b` quad or `.h`->`.d` quad). `false` is the vgx2 pair.
    pub fn four(self) -> bool {
        matches!(
            self,
            Self::Usvdot | Self::Suvdot | Self::SvdotHtoD | Self::UvdotHtoD
        )
    }

    /// The source element type (`.b` for the byte-quad forms, else `.h`).
    pub fn src_elem(self) -> Arm64VectorElement {
        match self {
            Self::Usvdot | Self::Suvdot => Arm64VectorElement::B,
            _ => Arm64VectorElement::H,
        }
    }

    /// The ZA accumulator element type (`.d` for the i16i64 forms, else `.s`).
    pub fn za_elem(self) -> Arm64VectorElement {
        match self {
            Self::SvdotHtoD | Self::UvdotHtoD => Arm64VectorElement::D,
            _ => Arm64VectorElement::S,
        }
    }

    /// Whether this op needs FEAT_SME_I16I64 (the `.d`-accumulator forms).
    pub fn needs_i16i64(self) -> bool {
        matches!(self, Self::SvdotHtoD | Self::UvdotHtoD)
    }

    /// The lowercase UAL mnemonic.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Fvdot => "fvdot",
            Self::Bfvdot => "bfvdot",
            Self::SvdotHtoS | Self::SvdotHtoD => "svdot",
            Self::UvdotHtoS | Self::UvdotHtoD => "uvdot",
            Self::Usvdot => "usvdot",
            Self::Suvdot => "suvdot",
        }
    }

    /// Recover the op from the `[23]` (.d), `[15]` (vgx4) and `[5:3]` selector bits; `None` for unallocated combinations
    /// (this is what keeps VDOT disjoint from the FP8 indexed FDOT, which shares the `[22:20]=101` region).
    pub fn from_bits(hi: u32, four: u32, code: u32) -> Option<Self> {
        Some(match (hi & 1, four & 1, code & 0b111) {
            (0, 0, 0b001) => Self::Fvdot,
            (0, 0, 0b011) => Self::Bfvdot,
            (0, 0, 0b100) => Self::SvdotHtoS,
            (0, 0, 0b110) => Self::UvdotHtoS,
            (0, 1, 0b101) => Self::Usvdot,
            (0, 1, 0b111) => Self::Suvdot,
            (1, 1, 0b001) => Self::SvdotHtoD,
            (1, 1, 0b011) => Self::UvdotHtoD,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 8] = [
        Self::Fvdot,
        Self::Bfvdot,
        Self::SvdotHtoS,
        Self::UvdotHtoS,
        Self::Usvdot,
        Self::Suvdot,
        Self::SvdotHtoD,
        Self::UvdotHtoD,
    ];
}
