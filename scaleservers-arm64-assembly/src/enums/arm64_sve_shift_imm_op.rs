// Copyright (c) Scaleservers LLC

/// An SVE **predicated shift-by-immediate** op (DDI0487 part C): `ASR`/`LSR`/`LSL`/`ASRD` (FEAT_SVE) and the
/// saturating/rounding FEAT_SVE2 rows over `Zdn.<T>, Pg/M, Zdn.<T>, #shift`. The element size and shift amount
/// pack into a 7-bit `tszh:tszl:imm3` value (`esize + shift` for the left shifts, `2*esize - shift` for the
/// right shifts).
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
    /// `SQSHL` -- signed saturating shift left (FEAT_SVE2).
    Sqshl,
    /// `UQSHL` -- unsigned saturating shift left (FEAT_SVE2).
    Uqshl,
    /// `SRSHR` -- signed rounding shift right (FEAT_SVE2).
    Srshr,
    /// `URSHR` -- unsigned rounding shift right (FEAT_SVE2).
    Urshr,
    /// `SQSHLU` -- signed saturating shift left, unsigned result (FEAT_SVE2).
    Sqshlu,
}

impl Arm64SveShiftImmOp {
    /// The 4-bit `opc` field (`[19:16]`): ASR 0000 / LSR 0001 / LSL 0011 / ASRD 0100, and the SVE2 rows
    /// SQSHL 0110 / UQSHL 0111 / SRSHR 1100 / URSHR 1101 / SQSHLU 1111.
    pub fn opc(self) -> u32 {
        match self {
            Self::Asr => 0b0000,
            Self::Lsr => 0b0001,
            Self::Lsl => 0b0011,
            Self::Asrd => 0b0100,
            Self::Sqshl => 0b0110,
            Self::Uqshl => 0b0111,
            Self::Srshr => 0b1100,
            Self::Urshr => 0b1101,
            Self::Sqshlu => 0b1111,
        }
    }

    /// Whether this is a left shift (`esize + shift` packing) vs a right shift (`2*esize - shift`).
    pub fn is_left(self) -> bool {
        matches!(self, Self::Lsl | Self::Sqshl | Self::Uqshl | Self::Sqshlu)
    }

    /// Whether this row gates on FEAT_SVE2 rather than plain SVE.
    pub fn is_sve2(self) -> bool {
        matches!(
            self,
            Self::Sqshl | Self::Uqshl | Self::Srshr | Self::Urshr | Self::Sqshlu
        )
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Asr => "asr",
            Self::Lsr => "lsr",
            Self::Lsl => "lsl",
            Self::Asrd => "asrd",
            Self::Sqshl => "sqshl",
            Self::Uqshl => "uqshl",
            Self::Srshr => "srshr",
            Self::Urshr => "urshr",
            Self::Sqshlu => "sqshlu",
        }
    }

    /// Recover the op from its 4-bit `opc` field, if a modeled op (the remaining values are unallocated).
    pub fn from_opc(opc: u32) -> Option<Self> {
        match opc & 0b1111 {
            0b0000 => Some(Self::Asr),
            0b0001 => Some(Self::Lsr),
            0b0011 => Some(Self::Lsl),
            0b0100 => Some(Self::Asrd),
            0b0110 => Some(Self::Sqshl),
            0b0111 => Some(Self::Uqshl),
            0b1100 => Some(Self::Srshr),
            0b1101 => Some(Self::Urshr),
            0b1111 => Some(Self::Sqshlu),
            _ => None,
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 9] = [
        Self::Asr,
        Self::Lsr,
        Self::Lsl,
        Self::Asrd,
        Self::Sqshl,
        Self::Uqshl,
        Self::Srshr,
        Self::Urshr,
        Self::Sqshlu,
    ];
}
