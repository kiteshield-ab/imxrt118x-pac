#[doc = "FLEXIO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "FLEXIO Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn flexen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FLEXIO Enable"]
    #[inline(always)]
    pub const fn set_flexen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn swrst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_swrst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fast Access"]
    #[must_use]
    #[inline(always)]
    pub const fn fastacc(&self) -> super::vals::Fastacc {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Fastacc::from_bits(val as u8)
    }
    #[doc = "Fast Access"]
    #[inline(always)]
    pub const fn set_fastacc(&mut self, val: super::vals::Fastacc) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbge(&self) -> super::vals::Dbge {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dbge::from_bits(val as u8)
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbge(&mut self, val: super::vals::Dbge) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Doze Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dozen(&self) -> super::vals::Dozen {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dozen::from_bits(val as u8)
    }
    #[doc = "Doze Enable"]
    #[inline(always)]
    pub const fn set_dozen(&mut self, val: super::vals::Dozen) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("flexen", &self.flexen())
            .field("swrst", &self.swrst())
            .field("fastacc", &self.fastacc())
            .field("dbge", &self.dbge())
            .field("dozen", &self.dozen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl {
            flexen: bool,
            swrst: bool,
            fastacc: super::vals::Fastacc,
            dbge: super::vals::Dbge,
            dozen: super::vals::Dozen,
        }
        let proxy = Ctrl {
            flexen: self.flexen(),
            swrst: self.swrst(),
            fastacc: self.fastacc(),
            dbge: self.dbge(),
            dozen: self.dozen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Shifter Number"]
    #[must_use]
    #[inline(always)]
    pub const fn shifter(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Number"]
    #[inline(always)]
    pub const fn set_shifter(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timer Number"]
    #[must_use]
    #[inline(always)]
    pub const fn timer(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Number"]
    #[inline(always)]
    pub const fn set_timer(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Pin Number"]
    #[must_use]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Pin Number"]
    #[inline(always)]
    pub const fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Trigger Number"]
    #[must_use]
    #[inline(always)]
    pub const fn trigger(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Trigger Number"]
    #[inline(always)]
    pub const fn set_trigger(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(69208072u64 as u32)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("shifter", &self.shifter())
            .field("timer", &self.timer())
            .field("pin", &self.pin())
            .field("trigger", &self.trigger())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Param {
            shifter: u8,
            timer: u8,
            pin: u8,
            trigger: u8,
        }
        let proxy = Param {
            shifter: self.shifter(),
            timer: self.timer(),
            pin: self.pin(),
            trigger: self.trigger(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shifter Configuration N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftcfg(pub u32);
impl Shiftcfg {
    #[doc = "Shifter Start Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sstart(&self) -> super::vals::Sstart {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sstart::from_bits(val as u8)
    }
    #[doc = "Shifter Start Bit"]
    #[inline(always)]
    pub const fn set_sstart(&mut self, val: super::vals::Sstart) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Shifter Stop bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sstop(&self) -> super::vals::Sstop {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Sstop::from_bits(val as u8)
    }
    #[doc = "Shifter Stop bit"]
    #[inline(always)]
    pub const fn set_sstop(&mut self, val: super::vals::Sstop) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input Source"]
    #[must_use]
    #[inline(always)]
    pub const fn insrc(&self) -> super::vals::Insrc {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Insrc::from_bits(val as u8)
    }
    #[doc = "Input Source"]
    #[inline(always)]
    pub const fn set_insrc(&mut self, val: super::vals::Insrc) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Late Store"]
    #[must_use]
    #[inline(always)]
    pub const fn latst(&self) -> super::vals::Latst {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Latst::from_bits(val as u8)
    }
    #[doc = "Late Store"]
    #[inline(always)]
    pub const fn set_latst(&mut self, val: super::vals::Latst) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Shifter Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ssize(&self) -> super::vals::Ssize {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Ssize::from_bits(val as u8)
    }
    #[doc = "Shifter Size"]
    #[inline(always)]
    pub const fn set_ssize(&mut self, val: super::vals::Ssize) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Parallel Width"]
    #[must_use]
    #[inline(always)]
    pub const fn pwidth(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Parallel Width"]
    #[inline(always)]
    pub const fn set_pwidth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Shiftcfg {
    #[inline(always)]
    fn default() -> Shiftcfg {
        Shiftcfg(0u64 as u32)
    }
}
impl core::fmt::Debug for Shiftcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftcfg")
            .field("sstart", &self.sstart())
            .field("sstop", &self.sstop())
            .field("insrc", &self.insrc())
            .field("latst", &self.latst())
            .field("ssize", &self.ssize())
            .field("pwidth", &self.pwidth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftcfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Shiftcfg {
            sstart: super::vals::Sstart,
            sstop: super::vals::Sstop,
            insrc: super::vals::Insrc,
            latst: super::vals::Latst,
            ssize: super::vals::Ssize,
            pwidth: u8,
        }
        let proxy = Shiftcfg {
            sstart: self.sstart(),
            sstop: self.sstop(),
            insrc: self.insrc(),
            latst: self.latst(),
            ssize: self.ssize(),
            pwidth: self.pwidth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shifter Control N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftctl(pub u32);
impl Shiftctl {
    #[doc = "Shifter Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn smod(&self) -> super::vals::Smod {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Smod::from_bits(val as u8)
    }
    #[doc = "Shifter Mode"]
    #[inline(always)]
    pub const fn set_smod(&mut self, val: super::vals::Smod) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Shifter Pin Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pinpol(&self) -> super::vals::ShiftctlPinpol {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::ShiftctlPinpol::from_bits(val as u8)
    }
    #[doc = "Shifter Pin Polarity"]
    #[inline(always)]
    pub const fn set_pinpol(&mut self, val: super::vals::ShiftctlPinpol) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Shifter Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pinsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shifter Pin Select"]
    #[inline(always)]
    pub const fn set_pinsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Shifter Pin Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::ShiftctlPincfg {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::ShiftctlPincfg::from_bits(val as u8)
    }
    #[doc = "Shifter Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::ShiftctlPincfg) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Timer Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn timpol(&self) -> super::vals::Timpol {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Timpol::from_bits(val as u8)
    }
    #[doc = "Timer Polarity"]
    #[inline(always)]
    pub const fn set_timpol(&mut self, val: super::vals::Timpol) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Timer Select"]
    #[must_use]
    #[inline(always)]
    pub const fn timsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Timer Select"]
    #[inline(always)]
    pub const fn set_timsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Shiftctl {
    #[inline(always)]
    fn default() -> Shiftctl {
        Shiftctl(0u64 as u32)
    }
}
impl core::fmt::Debug for Shiftctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftctl")
            .field("smod", &self.smod())
            .field("pinpol", &self.pinpol())
            .field("pinsel", &self.pinsel())
            .field("pincfg", &self.pincfg())
            .field("timpol", &self.timpol())
            .field("timsel", &self.timsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftctl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Shiftctl {
            smod: super::vals::Smod,
            pinpol: super::vals::ShiftctlPinpol,
            pinsel: u8,
            pincfg: super::vals::ShiftctlPincfg,
            timpol: super::vals::Timpol,
            timsel: u8,
        }
        let proxy = Shiftctl {
            smod: self.smod(),
            pinpol: self.pinpol(),
            pinsel: self.pinsel(),
            pincfg: self.pincfg(),
            timpol: self.timpol(),
            timsel: self.timsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shifter Error Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shifteien(pub u32);
impl Shifteien {
    #[doc = "Shifter Error Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn seie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub const fn set_seie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shifteien {
    #[inline(always)]
    fn default() -> Shifteien {
        Shifteien(0u64 as u32)
    }
}
impl core::fmt::Debug for Shifteien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shifteien")
            .field("seie", &self.seie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shifteien {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Shifteien {
            seie: u8,
        }
        let proxy = Shifteien { seie: self.seie() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shifter Error"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shifterr(pub u32);
impl Shifterr {
    #[doc = "Shifter Error Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn sef(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Error Flags"]
    #[inline(always)]
    pub const fn set_sef(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shifterr {
    #[inline(always)]
    fn default() -> Shifterr {
        Shifterr(0u64 as u32)
    }
}
impl core::fmt::Debug for Shifterr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shifterr")
            .field("sef", &self.sef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shifterr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Shifterr {
            sef: u8,
        }
        let proxy = Shifterr { sef: self.sef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shifter Status DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftsden(pub u32);
impl Shiftsden {
    #[doc = "Shifter Status DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssde(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status DMA Enable"]
    #[inline(always)]
    pub const fn set_ssde(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftsden {
    #[inline(always)]
    fn default() -> Shiftsden {
        Shiftsden(0u64 as u32)
    }
}
impl core::fmt::Debug for Shiftsden {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftsden")
            .field("ssde", &self.ssde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftsden {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Shiftsden {
            ssde: u8,
        }
        let proxy = Shiftsden { ssde: self.ssde() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shifter Status Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftsien(pub u32);
impl Shiftsien {
    #[doc = "Shifter Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ssie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_ssie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftsien {
    #[inline(always)]
    fn default() -> Shiftsien {
        Shiftsien(0u64 as u32)
    }
}
impl core::fmt::Debug for Shiftsien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftsien")
            .field("ssie", &self.ssie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftsien {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Shiftsien {
            ssie: u8,
        }
        let proxy = Shiftsien { ssie: self.ssie() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shifter Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftstat(pub u32);
impl Shiftstat {
    #[doc = "Shifter Status Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ssf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Shifter Status Flag"]
    #[inline(always)]
    pub const fn set_ssf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Shiftstat {
    #[inline(always)]
    fn default() -> Shiftstat {
        Shiftstat(0u64 as u32)
    }
}
impl core::fmt::Debug for Shiftstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftstat")
            .field("ssf", &self.ssf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftstat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Shiftstat {
            ssf: u8,
        }
        let proxy = Shiftstat { ssf: self.ssf() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shifter State"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shiftstate(pub u32);
impl Shiftstate {
    #[doc = "Current State Pointer"]
    #[must_use]
    #[inline(always)]
    pub const fn state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Current State Pointer"]
    #[inline(always)]
    pub const fn set_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Shiftstate {
    #[inline(always)]
    fn default() -> Shiftstate {
        Shiftstate(0u64 as u32)
    }
}
impl core::fmt::Debug for Shiftstate {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Shiftstate")
            .field("state", &self.state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Shiftstate {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Shiftstate {
            state: u8,
        }
        let proxy = Shiftstate {
            state: self.state(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Configuration N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timcfg(pub u32);
impl Timcfg {
    #[doc = "Timer Start Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Start Bit"]
    #[inline(always)]
    pub const fn set_tstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer Stop Bit"]
    #[must_use]
    #[inline(always)]
    pub const fn tstop(&self) -> super::vals::Tstop {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Tstop::from_bits(val as u8)
    }
    #[doc = "Timer Stop Bit"]
    #[inline(always)]
    pub const fn set_tstop(&mut self, val: super::vals::Tstop) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Timer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn timena(&self) -> super::vals::Timena {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Timena::from_bits(val as u8)
    }
    #[doc = "Timer Enable"]
    #[inline(always)]
    pub const fn set_timena(&mut self, val: super::vals::Timena) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Timer Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn timdis(&self) -> super::vals::Timdis {
        let val = (self.0 >> 12usize) & 0x07;
        super::vals::Timdis::from_bits(val as u8)
    }
    #[doc = "Timer Disable"]
    #[inline(always)]
    pub const fn set_timdis(&mut self, val: super::vals::Timdis) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
    }
    #[doc = "Timer Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn timrst(&self) -> super::vals::Timrst {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Timrst::from_bits(val as u8)
    }
    #[doc = "Timer Reset"]
    #[inline(always)]
    pub const fn set_timrst(&mut self, val: super::vals::Timrst) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Timer Decrement"]
    #[must_use]
    #[inline(always)]
    pub const fn timdec(&self) -> super::vals::Timdec {
        let val = (self.0 >> 20usize) & 0x07;
        super::vals::Timdec::from_bits(val as u8)
    }
    #[doc = "Timer Decrement"]
    #[inline(always)]
    pub const fn set_timdec(&mut self, val: super::vals::Timdec) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
    }
    #[doc = "Timer Output"]
    #[must_use]
    #[inline(always)]
    pub const fn timout(&self) -> super::vals::Timout {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Timout::from_bits(val as u8)
    }
    #[doc = "Timer Output"]
    #[inline(always)]
    pub const fn set_timout(&mut self, val: super::vals::Timout) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Timcfg {
    #[inline(always)]
    fn default() -> Timcfg {
        Timcfg(0u64 as u32)
    }
}
impl core::fmt::Debug for Timcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timcfg")
            .field("tstart", &self.tstart())
            .field("tstop", &self.tstop())
            .field("timena", &self.timena())
            .field("timdis", &self.timdis())
            .field("timrst", &self.timrst())
            .field("timdec", &self.timdec())
            .field("timout", &self.timout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timcfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timcfg {
            tstart: bool,
            tstop: super::vals::Tstop,
            timena: super::vals::Timena,
            timdis: super::vals::Timdis,
            timrst: super::vals::Timrst,
            timdec: super::vals::Timdec,
            timout: super::vals::Timout,
        }
        let proxy = Timcfg {
            tstart: self.tstart(),
            tstop: self.tstop(),
            timena: self.timena(),
            timdis: self.timdis(),
            timrst: self.timrst(),
            timdec: self.timdec(),
            timout: self.timout(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Compare N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timcmp(pub u32);
impl Timcmp {
    #[doc = "Timer Compare Value"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Timer Compare Value"]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Timcmp {
    #[inline(always)]
    fn default() -> Timcmp {
        Timcmp(0u64 as u32)
    }
}
impl core::fmt::Debug for Timcmp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timcmp").field("cmp", &self.cmp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timcmp {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timcmp {
            cmp: u16,
        }
        let proxy = Timcmp { cmp: self.cmp() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Control N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timctl(pub u32);
impl Timctl {
    #[doc = "Timer Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn timod(&self) -> super::vals::Timod {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Timod::from_bits(val as u8)
    }
    #[doc = "Timer Mode"]
    #[inline(always)]
    pub const fn set_timod(&mut self, val: super::vals::Timod) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Timer One Time Operation"]
    #[must_use]
    #[inline(always)]
    pub const fn onetim(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Timer One Time Operation"]
    #[inline(always)]
    pub const fn set_onetim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Timer Pin Input Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pinins(&self) -> super::vals::Pinins {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pinins::from_bits(val as u8)
    }
    #[doc = "Timer Pin Input Select"]
    #[inline(always)]
    pub const fn set_pinins(&mut self, val: super::vals::Pinins) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Timer Pin Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pinpol(&self) -> super::vals::TimctlPinpol {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::TimctlPinpol::from_bits(val as u8)
    }
    #[doc = "Timer Pin Polarity"]
    #[inline(always)]
    pub const fn set_pinpol(&mut self, val: super::vals::TimctlPinpol) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Timer Pin Select"]
    #[must_use]
    #[inline(always)]
    pub const fn pinsel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Timer Pin Select"]
    #[inline(always)]
    pub const fn set_pinsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Timer Pin Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn pincfg(&self) -> super::vals::TimctlPincfg {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::TimctlPincfg::from_bits(val as u8)
    }
    #[doc = "Timer Pin Configuration"]
    #[inline(always)]
    pub const fn set_pincfg(&mut self, val: super::vals::TimctlPincfg) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Trigger Source"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsrc(&self) -> super::vals::Trgsrc {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Trgsrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn set_trgsrc(&mut self, val: super::vals::Trgsrc) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Trigger Polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn trgpol(&self) -> super::vals::Trgpol {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Trgpol::from_bits(val as u8)
    }
    #[doc = "Trigger Polarity"]
    #[inline(always)]
    pub const fn set_trgpol(&mut self, val: super::vals::Trgpol) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn trgsel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_trgsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Timctl {
    #[inline(always)]
    fn default() -> Timctl {
        Timctl(0u64 as u32)
    }
}
impl core::fmt::Debug for Timctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timctl")
            .field("timod", &self.timod())
            .field("onetim", &self.onetim())
            .field("pinins", &self.pinins())
            .field("pinpol", &self.pinpol())
            .field("pinsel", &self.pinsel())
            .field("pincfg", &self.pincfg())
            .field("trgsrc", &self.trgsrc())
            .field("trgpol", &self.trgpol())
            .field("trgsel", &self.trgsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timctl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timctl {
            timod: super::vals::Timod,
            onetim: bool,
            pinins: super::vals::Pinins,
            pinpol: super::vals::TimctlPinpol,
            pinsel: u8,
            pincfg: super::vals::TimctlPincfg,
            trgsrc: super::vals::Trgsrc,
            trgpol: super::vals::Trgpol,
            trgsel: u8,
        }
        let proxy = Timctl {
            timod: self.timod(),
            onetim: self.onetim(),
            pinins: self.pinins(),
            pinpol: self.pinpol(),
            pinsel: self.pinsel(),
            pincfg: self.pincfg(),
            trgsrc: self.trgsrc(),
            trgpol: self.trgpol(),
            trgsel: self.trgsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Status DMA Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timersden(pub u32);
impl Timersden {
    #[doc = "Timer Status DMA Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tsde(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status DMA Enable"]
    #[inline(always)]
    pub const fn set_tsde(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Timersden {
    #[inline(always)]
    fn default() -> Timersden {
        Timersden(0u64 as u32)
    }
}
impl core::fmt::Debug for Timersden {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timersden")
            .field("tsde", &self.tsde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timersden {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timersden {
            tsde: u8,
        }
        let proxy = Timersden { tsde: self.tsde() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timien(pub u32);
impl Timien {
    #[doc = "Timer Status Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn teie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status Interrupt Enable"]
    #[inline(always)]
    pub const fn set_teie(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Timien {
    #[inline(always)]
    fn default() -> Timien {
        Timien(0u64 as u32)
    }
}
impl core::fmt::Debug for Timien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timien")
            .field("teie", &self.teie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timien {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timien {
            teie: u8,
        }
        let proxy = Timien { teie: self.teie() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timstat(pub u32);
impl Timstat {
    #[doc = "Timer Status Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn tsf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timer Status Flags"]
    #[inline(always)]
    pub const fn set_tsf(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Timstat {
    #[inline(always)]
    fn default() -> Timstat {
        Timstat(0u64 as u32)
    }
}
impl core::fmt::Debug for Timstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timstat").field("tsf", &self.tsf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timstat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timstat {
            tsf: u8,
        }
        let proxy = Timstat { tsf: self.tsf() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Trigger Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trgstat(pub u32);
impl Trgstat {
    #[doc = "External Trigger Status Flags"]
    #[must_use]
    #[inline(always)]
    pub const fn etsf(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "External Trigger Status Flags"]
    #[inline(always)]
    pub const fn set_etsf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Trgstat {
    #[inline(always)]
    fn default() -> Trgstat {
        Trgstat(0u64 as u32)
    }
}
impl core::fmt::Debug for Trgstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trgstat")
            .field("etsf", &self.etsf())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trgstat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Trgstat {
            etsf: u8,
        }
        let proxy = Trgstat { etsf: self.etsf() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "External Trigger Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigien(pub u32);
impl Trigien {
    #[doc = "External Trigger Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn trie(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "External Trigger Interrupt Enable"]
    #[inline(always)]
    pub const fn set_trie(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Trigien {
    #[inline(always)]
    fn default() -> Trigien {
        Trigien(0u64 as u32)
    }
}
impl core::fmt::Debug for Trigien {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trigien")
            .field("trie", &self.trie())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trigien {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Trigien {
            trie: u8,
        }
        let proxy = Trigien { trie: self.trie() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Specification Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> super::vals::Feature {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Feature::from_bits(val as u16)
    }
    #[doc = "Feature Specification Number"]
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
        Verid(33619971u64 as u32)
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
