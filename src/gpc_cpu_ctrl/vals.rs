#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpuModeCurrent {
    #[doc = "CPU is currently in RUN mode"]
    B0 = 0x0,
    #[doc = "CPU is currently in WAIT mode"]
    B1 = 0x01,
    #[doc = "CPU is currently in STOP mode"]
    B2 = 0x02,
    #[doc = "CPU is currently in SUSPEND mode"]
    B3 = 0x03,
}
impl CpuModeCurrent {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpuModeCurrent {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpuModeCurrent {
    #[inline(always)]
    fn from(val: u8) -> CpuModeCurrent {
        CpuModeCurrent::from_bits(val)
    }
}
impl From<CpuModeCurrent> for u8 {
    #[inline(always)]
    fn from(val: CpuModeCurrent) -> u8 {
        CpuModeCurrent::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpuModePrevious {
    #[doc = "CPU was previously in RUN mode"]
    B0 = 0x0,
    #[doc = "CPU was previously in WAIT mode"]
    B1 = 0x01,
    #[doc = "CPU was previously in STOP mode"]
    B2 = 0x02,
    #[doc = "CPU was previously in SUSPEND mode"]
    B3 = 0x03,
}
impl CpuModePrevious {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpuModePrevious {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpuModePrevious {
    #[inline(always)]
    fn from(val: u8) -> CpuModePrevious {
        CpuModePrevious::from_bits(val)
    }
}
impl From<CpuModePrevious> for u8 {
    #[inline(always)]
    fn from(val: CpuModePrevious) -> u8 {
        CpuModePrevious::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CpuModeTarget {
    #[doc = "Stay in RUN mode"]
    B0 = 0x0,
    #[doc = "Transit to WAIT mode"]
    B1 = 0x01,
    #[doc = "Transit to STOP mode"]
    B2 = 0x02,
    #[doc = "Transit to SUSPEND mode"]
    B3 = 0x03,
}
impl CpuModeTarget {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> CpuModeTarget {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for CpuModeTarget {
    #[inline(always)]
    fn from(val: u8) -> CpuModeTarget {
        CpuModeTarget::from_bits(val)
    }
}
impl From<CpuModeTarget> for u8 {
    #[inline(always)]
    fn from(val: CpuModeTarget) -> u8 {
        CpuModeTarget::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DebugWakeupStat {
    #[doc = "No debug wakeup is requested"]
    B0 = 0x0,
    #[doc = "Debug wakeup is requested"]
    B1 = 0x01,
}
impl DebugWakeupStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DebugWakeupStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DebugWakeupStat {
    #[inline(always)]
    fn from(val: u8) -> DebugWakeupStat {
        DebugWakeupStat::from_bits(val)
    }
}
impl From<DebugWakeupStat> for u8 {
    #[inline(always)]
    fn from(val: DebugWakeupStat) -> u8 {
        DebugWakeupStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockCfg {
    #[doc = "The value of low power configuration fields are not locked."]
    B0 = 0x0,
    #[doc = "The value of low power configuration fields are locked. It locks the CPUx_CM registers which are marked as \"Locked by LOCK_CFG field\" in the function field."]
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
pub enum LockList {
    #[doc = "WHITE_LIST is not locked"]
    B0 = 0x0,
    #[doc = "WHITE_LIST is locked"]
    B1 = 0x01,
}
impl LockList {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockList {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockList {
    #[inline(always)]
    fn from(val: u8) -> LockList {
        LockList::from_bits(val)
    }
}
impl From<LockList> for u8 {
    #[inline(always)]
    fn from(val: LockList) -> u8 {
        LockList::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSetting {
    #[doc = "NONSECURE and USER fields are not locked"]
    B0 = 0x0,
    #[doc = "NONSECURE and USER fields are locked"]
    B1 = 0x01,
}
impl LockSetting {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSetting {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSetting {
    #[inline(always)]
    fn from(val: u8) -> LockSetting {
        LockSetting::from_bits(val)
    }
}
impl From<LockSetting> for u8 {
    #[inline(always)]
    fn from(val: LockSetting) -> u8 {
        LockSetting::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum NmiStat {
    #[doc = "NMI is not asserted"]
    B0 = 0x0,
    #[doc = "NMI is asserted"]
    B1 = 0x01,
}
impl NmiStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> NmiStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for NmiStat {
    #[inline(always)]
    fn from(val: u8) -> NmiStat {
        NmiStat::from_bits(val)
    }
}
impl From<NmiStat> for u8 {
    #[inline(always)]
    fn from(val: NmiStat) -> u8 {
        NmiStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nonsecure {
    #[doc = "Allow only secure mode to access CPU mode control"]
    B0 = 0x0,
    #[doc = "Allow both secure and non-secure mode to access CPU mode control registers"]
    B1 = 0x01,
}
impl Nonsecure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nonsecure {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nonsecure {
    #[inline(always)]
    fn from(val: u8) -> Nonsecure {
        Nonsecure::from_bits(val)
    }
}
impl From<Nonsecure> for u8 {
    #[inline(always)]
    fn from(val: Nonsecure) -> u8 {
        Nonsecure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SleepHoldEn {
    #[doc = "Disable cpu_sleep_hold_req"]
    B0 = 0x0,
    #[doc = "Allow cpu_sleep_hold_req to assert during CPU low power status"]
    B1 = 0x01,
}
impl SleepHoldEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SleepHoldEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SleepHoldEn {
    #[inline(always)]
    fn from(val: u8) -> SleepHoldEn {
        SleepHoldEn::from_bits(val)
    }
}
impl From<SleepHoldEn> for u8 {
    #[inline(always)]
    fn from(val: SleepHoldEn) -> u8 {
        SleepHoldEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SleepHoldStat {
    #[doc = "CPU sleep hold is acknowledged"]
    B0 = 0x0,
    #[doc = "CPU is not in sleep hold"]
    B1 = 0x01,
}
impl SleepHoldStat {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SleepHoldStat {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SleepHoldStat {
    #[inline(always)]
    fn from(val: u8) -> SleepHoldStat {
        SleepHoldStat::from_bits(val)
    }
}
impl From<SleepHoldStat> for u8 {
    #[inline(always)]
    fn from(val: SleepHoldStat) -> u8 {
        SleepHoldStat::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsStop {
    #[doc = "Do not request system sleep when CPU is in STOP mode"]
    B0 = 0x0,
    #[doc = "Request system sleep when CPU is in STOP mode"]
    B1 = 0x01,
}
impl SsStop {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsStop {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsStop {
    #[inline(always)]
    fn from(val: u8) -> SsStop {
        SsStop::from_bits(val)
    }
}
impl From<SsStop> for u8 {
    #[inline(always)]
    fn from(val: SsStop) -> u8 {
        SsStop::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsSuspend {
    #[doc = "Do not request system sleep when CPU is in SUSPEND mode"]
    B0 = 0x0,
    #[doc = "Request system sleep when CPU is in SUSPEND mode"]
    B1 = 0x01,
}
impl SsSuspend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsSuspend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsSuspend {
    #[inline(always)]
    fn from(val: u8) -> SsSuspend {
        SsSuspend::from_bits(val)
    }
}
impl From<SsSuspend> for u8 {
    #[inline(always)]
    fn from(val: SsSuspend) -> u8 {
        SsSuspend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SsWait {
    #[doc = "Do not request system sleep when CPU is in WAIT mode"]
    B0 = 0x0,
    #[doc = "Request system sleep when CPU is in WAIT mode"]
    B1 = 0x01,
}
impl SsWait {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SsWait {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SsWait {
    #[inline(always)]
    fn from(val: u8) -> SsWait {
        SsWait::from_bits(val)
    }
}
impl From<SsWait> for u8 {
    #[inline(always)]
    fn from(val: SsWait) -> u8 {
        SsWait::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum User {
    #[doc = "Allow only privilege mode to access CPU mode control registers"]
    B0 = 0x0,
    #[doc = "Allow both privilege and user mode to access CPU mode control registers"]
    B1 = 0x01,
}
impl User {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> User {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for User {
    #[inline(always)]
    fn from(val: u8) -> User {
        User::from_bits(val)
    }
}
impl From<User> for u8 {
    #[inline(always)]
    fn from(val: User) -> u8 {
        User::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WfeEn {
    #[doc = "WFE assertion can not trigger low power"]
    B0 = 0x0,
    #[doc = "WFE assertion can trigger low power"]
    B1 = 0x01,
}
impl WfeEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WfeEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WfeEn {
    #[inline(always)]
    fn from(val: u8) -> WfeEn {
        WfeEn::from_bits(val)
    }
}
impl From<WfeEn> for u8 {
    #[inline(always)]
    fn from(val: WfeEn) -> u8 {
        WfeEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmSleepIsoCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmSleepIsoCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmSleepIsoCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmSleepIsoCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmSleepIsoCtrlDisable {
        _cmSleepIsoCtrlDisable::from_bits(val)
    }
}
impl From<_cmSleepIsoCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmSleepIsoCtrlDisable) -> u8 {
        _cmSleepIsoCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmSleepLpcgCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmSleepLpcgCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmSleepLpcgCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmSleepLpcgCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmSleepLpcgCtrlDisable {
        _cmSleepLpcgCtrlDisable::from_bits(val)
    }
}
impl From<_cmSleepLpcgCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmSleepLpcgCtrlDisable) -> u8 {
        _cmSleepLpcgCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmSleepPllCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmSleepPllCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmSleepPllCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmSleepPllCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmSleepPllCtrlDisable {
        _cmSleepPllCtrlDisable::from_bits(val)
    }
}
impl From<_cmSleepPllCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmSleepPllCtrlDisable) -> u8 {
        _cmSleepPllCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmSleepPowerCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmSleepPowerCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmSleepPowerCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmSleepPowerCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmSleepPowerCtrlDisable {
        _cmSleepPowerCtrlDisable::from_bits(val)
    }
}
impl From<_cmSleepPowerCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmSleepPowerCtrlDisable) -> u8 {
        _cmSleepPowerCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmSleepResetCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmSleepResetCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmSleepResetCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmSleepResetCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmSleepResetCtrlDisable {
        _cmSleepResetCtrlDisable::from_bits(val)
    }
}
impl From<_cmSleepResetCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmSleepResetCtrlDisable) -> u8 {
        _cmSleepResetCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmSleepSsarCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmSleepSsarCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmSleepSsarCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmSleepSsarCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmSleepSsarCtrlDisable {
        _cmSleepSsarCtrlDisable::from_bits(val)
    }
}
impl From<_cmSleepSsarCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmSleepSsarCtrlDisable) -> u8 {
        _cmSleepSsarCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmWakeupIsoCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmWakeupIsoCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmWakeupIsoCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmWakeupIsoCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmWakeupIsoCtrlDisable {
        _cmWakeupIsoCtrlDisable::from_bits(val)
    }
}
impl From<_cmWakeupIsoCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmWakeupIsoCtrlDisable) -> u8 {
        _cmWakeupIsoCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmWakeupLpcgCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmWakeupLpcgCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmWakeupLpcgCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmWakeupLpcgCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmWakeupLpcgCtrlDisable {
        _cmWakeupLpcgCtrlDisable::from_bits(val)
    }
}
impl From<_cmWakeupLpcgCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmWakeupLpcgCtrlDisable) -> u8 {
        _cmWakeupLpcgCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmWakeupPllCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmWakeupPllCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmWakeupPllCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmWakeupPllCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmWakeupPllCtrlDisable {
        _cmWakeupPllCtrlDisable::from_bits(val)
    }
}
impl From<_cmWakeupPllCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmWakeupPllCtrlDisable) -> u8 {
        _cmWakeupPllCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmWakeupPowerCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmWakeupPowerCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmWakeupPowerCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmWakeupPowerCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmWakeupPowerCtrlDisable {
        _cmWakeupPowerCtrlDisable::from_bits(val)
    }
}
impl From<_cmWakeupPowerCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmWakeupPowerCtrlDisable) -> u8 {
        _cmWakeupPowerCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmWakeupResetCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmWakeupResetCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmWakeupResetCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmWakeupResetCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmWakeupResetCtrlDisable {
        _cmWakeupResetCtrlDisable::from_bits(val)
    }
}
impl From<_cmWakeupResetCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmWakeupResetCtrlDisable) -> u8 {
        _cmWakeupResetCtrlDisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum _cmWakeupSsarCtrlDisable {
    #[doc = "This step is enabled."]
    B0 = 0x0,
    #[doc = "This step is disabled. GPC will skip this step and not send any request."]
    B1 = 0x01,
}
impl _cmWakeupSsarCtrlDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> _cmWakeupSsarCtrlDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for _cmWakeupSsarCtrlDisable {
    #[inline(always)]
    fn from(val: u8) -> _cmWakeupSsarCtrlDisable {
        _cmWakeupSsarCtrlDisable::from_bits(val)
    }
}
impl From<_cmWakeupSsarCtrlDisable> for u8 {
    #[inline(always)]
    fn from(val: _cmWakeupSsarCtrlDisable) -> u8 {
        _cmWakeupSsarCtrlDisable::to_bits(val)
    }
}
