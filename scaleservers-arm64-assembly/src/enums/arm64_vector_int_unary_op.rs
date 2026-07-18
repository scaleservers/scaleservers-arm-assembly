// Copyright (c) Scaleservers LLC

use super::Arm64VectorArrangement;

/// The operation of an AArch64 Advanced SIMD (NEON) **integer** "two-register miscellaneous" unary op -- the
/// encoding class `0 Q U 01110 size 10000 opcode 10 Rn Rd` (DDI0487 C7). The op is an orthogonal field over
/// the shared `{ arrangement, Vd, Vn }` shape (one source register); the arrangement supplies the element
/// `size`, and the set of valid arrangements is per-op ([`Self::allows_arrangement`]) -- the byte/halfword
/// reversals and `CNT` are size-restricted, `CLS`/`CLZ` exclude doubleword, and `ABS`/`NEG`/`SQABS`/`SQNEG`
/// accept every arrangement except the single-lane `.1d` scalar form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorIntUnaryOp {
    /// `ABS` -- per-lane absolute value (`U = 0`, opcode `01011`).
    Abs,
    /// `NEG` -- per-lane negate (`U = 1`, opcode `01011`).
    Neg,
    /// `REV64` -- reverse the elements within each 64-bit doubleword (`U = 0`, opcode `00000`; byte/half/word lanes).
    Rev64,
    /// `REV32` -- reverse the elements within each 32-bit word (`U = 1`, opcode `00000`; byte/half lanes).
    Rev32,
    /// `REV16` -- reverse the elements within each 16-bit halfword (`U = 0`, opcode `00001`; byte lanes only).
    Rev16,
    /// `CLS` -- count leading sign bits per lane (`U = 0`, opcode `00100`; no doubleword).
    Cls,
    /// `CLZ` -- count leading zero bits per lane (`U = 1`, opcode `00100`; no doubleword).
    Clz,
    /// `CNT` -- count set bits in each byte (`U = 0`, opcode `00101`; byte lanes only).
    Cnt,
    /// `SQABS` -- saturating absolute value per lane (`U = 0`, opcode `00111`).
    Sqabs,
    /// `SQNEG` -- saturating negate per lane (`U = 1`, opcode `00111`).
    Sqneg,
    /// `SUQADD` -- signed saturating accumulate of an unsigned value, per lane (`U = 0`, opcode `00011`).
    Suqadd,
    /// `USQADD` -- unsigned saturating accumulate of a signed value, per lane (`U = 1`, opcode `00011`).
    Usqadd,
}

impl Arm64VectorIntUnaryOp {
    /// The `size = 0`, `Q = 0` base word (`U` + the 5-bit opcode baked in); the arrangement supplies `Q<<30`
    /// and `size<<22`, and the registers `Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Abs => 0x0E20_B800,
            Self::Neg => 0x2E20_B800,
            Self::Rev64 => 0x0E20_0800,
            Self::Rev32 => 0x2E20_0800,
            Self::Rev16 => 0x0E20_1800,
            Self::Cls => 0x0E20_4800,
            Self::Clz => 0x2E20_4800,
            Self::Cnt => 0x0E20_5800,
            Self::Sqabs => 0x0E20_7800,
            Self::Sqneg => 0x2E20_7800,
            Self::Suqadd => 0x0E20_3800,
            Self::Usqadd => 0x2E20_3800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Abs => "abs",
            Self::Neg => "neg",
            Self::Rev64 => "rev64",
            Self::Rev32 => "rev32",
            Self::Rev16 => "rev16",
            Self::Cls => "cls",
            Self::Clz => "clz",
            Self::Cnt => "cnt",
            Self::Sqabs => "sqabs",
            Self::Sqneg => "sqneg",
            Self::Suqadd => "suqadd",
            Self::Usqadd => "usqadd",
        }
    }

    /// Whether `arr` is a valid arrangement for this op. The two-register-misc unary ops restrict the element
    /// size per-op: the reversals shrink with the reversed container (`REV16` byte-only, `REV32` byte/half,
    /// `REV64` byte/half/word), `CLS`/`CLZ` exclude doubleword, `CNT` is byte-only, and the arithmetic ops
    /// (`ABS`/`NEG`/`SQABS`/`SQNEG`) accept every arrangement except the single-lane `.1d` scalar form.
    pub fn allows_arrangement(self, arr: Arm64VectorArrangement) -> bool {
        use Arm64VectorArrangement::{B8, B16, H4, H8, S2, S4};
        match self {
            Self::Abs | Self::Neg | Self::Sqabs | Self::Sqneg | Self::Suqadd | Self::Usqadd => {
                arr != Arm64VectorArrangement::D1
            }
            Self::Rev64 | Self::Cls | Self::Clz => matches!(arr, B8 | B16 | H4 | H8 | S2 | S4),
            Self::Rev32 => matches!(arr, B8 | B16 | H4 | H8),
            Self::Rev16 | Self::Cnt => matches!(arr, B8 | B16),
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 12] = [
        Self::Abs,
        Self::Neg,
        Self::Rev64,
        Self::Rev32,
        Self::Rev16,
        Self::Cls,
        Self::Clz,
        Self::Cnt,
        Self::Sqabs,
        Self::Sqneg,
        Self::Suqadd,
        Self::Usqadd,
    ];
}
