#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr0 {
    #[doc = "OCRAM access error does not happen on OCRAM bank0."]
    AddrErr00 = 0x0,
    #[doc = "OCRAM access error happens on OCRAM bank0."]
    AddrErr01 = 0x01,
}
impl AddrErr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr0 {
    #[inline(always)]
    fn from(val: u8) -> AddrErr0 {
        AddrErr0::from_bits(val)
    }
}
impl From<AddrErr0> for u8 {
    #[inline(always)]
    fn from(val: AddrErr0) -> u8 {
        AddrErr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr0SigEn {
    #[doc = "Disabled"]
    AddrErr0SigEn0 = 0x0,
    #[doc = "Enabled"]
    AddrErr0SigEn1 = 0x01,
}
impl AddrErr0SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr0SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr0SigEn {
    #[inline(always)]
    fn from(val: u8) -> AddrErr0SigEn {
        AddrErr0SigEn::from_bits(val)
    }
}
impl From<AddrErr0SigEn> for u8 {
    #[inline(always)]
    fn from(val: AddrErr0SigEn) -> u8 {
        AddrErr0SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr0StatEn {
    #[doc = "Disabled"]
    AddrErr0StatEn0 = 0x0,
    #[doc = "Enabled"]
    AddrErr0StatEn1 = 0x01,
}
impl AddrErr0StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr0StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr0StatEn {
    #[inline(always)]
    fn from(val: u8) -> AddrErr0StatEn {
        AddrErr0StatEn::from_bits(val)
    }
}
impl From<AddrErr0StatEn> for u8 {
    #[inline(always)]
    fn from(val: AddrErr0StatEn) -> u8 {
        AddrErr0StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr1 {
    #[doc = "OCRAM access error does not happen on OCRAM bank1."]
    AddrErr10 = 0x0,
    #[doc = "OCRAM access error happens on OCRAM bank1."]
    AddrErr11 = 0x01,
}
impl AddrErr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr1 {
    #[inline(always)]
    fn from(val: u8) -> AddrErr1 {
        AddrErr1::from_bits(val)
    }
}
impl From<AddrErr1> for u8 {
    #[inline(always)]
    fn from(val: AddrErr1) -> u8 {
        AddrErr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr1SigEn {
    #[doc = "Disabled"]
    AddrErr1SigEn0 = 0x0,
    #[doc = "Enabled"]
    AddrErr1SigEn1 = 0x01,
}
impl AddrErr1SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr1SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr1SigEn {
    #[inline(always)]
    fn from(val: u8) -> AddrErr1SigEn {
        AddrErr1SigEn::from_bits(val)
    }
}
impl From<AddrErr1SigEn> for u8 {
    #[inline(always)]
    fn from(val: AddrErr1SigEn) -> u8 {
        AddrErr1SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr1StatEn {
    #[doc = "Disabled"]
    AddrErr1StatEn0 = 0x0,
    #[doc = "Enabled"]
    AddrErr1StatEn1 = 0x01,
}
impl AddrErr1StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr1StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr1StatEn {
    #[inline(always)]
    fn from(val: u8) -> AddrErr1StatEn {
        AddrErr1StatEn::from_bits(val)
    }
}
impl From<AddrErr1StatEn> for u8 {
    #[inline(always)]
    fn from(val: AddrErr1StatEn) -> u8 {
        AddrErr1StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr2 {
    #[doc = "OCRAM access error does not happen on OCRAM bank2."]
    AddrErr20 = 0x0,
    #[doc = "OCRAM access error happens on OCRAM bank2."]
    AddrErr21 = 0x01,
}
impl AddrErr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr2 {
    #[inline(always)]
    fn from(val: u8) -> AddrErr2 {
        AddrErr2::from_bits(val)
    }
}
impl From<AddrErr2> for u8 {
    #[inline(always)]
    fn from(val: AddrErr2) -> u8 {
        AddrErr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr2SigEn {
    #[doc = "Disabled"]
    AddrErr2SigEn0 = 0x0,
    #[doc = "Enabled"]
    AddrErr2SigEn1 = 0x01,
}
impl AddrErr2SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr2SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr2SigEn {
    #[inline(always)]
    fn from(val: u8) -> AddrErr2SigEn {
        AddrErr2SigEn::from_bits(val)
    }
}
impl From<AddrErr2SigEn> for u8 {
    #[inline(always)]
    fn from(val: AddrErr2SigEn) -> u8 {
        AddrErr2SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr2StatEn {
    #[doc = "Disabled"]
    AddrErr2StatEn0 = 0x0,
    #[doc = "Enabled"]
    AddrErr2StatEn1 = 0x01,
}
impl AddrErr2StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr2StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr2StatEn {
    #[inline(always)]
    fn from(val: u8) -> AddrErr2StatEn {
        AddrErr2StatEn::from_bits(val)
    }
}
impl From<AddrErr2StatEn> for u8 {
    #[inline(always)]
    fn from(val: AddrErr2StatEn) -> u8 {
        AddrErr2StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr3 {
    #[doc = "OCRAM access error does not happen on OCRAM bank3."]
    AddrErr30 = 0x0,
    #[doc = "OCRAM access error happens on OCRAM bank3."]
    AddrErr31 = 0x01,
}
impl AddrErr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr3 {
    #[inline(always)]
    fn from(val: u8) -> AddrErr3 {
        AddrErr3::from_bits(val)
    }
}
impl From<AddrErr3> for u8 {
    #[inline(always)]
    fn from(val: AddrErr3) -> u8 {
        AddrErr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr3SigEn {
    #[doc = "Disabled"]
    AddrErr3SigEn0 = 0x0,
    #[doc = "Enabled"]
    AddrErr3SigEn1 = 0x01,
}
impl AddrErr3SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr3SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr3SigEn {
    #[inline(always)]
    fn from(val: u8) -> AddrErr3SigEn {
        AddrErr3SigEn::from_bits(val)
    }
}
impl From<AddrErr3SigEn> for u8 {
    #[inline(always)]
    fn from(val: AddrErr3SigEn) -> u8 {
        AddrErr3SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr3StatEn {
    #[doc = "Disabled"]
    AddrErr3StatEn0 = 0x0,
    #[doc = "Enabled"]
    AddrErr3StatEn1 = 0x01,
}
impl AddrErr3StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr3StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr3StatEn {
    #[inline(always)]
    fn from(val: u8) -> AddrErr3StatEn {
        AddrErr3StatEn::from_bits(val)
    }
}
impl From<AddrErr3StatEn> for u8 {
    #[inline(always)]
    fn from(val: AddrErr3StatEn) -> u8 {
        AddrErr3StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccEn {
    #[doc = "Disable."]
    EccEn0 = 0x0,
    #[doc = "Enable."]
    EccEn1 = 0x01,
}
impl EccEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccEn {
    #[inline(always)]
    fn from(val: u8) -> EccEn {
        EccEn::from_bits(val)
    }
}
impl From<EccEn> for u8 {
    #[inline(always)]
    fn from(val: EccEn) -> u8 {
        EccEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr0 {
    #[doc = "Multiple bits error does not happen on OCRAM bank0."]
    MultiErr00 = 0x0,
    #[doc = "Multiple bits error happens on OCRAM bank0."]
    MultiErr01 = 0x01,
}
impl MultiErr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr0 {
    #[inline(always)]
    fn from(val: u8) -> MultiErr0 {
        MultiErr0::from_bits(val)
    }
}
impl From<MultiErr0> for u8 {
    #[inline(always)]
    fn from(val: MultiErr0) -> u8 {
        MultiErr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr0SigEn {
    #[doc = "Disabled"]
    MultiErr0SigEn0 = 0x0,
    #[doc = "Enabled"]
    MultiErr0SigEn1 = 0x01,
}
impl MultiErr0SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr0SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr0SigEn {
    #[inline(always)]
    fn from(val: u8) -> MultiErr0SigEn {
        MultiErr0SigEn::from_bits(val)
    }
}
impl From<MultiErr0SigEn> for u8 {
    #[inline(always)]
    fn from(val: MultiErr0SigEn) -> u8 {
        MultiErr0SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr0StatEn {
    #[doc = "Disabled"]
    MultiErr0StatEn0 = 0x0,
    #[doc = "Enabled"]
    MultiErr0StatEn1 = 0x01,
}
impl MultiErr0StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr0StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr0StatEn {
    #[inline(always)]
    fn from(val: u8) -> MultiErr0StatEn {
        MultiErr0StatEn::from_bits(val)
    }
}
impl From<MultiErr0StatEn> for u8 {
    #[inline(always)]
    fn from(val: MultiErr0StatEn) -> u8 {
        MultiErr0StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr1 {
    #[doc = "Multiple bits error does not happen on OCRAM bank1."]
    MultiErr10 = 0x0,
    #[doc = "Multiple bits error happens on OCRAM bank1."]
    MultiErr11 = 0x01,
}
impl MultiErr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr1 {
    #[inline(always)]
    fn from(val: u8) -> MultiErr1 {
        MultiErr1::from_bits(val)
    }
}
impl From<MultiErr1> for u8 {
    #[inline(always)]
    fn from(val: MultiErr1) -> u8 {
        MultiErr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr1SigEn {
    #[doc = "Disabled"]
    MultiErr1SigEn0 = 0x0,
    #[doc = "Enabled"]
    MultiErr1SigEn1 = 0x01,
}
impl MultiErr1SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr1SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr1SigEn {
    #[inline(always)]
    fn from(val: u8) -> MultiErr1SigEn {
        MultiErr1SigEn::from_bits(val)
    }
}
impl From<MultiErr1SigEn> for u8 {
    #[inline(always)]
    fn from(val: MultiErr1SigEn) -> u8 {
        MultiErr1SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr1StatEn {
    #[doc = "Disabled"]
    MultiErr1StatEn0 = 0x0,
    #[doc = "Enabled"]
    MultiErr1StatEn1 = 0x01,
}
impl MultiErr1StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr1StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr1StatEn {
    #[inline(always)]
    fn from(val: u8) -> MultiErr1StatEn {
        MultiErr1StatEn::from_bits(val)
    }
}
impl From<MultiErr1StatEn> for u8 {
    #[inline(always)]
    fn from(val: MultiErr1StatEn) -> u8 {
        MultiErr1StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr2 {
    #[doc = "Multiple bits error does not happen on OCRAM bank2."]
    MultiErr20 = 0x0,
    #[doc = "Multiple bits error happens on OCRAM bank2."]
    MultiErr21 = 0x01,
}
impl MultiErr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr2 {
    #[inline(always)]
    fn from(val: u8) -> MultiErr2 {
        MultiErr2::from_bits(val)
    }
}
impl From<MultiErr2> for u8 {
    #[inline(always)]
    fn from(val: MultiErr2) -> u8 {
        MultiErr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr2SigEn {
    #[doc = "Disabled"]
    MultiErr2SigEn0 = 0x0,
    #[doc = "Enabled"]
    MultiErr2SigEn1 = 0x01,
}
impl MultiErr2SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr2SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr2SigEn {
    #[inline(always)]
    fn from(val: u8) -> MultiErr2SigEn {
        MultiErr2SigEn::from_bits(val)
    }
}
impl From<MultiErr2SigEn> for u8 {
    #[inline(always)]
    fn from(val: MultiErr2SigEn) -> u8 {
        MultiErr2SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr2StatEn {
    #[doc = "Disabled"]
    MultiErr2StatEn0 = 0x0,
    #[doc = "Enabled"]
    MultiErr2StatEn1 = 0x01,
}
impl MultiErr2StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr2StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr2StatEn {
    #[inline(always)]
    fn from(val: u8) -> MultiErr2StatEn {
        MultiErr2StatEn::from_bits(val)
    }
}
impl From<MultiErr2StatEn> for u8 {
    #[inline(always)]
    fn from(val: MultiErr2StatEn) -> u8 {
        MultiErr2StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr3 {
    #[doc = "Multiple bits error does not happen on OCRAM bank3."]
    MultiErr30 = 0x0,
    #[doc = "Multiple bits error happens on OCRAM bank3."]
    MultiErr31 = 0x01,
}
impl MultiErr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr3 {
    #[inline(always)]
    fn from(val: u8) -> MultiErr3 {
        MultiErr3::from_bits(val)
    }
}
impl From<MultiErr3> for u8 {
    #[inline(always)]
    fn from(val: MultiErr3) -> u8 {
        MultiErr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr3SigEn {
    #[doc = "Disabled"]
    MultiErr3SigEn0 = 0x0,
    #[doc = "Enabled"]
    MultiErr3SigEn1 = 0x01,
}
impl MultiErr3SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr3SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr3SigEn {
    #[inline(always)]
    fn from(val: u8) -> MultiErr3SigEn {
        MultiErr3SigEn::from_bits(val)
    }
}
impl From<MultiErr3SigEn> for u8 {
    #[inline(always)]
    fn from(val: MultiErr3SigEn) -> u8 {
        MultiErr3SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MultiErr3StatEn {
    #[doc = "Disabled"]
    MultiErr3StatEn0 = 0x0,
    #[doc = "Enabled"]
    MultiErr3StatEn1 = 0x01,
}
impl MultiErr3StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MultiErr3StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MultiErr3StatEn {
    #[inline(always)]
    fn from(val: u8) -> MultiErr3StatEn {
        MultiErr3StatEn::from_bits(val)
    }
}
impl From<MultiErr3StatEn> for u8 {
    #[inline(always)]
    fn from(val: MultiErr3StatEn) -> u8 {
        MultiErr3StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadAddrPipeEn {
    #[doc = "Disable."]
    ReadAddrPipeEn0 = 0x0,
    #[doc = "Enable."]
    ReadAddrPipeEn1 = 0x01,
}
impl ReadAddrPipeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReadAddrPipeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReadAddrPipeEn {
    #[inline(always)]
    fn from(val: u8) -> ReadAddrPipeEn {
        ReadAddrPipeEn::from_bits(val)
    }
}
impl From<ReadAddrPipeEn> for u8 {
    #[inline(always)]
    fn from(val: ReadAddrPipeEn) -> u8 {
        ReadAddrPipeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadAddrPipePending {
    #[doc = "No update pending status for READ_ADDR_PIPE_EN."]
    ReadAddrPipePending0 = 0x0,
    #[doc = "When READ_ADDR_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller."]
    ReadAddrPipePending1 = 0x01,
}
impl ReadAddrPipePending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReadAddrPipePending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReadAddrPipePending {
    #[inline(always)]
    fn from(val: u8) -> ReadAddrPipePending {
        ReadAddrPipePending::from_bits(val)
    }
}
impl From<ReadAddrPipePending> for u8 {
    #[inline(always)]
    fn from(val: ReadAddrPipePending) -> u8 {
        ReadAddrPipePending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadDataWaitEn {
    #[doc = "Disable."]
    ReadDataWaitEn0 = 0x0,
    #[doc = "Enable."]
    ReadDataWaitEn1 = 0x01,
}
impl ReadDataWaitEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReadDataWaitEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReadDataWaitEn {
    #[inline(always)]
    fn from(val: u8) -> ReadDataWaitEn {
        ReadDataWaitEn::from_bits(val)
    }
}
impl From<ReadDataWaitEn> for u8 {
    #[inline(always)]
    fn from(val: ReadDataWaitEn) -> u8 {
        ReadDataWaitEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReadDataWaitPending {
    #[doc = "No update pending status for READ_DATA_WAIT_EN."]
    ReadDataWaitPending0 = 0x0,
    #[doc = "When READ_DATA_WAIT_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller."]
    ReadDataWaitPending1 = 0x01,
}
impl ReadDataWaitPending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ReadDataWaitPending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ReadDataWaitPending {
    #[inline(always)]
    fn from(val: u8) -> ReadDataWaitPending {
        ReadDataWaitPending::from_bits(val)
    }
}
impl From<ReadDataWaitPending> for u8 {
    #[inline(always)]
    fn from(val: ReadDataWaitPending) -> u8 {
        ReadDataWaitPending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr0 {
    #[doc = "Single bit error does not happen on OCRAM bank0."]
    SingleErr00 = 0x0,
    #[doc = "Single bit error happens on OCRAM bank0."]
    SingleErr01 = 0x01,
}
impl SingleErr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr0 {
    #[inline(always)]
    fn from(val: u8) -> SingleErr0 {
        SingleErr0::from_bits(val)
    }
}
impl From<SingleErr0> for u8 {
    #[inline(always)]
    fn from(val: SingleErr0) -> u8 {
        SingleErr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr0SigEn {
    #[doc = "Disabled"]
    SingleErr0SigEn0 = 0x0,
    #[doc = "Enabled"]
    SingleErr0SigEn1 = 0x01,
}
impl SingleErr0SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr0SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr0SigEn {
    #[inline(always)]
    fn from(val: u8) -> SingleErr0SigEn {
        SingleErr0SigEn::from_bits(val)
    }
}
impl From<SingleErr0SigEn> for u8 {
    #[inline(always)]
    fn from(val: SingleErr0SigEn) -> u8 {
        SingleErr0SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr0StatEn {
    #[doc = "Disabled"]
    SingleErr0StatEn0 = 0x0,
    #[doc = "Enabled"]
    SingleErr0StatEn1 = 0x01,
}
impl SingleErr0StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr0StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr0StatEn {
    #[inline(always)]
    fn from(val: u8) -> SingleErr0StatEn {
        SingleErr0StatEn::from_bits(val)
    }
}
impl From<SingleErr0StatEn> for u8 {
    #[inline(always)]
    fn from(val: SingleErr0StatEn) -> u8 {
        SingleErr0StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr1 {
    #[doc = "Single bit error does not happen on OCRAM bank1."]
    SingleErr10 = 0x0,
    #[doc = "Single bit error happens on OCRAM bank1."]
    SingleErr11 = 0x01,
}
impl SingleErr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr1 {
    #[inline(always)]
    fn from(val: u8) -> SingleErr1 {
        SingleErr1::from_bits(val)
    }
}
impl From<SingleErr1> for u8 {
    #[inline(always)]
    fn from(val: SingleErr1) -> u8 {
        SingleErr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr1SigEn {
    #[doc = "Disabled"]
    SingleErr1SigEn0 = 0x0,
    #[doc = "Enabled"]
    SingleErr1SigEn1 = 0x01,
}
impl SingleErr1SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr1SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr1SigEn {
    #[inline(always)]
    fn from(val: u8) -> SingleErr1SigEn {
        SingleErr1SigEn::from_bits(val)
    }
}
impl From<SingleErr1SigEn> for u8 {
    #[inline(always)]
    fn from(val: SingleErr1SigEn) -> u8 {
        SingleErr1SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr1StatEn {
    #[doc = "Disabled"]
    SingleErr1StatEn0 = 0x0,
    #[doc = "Enabled"]
    SingleErr1StatEn1 = 0x01,
}
impl SingleErr1StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr1StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr1StatEn {
    #[inline(always)]
    fn from(val: u8) -> SingleErr1StatEn {
        SingleErr1StatEn::from_bits(val)
    }
}
impl From<SingleErr1StatEn> for u8 {
    #[inline(always)]
    fn from(val: SingleErr1StatEn) -> u8 {
        SingleErr1StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr2 {
    #[doc = "Single bit error does not happen on OCRAM bank2."]
    SingleErr20 = 0x0,
    #[doc = "Single bit error happens on OCRAM bank2."]
    SingleErr21 = 0x01,
}
impl SingleErr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr2 {
    #[inline(always)]
    fn from(val: u8) -> SingleErr2 {
        SingleErr2::from_bits(val)
    }
}
impl From<SingleErr2> for u8 {
    #[inline(always)]
    fn from(val: SingleErr2) -> u8 {
        SingleErr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr2SigEn {
    #[doc = "Disabled"]
    SingleErr2SigEn0 = 0x0,
    #[doc = "Enabled"]
    SingleErr2SigEn1 = 0x01,
}
impl SingleErr2SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr2SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr2SigEn {
    #[inline(always)]
    fn from(val: u8) -> SingleErr2SigEn {
        SingleErr2SigEn::from_bits(val)
    }
}
impl From<SingleErr2SigEn> for u8 {
    #[inline(always)]
    fn from(val: SingleErr2SigEn) -> u8 {
        SingleErr2SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr2StatEn {
    #[doc = "Disabled"]
    SingleErr2StatEn0 = 0x0,
    #[doc = "Enabled"]
    SingleErr2StatEn1 = 0x01,
}
impl SingleErr2StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr2StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr2StatEn {
    #[inline(always)]
    fn from(val: u8) -> SingleErr2StatEn {
        SingleErr2StatEn::from_bits(val)
    }
}
impl From<SingleErr2StatEn> for u8 {
    #[inline(always)]
    fn from(val: SingleErr2StatEn) -> u8 {
        SingleErr2StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr3 {
    #[doc = "Single bit error does not happen on OCRAM bank3."]
    SingleErr30 = 0x0,
    #[doc = "Single bit error happens on OCRAM bank3."]
    SingleErr31 = 0x01,
}
impl SingleErr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr3 {
    #[inline(always)]
    fn from(val: u8) -> SingleErr3 {
        SingleErr3::from_bits(val)
    }
}
impl From<SingleErr3> for u8 {
    #[inline(always)]
    fn from(val: SingleErr3) -> u8 {
        SingleErr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr3SigEn {
    #[doc = "Disabled"]
    SingleErr3SigEn0 = 0x0,
    #[doc = "Enabled"]
    SingleErr3SigEn1 = 0x01,
}
impl SingleErr3SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr3SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr3SigEn {
    #[inline(always)]
    fn from(val: u8) -> SingleErr3SigEn {
        SingleErr3SigEn::from_bits(val)
    }
}
impl From<SingleErr3SigEn> for u8 {
    #[inline(always)]
    fn from(val: SingleErr3SigEn) -> u8 {
        SingleErr3SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SingleErr3StatEn {
    #[doc = "Disabled"]
    SingleErr3StatEn0 = 0x0,
    #[doc = "Enabled"]
    SingleErr3StatEn1 = 0x01,
}
impl SingleErr3StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SingleErr3StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SingleErr3StatEn {
    #[inline(always)]
    fn from(val: u8) -> SingleErr3StatEn {
        SingleErr3StatEn::from_bits(val)
    }
}
impl From<SingleErr3StatEn> for u8 {
    #[inline(always)]
    fn from(val: SingleErr3StatEn) -> u8 {
        SingleErr3StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr0 {
    #[doc = "AXI strobe error does not happen on OCRAM bank0."]
    StrbErr00 = 0x0,
    #[doc = "AXI strobe error happens on OCRAM bank0."]
    StrbErr01 = 0x01,
}
impl StrbErr0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr0 {
    #[inline(always)]
    fn from(val: u8) -> StrbErr0 {
        StrbErr0::from_bits(val)
    }
}
impl From<StrbErr0> for u8 {
    #[inline(always)]
    fn from(val: StrbErr0) -> u8 {
        StrbErr0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr0SigEn {
    #[doc = "Disabled"]
    StrbErr0SigEn0 = 0x0,
    #[doc = "Enabled"]
    StrbErr0SigEn1 = 0x01,
}
impl StrbErr0SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr0SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr0SigEn {
    #[inline(always)]
    fn from(val: u8) -> StrbErr0SigEn {
        StrbErr0SigEn::from_bits(val)
    }
}
impl From<StrbErr0SigEn> for u8 {
    #[inline(always)]
    fn from(val: StrbErr0SigEn) -> u8 {
        StrbErr0SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr0StatEn {
    #[doc = "Disabled"]
    StrbErr0StatEn0 = 0x0,
    #[doc = "Enabled"]
    StrbErr0StatEn1 = 0x01,
}
impl StrbErr0StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr0StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr0StatEn {
    #[inline(always)]
    fn from(val: u8) -> StrbErr0StatEn {
        StrbErr0StatEn::from_bits(val)
    }
}
impl From<StrbErr0StatEn> for u8 {
    #[inline(always)]
    fn from(val: StrbErr0StatEn) -> u8 {
        StrbErr0StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr1 {
    #[doc = "AXI strobe error does not happen on OCRAM bank1."]
    StrbErr10 = 0x0,
    #[doc = "AXI strobe error happens on OCRAM bank1."]
    StrbErr11 = 0x01,
}
impl StrbErr1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr1 {
    #[inline(always)]
    fn from(val: u8) -> StrbErr1 {
        StrbErr1::from_bits(val)
    }
}
impl From<StrbErr1> for u8 {
    #[inline(always)]
    fn from(val: StrbErr1) -> u8 {
        StrbErr1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr1SigEn {
    #[doc = "Disabled"]
    StrbErr1SigEn0 = 0x0,
    #[doc = "Enabled"]
    StrbErr1SigEn1 = 0x01,
}
impl StrbErr1SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr1SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr1SigEn {
    #[inline(always)]
    fn from(val: u8) -> StrbErr1SigEn {
        StrbErr1SigEn::from_bits(val)
    }
}
impl From<StrbErr1SigEn> for u8 {
    #[inline(always)]
    fn from(val: StrbErr1SigEn) -> u8 {
        StrbErr1SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr1StatEn {
    #[doc = "Disabled"]
    StrbErr1StatEn0 = 0x0,
    #[doc = "Enabled"]
    StrbErr1StatEn1 = 0x01,
}
impl StrbErr1StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr1StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr1StatEn {
    #[inline(always)]
    fn from(val: u8) -> StrbErr1StatEn {
        StrbErr1StatEn::from_bits(val)
    }
}
impl From<StrbErr1StatEn> for u8 {
    #[inline(always)]
    fn from(val: StrbErr1StatEn) -> u8 {
        StrbErr1StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr2 {
    #[doc = "AXI strobe error does not happen on OCRAM bank2."]
    StrbErr20 = 0x0,
    #[doc = "AXI strobe error happens on OCRAM bank2."]
    StrbErr21 = 0x01,
}
impl StrbErr2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr2 {
    #[inline(always)]
    fn from(val: u8) -> StrbErr2 {
        StrbErr2::from_bits(val)
    }
}
impl From<StrbErr2> for u8 {
    #[inline(always)]
    fn from(val: StrbErr2) -> u8 {
        StrbErr2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr2SigEn {
    #[doc = "Disabled"]
    StrbErr2SigEn0 = 0x0,
    #[doc = "Enabled"]
    StrbErr2SigEn1 = 0x01,
}
impl StrbErr2SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr2SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr2SigEn {
    #[inline(always)]
    fn from(val: u8) -> StrbErr2SigEn {
        StrbErr2SigEn::from_bits(val)
    }
}
impl From<StrbErr2SigEn> for u8 {
    #[inline(always)]
    fn from(val: StrbErr2SigEn) -> u8 {
        StrbErr2SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr2StatEn {
    #[doc = "Disabled"]
    StrbErr2StatEn0 = 0x0,
    #[doc = "Enabled"]
    StrbErr2StatEn1 = 0x01,
}
impl StrbErr2StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr2StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr2StatEn {
    #[inline(always)]
    fn from(val: u8) -> StrbErr2StatEn {
        StrbErr2StatEn::from_bits(val)
    }
}
impl From<StrbErr2StatEn> for u8 {
    #[inline(always)]
    fn from(val: StrbErr2StatEn) -> u8 {
        StrbErr2StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr3 {
    #[doc = "AXI strobe error does not happen on OCRAM bank3."]
    StrbErr30 = 0x0,
    #[doc = "AXI strobe error happens on OCRAM bank3."]
    StrbErr31 = 0x01,
}
impl StrbErr3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr3 {
    #[inline(always)]
    fn from(val: u8) -> StrbErr3 {
        StrbErr3::from_bits(val)
    }
}
impl From<StrbErr3> for u8 {
    #[inline(always)]
    fn from(val: StrbErr3) -> u8 {
        StrbErr3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr3SigEn {
    #[doc = "Disabled"]
    StrbErr3SigEn0 = 0x0,
    #[doc = "Enabled"]
    StrbErr3SigEn1 = 0x01,
}
impl StrbErr3SigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr3SigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr3SigEn {
    #[inline(always)]
    fn from(val: u8) -> StrbErr3SigEn {
        StrbErr3SigEn::from_bits(val)
    }
}
impl From<StrbErr3SigEn> for u8 {
    #[inline(always)]
    fn from(val: StrbErr3SigEn) -> u8 {
        StrbErr3SigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StrbErr3StatEn {
    #[doc = "Disabled"]
    StrbErr3StatEn0 = 0x0,
    #[doc = "Enabled"]
    StrbErr3StatEn1 = 0x01,
}
impl StrbErr3StatEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StrbErr3StatEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StrbErr3StatEn {
    #[inline(always)]
    fn from(val: u8) -> StrbErr3StatEn {
        StrbErr3StatEn::from_bits(val)
    }
}
impl From<StrbErr3StatEn> for u8 {
    #[inline(always)]
    fn from(val: StrbErr3StatEn) -> u8 {
        StrbErr3StatEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WriteAddrPipeEn {
    #[doc = "Disable."]
    WriteAddrPipeEn0 = 0x0,
    #[doc = "Enable."]
    WriteAddrPipeEn1 = 0x01,
}
impl WriteAddrPipeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WriteAddrPipeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WriteAddrPipeEn {
    #[inline(always)]
    fn from(val: u8) -> WriteAddrPipeEn {
        WriteAddrPipeEn::from_bits(val)
    }
}
impl From<WriteAddrPipeEn> for u8 {
    #[inline(always)]
    fn from(val: WriteAddrPipeEn) -> u8 {
        WriteAddrPipeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WriteAddrPipePending {
    #[doc = "No update pending status for WRITE_ADDR_PIPE_EN."]
    WriteAddrPipePending0 = 0x0,
    #[doc = "When WRITE_ADDR_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller."]
    WriteAddrPipePending1 = 0x01,
}
impl WriteAddrPipePending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WriteAddrPipePending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WriteAddrPipePending {
    #[inline(always)]
    fn from(val: u8) -> WriteAddrPipePending {
        WriteAddrPipePending::from_bits(val)
    }
}
impl From<WriteAddrPipePending> for u8 {
    #[inline(always)]
    fn from(val: WriteAddrPipePending) -> u8 {
        WriteAddrPipePending::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WriteDataPipeEn {
    #[doc = "Disable."]
    WriteDataPipeEn0 = 0x0,
    #[doc = "Enable."]
    WriteDataPipeEn1 = 0x01,
}
impl WriteDataPipeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WriteDataPipeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WriteDataPipeEn {
    #[inline(always)]
    fn from(val: u8) -> WriteDataPipeEn {
        WriteDataPipeEn::from_bits(val)
    }
}
impl From<WriteDataPipeEn> for u8 {
    #[inline(always)]
    fn from(val: WriteDataPipeEn) -> u8 {
        WriteDataPipeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WriteDataPipePending {
    #[doc = "No update pending status for WRITE_DATA_PIPE_EN."]
    WriteDataPipePending0 = 0x0,
    #[doc = "When WRITE_DATA_PIPE_EN register bit is changed, this register bit will be set until the new setup becomes valid in the controller."]
    WriteDataPipePending1 = 0x01,
}
impl WriteDataPipePending {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WriteDataPipePending {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WriteDataPipePending {
    #[inline(always)]
    fn from(val: u8) -> WriteDataPipePending {
        WriteDataPipePending::from_bits(val)
    }
}
impl From<WriteDataPipePending> for u8 {
    #[inline(always)]
    fn from(val: WriteDataPipePending) -> u8 {
        WriteDataPipePending::to_bits(val)
    }
}
