#[doc = "Compare Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpcr0(pub u32);
impl Cmpcr0 {
    #[doc = "Compare Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Request Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn imask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request Mask"]
    #[inline(always)]
    pub const fn set_imask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Compare Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn istat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Interrupt Status"]
    #[inline(always)]
    pub const fn set_istat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Cmpcr0 {
    #[inline(always)]
    fn default() -> Cmpcr0 {
        Cmpcr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmpcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpcr0")
            .field("en", &self.en())
            .field("imask", &self.imask())
            .field("istat", &self.istat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmpcr0 {
            en: bool,
            imask: bool,
            istat: bool,
        }
        let proxy = Cmpcr0 {
            en: self.en(),
            imask: self.imask(),
            istat: self.istat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Compare Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpcr1(pub u32);
impl Cmpcr1 {
    #[doc = "Compare Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt Request Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn imask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Request Mask"]
    #[inline(always)]
    pub const fn set_imask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Compare Interrupt Status"]
    #[must_use]
    #[inline(always)]
    pub const fn istat(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Compare Interrupt Status"]
    #[inline(always)]
    pub const fn set_istat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Cmpcr1 {
    #[inline(always)]
    fn default() -> Cmpcr1 {
        Cmpcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmpcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpcr1")
            .field("en", &self.en())
            .field("imask", &self.imask())
            .field("istat", &self.istat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmpcr1 {
            en: bool,
            imask: bool,
            istat: bool,
        }
        let proxy = Cmpcr1 {
            en: self.en(),
            imask: self.imask(),
            istat: self.istat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Compare Count Value High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpcvh0(pub u32);
impl Cmpcvh0 {
    #[doc = "Compare Count Value Bits \\[55:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare Count Value Bits \\[55:32\\]"]
    #[inline(always)]
    pub const fn set_cmpcv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Cmpcvh0 {
    #[inline(always)]
    fn default() -> Cmpcvh0 {
        Cmpcvh0(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmpcvh0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpcvh0")
            .field("cmpcv1", &self.cmpcv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpcvh0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmpcvh0 {
            cmpcv1: u32,
        }
        let proxy = Cmpcvh0 {
            cmpcv1: self.cmpcv1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Compare Count Value High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpcvh1(pub u32);
impl Cmpcvh1 {
    #[doc = "Compare Count Value Bits \\[55:32\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn cmpcv1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Compare Count Value Bits \\[55:32\\]"]
    #[inline(always)]
    pub const fn set_cmpcv1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Cmpcvh1 {
    #[inline(always)]
    fn default() -> Cmpcvh1 {
        Cmpcvh1(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmpcvh1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmpcvh1")
            .field("cmpcv1", &self.cmpcv1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmpcvh1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmpcvh1 {
            cmpcv1: u32,
        }
        let proxy = Cmpcvh1 {
            cmpcv1: self.cmpcv1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
