// Copyright (c) Scaleservers LLC

/// The family of an SVE **predicated floating-point convert** (DDI0487 part C): change of FP precision, or
/// conversion between a floating-point and an integer element. The exact source/destination element sizes are
/// carried alongside this kind (the encoding's size-pair field is irregular, so the model stores both sizes and a
/// lookup table maps `(kind, dest, src)` to the 8-bit opcode discriminant).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64SveFpConvertKind {
    /// `FCVT` -- convert floating-point precision (`.h`<->`.s`<->`.d`).
    Fcvt,
    /// `FCVTZS` -- convert floating-point to signed integer, rounding toward zero.
    Fcvtzs,
    /// `FCVTZU` -- convert floating-point to unsigned integer, rounding toward zero.
    Fcvtzu,
    /// `SCVTF` -- convert signed integer to floating-point.
    Scvtf,
    /// `UCVTF` -- convert unsigned integer to floating-point.
    Ucvtf,
}

impl Arm64SveFpConvertKind {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Fcvt => "fcvt",
            Self::Fcvtzs => "fcvtzs",
            Self::Fcvtzu => "fcvtzu",
            Self::Scvtf => "scvtf",
            Self::Ucvtf => "ucvtf",
        }
    }

    /// Whether the destination element is the integer one (`FCVTZS`/`FCVTZU`); for `SCVTF`/`UCVTF` the integer is
    /// the source, and for `FCVT` both are floating-point. (Used only to document operand roles; display uses the
    /// element sizes carried by the instruction directly.)
    pub fn dest_is_integer(self) -> bool {
        matches!(self, Self::Fcvtzs | Self::Fcvtzu)
    }
}
