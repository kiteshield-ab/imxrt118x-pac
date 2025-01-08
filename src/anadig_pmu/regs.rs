#[doc = "PMU_BIAS_CTRL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuBiasCtrl(pub u32);
impl PmuBiasCtrl {
    #[doc = "wb_cfg_1p8"]
    #[must_use]
    #[inline(always)]
    pub const fn wb_cfg_1p8(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "wb_cfg_1p8"]
    #[inline(always)]
    pub const fn set_wb_cfg_1p8(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "wb_vdd_sel_1p8"]
    #[must_use]
    #[inline(always)]
    pub const fn wb_vdd_sel_1p8(&self) -> super::vals::WbVddSel1p8 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::WbVddSel1p8::from_bits(val as u8)
    }
    #[doc = "wb_vdd_sel_1p8"]
    #[inline(always)]
    pub const fn set_wb_vdd_sel_1p8(&mut self, val: super::vals::WbVddSel1p8) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "standby enable bit of fbb m7"]
    #[must_use]
    #[inline(always)]
    pub const fn fbb_m7_stby_en(&self) -> super::vals::FbbM7StbyEn {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::FbbM7StbyEn::from_bits(val as u8)
    }
    #[doc = "standby enable bit of fbb m7"]
    #[inline(always)]
    pub const fn set_fbb_m7_stby_en(&mut self, val: super::vals::FbbM7StbyEn) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "wb_pw_lvl_1p8"]
    #[must_use]
    #[inline(always)]
    pub const fn wb_pw_lvl_1p8(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "wb_pw_lvl_1p8"]
    #[inline(always)]
    pub const fn set_wb_pw_lvl_1p8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "wb_nw_lvl_1p8"]
    #[must_use]
    #[inline(always)]
    pub const fn wb_nw_lvl_1p8(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "wb_nw_lvl_1p8"]
    #[inline(always)]
    pub const fn set_wb_nw_lvl_1p8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for PmuBiasCtrl {
    #[inline(always)]
    fn default() -> PmuBiasCtrl {
        PmuBiasCtrl(32768u64 as u32)
    }
}
impl core::fmt::Debug for PmuBiasCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuBiasCtrl")
            .field("wb_cfg_1p8", &self.wb_cfg_1p8())
            .field("wb_vdd_sel_1p8", &self.wb_vdd_sel_1p8())
            .field("fbb_m7_stby_en", &self.fbb_m7_stby_en())
            .field("wb_pw_lvl_1p8", &self.wb_pw_lvl_1p8())
            .field("wb_nw_lvl_1p8", &self.wb_nw_lvl_1p8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuBiasCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PmuBiasCtrl {
            wb_cfg_1p8: u16,
            wb_vdd_sel_1p8: super::vals::WbVddSel1p8,
            fbb_m7_stby_en: super::vals::FbbM7StbyEn,
            wb_pw_lvl_1p8: u8,
            wb_nw_lvl_1p8: u8,
        }
        let proxy = PmuBiasCtrl {
            wb_cfg_1p8: self.wb_cfg_1p8(),
            wb_vdd_sel_1p8: self.wb_vdd_sel_1p8(),
            fbb_m7_stby_en: self.fbb_m7_stby_en(),
            wb_pw_lvl_1p8: self.wb_pw_lvl_1p8(),
            wb_nw_lvl_1p8: self.wb_nw_lvl_1p8(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PMU_BIAS_CTRL2_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuBiasCtrl2(pub u32);
impl PmuBiasCtrl2 {
    #[doc = "MODSEL_wb_tst_md_1p8"]
    #[must_use]
    #[inline(always)]
    pub const fn wb_pwr_sw_en_1p8(&self) -> super::vals::WbPwrSwEn1p8 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::WbPwrSwEn1p8::from_bits(val as u8)
    }
    #[doc = "MODSEL_wb_tst_md_1p8"]
    #[inline(always)]
    pub const fn set_wb_pwr_sw_en_1p8(&mut self, val: super::vals::WbPwrSwEn1p8) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "wb_adj_1p8"]
    #[must_use]
    #[inline(always)]
    pub const fn wb_adj_1p8(&self) -> super::vals::WbAdj1p8 {
        let val = (self.0 >> 13usize) & 0xff;
        super::vals::WbAdj1p8::from_bits(val as u8)
    }
    #[doc = "wb_adj_1p8"]
    #[inline(always)]
    pub const fn set_wb_adj_1p8(&mut self, val: super::vals::WbAdj1p8) {
        self.0 = (self.0 & !(0xff << 13usize)) | (((val.to_bits() as u32) & 0xff) << 13usize);
    }
    #[doc = "wb_en"]
    #[must_use]
    #[inline(always)]
    pub const fn wb_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "wb_en"]
    #[inline(always)]
    pub const fn set_wb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Digital Output pin."]
    #[must_use]
    #[inline(always)]
    pub const fn wb_ok(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Digital Output pin."]
    #[inline(always)]
    pub const fn set_wb_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for PmuBiasCtrl2 {
    #[inline(always)]
    fn default() -> PmuBiasCtrl2 {
        PmuBiasCtrl2(0u64 as u32)
    }
}
impl core::fmt::Debug for PmuBiasCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuBiasCtrl2")
            .field("wb_pwr_sw_en_1p8", &self.wb_pwr_sw_en_1p8())
            .field("wb_adj_1p8", &self.wb_adj_1p8())
            .field("wb_en", &self.wb_en())
            .field("wb_ok", &self.wb_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuBiasCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PmuBiasCtrl2 {
            wb_pwr_sw_en_1p8: super::vals::WbPwrSwEn1p8,
            wb_adj_1p8: super::vals::WbAdj1p8,
            wb_en: bool,
            wb_ok: bool,
        }
        let proxy = PmuBiasCtrl2 {
            wb_pwr_sw_en_1p8: self.wb_pwr_sw_en_1p8(),
            wb_adj_1p8: self.wb_adj_1p8(),
            wb_en: self.wb_en(),
            wb_ok: self.wb_ok(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PMU_LDO_PLL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuLdoPll(pub u32);
impl PmuLdoPll {
    #[doc = "LDO_PLL_CONTROL_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_pll_control_mode(&self) -> super::vals::LdoPllControlMode {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::LdoPllControlMode::from_bits(val as u8)
    }
    #[doc = "LDO_PLL_CONTROL_MODE"]
    #[inline(always)]
    pub const fn set_ldo_pll_control_mode(&mut self, val: super::vals::LdoPllControlMode) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "standby enable bit of ldopll"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_pll_stby_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "standby enable bit of ldopll"]
    #[inline(always)]
    pub const fn set_ldo_pll_stby_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for PmuLdoPll {
    #[inline(always)]
    fn default() -> PmuLdoPll {
        PmuLdoPll(5u64 as u32)
    }
}
impl core::fmt::Debug for PmuLdoPll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuLdoPll")
            .field("ldo_pll_control_mode", &self.ldo_pll_control_mode())
            .field("ldo_pll_stby_en", &self.ldo_pll_stby_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuLdoPll {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PmuLdoPll {
            ldo_pll_control_mode: super::vals::LdoPllControlMode,
            ldo_pll_stby_en: bool,
        }
        let proxy = PmuLdoPll {
            ldo_pll_control_mode: self.ldo_pll_control_mode(),
            ldo_pll_stby_en: self.ldo_pll_stby_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PMU_POWER_DETECT_CTRL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuPowerDetectCtrl(pub u32);
impl PmuPowerDetectCtrl {
    #[doc = "ckgb_aon1p0"]
    #[must_use]
    #[inline(always)]
    pub const fn ckgb_aon1p0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ckgb_aon1p0"]
    #[inline(always)]
    pub const fn set_ckgb_aon1p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for PmuPowerDetectCtrl {
    #[inline(always)]
    fn default() -> PmuPowerDetectCtrl {
        PmuPowerDetectCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for PmuPowerDetectCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuPowerDetectCtrl")
            .field("ckgb_aon1p0", &self.ckgb_aon1p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuPowerDetectCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PmuPowerDetectCtrl {
            ckgb_aon1p0: bool,
        }
        let proxy = PmuPowerDetectCtrl {
            ckgb_aon1p0: self.ckgb_aon1p0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PMU_REF_CTRL_REGISTER"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuRefCtrl(pub u32);
impl PmuRefCtrl {
    #[doc = "REF_CONTROL_MODE"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_control_mode(&self) -> super::vals::RefControlMode {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::RefControlMode::from_bits(val as u8)
    }
    #[doc = "REF_CONTROL_MODE"]
    #[inline(always)]
    pub const fn set_ref_control_mode(&mut self, val: super::vals::RefControlMode) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "en_pll_vol_ref_buffer"]
    #[must_use]
    #[inline(always)]
    pub const fn en_pll_vol_ref_buffer(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "en_pll_vol_ref_buffer"]
    #[inline(always)]
    pub const fn set_en_pll_vol_ref_buffer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "standby enable bit of reftop"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_stby_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "standby enable bit of reftop"]
    #[inline(always)]
    pub const fn set_ref_stby_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for PmuRefCtrl {
    #[inline(always)]
    fn default() -> PmuRefCtrl {
        PmuRefCtrl(64u64 as u32)
    }
}
impl core::fmt::Debug for PmuRefCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuRefCtrl")
            .field("ref_control_mode", &self.ref_control_mode())
            .field("en_pll_vol_ref_buffer", &self.en_pll_vol_ref_buffer())
            .field("ref_stby_en", &self.ref_stby_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuRefCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PmuRefCtrl {
            ref_control_mode: super::vals::RefControlMode,
            en_pll_vol_ref_buffer: bool,
            ref_stby_en: bool,
        }
        let proxy = PmuRefCtrl {
            ref_control_mode: self.ref_control_mode(),
            en_pll_vol_ref_buffer: self.en_pll_vol_ref_buffer(),
            ref_stby_en: self.ref_stby_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
