#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc = "LinrReg master enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LinrReg master enable"]
    #[inline(always)]
    pub const fn set_linreg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LinReg power-up load disable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_pwrupload_dis(&self) -> super::vals::LinregPwruploadDis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::LinregPwruploadDis::from_bits(val as u8)
    }
    #[doc = "LinReg power-up load disable"]
    #[inline(always)]
    pub const fn set_linreg_pwrupload_dis(&mut self, val: super::vals::LinregPwruploadDis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "LinReg current-limit enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_ilimit_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LinReg current-limit enable"]
    #[inline(always)]
    pub const fn set_linreg_ilimit_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LinReg output voltage target setting"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_output_trg(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "LinReg output voltage target setting"]
    #[inline(always)]
    pub const fn set_linreg_output_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Isolation control for attached PHY load"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_phy_iso_b(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Isolation control for attached PHY load"]
    #[inline(always)]
    pub const fn set_linreg_phy_iso_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("linreg_en", &self.linreg_en())
            .field("linreg_pwrupload_dis", &self.linreg_pwrupload_dis())
            .field("linreg_ilimit_en", &self.linreg_ilimit_en())
            .field("linreg_output_trg", &self.linreg_output_trg())
            .field("linreg_phy_iso_b", &self.linreg_phy_iso_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0 {
            linreg_en: bool,
            linreg_pwrupload_dis: super::vals::LinregPwruploadDis,
            linreg_ilimit_en: bool,
            linreg_output_trg: u8,
            linreg_phy_iso_b: bool,
        }
        let proxy = Ctrl0 {
            linreg_en: self.linreg_en(),
            linreg_pwrupload_dis: self.linreg_pwrupload_dis(),
            linreg_ilimit_en: self.linreg_ilimit_en(),
            linreg_output_trg: self.linreg_output_trg(),
            linreg_phy_iso_b: self.linreg_phy_iso_b(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Clr(pub u32);
impl Ctrl0Clr {
    #[doc = "LinrReg master enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LinrReg master enable"]
    #[inline(always)]
    pub const fn set_linreg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LinReg power-up load disable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_pwrupload_dis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LinReg power-up load disable"]
    #[inline(always)]
    pub const fn set_linreg_pwrupload_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LinReg current-limit enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_ilimit_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LinReg current-limit enable"]
    #[inline(always)]
    pub const fn set_linreg_ilimit_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LinReg output voltage target setting"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_output_trg(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "LinReg output voltage target setting"]
    #[inline(always)]
    pub const fn set_linreg_output_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Isolation control for attached PHY load"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_phy_iso_b(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Isolation control for attached PHY load"]
    #[inline(always)]
    pub const fn set_linreg_phy_iso_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ctrl0Clr {
    #[inline(always)]
    fn default() -> Ctrl0Clr {
        Ctrl0Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Clr")
            .field("linreg_en", &self.linreg_en())
            .field("linreg_pwrupload_dis", &self.linreg_pwrupload_dis())
            .field("linreg_ilimit_en", &self.linreg_ilimit_en())
            .field("linreg_output_trg", &self.linreg_output_trg())
            .field("linreg_phy_iso_b", &self.linreg_phy_iso_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Clr {
            linreg_en: bool,
            linreg_pwrupload_dis: bool,
            linreg_ilimit_en: bool,
            linreg_output_trg: u8,
            linreg_phy_iso_b: bool,
        }
        let proxy = Ctrl0Clr {
            linreg_en: self.linreg_en(),
            linreg_pwrupload_dis: self.linreg_pwrupload_dis(),
            linreg_ilimit_en: self.linreg_ilimit_en(),
            linreg_output_trg: self.linreg_output_trg(),
            linreg_phy_iso_b: self.linreg_phy_iso_b(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Set(pub u32);
impl Ctrl0Set {
    #[doc = "LinrReg master enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LinrReg master enable"]
    #[inline(always)]
    pub const fn set_linreg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LinReg power-up load disable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_pwrupload_dis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LinReg power-up load disable"]
    #[inline(always)]
    pub const fn set_linreg_pwrupload_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LinReg current-limit enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_ilimit_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LinReg current-limit enable"]
    #[inline(always)]
    pub const fn set_linreg_ilimit_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LinReg output voltage target setting"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_output_trg(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "LinReg output voltage target setting"]
    #[inline(always)]
    pub const fn set_linreg_output_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Isolation control for attached PHY load"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_phy_iso_b(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Isolation control for attached PHY load"]
    #[inline(always)]
    pub const fn set_linreg_phy_iso_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ctrl0Set {
    #[inline(always)]
    fn default() -> Ctrl0Set {
        Ctrl0Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Set")
            .field("linreg_en", &self.linreg_en())
            .field("linreg_pwrupload_dis", &self.linreg_pwrupload_dis())
            .field("linreg_ilimit_en", &self.linreg_ilimit_en())
            .field("linreg_output_trg", &self.linreg_output_trg())
            .field("linreg_phy_iso_b", &self.linreg_phy_iso_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Set {
            linreg_en: bool,
            linreg_pwrupload_dis: bool,
            linreg_ilimit_en: bool,
            linreg_output_trg: u8,
            linreg_phy_iso_b: bool,
        }
        let proxy = Ctrl0Set {
            linreg_en: self.linreg_en(),
            linreg_pwrupload_dis: self.linreg_pwrupload_dis(),
            linreg_ilimit_en: self.linreg_ilimit_en(),
            linreg_output_trg: self.linreg_output_trg(),
            linreg_phy_iso_b: self.linreg_phy_iso_b(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Tog(pub u32);
impl Ctrl0Tog {
    #[doc = "LinrReg master enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LinrReg master enable"]
    #[inline(always)]
    pub const fn set_linreg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "LinReg power-up load disable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_pwrupload_dis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "LinReg power-up load disable"]
    #[inline(always)]
    pub const fn set_linreg_pwrupload_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LinReg current-limit enable"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_ilimit_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LinReg current-limit enable"]
    #[inline(always)]
    pub const fn set_linreg_ilimit_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "LinReg output voltage target setting"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_output_trg(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "LinReg output voltage target setting"]
    #[inline(always)]
    pub const fn set_linreg_output_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Isolation control for attached PHY load"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_phy_iso_b(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Isolation control for attached PHY load"]
    #[inline(always)]
    pub const fn set_linreg_phy_iso_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ctrl0Tog {
    #[inline(always)]
    fn default() -> Ctrl0Tog {
        Ctrl0Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Tog")
            .field("linreg_en", &self.linreg_en())
            .field("linreg_pwrupload_dis", &self.linreg_pwrupload_dis())
            .field("linreg_ilimit_en", &self.linreg_ilimit_en())
            .field("linreg_output_trg", &self.linreg_output_trg())
            .field("linreg_phy_iso_b", &self.linreg_phy_iso_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Tog {
            linreg_en: bool,
            linreg_pwrupload_dis: bool,
            linreg_ilimit_en: bool,
            linreg_output_trg: u8,
            linreg_phy_iso_b: bool,
        }
        let proxy = Ctrl0Tog {
            linreg_en: self.linreg_en(),
            linreg_pwrupload_dis: self.linreg_pwrupload_dis(),
            linreg_ilimit_en: self.linreg_ilimit_en(),
            linreg_output_trg: self.linreg_output_trg(),
            linreg_phy_iso_b: self.linreg_phy_iso_b(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0(pub u32);
impl Stat0 {
    #[doc = "LinReg Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "LinReg Status Bits"]
    #[inline(always)]
    pub const fn set_linreg_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Stat0 {
    #[inline(always)]
    fn default() -> Stat0 {
        Stat0(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0")
            .field("linreg_stat", &self.linreg_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0 {
            linreg_stat: u8,
        }
        let proxy = Stat0 {
            linreg_stat: self.linreg_stat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0Clr(pub u32);
impl Stat0Clr {
    #[doc = "LinReg Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "LinReg Status Bits"]
    #[inline(always)]
    pub const fn set_linreg_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Stat0Clr {
    #[inline(always)]
    fn default() -> Stat0Clr {
        Stat0Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0Clr")
            .field("linreg_stat", &self.linreg_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0Clr {
            linreg_stat: u8,
        }
        let proxy = Stat0Clr {
            linreg_stat: self.linreg_stat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0Set(pub u32);
impl Stat0Set {
    #[doc = "LinReg Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "LinReg Status Bits"]
    #[inline(always)]
    pub const fn set_linreg_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Stat0Set {
    #[inline(always)]
    fn default() -> Stat0Set {
        Stat0Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0Set")
            .field("linreg_stat", &self.linreg_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0Set {
            linreg_stat: u8,
        }
        let proxy = Stat0Set {
            linreg_stat: self.linreg_stat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0Tog(pub u32);
impl Stat0Tog {
    #[doc = "LinReg Status Bits"]
    #[must_use]
    #[inline(always)]
    pub const fn linreg_stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "LinReg Status Bits"]
    #[inline(always)]
    pub const fn set_linreg_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Stat0Tog {
    #[inline(always)]
    fn default() -> Stat0Tog {
        Stat0Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0Tog")
            .field("linreg_stat", &self.linreg_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0Tog {
            linreg_stat: u8,
        }
        let proxy = Stat0Tog {
            linreg_stat: self.linreg_stat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
