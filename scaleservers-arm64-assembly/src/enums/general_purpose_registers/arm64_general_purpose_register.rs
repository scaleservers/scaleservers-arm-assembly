// Copyright (c) Scaleservers LLC

use crate::enums::Arm64RegisterWidth;

/// A 64-bit (X) general-purpose register operand: the 5-bit register field found across A64.
///
/// AArch64 has 31 addressable general-purpose registers, `X0`..`X30`. The 32nd encoding, the field value
/// `31` (`0b11111`), is context-dependent: in MOST operand positions it names the **zero register** `XZR`
/// (reads as 0, writes are discarded), but in a handful of positions (the `Rn`/`Rd` of the add/subtract
/// *immediate* forms, and load/store base registers) it instead names the **stack pointer** `SP`.
///
/// ## How the 31 duality is modeled here
/// This enum carries `Xzr` and `Sp` as **two distinct variants that both encode to `0b11111`**. They are
/// therefore distinguishable in the model (so an emitter can print `sp` vs `xzr`, and so a future encoder
/// can reject `SP` where only `XZR` is legal and vice versa), yet they share one bit pattern on the wire.
/// Because the bit pattern is identical, the decoder cannot know from the field alone which was meant -- it
/// resolves `0b11111` to whichever interpretation the *instruction* dictates (SP for add/sub-immediate and
/// memory base; XZR elsewhere). The per-instruction decode arms pick the right one; this is documented on
/// the affected variants of `Arm64Instruction`.
///
/// `from_operand_bits` (mask, never panics) maps `31` to `Xzr` as the neutral default; instruction decode
/// arms that want `Sp` substitute it explicitly. `as_operand_bits` maps both `Xzr` and `Sp` to `31`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64GeneralPurposeRegister {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29, // frame pointer (FP) by convention
    X30, // link register (LR) by convention
    /// The zero register: field value `0b11111` in positions where `31` reads as zero / discards writes.
    Xzr,
    /// The stack pointer: field value `0b11111` in positions where `31` names SP (add/sub-immediate, memory base).
    Sp,
}

impl Arm64GeneralPurposeRegister {
    /// The 5-bit register field value. Both [`Self::Xzr`] and [`Self::Sp`] encode to `31` (`0b11111`);
    /// which one a given field value `31` means on decode depends on the instruction (see the type docs).
    pub fn as_operand_bits(&self) -> u8 {
        match self {
            Self::X0 => 0,
            Self::X1 => 1,
            Self::X2 => 2,
            Self::X3 => 3,
            Self::X4 => 4,
            Self::X5 => 5,
            Self::X6 => 6,
            Self::X7 => 7,
            Self::X8 => 8,
            Self::X9 => 9,
            Self::X10 => 10,
            Self::X11 => 11,
            Self::X12 => 12,
            Self::X13 => 13,
            Self::X14 => 14,
            Self::X15 => 15,
            Self::X16 => 16,
            Self::X17 => 17,
            Self::X18 => 18,
            Self::X19 => 19,
            Self::X20 => 20,
            Self::X21 => 21,
            Self::X22 => 22,
            Self::X23 => 23,
            Self::X24 => 24,
            Self::X25 => 25,
            Self::X26 => 26,
            Self::X27 => 27,
            Self::X28 => 28,
            Self::X29 => 29,
            Self::X30 => 30,
            Self::Xzr => 31,
            Self::Sp => 31,
        }
    }

    /// Map a 5-bit register field to its register. TOTAL: only the low five bits are significant (every
    /// value `0..=31` is covered), so this never panics -- important because the decoder derives register
    /// numbers from untrusted instruction words. Higher bits are ignored.
    ///
    /// Field value `31` resolves to [`Self::Xzr`] (the common case). Instruction-decode arms in positions
    /// where `31` means the stack pointer substitute [`Self::Sp`] themselves.
    pub fn from_operand_bits(bits: u8) -> Self {
        match bits & 0b1_1111 {
            0 => Self::X0,
            1 => Self::X1,
            2 => Self::X2,
            3 => Self::X3,
            4 => Self::X4,
            5 => Self::X5,
            6 => Self::X6,
            7 => Self::X7,
            8 => Self::X8,
            9 => Self::X9,
            10 => Self::X10,
            11 => Self::X11,
            12 => Self::X12,
            13 => Self::X13,
            14 => Self::X14,
            15 => Self::X15,
            16 => Self::X16,
            17 => Self::X17,
            18 => Self::X18,
            19 => Self::X19,
            20 => Self::X20,
            21 => Self::X21,
            22 => Self::X22,
            23 => Self::X23,
            24 => Self::X24,
            25 => Self::X25,
            26 => Self::X26,
            27 => Self::X27,
            28 => Self::X28,
            29 => Self::X29,
            30 => Self::X30,
            // `& 0b1_1111` makes 31 the only remaining value; the arm is exhaustive.
            _ => Self::Xzr,
        }
    }

    /// Like [`Self::from_operand_bits`], but resolves field value `31` to the **stack pointer** ([`Self::Sp`])
    /// instead of `XZR`. Used by the decode arms of instructions whose `31` slot is SP (add/sub-immediate,
    /// load/store base register).
    pub fn from_operand_bits_sp(bits: u8) -> Self {
        match bits & 0b1_1111 {
            31 => Self::Sp,
            other => Self::from_operand_bits(other),
        }
    }

    /// The lowercase UAL register name at the given operand [`Arm64RegisterWidth`], in the **zero-register**
    /// view of field `31` (`wzr`/`xzr`). `W` renders `w0`..`w30`/`wzr`; `X` renders `x0`..`x30`/`xzr`. This is
    /// the width-aware emit entry point for the data-processing families, where field `31` is the zero
    /// register. (The [`Self::Sp`] variant -- which only arises in SP-positioned operands -- also maps here to
    /// `wzr`/`xzr`; SP-positioned operands render through [`Self::name_for_width_sp`] instead.)
    pub fn name_for_width(&self, width: Arm64RegisterWidth) -> &'static str {
        match width {
            Arm64RegisterWidth::W => self.w_name(),
            Arm64RegisterWidth::X => self.x_name(),
        }
    }

    /// Like [`Self::name_for_width`], but renders field `31` in the **stack-pointer** view (`wsp`/`sp`). Used
    /// for the operand positions where `31` names the stack pointer (the add/sub-immediate `Rd`/`Rn`). The X
    /// stack pointer prints as the bare `sp` (not `xsp`); the W view prints as `wsp`.
    pub fn name_for_width_sp(&self, width: Arm64RegisterWidth) -> &'static str {
        match self {
            Self::Sp | Self::Xzr => match width {
                Arm64RegisterWidth::W => "wsp",
                Arm64RegisterWidth::X => "sp",
            },
            other => other.name_for_width(width),
        }
    }

    /// The 32-bit (`W`) view name: `w0`..`w30`, with field `31` as `wzr`.
    fn w_name(&self) -> &'static str {
        match self {
            Self::X0 => "w0",
            Self::X1 => "w1",
            Self::X2 => "w2",
            Self::X3 => "w3",
            Self::X4 => "w4",
            Self::X5 => "w5",
            Self::X6 => "w6",
            Self::X7 => "w7",
            Self::X8 => "w8",
            Self::X9 => "w9",
            Self::X10 => "w10",
            Self::X11 => "w11",
            Self::X12 => "w12",
            Self::X13 => "w13",
            Self::X14 => "w14",
            Self::X15 => "w15",
            Self::X16 => "w16",
            Self::X17 => "w17",
            Self::X18 => "w18",
            Self::X19 => "w19",
            Self::X20 => "w20",
            Self::X21 => "w21",
            Self::X22 => "w22",
            Self::X23 => "w23",
            Self::X24 => "w24",
            Self::X25 => "w25",
            Self::X26 => "w26",
            Self::X27 => "w27",
            Self::X28 => "w28",
            Self::X29 => "w29",
            Self::X30 => "w30",
            Self::Xzr => "wzr",
            Self::Sp => "wzr",
        }
    }

    /// The 64-bit (`X`) view name: `x0`..`x30`, with field `31` as `xzr`. (This is the same mapping as the
    /// legacy [`Self::ual_name`], minus the SP special-case -- callers wanting the SP view use
    /// [`Self::name_for_width_sp`].)
    fn x_name(&self) -> &'static str {
        match self {
            Self::Sp => "xzr",
            other => other.ual_name(),
        }
    }

    /// The lowercase UAL register name for this operand. `X29`/`X30` render as `x29`/`x30` (the `fp`/`lr`
    /// aliases are conventions, not separate encodings, so the canonical `x`-names are emitted); `31`
    /// renders as `xzr` or `sp` depending on the variant.
    pub fn ual_name(&self) -> &'static str {
        match self {
            Self::X0 => "x0",
            Self::X1 => "x1",
            Self::X2 => "x2",
            Self::X3 => "x3",
            Self::X4 => "x4",
            Self::X5 => "x5",
            Self::X6 => "x6",
            Self::X7 => "x7",
            Self::X8 => "x8",
            Self::X9 => "x9",
            Self::X10 => "x10",
            Self::X11 => "x11",
            Self::X12 => "x12",
            Self::X13 => "x13",
            Self::X14 => "x14",
            Self::X15 => "x15",
            Self::X16 => "x16",
            Self::X17 => "x17",
            Self::X18 => "x18",
            Self::X19 => "x19",
            Self::X20 => "x20",
            Self::X21 => "x21",
            Self::X22 => "x22",
            Self::X23 => "x23",
            Self::X24 => "x24",
            Self::X25 => "x25",
            Self::X26 => "x26",
            Self::X27 => "x27",
            Self::X28 => "x28",
            Self::X29 => "x29",
            Self::X30 => "x30",
            Self::Xzr => "xzr",
            Self::Sp => "sp",
        }
    }
}
