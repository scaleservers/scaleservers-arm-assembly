// Copyright (c) Scaleservers LLC

use crate::enums::Arm64VectorElement;

/// An SVE2 **predicated floating-point precision convert (top/bottom variants)** op (DDI0487 C8): `<op> Zd.<Td>,
/// Pg/M, Zn.<Tn>`. These widen the odd lanes (`FCVTLT`), narrow into the odd lanes (`FCVTNT`/`FCVTXNT`/`BFCVTNT`),
/// round-to-odd narrow (`FCVTX`), or convert to BFloat16 (`BFCVT`). The encoding's 9-bit `[24:16]` discriminant is
/// irregular, so the model carries the op kind plus the destination/source element sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2FpUpdownOp {
    /// `FCVTLT` -- floating-point widening convert, top (`.h`->`.s` or `.s`->`.d`).
    FcvtltHs,
    FcvtltSd,
    /// `FCVTNT` -- floating-point narrowing convert, top (`.s`->`.h` or `.d`->`.s`).
    FcvtntSh,
    FcvtntDs,
    /// `FCVTX` -- floating-point down convert, rounding to odd (`.d`->`.s`).
    Fcvtx,
    /// `FCVTXNT` -- floating-point down convert, rounding to odd, top (`.d`->`.s`).
    Fcvtxnt,
    /// `BFCVT` -- convert to BFloat16 (`.s`->`.h`).
    Bfcvt,
    /// `BFCVTNT` -- convert to BFloat16, top (`.s`->`.h`).
    Bfcvtnt,
}

impl Arm64Sve2FpUpdownOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::FcvtltHs | Self::FcvtltSd => "fcvtlt",
            Self::FcvtntSh | Self::FcvtntDs => "fcvtnt",
            Self::Fcvtx => "fcvtx",
            Self::Fcvtxnt => "fcvtxnt",
            Self::Bfcvt => "bfcvt",
            Self::Bfcvtnt => "bfcvtnt",
        }
    }

    /// The 9-bit `[24:16]` discriminant.
    pub fn discriminant(self) -> u32 {
        match self {
            Self::FcvtltHs => 0x089,
            Self::FcvtltSd => 0x0CB,
            Self::FcvtntSh => 0x088,
            Self::FcvtntDs => 0x0CA,
            Self::Fcvtx => 0x10A,
            Self::Fcvtxnt => 0x00A,
            Self::Bfcvt => 0x18A,
            Self::Bfcvtnt => 0x08A,
        }
    }

    /// The destination element size.
    pub fn dest(self) -> Arm64VectorElement {
        use Arm64VectorElement::{D, H, S};
        match self {
            Self::FcvtltHs => S,
            Self::FcvtltSd => D,
            Self::FcvtntSh | Self::Bfcvt | Self::Bfcvtnt => H,
            Self::FcvtntDs | Self::Fcvtx | Self::Fcvtxnt => S,
        }
    }

    /// The source element size.
    pub fn source(self) -> Arm64VectorElement {
        use Arm64VectorElement::{D, H, S};
        match self {
            Self::FcvtltHs => H,
            Self::FcvtltSd | Self::FcvtntSh | Self::Bfcvt | Self::Bfcvtnt => S,
            Self::FcvtntDs | Self::Fcvtx | Self::Fcvtxnt => D,
        }
    }

    /// Whether the op requires FEAT_BF16 (`BFCVT`/`BFCVTNT`) rather than baseline FEAT_SVE2.
    pub fn requires_bf16(self) -> bool {
        matches!(self, Self::Bfcvt | Self::Bfcvtnt)
    }

    /// Recover the op from its `[24:16]` discriminant.
    pub fn from_discriminant(disc: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.discriminant() == disc & 0x1FF)
    }

    /// The FEAT_SVE2p2 **zeroing** (`/z`) form's fully-fixed `[31:13]` base (operands `Pg`/`Zn`/`Zd` are zero).
    /// Relocated to the `0x64..` frame; irregular, so each is stored explicitly (confirmed vs binutils-trunk).
    pub fn zeroing_base(self) -> u32 {
        match self {
            Self::FcvtltHs => 0x6481_A000,
            Self::FcvtltSd => 0x64C3_A000,
            Self::FcvtntSh => 0x6480_A000,
            Self::FcvtntDs => 0x64C2_A000,
            Self::Fcvtx => 0x641A_C000,
            Self::Fcvtxnt => 0x6402_A000,
            Self::Bfcvt => 0x649A_C000,
            Self::Bfcvtnt => 0x6482_A000,
        }
    }

    /// Recover the op from its zeroing `[31:13]` base (operand bits already masked off), or `None`.
    pub fn from_zeroing_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.zeroing_base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 8] = [
        Self::FcvtltHs,
        Self::FcvtltSd,
        Self::FcvtntSh,
        Self::FcvtntDs,
        Self::Fcvtx,
        Self::Fcvtxnt,
        Self::Bfcvt,
        Self::Bfcvtnt,
    ];
}
