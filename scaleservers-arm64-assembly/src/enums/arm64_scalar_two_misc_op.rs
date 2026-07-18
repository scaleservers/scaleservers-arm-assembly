// Copyright (c) Scaleservers LLC

/// A scalar Advanced SIMD **two-register-misc integer** op (DDI0487 C7, the `01 U 11110 size 10000 opcode 10 Rn
/// Rd` encoding) -- the same-size scalar `b`/`h`/`s`/`d` unary ops (saturating accumulate/abs/negate and the
/// compare-against-zero forms). The narrowing `SQXTN`/`SQXTUN`/`UQXTN` are modeled separately (different shape).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ScalarTwoMiscOp {
    /// `SUQADD` -- signed saturating accumulate of an unsigned value (all sizes).
    Suqadd,
    /// `USQADD` -- unsigned saturating accumulate of a signed value (all sizes).
    Usqadd,
    /// `SQABS` -- signed saturating absolute value (all sizes).
    Sqabs,
    /// `SQNEG` -- signed saturating negate (all sizes).
    Sqneg,
    /// `CMGT #0` -- compare signed greater-than zero (`.d` only).
    CmgtZero,
    /// `CMGE #0` -- compare signed greater-or-equal zero (`.d` only).
    CmgeZero,
    /// `CMEQ #0` -- compare equal to zero (`.d` only).
    CmeqZero,
    /// `CMLE #0` -- compare signed less-or-equal zero (`.d` only).
    CmleZero,
    /// `CMLT #0` -- compare signed less-than zero (`.d` only).
    CmltZero,
    /// `ABS` -- absolute value (`.d` only).
    Abs,
    /// `NEG` -- negate (`.d` only).
    Neg,
}

impl Arm64ScalarTwoMiscOp {
    /// The base word (`size`/`Rn`/`Rd` zero): `0x5E20_0800 | (U<<29) | (opcode<<12)`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        let (u, opcode): (u32, u32) = match self {
            Self::Suqadd => (0, 0b00011),
            Self::Usqadd => (1, 0b00011),
            Self::Sqabs => (0, 0b00111),
            Self::Sqneg => (1, 0b00111),
            Self::CmgtZero => (0, 0b01000),
            Self::CmgeZero => (1, 0b01000),
            Self::CmeqZero => (0, 0b01001),
            Self::CmleZero => (1, 0b01001),
            Self::CmltZero => (0, 0b01010),
            Self::Abs => (0, 0b01011),
            Self::Neg => (1, 0b01011),
        };
        0x5E20_0800 | (u << 29) | (opcode << 12)
    }

    /// The lowercase UAL mnemonic (the compare-against-zero ops print a trailing `, #0`, handled by the emitter).
    pub fn name(self) -> &'static str {
        match self {
            Self::Suqadd => "suqadd",
            Self::Usqadd => "usqadd",
            Self::Sqabs => "sqabs",
            Self::Sqneg => "sqneg",
            Self::CmgtZero => "cmgt",
            Self::CmgeZero => "cmge",
            Self::CmeqZero => "cmeq",
            Self::CmleZero => "cmle",
            Self::CmltZero => "cmlt",
            Self::Abs => "abs",
            Self::Neg => "neg",
        }
    }

    /// Whether this op prints a trailing `, #0` (the compare-against-zero forms).
    pub fn is_compare_zero(self) -> bool {
        matches!(
            self,
            Self::CmgtZero | Self::CmgeZero | Self::CmeqZero | Self::CmleZero | Self::CmltZero
        )
    }

    /// Whether this op allocates the given 2-bit element size: `SUQADD`/`USQADD`/`SQABS`/`SQNEG` allow every size;
    /// the compares and `ABS`/`NEG` are `.d`-only.
    pub fn allows_size(self, size: u32) -> bool {
        match self {
            Self::Suqadd | Self::Usqadd | Self::Sqabs | Self::Sqneg => size <= 0b11,
            _ => size == 0b11,
        }
    }

    /// Recover the op from a masked base (`word & 0xFF3F_FC00`); `None` if it is not one of these.
    pub fn from_base(base: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|op| op.base() == base)
    }

    /// Every op, for tests.
    pub const ALL: [Self; 11] = [
        Self::Suqadd,
        Self::Usqadd,
        Self::Sqabs,
        Self::Sqneg,
        Self::CmgtZero,
        Self::CmgeZero,
        Self::CmeqZero,
        Self::CmleZero,
        Self::CmltZero,
        Self::Abs,
        Self::Neg,
    ];
}
