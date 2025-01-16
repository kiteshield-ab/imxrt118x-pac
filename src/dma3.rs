#[doc = "DMA MP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma3 {
    ptr: *mut u8,
}
unsafe impl Send for Dma3 {}
unsafe impl Sync for Dma3 {}
impl Dma3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Management Page Control"]
    #[inline(always)]
    pub const fn mp_csr(self) -> crate::common::Reg<regs::MpCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Management Page Error Status"]
    #[inline(always)]
    pub const fn mp_es(self) -> crate::common::Reg<regs::MpEs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Management Page Interrupt Request Status"]
    #[inline(always)]
    pub const fn mp_int(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Management Page Hardware Request Status"]
    #[inline(always)]
    pub const fn mp_hrs(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Channel Arbitration Group"]
    #[inline(always)]
    pub const fn ch_grpri(self, n: usize) -> crate::common::Reg<regs::ChGrpri, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
