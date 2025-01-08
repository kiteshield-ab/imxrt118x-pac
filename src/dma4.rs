#[doc = "DMA MP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma4 {
    ptr: *mut u8,
}
unsafe impl Send for Dma4 {}
unsafe impl Sync for Dma4 {}
impl Dma4 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Management Page Control Register"]
    #[inline(always)]
    pub const fn mp_csr(self) -> crate::common::Reg<regs::MpCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Management Page Error Status Register"]
    #[inline(always)]
    pub const fn mp_es(self) -> crate::common::Reg<regs::MpEs, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Management Page Interrupt Request Status Register - Low"]
    #[inline(always)]
    pub const fn mp_int_low(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Management Page Interrupt Request Status Register- High"]
    #[inline(always)]
    pub const fn mp_int_high(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Management Page Hardware Request Status Register - Low"]
    #[inline(always)]
    pub const fn mp_hrs_low(self) -> crate::common::Reg<regs::MpHrsLow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Management Page Hardware Request Status Register - High"]
    #[inline(always)]
    pub const fn mp_hrs_high(self) -> crate::common::Reg<regs::MpHrsHigh, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Channel Arbitration Group Register"]
    #[inline(always)]
    pub const fn ch_grpri(self, n: usize) -> crate::common::Reg<regs::ChGrpri, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
