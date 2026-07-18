// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 **AES** cryptographic instruction (DDI0487 C7, FEAT_AES) -- a fully-fixed
/// two-register `.16b` encoding (only `Vn`/`Vd` vary): `AESE`/`AESD` perform one AES encrypt/decrypt round
/// (SubBytes + ShiftRows + AddRoundKey), `AESMC`/`AESIMC` the (inverse) MixColumns step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorAesOp {
    /// `AESE Vd.16b, Vn.16b` -- AES single-round encryption.
    Aese,
    /// `AESD Vd.16b, Vn.16b` -- AES single-round decryption.
    Aesd,
    /// `AESMC Vd.16b, Vn.16b` -- AES mix columns.
    Aesmc,
    /// `AESIMC Vd.16b, Vn.16b` -- AES inverse mix columns.
    Aesimc,
}

impl Arm64VectorAesOp {
    /// The fixed base word; only `Vn<<5` and `Vd` are added. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Aese => 0x4E28_4800,
            Self::Aesd => 0x4E28_5800,
            Self::Aesmc => 0x4E28_6800,
            Self::Aesimc => 0x4E28_7800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Aese => "aese",
            Self::Aesd => "aesd",
            Self::Aesmc => "aesmc",
            Self::Aesimc => "aesimc",
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 4] = [Self::Aese, Self::Aesd, Self::Aesmc, Self::Aesimc];
}
