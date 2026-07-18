// Copyright (c) Scaleservers LLC

/// The operand width of an AArch64 general-purpose register access: the **W** (32-bit) or **X** (64-bit)
/// view of the register file.
///
/// ## Why a width field (the W-vs-X design decision)
/// On AArch64 the operand size is NOT part of the register *number* -- the same 5-bit field selects the same
/// architectural register; an instruction's `sf` ("64-bit") bit decides whether that register is accessed as
/// the 64-bit `X` or the 32-bit `W` view (`x0`/`w0` are the two views of one register). We therefore keep a
/// single register-number enum ([`super::Arm64GeneralPurposeRegister`], values `0..=31`) and carry the width
/// as a **separate per-instruction-variant field** that maps directly to the single `sf` bit -- rather than
/// splitting the register file into two parallel W/X enums. The width-capable data-processing variants of
/// [`crate::Arm64Instruction`] carry this as their first field; [`Self::sf`] yields the encoding's `sf` bit.
///
/// (A handful of forms are width-fixed and do NOT carry this: `SMULH`/`UMULH` are always 64-bit, `ADR`/`ADRP`
/// always write a 64-bit `Xd`, the branches operate on 64-bit addresses, and the modeled load/store forms are
/// the 64-bit access. Those variants pin `sf`/size in their base constant and omit this field.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64RegisterWidth {
    /// The 32-bit (`Wn` / `wzr` / `wsp`) view -- `sf = 0`.
    W,
    /// The 64-bit (`Xn` / `xzr` / `sp`) view -- `sf = 1`.
    X,
}

impl Arm64RegisterWidth {
    /// The `sf` ("64-bit operand") bit value for this width: `1` for [`Self::X`], `0` for [`Self::W`]. The
    /// encoder shifts this into bit 31 of the instruction word for every width-capable family.
    pub fn sf(self) -> u32 {
        match self {
            Self::X => 1,
            Self::W => 0,
        }
    }
}
