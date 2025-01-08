#[doc = "Counter Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntcr(pub u32);
impl Cntcr {
    #[doc = "Enable Counting"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Counting"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Debug Halt"]
    #[must_use]
    #[inline(always)]
    pub const fn hdbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug Halt"]
    #[inline(always)]
    pub const fn set_hdbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frequency Change Request, ID 0"]
    #[must_use]
    #[inline(always)]
    pub const fn fcr0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Change Request, ID 0"]
    #[inline(always)]
    pub const fn set_fcr0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frequency Change Request, ID 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fcr1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Change Request, ID 1"]
    #[inline(always)]
    pub const fn set_fcr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Cntcr {
    #[inline(always)]
    fn default() -> Cntcr {
        Cntcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cntcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntcr")
            .field("en", &self.en())
            .field("hdbg", &self.hdbg())
            .field("fcr0", &self.fcr0())
            .field("fcr1", &self.fcr1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cntcr {
            en: bool,
            hdbg: bool,
            fcr0: bool,
            fcr1: bool,
        }
        let proxy = Cntcr {
            en: self.en(),
            hdbg: self.hdbg(),
            fcr0: self.fcr0(),
            fcr1: self.fcr1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Counter Control 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntcr2(pub u32);
impl Cntcr2 {
    #[doc = "Hardware Frequency Change Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hwfc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Hardware Frequency Change Enable"]
    #[inline(always)]
    pub const fn set_hwfc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cntcr2 {
    #[inline(always)]
    fn default() -> Cntcr2 {
        Cntcr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Cntcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntcr2")
            .field("hwfc_en", &self.hwfc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cntcr2 {
            hwfc_en: bool,
        }
        let proxy = Cntcr2 {
            hwfc_en: self.hwfc_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
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
#[doc = "Counter Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntsr(pub u32);
impl Cntsr {
    #[doc = "Debug Halt"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgh(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Halt"]
    #[inline(always)]
    pub const fn set_dbgh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frequency Change Acknowledge, ID 0"]
    #[must_use]
    #[inline(always)]
    pub const fn fca0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Change Acknowledge, ID 0"]
    #[inline(always)]
    pub const fn set_fca0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frequency Change Acknowledge, ID 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fca1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frequency Change Acknowledge, ID 1"]
    #[inline(always)]
    pub const fn set_fca1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Cntsr {
    #[inline(always)]
    fn default() -> Cntsr {
        Cntsr(256u64 as u32)
    }
}
impl core::fmt::Debug for Cntsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cntsr")
            .field("dbgh", &self.dbgh())
            .field("fca0", &self.fca0())
            .field("fca1", &self.fca1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cntsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cntsr {
            dbgh: bool,
            fca0: bool,
            fca1: bool,
        }
        let proxy = Cntsr {
            dbgh: self.dbgh(),
            fca0: self.fca0(),
            fca1: self.fca1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
