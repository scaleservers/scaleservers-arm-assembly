// Copyright (c) Scaleservers LLC

/// An SVE2.1 **quadword (128-bit-segment) permute** operation: `ZIPQ1`/`ZIPQ2` (interleave the low/high halves of each
/// 128-bit segment) and `UZPQ1`/`UZPQ2` (deinterleave the even/odd elements of each 128-bit segment). The op is the
/// `[11:10]` selector of the shared 3-same frame. GNU+LLVM verified.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SveQuadPermuteOp {
    /// `ZIPQ1` -- interleave the low halves of each 128-bit segment.
    Zipq1,
    /// `ZIPQ2` -- interleave the high halves of each 128-bit segment.
    Zipq2,
    /// `UZPQ1` -- concatenate the even elements of each 128-bit segment.
    Uzpq1,
    /// `UZPQ2` -- concatenate the odd elements of each 128-bit segment.
    Uzpq2,
}

impl Arm64SveQuadPermuteOp {
    /// The `[11:10]` op-selector bits OR-ed into the base.
    pub const fn discriminant(self) -> u32 {
        (match self {
            Self::Zipq1 => 0b00,
            Self::Zipq2 => 0b01,
            Self::Uzpq1 => 0b10,
            Self::Uzpq2 => 0b11,
        }) << 10
    }

    /// Recover the op from the `[11:10]` selector. Total -- all four are allocated.
    pub const fn from_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0b00 => Self::Zipq1,
            0b01 => Self::Zipq2,
            0b10 => Self::Uzpq1,
            _ => Self::Uzpq2,
        }
    }

    /// The UAL mnemonic.
    pub const fn mnemonic(self) -> &'static str {
        match self {
            Self::Zipq1 => "zipq1",
            Self::Zipq2 => "zipq2",
            Self::Uzpq1 => "uzpq1",
            Self::Uzpq2 => "uzpq2",
        }
    }

    /// Every op, for tests.
    pub const ALL: [Self; 4] = [Self::Zipq1, Self::Zipq2, Self::Uzpq1, Self::Uzpq2];
}
