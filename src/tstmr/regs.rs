#[doc = "Timestamp Timer High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct High(pub u32);
impl High {
    #[doc = "Timestamp Timer High"]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Timestamp Timer High"]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for High {
    #[inline(always)]
    fn default() -> High {
        High(0u64 as u32)
    }
}
impl core::fmt::Debug for High {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("High")
            .field("value", &self.value())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for High {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct High {
            value: u32,
        }
        let proxy = High {
            value: self.value(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
