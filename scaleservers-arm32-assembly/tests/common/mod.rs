// Copyright (c) Scaleservers LLC
//
// Shared oracle-presence policy for the differential-oracle integration tests. The differential tests shell out
// to an external assembler (GNU `arm-none-eabi-as` for T32/A32, LLVM `llvm-mc` for A32/MVE) as a byte-for-byte
// spec oracle. Historically a missing oracle made each test SKIP green -- convenient on a dev box, but a footgun
// in CI: a lane that is *supposed* to validate against the oracle could silently pass without ever consulting it.
//
// `ARM32_REQUIRE_ORACLE` closes that gap. When it is set in the environment (the CI lane that installs the
// toolchains sets it), a missing oracle is a HARD FAILURE instead of a skip; otherwise the behaviour is the old
// loud-skip so local runs without a toolchain stay convenient.

/// Call at an oracle-absence skip site. Panics (failing the test) when `ARM32_REQUIRE_ORACLE` is set; otherwise
/// prints a `SKIP <context>` line. The caller still `return;`s afterward.
pub fn skip_or_require(context: &str) {
    if std::env::var_os("ARM32_REQUIRE_ORACLE").is_some() {
        panic!(
            "ARM32_REQUIRE_ORACLE is set but the differential oracle is unavailable: {context}. \
             Install the assembler (arm-none-eabi-as / llvm-mc) in this lane or unset ARM32_REQUIRE_ORACLE."
        );
    }
    eprintln!("SKIP {context}");
}
