#[doc = "NETC privileged"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPriv {
    ptr: *mut u8,
}
unsafe impl Send for NetcPriv {}
unsafe impl Sync for NetcPriv {}
impl NetcPriv {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "NETC reset register"]
    #[inline(always)]
    pub const fn netcrr(self) -> crate::common::Reg<regs::Netcrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "NETC status register"]
    #[inline(always)]
    pub const fn netcsr(self) -> crate::common::Reg<regs::Netcsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Memory Error Injection Config Register"]
    #[inline(always)]
    pub const fn meicr(self) -> crate::common::Reg<regs::Meicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Correctable memory error configuration register"]
    #[inline(always)]
    pub const fn cmecr(self) -> crate::common::Reg<regs::Cmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e00usize) as _) }
    }
    #[doc = "Correctable memory error status register"]
    #[inline(always)]
    pub const fn cmesr(self) -> crate::common::Reg<regs::Cmesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e04usize) as _) }
    }
    #[doc = "Correctable memory error count register"]
    #[inline(always)]
    pub const fn cmectr(self) -> crate::common::Reg<regs::Cmectr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e0cusize) as _) }
    }
    #[doc = "Uncorrectable non-fatal memory error configuration register"]
    #[inline(always)]
    pub const fn unmecr(self) -> crate::common::Reg<regs::Unmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e30usize) as _) }
    }
    #[doc = "Uncorrectable non-fatal memory error status register 0"]
    #[inline(always)]
    pub const fn unmesr0(self) -> crate::common::Reg<regs::Unmesr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e34usize) as _) }
    }
    #[doc = "Uncorrectable non-fatal memory error status register 1"]
    #[inline(always)]
    pub const fn unmesr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e38usize) as _) }
    }
    #[doc = "Uncorrectable non-fatal memory error count register"]
    #[inline(always)]
    pub const fn unmectr(self) -> crate::common::Reg<regs::Unmectr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e3cusize) as _) }
    }
    #[doc = "Uncorrectable fatal memory error configuration register"]
    #[inline(always)]
    pub const fn ufmecr(self) -> crate::common::Reg<regs::Ufmecr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e40usize) as _) }
    }
    #[doc = "Uncorrectable fatal memory error status register 0"]
    #[inline(always)]
    pub const fn ufmesr0(self) -> crate::common::Reg<regs::Ufmesr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e44usize) as _) }
    }
    #[doc = "Uncorrectable fatal memory error status register 1"]
    #[inline(always)]
    pub const fn ufmesr1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e48usize) as _) }
    }
}
pub mod regs;
