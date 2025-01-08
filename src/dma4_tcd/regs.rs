#[doc = "Channel Control and Status Register"]
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
    #[doc = "Swap size"]
    #[must_use]
    #[inline(always)]
    pub const fn swap(&self) -> super::vals::Swap {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Swap::from_bits(val as u8)
    }
    #[doc = "Swap size"]
    #[inline(always)]
    pub const fn set_swap(&mut self, val: super::vals::Swap) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
    #[doc = "Sign Extension"]
    #[must_use]
    #[inline(always)]
    pub const fn signext(&self) -> super::vals::Signext {
        let val = (self.0 >> 16usize) & 0x3f;
        super::vals::Signext::from_bits(val as u8)
    }
    #[doc = "Sign Extension"]
    #[inline(always)]
    pub const fn set_signext(&mut self, val: super::vals::Signext) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val.to_bits() as u32) & 0x3f) << 16usize);
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
            .field("swap", &self.swap())
            .field("signext", &self.signext())
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
            swap: super::vals::Swap,
            signext: super::vals::Signext,
            done: bool,
            active: bool,
        }
        let proxy = ChCsr {
            erq: self.erq(),
            earq: self.earq(),
            eei: self.eei(),
            swap: self.swap(),
            signext: self.signext(),
            done: self.done(),
            active: self.active(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Memory Attributes Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChMattr(pub u16);
impl ChMattr {
    #[doc = "Read Cache Attributes"]
    #[must_use]
    #[inline(always)]
    pub const fn rcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Read Cache Attributes"]
    #[inline(always)]
    pub const fn set_rcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Write Cache Attributes"]
    #[must_use]
    #[inline(always)]
    pub const fn wcache(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Write Cache Attributes"]
    #[inline(always)]
    pub const fn set_wcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
}
impl Default for ChMattr {
    #[inline(always)]
    fn default() -> ChMattr {
        ChMattr(0u64 as u16)
    }
}
impl core::fmt::Debug for ChMattr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChMattr")
            .field("rcache", &self.rcache())
            .field("wcache", &self.wcache())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChMattr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChMattr {
            rcache: u8,
            wcache: u8,
        }
        let proxy = ChMattr {
            rcache: self.rcache(),
            wcache: self.wcache(),
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
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Service Request Source"]
    #[inline(always)]
    pub const fn set_src(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
#[doc = "Channel System Bus Register"]
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
    #[doc = "Instruction/Data Access"]
    #[must_use]
    #[inline(always)]
    pub const fn instr(&self) -> super::vals::Instr {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Instr::from_bits(val as u8)
    }
    #[doc = "Instruction/Data Access"]
    #[inline(always)]
    pub const fn set_instr(&mut self, val: super::vals::Instr) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
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
    #[doc = "Enable Master ID replication"]
    #[must_use]
    #[inline(always)]
    pub const fn emi(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Master ID replication"]
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
            .field("instr", &self.instr())
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
            instr: super::vals::Instr,
            sec: bool,
            pal: super::super::dma_tcd::vals::Pal,
            emi: bool,
            attr: u8,
        }
        let proxy = ChSbr {
            mid: self.mid(),
            instr: self.instr(),
            sec: self.sec(),
            pal: self.pal(),
            emi: self.emi(),
            attr: self.attr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TCD Beginning Major Loop Count (Minor Loop Channel Linking Enabled) Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcdBiterElinkyes(pub u16);
impl TcdBiterElinkyes {
    #[doc = "Starting major iteration count"]
    #[must_use]
    #[inline(always)]
    pub const fn biter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Starting major iteration count"]
    #[inline(always)]
    pub const fn set_biter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "Link Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn linkch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u16) & 0x3f) << 9usize);
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enables channel-to-channel linking on minor loop complete"]
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
#[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Disabled) Register"]
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
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
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
#[doc = "TCD Current Major Loop Count (Minor Loop Channel Linking Enabled) Register"]
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
        let val = (self.0 >> 9usize) & 0x3f;
        val as u8
    }
    #[doc = "Minor Loop Link Channel Number"]
    #[inline(always)]
    pub const fn set_linkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 9usize)) | (((val as u16) & 0x3f) << 9usize);
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn elink(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on minor-loop complete"]
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
#[doc = "TCD Control and Status Register"]
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
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[must_use]
    #[inline(always)]
    pub const fn intmajor(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major iteration count completes."]
    #[inline(always)]
    pub const fn set_intmajor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[must_use]
    #[inline(always)]
    pub const fn inthalf(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub const fn set_inthalf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
    }
    #[doc = "Disable request"]
    #[must_use]
    #[inline(always)]
    pub const fn dreq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable request"]
    #[inline(always)]
    pub const fn set_dreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Enable Scatter/Gather processing"]
    #[must_use]
    #[inline(always)]
    pub const fn esg(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Scatter/Gather processing"]
    #[inline(always)]
    pub const fn set_esg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[must_use]
    #[inline(always)]
    pub const fn majorelink(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub const fn set_majorelink(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
    #[doc = "Enable end-of-packet processing"]
    #[must_use]
    #[inline(always)]
    pub const fn eeop(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable end-of-packet processing"]
    #[inline(always)]
    pub const fn set_eeop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
    }
    #[doc = "Enable store destination address"]
    #[must_use]
    #[inline(always)]
    pub const fn esda(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable store destination address"]
    #[inline(always)]
    pub const fn set_esda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "Major loop link channel number"]
    #[must_use]
    #[inline(always)]
    pub const fn majorlinkch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Major loop link channel number"]
    #[inline(always)]
    pub const fn set_majorlinkch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u16) & 0x3f) << 8usize);
    }
    #[doc = "Transfer Mode Control"]
    #[must_use]
    #[inline(always)]
    pub const fn tmc(&self) -> super::vals::Tmc {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Tmc::from_bits(val as u8)
    }
    #[doc = "Transfer Mode Control"]
    #[inline(always)]
    pub const fn set_tmc(&mut self, val: super::vals::Tmc) {
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
            .field("tmc", &self.tmc())
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
            tmc: super::vals::Tmc,
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
            tmc: self.tmc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
