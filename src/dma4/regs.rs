#[doc = "Channel Arbitration Group Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChGrpri(pub u32);
impl ChGrpri {
    #[doc = "Arbitration group per channel."]
    #[must_use]
    #[inline(always)]
    pub const fn grpri(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Arbitration group per channel."]
    #[inline(always)]
    pub const fn set_grpri(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for ChGrpri {
    #[inline(always)]
    fn default() -> ChGrpri {
        ChGrpri(0u64 as u32)
    }
}
impl core::fmt::Debug for ChGrpri {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChGrpri")
            .field("grpri", &self.grpri())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChGrpri {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChGrpri {
            grpri: u8,
        }
        let proxy = ChGrpri {
            grpri: self.grpri(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Management Page Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpCsr(pub u32);
impl MpCsr {
    #[doc = "Enable Debug"]
    #[must_use]
    #[inline(always)]
    pub const fn edbg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Debug"]
    #[inline(always)]
    pub const fn set_edbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    #[must_use]
    #[inline(always)]
    pub const fn erca(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Round Robin Channel Arbitration"]
    #[inline(always)]
    pub const fn set_erca(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Halt After Error"]
    #[must_use]
    #[inline(always)]
    pub const fn hae(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Halt After Error"]
    #[inline(always)]
    pub const fn set_hae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Halt DMA Operations"]
    #[must_use]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Halt DMA Operations"]
    #[inline(always)]
    pub const fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Global Channel Linking Control"]
    #[must_use]
    #[inline(always)]
    pub const fn gclc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Global Channel Linking Control"]
    #[inline(always)]
    pub const fn set_gclc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Global Master ID Replication Control"]
    #[must_use]
    #[inline(always)]
    pub const fn gmrc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Global Master ID Replication Control"]
    #[inline(always)]
    pub const fn set_gmrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "eDMA version"]
    #[must_use]
    #[inline(always)]
    pub const fn ver(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "eDMA version"]
    #[inline(always)]
    pub const fn set_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Active channel ID"]
    #[must_use]
    #[inline(always)]
    pub const fn active_id(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Active channel ID"]
    #[inline(always)]
    pub const fn set_active_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "DMA Active Status"]
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Active Status"]
    #[inline(always)]
    pub const fn set_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MpCsr {
    #[inline(always)]
    fn default() -> MpCsr {
        MpCsr(4194304u64 as u32)
    }
}
impl core::fmt::Debug for MpCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpCsr")
            .field("edbg", &self.edbg())
            .field("erca", &self.erca())
            .field("hae", &self.hae())
            .field("halt", &self.halt())
            .field("gclc", &self.gclc())
            .field("gmrc", &self.gmrc())
            .field("ver", &self.ver())
            .field("active_id", &self.active_id())
            .field("active", &self.active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpCsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MpCsr {
            edbg: bool,
            erca: bool,
            hae: bool,
            halt: bool,
            gclc: bool,
            gmrc: bool,
            ver: u8,
            active_id: u8,
            active: bool,
        }
        let proxy = MpCsr {
            edbg: self.edbg(),
            erca: self.erca(),
            hae: self.hae(),
            halt: self.halt(),
            gclc: self.gclc(),
            gmrc: self.gmrc(),
            ver: self.ver(),
            active_id: self.active_id(),
            active: self.active(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Management Page Error Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpEs(pub u32);
impl MpEs {
    #[doc = "Destination Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dbe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Bus Error"]
    #[inline(always)]
    pub const fn set_dbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Bus Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Bus Error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sge(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub const fn set_sge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[must_use]
    #[inline(always)]
    pub const fn nce(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub const fn set_nce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Destination Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn doe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Offset Error"]
    #[inline(always)]
    pub const fn set_doe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Destination Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn dae(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Destination Address Error"]
    #[inline(always)]
    pub const fn set_dae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Source Offset Error"]
    #[must_use]
    #[inline(always)]
    pub const fn soe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Source Offset Error"]
    #[inline(always)]
    pub const fn set_soe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Source Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn sae(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Source Address Error"]
    #[inline(always)]
    pub const fn set_sae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer Canceled"]
    #[must_use]
    #[inline(always)]
    pub const fn ecx(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Canceled"]
    #[inline(always)]
    pub const fn set_ecx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[must_use]
    #[inline(always)]
    pub const fn errchn(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub const fn set_errchn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MpEs {
    #[inline(always)]
    fn default() -> MpEs {
        MpEs(0u64 as u32)
    }
}
impl core::fmt::Debug for MpEs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpEs")
            .field("dbe", &self.dbe())
            .field("sbe", &self.sbe())
            .field("sge", &self.sge())
            .field("nce", &self.nce())
            .field("doe", &self.doe())
            .field("dae", &self.dae())
            .field("soe", &self.soe())
            .field("sae", &self.sae())
            .field("ecx", &self.ecx())
            .field("errchn", &self.errchn())
            .field("vld", &self.vld())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpEs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MpEs {
            dbe: bool,
            sbe: bool,
            sge: bool,
            nce: bool,
            doe: bool,
            dae: bool,
            soe: bool,
            sae: bool,
            ecx: bool,
            errchn: u8,
            vld: bool,
        }
        let proxy = MpEs {
            dbe: self.dbe(),
            sbe: self.sbe(),
            sge: self.sge(),
            nce: self.nce(),
            doe: self.doe(),
            dae: self.dae(),
            soe: self.soe(),
            sae: self.sae(),
            ecx: self.ecx(),
            errchn: self.errchn(),
            vld: self.vld(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Management Page Hardware Request Status Register - High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpHrsHigh(pub u32);
impl MpHrsHigh {
    #[doc = "Hardware Request Status for channels 63-32"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs(&self) -> super::vals::MpHrsHighHrs {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::MpHrsHighHrs::from_bits(val as u32)
    }
    #[doc = "Hardware Request Status for channels 63-32"]
    #[inline(always)]
    pub const fn set_hrs(&mut self, val: super::vals::MpHrsHighHrs) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MpHrsHigh {
    #[inline(always)]
    fn default() -> MpHrsHigh {
        MpHrsHigh(0u64 as u32)
    }
}
impl core::fmt::Debug for MpHrsHigh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpHrsHigh")
            .field("hrs", &self.hrs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpHrsHigh {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MpHrsHigh {
            hrs: super::vals::MpHrsHighHrs,
        }
        let proxy = MpHrsHigh { hrs: self.hrs() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Management Page Hardware Request Status Register - Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpHrsLow(pub u32);
impl MpHrsLow {
    #[doc = "Hardware Request Status for channels 31 - 0"]
    #[must_use]
    #[inline(always)]
    pub const fn hrs(&self) -> super::vals::MpHrsLowHrs {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::MpHrsLowHrs::from_bits(val as u32)
    }
    #[doc = "Hardware Request Status for channels 31 - 0"]
    #[inline(always)]
    pub const fn set_hrs(&mut self, val: super::vals::MpHrsLowHrs) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for MpHrsLow {
    #[inline(always)]
    fn default() -> MpHrsLow {
        MpHrsLow(0u64 as u32)
    }
}
impl core::fmt::Debug for MpHrsLow {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MpHrsLow")
            .field("hrs", &self.hrs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MpHrsLow {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MpHrsLow {
            hrs: super::vals::MpHrsLowHrs,
        }
        let proxy = MpHrsLow { hrs: self.hrs() };
        defmt::write!(f, "{}", proxy)
    }
}
