// Copyright (c) Scaleservers LLC

/// Decode-time context for resolving the one T32 "same bytes, different meaning" ambiguity -- the family-wide
/// **Rule R4**: disambiguate by an explicit decode context, never by guessing.
///
/// The case in T32 is the **CDE (Custom Datapath Extension) coprocessor space**. Coprocessors 0-7 *may* be
/// configured as CDE, and when a given coprocessor is CDE its `CX*`/`VCX*` custom-datapath instructions share
/// their *exact* encoding with a generic `CDP/MCR/LDC/STC/...` on that same coprocessor -- e.g. the word
/// `0xFD94_0000` is BOTH `vcx3a p0, d0, d0, d0, #0` AND `ldc2 p0, c4, [r4]`. [`encode`] is unambiguous (each
/// form is its own leaf variant); only [`decode`] must choose, and which instruction the bytes *are* depends
/// on the target's CDE configuration -- exactly the context this type carries.
///
/// [`ArmT32Instruction::decode`] uses [`ArmDecodeContext::default`], which treats **all** of coprocessors 0-7
/// as CDE -- the decoder's historical canonical interpretation, kept so the bare `decode` stays non-breaking.
/// Pass a custom context to [`ArmT32Instruction::decode_with`] to decode some or all of coprocessors 0-7 as
/// generic coprocessor instructions instead. The byte-stable `encode`<->`decode` round-trip then holds
/// **per context**.
///
/// ```
/// use scaleservers_arm32_assembly::{ArmT32Instruction, ArmDecodeContext};
///
/// // `ldc2 p0, c4, [r4]` and a CDE `vcx3a p0, ...` encode to the SAME bytes.
/// let ldc2 = ArmT32Instruction::Coproc_Ldc_T1(true, false, true, 0, 4,
///     scaleservers_arm32_assembly::Arm32GeneralPurposeRegister::R4, 0);
/// let bytes = ldc2.encode().unwrap();
/// let mut offset = 0;
///
/// // Default context: coprocessor 0 is CDE, so the bytes decode as the CDE form (NOT the LDC2).
/// let canonical = ArmT32Instruction::decode(&mut bytes.iter(), &mut offset).unwrap().unwrap();
/// assert!(matches!(canonical, ArmT32Instruction::Vcx3_T1(..)));
///
/// // A context where coprocessor 0 is NOT CDE decodes the same bytes as the generic LDC2 -- and it round-trips.
/// let mut offset = 0;
/// let generic = ArmT32Instruction::decode_with(&mut bytes.iter(), &mut offset, &ArmDecodeContext::no_cde())
///     .unwrap().unwrap();
/// assert_eq!(generic, ldc2);
/// ```
///
/// [`encode`]: crate::ArmT32Instruction::encode
/// [`decode`]: crate::ArmT32Instruction::decode
/// [`ArmT32Instruction::decode`]: crate::ArmT32Instruction::decode
/// [`ArmT32Instruction::decode_with`]: crate::ArmT32Instruction::decode_with
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArmDecodeContext {
    // Bit N set => coprocessor N (0-7) is a CDE coprocessor. Coprocessors 8-15 are NEVER CDE
    // (architecturally), so only the low 8 bits are meaningful.
    cde_coprocessors: u8,
}

impl ArmDecodeContext {
    /// A context in which the coprocessors named by `cde_coprocessors` are CDE. The argument is a bitmask:
    /// bit N (for N in 0..=7) set marks coprocessor N as a CDE coprocessor. Coprocessors 8-15 are never CDE,
    /// so they have no bit.
    pub const fn with_cde_coprocessors(cde_coprocessors: u8) -> Self {
        Self { cde_coprocessors }
    }

    /// All of coprocessors 0-7 are CDE -- the [`default`](Self::default) (and historical) behaviour: the
    /// decoder reads coprocessor 0-7 words as `CX*`/`VCX*` custom-datapath instructions.
    pub const fn all_cde() -> Self {
        Self::with_cde_coprocessors(0xFF)
    }

    /// No coprocessor is CDE -- coprocessor 0-7 words decode as generic `CDP/MCR/LDC/STC/...` instructions.
    pub const fn no_cde() -> Self {
        Self::with_cde_coprocessors(0x00)
    }

    /// Whether coprocessor `coproc` is a CDE coprocessor in this context. Always `false` for coprocessors
    /// 8-15 (they cannot be CDE).
    pub const fn is_cde_coprocessor(&self, coproc: u8) -> bool {
        coproc < 8 && (self.cde_coprocessors >> coproc) & 1 == 1
    }
}

impl Default for ArmDecodeContext {
    /// All of coprocessors 0-7 are CDE. This preserves the decoder's historical canonical interpretation so
    /// the bare [`ArmT32Instruction::decode`](crate::ArmT32Instruction::decode) is non-breaking (Rule R4).
    fn default() -> Self {
        Self::all_cde()
    }
}
