#[doc = "MSGINTR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msgintr {
    ptr: *mut u8,
}
unsafe impl Send for Msgintr {}
unsafe impl Sync for Msgintr {}
impl Msgintr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Message Signaled Interrupt Index Register 0"]
    #[inline(always)]
    pub const fn msiir0(self) -> crate::common::Reg<regs::Msiir0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Message Signaled Interrupt Register 0"]
    #[inline(always)]
    pub const fn msir0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Message Signaled Interrupt Index Register 1"]
    #[inline(always)]
    pub const fn msiir1(self) -> crate::common::Reg<regs::Msiir1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Message Signaled Interrupt Register 1"]
    #[inline(always)]
    pub const fn msir1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Message Signaled Interrupt Index Register 2"]
    #[inline(always)]
    pub const fn msiir2(self) -> crate::common::Reg<regs::Msiir2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Message Signaled Interrupt Register 2"]
    #[inline(always)]
    pub const fn msir2(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs;
