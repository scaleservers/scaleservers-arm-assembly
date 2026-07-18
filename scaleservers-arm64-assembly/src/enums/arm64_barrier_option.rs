// Copyright (c) Scaleservers LLC

/// The shareability/access option of an AArch64 memory barrier (`DMB`/`DSB`), encoded in the `CRm` field
/// `[11:8]` (DDI0487 C6.2). Selects which observers and which accesses the barrier orders -- e.g. `ISH` (inner
/// shareable, the common multi-core fence), `ISHST` (inner-shareable stores only), `SY` (full system).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64BarrierOption {
    /// `OSHLD` -- outer shareable, loads (`CRm = 0b0001`).
    OshLd,
    /// `OSHST` -- outer shareable, stores (`CRm = 0b0010`).
    OshSt,
    /// `OSH` -- outer shareable, loads and stores (`CRm = 0b0011`).
    Osh,
    /// `NSHLD` -- non-shareable, loads (`CRm = 0b0101`).
    NshLd,
    /// `NSHST` -- non-shareable, stores (`CRm = 0b0110`).
    NshSt,
    /// `NSH` -- non-shareable, loads and stores (`CRm = 0b0111`).
    Nsh,
    /// `ISHLD` -- inner shareable, loads (`CRm = 0b1001`).
    IshLd,
    /// `ISHST` -- inner shareable, stores (`CRm = 0b1010`).
    IshSt,
    /// `ISH` -- inner shareable, loads and stores (`CRm = 0b1011`) -- the common SMP fence.
    Ish,
    /// `LD` -- full system, loads (`CRm = 0b1101`).
    Ld,
    /// `ST` -- full system, stores (`CRm = 0b1110`).
    St,
    /// `SY` -- full system, loads and stores (`CRm = 0b1111`) -- the strongest, default barrier.
    Sy,
}

impl Arm64BarrierOption {
    /// The 4-bit `CRm` field value (`[11:8]`).
    pub fn crm_bits(self) -> u32 {
        match self {
            Self::OshLd => 0b0001,
            Self::OshSt => 0b0010,
            Self::Osh => 0b0011,
            Self::NshLd => 0b0101,
            Self::NshSt => 0b0110,
            Self::Nsh => 0b0111,
            Self::IshLd => 0b1001,
            Self::IshSt => 0b1010,
            Self::Ish => 0b1011,
            Self::Ld => 0b1101,
            Self::St => 0b1110,
            Self::Sy => 0b1111,
        }
    }

    /// Recover the option from its 4-bit `CRm` field, or `None` for the four reserved/full-system-only values
    /// (`0b0000`/`0b0100`/`0b1000`/`0b1100`), which this cut renders via the `#<imm>` form rather than a name.
    pub fn from_crm_bits(bits: u32) -> Option<Self> {
        match bits & 0b1111 {
            0b0001 => Some(Self::OshLd),
            0b0010 => Some(Self::OshSt),
            0b0011 => Some(Self::Osh),
            0b0101 => Some(Self::NshLd),
            0b0110 => Some(Self::NshSt),
            0b0111 => Some(Self::Nsh),
            0b1001 => Some(Self::IshLd),
            0b1010 => Some(Self::IshSt),
            0b1011 => Some(Self::Ish),
            0b1101 => Some(Self::Ld),
            0b1110 => Some(Self::St),
            0b1111 => Some(Self::Sy),
            _ => None,
        }
    }

    /// The lowercase UAL option name (`osh`/`ish`/`sy`/...).
    pub fn name(self) -> &'static str {
        match self {
            Self::OshLd => "oshld",
            Self::OshSt => "oshst",
            Self::Osh => "osh",
            Self::NshLd => "nshld",
            Self::NshSt => "nshst",
            Self::Nsh => "nsh",
            Self::IshLd => "ishld",
            Self::IshSt => "ishst",
            Self::Ish => "ish",
            Self::Ld => "ld",
            Self::St => "st",
            Self::Sy => "sy",
        }
    }
}
