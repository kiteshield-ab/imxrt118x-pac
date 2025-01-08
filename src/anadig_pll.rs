#[doc = "RT1180_ANADIG_REGISTER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnadigPll {
    ptr: *mut u8,
}
unsafe impl Send for AnadigPll {}
unsafe impl Sync for AnadigPll {}
impl AnadigPll {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ARM_PLL_CTRL_REGISTER"]
    #[inline(always)]
    pub const fn arm_pll_ctrl(self) -> crate::common::Reg<regs::ArmPllCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4000usize) as _) }
    }
    #[doc = "SYS_PLL3_CTRL_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll3_ctrl(self) -> crate::common::Reg<regs::SysPll3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4010usize) as _) }
    }
    #[doc = "SYS_PLL3_UPDATE_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll3_update(
        self,
    ) -> crate::common::Reg<regs::SysPll3Update, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4020usize) as _) }
    }
    #[doc = "SYS_PLL3_PFD_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll3_pfd(self) -> crate::common::Reg<regs::SysPll3Pfd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4030usize) as _) }
    }
    #[doc = "SYS_PLL2_CTRL_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll2_ctrl(self) -> crate::common::Reg<regs::SysPll2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4040usize) as _) }
    }
    #[doc = "SYS_PLL2_UPDATE_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll2_update(
        self,
    ) -> crate::common::Reg<regs::SysPll2Update, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4050usize) as _) }
    }
    #[doc = "SYS_PLL2_SS_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll2_ss(self) -> crate::common::Reg<regs::SysPll2Ss, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4060usize) as _) }
    }
    #[doc = "SYS_PLL2_PFD_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll2_pfd(self) -> crate::common::Reg<regs::SysPll2Pfd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4070usize) as _) }
    }
    #[doc = "SYS_PLL2_MFN_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll2_mfn(self) -> crate::common::Reg<regs::SysPll2Mfn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4080usize) as _) }
    }
    #[doc = "SYS_PLL2_MFI_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll2_mfi(self) -> crate::common::Reg<regs::SysPll2Mfi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4090usize) as _) }
    }
    #[doc = "SYS_PLL2_MFD_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll2_mfd(self) -> crate::common::Reg<regs::SysPll2Mfd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40a0usize) as _) }
    }
    #[doc = "SYS_PLL1_CTRL_REGISTER"]
    #[inline(always)]
    pub const fn sys_pll1_ctrl(self) -> crate::common::Reg<regs::SysPll1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4100usize) as _) }
    }
    #[doc = "PLL_AUDIO_CTRL_REGISTER"]
    #[inline(always)]
    pub const fn pll_audio_ctrl(self) -> crate::common::Reg<regs::PllAudioCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4200usize) as _) }
    }
}
pub mod regs;
pub mod vals;
