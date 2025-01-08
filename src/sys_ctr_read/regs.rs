#[doc = "Counter Count Value High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntcv1(pub u32);
impl Cntcv1 {
    #[doc = "Counter Count Value Bits \\[55:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn cntcv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Counter Count Value Bits \\[55:32\\]"]
    #[inline(always)]
    pub const fn set_cntcv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Cntcv1 {
    #[inline(always)]
    fn default() -> Cntcv1 {
        Cntcv1(0u64 as u32)
    }
}
impl core::fmt::Debug for Cntcv1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntcv1")
            .field("cntcv1", &self.cntcv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntcv1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cntcv1 {
            cntcv1: u32,
        }
        let proxy = Cntcv1 {
            cntcv1: self.cntcv1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
