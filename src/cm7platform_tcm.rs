#[doc = "MEM Type I with PSW"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cm7platformTcm {
    ptr: *mut u8,
}
unsafe impl Send for Cm7platformTcm {}
unsafe impl Sync for Cm7platformTcm {}
impl Cm7platformTcm {
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
    #[doc = "MIF MLPL control of LS"]
    #[inline(always)]
    pub const fn mif_mlpl_ls(self) -> crate::common::Reg<regs::MifMlplLs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "MIF Delay of LS"]
    #[inline(always)]
    pub const fn mif_dly_ls(self) -> crate::common::Reg<regs::MifDlyLs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "MIF MLPL control of HS"]
    #[inline(always)]
    pub const fn mif_mlpl_hs(self) -> crate::common::Reg<regs::MifMlplHs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "MIF Delay of HS"]
    #[inline(always)]
    pub const fn mif_dly_hs(self) -> crate::common::Reg<regs::MifDlyHs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "MIF MLPL control of Input Gating (IG)"]
    #[inline(always)]
    pub const fn mif_mlpl_ig(self) -> crate::common::Reg<regs::MifMlplIg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "MIF Delay of IG"]
    #[inline(always)]
    pub const fn mif_dly_ig(self) -> crate::common::Reg<regs::MifDlyIg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "MIF MLPL control of STDBY"]
    #[inline(always)]
    pub const fn mif_mlpl_stdby(self) -> crate::common::Reg<regs::MifMlplStdby, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "MIF Delay of STDBY"]
    #[inline(always)]
    pub const fn mif_dly_stdby(self) -> crate::common::Reg<regs::MifDlyStdby, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "MIF MLPL control of SLEEP"]
    #[inline(always)]
    pub const fn mif_mlpl_sleep(self) -> crate::common::Reg<regs::MifMlplSleep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "MIF Delay of SLEEP"]
    #[inline(always)]
    pub const fn mif_dly_sleep(self) -> crate::common::Reg<regs::MifDlySleep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "MIF MLPL control of array power down"]
    #[inline(always)]
    pub const fn mif_mlpl_arr_pdn(
        self,
    ) -> crate::common::Reg<regs::MifMlplArrPdn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "MIF Delay of array high-fanout power switch"]
    #[inline(always)]
    pub const fn mif_dly_arr_hf(self) -> crate::common::Reg<regs::MifDlyArrHf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
}
pub mod regs;
pub mod vals;
