#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyLdo {
    ptr: *mut u8,
}
unsafe impl Send for PhyLdo {}
unsafe impl Sync for PhyLdo {}
impl PhyLdo {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Analog Control Register CTRL0"]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Analog Control Register CTRL0"]
    #[inline(always)]
    pub const fn ctrl0_set(self) -> crate::common::Reg<regs::Ctrl0Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Analog Control Register CTRL0"]
    #[inline(always)]
    pub const fn ctrl0_clr(self) -> crate::common::Reg<regs::Ctrl0Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Analog Control Register CTRL0"]
    #[inline(always)]
    pub const fn ctrl0_tog(self) -> crate::common::Reg<regs::Ctrl0Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Analog Status Register STAT0"]
    #[inline(always)]
    pub const fn stat0(self) -> crate::common::Reg<regs::Stat0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Analog Status Register STAT0"]
    #[inline(always)]
    pub const fn stat0_set(self) -> crate::common::Reg<regs::Stat0Set, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Analog Status Register STAT0"]
    #[inline(always)]
    pub const fn stat0_clr(self) -> crate::common::Reg<regs::Stat0Clr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Analog Status Register STAT0"]
    #[inline(always)]
    pub const fn stat0_tog(self) -> crate::common::Reg<regs::Stat0Tog, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
