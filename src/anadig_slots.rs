#[doc = "IPS Domain"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnadigSlots {
    ptr: *mut u8,
}
unsafe impl Send for AnadigSlots {}
unsafe impl Sync for AnadigSlots {}
impl AnadigSlots {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Slot Control Register"]
    #[inline(always)]
    pub const fn slot_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SlotCtrl, crate::common::RW> {
        assert!(n < 35usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4c00usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
