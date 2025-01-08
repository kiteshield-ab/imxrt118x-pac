#[doc = "SYS_CTR_CONTROL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrControl {
    ptr: *mut u8,
}
unsafe impl Send for SysCtrControl {}
unsafe impl Sync for SysCtrControl {}
impl SysCtrControl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Counter Control"]
    #[inline(always)]
    pub const fn cntcr(self) -> crate::common::Reg<regs::Cntcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Counter Status"]
    #[inline(always)]
    pub const fn cntsr(self) -> crate::common::Reg<regs::Cntsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Counter Count Value Low"]
    #[inline(always)]
    pub const fn cntcv0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Counter Count Value High"]
    #[inline(always)]
    pub const fn cntcv1(self) -> crate::common::Reg<regs::Cntcv1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Frequency-Modes Table 0"]
    #[inline(always)]
    pub const fn cntfid0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Frequency-Modes Table 1"]
    #[inline(always)]
    pub const fn cntfid1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Frequency-Modes Table 2"]
    #[inline(always)]
    pub const fn cntfid2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Counter Control 2"]
    #[inline(always)]
    pub const fn cntcr2(self) -> crate::common::Reg<regs::Cntcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Counter ID"]
    #[inline(always)]
    pub const fn cntid0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
}
pub mod regs;
