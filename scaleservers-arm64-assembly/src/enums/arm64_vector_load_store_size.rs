// Copyright (c) Scaleservers LLC

use alloc::string::String;

use crate::enums::Arm64FloatRegister;

/// The access width of an AArch64 SIMD&FP single-register load/store (`LDR`/`STR` of a `B`/`H`/`S`/`D`/`Q`
/// register) -- which of the five SIMD&FP operand sizes (8/16/32/64/128-bit) the instruction transfers.
///
/// ## Encoding (DDI0487 C7, "LDR/STR (immediate, SIMD&FP)", unsigned-offset)
/// The size is split across two fields: the 2-bit `size` field `[31:30]` and the high `opc` bit `[23]`. For
/// the 8/16/32/64-bit accesses `opc<1> = 0` and `size` selects `B`/`H`/`S`/`D`; the 128-bit `Q` access is
/// `opc<1> = 1` with `size = 00`. As with the general-purpose load/store, the unsigned immediate offset is
/// **scaled by the access size** (`byte_offset = imm12 << scale`), so [`Self::scale`] gives `log2(bytes)`.
/// The transfer register is the SIMD&FP register named with the size letter (`b`/`h`/`s`/`d`/`q`), supplied by
/// [`Self::register_name`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm64VectorLoadStoreSize {
    /// 8-bit access (`LDR`/`STR` of a `Bn`) -- `size = 00`, `opc<1> = 0`, offset scale 1.
    Byte,
    /// 16-bit access (`Hn`) -- `size = 01`, `opc<1> = 0`, offset scale 2.
    Half,
    /// 32-bit access (`Sn`, the f32 view) -- `size = 10`, `opc<1> = 0`, offset scale 4.
    Single,
    /// 64-bit access (`Dn`, the f64 view) -- `size = 11`, `opc<1> = 0`, offset scale 8.
    Double,
    /// 128-bit access (`Qn`, the full v128 vector) -- `size = 00`, `opc<1> = 1`, offset scale 16.
    Quad,
}

impl Arm64VectorLoadStoreSize {
    /// The 2-bit `size` field value `[31:30]`: Byte/Half/Single/Double = 0/1/2/3, and Quad = 0 (its width comes
    /// from `opc<1>` instead). The encoder shifts this into bits `[31:30]`.
    pub fn size_field(self) -> u32 {
        match self {
            Self::Byte | Self::Quad => 0,
            Self::Half => 1,
            Self::Single => 2,
            Self::Double => 3,
        }
    }

    /// The high `opc` bit `[23]` -- 1 for the 128-bit [`Self::Quad`] access, 0 for the others. (The low `opc`
    /// bit `[22]` is the `L` load/store bit, supplied by the encoder, not the size.)
    pub fn opc_high(self) -> u32 {
        match self {
            Self::Quad => 1,
            _ => 0,
        }
    }

    /// The byte-offset scale as `log2(access_bytes)`: Byte 0, Half 1, Single 2, Double 3, Quad 4. The unsigned
    /// immediate offset must be a non-negative multiple of `1 << scale`, and `imm12 = byte_offset >> scale`.
    pub fn scale(self) -> u32 {
        match self {
            Self::Byte => 0,
            Self::Half => 1,
            Self::Single => 2,
            Self::Double => 3,
            Self::Quad => 4,
        }
    }

    /// Render the SIMD&FP transfer register with this size's letter prefix: `b`/`h`/`s`/`d`/`q` + the register
    /// number (e.g. `q0`, `d5`, `s2`). Used by the emitter.
    pub fn register_name(self, reg: &Arm64FloatRegister) -> String {
        let number = reg.as_operand_bits();
        let letter = match self {
            Self::Byte => 'b',
            Self::Half => 'h',
            Self::Single => 's',
            Self::Double => 'd',
            Self::Quad => 'q',
        };
        format!("{letter}{number}")
    }

    /// Recover the access size from a decoded word's `size` field `[31:30]` and high `opc` bit `[23]`. Returns
    /// `None` for the unallocated combinations (`opc<1> = 1` with a non-zero `size`), so decode can reject them.
    pub fn from_size_and_opc_high(size_field: u32, opc_high: u32) -> Option<Self> {
        match (opc_high & 1, size_field & 0b11) {
            (0, 0) => Some(Self::Byte),
            (0, 1) => Some(Self::Half),
            (0, 2) => Some(Self::Single),
            (0, 3) => Some(Self::Double),
            (1, 0) => Some(Self::Quad),
            _ => None, // opc<1>=1 with size != 00 is unallocated
        }
    }

    /// Every access size, for decode dispatch / exhaustive tests.
    pub const ALL: [Self; 5] = [
        Self::Byte,
        Self::Half,
        Self::Single,
        Self::Double,
        Self::Quad,
    ];
}
