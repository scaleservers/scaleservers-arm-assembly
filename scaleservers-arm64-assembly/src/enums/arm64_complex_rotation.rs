// Copyright (c) Scaleservers LLC

/// The rotation operand of an AArch64 complex-number SIMD instruction (`FCMLA`/`FCADD`, DDI0487 C7) -- one of the
/// four right-angle rotations applied to the multiplicand's imaginary parts. `FCMLA` accepts all four; `FCADD`
/// accepts only `#90`/`#270`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ComplexRotation {
    /// `#0`.
    R0,
    /// `#90`.
    R90,
    /// `#180`.
    R180,
    /// `#270`.
    R270,
}

impl Arm64ComplexRotation {
    /// The rotation in degrees (`0`/`90`/`180`/`270`), as it prints in `, #<deg>`.
    pub fn degrees(self) -> u16 {
        match self {
            Self::R0 => 0,
            Self::R90 => 90,
            Self::R180 => 180,
            Self::R270 => 270,
        }
    }

    /// The 2-bit rotation code (`FCMLA` encodes this at `[12:11]`): `#0`=0, `#90`=1, `#180`=2, `#270`=3.
    pub fn code(self) -> u32 {
        match self {
            Self::R0 => 0,
            Self::R90 => 1,
            Self::R180 => 2,
            Self::R270 => 3,
        }
    }

    /// Recover a rotation from its 2-bit code.
    pub fn from_code(code: u32) -> Self {
        match code & 0b11 {
            0 => Self::R0,
            1 => Self::R90,
            2 => Self::R180,
            _ => Self::R270,
        }
    }
}
