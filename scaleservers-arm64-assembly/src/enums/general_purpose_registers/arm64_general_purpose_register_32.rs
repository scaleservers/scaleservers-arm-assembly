// Copyright (c) Scaleservers LLC

/// A 32-bit (W) general-purpose register operand: the SAME 5-bit register field as the X view, but naming
/// the lower 32 bits of the register file.
///
/// ## Why a separate enum (the W-vs-X design decision)
/// On AArch64 the operand-size is NOT part of the register *field* -- the same 5-bit number selects the same
/// architectural register; an instruction's `sf` ("64-bit") bit decides whether it is accessed as the
/// 64-bit `X` or the 32-bit `W` view. We model the two views as **two distinct register enums**
/// ([`super::Arm64GeneralPurposeRegister`] for X, this for W) rather than as a width flag carried alongside
/// one enum, because it makes each `Arm64Instruction` variant's operand list self-describing: a variant that
/// takes `W` operands names `Arm64GeneralPurposeRegister32`, a variant that takes `X` operands names
/// `Arm64GeneralPurposeRegister`, and a 32-bit register can never be passed where a 64-bit one is required.
/// The `sf` bit in the machine encoding is derived from *which* operand type the variant holds, not stored
/// separately. (This type carries the 32-bit register
/// operands that the W-form instruction variants hold.)
///
/// The `31` duality mirrors the X view: [`Self::Wzr`] is the zero register, [`Self::Wsp`] the 32-bit stack
/// pointer; both encode to `0b11111`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64GeneralPurposeRegister32 {
    W0,
    W1,
    W2,
    W3,
    W4,
    W5,
    W6,
    W7,
    W8,
    W9,
    W10,
    W11,
    W12,
    W13,
    W14,
    W15,
    W16,
    W17,
    W18,
    W19,
    W20,
    W21,
    W22,
    W23,
    W24,
    W25,
    W26,
    W27,
    W28,
    W29,
    W30,
    /// The 32-bit zero register: field value `0b11111` where `31` reads as zero / discards writes.
    Wzr,
    /// The 32-bit stack pointer view: field value `0b11111` where `31` names the stack pointer.
    Wsp,
}

impl Arm64GeneralPurposeRegister32 {
    /// The 5-bit register field value. Both [`Self::Wzr`] and [`Self::Wsp`] encode to `31` (`0b11111`).
    pub fn as_operand_bits(&self) -> u8 {
        match self {
            Self::W0 => 0,
            Self::W1 => 1,
            Self::W2 => 2,
            Self::W3 => 3,
            Self::W4 => 4,
            Self::W5 => 5,
            Self::W6 => 6,
            Self::W7 => 7,
            Self::W8 => 8,
            Self::W9 => 9,
            Self::W10 => 10,
            Self::W11 => 11,
            Self::W12 => 12,
            Self::W13 => 13,
            Self::W14 => 14,
            Self::W15 => 15,
            Self::W16 => 16,
            Self::W17 => 17,
            Self::W18 => 18,
            Self::W19 => 19,
            Self::W20 => 20,
            Self::W21 => 21,
            Self::W22 => 22,
            Self::W23 => 23,
            Self::W24 => 24,
            Self::W25 => 25,
            Self::W26 => 26,
            Self::W27 => 27,
            Self::W28 => 28,
            Self::W29 => 29,
            Self::W30 => 30,
            Self::Wzr => 31,
            Self::Wsp => 31,
        }
    }

    /// Map a 5-bit register field to its W register. TOTAL (masks the low five bits), never panics. Field
    /// value `31` resolves to [`Self::Wzr`]; positions where `31` is the stack pointer use
    /// [`Self::from_operand_bits_sp`].
    pub fn from_operand_bits(bits: u8) -> Self {
        match bits & 0b1_1111 {
            0 => Self::W0,
            1 => Self::W1,
            2 => Self::W2,
            3 => Self::W3,
            4 => Self::W4,
            5 => Self::W5,
            6 => Self::W6,
            7 => Self::W7,
            8 => Self::W8,
            9 => Self::W9,
            10 => Self::W10,
            11 => Self::W11,
            12 => Self::W12,
            13 => Self::W13,
            14 => Self::W14,
            15 => Self::W15,
            16 => Self::W16,
            17 => Self::W17,
            18 => Self::W18,
            19 => Self::W19,
            20 => Self::W20,
            21 => Self::W21,
            22 => Self::W22,
            23 => Self::W23,
            24 => Self::W24,
            25 => Self::W25,
            26 => Self::W26,
            27 => Self::W27,
            28 => Self::W28,
            29 => Self::W29,
            30 => Self::W30,
            _ => Self::Wzr,
        }
    }

    /// Like [`Self::from_operand_bits`] but resolves `31` to the stack pointer ([`Self::Wsp`]).
    pub fn from_operand_bits_sp(bits: u8) -> Self {
        match bits & 0b1_1111 {
            31 => Self::Wsp,
            other => Self::from_operand_bits(other),
        }
    }

    /// The lowercase UAL register name (`w0`..`w30`, `wzr`, `wsp`).
    pub fn ual_name(&self) -> &'static str {
        match self {
            Self::W0 => "w0",
            Self::W1 => "w1",
            Self::W2 => "w2",
            Self::W3 => "w3",
            Self::W4 => "w4",
            Self::W5 => "w5",
            Self::W6 => "w6",
            Self::W7 => "w7",
            Self::W8 => "w8",
            Self::W9 => "w9",
            Self::W10 => "w10",
            Self::W11 => "w11",
            Self::W12 => "w12",
            Self::W13 => "w13",
            Self::W14 => "w14",
            Self::W15 => "w15",
            Self::W16 => "w16",
            Self::W17 => "w17",
            Self::W18 => "w18",
            Self::W19 => "w19",
            Self::W20 => "w20",
            Self::W21 => "w21",
            Self::W22 => "w22",
            Self::W23 => "w23",
            Self::W24 => "w24",
            Self::W25 => "w25",
            Self::W26 => "w26",
            Self::W27 => "w27",
            Self::W28 => "w28",
            Self::W29 => "w29",
            Self::W30 => "w30",
            Self::Wzr => "wzr",
            Self::Wsp => "wsp",
        }
    }
}
