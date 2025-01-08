#[doc = "SRC General"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrcGeneralReg {
    ptr: *mut u8,
}
unsafe impl Send for SrcGeneralReg {}
unsafe impl Sync for SrcGeneralReg {}
impl SrcGeneralReg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Authentication Control"]
    #[inline(always)]
    pub const fn authen_ctrl(self) -> crate::common::Reg<regs::AuthenCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SRC Control Register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SRC Reset Trigger Mode Register"]
    #[inline(always)]
    pub const fn srtmr(self) -> crate::common::Reg<regs::Srtmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SRC Reset Mask Register"]
    #[inline(always)]
    pub const fn srmask(self) -> crate::common::Reg<regs::Srmask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SRC Boot Mode Register 1"]
    #[inline(always)]
    pub const fn sbmr1(self) -> crate::common::Reg<regs::Sbmr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "SRC Boot Mode Register 2"]
    #[inline(always)]
    pub const fn sbmr2(self) -> crate::common::Reg<regs::Sbmr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "SRC Reset Status Register backup in BBSM domain"]
    #[inline(always)]
    pub const fn srsr_bbsm(self) -> crate::common::Reg<regs::SrsrBbsm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "SRC Reset Status Register"]
    #[inline(always)]
    pub const fn srsr(self) -> crate::common::Reg<regs::Srsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "SRC General Purpose Register"]
    #[inline(always)]
    pub const fn gpr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 20usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
