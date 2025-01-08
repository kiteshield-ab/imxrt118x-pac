#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc = "Divide value for ref_clk to generate slow_clk."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_clk_div(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Divide value for ref_clk to generate slow_clk."]
    #[inline(always)]
    pub const fn set_ref_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
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
            .field("ref_clk_div", &self.ref_clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0 {
            ref_clk_div: u8,
        }
        let proxy = Ctrl0 {
            ref_clk_div: self.ref_clk_div(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Clr(pub u32);
impl Ctrl0Clr {
    #[doc = "Divide value for ref_clk to generate slow_clk."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_clk_div(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Divide value for ref_clk to generate slow_clk."]
    #[inline(always)]
    pub const fn set_ref_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
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
            .field("ref_clk_div", &self.ref_clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Clr {
            ref_clk_div: u8,
        }
        let proxy = Ctrl0Clr {
            ref_clk_div: self.ref_clk_div(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Set(pub u32);
impl Ctrl0Set {
    #[doc = "Divide value for ref_clk to generate slow_clk."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_clk_div(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Divide value for ref_clk to generate slow_clk."]
    #[inline(always)]
    pub const fn set_ref_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
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
            .field("ref_clk_div", &self.ref_clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Set {
            ref_clk_div: u8,
        }
        let proxy = Ctrl0Set {
            ref_clk_div: self.ref_clk_div(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0Tog(pub u32);
impl Ctrl0Tog {
    #[doc = "Divide value for ref_clk to generate slow_clk."]
    #[must_use]
    #[inline(always)]
    pub const fn ref_clk_div(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Divide value for ref_clk to generate slow_clk."]
    #[inline(always)]
    pub const fn set_ref_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
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
            .field("ref_clk_div", &self.ref_clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0Tog {
            ref_clk_div: u8,
        }
        let proxy = Ctrl0Tog {
            ref_clk_div: self.ref_clk_div(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1(pub u32);
impl Ctrl1 {
    #[doc = "Negative hysteresis value for the tuned clock."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value for the tuned clock."]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Positive hysteresis value for the tuned clock."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value for the tuned clock."]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Target count for the fast clock."]
    #[must_use]
    #[inline(always)]
    pub const fn target_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Target count for the fast clock."]
    #[inline(always)]
    pub const fn set_target_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl1 {
    #[inline(always)]
    fn default() -> Ctrl1 {
        Ctrl1(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1")
            .field("hyst_minus", &self.hyst_minus())
            .field("hyst_plus", &self.hyst_plus())
            .field("target_count", &self.target_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1 {
            hyst_minus: u8,
            hyst_plus: u8,
            target_count: u16,
        }
        let proxy = Ctrl1 {
            hyst_minus: self.hyst_minus(),
            hyst_plus: self.hyst_plus(),
            target_count: self.target_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Clr(pub u32);
impl Ctrl1Clr {
    #[doc = "Negative hysteresis value for the tuned clock."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value for the tuned clock."]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Positive hysteresis value for the tuned clock."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value for the tuned clock."]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Target count for the fast clock."]
    #[must_use]
    #[inline(always)]
    pub const fn target_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Target count for the fast clock."]
    #[inline(always)]
    pub const fn set_target_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl1Clr {
    #[inline(always)]
    fn default() -> Ctrl1Clr {
        Ctrl1Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl1Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1Clr")
            .field("hyst_minus", &self.hyst_minus())
            .field("hyst_plus", &self.hyst_plus())
            .field("target_count", &self.target_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1Clr {
            hyst_minus: u8,
            hyst_plus: u8,
            target_count: u16,
        }
        let proxy = Ctrl1Clr {
            hyst_minus: self.hyst_minus(),
            hyst_plus: self.hyst_plus(),
            target_count: self.target_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Set(pub u32);
impl Ctrl1Set {
    #[doc = "Negative hysteresis value for the tuned clock."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value for the tuned clock."]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Positive hysteresis value for the tuned clock."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value for the tuned clock."]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Target count for the fast clock."]
    #[must_use]
    #[inline(always)]
    pub const fn target_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Target count for the fast clock."]
    #[inline(always)]
    pub const fn set_target_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl1Set {
    #[inline(always)]
    fn default() -> Ctrl1Set {
        Ctrl1Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl1Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1Set")
            .field("hyst_minus", &self.hyst_minus())
            .field("hyst_plus", &self.hyst_plus())
            .field("target_count", &self.target_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1Set {
            hyst_minus: u8,
            hyst_plus: u8,
            target_count: u16,
        }
        let proxy = Ctrl1Set {
            hyst_minus: self.hyst_minus(),
            hyst_plus: self.hyst_plus(),
            target_count: self.target_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl1Tog(pub u32);
impl Ctrl1Tog {
    #[doc = "Negative hysteresis value for the tuned clock."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_minus(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Negative hysteresis value for the tuned clock."]
    #[inline(always)]
    pub const fn set_hyst_minus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Positive hysteresis value for the tuned clock."]
    #[must_use]
    #[inline(always)]
    pub const fn hyst_plus(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Positive hysteresis value for the tuned clock."]
    #[inline(always)]
    pub const fn set_hyst_plus(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Target count for the fast clock."]
    #[must_use]
    #[inline(always)]
    pub const fn target_count(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Target count for the fast clock."]
    #[inline(always)]
    pub const fn set_target_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl1Tog {
    #[inline(always)]
    fn default() -> Ctrl1Tog {
        Ctrl1Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl1Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl1Tog")
            .field("hyst_minus", &self.hyst_minus())
            .field("hyst_plus", &self.hyst_plus())
            .field("target_count", &self.target_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl1Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl1Tog {
            hyst_minus: u8,
            hyst_plus: u8,
            target_count: u16,
        }
        let proxy = Ctrl1Tog {
            hyst_minus: self.hyst_minus(),
            hyst_plus: self.hyst_plus(),
            target_count: self.target_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2(pub u32);
impl Ctrl2 {
    #[doc = "Inverse tuning direction."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_inv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Inverse tuning direction."]
    #[inline(always)]
    pub const fn set_tune_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bypass the tuning logic"]
    #[must_use]
    #[inline(always)]
    pub const fn tune_byp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the tuning logic"]
    #[inline(always)]
    pub const fn set_tune_byp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    #[inline(always)]
    pub const fn set_tune_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start/Stop tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_start(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Start/Stop tuning."]
    #[inline(always)]
    pub const fn set_tune_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Program the oscillator frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_tune_val(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Program the oscillator frequency."]
    #[inline(always)]
    pub const fn set_osc_tune_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ctrl2 {
    #[inline(always)]
    fn default() -> Ctrl2 {
        Ctrl2(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2")
            .field("tune_inv", &self.tune_inv())
            .field("tune_byp", &self.tune_byp())
            .field("tune_en", &self.tune_en())
            .field("tune_start", &self.tune_start())
            .field("osc_tune_val", &self.osc_tune_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl2 {
            tune_inv: bool,
            tune_byp: bool,
            tune_en: bool,
            tune_start: bool,
            osc_tune_val: u8,
        }
        let proxy = Ctrl2 {
            tune_inv: self.tune_inv(),
            tune_byp: self.tune_byp(),
            tune_en: self.tune_en(),
            tune_start: self.tune_start(),
            osc_tune_val: self.osc_tune_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2Clr(pub u32);
impl Ctrl2Clr {
    #[doc = "Inverse tuning direction."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_inv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Inverse tuning direction."]
    #[inline(always)]
    pub const fn set_tune_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bypass the tuning logic"]
    #[must_use]
    #[inline(always)]
    pub const fn tune_byp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the tuning logic"]
    #[inline(always)]
    pub const fn set_tune_byp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    #[inline(always)]
    pub const fn set_tune_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start/Stop tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_start(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Start/Stop tuning."]
    #[inline(always)]
    pub const fn set_tune_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Program the oscillator frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_tune_val(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Program the oscillator frequency."]
    #[inline(always)]
    pub const fn set_osc_tune_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ctrl2Clr {
    #[inline(always)]
    fn default() -> Ctrl2Clr {
        Ctrl2Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl2Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2Clr")
            .field("tune_inv", &self.tune_inv())
            .field("tune_byp", &self.tune_byp())
            .field("tune_en", &self.tune_en())
            .field("tune_start", &self.tune_start())
            .field("osc_tune_val", &self.osc_tune_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl2Clr {
            tune_inv: bool,
            tune_byp: bool,
            tune_en: bool,
            tune_start: bool,
            osc_tune_val: u8,
        }
        let proxy = Ctrl2Clr {
            tune_inv: self.tune_inv(),
            tune_byp: self.tune_byp(),
            tune_en: self.tune_en(),
            tune_start: self.tune_start(),
            osc_tune_val: self.osc_tune_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2Set(pub u32);
impl Ctrl2Set {
    #[doc = "Inverse tuning direction."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_inv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Inverse tuning direction."]
    #[inline(always)]
    pub const fn set_tune_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bypass the tuning logic"]
    #[must_use]
    #[inline(always)]
    pub const fn tune_byp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the tuning logic"]
    #[inline(always)]
    pub const fn set_tune_byp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    #[inline(always)]
    pub const fn set_tune_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start/Stop tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_start(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Start/Stop tuning."]
    #[inline(always)]
    pub const fn set_tune_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Program the oscillator frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_tune_val(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Program the oscillator frequency."]
    #[inline(always)]
    pub const fn set_osc_tune_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ctrl2Set {
    #[inline(always)]
    fn default() -> Ctrl2Set {
        Ctrl2Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl2Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2Set")
            .field("tune_inv", &self.tune_inv())
            .field("tune_byp", &self.tune_byp())
            .field("tune_en", &self.tune_en())
            .field("tune_start", &self.tune_start())
            .field("osc_tune_val", &self.osc_tune_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl2Set {
            tune_inv: bool,
            tune_byp: bool,
            tune_en: bool,
            tune_start: bool,
            osc_tune_val: u8,
        }
        let proxy = Ctrl2Set {
            tune_inv: self.tune_inv(),
            tune_byp: self.tune_byp(),
            tune_en: self.tune_en(),
            tune_start: self.tune_start(),
            osc_tune_val: self.osc_tune_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl2Tog(pub u32);
impl Ctrl2Tog {
    #[doc = "Inverse tuning direction."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_inv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Inverse tuning direction."]
    #[inline(always)]
    pub const fn set_tune_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Bypass the tuning logic"]
    #[must_use]
    #[inline(always)]
    pub const fn tune_byp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass the tuning logic"]
    #[inline(always)]
    pub const fn set_tune_byp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Freeze/Unfreeze the tuning value."]
    #[inline(always)]
    pub const fn set_tune_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Start/Stop tuning."]
    #[must_use]
    #[inline(always)]
    pub const fn tune_start(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Start/Stop tuning."]
    #[inline(always)]
    pub const fn set_tune_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Program the oscillator frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn osc_tune_val(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Program the oscillator frequency."]
    #[inline(always)]
    pub const fn set_osc_tune_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Ctrl2Tog {
    #[inline(always)]
    fn default() -> Ctrl2Tog {
        Ctrl2Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl2Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl2Tog")
            .field("tune_inv", &self.tune_inv())
            .field("tune_byp", &self.tune_byp())
            .field("tune_en", &self.tune_en())
            .field("tune_start", &self.tune_start())
            .field("osc_tune_val", &self.osc_tune_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl2Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl2Tog {
            tune_inv: bool,
            tune_byp: bool,
            tune_en: bool,
            tune_start: bool,
            osc_tune_val: u8,
        }
        let proxy = Ctrl2Tog {
            tune_inv: self.tune_inv(),
            tune_byp: self.tune_byp(),
            tune_en: self.tune_en(),
            tune_start: self.tune_start(),
            osc_tune_val: self.osc_tune_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3(pub u32);
impl Ctrl3 {
    #[doc = "Clear the error flag CLK1M_ERR"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_err(&self) -> super::vals::ClrErr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ClrErr::from_bits(val as u8)
    }
    #[doc = "Clear the error flag CLK1M_ERR"]
    #[inline(always)]
    pub const fn set_clr_err(&mut self, val: super::vals::ClrErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Disable clk_1m_out."]
    #[must_use]
    #[inline(always)]
    pub const fn en_1m_clk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1: Disable clk_1m_out."]
    #[inline(always)]
    pub const fn set_en_1m_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Select free/locked 1MHz output"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_1m_clk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Select free/locked 1MHz output"]
    #[inline(always)]
    pub const fn set_mux_1m_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Count for the locked clk_1m_out."]
    #[must_use]
    #[inline(always)]
    pub const fn count_1m_clk(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Count for the locked clk_1m_out."]
    #[inline(always)]
    pub const fn set_count_1m_clk(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl3 {
    #[inline(always)]
    fn default() -> Ctrl3 {
        Ctrl3(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl3")
            .field("clr_err", &self.clr_err())
            .field("en_1m_clk", &self.en_1m_clk())
            .field("mux_1m_clk", &self.mux_1m_clk())
            .field("count_1m_clk", &self.count_1m_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl3 {
            clr_err: super::vals::ClrErr,
            en_1m_clk: bool,
            mux_1m_clk: bool,
            count_1m_clk: u16,
        }
        let proxy = Ctrl3 {
            clr_err: self.clr_err(),
            en_1m_clk: self.en_1m_clk(),
            mux_1m_clk: self.mux_1m_clk(),
            count_1m_clk: self.count_1m_clk(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3Clr(pub u32);
impl Ctrl3Clr {
    #[doc = "Clear the error flag CLK1M_ERR"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the error flag CLK1M_ERR"]
    #[inline(always)]
    pub const fn set_clr_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Disable clk_1m_out."]
    #[must_use]
    #[inline(always)]
    pub const fn en_1m_clk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1: Disable clk_1m_out."]
    #[inline(always)]
    pub const fn set_en_1m_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Select free/locked 1MHz output"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_1m_clk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Select free/locked 1MHz output"]
    #[inline(always)]
    pub const fn set_mux_1m_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Count for the locked clk_1m_out."]
    #[must_use]
    #[inline(always)]
    pub const fn count_1m_clk(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Count for the locked clk_1m_out."]
    #[inline(always)]
    pub const fn set_count_1m_clk(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl3Clr {
    #[inline(always)]
    fn default() -> Ctrl3Clr {
        Ctrl3Clr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl3Clr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl3Clr")
            .field("clr_err", &self.clr_err())
            .field("en_1m_clk", &self.en_1m_clk())
            .field("mux_1m_clk", &self.mux_1m_clk())
            .field("count_1m_clk", &self.count_1m_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl3Clr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl3Clr {
            clr_err: bool,
            en_1m_clk: bool,
            mux_1m_clk: bool,
            count_1m_clk: u16,
        }
        let proxy = Ctrl3Clr {
            clr_err: self.clr_err(),
            en_1m_clk: self.en_1m_clk(),
            mux_1m_clk: self.mux_1m_clk(),
            count_1m_clk: self.count_1m_clk(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3Set(pub u32);
impl Ctrl3Set {
    #[doc = "Clear the error flag CLK1M_ERR"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the error flag CLK1M_ERR"]
    #[inline(always)]
    pub const fn set_clr_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Disable clk_1m_out."]
    #[must_use]
    #[inline(always)]
    pub const fn en_1m_clk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1: Disable clk_1m_out."]
    #[inline(always)]
    pub const fn set_en_1m_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Select free/locked 1MHz output"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_1m_clk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Select free/locked 1MHz output"]
    #[inline(always)]
    pub const fn set_mux_1m_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Count for the locked clk_1m_out."]
    #[must_use]
    #[inline(always)]
    pub const fn count_1m_clk(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Count for the locked clk_1m_out."]
    #[inline(always)]
    pub const fn set_count_1m_clk(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl3Set {
    #[inline(always)]
    fn default() -> Ctrl3Set {
        Ctrl3Set(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl3Set {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl3Set")
            .field("clr_err", &self.clr_err())
            .field("en_1m_clk", &self.en_1m_clk())
            .field("mux_1m_clk", &self.mux_1m_clk())
            .field("count_1m_clk", &self.count_1m_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl3Set {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl3Set {
            clr_err: bool,
            en_1m_clk: bool,
            mux_1m_clk: bool,
            count_1m_clk: u16,
        }
        let proxy = Ctrl3Set {
            clr_err: self.clr_err(),
            en_1m_clk: self.en_1m_clk(),
            mux_1m_clk: self.mux_1m_clk(),
            count_1m_clk: self.count_1m_clk(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Control Register CTRL3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl3Tog(pub u32);
impl Ctrl3Tog {
    #[doc = "Clear the error flag CLK1M_ERR"]
    #[must_use]
    #[inline(always)]
    pub const fn clr_err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the error flag CLK1M_ERR"]
    #[inline(always)]
    pub const fn set_clr_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Disable clk_1m_out."]
    #[must_use]
    #[inline(always)]
    pub const fn en_1m_clk(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1: Disable clk_1m_out."]
    #[inline(always)]
    pub const fn set_en_1m_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Select free/locked 1MHz output"]
    #[must_use]
    #[inline(always)]
    pub const fn mux_1m_clk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Select free/locked 1MHz output"]
    #[inline(always)]
    pub const fn set_mux_1m_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Count for the locked clk_1m_out."]
    #[must_use]
    #[inline(always)]
    pub const fn count_1m_clk(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Count for the locked clk_1m_out."]
    #[inline(always)]
    pub const fn set_count_1m_clk(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl3Tog {
    #[inline(always)]
    fn default() -> Ctrl3Tog {
        Ctrl3Tog(0u64 as u32)
    }
}
impl core::fmt::Debug for Ctrl3Tog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl3Tog")
            .field("clr_err", &self.clr_err())
            .field("en_1m_clk", &self.en_1m_clk())
            .field("mux_1m_clk", &self.mux_1m_clk())
            .field("count_1m_clk", &self.count_1m_clk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl3Tog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl3Tog {
            clr_err: bool,
            en_1m_clk: bool,
            mux_1m_clk: bool,
            count_1m_clk: u16,
        }
        let proxy = Ctrl3Tog {
            clr_err: self.clr_err(),
            en_1m_clk: self.en_1m_clk(),
            mux_1m_clk: self.mux_1m_clk(),
            count_1m_clk: self.count_1m_clk(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat0(pub u32);
impl Stat0 {
    #[doc = "Error flag for clk_1m_locked."]
    #[must_use]
    #[inline(always)]
    pub const fn clk1m_err(&self) -> super::vals::Clk1mErr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Clk1mErr::from_bits(val as u8)
    }
    #[doc = "Error flag for clk_1m_locked."]
    #[inline(always)]
    pub const fn set_clk1m_err(&mut self, val: super::vals::Clk1mErr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Stat0 {
    #[inline(always)]
    fn default() -> Stat0 {
        Stat0(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat0")
            .field("clk1m_err", &self.clk1m_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat0 {
            clk1m_err: super::vals::Clk1mErr,
        }
        let proxy = Stat0 {
            clk1m_err: self.clk1m_err(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat1(pub u32);
impl Stat1 {
    #[doc = "Current count for the fast clock."]
    #[must_use]
    #[inline(always)]
    pub const fn curr_count_val(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Current count for the fast clock."]
    #[inline(always)]
    pub const fn set_curr_count_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Stat1 {
    #[inline(always)]
    fn default() -> Stat1 {
        Stat1(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat1")
            .field("curr_count_val", &self.curr_count_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat1 {
            curr_count_val: u16,
        }
        let proxy = Stat1 {
            curr_count_val: self.curr_count_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Analog Status Register STAT2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat2(pub u32);
impl Stat2 {
    #[doc = "Current tuning value used by oscillator."]
    #[must_use]
    #[inline(always)]
    pub const fn curr_osc_tune_val(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Current tuning value used by oscillator."]
    #[inline(always)]
    pub const fn set_curr_osc_tune_val(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Stat2 {
    #[inline(always)]
    fn default() -> Stat2 {
        Stat2(0u64 as u32)
    }
}
impl core::fmt::Debug for Stat2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat2")
            .field("curr_osc_tune_val", &self.curr_osc_tune_val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat2 {
            curr_osc_tune_val: u8,
        }
        let proxy = Stat2 {
            curr_osc_tune_val: self.curr_osc_tune_val(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
