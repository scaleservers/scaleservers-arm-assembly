// Copyright (c) Scaleservers LLC

/// The operation of a FEAT_LSUI unprivileged LSE atomic memory operation (`LDTADD`/`LDTCLR`/`LDTSET`/`SWPT`,
/// DDI0487 "Unprivileged load/store" atomics). Unlike the privileged LSE atomics ([`super::Arm64AtomicOp`]),
/// FEAT_LSUI defines only add/clr/set and swap (no eor/smax/smin/umax/umin), and only at 32-bit (`W`) and
/// 64-bit (`X`) sizes. The 4-bit field `[15:12]` (`o3` at `[15]` + `opc` at `[14:12]`) selects the op.
///
/// The `ST` aliases (`STTADD`/`STTCLR`/`STTSET`) are the `Rt == ZR` (discard the prior value) forms of the
/// corresponding `LDT*`; they decode back to the `LDT*` architectural mnemonic with `xzr`/`wzr`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64LsuiAtomicOp {
    /// `LDTADD` -- `[Xn] += Rs` (`o3:opc = 0b0000`).
    Add,
    /// `LDTCLR` -- `[Xn] &= ~Rs` (bit clear, `o3:opc = 0b0001`).
    Clr,
    /// `LDTSET` -- `[Xn] |= Rs` (bit set, `o3:opc = 0b0011`).
    Set,
    /// `SWPT` -- swap `Rs` into `[Xn]` (`o3:opc = 0b1000`).
    Swp,
}

impl Arm64LsuiAtomicOp {
    /// Every operation, for exhaustive round-trip tests.
    pub const ALL: [Self; 4] = [Self::Add, Self::Clr, Self::Set, Self::Swp];

    /// The 4-bit `o3:opc` field value (`[15:12]`).
    pub fn op_bits(self) -> u32 {
        match self {
            Self::Add => 0b0000,
            Self::Clr => 0b0001,
            Self::Set => 0b0011,
            Self::Swp => 0b1000,
        }
    }

    /// Recover the operation from its 4-bit `o3:opc` field (`[15:12]`); `None` for an unallocated value (FEAT_LSUI
    /// leaves eor/smax/smin/umax/umin and the `o3 = 1`, `opc != 0` slots unallocated).
    pub fn from_op_bits(bits: u32) -> Option<Self> {
        match bits & 0b1111 {
            0b0000 => Some(Self::Add),
            0b0001 => Some(Self::Clr),
            0b0011 => Some(Self::Set),
            0b1000 => Some(Self::Swp),
            _ => None,
        }
    }

    /// The full base mnemonic (before the ordering suffix): `ldtadd`/`ldtclr`/`ldtset`/`swpt`.
    pub fn mnemonic(self) -> &'static str {
        match self {
            Self::Add => "ldtadd",
            Self::Clr => "ldtclr",
            Self::Set => "ldtset",
            Self::Swp => "swpt",
        }
    }
}
