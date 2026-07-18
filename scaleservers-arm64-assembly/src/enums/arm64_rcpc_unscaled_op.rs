// Copyright (c) Scaleservers LLC

use crate::enums::Arm64RegisterWidth;

/// An AArch64 **RCpc unscaled** load/store op (DDI0487 C6, FEAT_LRCPC2) -- `STLUR`/`LDAPUR` and the signed
/// `LDAPURSB`/`LDAPURSH`/`LDAPURSW`. All share the operand shape `<Rt>, [<Xn|SP>{, #simm9}]`; the op selects the
/// access size, the load/store direction with acquire/release ordering, and (for the signed loads) the dest width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64RcpcUnscaledOp {
    /// `STLURB Wt, [Xn{, #imm}]` -- store-release byte.
    Stlurb,
    /// `STLURH Wt, [Xn{, #imm}]` -- store-release halfword.
    Stlurh,
    /// `STLUR Wt, [Xn{, #imm}]` -- store-release word.
    StlurW,
    /// `STLUR Xt, [Xn{, #imm}]` -- store-release doubleword.
    StlurX,
    /// `LDAPURB Wt, [Xn{, #imm}]` -- load-acquire RCpc byte.
    Ldapurb,
    /// `LDAPURH Wt, [Xn{, #imm}]` -- load-acquire RCpc halfword.
    Ldapurh,
    /// `LDAPUR Wt, [Xn{, #imm}]` -- load-acquire RCpc word.
    LdapurW,
    /// `LDAPUR Xt, [Xn{, #imm}]` -- load-acquire RCpc doubleword.
    LdapurX,
    /// `LDAPURSB Xt, [Xn{, #imm}]` -- load-acquire RCpc signed byte to 64-bit.
    LdapursbX,
    /// `LDAPURSH Xt, [Xn{, #imm}]` -- load-acquire RCpc signed halfword to 64-bit.
    LdapurshX,
    /// `LDAPURSW Xt, [Xn{, #imm}]` -- load-acquire RCpc signed word to 64-bit.
    Ldapursw,
    /// `LDAPURSB Wt, [Xn{, #imm}]` -- load-acquire RCpc signed byte to 32-bit.
    LdapursbW,
    /// `LDAPURSH Wt, [Xn{, #imm}]` -- load-acquire RCpc signed halfword to 32-bit.
    LdapurshW,
}

impl Arm64RcpcUnscaledOp {
    /// The `(size[31:30], opc[23:22])` selector pair for this op.
    pub fn size_opc(self) -> (u32, u32) {
        match self {
            Self::Stlurb => (0b00, 0b00),
            Self::Stlurh => (0b01, 0b00),
            Self::StlurW => (0b10, 0b00),
            Self::StlurX => (0b11, 0b00),
            Self::Ldapurb => (0b00, 0b01),
            Self::Ldapurh => (0b01, 0b01),
            Self::LdapurW => (0b10, 0b01),
            Self::LdapurX => (0b11, 0b01),
            Self::LdapursbX => (0b00, 0b10),
            Self::LdapurshX => (0b01, 0b10),
            Self::Ldapursw => (0b10, 0b10),
            Self::LdapursbW => (0b00, 0b11),
            Self::LdapurshW => (0b01, 0b11),
        }
    }

    /// Whether this op is a store (`STLUR*`).
    pub fn is_store(self) -> bool {
        matches!(
            self,
            Self::Stlurb | Self::Stlurh | Self::StlurW | Self::StlurX
        )
    }

    /// The width (`W`/`X`) the `Rt` operand renders in.
    pub fn rt_width(self) -> Arm64RegisterWidth {
        match self {
            Self::StlurX | Self::LdapurX | Self::LdapursbX | Self::LdapurshX | Self::Ldapursw => {
                Arm64RegisterWidth::X
            }
            _ => Arm64RegisterWidth::W,
        }
    }

    /// The lowercase UAL mnemonic.
    pub fn name(self) -> &'static str {
        match self {
            Self::Stlurb => "stlurb",
            Self::Stlurh => "stlurh",
            Self::StlurW | Self::StlurX => "stlur",
            Self::Ldapurb => "ldapurb",
            Self::Ldapurh => "ldapurh",
            Self::LdapurW | Self::LdapurX => "ldapur",
            Self::LdapursbX | Self::LdapursbW => "ldapursb",
            Self::LdapurshX | Self::LdapurshW => "ldapursh",
            Self::Ldapursw => "ldapursw",
        }
    }

    /// Recover the op from its `(size, opc)` selector, if allocated.
    pub fn from_size_opc(size: u32, opc: u32) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|op| op.size_opc() == (size, opc))
    }

    /// Every op, for tests and table-driven decode.
    pub const ALL: [Self; 13] = [
        Self::Stlurb,
        Self::Stlurh,
        Self::StlurW,
        Self::StlurX,
        Self::Ldapurb,
        Self::Ldapurh,
        Self::LdapurW,
        Self::LdapurX,
        Self::LdapursbX,
        Self::LdapurshX,
        Self::Ldapursw,
        Self::LdapursbW,
        Self::LdapurshW,
    ];
}
