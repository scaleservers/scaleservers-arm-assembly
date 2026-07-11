// Copyright (c) Scaleservers LLC

// The three CONDITIONAL "round to integral float" variants (ARMv8 VFP). Unlike VRINT{A,N,P,M}, these carry a
// normal condition code and live in the conditional VFP data-processing space. They differ in which rounding
// mode they use and whether they signal the Inexact exception:
//   R -- round per the FPSCR rounding mode                          (opc2=0110, op7=0)
//   Z -- round toward zero                                          (opc2=0110, op7=1)
//   X -- round per FPSCR, and raise Inexact when the result differs (opc2=0111, op7=0)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arm32VrintMode {
    R,
    Z,
    X,
}
impl Arm32VrintMode {
    // (opc2, op7) selector bits within the conditional VFP data-processing group
    pub fn selector_bits(self) -> (u32, u32) {
        match self {
            Self::R => (0b0110, 0),
            Self::Z => (0b0110, 1),
            Self::X => (0b0111, 0),
        }
    }
}
