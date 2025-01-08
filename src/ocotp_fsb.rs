#[doc = "no description available"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OcotpFsb {
    ptr: *mut u8,
}
unsafe impl Send for OcotpFsb {}
unsafe impl Sync for OcotpFsb {}
impl OcotpFsb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "OTP shadow register"]
    #[inline(always)]
    pub const fn otp_shadow_parta(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 52usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "OTP shadow register"]
    #[inline(always)]
    pub const fn otp_shadow_partb(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 200usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e0usize + n * 4usize) as _) }
    }
}
