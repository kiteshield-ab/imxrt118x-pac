#[doc = "RT1180_ANADIG_REGISTER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnadigOsc {
    ptr: *mut u8,
}
unsafe impl Send for AnadigOsc {}
unsafe impl Sync for AnadigOsc {}
impl AnadigOsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "24MHz RCOSC Control Register"]
    #[inline(always)]
    pub const fn osc_rc24m_ctrl(self) -> crate::common::Reg<regs::OscRc24mCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4310usize) as _) }
    }
    #[doc = "24MHz OSC Control Register"]
    #[inline(always)]
    pub const fn osc_24m_ctrl(self) -> crate::common::Reg<regs::Osc24mCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4320usize) as _) }
    }
    #[doc = "400MHz RCOSC Control0 Register"]
    #[inline(always)]
    pub const fn osc_400m_ctrl0(self) -> crate::common::Reg<regs::Osc400mCtrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4340usize) as _) }
    }
    #[doc = "400MHz RCOSC Control1 Register"]
    #[inline(always)]
    pub const fn osc_400m_ctrl1(self) -> crate::common::Reg<regs::Osc400mCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4350usize) as _) }
    }
}
pub mod regs;
pub mod vals;
