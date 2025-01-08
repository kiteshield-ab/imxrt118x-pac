#[doc = "Pseudo MAC port"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enetc1PseudoMacPort {
    ptr: *mut u8,
}
unsafe impl Send for Enetc1PseudoMacPort {}
unsafe impl Sync for Enetc1PseudoMacPort {}
impl Enetc1PseudoMacPort {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Port pseudo MAC status register"]
    #[inline(always)]
    pub const fn ppmsr(self) -> crate::common::Reg<regs::Ppmsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Port pseudo MAC configuration register"]
    #[inline(always)]
    pub const fn ppmcr(self) -> crate::common::Reg<regs::Ppmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port pseudo MAC receive octets counter"]
    #[inline(always)]
    pub const fn ppmrocr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Port pseudo MAC receive unicast frame counter register"]
    #[inline(always)]
    pub const fn ppmrufcr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize + n * 4usize) as _) }
    }
    #[doc = "Port pseudo MAC receive multicast frame counter register"]
    #[inline(always)]
    pub const fn ppmrmfcr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize + n * 4usize) as _) }
    }
    #[doc = "Port pseudo MAC receive broadcast frame counter register"]
    #[inline(always)]
    pub const fn ppmrbfcr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize + n * 4usize) as _) }
    }
    #[doc = "Port pseudo MAC transmit octets counter"]
    #[inline(always)]
    pub const fn ppmtocr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize + n * 4usize) as _) }
    }
    #[doc = "Port pseudo MAC transmit unicast frame counter register"]
    #[inline(always)]
    pub const fn ppmtufcr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize + n * 4usize) as _) }
    }
    #[doc = "Port pseudo MAC transmit multicast frame counter register"]
    #[inline(always)]
    pub const fn ppmtmfcr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize + n * 4usize) as _) }
    }
    #[doc = "Port pseudo MAC transmit broadcast frame counter register"]
    #[inline(always)]
    pub const fn ppmtbfcr(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize + n * 4usize) as _) }
    }
}
pub mod regs;
