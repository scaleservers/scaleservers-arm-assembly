// Copyright (c) Scaleservers LLC

/// The memory-ordering of an AArch64 atomic instruction -- the acquire/release semantics encoded in the `A`/`R`
/// (or `L`/`o0`) bits of the LSE atomics (`LDADD`/`SWP`/`CAS`/...). Selects the mnemonic suffix `""`/`a`/`l`/`al`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64AtomicOrdering {
    /// Relaxed -- no acquire or release (`LDADD`, `SWP`, `CAS`).
    None,
    /// Acquire -- the access has acquire semantics (`LDADDA`, `SWPA`, `CASA`).
    Acquire,
    /// Release -- the access has release semantics (`LDADDL`, `SWPL`, `CASL`).
    Release,
    /// Acquire + release (`LDADDAL`, `SWPAL`, `CASAL`).
    AcquireRelease,
}

impl Arm64AtomicOrdering {
    /// Whether the ordering includes acquire semantics.
    pub fn acquire(self) -> bool {
        matches!(self, Self::Acquire | Self::AcquireRelease)
    }

    /// Whether the ordering includes release semantics.
    pub fn release(self) -> bool {
        matches!(self, Self::Release | Self::AcquireRelease)
    }

    /// Build the ordering from the decoded acquire/release bits.
    pub fn from_acquire_release(acquire: bool, release: bool) -> Self {
        match (acquire, release) {
            (false, false) => Self::None,
            (true, false) => Self::Acquire,
            (false, true) => Self::Release,
            (true, true) => Self::AcquireRelease,
        }
    }

    /// The lowercase mnemonic suffix: `""` (relaxed), `"a"` (acquire), `"l"` (release), `"al"` (both).
    pub fn suffix(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Acquire => "a",
            Self::Release => "l",
            Self::AcquireRelease => "al",
        }
    }
}
