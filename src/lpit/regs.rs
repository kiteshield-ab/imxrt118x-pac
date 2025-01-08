#[doc = "Clear Timer Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clrten(pub u32);
impl Clrten {
    #[doc = "Clear Timer 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_t_en_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Timer 0 Enable"]
    #[inline(always)]
    pub const fn set_clr_t_en_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clear Timer 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_t_en_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Timer 1 Enable"]
    #[inline(always)]
    pub const fn set_clr_t_en_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clear Timer 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_t_en_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Timer 2 Enable"]
    #[inline(always)]
    pub const fn set_clr_t_en_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Clear Timer 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_t_en_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Clear Timer 3 Enable"]
    #[inline(always)]
    pub const fn set_clr_t_en_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Clrten {
    #[inline(always)]
    fn default() -> Clrten {
        Clrten(0u64 as u32)
    }
}
impl core::fmt::Debug for Clrten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clrten")
            .field("clr_t_en_0", &self.clr_t_en_0())
            .field("clr_t_en_1", &self.clr_t_en_1())
            .field("clr_t_en_2", &self.clr_t_en_2())
            .field("clr_t_en_3", &self.clr_t_en_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clrten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Clrten {
            clr_t_en_0: bool,
            clr_t_en_1: bool,
            clr_t_en_2: bool,
            clr_t_en_3: bool,
        }
        let proxy = Clrten {
            clr_t_en_0: self.clr_t_en_0(),
            clr_t_en_1: self.clr_t_en_1(),
            clr_t_en_2: self.clr_t_en_2(),
            clr_t_en_3: self.clr_t_en_3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcr(pub u32);
impl Mcr {
    #[doc = "Module Clock Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn m_cen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Module Clock Enable"]
    #[inline(always)]
    pub const fn set_m_cen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Software Reset"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_rst(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub const fn set_sw_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DOZE Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn doze_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DOZE Mode Enable"]
    #[inline(always)]
    pub const fn set_doze_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug Mode Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Mode Enable"]
    #[inline(always)]
    pub const fn set_dbg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        Mcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mcr")
            .field("m_cen", &self.m_cen())
            .field("sw_rst", &self.sw_rst())
            .field("doze_en", &self.doze_en())
            .field("dbg_en", &self.dbg_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mcr {
            m_cen: bool,
            sw_rst: bool,
            doze_en: bool,
            dbg_en: bool,
        }
        let proxy = Mcr {
            m_cen: self.m_cen(),
            sw_rst: self.sw_rst(),
            doze_en: self.doze_en(),
            dbg_en: self.dbg_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mier(pub u32);
impl Mier {
    #[doc = "Channel 0 Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 0 Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 1 Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 2 Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Timer Interrupt Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tie3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Channel 3 Timer Interrupt Enable"]
    #[inline(always)]
    pub const fn set_tie3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Mier {
    #[inline(always)]
    fn default() -> Mier {
        Mier(0u64 as u32)
    }
}
impl core::fmt::Debug for Mier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mier")
            .field("tie0", &self.tie0())
            .field("tie1", &self.tie1())
            .field("tie2", &self.tie2())
            .field("tie3", &self.tie3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mier {
            tie0: bool,
            tie1: bool,
            tie2: bool,
            tie3: bool,
        }
        let proxy = Mier {
            tie0: self.tie0(),
            tie1: self.tie1(),
            tie2: self.tie2(),
            tie3: self.tie3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Module Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msr(pub u32);
impl Msr {
    #[doc = "Channel 0 Timer Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif0(&self) -> super::vals::Tif0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Tif0::from_bits(val as u8)
    }
    #[doc = "Channel 0 Timer Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tif0(&mut self, val: super::vals::Tif0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Channel 1 Timer Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif1(&self) -> super::vals::Tif1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Tif1::from_bits(val as u8)
    }
    #[doc = "Channel 1 Timer Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tif1(&mut self, val: super::vals::Tif1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel 2 Timer Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif2(&self) -> super::vals::Tif2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Tif2::from_bits(val as u8)
    }
    #[doc = "Channel 2 Timer Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tif2(&mut self, val: super::vals::Tif2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Channel 3 Timer Interrupt Flag"]
    #[must_use]
    #[inline(always)]
    pub const fn tif3(&self) -> super::vals::Tif3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Tif3::from_bits(val as u8)
    }
    #[doc = "Channel 3 Timer Interrupt Flag"]
    #[inline(always)]
    pub const fn set_tif3(&mut self, val: super::vals::Tif3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Msr {
    #[inline(always)]
    fn default() -> Msr {
        Msr(0u64 as u32)
    }
}
impl core::fmt::Debug for Msr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msr")
            .field("tif0", &self.tif0())
            .field("tif1", &self.tif1())
            .field("tif2", &self.tif2())
            .field("tif3", &self.tif3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Msr {
            tif0: super::vals::Tif0,
            tif1: super::vals::Tif1,
            tif2: super::vals::Tif2,
            tif3: super::vals::Tif3,
        }
        let proxy = Msr {
            tif0: self.tif0(),
            tif1: self.tif1(),
            tif2: self.tif2(),
            tif3: self.tif3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Parameter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Param(pub u32);
impl Param {
    #[doc = "Number of Timer Channels"]
    #[must_use]
    #[inline(always)]
    pub const fn channel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Timer Channels"]
    #[inline(always)]
    pub const fn set_channel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Number of External Trigger Inputs"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_trig(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Number of External Trigger Inputs"]
    #[inline(always)]
    pub const fn set_ext_trig(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Param {
    #[inline(always)]
    fn default() -> Param {
        Param(1028u64 as u32)
    }
}
impl core::fmt::Debug for Param {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Param")
            .field("channel", &self.channel())
            .field("ext_trig", &self.ext_trig())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Param {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Param {
            channel: u8,
            ext_trig: u8,
        }
        let proxy = Param {
            channel: self.channel(),
            ext_trig: self.ext_trig(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Set Timer Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Setten(pub u32);
impl Setten {
    #[doc = "Set Timer 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn set_t_en_0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set Timer 0 Enable"]
    #[inline(always)]
    pub const fn set_set_t_en_0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set Timer 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn set_t_en_1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set Timer 1 Enable"]
    #[inline(always)]
    pub const fn set_set_t_en_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set Timer 2 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn set_t_en_2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set Timer 2 Enable"]
    #[inline(always)]
    pub const fn set_set_t_en_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set Timer 3 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn set_t_en_3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set Timer 3 Enable"]
    #[inline(always)]
    pub const fn set_set_t_en_3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Setten {
    #[inline(always)]
    fn default() -> Setten {
        Setten(0u64 as u32)
    }
}
impl core::fmt::Debug for Setten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Setten")
            .field("set_t_en_0", &self.set_t_en_0())
            .field("set_t_en_1", &self.set_t_en_1())
            .field("set_t_en_2", &self.set_t_en_2())
            .field("set_t_en_3", &self.set_t_en_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Setten {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Setten {
            set_t_en_0: bool,
            set_t_en_1: bool,
            set_t_en_2: bool,
            set_t_en_3: bool,
        }
        let proxy = Setten {
            set_t_en_0: self.set_t_en_0(),
            set_t_en_1: self.set_t_en_1(),
            set_t_en_2: self.set_t_en_2(),
            set_t_en_3: self.set_t_en_3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctrl(pub u32);
impl Tctrl {
    #[doc = "Timer Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn t_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Enable"]
    #[inline(always)]
    pub const fn set_t_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Chain Channel"]
    #[must_use]
    #[inline(always)]
    pub const fn chain(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Chain Channel"]
    #[inline(always)]
    pub const fn set_chain(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Timer Operation Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Timer Operation Mode"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Timer Start on Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn tsot(&self) -> super::vals::Tsot {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Tsot::from_bits(val as u8)
    }
    #[doc = "Timer Start on Trigger"]
    #[inline(always)]
    pub const fn set_tsot(&mut self, val: super::vals::Tsot) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Timer Stop on Interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn tsoi(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Stop on Interrupt"]
    #[inline(always)]
    pub const fn set_tsoi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timer Reload on Trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn trot(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Reload on Trigger"]
    #[inline(always)]
    pub const fn set_trot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Trigger Source"]
    #[must_use]
    #[inline(always)]
    pub const fn trg_src(&self) -> super::vals::TrgSrc {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::TrgSrc::from_bits(val as u8)
    }
    #[doc = "Trigger Source"]
    #[inline(always)]
    pub const fn set_trg_src(&mut self, val: super::vals::TrgSrc) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Trigger Select"]
    #[must_use]
    #[inline(always)]
    pub const fn trg_sel(&self) -> super::vals::TrgSel {
        let val = (self.0 >> 24usize) & 0x0f;
        super::vals::TrgSel::from_bits(val as u8)
    }
    #[doc = "Trigger Select"]
    #[inline(always)]
    pub const fn set_trg_sel(&mut self, val: super::vals::TrgSel) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
    }
}
impl Default for Tctrl {
    #[inline(always)]
    fn default() -> Tctrl {
        Tctrl(0u64 as u32)
    }
}
impl core::fmt::Debug for Tctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tctrl")
            .field("t_en", &self.t_en())
            .field("chain", &self.chain())
            .field("mode", &self.mode())
            .field("tsot", &self.tsot())
            .field("tsoi", &self.tsoi())
            .field("trot", &self.trot())
            .field("trg_src", &self.trg_src())
            .field("trg_sel", &self.trg_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tctrl {
            t_en: bool,
            chain: bool,
            mode: super::vals::Mode,
            tsot: super::vals::Tsot,
            tsoi: bool,
            trot: bool,
            trg_src: super::vals::TrgSrc,
            trg_sel: super::vals::TrgSel,
        }
        let proxy = Tctrl {
            t_en: self.t_en(),
            chain: self.chain(),
            mode: self.mode(),
            tsot: self.tsot(),
            tsoi: self.tsoi(),
            trot: self.trot(),
            trg_src: self.trg_src(),
            trg_sel: self.trg_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer Value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tval(pub u32);
impl Tval {
    #[doc = "Timer Value"]
    #[must_use]
    #[inline(always)]
    pub const fn tmr_val(&self) -> super::vals::TmrVal {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::TmrVal::from_bits(val as u32)
    }
    #[doc = "Timer Value"]
    #[inline(always)]
    pub const fn set_tmr_val(&mut self, val: super::vals::TmrVal) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tval {
    #[inline(always)]
    fn default() -> Tval {
        Tval(0u64 as u32)
    }
}
impl core::fmt::Debug for Tval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tval")
            .field("tmr_val", &self.tmr_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tval {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tval {
            tmr_val: super::vals::TmrVal,
        }
        let proxy = Tval {
            tmr_val: self.tmr_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Version ID"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Verid(pub u32);
impl Verid {
    #[doc = "Feature Number"]
    #[must_use]
    #[inline(always)]
    pub const fn feature(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Feature Number"]
    #[inline(always)]
    pub const fn set_feature(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
        Verid(16777216u64 as u32)
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
            feature: u16,
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
