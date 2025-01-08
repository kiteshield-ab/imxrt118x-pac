#[doc = "DCDC Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl0(pub u32);
impl Ctrl0 {
    #[doc = "Enable internal count for DCDC_OK timeout"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ok_cnt(&self) -> super::vals::EnableOkCnt {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::EnableOkCnt::from_bits(val as u8)
    }
    #[doc = "Enable internal count for DCDC_OK timeout"]
    #[inline(always)]
    pub const fn set_enable_ok_cnt(&mut self, val: super::vals::EnableOkCnt) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Hold trim input"]
    #[must_use]
    #[inline(always)]
    pub const fn trim_hold(&self) -> super::vals::TrimHold {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::TrimHold::from_bits(val as u8)
    }
    #[doc = "Hold trim input"]
    #[inline(always)]
    pub const fn set_trim_hold(&mut self, val: super::vals::TrimHold) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "IN_BROWNOUT_WARN_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn in_brownout_warn_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "IN_BROWNOUT_WARN_EN"]
    #[inline(always)]
    pub const fn set_in_brownout_warn_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set to 0x1: To improve loading ability under heavy load."]
    #[must_use]
    #[inline(always)]
    pub const fn debug_bits(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x0fff;
        val as u16
    }
    #[doc = "Set to 0x1: To improve loading ability under heavy load."]
    #[inline(always)]
    pub const fn set_debug_bits(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 19usize)) | (((val as u32) & 0x0fff) << 19usize);
    }
    #[doc = "TRG_GPC_EN: used to enable TRG_GPC_* value or not."]
    #[must_use]
    #[inline(always)]
    pub const fn trg_gpc_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "TRG_GPC_EN: used to enable TRG_GPC_* value or not."]
    #[inline(always)]
    pub const fn set_trg_gpc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("enable_ok_cnt", &self.enable_ok_cnt())
            .field("trim_hold", &self.trim_hold())
            .field("in_brownout_warn_en", &self.in_brownout_warn_en())
            .field("debug_bits", &self.debug_bits())
            .field("trg_gpc_en", &self.trg_gpc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl0 {
            enable_ok_cnt: super::vals::EnableOkCnt,
            trim_hold: super::vals::TrimHold,
            in_brownout_warn_en: bool,
            debug_bits: u16,
            trg_gpc_en: bool,
        }
        let proxy = Ctrl0 {
            enable_ok_cnt: self.enable_ok_cnt(),
            trim_hold: self.trim_hold(),
            in_brownout_warn_en: self.in_brownout_warn_en(),
            debug_bits: self.debug_bits(),
            trg_gpc_en: self.trg_gpc_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "CURRENT TARGET VALUE for DCDC ANALOG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CurrentTrg(pub u32);
impl CurrentTrg {
    #[doc = "This value is current value used by DCDC analog"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is current value used by DCDC analog"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "This value is current value used by DCDC analog"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p8ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is current value used by DCDC analog"]
    #[inline(always)]
    pub const fn set_vdd1p8ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Notice: Only if the time of the bit's 1 is too long(more than several seconds) and REG0\\[STS_DC_OK\\]=1, then you can write 1 to clear it"]
    #[must_use]
    #[inline(always)]
    pub const fn dcdc_updating(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Notice: Only if the time of the bit's 1 is too long(more than several seconds) and REG0\\[STS_DC_OK\\]=1, then you can write 1 to clear it"]
    #[inline(always)]
    pub const fn set_dcdc_updating(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_lp_trg(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_lp_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "This value comes from the smaller one between TRG_SW_0 and TRG_SW_1. This bit only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en_1p0(&self) -> super::vals::LpEn1p0 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::LpEn1p0::from_bits(val as u8)
    }
    #[doc = "This value comes from the smaller one between TRG_SW_0 and TRG_SW_1. This bit only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[inline(always)]
    pub const fn set_lp_en_1p0(&mut self, val: super::vals::LpEn1p0) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for CurrentTrg {
    #[inline(always)]
    fn default() -> CurrentTrg {
        CurrentTrg(1051664u64 as u32)
    }
}
impl core::fmt::Debug for CurrentTrg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CurrentTrg")
            .field("vdd1p0ctrl_trg", &self.vdd1p0ctrl_trg())
            .field("vdd1p8ctrl_trg", &self.vdd1p8ctrl_trg())
            .field("dcdc_updating", &self.dcdc_updating())
            .field("vdd1p0ctrl_lp_trg", &self.vdd1p0ctrl_lp_trg())
            .field("lp_en_1p0", &self.lp_en_1p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CurrentTrg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CurrentTrg {
            vdd1p0ctrl_trg: u8,
            vdd1p8ctrl_trg: u8,
            dcdc_updating: bool,
            vdd1p0ctrl_lp_trg: u8,
            lp_en_1p0: super::vals::LpEn1p0,
        }
        let proxy = CurrentTrg {
            vdd1p0ctrl_trg: self.vdd1p0ctrl_trg(),
            vdd1p8ctrl_trg: self.vdd1p8ctrl_trg(),
            dcdc_updating: self.dcdc_updating(),
            vdd1p0ctrl_lp_trg: self.vdd1p0ctrl_lp_trg(),
            lp_en_1p0: self.lp_en_1p0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DCDC Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg0(pub u32);
impl Reg0 {
    #[doc = "Power Down Zero Cross Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_zcd(&self) -> super::vals::PwdZcd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PwdZcd::from_bits(val as u8)
    }
    #[doc = "Power Down Zero Cross Detection"]
    #[inline(always)]
    pub const fn set_pwd_zcd(&mut self, val: super::vals::PwdZcd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Disable Auto Clock Switch"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_auto_clk_switch(&self) -> super::vals::DisableAutoClkSwitch {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DisableAutoClkSwitch::from_bits(val as u8)
    }
    #[doc = "Disable Auto Clock Switch"]
    #[inline(always)]
    pub const fn set_disable_auto_clk_switch(&mut self, val: super::vals::DisableAutoClkSwitch) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Select Clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk(&self) -> super::vals::SelClk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::SelClk::from_bits(val as u8)
    }
    #[doc = "Select Clock"]
    #[inline(always)]
    pub const fn set_sel_clk(&mut self, val: super::vals::SelClk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Power down internal osc"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_osc_int(&self) -> super::vals::PwdOscInt {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::PwdOscInt::from_bits(val as u8)
    }
    #[doc = "Power down internal osc"]
    #[inline(always)]
    pub const fn set_pwd_osc_int(&mut self, val: super::vals::PwdOscInt) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Power down overcurrent detection comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_overcur_det(&self) -> super::vals::PwdOvercurDet {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::PwdOvercurDet::from_bits(val as u8)
    }
    #[doc = "Power down overcurrent detection comparator"]
    #[inline(always)]
    pub const fn set_pwd_overcur_det(&mut self, val: super::vals::PwdOvercurDet) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Overcurrent Trigger Adjust"]
    #[must_use]
    #[inline(always)]
    pub const fn overcur_trig_adj(&self) -> super::vals::OvercurTrigAdj {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::OvercurTrigAdj::from_bits(val as u8)
    }
    #[doc = "Overcurrent Trigger Adjust"]
    #[inline(always)]
    pub const fn set_overcur_trig_adj(&mut self, val: super::vals::OvercurTrigAdj) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Set to \"1\" to power down the low voltage detection comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_cmp_dcdc_in_det(&self) -> super::vals::PwdCmpDcdcInDet {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::PwdCmpDcdcInDet::from_bits(val as u8)
    }
    #[doc = "Set to \"1\" to power down the low voltage detection comparator"]
    #[inline(always)]
    pub const fn set_pwd_cmp_dcdc_in_det(&mut self, val: super::vals::PwdCmpDcdcInDet) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Power Down High Voltage Detection for VDD1P8"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_high_vdd1p8_det(&self) -> super::vals::PwdHighVdd1p8Det {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::PwdHighVdd1p8Det::from_bits(val as u8)
    }
    #[doc = "Power Down High Voltage Detection for VDD1P8"]
    #[inline(always)]
    pub const fn set_pwd_high_vdd1p8_det(&mut self, val: super::vals::PwdHighVdd1p8Det) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Power Down High Voltage Detection for VDD1P0"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_high_vdd1p0_det(&self) -> super::vals::PwdHighVdd1p0Det {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::PwdHighVdd1p0Det::from_bits(val as u8)
    }
    #[doc = "Power Down High Voltage Detection for VDD1P0"]
    #[inline(always)]
    pub const fn set_pwd_high_vdd1p0_det(&mut self, val: super::vals::PwdHighVdd1p0Det) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "power down the out-of-range detection comparator"]
    #[must_use]
    #[inline(always)]
    pub const fn pwd_cmp_offset(&self) -> super::vals::PwdCmpOffset {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::PwdCmpOffset::from_bits(val as u8)
    }
    #[doc = "power down the out-of-range detection comparator"]
    #[inline(always)]
    pub const fn set_pwd_cmp_offset(&mut self, val: super::vals::PwdCmpOffset) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Disable xtalok detection circuit"]
    #[must_use]
    #[inline(always)]
    pub const fn xtalok_disable(&self) -> super::vals::XtalokDisable {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::XtalokDisable::from_bits(val as u8)
    }
    #[doc = "Disable xtalok detection circuit"]
    #[inline(always)]
    pub const fn set_xtalok_disable(&mut self, val: super::vals::XtalokDisable) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "24M XTAL OK"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_24m_ok(&self) -> super::vals::Xtal24mOk {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Xtal24mOk::from_bits(val as u8)
    }
    #[doc = "24M XTAL OK"]
    #[inline(always)]
    pub const fn set_xtal_24m_ok(&mut self, val: super::vals::Xtal24mOk) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "DCDC Output OK"]
    #[must_use]
    #[inline(always)]
    pub const fn sts_dc_ok(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "DCDC Output OK"]
    #[inline(always)]
    pub const fn set_sts_dc_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Reg0 {
    #[inline(always)]
    fn default() -> Reg0 {
        Reg0(67306753u64 as u32)
    }
}
impl core::fmt::Debug for Reg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg0")
            .field("pwd_zcd", &self.pwd_zcd())
            .field("disable_auto_clk_switch", &self.disable_auto_clk_switch())
            .field("sel_clk", &self.sel_clk())
            .field("pwd_osc_int", &self.pwd_osc_int())
            .field("pwd_overcur_det", &self.pwd_overcur_det())
            .field("overcur_trig_adj", &self.overcur_trig_adj())
            .field("pwd_cmp_dcdc_in_det", &self.pwd_cmp_dcdc_in_det())
            .field("pwd_high_vdd1p8_det", &self.pwd_high_vdd1p8_det())
            .field("pwd_high_vdd1p0_det", &self.pwd_high_vdd1p0_det())
            .field("pwd_cmp_offset", &self.pwd_cmp_offset())
            .field("xtalok_disable", &self.xtalok_disable())
            .field("xtal_24m_ok", &self.xtal_24m_ok())
            .field("sts_dc_ok", &self.sts_dc_ok())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Reg0 {
            pwd_zcd: super::vals::PwdZcd,
            disable_auto_clk_switch: super::vals::DisableAutoClkSwitch,
            sel_clk: super::vals::SelClk,
            pwd_osc_int: super::vals::PwdOscInt,
            pwd_overcur_det: super::vals::PwdOvercurDet,
            overcur_trig_adj: super::vals::OvercurTrigAdj,
            pwd_cmp_dcdc_in_det: super::vals::PwdCmpDcdcInDet,
            pwd_high_vdd1p8_det: super::vals::PwdHighVdd1p8Det,
            pwd_high_vdd1p0_det: super::vals::PwdHighVdd1p0Det,
            pwd_cmp_offset: super::vals::PwdCmpOffset,
            xtalok_disable: super::vals::XtalokDisable,
            xtal_24m_ok: super::vals::Xtal24mOk,
            sts_dc_ok: bool,
        }
        let proxy = Reg0 {
            pwd_zcd: self.pwd_zcd(),
            disable_auto_clk_switch: self.disable_auto_clk_switch(),
            sel_clk: self.sel_clk(),
            pwd_osc_int: self.pwd_osc_int(),
            pwd_overcur_det: self.pwd_overcur_det(),
            overcur_trig_adj: self.overcur_trig_adj(),
            pwd_cmp_dcdc_in_det: self.pwd_cmp_dcdc_in_det(),
            pwd_high_vdd1p8_det: self.pwd_high_vdd1p8_det(),
            pwd_high_vdd1p0_det: self.pwd_high_vdd1p0_det(),
            pwd_cmp_offset: self.pwd_cmp_offset(),
            xtalok_disable: self.xtalok_disable(),
            xtal_24m_ok: self.xtal_24m_ok(),
            sts_dc_ok: self.sts_dc_ok(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DCDC Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg1(pub u32);
impl Reg1 {
    #[doc = "Resistor Load of Regulator Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rload_reg_en(&self) -> super::vals::RloadRegEn {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::RloadRegEn::from_bits(val as u8)
    }
    #[doc = "Resistor Load of Regulator Enable"]
    #[inline(always)]
    pub const fn set_rload_reg_en(&mut self, val: super::vals::RloadRegEn) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Trim Bandgap Voltage Trim step is 3mV. 00000 - 0.452V 10000 - 0.5V 11111 - 0.545V"]
    #[must_use]
    #[inline(always)]
    pub const fn vbg_trim(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "Trim Bandgap Voltage Trim step is 3mV. 00000 - 0.452V 10000 - 0.5V 11111 - 0.545V"]
    #[inline(always)]
    pub const fn set_vbg_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "Negative duty cycle limit of DC-DC converter"]
    #[must_use]
    #[inline(always)]
    pub const fn neglimit_in(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x7f;
        val as u8
    }
    #[doc = "Negative duty cycle limit of DC-DC converter"]
    #[inline(always)]
    pub const fn set_neglimit_in(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 13usize)) | (((val as u32) & 0x7f) << 13usize);
    }
    #[doc = "Increase Threshold Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_cm_hst_thresh(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Increase Threshold Detection"]
    #[inline(always)]
    pub const fn set_loopctrl_cm_hst_thresh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Increase Threshold Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_df_hst_thresh(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Increase Threshold Detection"]
    #[inline(always)]
    pub const fn set_loopctrl_df_hst_thresh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Enable hysteresis in switching converter common mode analog comparators"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_en_cm_hyst(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Enable hysteresis in switching converter common mode analog comparators"]
    #[inline(always)]
    pub const fn set_loopctrl_en_cm_hyst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Enable hysteresis in switching converter differential mode analog comparators"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_en_df_hyst(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Enable hysteresis in switching converter differential mode analog comparators"]
    #[inline(always)]
    pub const fn set_loopctrl_en_df_hyst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Reg1 {
    #[inline(always)]
    fn default() -> Reg1 {
        Reg1(30229520u64 as u32)
    }
}
impl core::fmt::Debug for Reg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg1")
            .field("rload_reg_en", &self.rload_reg_en())
            .field("vbg_trim", &self.vbg_trim())
            .field("neglimit_in", &self.neglimit_in())
            .field("loopctrl_cm_hst_thresh", &self.loopctrl_cm_hst_thresh())
            .field("loopctrl_df_hst_thresh", &self.loopctrl_df_hst_thresh())
            .field("loopctrl_en_cm_hyst", &self.loopctrl_en_cm_hyst())
            .field("loopctrl_en_df_hyst", &self.loopctrl_en_df_hyst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Reg1 {
            rload_reg_en: super::vals::RloadRegEn,
            vbg_trim: u8,
            neglimit_in: u8,
            loopctrl_cm_hst_thresh: bool,
            loopctrl_df_hst_thresh: bool,
            loopctrl_en_cm_hyst: bool,
            loopctrl_en_df_hyst: bool,
        }
        let proxy = Reg1 {
            rload_reg_en: self.rload_reg_en(),
            vbg_trim: self.vbg_trim(),
            neglimit_in: self.neglimit_in(),
            loopctrl_cm_hst_thresh: self.loopctrl_cm_hst_thresh(),
            loopctrl_df_hst_thresh: self.loopctrl_df_hst_thresh(),
            loopctrl_en_cm_hyst: self.loopctrl_en_cm_hyst(),
            loopctrl_en_df_hyst: self.loopctrl_en_df_hyst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DCDC Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg2(pub u32);
impl Reg2 {
    #[doc = "Ratio of integral control parameter to proportional control parameter in the switching DC-DC converter, and can be used to optimize efficiency and loop response"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_dc_c(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Ratio of integral control parameter to proportional control parameter in the switching DC-DC converter, and can be used to optimize efficiency and loop response"]
    #[inline(always)]
    pub const fn set_loopctrl_dc_c(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Magnitude of proportional control parameter in the switching DC-DC converter control loop."]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_dc_r(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Magnitude of proportional control parameter in the switching DC-DC converter control loop."]
    #[inline(always)]
    pub const fn set_loopctrl_dc_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_dc_ff(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "Two's complement feed forward step in duty cycle in the switching DC-DC converter"]
    #[inline(always)]
    pub const fn set_loopctrl_dc_ff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "Enable RC Scale"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_en_rcscale(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Enable RC Scale"]
    #[inline(always)]
    pub const fn set_loopctrl_en_rcscale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Increase the threshold detection for RC scale circuit."]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_rcscale_thrsh(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Increase the threshold detection for RC scale circuit."]
    #[inline(always)]
    pub const fn set_loopctrl_rcscale_thrsh(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_hyst_sign(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Invert the sign of the hysteresis in DC-DC analog comparators."]
    #[inline(always)]
    pub const fn set_loopctrl_hyst_sign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "This bit enables the DC-DC to improve efficiency and minimize ripple using the information from the BATTMONITOR_BATT_VAL field"]
    #[must_use]
    #[inline(always)]
    pub const fn battmonitor_en_batadj(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the DC-DC to improve efficiency and minimize ripple using the information from the BATTMONITOR_BATT_VAL field"]
    #[inline(always)]
    pub const fn set_battmonitor_en_batadj(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Software should be configured to place the battery voltage in this register measured with an 8-mV LSB resolution through the ADC"]
    #[must_use]
    #[inline(always)]
    pub const fn battmonitor_batt_val(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Software should be configured to place the battery voltage in this register measured with an 8-mV LSB resolution through the ADC"]
    #[inline(always)]
    pub const fn set_battmonitor_batt_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Set high to enable supply stepping to change only after the differential control loop has toggled as well"]
    #[must_use]
    #[inline(always)]
    pub const fn loopctrl_toggle_dif(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Set high to enable supply stepping to change only after the differential control loop has toggled as well"]
    #[inline(always)]
    pub const fn set_loopctrl_toggle_dif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Reg2 {
    #[inline(always)]
    fn default() -> Reg2 {
        Reg2(34635913u64 as u32)
    }
}
impl core::fmt::Debug for Reg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg2")
            .field("loopctrl_dc_c", &self.loopctrl_dc_c())
            .field("loopctrl_dc_r", &self.loopctrl_dc_r())
            .field("loopctrl_dc_ff", &self.loopctrl_dc_ff())
            .field("loopctrl_en_rcscale", &self.loopctrl_en_rcscale())
            .field("loopctrl_rcscale_thrsh", &self.loopctrl_rcscale_thrsh())
            .field("loopctrl_hyst_sign", &self.loopctrl_hyst_sign())
            .field("battmonitor_en_batadj", &self.battmonitor_en_batadj())
            .field("battmonitor_batt_val", &self.battmonitor_batt_val())
            .field("loopctrl_toggle_dif", &self.loopctrl_toggle_dif())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Reg2 {
            loopctrl_dc_c: u8,
            loopctrl_dc_r: u8,
            loopctrl_dc_ff: u8,
            loopctrl_en_rcscale: u8,
            loopctrl_rcscale_thrsh: bool,
            loopctrl_hyst_sign: bool,
            battmonitor_en_batadj: bool,
            battmonitor_batt_val: u16,
            loopctrl_toggle_dif: bool,
        }
        let proxy = Reg2 {
            loopctrl_dc_c: self.loopctrl_dc_c(),
            loopctrl_dc_r: self.loopctrl_dc_r(),
            loopctrl_dc_ff: self.loopctrl_dc_ff(),
            loopctrl_en_rcscale: self.loopctrl_en_rcscale(),
            loopctrl_rcscale_thrsh: self.loopctrl_rcscale_thrsh(),
            loopctrl_hyst_sign: self.loopctrl_hyst_sign(),
            battmonitor_en_batadj: self.battmonitor_en_batadj(),
            battmonitor_batt_val: self.battmonitor_batt_val(),
            loopctrl_toggle_dif: self.loopctrl_toggle_dif(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DCDC Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reg3(pub u32);
impl Reg3 {
    #[doc = "The voltage on DCDC_IN is lower than 2.8V"]
    #[must_use]
    #[inline(always)]
    pub const fn in_brownout_warn(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "The voltage on DCDC_IN is lower than 2.8V"]
    #[inline(always)]
    pub const fn set_in_brownout_warn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable feed-forward (FF) function that can speed up transient settling."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_ff(&self) -> super::vals::EnableFf {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::EnableFf::from_bits(val as u8)
    }
    #[doc = "Enable feed-forward (FF) function that can speed up transient settling."]
    #[inline(always)]
    pub const fn set_enable_ff(&mut self, val: super::vals::EnableFf) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Disable Pulse Skip"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_pulse_skip(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Pulse Skip"]
    #[inline(always)]
    pub const fn set_disable_pulse_skip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "The DCDC will be idle when out-of-range comparator detects the output voltage is higher than the target by 25mV"]
    #[must_use]
    #[inline(always)]
    pub const fn disable_idle_skip(&self) -> super::vals::DisableIdleSkip {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::DisableIdleSkip::from_bits(val as u8)
    }
    #[doc = "The DCDC will be idle when out-of-range comparator detects the output voltage is higher than the target by 25mV"]
    #[inline(always)]
    pub const fn set_disable_idle_skip(&mut self, val: super::vals::DisableIdleSkip) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "Double the bias current of the comparator for low-voltage detector in LP (low-power) mode."]
    #[must_use]
    #[inline(always)]
    pub const fn double_ibias_cmp_lp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Double the bias current of the comparator for low-voltage detector in LP (low-power) mode."]
    #[inline(always)]
    pub const fn set_double_ibias_cmp_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Select the feedback point of the internal regulator. No need to change this field in user mode."]
    #[must_use]
    #[inline(always)]
    pub const fn reg_fbk_sel(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Select the feedback point of the internal regulator. No need to change this field in user mode."]
    #[inline(always)]
    pub const fn set_reg_fbk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Use half switch FET"]
    #[must_use]
    #[inline(always)]
    pub const fn minpwr_half_fets(&self) -> super::vals::MinpwrHalfFets {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::MinpwrHalfFets::from_bits(val as u8)
    }
    #[doc = "Use half switch FET"]
    #[inline(always)]
    pub const fn set_minpwr_half_fets(&mut self, val: super::vals::MinpwrHalfFets) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "Miscellaneous Delay Timing"]
    #[must_use]
    #[inline(always)]
    pub const fn misc_delay_timing(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Miscellaneous Delay Timing"]
    #[inline(always)]
    pub const fn set_misc_delay_timing(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Disable Step for VDD1P0"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_disable_step(&self) -> super::vals::Vdd1p0ctrlDisableStep {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Vdd1p0ctrlDisableStep::from_bits(val as u8)
    }
    #[doc = "Disable Step for VDD1P0"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_disable_step(&mut self, val: super::vals::Vdd1p0ctrlDisableStep) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Reg3 {
    #[inline(always)]
    fn default() -> Reg3 {
        Reg3(9961472u64 as u32)
    }
}
impl core::fmt::Debug for Reg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reg3")
            .field("in_brownout_warn", &self.in_brownout_warn())
            .field("enable_ff", &self.enable_ff())
            .field("disable_pulse_skip", &self.disable_pulse_skip())
            .field("disable_idle_skip", &self.disable_idle_skip())
            .field("double_ibias_cmp_lp", &self.double_ibias_cmp_lp())
            .field("reg_fbk_sel", &self.reg_fbk_sel())
            .field("minpwr_half_fets", &self.minpwr_half_fets())
            .field("misc_delay_timing", &self.misc_delay_timing())
            .field("vdd1p0ctrl_disable_step", &self.vdd1p0ctrl_disable_step())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reg3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Reg3 {
            in_brownout_warn: bool,
            enable_ff: super::vals::EnableFf,
            disable_pulse_skip: bool,
            disable_idle_skip: super::vals::DisableIdleSkip,
            double_ibias_cmp_lp: bool,
            reg_fbk_sel: u8,
            minpwr_half_fets: super::vals::MinpwrHalfFets,
            misc_delay_timing: bool,
            vdd1p0ctrl_disable_step: super::vals::Vdd1p0ctrlDisableStep,
        }
        let proxy = Reg3 {
            in_brownout_warn: self.in_brownout_warn(),
            enable_ff: self.enable_ff(),
            disable_pulse_skip: self.disable_pulse_skip(),
            disable_idle_skip: self.disable_idle_skip(),
            double_ibias_cmp_lp: self.double_ibias_cmp_lp(),
            reg_fbk_sel: self.reg_fbk_sel(),
            minpwr_half_fets: self.minpwr_half_fets(),
            misc_delay_timing: self.misc_delay_timing(),
            vdd1p0ctrl_disable_step: self.vdd1p0ctrl_disable_step(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRG_0 Authentication Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trg0Authen(pub u32);
impl Trg0Authen {
    #[doc = "Allow user mode write"]
    #[must_use]
    #[inline(always)]
    pub const fn tz_user(&self) -> super::vals::Trg0AuthenTzUser {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Trg0AuthenTzUser::from_bits(val as u8)
    }
    #[doc = "Allow user mode write"]
    #[inline(always)]
    pub const fn set_tz_user(&mut self, val: super::vals::Trg0AuthenTzUser) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Allow non-secure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn tz_ns(&self) -> super::vals::Trg0AuthenTzNs {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Trg0AuthenTzNs::from_bits(val as u8)
    }
    #[doc = "Allow non-secure mode access"]
    #[inline(always)]
    pub const fn set_tz_ns(&mut self, val: super::vals::Trg0AuthenTzNs) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Lock TZ_NS and TZ_USER"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_tz(&self) -> super::vals::Trg0AuthenLockTz {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Trg0AuthenLockTz::from_bits(val as u8)
    }
    #[doc = "Lock TZ_NS and TZ_USER"]
    #[inline(always)]
    pub const fn set_lock_tz(&mut self, val: super::vals::Trg0AuthenLockTz) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "White list lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_list(&self) -> super::vals::Trg0AuthenLockList {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Trg0AuthenLockList::from_bits(val as u8)
    }
    #[doc = "White list lock"]
    #[inline(always)]
    pub const fn set_lock_list(&mut self, val: super::vals::Trg0AuthenLockList) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Domain ID white list"]
    #[must_use]
    #[inline(always)]
    pub const fn white_list(&self) -> super::vals::Trg0AuthenWhiteList {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Trg0AuthenWhiteList::from_bits(val as u16)
    }
    #[doc = "Domain ID white list"]
    #[inline(always)]
    pub const fn set_white_list(&mut self, val: super::vals::Trg0AuthenWhiteList) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trg0Authen {
    #[inline(always)]
    fn default() -> Trg0Authen {
        Trg0Authen(4294901760u64 as u32)
    }
}
impl core::fmt::Debug for Trg0Authen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trg0Authen")
            .field("tz_user", &self.tz_user())
            .field("tz_ns", &self.tz_ns())
            .field("lock_tz", &self.lock_tz())
            .field("lock_list", &self.lock_list())
            .field("white_list", &self.white_list())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trg0Authen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Trg0Authen {
            tz_user: super::vals::Trg0AuthenTzUser,
            tz_ns: super::vals::Trg0AuthenTzNs,
            lock_tz: super::vals::Trg0AuthenLockTz,
            lock_list: super::vals::Trg0AuthenLockList,
            white_list: super::vals::Trg0AuthenWhiteList,
        }
        let proxy = Trg0Authen {
            tz_user: self.tz_user(),
            tz_ns: self.tz_ns(),
            lock_tz: self.lock_tz(),
            lock_list: self.lock_list(),
            white_list: self.white_list(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TRG_1 Authentication Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trg1Authen(pub u32);
impl Trg1Authen {
    #[doc = "Allow user mode write"]
    #[must_use]
    #[inline(always)]
    pub const fn tz_user(&self) -> super::vals::Trg1AuthenTzUser {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Trg1AuthenTzUser::from_bits(val as u8)
    }
    #[doc = "Allow user mode write"]
    #[inline(always)]
    pub const fn set_tz_user(&mut self, val: super::vals::Trg1AuthenTzUser) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Allow non-secure mode access"]
    #[must_use]
    #[inline(always)]
    pub const fn tz_ns(&self) -> super::vals::Trg1AuthenTzNs {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Trg1AuthenTzNs::from_bits(val as u8)
    }
    #[doc = "Allow non-secure mode access"]
    #[inline(always)]
    pub const fn set_tz_ns(&mut self, val: super::vals::Trg1AuthenTzNs) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Lock TZ_NS and TZ_USER"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_tz(&self) -> super::vals::Trg1AuthenLockTz {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Trg1AuthenLockTz::from_bits(val as u8)
    }
    #[doc = "Lock TZ_NS and TZ_USER"]
    #[inline(always)]
    pub const fn set_lock_tz(&mut self, val: super::vals::Trg1AuthenLockTz) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "White list lock"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_list(&self) -> super::vals::Trg1AuthenLockList {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Trg1AuthenLockList::from_bits(val as u8)
    }
    #[doc = "White list lock"]
    #[inline(always)]
    pub const fn set_lock_list(&mut self, val: super::vals::Trg1AuthenLockList) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Domain ID white list"]
    #[must_use]
    #[inline(always)]
    pub const fn white_list(&self) -> super::vals::Trg1AuthenWhiteList {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Trg1AuthenWhiteList::from_bits(val as u16)
    }
    #[doc = "Domain ID white list"]
    #[inline(always)]
    pub const fn set_white_list(&mut self, val: super::vals::Trg1AuthenWhiteList) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Trg1Authen {
    #[inline(always)]
    fn default() -> Trg1Authen {
        Trg1Authen(4294901760u64 as u32)
    }
}
impl core::fmt::Debug for Trg1Authen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Trg1Authen")
            .field("tz_user", &self.tz_user())
            .field("tz_ns", &self.tz_ns())
            .field("lock_tz", &self.lock_tz())
            .field("lock_list", &self.lock_list())
            .field("white_list", &self.white_list())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trg1Authen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Trg1Authen {
            tz_user: super::vals::Trg1AuthenTzUser,
            tz_ns: super::vals::Trg1AuthenTzNs,
            lock_tz: super::vals::Trg1AuthenLockTz,
            lock_list: super::vals::Trg1AuthenLockList,
            white_list: super::vals::Trg1AuthenWhiteList,
        }
        let proxy = Trg1Authen {
            tz_user: self.tz_user(),
            tz_ns: self.tz_ns(),
            lock_tz: self.lock_tz(),
            lock_list: self.lock_list(),
            white_list: self.white_list(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Target GPC Control for CORE 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrgGpc0(pub u32);
impl TrgGpc0 {
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p8ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    #[inline(always)]
    pub const fn set_vdd1p8ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_lp_trg(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_lp_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en_1p0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[inline(always)]
    pub const fn set_lp_en_1p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TrgGpc0 {
    #[inline(always)]
    fn default() -> TrgGpc0 {
        TrgGpc0(1051664u64 as u32)
    }
}
impl core::fmt::Debug for TrgGpc0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrgGpc0")
            .field("vdd1p0ctrl_trg", &self.vdd1p0ctrl_trg())
            .field("vdd1p8ctrl_trg", &self.vdd1p8ctrl_trg())
            .field("vdd1p0ctrl_lp_trg", &self.vdd1p0ctrl_lp_trg())
            .field("lp_en_1p0", &self.lp_en_1p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrgGpc0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrgGpc0 {
            vdd1p0ctrl_trg: u8,
            vdd1p8ctrl_trg: u8,
            vdd1p0ctrl_lp_trg: u8,
            lp_en_1p0: bool,
        }
        let proxy = TrgGpc0 {
            vdd1p0ctrl_trg: self.vdd1p0ctrl_trg(),
            vdd1p8ctrl_trg: self.vdd1p8ctrl_trg(),
            vdd1p0ctrl_lp_trg: self.vdd1p0ctrl_lp_trg(),
            lp_en_1p0: self.lp_en_1p0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Target GPC Control for CORE 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrgGpc1(pub u32);
impl TrgGpc1 {
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p8ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    #[inline(always)]
    pub const fn set_vdd1p8ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_lp_trg(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_lp_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en_1p0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[inline(always)]
    pub const fn set_lp_en_1p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TrgGpc1 {
    #[inline(always)]
    fn default() -> TrgGpc1 {
        TrgGpc1(1051664u64 as u32)
    }
}
impl core::fmt::Debug for TrgGpc1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrgGpc1")
            .field("vdd1p0ctrl_trg", &self.vdd1p0ctrl_trg())
            .field("vdd1p8ctrl_trg", &self.vdd1p8ctrl_trg())
            .field("vdd1p0ctrl_lp_trg", &self.vdd1p0ctrl_lp_trg())
            .field("lp_en_1p0", &self.lp_en_1p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrgGpc1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrgGpc1 {
            vdd1p0ctrl_trg: u8,
            vdd1p8ctrl_trg: u8,
            vdd1p0ctrl_lp_trg: u8,
            lp_en_1p0: bool,
        }
        let proxy = TrgGpc1 {
            vdd1p0ctrl_trg: self.vdd1p0ctrl_trg(),
            vdd1p8ctrl_trg: self.vdd1p8ctrl_trg(),
            vdd1p0ctrl_lp_trg: self.vdd1p0ctrl_lp_trg(),
            lp_en_1p0: self.lp_en_1p0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Target SW Control for CORE 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrgSw0(pub u32);
impl TrgSw0 {
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p8ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    #[inline(always)]
    pub const fn set_vdd1p8ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_lp_trg(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_lp_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en_1p0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[inline(always)]
    pub const fn set_lp_en_1p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TrgSw0 {
    #[inline(always)]
    fn default() -> TrgSw0 {
        TrgSw0(1051664u64 as u32)
    }
}
impl core::fmt::Debug for TrgSw0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrgSw0")
            .field("vdd1p0ctrl_trg", &self.vdd1p0ctrl_trg())
            .field("vdd1p8ctrl_trg", &self.vdd1p8ctrl_trg())
            .field("vdd1p0ctrl_lp_trg", &self.vdd1p0ctrl_lp_trg())
            .field("lp_en_1p0", &self.lp_en_1p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrgSw0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrgSw0 {
            vdd1p0ctrl_trg: u8,
            vdd1p8ctrl_trg: u8,
            vdd1p0ctrl_lp_trg: u8,
            lp_en_1p0: bool,
        }
        let proxy = TrgSw0 {
            vdd1p0ctrl_trg: self.vdd1p0ctrl_trg(),
            vdd1p8ctrl_trg: self.vdd1p8ctrl_trg(),
            vdd1p0ctrl_lp_trg: self.vdd1p0ctrl_lp_trg(),
            lp_en_1p0: self.lp_en_1p0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Target SW Control for CORE 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrgSw1(pub u32);
impl TrgSw1 {
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD1P0 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p8ctrl_trg(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Target value of VDD1P8 in run mode, 25mV each step from 0x00 to 0x1F: 0x1F: 2"]
    #[inline(always)]
    pub const fn set_vdd1p8ctrl_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[must_use]
    #[inline(always)]
    pub const fn vdd1p0ctrl_lp_trg(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "This value is only valid when LP_EN_1P0 =1"]
    #[inline(always)]
    pub const fn set_vdd1p0ctrl_lp_trg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_en_1p0(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "LP_EN_1P0 only controls 1P0. 1P8 is always controlled by VDD1P8CTRL_TRG"]
    #[inline(always)]
    pub const fn set_lp_en_1p0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for TrgSw1 {
    #[inline(always)]
    fn default() -> TrgSw1 {
        TrgSw1(1051664u64 as u32)
    }
}
impl core::fmt::Debug for TrgSw1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrgSw1")
            .field("vdd1p0ctrl_trg", &self.vdd1p0ctrl_trg())
            .field("vdd1p8ctrl_trg", &self.vdd1p8ctrl_trg())
            .field("vdd1p0ctrl_lp_trg", &self.vdd1p0ctrl_lp_trg())
            .field("lp_en_1p0", &self.lp_en_1p0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrgSw1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrgSw1 {
            vdd1p0ctrl_trg: u8,
            vdd1p8ctrl_trg: u8,
            vdd1p0ctrl_lp_trg: u8,
            lp_en_1p0: bool,
        }
        let proxy = TrgSw1 {
            vdd1p0ctrl_trg: self.vdd1p0ctrl_trg(),
            vdd1p8ctrl_trg: self.vdd1p8ctrl_trg(),
            vdd1p0ctrl_lp_trg: self.vdd1p0ctrl_lp_trg(),
            lp_en_1p0: self.lp_en_1p0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
