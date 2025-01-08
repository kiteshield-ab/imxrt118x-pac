#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OscRc400m {
    ptr: *mut u8,
}
unsafe impl Send for OscRc400m {}
unsafe impl Sync for OscRc400m {}
impl OscRc400m {
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
    #[doc = "Analog Control Register CTRL1"]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Analog Control Register CTRL1"]
    #[inline(always)]
    pub const fn ctrl1_set(self) -> crate::common::Reg<regs::Ctrl1Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Analog Control Register CTRL1"]
    #[inline(always)]
    pub const fn ctrl1_clr(self) -> crate::common::Reg<regs::Ctrl1Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Analog Control Register CTRL1"]
    #[inline(always)]
    pub const fn ctrl1_tog(self) -> crate::common::Reg<regs::Ctrl1Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Analog Control Register CTRL2"]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Analog Control Register CTRL2"]
    #[inline(always)]
    pub const fn ctrl2_set(self) -> crate::common::Reg<regs::Ctrl2Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Analog Control Register CTRL2"]
    #[inline(always)]
    pub const fn ctrl2_clr(self) -> crate::common::Reg<regs::Ctrl2Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Analog Control Register CTRL2"]
    #[inline(always)]
    pub const fn ctrl2_tog(self) -> crate::common::Reg<regs::Ctrl2Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Analog Control Register CTRL3"]
    #[inline(always)]
    pub const fn ctrl3(self) -> crate::common::Reg<regs::Ctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Analog Control Register CTRL3"]
    #[inline(always)]
    pub const fn ctrl3_set(self) -> crate::common::Reg<regs::Ctrl3Set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Analog Control Register CTRL3"]
    #[inline(always)]
    pub const fn ctrl3_clr(self) -> crate::common::Reg<regs::Ctrl3Clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Analog Control Register CTRL3"]
    #[inline(always)]
    pub const fn ctrl3_tog(self) -> crate::common::Reg<regs::Ctrl3Tog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Analog Status Register STAT0"]
    #[inline(always)]
    pub const fn stat0(self) -> crate::common::Reg<regs::Stat0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Analog Status Register STAT1"]
    #[inline(always)]
    pub const fn stat1(self) -> crate::common::Reg<regs::Stat1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Analog Status Register STAT2"]
    #[inline(always)]
    pub const fn stat2(self) -> crate::common::Reg<regs::Stat2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
}
pub mod regs;
pub mod vals;
