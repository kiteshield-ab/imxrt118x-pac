#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs0(pub u32);
impl Crs0 {
    #[doc = "Park"]
    #[must_use]
    #[inline(always)]
    pub const fn park(&self) -> super::vals::Crs0Park {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crs0Park::from_bits(val as u8)
    }
    #[doc = "Park"]
    #[inline(always)]
    pub const fn set_park(&mut self, val: super::vals::Crs0Park) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Parking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pctl(&self) -> super::vals::Crs0Pctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Crs0Pctl::from_bits(val as u8)
    }
    #[doc = "Parking Control"]
    #[inline(always)]
    pub const fn set_pctl(&mut self, val: super::vals::Crs0Pctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Arbitration Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> super::vals::Crs0Arb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Crs0Arb::from_bits(val as u8)
    }
    #[doc = "Arbitration Mode"]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: super::vals::Crs0Arb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Priority Elevation 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe0(&self) -> super::vals::Crs0Hpe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Crs0Hpe0::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 0"]
    #[inline(always)]
    pub const fn set_hpe0(&mut self, val: super::vals::Crs0Hpe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "High Priority Elevation 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe1(&self) -> super::vals::Crs0Hpe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Crs0Hpe1::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 1"]
    #[inline(always)]
    pub const fn set_hpe1(&mut self, val: super::vals::Crs0Hpe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Priority Elevation 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe2(&self) -> super::vals::Crs0Hpe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Crs0Hpe2::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 2"]
    #[inline(always)]
    pub const fn set_hpe2(&mut self, val: super::vals::Crs0Hpe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Priority Elevation 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe3(&self) -> super::vals::Crs0Hpe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Crs0Hpe3::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 3"]
    #[inline(always)]
    pub const fn set_hpe3(&mut self, val: super::vals::Crs0Hpe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Priority Elevation 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe4(&self) -> super::vals::Crs0Hpe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Crs0Hpe4::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 4"]
    #[inline(always)]
    pub const fn set_hpe4(&mut self, val: super::vals::Crs0Hpe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Priority Elevation 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe5(&self) -> super::vals::Crs0Hpe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Crs0Hpe5::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 5"]
    #[inline(always)]
    pub const fn set_hpe5(&mut self, val: super::vals::Crs0Hpe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Crs0Ro {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Crs0Ro::from_bits(val as u8)
    }
    #[doc = "Read Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: super::vals::Crs0Ro) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Crs0 {
    #[inline(always)]
    fn default() -> Crs0 {
        Crs0(131072u64 as u32)
    }
}
impl core::fmt::Debug for Crs0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crs0")
            .field("park", &self.park())
            .field("pctl", &self.pctl())
            .field("arb", &self.arb())
            .field("hpe0", &self.hpe0())
            .field("hpe1", &self.hpe1())
            .field("hpe2", &self.hpe2())
            .field("hpe3", &self.hpe3())
            .field("hpe4", &self.hpe4())
            .field("hpe5", &self.hpe5())
            .field("ro", &self.ro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crs0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crs0 {
            park: super::vals::Crs0Park,
            pctl: super::vals::Crs0Pctl,
            arb: super::vals::Crs0Arb,
            hpe0: super::vals::Crs0Hpe0,
            hpe1: super::vals::Crs0Hpe1,
            hpe2: super::vals::Crs0Hpe2,
            hpe3: super::vals::Crs0Hpe3,
            hpe4: super::vals::Crs0Hpe4,
            hpe5: super::vals::Crs0Hpe5,
            ro: super::vals::Crs0Ro,
        }
        let proxy = Crs0 {
            park: self.park(),
            pctl: self.pctl(),
            arb: self.arb(),
            hpe0: self.hpe0(),
            hpe1: self.hpe1(),
            hpe2: self.hpe2(),
            hpe3: self.hpe3(),
            hpe4: self.hpe4(),
            hpe5: self.hpe5(),
            ro: self.ro(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs1(pub u32);
impl Crs1 {
    #[doc = "Park"]
    #[must_use]
    #[inline(always)]
    pub const fn park(&self) -> super::vals::Crs1Park {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crs1Park::from_bits(val as u8)
    }
    #[doc = "Park"]
    #[inline(always)]
    pub const fn set_park(&mut self, val: super::vals::Crs1Park) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Parking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pctl(&self) -> super::vals::Crs1Pctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Crs1Pctl::from_bits(val as u8)
    }
    #[doc = "Parking Control"]
    #[inline(always)]
    pub const fn set_pctl(&mut self, val: super::vals::Crs1Pctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Arbitration Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> super::vals::Crs1Arb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Crs1Arb::from_bits(val as u8)
    }
    #[doc = "Arbitration Mode"]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: super::vals::Crs1Arb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Priority Elevation 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe0(&self) -> super::vals::Crs1Hpe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Crs1Hpe0::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 0"]
    #[inline(always)]
    pub const fn set_hpe0(&mut self, val: super::vals::Crs1Hpe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "High Priority Elevation 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe1(&self) -> super::vals::Crs1Hpe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Crs1Hpe1::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 1"]
    #[inline(always)]
    pub const fn set_hpe1(&mut self, val: super::vals::Crs1Hpe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Priority Elevation 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe2(&self) -> super::vals::Crs1Hpe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Crs1Hpe2::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 2"]
    #[inline(always)]
    pub const fn set_hpe2(&mut self, val: super::vals::Crs1Hpe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Priority Elevation 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe3(&self) -> super::vals::Crs1Hpe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Crs1Hpe3::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 3"]
    #[inline(always)]
    pub const fn set_hpe3(&mut self, val: super::vals::Crs1Hpe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Priority Elevation 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe4(&self) -> super::vals::Crs1Hpe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Crs1Hpe4::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 4"]
    #[inline(always)]
    pub const fn set_hpe4(&mut self, val: super::vals::Crs1Hpe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Priority Elevation 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe5(&self) -> super::vals::Crs1Hpe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Crs1Hpe5::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 5"]
    #[inline(always)]
    pub const fn set_hpe5(&mut self, val: super::vals::Crs1Hpe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Crs1Ro {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Crs1Ro::from_bits(val as u8)
    }
    #[doc = "Read Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: super::vals::Crs1Ro) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Crs1 {
    #[inline(always)]
    fn default() -> Crs1 {
        Crs1(131072u64 as u32)
    }
}
impl core::fmt::Debug for Crs1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crs1")
            .field("park", &self.park())
            .field("pctl", &self.pctl())
            .field("arb", &self.arb())
            .field("hpe0", &self.hpe0())
            .field("hpe1", &self.hpe1())
            .field("hpe2", &self.hpe2())
            .field("hpe3", &self.hpe3())
            .field("hpe4", &self.hpe4())
            .field("hpe5", &self.hpe5())
            .field("ro", &self.ro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crs1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crs1 {
            park: super::vals::Crs1Park,
            pctl: super::vals::Crs1Pctl,
            arb: super::vals::Crs1Arb,
            hpe0: super::vals::Crs1Hpe0,
            hpe1: super::vals::Crs1Hpe1,
            hpe2: super::vals::Crs1Hpe2,
            hpe3: super::vals::Crs1Hpe3,
            hpe4: super::vals::Crs1Hpe4,
            hpe5: super::vals::Crs1Hpe5,
            ro: super::vals::Crs1Ro,
        }
        let proxy = Crs1 {
            park: self.park(),
            pctl: self.pctl(),
            arb: self.arb(),
            hpe0: self.hpe0(),
            hpe1: self.hpe1(),
            hpe2: self.hpe2(),
            hpe3: self.hpe3(),
            hpe4: self.hpe4(),
            hpe5: self.hpe5(),
            ro: self.ro(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs2(pub u32);
impl Crs2 {
    #[doc = "Park"]
    #[must_use]
    #[inline(always)]
    pub const fn park(&self) -> super::vals::Crs2Park {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crs2Park::from_bits(val as u8)
    }
    #[doc = "Park"]
    #[inline(always)]
    pub const fn set_park(&mut self, val: super::vals::Crs2Park) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Parking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pctl(&self) -> super::vals::Crs2Pctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Crs2Pctl::from_bits(val as u8)
    }
    #[doc = "Parking Control"]
    #[inline(always)]
    pub const fn set_pctl(&mut self, val: super::vals::Crs2Pctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Arbitration Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> super::vals::Crs2Arb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Crs2Arb::from_bits(val as u8)
    }
    #[doc = "Arbitration Mode"]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: super::vals::Crs2Arb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Priority Elevation 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe0(&self) -> super::vals::Crs2Hpe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Crs2Hpe0::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 0"]
    #[inline(always)]
    pub const fn set_hpe0(&mut self, val: super::vals::Crs2Hpe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "High Priority Elevation 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe1(&self) -> super::vals::Crs2Hpe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Crs2Hpe1::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 1"]
    #[inline(always)]
    pub const fn set_hpe1(&mut self, val: super::vals::Crs2Hpe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Priority Elevation 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe2(&self) -> super::vals::Crs2Hpe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Crs2Hpe2::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 2"]
    #[inline(always)]
    pub const fn set_hpe2(&mut self, val: super::vals::Crs2Hpe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Priority Elevation 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe3(&self) -> super::vals::Crs2Hpe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Crs2Hpe3::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 3"]
    #[inline(always)]
    pub const fn set_hpe3(&mut self, val: super::vals::Crs2Hpe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Priority Elevation 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe4(&self) -> super::vals::Crs2Hpe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Crs2Hpe4::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 4"]
    #[inline(always)]
    pub const fn set_hpe4(&mut self, val: super::vals::Crs2Hpe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Priority Elevation 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe5(&self) -> super::vals::Crs2Hpe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Crs2Hpe5::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 5"]
    #[inline(always)]
    pub const fn set_hpe5(&mut self, val: super::vals::Crs2Hpe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Crs2Ro {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Crs2Ro::from_bits(val as u8)
    }
    #[doc = "Read Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: super::vals::Crs2Ro) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Crs2 {
    #[inline(always)]
    fn default() -> Crs2 {
        Crs2(131072u64 as u32)
    }
}
impl core::fmt::Debug for Crs2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crs2")
            .field("park", &self.park())
            .field("pctl", &self.pctl())
            .field("arb", &self.arb())
            .field("hpe0", &self.hpe0())
            .field("hpe1", &self.hpe1())
            .field("hpe2", &self.hpe2())
            .field("hpe3", &self.hpe3())
            .field("hpe4", &self.hpe4())
            .field("hpe5", &self.hpe5())
            .field("ro", &self.ro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crs2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crs2 {
            park: super::vals::Crs2Park,
            pctl: super::vals::Crs2Pctl,
            arb: super::vals::Crs2Arb,
            hpe0: super::vals::Crs2Hpe0,
            hpe1: super::vals::Crs2Hpe1,
            hpe2: super::vals::Crs2Hpe2,
            hpe3: super::vals::Crs2Hpe3,
            hpe4: super::vals::Crs2Hpe4,
            hpe5: super::vals::Crs2Hpe5,
            ro: super::vals::Crs2Ro,
        }
        let proxy = Crs2 {
            park: self.park(),
            pctl: self.pctl(),
            arb: self.arb(),
            hpe0: self.hpe0(),
            hpe1: self.hpe1(),
            hpe2: self.hpe2(),
            hpe3: self.hpe3(),
            hpe4: self.hpe4(),
            hpe5: self.hpe5(),
            ro: self.ro(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs3(pub u32);
impl Crs3 {
    #[doc = "Park"]
    #[must_use]
    #[inline(always)]
    pub const fn park(&self) -> super::vals::Crs3Park {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crs3Park::from_bits(val as u8)
    }
    #[doc = "Park"]
    #[inline(always)]
    pub const fn set_park(&mut self, val: super::vals::Crs3Park) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Parking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pctl(&self) -> super::vals::Crs3Pctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Crs3Pctl::from_bits(val as u8)
    }
    #[doc = "Parking Control"]
    #[inline(always)]
    pub const fn set_pctl(&mut self, val: super::vals::Crs3Pctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Arbitration Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> super::vals::Crs3Arb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Crs3Arb::from_bits(val as u8)
    }
    #[doc = "Arbitration Mode"]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: super::vals::Crs3Arb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Priority Elevation 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe0(&self) -> super::vals::Crs3Hpe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Crs3Hpe0::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 0"]
    #[inline(always)]
    pub const fn set_hpe0(&mut self, val: super::vals::Crs3Hpe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "High Priority Elevation 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe1(&self) -> super::vals::Crs3Hpe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Crs3Hpe1::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 1"]
    #[inline(always)]
    pub const fn set_hpe1(&mut self, val: super::vals::Crs3Hpe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Priority Elevation 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe2(&self) -> super::vals::Crs3Hpe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Crs3Hpe2::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 2"]
    #[inline(always)]
    pub const fn set_hpe2(&mut self, val: super::vals::Crs3Hpe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Priority Elevation 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe3(&self) -> super::vals::Crs3Hpe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Crs3Hpe3::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 3"]
    #[inline(always)]
    pub const fn set_hpe3(&mut self, val: super::vals::Crs3Hpe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Priority Elevation 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe4(&self) -> super::vals::Crs3Hpe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Crs3Hpe4::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 4"]
    #[inline(always)]
    pub const fn set_hpe4(&mut self, val: super::vals::Crs3Hpe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Priority Elevation 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe5(&self) -> super::vals::Crs3Hpe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Crs3Hpe5::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 5"]
    #[inline(always)]
    pub const fn set_hpe5(&mut self, val: super::vals::Crs3Hpe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Crs3Ro {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Crs3Ro::from_bits(val as u8)
    }
    #[doc = "Read Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: super::vals::Crs3Ro) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Crs3 {
    #[inline(always)]
    fn default() -> Crs3 {
        Crs3(131072u64 as u32)
    }
}
impl core::fmt::Debug for Crs3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crs3")
            .field("park", &self.park())
            .field("pctl", &self.pctl())
            .field("arb", &self.arb())
            .field("hpe0", &self.hpe0())
            .field("hpe1", &self.hpe1())
            .field("hpe2", &self.hpe2())
            .field("hpe3", &self.hpe3())
            .field("hpe4", &self.hpe4())
            .field("hpe5", &self.hpe5())
            .field("ro", &self.ro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crs3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crs3 {
            park: super::vals::Crs3Park,
            pctl: super::vals::Crs3Pctl,
            arb: super::vals::Crs3Arb,
            hpe0: super::vals::Crs3Hpe0,
            hpe1: super::vals::Crs3Hpe1,
            hpe2: super::vals::Crs3Hpe2,
            hpe3: super::vals::Crs3Hpe3,
            hpe4: super::vals::Crs3Hpe4,
            hpe5: super::vals::Crs3Hpe5,
            ro: super::vals::Crs3Ro,
        }
        let proxy = Crs3 {
            park: self.park(),
            pctl: self.pctl(),
            arb: self.arb(),
            hpe0: self.hpe0(),
            hpe1: self.hpe1(),
            hpe2: self.hpe2(),
            hpe3: self.hpe3(),
            hpe4: self.hpe4(),
            hpe5: self.hpe5(),
            ro: self.ro(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs4(pub u32);
impl Crs4 {
    #[doc = "Park"]
    #[must_use]
    #[inline(always)]
    pub const fn park(&self) -> super::vals::Crs4Park {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crs4Park::from_bits(val as u8)
    }
    #[doc = "Park"]
    #[inline(always)]
    pub const fn set_park(&mut self, val: super::vals::Crs4Park) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Parking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pctl(&self) -> super::vals::Crs4Pctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Crs4Pctl::from_bits(val as u8)
    }
    #[doc = "Parking Control"]
    #[inline(always)]
    pub const fn set_pctl(&mut self, val: super::vals::Crs4Pctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Arbitration Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> super::vals::Crs4Arb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Crs4Arb::from_bits(val as u8)
    }
    #[doc = "Arbitration Mode"]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: super::vals::Crs4Arb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Priority Elevation 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe0(&self) -> super::vals::Crs4Hpe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Crs4Hpe0::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 0"]
    #[inline(always)]
    pub const fn set_hpe0(&mut self, val: super::vals::Crs4Hpe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "High Priority Elevation 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe1(&self) -> super::vals::Crs4Hpe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Crs4Hpe1::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 1"]
    #[inline(always)]
    pub const fn set_hpe1(&mut self, val: super::vals::Crs4Hpe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Priority Elevation 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe2(&self) -> super::vals::Crs4Hpe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Crs4Hpe2::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 2"]
    #[inline(always)]
    pub const fn set_hpe2(&mut self, val: super::vals::Crs4Hpe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Priority Elevation 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe3(&self) -> super::vals::Crs4Hpe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Crs4Hpe3::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 3"]
    #[inline(always)]
    pub const fn set_hpe3(&mut self, val: super::vals::Crs4Hpe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Priority Elevation 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe4(&self) -> super::vals::Crs4Hpe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Crs4Hpe4::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 4"]
    #[inline(always)]
    pub const fn set_hpe4(&mut self, val: super::vals::Crs4Hpe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Priority Elevation 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe5(&self) -> super::vals::Crs4Hpe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Crs4Hpe5::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 5"]
    #[inline(always)]
    pub const fn set_hpe5(&mut self, val: super::vals::Crs4Hpe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Crs4Ro {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Crs4Ro::from_bits(val as u8)
    }
    #[doc = "Read Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: super::vals::Crs4Ro) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Crs4 {
    #[inline(always)]
    fn default() -> Crs4 {
        Crs4(131072u64 as u32)
    }
}
impl core::fmt::Debug for Crs4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crs4")
            .field("park", &self.park())
            .field("pctl", &self.pctl())
            .field("arb", &self.arb())
            .field("hpe0", &self.hpe0())
            .field("hpe1", &self.hpe1())
            .field("hpe2", &self.hpe2())
            .field("hpe3", &self.hpe3())
            .field("hpe4", &self.hpe4())
            .field("hpe5", &self.hpe5())
            .field("ro", &self.ro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crs4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crs4 {
            park: super::vals::Crs4Park,
            pctl: super::vals::Crs4Pctl,
            arb: super::vals::Crs4Arb,
            hpe0: super::vals::Crs4Hpe0,
            hpe1: super::vals::Crs4Hpe1,
            hpe2: super::vals::Crs4Hpe2,
            hpe3: super::vals::Crs4Hpe3,
            hpe4: super::vals::Crs4Hpe4,
            hpe5: super::vals::Crs4Hpe5,
            ro: super::vals::Crs4Ro,
        }
        let proxy = Crs4 {
            park: self.park(),
            pctl: self.pctl(),
            arb: self.arb(),
            hpe0: self.hpe0(),
            hpe1: self.hpe1(),
            hpe2: self.hpe2(),
            hpe3: self.hpe3(),
            hpe4: self.hpe4(),
            hpe5: self.hpe5(),
            ro: self.ro(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs5(pub u32);
impl Crs5 {
    #[doc = "Park"]
    #[must_use]
    #[inline(always)]
    pub const fn park(&self) -> super::vals::Crs5Park {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crs5Park::from_bits(val as u8)
    }
    #[doc = "Park"]
    #[inline(always)]
    pub const fn set_park(&mut self, val: super::vals::Crs5Park) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Parking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pctl(&self) -> super::vals::Crs5Pctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Crs5Pctl::from_bits(val as u8)
    }
    #[doc = "Parking Control"]
    #[inline(always)]
    pub const fn set_pctl(&mut self, val: super::vals::Crs5Pctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Arbitration Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> super::vals::Crs5Arb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Crs5Arb::from_bits(val as u8)
    }
    #[doc = "Arbitration Mode"]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: super::vals::Crs5Arb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Priority Elevation 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe0(&self) -> super::vals::Crs5Hpe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Crs5Hpe0::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 0"]
    #[inline(always)]
    pub const fn set_hpe0(&mut self, val: super::vals::Crs5Hpe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "High Priority Elevation 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe1(&self) -> super::vals::Crs5Hpe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Crs5Hpe1::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 1"]
    #[inline(always)]
    pub const fn set_hpe1(&mut self, val: super::vals::Crs5Hpe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Priority Elevation 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe2(&self) -> super::vals::Crs5Hpe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Crs5Hpe2::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 2"]
    #[inline(always)]
    pub const fn set_hpe2(&mut self, val: super::vals::Crs5Hpe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Priority Elevation 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe3(&self) -> super::vals::Crs5Hpe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Crs5Hpe3::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 3"]
    #[inline(always)]
    pub const fn set_hpe3(&mut self, val: super::vals::Crs5Hpe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Priority Elevation 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe4(&self) -> super::vals::Crs5Hpe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Crs5Hpe4::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 4"]
    #[inline(always)]
    pub const fn set_hpe4(&mut self, val: super::vals::Crs5Hpe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Priority Elevation 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe5(&self) -> super::vals::Crs5Hpe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Crs5Hpe5::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 5"]
    #[inline(always)]
    pub const fn set_hpe5(&mut self, val: super::vals::Crs5Hpe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Crs5Ro {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Crs5Ro::from_bits(val as u8)
    }
    #[doc = "Read Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: super::vals::Crs5Ro) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Crs5 {
    #[inline(always)]
    fn default() -> Crs5 {
        Crs5(131072u64 as u32)
    }
}
impl core::fmt::Debug for Crs5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crs5")
            .field("park", &self.park())
            .field("pctl", &self.pctl())
            .field("arb", &self.arb())
            .field("hpe0", &self.hpe0())
            .field("hpe1", &self.hpe1())
            .field("hpe2", &self.hpe2())
            .field("hpe3", &self.hpe3())
            .field("hpe4", &self.hpe4())
            .field("hpe5", &self.hpe5())
            .field("ro", &self.ro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crs5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crs5 {
            park: super::vals::Crs5Park,
            pctl: super::vals::Crs5Pctl,
            arb: super::vals::Crs5Arb,
            hpe0: super::vals::Crs5Hpe0,
            hpe1: super::vals::Crs5Hpe1,
            hpe2: super::vals::Crs5Hpe2,
            hpe3: super::vals::Crs5Hpe3,
            hpe4: super::vals::Crs5Hpe4,
            hpe5: super::vals::Crs5Hpe5,
            ro: super::vals::Crs5Ro,
        }
        let proxy = Crs5 {
            park: self.park(),
            pctl: self.pctl(),
            arb: self.arb(),
            hpe0: self.hpe0(),
            hpe1: self.hpe1(),
            hpe2: self.hpe2(),
            hpe3: self.hpe3(),
            hpe4: self.hpe4(),
            hpe5: self.hpe5(),
            ro: self.ro(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs6(pub u32);
impl Crs6 {
    #[doc = "Park"]
    #[must_use]
    #[inline(always)]
    pub const fn park(&self) -> super::vals::Crs6Park {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crs6Park::from_bits(val as u8)
    }
    #[doc = "Park"]
    #[inline(always)]
    pub const fn set_park(&mut self, val: super::vals::Crs6Park) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Parking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pctl(&self) -> super::vals::Crs6Pctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Crs6Pctl::from_bits(val as u8)
    }
    #[doc = "Parking Control"]
    #[inline(always)]
    pub const fn set_pctl(&mut self, val: super::vals::Crs6Pctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Arbitration Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> super::vals::Crs6Arb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Crs6Arb::from_bits(val as u8)
    }
    #[doc = "Arbitration Mode"]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: super::vals::Crs6Arb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Priority Elevation 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe0(&self) -> super::vals::Crs6Hpe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Crs6Hpe0::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 0"]
    #[inline(always)]
    pub const fn set_hpe0(&mut self, val: super::vals::Crs6Hpe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "High Priority Elevation 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe1(&self) -> super::vals::Crs6Hpe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Crs6Hpe1::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 1"]
    #[inline(always)]
    pub const fn set_hpe1(&mut self, val: super::vals::Crs6Hpe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Priority Elevation 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe2(&self) -> super::vals::Crs6Hpe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Crs6Hpe2::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 2"]
    #[inline(always)]
    pub const fn set_hpe2(&mut self, val: super::vals::Crs6Hpe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Priority Elevation 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe3(&self) -> super::vals::Crs6Hpe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Crs6Hpe3::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 3"]
    #[inline(always)]
    pub const fn set_hpe3(&mut self, val: super::vals::Crs6Hpe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Priority Elevation 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe4(&self) -> super::vals::Crs6Hpe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Crs6Hpe4::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 4"]
    #[inline(always)]
    pub const fn set_hpe4(&mut self, val: super::vals::Crs6Hpe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Priority Elevation 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe5(&self) -> super::vals::Crs6Hpe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Crs6Hpe5::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 5"]
    #[inline(always)]
    pub const fn set_hpe5(&mut self, val: super::vals::Crs6Hpe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Crs6Ro {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Crs6Ro::from_bits(val as u8)
    }
    #[doc = "Read Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: super::vals::Crs6Ro) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Crs6 {
    #[inline(always)]
    fn default() -> Crs6 {
        Crs6(131072u64 as u32)
    }
}
impl core::fmt::Debug for Crs6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crs6")
            .field("park", &self.park())
            .field("pctl", &self.pctl())
            .field("arb", &self.arb())
            .field("hpe0", &self.hpe0())
            .field("hpe1", &self.hpe1())
            .field("hpe2", &self.hpe2())
            .field("hpe3", &self.hpe3())
            .field("hpe4", &self.hpe4())
            .field("hpe5", &self.hpe5())
            .field("ro", &self.ro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crs6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crs6 {
            park: super::vals::Crs6Park,
            pctl: super::vals::Crs6Pctl,
            arb: super::vals::Crs6Arb,
            hpe0: super::vals::Crs6Hpe0,
            hpe1: super::vals::Crs6Hpe1,
            hpe2: super::vals::Crs6Hpe2,
            hpe3: super::vals::Crs6Hpe3,
            hpe4: super::vals::Crs6Hpe4,
            hpe5: super::vals::Crs6Hpe5,
            ro: super::vals::Crs6Ro,
        }
        let proxy = Crs6 {
            park: self.park(),
            pctl: self.pctl(),
            arb: self.arb(),
            hpe0: self.hpe0(),
            hpe1: self.hpe1(),
            hpe2: self.hpe2(),
            hpe3: self.hpe3(),
            hpe4: self.hpe4(),
            hpe5: self.hpe5(),
            ro: self.ro(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crs7(pub u32);
impl Crs7 {
    #[doc = "Park"]
    #[must_use]
    #[inline(always)]
    pub const fn park(&self) -> super::vals::Crs7Park {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Crs7Park::from_bits(val as u8)
    }
    #[doc = "Park"]
    #[inline(always)]
    pub const fn set_park(&mut self, val: super::vals::Crs7Park) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Parking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn pctl(&self) -> super::vals::Crs7Pctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Crs7Pctl::from_bits(val as u8)
    }
    #[doc = "Parking Control"]
    #[inline(always)]
    pub const fn set_pctl(&mut self, val: super::vals::Crs7Pctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Arbitration Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> super::vals::Crs7Arb {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Crs7Arb::from_bits(val as u8)
    }
    #[doc = "Arbitration Mode"]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: super::vals::Crs7Arb) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "High Priority Elevation 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe0(&self) -> super::vals::Crs7Hpe0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Crs7Hpe0::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 0"]
    #[inline(always)]
    pub const fn set_hpe0(&mut self, val: super::vals::Crs7Hpe0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "High Priority Elevation 1"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe1(&self) -> super::vals::Crs7Hpe1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Crs7Hpe1::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 1"]
    #[inline(always)]
    pub const fn set_hpe1(&mut self, val: super::vals::Crs7Hpe1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "High Priority Elevation 2"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe2(&self) -> super::vals::Crs7Hpe2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Crs7Hpe2::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 2"]
    #[inline(always)]
    pub const fn set_hpe2(&mut self, val: super::vals::Crs7Hpe2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "High Priority Elevation 3"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe3(&self) -> super::vals::Crs7Hpe3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Crs7Hpe3::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 3"]
    #[inline(always)]
    pub const fn set_hpe3(&mut self, val: super::vals::Crs7Hpe3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Priority Elevation 4"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe4(&self) -> super::vals::Crs7Hpe4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Crs7Hpe4::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 4"]
    #[inline(always)]
    pub const fn set_hpe4(&mut self, val: super::vals::Crs7Hpe4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "High Priority Elevation 5"]
    #[must_use]
    #[inline(always)]
    pub const fn hpe5(&self) -> super::vals::Crs7Hpe5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Crs7Hpe5::from_bits(val as u8)
    }
    #[doc = "High Priority Elevation 5"]
    #[inline(always)]
    pub const fn set_hpe5(&mut self, val: super::vals::Crs7Hpe5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Read Only"]
    #[must_use]
    #[inline(always)]
    pub const fn ro(&self) -> super::vals::Crs7Ro {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Crs7Ro::from_bits(val as u8)
    }
    #[doc = "Read Only"]
    #[inline(always)]
    pub const fn set_ro(&mut self, val: super::vals::Crs7Ro) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Crs7 {
    #[inline(always)]
    fn default() -> Crs7 {
        Crs7(131072u64 as u32)
    }
}
impl core::fmt::Debug for Crs7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crs7")
            .field("park", &self.park())
            .field("pctl", &self.pctl())
            .field("arb", &self.arb())
            .field("hpe0", &self.hpe0())
            .field("hpe1", &self.hpe1())
            .field("hpe2", &self.hpe2())
            .field("hpe3", &self.hpe3())
            .field("hpe4", &self.hpe4())
            .field("hpe5", &self.hpe5())
            .field("ro", &self.ro())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crs7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crs7 {
            park: super::vals::Crs7Park,
            pctl: super::vals::Crs7Pctl,
            arb: super::vals::Crs7Arb,
            hpe0: super::vals::Crs7Hpe0,
            hpe1: super::vals::Crs7Hpe1,
            hpe2: super::vals::Crs7Hpe2,
            hpe3: super::vals::Crs7Hpe3,
            hpe4: super::vals::Crs7Hpe4,
            hpe5: super::vals::Crs7Hpe5,
            ro: super::vals::Crs7Ro,
        }
        let proxy = Crs7 {
            park: self.park(),
            pctl: self.pctl(),
            arb: self.arb(),
            hpe0: self.hpe0(),
            hpe1: self.hpe1(),
            hpe2: self.hpe2(),
            hpe3: self.hpe3(),
            hpe4: self.hpe4(),
            hpe5: self.hpe5(),
            ro: self.ro(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Priority Slave Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs0(pub u32);
impl Prs0 {
    #[doc = "Master 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> super::vals::Prs0M0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prs0M0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priority"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: super::vals::Prs0M0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> super::vals::Prs0M1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prs0M1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priority"]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: super::vals::Prs0M1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master 2 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> super::vals::Prs0M2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Prs0M2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priority"]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: super::vals::Prs0M2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Master 3 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> super::vals::Prs0M3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Prs0M3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priority"]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: super::vals::Prs0M3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Master 4 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> super::vals::Prs0M4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Prs0M4::from_bits(val as u8)
    }
    #[doc = "Master 4 Priority"]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: super::vals::Prs0M4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Master 5 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> super::vals::Prs0M5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Prs0M5::from_bits(val as u8)
    }
    #[doc = "Master 5 Priority"]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: super::vals::Prs0M5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Prs0 {
    #[inline(always)]
    fn default() -> Prs0 {
        Prs0(1263109u64 as u32)
    }
}
impl core::fmt::Debug for Prs0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prs0")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prs0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prs0 {
            m0: super::vals::Prs0M0,
            m1: super::vals::Prs0M1,
            m2: super::vals::Prs0M2,
            m3: super::vals::Prs0M3,
            m4: super::vals::Prs0M4,
            m5: super::vals::Prs0M5,
        }
        let proxy = Prs0 {
            m0: self.m0(),
            m1: self.m1(),
            m2: self.m2(),
            m3: self.m3(),
            m4: self.m4(),
            m5: self.m5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Priority Slave Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs1(pub u32);
impl Prs1 {
    #[doc = "Master 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> super::vals::Prs1M0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prs1M0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priority"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: super::vals::Prs1M0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> super::vals::Prs1M1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prs1M1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priority"]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: super::vals::Prs1M1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master 2 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> super::vals::Prs1M2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Prs1M2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priority"]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: super::vals::Prs1M2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Master 3 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> super::vals::Prs1M3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Prs1M3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priority"]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: super::vals::Prs1M3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Master 4 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> super::vals::Prs1M4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Prs1M4::from_bits(val as u8)
    }
    #[doc = "Master 4 Priority"]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: super::vals::Prs1M4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Master 5 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> super::vals::Prs1M5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Prs1M5::from_bits(val as u8)
    }
    #[doc = "Master 5 Priority"]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: super::vals::Prs1M5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Prs1 {
    #[inline(always)]
    fn default() -> Prs1 {
        Prs1(1263109u64 as u32)
    }
}
impl core::fmt::Debug for Prs1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prs1")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prs1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prs1 {
            m0: super::vals::Prs1M0,
            m1: super::vals::Prs1M1,
            m2: super::vals::Prs1M2,
            m3: super::vals::Prs1M3,
            m4: super::vals::Prs1M4,
            m5: super::vals::Prs1M5,
        }
        let proxy = Prs1 {
            m0: self.m0(),
            m1: self.m1(),
            m2: self.m2(),
            m3: self.m3(),
            m4: self.m4(),
            m5: self.m5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Priority Slave Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs2(pub u32);
impl Prs2 {
    #[doc = "Master 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> super::vals::Prs2M0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prs2M0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priority"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: super::vals::Prs2M0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> super::vals::Prs2M1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prs2M1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priority"]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: super::vals::Prs2M1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master 2 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> super::vals::Prs2M2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Prs2M2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priority"]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: super::vals::Prs2M2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Master 3 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> super::vals::Prs2M3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Prs2M3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priority"]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: super::vals::Prs2M3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Master 4 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> super::vals::Prs2M4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Prs2M4::from_bits(val as u8)
    }
    #[doc = "Master 4 Priority"]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: super::vals::Prs2M4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Master 5 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> super::vals::Prs2M5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Prs2M5::from_bits(val as u8)
    }
    #[doc = "Master 5 Priority"]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: super::vals::Prs2M5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Prs2 {
    #[inline(always)]
    fn default() -> Prs2 {
        Prs2(1263109u64 as u32)
    }
}
impl core::fmt::Debug for Prs2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prs2")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prs2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prs2 {
            m0: super::vals::Prs2M0,
            m1: super::vals::Prs2M1,
            m2: super::vals::Prs2M2,
            m3: super::vals::Prs2M3,
            m4: super::vals::Prs2M4,
            m5: super::vals::Prs2M5,
        }
        let proxy = Prs2 {
            m0: self.m0(),
            m1: self.m1(),
            m2: self.m2(),
            m3: self.m3(),
            m4: self.m4(),
            m5: self.m5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Priority Slave Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs3(pub u32);
impl Prs3 {
    #[doc = "Master 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> super::vals::Prs3M0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prs3M0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priority"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: super::vals::Prs3M0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> super::vals::Prs3M1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prs3M1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priority"]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: super::vals::Prs3M1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master 2 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> super::vals::Prs3M2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Prs3M2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priority"]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: super::vals::Prs3M2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Master 3 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> super::vals::Prs3M3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Prs3M3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priority"]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: super::vals::Prs3M3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Master 4 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> super::vals::Prs3M4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Prs3M4::from_bits(val as u8)
    }
    #[doc = "Master 4 Priority"]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: super::vals::Prs3M4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Master 5 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> super::vals::Prs3M5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Prs3M5::from_bits(val as u8)
    }
    #[doc = "Master 5 Priority"]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: super::vals::Prs3M5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Prs3 {
    #[inline(always)]
    fn default() -> Prs3 {
        Prs3(1263109u64 as u32)
    }
}
impl core::fmt::Debug for Prs3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prs3")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prs3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prs3 {
            m0: super::vals::Prs3M0,
            m1: super::vals::Prs3M1,
            m2: super::vals::Prs3M2,
            m3: super::vals::Prs3M3,
            m4: super::vals::Prs3M4,
            m5: super::vals::Prs3M5,
        }
        let proxy = Prs3 {
            m0: self.m0(),
            m1: self.m1(),
            m2: self.m2(),
            m3: self.m3(),
            m4: self.m4(),
            m5: self.m5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Priority Slave Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs4(pub u32);
impl Prs4 {
    #[doc = "Master 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> super::vals::Prs4M0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prs4M0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priority"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: super::vals::Prs4M0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> super::vals::Prs4M1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prs4M1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priority"]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: super::vals::Prs4M1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master 2 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> super::vals::Prs4M2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Prs4M2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priority"]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: super::vals::Prs4M2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Master 3 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> super::vals::Prs4M3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Prs4M3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priority"]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: super::vals::Prs4M3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Master 4 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> super::vals::Prs4M4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Prs4M4::from_bits(val as u8)
    }
    #[doc = "Master 4 Priority"]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: super::vals::Prs4M4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Master 5 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> super::vals::Prs4M5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Prs4M5::from_bits(val as u8)
    }
    #[doc = "Master 5 Priority"]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: super::vals::Prs4M5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Prs4 {
    #[inline(always)]
    fn default() -> Prs4 {
        Prs4(1263109u64 as u32)
    }
}
impl core::fmt::Debug for Prs4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prs4")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prs4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prs4 {
            m0: super::vals::Prs4M0,
            m1: super::vals::Prs4M1,
            m2: super::vals::Prs4M2,
            m3: super::vals::Prs4M3,
            m4: super::vals::Prs4M4,
            m5: super::vals::Prs4M5,
        }
        let proxy = Prs4 {
            m0: self.m0(),
            m1: self.m1(),
            m2: self.m2(),
            m3: self.m3(),
            m4: self.m4(),
            m5: self.m5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Priority Slave Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs5(pub u32);
impl Prs5 {
    #[doc = "Master 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> super::vals::Prs5M0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prs5M0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priority"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: super::vals::Prs5M0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> super::vals::Prs5M1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prs5M1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priority"]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: super::vals::Prs5M1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master 2 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> super::vals::Prs5M2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Prs5M2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priority"]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: super::vals::Prs5M2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Master 3 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> super::vals::Prs5M3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Prs5M3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priority"]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: super::vals::Prs5M3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Master 4 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> super::vals::Prs5M4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Prs5M4::from_bits(val as u8)
    }
    #[doc = "Master 4 Priority"]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: super::vals::Prs5M4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Master 5 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> super::vals::Prs5M5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Prs5M5::from_bits(val as u8)
    }
    #[doc = "Master 5 Priority"]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: super::vals::Prs5M5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Prs5 {
    #[inline(always)]
    fn default() -> Prs5 {
        Prs5(1263109u64 as u32)
    }
}
impl core::fmt::Debug for Prs5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prs5")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prs5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prs5 {
            m0: super::vals::Prs5M0,
            m1: super::vals::Prs5M1,
            m2: super::vals::Prs5M2,
            m3: super::vals::Prs5M3,
            m4: super::vals::Prs5M4,
            m5: super::vals::Prs5M5,
        }
        let proxy = Prs5 {
            m0: self.m0(),
            m1: self.m1(),
            m2: self.m2(),
            m3: self.m3(),
            m4: self.m4(),
            m5: self.m5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Priority Slave Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs6(pub u32);
impl Prs6 {
    #[doc = "Master 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> super::vals::Prs6M0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prs6M0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priority"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: super::vals::Prs6M0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> super::vals::Prs6M1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prs6M1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priority"]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: super::vals::Prs6M1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master 2 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> super::vals::Prs6M2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Prs6M2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priority"]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: super::vals::Prs6M2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Master 3 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> super::vals::Prs6M3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Prs6M3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priority"]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: super::vals::Prs6M3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Master 4 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> super::vals::Prs6M4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Prs6M4::from_bits(val as u8)
    }
    #[doc = "Master 4 Priority"]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: super::vals::Prs6M4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Master 5 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> super::vals::Prs6M5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Prs6M5::from_bits(val as u8)
    }
    #[doc = "Master 5 Priority"]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: super::vals::Prs6M5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Prs6 {
    #[inline(always)]
    fn default() -> Prs6 {
        Prs6(1263109u64 as u32)
    }
}
impl core::fmt::Debug for Prs6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prs6")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prs6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prs6 {
            m0: super::vals::Prs6M0,
            m1: super::vals::Prs6M1,
            m2: super::vals::Prs6M2,
            m3: super::vals::Prs6M3,
            m4: super::vals::Prs6M4,
            m5: super::vals::Prs6M5,
        }
        let proxy = Prs6 {
            m0: self.m0(),
            m1: self.m1(),
            m2: self.m2(),
            m3: self.m3(),
            m4: self.m4(),
            m5: self.m5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Priority Slave Registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prs7(pub u32);
impl Prs7 {
    #[doc = "Master 0 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m0(&self) -> super::vals::Prs7M0 {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Prs7M0::from_bits(val as u8)
    }
    #[doc = "Master 0 Priority"]
    #[inline(always)]
    pub const fn set_m0(&mut self, val: super::vals::Prs7M0) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master 1 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m1(&self) -> super::vals::Prs7M1 {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Prs7M1::from_bits(val as u8)
    }
    #[doc = "Master 1 Priority"]
    #[inline(always)]
    pub const fn set_m1(&mut self, val: super::vals::Prs7M1) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Master 2 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m2(&self) -> super::vals::Prs7M2 {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Prs7M2::from_bits(val as u8)
    }
    #[doc = "Master 2 Priority"]
    #[inline(always)]
    pub const fn set_m2(&mut self, val: super::vals::Prs7M2) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Master 3 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m3(&self) -> super::vals::Prs7M3 {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Prs7M3::from_bits(val as u8)
    }
    #[doc = "Master 3 Priority"]
    #[inline(always)]
    pub const fn set_m3(&mut self, val: super::vals::Prs7M3) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Master 4 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m4(&self) -> super::vals::Prs7M4 {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Prs7M4::from_bits(val as u8)
    }
    #[doc = "Master 4 Priority"]
    #[inline(always)]
    pub const fn set_m4(&mut self, val: super::vals::Prs7M4) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Master 5 Priority"]
    #[must_use]
    #[inline(always)]
    pub const fn m5(&self) -> super::vals::Prs7M5 {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Prs7M5::from_bits(val as u8)
    }
    #[doc = "Master 5 Priority"]
    #[inline(always)]
    pub const fn set_m5(&mut self, val: super::vals::Prs7M5) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
}
impl Default for Prs7 {
    #[inline(always)]
    fn default() -> Prs7 {
        Prs7(1263109u64 as u32)
    }
}
impl core::fmt::Debug for Prs7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prs7")
            .field("m0", &self.m0())
            .field("m1", &self.m1())
            .field("m2", &self.m2())
            .field("m3", &self.m3())
            .field("m4", &self.m4())
            .field("m5", &self.m5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prs7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prs7 {
            m0: super::vals::Prs7M0,
            m1: super::vals::Prs7M1,
            m2: super::vals::Prs7M2,
            m3: super::vals::Prs7M3,
            m4: super::vals::Prs7M4,
            m5: super::vals::Prs7M5,
        }
        let proxy = Prs7 {
            m0: self.m0(),
            m1: self.m1(),
            m2: self.m2(),
            m3: self.m3(),
            m4: self.m4(),
            m5: self.m5(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
