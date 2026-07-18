// Copyright (c) Scaleservers LLC

use alloc::borrow::Cow;

/// A system register accessible via `MRS`/`MSR` (DDI0487 C5). The condition flags (`NZCV`), the floating-point
/// control/status registers (`FPCR`/`FPSR`), and the user read/write thread pointer (`TPIDR_EL0`) are modeled by
/// name; every OTHER register is carried by its raw 15-bit specifier via [`Self::Raw`], so a code-generator can
/// read or write registers this cut does not name individually (the cycle counter `PMCCNTR_EL0`, the virtual
/// counter `CNTVCT_EL0`, ...). Each maps to a fixed 15-bit `o0:op1:CRn:CRm:op2` specifier placed at bits `[19:5]`
/// of the `MRS`/`MSR` encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SystemRegister {
    /// `NZCV` -- the condition flags (negative/zero/carry/overflow).
    Nzcv,
    /// `FPCR` -- the floating-point control register (rounding mode, exception masks).
    Fpcr,
    /// `FPSR` -- the floating-point status register (cumulative exception flags).
    Fpsr,
    /// `TPIDR_EL0` -- the EL0 read/write software thread-ID register (thread-local base).
    TpidrEl0,
    /// Any other system register, by its raw 15-bit `o0:op1:CRn:CRm:op2` specifier (the `[19:5]` field). This is
    /// the escape hatch a backend uses to reach a register this cut does not name -- e.g. `MRS x0, PMCCNTR_EL0`
    /// (the cycle counter) is `Raw(0x5CE8)`. It emits/parses as the generic `s<op0>_<op1>_c<Cn>_c<Cm>_<op2>` form
    /// that GNU `as` and LLVM accept. The value is masked to 15 bits.
    Raw(u16),
}

impl Arm64SystemRegister {
    /// The 15-bit `o0:op1:CRn:CRm:op2` specifier (placed at bits `[19:5]` of the `MRS`/`MSR` word).
    pub fn field_bits(self) -> u32 {
        match self {
            Self::Nzcv => 0x5A10,
            Self::Fpcr => 0x5A20,
            Self::Fpsr => 0x5A21,
            Self::TpidrEl0 => 0x5E82,
            Self::Raw(bits) => u32::from(bits & 0x7FFF),
        }
    }

    /// Recover a register from its 15-bit specifier: a modeled register by name, otherwise [`Self::Raw`]. Total,
    /// so every `MRS`/`MSR` (register) encoding round-trips (the raw form is never rejected).
    pub fn from_field_bits(bits: u32) -> Self {
        match bits & 0x7FFF {
            0x5A10 => Self::Nzcv,
            0x5A20 => Self::Fpcr,
            0x5A21 => Self::Fpsr,
            0x5E82 => Self::TpidrEl0,
            other => Self::Raw(other as u16),
        }
    }

    /// The lowercase UAL register operand: the mnemonic for a modeled register, else the generic
    /// `s<op0>_<op1>_c<Cn>_c<Cm>_<op2>` form GNU and LLVM accept for an arbitrary system register (`op0 = 2 + o0`).
    pub fn name(self) -> Cow<'static, str> {
        match self {
            Self::Nzcv => Cow::Borrowed("nzcv"),
            Self::Fpcr => Cow::Borrowed("fpcr"),
            Self::Fpsr => Cow::Borrowed("fpsr"),
            Self::TpidrEl0 => Cow::Borrowed("tpidr_el0"),
            Self::Raw(bits) => {
                let bits = u32::from(bits & 0x7FFF);
                Cow::Owned(format!(
                    "s{}_{}_c{}_c{}_{}",
                    2 + (bits >> 14),
                    (bits >> 11) & 0b111,
                    (bits >> 7) & 0xF,
                    (bits >> 3) & 0xF,
                    bits & 0b111
                ))
            }
        }
    }
}
