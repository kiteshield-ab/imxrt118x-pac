#[doc = "CMX_PERFMON"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspm {
    ptr: *mut u8,
}
unsafe impl Send for Syspm {}
unsafe impl Sync for Syspm {}
impl Syspm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Performance Monitor Control"]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Performance Monitor Instruction Counter"]
    #[inline(always)]
    pub const fn pmictr_hi(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Performance Monitor Instruction Counter"]
    #[inline(always)]
    pub const fn pmictr_lo(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Performance Monitor Event Counter"]
    #[inline(always)]
    pub const fn pmectr1_hi(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Performance Monitor Event Counter"]
    #[inline(always)]
    pub const fn pmectr1_lo(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Performance Monitor Event Counter"]
    #[inline(always)]
    pub const fn pmectr2_hi(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Performance Monitor Event Counter"]
    #[inline(always)]
    pub const fn pmectr2_lo(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Performance Monitor Event Counter"]
    #[inline(always)]
    pub const fn pmectr3_hi(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Performance Monitor Event Counter"]
    #[inline(always)]
    pub const fn pmectr3_lo(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
