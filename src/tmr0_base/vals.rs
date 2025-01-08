#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcd {
    #[doc = "Reference Clock has not been detected as active. Registers in timer clock domain are not allowed to be accessed; reads return 0, writes are ignored."]
    REF_CLK_NOT_ACTIVE = 0x0,
    #[doc = "Reference Clock has been detected as active. Registers in timer clock domain are allowed to be accessed."]
    REF_CLK_ACTIVE = 0x01,
}
impl Rcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcd {
    #[inline(always)]
    fn from(val: u8) -> Rcd {
        Rcd::from_bits(val)
    }
}
impl From<Rcd> for u8 {
    #[inline(always)]
    fn from(val: Rcd) -> u8 {
        Rcd::to_bits(val)
    }
}
