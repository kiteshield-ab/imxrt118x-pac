#[doc = "ERM Memory 0 Correctable Error Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CorrErrCnt0(pub u32);
impl CorrErrCnt0 {
    #[doc = "Memory n Correctable Error Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory n Correctable Error Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CorrErrCnt0 {
    #[inline(always)]
    fn default() -> CorrErrCnt0 {
        CorrErrCnt0(0u64 as u32)
    }
}
impl core::fmt::Debug for CorrErrCnt0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CorrErrCnt0")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CorrErrCnt0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CorrErrCnt0 {
            count: u8,
        }
        let proxy = CorrErrCnt0 {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ERM Memory 1 Correctable Error Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CorrErrCnt1(pub u32);
impl CorrErrCnt1 {
    #[doc = "Memory n Correctable Error Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory n Correctable Error Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CorrErrCnt1 {
    #[inline(always)]
    fn default() -> CorrErrCnt1 {
        CorrErrCnt1(0u64 as u32)
    }
}
impl core::fmt::Debug for CorrErrCnt1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CorrErrCnt1")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CorrErrCnt1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CorrErrCnt1 {
            count: u8,
        }
        let proxy = CorrErrCnt1 {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ERM Memory 2 Correctable Error Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CorrErrCnt2(pub u32);
impl CorrErrCnt2 {
    #[doc = "Memory n Correctable Error Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory n Correctable Error Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CorrErrCnt2 {
    #[inline(always)]
    fn default() -> CorrErrCnt2 {
        CorrErrCnt2(0u64 as u32)
    }
}
impl core::fmt::Debug for CorrErrCnt2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CorrErrCnt2")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CorrErrCnt2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CorrErrCnt2 {
            count: u8,
        }
        let proxy = CorrErrCnt2 {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ERM Memory 3 Correctable Error Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CorrErrCnt3(pub u32);
impl CorrErrCnt3 {
    #[doc = "Memory n Correctable Error Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Memory n Correctable Error Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for CorrErrCnt3 {
    #[inline(always)]
    fn default() -> CorrErrCnt3 {
        CorrErrCnt3(0u64 as u32)
    }
}
impl core::fmt::Debug for CorrErrCnt3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CorrErrCnt3")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CorrErrCnt3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CorrErrCnt3 {
            count: u8,
        }
        let proxy = CorrErrCnt3 {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ERM Configuration Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr0(pub u32);
impl Cr0 {
    #[doc = "ENCIE3"]
    #[must_use]
    #[inline(always)]
    pub const fn encie3(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE3"]
    #[inline(always)]
    pub const fn set_encie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ESCIE3"]
    #[must_use]
    #[inline(always)]
    pub const fn escie3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE3"]
    #[inline(always)]
    pub const fn set_escie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "ENCIE2"]
    #[must_use]
    #[inline(always)]
    pub const fn encie2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE2"]
    #[inline(always)]
    pub const fn set_encie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ESCIE2"]
    #[must_use]
    #[inline(always)]
    pub const fn escie2(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE2"]
    #[inline(always)]
    pub const fn set_escie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "ENCIE1"]
    #[must_use]
    #[inline(always)]
    pub const fn encie1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE1"]
    #[inline(always)]
    pub const fn set_encie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "ESCIE1"]
    #[must_use]
    #[inline(always)]
    pub const fn escie1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE1"]
    #[inline(always)]
    pub const fn set_escie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "ENCIE0"]
    #[must_use]
    #[inline(always)]
    pub const fn encie0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ENCIE0"]
    #[inline(always)]
    pub const fn set_encie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "ESCIE0"]
    #[must_use]
    #[inline(always)]
    pub const fn escie0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "ESCIE0"]
    #[inline(always)]
    pub const fn set_escie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr0 {
    #[inline(always)]
    fn default() -> Cr0 {
        Cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr0")
            .field("encie3", &self.encie3())
            .field("escie3", &self.escie3())
            .field("encie2", &self.encie2())
            .field("escie2", &self.escie2())
            .field("encie1", &self.encie1())
            .field("escie1", &self.escie1())
            .field("encie0", &self.encie0())
            .field("escie0", &self.escie0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cr0 {
            encie3: bool,
            escie3: bool,
            encie2: bool,
            escie2: bool,
            encie1: bool,
            escie1: bool,
            encie0: bool,
            escie0: bool,
        }
        let proxy = Cr0 {
            encie3: self.encie3(),
            escie3: self.escie3(),
            encie2: self.encie2(),
            escie2: self.escie2(),
            encie1: self.encie1(),
            escie1: self.escie1(),
            encie0: self.encie0(),
            escie0: self.escie0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ERM Status Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr0(pub u32);
impl Sr0 {
    #[doc = "NCE3"]
    #[must_use]
    #[inline(always)]
    pub const fn nce3(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "NCE3"]
    #[inline(always)]
    pub const fn set_nce3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SBC3"]
    #[must_use]
    #[inline(always)]
    pub const fn sbc3(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "SBC3"]
    #[inline(always)]
    pub const fn set_sbc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "NCE2"]
    #[must_use]
    #[inline(always)]
    pub const fn nce2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "NCE2"]
    #[inline(always)]
    pub const fn set_nce2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "SBC2"]
    #[must_use]
    #[inline(always)]
    pub const fn sbc2(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "SBC2"]
    #[inline(always)]
    pub const fn set_sbc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "NCE1"]
    #[must_use]
    #[inline(always)]
    pub const fn nce1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "NCE1"]
    #[inline(always)]
    pub const fn set_nce1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "SBC1"]
    #[must_use]
    #[inline(always)]
    pub const fn sbc1(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "SBC1"]
    #[inline(always)]
    pub const fn set_sbc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "NCE0"]
    #[must_use]
    #[inline(always)]
    pub const fn nce0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "NCE0"]
    #[inline(always)]
    pub const fn set_nce0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "SBC0"]
    #[must_use]
    #[inline(always)]
    pub const fn sbc0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SBC0"]
    #[inline(always)]
    pub const fn set_sbc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sr0 {
    #[inline(always)]
    fn default() -> Sr0 {
        Sr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr0")
            .field("nce3", &self.nce3())
            .field("sbc3", &self.sbc3())
            .field("nce2", &self.nce2())
            .field("sbc2", &self.sbc2())
            .field("nce1", &self.nce1())
            .field("sbc1", &self.sbc1())
            .field("nce0", &self.nce0())
            .field("sbc0", &self.sbc0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sr0 {
            nce3: bool,
            sbc3: bool,
            nce2: bool,
            sbc2: bool,
            nce1: bool,
            sbc1: bool,
            nce0: bool,
            sbc0: bool,
        }
        let proxy = Sr0 {
            nce3: self.nce3(),
            sbc3: self.sbc3(),
            nce2: self.nce2(),
            sbc2: self.sbc2(),
            nce1: self.nce1(),
            sbc1: self.sbc1(),
            nce0: self.nce0(),
            sbc0: self.sbc0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
