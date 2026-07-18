// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **shift by immediate** instruction (DDI0487 C7) -- the
/// encoding class `0 Q U 011110 immh immb opcode 1 Rn Rd`, where the 7-bit `immh:immb` field folds together
/// the element size *and* the shift amount. The op is an orthogonal field over the shared
/// `{ arrangement, Vd, Vn, shift }` shape; the encoder derives `immh:immb` from the arrangement's element size
/// and the shift (see [`Self::is_left`]). These are the *same-size* shifts (`Vd` and `Vn` share the
/// arrangement); the narrowing/widening shifts live in a separate form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorShiftImmOp {
    /// `SHL` -- shift left by immediate (`U = 0`, opcode `01010`); shift amount `0..element_bits-1`.
    Shl,
    /// `SSHR` -- signed shift right by immediate (`U = 0`, opcode `00000`); shift amount `1..element_bits`.
    Sshr,
    /// `USHR` -- unsigned shift right by immediate (`U = 1`, opcode `00000`); shift amount `1..element_bits`.
    Ushr,
    /// `SRSHR` -- signed rounding shift right by immediate (`U = 0`, opcode `00100`); shift `1..element_bits`.
    Srshr,
    /// `URSHR` -- unsigned rounding shift right by immediate (`U = 1`, opcode `00100`); shift `1..element_bits`.
    Urshr,
    /// `SSRA` -- signed shift right and accumulate (`U = 0`, opcode `00010`); shift `1..element_bits`.
    Ssra,
    /// `USRA` -- unsigned shift right and accumulate (`U = 1`, opcode `00010`); shift `1..element_bits`.
    Usra,
    /// `SRSRA` -- signed rounding shift right and accumulate (`U = 0`, opcode `00110`); shift `1..element_bits`.
    Srsra,
    /// `URSRA` -- unsigned rounding shift right and accumulate (`U = 1`, opcode `00110`); shift `1..element_bits`.
    Ursra,
    /// `SRI` -- shift right and insert (`U = 1`, opcode `01000`); shift `1..element_bits`.
    Sri,
    /// `SLI` -- shift left and insert (`U = 1`, opcode `01010`); shift `0..element_bits-1`.
    Sli,
    /// `SQSHL` (immediate) -- signed saturating shift left by immediate (`U = 0`, opcode `01110`); shift
    /// `0..element_bits-1`. (Distinct from the same-named three-same register form.)
    Sqshl,
    /// `UQSHL` (immediate) -- unsigned saturating shift left by immediate (`U = 1`, opcode `01110`); shift
    /// `0..element_bits-1`.
    Uqshl,
    /// `SQSHLU` -- signed saturating shift left unsigned by immediate (`U = 1`, opcode `01100`); shift
    /// `0..element_bits-1`.
    Sqshlu,
}

impl Arm64VectorShiftImmOp {
    /// The base word with `Q = 0` and `immh:immb = 0` (`U` and opcode baked in); the encoder adds `Q<<30`, the
    /// computed `immh:immb << 16`, and `Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Shl => 0x0F00_5400,
            Self::Sshr => 0x0F00_0400,
            Self::Ushr => 0x2F00_0400,
            Self::Srshr => 0x0F00_2400,
            Self::Urshr => 0x2F00_2400,
            Self::Ssra => 0x0F00_1400,
            Self::Usra => 0x2F00_1400,
            Self::Srsra => 0x0F00_3400,
            Self::Ursra => 0x2F00_3400,
            Self::Sri => 0x2F00_4400,
            Self::Sli => 0x2F00_5400,
            Self::Sqshl => 0x0F00_7400,
            Self::Uqshl => 0x2F00_7400,
            Self::Sqshlu => 0x2F00_6400,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Shl => "shl",
            Self::Sshr => "sshr",
            Self::Ushr => "ushr",
            Self::Srshr => "srshr",
            Self::Urshr => "urshr",
            Self::Ssra => "ssra",
            Self::Usra => "usra",
            Self::Srsra => "srsra",
            Self::Ursra => "ursra",
            Self::Sri => "sri",
            Self::Sli => "sli",
            Self::Sqshl => "sqshl",
            Self::Uqshl => "uqshl",
            Self::Sqshlu => "sqshlu",
        }
    }

    /// Whether this is a left shift. Left shifts (`SHL`/`SLI`/`SQSHL`/`UQSHL`/`SQSHLU`) accept
    /// `0..element_bits-1` and encode `immh:immb = element_bits + shift`; the right shifts accept
    /// `1..element_bits` and encode `immh:immb = 2*element_bits - shift`.
    pub fn is_left(self) -> bool {
        matches!(
            self,
            Self::Shl | Self::Sli | Self::Sqshl | Self::Uqshl | Self::Sqshlu
        )
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 14] = [
        Self::Shl,
        Self::Sshr,
        Self::Ushr,
        Self::Srshr,
        Self::Urshr,
        Self::Ssra,
        Self::Usra,
        Self::Srsra,
        Self::Ursra,
        Self::Sri,
        Self::Sli,
        Self::Sqshl,
        Self::Uqshl,
        Self::Sqshlu,
    ];
}
