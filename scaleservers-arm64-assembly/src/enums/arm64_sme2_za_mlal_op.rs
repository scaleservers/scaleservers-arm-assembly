// Copyright (c) Scaleservers LLC

/// An SME2 **integer multiply-long accumulate into ZA** operation (FEAT_SME2; DDI0487 part C). Selects the signedness
/// of each source and add-vs-subtract; the op occupies `[4:2]` of every MLAL/MLALL-into-ZA form. The `us`/`su` mixed
/// forms exist only for the 4-way `.b`->`.s` widening (and `su` only in the indexed form). GNU+LLVM verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sme2ZaMlalOp {
    /// `SMLAL`/`SMLALL` -- signed multiply-add.
    Smlal,
    /// `UMLAL`/`UMLALL` -- unsigned multiply-add.
    Umlal,
    /// `SMLSL`/`SMLSLL` -- signed multiply-subtract.
    Smlsl,
    /// `UMLSL`/`UMLSLL` -- unsigned multiply-subtract.
    Umlsl,
    /// `USMLALL` -- unsigned-by-signed multiply-add (4-way `.b`->`.s` only).
    Usmlall,
    /// `SUMLALL` -- signed-by-unsigned multiply-add (4-way `.b`->`.s`, indexed only).
    Sumlall,
}

impl Arm64Sme2ZaMlalOp {
    /// The `[4:2]` op-bit contribution: U`[4]`, subtract`[3]`, mixed`[2]`.
    pub fn op_bits(self) -> u32 {
        match self {
            Self::Smlal => 0b000 << 2,
            Self::Umlal => 0b100 << 2,
            Self::Smlsl => 0b010 << 2,
            Self::Umlsl => 0b110 << 2,
            Self::Usmlall => 0b001 << 2,
            Self::Sumlall => 0b101 << 2,
        }
    }

    /// Whether this is a mixed-signedness op (`USMLALL`/`SUMLALL`, valid only for the 4-way `.b`->`.s` widening).
    pub fn is_mixed(self) -> bool {
        matches!(self, Self::Usmlall | Self::Sumlall)
    }

    /// The lowercase UAL mnemonic; `two_way` picks the single-`L` (`smlal`) vs double-`L` (`smlall`) suffix. The mixed
    /// `USMLALL`/`SUMLALL` ops exist only in the 4-way form, so they always render with the double `L`.
    pub fn mnemonic(self, two_way: bool) -> &'static str {
        match (self, two_way) {
            (Self::Smlal, true) => "smlal",
            (Self::Smlal, false) => "smlall",
            (Self::Umlal, true) => "umlal",
            (Self::Umlal, false) => "umlall",
            (Self::Smlsl, true) => "smlsl",
            (Self::Smlsl, false) => "smlsll",
            (Self::Umlsl, true) => "umlsl",
            (Self::Umlsl, false) => "umlsll",
            (Self::Usmlall, _) => "usmlall",
            (Self::Sumlall, _) => "sumlall",
        }
    }

    /// Recover the op from the `[4:2]` bits; `None` for the two unallocated values (`011`, `111`).
    pub fn from_op_bits(bits: u32) -> Option<Self> {
        Some(match bits & 0b111 {
            0b000 => Self::Smlal,
            0b100 => Self::Umlal,
            0b010 => Self::Smlsl,
            0b110 => Self::Umlsl,
            0b001 => Self::Usmlall,
            0b101 => Self::Sumlall,
            _ => return None,
        })
    }

    /// Every op, for tests.
    pub const ALL: [Self; 6] = [
        Self::Smlal,
        Self::Umlal,
        Self::Smlsl,
        Self::Umlsl,
        Self::Usmlall,
        Self::Sumlall,
    ];
}

/// The widening of an SME2 integer MLAL/MLALL-into-ZA instruction: which source element widens into which `ZA`
/// accumulator. `HtoS` is the 2-way `.h`->`.s` (`MLAL`, a ZA slice **pair**); `BtoS`/`HtoD` are the 4-way `MLALL`
/// (a ZA slice **quad**), the latter needing FEAT_SME_I16I64.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sme2ZaMlalWiden {
    /// `.h`->`.s`, 2-way (`MLAL`).
    HtoS,
    /// `.b`->`.s`, 4-way (`MLALL`).
    BtoS,
    /// `.h`->`.d`, 4-way (`MLALL`, FEAT_SME_I16I64).
    HtoD,
}

impl Arm64Sme2ZaMlalWiden {
    /// Whether the source element is `.h` (sets `[22]` in the non-indexed forms / `[23]` in the indexed form).
    pub fn h_source(self) -> bool {
        matches!(self, Self::HtoS | Self::HtoD)
    }

    /// Whether this is the 2-way (`.h`->`.s`) widening (a ZA slice pair, step 2).
    pub fn two_way(self) -> bool {
        matches!(self, Self::HtoS)
    }

    /// The ZA-slice stride: `2` for the 2-way pair, `4` for the 4-way quad.
    pub fn step(self) -> u8 {
        if self.two_way() { 2 } else { 4 }
    }

    /// The source element name.
    pub fn src_elem(self) -> &'static str {
        if self.h_source() { "h" } else { "b" }
    }

    /// The ZA accumulator element name.
    pub fn za_elem(self) -> &'static str {
        if matches!(self, Self::HtoD) { "d" } else { "s" }
    }

    /// Whether this widening needs FEAT_SME_I16I64 (the `.h`->`.d` form).
    pub fn needs_i16i64(self) -> bool {
        matches!(self, Self::HtoD)
    }

    /// The maximum `Zm` element index for the indexed form (`.b` -> `0..=15`, `.h` -> `0..=7`).
    pub fn max_index(self) -> u8 {
        if self.h_source() { 7 } else { 15 }
    }
}
