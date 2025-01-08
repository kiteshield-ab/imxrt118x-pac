#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheEccmTag {
    #[doc = "Data"]
    DATA = 0x0,
    #[doc = "Tag"]
    TAG = 0x01,
}
impl CodeCacheEccmTag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheEccmTag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheEccmTag {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheEccmTag {
        CodeCacheEccmTag::from_bits(val)
    }
}
impl From<CodeCacheEccmTag> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheEccmTag) -> u8 {
        CodeCacheEccmTag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheEccsTag {
    #[doc = "Data"]
    DATA = 0x0,
    #[doc = "Tag"]
    TAG = 0x01,
}
impl CodeCacheEccsTag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheEccsTag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheEccsTag {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheEccsTag {
        CodeCacheEccsTag::from_bits(val)
    }
}
impl From<CodeCacheEccsTag> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheEccsTag) -> u8 {
        CodeCacheEccsTag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheErrmIntEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl CodeCacheErrmIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheErrmIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheErrmIntEn {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheErrmIntEn {
        CodeCacheErrmIntEn::from_bits(val)
    }
}
impl From<CodeCacheErrmIntEn> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheErrmIntEn) -> u8 {
        CodeCacheErrmIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheErrmIntSigEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl CodeCacheErrmIntSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheErrmIntSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheErrmIntSigEn {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheErrmIntSigEn {
        CodeCacheErrmIntSigEn::from_bits(val)
    }
}
impl From<CodeCacheErrmIntSigEn> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheErrmIntSigEn) -> u8 {
        CodeCacheErrmIntSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheErrmOverIntEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl CodeCacheErrmOverIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheErrmOverIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheErrmOverIntEn {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheErrmOverIntEn {
        CodeCacheErrmOverIntEn::from_bits(val)
    }
}
impl From<CodeCacheErrmOverIntEn> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheErrmOverIntEn) -> u8 {
        CodeCacheErrmOverIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheErrmOverIntSigEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl CodeCacheErrmOverIntSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheErrmOverIntSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheErrmOverIntSigEn {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheErrmOverIntSigEn {
        CodeCacheErrmOverIntSigEn::from_bits(val)
    }
}
impl From<CodeCacheErrmOverIntSigEn> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheErrmOverIntSigEn) -> u8 {
        CodeCacheErrmOverIntSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheErrsIntEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl CodeCacheErrsIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheErrsIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheErrsIntEn {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheErrsIntEn {
        CodeCacheErrsIntEn::from_bits(val)
    }
}
impl From<CodeCacheErrsIntEn> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheErrsIntEn) -> u8 {
        CodeCacheErrsIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheErrsIntSigEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl CodeCacheErrsIntSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheErrsIntSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheErrsIntSigEn {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheErrsIntSigEn {
        CodeCacheErrsIntSigEn::from_bits(val)
    }
}
impl From<CodeCacheErrsIntSigEn> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheErrsIntSigEn) -> u8 {
        CodeCacheErrsIntSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheErrsOverIntEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl CodeCacheErrsOverIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheErrsOverIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheErrsOverIntEn {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheErrsOverIntEn {
        CodeCacheErrsOverIntEn::from_bits(val)
    }
}
impl From<CodeCacheErrsOverIntEn> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheErrsOverIntEn) -> u8 {
        CodeCacheErrsOverIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CodeCacheErrsOverIntSigEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl CodeCacheErrsOverIntSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CodeCacheErrsOverIntSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CodeCacheErrsOverIntSigEn {
    #[inline(always)]
    fn from(val: u8) -> CodeCacheErrsOverIntSigEn {
        CodeCacheErrsOverIntSigEn::from_bits(val)
    }
}
impl From<CodeCacheErrsOverIntSigEn> for u8 {
    #[inline(always)]
    fn from(val: CodeCacheErrsOverIntSigEn) -> u8 {
        CodeCacheErrsOverIntSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReccDis {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
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
pub enum SystemCacheEccErrmIntEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SystemCacheEccErrmIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheEccErrmIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheEccErrmIntEn {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheEccErrmIntEn {
        SystemCacheEccErrmIntEn::from_bits(val)
    }
}
impl From<SystemCacheEccErrmIntEn> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheEccErrmIntEn) -> u8 {
        SystemCacheEccErrmIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheEccErrmOverIntEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SystemCacheEccErrmOverIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheEccErrmOverIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheEccErrmOverIntEn {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheEccErrmOverIntEn {
        SystemCacheEccErrmOverIntEn::from_bits(val)
    }
}
impl From<SystemCacheEccErrmOverIntEn> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheEccErrmOverIntEn) -> u8 {
        SystemCacheEccErrmOverIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheEccErrsIntEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SystemCacheEccErrsIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheEccErrsIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheEccErrsIntEn {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheEccErrsIntEn {
        SystemCacheEccErrsIntEn::from_bits(val)
    }
}
impl From<SystemCacheEccErrsIntEn> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheEccErrsIntEn) -> u8 {
        SystemCacheEccErrsIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheEccErrsOverIntEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SystemCacheEccErrsOverIntEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheEccErrsOverIntEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheEccErrsOverIntEn {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheEccErrsOverIntEn {
        SystemCacheEccErrsOverIntEn::from_bits(val)
    }
}
impl From<SystemCacheEccErrsOverIntEn> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheEccErrsOverIntEn) -> u8 {
        SystemCacheEccErrsOverIntEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheEccmTag {
    #[doc = "Data"]
    DATA = 0x0,
    #[doc = "Tag"]
    TAG = 0x01,
}
impl SystemCacheEccmTag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheEccmTag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheEccmTag {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheEccmTag {
        SystemCacheEccmTag::from_bits(val)
    }
}
impl From<SystemCacheEccmTag> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheEccmTag) -> u8 {
        SystemCacheEccmTag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheEccsTag {
    #[doc = "Data"]
    DATA = 0x0,
    #[doc = "Tag"]
    TAG = 0x01,
}
impl SystemCacheEccsTag {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheEccsTag {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheEccsTag {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheEccsTag {
        SystemCacheEccsTag::from_bits(val)
    }
}
impl From<SystemCacheEccsTag> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheEccsTag) -> u8 {
        SystemCacheEccsTag::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheErrmIntSigEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SystemCacheErrmIntSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheErrmIntSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheErrmIntSigEn {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheErrmIntSigEn {
        SystemCacheErrmIntSigEn::from_bits(val)
    }
}
impl From<SystemCacheErrmIntSigEn> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheErrmIntSigEn) -> u8 {
        SystemCacheErrmIntSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheErrmOverIntSigEn {
    #[doc = "Masked"]
    MASKED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl SystemCacheErrmOverIntSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheErrmOverIntSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheErrmOverIntSigEn {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheErrmOverIntSigEn {
        SystemCacheErrmOverIntSigEn::from_bits(val)
    }
}
impl From<SystemCacheErrmOverIntSigEn> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheErrmOverIntSigEn) -> u8 {
        SystemCacheErrmOverIntSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheErrsIntSigEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SystemCacheErrsIntSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheErrsIntSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheErrsIntSigEn {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheErrsIntSigEn {
        SystemCacheErrsIntSigEn::from_bits(val)
    }
}
impl From<SystemCacheErrsIntSigEn> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheErrsIntSigEn) -> u8 {
        SystemCacheErrsIntSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SystemCacheErrsOverIntSigEn {
    #[doc = "Mask"]
    MASKED = 0x0,
    #[doc = "Enable"]
    ENABLED = 0x01,
}
impl SystemCacheErrsOverIntSigEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystemCacheErrsOverIntSigEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystemCacheErrsOverIntSigEn {
    #[inline(always)]
    fn from(val: u8) -> SystemCacheErrsOverIntSigEn {
        SystemCacheErrsOverIntSigEn::from_bits(val)
    }
}
impl From<SystemCacheErrsOverIntSigEn> for u8 {
    #[inline(always)]
    fn from(val: SystemCacheErrsOverIntSigEn) -> u8 {
        SystemCacheErrsOverIntSigEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WeccDis {
    #[doc = "Enable"]
    ENABLE = 0x0,
    #[doc = "Disable"]
    DISABLE = 0x01,
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
