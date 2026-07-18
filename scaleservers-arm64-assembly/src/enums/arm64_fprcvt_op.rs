// Copyright (c) Scaleservers LLC

/// A FEAT_FPRCVT scalar convert between a floating-point register and an integer held in a **floating-point** register of
/// the other size (`Sd, Dn` / `Dd, Sn`). The op occupies `[21:16]`; its values are disjoint from the regular FP<->GP
/// converts so the two share the scalar 1-source frame without colliding. GNU+LLVM verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64FprcvtOp {
    /// `FCVTNS` -- FP to signed int, round to nearest (ties to even).
    Fcvtns,
    /// `FCVTNU` -- FP to unsigned int, round to nearest.
    Fcvtnu,
    /// `FCVTPS` -- FP to signed int, round toward +inf.
    Fcvtps,
    /// `FCVTPU` -- FP to unsigned int, round toward +inf.
    Fcvtpu,
    /// `FCVTMS` -- FP to signed int, round toward -inf.
    Fcvtms,
    /// `FCVTMU` -- FP to unsigned int, round toward -inf.
    Fcvtmu,
    /// `FCVTZS` -- FP to signed int, round toward zero.
    Fcvtzs,
    /// `FCVTZU` -- FP to unsigned int, round toward zero.
    Fcvtzu,
    /// `FCVTAS` -- FP to signed int, round to nearest (ties away).
    Fcvtas,
    /// `FCVTAU` -- FP to unsigned int, round to nearest (ties away).
    Fcvtau,
    /// `SCVTF` -- signed int to FP.
    Scvtf,
    /// `UCVTF` -- unsigned int to FP.
    Ucvtf,
}

impl Arm64FprcvtOp {
    /// The `[21:16]` op-selector value.
    pub fn opcode(self) -> u32 {
        match self {
            Self::Fcvtns => 0b101010,
            Self::Fcvtnu => 0b101011,
            Self::Fcvtps => 0b110010,
            Self::Fcvtpu => 0b110011,
            Self::Fcvtms => 0b110100,
            Self::Fcvtmu => 0b110101,
            Self::Fcvtzs => 0b110110,
            Self::Fcvtzu => 0b110111,
            Self::Fcvtas => 0b111010,
            Self::Fcvtau => 0b111011,
            Self::Scvtf => 0b111100,
            Self::Ucvtf => 0b111101,
        }
    }

    /// Whether this is an integer-to-FP convert (`SCVTF`/`UCVTF`); the rest are FP-to-integer.
    pub fn is_int_to_fp(self) -> bool {
        matches!(self, Self::Scvtf | Self::Ucvtf)
    }

    /// The lowercase UAL mnemonic.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Fcvtns => "fcvtns",
            Self::Fcvtnu => "fcvtnu",
            Self::Fcvtps => "fcvtps",
            Self::Fcvtpu => "fcvtpu",
            Self::Fcvtms => "fcvtms",
            Self::Fcvtmu => "fcvtmu",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtzu => "fcvtzu",
            Self::Fcvtas => "fcvtas",
            Self::Fcvtau => "fcvtau",
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",
        }
    }

    /// Recover the op from its `[21:16]` selector; `None` for any value not in the FPRCVT set (which keeps it disjoint
    /// from the regular FP<->GP converts that share the frame).
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        Some(match opcode & 0b111111 {
            0b101010 => Self::Fcvtns,
            0b101011 => Self::Fcvtnu,
            0b110010 => Self::Fcvtps,
            0b110011 => Self::Fcvtpu,
            0b110100 => Self::Fcvtms,
            0b110101 => Self::Fcvtmu,
            0b110110 => Self::Fcvtzs,
            0b110111 => Self::Fcvtzu,
            0b111010 => Self::Fcvtas,
            0b111011 => Self::Fcvtau,
            0b111100 => Self::Scvtf,
            0b111101 => Self::Ucvtf,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 12] = [
        Self::Fcvtns,
        Self::Fcvtnu,
        Self::Fcvtps,
        Self::Fcvtpu,
        Self::Fcvtms,
        Self::Fcvtmu,
        Self::Fcvtzs,
        Self::Fcvtzu,
        Self::Fcvtas,
        Self::Fcvtau,
        Self::Scvtf,
        Self::Ucvtf,
    ];
}
