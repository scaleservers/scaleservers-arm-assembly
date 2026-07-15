// Copyright (c) Scaleservers LLC

// Orthogonal architecture-extension flags. These gate instructions whose availability is not captured
// by the coarse ISA version alone. The v6-M and v7-M base sets require none of these; the ARMv7E-M
// DSP/SIMD and hardware floating-point families that are now implemented require `DspExtension` and
// `FloatingPoint` respectively. `PartialOrd, Ord` are derived so this can be the element type of an
// `alloc::collections::BTreeSet` (used by `ArmTargetProfile` under `no_std`, replacing the std
// `HashSet`). The ordering is a total order over the variants and is used ONLY for set storage --
// feature gating is pure membership (`.contains()`), so the particular order never reaches an emitted
// byte. `Hash` is retained (harmless; keeps the type hashable).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArmCpuFeature {
    DspExtension,  // ARMv7E-M DSP / SIMD instructions (Cortex-M4/M7)
    FloatingPoint, // FPv4-SP / FPv5 hardware floating point (Cortex-M4F/M7) -- reserved for the FP subsystem
    AdvancedSimd,  // NEON Advanced SIMD (the A-profile vector unit, D0-31 / Q0-15)
    Crypto,        // ARMv8 cryptography extension (AES / SHA1 / SHA256 / polynomial VMULL.p64)
    Security,      // ARMv8-M Security Extension / TrustZone-M (SG, BXNS, BLXNS, TT/TTT/TTA/TTAT)
    Mve, // ARMv8.1-M MVE "Helium" integer vector extension (Cortex-M55/M85) -- the Q0..Q7 vector ISA
    MveFloat, // the MVE floating-point option (.f16/.f32 vector ops); implies Mve + FloatingPoint
}
