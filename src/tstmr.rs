#[doc = "TSTMR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstmr {
    ptr: *mut u8,
}
unsafe impl Send for Tstmr {}
unsafe impl Sync for Tstmr {}
impl Tstmr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Timestamp Timer Low"]
    #[inline(always)]
    pub const fn low(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Timestamp Timer High"]
    #[inline(always)]
    pub const fn high(self) -> crate::common::Reg<regs::High, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
