// Copyright (c) Scaleservers LLC

/// The offset-register extend/shift mode for an SVE scalar-plus-vector gather/scatter address `[Xn, Zm.<T>, <mode>]`:
/// the 32-bit unsigned/signed extends `UXTW`/`SXTW` (valid for a `.s` or `.d` offset vector) and the 64-bit logical
/// shift `LSL` (valid only for a `.d` offset vector). The mode is encoded as `[14]` = signed (`SXTW`), `[13]` = 64-bit
/// `LSL`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SveOffsetMode {
    /// `UXTW` -- zero-extend the 32-bit offset.
    Uxtw,
    /// `SXTW` -- sign-extend the 32-bit offset.
    Sxtw,
    /// `LSL` -- use the full 64-bit offset (only with a `.d` offset vector).
    Lsl,
}

impl Arm64SveOffsetMode {
    /// The UAL keyword.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Uxtw => "uxtw",
            Self::Sxtw => "sxtw",
            Self::Lsl => "lsl",
        }
    }

    /// The `[14]` signed bit (set for `SXTW`). Used by the scatter scalar+vector form, where the mode is `[14]`
    /// signed, `[13]` 64-bit-LSL (so `LSL` = `0,1`).
    pub const fn signed_bit(self) -> u32 {
        matches!(self, Self::Sxtw) as u32
    }

    /// The `xs` bit (set for `SXTW` or `LSL`, i.e. any non-`UXTW`). Used by the gather forms, where the mode is the
    /// `xs` bit (`[22]`) plus an `lsl` bit (so `LSL` = `1,1`).
    pub const fn xs_bit(self) -> u32 {
        !matches!(self, Self::Uxtw) as u32
    }

    /// Recover the gather mode from its `xs` and `lsl` bits, or `None` for the unallocated `0,1` pair.
    pub const fn from_xs_lsl(xs: u32, lsl: u32) -> Option<Self> {
        match (xs & 1, lsl & 1) {
            (0, 0) => Some(Self::Uxtw),
            (1, 0) => Some(Self::Sxtw),
            (1, 1) => Some(Self::Lsl),
            _ => None,
        }
    }

    /// The `[13]` 64-bit-`LSL` bit.
    pub const fn lsl_bit(self) -> u32 {
        matches!(self, Self::Lsl) as u32
    }

    /// Recover the mode from the `[14]` signed and `[13]` lsl bits, or `None` for the unallocated `11` pair.
    pub const fn from_bits(signed: u32, lsl: u32) -> Option<Self> {
        match (signed & 1, lsl & 1) {
            (0, 0) => Some(Self::Uxtw),
            (1, 0) => Some(Self::Sxtw),
            (0, 1) => Some(Self::Lsl),
            _ => None,
        }
    }
}
