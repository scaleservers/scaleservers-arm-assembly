// Copyright (c) Scaleservers LLC

/// How a SHA instruction displays one of its SIMD&FP register operands -- the same `V` register shown as a 128-bit
/// `Qn`, a 32-bit scalar `Sn`, or a `.4s` vector, depending on the op and operand position. The emitter renders
/// the actual `q0`/`s0`/`v0.4s` text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64ShaRegView {
    /// `Qn` -- the full 128-bit view.
    Q,
    /// `Sn` -- the low 32-bit scalar view.
    S,
    /// `Vn.4s` -- the four-lane single-precision vector view.
    V4s,
    /// `Vn.2d` -- the two-lane doubleword vector view (the SHA512 ops).
    V2d,
}

/// A three-register SHA1/SHA256 acceleration op (DDI0487 C7, FEAT_SHA1/FEAT_SHA256) -- the
/// `01011110 000 Rm 0 opcode 00 Rn Rd` encoding. The `Rm` operand is always `.4s`; `Vd`/`Vn` use the views
/// `rd_view`/`rn_view` (the SHA1 "choose/parity/majority" ops mix `Qd, Sn`, the SHA256 hash ops
/// use `Qd, Qn`, and the schedule-update ops are all `.4s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorSha3Op {
    /// `SHA1C Qd, Sn, Vm.4s` -- SHA1 hash update (choose) (opcode `0000`).
    Sha1c,
    /// `SHA1P Qd, Sn, Vm.4s` -- SHA1 hash update (parity) (opcode `0001`).
    Sha1p,
    /// `SHA1M Qd, Sn, Vm.4s` -- SHA1 hash update (majority) (opcode `0010`).
    Sha1m,
    /// `SHA1SU0 Vd.4s, Vn.4s, Vm.4s` -- SHA1 schedule update 0 (opcode `0011`).
    Sha1su0,
    /// `SHA256H Qd, Qn, Vm.4s` -- SHA256 hash update (part 1) (opcode `0100`).
    Sha256h,
    /// `SHA256H2 Qd, Qn, Vm.4s` -- SHA256 hash update (part 2) (opcode `0101`).
    Sha256h2,
    /// `SHA256SU1 Vd.4s, Vn.4s, Vm.4s` -- SHA256 schedule update 1 (opcode `0110`).
    Sha256su1,
}

impl Arm64VectorSha3Op {
    /// The base word with `Rm`/`Rn`/`Rd` zero (the opcode baked in). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Sha1c => 0x5E00_0000,
            Self::Sha1p => 0x5E00_1000,
            Self::Sha1m => 0x5E00_2000,
            Self::Sha1su0 => 0x5E00_3000,
            Self::Sha256h => 0x5E00_4000,
            Self::Sha256h2 => 0x5E00_5000,
            Self::Sha256su1 => 0x5E00_6000,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sha1c => "sha1c",
            Self::Sha1p => "sha1p",
            Self::Sha1m => "sha1m",
            Self::Sha1su0 => "sha1su0",
            Self::Sha256h => "sha256h",
            Self::Sha256h2 => "sha256h2",
            Self::Sha256su1 => "sha256su1",
        }
    }

    /// The display view of the `Vd` operand.
    pub fn rd_view(self) -> Arm64ShaRegView {
        match self {
            Self::Sha1c | Self::Sha1p | Self::Sha1m | Self::Sha256h | Self::Sha256h2 => {
                Arm64ShaRegView::Q
            }
            Self::Sha1su0 | Self::Sha256su1 => Arm64ShaRegView::V4s,
        }
    }

    /// The display view of the `Vn` operand. (`Vm` is always `.4s`.)
    pub fn rn_view(self) -> Arm64ShaRegView {
        match self {
            Self::Sha1c | Self::Sha1p | Self::Sha1m => Arm64ShaRegView::S,
            Self::Sha256h | Self::Sha256h2 => Arm64ShaRegView::Q,
            Self::Sha1su0 | Self::Sha256su1 => Arm64ShaRegView::V4s,
        }
    }

    /// Recover the op from the 4-bit opcode `[15:12]`; `None` for unallocated opcodes.
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        match opcode & 0b1111 {
            0b0000 => Some(Self::Sha1c),
            0b0001 => Some(Self::Sha1p),
            0b0010 => Some(Self::Sha1m),
            0b0011 => Some(Self::Sha1su0),
            0b0100 => Some(Self::Sha256h),
            0b0101 => Some(Self::Sha256h2),
            0b0110 => Some(Self::Sha256su1),
            _ => None,
        }
    }

    /// Every operation, for tests.
    pub const ALL: [Self; 7] = [
        Self::Sha1c,
        Self::Sha1p,
        Self::Sha1m,
        Self::Sha1su0,
        Self::Sha256h,
        Self::Sha256h2,
        Self::Sha256su1,
    ];
}

/// A two-register SHA1/SHA256 acceleration op (DDI0487 C7) -- the `01011110 00 10100 0 opcode 10 Rn Rd`
/// encoding. `Vd`/`Vn` use the views `rd_view`/`rn_view` (`SHA1H` is scalar `Sd, Sn`; the
/// schedule-update ops are `.4s`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64VectorSha2Op {
    /// `SHA1H Sd, Sn` -- SHA1 fixed rotate (opcode `00000`).
    Sha1h,
    /// `SHA1SU1 Vd.4s, Vn.4s` -- SHA1 schedule update 1 (opcode `00001`).
    Sha1su1,
    /// `SHA256SU0 Vd.4s, Vn.4s` -- SHA256 schedule update 0 (opcode `00010`).
    Sha256su0,
}

impl Arm64VectorSha2Op {
    /// The base word with `Rn`/`Rd` zero (the opcode baked in). GNU+LLVM dual-oracle verified.
    pub fn base(self) -> u32 {
        match self {
            Self::Sha1h => 0x5E28_0800,
            Self::Sha1su1 => 0x5E28_1800,
            Self::Sha256su0 => 0x5E28_2800,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sha1h => "sha1h",
            Self::Sha1su1 => "sha1su1",
            Self::Sha256su0 => "sha256su0",
        }
    }

    /// The display views of `Vd` and `Vn` (`SHA1H` is scalar `S`, the rest `.4s`).
    pub fn views(self) -> (Arm64ShaRegView, Arm64ShaRegView) {
        match self {
            Self::Sha1h => (Arm64ShaRegView::S, Arm64ShaRegView::S),
            Self::Sha1su1 | Self::Sha256su0 => (Arm64ShaRegView::V4s, Arm64ShaRegView::V4s),
        }
    }

    /// Recover the op from the 5-bit opcode `[16:12]`; `None` for unallocated opcodes.
    pub fn from_opcode(opcode: u32) -> Option<Self> {
        match opcode & 0b11111 {
            0b00000 => Some(Self::Sha1h),
            0b00001 => Some(Self::Sha1su1),
            0b00010 => Some(Self::Sha256su0),
            _ => None,
        }
    }

    /// Every operation, for tests.
    pub const ALL: [Self; 3] = [Self::Sha1h, Self::Sha1su1, Self::Sha256su0];
}
