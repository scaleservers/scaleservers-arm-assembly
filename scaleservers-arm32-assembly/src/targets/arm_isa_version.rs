// Copyright (c) Scaleservers LLC

// ARM architecture version, spanning BOTH 32-bit lineages this library targets:
//   * the M-profile / Thumb-only lineage   (Armv6M subset of Armv7M subset of Armv7EM), used by `ArmT32Instruction`, and
//   * the A/R-profile lineage (Armv4T subset of Armv5TE subset of Armv6 subset of Armv7AR subset of Armv8A), which additionally runs the
//     A32 ("ARM" state) instruction set used by `ArmA32Instruction`.
//
// The two lineages are NOT linearly comparable (asking whether "v7-A" is newer than "v7E-M" is
// meaningless), so instruction availability is decided by `satisfies()` -- same lineage AND a high-enough
// rank -- never by a naive `<`. The derived `Ord` is retained only for stable sort/debug ordering and
// orders by declaration position; do not use it to compare across lineages.
//
// NOTE: A-profile and R-profile share one lineage here (`Armv7AR`); their few system-instruction
// differences are modeled with feature flags / validation rather than a separate version. The ARMv8-M
// Baseline/Mainline cores are placed at the top of the M lineage; the Security Extension (TrustZone-M)
// instructions they may add are additionally gated by the `Security` feature. CAVEAT: v8-M Baseline is a
// superset of v6-M (not v7-M), so the linear rank slightly over-approximates (a Baseline target's rank also
// satisfies a v7-M/v7E-M requirement) -- the same pragmatic linearization already used for the A/R lineage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArmIsaVersion {
    // ---- M-profile (Thumb-only) lineage ----
    Armv6M,           // Cortex-M0 / M0+ / M1 -- the Thumb subset
    Armv7M,           // Cortex-M3 -- adds the bulk of Thumb-2
    Armv7EM,          // Cortex-M4 / M7 -- adds the DSP (SIMD) extension
    Armv8MBaseline,   // Cortex-M23 -- v6-M superset + the optional Security Extension (TrustZone-M)
    Armv8MMainline,   // Cortex-M33 / M35P / M55 -- v7-M superset + optional Security / DSP / FP
    Armv8_1MMainline, // Cortex-M55 / M85 -- v8-M Mainline superset + optional MVE (Helium) / low-overhead loops

    // ---- A/R-profile lineage (A32 + Thumb) ----
    Armv4T,  // ARM7TDMI / ARM9 -- the classic A32 + Thumb-1 baseline
    Armv5TE, // ARM9E / ARM10E -- adds CLZ, the saturating/DSP (E) multiplies, BLX, BKPT, PLD
    Armv6, // ARM11 -- adds the media/SIMD (GE) instructions, REV, SETEND, SRS/RFE, exclusive access
    Armv7AR, // Cortex-A / Cortex-R -- the full A32 base ISA (MOVW/MOVT, bitfield, DMB/DSB/ISB, ...)
    Armv8A, // ARMv8-A AArch32 -- adds CRC32, the crypto extension, VSEL/VRINT/VMAXNM/VCVTA-N-P-M, ...
}

// Which of the two non-comparable version lineages a version belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmIsaLineage {
    MProfile,  // Thumb-only (Cortex-M) -- runs T32 only
    ARProfile, // A/R-profile + classic cores -- run A32 and Thumb
}

impl ArmIsaVersion {
    // which lineage this version belongs to
    pub fn lineage(self) -> ArmIsaLineage {
        match self {
            Self::Armv6M
            | Self::Armv7M
            | Self::Armv7EM
            | Self::Armv8MBaseline
            | Self::Armv8MMainline
            | Self::Armv8_1MMainline => ArmIsaLineage::MProfile,
            Self::Armv4T | Self::Armv5TE | Self::Armv6 | Self::Armv7AR | Self::Armv8A => {
                ArmIsaLineage::ARProfile
            }
        }
    }

    // monotonically increasing rank WITHIN the lineage; cross-lineage ranks are not comparable
    pub fn rank_within_lineage(self) -> u8 {
        match self {
            Self::Armv6M => 0,
            Self::Armv7M => 1,
            Self::Armv7EM => 2,
            Self::Armv8MBaseline => 3,
            Self::Armv8MMainline => 4,
            Self::Armv8_1MMainline => 5,
            //
            Self::Armv4T => 0,
            Self::Armv5TE => 1,
            Self::Armv6 => 2,
            Self::Armv7AR => 3,
            Self::Armv8A => 4,
        }
    }

    // Does a target running THIS version satisfy a requirement of at least `required`? True only when both
    // are in the same lineage and this version's rank is >= the requirement's. (A future enhancement could
    // let an A/R target at v6T2+ also satisfy a Thumb-2 / M-profile requirement, since Cortex-A/R run
    // Thumb; that cross-lineage path is unused today and intentionally returns false.)
    pub fn satisfies(self, required: ArmIsaVersion) -> bool {
        self.lineage() == required.lineage()
            && self.rank_within_lineage() >= required.rank_within_lineage()
    }
}
