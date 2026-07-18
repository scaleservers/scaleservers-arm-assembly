// Copyright (c) Scaleservers LLC

/// An SVE **predicated shift-by-vector** op over the destructive form `<op> Zdn.<T>, Pg/M, Zdn.<T>, Zm.<T>`
/// (DDI0487 part C, "SVE bitwise shift by vector (predicated)"). `Zdn` is shifted by the per-lane amount in `Zm`;
/// the reversed forms (`ASRR`/`LSRR`/`LSLR`) shift `Zm` by `Zdn` instead. The op is the 3-bit field `[18:16]`
/// over the shared base `0x0410_8000`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SvePredShiftVectorOp {
    /// `ASR` -- arithmetic (signed) shift right.
    Asr,
    /// `LSR` -- logical shift right.
    Lsr,
    /// `LSL` -- logical shift left.
    Lsl,
    /// `ASRR` -- reversed arithmetic shift right (`Zm >> Zdn`).
    Asrr,
    /// `LSRR` -- reversed logical shift right.
    Lsrr,
    /// `LSLR` -- reversed logical shift left.
    Lslr,
}

impl Arm64SvePredShiftVectorOp {
    /// The 3-bit op field (`[18:16]`).
    pub fn opc(self) -> u32 {
        match self {
            Self::Asr => 0b000,
            Self::Lsr => 0b001,
            Self::Lsl => 0b011,
            Self::Asrr => 0b100,
            Self::Lsrr => 0b101,
            Self::Lslr => 0b111,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Asr => "asr",
            Self::Lsr => "lsr",
            Self::Lsl => "lsl",
            Self::Asrr => "asrr",
            Self::Lsrr => "lsrr",
            Self::Lslr => "lslr",
        }
    }

    /// Recover the op from its 3-bit `[18:16]` field, if a modeled op (`010`/`110` are unallocated).
    pub fn from_opc(opc: u32) -> Option<Self> {
        match opc & 0b111 {
            0b000 => Some(Self::Asr),
            0b001 => Some(Self::Lsr),
            0b011 => Some(Self::Lsl),
            0b100 => Some(Self::Asrr),
            0b101 => Some(Self::Lsrr),
            0b111 => Some(Self::Lslr),
            _ => None,
        }
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 6] = [
        Self::Asr,
        Self::Lsr,
        Self::Lsl,
        Self::Asrr,
        Self::Lsrr,
        Self::Lslr,
    ];
}
