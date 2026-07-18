// Copyright (c) Scaleservers LLC

/// The operation of an AArch64 Advanced SIMD (NEON) **shift by immediate with a size change** -- the
/// `0 Q U 011110 immh immb opcode 1 Rn Rd` class restricted to the opcodes whose source and destination element
/// widths differ: the *long* (widening) left shift `SSHLL`/`USHLL` (`SXTL`/`UXTL` are the `#0` aliases) and the
/// *narrowing* right shifts (`SHRN`/`RSHRN`/`SQSHRN`/`SQRSHRN`/`SQSHRUN`/`SQRSHRUN`/`UQSHRN`/`UQRSHRN`). The op is
/// an orthogonal field over a shared `{ narrow arrangement, Vd, Vn, shift }` shape: the *narrow* side
/// (`.8b`/`.16b`/`.4h`/`.8h`/`.2s`/`.4s`) gives the element size that `immh:immb` folds with the shift and the
/// `Q` upper-half (`2`-suffix) bit; the *wide* side is twice as wide. [`Self::is_long`] says which side is the
/// destination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorShiftLongNarrowOp {
    /// `SSHLL` -- signed shift left long (`U = 0`, opcode `10100`; `SXTL` is the `#0` alias). Widens.
    Sshll,
    /// `USHLL` -- unsigned shift left long (`U = 1`, opcode `10100`; `UXTL` is the `#0` alias). Widens.
    Ushll,
    /// `SHRN` -- shift right narrow (`U = 0`, opcode `10000`). Narrows.
    Shrn,
    /// `RSHRN` -- rounding shift right narrow (`U = 0`, opcode `10001`). Narrows.
    Rshrn,
    /// `SQSHRN` -- signed saturating shift right narrow (`U = 0`, opcode `10010`). Narrows.
    Sqshrn,
    /// `SQRSHRN` -- signed saturating rounding shift right narrow (`U = 0`, opcode `10011`). Narrows.
    Sqrshrn,
    /// `SQSHRUN` -- signed saturating shift right unsigned narrow (`U = 1`, opcode `10000`). Narrows.
    Sqshrun,
    /// `SQRSHRUN` -- signed saturating rounding shift right unsigned narrow (`U = 1`, opcode `10001`). Narrows.
    Sqrshrun,
    /// `UQSHRN` -- unsigned saturating shift right narrow (`U = 1`, opcode `10010`). Narrows.
    Uqshrn,
    /// `UQRSHRN` -- unsigned saturating rounding shift right narrow (`U = 1`, opcode `10011`). Narrows.
    Uqrshrn,
}

impl Arm64VectorShiftLongNarrowOp {
    /// The base word with `Q = 0` and `immh:immb = 0` (`U` and opcode baked in); the encoder adds `Q<<30`, the
    /// computed `immh:immb << 16`, and `Vn<<5 | Vd`. GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Sshll => 0x0F00_A400,
            Self::Ushll => 0x2F00_A400,
            Self::Shrn => 0x0F00_8400,
            Self::Rshrn => 0x0F00_8C00,
            Self::Sqshrn => 0x0F00_9400,
            Self::Sqrshrn => 0x0F00_9C00,
            Self::Sqshrun => 0x2F00_8400,
            Self::Sqrshrun => 0x2F00_8C00,
            Self::Uqshrn => 0x2F00_9400,
            Self::Uqrshrn => 0x2F00_9C00,
        }
    }

    /// The lowercase UAL mnemonic (without the `2` upper-half suffix, appended by the emitter from the narrow
    /// arrangement's `Q` bit).
    pub fn name(self) -> &'static str {
        match self {
            Self::Sshll => "sshll",
            Self::Ushll => "ushll",
            Self::Shrn => "shrn",
            Self::Rshrn => "rshrn",
            Self::Sqshrn => "sqshrn",
            Self::Sqrshrn => "sqrshrn",
            Self::Sqshrun => "sqshrun",
            Self::Sqrshrun => "sqrshrun",
            Self::Uqshrn => "uqshrn",
            Self::Uqrshrn => "uqrshrn",
        }
    }

    /// Whether this is a *long* (widening) left shift (`SSHLL`/`USHLL`) -- destination is the wide side, shift
    /// `0..narrow_bits-1`, `immh:immb = narrow_bits + shift`. The narrowing ops shift right: destination is the
    /// narrow side, shift `1..narrow_bits`, `immh:immb = 2*narrow_bits - shift`.
    pub fn is_long(self) -> bool {
        matches!(self, Self::Sshll | Self::Ushll)
    }

    /// Every operation, for decode dispatch.
    pub const ALL: [Self; 10] = [
        Self::Sshll,
        Self::Ushll,
        Self::Shrn,
        Self::Rshrn,
        Self::Sqshrn,
        Self::Sqrshrn,
        Self::Sqshrun,
        Self::Sqrshrun,
        Self::Uqshrn,
        Self::Uqrshrn,
    ];
}
