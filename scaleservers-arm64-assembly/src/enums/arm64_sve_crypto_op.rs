// Copyright (c) Scaleservers LLC

use crate::enums::Arm64VectorElement;

/// The SVE2 cryptographic operations with the destructive `Zdn, Zdn, Zm` shape (a single round applied to the
/// destination-and-first-source register): the AES single-round encrypt/decrypt `AESE`/`AESD` (`.b`,
/// FEAT_SVE_AES) and the SM4 encrypt-round `SM4E` (`.s`, FEAT_SVE_SM4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SveCryptoDestructiveOp {
    /// `AESE Zdn.B, Zdn.B, Zm.B` -- AES single-round encryption.
    Aese,
    /// `AESD Zdn.B, Zdn.B, Zm.B` -- AES single-round decryption.
    Aesd,
    /// `SM4E Zdn.S, Zdn.S, Zm.S` -- SM4 encryption round.
    Sm4e,
}

impl Arm64SveCryptoDestructiveOp {
    /// The 32-bit encoding base (opcode in place; `Zm`/`Zdn` zero). GNU+LLVM verified.
    pub const fn base(self) -> u32 {
        match self {
            Self::Aese => 0x4522_E000,
            Self::Aesd => 0x4522_E400,
            Self::Sm4e => 0x4523_E000,
        }
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Aese => "aese",
            Self::Aesd => "aesd",
            Self::Sm4e => "sm4e",
        }
    }

    /// The element type: `.b` for the AES rounds, `.s` for the SM4 round.
    pub const fn element(self) -> Arm64VectorElement {
        match self {
            Self::Aese | Self::Aesd => Arm64VectorElement::B,
            Self::Sm4e => Arm64VectorElement::S,
        }
    }

    /// Every op, for round-trip testing.
    pub const ALL: [Self; 3] = [Self::Aese, Self::Aesd, Self::Sm4e];
}

/// The SVE2 cryptographic operations with the constructive `Zd, Zn, Zm` shape: the SM4 key-expansion
/// `SM4EKEY` (`.s`, FEAT_SVE_SM4) and the SHA-3 rotate-and-XOR `RAX1` (`.d`, FEAT_SVE_SHA3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SveCryptoBinaryOp {
    /// `SM4EKEY Zd.S, Zn.S, Zm.S` -- SM4 key updates.
    Sm4ekey,
    /// `RAX1 Zd.D, Zn.D, Zm.D` -- rotate by 1 and exclusive-OR.
    Rax1,
}

impl Arm64SveCryptoBinaryOp {
    /// The 32-bit encoding base (opcode in place; `Zm`/`Zn`/`Zd` zero). GNU+LLVM verified.
    pub const fn base(self) -> u32 {
        match self {
            Self::Sm4ekey => 0x4520_F000,
            Self::Rax1 => 0x4520_F400,
        }
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sm4ekey => "sm4ekey",
            Self::Rax1 => "rax1",
        }
    }

    /// The element type: `.s` for SM4 key expansion, `.d` for RAX1.
    pub const fn element(self) -> Arm64VectorElement {
        match self {
            Self::Sm4ekey => Arm64VectorElement::S,
            Self::Rax1 => Arm64VectorElement::D,
        }
    }

    /// Every op, for round-trip testing.
    pub const ALL: [Self; 2] = [Self::Sm4ekey, Self::Rax1];
}
