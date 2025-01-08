#[doc = "EMDIO uncorrectable fatal system bus error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emdioufsbecr(pub u32);
impl Emdioufsbecr {
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
impl Default for Emdioufsbecr {
    #[inline(always)]
    fn default() -> Emdioufsbecr {
        Emdioufsbecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Emdioufsbecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emdioufsbecr")
            .field("rd", &self.rd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emdioufsbecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Emdioufsbecr {
            rd: bool,
        }
        let proxy = Emdioufsbecr { rd: self.rd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EMDIO uncorrectable fatal system bus error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emdioufsbesr(pub u32);
impl Emdioufsbesr {
    #[doc = "System Bus ID"]
    #[must_use]
    #[inline(always)]
    pub const fn sb_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "System Bus ID"]
    #[inline(always)]
    pub const fn set_sb_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
    #[doc = "System bus error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "System bus error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Emdioufsbesr {
    #[inline(always)]
    fn default() -> Emdioufsbesr {
        Emdioufsbesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Emdioufsbesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emdioufsbesr")
            .field("sb_id", &self.sb_id())
            .field("m", &self.m())
            .field("sbe", &self.sbe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emdioufsbesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Emdioufsbesr {
            sb_id: u8,
            m: bool,
            sbe: bool,
        }
        let proxy = Emdioufsbesr {
            sb_id: self.sb_id(),
            m: self.m(),
            sbe: self.sbe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Hash bucket table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hbtcapr(pub u32);
impl Hbtcapr {
    #[doc = "Specifies the total number of allocate bucket entries for use."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Specifies the total number of allocate bucket entries for use."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Specifies the maximum exact match hash collisions chain allowed"]
    #[must_use]
    #[inline(always)]
    pub const fn max_col(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Specifies the maximum exact match hash collisions chain allowed"]
    #[inline(always)]
    pub const fn set_max_col(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Specifies the maximum number of hash entries visited during a table management search command"]
    #[must_use]
    #[inline(always)]
    pub const fn max_visits(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the maximum number of hash entries visited during a table management search command"]
    #[inline(always)]
    pub const fn set_max_visits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Hbtcapr {
    #[inline(always)]
    fn default() -> Hbtcapr {
        Hbtcapr(838863872u64 as u32)
    }
}
impl core::fmt::Debug for Hbtcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hbtcapr")
            .field("num_entries", &self.num_entries())
            .field("max_col", &self.max_col())
            .field("max_visits", &self.max_visits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hbtcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hbtcapr {
            num_entries: u16,
            max_col: u8,
            max_visits: u8,
        }
        let proxy = Hbtcapr {
            num_entries: self.num_entries(),
            max_col: self.max_col(),
            max_visits: self.max_visits(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Hash bucket table operational register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hbtor0(pub u32);
impl Hbtor0 {
    #[doc = "Specifies the amount of entries used"]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Specifies the amount of entries used"]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Specifies the high watermark of used buckets Register is reset to NUM_ENTRIES after read."]
    #[must_use]
    #[inline(always)]
    pub const fn hwm_entries(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Specifies the high watermark of used buckets Register is reset to NUM_ENTRIES after read."]
    #[inline(always)]
    pub const fn set_hwm_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Hbtor0 {
    #[inline(always)]
    fn default() -> Hbtor0 {
        Hbtor0(0u64 as u32)
    }
}
impl core::fmt::Debug for Hbtor0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hbtor0")
            .field("num_entries", &self.num_entries())
            .field("hwm_entries", &self.hwm_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hbtor0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hbtor0 {
            num_entries: u16,
            hwm_entries: u16,
        }
        let proxy = Hbtor0 {
            num_entries: self.num_entries(),
            hwm_entries: self.hwm_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Hash bucket table operational register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hbtor2(pub u32);
impl Hbtor2 {
    #[doc = "The fractional portion of the running average length of hash lookup"]
    #[must_use]
    #[inline(always)]
    pub const fn run_avg_fract(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The fractional portion of the running average length of hash lookup"]
    #[inline(always)]
    pub const fn set_run_avg_fract(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The integer portion of the running average length of hash lookup"]
    #[must_use]
    #[inline(always)]
    pub const fn run_avg_int(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The integer portion of the running average length of hash lookup"]
    #[inline(always)]
    pub const fn set_run_avg_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Length of the longest hash collision chain noticed since the last read. Range: 0..8"]
    #[must_use]
    #[inline(always)]
    pub const fn hwm_col(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Length of the longest hash collision chain noticed since the last read. Range: 0..8"]
    #[inline(always)]
    pub const fn set_hwm_col(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Hbtor2 {
    #[inline(always)]
    fn default() -> Hbtor2 {
        Hbtor2(256u64 as u32)
    }
}
impl core::fmt::Debug for Hbtor2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hbtor2")
            .field("run_avg_fract", &self.run_avg_fract())
            .field("run_avg_int", &self.run_avg_int())
            .field("hwm_col", &self.hwm_col())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hbtor2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hbtor2 {
            run_avg_fract: u8,
            run_avg_int: u8,
            hwm_col: u8,
        }
        let proxy = Hbtor2 {
            run_avg_fract: self.run_avg_fract(),
            run_avg_int: self.run_avg_int(),
            hwm_col: self.hwm_col(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HTA 0 capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hta0capr(pub u32);
impl Hta0capr {
    #[doc = "Maximum number of Rx frames the HTA hardware block can process concurrently."]
    #[must_use]
    #[inline(always)]
    pub const fn max_rx_frames(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum number of Rx frames the HTA hardware block can process concurrently."]
    #[inline(always)]
    pub const fn set_max_rx_frames(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Maximum number of Tx frames the HTA hardware block can process concurrently."]
    #[must_use]
    #[inline(always)]
    pub const fn max_tx_frames(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Maximum number of Tx frames the HTA hardware block can process concurrently."]
    #[inline(always)]
    pub const fn set_max_tx_frames(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Hta0capr {
    #[inline(always)]
    fn default() -> Hta0capr {
        Hta0capr(2056u64 as u32)
    }
}
impl core::fmt::Debug for Hta0capr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hta0capr")
            .field("max_rx_frames", &self.max_rx_frames())
            .field("max_tx_frames", &self.max_tx_frames())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hta0capr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hta0capr {
            max_rx_frames: u8,
            max_tx_frames: u8,
        }
        let proxy = Hta0capr {
            max_rx_frames: self.max_rx_frames(),
            max_tx_frames: self.max_tx_frames(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HTA 0 high priority byte count operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hta0hpbcor(pub u32);
impl Hta0hpbcor {
    #[doc = "Amount of DMA bytes in progress for all high priority tier HTA threads."]
    #[must_use]
    #[inline(always)]
    pub const fn hp_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Amount of DMA bytes in progress for all high priority tier HTA threads."]
    #[inline(always)]
    pub const fn set_hp_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn hwm(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    #[inline(always)]
    pub const fn set_hwm(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Hta0hpbcor {
    #[inline(always)]
    fn default() -> Hta0hpbcor {
        Hta0hpbcor(0u64 as u32)
    }
}
impl core::fmt::Debug for Hta0hpbcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hta0hpbcor")
            .field("hp_count", &self.hp_count())
            .field("hwm", &self.hwm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hta0hpbcor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hta0hpbcor {
            hp_count: u16,
            hwm: u16,
        }
        let proxy = Hta0hpbcor {
            hp_count: self.hp_count(),
            hwm: self.hwm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HTA 0 low priority byte count operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hta0lpbcor(pub u32);
impl Hta0lpbcor {
    #[doc = "Amount of DMA bytes in progress for all low priority tier HTA threads."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Amount of DMA bytes in progress for all low priority tier HTA threads."]
    #[inline(always)]
    pub const fn set_lp_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn hwm(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    #[inline(always)]
    pub const fn set_hwm(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Hta0lpbcor {
    #[inline(always)]
    fn default() -> Hta0lpbcor {
        Hta0lpbcor(0u64 as u32)
    }
}
impl core::fmt::Debug for Hta0lpbcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hta0lpbcor")
            .field("lp_count", &self.lp_count())
            .field("hwm", &self.hwm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hta0lpbcor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hta0lpbcor {
            lp_count: u16,
            hwm: u16,
        }
        let proxy = Hta0lpbcor {
            lp_count: self.lp_count(),
            hwm: self.hwm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HTA 0 receive frame count operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hta0rfcor(pub u32);
impl Hta0rfcor {
    #[doc = "Number of active high priority tier Rx frames currently being processed by the host transfer engine"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of active high priority tier Rx frames currently being processed by the host transfer engine"]
    #[inline(always)]
    pub const fn set_hp_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High watermark of concurrent Rx frames being processed by HTA since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_hwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High watermark of concurrent Rx frames being processed by HTA since the last read of this register"]
    #[inline(always)]
    pub const fn set_hp_hwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of active low priority tier Rx frames currently being processed by the host transfer engine."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_count(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of active low priority tier Rx frames currently being processed by the host transfer engine."]
    #[inline(always)]
    pub const fn set_lp_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Low watermark of concurrent Rx frames being processed since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_hwm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Low watermark of concurrent Rx frames being processed since the last read of this register"]
    #[inline(always)]
    pub const fn set_lp_hwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Hta0rfcor {
    #[inline(always)]
    fn default() -> Hta0rfcor {
        Hta0rfcor(0u64 as u32)
    }
}
impl core::fmt::Debug for Hta0rfcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hta0rfcor")
            .field("hp_count", &self.hp_count())
            .field("hp_hwm", &self.hp_hwm())
            .field("lp_count", &self.lp_count())
            .field("lp_hwm", &self.lp_hwm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hta0rfcor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hta0rfcor {
            hp_count: u8,
            hp_hwm: u8,
            lp_count: u8,
            lp_hwm: u8,
        }
        let proxy = Hta0rfcor {
            hp_count: self.hp_count(),
            hp_hwm: self.hp_hwm(),
            lp_count: self.lp_count(),
            lp_hwm: self.lp_hwm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HTA 0 transmit frame count operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hta0tfcor(pub u32);
impl Hta0tfcor {
    #[doc = "Number of active high priority tier Tx frames currently being processed by the host transfer engine"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of active high priority tier Tx frames currently being processed by the host transfer engine"]
    #[inline(always)]
    pub const fn set_hp_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High watermark of concurrent Tx frames being processed by HTA since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_hwm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High watermark of concurrent Tx frames being processed by HTA since the last read of this register"]
    #[inline(always)]
    pub const fn set_hp_hwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Number of active low priority tier Tx frames currently being processed by the host transfer engine."]
    #[must_use]
    #[inline(always)]
    pub const fn lp_count(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of active low priority tier Tx frames currently being processed by the host transfer engine."]
    #[inline(always)]
    pub const fn set_lp_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Low watermark of concurrent Tx frames being processed since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_hwm(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Low watermark of concurrent Tx frames being processed since the last read of this register"]
    #[inline(always)]
    pub const fn set_lp_hwm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Hta0tfcor {
    #[inline(always)]
    fn default() -> Hta0tfcor {
        Hta0tfcor(0u64 as u32)
    }
}
impl core::fmt::Debug for Hta0tfcor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hta0tfcor")
            .field("hp_count", &self.hp_count())
            .field("hp_hwm", &self.hp_hwm())
            .field("lp_count", &self.lp_count())
            .field("lp_hwm", &self.lp_hwm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hta0tfcor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hta0tfcor {
            hp_count: u8,
            hp_hwm: u8,
            lp_count: u8,
            lp_hwm: u8,
        }
        let proxy = Hta0tfcor {
            hp_count: self.hp_count(),
            hp_hwm: self.hp_hwm(),
            lp_count: self.lp_count(),
            lp_hwm: self.lp_hwm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP block revision register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipbrr0(pub u32);
impl Ipbrr0 {
    #[doc = "Minor revision"]
    #[must_use]
    #[inline(always)]
    pub const fn ip_mn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Minor revision"]
    #[inline(always)]
    pub const fn set_ip_mn(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Major revision"]
    #[must_use]
    #[inline(always)]
    pub const fn ip_mj(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Major revision"]
    #[inline(always)]
    pub const fn set_ip_mj(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "IP block ID"]
    #[must_use]
    #[inline(always)]
    pub const fn ip_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "IP block ID"]
    #[inline(always)]
    pub const fn set_ip_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ipbrr0 {
    #[inline(always)]
    fn default() -> Ipbrr0 {
        Ipbrr0(768u64 as u32)
    }
}
impl core::fmt::Debug for Ipbrr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipbrr0")
            .field("ip_mn", &self.ip_mn())
            .field("ip_mj", &self.ip_mj())
            .field("ip_id", &self.ip_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipbrr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipbrr0 {
            ip_mn: u8,
            ip_mj: u8,
            ip_id: u16,
        }
        let proxy = Ipbrr0 {
            ip_mn: self.ip_mn(),
            ip_mj: self.ip_mj(),
            ip_id: self.ip_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IP block revision register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipbrr1(pub u32);
impl Ipbrr1 {
    #[doc = "IP block configuration options"]
    #[must_use]
    #[inline(always)]
    pub const fn ip_cfg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "IP block configuration options"]
    #[inline(always)]
    pub const fn set_ip_cfg(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "IP block maintenance version"]
    #[must_use]
    #[inline(always)]
    pub const fn ip_mnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "IP block maintenance version"]
    #[inline(always)]
    pub const fn set_ip_mnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "IP block integration options Bit 23-16: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn ip_int(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "IP block integration options Bit 23-16: Reserved"]
    #[inline(always)]
    pub const fn set_ip_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Ipbrr1 {
    #[inline(always)]
    fn default() -> Ipbrr1 {
        Ipbrr1(65792u64 as u32)
    }
}
impl core::fmt::Debug for Ipbrr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipbrr1")
            .field("ip_cfg", &self.ip_cfg())
            .field("ip_mnt", &self.ip_mnt())
            .field("ip_int", &self.ip_int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipbrr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipbrr1 {
            ip_cfg: u8,
            ip_mnt: u8,
            ip_int: u8,
        }
        let proxy = Ipbrr1 {
            ip_cfg: self.ip_cfg(),
            ip_mnt: self.ip_mnt(),
            ip_int: self.ip_int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC clock register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Netcclkr(pub u32);
impl Netcclkr {
    #[doc = "Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Frequency"]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for Netcclkr {
    #[inline(always)]
    fn default() -> Netcclkr {
        Netcclkr(240u64 as u32)
    }
}
impl core::fmt::Debug for Netcclkr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Netcclkr")
            .field("freq", &self.freq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Netcclkr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Netcclkr {
            freq: u16,
        }
        let proxy = Netcclkr { freq: self.freq() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCE 0 operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pce0or(pub u32);
impl Pce0or {
    #[doc = "Number of active frames currently being processed by the Parse Classifier Engine."]
    #[must_use]
    #[inline(always)]
    pub const fn num_frames(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of active frames currently being processed by the Parse Classifier Engine."]
    #[inline(always)]
    pub const fn set_num_frames(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn hwm_frames(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    #[inline(always)]
    pub const fn set_hwm_frames(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Maximum number of concurrent frames that can be processed by the PCE block"]
    #[must_use]
    #[inline(always)]
    pub const fn max_frames(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Maximum number of concurrent frames that can be processed by the PCE block"]
    #[inline(always)]
    pub const fn set_max_frames(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Pce0or {
    #[inline(always)]
    fn default() -> Pce0or {
        Pce0or(1507328u64 as u32)
    }
}
impl core::fmt::Debug for Pce0or {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pce0or")
            .field("num_frames", &self.num_frames())
            .field("hwm_frames", &self.hwm_frames())
            .field("max_frames", &self.max_frames())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pce0or {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pce0or {
            num_frames: u8,
            hwm_frames: u8,
            max_frames: u8,
        }
        let proxy = Pce0or {
            num_frames: self.num_frames(),
            hwm_frames: self.hwm_frames(),
            max_frames: self.max_frames(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Root complex 0 system bus read latency average register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rc0sbrlar(pub u32);
impl Rc0sbrlar {
    #[doc = "Fractional portion of the latency value."]
    #[must_use]
    #[inline(always)]
    pub const fn fract(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fractional portion of the latency value."]
    #[inline(always)]
    pub const fn set_fract(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Integer portion of the latency value."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Integer portion of the latency value."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Rc0sbrlar {
    #[inline(always)]
    fn default() -> Rc0sbrlar {
        Rc0sbrlar(0u64 as u32)
    }
}
impl core::fmt::Debug for Rc0sbrlar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rc0sbrlar")
            .field("fract", &self.fract())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rc0sbrlar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rc0sbrlar {
            fract: u8,
            int: u16,
        }
        let proxy = Rc0sbrlar {
            fract: self.fract(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Root complex 0 system bus read latency high watermark register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rc0sbrlhwmr(pub u32);
impl Rc0sbrlhwmr {
    #[doc = "Fractional portion of the latency value."]
    #[must_use]
    #[inline(always)]
    pub const fn fract(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fractional portion of the latency value."]
    #[inline(always)]
    pub const fn set_fract(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Integer portion of the latency value."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Integer portion of the latency value."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Rc0sbrlhwmr {
    #[inline(always)]
    fn default() -> Rc0sbrlhwmr {
        Rc0sbrlhwmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rc0sbrlhwmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rc0sbrlhwmr")
            .field("fract", &self.fract())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rc0sbrlhwmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rc0sbrlhwmr {
            fract: u8,
            int: u16,
        }
        let proxy = Rc0sbrlhwmr {
            fract: self.fract(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Root complex 0 system bus write latency average register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rc0sbwlar(pub u32);
impl Rc0sbwlar {
    #[doc = "Fractional portion of the latency value."]
    #[must_use]
    #[inline(always)]
    pub const fn fract(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fractional portion of the latency value."]
    #[inline(always)]
    pub const fn set_fract(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Integer portion of the latency value."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Integer portion of the latency value."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Rc0sbwlar {
    #[inline(always)]
    fn default() -> Rc0sbwlar {
        Rc0sbwlar(0u64 as u32)
    }
}
impl core::fmt::Debug for Rc0sbwlar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rc0sbwlar")
            .field("fract", &self.fract())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rc0sbwlar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rc0sbwlar {
            fract: u8,
            int: u16,
        }
        let proxy = Rc0sbwlar {
            fract: self.fract(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Root complex 0 system bus write latency high watermark register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rc0sbwlhwmr(pub u32);
impl Rc0sbwlhwmr {
    #[doc = "Fractional portion of the latency value."]
    #[must_use]
    #[inline(always)]
    pub const fn fract(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fractional portion of the latency value."]
    #[inline(always)]
    pub const fn set_fract(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Integer portion of the latency value."]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Integer portion of the latency value."]
    #[inline(always)]
    pub const fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Rc0sbwlhwmr {
    #[inline(always)]
    fn default() -> Rc0sbwlhwmr {
        Rc0sbwlhwmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Rc0sbwlhwmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rc0sbwlhwmr")
            .field("fract", &self.fract())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rc0sbwlhwmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rc0sbwlhwmr {
            fract: u8,
            int: u16,
        }
        let proxy = Rc0sbwlhwmr {
            fract: self.fract(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Replication Forwarding Engine 0 operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfe0or(pub u32);
impl Rfe0or {
    #[doc = "Number of active frames currently being processed by the Replication and Forwarding Engine."]
    #[must_use]
    #[inline(always)]
    pub const fn num_frames(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of active frames currently being processed by the Replication and Forwarding Engine."]
    #[inline(always)]
    pub const fn set_num_frames(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn hwm_frames(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "High watermark of concurrent frames being processed since the last read of this register"]
    #[inline(always)]
    pub const fn set_hwm_frames(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Maximum number of concurrent frames that can be processed by the RFE block"]
    #[must_use]
    #[inline(always)]
    pub const fn max_frames(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "Maximum number of concurrent frames that can be processed by the RFE block"]
    #[inline(always)]
    pub const fn set_max_frames(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
}
impl Default for Rfe0or {
    #[inline(always)]
    fn default() -> Rfe0or {
        Rfe0or(1048576u64 as u32)
    }
}
impl core::fmt::Debug for Rfe0or {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rfe0or")
            .field("num_frames", &self.num_frames())
            .field("hwm_frames", &self.hwm_frames())
            .field("max_frames", &self.max_frames())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rfe0or {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rfe0or {
            num_frames: u8,
            hwm_frames: u8,
            max_frames: u8,
        }
        let proxy = Rfe0or {
            num_frames: self.num_frames(),
            hwm_frames: self.hwm_frames(),
            max_frames: self.max_frames(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory available count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smacr(pub u32);
impl Smacr {
    #[doc = "Shows the current amount of available shared memory in words"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Shows the current amount of available shared memory in words"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smacr {
    #[inline(always)]
    fn default() -> Smacr {
        Smacr(8065u64 as u32)
    }
}
impl core::fmt::Debug for Smacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smacr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smacr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smacr {
            count: u32,
        }
        let proxy = Smacr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory buffer unassigned count high watermark register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smbuchwmr(pub u32);
impl Smbuchwmr {
    #[doc = "Shows the high watermark for unassigned memory since the last read of this register, in words"]
    #[must_use]
    #[inline(always)]
    pub const fn watermark(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Shows the high watermark for unassigned memory since the last read of this register, in words"]
    #[inline(always)]
    pub const fn set_watermark(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smbuchwmr {
    #[inline(always)]
    fn default() -> Smbuchwmr {
        Smbuchwmr(47u64 as u32)
    }
}
impl core::fmt::Debug for Smbuchwmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smbuchwmr")
            .field("watermark", &self.watermark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smbuchwmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smbuchwmr {
            watermark: u32,
        }
        let proxy = Smbuchwmr {
            watermark: self.watermark(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory buffer unassigned count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smbucr(pub u32);
impl Smbucr {
    #[doc = "Shows the current amount of unassigned Shared memory buffer, in words"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Shows the current amount of unassigned Shared memory buffer, in words"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smbucr {
    #[inline(always)]
    fn default() -> Smbucr {
        Smbucr(47u64 as u32)
    }
}
impl core::fmt::Debug for Smbucr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smbucr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smbucr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smbucr {
            count: u32,
        }
        let proxy = Smbucr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcapr(pub u32);
impl Smcapr {
    #[doc = "Total amount of words in common memory available for free list to NETC buffers, frame descriptors and hash tables"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Total amount of words in common memory available for free list to NETC buffers, frame descriptors and hash tables"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smcapr {
    #[inline(always)]
    fn default() -> Smcapr {
        Smcapr(8112u64 as u32)
    }
}
impl core::fmt::Debug for Smcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcapr")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smcapr {
            num_words: u32,
        }
        let proxy = Smcapr {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory count low watermark register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smclwmr(pub u32);
impl Smclwmr {
    #[doc = "Shows the low watermark for shared memory in words since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn watermark(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Shows the low watermark for shared memory in words since the last read of this register"]
    #[inline(always)]
    pub const fn set_watermark(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smclwmr {
    #[inline(always)]
    fn default() -> Smclwmr {
        Smclwmr(8065u64 as u32)
    }
}
impl core::fmt::Debug for Smclwmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smclwmr")
            .field("watermark", &self.watermark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smclwmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smclwmr {
            watermark: u32,
        }
        let proxy = Smclwmr {
            watermark: self.watermark(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory depletion threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smdtr(pub u32);
impl Smdtr {
    #[doc = "Shared memory depletion threshold in Words"]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Shared memory depletion threshold in Words"]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smdtr {
    #[inline(always)]
    fn default() -> Smdtr {
        Smdtr(64u64 as u32)
    }
}
impl core::fmt::Debug for Smdtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smdtr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smdtr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smdtr {
            thresh: u32,
        }
        let proxy = Smdtr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory ENETC receive buffer capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smerbcapr(pub u32);
impl Smerbcapr {
    #[doc = "Threshold in words of receive buffer memory used by all ENETC functions"]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Threshold in words of receive buffer memory used by all ENETC functions"]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Word size in bytes. 0: 24 Bytes 1-3: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn word_size(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Word size in bytes. 0: 24 Bytes 1-3: reserved"]
    #[inline(always)]
    pub const fn set_word_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Indicates memory location 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Smerbcapr {
    #[inline(always)]
    fn default() -> Smerbcapr {
        Smerbcapr(1490u64 as u32)
    }
}
impl core::fmt::Debug for Smerbcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smerbcapr")
            .field("thresh", &self.thresh())
            .field("word_size", &self.word_size())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smerbcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smerbcapr {
            thresh: u32,
            word_size: u8,
            mloc: u8,
        }
        let proxy = Smerbcapr {
            thresh: self.thresh(),
            word_size: self.word_size(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory ENETC receive buffer operational register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smerbor0(pub u32);
impl Smerbor0 {
    #[doc = "Number of words in use for buffers and frame descriptors."]
    #[must_use]
    #[inline(always)]
    pub const fn amount(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Number of words in use for buffers and frame descriptors."]
    #[inline(always)]
    pub const fn set_amount(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smerbor0 {
    #[inline(always)]
    fn default() -> Smerbor0 {
        Smerbor0(0u64 as u32)
    }
}
impl core::fmt::Debug for Smerbor0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smerbor0")
            .field("amount", &self.amount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smerbor0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smerbor0 {
            amount: u32,
        }
        let proxy = Smerbor0 {
            amount: self.amount(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory ENETC receive buffer operational 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smerbor1(pub u32);
impl Smerbor1 {
    #[doc = "High watermark of words in use by buffers and frame descriptors since the last read"]
    #[must_use]
    #[inline(always)]
    pub const fn watermark(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "High watermark of words in use by buffers and frame descriptors since the last read"]
    #[inline(always)]
    pub const fn set_watermark(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smerbor1 {
    #[inline(always)]
    fn default() -> Smerbor1 {
        Smerbor1(0u64 as u32)
    }
}
impl core::fmt::Debug for Smerbor1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smerbor1")
            .field("watermark", &self.watermark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smerbor1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smerbor1 {
            watermark: u32,
        }
        let proxy = Smerbor1 {
            watermark: self.watermark(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory loss count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smlcr(pub u32);
impl Smlcr {
    #[doc = "Determinate number of lost words"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Determinate number of lost words"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indeterminate free list corruption Number of words lost due to free list corruption"]
    #[must_use]
    #[inline(always)]
    pub const fn iflc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Indeterminate free list corruption Number of words lost due to free list corruption"]
    #[inline(always)]
    pub const fn set_iflc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Indeterminate frame descriptor corruption Number of words lost due to frame's key meta data (reference count or number of IMUs) corruption"]
    #[must_use]
    #[inline(always)]
    pub const fn ifdc(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indeterminate frame descriptor corruption Number of words lost due to frame's key meta data (reference count or number of IMUs) corruption"]
    #[inline(always)]
    pub const fn set_ifdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Smlcr {
    #[inline(always)]
    fn default() -> Smlcr {
        Smlcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Smlcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smlcr")
            .field("count", &self.count())
            .field("iflc", &self.iflc())
            .field("ifdc", &self.ifdc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smlcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smlcr {
            count: u32,
            iflc: bool,
            ifdc: bool,
        }
        let proxy = Smlcr {
            count: self.count(),
            iflc: self.iflc(),
            ifdc: self.ifdc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
