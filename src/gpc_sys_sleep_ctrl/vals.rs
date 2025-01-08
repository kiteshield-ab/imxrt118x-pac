#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceCpu0SysSleep {
    #[doc = "Do not force CPU0 to request system sleep mode"]
    B0 = 0x0,
    #[doc = "Force CPU0 to request system sleep mode"]
    B1 = 0x01,
}
impl ForceCpu0SysSleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceCpu0SysSleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceCpu0SysSleep {
    #[inline(always)]
    fn from(val: u8) -> ForceCpu0SysSleep {
        ForceCpu0SysSleep::from_bits(val)
    }
}
impl From<ForceCpu0SysSleep> for u8 {
    #[inline(always)]
    fn from(val: ForceCpu0SysSleep) -> u8 {
        ForceCpu0SysSleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ForceCpu1SysSleep {
    #[doc = "Do not force CPU1 to request system sleep mode"]
    B0 = 0x0,
    #[doc = "Force CPU1 to request system sleep mode"]
    B1 = 0x01,
}
impl ForceCpu1SysSleep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ForceCpu1SysSleep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ForceCpu1SysSleep {
    #[inline(always)]
    fn from(val: u8) -> ForceCpu1SysSleep {
        ForceCpu1SysSleep::from_bits(val)
    }
}
impl From<ForceCpu1SysSleep> for u8 {
    #[inline(always)]
    fn from(val: ForceCpu1SysSleep) -> u8 {
        ForceCpu1SysSleep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockCfg {
    #[doc = "The value of low power configuration fields are not locked."]
    B0 = 0x0,
    #[doc = "The value of low power configuration fields are locked. Refer to the function field of each gpc_sys_sleep_ctrl registers."]
    B1 = 0x01,
}
impl LockCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockCfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockCfg {
    #[inline(always)]
    fn from(val: u8) -> LockCfg {
        LockCfg::from_bits(val)
    }
}
impl From<LockCfg> for u8 {
    #[inline(always)]
    fn from(val: LockCfg) -> u8 {
        LockCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmicReady {
    #[doc = "PMIC_READY not asserted"]
    B0 = 0x0,
    #[doc = "PMIC_READY asserted"]
    B1 = 0x01,
}
impl PmicReady {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmicReady {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmicReady {
    #[inline(always)]
    fn from(val: u8) -> PmicReady {
        PmicReady::from_bits(val)
    }
}
impl From<PmicReady> for u8 {
    #[inline(always)]
    fn from(val: PmicReady) -> u8 {
        PmicReady::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmicReadyExist {
    #[doc = "PMIC_READY is not driven."]
    B0 = 0x0,
    #[doc = "PMIC_READY is driven from pad. If PMIC_READY_EXIST = 1, PMIC_READY = 1 means external PMIC drives PMIC_READY pin."]
    B1 = 0x01,
}
impl PmicReadyExist {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmicReadyExist {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmicReadyExist {
    #[inline(always)]
    fn from(val: u8) -> PmicReadyExist {
        PmicReadyExist::from_bits(val)
    }
}
impl From<PmicReadyExist> for u8 {
    #[inline(always)]
    fn from(val: PmicReadyExist) -> u8 {
        PmicReadyExist::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmicStbyEn {
    #[doc = "Do not assert PMIC_STBY_REQ when system sleep is entered"]
    B0 = 0x0,
    #[doc = "Assert PMIC_STBY_REQ when system sleep is entered"]
    B1 = 0x01,
}
impl PmicStbyEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmicStbyEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmicStbyEn {
    #[inline(always)]
    fn from(val: u8) -> PmicStbyEn {
        PmicStbyEn::from_bits(val)
    }
}
impl From<PmicStbyEn> for u8 {
    #[inline(always)]
    fn from(val: PmicStbyEn) -> u8 {
        PmicStbyEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PmicStbySoft {
    #[doc = "Exit PMIC standby"]
    B0 = 0x0,
    #[doc = "Trigger PMIC standby"]
    B1 = 0x01,
}
impl PmicStbySoft {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PmicStbySoft {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PmicStbySoft {
    #[inline(always)]
    fn from(val: u8) -> PmicStbySoft {
        PmicStbySoft::from_bits(val)
    }
}
impl From<PmicStbySoft> for u8 {
    #[inline(always)]
    fn from(val: PmicStbySoft) -> u8 {
        PmicStbySoft::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsDcdcInCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsDcdcInCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsDcdcInCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsDcdcInCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsDcdcInCtrlDisable {
        SsDcdcInCtrlDisable::from_bits(val)
    }
}
impl From<SsDcdcInCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsDcdcInCtrlDisable) -> u8 {
        SsDcdcInCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsDcdcOutCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsDcdcOutCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsDcdcOutCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsDcdcOutCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsDcdcOutCtrlDisable {
        SsDcdcOutCtrlDisable::from_bits(val)
    }
}
impl From<SsDcdcOutCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsDcdcOutCtrlDisable) -> u8 {
        SsDcdcOutCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsPmicInCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsPmicInCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsPmicInCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsPmicInCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsPmicInCtrlDisable {
        SsPmicInCtrlDisable::from_bits(val)
    }
}
impl From<SsPmicInCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsPmicInCtrlDisable) -> u8 {
        SsPmicInCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsPmicOutCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsPmicOutCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsPmicOutCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsPmicOutCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsPmicOutCtrlDisable {
        SsPmicOutCtrlDisable::from_bits(val)
    }
}
impl From<SsPmicOutCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsPmicOutCtrlDisable) -> u8 {
        SsPmicOutCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStep0InCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsStep0InCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStep0InCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStep0InCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsStep0InCtrlDisable {
        SsStep0InCtrlDisable::from_bits(val)
    }
}
impl From<SsStep0InCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsStep0InCtrlDisable) -> u8 {
        SsStep0InCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStep0OutCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsStep0OutCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStep0OutCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStep0OutCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsStep0OutCtrlDisable {
        SsStep0OutCtrlDisable::from_bits(val)
    }
}
impl From<SsStep0OutCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsStep0OutCtrlDisable) -> u8 {
        SsStep0OutCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStep1InCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsStep1InCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStep1InCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStep1InCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsStep1InCtrlDisable {
        SsStep1InCtrlDisable::from_bits(val)
    }
}
impl From<SsStep1InCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsStep1InCtrlDisable) -> u8 {
        SsStep1InCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStep1OutCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsStep1OutCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStep1OutCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStep1OutCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsStep1OutCtrlDisable {
        SsStep1OutCtrlDisable::from_bits(val)
    }
}
impl From<SsStep1OutCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsStep1OutCtrlDisable) -> u8 {
        SsStep1OutCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStep2InCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsStep2InCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStep2InCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStep2InCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsStep2InCtrlDisable {
        SsStep2InCtrlDisable::from_bits(val)
    }
}
impl From<SsStep2InCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsStep2InCtrlDisable) -> u8 {
        SsStep2InCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStep2OutCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsStep2OutCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStep2OutCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStep2OutCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsStep2OutCtrlDisable {
        SsStep2OutCtrlDisable::from_bits(val)
    }
}
impl From<SsStep2OutCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsStep2OutCtrlDisable) -> u8 {
        SsStep2OutCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStep3InCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsStep3InCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStep3InCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStep3InCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsStep3InCtrlDisable {
        SsStep3InCtrlDisable::from_bits(val)
    }
}
impl From<SsStep3InCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsStep3InCtrlDisable) -> u8 {
        SsStep3InCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStep3OutCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl SsStep3OutCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStep3OutCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStep3OutCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> SsStep3OutCtrlDisable {
        SsStep3OutCtrlDisable::from_bits(val)
    }
}
impl From<SsStep3OutCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: SsStep3OutCtrlDisable) -> u8 {
        SsStep3OutCtrlDisable::to_bits(val)
    }
}
