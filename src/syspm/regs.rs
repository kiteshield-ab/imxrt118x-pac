#[doc = "Performance Monitor Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcr(pub u32);
impl Pmcr {
    #[doc = "Module Is Enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn menb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Module Is Enabled"]
    #[inline(always)]
    pub const fn set_menb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Start and Stop Control"]
    #[must_use]
    #[inline(always)]
    pub const fn ssc(&self) -> super::vals::Ssc {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Ssc::from_bits(val as u8)
    }
    #[doc = "Start and Stop Control"]
    #[inline(always)]
    pub const fn set_ssc(&mut self, val: super::vals::Ssc) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Count Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn cmode(&self) -> super::vals::Cmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Cmode::from_bits(val as u8)
    }
    #[doc = "Count Mode"]
    #[inline(always)]
    pub const fn set_cmode(&mut self, val: super::vals::Cmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Disable Counters if Stopped or Halted"]
    #[must_use]
    #[inline(always)]
    pub const fn dcifsh(&self) -> super::vals::Dcifsh {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dcifsh::from_bits(val as u8)
    }
    #[doc = "Disable Counters if Stopped or Halted"]
    #[inline(always)]
    pub const fn set_dcifsh(&mut self, val: super::vals::Dcifsh) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Reset Instruction Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn rictr(&self) -> super::vals::Rictr {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Rictr::from_bits(val as u8)
    }
    #[doc = "Reset Instruction Counter"]
    #[inline(always)]
    pub const fn set_rictr(&mut self, val: super::vals::Rictr) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Reset Event Counter 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rectr1(&self) -> super::vals::Rectr1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Rectr1::from_bits(val as u8)
    }
    #[doc = "Reset Event Counter 1"]
    #[inline(always)]
    pub const fn set_rectr1(&mut self, val: super::vals::Rectr1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Reset Event Counter 2"]
    #[must_use]
    #[inline(always)]
    pub const fn rectr2(&self) -> super::vals::Rectr2 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Rectr2::from_bits(val as u8)
    }
    #[doc = "Reset Event Counter 2"]
    #[inline(always)]
    pub const fn set_rectr2(&mut self, val: super::vals::Rectr2) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Reset Event Counter 3"]
    #[must_use]
    #[inline(always)]
    pub const fn rectr3(&self) -> super::vals::Rectr3 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rectr3::from_bits(val as u8)
    }
    #[doc = "Reset Event Counter 3"]
    #[inline(always)]
    pub const fn set_rectr3(&mut self, val: super::vals::Rectr3) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Select Event 1"]
    #[must_use]
    #[inline(always)]
    pub const fn selevt1(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x7f;
        val as u8
    }
    #[doc = "Select Event 1"]
    #[inline(always)]
    pub const fn set_selevt1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 11usize)) | (((val as u32) & 0x7f) << 11usize);
    }
    #[doc = "Select Event 2"]
    #[must_use]
    #[inline(always)]
    pub const fn selevt2(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x7f;
        val as u8
    }
    #[doc = "Select Event 2"]
    #[inline(always)]
    pub const fn set_selevt2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
    }
    #[doc = "Select Event 3"]
    #[must_use]
    #[inline(always)]
    pub const fn selevt3(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Select Event 3"]
    #[inline(always)]
    pub const fn set_selevt3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Pmcr {
    #[inline(always)]
    fn default() -> Pmcr {
        Pmcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmcr")
            .field("menb", &self.menb())
            .field("ssc", &self.ssc())
            .field("cmode", &self.cmode())
            .field("dcifsh", &self.dcifsh())
            .field("rictr", &self.rictr())
            .field("rectr1", &self.rectr1())
            .field("rectr2", &self.rectr2())
            .field("rectr3", &self.rectr3())
            .field("selevt1", &self.selevt1())
            .field("selevt2", &self.selevt2())
            .field("selevt3", &self.selevt3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pmcr {
            menb: bool,
            ssc: super::vals::Ssc,
            cmode: super::vals::Cmode,
            dcifsh: super::vals::Dcifsh,
            rictr: super::vals::Rictr,
            rectr1: super::vals::Rectr1,
            rectr2: super::vals::Rectr2,
            rectr3: super::vals::Rectr3,
            selevt1: u8,
            selevt2: u8,
            selevt3: u8,
        }
        let proxy = Pmcr {
            menb: self.menb(),
            ssc: self.ssc(),
            cmode: self.cmode(),
            dcifsh: self.dcifsh(),
            rictr: self.rictr(),
            rectr1: self.rectr1(),
            rectr2: self.rectr2(),
            rectr3: self.rectr3(),
            selevt1: self.selevt1(),
            selevt2: self.selevt2(),
            selevt3: self.selevt3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
