// Copyright (c) Scaleservers LLC

/// A PSTATE field writable by `MSR (immediate)` (DDI0487 C6 -- `MSR <pstatefield>, #<imm>`). The field is selected
/// by the `(op1, op2)` pair; the 4-bit `CRm` carries the immediate. The single-bit fields take `#0`/`#1`; the
/// `DAIF` masks take a 4-bit value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64PstateField {
    /// `SPSel` -- stack-pointer select (op1 0, op2 5).
    Spsel,
    /// `DAIFSet` -- set the DAIF interrupt-mask bits (op1 3, op2 6).
    Daifset,
    /// `DAIFClr` -- clear the DAIF interrupt-mask bits (op1 3, op2 7).
    Daifclr,
    /// `PAN` -- privileged access never (FEAT_PAN, op1 0, op2 4).
    Pan,
    /// `UAO` -- user access override (FEAT_UAO, op1 0, op2 3).
    Uao,
    /// `DIT` -- data-independent timing (FEAT_DIT, op1 3, op2 2).
    Dit,
    /// `SSBS` -- speculative store-bypass safe (FEAT_SSBS, op1 3, op2 1).
    Ssbs,
    /// `TCO` -- tag check override (FEAT_MTE, op1 3, op2 4).
    Tco,
    /// `ALLINT` -- mask all interrupts (FEAT_NMI, op1 1, op2 0).
    Allint,
}

impl Arm64PstateField {
    /// The `(op1, op2)` selector pair.
    pub fn op1_op2(self) -> (u32, u32) {
        match self {
            Self::Spsel => (0, 5),
            Self::Daifset => (3, 6),
            Self::Daifclr => (3, 7),
            Self::Pan => (0, 4),
            Self::Uao => (0, 3),
            Self::Dit => (3, 2),
            Self::Ssbs => (3, 1),
            Self::Tco => (3, 4),
            Self::Allint => (1, 0),
        }
    }

    /// The lowercase UAL field name (matching GNU's disassembly).
    pub fn name(self) -> &'static str {
        match self {
            Self::Spsel => "spsel",
            Self::Daifset => "daifset",
            Self::Daifclr => "daifclr",
            Self::Pan => "pan",
            Self::Uao => "uao",
            Self::Dit => "dit",
            Self::Ssbs => "ssbs",
            Self::Tco => "tco",
            Self::Allint => "allint",
        }
    }

    /// Recover the field from its `(op1, op2)` selector, if it is a modeled PSTATE field.
    pub fn from_op1_op2(op1: u32, op2: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|field| field.op1_op2() == (op1, op2))
    }

    /// Every modeled field, for tests and table-driven decode.
    pub const ALL: [Self; 9] = [
        Self::Spsel,
        Self::Daifset,
        Self::Daifclr,
        Self::Pan,
        Self::Uao,
        Self::Dit,
        Self::Ssbs,
        Self::Tco,
        Self::Allint,
    ];
}
