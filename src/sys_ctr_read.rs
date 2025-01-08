#[doc = "SYS_CTR_READ"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrRead {
    ptr: *mut u8,
}
unsafe impl Send for SysCtrRead {}
unsafe impl Sync for SysCtrRead {}
impl SysCtrRead {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Counter Count Value Low"]
    #[inline(always)]
    pub const fn cntcv0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Counter Count Value High"]
    #[inline(always)]
    pub const fn cntcv1(self) -> crate::common::Reg<regs::Cntcv1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Counter ID"]
    #[inline(always)]
    pub const fn cntid0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
}
pub mod regs;
