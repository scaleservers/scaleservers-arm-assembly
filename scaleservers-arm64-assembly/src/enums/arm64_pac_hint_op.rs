// Copyright (c) Scaleservers LLC

/// A **pointer-authentication hint** (DDI0487 C6, FEAT_PAuth) -- the operand-free `PAC*`/`AUT*` forms that act on
/// `X30` (LR) with the SP or a zero modifier, plus `XPACLRI`. They sit in the hint (NOP) space, so each is a fixed
/// 32-bit word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64PacHintOp {
    /// `PACIAZ` -- sign LR, key A, zero modifier.
    Paciaz,
    /// `PACIASP` -- sign LR, key A, SP modifier.
    Paciasp,
    /// `PACIBZ` -- sign LR, key B, zero modifier.
    Pacibz,
    /// `PACIBSP` -- sign LR, key B, SP modifier.
    Pacibsp,
    /// `AUTIAZ` -- authenticate LR, key A, zero modifier.
    Autiaz,
    /// `AUTIASP` -- authenticate LR, key A, SP modifier.
    Autiasp,
    /// `AUTIBZ` -- authenticate LR, key B, zero modifier.
    Autibz,
    /// `AUTIBSP` -- authenticate LR, key B, SP modifier.
    Autibsp,
    /// `XPACLRI` -- strip the instruction PAC from LR.
    Xpaclri,
    /// `PACIA1716` -- sign LR, key A, X17 modifier (the X16/X17 hint form).
    Pacia1716,
    /// `PACIB1716` -- sign LR, key B, X17 modifier.
    Pacib1716,
    /// `AUTIA1716` -- authenticate LR, key A, X17 modifier.
    Autia1716,
    /// `AUTIB1716` -- authenticate LR, key B, X17 modifier.
    Autib1716,
}

impl Arm64PacHintOp {
    /// The fixed instruction word. GNU+LLVM dual-oracle verified.
    pub fn word(self) -> u32 {
        match self {
            Self::Paciaz => 0xD503_231F,
            Self::Paciasp => 0xD503_233F,
            Self::Pacibz => 0xD503_235F,
            Self::Pacibsp => 0xD503_237F,
            Self::Autiaz => 0xD503_239F,
            Self::Autiasp => 0xD503_23BF,
            Self::Autibz => 0xD503_23DF,
            Self::Autibsp => 0xD503_23FF,
            Self::Xpaclri => 0xD503_20FF,
            Self::Pacia1716 => 0xD503_211F,
            Self::Pacib1716 => 0xD503_215F,
            Self::Autia1716 => 0xD503_219F,
            Self::Autib1716 => 0xD503_21DF,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Paciaz => "paciaz",
            Self::Paciasp => "paciasp",
            Self::Pacibz => "pacibz",
            Self::Pacibsp => "pacibsp",
            Self::Autiaz => "autiaz",
            Self::Autiasp => "autiasp",
            Self::Autibz => "autibz",
            Self::Autibsp => "autibsp",
            Self::Xpaclri => "xpaclri",
            Self::Pacia1716 => "pacia1716",
            Self::Pacib1716 => "pacib1716",
            Self::Autia1716 => "autia1716",
            Self::Autib1716 => "autib1716",
        }
    }

    /// Recover the op from a full instruction word; `None` if it is not one of these.
    pub fn from_word(word: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.word() == word)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 13] = [
        Self::Paciaz,
        Self::Paciasp,
        Self::Pacibz,
        Self::Pacibsp,
        Self::Autiaz,
        Self::Autiasp,
        Self::Autibz,
        Self::Autibsp,
        Self::Xpaclri,
        Self::Pacia1716,
        Self::Pacib1716,
        Self::Autia1716,
        Self::Autib1716,
    ];
}

/// The target operand of a **`BTI`** (Branch Target Identification, FEAT_BTI) hint -- which indirect-branch kinds
/// are permitted to land on the following instruction. A fixed word in the hint space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64BtiTarget {
    /// `BTI` (no target) -- neither call nor jump may land here (only fall-through).
    Unset,
    /// `BTI C` -- a call (`BLR`) may land here.
    Call,
    /// `BTI J` -- a jump (`BR`) may land here.
    Jump,
    /// `BTI JC` -- either a call or a jump may land here.
    JumpCall,
}

impl Arm64BtiTarget {
    /// The fixed instruction word. GNU+LLVM dual-oracle verified.
    pub fn word(self) -> u32 {
        match self {
            Self::Unset => 0xD503_241F,
            Self::Call => 0xD503_245F,
            Self::Jump => 0xD503_249F,
            Self::JumpCall => 0xD503_24DF,
        }
    }

    /// The operand suffix printed after `bti` (`""`/` c`/` j`/` jc`).
    pub fn suffix(self) -> &'static str {
        match self {
            Self::Unset => "",
            Self::Call => " c",
            Self::Jump => " j",
            Self::JumpCall => " jc",
        }
    }

    /// Recover the target from a full instruction word; `None` if it is not a `BTI`.
    pub fn from_word(word: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|t| t.word() == word)
    }

    /// Every target, for tests.
    pub const ALL: [Self; 4] = [Self::Unset, Self::Call, Self::Jump, Self::JumpCall];
}
