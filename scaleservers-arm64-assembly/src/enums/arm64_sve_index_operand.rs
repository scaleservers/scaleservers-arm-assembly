// Copyright (c) Scaleservers LLC

use crate::enums::Arm64GeneralPurposeRegister;

/// A base or step operand of the SVE `INDEX` instruction (DDI0487 part C). Each is independently either a 5-bit
/// signed immediate (`-16..=15`) or a general-purpose scalar register, giving the four `INDEX` forms
/// (imm/imm, reg/imm, imm/reg, reg/reg).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arm64SveIndexOperand {
    /// A 5-bit signed immediate (`-16..=15`).
    Immediate(i8),
    /// A general-purpose scalar register (rendered `Wn` for `.b`/`.h`/`.s`, `Xn` for `.d`).
    Register(Arm64GeneralPurposeRegister),
}

impl Arm64SveIndexOperand {
    /// Whether this operand is a register (selects the `reg` vs `imm` form bit).
    pub fn is_register(self) -> bool {
        matches!(self, Self::Register(_))
    }
}
