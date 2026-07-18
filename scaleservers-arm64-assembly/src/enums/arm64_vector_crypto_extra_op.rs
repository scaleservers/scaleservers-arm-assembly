// Copyright (c) Scaleservers LLC

use crate::enums::Arm64ShaRegView;

/// Which optional ARMv8.2 cryptography extension a SHA512/SHA3/SM3/SM4 op belongs to -- each is an independent
/// `FEAT_*` flag, so the instruction requirement is derived per-op via this family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64CryptoFamily {
    /// FEAT_SHA512.
    Sha512,
    /// FEAT_SHA3.
    Sha3,
    /// FEAT_SM3.
    Sm3,
    /// FEAT_SM4.
    Sm4,
}

/// A three-register SHA512/SM3/SM4 op (DDI0487 C7) -- the `11001110 011 Rm 1 opcode Rn Rd` family (`Rd`/`Rn`/`Rm`
/// at the usual positions). The display views differ per op (the SHA512 hash ops mix `Qd, Qn`; the schedule/SM
/// ops are `.2d` or `.4s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorCrypto3Op {
    /// `SHA512H Qd, Qn, Vm.2d`.
    Sha512h,
    /// `SHA512H2 Qd, Qn, Vm.2d`.
    Sha512h2,
    /// `SHA512SU1 Vd.2d, Vn.2d, Vm.2d`.
    Sha512su1,
    /// `RAX1 Vd.2d, Vn.2d, Vm.2d` (FEAT_SHA3).
    Rax1,
    /// `SM3PARTW1 Vd.4s, Vn.4s, Vm.4s`.
    Sm3partw1,
    /// `SM3PARTW2 Vd.4s, Vn.4s, Vm.4s`.
    Sm3partw2,
    /// `SM4EKEY Vd.4s, Vn.4s, Vm.4s`.
    Sm4ekey,
}

impl Arm64VectorCrypto3Op {
    /// The base word (`Rm`/`Rn`/`Rd` zero). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Sha512h => 0xCE60_8000,
            Self::Sha512h2 => 0xCE60_8400,
            Self::Sha512su1 => 0xCE60_8800,
            Self::Rax1 => 0xCE60_8C00,
            Self::Sm3partw1 => 0xCE60_C000,
            Self::Sm3partw2 => 0xCE60_C400,
            Self::Sm4ekey => 0xCE60_C800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sha512h => "sha512h",
            Self::Sha512h2 => "sha512h2",
            Self::Sha512su1 => "sha512su1",
            Self::Rax1 => "rax1",
            Self::Sm3partw1 => "sm3partw1",
            Self::Sm3partw2 => "sm3partw2",
            Self::Sm4ekey => "sm4ekey",
        }
    }

    /// The display view shared by `Vd` and `Vn`.
    pub fn dn_view(self) -> Arm64ShaRegView {
        match self {
            Self::Sha512h | Self::Sha512h2 => Arm64ShaRegView::Q,
            Self::Sha512su1 | Self::Rax1 => Arm64ShaRegView::V2d,
            Self::Sm3partw1 | Self::Sm3partw2 | Self::Sm4ekey => Arm64ShaRegView::V4s,
        }
    }

    /// The display view of `Vm`.
    pub fn m_view(self) -> Arm64ShaRegView {
        match self {
            Self::Sha512h | Self::Sha512h2 | Self::Sha512su1 | Self::Rax1 => Arm64ShaRegView::V2d,
            Self::Sm3partw1 | Self::Sm3partw2 | Self::Sm4ekey => Arm64ShaRegView::V4s,
        }
    }

    /// The extension this op needs.
    pub fn family(self) -> Arm64CryptoFamily {
        match self {
            Self::Sha512h | Self::Sha512h2 | Self::Sha512su1 => Arm64CryptoFamily::Sha512,
            Self::Rax1 => Arm64CryptoFamily::Sha3,
            Self::Sm3partw1 | Self::Sm3partw2 => Arm64CryptoFamily::Sm3,
            Self::Sm4ekey => Arm64CryptoFamily::Sm4,
        }
    }

    /// Recover an op from a masked base (`word & 0xFFFF_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op.
    pub const ALL: [Self; 7] = [
        Self::Sha512h,
        Self::Sha512h2,
        Self::Sha512su1,
        Self::Rax1,
        Self::Sm3partw1,
        Self::Sm3partw2,
        Self::Sm4ekey,
    ];
}

/// A two-register SHA512/SM4 op (DDI0487 C7) -- the `11001110 110 00000 100000 opcode Rn Rd` family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorCrypto2Op {
    /// `SHA512SU0 Vd.2d, Vn.2d`.
    Sha512su0,
    /// `SM4E Vd.4s, Vn.4s`.
    Sm4e,
}

impl Arm64VectorCrypto2Op {
    /// The base word (`Rn`/`Rd` zero). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Sha512su0 => 0xCEC0_8000,
            Self::Sm4e => 0xCEC0_8400,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sha512su0 => "sha512su0",
            Self::Sm4e => "sm4e",
        }
    }

    /// The display view shared by `Vd` and `Vn`.
    pub fn view(self) -> Arm64ShaRegView {
        match self {
            Self::Sha512su0 => Arm64ShaRegView::V2d,
            Self::Sm4e => Arm64ShaRegView::V4s,
        }
    }

    /// The extension this op needs.
    pub fn family(self) -> Arm64CryptoFamily {
        match self {
            Self::Sha512su0 => Arm64CryptoFamily::Sha512,
            Self::Sm4e => Arm64CryptoFamily::Sm4,
        }
    }

    /// Recover an op from a masked base (`word & 0xFFFF_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op.
    pub const ALL: [Self; 2] = [Self::Sha512su0, Self::Sm4e];
}

/// A four-register SHA3/SM3 op (DDI0487 C7) -- the `11001110 0 sel 0 Rm 0 Ra Rn Rd` family with three source
/// registers `Vn`/`Vm`/`Va`. `EOR3`/`BCAX` are `.16b`; `SM3SS1` is `.4s`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorCrypto4Op {
    /// `EOR3 Vd.16b, Vn.16b, Vm.16b, Va.16b` -- three-way XOR (FEAT_SHA3).
    Eor3,
    /// `BCAX Vd.16b, Vn.16b, Vm.16b, Va.16b` -- bit-clear and XOR (FEAT_SHA3).
    Bcax,
    /// `SM3SS1 Vd.4s, Vn.4s, Vm.4s, Va.4s` (FEAT_SM3).
    Sm3ss1,
}

impl Arm64VectorCrypto4Op {
    /// The base word (all register fields zero). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Eor3 => 0xCE00_0000,
            Self::Bcax => 0xCE20_0000,
            Self::Sm3ss1 => 0xCE40_0000,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Eor3 => "eor3",
            Self::Bcax => "bcax",
            Self::Sm3ss1 => "sm3ss1",
        }
    }

    /// The per-lane arrangement suffix (`16b` for the SHA3 ops, `4s` for SM3SS1).
    pub fn arrangement(self) -> &'static str {
        match self {
            Self::Eor3 | Self::Bcax => "16b",
            Self::Sm3ss1 => "4s",
        }
    }

    /// The extension this op needs.
    pub fn family(self) -> Arm64CryptoFamily {
        match self {
            Self::Eor3 | Self::Bcax => Arm64CryptoFamily::Sha3,
            Self::Sm3ss1 => Arm64CryptoFamily::Sm3,
        }
    }

    /// Recover an op from a masked base (`word & 0xFFE0_8000`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op.
    pub const ALL: [Self; 3] = [Self::Eor3, Self::Bcax, Self::Sm3ss1];
}

/// An indexed SM3 "TT" op (DDI0487 C7, FEAT_SM3) -- the `11001110 010 Rm 10 opcode imm2 Rn Rd` family. The
/// operands are `Vd.4s, Vn.4s, Vm.s[index]` (a `.4s` lane of `Vm` at `imm2[13:12]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorSm3TtOp {
    /// `SM3TT1A`.
    Sm3tt1a,
    /// `SM3TT1B`.
    Sm3tt1b,
    /// `SM3TT2A`.
    Sm3tt2a,
    /// `SM3TT2B`.
    Sm3tt2b,
}

impl Arm64VectorSm3TtOp {
    /// The base word (`Rm`/`index`/`Rn`/`Rd` zero). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Sm3tt1a => 0xCE40_8000,
            Self::Sm3tt1b => 0xCE40_8400,
            Self::Sm3tt2a => 0xCE40_8800,
            Self::Sm3tt2b => 0xCE40_8C00,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sm3tt1a => "sm3tt1a",
            Self::Sm3tt1b => "sm3tt1b",
            Self::Sm3tt2a => "sm3tt2a",
            Self::Sm3tt2b => "sm3tt2b",
        }
    }

    /// Recover an op from a masked base (`word & 0xFFE0_CC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op.
    pub const ALL: [Self; 4] = [Self::Sm3tt1a, Self::Sm3tt1b, Self::Sm3tt2a, Self::Sm3tt2b];
}
