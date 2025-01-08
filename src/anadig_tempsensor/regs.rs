#[doc = "TEMPSNS_OTP_TRIM_VALUE_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TempsnsOtpTrimValue(pub u32);
impl TempsnsOtpTrimValue {
    #[doc = "Temperature Value at 25C"]
    #[must_use]
    #[inline(always)]
    pub const fn tempsns_temp_val(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x0fff;
        val as u16
    }
    #[doc = "Temperature Value at 25C"]
    #[inline(always)]
    pub const fn set_tempsns_temp_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 10usize)) | (((val as u32) & 0x0fff) << 10usize);
    }
}
impl Default for TempsnsOtpTrimValue {
    #[inline(always)]
    fn default() -> TempsnsOtpTrimValue {
        TempsnsOtpTrimValue(0u64 as u32)
    }
}
impl core::fmt::Debug for TempsnsOtpTrimValue {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TempsnsOtpTrimValue")
            .field("tempsns_temp_val", &self.tempsns_temp_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TempsnsOtpTrimValue {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TempsnsOtpTrimValue {
            tempsns_temp_val: u16,
        }
        let proxy = TempsnsOtpTrimValue {
            tempsns_temp_val: self.tempsns_temp_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
