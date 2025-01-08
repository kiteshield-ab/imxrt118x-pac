#[doc = "CM Authentication Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmAuthenCtrl(pub u32);
impl _cmAuthenCtrl {
    #[doc = "Configuration lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_cfg(&self) -> super::vals::LockCfg {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::LockCfg::from_bits(val as u8)
    }
    #[doc = "Configuration lock"]
    #[inline(always)]
    pub const fn set_lock_cfg(&mut self, val: super::vals::LockCfg) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Allow user mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn user(&self) -> super::vals::User {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::User::from_bits(val as u8)
    }
    #[doc = "Allow user mode access"]
    #[inline(always)]
    pub const fn set_user(&mut self, val: super::vals::User) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Allow non-secure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn nonsecure(&self) -> super::vals::Nonsecure {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Nonsecure::from_bits(val as u8)
    }
    #[doc = "Allow non-secure mode access"]
    #[inline(always)]
    pub const fn set_nonsecure(&mut self, val: super::vals::Nonsecure) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Lock NONSECURE and USER"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_setting(&self) -> super::vals::LockSetting {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::LockSetting::from_bits(val as u8)
    }
    #[doc = "Lock NONSECURE and USER"]
    #[inline(always)]
    pub const fn set_lock_setting(&mut self, val: super::vals::LockSetting) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "White list lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_list(&self) -> super::vals::LockList {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::LockList::from_bits(val as u8)
    }
    #[doc = "White list lock"]
    #[inline(always)]
    pub const fn set_lock_list(&mut self, val: super::vals::LockList) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Domain ID white list"]
    #[must_use]
    #[inline(always)]
    pub const fn white_list(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Domain ID white list"]
    #[inline(always)]
    pub const fn set_white_list(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for _cmAuthenCtrl {
    #[inline(always)]
    fn default() -> _cmAuthenCtrl {
        _cmAuthenCtrl(4294901760u64 as u32)
    }
}
impl core::fmt::Debug for _cmAuthenCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmAuthenCtrl")
            .field("lock_cfg", &self.lock_cfg())
            .field("user", &self.user())
            .field("nonsecure", &self.nonsecure())
            .field("lock_setting", &self.lock_setting())
            .field("lock_list", &self.lock_list())
            .field("white_list", &self.white_list())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmAuthenCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmAuthenCtrl {
            lock_cfg: super::vals::LockCfg,
            user: super::vals::User,
            nonsecure: super::vals::Nonsecure,
            lock_setting: super::vals::LockSetting,
            lock_list: super::vals::LockList,
            white_list: u16,
        }
        let proxy = _cmAuthenCtrl {
            lock_cfg: self.lock_cfg(),
            user: self.user(),
            nonsecure: self.nonsecure(),
            lock_setting: self.lock_setting(),
            lock_list: self.lock_list(),
            white_list: self.white_list(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Miscellaneous"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmMisc(pub u32);
impl _cmMisc {
    #[doc = "Non-masked interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn nmi_stat(&self) -> super::vals::NmiStat {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::NmiStat::from_bits(val as u8)
    }
    #[doc = "Non-masked interrupt status"]
    #[inline(always)]
    pub const fn set_nmi_stat(&mut self, val: super::vals::NmiStat) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Allow cpu_sleep_hold_req to assert during CPU low power status"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep_hold_en(&self) -> super::vals::SleepHoldEn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SleepHoldEn::from_bits(val as u8)
    }
    #[doc = "Allow cpu_sleep_hold_req to assert during CPU low power status"]
    #[inline(always)]
    pub const fn set_sleep_hold_en(&mut self, val: super::vals::SleepHoldEn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "CPU sleep hold status"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep_hold_stat(&self) -> super::vals::SleepHoldStat {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SleepHoldStat::from_bits(val as u8)
    }
    #[doc = "CPU sleep hold status"]
    #[inline(always)]
    pub const fn set_sleep_hold_stat(&mut self, val: super::vals::SleepHoldStat) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for _cmMisc {
    #[inline(always)]
    fn default() -> _cmMisc {
        _cmMisc(14u64 as u32)
    }
}
impl core::fmt::Debug for _cmMisc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmMisc")
            .field("nmi_stat", &self.nmi_stat())
            .field("sleep_hold_en", &self.sleep_hold_en())
            .field("sleep_hold_stat", &self.sleep_hold_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmMisc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmMisc {
            nmi_stat: super::vals::NmiStat,
            sleep_hold_en: super::vals::SleepHoldEn,
            sleep_hold_stat: super::vals::SleepHoldStat,
        }
        let proxy = _cmMisc {
            nmi_stat: self.nmi_stat(),
            sleep_hold_en: self.sleep_hold_en(),
            sleep_hold_stat: self.sleep_hold_stat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CPU mode control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmModeCtrl(pub u32);
impl _cmModeCtrl {
    #[doc = "The CPU mode the CPU platform should transit to on next sleep event"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_mode_target(&self) -> super::vals::CpuModeTarget {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CpuModeTarget::from_bits(val as u8)
    }
    #[doc = "The CPU mode the CPU platform should transit to on next sleep event"]
    #[inline(always)]
    pub const fn set_cpu_mode_target(&mut self, val: super::vals::CpuModeTarget) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "WFE assertion can be sleep event"]
    #[must_use]
    #[inline(always)]
    pub const fn wfe_en(&self) -> super::vals::WfeEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::WfeEn::from_bits(val as u8)
    }
    #[doc = "WFE assertion can be sleep event"]
    #[inline(always)]
    pub const fn set_wfe_en(&mut self, val: super::vals::WfeEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
}
impl Default for _cmModeCtrl {
    #[inline(always)]
    fn default() -> _cmModeCtrl {
        _cmModeCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for _cmModeCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmModeCtrl")
            .field("cpu_mode_target", &self.cpu_mode_target())
            .field("wfe_en", &self.wfe_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmModeCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmModeCtrl {
            cpu_mode_target: super::vals::CpuModeTarget,
            wfe_en: super::vals::WfeEn,
        }
        let proxy = _cmModeCtrl {
            cpu_mode_target: self.cpu_mode_target(),
            wfe_en: self.wfe_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM CPU mode Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmModeStat(pub u32);
impl _cmModeStat {
    #[doc = "Current CPU mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_mode_current(&self) -> super::vals::CpuModeCurrent {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::CpuModeCurrent::from_bits(val as u8)
    }
    #[doc = "Current CPU mode"]
    #[inline(always)]
    pub const fn set_cpu_mode_current(&mut self, val: super::vals::CpuModeCurrent) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Previous CPU mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cpu_mode_previous(&self) -> super::vals::CpuModePrevious {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::CpuModePrevious::from_bits(val as u8)
    }
    #[doc = "Previous CPU mode"]
    #[inline(always)]
    pub const fn set_cpu_mode_previous(&mut self, val: super::vals::CpuModePrevious) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for _cmModeStat {
    #[inline(always)]
    fn default() -> _cmModeStat {
        _cmModeStat(0u64 as u32)
    }
}
impl core::fmt::Debug for _cmModeStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmModeStat")
            .field("cpu_mode_current", &self.cpu_mode_current())
            .field("cpu_mode_previous", &self.cpu_mode_previous())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmModeStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmModeStat {
            cpu_mode_current: super::vals::CpuModeCurrent,
            cpu_mode_previous: super::vals::CpuModePrevious,
        }
        let proxy = _cmModeStat {
            cpu_mode_current: self.cpu_mode_current(),
            cpu_mode_previous: self.cpu_mode_previous(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM non-IRQ wakeup mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmNonIrqWakeupMask(pub u32);
impl _cmNonIrqWakeupMask {
    #[doc = "\"1\" means the debug_wakeup_request cannot wakeup CPU platform"]
    #[must_use]
    #[inline(always)]
    pub const fn debug_wakeup_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "\"1\" means the debug_wakeup_request cannot wakeup CPU platform"]
    #[inline(always)]
    pub const fn set_debug_wakeup_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for _cmNonIrqWakeupMask {
    #[inline(always)]
    fn default() -> _cmNonIrqWakeupMask {
        _cmNonIrqWakeupMask(1u64 as u32)
    }
}
impl core::fmt::Debug for _cmNonIrqWakeupMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmNonIrqWakeupMask")
            .field("debug_wakeup_mask", &self.debug_wakeup_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmNonIrqWakeupMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmNonIrqWakeupMask {
            debug_wakeup_mask: bool,
        }
        let proxy = _cmNonIrqWakeupMask {
            debug_wakeup_mask: self.debug_wakeup_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM non-irq wakeup status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmNonIrqWakeupStat(pub u32);
impl _cmNonIrqWakeupStat {
    #[doc = "Debug wakeup status"]
    #[must_use]
    #[inline(always)]
    pub const fn debug_wakeup_stat(&self) -> super::vals::DebugWakeupStat {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DebugWakeupStat::from_bits(val as u8)
    }
    #[doc = "Debug wakeup status"]
    #[inline(always)]
    pub const fn set_debug_wakeup_stat(&mut self, val: super::vals::DebugWakeupStat) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for _cmNonIrqWakeupStat {
    #[inline(always)]
    fn default() -> _cmNonIrqWakeupStat {
        _cmNonIrqWakeupStat(0u64 as u32)
    }
}
impl core::fmt::Debug for _cmNonIrqWakeupStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmNonIrqWakeupStat")
            .field("debug_wakeup_stat", &self.debug_wakeup_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmNonIrqWakeupStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmNonIrqWakeupStat {
            debug_wakeup_stat: super::vals::DebugWakeupStat,
        }
        let proxy = _cmNonIrqWakeupStat {
            debug_wakeup_stat: self.debug_wakeup_stat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM sleep isolation control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmSleepIsoCtrl(pub u32);
impl _cmSleepIsoCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmSleepIsoCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmSleepIsoCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmSleepIsoCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmSleepIsoCtrl {
    #[inline(always)]
    fn default() -> _cmSleepIsoCtrl {
        _cmSleepIsoCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmSleepIsoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmSleepIsoCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmSleepIsoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmSleepIsoCtrl {
            disable: super::vals::_cmSleepIsoCtrlDisable,
        }
        let proxy = _cmSleepIsoCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM sleep LPCG control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmSleepLpcgCtrl(pub u32);
impl _cmSleepLpcgCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmSleepLpcgCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmSleepLpcgCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmSleepLpcgCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmSleepLpcgCtrl {
    #[inline(always)]
    fn default() -> _cmSleepLpcgCtrl {
        _cmSleepLpcgCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmSleepLpcgCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmSleepLpcgCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmSleepLpcgCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmSleepLpcgCtrl {
            disable: super::vals::_cmSleepLpcgCtrlDisable,
        }
        let proxy = _cmSleepLpcgCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM sleep PLL control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmSleepPllCtrl(pub u32);
impl _cmSleepPllCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmSleepPllCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmSleepPllCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmSleepPllCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmSleepPllCtrl {
    #[inline(always)]
    fn default() -> _cmSleepPllCtrl {
        _cmSleepPllCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmSleepPllCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmSleepPllCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmSleepPllCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmSleepPllCtrl {
            disable: super::vals::_cmSleepPllCtrlDisable,
        }
        let proxy = _cmSleepPllCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM sleep power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmSleepPowerCtrl(pub u32);
impl _cmSleepPowerCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmSleepPowerCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmSleepPowerCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmSleepPowerCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmSleepPowerCtrl {
    #[inline(always)]
    fn default() -> _cmSleepPowerCtrl {
        _cmSleepPowerCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmSleepPowerCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmSleepPowerCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmSleepPowerCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmSleepPowerCtrl {
            disable: super::vals::_cmSleepPowerCtrlDisable,
        }
        let proxy = _cmSleepPowerCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM sleep reset control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmSleepResetCtrl(pub u32);
impl _cmSleepResetCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmSleepResetCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmSleepResetCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmSleepResetCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmSleepResetCtrl {
    #[inline(always)]
    fn default() -> _cmSleepResetCtrl {
        _cmSleepResetCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmSleepResetCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmSleepResetCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmSleepResetCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmSleepResetCtrl {
            disable: super::vals::_cmSleepResetCtrlDisable,
        }
        let proxy = _cmSleepResetCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM sleep SSAR control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmSleepSsarCtrl(pub u32);
impl _cmSleepSsarCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmSleepSsarCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmSleepSsarCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmSleepSsarCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmSleepSsarCtrl {
    #[inline(always)]
    fn default() -> _cmSleepSsarCtrl {
        _cmSleepSsarCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmSleepSsarCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmSleepSsarCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmSleepSsarCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmSleepSsarCtrl {
            disable: super::vals::_cmSleepSsarCtrlDisable,
        }
        let proxy = _cmSleepSsarCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM system sleep control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmSysSleepCtrl(pub u32);
impl _cmSysSleepCtrl {
    #[doc = "Request system sleep when CPU is in WAIT mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_wait(&self) -> super::vals::SsWait {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::SsWait::from_bits(val as u8)
    }
    #[doc = "Request system sleep when CPU is in WAIT mode"]
    #[inline(always)]
    pub const fn set_ss_wait(&mut self, val: super::vals::SsWait) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Request system sleep when CPU is in STOP mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_stop(&self) -> super::vals::SsStop {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::SsStop::from_bits(val as u8)
    }
    #[doc = "Request system sleep when CPU is in STOP mode"]
    #[inline(always)]
    pub const fn set_ss_stop(&mut self, val: super::vals::SsStop) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Request system sleep when CPU is in SUSPEND mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ss_suspend(&self) -> super::vals::SsSuspend {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SsSuspend::from_bits(val as u8)
    }
    #[doc = "Request system sleep when CPU is in SUSPEND mode"]
    #[inline(always)]
    pub const fn set_ss_suspend(&mut self, val: super::vals::SsSuspend) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for _cmSysSleepCtrl {
    #[inline(always)]
    fn default() -> _cmSysSleepCtrl {
        _cmSysSleepCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmSysSleepCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmSysSleepCtrl")
            .field("ss_wait", &self.ss_wait())
            .field("ss_stop", &self.ss_stop())
            .field("ss_suspend", &self.ss_suspend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmSysSleepCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmSysSleepCtrl {
            ss_wait: super::vals::SsWait,
            ss_stop: super::vals::SsStop,
            ss_suspend: super::vals::SsSuspend,
        }
        let proxy = _cmSysSleepCtrl {
            ss_wait: self.ss_wait(),
            ss_stop: self.ss_stop(),
            ss_suspend: self.ss_suspend(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM wakeup isolation control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmWakeupIsoCtrl(pub u32);
impl _cmWakeupIsoCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmWakeupIsoCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmWakeupIsoCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmWakeupIsoCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmWakeupIsoCtrl {
    #[inline(always)]
    fn default() -> _cmWakeupIsoCtrl {
        _cmWakeupIsoCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmWakeupIsoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmWakeupIsoCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmWakeupIsoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmWakeupIsoCtrl {
            disable: super::vals::_cmWakeupIsoCtrlDisable,
        }
        let proxy = _cmWakeupIsoCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM wakeup LPCG control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmWakeupLpcgCtrl(pub u32);
impl _cmWakeupLpcgCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmWakeupLpcgCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmWakeupLpcgCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmWakeupLpcgCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmWakeupLpcgCtrl {
    #[inline(always)]
    fn default() -> _cmWakeupLpcgCtrl {
        _cmWakeupLpcgCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmWakeupLpcgCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmWakeupLpcgCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmWakeupLpcgCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmWakeupLpcgCtrl {
            disable: super::vals::_cmWakeupLpcgCtrlDisable,
        }
        let proxy = _cmWakeupLpcgCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM wakeup PLL control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmWakeupPllCtrl(pub u32);
impl _cmWakeupPllCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmWakeupPllCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmWakeupPllCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmWakeupPllCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmWakeupPllCtrl {
    #[inline(always)]
    fn default() -> _cmWakeupPllCtrl {
        _cmWakeupPllCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmWakeupPllCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmWakeupPllCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmWakeupPllCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmWakeupPllCtrl {
            disable: super::vals::_cmWakeupPllCtrlDisable,
        }
        let proxy = _cmWakeupPllCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM wakeup power control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmWakeupPowerCtrl(pub u32);
impl _cmWakeupPowerCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmWakeupPowerCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmWakeupPowerCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmWakeupPowerCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmWakeupPowerCtrl {
    #[inline(always)]
    fn default() -> _cmWakeupPowerCtrl {
        _cmWakeupPowerCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmWakeupPowerCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmWakeupPowerCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmWakeupPowerCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmWakeupPowerCtrl {
            disable: super::vals::_cmWakeupPowerCtrlDisable,
        }
        let proxy = _cmWakeupPowerCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM wakeup reset control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmWakeupResetCtrl(pub u32);
impl _cmWakeupResetCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmWakeupResetCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmWakeupResetCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmWakeupResetCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmWakeupResetCtrl {
    #[inline(always)]
    fn default() -> _cmWakeupResetCtrl {
        _cmWakeupResetCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmWakeupResetCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmWakeupResetCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmWakeupResetCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmWakeupResetCtrl {
            disable: super::vals::_cmWakeupResetCtrlDisable,
        }
        let proxy = _cmWakeupResetCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CM wakeup SSAR control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct _cmWakeupSsarCtrl(pub u32);
impl _cmWakeupSsarCtrl {
    #[doc = "Disable this step"]
    #[must_use]
    #[inline(always)]
    pub const fn disable(&self) -> super::vals::_cmWakeupSsarCtrlDisable {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::_cmWakeupSsarCtrlDisable::from_bits(val as u8)
    }
    #[doc = "Disable this step"]
    #[inline(always)]
    pub const fn set_disable(&mut self, val: super::vals::_cmWakeupSsarCtrlDisable) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for _cmWakeupSsarCtrl {
    #[inline(always)]
    fn default() -> _cmWakeupSsarCtrl {
        _cmWakeupSsarCtrl(4u64 as u32)
    }
}
impl core::fmt::Debug for _cmWakeupSsarCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_cmWakeupSsarCtrl")
            .field("disable", &self.disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for _cmWakeupSsarCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct _cmWakeupSsarCtrl {
            disable: super::vals::_cmWakeupSsarCtrlDisable,
        }
        let proxy = _cmWakeupSsarCtrl {
            disable: self.disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
