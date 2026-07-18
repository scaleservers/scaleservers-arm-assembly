// Copyright (c) Scaleservers LLC

/// An SVE2 **widening integer** op (DDI0487 part C): the result element is one size wider than the source(s), with
/// `B`/`T` (bottom/top) variants reading the even/odd source lanes. Three sub-shapes:
/// - *long* (`SADDLB`, `SMULLB`, `PMULLB`, `SADDLBT`, ...): both `Zn`/`Zm` are the narrow element.
/// - *wide* (`SADDWB`/`SSUBWB`, ...): `Zn` is the wide element, only `Zm` is narrow.
/// - *multiply-accumulate long* (`SMLALB`, `SQDMLSLT`, ...): accumulates into `Zda` (encoded at the `0x44` prefix
///   instead of `0x45`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Sve2WideningOp {
    // -- non-accumulating, long (both sources narrow), 0x45 prefix --
    Saddlb,
    Saddlt,
    Uaddlb,
    Uaddlt,
    Ssublb,
    Ssublt,
    Usublb,
    Usublt,
    Sabdlb,
    Sabdlt,
    Uabdlb,
    Uabdlt,
    Sqdmullb,
    Sqdmullt,
    Pmullb,
    Pmullt,
    Smullb,
    Smullt,
    Umullb,
    Umullt,
    Saddlbt,
    Ssublbt,
    Ssubltb,
    // -- non-accumulating, wide (Zn is wide), 0x45 prefix --
    Saddwb,
    Saddwt,
    Uaddwb,
    Uaddwt,
    Ssubwb,
    Ssubwt,
    Usubwb,
    Usubwt,
    // -- multiply-accumulate long (accumulate into Zda), 0x44 prefix --
    Smlalb,
    Smlalt,
    Umlalb,
    Umlalt,
    Smlslb,
    Smlslt,
    Umlslb,
    Umlslt,
    Sqdmlalb,
    Sqdmlalt,
    Sqdmlslb,
    Sqdmlslt,
    // -- BT-cross of the saturating-doubling MAC-long (FEAT_SVE2), 0x44 prefix, opcodes 0x02/0x03 --
    Sqdmlalbt,
    Sqdmlslbt,
}

impl Arm64Sve2WideningOp {
    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        use Arm64Sve2WideningOp::*;
        match self {
            Saddlb => "saddlb",
            Saddlt => "saddlt",
            Uaddlb => "uaddlb",
            Uaddlt => "uaddlt",
            Ssublb => "ssublb",
            Ssublt => "ssublt",
            Usublb => "usublb",
            Usublt => "usublt",
            Sabdlb => "sabdlb",
            Sabdlt => "sabdlt",
            Uabdlb => "uabdlb",
            Uabdlt => "uabdlt",
            Sqdmullb => "sqdmullb",
            Sqdmullt => "sqdmullt",
            Pmullb => "pmullb",
            Pmullt => "pmullt",
            Smullb => "smullb",
            Smullt => "smullt",
            Umullb => "umullb",
            Umullt => "umullt",
            Saddlbt => "saddlbt",
            Ssublbt => "ssublbt",
            Ssubltb => "ssubltb",
            Saddwb => "saddwb",
            Saddwt => "saddwt",
            Uaddwb => "uaddwb",
            Uaddwt => "uaddwt",
            Ssubwb => "ssubwb",
            Ssubwt => "ssubwt",
            Usubwb => "usubwb",
            Usubwt => "usubwt",
            Smlalb => "smlalb",
            Smlalt => "smlalt",
            Umlalb => "umlalb",
            Umlalt => "umlalt",
            Smlslb => "smlslb",
            Smlslt => "smlslt",
            Umlslb => "umlslb",
            Umlslt => "umlslt",
            Sqdmlalb => "sqdmlalb",
            Sqdmlalt => "sqdmlalt",
            Sqdmlslb => "sqdmlslb",
            Sqdmlslt => "sqdmlslt",
            Sqdmlalbt => "sqdmlalbt",
            Sqdmlslbt => "sqdmlslbt",
        }
    }

    /// The 6-bit `[15:10]` opcode.
    pub fn opcode(self) -> u32 {
        use Arm64Sve2WideningOp::*;
        match self {
            Saddlb => 0x00,
            Saddlt => 0x01,
            Uaddlb => 0x02,
            Uaddlt => 0x03,
            Ssublb => 0x04,
            Ssublt => 0x05,
            Usublb => 0x06,
            Usublt => 0x07,
            Sabdlb => 0x0C,
            Sabdlt => 0x0D,
            Uabdlb => 0x0E,
            Uabdlt => 0x0F,
            Saddwb => 0x10,
            Saddwt => 0x11,
            Uaddwb => 0x12,
            Uaddwt => 0x13,
            Ssubwb => 0x14,
            Ssubwt => 0x15,
            Usubwb => 0x16,
            Usubwt => 0x17,
            Sqdmullb => 0x18,
            Sqdmullt => 0x19,
            Pmullb => 0x1A,
            Pmullt => 0x1B,
            Smullb => 0x1C,
            Smullt => 0x1D,
            Umullb => 0x1E,
            Umullt => 0x1F,
            Saddlbt => 0x20,
            Ssublbt => 0x22,
            Ssubltb => 0x23,
            // accumulating group (distinguished by the prefix, not the opcode):
            Smlalb => 0x10,
            Smlalt => 0x11,
            Umlalb => 0x12,
            Umlalt => 0x13,
            Smlslb => 0x14,
            Smlslt => 0x15,
            Umlslb => 0x16,
            Umlslt => 0x17,
            Sqdmlalb => 0x18,
            Sqdmlalt => 0x19,
            Sqdmlslb => 0x1A,
            Sqdmlslt => 0x1B,
            Sqdmlalbt => 0x02,
            Sqdmlslbt => 0x03,
        }
    }

    /// Whether the op accumulates into `Zda` (the `MLAL`/`MLSL` family, encoded at the `0x44` prefix); the rest use
    /// the `0x45` prefix and write a fresh `Zd`.
    pub fn accumulates(self) -> bool {
        use Arm64Sve2WideningOp::*;
        matches!(
            self,
            Smlalb
                | Smlalt
                | Umlalb
                | Umlalt
                | Smlslb
                | Smlslt
                | Umlslb
                | Umlslt
                | Sqdmlalb
                | Sqdmlalt
                | Sqdmlslb
                | Sqdmlslt
                | Sqdmlalbt
                | Sqdmlslbt
        )
    }

    /// Whether `Zn` is the wide (result-sized) element rather than the narrow source element (true only for the
    /// `ADDW`/`SUBW` family).
    pub fn zn_is_wide(self) -> bool {
        use Arm64Sve2WideningOp::*;
        matches!(
            self,
            Saddwb | Saddwt | Uaddwb | Uaddwt | Ssubwb | Ssubwt | Usubwb | Usubwt
        )
    }

    /// Recover the op from the prefix-group flag (`true` = `0x44` accumulate group) and 6-bit opcode.
    pub fn from_bits(accumulate: bool, opcode: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.accumulates() == accumulate && op.opcode() == (opcode & 0x3F))
    }

    /// Every op (the non-accumulating group then the accumulating group).
    pub const ALL: [Self; 45] = {
        use Arm64Sve2WideningOp::*;
        [
            Saddlb, Saddlt, Uaddlb, Uaddlt, Ssublb, Ssublt, Usublb, Usublt, Sabdlb, Sabdlt, Uabdlb,
            Uabdlt, Sqdmullb, Sqdmullt, Pmullb, Pmullt, Smullb, Smullt, Umullb, Umullt, Saddlbt,
            Ssublbt, Ssubltb, Saddwb, Saddwt, Uaddwb, Uaddwt, Ssubwb, Ssubwt, Usubwb, Usubwt,
            Smlalb, Smlalt, Umlalb, Umlalt, Smlslb, Smlslt, Umlslb, Umlslt, Sqdmlalb, Sqdmlalt,
            Sqdmlslb, Sqdmlslt, Sqdmlalbt, Sqdmlslbt,
        ]
    };
}
