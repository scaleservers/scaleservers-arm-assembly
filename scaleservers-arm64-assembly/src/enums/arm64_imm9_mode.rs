// Copyright (c) Scaleservers LLC

/// The addressing mode of an AArch64 single-register load/store that uses the **9-bit unscaled immediate**
/// (`imm9`, bits `[20:12]`) -- i.e. the non-`uimm12` forms that share one encoding distinguished by the index
/// field `idx[11:10]` (DDI0487 C6.2 "Load/store register (unscaled immediate)" and "(immediate pre/post-indexed)").
///
/// All four carry the same signed `-256..=255` byte offset; they differ in writeback and mnemonic:
/// * [`Self::Unscaled`] -- `LDUR`/`STUR Rt, [Xn, #imm]`: no writeback (the unscaled counterpart of the scaled
///   `uimm12` `LDR`/`STR`, used when the offset is negative or not a multiple of the access size).
/// * [`Self::PostIndex`] -- `LDR Rt, [Xn], #imm`: use `Xn` as the address, then write `Xn + imm` back.
/// * [`Self::Unprivileged`] -- `LDTR`/`STTR Rt, [Xn, #imm]`: unprivileged access (EL0 permission checks when
///   executed at EL1), no writeback. Same address form as `Unscaled`. Index field `0b10`.
/// * [`Self::PreIndex`] -- `LDR Rt, [Xn, #imm]!`: write `Xn + imm` back and use it as the address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Imm9Mode {
    /// `LDUR`/`STUR` -- `[Xn, #imm]`, no writeback. Index field `0b00`.
    Unscaled,
    /// `[Xn], #imm` -- post-index: address is `Xn`, then `Xn += imm`. Index field `0b01`.
    PostIndex,
    /// `LDTR`/`STTR` -- `[Xn, #imm]`, unprivileged, no writeback. Index field `0b10`.
    Unprivileged,
    /// `[Xn, #imm]!` -- pre-index: `Xn += imm`, then use `Xn` as the address. Index field `0b11`.
    PreIndex,
}

impl Arm64Imm9Mode {
    /// The 2-bit index field value (`[11:10]`): `0b00` unscaled, `0b01` post-index, `0b10` unprivileged, `0b11`
    /// pre-index.
    pub fn index_bits(self) -> u32 {
        match self {
            Self::Unscaled => 0b00,
            Self::PostIndex => 0b01,
            Self::Unprivileged => 0b10,
            Self::PreIndex => 0b11,
        }
    }

    /// Recover the mode from the 2-bit index field. All four values are allocated (`0b10` is the unprivileged
    /// `LDTR`/`STTR` form); the caller still rejects size/opc combinations that are unallocated for a given mode.
    pub fn from_index_bits(bits: u32) -> Option<Self> {
        match bits & 0b11 {
            0b00 => Some(Self::Unscaled),
            0b01 => Some(Self::PostIndex),
            0b10 => Some(Self::Unprivileged),
            0b11 => Some(Self::PreIndex),
            _ => unreachable!("2-bit field has no other values"),
        }
    }
}
