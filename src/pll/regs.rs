#[doc = "Fractional PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc = "DIV_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "DIV_SELECT"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ENABLE_ALT"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_alt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE_ALT"]
    #[inline(always)]
    pub const fn set_enable_alt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL Start up initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn hold_ring_off(&self) -> super::vals::HoldRingOff {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::HoldRingOff::from_bits(val as u8)
    }
    #[doc = "PLL Start up initialization"]
    #[inline(always)]
    pub const fn set_hold_ring_off(&mut self, val: super::vals::HoldRingOff) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "POWERUP"]
    #[must_use]
    #[inline(always)]
    pub const fn powerup(&self) -> super::vals::Powerup {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Powerup::from_bits(val as u8)
    }
    #[doc = "POWERUP"]
    #[inline(always)]
    pub const fn set_powerup(&mut self, val: super::vals::Powerup) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "BYPASS"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "BYPASS"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DITHER_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DITHER_EN"]
    #[inline(always)]
    pub const fn set_dither_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PLL_REG_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PLL_REG_EN"]
    #[inline(always)]
    pub const fn set_pll_reg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Post Divide Select"]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_sel(&self) -> super::vals::PostDivSel {
        let val = (self.0 >> 25usize) & 0x07;
        super::vals::PostDivSel::from_bits(val as u8)
    }
    #[doc = "Post Divide Select"]
    #[inline(always)]
    pub const fn set_post_div_sel(&mut self, val: super::vals::PostDivSel) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val.to_bits() as u32) & 0x07) << 25usize);
    }
    #[doc = "BIAS_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn bias_select(&self) -> super::vals::BiasSelect {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::BiasSelect::from_bits(val as u8)
    }
    #[doc = "BIAS_SELECT"]
    #[inline(always)]
    pub const fn set_bias_select(&mut self, val: super::vals::BiasSelect) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Ctrl0 {
    #[inline(always)]
    fn default() -> Ctrl0 {
        Ctrl0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0")
            .field("div_select", &self.div_select())
            .field("enable_alt", &self.enable_alt())
            .field("hold_ring_off", &self.hold_ring_off())
            .field("powerup", &self.powerup())
            .field("enable", &self.enable())
            .field("bypass", &self.bypass())
            .field("dither_en", &self.dither_en())
            .field("pll_reg_en", &self.pll_reg_en())
            .field("post_div_sel", &self.post_div_sel())
            .field("bias_select", &self.bias_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0 {
            div_select: u8,
            enable_alt: bool,
            hold_ring_off: super::vals::HoldRingOff,
            powerup: super::vals::Powerup,
            enable: bool,
            bypass: bool,
            dither_en: bool,
            pll_reg_en: bool,
            post_div_sel: super::vals::PostDivSel,
            bias_select: super::vals::BiasSelect,
        }
        let proxy = Ctrl0 {
            div_select: self.div_select(),
            enable_alt: self.enable_alt(),
            hold_ring_off: self.hold_ring_off(),
            powerup: self.powerup(),
            enable: self.enable(),
            bypass: self.bypass(),
            dither_en: self.dither_en(),
            pll_reg_en: self.pll_reg_en(),
            post_div_sel: self.post_div_sel(),
            bias_select: self.bias_select(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Clr(pub u32);
impl Ctrl0Clr {
    #[doc = "DIV_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "DIV_SELECT"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ENABLE_ALT"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_alt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE_ALT"]
    #[inline(always)]
    pub const fn set_enable_alt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL Start up initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn hold_ring_off(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Start up initialization"]
    #[inline(always)]
    pub const fn set_hold_ring_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "POWERUP"]
    #[must_use]
    #[inline(always)]
    pub const fn powerup(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "POWERUP"]
    #[inline(always)]
    pub const fn set_powerup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "BYPASS"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "BYPASS"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DITHER_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DITHER_EN"]
    #[inline(always)]
    pub const fn set_dither_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PLL_REG_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PLL_REG_EN"]
    #[inline(always)]
    pub const fn set_pll_reg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Post Divide Select"]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_sel(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "Post Divide Select"]
    #[inline(always)]
    pub const fn set_post_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "BIAS_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn bias_select(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "BIAS_SELECT"]
    #[inline(always)]
    pub const fn set_bias_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ctrl0Clr {
    #[inline(always)]
    fn default() -> Ctrl0Clr {
        Ctrl0Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Clr")
            .field("div_select", &self.div_select())
            .field("enable_alt", &self.enable_alt())
            .field("hold_ring_off", &self.hold_ring_off())
            .field("powerup", &self.powerup())
            .field("enable", &self.enable())
            .field("bypass", &self.bypass())
            .field("dither_en", &self.dither_en())
            .field("pll_reg_en", &self.pll_reg_en())
            .field("post_div_sel", &self.post_div_sel())
            .field("bias_select", &self.bias_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Clr {
            div_select: u8,
            enable_alt: bool,
            hold_ring_off: bool,
            powerup: bool,
            enable: bool,
            bypass: bool,
            dither_en: bool,
            pll_reg_en: bool,
            post_div_sel: u8,
            bias_select: bool,
        }
        let proxy = Ctrl0Clr {
            div_select: self.div_select(),
            enable_alt: self.enable_alt(),
            hold_ring_off: self.hold_ring_off(),
            powerup: self.powerup(),
            enable: self.enable(),
            bypass: self.bypass(),
            dither_en: self.dither_en(),
            pll_reg_en: self.pll_reg_en(),
            post_div_sel: self.post_div_sel(),
            bias_select: self.bias_select(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Set(pub u32);
impl Ctrl0Set {
    #[doc = "DIV_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "DIV_SELECT"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ENABLE_ALT"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_alt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE_ALT"]
    #[inline(always)]
    pub const fn set_enable_alt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL Start up initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn hold_ring_off(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Start up initialization"]
    #[inline(always)]
    pub const fn set_hold_ring_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "POWERUP"]
    #[must_use]
    #[inline(always)]
    pub const fn powerup(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "POWERUP"]
    #[inline(always)]
    pub const fn set_powerup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "BYPASS"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "BYPASS"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DITHER_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DITHER_EN"]
    #[inline(always)]
    pub const fn set_dither_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PLL_REG_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PLL_REG_EN"]
    #[inline(always)]
    pub const fn set_pll_reg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Post Divide Select"]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_sel(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "Post Divide Select"]
    #[inline(always)]
    pub const fn set_post_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "BIAS_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn bias_select(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "BIAS_SELECT"]
    #[inline(always)]
    pub const fn set_bias_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ctrl0Set {
    #[inline(always)]
    fn default() -> Ctrl0Set {
        Ctrl0Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Set")
            .field("div_select", &self.div_select())
            .field("enable_alt", &self.enable_alt())
            .field("hold_ring_off", &self.hold_ring_off())
            .field("powerup", &self.powerup())
            .field("enable", &self.enable())
            .field("bypass", &self.bypass())
            .field("dither_en", &self.dither_en())
            .field("pll_reg_en", &self.pll_reg_en())
            .field("post_div_sel", &self.post_div_sel())
            .field("bias_select", &self.bias_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Set {
            div_select: u8,
            enable_alt: bool,
            hold_ring_off: bool,
            powerup: bool,
            enable: bool,
            bypass: bool,
            dither_en: bool,
            pll_reg_en: bool,
            post_div_sel: u8,
            bias_select: bool,
        }
        let proxy = Ctrl0Set {
            div_select: self.div_select(),
            enable_alt: self.enable_alt(),
            hold_ring_off: self.hold_ring_off(),
            powerup: self.powerup(),
            enable: self.enable(),
            bypass: self.bypass(),
            dither_en: self.dither_en(),
            pll_reg_en: self.pll_reg_en(),
            post_div_sel: self.post_div_sel(),
            bias_select: self.bias_select(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Tog(pub u32);
impl Ctrl0Tog {
    #[doc = "DIV_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn div_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "DIV_SELECT"]
    #[inline(always)]
    pub const fn set_div_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "ENABLE_ALT"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_alt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE_ALT"]
    #[inline(always)]
    pub const fn set_enable_alt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL Start up initialization"]
    #[must_use]
    #[inline(always)]
    pub const fn hold_ring_off(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "PLL Start up initialization"]
    #[inline(always)]
    pub const fn set_hold_ring_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "POWERUP"]
    #[must_use]
    #[inline(always)]
    pub const fn powerup(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "POWERUP"]
    #[inline(always)]
    pub const fn set_powerup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ENABLE"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "BYPASS"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "BYPASS"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DITHER_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DITHER_EN"]
    #[inline(always)]
    pub const fn set_dither_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "PLL_REG_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_reg_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PLL_REG_EN"]
    #[inline(always)]
    pub const fn set_pll_reg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Post Divide Select"]
    #[must_use]
    #[inline(always)]
    pub const fn post_div_sel(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "Post Divide Select"]
    #[inline(always)]
    pub const fn set_post_div_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "BIAS_SELECT"]
    #[must_use]
    #[inline(always)]
    pub const fn bias_select(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "BIAS_SELECT"]
    #[inline(always)]
    pub const fn set_bias_select(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for Ctrl0Tog {
    #[inline(always)]
    fn default() -> Ctrl0Tog {
        Ctrl0Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl0Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl0Tog")
            .field("div_select", &self.div_select())
            .field("enable_alt", &self.enable_alt())
            .field("hold_ring_off", &self.hold_ring_off())
            .field("powerup", &self.powerup())
            .field("enable", &self.enable())
            .field("bypass", &self.bypass())
            .field("dither_en", &self.dither_en())
            .field("pll_reg_en", &self.pll_reg_en())
            .field("post_div_sel", &self.post_div_sel())
            .field("bias_select", &self.bias_select())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Tog {
            div_select: u8,
            enable_alt: bool,
            hold_ring_off: bool,
            powerup: bool,
            enable: bool,
            bypass: bool,
            dither_en: bool,
            pll_reg_en: bool,
            post_div_sel: u8,
            bias_select: bool,
        }
        let proxy = Ctrl0Tog {
            div_select: self.div_select(),
            enable_alt: self.enable_alt(),
            hold_ring_off: self.hold_ring_off(),
            powerup: self.powerup(),
            enable: self.enable(),
            bypass: self.bypass(),
            dither_en: self.dither_en(),
            pll_reg_en: self.pll_reg_en(),
            post_div_sel: self.post_div_sel(),
            bias_select: self.bias_select(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Denominator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Denominator(pub u32);
impl Denominator {
    #[doc = "Denominator"]
    #[must_use]
    #[inline(always)]
    pub const fn denom(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Denominator"]
    #[inline(always)]
    pub const fn set_denom(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Denominator {
    #[inline(always)]
    fn default() -> Denominator {
        Denominator(0u64 as u32)
    }
}
impl core::fmt::Debug for Denominator {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Denominator")
            .field("denom", &self.denom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Denominator {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Denominator {
            denom: u32,
        }
        let proxy = Denominator {
            denom: self.denom(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Denominator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DenominatorClr(pub u32);
impl DenominatorClr {
    #[doc = "Denominator"]
    #[must_use]
    #[inline(always)]
    pub const fn denom(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Denominator"]
    #[inline(always)]
    pub const fn set_denom(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for DenominatorClr {
    #[inline(always)]
    fn default() -> DenominatorClr {
        DenominatorClr(0u64 as u32)
    }
}
impl core::fmt::Debug for DenominatorClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DenominatorClr")
            .field("denom", &self.denom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DenominatorClr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DenominatorClr {
            denom: u32,
        }
        let proxy = DenominatorClr {
            denom: self.denom(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Denominator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DenominatorSet(pub u32);
impl DenominatorSet {
    #[doc = "Denominator"]
    #[must_use]
    #[inline(always)]
    pub const fn denom(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Denominator"]
    #[inline(always)]
    pub const fn set_denom(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for DenominatorSet {
    #[inline(always)]
    fn default() -> DenominatorSet {
        DenominatorSet(0u64 as u32)
    }
}
impl core::fmt::Debug for DenominatorSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DenominatorSet")
            .field("denom", &self.denom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DenominatorSet {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DenominatorSet {
            denom: u32,
        }
        let proxy = DenominatorSet {
            denom: self.denom(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Denominator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DenominatorTog(pub u32);
impl DenominatorTog {
    #[doc = "Denominator"]
    #[must_use]
    #[inline(always)]
    pub const fn denom(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Denominator"]
    #[inline(always)]
    pub const fn set_denom(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for DenominatorTog {
    #[inline(always)]
    fn default() -> DenominatorTog {
        DenominatorTog(0u64 as u32)
    }
}
impl core::fmt::Debug for DenominatorTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DenominatorTog")
            .field("denom", &self.denom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DenominatorTog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DenominatorTog {
            denom: u32,
        }
        let proxy = DenominatorTog {
            denom: self.denom(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Numerator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Numerator(pub u32);
impl Numerator {
    #[doc = "Numerator"]
    #[must_use]
    #[inline(always)]
    pub const fn num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Numerator"]
    #[inline(always)]
    pub const fn set_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Numerator {
    #[inline(always)]
    fn default() -> Numerator {
        Numerator(0u64 as u32)
    }
}
impl core::fmt::Debug for Numerator {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Numerator")
            .field("num", &self.num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Numerator {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Numerator {
            num: u32,
        }
        let proxy = Numerator { num: self.num() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Numerator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumeratorClr(pub u32);
impl NumeratorClr {
    #[doc = "Numerator"]
    #[must_use]
    #[inline(always)]
    pub const fn num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Numerator"]
    #[inline(always)]
    pub const fn set_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for NumeratorClr {
    #[inline(always)]
    fn default() -> NumeratorClr {
        NumeratorClr(0u64 as u32)
    }
}
impl core::fmt::Debug for NumeratorClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NumeratorClr")
            .field("num", &self.num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NumeratorClr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NumeratorClr {
            num: u32,
        }
        let proxy = NumeratorClr { num: self.num() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Numerator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumeratorSet(pub u32);
impl NumeratorSet {
    #[doc = "Numerator"]
    #[must_use]
    #[inline(always)]
    pub const fn num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Numerator"]
    #[inline(always)]
    pub const fn set_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for NumeratorSet {
    #[inline(always)]
    fn default() -> NumeratorSet {
        NumeratorSet(0u64 as u32)
    }
}
impl core::fmt::Debug for NumeratorSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NumeratorSet")
            .field("num", &self.num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NumeratorSet {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NumeratorSet {
            num: u32,
        }
        let proxy = NumeratorSet { num: self.num() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Numerator Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumeratorTog(pub u32);
impl NumeratorTog {
    #[doc = "Numerator"]
    #[must_use]
    #[inline(always)]
    pub const fn num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Numerator"]
    #[inline(always)]
    pub const fn set_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for NumeratorTog {
    #[inline(always)]
    fn default() -> NumeratorTog {
        NumeratorTog(0u64 as u32)
    }
}
impl core::fmt::Debug for NumeratorTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NumeratorTog")
            .field("num", &self.num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NumeratorTog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NumeratorTog {
            num: u32,
        }
        let proxy = NumeratorTog { num: self.num() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Spread Spectrum Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpreadSpectrum(pub u32);
impl SpreadSpectrum {
    #[doc = "Step"]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Step"]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SpreadSpectrum {
    #[inline(always)]
    fn default() -> SpreadSpectrum {
        SpreadSpectrum(0u64 as u32)
    }
}
impl core::fmt::Debug for SpreadSpectrum {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpreadSpectrum")
            .field("step", &self.step())
            .field("enable", &self.enable())
            .field("stop", &self.stop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpreadSpectrum {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpreadSpectrum {
            step: u16,
            enable: bool,
            stop: u16,
        }
        let proxy = SpreadSpectrum {
            step: self.step(),
            enable: self.enable(),
            stop: self.stop(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Spread Spectrum Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpreadSpectrumClr(pub u32);
impl SpreadSpectrumClr {
    #[doc = "Step"]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Step"]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SpreadSpectrumClr {
    #[inline(always)]
    fn default() -> SpreadSpectrumClr {
        SpreadSpectrumClr(0u64 as u32)
    }
}
impl core::fmt::Debug for SpreadSpectrumClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpreadSpectrumClr")
            .field("step", &self.step())
            .field("enable", &self.enable())
            .field("stop", &self.stop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpreadSpectrumClr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpreadSpectrumClr {
            step: u16,
            enable: bool,
            stop: u16,
        }
        let proxy = SpreadSpectrumClr {
            step: self.step(),
            enable: self.enable(),
            stop: self.stop(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Spread Spectrum Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpreadSpectrumSet(pub u32);
impl SpreadSpectrumSet {
    #[doc = "Step"]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Step"]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SpreadSpectrumSet {
    #[inline(always)]
    fn default() -> SpreadSpectrumSet {
        SpreadSpectrumSet(0u64 as u32)
    }
}
impl core::fmt::Debug for SpreadSpectrumSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpreadSpectrumSet")
            .field("step", &self.step())
            .field("enable", &self.enable())
            .field("stop", &self.stop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpreadSpectrumSet {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpreadSpectrumSet {
            step: u16,
            enable: bool,
            stop: u16,
        }
        let proxy = SpreadSpectrumSet {
            step: self.step(),
            enable: self.enable(),
            stop: self.stop(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional PLL Spread Spectrum Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpreadSpectrumTog(pub u32);
impl SpreadSpectrumTog {
    #[doc = "Step"]
    #[must_use]
    #[inline(always)]
    pub const fn step(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Step"]
    #[inline(always)]
    pub const fn set_step(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SpreadSpectrumTog {
    #[inline(always)]
    fn default() -> SpreadSpectrumTog {
        SpreadSpectrumTog(0u64 as u32)
    }
}
impl core::fmt::Debug for SpreadSpectrumTog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpreadSpectrumTog")
            .field("step", &self.step())
            .field("enable", &self.enable())
            .field("stop", &self.stop())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpreadSpectrumTog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpreadSpectrumTog {
            step: u16,
            enable: bool,
            stop: u16,
        }
        let proxy = SpreadSpectrumTog {
            step: self.step(),
            enable: self.enable(),
            stop: self.stop(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
