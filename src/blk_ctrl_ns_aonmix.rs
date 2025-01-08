#[doc = "Block Control Non-Secure AON Domain"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlkCtrlNsAonmix {
    ptr: *mut u8,
}
unsafe impl Send for BlkCtrlNsAonmix {}
unsafe impl Sync for BlkCtrlNsAonmix {}
impl BlkCtrlNsAonmix {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPC CORE SLEEP Request Select"]
    #[inline(always)]
    pub const fn gpc_cfg(self) -> crate::common::Reg<regs::GpcCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "IPG Debug mask"]
    #[inline(always)]
    pub const fn ipg_debug(self) -> crate::common::Reg<regs::IpgDebug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SSI Master: AXI Async Bridge from AXIM to NIC400. Low power mode control."]
    #[inline(always)]
    pub const fn ssi(self) -> crate::common::Reg<regs::Ssi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SAI1 MCLK control register"]
    #[inline(always)]
    pub const fn sai1_mclk_ctrl(self) -> crate::common::Reg<regs::Sai1MclkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "DCDC status register"]
    #[inline(always)]
    pub const fn dcdc_status(self) -> crate::common::Reg<regs::DcdcStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Fuse access disable register"]
    #[inline(always)]
    pub const fn fuse_acc_dis(self) -> crate::common::Reg<regs::FuseAccDis, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "M33 NMI interrupt clear register"]
    #[inline(always)]
    pub const fn m33_nmi_clr(self) -> crate::common::Reg<regs::M33NmiClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "I3C1 async wakeup control register"]
    #[inline(always)]
    pub const fn i3c1_async_wakeup_ctrl(
        self,
    ) -> crate::common::Reg<regs::I3c1AsyncWakeupCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Miscellaneous control register of IO"]
    #[inline(always)]
    pub const fn misc_io_ctrl(self) -> crate::common::Reg<regs::MiscIoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
}
pub mod regs;
pub mod vals;
