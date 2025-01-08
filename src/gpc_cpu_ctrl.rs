#[doc = "Array of registers: _CM_AUTHEN_CTRL, _CM_MISC, _CM_MODE_CTRL, _CM_MODE_STAT, _CM_IRQ_WAKEUP_MASK_0, _CM_IRQ_WAKEUP_MASK_1, _CM_IRQ_WAKEUP_MASK_2, _CM_IRQ_WAKEUP_MASK_3, _CM_IRQ_WAKEUP_MASK_4, _CM_IRQ_WAKEUP_MASK_5, _CM_IRQ_WAKEUP_MASK_6, _CM_IRQ_WAKEUP_MASK_7, _CM_NON_IRQ_WAKEUP_MASK, _CM_IRQ_WAKEUP_STAT_0, _CM_IRQ_WAKEUP_STAT_1, _CM_IRQ_WAKEUP_STAT_2, _CM_IRQ_WAKEUP_STAT_3, _CM_IRQ_WAKEUP_STAT_4, _CM_IRQ_WAKEUP_STAT_5, _CM_IRQ_WAKEUP_STAT_6, _CM_IRQ_WAKEUP_STAT_7, _CM_NON_IRQ_WAKEUP_STAT, _CM_SLEEP_SSAR_CTRL, _CM_SLEEP_LPCG_CTRL, _CM_SLEEP_PLL_CTRL, _CM_SLEEP_ISO_CTRL, _CM_SLEEP_RESET_CTRL, _CM_SLEEP_POWER_CTRL, _CM_WAKEUP_POWER_CTRL, _CM_WAKEUP_RESET_CTRL, _CM_WAKEUP_ISO_CTRL, _CM_WAKEUP_PLL_CTRL, _CM_WAKEUP_LPCG_CTRL, _CM_WAKEUP_SSAR_CTRL, _CM_SYS_SLEEP_CTRL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Authen {
    ptr: *mut u8,
}
unsafe impl Send for Authen {}
unsafe impl Sync for Authen {}
impl Authen {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CM Authentication Control"]
    #[inline(always)]
    pub const fn _cm_authen_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmAuthenCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Miscellaneous"]
    #[inline(always)]
    pub const fn _cm_misc(self) -> crate::common::Reg<regs::_cmMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "CPU mode control"]
    #[inline(always)]
    pub const fn _cm_mode_ctrl(self) -> crate::common::Reg<regs::_cmModeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "CM CPU mode Status"]
    #[inline(always)]
    pub const fn _cm_mode_stat(self) -> crate::common::Reg<regs::_cmModeStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "CM IRQ0~31 wakeup mask"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_mask_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "CM IRQ32~63 wakeup mask"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_mask_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "CM IRQ64~95 wakeup mask"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_mask_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "CM IRQ96~127 wakeup mask"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_mask_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "CM IRQ128~159 wakeup mask"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_mask_4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "CM IRQ160~191 wakeup mask"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_mask_5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "CM IRQ192~223 wakeup mask"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_mask_6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "CM IRQ224~255 wakeup mask"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_mask_7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "CM non-IRQ wakeup mask"]
    #[inline(always)]
    pub const fn _cm_non_irq_wakeup_mask(
        self,
    ) -> crate::common::Reg<regs::_cmNonIrqWakeupMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "CM IRQ0~31 wakeup status"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_stat_0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "CM IRQ32~63 wakeup status"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_stat_1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "CM IRQ64~95 wakeup status"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_stat_2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "CM IRQ96~127 wakeup status"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_stat_3(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "CM IRQ128~159 wakeup status"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_stat_4(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "CM IRQ160~191 wakeup status"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_stat_5(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "CM IRQ192~223 wakeup status"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_stat_6(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "CM IRQ224~255 wakeup status"]
    #[inline(always)]
    pub const fn _cm_irq_wakeup_stat_7(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "CM non-irq wakeup status"]
    #[inline(always)]
    pub const fn _cm_non_irq_wakeup_stat(
        self,
    ) -> crate::common::Reg<regs::_cmNonIrqWakeupStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "CM sleep SSAR control"]
    #[inline(always)]
    pub const fn _cm_sleep_ssar_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmSleepSsarCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "CM sleep LPCG control"]
    #[inline(always)]
    pub const fn _cm_sleep_lpcg_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmSleepLpcgCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "CM sleep PLL control"]
    #[inline(always)]
    pub const fn _cm_sleep_pll_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmSleepPllCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "CM sleep isolation control"]
    #[inline(always)]
    pub const fn _cm_sleep_iso_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmSleepIsoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "CM sleep reset control"]
    #[inline(always)]
    pub const fn _cm_sleep_reset_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmSleepResetCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "CM sleep power control"]
    #[inline(always)]
    pub const fn _cm_sleep_power_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmSleepPowerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[doc = "CM wakeup power control"]
    #[inline(always)]
    pub const fn _cm_wakeup_power_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmWakeupPowerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[doc = "CM wakeup reset control"]
    #[inline(always)]
    pub const fn _cm_wakeup_reset_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmWakeupResetCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0298usize) as _) }
    }
    #[doc = "CM wakeup isolation control"]
    #[inline(always)]
    pub const fn _cm_wakeup_iso_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmWakeupIsoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[doc = "CM wakeup PLL control"]
    #[inline(always)]
    pub const fn _cm_wakeup_pll_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmWakeupPllCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a8usize) as _) }
    }
    #[doc = "CM wakeup LPCG control"]
    #[inline(always)]
    pub const fn _cm_wakeup_lpcg_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmWakeupLpcgCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize) as _) }
    }
    #[doc = "CM wakeup SSAR control"]
    #[inline(always)]
    pub const fn _cm_wakeup_ssar_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmWakeupSsarCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[doc = "CM system sleep control"]
    #[inline(always)]
    pub const fn _cm_sys_sleep_ctrl(
        self,
    ) -> crate::common::Reg<regs::_cmSysSleepCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
}
#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpcCpuCtrl {
    ptr: *mut u8,
}
unsafe impl Send for GpcCpuCtrl {}
unsafe impl Sync for GpcCpuCtrl {}
impl GpcCpuCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Array of registers: _CM_AUTHEN_CTRL, _CM_MISC, _CM_MODE_CTRL, _CM_MODE_STAT, _CM_IRQ_WAKEUP_MASK_0, _CM_IRQ_WAKEUP_MASK_1, _CM_IRQ_WAKEUP_MASK_2, _CM_IRQ_WAKEUP_MASK_3, _CM_IRQ_WAKEUP_MASK_4, _CM_IRQ_WAKEUP_MASK_5, _CM_IRQ_WAKEUP_MASK_6, _CM_IRQ_WAKEUP_MASK_7, _CM_NON_IRQ_WAKEUP_MASK, _CM_IRQ_WAKEUP_STAT_0, _CM_IRQ_WAKEUP_STAT_1, _CM_IRQ_WAKEUP_STAT_2, _CM_IRQ_WAKEUP_STAT_3, _CM_IRQ_WAKEUP_STAT_4, _CM_IRQ_WAKEUP_STAT_5, _CM_IRQ_WAKEUP_STAT_6, _CM_IRQ_WAKEUP_STAT_7, _CM_NON_IRQ_WAKEUP_STAT, _CM_SLEEP_SSAR_CTRL, _CM_SLEEP_LPCG_CTRL, _CM_SLEEP_PLL_CTRL, _CM_SLEEP_ISO_CTRL, _CM_SLEEP_RESET_CTRL, _CM_SLEEP_POWER_CTRL, _CM_WAKEUP_POWER_CTRL, _CM_WAKEUP_RESET_CTRL, _CM_WAKEUP_ISO_CTRL, _CM_WAKEUP_PLL_CTRL, _CM_WAKEUP_LPCG_CTRL, _CM_WAKEUP_SSAR_CTRL, _CM_SYS_SLEEP_CTRL"]
    #[inline(always)]
    pub const fn authen(self, n: usize) -> Authen {
        assert!(n < 2usize);
        unsafe { Authen::from_ptr(self.ptr.add(0x0usize + n * 2048usize) as _) }
    }
}
pub mod regs;
pub mod vals;
