// Copyright (c) Scaleservers LLC

/// The element width of a `VMOV` core <-> scalar-lane transfer (`VMOV.<size> Dd[x], Rt` and the reverse): the
/// lane is a `.8`/`.16`/`.32` slice of a doubleword register. The width and the lane index are packed together
/// into the encoding's `opc1` (`[22:21]`) and `opc2` (`[6:5]`) fields -- see the ARM ARM "VMOV (general-purpose
/// register to scalar)" / "VMOV (scalar to general-purpose register)" descriptions (DDI0487 / DDI0553).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm32VmovLaneSize {
    /// `.8` -- a byte lane (index `0..=7`).
    Byte,
    /// `.16` -- a halfword lane (index `0..=3`).
    Half,
    /// `.32` -- a word lane (index `0..=1`).
    Word,
}

impl Arm32VmovLaneSize {
    /// The number of lanes a doubleword holds at this width (`8` / `4` / `2`).
    pub fn lane_count(self) -> u8 {
        match self {
            Self::Byte => 8,
            Self::Half => 4,
            Self::Word => 2,
        }
    }

    /// The size suffix as written in UAL (`8` / `16` / `32`).
    pub fn suffix(self) -> &'static str {
        match self {
            Self::Byte => "8",
            Self::Half => "16",
            Self::Word => "32",
        }
    }

    /// Pack `(self, index)` into the `(opc1 [22:21], opc2 [6:5])` encoding fields. The caller is responsible for
    /// `index < self.lane_count()` (the encoder validates it).
    pub fn opc_fields(self, index: u8) -> (u32, u32) {
        let i = index as u32;
        match self {
            Self::Byte => (0b10 | ((i >> 2) & 1), i & 0b11), // [22]=1, [21]=idx<2>, [6:5]=idx<1:0>
            Self::Half => ((i >> 1) & 1, ((i & 1) << 1) | 1), // [22]=0, [21]=idx<1>, [6]=idx<0>, [5]=1
            Self::Word => (i & 1, 0),                         // [22]=0, [21]=idx<0>, [6:5]=00
        }
    }

    /// Recover `(width, lane index)` from the `opc1`/`opc2` fields (always a valid pattern -- every `[22],[5]`
    /// combination is allocated).
    pub fn from_opc_fields(opc1: u32, opc2: u32) -> (Self, u8) {
        if opc1 & 0b10 != 0 {
            (Self::Byte, (((opc1 & 1) << 2) | (opc2 & 0b11)) as u8) // [22]=1 -> .8
        } else if opc2 & 1 != 0 {
            (Self::Half, (((opc1 & 1) << 1) | ((opc2 >> 1) & 1)) as u8) // [22]=0,[5]=1 -> .16
        } else {
            (Self::Word, (opc1 & 1) as u8) // [22]=0,[5]=0 -> .32
        }
    }
}
