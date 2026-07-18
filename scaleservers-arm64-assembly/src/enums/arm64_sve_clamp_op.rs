// Copyright (c) Scaleservers LLC

/// The SVE2.1 clamp operations `<op> Zd.<T>, Zn.<T>, Zm.<T>`, which clamp each `Zd` element to the inclusive range
/// `[Zn, Zm]` (FEAT_SVE2p1, also present under FEAT_SME2): the signed/unsigned integer `SCLAMP`/`UCLAMP` and the
/// floating-point `FCLAMP`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64SveClampOp {
    /// `SCLAMP` -- signed integer clamp (`.b`/`.h`/`.s`/`.d`).
    Sclamp,
    /// `UCLAMP` -- unsigned integer clamp (`.b`/`.h`/`.s`/`.d`).
    Uclamp,
    /// `FCLAMP` -- floating-point clamp (`.h`/`.s`/`.d`).
    Fclamp,
}

impl Arm64SveClampOp {
    /// The 32-bit encoding base (opcode and the fixed `[21]` bit in place; size/`Zm`/`Zn`/`Zd` zero). GNU+LLVM verified.
    pub const fn base(self) -> u32 {
        match self {
            Self::Sclamp => 0x4400_C000,
            Self::Uclamp => 0x4400_C400,
            Self::Fclamp => 0x6420_2400,
        }
    }

    /// The UAL mnemonic.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Sclamp => "sclamp",
            Self::Uclamp => "uclamp",
            Self::Fclamp => "fclamp",
        }
    }

    /// Whether this is the floating-point clamp (which excludes the `.b` element).
    pub const fn is_fp(self) -> bool {
        matches!(self, Self::Fclamp)
    }

    /// Every op, for round-trip testing.
    pub const ALL: [Self; 3] = [Self::Sclamp, Self::Uclamp, Self::Fclamp];
}
