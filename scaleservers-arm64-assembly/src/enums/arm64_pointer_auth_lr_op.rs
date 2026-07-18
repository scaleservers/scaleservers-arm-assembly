// Copyright (c) Scaleservers LLC

/// The operand-free **pointer-authentication link-register** operations (FEAT_PAuth_LR, ARMv9.5). Each computes a PAC
/// for the return address combining the link register, SP, and PC -- the LR-aware variants of `PACIA`/`PACIB`. `PACM`
/// is the associated "mask" hint. All are fixed 32-bit words with no register operands. (The `*SPPC <label>` PC-relative
/// forms -- `AUTIASPPC`/`RETAASPPC` -- take a label operand and are modeled separately when a label operand lands.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64PointerAuthLrOp {
    /// `PACIASPPC` -- PAC the return address with key A, using SP and the current PC.
    Paciasppc,
    /// `PACIBSPPC` -- PAC the return address with key B, using SP and the current PC.
    Pacibsppc,
    /// `PACNBIASPPC` -- the non-branch-target key-A PAC variant.
    Pacnbiasppc,
    /// `PACNBIBSPPC` -- the non-branch-target key-B PAC variant.
    Pacnbibsppc,
    /// `PACM` -- the pointer-authentication "mask" hint (a `NOP` where FEAT_PAuth_LR is absent).
    Pacm,
}

impl Arm64PointerAuthLrOp {
    /// Every variant, for exhaustive sweeps.
    pub const ALL: [Self; 5] = [
        Self::Paciasppc,
        Self::Pacibsppc,
        Self::Pacnbiasppc,
        Self::Pacnbibsppc,
        Self::Pacm,
    ];

    /// The full fixed 32-bit encoding.
    pub fn word(self) -> u32 {
        match self {
            Self::Paciasppc => 0xDAC1_A3FE,
            Self::Pacibsppc => 0xDAC1_A7FE,
            Self::Pacnbiasppc => 0xDAC1_83FE,
            Self::Pacnbibsppc => 0xDAC1_87FE,
            Self::Pacm => 0xD503_24FF,
        }
    }

    /// Recover the operation from a 32-bit word, if it is one of these fixed encodings.
    pub fn from_word(word: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.word() == word)
    }

    /// The assembly mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Paciasppc => "paciasppc",
            Self::Pacibsppc => "pacibsppc",
            Self::Pacnbiasppc => "pacnbiasppc",
            Self::Pacnbibsppc => "pacnbibsppc",
            Self::Pacm => "pacm",
        }
    }
}
