#[doc = "BBSM miscellaneous register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbsmMisc(pub u32);
impl BbsmMisc {
    #[doc = "LDO_BBSM_ANA bypass enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bbsm_bypass_en(&self) -> super::vals::BbsmBypassEn {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::BbsmBypassEn::from_bits(val as u8)
    }
    #[doc = "LDO_BBSM_ANA bypass enable"]
    #[inline(always)]
    pub const fn set_bbsm_bypass_en(&mut self, val: super::vals::BbsmBypassEn) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "32K OSC ok flag"]
    #[must_use]
    #[inline(always)]
    pub const fn bbsm_xtal_clk_ok(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "32K OSC ok flag"]
    #[inline(always)]
    pub const fn set_bbsm_xtal_clk_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for BbsmMisc {
    #[inline(always)]
    fn default() -> BbsmMisc {
        BbsmMisc(0u64 as u32)
    }
}
impl core::fmt::Debug for BbsmMisc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbsmMisc")
            .field("bbsm_bypass_en", &self.bbsm_bypass_en())
            .field("bbsm_xtal_clk_ok", &self.bbsm_xtal_clk_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbsmMisc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbsmMisc {
            bbsm_bypass_en: super::vals::BbsmBypassEn,
            bbsm_xtal_clk_ok: bool,
        }
        let proxy = BbsmMisc {
            bbsm_bypass_en: self.bbsm_bypass_en(),
            bbsm_xtal_clk_ok: self.bbsm_xtal_clk_ok(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "BBSM TRIM register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbsmTrim(pub u32);
impl BbsmTrim {
    #[doc = "BBSM core voltage detect trim select"]
    #[must_use]
    #[inline(always)]
    pub const fn bbsm_core_volt_det_trim_sel(&self) -> super::vals::BbsmCoreVoltDetTrimSel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::BbsmCoreVoltDetTrimSel::from_bits(val as u8)
    }
    #[doc = "BBSM core voltage detect trim select"]
    #[inline(always)]
    pub const fn set_bbsm_core_volt_det_trim_sel(
        &mut self,
        val: super::vals::BbsmCoreVoltDetTrimSel,
    ) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "BBSM core voltage detect trim"]
    #[must_use]
    #[inline(always)]
    pub const fn bbsm_core_volt_det_trim(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "BBSM core voltage detect trim"]
    #[inline(always)]
    pub const fn set_bbsm_core_volt_det_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "BBSM OSC load capacitor trim select"]
    #[must_use]
    #[inline(always)]
    pub const fn bbsm_cap_trim_sel(&self) -> super::vals::BbsmCapTrimSel {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::BbsmCapTrimSel::from_bits(val as u8)
    }
    #[doc = "BBSM OSC load capacitor trim select"]
    #[inline(always)]
    pub const fn set_bbsm_cap_trim_sel(&mut self, val: super::vals::BbsmCapTrimSel) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "BBSM OSC load capacitor trim"]
    #[must_use]
    #[inline(always)]
    pub const fn bbsm_osc_cap_trim(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "BBSM OSC load capacitor trim"]
    #[inline(always)]
    pub const fn set_bbsm_osc_cap_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for BbsmTrim {
    #[inline(always)]
    fn default() -> BbsmTrim {
        BbsmTrim(0u64 as u32)
    }
}
impl core::fmt::Debug for BbsmTrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbsmTrim")
            .field(
                "bbsm_core_volt_det_trim_sel",
                &self.bbsm_core_volt_det_trim_sel(),
            )
            .field("bbsm_core_volt_det_trim", &self.bbsm_core_volt_det_trim())
            .field("bbsm_cap_trim_sel", &self.bbsm_cap_trim_sel())
            .field("bbsm_osc_cap_trim", &self.bbsm_osc_cap_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbsmTrim {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BbsmTrim {
            bbsm_core_volt_det_trim_sel: super::vals::BbsmCoreVoltDetTrimSel,
            bbsm_core_volt_det_trim: u8,
            bbsm_cap_trim_sel: super::vals::BbsmCapTrimSel,
            bbsm_osc_cap_trim: u8,
        }
        let proxy = BbsmTrim {
            bbsm_core_volt_det_trim_sel: self.bbsm_core_volt_det_trim_sel(),
            bbsm_core_volt_det_trim: self.bbsm_core_volt_det_trim(),
            bbsm_cap_trim_sel: self.bbsm_cap_trim_sel(),
            bbsm_osc_cap_trim: self.bbsm_osc_cap_trim(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
