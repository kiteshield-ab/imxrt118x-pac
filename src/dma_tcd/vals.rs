#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pal {
    #[doc = "User protection level for DMA transfers"]
    UserProtection = 0x0,
    #[doc = "Privileged protection level for DMA transfers"]
    PrivilegedProtection = 0x01,
}
impl Pal {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pal {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pal {
    #[inline(always)]
    fn from(val: u8) -> Pal {
        Pal::from_bits(val)
    }
}
impl From<Pal> for u8 {
    #[inline(always)]
    fn from(val: Pal) -> u8 {
        Pal::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Size {
    #[doc = "8-bit"]
    EightBit = 0x0,
    #[doc = "16-bit"]
    SixteenBit = 0x01,
    #[doc = "32-bit"]
    ThirtytwoBit = 0x02,
    #[doc = "64-bit"]
    SixtyfourBit = 0x03,
    #[doc = "16-byte"]
    SixteenByte = 0x04,
    #[doc = "32-byte"]
    ThirtytwoByte = 0x05,
    #[doc = "64-byte"]
    SixtyfourByte = 0x06,
    _RESERVED_7 = 0x07,
}
impl Size {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Size {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Size {
    #[inline(always)]
    fn from(val: u8) -> Size {
        Size::from_bits(val)
    }
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(val: Size) -> u8 {
        Size::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Smod(pub u8);
impl Smod {
    #[doc = "Source address modulo feature disabled"]
    pub const DISABLE: Self = Self(0x0);
    #[doc = "Source address modulo feature enabled for any non-zero value \\[1-31\\]"]
    pub const ENABLE: Self = Self(0x01);
}
impl Smod {
    pub const fn from_bits(val: u8) -> Smod {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Smod {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("DISABLE"),
            0x01 => f.write_str("ENABLE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smod {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "DISABLE"),
            0x01 => defmt::write!(f, "ENABLE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Smod {
    #[inline(always)]
    fn from(val: u8) -> Smod {
        Smod::from_bits(val)
    }
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(val: Smod) -> u8 {
        Smod::to_bits(val)
    }
}
