#[doc = "Event Collector Integrated Endpoint Register Block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IercIerb {
    ptr: *mut u8,
}
unsafe impl Send for IercIerb {}
unsafe impl Sync for IercIerb {}
impl IercIerb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Function 0 EC config header device ID and vendor ID register"]
    #[inline(always)]
    pub const fn f0_ec_cfh_didvid(
        self,
    ) -> crate::common::Reg<regs::F0EcCfhDidvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Function 0 EC config header subsystem ID and subsystem vendor ID register"]
    #[inline(always)]
    pub const fn f0_ec_cfh_sidsvid(
        self,
    ) -> crate::common::Reg<regs::F0EcCfhSidsvid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
