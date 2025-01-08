#[doc = "TMPSNS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpsns {
    ptr: *mut u8,
}
unsafe impl Send for Tmpsns {}
unsafe impl Sync for Tmpsns {}
impl Tmpsns {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control 1"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Control 1"]
    #[inline(always)]
    pub const fn ctrl1_set(self) -> crate::common::Reg<regs::Ctrl1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Control 1"]
    #[inline(always)]
    pub const fn ctrl1_clr(self) -> crate::common::Reg<regs::Ctrl1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Control 1"]
    #[inline(always)]
    pub const fn ctrl1_tog(self) -> crate::common::Reg<regs::Ctrl1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Range 0"]
    #[inline(always)]
    pub const fn range0(self) -> crate::common::Reg<regs::Range0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Range 0"]
    #[inline(always)]
    pub const fn range0_set(self) -> crate::common::Reg<regs::Range0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Range 0"]
    #[inline(always)]
    pub const fn range0_clr(self) -> crate::common::Reg<regs::Range0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Range 0"]
    #[inline(always)]
    pub const fn range0_tog(self) -> crate::common::Reg<regs::Range0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub const fn range1(self) -> crate::common::Reg<regs::Range1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub const fn range1_set(self) -> crate::common::Reg<regs::Range1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub const fn range1_clr(self) -> crate::common::Reg<regs::Range1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Range 1"]
    #[inline(always)]
    pub const fn range1_tog(self) -> crate::common::Reg<regs::Range1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Status 0"]
    #[inline(always)]
    pub const fn status0(self) -> crate::common::Reg<regs::Status0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
pub mod regs;
pub mod vals;
