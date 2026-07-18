// Copyright (c) Scaleservers LLC

use crate::enums::Arm64RegisterWidth;

/// The index-extend operator of an AArch64 load/store **register-offset** address `[Xn, Rm{, <ext> #amount}]`
/// (DDI0487 C6.2 "Load/store register (register offset)"). The 3-bit `option` field `[15:13]` selects how the
/// index register `Rm` is extended before being added to the base; memory addressing allows only these four of
/// the eight extend codes (the byte/halfword extends are reserved here). `LSL` is the `UXTX` (`0b011`) code -- an
/// `X` index used directly -- and is the operator of the common `[Xn, Xm]` / `[Xn, Xm, lsl #n]` form.
///
/// The shift amount is not free: the `S` bit `[12]` of the encoding selects either no shift (`S = 0`) or a left
/// shift by `log2(access_size)` (`S = 1`), so the instruction carries `S` as a boolean and the rendered amount
/// is derived from the access size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64MemoryExtend {
    /// `UXTW` -- the `W` index is zero-extended (option `0b010`).
    Uxtw,
    /// `LSL` -- the `X` index is used directly (option `0b011`, architecturally `UXTX`); the operator of the
    /// plain `[Xn, Xm]` form.
    Lsl,
    /// `SXTW` -- the `W` index is sign-extended (option `0b110`).
    Sxtw,
    /// `SXTX` -- the `X` index is used directly with the sign-extend code (option `0b111`).
    Sxtx,
}

impl Arm64MemoryExtend {
    /// The 3-bit `option` field value (`[15:13]`).
    pub fn as_bits(self) -> u32 {
        match self {
            Self::Uxtw => 0b010,
            Self::Lsl => 0b011,
            Self::Sxtw => 0b110,
            Self::Sxtx => 0b111,
        }
    }

    /// Recover the operator from its 3-bit field, or `None` for the four codes reserved in memory addressing
    /// (`0b000`/`0b001`/`0b100`/`0b101`) -- the decoder treats those as `InvalidOpcode`.
    pub fn from_bits(bits: u32) -> Option<Self> {
        match bits & 0b111 {
            0b010 => Some(Self::Uxtw),
            0b011 => Some(Self::Lsl),
            0b110 => Some(Self::Sxtw),
            0b111 => Some(Self::Sxtx),
            _ => None,
        }
    }

    /// The lowercase UAL mnemonic (`uxtw`/`lsl`/`sxtw`/`sxtx`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Uxtw => "uxtw",
            Self::Lsl => "lsl",
            Self::Sxtw => "sxtw",
            Self::Sxtx => "sxtx",
        }
    }

    /// The register width of the index `Rm`: `W` for the word extends (`UXTW`/`SXTW`), `X` for `LSL`/`SXTX`.
    pub fn index_width(self) -> Arm64RegisterWidth {
        match self {
            Self::Uxtw | Self::Sxtw => Arm64RegisterWidth::W,
            Self::Lsl | Self::Sxtx => Arm64RegisterWidth::X,
        }
    }
}
