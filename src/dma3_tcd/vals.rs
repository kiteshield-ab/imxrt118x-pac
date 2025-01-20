#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bwc {
    #[doc = "No eDMA engine stalls"]
    NoStall = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W"]
    EngineStallsFour = 0x02,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W"]
    EngineStallsEight = 0x03,
}
impl Bwc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bwc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bwc {
    #[inline(always)]
    fn from(val: u8) -> Bwc {
        Bwc::from_bits(val)
    }
}
impl From<Bwc> for u8 {
    #[inline(always)]
    fn from(val: Bwc) -> u8 {
        Bwc::to_bits(val)
    }
}
