#[doc = "MEM Type II"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrcMifLn28fdsoiSpllram {
    ptr: *mut u8,
}
unsafe impl Send for SrcMifLn28fdsoiSpllram {}
unsafe impl Sync for SrcMifLn28fdsoiSpllram {}
impl SrcMifLn28fdsoiSpllram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MPC Control"]
    #[inline(always)]
    pub const fn mif_ctrl(self) -> crate::common::Reg<regs::MifCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "MIF Status"]
    #[inline(always)]
    pub const fn mif_stat(self) -> crate::common::Reg<regs::MifStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "MIF MLPL control of IG"]
    #[inline(always)]
    pub const fn mif_mlpl_ig(self) -> crate::common::Reg<regs::MifMlplIg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "MIF Delay of IG"]
    #[inline(always)]
    pub const fn mif_dly_ig(self) -> crate::common::Reg<regs::MifDlyIg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "MIF MLPL control of WLPD"]
    #[inline(always)]
    pub const fn mif_mlpl_wlpd(self) -> crate::common::Reg<regs::MifMlplWlpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "MIF Delay of WLPD"]
    #[inline(always)]
    pub const fn mif_dly_wlpd(self) -> crate::common::Reg<regs::MifDlyWlpd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "MIF MLPL control of PD_B"]
    #[inline(always)]
    pub const fn mif_mlpl_pd_b(self) -> crate::common::Reg<regs::MifMlplPdB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "MIF Delay of PD_B"]
    #[inline(always)]
    pub const fn mif_dly_pd_b(self) -> crate::common::Reg<regs::MifDlyPdB, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
}
pub mod regs;
