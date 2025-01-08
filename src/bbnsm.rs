#[doc = "BBNSM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bbnsm {
    ptr: *mut u8,
}
unsafe impl Send for Bbnsm {}
unsafe impl Sync for Bbnsm {}
impl Bbnsm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BBNSM Version ID Register"]
    #[inline(always)]
    pub const fn bbnsm_vid(self) -> crate::common::Reg<regs::BbnsmVid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "BBNSM Features Register"]
    #[inline(always)]
    pub const fn bbnsm_features(self) -> crate::common::Reg<regs::BbnsmFeatures, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "BBNSM Control Register"]
    #[inline(always)]
    pub const fn bbnsm_ctrl(self) -> crate::common::Reg<regs::BbnsmCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "BBNSM Interrupt Enable Register"]
    #[inline(always)]
    pub const fn bbnsm_int_en(self) -> crate::common::Reg<regs::BbnsmIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "BBNSM Events Register"]
    #[inline(always)]
    pub const fn bbnsm_events(self) -> crate::common::Reg<regs::BbnsmEvents, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "BBNSM External Pad Control Register"]
    #[inline(always)]
    pub const fn bbnsm_pad_ctrl(self) -> crate::common::Reg<regs::BbnsmPadCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "BBNSM Real-Time Counter LS Register"]
    #[inline(always)]
    pub const fn bbnsm_rtc_ls(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "BBNSM Real-Time Counter MS Register"]
    #[inline(always)]
    pub const fn bbnsm_rtc_ms(self) -> crate::common::Reg<regs::BbnsmRtcMs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "BBNSM Time Alarm Register"]
    #[inline(always)]
    pub const fn bbnsm_ta(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "General Purpose Register Word word"]
    #[inline(always)]
    pub const fn gpr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
