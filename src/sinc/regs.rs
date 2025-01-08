#[doc = "Channel n Advanced Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacfr(pub u32);
impl Cacfr {
    #[doc = "Alternate DMA Source Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn admasel(&self) -> super::vals::Admasel {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Admasel::from_bits(val as u8)
    }
    #[doc = "Alternate DMA Source Selection"]
    #[inline(always)]
    pub const fn set_admasel(&mut self, val: super::vals::Admasel) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "HPF DC Remover Alpha Coefficient"]
    #[must_use]
    #[inline(always)]
    pub const fn hpfa(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "HPF DC Remover Alpha Coefficient"]
    #[inline(always)]
    pub const fn set_hpfa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Input Modulator Bitstream Delay"]
    #[must_use]
    #[inline(always)]
    pub const fn ibdly(&self) -> super::vals::Ibdly {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Ibdly::from_bits(val as u8)
    }
    #[doc = "Input Modulator Bitstream Delay"]
    #[inline(always)]
    pub const fn set_ibdly(&mut self, val: super::vals::Ibdly) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Pulse Trigger Mux Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ptmux(&self) -> super::vals::Ptmux {
        let val = (self.0 >> 24usize) & 0x1f;
        super::vals::Ptmux::from_bits(val as u8)
    }
    #[doc = "Pulse Trigger Mux Select"]
    #[inline(always)]
    pub const fn set_ptmux(&mut self, val: super::vals::Ptmux) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val.to_bits() as u32) & 0x1f) << 24usize);
    }
}
impl Default for Cacfr {
    #[inline(always)]
    fn default() -> Cacfr {
        Cacfr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cacfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacfr")
            .field("admasel", &self.admasel())
            .field("hpfa", &self.hpfa())
            .field("ibdly", &self.ibdly())
            .field("ptmux", &self.ptmux())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacfr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cacfr {
            admasel: super::vals::Admasel,
            hpfa: u8,
            ibdly: super::vals::Ibdly,
            ptmux: super::vals::Ptmux,
        }
        let proxy = Cacfr {
            admasel: self.admasel(),
            hpfa: self.hpfa(),
            ibdly: self.ibdly(),
            ptmux: self.ptmux(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Bias"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbias(pub u32);
impl Cbias {
    #[doc = "Bias Value"]
    #[must_use]
    #[inline(always)]
    pub const fn bias(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Bias Value"]
    #[inline(always)]
    pub const fn set_bias(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Cbias {
    #[inline(always)]
    fn default() -> Cbias {
        Cbias(0u64 as u32)
    }
}
impl core::fmt::Debug for Cbias {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cbias").field("bias", &self.bias()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cbias {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cbias {
            bias: u32,
        }
        let proxy = Cbias { bias: self.bias() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccfr(pub u32);
impl Ccfr {
    #[doc = "PF Shift"]
    #[must_use]
    #[inline(always)]
    pub const fn pfsft(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "PF Shift"]
    #[inline(always)]
    pub const fn set_pfsft(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Result Data Format"]
    #[must_use]
    #[inline(always)]
    pub const fn rdfmt(&self) -> super::vals::Rdfmt {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Rdfmt::from_bits(val as u8)
    }
    #[doc = "Result Data Format"]
    #[inline(always)]
    pub const fn set_rdfmt(&mut self, val: super::vals::Rdfmt) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FIFO Watermark"]
    #[must_use]
    #[inline(always)]
    pub const fn fifowmk(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[doc = "FIFO Watermark"]
    #[inline(always)]
    pub const fn set_fifowmk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "Input Bit Format"]
    #[must_use]
    #[inline(always)]
    pub const fn ibfmt(&self) -> super::vals::Ibfmt {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Ibfmt::from_bits(val as u8)
    }
    #[doc = "Input Bit Format"]
    #[inline(always)]
    pub const fn set_ibfmt(&mut self, val: super::vals::Ibfmt) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Input Clock Select"]
    #[must_use]
    #[inline(always)]
    pub const fn icsel(&self) -> super::vals::Icsel {
        let val = (self.0 >> 18usize) & 0x07;
        super::vals::Icsel::from_bits(val as u8)
    }
    #[doc = "Input Clock Select"]
    #[inline(always)]
    pub const fn set_icsel(&mut self, val: super::vals::Icsel) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
    }
    #[doc = "Input Clock Edge Select"]
    #[must_use]
    #[inline(always)]
    pub const fn icesel(&self) -> super::vals::Icesel {
        let val = (self.0 >> 21usize) & 0x07;
        super::vals::Icesel::from_bits(val as u8)
    }
    #[doc = "Input Clock Edge Select"]
    #[inline(always)]
    pub const fn set_icesel(&mut self, val: super::vals::Icesel) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
    }
    #[doc = "Input Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn itsel(&self) -> super::vals::Itsel {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Itsel::from_bits(val as u8)
    }
    #[doc = "Input Trigger Select"]
    #[inline(always)]
    pub const fn set_itsel(&mut self, val: super::vals::Itsel) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
    #[doc = "Input Bit Select"]
    #[must_use]
    #[inline(always)]
    pub const fn ibsel(&self) -> super::vals::Ibsel {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Ibsel::from_bits(val as u8)
    }
    #[doc = "Input Bit Select"]
    #[inline(always)]
    pub const fn set_ibsel(&mut self, val: super::vals::Ibsel) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Input Trigger Level Type"]
    #[must_use]
    #[inline(always)]
    pub const fn itlvl(&self) -> super::vals::Itlvl {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Itlvl::from_bits(val as u8)
    }
    #[doc = "Input Trigger Level Type"]
    #[inline(always)]
    pub const fn set_itlvl(&mut self, val: super::vals::Itlvl) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Zero Cross Option"]
    #[must_use]
    #[inline(always)]
    pub const fn zcop(&self) -> super::vals::Zcop {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::Zcop::from_bits(val as u8)
    }
    #[doc = "Zero Cross Option"]
    #[inline(always)]
    pub const fn set_zcop(&mut self, val: super::vals::Zcop) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Ccfr {
    #[inline(always)]
    fn default() -> Ccfr {
        Ccfr(2097152u64 as u32)
    }
}
impl core::fmt::Debug for Ccfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccfr")
            .field("pfsft", &self.pfsft())
            .field("rdfmt", &self.rdfmt())
            .field("fifowmk", &self.fifowmk())
            .field("ibfmt", &self.ibfmt())
            .field("icsel", &self.icsel())
            .field("icesel", &self.icesel())
            .field("itsel", &self.itsel())
            .field("ibsel", &self.ibsel())
            .field("itlvl", &self.itlvl())
            .field("zcop", &self.zcop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccfr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ccfr {
            pfsft: u8,
            rdfmt: super::vals::Rdfmt,
            fifowmk: u8,
            ibfmt: super::vals::Ibfmt,
            icsel: super::vals::Icsel,
            icesel: super::vals::Icesel,
            itsel: super::vals::Itsel,
            ibsel: super::vals::Ibsel,
            itlvl: super::vals::Itlvl,
            zcop: super::vals::Zcop,
        }
        let proxy = Ccfr {
            pfsft: self.pfsft(),
            rdfmt: self.rdfmt(),
            fifowmk: self.fifowmk(),
            ibfmt: self.ibfmt(),
            icsel: self.icsel(),
            icesel: self.icesel(),
            itsel: self.itsel(),
            ibsel: self.ibsel(),
            itlvl: self.itlvl(),
            zcop: self.zcop(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Channel Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chen(&self) -> super::vals::Chen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Chen::from_bits(val as u8)
    }
    #[doc = "Channel Enable"]
    #[inline(always)]
    pub const fn set_chen(&mut self, val: super::vals::Chen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PF Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pfen(&self) -> super::vals::Pfen {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pfen::from_bits(val as u8)
    }
    #[doc = "PF Enable"]
    #[inline(always)]
    pub const fn set_pfen(&mut self, val: super::vals::Pfen) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> super::vals::Dmaen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: super::vals::Dmaen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Short Circuit Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn scden(&self) -> super::vals::Scden {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Scden::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detect Enable"]
    #[inline(always)]
    pub const fn set_scden(&mut self, val: super::vals::Scden) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Clock Absence Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn caden(&self) -> super::vals::Caden {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Caden::from_bits(val as u8)
    }
    #[doc = "Clock Absence Detect Enable"]
    #[inline(always)]
    pub const fn set_caden(&mut self, val: super::vals::Caden) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Zero Cross Detect Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn zcden(&self) -> super::vals::Zcden {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Zcden::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detect Enable"]
    #[inline(always)]
    pub const fn set_zcden(&mut self, val: super::vals::Zcden) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Limit Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn lmten(&self) -> super::vals::Lmten {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Lmten::from_bits(val as u8)
    }
    #[doc = "Limit Enable"]
    #[inline(always)]
    pub const fn set_lmten(&mut self, val: super::vals::Lmten) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FIFO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoen(&self) -> super::vals::Fifoen {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Fifoen::from_bits(val as u8)
    }
    #[doc = "FIFO Enable"]
    #[inline(always)]
    pub const fn set_fifoen(&mut self, val: super::vals::Fifoen) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Debug Output Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgsel(&self) -> super::vals::Dbgsel {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::Dbgsel::from_bits(val as u8)
    }
    #[doc = "Debug Output Selection"]
    #[inline(always)]
    pub const fn set_dbgsel(&mut self, val: super::vals::Dbgsel) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("chen", &self.chen())
            .field("pfen", &self.pfen())
            .field("dmaen", &self.dmaen())
            .field("scden", &self.scden())
            .field("caden", &self.caden())
            .field("zcden", &self.zcden())
            .field("lmten", &self.lmten())
            .field("fifoen", &self.fifoen())
            .field("dbgsel", &self.dbgsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ccr {
            chen: super::vals::Chen,
            pfen: super::vals::Pfen,
            dmaen: super::vals::Dmaen,
            scden: super::vals::Scden,
            caden: super::vals::Caden,
            zcden: super::vals::Zcden,
            lmten: super::vals::Lmten,
            fifoen: super::vals::Fifoen,
            dbgsel: super::vals::Dbgsel,
        }
        let proxy = Ccr {
            chen: self.chen(),
            pfen: self.pfen(),
            dmaen: self.dmaen(),
            scden: self.scden(),
            caden: self.caden(),
            zcden: self.zcden(),
            lmten: self.lmten(),
            fifoen: self.fifoen(),
            dbgsel: self.dbgsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Data Rate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr(pub u32);
impl Cdr {
    #[doc = "PF OSR"]
    #[must_use]
    #[inline(always)]
    pub const fn pfosr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "PF OSR"]
    #[inline(always)]
    pub const fn set_pfosr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "PF Order"]
    #[must_use]
    #[inline(always)]
    pub const fn pford(&self) -> super::vals::Pford {
        let val = (self.0 >> 11usize) & 0x03;
        super::vals::Pford::from_bits(val as u8)
    }
    #[doc = "PF Order"]
    #[inline(always)]
    pub const fn set_pford(&mut self, val: super::vals::Pford) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
    }
    #[doc = "PF Conversion Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pfcm(&self) -> super::vals::Pfcm {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Pfcm::from_bits(val as u8)
    }
    #[doc = "PF Conversion Mode"]
    #[inline(always)]
    pub const fn set_pfcm(&mut self, val: super::vals::Pfcm) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        Cdr(6144u64 as u32)
    }
}
impl core::fmt::Debug for Cdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdr")
            .field("pfosr", &self.pfosr())
            .field("pford", &self.pford())
            .field("pfcm", &self.pfcm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cdr {
            pfosr: u16,
            pford: super::vals::Pford,
            pfcm: super::vals::Pfcm,
        }
        let proxy = Cdr {
            pfosr: self.pfosr(),
            pford: self.pford(),
            pfcm: self.pfcm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n High Limit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chilmt(pub u32);
impl Chilmt {
    #[doc = "High Limit Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn hilmt(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "High Limit Threshold"]
    #[inline(always)]
    pub const fn set_hilmt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Chilmt {
    #[inline(always)]
    fn default() -> Chilmt {
        Chilmt(0u64 as u32)
    }
}
impl core::fmt::Debug for Chilmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Chilmt")
            .field("hilmt", &self.hilmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Chilmt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Chilmt {
            hilmt: u32,
        }
        let proxy = Chilmt {
            hilmt: self.hilmt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Low Limit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clolmt(pub u32);
impl Clolmt {
    #[doc = "Low Limit Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn lolmt(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Low Limit Threshold"]
    #[inline(always)]
    pub const fn set_lolmt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Clolmt {
    #[inline(always)]
    fn default() -> Clolmt {
        Clolmt(0u64 as u32)
    }
}
impl core::fmt::Debug for Clolmt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clolmt")
            .field("lolmt", &self.lolmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clolmt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Clolmt {
            lolmt: u32,
        }
        let proxy = Clolmt {
            lolmt: self.lolmt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Protection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cprot(pub u32);
impl Cprot {
    #[doc = "SCD Limit Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn scdlmt(&self) -> super::vals::Scdlmt {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Scdlmt::from_bits(val as u8)
    }
    #[doc = "SCD Limit Threshold"]
    #[inline(always)]
    pub const fn set_scdlmt(&mut self, val: super::vals::Scdlmt) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "SCD Conversion Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn scdcm(&self) -> super::vals::Scdcm {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Scdcm::from_bits(val as u8)
    }
    #[doc = "SCD Conversion Mode"]
    #[inline(always)]
    pub const fn set_scdcm(&mut self, val: super::vals::Scdcm) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SCD Option"]
    #[must_use]
    #[inline(always)]
    pub const fn scdop(&self) -> super::vals::Scdop {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Scdop::from_bits(val as u8)
    }
    #[doc = "SCD Option"]
    #[inline(always)]
    pub const fn set_scdop(&mut self, val: super::vals::Scdop) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Limit Detection Option"]
    #[must_use]
    #[inline(always)]
    pub const fn lmtop(&self) -> super::vals::Lmtop {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Lmtop::from_bits(val as u8)
    }
    #[doc = "Limit Detection Option"]
    #[inline(always)]
    pub const fn set_lmtop(&mut self, val: super::vals::Lmtop) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "CAD Limit Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn cadlmt(&self) -> super::vals::Cadlmt {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::Cadlmt::from_bits(val as u8)
    }
    #[doc = "CAD Limit Threshold"]
    #[inline(always)]
    pub const fn set_cadlmt(&mut self, val: super::vals::Cadlmt) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "CAD Break Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn cadbk(&self) -> super::vals::Cadbk {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Cadbk::from_bits(val as u8)
    }
    #[doc = "CAD Break Signal"]
    #[inline(always)]
    pub const fn set_cadbk(&mut self, val: super::vals::Cadbk) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "SCD Break Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn scdbk(&self) -> super::vals::Scdbk {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Scdbk::from_bits(val as u8)
    }
    #[doc = "SCD Break Signal"]
    #[inline(always)]
    pub const fn set_scdbk(&mut self, val: super::vals::Scdbk) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Low Limit Break Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn llmtbk(&self) -> super::vals::Llmtbk {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Llmtbk::from_bits(val as u8)
    }
    #[doc = "Low Limit Break Signal"]
    #[inline(always)]
    pub const fn set_llmtbk(&mut self, val: super::vals::Llmtbk) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "Window Limit Break Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmtbk(&self) -> super::vals::Wlmtbk {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Wlmtbk::from_bits(val as u8)
    }
    #[doc = "Window Limit Break Signal"]
    #[inline(always)]
    pub const fn set_wlmtbk(&mut self, val: super::vals::Wlmtbk) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "High Limit Break Signal"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmtbk(&self) -> super::vals::Hlmtbk {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Hlmtbk::from_bits(val as u8)
    }
    #[doc = "High Limit Break Signal"]
    #[inline(always)]
    pub const fn set_hlmtbk(&mut self, val: super::vals::Hlmtbk) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Cprot {
    #[inline(always)]
    fn default() -> Cprot {
        Cprot(0u64 as u32)
    }
}
impl core::fmt::Debug for Cprot {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cprot")
            .field("scdlmt", &self.scdlmt())
            .field("scdcm", &self.scdcm())
            .field("scdop", &self.scdop())
            .field("lmtop", &self.lmtop())
            .field("cadlmt", &self.cadlmt())
            .field("cadbk", &self.cadbk())
            .field("scdbk", &self.scdbk())
            .field("llmtbk", &self.llmtbk())
            .field("wlmtbk", &self.wlmtbk())
            .field("hlmtbk", &self.hlmtbk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cprot {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cprot {
            scdlmt: super::vals::Scdlmt,
            scdcm: super::vals::Scdcm,
            scdop: super::vals::Scdop,
            lmtop: super::vals::Lmtop,
            cadlmt: super::vals::Cadlmt,
            cadbk: super::vals::Cadbk,
            scdbk: super::vals::Scdbk,
            llmtbk: super::vals::Llmtbk,
            wlmtbk: super::vals::Wlmtbk,
            hlmtbk: super::vals::Hlmtbk,
        }
        let proxy = Cprot {
            scdlmt: self.scdlmt(),
            scdcm: self.scdcm(),
            scdop: self.scdop(),
            lmtop: self.lmtop(),
            cadlmt: self.cadlmt(),
            cadbk: self.cadbk(),
            scdbk: self.scdbk(),
            llmtbk: self.llmtbk(),
            wlmtbk: self.wlmtbk(),
            hlmtbk: self.hlmtbk(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Result Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crdata(pub u32);
impl Crdata {
    #[doc = "Result Data"]
    #[must_use]
    #[inline(always)]
    pub const fn rdata(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Result Data"]
    #[inline(always)]
    pub const fn set_rdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for Crdata {
    #[inline(always)]
    fn default() -> Crdata {
        Crdata(0u64 as u32)
    }
}
impl core::fmt::Debug for Crdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crdata")
            .field("rdata", &self.rdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crdata {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crdata {
            rdata: u32,
        }
        let proxy = Crdata {
            rdata: self.rdata(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "FIFO Available Data"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoavil(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "FIFO Available Data"]
    #[inline(always)]
    pub const fn set_fifoavil(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Parallel or Serial Data Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn psrdy(&self) -> super::vals::Psrdy {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Psrdy::from_bits(val as u8)
    }
    #[doc = "Parallel or Serial Data Ready"]
    #[inline(always)]
    pub const fn set_psrdy(&mut self, val: super::vals::Psrdy) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Primary CIC Saturation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pfsat(&self) -> super::vals::Pfsat {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pfsat::from_bits(val as u8)
    }
    #[doc = "Primary CIC Saturation Flag"]
    #[inline(always)]
    pub const fn set_pfsat(&mut self, val: super::vals::Pfsat) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "HPF Saturation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hpfsat(&self) -> super::vals::Hpfsat {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Hpfsat::from_bits(val as u8)
    }
    #[doc = "HPF Saturation Flag"]
    #[inline(always)]
    pub const fn set_hpfsat(&mut self, val: super::vals::Hpfsat) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Shift Saturation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sftsat(&self) -> super::vals::Sftsat {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Sftsat::from_bits(val as u8)
    }
    #[doc = "Shift Saturation Flag"]
    #[inline(always)]
    pub const fn set_sftsat(&mut self, val: super::vals::Sftsat) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Bias Saturation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn biassat(&self) -> super::vals::Biassat {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Biassat::from_bits(val as u8)
    }
    #[doc = "Bias Saturation Flag"]
    #[inline(always)]
    pub const fn set_biassat(&mut self, val: super::vals::Biassat) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Result Data Direct Read Status"]
    #[must_use]
    #[inline(always)]
    pub const fn rdrs(&self) -> super::vals::Rdrs {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Rdrs::from_bits(val as u8)
    }
    #[doc = "Result Data Direct Read Status"]
    #[inline(always)]
    pub const fn set_rdrs(&mut self, val: super::vals::Rdrs) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Start Read Debug Data Sync"]
    #[must_use]
    #[inline(always)]
    pub const fn srds(&self) -> super::vals::Srds {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Srds::from_bits(val as u8)
    }
    #[doc = "Start Read Debug Data Sync"]
    #[inline(always)]
    pub const fn set_srds(&mut self, val: super::vals::Srds) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Debug Data Read Status"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgrs(&self) -> super::vals::Dbgrs {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Dbgrs::from_bits(val as u8)
    }
    #[doc = "Debug Data Read Status"]
    #[inline(always)]
    pub const fn set_dbgrs(&mut self, val: super::vals::Dbgrs) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Number Of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn cnum(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Number Of Conversions"]
    #[inline(always)]
    pub const fn set_cnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Overflow In Number Of Conversions"]
    #[must_use]
    #[inline(always)]
    pub const fn cnum_ov(&self) -> super::vals::CnumOv {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::CnumOv::from_bits(val as u8)
    }
    #[doc = "Overflow In Number Of Conversions"]
    #[inline(always)]
    pub const fn set_cnum_ov(&mut self, val: super::vals::CnumOv) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
            .field("fifoavil", &self.fifoavil())
            .field("psrdy", &self.psrdy())
            .field("pfsat", &self.pfsat())
            .field("hpfsat", &self.hpfsat())
            .field("sftsat", &self.sftsat())
            .field("biassat", &self.biassat())
            .field("rdrs", &self.rdrs())
            .field("srds", &self.srds())
            .field("dbgrs", &self.dbgrs())
            .field("cnum", &self.cnum())
            .field("cnum_ov", &self.cnum_ov())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Csr {
            fifoavil: u8,
            psrdy: super::vals::Psrdy,
            pfsat: super::vals::Pfsat,
            hpfsat: super::vals::Hpfsat,
            sftsat: super::vals::Sftsat,
            biassat: super::vals::Biassat,
            rdrs: super::vals::Rdrs,
            srds: super::vals::Srds,
            dbgrs: super::vals::Dbgrs,
            cnum: u8,
            cnum_ov: super::vals::CnumOv,
        }
        let proxy = Csr {
            fifoavil: self.fifoavil(),
            psrdy: self.psrdy(),
            pfsat: self.pfsat(),
            hpfsat: self.hpfsat(),
            sftsat: self.sftsat(),
            biassat: self.biassat(),
            rdrs: self.rdrs(),
            srds: self.srds(),
            dbgrs: self.dbgrs(),
            cnum: self.cnum(),
            cnum_ov: self.cnum_ov(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eie(pub u32);
impl Eie {
    #[doc = "Short Circuit Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn scdie0(&self) -> super::vals::Scdie0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Scdie0::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_scdie0(&mut self, val: super::vals::Scdie0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn scdie1(&self) -> super::vals::Scdie1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Scdie1::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_scdie1(&mut self, val: super::vals::Scdie1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn scdie2(&self) -> super::vals::Scdie2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Scdie2::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_scdie2(&mut self, val: super::vals::Scdie2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn scdie3(&self) -> super::vals::Scdie3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Scdie3::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_scdie3(&mut self, val: super::vals::Scdie3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Window Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmtie0(&self) -> super::vals::Wlmtie0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Wlmtie0::from_bits(val as u8)
    }
    #[doc = "Window Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wlmtie0(&mut self, val: super::vals::Wlmtie0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Window Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmtie1(&self) -> super::vals::Wlmtie1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Wlmtie1::from_bits(val as u8)
    }
    #[doc = "Window Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wlmtie1(&mut self, val: super::vals::Wlmtie1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Window Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmtie2(&self) -> super::vals::Wlmtie2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wlmtie2::from_bits(val as u8)
    }
    #[doc = "Window Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wlmtie2(&mut self, val: super::vals::Wlmtie2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Window Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmtie3(&self) -> super::vals::Wlmtie3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Wlmtie3::from_bits(val as u8)
    }
    #[doc = "Window Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_wlmtie3(&mut self, val: super::vals::Wlmtie3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Low Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn llmtie0(&self) -> super::vals::Llmtie0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Llmtie0::from_bits(val as u8)
    }
    #[doc = "Low Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_llmtie0(&mut self, val: super::vals::Llmtie0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Low Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn llmtie1(&self) -> super::vals::Llmtie1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Llmtie1::from_bits(val as u8)
    }
    #[doc = "Low Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_llmtie1(&mut self, val: super::vals::Llmtie1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Low Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn llmtie2(&self) -> super::vals::Llmtie2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Llmtie2::from_bits(val as u8)
    }
    #[doc = "Low Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_llmtie2(&mut self, val: super::vals::Llmtie2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Low Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn llmtie3(&self) -> super::vals::Llmtie3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Llmtie3::from_bits(val as u8)
    }
    #[doc = "Low Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_llmtie3(&mut self, val: super::vals::Llmtie3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmtie0(&self) -> super::vals::Hlmtie0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Hlmtie0::from_bits(val as u8)
    }
    #[doc = "High Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hlmtie0(&mut self, val: super::vals::Hlmtie0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "High Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmtie1(&self) -> super::vals::Hlmtie1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Hlmtie1::from_bits(val as u8)
    }
    #[doc = "High Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hlmtie1(&mut self, val: super::vals::Hlmtie1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "High Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmtie2(&self) -> super::vals::Hlmtie2 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Hlmtie2::from_bits(val as u8)
    }
    #[doc = "High Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hlmtie2(&mut self, val: super::vals::Hlmtie2) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "High Limit Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmtie3(&self) -> super::vals::Hlmtie3 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Hlmtie3::from_bits(val as u8)
    }
    #[doc = "High Limit Interrupt Enable"]
    #[inline(always)]
    pub const fn set_hlmtie3(&mut self, val: super::vals::Hlmtie3) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Eie {
    #[inline(always)]
    fn default() -> Eie {
        Eie(0u64 as u32)
    }
}
impl core::fmt::Debug for Eie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eie")
            .field("scdie0", &self.scdie0())
            .field("scdie1", &self.scdie1())
            .field("scdie2", &self.scdie2())
            .field("scdie3", &self.scdie3())
            .field("wlmtie0", &self.wlmtie0())
            .field("wlmtie1", &self.wlmtie1())
            .field("wlmtie2", &self.wlmtie2())
            .field("wlmtie3", &self.wlmtie3())
            .field("llmtie0", &self.llmtie0())
            .field("llmtie1", &self.llmtie1())
            .field("llmtie2", &self.llmtie2())
            .field("llmtie3", &self.llmtie3())
            .field("hlmtie0", &self.hlmtie0())
            .field("hlmtie1", &self.hlmtie1())
            .field("hlmtie2", &self.hlmtie2())
            .field("hlmtie3", &self.hlmtie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eie {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eie {
            scdie0: super::vals::Scdie0,
            scdie1: super::vals::Scdie1,
            scdie2: super::vals::Scdie2,
            scdie3: super::vals::Scdie3,
            wlmtie0: super::vals::Wlmtie0,
            wlmtie1: super::vals::Wlmtie1,
            wlmtie2: super::vals::Wlmtie2,
            wlmtie3: super::vals::Wlmtie3,
            llmtie0: super::vals::Llmtie0,
            llmtie1: super::vals::Llmtie1,
            llmtie2: super::vals::Llmtie2,
            llmtie3: super::vals::Llmtie3,
            hlmtie0: super::vals::Hlmtie0,
            hlmtie1: super::vals::Hlmtie1,
            hlmtie2: super::vals::Hlmtie2,
            hlmtie3: super::vals::Hlmtie3,
        }
        let proxy = Eie {
            scdie0: self.scdie0(),
            scdie1: self.scdie1(),
            scdie2: self.scdie2(),
            scdie3: self.scdie3(),
            wlmtie0: self.wlmtie0(),
            wlmtie1: self.wlmtie1(),
            wlmtie2: self.wlmtie2(),
            wlmtie3: self.wlmtie3(),
            llmtie0: self.llmtie0(),
            llmtie1: self.llmtie1(),
            llmtie2: self.llmtie2(),
            llmtie3: self.llmtie3(),
            hlmtie0: self.hlmtie0(),
            hlmtie1: self.hlmtie1(),
            hlmtie2: self.hlmtie2(),
            hlmtie3: self.hlmtie3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Error Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eis(pub u32);
impl Eis {
    #[doc = "Short Circuit Detected Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn scd0(&self) -> super::vals::Scd0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Scd0::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Flag"]
    #[inline(always)]
    pub const fn set_scd0(&mut self, val: super::vals::Scd0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Short Circuit Detected Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn scd1(&self) -> super::vals::Scd1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Scd1::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Flag"]
    #[inline(always)]
    pub const fn set_scd1(&mut self, val: super::vals::Scd1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Short Circuit Detected Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn scd2(&self) -> super::vals::Scd2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Scd2::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Flag"]
    #[inline(always)]
    pub const fn set_scd2(&mut self, val: super::vals::Scd2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Short Circuit Detected Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn scd3(&self) -> super::vals::Scd3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Scd3::from_bits(val as u8)
    }
    #[doc = "Short Circuit Detected Flag"]
    #[inline(always)]
    pub const fn set_scd3(&mut self, val: super::vals::Scd3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Window Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmt0(&self) -> super::vals::Wlmt0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Wlmt0::from_bits(val as u8)
    }
    #[doc = "Window Limit Flag"]
    #[inline(always)]
    pub const fn set_wlmt0(&mut self, val: super::vals::Wlmt0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Window Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmt1(&self) -> super::vals::Wlmt1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Wlmt1::from_bits(val as u8)
    }
    #[doc = "Window Limit Flag"]
    #[inline(always)]
    pub const fn set_wlmt1(&mut self, val: super::vals::Wlmt1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Window Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmt2(&self) -> super::vals::Wlmt2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Wlmt2::from_bits(val as u8)
    }
    #[doc = "Window Limit Flag"]
    #[inline(always)]
    pub const fn set_wlmt2(&mut self, val: super::vals::Wlmt2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Window Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wlmt3(&self) -> super::vals::Wlmt3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Wlmt3::from_bits(val as u8)
    }
    #[doc = "Window Limit Flag"]
    #[inline(always)]
    pub const fn set_wlmt3(&mut self, val: super::vals::Wlmt3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Low Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn llmt0(&self) -> super::vals::Llmt0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Llmt0::from_bits(val as u8)
    }
    #[doc = "Low Limit Flag"]
    #[inline(always)]
    pub const fn set_llmt0(&mut self, val: super::vals::Llmt0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Low Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn llmt1(&self) -> super::vals::Llmt1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Llmt1::from_bits(val as u8)
    }
    #[doc = "Low Limit Flag"]
    #[inline(always)]
    pub const fn set_llmt1(&mut self, val: super::vals::Llmt1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Low Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn llmt2(&self) -> super::vals::Llmt2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Llmt2::from_bits(val as u8)
    }
    #[doc = "Low Limit Flag"]
    #[inline(always)]
    pub const fn set_llmt2(&mut self, val: super::vals::Llmt2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Low Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn llmt3(&self) -> super::vals::Llmt3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Llmt3::from_bits(val as u8)
    }
    #[doc = "Low Limit Flag"]
    #[inline(always)]
    pub const fn set_llmt3(&mut self, val: super::vals::Llmt3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "High Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmt0(&self) -> super::vals::Hlmt0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Hlmt0::from_bits(val as u8)
    }
    #[doc = "High Limit Flag"]
    #[inline(always)]
    pub const fn set_hlmt0(&mut self, val: super::vals::Hlmt0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "High Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmt1(&self) -> super::vals::Hlmt1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Hlmt1::from_bits(val as u8)
    }
    #[doc = "High Limit Flag"]
    #[inline(always)]
    pub const fn set_hlmt1(&mut self, val: super::vals::Hlmt1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "High Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmt2(&self) -> super::vals::Hlmt2 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Hlmt2::from_bits(val as u8)
    }
    #[doc = "High Limit Flag"]
    #[inline(always)]
    pub const fn set_hlmt2(&mut self, val: super::vals::Hlmt2) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "High Limit Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hlmt3(&self) -> super::vals::Hlmt3 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Hlmt3::from_bits(val as u8)
    }
    #[doc = "High Limit Flag"]
    #[inline(always)]
    pub const fn set_hlmt3(&mut self, val: super::vals::Hlmt3) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Eis {
    #[inline(always)]
    fn default() -> Eis {
        Eis(0u64 as u32)
    }
}
impl core::fmt::Debug for Eis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Eis")
            .field("scd0", &self.scd0())
            .field("scd1", &self.scd1())
            .field("scd2", &self.scd2())
            .field("scd3", &self.scd3())
            .field("wlmt0", &self.wlmt0())
            .field("wlmt1", &self.wlmt1())
            .field("wlmt2", &self.wlmt2())
            .field("wlmt3", &self.wlmt3())
            .field("llmt0", &self.llmt0())
            .field("llmt1", &self.llmt1())
            .field("llmt2", &self.llmt2())
            .field("llmt3", &self.llmt3())
            .field("hlmt0", &self.hlmt0())
            .field("hlmt1", &self.hlmt1())
            .field("hlmt2", &self.hlmt2())
            .field("hlmt3", &self.hlmt3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Eis {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Eis {
            scd0: super::vals::Scd0,
            scd1: super::vals::Scd1,
            scd2: super::vals::Scd2,
            scd3: super::vals::Scd3,
            wlmt0: super::vals::Wlmt0,
            wlmt1: super::vals::Wlmt1,
            wlmt2: super::vals::Wlmt2,
            wlmt3: super::vals::Wlmt3,
            llmt0: super::vals::Llmt0,
            llmt1: super::vals::Llmt1,
            llmt2: super::vals::Llmt2,
            llmt3: super::vals::Llmt3,
            hlmt0: super::vals::Hlmt0,
            hlmt1: super::vals::Hlmt1,
            hlmt2: super::vals::Hlmt2,
            hlmt3: super::vals::Hlmt3,
        }
        let proxy = Eis {
            scd0: self.scd0(),
            scd1: self.scd1(),
            scd2: self.scd2(),
            scd3: self.scd3(),
            wlmt0: self.wlmt0(),
            wlmt1: self.wlmt1(),
            wlmt2: self.wlmt2(),
            wlmt3: self.wlmt3(),
            llmt0: self.llmt0(),
            llmt1: self.llmt1(),
            llmt2: self.llmt2(),
            llmt3: self.llmt3(),
            hlmt0: self.hlmt0(),
            hlmt1: self.hlmt1(),
            hlmt2: self.hlmt2(),
            hlmt3: self.hlmt3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO And CAD Error Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoie(pub u32);
impl Fifoie {
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn funfie0(&self) -> super::vals::Funfie0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Funfie0::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_funfie0(&mut self, val: super::vals::Funfie0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn funfie1(&self) -> super::vals::Funfie1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Funfie1::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_funfie1(&mut self, val: super::vals::Funfie1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn funfie2(&self) -> super::vals::Funfie2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Funfie2::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_funfie2(&mut self, val: super::vals::Funfie2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn funfie3(&self) -> super::vals::Funfie3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Funfie3::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_funfie3(&mut self, val: super::vals::Funfie3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fovfie0(&self) -> super::vals::Fovfie0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fovfie0::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fovfie0(&mut self, val: super::vals::Fovfie0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fovfie1(&self) -> super::vals::Fovfie1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Fovfie1::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fovfie1(&mut self, val: super::vals::Fovfie1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fovfie2(&self) -> super::vals::Fovfie2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Fovfie2::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fovfie2(&mut self, val: super::vals::Fovfie2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fovfie3(&self) -> super::vals::Fovfie3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Fovfie3::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_fovfie3(&mut self, val: super::vals::Fovfie3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Clock Absence Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cadie0(&self) -> super::vals::Cadie0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Cadie0::from_bits(val as u8)
    }
    #[doc = "Clock Absence Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cadie0(&mut self, val: super::vals::Cadie0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Clock Absence Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cadie1(&self) -> super::vals::Cadie1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cadie1::from_bits(val as u8)
    }
    #[doc = "Clock Absence Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cadie1(&mut self, val: super::vals::Cadie1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Clock Absence Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cadie2(&self) -> super::vals::Cadie2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cadie2::from_bits(val as u8)
    }
    #[doc = "Clock Absence Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cadie2(&mut self, val: super::vals::Cadie2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Clock Absence Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cadie3(&self) -> super::vals::Cadie3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cadie3::from_bits(val as u8)
    }
    #[doc = "Clock Absence Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cadie3(&mut self, val: super::vals::Cadie3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Saturation Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn satie0(&self) -> super::vals::Satie0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Satie0::from_bits(val as u8)
    }
    #[doc = "Saturation Interrupt Enable"]
    #[inline(always)]
    pub const fn set_satie0(&mut self, val: super::vals::Satie0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Saturation Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn satie1(&self) -> super::vals::Satie1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Satie1::from_bits(val as u8)
    }
    #[doc = "Saturation Interrupt Enable"]
    #[inline(always)]
    pub const fn set_satie1(&mut self, val: super::vals::Satie1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Saturation Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn satie2(&self) -> super::vals::Satie2 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Satie2::from_bits(val as u8)
    }
    #[doc = "Saturation Interrupt Enable"]
    #[inline(always)]
    pub const fn set_satie2(&mut self, val: super::vals::Satie2) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Saturation Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn satie3(&self) -> super::vals::Satie3 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Satie3::from_bits(val as u8)
    }
    #[doc = "Saturation Interrupt Enable"]
    #[inline(always)]
    pub const fn set_satie3(&mut self, val: super::vals::Satie3) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Fifoie {
    #[inline(always)]
    fn default() -> Fifoie {
        Fifoie(0u64 as u32)
    }
}
impl core::fmt::Debug for Fifoie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoie")
            .field("funfie0", &self.funfie0())
            .field("funfie1", &self.funfie1())
            .field("funfie2", &self.funfie2())
            .field("funfie3", &self.funfie3())
            .field("fovfie0", &self.fovfie0())
            .field("fovfie1", &self.fovfie1())
            .field("fovfie2", &self.fovfie2())
            .field("fovfie3", &self.fovfie3())
            .field("cadie0", &self.cadie0())
            .field("cadie1", &self.cadie1())
            .field("cadie2", &self.cadie2())
            .field("cadie3", &self.cadie3())
            .field("satie0", &self.satie0())
            .field("satie1", &self.satie1())
            .field("satie2", &self.satie2())
            .field("satie3", &self.satie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoie {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fifoie {
            funfie0: super::vals::Funfie0,
            funfie1: super::vals::Funfie1,
            funfie2: super::vals::Funfie2,
            funfie3: super::vals::Funfie3,
            fovfie0: super::vals::Fovfie0,
            fovfie1: super::vals::Fovfie1,
            fovfie2: super::vals::Fovfie2,
            fovfie3: super::vals::Fovfie3,
            cadie0: super::vals::Cadie0,
            cadie1: super::vals::Cadie1,
            cadie2: super::vals::Cadie2,
            cadie3: super::vals::Cadie3,
            satie0: super::vals::Satie0,
            satie1: super::vals::Satie1,
            satie2: super::vals::Satie2,
            satie3: super::vals::Satie3,
        }
        let proxy = Fifoie {
            funfie0: self.funfie0(),
            funfie1: self.funfie1(),
            funfie2: self.funfie2(),
            funfie3: self.funfie3(),
            fovfie0: self.fovfie0(),
            fovfie1: self.fovfie1(),
            fovfie2: self.fovfie2(),
            fovfie3: self.fovfie3(),
            cadie0: self.cadie0(),
            cadie1: self.cadie1(),
            cadie2: self.cadie2(),
            cadie3: self.cadie3(),
            satie0: self.satie0(),
            satie1: self.satie1(),
            satie2: self.satie2(),
            satie3: self.satie3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO And CAD Error Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifois(pub u32);
impl Fifois {
    #[doc = "FIFO Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn funf0(&self) -> super::vals::Funf0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Funf0::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn set_funf0(&mut self, val: super::vals::Funf0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn funf1(&self) -> super::vals::Funf1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Funf1::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn set_funf1(&mut self, val: super::vals::Funf1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn funf2(&self) -> super::vals::Funf2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Funf2::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn set_funf2(&mut self, val: super::vals::Funf2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO Underflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn funf3(&self) -> super::vals::Funf3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Funf3::from_bits(val as u8)
    }
    #[doc = "FIFO Underflow Flag"]
    #[inline(always)]
    pub const fn set_funf3(&mut self, val: super::vals::Funf3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fovf0(&self) -> super::vals::Fovf0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Fovf0::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_fovf0(&mut self, val: super::vals::Fovf0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fovf1(&self) -> super::vals::Fovf1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Fovf1::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_fovf1(&mut self, val: super::vals::Fovf1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fovf2(&self) -> super::vals::Fovf2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Fovf2::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_fovf2(&mut self, val: super::vals::Fovf2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FIFO Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn fovf3(&self) -> super::vals::Fovf3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Fovf3::from_bits(val as u8)
    }
    #[doc = "FIFO Overflow Flag"]
    #[inline(always)]
    pub const fn set_fovf3(&mut self, val: super::vals::Fovf3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Clock Absence Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cad0(&self) -> super::vals::Cad0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Cad0::from_bits(val as u8)
    }
    #[doc = "Clock Absence Flag"]
    #[inline(always)]
    pub const fn set_cad0(&mut self, val: super::vals::Cad0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Clock Absence Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cad1(&self) -> super::vals::Cad1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cad1::from_bits(val as u8)
    }
    #[doc = "Clock Absence Flag"]
    #[inline(always)]
    pub const fn set_cad1(&mut self, val: super::vals::Cad1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Clock Absence Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cad2(&self) -> super::vals::Cad2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Cad2::from_bits(val as u8)
    }
    #[doc = "Clock Absence Flag"]
    #[inline(always)]
    pub const fn set_cad2(&mut self, val: super::vals::Cad2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Clock Absence Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cad3(&self) -> super::vals::Cad3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Cad3::from_bits(val as u8)
    }
    #[doc = "Clock Absence Flag"]
    #[inline(always)]
    pub const fn set_cad3(&mut self, val: super::vals::Cad3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Saturation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sat0(&self) -> super::vals::Sat0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Sat0::from_bits(val as u8)
    }
    #[doc = "Saturation Flag"]
    #[inline(always)]
    pub const fn set_sat0(&mut self, val: super::vals::Sat0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Saturation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sat1(&self) -> super::vals::Sat1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Sat1::from_bits(val as u8)
    }
    #[doc = "Saturation Flag"]
    #[inline(always)]
    pub const fn set_sat1(&mut self, val: super::vals::Sat1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Saturation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sat2(&self) -> super::vals::Sat2 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Sat2::from_bits(val as u8)
    }
    #[doc = "Saturation Flag"]
    #[inline(always)]
    pub const fn set_sat2(&mut self, val: super::vals::Sat2) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Saturation Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn sat3(&self) -> super::vals::Sat3 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Sat3::from_bits(val as u8)
    }
    #[doc = "Saturation Flag"]
    #[inline(always)]
    pub const fn set_sat3(&mut self, val: super::vals::Sat3) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for Fifois {
    #[inline(always)]
    fn default() -> Fifois {
        Fifois(0u64 as u32)
    }
}
impl core::fmt::Debug for Fifois {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifois")
            .field("funf0", &self.funf0())
            .field("funf1", &self.funf1())
            .field("funf2", &self.funf2())
            .field("funf3", &self.funf3())
            .field("fovf0", &self.fovf0())
            .field("fovf1", &self.fovf1())
            .field("fovf2", &self.fovf2())
            .field("fovf3", &self.fovf3())
            .field("cad0", &self.cad0())
            .field("cad1", &self.cad1())
            .field("cad2", &self.cad2())
            .field("cad3", &self.cad3())
            .field("sat0", &self.sat0())
            .field("sat1", &self.sat1())
            .field("sat2", &self.sat2())
            .field("sat3", &self.sat3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifois {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fifois {
            funf0: super::vals::Funf0,
            funf1: super::vals::Funf1,
            funf2: super::vals::Funf2,
            funf3: super::vals::Funf3,
            fovf0: super::vals::Fovf0,
            fovf1: super::vals::Fovf1,
            fovf2: super::vals::Fovf2,
            fovf3: super::vals::Fovf3,
            cad0: super::vals::Cad0,
            cad1: super::vals::Cad1,
            cad2: super::vals::Cad2,
            cad3: super::vals::Cad3,
            sat0: super::vals::Sat0,
            sat1: super::vals::Sat1,
            sat2: super::vals::Sat2,
            sat3: super::vals::Sat3,
        }
        let proxy = Fifois {
            funf0: self.funf0(),
            funf1: self.funf1(),
            funf2: self.funf2(),
            funf3: self.funf3(),
            fovf0: self.fovf0(),
            fovf1: self.fovf1(),
            fovf2: self.fovf2(),
            fovf3: self.fovf3(),
            cad0: self.cad0(),
            cad1: self.cad1(),
            cad2: self.cad2(),
            cad3: self.cad3(),
            sat0: self.sat0(),
            sat1: self.sat1(),
            sat2: self.sat2(),
            sat3: self.sat3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Main Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Software Trigger For Channel 0"]
    #[must_use]
    #[inline(always)]
    pub const fn strig0(&self) -> super::vals::Strig0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Strig0::from_bits(val as u8)
    }
    #[doc = "Software Trigger For Channel 0"]
    #[inline(always)]
    pub const fn set_strig0(&mut self, val: super::vals::Strig0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Trigger For Channel 1"]
    #[must_use]
    #[inline(always)]
    pub const fn strig1(&self) -> super::vals::Strig1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Strig1::from_bits(val as u8)
    }
    #[doc = "Software Trigger For Channel 1"]
    #[inline(always)]
    pub const fn set_strig1(&mut self, val: super::vals::Strig1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Software Trigger For Channel 2"]
    #[must_use]
    #[inline(always)]
    pub const fn strig2(&self) -> super::vals::Strig2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Strig2::from_bits(val as u8)
    }
    #[doc = "Software Trigger For Channel 2"]
    #[inline(always)]
    pub const fn set_strig2(&mut self, val: super::vals::Strig2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Software Trigger For Channel 3"]
    #[must_use]
    #[inline(always)]
    pub const fn strig3(&self) -> super::vals::Strig3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Strig3::from_bits(val as u8)
    }
    #[doc = "Software Trigger For Channel 3"]
    #[inline(always)]
    pub const fn set_strig3(&mut self, val: super::vals::Strig3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Doze Or Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> super::vals::Dozen {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Or Stop Enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: super::vals::Dozen) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> super::vals::Rst {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Rst::from_bits(val as u8)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: super::vals::Rst) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Master Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn men(&self) -> super::vals::Men {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Men::from_bits(val as u8)
    }
    #[doc = "Master Enable"]
    #[inline(always)]
    pub const fn set_men(&mut self, val: super::vals::Men) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Modulator Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn mclkdiv(&self) -> super::vals::Mclkdiv {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Mclkdiv::from_bits(val as u8)
    }
    #[doc = "Modulator Clock Divider"]
    #[inline(always)]
    pub const fn set_mclkdiv(&mut self, val: super::vals::Mclkdiv) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Prescale Before Clock Divider"]
    #[must_use]
    #[inline(always)]
    pub const fn prescale(&self) -> super::vals::Prescale {
        let val = (self.0 >> 25usize) & 0x03;
        super::vals::Prescale::from_bits(val as u8)
    }
    #[doc = "Prescale Before Clock Divider"]
    #[inline(always)]
    pub const fn set_prescale(&mut self, val: super::vals::Prescale) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
    }
    #[doc = "Disable Modulator Clock 0 Output"]
    #[must_use]
    #[inline(always)]
    pub const fn mclk0dis(&self) -> super::vals::Mclk0dis {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Mclk0dis::from_bits(val as u8)
    }
    #[doc = "Disable Modulator Clock 0 Output"]
    #[inline(always)]
    pub const fn set_mclk0dis(&mut self, val: super::vals::Mclk0dis) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Disable Modulator Clock 1 Output"]
    #[must_use]
    #[inline(always)]
    pub const fn mclk1dis(&self) -> super::vals::Mclk1dis {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Mclk1dis::from_bits(val as u8)
    }
    #[doc = "Disable Modulator Clock 1 Output"]
    #[inline(always)]
    pub const fn set_mclk1dis(&mut self, val: super::vals::Mclk1dis) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "Disable Modulator Clock 2 Output"]
    #[must_use]
    #[inline(always)]
    pub const fn mclk2dis(&self) -> super::vals::Mclk2dis {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Mclk2dis::from_bits(val as u8)
    }
    #[doc = "Disable Modulator Clock 2 Output"]
    #[inline(always)]
    pub const fn set_mclk2dis(&mut self, val: super::vals::Mclk2dis) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("strig0", &self.strig0())
            .field("strig1", &self.strig1())
            .field("strig2", &self.strig2())
            .field("strig3", &self.strig3())
            .field("dozen", &self.dozen())
            .field("rst", &self.rst())
            .field("men", &self.men())
            .field("mclkdiv", &self.mclkdiv())
            .field("prescale", &self.prescale())
            .field("mclk0dis", &self.mclk0dis())
            .field("mclk1dis", &self.mclk1dis())
            .field("mclk2dis", &self.mclk2dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mcr {
            strig0: super::vals::Strig0,
            strig1: super::vals::Strig1,
            strig2: super::vals::Strig2,
            strig3: super::vals::Strig3,
            dozen: super::vals::Dozen,
            rst: super::vals::Rst,
            men: super::vals::Men,
            mclkdiv: super::vals::Mclkdiv,
            prescale: super::vals::Prescale,
            mclk0dis: super::vals::Mclk0dis,
            mclk1dis: super::vals::Mclk1dis,
            mclk2dis: super::vals::Mclk2dis,
        }
        let proxy = Mcr {
            strig0: self.strig0(),
            strig1: self.strig1(),
            strig2: self.strig2(),
            strig3: self.strig3(),
            dozen: self.dozen(),
            rst: self.rst(),
            men: self.men(),
            mclkdiv: self.mclkdiv(),
            prescale: self.prescale(),
            mclk0dis: self.mclk0dis(),
            mclk1dis: self.mclk1dis(),
            mclk2dis: self.mclk2dis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Normal Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nie(pub u32);
impl Nie {
    #[doc = "Conversion Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cocie0(&self) -> super::vals::Cocie0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cocie0::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cocie0(&mut self, val: super::vals::Cocie0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cocie1(&self) -> super::vals::Cocie1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cocie1::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cocie1(&mut self, val: super::vals::Cocie1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cocie2(&self) -> super::vals::Cocie2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cocie2::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cocie2(&mut self, val: super::vals::Cocie2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cocie3(&self) -> super::vals::Cocie3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cocie3::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Interrupt Enable"]
    #[inline(always)]
    pub const fn set_cocie3(&mut self, val: super::vals::Cocie3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chfie0(&self) -> super::vals::Chfie0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Chfie0::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn set_chfie0(&mut self, val: super::vals::Chfie0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chfie1(&self) -> super::vals::Chfie1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Chfie1::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn set_chfie1(&mut self, val: super::vals::Chfie1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chfie2(&self) -> super::vals::Chfie2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Chfie2::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn set_chfie2(&mut self, val: super::vals::Chfie2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chfie3(&self) -> super::vals::Chfie3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Chfie3::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Interrupt Enable"]
    #[inline(always)]
    pub const fn set_chfie3(&mut self, val: super::vals::Chfie3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn zcdie0(&self) -> super::vals::Zcdie0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Zcdie0::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_zcdie0(&mut self, val: super::vals::Zcdie0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn zcdie1(&self) -> super::vals::Zcdie1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Zcdie1::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_zcdie1(&mut self, val: super::vals::Zcdie1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn zcdie2(&self) -> super::vals::Zcdie2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Zcdie2::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_zcdie2(&mut self, val: super::vals::Zcdie2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn zcdie3(&self) -> super::vals::Zcdie3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Zcdie3::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Interrupt Enable"]
    #[inline(always)]
    pub const fn set_zcdie3(&mut self, val: super::vals::Zcdie3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Nie {
    #[inline(always)]
    fn default() -> Nie {
        Nie(0u64 as u32)
    }
}
impl core::fmt::Debug for Nie {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nie")
            .field("cocie0", &self.cocie0())
            .field("cocie1", &self.cocie1())
            .field("cocie2", &self.cocie2())
            .field("cocie3", &self.cocie3())
            .field("chfie0", &self.chfie0())
            .field("chfie1", &self.chfie1())
            .field("chfie2", &self.chfie2())
            .field("chfie3", &self.chfie3())
            .field("zcdie0", &self.zcdie0())
            .field("zcdie1", &self.zcdie1())
            .field("zcdie2", &self.zcdie2())
            .field("zcdie3", &self.zcdie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nie {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Nie {
            cocie0: super::vals::Cocie0,
            cocie1: super::vals::Cocie1,
            cocie2: super::vals::Cocie2,
            cocie3: super::vals::Cocie3,
            chfie0: super::vals::Chfie0,
            chfie1: super::vals::Chfie1,
            chfie2: super::vals::Chfie2,
            chfie3: super::vals::Chfie3,
            zcdie0: super::vals::Zcdie0,
            zcdie1: super::vals::Zcdie1,
            zcdie2: super::vals::Zcdie2,
            zcdie3: super::vals::Zcdie3,
        }
        let proxy = Nie {
            cocie0: self.cocie0(),
            cocie1: self.cocie1(),
            cocie2: self.cocie2(),
            cocie3: self.cocie3(),
            chfie0: self.chfie0(),
            chfie1: self.chfie1(),
            chfie2: self.chfie2(),
            chfie3: self.chfie3(),
            zcdie0: self.zcdie0(),
            zcdie1: self.zcdie1(),
            zcdie2: self.zcdie2(),
            zcdie3: self.zcdie3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Normal Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nis(pub u32);
impl Nis {
    #[doc = "Conversion Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn coc0(&self) -> super::vals::Coc0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Coc0::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Flag"]
    #[inline(always)]
    pub const fn set_coc0(&mut self, val: super::vals::Coc0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Conversion Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn coc1(&self) -> super::vals::Coc1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Coc1::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Flag"]
    #[inline(always)]
    pub const fn set_coc1(&mut self, val: super::vals::Coc1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Conversion Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn coc2(&self) -> super::vals::Coc2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Coc2::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Flag"]
    #[inline(always)]
    pub const fn set_coc2(&mut self, val: super::vals::Coc2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Conversion Complete Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn coc3(&self) -> super::vals::Coc3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Coc3::from_bits(val as u8)
    }
    #[doc = "Conversion Complete Flag"]
    #[inline(always)]
    pub const fn set_coc3(&mut self, val: super::vals::Coc3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Data Output Ready Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn chf0(&self) -> super::vals::Chf0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Chf0::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Flag"]
    #[inline(always)]
    pub const fn set_chf0(&mut self, val: super::vals::Chf0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Data Output Ready Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn chf1(&self) -> super::vals::Chf1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Chf1::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Flag"]
    #[inline(always)]
    pub const fn set_chf1(&mut self, val: super::vals::Chf1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Data Output Ready Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn chf2(&self) -> super::vals::Chf2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Chf2::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Flag"]
    #[inline(always)]
    pub const fn set_chf2(&mut self, val: super::vals::Chf2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Data Output Ready Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn chf3(&self) -> super::vals::Chf3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Chf3::from_bits(val as u8)
    }
    #[doc = "Data Output Ready Flag"]
    #[inline(always)]
    pub const fn set_chf3(&mut self, val: super::vals::Chf3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Zero Cross Detected Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn zcd0(&self) -> super::vals::Zcd0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Zcd0::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Flag"]
    #[inline(always)]
    pub const fn set_zcd0(&mut self, val: super::vals::Zcd0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Zero Cross Detected Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn zcd1(&self) -> super::vals::Zcd1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Zcd1::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Flag"]
    #[inline(always)]
    pub const fn set_zcd1(&mut self, val: super::vals::Zcd1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Zero Cross Detected Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn zcd2(&self) -> super::vals::Zcd2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Zcd2::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Flag"]
    #[inline(always)]
    pub const fn set_zcd2(&mut self, val: super::vals::Zcd2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Zero Cross Detected Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn zcd3(&self) -> super::vals::Zcd3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Zcd3::from_bits(val as u8)
    }
    #[doc = "Zero Cross Detected Flag"]
    #[inline(always)]
    pub const fn set_zcd3(&mut self, val: super::vals::Zcd3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for Nis {
    #[inline(always)]
    fn default() -> Nis {
        Nis(0u64 as u32)
    }
}
impl core::fmt::Debug for Nis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nis")
            .field("coc0", &self.coc0())
            .field("coc1", &self.coc1())
            .field("coc2", &self.coc2())
            .field("coc3", &self.coc3())
            .field("chf0", &self.chf0())
            .field("chf1", &self.chf1())
            .field("chf2", &self.chf2())
            .field("chf3", &self.chf3())
            .field("zcd0", &self.zcd0())
            .field("zcd1", &self.zcd1())
            .field("zcd2", &self.zcd2())
            .field("zcd3", &self.zcd3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nis {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Nis {
            coc0: super::vals::Coc0,
            coc1: super::vals::Coc1,
            coc2: super::vals::Coc2,
            coc3: super::vals::Coc3,
            chf0: super::vals::Chf0,
            chf1: super::vals::Chf1,
            chf2: super::vals::Chf2,
            chf3: super::vals::Chf3,
            zcd0: super::vals::Zcd0,
            zcd1: super::vals::Zcd1,
            zcd2: super::vals::Zcd2,
            zcd3: super::vals::Zcd3,
        }
        let proxy = Nis {
            coc0: self.coc0(),
            coc1: self.coc1(),
            coc2: self.coc2(),
            coc3: self.coc3(),
            chf0: self.chf0(),
            chf1: self.chf1(),
            chf2: self.chf2(),
            chf3: self.chf3(),
            zcd0: self.zcd0(),
            zcd1: self.zcd1(),
            zcd2: self.zcd2(),
            zcd3: self.zcd3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Parameters"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parameter(pub u32);
impl Parameter {
    #[doc = "FIFO Depth"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_depth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "FIFO Depth"]
    #[inline(always)]
    pub const fn set_fifo_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Filter Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn flt_num(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Filter Channel Number"]
    #[inline(always)]
    pub const fn set_flt_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "PF Order Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pf_ord_sel(&self) -> super::vals::PfOrdSel {
        let val = (self.0 >> 19usize) & 0x03;
        super::vals::PfOrdSel::from_bits(val as u8)
    }
    #[doc = "PF Order Select"]
    #[inline(always)]
    pub const fn set_pf_ord_sel(&mut self, val: super::vals::PfOrdSel) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val.to_bits() as u32) & 0x03) << 19usize);
    }
}
impl Default for Parameter {
    #[inline(always)]
    fn default() -> Parameter {
        Parameter(7341072u64 as u32)
    }
}
impl core::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Parameter")
            .field("fifo_depth", &self.fifo_depth())
            .field("flt_num", &self.flt_num())
            .field("pf_ord_sel", &self.pf_ord_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Parameter {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Parameter {
            fifo_depth: u8,
            flt_num: u8,
            pf_ord_sel: super::vals::PfOrdSel,
        }
        let proxy = Parameter {
            fifo_depth: self.fifo_depth(),
            flt_num: self.flt_num(),
            pf_ord_sel: self.pf_ord_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Conversion In Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn cip0(&self) -> super::vals::Cip0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Cip0::from_bits(val as u8)
    }
    #[doc = "Conversion In Progress"]
    #[inline(always)]
    pub const fn set_cip0(&mut self, val: super::vals::Cip0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Conversion In Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn cip1(&self) -> super::vals::Cip1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cip1::from_bits(val as u8)
    }
    #[doc = "Conversion In Progress"]
    #[inline(always)]
    pub const fn set_cip1(&mut self, val: super::vals::Cip1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Conversion In Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn cip2(&self) -> super::vals::Cip2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Cip2::from_bits(val as u8)
    }
    #[doc = "Conversion In Progress"]
    #[inline(always)]
    pub const fn set_cip2(&mut self, val: super::vals::Cip2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Conversion In Progress"]
    #[must_use]
    #[inline(always)]
    pub const fn cip3(&self) -> super::vals::Cip3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Cip3::from_bits(val as u8)
    }
    #[doc = "Conversion In Progress"]
    #[inline(always)]
    pub const fn set_cip3(&mut self, val: super::vals::Cip3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Ready For Conversion"]
    #[must_use]
    #[inline(always)]
    pub const fn chrdy0(&self) -> super::vals::Chrdy0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Chrdy0::from_bits(val as u8)
    }
    #[doc = "Channel Ready For Conversion"]
    #[inline(always)]
    pub const fn set_chrdy0(&mut self, val: super::vals::Chrdy0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Channel Ready For Conversion"]
    #[must_use]
    #[inline(always)]
    pub const fn chrdy1(&self) -> super::vals::Chrdy1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Chrdy1::from_bits(val as u8)
    }
    #[doc = "Channel Ready For Conversion"]
    #[inline(always)]
    pub const fn set_chrdy1(&mut self, val: super::vals::Chrdy1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Channel Ready For Conversion"]
    #[must_use]
    #[inline(always)]
    pub const fn chrdy2(&self) -> super::vals::Chrdy2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Chrdy2::from_bits(val as u8)
    }
    #[doc = "Channel Ready For Conversion"]
    #[inline(always)]
    pub const fn set_chrdy2(&mut self, val: super::vals::Chrdy2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Channel Ready For Conversion"]
    #[must_use]
    #[inline(always)]
    pub const fn chrdy3(&self) -> super::vals::Chrdy3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Chrdy3::from_bits(val as u8)
    }
    #[doc = "Channel Ready For Conversion"]
    #[inline(always)]
    pub const fn set_chrdy3(&mut self, val: super::vals::Chrdy3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FIFO Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoempty0(&self) -> super::vals::Fifoempty0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Fifoempty0::from_bits(val as u8)
    }
    #[doc = "FIFO Empty"]
    #[inline(always)]
    pub const fn set_fifoempty0(&mut self, val: super::vals::Fifoempty0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "FIFO Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoempty1(&self) -> super::vals::Fifoempty1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Fifoempty1::from_bits(val as u8)
    }
    #[doc = "FIFO Empty"]
    #[inline(always)]
    pub const fn set_fifoempty1(&mut self, val: super::vals::Fifoempty1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "FIFO Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoempty2(&self) -> super::vals::Fifoempty2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Fifoempty2::from_bits(val as u8)
    }
    #[doc = "FIFO Empty"]
    #[inline(always)]
    pub const fn set_fifoempty2(&mut self, val: super::vals::Fifoempty2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "FIFO Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn fifoempty3(&self) -> super::vals::Fifoempty3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Fifoempty3::from_bits(val as u8)
    }
    #[doc = "FIFO Empty"]
    #[inline(always)]
    pub const fn set_fifoempty3(&mut self, val: super::vals::Fifoempty3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Modulator Clock 0 Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn mclkrdy0(&self) -> super::vals::Mclkrdy0 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Mclkrdy0::from_bits(val as u8)
    }
    #[doc = "Modulator Clock 0 Ready"]
    #[inline(always)]
    pub const fn set_mclkrdy0(&mut self, val: super::vals::Mclkrdy0) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "Modulator Clock 1 Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn mclkrdy1(&self) -> super::vals::Mclkrdy1 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Mclkrdy1::from_bits(val as u8)
    }
    #[doc = "Modulator Clock 1 Ready"]
    #[inline(always)]
    pub const fn set_mclkrdy1(&mut self, val: super::vals::Mclkrdy1) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "Modulator Clock 2 Ready"]
    #[must_use]
    #[inline(always)]
    pub const fn mclkrdy2(&self) -> super::vals::Mclkrdy2 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Mclkrdy2::from_bits(val as u8)
    }
    #[doc = "Modulator Clock 2 Ready"]
    #[inline(always)]
    pub const fn set_mclkrdy2(&mut self, val: super::vals::Mclkrdy2) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(983040u64 as u32)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("cip0", &self.cip0())
            .field("cip1", &self.cip1())
            .field("cip2", &self.cip2())
            .field("cip3", &self.cip3())
            .field("chrdy0", &self.chrdy0())
            .field("chrdy1", &self.chrdy1())
            .field("chrdy2", &self.chrdy2())
            .field("chrdy3", &self.chrdy3())
            .field("fifoempty0", &self.fifoempty0())
            .field("fifoempty1", &self.fifoempty1())
            .field("fifoempty2", &self.fifoempty2())
            .field("fifoempty3", &self.fifoempty3())
            .field("mclkrdy0", &self.mclkrdy0())
            .field("mclkrdy1", &self.mclkrdy1())
            .field("mclkrdy2", &self.mclkrdy2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sr {
            cip0: super::vals::Cip0,
            cip1: super::vals::Cip1,
            cip2: super::vals::Cip2,
            cip3: super::vals::Cip3,
            chrdy0: super::vals::Chrdy0,
            chrdy1: super::vals::Chrdy1,
            chrdy2: super::vals::Chrdy2,
            chrdy3: super::vals::Chrdy3,
            fifoempty0: super::vals::Fifoempty0,
            fifoempty1: super::vals::Fifoempty1,
            fifoempty2: super::vals::Fifoempty2,
            fifoempty3: super::vals::Fifoempty3,
            mclkrdy0: super::vals::Mclkrdy0,
            mclkrdy1: super::vals::Mclkrdy1,
            mclkrdy2: super::vals::Mclkrdy2,
        }
        let proxy = Sr {
            cip0: self.cip0(),
            cip1: self.cip1(),
            cip2: self.cip2(),
            cip3: self.cip3(),
            chrdy0: self.chrdy0(),
            chrdy1: self.chrdy1(),
            chrdy2: self.chrdy2(),
            chrdy3: self.chrdy3(),
            fifoempty0: self.fifoempty0(),
            fifoempty1: self.fifoempty1(),
            fifoempty2: self.fifoempty2(),
            fifoempty3: self.fifoempty3(),
            mclkrdy0: self.mclkrdy0(),
            mclkrdy1: self.mclkrdy1(),
            mclkrdy2: self.mclkrdy2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Code"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Specification Code"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> super::vals::Minor {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Minor::from_bits(val as u8)
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: super::vals::Minor) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> super::vals::Major {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Major::from_bits(val as u8)
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: super::vals::Major) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(33554541u64 as u32)
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
            minor: super::vals::Minor,
            major: super::vals::Major,
        }
        let proxy = Verid {
            feature: self.feature(),
            minor: self.minor(),
            major: self.major(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
