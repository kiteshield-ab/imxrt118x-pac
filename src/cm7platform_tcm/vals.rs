#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MemPwrStEn {
    #[doc = "Memory power status will not be considered when determining slice power status."]
    EN = 0x0,
    #[doc = "Memory power status will be considered when determining slice power status."]
    DIS = 0x01,
}
impl MemPwrStEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MemPwrStEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MemPwrStEn {
    #[inline(always)]
    fn from(val: u8) -> MemPwrStEn {
        MemPwrStEn::from_bits(val)
    }
}
impl From<MemPwrStEn> for u8 {
    #[inline(always)]
    fn from(val: MemPwrStEn) -> u8 {
        MemPwrStEn::to_bits(val)
    }
}
