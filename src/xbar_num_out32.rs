#[doc = "XBAR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XbarNumOut32 {
    ptr: *mut u8,
}
unsafe impl Send for XbarNumOut32 {}
unsafe impl Sync for XbarNumOut32 {}
impl XbarNumOut32 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel0(self) -> crate::common::Reg<regs::Sel0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel1(self) -> crate::common::Reg<regs::Sel1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel2(self) -> crate::common::Reg<regs::Sel2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel3(self) -> crate::common::Reg<regs::Sel3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel4(self) -> crate::common::Reg<regs::Sel4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel5(self) -> crate::common::Reg<regs::Sel5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel6(self) -> crate::common::Reg<regs::Sel6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel7(self) -> crate::common::Reg<regs::Sel7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel8(self) -> crate::common::Reg<regs::Sel8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel9(self) -> crate::common::Reg<regs::Sel9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x12usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel10(self) -> crate::common::Reg<regs::Sel10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel11(self) -> crate::common::Reg<regs::Sel11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x16usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel12(self) -> crate::common::Reg<regs::Sel12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel13(self) -> crate::common::Reg<regs::Sel13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1ausize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel14(self) -> crate::common::Reg<regs::Sel14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Crossbar Select Register"]
    #[inline(always)]
    pub const fn sel15(self) -> crate::common::Reg<regs::Sel15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1eusize) as _) }
    }
}
pub mod regs;
