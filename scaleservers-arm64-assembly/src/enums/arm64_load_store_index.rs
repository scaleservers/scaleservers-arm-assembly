// Copyright (c) Scaleservers LLC

/// The addressing/indexing mode of an AArch64 load/store **pair** (`LDP`/`STP`): how the base register `Xn`
/// and the signed offset combine, encoded in the 2-bit index field `[24:23]`.
///
/// The three modes are the prologue/epilogue workhorses: `[Xn, #imm]` (plain offset, `Xn` unchanged),
/// `[Xn, #imm]!` (pre-index -- add the offset to `Xn`, use the result as the address, write it back), and
/// `[Xn], #imm` (post-index -- use `Xn` as the address, THEN add the offset and write it back). The classic
/// `stp x29, x30, [sp, #-16]!` / `ldp x29, x30, [sp], #16` frame save/restore use pre- and post-index
/// respectively.
///
/// (This is the pair-encoding's index field. The single-register loads model the plain offset form here;
/// their pre/post-index variants live in a different sub-encoding, the 9-bit signed-immediate form modeled via
/// `Arm64Imm9Mode`.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64LoadStoreIndex {
    /// `[Xn, #imm]` -- signed offset, base unchanged. Index field `0b10`.
    Offset,
    /// `[Xn, #imm]!` -- pre-index: address is `Xn + imm`, written back to `Xn`. Index field `0b11`.
    PreIndex,
    /// `[Xn], #imm` -- post-index: address is `Xn`, then `Xn += imm` written back. Index field `0b01`.
    PostIndex,
}

impl Arm64LoadStoreIndex {
    /// The 2-bit index field value (`[24:23]` of the load/store pair encoding): `0b10` offset, `0b11`
    /// pre-index, `0b01` post-index. (The `0b00` value is the non-allocated / "no-allocate" `LDNP`/`STNP`
    /// slot -- a distinct instruction, modeled as `LoadStorePairNonTemporal`, not an LDP/STP index mode.) The
    /// encoder shifts this into bits `[24:23]`.
    pub fn index_bits(self) -> u32 {
        match self {
            Self::PostIndex => 0b01,
            Self::Offset => 0b10,
            Self::PreIndex => 0b11,
        }
    }

    /// Recover the index mode from the 2-bit field of a decoded pair word. The `0b00` slot is the
    /// `LDNP`/`STNP` (no-allocate) form (a distinct instruction, modeled as `LoadStorePairNonTemporal`), so it
    /// returns `None` (the decode arm then falls through to
    /// `InvalidOpcode`); the other three map to their mode. TOTAL over the two bits, never panics.
    pub fn from_index_bits(bits: u32) -> Option<Self> {
        match bits & 0b11 {
            0b01 => Some(Self::PostIndex),
            0b10 => Some(Self::Offset),
            0b11 => Some(Self::PreIndex),
            // 0b00 = LDNP/STNP (no-allocate) -- a distinct instruction (LoadStorePairNonTemporal), not an LDP/STP index mode.
            _ => None,
        }
    }
}
