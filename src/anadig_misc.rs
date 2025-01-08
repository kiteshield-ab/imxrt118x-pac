#[doc = "RT1180_ANADIG_REGISTER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnadigMisc {
    ptr: *mut u8,
}
unsafe impl Send for AnadigMisc {}
unsafe impl Sync for AnadigMisc {}
impl AnadigMisc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Chip Silicon Version Register"]
    #[inline(always)]
    pub const fn misc_difprog(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4800usize) as _) }
    }
}
