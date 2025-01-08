#[doc = "Correctable memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmecr(pub u32);
impl Cmecr {
    #[doc = "Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub const fn set_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cmecr {
    #[inline(always)]
    fn default() -> Cmecr {
        Cmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmecr")
            .field("threshold", &self.threshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmecr {
            threshold: u8,
        }
        let proxy = Cmecr {
            threshold: self.threshold(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Correctable memory error count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmectr(pub u32);
impl Cmectr {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cmectr {
    #[inline(always)]
    fn default() -> Cmectr {
        Cmectr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmectr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmectr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmectr {
            count: u8,
        }
        let proxy = Cmectr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Correctable memory error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmesr(pub u32);
impl Cmesr {
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Single-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Single-bit ECC error"]
    #[inline(always)]
    pub const fn set_sbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmesr {
    #[inline(always)]
    fn default() -> Cmesr {
        Cmesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmesr")
            .field("mem_id", &self.mem_id())
            .field("sbee", &self.sbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmesr {
            mem_id: u8,
            sbee: bool,
        }
        let proxy = Cmesr {
            mem_id: self.mem_id(),
            sbee: self.sbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Memory Error Injection Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Meicr(pub u32);
impl Meicr {
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Armed"]
    #[must_use]
    #[inline(always)]
    pub const fn arm(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Armed"]
    #[inline(always)]
    pub const fn set_arm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Meicr {
    #[inline(always)]
    fn default() -> Meicr {
        Meicr(0u64 as u32)
    }
}
impl core::fmt::Debug for Meicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Meicr")
            .field("mem_id", &self.mem_id())
            .field("arm", &self.arm())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Meicr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Meicr {
            mem_id: u8,
            arm: u8,
            en: u8,
        }
        let proxy = Meicr {
            mem_id: self.mem_id(),
            arm: self.arm(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Netcrr(pub u32);
impl Netcrr {
    #[doc = "Soft reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Soft reset"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Lock"]
    #[inline(always)]
    pub const fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Netcrr {
    #[inline(always)]
    fn default() -> Netcrr {
        Netcrr(0u64 as u32)
    }
}
impl core::fmt::Debug for Netcrr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Netcrr")
            .field("sr", &self.sr())
            .field("lock", &self.lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Netcrr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Netcrr {
            sr: bool,
            lock: bool,
        }
        let proxy = Netcrr {
            sr: self.sr(),
            lock: self.lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Netcsr(pub u32);
impl Netcsr {
    #[doc = "Error"]
    #[must_use]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub const fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates NETC's global operational state"]
    #[must_use]
    #[inline(always)]
    pub const fn state(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates NETC's global operational state"]
    #[inline(always)]
    pub const fn set_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Netcsr {
    #[inline(always)]
    fn default() -> Netcsr {
        Netcsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Netcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Netcsr")
            .field("error", &self.error())
            .field("state", &self.state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Netcsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Netcsr {
            error: bool,
            state: bool,
        }
        let proxy = Netcsr {
            error: self.error(),
            state: self.state(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable fatal memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ufmecr(pub u32);
impl Ufmecr {
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ufmecr {
    #[inline(always)]
    fn default() -> Ufmecr {
        Ufmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ufmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ufmecr").field("rd", &self.rd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ufmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ufmecr {
            rd: bool,
        }
        let proxy = Ufmecr { rd: self.rd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable fatal memory error status register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ufmesr0(pub u32);
impl Ufmesr0 {
    #[doc = "Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Syndrome"]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Multi-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn mbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-bit ECC error"]
    #[inline(always)]
    pub const fn set_mbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ufmesr0 {
    #[inline(always)]
    fn default() -> Ufmesr0 {
        Ufmesr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ufmesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ufmesr0")
            .field("syndrome", &self.syndrome())
            .field("mem_id", &self.mem_id())
            .field("m", &self.m())
            .field("mbee", &self.mbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ufmesr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ufmesr0 {
            syndrome: u16,
            mem_id: u8,
            m: bool,
            mbee: bool,
        }
        let proxy = Ufmesr0 {
            syndrome: self.syndrome(),
            mem_id: self.mem_id(),
            m: self.m(),
            mbee: self.mbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unmecr(pub u32);
impl Unmecr {
    #[doc = "Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub const fn set_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Unmecr {
    #[inline(always)]
    fn default() -> Unmecr {
        Unmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unmecr")
            .field("threshold", &self.threshold())
            .field("rd", &self.rd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unmecr {
            threshold: u8,
            rd: bool,
        }
        let proxy = Unmecr {
            threshold: self.threshold(),
            rd: self.rd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal memory error count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unmectr(pub u32);
impl Unmectr {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Unmectr {
    #[inline(always)]
    fn default() -> Unmectr {
        Unmectr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unmectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unmectr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unmectr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unmectr {
            count: u8,
        }
        let proxy = Unmectr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal memory error status register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unmesr0(pub u32);
impl Unmesr0 {
    #[doc = "Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Syndrome"]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Multi-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn mbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-bit ECC error"]
    #[inline(always)]
    pub const fn set_mbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Unmesr0 {
    #[inline(always)]
    fn default() -> Unmesr0 {
        Unmesr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Unmesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unmesr0")
            .field("syndrome", &self.syndrome())
            .field("mem_id", &self.mem_id())
            .field("mbee", &self.mbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unmesr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unmesr0 {
            syndrome: u16,
            mem_id: u8,
            mbee: bool,
        }
        let proxy = Unmesr0 {
            syndrome: self.syndrome(),
            mem_id: self.mem_id(),
            mbee: self.mbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
