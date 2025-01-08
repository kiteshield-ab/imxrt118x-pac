#[doc = "Error Injection Channel Descriptor 0, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd0Word0(pub u32);
impl Eichd0Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x3fff;
        val as u16
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 18usize)) | (((val as u32) & 0x3fff) << 18usize);
    }
}
impl Default for Eichd0Word0 {
    #[inline(always)]
    fn default() -> Eichd0Word0 {
        Eichd0Word0(0u64 as u32)
    }
}
impl core::fmt::Debug for Eichd0Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd0Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd0Word0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eichd0Word0 {
            chkbit_mask: u16,
        }
        let proxy = Eichd0Word0 {
            chkbit_mask: self.chkbit_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection Channel Descriptor 0, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd0Word1(pub u32);
impl Eichd0Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Eichd0Word1 {
    #[inline(always)]
    fn default() -> Eichd0Word1 {
        Eichd0Word1(0u64 as u32)
    }
}
impl core::fmt::Debug for Eichd0Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd0Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd0Word1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eichd0Word1 {
            b0_3data_mask: u16,
        }
        let proxy = Eichd0Word1 {
            b0_3data_mask: self.b0_3data_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection Channel Descriptor 1, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd1Word0(pub u32);
impl Eichd1Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Eichd1Word0 {
    #[inline(always)]
    fn default() -> Eichd1Word0 {
        Eichd1Word0(0u64 as u32)
    }
}
impl core::fmt::Debug for Eichd1Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd1Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd1Word0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eichd1Word0 {
            chkbit_mask: u16,
        }
        let proxy = Eichd1Word0 {
            chkbit_mask: self.chkbit_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd2Word0(pub u32);
impl Eichd2Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Eichd2Word0 {
    #[inline(always)]
    fn default() -> Eichd2Word0 {
        Eichd2Word0(0u64 as u32)
    }
}
impl core::fmt::Debug for Eichd2Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd2Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd2Word0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eichd2Word0 {
            chkbit_mask: u32,
        }
        let proxy = Eichd2Word0 {
            chkbit_mask: self.chkbit_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection Channel Descriptor 2, Word1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd2Word1(pub u32);
impl Eichd2Word1 {
    #[doc = "Data Mask Bytes 0-3"]
    #[must_use]
    #[inline(always)]
    pub const fn b0_3data_mask(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data Mask Bytes 0-3"]
    #[inline(always)]
    pub const fn set_b0_3data_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Eichd2Word1 {
    #[inline(always)]
    fn default() -> Eichd2Word1 {
        Eichd2Word1(0u64 as u32)
    }
}
impl core::fmt::Debug for Eichd2Word1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd2Word1")
            .field("b0_3data_mask", &self.b0_3data_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd2Word1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eichd2Word1 {
            b0_3data_mask: u8,
        }
        let proxy = Eichd2Word1 {
            b0_3data_mask: self.b0_3data_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection Channel Descriptor 3, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd3Word0(pub u32);
impl Eichd3Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Eichd3Word0 {
    #[inline(always)]
    fn default() -> Eichd3Word0 {
        Eichd3Word0(0u64 as u32)
    }
}
impl core::fmt::Debug for Eichd3Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd3Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd3Word0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eichd3Word0 {
            chkbit_mask: u32,
        }
        let proxy = Eichd3Word0 {
            chkbit_mask: self.chkbit_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection Channel Descriptor 4, Word0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichd4Word0(pub u32);
impl Eichd4Word0 {
    #[doc = "Checkbit Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn chkbit_mask(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "Checkbit Mask"]
    #[inline(always)]
    pub const fn set_chkbit_mask(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for Eichd4Word0 {
    #[inline(always)]
    fn default() -> Eichd4Word0 {
        Eichd4Word0(0u64 as u32)
    }
}
impl core::fmt::Debug for Eichd4Word0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichd4Word0")
            .field("chkbit_mask", &self.chkbit_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichd4Word0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eichd4Word0 {
            chkbit_mask: u32,
        }
        let proxy = Eichd4Word0 {
            chkbit_mask: self.chkbit_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection Channel Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eichen(pub u32);
impl Eichen {
    #[doc = "Error Injection Channel 4 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich4en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 4 Enable"]
    #[inline(always)]
    pub const fn set_eich4en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Error Injection Channel 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich3en(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 3 Enable"]
    #[inline(always)]
    pub const fn set_eich3en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Error Injection Channel 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich2en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 2 Enable"]
    #[inline(always)]
    pub const fn set_eich2en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Error Injection Channel 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich1en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 1 Enable"]
    #[inline(always)]
    pub const fn set_eich1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Error Injection Channel 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn eich0en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Error Injection Channel 0 Enable"]
    #[inline(always)]
    pub const fn set_eich0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Eichen {
    #[inline(always)]
    fn default() -> Eichen {
        Eichen(0u64 as u32)
    }
}
impl core::fmt::Debug for Eichen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eichen")
            .field("eich4en", &self.eich4en())
            .field("eich3en", &self.eich3en())
            .field("eich2en", &self.eich2en())
            .field("eich1en", &self.eich1en())
            .field("eich0en", &self.eich0en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eichen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eichen {
            eich4en: bool,
            eich3en: bool,
            eich2en: bool,
            eich1en: bool,
            eich0en: bool,
        }
        let proxy = Eichen {
            eich4en: self.eich4en(),
            eich3en: self.eich3en(),
            eich2en: self.eich2en(),
            eich1en: self.eich1en(),
            eich0en: self.eich0en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Injection Module Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eimcr(pub u32);
impl Eimcr {
    #[doc = "Global Error Injection Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn geien(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Global Error Injection Enable"]
    #[inline(always)]
    pub const fn set_geien(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Eimcr {
    #[inline(always)]
    fn default() -> Eimcr {
        Eimcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Eimcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eimcr")
            .field("geien", &self.geien())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eimcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eimcr {
            geien: bool,
        }
        let proxy = Eimcr {
            geien: self.geien(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
