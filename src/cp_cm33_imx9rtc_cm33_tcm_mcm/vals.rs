#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReccDis {
    #[doc = "Enable"]
    Enable = 0x0,
    #[doc = "Disable"]
    Disable = 0x01,
}
impl ReccDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReccDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReccDis {
    #[inline(always)]
    fn from(val: u8) -> ReccDis {
        ReccDis::from_bits(val)
    }
}
impl From<ReccDis> for u8 {
    #[inline(always)]
    fn from(val: ReccDis) -> u8 {
        ReccDis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WeccDis {
    #[doc = "Enable"]
    Enable = 0x0,
    #[doc = "Disable"]
    Disable = 0x01,
}
impl WeccDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WeccDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WeccDis {
    #[inline(always)]
    fn from(val: u8) -> WeccDis {
        WeccDis::from_bits(val)
    }
}
impl From<WeccDis> for u8 {
    #[inline(always)]
    fn from(val: WeccDis) -> u8 {
        WeccDis::to_bits(val)
    }
}
