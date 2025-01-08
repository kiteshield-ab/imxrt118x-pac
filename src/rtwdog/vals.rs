#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rcs {
    #[doc = "Unsuccessful"]
    RECONFIG = 0x0,
    #[doc = "Successful"]
    SUCCESS = 0x01,
}
impl Rcs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rcs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rcs {
    #[inline(always)]
    fn from(val: u8) -> Rcs {
        Rcs::from_bits(val)
    }
}
impl From<Rcs> for u8 {
    #[inline(always)]
    fn from(val: Rcs) -> u8 {
        Rcs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Tst {
    #[doc = "Disable WDOG Test mode"]
    DIS = 0x0,
    #[doc = "Enable WDOG User mode"]
    EN = 0x01,
    #[doc = "Enable WDOG Test mode"]
    ENABLES_2 = 0x02,
    #[doc = "Enable WDOG Test mode"]
    ENABLES_3 = 0x03,
}
impl Tst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Tst {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Tst {
    #[inline(always)]
    fn from(val: u8) -> Tst {
        Tst::from_bits(val)
    }
}
impl From<Tst> for u8 {
    #[inline(always)]
    fn from(val: Tst) -> u8 {
        Tst::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ulk {
    #[doc = "Locked"]
    LOCK = 0x0,
    #[doc = "Unlocked"]
    UNLOCK = 0x01,
}
impl Ulk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ulk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ulk {
    #[inline(always)]
    fn from(val: u8) -> Ulk {
        Ulk::from_bits(val)
    }
}
impl From<Ulk> for u8 {
    #[inline(always)]
    fn from(val: Ulk) -> u8 {
        Ulk::to_bits(val)
    }
}
