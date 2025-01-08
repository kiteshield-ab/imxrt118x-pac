#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LinregPwruploadDis {
    #[doc = "Internal pull-down enabled"]
    LINREG_PWRUPLOAD_DIS_0 = 0x0,
    #[doc = "Internal pull-down disabled"]
    LINREG_PWRUPLOAD_DIS_1 = 0x01,
}
impl LinregPwruploadDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LinregPwruploadDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LinregPwruploadDis {
    #[inline(always)]
    fn from(val: u8) -> LinregPwruploadDis {
        LinregPwruploadDis::from_bits(val)
    }
}
impl From<LinregPwruploadDis> for u8 {
    #[inline(always)]
    fn from(val: LinregPwruploadDis) -> u8 {
        LinregPwruploadDis::to_bits(val)
    }
}
