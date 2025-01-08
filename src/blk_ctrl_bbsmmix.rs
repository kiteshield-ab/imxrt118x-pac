#[doc = "blk_ctrl_bbsmmix"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlkCtrlBbsmmix {
    ptr: *mut u8,
}
unsafe impl Send for BlkCtrlBbsmmix {}
unsafe impl Sync for BlkCtrlBbsmmix {}
impl BlkCtrlBbsmmix {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BBSM miscellaneous register"]
    #[inline(always)]
    pub const fn bbsm_misc(self) -> crate::common::Reg<regs::BbsmMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "BBSM TRIM register"]
    #[inline(always)]
    pub const fn bbsm_trim(self) -> crate::common::Reg<regs::BbsmTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
