// Copyright (c) Scaleservers LLC

// How an ARMv7-M load/store with an 8-bit immediate (the T4 / T3 / T2 "indexed" forms, and LDRD/STRD)
// applies its offset to the base register Rn. This is the P / W pair of the encoding:
//   - `Offset`    (P=1, W=0): `[Rn, #+/-imm]`  -- Rn is unchanged.
//   - `PreIndex`  (P=1, W=1): `[Rn, #+/-imm]!` -- Rn += offset before the access, and is written back.
//   - `PostIndex` (P=0, W=1): `[Rn], #+/-imm`  -- the access uses Rn, then Rn += offset (written back).
// The sign of the offset is carried separately (the U bit); the magnitude is the imm8 (scaled by 4 for
// LDRD/STRD). The fourth P/W combination (P=0, W=0) is the unprivileged LDRT/STRT form, which is not
// modeled here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmT32IndexMode {
    Offset,
    PreIndex,
    PostIndex,
}
impl ArmT32IndexMode {
    // the (P, W) encoding bits for this mode (bit 10 / bit 8 in the single-register T4 form).
    pub fn p_w_bits(&self) -> (u32, u32) {
        match self {
            Self::Offset => (1, 0),
            Self::PreIndex => (1, 1),
            Self::PostIndex => (0, 1),
        }
    }

    // Recover the mode from a decoded (P, W) pair; `None` for the unmodeled (P=0, W=0) LDRT/STRT case.
    pub fn from_p_w_bits(p: u32, w: u32) -> Option<Self> {
        // p and w are 1-bit fields; mask so stray high bits are ignored and the fallback is exactly (0, 0).
        match (p & 1, w & 1) {
            (1, 0) => Some(Self::Offset),
            (1, 1) => Some(Self::PreIndex),
            (0, 1) => Some(Self::PostIndex),
            _ => None, // (0, 0) -- the only combo `& 1` leaves
        }
    }
}
