// Copyright (c) Scaleservers LLC

/// A **pointer-authentication** data-processing op (DDI0487 C6, FEAT_PAuth) -- the data-processing (1-source)
/// `1 1 0 11010110 00001 opcode Rn Rd` encoding (always 64-bit). PAC*/AUT* sign / authenticate a pointer in `Xd`
/// with a modifier; the `*Z*` forms use a zero modifier (and `XPACI`/`XPACD`) so `Rn` is fixed to `11111`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64PacOp {
    /// `PACIA Xd, Xn` -- sign with key A, instruction.
    Pacia,
    /// `PACIB Xd, Xn` -- sign with key B, instruction.
    Pacib,
    /// `PACDA Xd, Xn` -- sign with key A, data.
    Pacda,
    /// `PACDB Xd, Xn` -- sign with key B, data.
    Pacdb,
    /// `AUTIA Xd, Xn` -- authenticate key A, instruction.
    Autia,
    /// `AUTIB Xd, Xn` -- authenticate key B, instruction.
    Autib,
    /// `AUTDA Xd, Xn` -- authenticate key A, data.
    Autda,
    /// `AUTDB Xd, Xn` -- authenticate key B, data.
    Autdb,
    /// `PACIZA Xd` -- sign with key A, instruction, zero modifier.
    Paciza,
    /// `PACIZB Xd` -- sign with key B, instruction, zero modifier.
    Pacizb,
    /// `PACDZA Xd` -- sign with key A, data, zero modifier.
    Pacdza,
    /// `PACDZB Xd` -- sign with key B, data, zero modifier.
    Pacdzb,
    /// `AUTIZA Xd` -- authenticate key A, instruction, zero modifier.
    Autiza,
    /// `AUTIZB Xd` -- authenticate key B, instruction, zero modifier.
    Autizb,
    /// `AUTDZA Xd` -- authenticate key A, data, zero modifier.
    Autdza,
    /// `AUTDZB Xd` -- authenticate key B, data, zero modifier.
    Autdzb,
    /// `XPACI Xd` -- strip the instruction pointer-auth code.
    Xpaci,
    /// `XPACD Xd` -- strip the data pointer-auth code.
    Xpacd,
}

impl Arm64PacOp {
    /// The base word (`Rn`/`Rd` zero): `0xDAC1_0000 | (opcode<<10)`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        0xDAC1_0000 | (self.opcode() << 10)
    }

    fn opcode(self) -> u32 {
        match self {
            Self::Pacia => 0,
            Self::Pacib => 1,
            Self::Pacda => 2,
            Self::Pacdb => 3,
            Self::Autia => 4,
            Self::Autib => 5,
            Self::Autda => 6,
            Self::Autdb => 7,
            Self::Paciza => 8,
            Self::Pacizb => 9,
            Self::Pacdza => 10,
            Self::Pacdzb => 11,
            Self::Autiza => 12,
            Self::Autizb => 13,
            Self::Autdza => 14,
            Self::Autdzb => 15,
            Self::Xpaci => 16,
            Self::Xpacd => 17,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Pacia => "pacia",
            Self::Pacib => "pacib",
            Self::Pacda => "pacda",
            Self::Pacdb => "pacdb",
            Self::Autia => "autia",
            Self::Autib => "autib",
            Self::Autda => "autda",
            Self::Autdb => "autdb",
            Self::Paciza => "paciza",
            Self::Pacizb => "pacizb",
            Self::Pacdza => "pacdza",
            Self::Pacdzb => "pacdzb",
            Self::Autiza => "autiza",
            Self::Autizb => "autizb",
            Self::Autdza => "autdza",
            Self::Autdzb => "autdzb",
            Self::Xpaci => "xpaci",
            Self::Xpacd => "xpacd",
        }
    }

    /// Whether this op takes a modifier register `Xn` (`PAC*`/`AUT*` with opcode 0..7); the `*Z*`/`XPAC*` forms
    /// have a fixed `Rn = 11111` and print only `Xd`.
    pub fn uses_modifier(self) -> bool {
        self.opcode() < 8
    }

    /// Recover the op from a masked base (`word & 0xFFFF_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 18] = [
        Self::Pacia,
        Self::Pacib,
        Self::Pacda,
        Self::Pacdb,
        Self::Autia,
        Self::Autib,
        Self::Autda,
        Self::Autdb,
        Self::Paciza,
        Self::Pacizb,
        Self::Pacdza,
        Self::Pacdzb,
        Self::Autiza,
        Self::Autizb,
        Self::Autdza,
        Self::Autdzb,
        Self::Xpaci,
        Self::Xpacd,
    ];
}
