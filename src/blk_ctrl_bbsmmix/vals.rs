#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BbsmBypassEn {
    #[doc = "Disable bypass"]
    NO = 0x0,
    #[doc = "Enable bypass"]
    OVER = 0x01,
}
impl BbsmBypassEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BbsmBypassEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BbsmBypassEn {
    #[inline(always)]
    fn from(val: u8) -> BbsmBypassEn {
        BbsmBypassEn::from_bits(val)
    }
}
impl From<BbsmBypassEn> for u8 {
    #[inline(always)]
    fn from(val: BbsmBypassEn) -> u8 {
        BbsmBypassEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BbsmCapTrimSel {
    #[doc = "The trimming codes are selected from eFuse"]
    OVER = 0x0,
    #[doc = "The trimming codes are used from BBSM_OSC_CAP_TRIM (osc32k's load capacitor)"]
    NO = 0x01,
}
impl BbsmCapTrimSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BbsmCapTrimSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BbsmCapTrimSel {
    #[inline(always)]
    fn from(val: u8) -> BbsmCapTrimSel {
        BbsmCapTrimSel::from_bits(val)
    }
}
impl From<BbsmCapTrimSel> for u8 {
    #[inline(always)]
    fn from(val: BbsmCapTrimSel) -> u8 {
        BbsmCapTrimSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BbsmCoreVoltDetTrimSel {
    #[doc = "The trimming codes are selected from eFuse"]
    OVER1 = 0x0,
    #[doc = "The trimming codes of core voltage detectors used to change the voltage falling trip point are selected from BBSM_CORE_VOLT_DET_TRIM"]
    NO1 = 0x01,
}
impl BbsmCoreVoltDetTrimSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BbsmCoreVoltDetTrimSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BbsmCoreVoltDetTrimSel {
    #[inline(always)]
    fn from(val: u8) -> BbsmCoreVoltDetTrimSel {
        BbsmCoreVoltDetTrimSel::from_bits(val)
    }
}
impl From<BbsmCoreVoltDetTrimSel> for u8 {
    #[inline(always)]
    fn from(val: BbsmCoreVoltDetTrimSel) -> u8 {
        BbsmCoreVoltDetTrimSel::to_bits(val)
    }
}
