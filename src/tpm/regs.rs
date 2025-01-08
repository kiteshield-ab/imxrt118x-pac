#[doc = "Combine Channel"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Combine(pub u32);
impl Combine {
    #[doc = "Combine Channels 0 and 1"]
    #[must_use]
    #[inline(always)]
    pub const fn combine0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Combine Channels 0 and 1"]
    #[inline(always)]
    pub const fn set_combine0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Combine Channel 0 and 1 Swap"]
    #[must_use]
    #[inline(always)]
    pub const fn comswap0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Combine Channel 0 and 1 Swap"]
    #[inline(always)]
    pub const fn set_comswap0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Combine Channels 2 and 3"]
    #[must_use]
    #[inline(always)]
    pub const fn combine1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Combine Channels 2 and 3"]
    #[inline(always)]
    pub const fn set_combine1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Combine Channels 2 and 3 Swap"]
    #[must_use]
    #[inline(always)]
    pub const fn comswap1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Combine Channels 2 and 3 Swap"]
    #[inline(always)]
    pub const fn set_comswap1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Combine {
    #[inline(always)]
    fn default() -> Combine {
        Combine(0u64 as u32)
    }
}
impl core::fmt::Debug for Combine {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Combine")
            .field("combine0", &self.combine0())
            .field("comswap0", &self.comswap0())
            .field("combine1", &self.combine1())
            .field("comswap1", &self.comswap1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Combine {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Combine {
            combine0: bool,
            comswap0: bool,
            combine1: bool,
            comswap1: bool,
        }
        let proxy = Combine {
            combine0: self.combine0(),
            comswap0: self.comswap0(),
            combine1: self.combine1(),
            comswap1: self.comswap1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Conf(pub u32);
impl Conf {
    #[doc = "Doze Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozeen(&self) -> super::vals::Dozeen {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dozeen::from_bits(val as u8)
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn set_dozeen(&mut self, val: super::vals::Dozeen) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Debug Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dbgmode(&self) -> super::vals::Dbgmode {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Dbgmode::from_bits(val as u8)
    }
    #[doc = "Debug Mode"]
    #[inline(always)]
    pub const fn set_dbgmode(&mut self, val: super::vals::Dbgmode) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "GTB Synchronization"]
    #[must_use]
    #[inline(always)]
    pub const fn gtbsync(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "GTB Synchronization"]
    #[inline(always)]
    pub const fn set_gtbsync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "GTB Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gtbeen(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "GTB Enable"]
    #[inline(always)]
    pub const fn set_gtbeen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Counter Start on Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn csot(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Start on Trigger"]
    #[inline(always)]
    pub const fn set_csot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Counter Stop on Overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn csoo(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Stop on Overflow"]
    #[inline(always)]
    pub const fn set_csoo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Counter Reload on Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn crot(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Reload on Trigger"]
    #[inline(always)]
    pub const fn set_crot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Counter Pause on Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn cpot(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Counter Pause on Trigger"]
    #[inline(always)]
    pub const fn set_cpot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Trigger Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn trgpol(&self) -> super::vals::Trgpol {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Trgpol::from_bits(val as u8)
    }
    #[doc = "Trigger Polarity"]
    #[inline(always)]
    pub const fn set_trgpol(&mut self, val: super::vals::Trgpol) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Trigger Source"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsrc(&self) -> super::vals::Trgsrc {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Trgsrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn set_trgsrc(&mut self, val: super::vals::Trgsrc) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsel(&self) -> super::vals::Trgsel {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Trgsel::from_bits(val as u8)
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: super::vals::Trgsel) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Conf {
    #[inline(always)]
    fn default() -> Conf {
        Conf(0u64 as u32)
    }
}
impl core::fmt::Debug for Conf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Conf")
            .field("dozeen", &self.dozeen())
            .field("dbgmode", &self.dbgmode())
            .field("gtbsync", &self.gtbsync())
            .field("gtbeen", &self.gtbeen())
            .field("csot", &self.csot())
            .field("csoo", &self.csoo())
            .field("crot", &self.crot())
            .field("cpot", &self.cpot())
            .field("trgpol", &self.trgpol())
            .field("trgsrc", &self.trgsrc())
            .field("trgsel", &self.trgsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Conf {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Conf {
            dozeen: super::vals::Dozeen,
            dbgmode: super::vals::Dbgmode,
            gtbsync: bool,
            gtbeen: bool,
            csot: bool,
            csoo: bool,
            crot: bool,
            cpot: bool,
            trgpol: super::vals::Trgpol,
            trgsrc: super::vals::Trgsrc,
            trgsel: super::vals::Trgsel,
        }
        let proxy = Conf {
            dozeen: self.dozeen(),
            dbgmode: self.dbgmode(),
            gtbsync: self.gtbsync(),
            gtbeen: self.gtbeen(),
            csot: self.csot(),
            csoo: self.csoo(),
            crot: self.crot(),
            cpot: self.cpot(),
            trgpol: self.trgpol(),
            trgsrc: self.trgsrc(),
            trgsel: self.trgsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel n Status and Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csc(pub u32);
impl Csc {
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Edge or Level Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn elsa(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Edge or Level Select A"]
    #[inline(always)]
    pub const fn set_elsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Edge or Level Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn elsb(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Edge or Level Select B"]
    #[inline(always)]
    pub const fn set_elsb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Mode Select A"]
    #[must_use]
    #[inline(always)]
    pub const fn msa(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Mode Select A"]
    #[inline(always)]
    pub const fn set_msa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Channel Mode Select B"]
    #[must_use]
    #[inline(always)]
    pub const fn msb(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Mode Select B"]
    #[inline(always)]
    pub const fn set_msb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Channel Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Interrupt Enable"]
    #[inline(always)]
    pub const fn set_chie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Channel Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn chf(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Flag"]
    #[inline(always)]
    pub const fn set_chf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Csc {
    #[inline(always)]
    fn default() -> Csc {
        Csc(0u64 as u32)
    }
}
impl core::fmt::Debug for Csc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Csc")
            .field("dma", &self.dma())
            .field("elsa", &self.elsa())
            .field("elsb", &self.elsb())
            .field("msa", &self.msa())
            .field("msb", &self.msb())
            .field("chie", &self.chie())
            .field("chf", &self.chf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Csc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Csc {
            dma: bool,
            elsa: bool,
            elsb: bool,
            msa: bool,
            msb: bool,
            chie: bool,
            chf: bool,
        }
        let proxy = Csc {
            dma: self.dma(),
            elsa: self.elsa(),
            elsb: self.elsb(),
            msa: self.msa(),
            msb: self.msb(),
            chie: self.chie(),
            chf: self.chf(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Filter Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Filter(pub u32);
impl Filter {
    #[doc = "Channel 0 Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0fval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 0 Filter Value"]
    #[inline(always)]
    pub const fn set_ch0fval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Channel 1 Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1fval(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 1 Filter Value"]
    #[inline(always)]
    pub const fn set_ch1fval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Channel 2 Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2fval(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 2 Filter Value"]
    #[inline(always)]
    pub const fn set_ch2fval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Channel 3 Filter Value"]
    #[must_use]
    #[inline(always)]
    pub const fn ch3fval(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Channel 3 Filter Value"]
    #[inline(always)]
    pub const fn set_ch3fval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Filter {
    #[inline(always)]
    fn default() -> Filter {
        Filter(0u64 as u32)
    }
}
impl core::fmt::Debug for Filter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Filter")
            .field("ch0fval", &self.ch0fval())
            .field("ch1fval", &self.ch1fval())
            .field("ch2fval", &self.ch2fval())
            .field("ch3fval", &self.ch3fval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Filter {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Filter {
            ch0fval: u8,
            ch1fval: u8,
            ch2fval: u8,
            ch3fval: u8,
        }
        let proxy = Filter {
            ch0fval: self.ch0fval(),
            ch1fval: self.ch1fval(),
            ch2fval: self.ch2fval(),
            ch3fval: self.ch3fval(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TPM Global"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Global(pub u32);
impl Global {
    #[doc = "No Update"]
    #[must_use]
    #[inline(always)]
    pub const fn noupdate(&self) -> super::vals::Noupdate {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Noupdate::from_bits(val as u8)
    }
    #[doc = "No Update"]
    #[inline(always)]
    pub const fn set_noupdate(&mut self, val: super::vals::Noupdate) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Global {
    #[inline(always)]
    fn default() -> Global {
        Global(0u64 as u32)
    }
}
impl core::fmt::Debug for Global {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Global")
            .field("noupdate", &self.noupdate())
            .field("rst", &self.rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Global {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Global {
            noupdate: super::vals::Noupdate,
            rst: bool,
        }
        let proxy = Global {
            noupdate: self.noupdate(),
            rst: self.rst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Channel Count"]
    #[must_use]
    #[inline(always)]
    pub const fn chan(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Channel Count"]
    #[inline(always)]
    pub const fn set_chan(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Trigger Count"]
    #[must_use]
    #[inline(always)]
    pub const fn trig(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Count"]
    #[inline(always)]
    pub const fn set_trig(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Counter Width"]
    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Counter Width"]
    #[inline(always)]
    pub const fn set_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(2098180u64 as u32)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("chan", &self.chan())
            .field("trig", &self.trig())
            .field("width", &self.width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Param {
            chan: u8,
            trig: u8,
            width: u8,
        }
        let proxy = Param {
            chan: self.chan(),
            trig: self.trig(),
            width: self.width(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel Polarity"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pol(pub u32);
impl Pol {
    #[doc = "Channel 0 Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol0(&self) -> super::vals::Pol0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pol0::from_bits(val as u8)
    }
    #[doc = "Channel 0 Polarity"]
    #[inline(always)]
    pub const fn set_pol0(&mut self, val: super::vals::Pol0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol1(&self) -> super::vals::Pol1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pol1::from_bits(val as u8)
    }
    #[doc = "Channel 1 Polarity"]
    #[inline(always)]
    pub const fn set_pol1(&mut self, val: super::vals::Pol1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol2(&self) -> super::vals::Pol2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pol2::from_bits(val as u8)
    }
    #[doc = "Channel 2 Polarity"]
    #[inline(always)]
    pub const fn set_pol2(&mut self, val: super::vals::Pol2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol3(&self) -> super::vals::Pol3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pol3::from_bits(val as u8)
    }
    #[doc = "Channel 3 Polarity"]
    #[inline(always)]
    pub const fn set_pol3(&mut self, val: super::vals::Pol3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Pol {
    #[inline(always)]
    fn default() -> Pol {
        Pol(0u64 as u32)
    }
}
impl core::fmt::Debug for Pol {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pol")
            .field("pol0", &self.pol0())
            .field("pol1", &self.pol1())
            .field("pol2", &self.pol2())
            .field("pol3", &self.pol3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pol {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pol {
            pol0: super::vals::Pol0,
            pol1: super::vals::Pol1,
            pol2: super::vals::Pol2,
            pol3: super::vals::Pol3,
        }
        let proxy = Pol {
            pol0: self.pol0(),
            pol1: self.pol1(),
            pol2: self.pol2(),
            pol3: self.pol3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Quadrature Decoder Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qdctrl(pub u32);
impl Qdctrl {
    #[doc = "Quadrature Decoder Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn quaden(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Quadrature Decoder Enable"]
    #[inline(always)]
    pub const fn set_quaden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer Overflow Direction"]
    #[must_use]
    #[inline(always)]
    pub const fn tofdir(&self) -> super::vals::Tofdir {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tofdir::from_bits(val as u8)
    }
    #[doc = "Timer Overflow Direction"]
    #[inline(always)]
    pub const fn set_tofdir(&mut self, val: super::vals::Tofdir) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Counter Direction in Quadrature Decode Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn quadir(&self) -> super::vals::Quadir {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Quadir::from_bits(val as u8)
    }
    #[doc = "Counter Direction in Quadrature Decode Mode"]
    #[inline(always)]
    pub const fn set_quadir(&mut self, val: super::vals::Quadir) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Quadrature Decoder Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn quadmode(&self) -> super::vals::Quadmode {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Quadmode::from_bits(val as u8)
    }
    #[doc = "Quadrature Decoder Mode"]
    #[inline(always)]
    pub const fn set_quadmode(&mut self, val: super::vals::Quadmode) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Qdctrl {
    #[inline(always)]
    fn default() -> Qdctrl {
        Qdctrl(0u64 as u32)
    }
}
impl core::fmt::Debug for Qdctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Qdctrl")
            .field("quaden", &self.quaden())
            .field("tofdir", &self.tofdir())
            .field("quadir", &self.quadir())
            .field("quadmode", &self.quadmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Qdctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Qdctrl {
            quaden: bool,
            tofdir: super::vals::Tofdir,
            quadir: super::vals::Quadir,
            quadmode: super::vals::Quadmode,
        }
        let proxy = Qdctrl {
            quaden: self.quaden(),
            tofdir: self.tofdir(),
            quadir: self.quadir(),
            quadmode: self.quadmode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status and Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sc(pub u32);
impl Sc {
    #[doc = "Prescale Factor Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Prescale Factor Selection"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Clock Mode Selection"]
    #[must_use]
    #[inline(always)]
    pub const fn cmod(&self) -> super::vals::Cmod {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Cmod::from_bits(val as u8)
    }
    #[doc = "Clock Mode Selection"]
    #[inline(always)]
    pub const fn set_cmod(&mut self, val: super::vals::Cmod) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Center-Aligned PWM Select"]
    #[must_use]
    #[inline(always)]
    pub const fn cpwms(&self) -> super::vals::Cpwms {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cpwms::from_bits(val as u8)
    }
    #[doc = "Center-Aligned PWM Select"]
    #[inline(always)]
    pub const fn set_cpwms(&mut self, val: super::vals::Cpwms) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Timer Overflow Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn toie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub const fn set_toie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable"]
    #[inline(always)]
    pub const fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Sc {
    #[inline(always)]
    fn default() -> Sc {
        Sc(0u64 as u32)
    }
}
impl core::fmt::Debug for Sc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sc")
            .field("ps", &self.ps())
            .field("cmod", &self.cmod())
            .field("cpwms", &self.cpwms())
            .field("toie", &self.toie())
            .field("tof", &self.tof())
            .field("dma", &self.dma())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sc {
            ps: super::vals::Ps,
            cmod: super::vals::Cmod,
            cpwms: super::vals::Cpwms,
            toie: bool,
            tof: bool,
            dma: bool,
        }
        let proxy = Sc {
            ps: self.ps(),
            cmod: self.cmod(),
            cpwms: self.cpwms(),
            toie: self.toie(),
            tof: self.tof(),
            dma: self.dma(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capture and Compare Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Channel 0 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ch0f(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Flag"]
    #[inline(always)]
    pub const fn set_ch0f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ch1f(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Flag"]
    #[inline(always)]
    pub const fn set_ch1f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ch2f(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Flag"]
    #[inline(always)]
    pub const fn set_ch2f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ch3f(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Flag"]
    #[inline(always)]
    pub const fn set_ch3f(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Timer Overflow Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tof(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Overflow Flag"]
    #[inline(always)]
    pub const fn set_tof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0u64 as u32)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("ch0f", &self.ch0f())
            .field("ch1f", &self.ch1f())
            .field("ch2f", &self.ch2f())
            .field("ch3f", &self.ch3f())
            .field("tof", &self.tof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Status {
            ch0f: bool,
            ch1f: bool,
            ch2f: bool,
            ch3f: bool,
            tof: bool,
        }
        let proxy = Status {
            ch0f: self.ch0f(),
            ch1f: self.ch1f(),
            ch2f: self.ch2f(),
            ch3f: self.ch3f(),
            tof: self.tof(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel Trigger"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trig(pub u32);
impl Trig {
    #[doc = "Channel 0 Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trig0(&self) -> super::vals::Trig0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Trig0::from_bits(val as u8)
    }
    #[doc = "Channel 0 Trigger"]
    #[inline(always)]
    pub const fn set_trig0(&mut self, val: super::vals::Trig0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trig1(&self) -> super::vals::Trig1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Trig1::from_bits(val as u8)
    }
    #[doc = "Channel 1 Trigger"]
    #[inline(always)]
    pub const fn set_trig1(&mut self, val: super::vals::Trig1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trig2(&self) -> super::vals::Trig2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Trig2::from_bits(val as u8)
    }
    #[doc = "Channel 2 Trigger"]
    #[inline(always)]
    pub const fn set_trig2(&mut self, val: super::vals::Trig2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trig3(&self) -> super::vals::Trig3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Trig3::from_bits(val as u8)
    }
    #[doc = "Channel 3 Trigger"]
    #[inline(always)]
    pub const fn set_trig3(&mut self, val: super::vals::Trig3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Trig {
    #[inline(always)]
    fn default() -> Trig {
        Trig(0u64 as u32)
    }
}
impl core::fmt::Debug for Trig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trig")
            .field("trig0", &self.trig0())
            .field("trig1", &self.trig1())
            .field("trig2", &self.trig2())
            .field("trig3", &self.trig3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Trig {
            trig0: super::vals::Trig0,
            trig1: super::vals::Trig1,
            trig2: super::vals::Trig2,
            trig3: super::vals::Trig3,
        }
        let proxy = Trig {
            trig0: self.trig0(),
            trig1: self.trig1(),
            trig2: self.trig2(),
            trig3: self.trig3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Identification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Identification Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Verid {
    #[inline(always)]
    fn default() -> Verid {
        Verid(100663303u64 as u32)
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
            feature: super::vals::Feature,
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
