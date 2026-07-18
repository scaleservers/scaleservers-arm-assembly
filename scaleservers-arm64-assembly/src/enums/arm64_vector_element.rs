// Copyright (c) Scaleservers LLC

/// A single AArch64 Advanced SIMD (NEON) **vector lane element size** -- `.b` (8-bit) / `.h` (16-bit) /
/// `.s` (32-bit) / `.d` (64-bit). Used by the lane-move ("AdvSIMD copy") instructions, where one element at a
/// lane index is moved to or from a general-purpose register -- as opposed to a whole-vector
/// [`super::Arm64VectorArrangement`]. The element size and lane index pack together into the `imm5` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorElement {
    /// `.b` -- an 8-bit byte lane (16 lanes; index `0..15`).
    B,
    /// `.h` -- a 16-bit halfword lane (8 lanes; index `0..7`).
    H,
    /// `.s` -- a 32-bit word lane (4 lanes; index `0..3`).
    S,
    /// `.d` -- a 64-bit doubleword lane (2 lanes; index `0..1`).
    D,
}

impl Arm64VectorElement {
    /// The 2-bit element-size value (`.b` 0 / `.h` 1 / `.s` 2 / `.d` 3) -- the position of the low set bit of `imm5`.
    pub fn size_bits(self) -> u32 {
        match self {
            Self::B => 0,
            Self::H => 1,
            Self::S => 2,
            Self::D => 3,
        }
    }

    /// The number of lanes of this element size in a 128-bit register (16 / 8 / 4 / 2). A lane index must be
    /// strictly less than this.
    pub fn lane_count(self) -> u8 {
        16 >> self.size_bits()
    }

    /// The lowercase lane suffix (`b`/`h`/`s`/`d`), rendered as `vN.<suffix>[index]`.
    pub fn name(self) -> &'static str {
        match self {
            Self::B => "b",
            Self::H => "h",
            Self::S => "s",
            Self::D => "d",
        }
    }

    /// The element size from a plain 2-bit size field (`.b` 0 / `.h` 1 / `.s` 2 / `.d` 3) -- used by the scalar
    /// SIMD forms, where the operand register is named by element (`b`/`h`/`s`/`d`).
    pub fn from_size_bits(bits: u32) -> Self {
        match bits & 0b11 {
            0 => Self::B,
            1 => Self::H,
            2 => Self::S,
            _ => Self::D,
        }
    }

    /// Recover the element size from the position of the lowest set bit of `imm5` (`None` when `imm5` is 0,
    /// which is not a lane specifier).
    pub fn from_imm5(imm5: u32) -> Option<Self> {
        if imm5 & 0b00001 != 0 {
            Some(Self::B)
        } else if imm5 & 0b00010 != 0 {
            Some(Self::H)
        } else if imm5 & 0b00100 != 0 {
            Some(Self::S)
        } else if imm5 & 0b01000 != 0 {
            Some(Self::D)
        } else {
            None
        }
    }
}
