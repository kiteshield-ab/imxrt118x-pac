#[doc = "SYS_CTR_COMPARE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SysCtrCompare {
    ptr: *mut u8,
}
unsafe impl Send for SysCtrCompare {}
unsafe impl Sync for SysCtrCompare {}
impl SysCtrCompare {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Compare Count Value Low"]
    #[inline(always)]
    pub const fn cmpcvl0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Compare Count Value High"]
    #[inline(always)]
    pub const fn cmpcvh0(self) -> crate::common::Reg<regs::Cmpcvh0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Compare Control"]
    #[inline(always)]
    pub const fn cmpcr0(self) -> crate::common::Reg<regs::Cmpcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Compare Count Value Low"]
    #[inline(always)]
    pub const fn cmpcvl1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "Compare Count Value High"]
    #[inline(always)]
    pub const fn cmpcvh1(self) -> crate::common::Reg<regs::Cmpcvh1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "Compare Control"]
    #[inline(always)]
    pub const fn cmpcr1(self) -> crate::common::Reg<regs::Cmpcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Counter ID"]
    #[inline(always)]
    pub const fn cntid0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
}
pub mod regs;
