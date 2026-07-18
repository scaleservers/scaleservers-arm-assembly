// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **shift-by-immediate** op (DDI0487 C7, the `01 U 111110 immh immb opcode 1 Rn Rd`
/// encoding) -- the single-register `b`/`h`/`s`/`d` shifts. The shift amount folds with the element size into
/// `immh:immb`. The non-saturating shifts are `.d`-only; the saturating shifts (`SQSHL`/`UQSHL`/`SQSHLU`) allow
/// every element size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarShiftImmOp {
    /// `SSHR` -- signed shift right (`.d` only).
    Sshr,
    /// `USHR` -- unsigned shift right (`.d` only).
    Ushr,
    /// `SSRA` -- signed shift right and accumulate (`.d` only).
    Ssra,
    /// `USRA` -- unsigned shift right and accumulate (`.d` only).
    Usra,
    /// `SRSHR` -- signed rounding shift right (`.d` only).
    Srshr,
    /// `URSHR` -- unsigned rounding shift right (`.d` only).
    Urshr,
    /// `SRSRA` -- signed rounding shift right and accumulate (`.d` only).
    Srsra,
    /// `URSRA` -- unsigned rounding shift right and accumulate (`.d` only).
    Ursra,
    /// `SRI` -- shift right and insert (`.d` only).
    Sri,
    /// `SHL` -- shift left (`.d` only).
    Shl,
    /// `SLI` -- shift left and insert (`.d` only).
    Sli,
    /// `SQSHLU` -- signed saturating shift left unsigned (all sizes).
    Sqshlu,
    /// `SQSHL` -- signed saturating shift left (all sizes).
    Sqshl,
    /// `UQSHL` -- unsigned saturating shift left (all sizes).
    Uqshl,
}

impl Arm64ScalarShiftImmOp {
    /// The base word (`immh:immb`/`Rn`/`Rd` zero): `0x5F00_0400 | (U<<29) | (opcode<<11)`. GNU+LLVM dual-oracle
    /// verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Sshr => (0, 0b00000),
            Self::Ushr => (1, 0b00000),
            Self::Ssra => (0, 0b00010),
            Self::Usra => (1, 0b00010),
            Self::Srshr => (0, 0b00100),
            Self::Urshr => (1, 0b00100),
            Self::Srsra => (0, 0b00110),
            Self::Ursra => (1, 0b00110),
            Self::Sri => (1, 0b01000),
            Self::Shl => (0, 0b01010),
            Self::Sli => (1, 0b01010),
            Self::Sqshlu => (1, 0b01100),
            Self::Sqshl => (0, 0b01110),
            Self::Uqshl => (1, 0b01110),
        };
        0x5F00_0400 | (u << 29) | (opcode << 11)
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sshr => "sshr",
            Self::Ushr => "ushr",
            Self::Ssra => "ssra",
            Self::Usra => "usra",
            Self::Srshr => "srshr",
            Self::Urshr => "urshr",
            Self::Srsra => "srsra",
            Self::Ursra => "ursra",
            Self::Sri => "sri",
            Self::Shl => "shl",
            Self::Sli => "sli",
            Self::Sqshlu => "sqshlu",
            Self::Sqshl => "sqshl",
            Self::Uqshl => "uqshl",
        }
    }

    /// Whether this is a left shift (`immh:immb = element_bits + shift`) rather than a right shift
    /// (`immh:immb = 2*element_bits - shift`).
    pub fn is_left(self) -> bool {
        matches!(
            self,
            Self::Shl | Self::Sli | Self::Sqshlu | Self::Sqshl | Self::Uqshl
        )
    }

    /// Whether this op allocates the given 2-bit element size: the saturating left shifts allow every size; the
    /// rest are `.d`-only.
    pub fn allows_size(self, size: u32) -> bool {
        match self {
            Self::Sqshlu | Self::Sqshl | Self::Uqshl => size <= 0b11,
            _ => size == 0b11,
        }
    }

    /// Recover the op from a masked base (`word & 0xFF80_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 14] = [
        Self::Sshr,
        Self::Ushr,
        Self::Ssra,
        Self::Usra,
        Self::Srshr,
        Self::Urshr,
        Self::Srsra,
        Self::Ursra,
        Self::Sri,
        Self::Shl,
        Self::Sli,
        Self::Sqshlu,
        Self::Sqshl,
        Self::Uqshl,
    ];
}
