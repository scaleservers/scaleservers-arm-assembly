// Copyright (c) Scaleservers LLC

/// A scalar **floating-point to general-purpose integer** convert with an explicit rounding mode (DDI0487 C7):
/// `FCVT{N,A,P,M}{S,U}`. The toward-zero forms `FCVTZS`/`FCVTZU` are modeled separately (they predate this enum);
/// these cover the other four rounding modes -- nearest-ties-even (`N`), nearest-ties-away (`A`), toward +inf (`P`,
/// plus), toward -inf (`M`, minus) -- in signed (`S`) and unsigned (`U`) forms. The convert reads `Sn`/`Dn`/`Hn`
/// and writes `Wd`/`Xd`.
///
/// The op is carried in the `rmode<20:19>` + `opcode<18:16>` fields: rounding via `rmode` (N/A = 00, P = 01, M = 10),
/// the away-from-zero `A` forms via opcode bit 2, and the unsigned `U` forms via opcode bit 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64FpToIntRoundOp {
    /// `FCVTNS` -- round to nearest, ties to even; signed.
    Fcvtns,
    /// `FCVTNU` -- round to nearest, ties to even; unsigned.
    Fcvtnu,
    /// `FCVTAS` -- round to nearest, ties away from zero; signed.
    Fcvtas,
    /// `FCVTAU` -- round to nearest, ties away from zero; unsigned.
    Fcvtau,
    /// `FCVTPS` -- round toward +infinity; signed.
    Fcvtps,
    /// `FCVTPU` -- round toward +infinity; unsigned.
    Fcvtpu,
    /// `FCVTMS` -- round toward -infinity; signed.
    Fcvtms,
    /// `FCVTMU` -- round toward -infinity; unsigned.
    Fcvtmu,
}

impl Arm64FpToIntRoundOp {
    /// The 2-bit `rmode<20:19>` field.
    pub fn rmode(self) -> u32 {
        match self {
            Self::Fcvtns | Self::Fcvtnu | Self::Fcvtas | Self::Fcvtau => 0b00,
            Self::Fcvtps | Self::Fcvtpu => 0b01,
            Self::Fcvtms | Self::Fcvtmu => 0b10,
        }
    }

    /// The 3-bit `opcode<18:16>` field (bit 2 = the away-from-zero `A` forms; bit 0 = unsigned).
    pub fn opcode(self) -> u32 {
        match self {
            Self::Fcvtns => 0b000,
            Self::Fcvtnu => 0b001,
            Self::Fcvtas => 0b100,
            Self::Fcvtau => 0b101,
            Self::Fcvtps => 0b000,
            Self::Fcvtpu => 0b001,
            Self::Fcvtms => 0b000,
            Self::Fcvtmu => 0b001,
        }
    }

    /// Recover the op from the `rmode<20:19>` + `opcode<18:16>` fields, or `None` if the pair is not one of these
    /// eight (e.g. rmode 11 = the toward-zero `FCVTZS`/`FCVTZU`, opcode 01x = `SCVTF`/`UCVTF`, opcode 11x = `FMOV`).
    pub fn from_fields(rmode: u32, opcode: u32) -> Option<Self> {
        Some(match (rmode & 0b11, opcode & 0b111) {
            (0b00, 0b000) => Self::Fcvtns,
            (0b00, 0b001) => Self::Fcvtnu,
            (0b00, 0b100) => Self::Fcvtas,
            (0b00, 0b101) => Self::Fcvtau,
            (0b01, 0b000) => Self::Fcvtps,
            (0b01, 0b001) => Self::Fcvtpu,
            (0b10, 0b000) => Self::Fcvtms,
            (0b10, 0b001) => Self::Fcvtmu,
            _ => return None,
        })
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fcvtns => "fcvtns",
            Self::Fcvtnu => "fcvtnu",
            Self::Fcvtas => "fcvtas",
            Self::Fcvtau => "fcvtau",
            Self::Fcvtps => "fcvtps",
            Self::Fcvtpu => "fcvtpu",
            Self::Fcvtms => "fcvtms",
            Self::Fcvtmu => "fcvtmu",
        }
    }

    /// Every op, for exhaustive round-trip testing.
    pub const ALL: [Self; 8] = [
        Self::Fcvtns,
        Self::Fcvtnu,
        Self::Fcvtas,
        Self::Fcvtau,
        Self::Fcvtps,
        Self::Fcvtpu,
        Self::Fcvtms,
        Self::Fcvtmu,
    ];
}
