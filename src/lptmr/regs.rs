#[doc = "Control Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "Timer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> super::vals::Ten {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ten::from_bits(val as u8)
    }
    #[doc = "Timer Enable"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: super::vals::Ten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Mode Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tms(&self) -> super::vals::Tms {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tms::from_bits(val as u8)
    }
    #[doc = "Timer Mode Select"]
    #[inline(always)]
    pub const fn set_tms(&mut self, val: super::vals::Tms) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer Free-Running Counter"]
    #[must_use]
    #[inline(always)]
    pub const fn tfc(&self) -> super::vals::Tfc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tfc::from_bits(val as u8)
    }
    #[doc = "Timer Free-Running Counter"]
    #[inline(always)]
    pub const fn set_tfc(&mut self, val: super::vals::Tfc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Timer Pin Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn tpp(&self) -> super::vals::Tpp {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tpp::from_bits(val as u8)
    }
    #[doc = "Timer Pin Polarity"]
    #[inline(always)]
    pub const fn set_tpp(&mut self, val: super::vals::Tpp) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Timer Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn tps(&self) -> super::vals::Tps {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Tps::from_bits(val as u8)
    }
    #[doc = "Timer Pin Select"]
    #[inline(always)]
    pub const fn set_tps(&mut self, val: super::vals::Tps) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie(&self) -> super::vals::Tie {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Tie::from_bits(val as u8)
    }
    #[doc = "Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie(&mut self, val: super::vals::Tie) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Compare Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tcf(&self) -> super::vals::Tcf {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Tcf::from_bits(val as u8)
    }
    #[doc = "Timer Compare Flag"]
    #[inline(always)]
    pub const fn set_tcf(&mut self, val: super::vals::Tcf) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer DMA Request Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tdre(&self) -> super::vals::Tdre {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Tdre::from_bits(val as u8)
    }
    #[doc = "Timer DMA Request Enable"]
    #[inline(always)]
    pub const fn set_tdre(&mut self, val: super::vals::Tdre) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
            .field("ten", &self.ten())
            .field("tms", &self.tms())
            .field("tfc", &self.tfc())
            .field("tpp", &self.tpp())
            .field("tps", &self.tps())
            .field("tie", &self.tie())
            .field("tcf", &self.tcf())
            .field("tdre", &self.tdre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Csr {
            ten: super::vals::Ten,
            tms: super::vals::Tms,
            tfc: super::vals::Tfc,
            tpp: super::vals::Tpp,
            tps: super::vals::Tps,
            tie: super::vals::Tie,
            tcf: super::vals::Tcf,
            tdre: super::vals::Tdre,
        }
        let proxy = Csr {
            ten: self.ten(),
            tms: self.tms(),
            tfc: self.tfc(),
            tpp: self.tpp(),
            tps: self.tps(),
            tie: self.tie(),
            tcf: self.tcf(),
            tdre: self.tdre(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Prescaler and Glitch Filter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc = "Prescaler and Glitch Filter Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs(&self) -> super::vals::Pcs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Pcs::from_bits(val as u8)
    }
    #[doc = "Prescaler and Glitch Filter Clock Select"]
    #[inline(always)]
    pub const fn set_pcs(&mut self, val: super::vals::Pcs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Prescaler and Glitch Filter Bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn pbyp(&self) -> super::vals::Pbyp {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pbyp::from_bits(val as u8)
    }
    #[doc = "Prescaler and Glitch Filter Bypass"]
    #[inline(always)]
    pub const fn set_pbyp(&mut self, val: super::vals::Pbyp) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Prescaler and Glitch Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> super::vals::Prescale {
        let val = (self.0 >> 3usize) & 0x0f;
        super::vals::Prescale::from_bits(val as u8)
    }
    #[doc = "Prescaler and Glitch Filter Value"]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: super::vals::Prescale) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val.to_bits() as u32) & 0x0f) << 3usize);
    }
}
impl Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        Psr(0u64 as u32)
    }
}
impl core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psr")
            .field("pcs", &self.pcs())
            .field("pbyp", &self.pbyp())
            .field("prescale", &self.prescale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psr {
            pcs: super::vals::Pcs,
            pbyp: super::vals::Pbyp,
            prescale: super::vals::Prescale,
        }
        let proxy = Psr {
            pcs: self.pcs(),
            pbyp: self.pbyp(),
            prescale: self.prescale(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
