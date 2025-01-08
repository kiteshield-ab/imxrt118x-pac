#[doc = "Port pseudo MAC status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppmsr(pub u32);
impl Ppmsr {
    #[doc = "Local link end's state 0 - Link is down 1 - Link is up The operational state is always \"Link is up\" for the pseudo link"]
    #[must_use]
    #[inline(always)]
    pub const fn lstate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Local link end's state 0 - Link is down 1 - Link is up The operational state is always \"Link is up\" for the pseudo link"]
    #[inline(always)]
    pub const fn set_lstate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Remote link end's state 0 - Link is down 1 - Link is up The operational state is always \"Link is up\" for the pseudo link"]
    #[must_use]
    #[inline(always)]
    pub const fn rstate(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Remote link end's state 0 - Link is down 1 - Link is up The operational state is always \"Link is up\" for the pseudo link"]
    #[inline(always)]
    pub const fn set_rstate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Ppmsr {
    #[inline(always)]
    fn default() -> Ppmsr {
        Ppmsr(257u64 as u32)
    }
}
impl core::fmt::Debug for Ppmsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppmsr")
            .field("lstate", &self.lstate())
            .field("rstate", &self.rstate())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppmsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppmsr {
            lstate: bool,
            rstate: bool,
        }
        let proxy = Ppmsr {
            lstate: self.lstate(),
            rstate: self.rstate(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
