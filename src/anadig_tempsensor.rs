#[doc = "RT1180_ANADIG_REGISTER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AnadigTempsensor {
    ptr: *mut u8,
}
unsafe impl Send for AnadigTempsensor {}
unsafe impl Sync for AnadigTempsensor {}
impl AnadigTempsensor {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TEMPSNS_OTP_TRIM_VALUE_REGISTER"]
    #[inline(always)]
    pub const fn tempsns_otp_trim_value(
        self,
    ) -> crate::common::Reg<regs::TempsnsOtpTrimValue, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4530usize) as _) }
    }
}
pub mod regs;
