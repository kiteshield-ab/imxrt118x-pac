#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Feature(pub u16);
impl Feature {
    #[doc = "Basic implementation."]
    pub const FEATURE0: Self = Self(0x0);
    #[doc = "Protection registers implemented."]
    pub const FEATURE1: Self = Self(0x01);
}
impl Feature {
    pub const fn from_bits(val: u16) -> Feature {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("FEATURE0"),
            0x01 => f.write_str("FEATURE1"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Feature {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "FEATURE0"),
            0x01 => defmt::write!(f, "FEATURE1"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Feature {
    #[inline(always)]
    fn from(val: u16) -> Feature {
        Feature::from_bits(val)
    }
}
impl From<Feature> for u16 {
    #[inline(always)]
    fn from(val: Feature) -> u16 {
        Feature::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Irqc {
    #[doc = "Interrupt Status Flag (ISF) is disabled."]
    IRQC0 = 0x0,
    #[doc = "ISF flag and DMA request on rising edge."]
    IRQC1 = 0x01,
    #[doc = "ISF flag and DMA request on falling edge."]
    IRQC2 = 0x02,
    #[doc = "ISF flag and DMA request on either edge."]
    IRQC3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "ISF flag sets on rising edge."]
    IRQC5 = 0x05,
    #[doc = "ISF flag sets on falling edge."]
    IRQC6 = 0x06,
    #[doc = "ISF flag sets on either edge."]
    IRQC7 = 0x07,
    #[doc = "ISF flag and Interrupt when logic 0."]
    IRQC8 = 0x08,
    #[doc = "ISF flag and Interrupt on rising-edge."]
    IRQC9 = 0x09,
    #[doc = "ISF flag and Interrupt on falling-edge."]
    IRQC10 = 0x0a,
    #[doc = "ISF flag and Interrupt on either edge."]
    IRQC11 = 0x0b,
    #[doc = "ISF flag and Interrupt when logic 1."]
    IRQC12 = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Irqc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqc {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqc {
    #[inline(always)]
    fn from(val: u8) -> Irqc {
        Irqc::from_bits(val)
    }
}
impl From<Irqc> for u8 {
    #[inline(always)]
    fn from(val: Irqc) -> u8 {
        Irqc::to_bits(val)
    }
}
