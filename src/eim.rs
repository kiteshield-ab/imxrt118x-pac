#[doc = "EIM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eim {
    ptr: *mut u8,
}
unsafe impl Send for Eim {}
unsafe impl Sync for Eim {}
impl Eim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Error Injection Module Configuration Register"]
    #[inline(always)]
    pub const fn eimcr(self) -> crate::common::Reg<regs::Eimcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Error Injection Channel Enable register"]
    #[inline(always)]
    pub const fn eichen(self) -> crate::common::Reg<regs::Eichen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word0"]
    #[inline(always)]
    pub const fn eichd0_word0(self) -> crate::common::Reg<regs::Eichd0Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word1"]
    #[inline(always)]
    pub const fn eichd0_word1(self) -> crate::common::Reg<regs::Eichd0Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 0, Word2"]
    #[inline(always)]
    pub const fn eichd0_word2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word0"]
    #[inline(always)]
    pub const fn eichd1_word0(self) -> crate::common::Reg<regs::Eichd1Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word1"]
    #[inline(always)]
    pub const fn eichd1_word1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word2"]
    #[inline(always)]
    pub const fn eichd1_word2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word3"]
    #[inline(always)]
    pub const fn eichd1_word3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 1, Word4"]
    #[inline(always)]
    pub const fn eichd1_word4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word0"]
    #[inline(always)]
    pub const fn eichd2_word0(self) -> crate::common::Reg<regs::Eichd2Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word1"]
    #[inline(always)]
    pub const fn eichd2_word1(self) -> crate::common::Reg<regs::Eichd2Word1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word2"]
    #[inline(always)]
    pub const fn eichd2_word2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word3"]
    #[inline(always)]
    pub const fn eichd2_word3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 2, Word4"]
    #[inline(always)]
    pub const fn eichd2_word4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word0"]
    #[inline(always)]
    pub const fn eichd3_word0(self) -> crate::common::Reg<regs::Eichd3Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word1"]
    #[inline(always)]
    pub const fn eichd3_word1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word2"]
    #[inline(always)]
    pub const fn eichd3_word2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word3"]
    #[inline(always)]
    pub const fn eichd3_word3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 3, Word4"]
    #[inline(always)]
    pub const fn eichd3_word4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word0"]
    #[inline(always)]
    pub const fn eichd4_word0(self) -> crate::common::Reg<regs::Eichd4Word0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word1"]
    #[inline(always)]
    pub const fn eichd4_word1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word2"]
    #[inline(always)]
    pub const fn eichd4_word2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word3"]
    #[inline(always)]
    pub const fn eichd4_word3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "Error Injection Channel Descriptor 4, Word4"]
    #[inline(always)]
    pub const fn eichd4_word4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
}
pub mod regs;
