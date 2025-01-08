#[doc = "Parameter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Par(pub u32);
impl Par {
    #[doc = "Number of Transmit (TRn) registers (8)"]
    #[must_use]
    #[inline(always)]
    pub const fn tr_num(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Transmit (TRn) registers (8)"]
    #[inline(always)]
    pub const fn set_tr_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number of Receive (RRn) registers (4)"]
    #[must_use]
    #[inline(always)]
    pub const fn rr_num(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Receive (RRn) registers (4)"]
    #[inline(always)]
    pub const fn set_rr_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Par {
    #[inline(always)]
    fn default() -> Par {
        Par(1032u64 as u32)
    }
}
impl core::fmt::Debug for Par {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Par")
            .field("tr_num", &self.tr_num())
            .field("rr_num", &self.rr_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Par {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Par {
            tr_num: u8,
            rr_num: u8,
        }
        let proxy = Par {
            tr_num: self.tr_num(),
            rr_num: self.rr_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcr(pub u32);
impl Rcr {
    #[doc = "Receive Register n Full Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rfien(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive Register n Full Interrupt Enable"]
    #[inline(always)]
    pub const fn set_rfien(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Rcr {
    #[inline(always)]
    fn default() -> Rcr {
        Rcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rcr").field("rfien", &self.rfien()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rcr {
            rfien: u8,
        }
        let proxy = Rcr {
            rfien: self.rfien(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsr(pub u32);
impl Rsr {
    #[doc = "Receive Register n Full"]
    #[must_use]
    #[inline(always)]
    pub const fn rfn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive Register n Full"]
    #[inline(always)]
    pub const fn set_rfn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Rsr {
    #[inline(always)]
    fn default() -> Rsr {
        Rsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsr").field("rfn", &self.rfn()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rsr {
            rfn: u8,
        }
        let proxy = Rsr { rfn: self.rfn() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Transmit Empty Pending"]
    #[must_use]
    #[inline(always)]
    pub const fn tep(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Empty Pending"]
    #[inline(always)]
    pub const fn set_tep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive Full Pending Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn rfp(&self) -> super::vals::Rfp {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Rfp::from_bits(val as u8)
    }
    #[doc = "Receive Full Pending Flag"]
    #[inline(always)]
    pub const fn set_rfp(&mut self, val: super::vals::Rfp) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(32u64 as u32)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("tep", &self.tep())
            .field("rfp", &self.rfp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sr {
            tep: bool,
            rfp: super::vals::Rfp,
        }
        let proxy = Sr {
            tep: self.tep(),
            rfp: self.rfp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Transmit Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr(pub u32);
impl Tcr {
    #[doc = "Transmit Register n Empty Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teien(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Register n Empty Interrupt Enable"]
    #[inline(always)]
    pub const fn set_teien(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        Tcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcr").field("teien", &self.teien()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tcr {
            teien: u8,
        }
        let proxy = Tcr {
            teien: self.teien(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Transmit Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr(pub u32);
impl Tsr {
    #[doc = "Transmit Register n Empty"]
    #[must_use]
    #[inline(always)]
    pub const fn ten(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit Register n Empty"]
    #[inline(always)]
    pub const fn set_ten(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        Tsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tsr").field("ten", &self.ten()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tsr {
            ten: u8,
        }
        let proxy = Tsr { ten: self.ten() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Unused Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unused1(pub u32);
impl Unused1 {
    #[doc = "Unused 16-bit Register"]
    #[must_use]
    #[inline(always)]
    pub const fn data16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Unused 16-bit Register"]
    #[inline(always)]
    pub const fn set_data16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Unused1 {
    #[inline(always)]
    fn default() -> Unused1 {
        Unused1(0u64 as u32)
    }
}
impl core::fmt::Debug for Unused1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unused1")
            .field("data16", &self.data16())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unused1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unused1 {
            data16: u16,
        }
        let proxy = Unused1 {
            data16: self.data16(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Version ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ver(pub u32);
impl Ver {
    #[doc = "Feature Set Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Set Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: super::vals::Feature) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
    #[doc = "Minor Version Number (0x00 )"]
    #[must_use]
    #[inline(always)]
    pub const fn minor(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor Version Number (0x00 )"]
    #[inline(always)]
    pub const fn set_minor(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major Version Number (0x01 )"]
    #[must_use]
    #[inline(always)]
    pub const fn major(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major Version Number (0x01 )"]
    #[inline(always)]
    pub const fn set_major(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ver {
    #[inline(always)]
    fn default() -> Ver {
        Ver(16777216u64 as u32)
    }
}
impl core::fmt::Debug for Ver {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ver")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ver {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ver {
            feature: super::vals::Feature,
            minor: u8,
            major: u8,
        }
        let proxy = Ver {
            feature: self.feature(),
            minor: self.minor(),
            major: self.major(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
