// Copyright (c) Scaleservers LLC

/// An SVE **predicated shift-by-immediate** op (DDI0487 part C): `ASR`/`LSR`/`LSL` over `Zdn.<T>, Pg/M, Zdn.<T>,
/// #shift`. The element size and shift amount pack into a 7-bit `tszh:tszl:imm3` value (`esize + shift` for the
/// left shift, `2*esize - shift` for the right shifts).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveShiftImmOp {
    /// `ASR` -- arithmetic (signed) shift right.
    Asr,
    /// `LSR` -- logical shift right.
    Lsr,
    /// `LSL` -- logical shift left.
    Lsl,
    /// `ASRD` -- arithmetic shift right for divide (rounds toward zero; the `signed-divide-by-power-of-two` helper).
    Asrd,
}

impl Arm64SveShiftImmOp {
    /// The 3-bit `opc` field (`[18:16]`): ASR 000 / LSR 001 / LSL 011 / ASRD 100.
    pub fn opc(self) -> u32 {
        match self {
            Self::Asr => 0b000,
            Self::Lsr => 0b001,
            Self::Lsl => 0b011,
            Self::Asrd => 0b100,
        }
    }

    /// Whether this is a left shift (`esize + shift` packing) vs a right shift (`2*esize - shift`). Only `LSL` is
    /// left; `ASR`/`LSR`/`ASRD` are right shifts.
    pub fn is_left(self) -> bool {
        matches!(self, Self::Lsl)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Asr => "asr",
            Self::Lsr => "lsr",
            Self::Lsl => "lsl",
            Self::Asrd => "asrd",
        }
    }

    /// Recover the op from its 3-bit `opc` field, if a modeled op (`010` and `101`..`111` are unallocated /
    /// SVE2 forms handled elsewhere).
    pub fn from_opc(opc: u32) -> Option<Self> {
        match opc & 0b111 {
            0b000 => Some(Self::Asr),
            0b001 => Some(Self::Lsr),
            0b011 => Some(Self::Lsl),
            0b100 => Some(Self::Asrd),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Asr, Self::Lsr, Self::Lsl, Self::Asrd];
}
