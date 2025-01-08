#[doc = "Keypad Data Direction Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kddr(pub u16);
impl Kddr {
    #[doc = "KRDD"]
    #[must_use]
    #[inline(always)]
    pub const fn krdd(&self) -> super::vals::Krdd {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Krdd::from_bits(val as u8)
    }
    #[doc = "KRDD"]
    #[inline(always)]
    pub const fn set_krdd(&mut self, val: super::vals::Krdd) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
    #[doc = "KCDD"]
    #[must_use]
    #[inline(always)]
    pub const fn kcdd(&self) -> super::vals::Kcdd {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Kcdd::from_bits(val as u8)
    }
    #[doc = "KCDD"]
    #[inline(always)]
    pub const fn set_kcdd(&mut self, val: super::vals::Kcdd) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Kddr {
    #[inline(always)]
    fn default() -> Kddr {
        Kddr(0u64 as u16)
    }
}
impl core::fmt::Debug for Kddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kddr")
            .field("krdd", &self.krdd())
            .field("kcdd", &self.kcdd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Kddr {
            krdd: super::vals::Krdd,
            kcdd: super::vals::Kcdd,
        }
        let proxy = Kddr {
            krdd: self.krdd(),
            kcdd: self.kcdd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Keypad Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kpcr(pub u16);
impl Kpcr {
    #[doc = "KRE"]
    #[must_use]
    #[inline(always)]
    pub const fn kre(&self) -> super::vals::Kre {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Kre::from_bits(val as u8)
    }
    #[doc = "KRE"]
    #[inline(always)]
    pub const fn set_kre(&mut self, val: super::vals::Kre) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u16) & 0xff) << 0usize);
    }
    #[doc = "KCO"]
    #[must_use]
    #[inline(always)]
    pub const fn kco(&self) -> super::vals::Kco {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::Kco::from_bits(val as u8)
    }
    #[doc = "KCO"]
    #[inline(always)]
    pub const fn set_kco(&mut self, val: super::vals::Kco) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u16) & 0xff) << 8usize);
    }
}
impl Default for Kpcr {
    #[inline(always)]
    fn default() -> Kpcr {
        Kpcr(0u64 as u16)
    }
}
impl core::fmt::Debug for Kpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kpcr")
            .field("kre", &self.kre())
            .field("kco", &self.kco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kpcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Kpcr {
            kre: super::vals::Kre,
            kco: super::vals::Kco,
        }
        let proxy = Kpcr {
            kre: self.kre(),
            kco: self.kco(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Keypad Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kpdr(pub u16);
impl Kpdr {
    #[doc = "KRD"]
    #[must_use]
    #[inline(always)]
    pub const fn krd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "KRD"]
    #[inline(always)]
    pub const fn set_krd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "KCD"]
    #[must_use]
    #[inline(always)]
    pub const fn kcd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "KCD"]
    #[inline(always)]
    pub const fn set_kcd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for Kpdr {
    #[inline(always)]
    fn default() -> Kpdr {
        Kpdr(0u64 as u16)
    }
}
impl core::fmt::Debug for Kpdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kpdr")
            .field("krd", &self.krd())
            .field("kcd", &self.kcd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kpdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Kpdr {
            krd: u8,
            kcd: u8,
        }
        let proxy = Kpdr {
            krd: self.krd(),
            kcd: self.kcd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Keypad Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Kpsr(pub u16);
impl Kpsr {
    #[doc = "KPKD"]
    #[must_use]
    #[inline(always)]
    pub const fn kpkd(&self) -> super::vals::Kpkd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Kpkd::from_bits(val as u8)
    }
    #[doc = "KPKD"]
    #[inline(always)]
    pub const fn set_kpkd(&mut self, val: super::vals::Kpkd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u16) & 0x01) << 0usize);
    }
    #[doc = "KPKR"]
    #[must_use]
    #[inline(always)]
    pub const fn kpkr(&self) -> super::vals::Kpkr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Kpkr::from_bits(val as u8)
    }
    #[doc = "KPKR"]
    #[inline(always)]
    pub const fn set_kpkr(&mut self, val: super::vals::Kpkr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u16) & 0x01) << 1usize);
    }
    #[doc = "KDSC"]
    #[must_use]
    #[inline(always)]
    pub const fn kdsc(&self) -> super::vals::Kdsc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Kdsc::from_bits(val as u8)
    }
    #[doc = "KDSC"]
    #[inline(always)]
    pub const fn set_kdsc(&mut self, val: super::vals::Kdsc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u16) & 0x01) << 2usize);
    }
    #[doc = "KRSS"]
    #[must_use]
    #[inline(always)]
    pub const fn krss(&self) -> super::vals::Krss {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Krss::from_bits(val as u8)
    }
    #[doc = "KRSS"]
    #[inline(always)]
    pub const fn set_krss(&mut self, val: super::vals::Krss) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u16) & 0x01) << 3usize);
    }
    #[doc = "KDIE"]
    #[must_use]
    #[inline(always)]
    pub const fn kdie(&self) -> super::vals::Kdie {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Kdie::from_bits(val as u8)
    }
    #[doc = "KDIE"]
    #[inline(always)]
    pub const fn set_kdie(&mut self, val: super::vals::Kdie) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u16) & 0x01) << 8usize);
    }
    #[doc = "KRIE"]
    #[must_use]
    #[inline(always)]
    pub const fn krie(&self) -> super::vals::Krie {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Krie::from_bits(val as u8)
    }
    #[doc = "KRIE"]
    #[inline(always)]
    pub const fn set_krie(&mut self, val: super::vals::Krie) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u16) & 0x01) << 9usize);
    }
}
impl Default for Kpsr {
    #[inline(always)]
    fn default() -> Kpsr {
        Kpsr(2u64 as u16)
    }
}
impl core::fmt::Debug for Kpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Kpsr")
            .field("kpkd", &self.kpkd())
            .field("kpkr", &self.kpkr())
            .field("kdsc", &self.kdsc())
            .field("krss", &self.krss())
            .field("kdie", &self.kdie())
            .field("krie", &self.krie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Kpsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Kpsr {
            kpkd: super::vals::Kpkd,
            kpkr: super::vals::Kpkr,
            kdsc: super::vals::Kdsc,
            krss: super::vals::Krss,
            kdie: super::vals::Kdie,
            krie: super::vals::Krie,
        }
        let proxy = Kpsr {
            kpkd: self.kpkd(),
            kpkr: self.kpkr(),
            kdsc: self.kdsc(),
            krss: self.krss(),
            kdie: self.kdie(),
            krie: self.krie(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
