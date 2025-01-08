#[doc = "PMIC standby control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmicCtrl(pub u32);
impl PmicCtrl {
    #[doc = "Assert the PMIC standby request when system sleep"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_stby_en(&self) -> super::vals::PmicStbyEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PmicStbyEn::from_bits(val as u8)
    }
    #[doc = "Assert the PMIC standby request when system sleep"]
    #[inline(always)]
    pub const fn set_pmic_stby_en(&mut self, val: super::vals::PmicStbyEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PMIC_READY pin status"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_ready(&self) -> super::vals::PmicReady {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PmicReady::from_bits(val as u8)
    }
    #[doc = "PMIC_READY pin status"]
    #[inline(always)]
    pub const fn set_pmic_ready(&mut self, val: super::vals::PmicReady) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "PMIC_READY is driven from pad"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_ready_exist(&self) -> super::vals::PmicReadyExist {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PmicReadyExist::from_bits(val as u8)
    }
    #[doc = "PMIC_READY is driven from pad"]
    #[inline(always)]
    pub const fn set_pmic_ready_exist(&mut self, val: super::vals::PmicReadyExist) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Software PMIC standby trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pmic_stby_soft(&self) -> super::vals::PmicStbySoft {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::PmicStbySoft::from_bits(val as u8)
    }
    #[doc = "Software PMIC standby trigger"]
    #[inline(always)]
    pub const fn set_pmic_stby_soft(&mut self, val: super::vals::PmicStbySoft) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for PmicCtrl {
    #[inline(always)]
    fn default() -> PmicCtrl {
        PmicCtrl(6u64 as u32)
    }
}
impl core::fmt::Debug for PmicCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmicCtrl")
            .field("pmic_stby_en", &self.pmic_stby_en())
            .field("pmic_ready", &self.pmic_ready())
            .field("pmic_ready_exist", &self.pmic_ready_exist())
            .field("pmic_stby_soft", &self.pmic_stby_soft())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmicCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PmicCtrl {
            pmic_stby_en: super::vals::PmicStbyEn,
            pmic_ready: super::vals::PmicReady,
            pmic_ready_exist: super::vals::PmicReadyExist,
            pmic_stby_soft: super::vals::PmicStbySoft,
        }
        let proxy = PmicCtrl {
            pmic_stby_en: self.pmic_stby_en(),
            pmic_ready: self.pmic_ready(),
            pmic_ready_exist: self.pmic_ready_exist(),
            pmic_stby_soft: self.pmic_stby_soft(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep Authentication Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsAuthenCtrl(pub u32);
impl SsAuthenCtrl {
    #[doc = "Configuration lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_cfg(&self) -> super::vals::LockCfg {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::LockCfg::from_bits(val as u8)
    }
    #[doc = "Configuration lock"]
    #[inline(always)]
    pub const fn set_lock_cfg(&mut self, val: super::vals::LockCfg) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for SsAuthenCtrl {
    #[inline(always)]
    fn default() -> SsAuthenCtrl {
        SsAuthenCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for SsAuthenCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsAuthenCtrl")
            .field("lock_cfg", &self.lock_cfg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsAuthenCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsAuthenCtrl {
            lock_cfg: super::vals::LockCfg,
        }
        let proxy = SsAuthenCtrl {
            lock_cfg: self.lock_cfg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep DCDC in control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsDcdcInCtrl(pub u32);
impl SsDcdcInCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsDcdcInCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsDcdcInCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsDcdcInCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsDcdcInCtrl {
    #[inline(always)]
    fn default() -> SsDcdcInCtrl {
        SsDcdcInCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsDcdcInCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsDcdcInCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsDcdcInCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsDcdcInCtrl {
            disable: super::vals::SsDcdcInCtrlDisable,
        }
        let proxy = SsDcdcInCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep DCDC out control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsDcdcOutCtrl(pub u32);
impl SsDcdcOutCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsDcdcOutCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsDcdcOutCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsDcdcOutCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsDcdcOutCtrl {
    #[inline(always)]
    fn default() -> SsDcdcOutCtrl {
        SsDcdcOutCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsDcdcOutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsDcdcOutCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsDcdcOutCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsDcdcOutCtrl {
            disable: super::vals::SsDcdcOutCtrlDisable,
        }
        let proxy = SsDcdcOutCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep Misc"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsMisc(pub u32);
impl SsMisc {
    #[doc = "Force CPU0 to request system sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn force_cpu0_sys_sleep(&self) -> super::vals::ForceCpu0SysSleep {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ForceCpu0SysSleep::from_bits(val as u8)
    }
    #[doc = "Force CPU0 to request system sleep mode"]
    #[inline(always)]
    pub const fn set_force_cpu0_sys_sleep(&mut self, val: super::vals::ForceCpu0SysSleep) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Force CPU1 to request system sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn force_cpu1_sys_sleep(&self) -> super::vals::ForceCpu1SysSleep {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ForceCpu1SysSleep::from_bits(val as u8)
    }
    #[doc = "Force CPU1 to request system sleep mode"]
    #[inline(always)]
    pub const fn set_force_cpu1_sys_sleep(&mut self, val: super::vals::ForceCpu1SysSleep) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for SsMisc {
    #[inline(always)]
    fn default() -> SsMisc {
        SsMisc(0u64 as u32)
    }
}
impl core::fmt::Debug for SsMisc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsMisc")
            .field("force_cpu0_sys_sleep", &self.force_cpu0_sys_sleep())
            .field("force_cpu1_sys_sleep", &self.force_cpu1_sys_sleep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsMisc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsMisc {
            force_cpu0_sys_sleep: super::vals::ForceCpu0SysSleep,
            force_cpu1_sys_sleep: super::vals::ForceCpu1SysSleep,
        }
        let proxy = SsMisc {
            force_cpu0_sys_sleep: self.force_cpu0_sys_sleep(),
            force_cpu1_sys_sleep: self.force_cpu1_sys_sleep(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep PMIC in control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsPmicInCtrl(pub u32);
impl SsPmicInCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsPmicInCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsPmicInCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsPmicInCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsPmicInCtrl {
    #[inline(always)]
    fn default() -> SsPmicInCtrl {
        SsPmicInCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsPmicInCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsPmicInCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsPmicInCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsPmicInCtrl {
            disable: super::vals::SsPmicInCtrlDisable,
        }
        let proxy = SsPmicInCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep PMIC out control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsPmicOutCtrl(pub u32);
impl SsPmicOutCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsPmicOutCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsPmicOutCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsPmicOutCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsPmicOutCtrl {
    #[inline(always)]
    fn default() -> SsPmicOutCtrl {
        SsPmicOutCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsPmicOutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsPmicOutCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsPmicOutCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsPmicOutCtrl {
            disable: super::vals::SsPmicOutCtrlDisable,
        }
        let proxy = SsPmicOutCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep STEP0 (BIAS) in control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsStep0InCtrl(pub u32);
impl SsStep0InCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsStep0InCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsStep0InCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsStep0InCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsStep0InCtrl {
    #[inline(always)]
    fn default() -> SsStep0InCtrl {
        SsStep0InCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsStep0InCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsStep0InCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsStep0InCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsStep0InCtrl {
            disable: super::vals::SsStep0InCtrlDisable,
        }
        let proxy = SsStep0InCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep STEP0 (BIAS) out control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsStep0OutCtrl(pub u32);
impl SsStep0OutCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsStep0OutCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsStep0OutCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsStep0OutCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsStep0OutCtrl {
    #[inline(always)]
    fn default() -> SsStep0OutCtrl {
        SsStep0OutCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsStep0OutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsStep0OutCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsStep0OutCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsStep0OutCtrl {
            disable: super::vals::SsStep0OutCtrlDisable,
        }
        let proxy = SsStep0OutCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep STEP1 (PLDO) in control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsStep1InCtrl(pub u32);
impl SsStep1InCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsStep1InCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsStep1InCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsStep1InCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsStep1InCtrl {
    #[inline(always)]
    fn default() -> SsStep1InCtrl {
        SsStep1InCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsStep1InCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsStep1InCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsStep1InCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsStep1InCtrl {
            disable: super::vals::SsStep1InCtrlDisable,
        }
        let proxy = SsStep1InCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep STEP1 (PLDO) out control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsStep1OutCtrl(pub u32);
impl SsStep1OutCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsStep1OutCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsStep1OutCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsStep1OutCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsStep1OutCtrl {
    #[inline(always)]
    fn default() -> SsStep1OutCtrl {
        SsStep1OutCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsStep1OutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsStep1OutCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsStep1OutCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsStep1OutCtrl {
            disable: super::vals::SsStep1OutCtrlDisable,
        }
        let proxy = SsStep1OutCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep STEP2 (BANDGAP) in control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsStep2InCtrl(pub u32);
impl SsStep2InCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsStep2InCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsStep2InCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsStep2InCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsStep2InCtrl {
    #[inline(always)]
    fn default() -> SsStep2InCtrl {
        SsStep2InCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsStep2InCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsStep2InCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsStep2InCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsStep2InCtrl {
            disable: super::vals::SsStep2InCtrlDisable,
        }
        let proxy = SsStep2InCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep STEP2 (BANDGAP) out control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsStep2OutCtrl(pub u32);
impl SsStep2OutCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsStep2OutCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsStep2OutCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsStep2OutCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsStep2OutCtrl {
    #[inline(always)]
    fn default() -> SsStep2OutCtrl {
        SsStep2OutCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsStep2OutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsStep2OutCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsStep2OutCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsStep2OutCtrl {
            disable: super::vals::SsStep2OutCtrlDisable,
        }
        let proxy = SsStep2OutCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep STEP3 (LDO) in control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsStep3InCtrl(pub u32);
impl SsStep3InCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsStep3InCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsStep3InCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsStep3InCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsStep3InCtrl {
    #[inline(always)]
    fn default() -> SsStep3InCtrl {
        SsStep3InCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsStep3InCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsStep3InCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsStep3InCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsStep3InCtrl {
            disable: super::vals::SsStep3InCtrlDisable,
        }
        let proxy = SsStep3InCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System Sleep STEP3 (LDO) out control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SsStep3OutCtrl(pub u32);
impl SsStep3OutCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::SsStep3OutCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::SsStep3OutCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::SsStep3OutCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for SsStep3OutCtrl {
    #[inline(always)]
    fn default() -> SsStep3OutCtrl {
        SsStep3OutCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for SsStep3OutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SsStep3OutCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SsStep3OutCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SsStep3OutCtrl {
            disable: super::vals::SsStep3OutCtrlDisable,
        }
        let proxy = SsStep3OutCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
