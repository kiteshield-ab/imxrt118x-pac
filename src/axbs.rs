#[doc = "AXBS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Axbs {
    ptr: *mut u8,
}
unsafe impl Send for Axbs {}
unsafe impl Sync for Axbs {}
impl Axbs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Priority Slave Registers"]
    #[inline(always)]
    pub const fn prs0(self) -> crate::common::Reg<regs::Prs0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn crs0(self) -> crate::common::Reg<regs::Crs0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Priority Slave Registers"]
    #[inline(always)]
    pub const fn prs1(self) -> crate::common::Reg<regs::Prs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn crs1(self) -> crate::common::Reg<regs::Crs1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Priority Slave Registers"]
    #[inline(always)]
    pub const fn prs2(self) -> crate::common::Reg<regs::Prs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn crs2(self) -> crate::common::Reg<regs::Crs2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "Priority Slave Registers"]
    #[inline(always)]
    pub const fn prs3(self) -> crate::common::Reg<regs::Prs3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn crs3(self) -> crate::common::Reg<regs::Crs3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "Priority Slave Registers"]
    #[inline(always)]
    pub const fn prs4(self) -> crate::common::Reg<regs::Prs4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn crs4(self) -> crate::common::Reg<regs::Crs4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "Priority Slave Registers"]
    #[inline(always)]
    pub const fn prs5(self) -> crate::common::Reg<regs::Prs5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn crs5(self) -> crate::common::Reg<regs::Crs5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "Priority Slave Registers"]
    #[inline(always)]
    pub const fn prs6(self) -> crate::common::Reg<regs::Prs6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn crs6(self) -> crate::common::Reg<regs::Crs6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize) as _) }
    }
    #[doc = "Priority Slave Registers"]
    #[inline(always)]
    pub const fn prs7(self) -> crate::common::Reg<regs::Prs7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn crs7(self) -> crate::common::Reg<regs::Crs7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0710usize) as _) }
    }
}
pub mod regs;
pub mod vals;
