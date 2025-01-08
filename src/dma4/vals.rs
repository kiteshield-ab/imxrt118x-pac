#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MpHrsHighHrs(pub u32);
impl MpHrsHighHrs {
    #[doc = "A hardware service request for the channel is not present"]
    pub const IDLE: Self = Self(0x0);
    #[doc = "A hardware service request for channel 0 is present"]
    pub const ACTIVE: Self = Self(0x01);
}
impl MpHrsHighHrs {
    pub const fn from_bits(val: u32) -> MpHrsHighHrs {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for MpHrsHighHrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("IDLE"),
            0x01 => f.write_str("ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpHrsHighHrs {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "IDLE"),
            0x01 => defmt::write!(f, "ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for MpHrsHighHrs {
    #[inline(always)]
    fn from(val: u32) -> MpHrsHighHrs {
        MpHrsHighHrs::from_bits(val)
    }
}
impl From<MpHrsHighHrs> for u32 {
    #[inline(always)]
    fn from(val: MpHrsHighHrs) -> u32 {
        MpHrsHighHrs::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct MpHrsLowHrs(pub u32);
impl MpHrsLowHrs {
    #[doc = "A hardware service request for the channel is not present"]
    pub const IDLE: Self = Self(0x0);
    #[doc = "A hardware service request for channel 0 is present"]
    pub const ACTIVE: Self = Self(0x01);
}
impl MpHrsLowHrs {
    pub const fn from_bits(val: u32) -> MpHrsLowHrs {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for MpHrsLowHrs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("IDLE"),
            0x01 => f.write_str("ACTIVE"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpHrsLowHrs {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "IDLE"),
            0x01 => defmt::write!(f, "ACTIVE"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for MpHrsLowHrs {
    #[inline(always)]
    fn from(val: u32) -> MpHrsLowHrs {
        MpHrsLowHrs::from_bits(val)
    }
}
impl From<MpHrsLowHrs> for u32 {
    #[inline(always)]
    fn from(val: MpHrsLowHrs) -> u32 {
        MpHrsLowHrs::to_bits(val)
    }
}
