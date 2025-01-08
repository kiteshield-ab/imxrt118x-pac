#[doc = "VREF Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "High Accuracy Bandgap enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn hcbgen(&self) -> super::vals::Hcbgen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Hcbgen::from_bits(val as u8)
    }
    #[doc = "High Accuracy Bandgap enabled"]
    #[inline(always)]
    pub const fn set_hcbgen(&mut self, val: super::vals::Hcbgen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Low Power Bandgap enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbgen(&self) -> super::vals::Lpbgen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lpbgen::from_bits(val as u8)
    }
    #[doc = "Low Power Bandgap enable"]
    #[inline(always)]
    pub const fn set_lpbgen(&mut self, val: super::vals::Lpbgen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Chop oscillator enable. When set, the internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[must_use]
    #[inline(always)]
    pub const fn chopen(&self) -> super::vals::Chopen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Chopen::from_bits(val as u8)
    }
    #[doc = "Chop oscillator enable. When set, the internal chopping operation is enabled and the internal analog offset will be minimized."]
    #[inline(always)]
    pub const fn set_chopen(&mut self, val: super::vals::Chopen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Second order curvature compensation enable"]
    #[must_use]
    #[inline(always)]
    pub const fn icompen(&self) -> super::vals::Icompen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Icompen::from_bits(val as u8)
    }
    #[doc = "Second order curvature compensation enable"]
    #[inline(always)]
    pub const fn set_icompen(&mut self, val: super::vals::Icompen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Regulator enable"]
    #[must_use]
    #[inline(always)]
    pub const fn regen(&self) -> super::vals::Regen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Regen::from_bits(val as u8)
    }
    #[doc = "Regulator enable"]
    #[inline(always)]
    pub const fn set_regen(&mut self, val: super::vals::Regen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Buffer mode control"]
    #[must_use]
    #[inline(always)]
    pub const fn hi_pwr_lv(&self) -> super::vals::HiPwrLv {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::HiPwrLv::from_bits(val as u8)
    }
    #[doc = "Buffer mode control"]
    #[inline(always)]
    pub const fn set_hi_pwr_lv(&mut self, val: super::vals::HiPwrLv) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Internal buffer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn buf21en(&self) -> super::vals::Buf21en {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Buf21en::from_bits(val as u8)
    }
    #[doc = "Internal buffer enable"]
    #[inline(always)]
    pub const fn set_buf21en(&mut self, val: super::vals::Buf21en) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Internal High Accuracy Voltage Reference stable"]
    #[must_use]
    #[inline(always)]
    pub const fn vrefst(&self) -> super::vals::Vrefst {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Vrefst::from_bits(val as u8)
    }
    #[doc = "Internal High Accuracy Voltage Reference stable"]
    #[inline(always)]
    pub const fn set_vrefst(&mut self, val: super::vals::Vrefst) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0u64 as u32)
    }
}
impl core::fmt::Debug for Csr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csr")
            .field("hcbgen", &self.hcbgen())
            .field("lpbgen", &self.lpbgen())
            .field("chopen", &self.chopen())
            .field("icompen", &self.icompen())
            .field("regen", &self.regen())
            .field("hi_pwr_lv", &self.hi_pwr_lv())
            .field("buf21en", &self.buf21en())
            .field("vrefst", &self.vrefst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Csr {
            hcbgen: super::vals::Hcbgen,
            lpbgen: super::vals::Lpbgen,
            chopen: super::vals::Chopen,
            icompen: super::vals::Icompen,
            regen: super::vals::Regen,
            hi_pwr_lv: super::vals::HiPwrLv,
            buf21en: super::vals::Buf21en,
            vrefst: super::vals::Vrefst,
        }
        let proxy = Csr {
            hcbgen: self.hcbgen(),
            lpbgen: self.lpbgen(),
            chopen: self.chopen(),
            icompen: self.icompen(),
            regen: self.regen(),
            hi_pwr_lv: self.hi_pwr_lv(),
            buf21en: self.buf21en(),
            vrefst: self.vrefst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VREF User Trim"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utrim(pub u32);
impl Utrim {
    #[doc = "VREF Trim bits"]
    #[must_use]
    #[inline(always)]
    pub const fn vreftrim(&self) -> super::vals::Vreftrim {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Vreftrim::from_bits(val as u8)
    }
    #[doc = "VREF Trim bits"]
    #[inline(always)]
    pub const fn set_vreftrim(&mut self, val: super::vals::Vreftrim) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
}
impl Default for Utrim {
    #[inline(always)]
    fn default() -> Utrim {
        Utrim(0u64 as u32)
    }
}
impl core::fmt::Debug for Utrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Utrim")
            .field("vreftrim", &self.vreftrim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Utrim {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Utrim {
            vreftrim: super::vals::Vreftrim,
        }
        let proxy = Utrim {
            vreftrim: self.vreftrim(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VREF Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "FEATURE"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "FEATURE"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "MINOR"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "MINOR"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "MAJOR"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "MAJOR"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(16777216u64 as u32)
    }
}
impl core::fmt::Debug for Verid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Verid")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Verid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Verid {
            feature: u16,
            minor: u8,
            major: u8,
        }
        let proxy = Verid {
            feature: self.feature(),
            minor: self.minor(),
            major: self.major(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
