// Copyright (c) Scaleservers LLC

use crate::enums::{
    Arm32GeneralPurposeRegister,
    Arm32RegisterShift,
};

// The offset operand of an A32 single load/store (LDR/STR/LDRB/STRB and the unprivileged LDRT/... forms):
// either a 12-bit immediate or a barrel-shifted register, each carrying its own add/subtract sign (the U
// bit of the encoding). It is combined with an `Arm32IndexMode` (offset / pre-index / post-index) to form
// the full `[Rn, ...]` addressing. Note the A32 (P,W) -> index-mode mapping differs from T32: A32
// post-index is W=0, and (P=0, W=1) is the unprivileged "T" form (a distinct mnemonic), not post-index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MemoryOffset {
    Immediate { add: bool, imm12: u16 },
    Register { add: bool, rm: Arm32GeneralPurposeRegister, shift: Arm32RegisterShift },
}

// The offset operand of an A32 "extra" load/store (LDRH/STRH/LDRSB/LDRSH/LDRD/STRD and their unprivileged
// "T" forms): either an 8-bit immediate (split imm4H:imm4L in the encoding) or a plain register (NO shift),
// each with an add/subtract sign. The narrower immediate and shift-less register are why this is a
// separate type from `Arm32MemoryOffset`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32MemoryOffset8 {
    Immediate { add: bool, imm8: u8 },
    Register { add: bool, rm: Arm32GeneralPurposeRegister },
}
