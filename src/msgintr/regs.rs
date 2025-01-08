#[doc = "Message Signaled Interrupt Index Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msiir0(pub u32);
impl Msiir0 {
    #[doc = "Interrupt Bit Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ibs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Interrupt Bit Select"]
    #[inline(always)]
    pub const fn set_ibs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Msiir0 {
    #[inline(always)]
    fn default() -> Msiir0 {
        Msiir0(0u64 as u32)
    }
}
impl core::fmt::Debug for Msiir0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msiir0").field("ibs", &self.ibs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msiir0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Msiir0 {
            ibs: u8,
        }
        let proxy = Msiir0 { ibs: self.ibs() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Message Signaled Interrupt Index Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msiir1(pub u32);
impl Msiir1 {
    #[doc = "Interrupt Bit Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ibs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Interrupt Bit Select"]
    #[inline(always)]
    pub const fn set_ibs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Msiir1 {
    #[inline(always)]
    fn default() -> Msiir1 {
        Msiir1(0u64 as u32)
    }
}
impl core::fmt::Debug for Msiir1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msiir1").field("ibs", &self.ibs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msiir1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Msiir1 {
            ibs: u8,
        }
        let proxy = Msiir1 { ibs: self.ibs() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Message Signaled Interrupt Index Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msiir2(pub u32);
impl Msiir2 {
    #[doc = "Interrupt Bit Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ibs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Interrupt Bit Select"]
    #[inline(always)]
    pub const fn set_ibs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Msiir2 {
    #[inline(always)]
    fn default() -> Msiir2 {
        Msiir2(0u64 as u32)
    }
}
impl core::fmt::Debug for Msiir2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msiir2").field("ibs", &self.ibs()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msiir2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Msiir2 {
            ibs: u8,
        }
        let proxy = Msiir2 { ibs: self.ibs() };
        defmt::write!(f, "{}", proxy)
    }
}
