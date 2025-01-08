#[doc = "Channel Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChCsr(pub u32);
impl ChCsr {
    #[doc = "Enable DMA Request"]
    #[must_use]
    #[inline(always)]
    pub const fn erq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA Request"]
    #[inline(always)]
    pub const fn set_erq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable Asynchronous DMA Request"]
    #[must_use]
    #[inline(always)]
    pub const fn earq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Asynchronous DMA Request"]
    #[inline(always)]
    pub const fn set_earq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Error Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn eei(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Error Interrupt"]
    #[inline(always)]
    pub const fn set_eei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Buffered Writes"]
    #[must_use]
    #[inline(always)]
    pub const fn ebw(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Buffered Writes"]
    #[inline(always)]
    pub const fn set_ebw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Channel Done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Channel Active"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Active"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChCsr {
    #[inline(always)]
    fn default() -> ChCsr {
        ChCsr(0u64 as u32)
    }
}
impl core::fmt::Debug for ChCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChCsr")
            .field("erq", &self.erq())
            .field("earq", &self.earq())
            .field("eei", &self.eei())
            .field("ebw", &self.ebw())
            .field("done", &self.done())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChCsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChCsr {
            erq: bool,
            earq: bool,
            eei: bool,
            ebw: bool,
            done: bool,
            active: bool,
        }
        let proxy = ChCsr {
            erq: self.erq(),
            earq: self.earq(),
            eei: self.eei(),
            ebw: self.ebw(),
            done: self.done(),
            active: self.active(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel Multiplexor Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChMux(pub u32);
impl ChMux {
    #[doc = "Service Request Source"]
    #[must_use]
    #[inline(always)]
    pub const fn src(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Service Request Source"]
    #[inline(always)]
    pub const fn set_src(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for ChMux {
    #[inline(always)]
    fn default() -> ChMux {
        ChMux(0u64 as u32)
    }
}
impl core::fmt::Debug for ChMux {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChMux").field("src", &self.src()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChMux {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChMux {
            src: u8,
        }
        let proxy = ChMux { src: self.src() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Channel System Bus"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChSbr(pub u32);
impl ChSbr {
    #[doc = "Master ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Master ID"]
    #[inline(always)]
    pub const fn set_mid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Security Level"]
    #[must_use]
    #[inline(always)]
    pub const fn sec(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Security Level"]
    #[inline(always)]
    pub const fn set_sec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Privileged Access Level"]
    #[must_use]
    #[inline(always)]
    pub const fn pal(&self) -> super::super::dma_tcd::vals::Pal {
        let val = (self.0 >> 15usize) & 0x01;
        super::super::dma_tcd::vals::Pal::from_bits(val as u8)
    }
    #[doc = "Privileged Access Level"]
    #[inline(always)]
    pub const fn set_pal(&mut self, val: super::super::dma_tcd::vals::Pal) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Enable Master ID Replication"]
    #[must_use]
    #[inline(always)]
    pub const fn emi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Master ID Replication"]
    #[inline(always)]
    pub const fn set_emi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Attribute Output"]
    #[must_use]
    #[inline(always)]
    pub const fn attr(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x3f;
        val as u8
    }
    #[doc = "Attribute Output"]
    #[inline(always)]
    pub const fn set_attr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
    }
}
impl Default for ChSbr {
    #[inline(always)]
    fn default() -> ChSbr {
        ChSbr(32775u64 as u32)
    }
}
impl core::fmt::Debug for ChSbr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChSbr")
            .field("mid", &self.mid())
            .field("sec", &self.sec())
            .field("pal", &self.pal())
            .field("emi", &self.emi())
            .field("attr", &self.attr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChSbr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChSbr {
            mid: u8,
            sec: bool,
            pal: super::super::dma_tcd::vals::Pal,
            emi: bool,
            attr: u8,
        }
        let proxy = ChSbr {
            mid: self.mid(),
            sec: self.sec(),
            pal: self.pal(),
            emi: self.emi(),
            attr: self.attr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
    #[doc = "Starting Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting Major Iteration Count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
    #[doc = "Enable Link"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Link"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdBiterElinkyes {
    #[inline(always)]
    fn default() -> TcdBiterElinkyes {
        TcdBiterElinkyes(0u64 as u16)
    }
}
impl core::fmt::Debug for TcdBiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdBiterElinkyes")
            .field("biter", &self.biter())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdBiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TcdBiterElinkyes {
            biter: u16,
            linkch: u8,
            elink: bool,
        }
        let proxy = TcdBiterElinkyes {
            biter: self.biter(),
            linkch: self.linkch(),
            elink: self.elink(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkno(pub u16);
impl TcdCiterElinkno {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u16) & 0x7fff) << 0usize);
    }
    #[doc = "Enable Link"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Link"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkno {
    #[inline(always)]
    fn default() -> TcdCiterElinkno {
        TcdCiterElinkno(0u64 as u16)
    }
}
impl core::fmt::Debug for TcdCiterElinkno {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkno")
            .field("citer", &self.citer())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkno {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TcdCiterElinkno {
            citer: u16,
            elink: bool,
        }
        let proxy = TcdCiterElinkno {
            citer: self.citer(),
            elink: self.elink(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Enabled)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCiterElinkyes(pub u16);
impl TcdCiterElinkyes {
    #[doc = "Current Major Iteration Count"]
    #[must_use]
    #[inline(always)]
    pub const fn citer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Current Major Iteration Count"]
    #[inline(always)]
    pub const fn set_citer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
    #[doc = "Enable Link"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Link"]
    #[inline(always)]
    pub const fn set_elink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for TcdCiterElinkyes {
    #[inline(always)]
    fn default() -> TcdCiterElinkyes {
        TcdCiterElinkyes(0u64 as u16)
    }
}
impl core::fmt::Debug for TcdCiterElinkyes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCiterElinkyes")
            .field("citer", &self.citer())
            .field("linkch", &self.linkch())
            .field("elink", &self.elink())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCiterElinkyes {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TcdCiterElinkyes {
            citer: u16,
            linkch: u8,
            elink: bool,
        }
        let proxy = TcdCiterElinkyes {
            citer: self.citer(),
            linkch: self.linkch(),
            elink: self.elink(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TCD Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdCsr(pub u16);
impl TcdCsr {
    #[doc = "Channel Start"]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel Start"]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
    }
    #[doc = "Enable Interrupt If Major count complete"]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt If Major count complete"]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable Interrupt If Major Counter Half-complete"]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Interrupt If Major Counter Half-complete"]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable Request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable Link When Major Loop Complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Link When Major Loop Complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Enable End-Of-Packet Processing"]
    #[must_use]
    #[inline(always)]
    pub const fn eeop(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable End-Of-Packet Processing"]
    #[inline(always)]
    pub const fn set_eeop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Enable Store Destination Address"]
    #[must_use]
    #[inline(always)]
    pub const fn esda(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Store Destination Address"]
    #[inline(always)]
    pub const fn set_esda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major Loop Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Major Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
    }
    #[doc = "Bandwidth Control"]
    #[must_use]
    #[inline(always)]
    pub const fn bwc(&self) -> super::vals::Bwc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Bwc::from_bits(val as u8)
    }
    #[doc = "Bandwidth Control"]
    #[inline(always)]
    pub const fn set_bwc(&mut self, val: super::vals::Bwc) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for TcdCsr {
    #[inline(always)]
    fn default() -> TcdCsr {
        TcdCsr(0u64 as u16)
    }
}
impl core::fmt::Debug for TcdCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TcdCsr")
            .field("start", &self.start())
            .field("intmajor", &self.intmajor())
            .field("inthalf", &self.inthalf())
            .field("dreq", &self.dreq())
            .field("esg", &self.esg())
            .field("majorelink", &self.majorelink())
            .field("eeop", &self.eeop())
            .field("esda", &self.esda())
            .field("majorlinkch", &self.majorlinkch())
            .field("bwc", &self.bwc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TcdCsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TcdCsr {
            start: bool,
            intmajor: bool,
            inthalf: bool,
            dreq: bool,
            esg: bool,
            majorelink: bool,
            eeop: bool,
            esda: bool,
            majorlinkch: u8,
            bwc: super::vals::Bwc,
        }
        let proxy = TcdCsr {
            start: self.start(),
            intmajor: self.intmajor(),
            inthalf: self.inthalf(),
            dreq: self.dreq(),
            esg: self.esg(),
            majorelink: self.majorelink(),
            eeop: self.eeop(),
            esda: self.esda(),
            majorlinkch: self.majorlinkch(),
            bwc: self.bwc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
