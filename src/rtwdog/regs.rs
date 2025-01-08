#[doc = "WDOG Counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cnt(pub u32);
impl Cnt {
    #[doc = "Counter High Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn cntlow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Counter High Byte"]
    #[inline(always)]
    pub const fn set_cntlow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Counter Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn cnthigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Counter Low Byte"]
    #[inline(always)]
    pub const fn set_cnthigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Cnt {
    #[inline(always)]
    fn default() -> Cnt {
        Cnt(0u64 as u32)
    }
}
impl core::fmt::Debug for Cnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cnt")
            .field("cntlow", &self.cntlow())
            .field("cnthigh", &self.cnthigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cnt {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cnt {
            cntlow: u8,
            cnthigh: u8,
        }
        let proxy = Cnt {
            cntlow: self.cntlow(),
            cnthigh: self.cnthigh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "WDOG Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs(pub u32);
impl Cs {
    #[doc = "Stop Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Stop Enable"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Wait Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wait(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Wait Enable"]
    #[inline(always)]
    pub const fn set_wait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Debug Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable"]
    #[inline(always)]
    pub const fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "WDOG Test"]
    #[must_use]
    #[inline(always)]
    pub const fn tst(&self) -> super::vals::Tst {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Tst::from_bits(val as u8)
    }
    #[doc = "WDOG Test"]
    #[inline(always)]
    pub const fn set_tst(&mut self, val: super::vals::Tst) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Updates Allowed"]
    #[must_use]
    #[inline(always)]
    pub const fn update(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Updates Allowed"]
    #[inline(always)]
    pub const fn set_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "WDOG Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn int(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG Interrupt"]
    #[inline(always)]
    pub const fn set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "WDOG Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "WDOG Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn clk(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "WDOG Clock"]
    #[inline(always)]
    pub const fn set_clk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Reconfiguration Success"]
    #[must_use]
    #[inline(always)]
    pub const fn rcs(&self) -> super::vals::Rcs {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Rcs::from_bits(val as u8)
    }
    #[doc = "Reconfiguration Success"]
    #[inline(always)]
    pub const fn set_rcs(&mut self, val: super::vals::Rcs) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Unlock Status"]
    #[must_use]
    #[inline(always)]
    pub const fn ulk(&self) -> super::vals::Ulk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Ulk::from_bits(val as u8)
    }
    #[doc = "Unlock Status"]
    #[inline(always)]
    pub const fn set_ulk(&mut self, val: super::vals::Ulk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "WDOG Prescaler"]
    #[must_use]
    #[inline(always)]
    pub const fn pres(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG Prescaler"]
    #[inline(always)]
    pub const fn set_pres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Command 32 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd32en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Command 32 Enable"]
    #[inline(always)]
    pub const fn set_cmd32en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "WDOG Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn flg(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG Interrupt Flag"]
    #[inline(always)]
    pub const fn set_flg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "WDOG Window"]
    #[must_use]
    #[inline(always)]
    pub const fn win(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG Window"]
    #[inline(always)]
    pub const fn set_win(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        Cs(2304u64 as u32)
    }
}
impl core::fmt::Debug for Cs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cs")
            .field("stop", &self.stop())
            .field("wait", &self.wait())
            .field("dbg", &self.dbg())
            .field("tst", &self.tst())
            .field("update", &self.update())
            .field("int", &self.int())
            .field("en", &self.en())
            .field("clk", &self.clk())
            .field("rcs", &self.rcs())
            .field("ulk", &self.ulk())
            .field("pres", &self.pres())
            .field("cmd32en", &self.cmd32en())
            .field("flg", &self.flg())
            .field("win", &self.win())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cs {
            stop: bool,
            wait: bool,
            dbg: bool,
            tst: super::vals::Tst,
            update: bool,
            int: bool,
            en: bool,
            clk: u8,
            rcs: super::vals::Rcs,
            ulk: super::vals::Ulk,
            pres: bool,
            cmd32en: bool,
            flg: bool,
            win: bool,
        }
        let proxy = Cs {
            stop: self.stop(),
            wait: self.wait(),
            dbg: self.dbg(),
            tst: self.tst(),
            update: self.update(),
            int: self.int(),
            en: self.en(),
            clk: self.clk(),
            rcs: self.rcs(),
            ulk: self.ulk(),
            pres: self.pres(),
            cmd32en: self.cmd32en(),
            flg: self.flg(),
            win: self.win(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "WDOG Timeout Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Toval(pub u32);
impl Toval {
    #[doc = "Timeout Value Low"]
    #[must_use]
    #[inline(always)]
    pub const fn tovallow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout Value Low"]
    #[inline(always)]
    pub const fn set_tovallow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Timeout Value High"]
    #[must_use]
    #[inline(always)]
    pub const fn tovalhigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Timeout Value High"]
    #[inline(always)]
    pub const fn set_tovalhigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Toval {
    #[inline(always)]
    fn default() -> Toval {
        Toval(1024u64 as u32)
    }
}
impl core::fmt::Debug for Toval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Toval")
            .field("tovallow", &self.tovallow())
            .field("tovalhigh", &self.tovalhigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Toval {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Toval {
            tovallow: u8,
            tovalhigh: u8,
        }
        let proxy = Toval {
            tovallow: self.tovallow(),
            tovalhigh: self.tovalhigh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Watchdog Window"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Win(pub u32);
impl Win {
    #[doc = "Low Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn winlow(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Low Byte"]
    #[inline(always)]
    pub const fn set_winlow(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "High Byte"]
    #[must_use]
    #[inline(always)]
    pub const fn winhigh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "High Byte"]
    #[inline(always)]
    pub const fn set_winhigh(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Win {
    #[inline(always)]
    fn default() -> Win {
        Win(0u64 as u32)
    }
}
impl core::fmt::Debug for Win {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Win")
            .field("winlow", &self.winlow())
            .field("winhigh", &self.winhigh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Win {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Win {
            winlow: u8,
            winhigh: u8,
        }
        let proxy = Win {
            winlow: self.winlow(),
            winhigh: self.winhigh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
