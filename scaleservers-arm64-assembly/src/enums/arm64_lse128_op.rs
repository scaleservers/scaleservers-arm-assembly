// Copyright (c) Scaleservers LLC

/// A FEAT_LSE128 128-bit atomic memory operation on a 64-bit register pair (`SWPP`/`LDCLRP`/`LDSETP`). The op is the
/// `[15:12]` selector of the shared frame; the `A`/`L` acquire/release bits are carried separately. GNU+LLVM verified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64Lse128Op {
    /// `SWPP` -- swap a 128-bit register pair with memory.
    Swpp,
    /// `LDCLRP` -- atomic bit-clear of a 128-bit pair (load original, store `mem & ~value`).
    Ldclrp,
    /// `LDSETP` -- atomic bit-set of a 128-bit pair (load original, store `mem | value`).
    Ldsetp,
}

impl Arm64Lse128Op {
    /// The `[15:12]` op-selector value.
    pub const fn opcode(self) -> u32 {
        match self {
            Self::Swpp => 0b1000,
            Self::Ldclrp => 0b0001,
            Self::Ldsetp => 0b0011,
        }
    }

    /// The base (no-ordering) mnemonic; the `A`/`L` ordering adds the `a`/`l`/`al` suffix.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Swpp => "swpp",
            Self::Ldclrp => "ldclrp",
            Self::Ldsetp => "ldsetp",
        }
    }

    /// Recover the op from the `[15:12]` selector; `None` for an unmodeled value.
    pub const fn from_opcode(opcode: u32) -> Option<Self> {
        Some(match opcode & 0b1111 {
            0b1000 => Self::Swpp,
            0b0001 => Self::Ldclrp,
            0b0011 => Self::Ldsetp,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 3] = [Self::Swpp, Self::Ldclrp, Self::Ldsetp];
}
