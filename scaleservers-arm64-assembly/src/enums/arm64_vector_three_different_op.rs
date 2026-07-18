// Copyright (c) Scaleservers LLC

use super::Arm64VectorArrangement;

/// Whether an AArch64 "three different" op widens, takes one wide source, or narrows -- which fixes the
/// arrangements of its operands relative to the shared 128-bit "wide" arrangement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorThreeDifferentShape {
    /// `<op>(2)` long form -- `Vd.<wide>, Vn.<narrow>, Vm.<narrow>` (both sources narrow, result wide).
    Long,
    /// `<op>(2)` wide form -- `Vd.<wide>, Vn.<wide>, Vm.<narrow>` (first source already wide).
    Wide,
    /// `<op>(2)` narrow form -- `Vd.<narrow>, Vn.<wide>, Vm.<wide>` (sources wide, result narrow).
    Narrow,
}

/// The operation of an AArch64 Advanced SIMD (NEON) **three different** instruction (DDI0487 C7) -- the encoding
/// class `0 Q U 01110 size 1 Rm opcode 00 Rn Rd`, where the source and destination have *different* element
/// widths. The op is an orthogonal field over a shared `{ wide arrangement, high, Vd, Vn, Vm }` shape: the 2-bit
/// `size` field gives the source element size (so the 128-bit "wide" side is `.8h`/`.4s`/`.2d`), `Q` (`high`)
/// selects the lower (`Q=0`) or upper (`Q=1`, the `2`-suffix) half of the 128-bit narrow operand, and
/// [`Self::shape`] says which operands are wide vs narrow. (PMULL's 64->128 poly form is modeled separately as
/// the dedicated `VecPmull` variant -- it needs a `.1q` arrangement this lane enum cannot express.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorThreeDifferentOp {
    /// `SADDL` -- signed add long (`U = 0`, opcode `0000`).
    Saddl,
    /// `UADDL` -- unsigned add long (`U = 1`, opcode `0000`).
    Uaddl,
    /// `SADDW` -- signed add wide (`U = 0`, opcode `0001`).
    Saddw,
    /// `UADDW` -- unsigned add wide (`U = 1`, opcode `0001`).
    Uaddw,
    /// `SSUBL` -- signed subtract long (`U = 0`, opcode `0010`).
    Ssubl,
    /// `USUBL` -- unsigned subtract long (`U = 1`, opcode `0010`).
    Usubl,
    /// `SSUBW` -- signed subtract wide (`U = 0`, opcode `0011`).
    Ssubw,
    /// `USUBW` -- unsigned subtract wide (`U = 1`, opcode `0011`).
    Usubw,
    /// `ADDHN` -- add returning high narrow (`U = 0`, opcode `0100`).
    Addhn,
    /// `RADDHN` -- rounding add returning high narrow (`U = 1`, opcode `0100`).
    Raddhn,
    /// `SABAL` -- signed absolute-difference accumulate long (`U = 0`, opcode `0101`).
    Sabal,
    /// `UABAL` -- unsigned absolute-difference accumulate long (`U = 1`, opcode `0101`).
    Uabal,
    /// `SUBHN` -- subtract returning high narrow (`U = 0`, opcode `0110`).
    Subhn,
    /// `RSUBHN` -- rounding subtract returning high narrow (`U = 1`, opcode `0110`).
    Rsubhn,
    /// `SABDL` -- signed absolute-difference long (`U = 0`, opcode `0111`).
    Sabdl,
    /// `UABDL` -- unsigned absolute-difference long (`U = 1`, opcode `0111`).
    Uabdl,
    /// `SMLAL` -- signed multiply-add long (`U = 0`, opcode `1000`).
    Smlal,
    /// `UMLAL` -- unsigned multiply-add long (`U = 1`, opcode `1000`).
    Umlal,
    /// `SQDMLAL` -- signed saturating doubling multiply-add long (`U = 0`, opcode `1001`; half/word source only).
    Sqdmlal,
    /// `SMLSL` -- signed multiply-subtract long (`U = 0`, opcode `1010`).
    Smlsl,
    /// `UMLSL` -- unsigned multiply-subtract long (`U = 1`, opcode `1010`).
    Umlsl,
    /// `SQDMLSL` -- signed saturating doubling multiply-subtract long (`U = 0`, opcode `1011`; half/word source).
    Sqdmlsl,
    /// `SMULL` -- signed multiply long (`U = 0`, opcode `1100`).
    Smull,
    /// `UMULL` -- unsigned multiply long (`U = 1`, opcode `1100`).
    Umull,
    /// `SQDMULL` -- signed saturating doubling multiply long (`U = 0`, opcode `1101`; half/word source only).
    Sqdmull,
}

impl Arm64VectorThreeDifferentOp {
    /// The base word with `Q = 0` and `size = 0` (`U` + the 4-bit opcode baked in); the encoder adds `Q<<30`
    /// (high), the source `size<<22`, and `Vm<<16 | Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Saddl => 0x0E20_0000,
            Self::Uaddl => 0x2E20_0000,
            Self::Saddw => 0x0E20_1000,
            Self::Uaddw => 0x2E20_1000,
            Self::Ssubl => 0x0E20_2000,
            Self::Usubl => 0x2E20_2000,
            Self::Ssubw => 0x0E20_3000,
            Self::Usubw => 0x2E20_3000,
            Self::Addhn => 0x0E20_4000,
            Self::Raddhn => 0x2E20_4000,
            Self::Sabal => 0x0E20_5000,
            Self::Uabal => 0x2E20_5000,
            Self::Subhn => 0x0E20_6000,
            Self::Rsubhn => 0x2E20_6000,
            Self::Sabdl => 0x0E20_7000,
            Self::Uabdl => 0x2E20_7000,
            Self::Smlal => 0x0E20_8000,
            Self::Umlal => 0x2E20_8000,
            Self::Sqdmlal => 0x0E20_9000,
            Self::Smlsl => 0x0E20_A000,
            Self::Umlsl => 0x2E20_A000,
            Self::Sqdmlsl => 0x0E20_B000,
            Self::Smull => 0x0E20_C000,
            Self::Umull => 0x2E20_C000,
            Self::Sqdmull => 0x0E20_D000,
        }
    }

    /// The lowercase UAL mnemonic (without the `2` upper-half suffix, which the emitter appends from `high`).
    pub fn name(self) -> &'static str {
        match self {
            Self::Saddl => "saddl",
            Self::Uaddl => "uaddl",
            Self::Saddw => "saddw",
            Self::Uaddw => "uaddw",
            Self::Ssubl => "ssubl",
            Self::Usubl => "usubl",
            Self::Ssubw => "ssubw",
            Self::Usubw => "usubw",
            Self::Addhn => "addhn",
            Self::Raddhn => "raddhn",
            Self::Sabal => "sabal",
            Self::Uabal => "uabal",
            Self::Subhn => "subhn",
            Self::Rsubhn => "rsubhn",
            Self::Sabdl => "sabdl",
            Self::Uabdl => "uabdl",
            Self::Smlal => "smlal",
            Self::Umlal => "umlal",
            Self::Sqdmlal => "sqdmlal",
            Self::Smlsl => "smlsl",
            Self::Umlsl => "umlsl",
            Self::Sqdmlsl => "sqdmlsl",
            Self::Smull => "smull",
            Self::Umull => "umull",
            Self::Sqdmull => "sqdmull",
        }
    }

    /// Which operands are wide vs narrow (the long widening forms, the wide forms with one wide source, or the
    /// high-narrowing forms).
    pub fn shape(self) -> Arm64VectorThreeDifferentShape {
        use Arm64VectorThreeDifferentShape::{Long, Narrow, Wide};
        match self {
            Self::Saddw | Self::Uaddw | Self::Ssubw | Self::Usubw => Wide,
            Self::Addhn | Self::Raddhn | Self::Subhn | Self::Rsubhn => Narrow,
            _ => Long,
        }
    }

    /// Whether `wide` (the 128-bit side, one of `.8h`/`.4s`/`.2d`) is a valid arrangement for this op. The
    /// saturating-doubling ops (`SQDMULL`/`SQDMLAL`/`SQDMLSL`) have no byte-source (`.8h`-wide) form.
    pub fn allows_wide(self, wide: Arm64VectorArrangement) -> bool {
        use Arm64VectorArrangement::{D2, H8, S4};
        match self {
            Self::Sqdmlal | Self::Sqdmlsl | Self::Sqdmull => matches!(wide, S4 | D2),
            _ => matches!(wide, H8 | S4 | D2),
        }
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 25] = [
        Self::Saddl,
        Self::Uaddl,
        Self::Saddw,
        Self::Uaddw,
        Self::Ssubl,
        Self::Usubl,
        Self::Ssubw,
        Self::Usubw,
        Self::Addhn,
        Self::Raddhn,
        Self::Sabal,
        Self::Uabal,
        Self::Subhn,
        Self::Rsubhn,
        Self::Sabdl,
        Self::Uabdl,
        Self::Smlal,
        Self::Umlal,
        Self::Sqdmlal,
        Self::Smlsl,
        Self::Umlsl,
        Self::Sqdmlsl,
        Self::Smull,
        Self::Umull,
        Self::Sqdmull,
    ];
}
