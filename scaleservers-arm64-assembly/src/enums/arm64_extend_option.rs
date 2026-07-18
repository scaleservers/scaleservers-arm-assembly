// Copyright (c) Scaleservers LLC

use crate::enums::Arm64RegisterWidth;

/// The extend operator of an add/subtract **extended-register** instruction (`ADD`/`ADDS`/`SUB`/`SUBS`,
/// extended-register form -- DDI0487 C6.2). The 3-bit `option` field (`[15:13]`) selects how the second source
/// register `Rm` is zero- or sign-extended from a sub-word before an optional left shift of `0..=4`.
///
/// This is the **stack-pointer-capable** add/sub form: unlike the shifted-register form, `Rd`/`Rn` may name the
/// stack pointer at field `31`, which is why compilers reach for it for stack-pointer arithmetic
/// (`add sp, sp, x0`) and for folding a 32-bit index into a 64-bit base with sign/zero extension
/// (`add x0, x1, w2, sxtw #2`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ExtendOption {
    /// `UXTB` -- zero-extend from byte (option `0b000`). `Rm` is a W register.
    Uxtb,
    /// `UXTH` -- zero-extend from halfword (option `0b001`). `Rm` is a W register.
    Uxth,
    /// `UXTW` -- zero-extend from word (option `0b010`). `Rm` is a W register; this is the `LSL` disassembly
    /// alias for the 32-bit form when a stack pointer is present.
    Uxtw,
    /// `UXTX` -- identity "extend" from doubleword (option `0b011`). `Rm` is an X register; this is the `LSL`
    /// disassembly alias for the 64-bit form when a stack pointer is present.
    Uxtx,
    /// `SXTB` -- sign-extend from byte (option `0b100`). `Rm` is a W register.
    Sxtb,
    /// `SXTH` -- sign-extend from halfword (option `0b101`). `Rm` is a W register.
    Sxth,
    /// `SXTW` -- sign-extend from word (option `0b110`). `Rm` is a W register.
    Sxtw,
    /// `SXTX` -- identity sign-"extend" from doubleword (option `0b111`). `Rm` is an X register.
    Sxtx,
}

impl Arm64ExtendOption {
    /// The 3-bit `option` field value (bits `[15:13]`), `0b000`..=`0b111`.
    pub fn as_bits(self) -> u32 {
        match self {
            Self::Uxtb => 0b000,
            Self::Uxth => 0b001,
            Self::Uxtw => 0b010,
            Self::Uxtx => 0b011,
            Self::Sxtb => 0b100,
            Self::Sxth => 0b101,
            Self::Sxtw => 0b110,
            Self::Sxtx => 0b111,
        }
    }

    /// Recover the option from its 3-bit field (only the low three bits are inspected).
    pub fn from_bits(bits: u32) -> Self {
        match bits & 0b111 {
            0b000 => Self::Uxtb,
            0b001 => Self::Uxth,
            0b010 => Self::Uxtw,
            0b011 => Self::Uxtx,
            0b100 => Self::Sxtb,
            0b101 => Self::Sxth,
            0b110 => Self::Sxtw,
            _ => Self::Sxtx,
        }
    }

    /// The lowercase UAL mnemonic (`uxtb`..`sxtx`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Uxtb => "uxtb",
            Self::Uxth => "uxth",
            Self::Uxtw => "uxtw",
            Self::Uxtx => "uxtx",
            Self::Sxtb => "sxtb",
            Self::Sxth => "sxth",
            Self::Sxtw => "sxtw",
            Self::Sxtx => "sxtx",
        }
    }

    /// The register width of the extended source `Rm`: an X register for `UXTX`/`SXTX` (the doubleword
    /// "extends"), a W register for every sub-word extend.
    pub fn source_width(self) -> Arm64RegisterWidth {
        match self {
            Self::Uxtx | Self::Sxtx => Arm64RegisterWidth::X,
            _ => Arm64RegisterWidth::W,
        }
    }
}
