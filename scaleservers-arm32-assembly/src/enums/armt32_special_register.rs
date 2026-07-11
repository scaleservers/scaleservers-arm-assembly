// Copyright (c) Scaleservers LLC

#[derive(Debug, PartialEq)]
pub enum ArmT32SpecialRegister {
    Apsr,
    Iapsr,
    Eapsr,
    Xpsr,
    Ipsr,
    Epsr,
    Iepsr,
    //
    Msp,
    Psp,
    Primask,
    Control,
    //
    Reserved(u8)
}

impl ArmT32SpecialRegister {
    pub fn as_operand_bits(&self) -> u8 {
        match self {
            Self::Apsr    => 0b00000_000,
            Self::Iapsr   => 0b00000_001,
            Self::Eapsr   => 0b00000_010,
            Self::Xpsr    => 0b00000_011,
            Self::Ipsr    => 0b00000_101,
            Self::Epsr    => 0b00000_110,
            Self::Iepsr   => 0b00000_111,
            //
            Self::Msp     => 0b00001_000,
            Self::Psp     => 0b00001_001,
            Self::Primask => 0b00010_000,
            Self::Control => 0b00010_100,
            //
            Self::Reserved(bits) => *bits,
        }
    }

    pub fn from_operand_bits(bits: u8) -> Self {
        match bits {
            0b00000_000 => Self::Apsr,
            0b00000_001 => Self::Iapsr,
            0b00000_010 => Self::Eapsr,
            0b00000_011 => Self::Xpsr,
            0b00000_101 => Self::Ipsr,
            0b00000_110 => Self::Epsr,
            0b00000_111 => Self::Iepsr,
            //
            0b00001_000 => Self::Msp,
            0b00001_001 => Self::Psp,
            0b00010_000 => Self::Primask,
            0b00010_100 => Self::Control,
            //
            _ => Self::Reserved(bits),
        }
    }
}
