// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 LSE atomic read-modify-write (`LDADD`/`LDCLR`/`LDEOR`/`LDSET`/`LD{S,U}{MAX,MIN}`,
/// DDI0487 C6.2 "Atomic memory operations"). The 3-bit `opc` field `[14:12]` selects the operation applied to
/// `[Xn]` with the source `Rs`; the prior value of `[Xn]` is returned in `Rt`. (`SWP` is a sibling with `o3 = 1`
/// and is modeled separately.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64AtomicOp {
    /// `LDADD` -- `[Xn] += Rs` (opc `0b000`).
    Add,
    /// `LDCLR` -- `[Xn] &= ~Rs` (bit clear, opc `0b001`).
    Clr,
    /// `LDEOR` -- `[Xn] ^= Rs` (opc `0b010`).
    Eor,
    /// `LDSET` -- `[Xn] |= Rs` (bit set, opc `0b011`).
    Set,
    /// `LDSMAX` -- `[Xn] = max(signed)` (opc `0b100`).
    Smax,
    /// `LDSMIN` -- `[Xn] = min(signed)` (opc `0b101`).
    Smin,
    /// `LDUMAX` -- `[Xn] = max(unsigned)` (opc `0b110`).
    Umax,
    /// `LDUMIN` -- `[Xn] = min(unsigned)` (opc `0b111`).
    Umin,
}

impl Arm64AtomicOp {
    /// The 3-bit `opc` field value (`[14:12]`).
    pub fn opc_bits(self) -> u32 {
        match self {
            Self::Add => 0b000,
            Self::Clr => 0b001,
            Self::Eor => 0b010,
            Self::Set => 0b011,
            Self::Smax => 0b100,
            Self::Smin => 0b101,
            Self::Umax => 0b110,
            Self::Umin => 0b111,
        }
    }

    /// Recover the operation from its 3-bit `opc` field (total over the low three bits).
    pub fn from_opc_bits(bits: u32) -> Self {
        match bits & 0b111 {
            0b000 => Self::Add,
            0b001 => Self::Clr,
            0b010 => Self::Eor,
            0b011 => Self::Set,
            0b100 => Self::Smax,
            0b101 => Self::Smin,
            0b110 => Self::Umax,
            _ => Self::Umin,
        }
    }

    /// The mnemonic stem after the `ld` prefix (`add`/`clr`/`eor`/`set`/`smax`/`smin`/`umax`/`umin`); the full
    /// mnemonic is `ld<stem><ordering><size>` (e.g. `ldaddal`, `ldclrb`).
    pub fn stem(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Clr => "clr",
            Self::Eor => "eor",
            Self::Set => "set",
            Self::Smax => "smax",
            Self::Smin => "smin",
            Self::Umax => "umax",
            Self::Umin => "umin",
        }
    }
}
