#[doc = "AXI bus attribute configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AxiAttrCfg(pub u32);
impl AxiAttrCfg {
    #[doc = "uSDHC1 block cacheable attribute value of AXI read transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn arcache_usdhc1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "uSDHC1 block cacheable attribute value of AXI read transactions"]
    #[inline(always)]
    pub const fn set_arcache_usdhc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "uSDHC1 block cacheable attribute value of AXI write transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn awcache_usdhc1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "uSDHC1 block cacheable attribute value of AXI write transactions"]
    #[inline(always)]
    pub const fn set_awcache_usdhc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "uSDHC2 block cacheable attribute value of AXI read transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn arcache_usdhc2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "uSDHC2 block cacheable attribute value of AXI read transactions"]
    #[inline(always)]
    pub const fn set_arcache_usdhc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "uSDHC2 block cacheable attribute value of AXI write transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn awcache_usdhc2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "uSDHC2 block cacheable attribute value of AXI write transactions"]
    #[inline(always)]
    pub const fn set_awcache_usdhc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "USB block cacheable attribute value of AXI read transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn arcache_usb(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "USB block cacheable attribute value of AXI read transactions"]
    #[inline(always)]
    pub const fn set_arcache_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "USB block cacheable attribute value of AXI write transactions"]
    #[must_use]
    #[inline(always)]
    pub const fn awcache_usb(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "USB block cacheable attribute value of AXI write transactions"]
    #[inline(always)]
    pub const fn set_awcache_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for AxiAttrCfg {
    #[inline(always)]
    fn default() -> AxiAttrCfg {
        AxiAttrCfg(7u64 as u32)
    }
}
impl core::fmt::Debug for AxiAttrCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AxiAttrCfg")
            .field("arcache_usdhc1", &self.arcache_usdhc1())
            .field("awcache_usdhc1", &self.awcache_usdhc1())
            .field("arcache_usdhc2", &self.arcache_usdhc2())
            .field("awcache_usdhc2", &self.awcache_usdhc2())
            .field("arcache_usb", &self.arcache_usb())
            .field("awcache_usb", &self.awcache_usb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AxiAttrCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AxiAttrCfg {
            arcache_usdhc1: bool,
            awcache_usdhc1: bool,
            arcache_usdhc2: bool,
            awcache_usdhc2: bool,
            arcache_usb: bool,
            awcache_usb: bool,
        }
        let proxy = AxiAttrCfg {
            arcache_usdhc1: self.arcache_usdhc1(),
            awcache_usdhc1: self.awcache_usdhc1(),
            arcache_usdhc2: self.arcache_usdhc2(),
            awcache_usdhc2: self.awcache_usdhc2(),
            arcache_usb: self.arcache_usb(),
            awcache_usb: self.awcache_usb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DEXSC error response configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DexscErr(pub u32);
impl DexscErr {
    #[doc = "Exclusive error response enable"]
    #[must_use]
    #[inline(always)]
    pub const fn exc_err_resp_en(&self) -> super::vals::ExcErrRespEn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ExcErrRespEn::from_bits(val as u8)
    }
    #[doc = "Exclusive error response enable"]
    #[inline(always)]
    pub const fn set_exc_err_resp_en(&mut self, val: super::vals::ExcErrRespEn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Lock bit of EXC_ERR_RESP_EN"]
    #[must_use]
    #[inline(always)]
    pub const fn lock_exc_err_resp_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Lock bit of EXC_ERR_RESP_EN"]
    #[inline(always)]
    pub const fn set_lock_exc_err_resp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for DexscErr {
    #[inline(always)]
    fn default() -> DexscErr {
        DexscErr(0u64 as u32)
    }
}
impl core::fmt::Debug for DexscErr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DexscErr")
            .field("exc_err_resp_en", &self.exc_err_resp_en())
            .field("lock_exc_err_resp_en", &self.lock_exc_err_resp_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DexscErr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DexscErr {
            exc_err_resp_en: super::vals::ExcErrRespEn,
            lock_exc_err_resp_en: bool,
        }
        let proxy = DexscErr {
            exc_err_resp_en: self.exc_err_resp_en(),
            lock_exc_err_resp_en: self.lock_exc_err_resp_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EtherCAT miscellaneous configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcatMiscCfg(pub u32);
impl EcatMiscCfg {
    #[doc = "RMII mode selection for EtherCAT port 0"]
    #[must_use]
    #[inline(always)]
    pub const fn rmii_sel0(&self) -> super::vals::RmiiSel0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RmiiSel0::from_bits(val as u8)
    }
    #[doc = "RMII mode selection for EtherCAT port 0"]
    #[inline(always)]
    pub const fn set_rmii_sel0(&mut self, val: super::vals::RmiiSel0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "RMII mode selection for EtherCAT port 1"]
    #[must_use]
    #[inline(always)]
    pub const fn rmii_sel1(&self) -> super::vals::RmiiSel1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RmiiSel1::from_bits(val as u8)
    }
    #[doc = "RMII mode selection for EtherCAT port 1"]
    #[inline(always)]
    pub const fn set_rmii_sel1(&mut self, val: super::vals::RmiiSel1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Global enable of EtherCAT"]
    #[must_use]
    #[inline(always)]
    pub const fn glb_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Global enable of EtherCAT"]
    #[inline(always)]
    pub const fn set_glb_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Global reset of EtherCAT"]
    #[must_use]
    #[inline(always)]
    pub const fn glb_rst(&self) -> super::vals::GlbRst {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::GlbRst::from_bits(val as u8)
    }
    #[doc = "Global reset of EtherCAT"]
    #[inline(always)]
    pub const fn set_glb_rst(&mut self, val: super::vals::GlbRst) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "RMII Port0 REF_CLK direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn rmii_ref_clk_dir0(&self) -> super::vals::RmiiRefClkDir0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::RmiiRefClkDir0::from_bits(val as u8)
    }
    #[doc = "RMII Port0 REF_CLK direction control"]
    #[inline(always)]
    pub const fn set_rmii_ref_clk_dir0(&mut self, val: super::vals::RmiiRefClkDir0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "RMII Port1 REF_CLK direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn rmii_ref_clk_dir1(&self) -> super::vals::RmiiRefClkDir1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::RmiiRefClkDir1::from_bits(val as u8)
    }
    #[doc = "RMII Port1 REF_CLK direction control"]
    #[inline(always)]
    pub const fn set_rmii_ref_clk_dir1(&mut self, val: super::vals::RmiiRefClkDir1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "EtherCAT PHY_OFFSET"]
    #[must_use]
    #[inline(always)]
    pub const fn phy_offset(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "EtherCAT PHY_OFFSET"]
    #[inline(always)]
    pub const fn set_phy_offset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "EtherCAT PHY_OFFSET_VEC"]
    #[must_use]
    #[inline(always)]
    pub const fn phy_offset_vec(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[doc = "EtherCAT PHY_OFFSET_VEC"]
    #[inline(always)]
    pub const fn set_phy_offset_vec(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[doc = "EtherCAT EEPROM SIZE OPTION"]
    #[must_use]
    #[inline(always)]
    pub const fn eeprom_size_option(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "EtherCAT EEPROM SIZE OPTION"]
    #[inline(always)]
    pub const fn set_eeprom_size_option(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for EcatMiscCfg {
    #[inline(always)]
    fn default() -> EcatMiscCfg {
        EcatMiscCfg(8u64 as u32)
    }
}
impl core::fmt::Debug for EcatMiscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EcatMiscCfg")
            .field("rmii_sel0", &self.rmii_sel0())
            .field("rmii_sel1", &self.rmii_sel1())
            .field("glb_en", &self.glb_en())
            .field("glb_rst", &self.glb_rst())
            .field("rmii_ref_clk_dir0", &self.rmii_ref_clk_dir0())
            .field("rmii_ref_clk_dir1", &self.rmii_ref_clk_dir1())
            .field("phy_offset", &self.phy_offset())
            .field("phy_offset_vec", &self.phy_offset_vec())
            .field("eeprom_size_option", &self.eeprom_size_option())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EcatMiscCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EcatMiscCfg {
            rmii_sel0: super::vals::RmiiSel0,
            rmii_sel1: super::vals::RmiiSel1,
            glb_en: bool,
            glb_rst: super::vals::GlbRst,
            rmii_ref_clk_dir0: super::vals::RmiiRefClkDir0,
            rmii_ref_clk_dir1: super::vals::RmiiRefClkDir1,
            phy_offset: bool,
            phy_offset_vec: u8,
            eeprom_size_option: bool,
        }
        let proxy = EcatMiscCfg {
            rmii_sel0: self.rmii_sel0(),
            rmii_sel1: self.rmii_sel1(),
            glb_en: self.glb_en(),
            glb_rst: self.glb_rst(),
            rmii_ref_clk_dir0: self.rmii_ref_clk_dir0(),
            rmii_ref_clk_dir1: self.rmii_ref_clk_dir1(),
            phy_offset: self.phy_offset(),
            phy_offset_vec: self.phy_offset_vec(),
            eeprom_size_option: self.eeprom_size_option(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPIO_EMC_B1 bank IO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmcB1IoCtrl(pub u32);
impl EmcB1IoCtrl {
    #[doc = "Compensation code freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_freeze(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code freeze"]
    #[inline(always)]
    pub const fn set_gpio_emc1_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_comptq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_emc1_comptq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_compen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_emc1_compen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compensation code fast freeze enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_fastfrz_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast freeze enable"]
    #[inline(always)]
    pub const fn set_gpio_emc1_fastfrz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO_EMC_B1 IO bank's 4-bit PMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_rasrcp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_EMC_B1 IO bank's 4-bit PMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_emc1_rasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GPIO_EMC_B1 IO bank's 4-bit NMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_rasrcn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_EMC_B1 IO bank's 4-bit NMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_emc1_rasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "GPIO_EMC1_NASRC selection"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_select_nasrc(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC1_NASRC selection"]
    #[inline(always)]
    pub const fn set_gpio_emc1_select_nasrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO_EMC_B1 IO bank reference voltage generator cell sleep enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_refgen_sleep(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC_B1 IO bank reference voltage generator cell sleep enable"]
    #[inline(always)]
    pub const fn set_gpio_emc1_refgen_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO_EMC_B1 IO bank power supply mode latch enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_suplydet_latch(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC_B1 IO bank power supply mode latch enable"]
    #[inline(always)]
    pub const fn set_gpio_emc1_suplydet_latch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Compensation code fast-freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_fastfrz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast-freeze"]
    #[inline(always)]
    pub const fn set_gpio_emc1_fastfrz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO_EMC_B1 IO bank compensation OK flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_compok(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC_B1 IO bank compensation OK flag"]
    #[inline(always)]
    pub const fn set_gpio_emc1_compok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO_EMC_B1 IO bank compensation codes"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc1_nasrc(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_EMC_B1 IO bank compensation codes"]
    #[inline(always)]
    pub const fn set_gpio_emc1_nasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
}
impl Default for EmcB1IoCtrl {
    #[inline(always)]
    fn default() -> EmcB1IoCtrl {
        EmcB1IoCtrl(22036480u64 as u32)
    }
}
impl core::fmt::Debug for EmcB1IoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmcB1IoCtrl")
            .field("gpio_emc1_freeze", &self.gpio_emc1_freeze())
            .field("gpio_emc1_comptq", &self.gpio_emc1_comptq())
            .field("gpio_emc1_compen", &self.gpio_emc1_compen())
            .field("gpio_emc1_fastfrz_en", &self.gpio_emc1_fastfrz_en())
            .field("gpio_emc1_rasrcp", &self.gpio_emc1_rasrcp())
            .field("gpio_emc1_rasrcn", &self.gpio_emc1_rasrcn())
            .field("gpio_emc1_select_nasrc", &self.gpio_emc1_select_nasrc())
            .field("gpio_emc1_refgen_sleep", &self.gpio_emc1_refgen_sleep())
            .field("gpio_emc1_suplydet_latch", &self.gpio_emc1_suplydet_latch())
            .field("gpio_emc1_fastfrz", &self.gpio_emc1_fastfrz())
            .field("gpio_emc1_compok", &self.gpio_emc1_compok())
            .field("gpio_emc1_nasrc", &self.gpio_emc1_nasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmcB1IoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmcB1IoCtrl {
            gpio_emc1_freeze: bool,
            gpio_emc1_comptq: bool,
            gpio_emc1_compen: bool,
            gpio_emc1_fastfrz_en: bool,
            gpio_emc1_rasrcp: u8,
            gpio_emc1_rasrcn: u8,
            gpio_emc1_select_nasrc: bool,
            gpio_emc1_refgen_sleep: bool,
            gpio_emc1_suplydet_latch: bool,
            gpio_emc1_fastfrz: bool,
            gpio_emc1_compok: bool,
            gpio_emc1_nasrc: u8,
        }
        let proxy = EmcB1IoCtrl {
            gpio_emc1_freeze: self.gpio_emc1_freeze(),
            gpio_emc1_comptq: self.gpio_emc1_comptq(),
            gpio_emc1_compen: self.gpio_emc1_compen(),
            gpio_emc1_fastfrz_en: self.gpio_emc1_fastfrz_en(),
            gpio_emc1_rasrcp: self.gpio_emc1_rasrcp(),
            gpio_emc1_rasrcn: self.gpio_emc1_rasrcn(),
            gpio_emc1_select_nasrc: self.gpio_emc1_select_nasrc(),
            gpio_emc1_refgen_sleep: self.gpio_emc1_refgen_sleep(),
            gpio_emc1_suplydet_latch: self.gpio_emc1_suplydet_latch(),
            gpio_emc1_fastfrz: self.gpio_emc1_fastfrz(),
            gpio_emc1_compok: self.gpio_emc1_compok(),
            gpio_emc1_nasrc: self.gpio_emc1_nasrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPIO_EMC_B2 bank IO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmcB2IoCtrl(pub u32);
impl EmcB2IoCtrl {
    #[doc = "Compensation code freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_freeze(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code freeze"]
    #[inline(always)]
    pub const fn set_gpio_emc2_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_comptq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_emc2_comptq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_compen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_emc2_compen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compensation code fast freeze enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_fastfrz_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast freeze enable"]
    #[inline(always)]
    pub const fn set_gpio_emc2_fastfrz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO_EMC_B2 IO bank's 4-bit PMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_rasrcp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_EMC_B2 IO bank's 4-bit PMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_emc2_rasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GPIO_EMC_B2 IO bank's 4-bit NMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_rasrcn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_EMC_B2 IO bank's 4-bit NMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_emc2_rasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "GPIO_EMC2_NASRC selection"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_select_nasrc(&self) -> super::vals::GpioEmc2SelectNasrc {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::GpioEmc2SelectNasrc::from_bits(val as u8)
    }
    #[doc = "GPIO_EMC2_NASRC selection"]
    #[inline(always)]
    pub const fn set_gpio_emc2_select_nasrc(&mut self, val: super::vals::GpioEmc2SelectNasrc) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO_EMC_B2 IO bank reference voltage generator cell sleep enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_refgen_sleep(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC_B2 IO bank reference voltage generator cell sleep enable"]
    #[inline(always)]
    pub const fn set_gpio_emc2_refgen_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO_EMC_B2 IO bank power supply mode latch enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_suplydet_latch(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC_B2 IO bank power supply mode latch enable"]
    #[inline(always)]
    pub const fn set_gpio_emc2_suplydet_latch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Compensation code fast-freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_fastfrz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast-freeze"]
    #[inline(always)]
    pub const fn set_gpio_emc2_fastfrz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO_EMC_B2 IO bank compensation OK flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_compok(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC_B2 IO bank compensation OK flag"]
    #[inline(always)]
    pub const fn set_gpio_emc2_compok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO_EMC_B2 IO bank compensation codes"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_emc2_nasrc(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_EMC_B2 IO bank compensation codes"]
    #[inline(always)]
    pub const fn set_gpio_emc2_nasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
}
impl Default for EmcB2IoCtrl {
    #[inline(always)]
    fn default() -> EmcB2IoCtrl {
        EmcB2IoCtrl(22036480u64 as u32)
    }
}
impl core::fmt::Debug for EmcB2IoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmcB2IoCtrl")
            .field("gpio_emc2_freeze", &self.gpio_emc2_freeze())
            .field("gpio_emc2_comptq", &self.gpio_emc2_comptq())
            .field("gpio_emc2_compen", &self.gpio_emc2_compen())
            .field("gpio_emc2_fastfrz_en", &self.gpio_emc2_fastfrz_en())
            .field("gpio_emc2_rasrcp", &self.gpio_emc2_rasrcp())
            .field("gpio_emc2_rasrcn", &self.gpio_emc2_rasrcn())
            .field("gpio_emc2_select_nasrc", &self.gpio_emc2_select_nasrc())
            .field("gpio_emc2_refgen_sleep", &self.gpio_emc2_refgen_sleep())
            .field("gpio_emc2_suplydet_latch", &self.gpio_emc2_suplydet_latch())
            .field("gpio_emc2_fastfrz", &self.gpio_emc2_fastfrz())
            .field("gpio_emc2_compok", &self.gpio_emc2_compok())
            .field("gpio_emc2_nasrc", &self.gpio_emc2_nasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmcB2IoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmcB2IoCtrl {
            gpio_emc2_freeze: bool,
            gpio_emc2_comptq: bool,
            gpio_emc2_compen: bool,
            gpio_emc2_fastfrz_en: bool,
            gpio_emc2_rasrcp: u8,
            gpio_emc2_rasrcn: u8,
            gpio_emc2_select_nasrc: super::vals::GpioEmc2SelectNasrc,
            gpio_emc2_refgen_sleep: bool,
            gpio_emc2_suplydet_latch: bool,
            gpio_emc2_fastfrz: bool,
            gpio_emc2_compok: bool,
            gpio_emc2_nasrc: u8,
        }
        let proxy = EmcB2IoCtrl {
            gpio_emc2_freeze: self.gpio_emc2_freeze(),
            gpio_emc2_comptq: self.gpio_emc2_comptq(),
            gpio_emc2_compen: self.gpio_emc2_compen(),
            gpio_emc2_fastfrz_en: self.gpio_emc2_fastfrz_en(),
            gpio_emc2_rasrcp: self.gpio_emc2_rasrcp(),
            gpio_emc2_rasrcn: self.gpio_emc2_rasrcn(),
            gpio_emc2_select_nasrc: self.gpio_emc2_select_nasrc(),
            gpio_emc2_refgen_sleep: self.gpio_emc2_refgen_sleep(),
            gpio_emc2_suplydet_latch: self.gpio_emc2_suplydet_latch(),
            gpio_emc2_fastfrz: self.gpio_emc2_fastfrz(),
            gpio_emc2_compok: self.gpio_emc2_compok(),
            gpio_emc2_nasrc: self.gpio_emc2_nasrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPIO_B1 bank IO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioB1IoCtrl(pub u32);
impl GpioB1IoCtrl {
    #[doc = "Compensation code freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_freeze(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code freeze"]
    #[inline(always)]
    pub const fn set_gpio_b1_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_comptq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_b1_comptq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_compen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_b1_compen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compensation code fast freeze enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_fastfrz_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast freeze enable"]
    #[inline(always)]
    pub const fn set_gpio_b1_fastfrz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO_B1 IO bank's 4-bit PMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_rasrcp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_B1 IO bank's 4-bit PMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_b1_rasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GPIO_B1 IO bank's 4-bit NMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_rasrcn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_B1 IO bank's 4-bit NMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_b1_rasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "GPIO_B1_NASRC selection"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_select_nasrc(&self) -> super::vals::GpioB1SelectNasrc {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::GpioB1SelectNasrc::from_bits(val as u8)
    }
    #[doc = "GPIO_B1_NASRC selection"]
    #[inline(always)]
    pub const fn set_gpio_b1_select_nasrc(&mut self, val: super::vals::GpioB1SelectNasrc) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO_B1 IO bank reference voltage generator cell sleep enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_refgen_sleep(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_B1 IO bank reference voltage generator cell sleep enable"]
    #[inline(always)]
    pub const fn set_gpio_b1_refgen_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO_B1 IO bank power supply mode latch enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_suplydet_latch(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_B1 IO bank power supply mode latch enable"]
    #[inline(always)]
    pub const fn set_gpio_b1_suplydet_latch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Compensation code fast-freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_fastfrz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast-freeze"]
    #[inline(always)]
    pub const fn set_gpio_b1_fastfrz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO_B1 IO bank compensation OK flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_compok(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_B1 IO bank compensation OK flag"]
    #[inline(always)]
    pub const fn set_gpio_b1_compok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO_B1 IO bank compensation codes"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b1_nasrc(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_B1 IO bank compensation codes"]
    #[inline(always)]
    pub const fn set_gpio_b1_nasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
}
impl Default for GpioB1IoCtrl {
    #[inline(always)]
    fn default() -> GpioB1IoCtrl {
        GpioB1IoCtrl(22036480u64 as u32)
    }
}
impl core::fmt::Debug for GpioB1IoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpioB1IoCtrl")
            .field("gpio_b1_freeze", &self.gpio_b1_freeze())
            .field("gpio_b1_comptq", &self.gpio_b1_comptq())
            .field("gpio_b1_compen", &self.gpio_b1_compen())
            .field("gpio_b1_fastfrz_en", &self.gpio_b1_fastfrz_en())
            .field("gpio_b1_rasrcp", &self.gpio_b1_rasrcp())
            .field("gpio_b1_rasrcn", &self.gpio_b1_rasrcn())
            .field("gpio_b1_select_nasrc", &self.gpio_b1_select_nasrc())
            .field("gpio_b1_refgen_sleep", &self.gpio_b1_refgen_sleep())
            .field("gpio_b1_suplydet_latch", &self.gpio_b1_suplydet_latch())
            .field("gpio_b1_fastfrz", &self.gpio_b1_fastfrz())
            .field("gpio_b1_compok", &self.gpio_b1_compok())
            .field("gpio_b1_nasrc", &self.gpio_b1_nasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpioB1IoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpioB1IoCtrl {
            gpio_b1_freeze: bool,
            gpio_b1_comptq: bool,
            gpio_b1_compen: bool,
            gpio_b1_fastfrz_en: bool,
            gpio_b1_rasrcp: u8,
            gpio_b1_rasrcn: u8,
            gpio_b1_select_nasrc: super::vals::GpioB1SelectNasrc,
            gpio_b1_refgen_sleep: bool,
            gpio_b1_suplydet_latch: bool,
            gpio_b1_fastfrz: bool,
            gpio_b1_compok: bool,
            gpio_b1_nasrc: u8,
        }
        let proxy = GpioB1IoCtrl {
            gpio_b1_freeze: self.gpio_b1_freeze(),
            gpio_b1_comptq: self.gpio_b1_comptq(),
            gpio_b1_compen: self.gpio_b1_compen(),
            gpio_b1_fastfrz_en: self.gpio_b1_fastfrz_en(),
            gpio_b1_rasrcp: self.gpio_b1_rasrcp(),
            gpio_b1_rasrcn: self.gpio_b1_rasrcn(),
            gpio_b1_select_nasrc: self.gpio_b1_select_nasrc(),
            gpio_b1_refgen_sleep: self.gpio_b1_refgen_sleep(),
            gpio_b1_suplydet_latch: self.gpio_b1_suplydet_latch(),
            gpio_b1_fastfrz: self.gpio_b1_fastfrz(),
            gpio_b1_compok: self.gpio_b1_compok(),
            gpio_b1_nasrc: self.gpio_b1_nasrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPIO_B2 bank IO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioB2IoCtrl(pub u32);
impl GpioB2IoCtrl {
    #[doc = "Compensation code freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_freeze(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code freeze"]
    #[inline(always)]
    pub const fn set_gpio_b2_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_comptq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_b2_comptq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_compen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_b2_compen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compensation code fast freeze enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_fastfrz_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast freeze enable"]
    #[inline(always)]
    pub const fn set_gpio_b2_fastfrz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO_B2 IO bank's 4-bit PMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_rasrcp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_B2 IO bank's 4-bit PMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_b2_rasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GPIO_B2 IO bank's 4-bit NMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_rasrcn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_B2 IO bank's 4-bit NMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_b2_rasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "GPIO_B2_NASRC selection"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_select_nasrc(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_B2_NASRC selection"]
    #[inline(always)]
    pub const fn set_gpio_b2_select_nasrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO_B2 IO bank reference voltage generator cell sleep enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_refgen_sleep(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_B2 IO bank reference voltage generator cell sleep enable"]
    #[inline(always)]
    pub const fn set_gpio_b2_refgen_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO_B2 IO bank power supply mode latch enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_suplydet_latch(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_B2 IO bank power supply mode latch enable"]
    #[inline(always)]
    pub const fn set_gpio_b2_suplydet_latch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Compensation code fast-freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_fastfrz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast-freeze"]
    #[inline(always)]
    pub const fn set_gpio_b2_fastfrz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO_B2 IO bank compensation OK flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_compok(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_B2 IO bank compensation OK flag"]
    #[inline(always)]
    pub const fn set_gpio_b2_compok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO_B2 IO bank compensation codes"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_b2_nasrc(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_B2 IO bank compensation codes"]
    #[inline(always)]
    pub const fn set_gpio_b2_nasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
}
impl Default for GpioB2IoCtrl {
    #[inline(always)]
    fn default() -> GpioB2IoCtrl {
        GpioB2IoCtrl(22036480u64 as u32)
    }
}
impl core::fmt::Debug for GpioB2IoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpioB2IoCtrl")
            .field("gpio_b2_freeze", &self.gpio_b2_freeze())
            .field("gpio_b2_comptq", &self.gpio_b2_comptq())
            .field("gpio_b2_compen", &self.gpio_b2_compen())
            .field("gpio_b2_fastfrz_en", &self.gpio_b2_fastfrz_en())
            .field("gpio_b2_rasrcp", &self.gpio_b2_rasrcp())
            .field("gpio_b2_rasrcn", &self.gpio_b2_rasrcn())
            .field("gpio_b2_select_nasrc", &self.gpio_b2_select_nasrc())
            .field("gpio_b2_refgen_sleep", &self.gpio_b2_refgen_sleep())
            .field("gpio_b2_suplydet_latch", &self.gpio_b2_suplydet_latch())
            .field("gpio_b2_fastfrz", &self.gpio_b2_fastfrz())
            .field("gpio_b2_compok", &self.gpio_b2_compok())
            .field("gpio_b2_nasrc", &self.gpio_b2_nasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpioB2IoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpioB2IoCtrl {
            gpio_b2_freeze: bool,
            gpio_b2_comptq: bool,
            gpio_b2_compen: bool,
            gpio_b2_fastfrz_en: bool,
            gpio_b2_rasrcp: u8,
            gpio_b2_rasrcn: u8,
            gpio_b2_select_nasrc: bool,
            gpio_b2_refgen_sleep: bool,
            gpio_b2_suplydet_latch: bool,
            gpio_b2_fastfrz: bool,
            gpio_b2_compok: bool,
            gpio_b2_nasrc: u8,
        }
        let proxy = GpioB2IoCtrl {
            gpio_b2_freeze: self.gpio_b2_freeze(),
            gpio_b2_comptq: self.gpio_b2_comptq(),
            gpio_b2_compen: self.gpio_b2_compen(),
            gpio_b2_fastfrz_en: self.gpio_b2_fastfrz_en(),
            gpio_b2_rasrcp: self.gpio_b2_rasrcp(),
            gpio_b2_rasrcn: self.gpio_b2_rasrcn(),
            gpio_b2_select_nasrc: self.gpio_b2_select_nasrc(),
            gpio_b2_refgen_sleep: self.gpio_b2_refgen_sleep(),
            gpio_b2_suplydet_latch: self.gpio_b2_suplydet_latch(),
            gpio_b2_fastfrz: self.gpio_b2_fastfrz(),
            gpio_b2_compok: self.gpio_b2_compok(),
            gpio_b2_nasrc: self.gpio_b2_nasrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "I3C2 async wakeup control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c2AsyncWakeupCtrl(pub u32);
impl I3c2AsyncWakeupCtrl {
    #[doc = "Async wakeup interrupt clear"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Async wakeup interrupt clear"]
    #[inline(always)]
    pub const fn set_irq_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Async wakeup interrupt status"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_status(&self) -> super::vals::IrqStatus {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::IrqStatus::from_bits(val as u8)
    }
    #[doc = "Async wakeup interrupt status"]
    #[inline(always)]
    pub const fn set_irq_status(&mut self, val: super::vals::IrqStatus) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Master mode async wakeup interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Master mode async wakeup interrupt enable"]
    #[inline(always)]
    pub const fn set_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for I3c2AsyncWakeupCtrl {
    #[inline(always)]
    fn default() -> I3c2AsyncWakeupCtrl {
        I3c2AsyncWakeupCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for I3c2AsyncWakeupCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3c2AsyncWakeupCtrl")
            .field("irq_clr", &self.irq_clr())
            .field("irq_status", &self.irq_status())
            .field("irq_en", &self.irq_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for I3c2AsyncWakeupCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct I3c2AsyncWakeupCtrl {
            irq_clr: bool,
            irq_status: super::vals::IrqStatus,
            irq_en: bool,
        }
        let proxy = I3c2AsyncWakeupCtrl {
            irq_clr: self.irq_clr(),
            irq_status: self.irq_status(),
            irq_en: self.irq_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IPG DEBUG mask bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpgDebug1(pub u32);
impl IpgDebug1 {
    #[doc = "CAN2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_can2(&self) -> super::vals::M33Can2 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::M33Can2::from_bits(val as u8)
    }
    #[doc = "CAN2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m33_can2(&mut self, val: super::vals::M33Can2) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "EDMA4 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_edma4(&self) -> super::vals::M33Edma4 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::M33Edma4::from_bits(val as u8)
    }
    #[doc = "EDMA4 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_edma4(&mut self, val: super::vals::M33Edma4) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLEXIO1 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_flexio1(&self) -> super::vals::M33Flexio1 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::M33Flexio1::from_bits(val as u8)
    }
    #[doc = "FLEXIO1 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_flexio1(&mut self, val: super::vals::M33Flexio1) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FLEXIO2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_flexio2(&self) -> super::vals::M33Flexio2 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::M33Flexio2::from_bits(val as u8)
    }
    #[doc = "FLEXIO2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_flexio2(&mut self, val: super::vals::M33Flexio2) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPI2C3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpi2c3(&self) -> super::vals::M33Lpi2c3 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::M33Lpi2c3::from_bits(val as u8)
    }
    #[doc = "LPI2C3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpi2c3(&mut self, val: super::vals::M33Lpi2c3) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LPI2C4 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpi2c4(&self) -> super::vals::M33Lpi2c4 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::M33Lpi2c4::from_bits(val as u8)
    }
    #[doc = "LPI2C4 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpi2c4(&mut self, val: super::vals::M33Lpi2c4) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LPIT2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpit2(&self) -> super::vals::M33Lpit2 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::M33Lpit2::from_bits(val as u8)
    }
    #[doc = "LPIT2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpit2(&mut self, val: super::vals::M33Lpit2) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "LPSPI3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpspi3(&self) -> super::vals::M33Lpspi3 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::M33Lpspi3::from_bits(val as u8)
    }
    #[doc = "LPSPI3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpspi3(&mut self, val: super::vals::M33Lpspi3) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "LPSPI4 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpspi4(&self) -> super::vals::M33Lpspi4 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::M33Lpspi4::from_bits(val as u8)
    }
    #[doc = "LPSPI4 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpspi4(&mut self, val: super::vals::M33Lpspi4) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "LPTMR2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lptmr2(&self) -> super::vals::M33Lptmr2 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::M33Lptmr2::from_bits(val as u8)
    }
    #[doc = "LPTMR2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lptmr2(&mut self, val: super::vals::M33Lptmr2) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_tpm3(&self) -> super::vals::M33Tpm3 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::M33Tpm3::from_bits(val as u8)
    }
    #[doc = "debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_tpm3(&mut self, val: super::vals::M33Tpm3) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "TPM3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_tpm4(&self) -> super::vals::M33Tpm4 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::M33Tpm4::from_bits(val as u8)
    }
    #[doc = "TPM3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_tpm4(&mut self, val: super::vals::M33Tpm4) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "TPM5 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_tpm5(&self) -> super::vals::M33Tpm5 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::M33Tpm5::from_bits(val as u8)
    }
    #[doc = "TPM5 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_tpm5(&mut self, val: super::vals::M33Tpm5) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "TPM6 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_tpm6(&self) -> super::vals::M33Tpm6 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::M33Tpm6::from_bits(val as u8)
    }
    #[doc = "TPM6 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_tpm6(&mut self, val: super::vals::M33Tpm6) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "WDOG3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_wdog3(&self) -> super::vals::M33Wdog3 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::M33Wdog3::from_bits(val as u8)
    }
    #[doc = "WDOG3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_wdog3(&mut self, val: super::vals::M33Wdog3) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "WDOG4 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_wdog4(&self) -> super::vals::M33Wdog4 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::M33Wdog4::from_bits(val as u8)
    }
    #[doc = "WDOG4 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_wdog4(&mut self, val: super::vals::M33Wdog4) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "CAN2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_can2(&self) -> super::vals::M7Can2 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::M7Can2::from_bits(val as u8)
    }
    #[doc = "CAN2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_can2(&mut self, val: super::vals::M7Can2) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "EDMA4 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_edma4(&self) -> super::vals::M7Edma4 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::M7Edma4::from_bits(val as u8)
    }
    #[doc = "EDMA4 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_edma4(&mut self, val: super::vals::M7Edma4) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "FLEXIO1 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_flexio1(&self) -> super::vals::M7Flexio1 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::M7Flexio1::from_bits(val as u8)
    }
    #[doc = "FLEXIO1 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_flexio1(&mut self, val: super::vals::M7Flexio1) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "FLEXIO2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_flexio2(&self) -> super::vals::M7Flexio2 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::M7Flexio2::from_bits(val as u8)
    }
    #[doc = "FLEXIO2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_flexio2(&mut self, val: super::vals::M7Flexio2) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "LPI2C3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpi2c3(&self) -> super::vals::M7Lpi2c3 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::M7Lpi2c3::from_bits(val as u8)
    }
    #[doc = "LPI2C3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpi2c3(&mut self, val: super::vals::M7Lpi2c3) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C4 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpi2c4(&self) -> super::vals::M7Lpi2c4 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::M7Lpi2c4::from_bits(val as u8)
    }
    #[doc = "LPI2C4 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpi2c4(&mut self, val: super::vals::M7Lpi2c4) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "LPIT2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpit2(&self) -> super::vals::M7Lpit2 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::M7Lpit2::from_bits(val as u8)
    }
    #[doc = "LPIT2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpit2(&mut self, val: super::vals::M7Lpit2) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "WDOG3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpspi3(&self) -> super::vals::M7Lpspi3 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::M7Lpspi3::from_bits(val as u8)
    }
    #[doc = "WDOG3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpspi3(&mut self, val: super::vals::M7Lpspi3) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "LPSPI4 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpspi4(&self) -> super::vals::M7Lpspi4 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::M7Lpspi4::from_bits(val as u8)
    }
    #[doc = "LPSPI4 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpspi4(&mut self, val: super::vals::M7Lpspi4) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "LPTMR2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lptmr2(&self) -> super::vals::M7Lptmr2 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::M7Lptmr2::from_bits(val as u8)
    }
    #[doc = "LPTMR2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lptmr2(&mut self, val: super::vals::M7Lptmr2) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "TPM3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_tpm3(&self) -> super::vals::M7Tpm3 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::M7Tpm3::from_bits(val as u8)
    }
    #[doc = "TPM3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_tpm3(&mut self, val: super::vals::M7Tpm3) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "TPM4 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_tpm4(&self) -> super::vals::M7Tpm4 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::M7Tpm4::from_bits(val as u8)
    }
    #[doc = "TPM4 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_tpm4(&mut self, val: super::vals::M7Tpm4) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "TPM5 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_tpm5(&self) -> super::vals::M7Tpm5 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::M7Tpm5::from_bits(val as u8)
    }
    #[doc = "TPM5 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_tpm5(&mut self, val: super::vals::M7Tpm5) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "TPM6 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_tpm6(&self) -> super::vals::M7Tpm6 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::M7Tpm6::from_bits(val as u8)
    }
    #[doc = "TPM6 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_tpm6(&mut self, val: super::vals::M7Tpm6) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "WDOG3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_wdog3(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_wdog3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "WDOG4 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_wdog4(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "WDOG4 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_wdog4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for IpgDebug1 {
    #[inline(always)]
    fn default() -> IpgDebug1 {
        IpgDebug1(0u64 as u32)
    }
}
impl core::fmt::Debug for IpgDebug1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpgDebug1")
            .field("m33_can2", &self.m33_can2())
            .field("m33_edma4", &self.m33_edma4())
            .field("m33_flexio1", &self.m33_flexio1())
            .field("m33_flexio2", &self.m33_flexio2())
            .field("m33_lpi2c3", &self.m33_lpi2c3())
            .field("m33_lpi2c4", &self.m33_lpi2c4())
            .field("m33_lpit2", &self.m33_lpit2())
            .field("m33_lpspi3", &self.m33_lpspi3())
            .field("m33_lpspi4", &self.m33_lpspi4())
            .field("m33_lptmr2", &self.m33_lptmr2())
            .field("m33_tpm3", &self.m33_tpm3())
            .field("m33_tpm4", &self.m33_tpm4())
            .field("m33_tpm5", &self.m33_tpm5())
            .field("m33_tpm6", &self.m33_tpm6())
            .field("m33_wdog3", &self.m33_wdog3())
            .field("m33_wdog4", &self.m33_wdog4())
            .field("m7_can2", &self.m7_can2())
            .field("m7_edma4", &self.m7_edma4())
            .field("m7_flexio1", &self.m7_flexio1())
            .field("m7_flexio2", &self.m7_flexio2())
            .field("m7_lpi2c3", &self.m7_lpi2c3())
            .field("m7_lpi2c4", &self.m7_lpi2c4())
            .field("m7_lpit2", &self.m7_lpit2())
            .field("m7_lpspi3", &self.m7_lpspi3())
            .field("m7_lpspi4", &self.m7_lpspi4())
            .field("m7_lptmr2", &self.m7_lptmr2())
            .field("m7_tpm3", &self.m7_tpm3())
            .field("m7_tpm4", &self.m7_tpm4())
            .field("m7_tpm5", &self.m7_tpm5())
            .field("m7_tpm6", &self.m7_tpm6())
            .field("m7_wdog3", &self.m7_wdog3())
            .field("m7_wdog4", &self.m7_wdog4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpgDebug1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IpgDebug1 {
            m33_can2: super::vals::M33Can2,
            m33_edma4: super::vals::M33Edma4,
            m33_flexio1: super::vals::M33Flexio1,
            m33_flexio2: super::vals::M33Flexio2,
            m33_lpi2c3: super::vals::M33Lpi2c3,
            m33_lpi2c4: super::vals::M33Lpi2c4,
            m33_lpit2: super::vals::M33Lpit2,
            m33_lpspi3: super::vals::M33Lpspi3,
            m33_lpspi4: super::vals::M33Lpspi4,
            m33_lptmr2: super::vals::M33Lptmr2,
            m33_tpm3: super::vals::M33Tpm3,
            m33_tpm4: super::vals::M33Tpm4,
            m33_tpm5: super::vals::M33Tpm5,
            m33_tpm6: super::vals::M33Tpm6,
            m33_wdog3: super::vals::M33Wdog3,
            m33_wdog4: super::vals::M33Wdog4,
            m7_can2: super::vals::M7Can2,
            m7_edma4: super::vals::M7Edma4,
            m7_flexio1: super::vals::M7Flexio1,
            m7_flexio2: super::vals::M7Flexio2,
            m7_lpi2c3: super::vals::M7Lpi2c3,
            m7_lpi2c4: super::vals::M7Lpi2c4,
            m7_lpit2: super::vals::M7Lpit2,
            m7_lpspi3: super::vals::M7Lpspi3,
            m7_lpspi4: super::vals::M7Lpspi4,
            m7_lptmr2: super::vals::M7Lptmr2,
            m7_tpm3: super::vals::M7Tpm3,
            m7_tpm4: super::vals::M7Tpm4,
            m7_tpm5: super::vals::M7Tpm5,
            m7_tpm6: super::vals::M7Tpm6,
            m7_wdog3: bool,
            m7_wdog4: bool,
        }
        let proxy = IpgDebug1 {
            m33_can2: self.m33_can2(),
            m33_edma4: self.m33_edma4(),
            m33_flexio1: self.m33_flexio1(),
            m33_flexio2: self.m33_flexio2(),
            m33_lpi2c3: self.m33_lpi2c3(),
            m33_lpi2c4: self.m33_lpi2c4(),
            m33_lpit2: self.m33_lpit2(),
            m33_lpspi3: self.m33_lpspi3(),
            m33_lpspi4: self.m33_lpspi4(),
            m33_lptmr2: self.m33_lptmr2(),
            m33_tpm3: self.m33_tpm3(),
            m33_tpm4: self.m33_tpm4(),
            m33_tpm5: self.m33_tpm5(),
            m33_tpm6: self.m33_tpm6(),
            m33_wdog3: self.m33_wdog3(),
            m33_wdog4: self.m33_wdog4(),
            m7_can2: self.m7_can2(),
            m7_edma4: self.m7_edma4(),
            m7_flexio1: self.m7_flexio1(),
            m7_flexio2: self.m7_flexio2(),
            m7_lpi2c3: self.m7_lpi2c3(),
            m7_lpi2c4: self.m7_lpi2c4(),
            m7_lpit2: self.m7_lpit2(),
            m7_lpspi3: self.m7_lpspi3(),
            m7_lpspi4: self.m7_lpspi4(),
            m7_lptmr2: self.m7_lptmr2(),
            m7_tpm3: self.m7_tpm3(),
            m7_tpm4: self.m7_tpm4(),
            m7_tpm5: self.m7_tpm5(),
            m7_tpm6: self.m7_tpm6(),
            m7_wdog3: self.m7_wdog3(),
            m7_wdog4: self.m7_wdog4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IPG DEBUG mask bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpgDebug2(pub u32);
impl IpgDebug2 {
    #[doc = "WDOG5 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_wdog5(&self) -> super::vals::M33Wdog5 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::M33Wdog5::from_bits(val as u8)
    }
    #[doc = "WDOG5 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m33_wdog5(&mut self, val: super::vals::M33Wdog5) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LPTMR3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lptmr3(&self) -> super::vals::M33Lptmr3 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::M33Lptmr3::from_bits(val as u8)
    }
    #[doc = "LPTMR3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lptmr3(&mut self, val: super::vals::M33Lptmr3) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "LPSPI5 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpspi5(&self) -> super::vals::M33Lpspi5 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::M33Lpspi5::from_bits(val as u8)
    }
    #[doc = "LPSPI5 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpspi5(&mut self, val: super::vals::M33Lpspi5) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LPSPI6 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpspi6(&self) -> super::vals::M33Lpspi6 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::M33Lpspi6::from_bits(val as u8)
    }
    #[doc = "LPSPI6 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpspi6(&mut self, val: super::vals::M33Lpspi6) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPIT3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpit3(&self) -> super::vals::M33Lpit3 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::M33Lpit3::from_bits(val as u8)
    }
    #[doc = "LPIT3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpit3(&mut self, val: super::vals::M33Lpit3) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "LPI2C5 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpi2c5(&self) -> super::vals::M33Lpi2c5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::M33Lpi2c5::from_bits(val as u8)
    }
    #[doc = "LPI2C5 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpi2c5(&mut self, val: super::vals::M33Lpi2c5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "LPI2C6 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_lpi2c6(&self) -> super::vals::M33Lpi2c6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::M33Lpi2c6::from_bits(val as u8)
    }
    #[doc = "LPI2C6 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_lpi2c6(&mut self, val: super::vals::M33Lpi2c6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "GPT2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_gpt2(&self) -> super::vals::M33Gpt2 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::M33Gpt2::from_bits(val as u8)
    }
    #[doc = "GPT2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_gpt2(&mut self, val: super::vals::M33Gpt2) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FLEXPWM1 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_flexpwm1(&self) -> super::vals::M33Flexpwm1 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::M33Flexpwm1::from_bits(val as u8)
    }
    #[doc = "FLEXPWM1 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_flexpwm1(&mut self, val: super::vals::M33Flexpwm1) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXPWM2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_flexpwm2(&self) -> super::vals::M33Flexpwm2 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::M33Flexpwm2::from_bits(val as u8)
    }
    #[doc = "FLEXPWM2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_flexpwm2(&mut self, val: super::vals::M33Flexpwm2) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXPWM3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_flexpwm3(&self) -> super::vals::M33Flexpwm3 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::M33Flexpwm3::from_bits(val as u8)
    }
    #[doc = "FLEXPWM3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_flexpwm3(&mut self, val: super::vals::M33Flexpwm3) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXPWM4 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_flexpwm4(&self) -> super::vals::M33Flexpwm4 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::M33Flexpwm4::from_bits(val as u8)
    }
    #[doc = "FLEXPWM4 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_flexpwm4(&mut self, val: super::vals::M33Flexpwm4) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "MIC debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_mic(&self) -> super::vals::M33Mic {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::M33Mic::from_bits(val as u8)
    }
    #[doc = "MIC debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_mic(&mut self, val: super::vals::M33Mic) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "SAI2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_sai2(&self) -> super::vals::M33Sai2 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::M33Sai2::from_bits(val as u8)
    }
    #[doc = "SAI2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_sai2(&mut self, val: super::vals::M33Sai2) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "SAI3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_sai3(&self) -> super::vals::M33Sai3 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::M33Sai3::from_bits(val as u8)
    }
    #[doc = "SAI3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_sai3(&mut self, val: super::vals::M33Sai3) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "SAI4 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_sai4(&self) -> super::vals::M33Sai4 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::M33Sai4::from_bits(val as u8)
    }
    #[doc = "SAI4 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_sai4(&mut self, val: super::vals::M33Sai4) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "WDOG5 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_wdog5(&self) -> super::vals::M7Wdog5 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::M7Wdog5::from_bits(val as u8)
    }
    #[doc = "WDOG5 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_wdog5(&mut self, val: super::vals::M7Wdog5) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "LPTMR3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lptmr3(&self) -> super::vals::M7Lptmr3 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::M7Lptmr3::from_bits(val as u8)
    }
    #[doc = "LPTMR3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lptmr3(&mut self, val: super::vals::M7Lptmr3) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "LPTMR3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpspi5(&self) -> super::vals::M7Lpspi5 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::M7Lpspi5::from_bits(val as u8)
    }
    #[doc = "LPTMR3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpspi5(&mut self, val: super::vals::M7Lpspi5) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "LPSPI6 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpspi6(&self) -> super::vals::M7Lpspi6 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::M7Lpspi6::from_bits(val as u8)
    }
    #[doc = "LPSPI6 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpspi6(&mut self, val: super::vals::M7Lpspi6) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "LPIT3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpit3(&self) -> super::vals::M7Lpit3 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::M7Lpit3::from_bits(val as u8)
    }
    #[doc = "LPIT3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpit3(&mut self, val: super::vals::M7Lpit3) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "LPI2C5 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpi2c5(&self) -> super::vals::M7Lpi2c5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::M7Lpi2c5::from_bits(val as u8)
    }
    #[doc = "LPI2C5 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpi2c5(&mut self, val: super::vals::M7Lpi2c5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "LPI2C6\" debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_lpi2c6(&self) -> super::vals::M7Lpi2c6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::M7Lpi2c6::from_bits(val as u8)
    }
    #[doc = "LPI2C6\" debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_lpi2c6(&mut self, val: super::vals::M7Lpi2c6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "GPT2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_gpt2(&self) -> super::vals::M7Gpt2 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::M7Gpt2::from_bits(val as u8)
    }
    #[doc = "GPT2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_gpt2(&mut self, val: super::vals::M7Gpt2) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "FLEXPWM1 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_flexpwm1(&self) -> super::vals::M7Flexpwm1 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::M7Flexpwm1::from_bits(val as u8)
    }
    #[doc = "FLEXPWM1 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_flexpwm1(&mut self, val: super::vals::M7Flexpwm1) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "FLEXPWM2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_flexpwm2(&self) -> super::vals::M7Flexpwm2 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::M7Flexpwm2::from_bits(val as u8)
    }
    #[doc = "FLEXPWM2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_flexpwm2(&mut self, val: super::vals::M7Flexpwm2) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FLEXPWM3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_flexpwm3(&self) -> super::vals::M7Flexpwm3 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::M7Flexpwm3::from_bits(val as u8)
    }
    #[doc = "FLEXPWM3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_flexpwm3(&mut self, val: super::vals::M7Flexpwm3) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXPWM4 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_flexpwm4(&self) -> super::vals::M7Flexpwm4 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::M7Flexpwm4::from_bits(val as u8)
    }
    #[doc = "FLEXPWM4 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_flexpwm4(&mut self, val: super::vals::M7Flexpwm4) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "MIC debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_mic(&self) -> super::vals::M7Mic {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::M7Mic::from_bits(val as u8)
    }
    #[doc = "MIC debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_mic(&mut self, val: super::vals::M7Mic) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "SAI2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_sai2(&self) -> super::vals::M7Sai2 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::M7Sai2::from_bits(val as u8)
    }
    #[doc = "SAI2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_sai2(&mut self, val: super::vals::M7Sai2) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "SAI3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_sai3(&self) -> super::vals::M7Sai3 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::M7Sai3::from_bits(val as u8)
    }
    #[doc = "SAI3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_sai3(&mut self, val: super::vals::M7Sai3) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "SAI4 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_sai4(&self) -> super::vals::M7Sai4 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::M7Sai4::from_bits(val as u8)
    }
    #[doc = "SAI4 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_sai4(&mut self, val: super::vals::M7Sai4) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for IpgDebug2 {
    #[inline(always)]
    fn default() -> IpgDebug2 {
        IpgDebug2(0u64 as u32)
    }
}
impl core::fmt::Debug for IpgDebug2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpgDebug2")
            .field("m33_wdog5", &self.m33_wdog5())
            .field("m33_lptmr3", &self.m33_lptmr3())
            .field("m33_lpspi5", &self.m33_lpspi5())
            .field("m33_lpspi6", &self.m33_lpspi6())
            .field("m33_lpit3", &self.m33_lpit3())
            .field("m33_lpi2c5", &self.m33_lpi2c5())
            .field("m33_lpi2c6", &self.m33_lpi2c6())
            .field("m33_gpt2", &self.m33_gpt2())
            .field("m33_flexpwm1", &self.m33_flexpwm1())
            .field("m33_flexpwm2", &self.m33_flexpwm2())
            .field("m33_flexpwm3", &self.m33_flexpwm3())
            .field("m33_flexpwm4", &self.m33_flexpwm4())
            .field("m33_mic", &self.m33_mic())
            .field("m33_sai2", &self.m33_sai2())
            .field("m33_sai3", &self.m33_sai3())
            .field("m33_sai4", &self.m33_sai4())
            .field("m7_wdog5", &self.m7_wdog5())
            .field("m7_lptmr3", &self.m7_lptmr3())
            .field("m7_lpspi5", &self.m7_lpspi5())
            .field("m7_lpspi6", &self.m7_lpspi6())
            .field("m7_lpit3", &self.m7_lpit3())
            .field("m7_lpi2c5", &self.m7_lpi2c5())
            .field("m7_lpi2c6", &self.m7_lpi2c6())
            .field("m7_gpt2", &self.m7_gpt2())
            .field("m7_flexpwm1", &self.m7_flexpwm1())
            .field("m7_flexpwm2", &self.m7_flexpwm2())
            .field("m7_flexpwm3", &self.m7_flexpwm3())
            .field("m7_flexpwm4", &self.m7_flexpwm4())
            .field("m7_mic", &self.m7_mic())
            .field("m7_sai2", &self.m7_sai2())
            .field("m7_sai3", &self.m7_sai3())
            .field("m7_sai4", &self.m7_sai4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpgDebug2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IpgDebug2 {
            m33_wdog5: super::vals::M33Wdog5,
            m33_lptmr3: super::vals::M33Lptmr3,
            m33_lpspi5: super::vals::M33Lpspi5,
            m33_lpspi6: super::vals::M33Lpspi6,
            m33_lpit3: super::vals::M33Lpit3,
            m33_lpi2c5: super::vals::M33Lpi2c5,
            m33_lpi2c6: super::vals::M33Lpi2c6,
            m33_gpt2: super::vals::M33Gpt2,
            m33_flexpwm1: super::vals::M33Flexpwm1,
            m33_flexpwm2: super::vals::M33Flexpwm2,
            m33_flexpwm3: super::vals::M33Flexpwm3,
            m33_flexpwm4: super::vals::M33Flexpwm4,
            m33_mic: super::vals::M33Mic,
            m33_sai2: super::vals::M33Sai2,
            m33_sai3: super::vals::M33Sai3,
            m33_sai4: super::vals::M33Sai4,
            m7_wdog5: super::vals::M7Wdog5,
            m7_lptmr3: super::vals::M7Lptmr3,
            m7_lpspi5: super::vals::M7Lpspi5,
            m7_lpspi6: super::vals::M7Lpspi6,
            m7_lpit3: super::vals::M7Lpit3,
            m7_lpi2c5: super::vals::M7Lpi2c5,
            m7_lpi2c6: super::vals::M7Lpi2c6,
            m7_gpt2: super::vals::M7Gpt2,
            m7_flexpwm1: super::vals::M7Flexpwm1,
            m7_flexpwm2: super::vals::M7Flexpwm2,
            m7_flexpwm3: super::vals::M7Flexpwm3,
            m7_flexpwm4: super::vals::M7Flexpwm4,
            m7_mic: super::vals::M7Mic,
            m7_sai2: super::vals::M7Sai2,
            m7_sai3: super::vals::M7Sai3,
            m7_sai4: super::vals::M7Sai4,
        }
        let proxy = IpgDebug2 {
            m33_wdog5: self.m33_wdog5(),
            m33_lptmr3: self.m33_lptmr3(),
            m33_lpspi5: self.m33_lpspi5(),
            m33_lpspi6: self.m33_lpspi6(),
            m33_lpit3: self.m33_lpit3(),
            m33_lpi2c5: self.m33_lpi2c5(),
            m33_lpi2c6: self.m33_lpi2c6(),
            m33_gpt2: self.m33_gpt2(),
            m33_flexpwm1: self.m33_flexpwm1(),
            m33_flexpwm2: self.m33_flexpwm2(),
            m33_flexpwm3: self.m33_flexpwm3(),
            m33_flexpwm4: self.m33_flexpwm4(),
            m33_mic: self.m33_mic(),
            m33_sai2: self.m33_sai2(),
            m33_sai3: self.m33_sai3(),
            m33_sai4: self.m33_sai4(),
            m7_wdog5: self.m7_wdog5(),
            m7_lptmr3: self.m7_lptmr3(),
            m7_lpspi5: self.m7_lpspi5(),
            m7_lpspi6: self.m7_lpspi6(),
            m7_lpit3: self.m7_lpit3(),
            m7_lpi2c5: self.m7_lpi2c5(),
            m7_lpi2c6: self.m7_lpi2c6(),
            m7_gpt2: self.m7_gpt2(),
            m7_flexpwm1: self.m7_flexpwm1(),
            m7_flexpwm2: self.m7_flexpwm2(),
            m7_flexpwm3: self.m7_flexpwm3(),
            m7_flexpwm4: self.m7_flexpwm4(),
            m7_mic: self.m7_mic(),
            m7_sai2: self.m7_sai2(),
            m7_sai3: self.m7_sai3(),
            m7_sai4: self.m7_sai4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IPG DEBUG mask bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpgDebug3(pub u32);
impl IpgDebug3 {
    #[doc = "I3C2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_sinc1(&self) -> super::vals::M33Sinc1 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::M33Sinc1::from_bits(val as u8)
    }
    #[doc = "I3C2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_sinc1(&mut self, val: super::vals::M33Sinc1) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SINC2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_sinc2(&self) -> super::vals::M33Sinc2 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::M33Sinc2::from_bits(val as u8)
    }
    #[doc = "SINC2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_sinc2(&mut self, val: super::vals::M33Sinc2) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "SINC3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_sinc3(&self) -> super::vals::M33Sinc3 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::M33Sinc3::from_bits(val as u8)
    }
    #[doc = "SINC3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_sinc3(&mut self, val: super::vals::M33Sinc3) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "QTIMER1 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_qtimer1(&self) -> super::vals::M33Qtimer1 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::M33Qtimer1::from_bits(val as u8)
    }
    #[doc = "QTIMER1 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_qtimer1(&mut self, val: super::vals::M33Qtimer1) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "QTIMER2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_qtimer2(&self) -> super::vals::M33Qtimer2 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::M33Qtimer2::from_bits(val as u8)
    }
    #[doc = "QTIMER2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_qtimer2(&mut self, val: super::vals::M33Qtimer2) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "QTIMER3 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_qtimer3(&self) -> super::vals::M33Qtimer3 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::M33Qtimer3::from_bits(val as u8)
    }
    #[doc = "QTIMER3 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_qtimer3(&mut self, val: super::vals::M33Qtimer3) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "QTIMER4 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_qtimer4(&self) -> super::vals::M33Qtimer4 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::M33Qtimer4::from_bits(val as u8)
    }
    #[doc = "QTIMER4 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_qtimer4(&mut self, val: super::vals::M33Qtimer4) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "QTIMER5 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_qtimer5(&self) -> super::vals::M33Qtimer5 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::M33Qtimer5::from_bits(val as u8)
    }
    #[doc = "QTIMER5 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_qtimer5(&mut self, val: super::vals::M33Qtimer5) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "QTIMER6 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_qtimer6(&self) -> super::vals::M33Qtimer6 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::M33Qtimer6::from_bits(val as u8)
    }
    #[doc = "QTIMER6 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_qtimer6(&mut self, val: super::vals::M33Qtimer6) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "QTIMER7 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_qtimer7(&self) -> super::vals::M33Qtimer7 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::M33Qtimer7::from_bits(val as u8)
    }
    #[doc = "QTIMER7 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_qtimer7(&mut self, val: super::vals::M33Qtimer7) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "QTIMER8 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_qtimer8(&self) -> super::vals::M33Qtimer8 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::M33Qtimer8::from_bits(val as u8)
    }
    #[doc = "QTIMER8 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_qtimer8(&mut self, val: super::vals::M33Qtimer8) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "I3C2 debug halted mode with M33"]
    #[must_use]
    #[inline(always)]
    pub const fn m33_i3c2(&self) -> super::vals::M33I3c2 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::M33I3c2::from_bits(val as u8)
    }
    #[doc = "I3C2 debug halted mode with M33"]
    #[inline(always)]
    pub const fn set_m33_i3c2(&mut self, val: super::vals::M33I3c2) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "SINC1 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_sinc1(&self) -> super::vals::M7Sinc1 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::M7Sinc1::from_bits(val as u8)
    }
    #[doc = "SINC1 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_sinc1(&mut self, val: super::vals::M7Sinc1) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SINC2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_sinc2(&self) -> super::vals::M7Sinc2 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::M7Sinc2::from_bits(val as u8)
    }
    #[doc = "SINC2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_sinc2(&mut self, val: super::vals::M7Sinc2) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "SINC3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_sinc3(&self) -> super::vals::M7Sinc3 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::M7Sinc3::from_bits(val as u8)
    }
    #[doc = "SINC3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_sinc3(&mut self, val: super::vals::M7Sinc3) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "QTIMER1 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_qtimer1(&self) -> super::vals::M7Qtimer1 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::M7Qtimer1::from_bits(val as u8)
    }
    #[doc = "QTIMER1 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_qtimer1(&mut self, val: super::vals::M7Qtimer1) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "QTIMER2 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_qtimer2(&self) -> super::vals::M7Qtimer2 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::M7Qtimer2::from_bits(val as u8)
    }
    #[doc = "QTIMER2 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_qtimer2(&mut self, val: super::vals::M7Qtimer2) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "QTIMER3 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_qtimer3(&self) -> super::vals::M7Qtimer3 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::M7Qtimer3::from_bits(val as u8)
    }
    #[doc = "QTIMER3 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_qtimer3(&mut self, val: super::vals::M7Qtimer3) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "QTIMER4 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_qtimer4(&self) -> super::vals::M7Qtimer4 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::M7Qtimer4::from_bits(val as u8)
    }
    #[doc = "QTIMER4 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_qtimer4(&mut self, val: super::vals::M7Qtimer4) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "QTIMER5 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_qtimer5(&self) -> super::vals::M7Qtimer5 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::M7Qtimer5::from_bits(val as u8)
    }
    #[doc = "QTIMER5 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_qtimer5(&mut self, val: super::vals::M7Qtimer5) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_qtimer6(&self) -> super::vals::M7Qtimer6 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::M7Qtimer6::from_bits(val as u8)
    }
    #[doc = "debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_qtimer6(&mut self, val: super::vals::M7Qtimer6) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "QTIMER7 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_qtimer7(&self) -> super::vals::M7Qtimer7 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::M7Qtimer7::from_bits(val as u8)
    }
    #[doc = "QTIMER7 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_qtimer7(&mut self, val: super::vals::M7Qtimer7) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "QTIMER8 debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_qtimer8(&self) -> super::vals::M7Qtimer8 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::M7Qtimer8::from_bits(val as u8)
    }
    #[doc = "QTIMER8 debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_qtimer8(&mut self, val: super::vals::M7Qtimer8) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "debug halted mode with M7"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_i3c2(&self) -> super::vals::M7I3c2 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::M7I3c2::from_bits(val as u8)
    }
    #[doc = "debug halted mode with M7"]
    #[inline(always)]
    pub const fn set_m7_i3c2(&mut self, val: super::vals::M7I3c2) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
}
impl Default for IpgDebug3 {
    #[inline(always)]
    fn default() -> IpgDebug3 {
        IpgDebug3(0u64 as u32)
    }
}
impl core::fmt::Debug for IpgDebug3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IpgDebug3")
            .field("m33_sinc1", &self.m33_sinc1())
            .field("m33_sinc2", &self.m33_sinc2())
            .field("m33_sinc3", &self.m33_sinc3())
            .field("m33_qtimer1", &self.m33_qtimer1())
            .field("m33_qtimer2", &self.m33_qtimer2())
            .field("m33_qtimer3", &self.m33_qtimer3())
            .field("m33_qtimer4", &self.m33_qtimer4())
            .field("m33_qtimer5", &self.m33_qtimer5())
            .field("m33_qtimer6", &self.m33_qtimer6())
            .field("m33_qtimer7", &self.m33_qtimer7())
            .field("m33_qtimer8", &self.m33_qtimer8())
            .field("m33_i3c2", &self.m33_i3c2())
            .field("m7_sinc1", &self.m7_sinc1())
            .field("m7_sinc2", &self.m7_sinc2())
            .field("m7_sinc3", &self.m7_sinc3())
            .field("m7_qtimer1", &self.m7_qtimer1())
            .field("m7_qtimer2", &self.m7_qtimer2())
            .field("m7_qtimer3", &self.m7_qtimer3())
            .field("m7_qtimer4", &self.m7_qtimer4())
            .field("m7_qtimer5", &self.m7_qtimer5())
            .field("m7_qtimer6", &self.m7_qtimer6())
            .field("m7_qtimer7", &self.m7_qtimer7())
            .field("m7_qtimer8", &self.m7_qtimer8())
            .field("m7_i3c2", &self.m7_i3c2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IpgDebug3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IpgDebug3 {
            m33_sinc1: super::vals::M33Sinc1,
            m33_sinc2: super::vals::M33Sinc2,
            m33_sinc3: super::vals::M33Sinc3,
            m33_qtimer1: super::vals::M33Qtimer1,
            m33_qtimer2: super::vals::M33Qtimer2,
            m33_qtimer3: super::vals::M33Qtimer3,
            m33_qtimer4: super::vals::M33Qtimer4,
            m33_qtimer5: super::vals::M33Qtimer5,
            m33_qtimer6: super::vals::M33Qtimer6,
            m33_qtimer7: super::vals::M33Qtimer7,
            m33_qtimer8: super::vals::M33Qtimer8,
            m33_i3c2: super::vals::M33I3c2,
            m7_sinc1: super::vals::M7Sinc1,
            m7_sinc2: super::vals::M7Sinc2,
            m7_sinc3: super::vals::M7Sinc3,
            m7_qtimer1: super::vals::M7Qtimer1,
            m7_qtimer2: super::vals::M7Qtimer2,
            m7_qtimer3: super::vals::M7Qtimer3,
            m7_qtimer4: super::vals::M7Qtimer4,
            m7_qtimer5: super::vals::M7Qtimer5,
            m7_qtimer6: super::vals::M7Qtimer6,
            m7_qtimer7: super::vals::M7Qtimer7,
            m7_qtimer8: super::vals::M7Qtimer8,
            m7_i3c2: super::vals::M7I3c2,
        }
        let proxy = IpgDebug3 {
            m33_sinc1: self.m33_sinc1(),
            m33_sinc2: self.m33_sinc2(),
            m33_sinc3: self.m33_sinc3(),
            m33_qtimer1: self.m33_qtimer1(),
            m33_qtimer2: self.m33_qtimer2(),
            m33_qtimer3: self.m33_qtimer3(),
            m33_qtimer4: self.m33_qtimer4(),
            m33_qtimer5: self.m33_qtimer5(),
            m33_qtimer6: self.m33_qtimer6(),
            m33_qtimer7: self.m33_qtimer7(),
            m33_qtimer8: self.m33_qtimer8(),
            m33_i3c2: self.m33_i3c2(),
            m7_sinc1: self.m7_sinc1(),
            m7_sinc2: self.m7_sinc2(),
            m7_sinc3: self.m7_sinc3(),
            m7_qtimer1: self.m7_qtimer1(),
            m7_qtimer2: self.m7_qtimer2(),
            m7_qtimer3: self.m7_qtimer3(),
            m7_qtimer4: self.m7_qtimer4(),
            m7_qtimer5: self.m7_qtimer5(),
            m7_qtimer6: self.m7_qtimer6(),
            m7_qtimer7: self.m7_qtimer7(),
            m7_qtimer8: self.m7_qtimer8(),
            m7_i3c2: self.m7_i3c2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "LPIT trigger input select register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpitTrigSel(pub u32);
impl LpitTrigSel {
    #[doc = "LPIT1 TRIG0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit1_trig0_input_sel(&self) -> super::vals::Lpit1Trig0InputSel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Lpit1Trig0InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT1 TRIG0 input select"]
    #[inline(always)]
    pub const fn set_lpit1_trig0_input_sel(&mut self, val: super::vals::Lpit1Trig0InputSel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "LPIT1 TRIG1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit1_trig1_input_sel(&self) -> super::vals::Lpit1Trig1InputSel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Lpit1Trig1InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT1 TRIG1 input select"]
    #[inline(always)]
    pub const fn set_lpit1_trig1_input_sel(&mut self, val: super::vals::Lpit1Trig1InputSel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "LPIT1 TRIG2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit1_trig2_input_sel(&self) -> super::vals::Lpit1Trig2InputSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lpit1Trig2InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT1 TRIG2 input select"]
    #[inline(always)]
    pub const fn set_lpit1_trig2_input_sel(&mut self, val: super::vals::Lpit1Trig2InputSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LPIT1 TRIG3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit1_trig3_input_sel(&self) -> super::vals::Lpit1Trig3InputSel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lpit1Trig3InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT1 TRIG3 input select"]
    #[inline(always)]
    pub const fn set_lpit1_trig3_input_sel(&mut self, val: super::vals::Lpit1Trig3InputSel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "LPIT2 TRIG0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit2_trig0_input_sel(&self) -> super::vals::Lpit2Trig0InputSel {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Lpit2Trig0InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT2 TRIG0 input select"]
    #[inline(always)]
    pub const fn set_lpit2_trig0_input_sel(&mut self, val: super::vals::Lpit2Trig0InputSel) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "LPIT2 TRIG1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit2_trig1_input_sel(&self) -> super::vals::Lpit2Trig1InputSel {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Lpit2Trig1InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT2 TRIG1 input select"]
    #[inline(always)]
    pub const fn set_lpit2_trig1_input_sel(&mut self, val: super::vals::Lpit2Trig1InputSel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "LPIT2 TRIG2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit2_trig2_input_sel(&self) -> super::vals::Lpit2Trig2InputSel {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Lpit2Trig2InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT2 TRIG2 input select"]
    #[inline(always)]
    pub const fn set_lpit2_trig2_input_sel(&mut self, val: super::vals::Lpit2Trig2InputSel) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "LPIT2 TRIG3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit2_trig3_input_sel(&self) -> super::vals::Lpit2Trig3InputSel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Lpit2Trig3InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT2 TRIG3 input select"]
    #[inline(always)]
    pub const fn set_lpit2_trig3_input_sel(&mut self, val: super::vals::Lpit2Trig3InputSel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "LPIT3 TRIG0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit3_trig0_input_sel(&self) -> super::vals::Lpit3Trig0InputSel {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Lpit3Trig0InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT3 TRIG0 input select"]
    #[inline(always)]
    pub const fn set_lpit3_trig0_input_sel(&mut self, val: super::vals::Lpit3Trig0InputSel) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "LPIT3 TRIG1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit3_trig1_input_sel(&self) -> super::vals::Lpit3Trig1InputSel {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Lpit3Trig1InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT3 TRIG1 input select"]
    #[inline(always)]
    pub const fn set_lpit3_trig1_input_sel(&mut self, val: super::vals::Lpit3Trig1InputSel) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "LPIT3 TRIG2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit3_trig2_input_sel(&self) -> super::vals::Lpit3Trig2InputSel {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Lpit3Trig2InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT3 TRIG2 input select"]
    #[inline(always)]
    pub const fn set_lpit3_trig2_input_sel(&mut self, val: super::vals::Lpit3Trig2InputSel) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "LPIT3 TRIG3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn lpit3_trig3_input_sel(&self) -> super::vals::Lpit3Trig3InputSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Lpit3Trig3InputSel::from_bits(val as u8)
    }
    #[doc = "LPIT3 TRIG3 input select"]
    #[inline(always)]
    pub const fn set_lpit3_trig3_input_sel(&mut self, val: super::vals::Lpit3Trig3InputSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
}
impl Default for LpitTrigSel {
    #[inline(always)]
    fn default() -> LpitTrigSel {
        LpitTrigSel(0u64 as u32)
    }
}
impl core::fmt::Debug for LpitTrigSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpitTrigSel")
            .field("lpit1_trig0_input_sel", &self.lpit1_trig0_input_sel())
            .field("lpit1_trig1_input_sel", &self.lpit1_trig1_input_sel())
            .field("lpit1_trig2_input_sel", &self.lpit1_trig2_input_sel())
            .field("lpit1_trig3_input_sel", &self.lpit1_trig3_input_sel())
            .field("lpit2_trig0_input_sel", &self.lpit2_trig0_input_sel())
            .field("lpit2_trig1_input_sel", &self.lpit2_trig1_input_sel())
            .field("lpit2_trig2_input_sel", &self.lpit2_trig2_input_sel())
            .field("lpit2_trig3_input_sel", &self.lpit2_trig3_input_sel())
            .field("lpit3_trig0_input_sel", &self.lpit3_trig0_input_sel())
            .field("lpit3_trig1_input_sel", &self.lpit3_trig1_input_sel())
            .field("lpit3_trig2_input_sel", &self.lpit3_trig2_input_sel())
            .field("lpit3_trig3_input_sel", &self.lpit3_trig3_input_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpitTrigSel {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct LpitTrigSel {
            lpit1_trig0_input_sel: super::vals::Lpit1Trig0InputSel,
            lpit1_trig1_input_sel: super::vals::Lpit1Trig1InputSel,
            lpit1_trig2_input_sel: super::vals::Lpit1Trig2InputSel,
            lpit1_trig3_input_sel: super::vals::Lpit1Trig3InputSel,
            lpit2_trig0_input_sel: super::vals::Lpit2Trig0InputSel,
            lpit2_trig1_input_sel: super::vals::Lpit2Trig1InputSel,
            lpit2_trig2_input_sel: super::vals::Lpit2Trig2InputSel,
            lpit2_trig3_input_sel: super::vals::Lpit2Trig3InputSel,
            lpit3_trig0_input_sel: super::vals::Lpit3Trig0InputSel,
            lpit3_trig1_input_sel: super::vals::Lpit3Trig1InputSel,
            lpit3_trig2_input_sel: super::vals::Lpit3Trig2InputSel,
            lpit3_trig3_input_sel: super::vals::Lpit3Trig3InputSel,
        }
        let proxy = LpitTrigSel {
            lpit1_trig0_input_sel: self.lpit1_trig0_input_sel(),
            lpit1_trig1_input_sel: self.lpit1_trig1_input_sel(),
            lpit1_trig2_input_sel: self.lpit1_trig2_input_sel(),
            lpit1_trig3_input_sel: self.lpit1_trig3_input_sel(),
            lpit2_trig0_input_sel: self.lpit2_trig0_input_sel(),
            lpit2_trig1_input_sel: self.lpit2_trig1_input_sel(),
            lpit2_trig2_input_sel: self.lpit2_trig2_input_sel(),
            lpit2_trig3_input_sel: self.lpit2_trig3_input_sel(),
            lpit3_trig0_input_sel: self.lpit3_trig0_input_sel(),
            lpit3_trig1_input_sel: self.lpit3_trig1_input_sel(),
            lpit3_trig2_input_sel: self.lpit3_trig2_input_sel(),
            lpit3_trig3_input_sel: self.lpit3_trig3_input_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "M7 NMI interrupt clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M7NmiClr(pub u32);
impl M7NmiClr {
    #[doc = "Clear CM7 NMI holding register"]
    #[must_use]
    #[inline(always)]
    pub const fn m7_nmi_clear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear CM7 NMI holding register"]
    #[inline(always)]
    pub const fn set_m7_nmi_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for M7NmiClr {
    #[inline(always)]
    fn default() -> M7NmiClr {
        M7NmiClr(0u64 as u32)
    }
}
impl core::fmt::Debug for M7NmiClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M7NmiClr")
            .field("m7_nmi_clear", &self.m7_nmi_clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for M7NmiClr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct M7NmiClr {
            m7_nmi_clear: bool,
        }
        let proxy = M7NmiClr {
            m7_nmi_clear: self.m7_nmi_clear(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Miscellaneous control register of IO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscIoCtrl(pub u32);
impl MiscIoCtrl {
    #[doc = "Disable I3C on-chip strong pull for I3C2"]
    #[must_use]
    #[inline(always)]
    pub const fn i3c_on_chip_strong_pull_dis(&self) -> super::vals::I3cOnChipStrongPullDis {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::I3cOnChipStrongPullDis::from_bits(val as u8)
    }
    #[doc = "Disable I3C on-chip strong pull for I3C2"]
    #[inline(always)]
    pub const fn set_i3c_on_chip_strong_pull_dis(
        &mut self,
        val: super::vals::I3cOnChipStrongPullDis,
    ) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_ad_high_range(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17"]
    #[inline(always)]
    pub const fn set_gpio_ad_high_range(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_ad_low_range(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_AD IO bank supply voltage range selection for GPIO_AD_00 to GPIO_AD_17"]
    #[inline(always)]
    pub const fn set_gpio_ad_low_range(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "GPIO_EMC_B1 IO bank supply voltage detector sleep mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn suplydet_emc1_sleep(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC_B1 IO bank supply voltage detector sleep mode enable"]
    #[inline(always)]
    pub const fn set_suplydet_emc1_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "GPIO_EMC_B2 IO bank supply voltage detector sleep mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn suplydet_emc2_sleep(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_EMC_B2 IO bank supply voltage detector sleep mode enable"]
    #[inline(always)]
    pub const fn set_suplydet_emc2_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "GPIO_SD_B1 IO bank supply voltage detector sleep mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn suplydet_sd1_sleep(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD_B1 IO bank supply voltage detector sleep mode enable"]
    #[inline(always)]
    pub const fn set_suplydet_sd1_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO_SD_B2 IO bank supply voltage detector sleep mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn suplydet_sd2_sleep(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD_B2 IO bank supply voltage detector sleep mode enable"]
    #[inline(always)]
    pub const fn set_suplydet_sd2_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO_GPIO_B1 IO bank supply voltage detector sleep mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn suplydet_gpio_b1_sleep(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_GPIO_B1 IO bank supply voltage detector sleep mode enable"]
    #[inline(always)]
    pub const fn set_suplydet_gpio_b1_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "GPIO_GPIO_B1 IO bank supply voltage detector sleep mode enable"]
    #[must_use]
    #[inline(always)]
    pub const fn suplydet_gpio_b2_sleep(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_GPIO_B1 IO bank supply voltage detector sleep mode enable"]
    #[inline(always)]
    pub const fn set_suplydet_gpio_b2_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "ECAT_LINK_ACT\\[0\\] polarity control"]
    #[must_use]
    #[inline(always)]
    pub const fn ecat_link_act0_pol(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "ECAT_LINK_ACT\\[0\\] polarity control"]
    #[inline(always)]
    pub const fn set_ecat_link_act0_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "ECAT_LINK_ACT\\[1\\] polarity control"]
    #[must_use]
    #[inline(always)]
    pub const fn ecat_link_act1_pol(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "ECAT_LINK_ACT\\[1\\] polarity control"]
    #[inline(always)]
    pub const fn set_ecat_link_act1_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for MiscIoCtrl {
    #[inline(always)]
    fn default() -> MiscIoCtrl {
        MiscIoCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for MiscIoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscIoCtrl")
            .field(
                "i3c_on_chip_strong_pull_dis",
                &self.i3c_on_chip_strong_pull_dis(),
            )
            .field("gpio_ad_high_range", &self.gpio_ad_high_range())
            .field("gpio_ad_low_range", &self.gpio_ad_low_range())
            .field("suplydet_emc1_sleep", &self.suplydet_emc1_sleep())
            .field("suplydet_emc2_sleep", &self.suplydet_emc2_sleep())
            .field("suplydet_sd1_sleep", &self.suplydet_sd1_sleep())
            .field("suplydet_sd2_sleep", &self.suplydet_sd2_sleep())
            .field("suplydet_gpio_b1_sleep", &self.suplydet_gpio_b1_sleep())
            .field("suplydet_gpio_b2_sleep", &self.suplydet_gpio_b2_sleep())
            .field("ecat_link_act0_pol", &self.ecat_link_act0_pol())
            .field("ecat_link_act1_pol", &self.ecat_link_act1_pol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscIoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MiscIoCtrl {
            i3c_on_chip_strong_pull_dis: super::vals::I3cOnChipStrongPullDis,
            gpio_ad_high_range: bool,
            gpio_ad_low_range: bool,
            suplydet_emc1_sleep: bool,
            suplydet_emc2_sleep: bool,
            suplydet_sd1_sleep: bool,
            suplydet_sd2_sleep: bool,
            suplydet_gpio_b1_sleep: bool,
            suplydet_gpio_b2_sleep: bool,
            ecat_link_act0_pol: bool,
            ecat_link_act1_pol: bool,
        }
        let proxy = MiscIoCtrl {
            i3c_on_chip_strong_pull_dis: self.i3c_on_chip_strong_pull_dis(),
            gpio_ad_high_range: self.gpio_ad_high_range(),
            gpio_ad_low_range: self.gpio_ad_low_range(),
            suplydet_emc1_sleep: self.suplydet_emc1_sleep(),
            suplydet_emc2_sleep: self.suplydet_emc2_sleep(),
            suplydet_sd1_sleep: self.suplydet_sd1_sleep(),
            suplydet_sd2_sleep: self.suplydet_sd2_sleep(),
            suplydet_gpio_b1_sleep: self.suplydet_gpio_b1_sleep(),
            suplydet_gpio_b2_sleep: self.suplydet_gpio_b2_sleep(),
            ecat_link_act0_pol: self.ecat_link_act0_pol(),
            ecat_link_act1_pol: self.ecat_link_act1_pol(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC link configuration for port0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcLinkCfg0(pub u32);
impl NetcLinkCfg0 {
    #[doc = "MII protocol selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> super::vals::NetcLinkCfg0MiiProt {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::NetcLinkCfg0MiiProt::from_bits(val as u8)
    }
    #[doc = "MII protocol selection"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: super::vals::NetcLinkCfg0MiiProt) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "IO variant selection"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> super::vals::NetcLinkCfg0IoVar {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::NetcLinkCfg0IoVar::from_bits(val as u8)
    }
    #[doc = "IO variant selection"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: super::vals::NetcLinkCfg0IoVar) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for NetcLinkCfg0 {
    #[inline(always)]
    fn default() -> NetcLinkCfg0 {
        NetcLinkCfg0(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcLinkCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcLinkCfg0")
            .field("mii_prot", &self.mii_prot())
            .field("io_var", &self.io_var())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcLinkCfg0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcLinkCfg0 {
            mii_prot: super::vals::NetcLinkCfg0MiiProt,
            io_var: super::vals::NetcLinkCfg0IoVar,
        }
        let proxy = NetcLinkCfg0 {
            mii_prot: self.mii_prot(),
            io_var: self.io_var(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC link configuration for port1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcLinkCfg1(pub u32);
impl NetcLinkCfg1 {
    #[doc = "MII protocol selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> super::vals::NetcLinkCfg1MiiProt {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::NetcLinkCfg1MiiProt::from_bits(val as u8)
    }
    #[doc = "MII protocol selection"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: super::vals::NetcLinkCfg1MiiProt) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "IO variant selection"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> super::vals::NetcLinkCfg1IoVar {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::NetcLinkCfg1IoVar::from_bits(val as u8)
    }
    #[doc = "IO variant selection"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: super::vals::NetcLinkCfg1IoVar) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
}
impl Default for NetcLinkCfg1 {
    #[inline(always)]
    fn default() -> NetcLinkCfg1 {
        NetcLinkCfg1(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcLinkCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcLinkCfg1")
            .field("mii_prot", &self.mii_prot())
            .field("io_var", &self.io_var())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcLinkCfg1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcLinkCfg1 {
            mii_prot: super::vals::NetcLinkCfg1MiiProt,
            io_var: super::vals::NetcLinkCfg1IoVar,
        }
        let proxy = NetcLinkCfg1 {
            mii_prot: self.mii_prot(),
            io_var: self.io_var(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC link configuration for port2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcLinkCfg2(pub u32);
impl NetcLinkCfg2 {
    #[doc = "MII protocol selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> super::vals::NetcLinkCfg2MiiProt {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::NetcLinkCfg2MiiProt::from_bits(val as u8)
    }
    #[doc = "MII protocol selection"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: super::vals::NetcLinkCfg2MiiProt) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "IO variant selection"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> super::vals::NetcLinkCfg2IoVar {
        let val = (self.0 >> 16usize) & 0x0f;
        super::vals::NetcLinkCfg2IoVar::from_bits(val as u8)
    }
    #[doc = "IO variant selection"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: super::vals::NetcLinkCfg2IoVar) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    #[must_use]
    #[inline(always)]
    pub const fn revmii_rate(&self) -> super::vals::RevmiiRate {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::RevmiiRate::from_bits(val as u8)
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    #[inline(always)]
    pub const fn set_revmii_rate(&mut self, val: super::vals::RevmiiRate) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "RevMII selection"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RevMII selection"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NetcLinkCfg2 {
    #[inline(always)]
    fn default() -> NetcLinkCfg2 {
        NetcLinkCfg2(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcLinkCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcLinkCfg2")
            .field("mii_prot", &self.mii_prot())
            .field("io_var", &self.io_var())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcLinkCfg2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcLinkCfg2 {
            mii_prot: super::vals::NetcLinkCfg2MiiProt,
            io_var: super::vals::NetcLinkCfg2IoVar,
            revmii_rate: super::vals::RevmiiRate,
            revmii: bool,
        }
        let proxy = NetcLinkCfg2 {
            mii_prot: self.mii_prot(),
            io_var: self.io_var(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC link configuration for port3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcLinkCfg3(pub u32);
impl NetcLinkCfg3 {
    #[doc = "MII protocol selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "MII protocol selection"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "IO variant selection"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "IO variant selection"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    #[must_use]
    #[inline(always)]
    pub const fn revmii_rate(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    #[inline(always)]
    pub const fn set_revmii_rate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "RevMII selection"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RevMII selection"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NetcLinkCfg3 {
    #[inline(always)]
    fn default() -> NetcLinkCfg3 {
        NetcLinkCfg3(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcLinkCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcLinkCfg3")
            .field("mii_prot", &self.mii_prot())
            .field("io_var", &self.io_var())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcLinkCfg3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcLinkCfg3 {
            mii_prot: u8,
            io_var: u8,
            revmii_rate: bool,
            revmii: bool,
        }
        let proxy = NetcLinkCfg3 {
            mii_prot: self.mii_prot(),
            io_var: self.io_var(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC link configuration for port4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcLinkCfg4(pub u32);
impl NetcLinkCfg4 {
    #[doc = "MII protocol selection"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "MII protocol selection"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "IO variant selection"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "IO variant selection"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    #[must_use]
    #[inline(always)]
    pub const fn revmii_rate(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "When REVMII=1 and MII_PROT=MII, this bit configures RevMII rates, otherwise this field has no meaning."]
    #[inline(always)]
    pub const fn set_revmii_rate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "RevMII selection"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RevMII selection"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for NetcLinkCfg4 {
    #[inline(always)]
    fn default() -> NetcLinkCfg4 {
        NetcLinkCfg4(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcLinkCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcLinkCfg4")
            .field("mii_prot", &self.mii_prot())
            .field("io_var", &self.io_var())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcLinkCfg4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcLinkCfg4 {
            mii_prot: u8,
            io_var: u8,
            revmii_rate: bool,
            revmii: bool,
        }
        let proxy = NetcLinkCfg4 {
            mii_prot: self.mii_prot(),
            io_var: self.io_var(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC Port miscellaneous configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcPortMiscCfg(pub u32);
impl NetcPortMiscCfg {
    #[doc = "Port0 RMII Reference clock direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn port0_rmii_ref_clk_dir(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port0 RMII Reference clock direction control"]
    #[inline(always)]
    pub const fn set_port0_rmii_ref_clk_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port1 RMII Reference clock direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn port1_rmii_ref_clk_dir(&self) -> super::vals::Port1RmiiRefClkDir {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Port1RmiiRefClkDir::from_bits(val as u8)
    }
    #[doc = "Port1 RMII Reference clock direction control"]
    #[inline(always)]
    pub const fn set_port1_rmii_ref_clk_dir(&mut self, val: super::vals::Port1RmiiRefClkDir) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Port2 RMII Reference clock direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn port2_rmii_ref_clk_dir(&self) -> super::vals::Port2RmiiRefClkDir {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Port2RmiiRefClkDir::from_bits(val as u8)
    }
    #[doc = "Port2 RMII Reference clock direction control"]
    #[inline(always)]
    pub const fn set_port2_rmii_ref_clk_dir(&mut self, val: super::vals::Port2RmiiRefClkDir) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Port3 RMII Reference clock direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn port3_rmii_ref_clk_dir(&self) -> super::vals::Port3RmiiRefClkDir {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Port3RmiiRefClkDir::from_bits(val as u8)
    }
    #[doc = "Port3 RMII Reference clock direction control"]
    #[inline(always)]
    pub const fn set_port3_rmii_ref_clk_dir(&mut self, val: super::vals::Port3RmiiRefClkDir) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Port4 RMII Reference clock direction control"]
    #[must_use]
    #[inline(always)]
    pub const fn port4_rmii_ref_clk_dir(&self) -> super::vals::Port4RmiiRefClkDir {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Port4RmiiRefClkDir::from_bits(val as u8)
    }
    #[doc = "Port4 RMII Reference clock direction control"]
    #[inline(always)]
    pub const fn set_port4_rmii_ref_clk_dir(&mut self, val: super::vals::Port4RmiiRefClkDir) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Default value for IERB NETCRR\\[LOCK\\] bit. Determines write accessibility of IERB registers after power-on-reset"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_ierb_lock(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Default value for IERB NETCRR\\[LOCK\\] bit. Determines write accessibility of IERB registers after power-on-reset"]
    #[inline(always)]
    pub const fn set_cfg_ierb_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1588 timer external clock selection"]
    #[must_use]
    #[inline(always)]
    pub const fn tmr_ext_clk_sel(&self) -> super::vals::TmrExtClkSel {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::TmrExtClkSel::from_bits(val as u8)
    }
    #[doc = "1588 timer external clock selection"]
    #[inline(always)]
    pub const fn set_tmr_ext_clk_sel(&mut self, val: super::vals::TmrExtClkSel) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "1588 timer trigger1 input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn tmr_trig1_sel(&self) -> super::vals::TmrTrig1Sel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::TmrTrig1Sel::from_bits(val as u8)
    }
    #[doc = "1588 timer trigger1 input selection"]
    #[inline(always)]
    pub const fn set_tmr_trig1_sel(&mut self, val: super::vals::TmrTrig1Sel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "1588 timer trigger2 input selection"]
    #[must_use]
    #[inline(always)]
    pub const fn tmr_trig2_sel(&self) -> super::vals::TmrTrig2Sel {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::TmrTrig2Sel::from_bits(val as u8)
    }
    #[doc = "1588 timer trigger2 input selection"]
    #[inline(always)]
    pub const fn set_tmr_trig2_sel(&mut self, val: super::vals::TmrTrig2Sel) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
}
impl Default for NetcPortMiscCfg {
    #[inline(always)]
    fn default() -> NetcPortMiscCfg {
        NetcPortMiscCfg(0u64 as u32)
    }
}
impl core::fmt::Debug for NetcPortMiscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcPortMiscCfg")
            .field("port0_rmii_ref_clk_dir", &self.port0_rmii_ref_clk_dir())
            .field("port1_rmii_ref_clk_dir", &self.port1_rmii_ref_clk_dir())
            .field("port2_rmii_ref_clk_dir", &self.port2_rmii_ref_clk_dir())
            .field("port3_rmii_ref_clk_dir", &self.port3_rmii_ref_clk_dir())
            .field("port4_rmii_ref_clk_dir", &self.port4_rmii_ref_clk_dir())
            .field("cfg_ierb_lock", &self.cfg_ierb_lock())
            .field("tmr_ext_clk_sel", &self.tmr_ext_clk_sel())
            .field("tmr_trig1_sel", &self.tmr_trig1_sel())
            .field("tmr_trig2_sel", &self.tmr_trig2_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcPortMiscCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcPortMiscCfg {
            port0_rmii_ref_clk_dir: bool,
            port1_rmii_ref_clk_dir: super::vals::Port1RmiiRefClkDir,
            port2_rmii_ref_clk_dir: super::vals::Port2RmiiRefClkDir,
            port3_rmii_ref_clk_dir: super::vals::Port3RmiiRefClkDir,
            port4_rmii_ref_clk_dir: super::vals::Port4RmiiRefClkDir,
            cfg_ierb_lock: bool,
            tmr_ext_clk_sel: super::vals::TmrExtClkSel,
            tmr_trig1_sel: super::vals::TmrTrig1Sel,
            tmr_trig2_sel: super::vals::TmrTrig2Sel,
        }
        let proxy = NetcPortMiscCfg {
            port0_rmii_ref_clk_dir: self.port0_rmii_ref_clk_dir(),
            port1_rmii_ref_clk_dir: self.port1_rmii_ref_clk_dir(),
            port2_rmii_ref_clk_dir: self.port2_rmii_ref_clk_dir(),
            port3_rmii_ref_clk_dir: self.port3_rmii_ref_clk_dir(),
            port4_rmii_ref_clk_dir: self.port4_rmii_ref_clk_dir(),
            cfg_ierb_lock: self.cfg_ierb_lock(),
            tmr_ext_clk_sel: self.tmr_ext_clk_sel(),
            tmr_trig1_sel: self.tmr_trig1_sel(),
            tmr_trig2_sel: self.tmr_trig2_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcRevmiiDll0(pub u32);
impl NetcRevmiiDll0 {
    #[doc = "Delay target of slave delay line"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_target(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay target of slave delay line"]
    #[inline(always)]
    pub const fn set_dly_target(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reference delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Reference delay line lock flag"]
    #[inline(always)]
    pub const fn set_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn slv_lock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave delay line lock flag"]
    #[inline(always)]
    pub const fn set_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for NetcRevmiiDll0 {
    #[inline(always)]
    fn default() -> NetcRevmiiDll0 {
        NetcRevmiiDll0(12u64 as u32)
    }
}
impl core::fmt::Debug for NetcRevmiiDll0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcRevmiiDll0")
            .field("dly_target", &self.dly_target())
            .field("ref_lock", &self.ref_lock())
            .field("slv_lock", &self.slv_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcRevmiiDll0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcRevmiiDll0 {
            dly_target: u8,
            ref_lock: bool,
            slv_lock: bool,
        }
        let proxy = NetcRevmiiDll0 {
            dly_target: self.dly_target(),
            ref_lock: self.ref_lock(),
            slv_lock: self.slv_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcRevmiiDll1(pub u32);
impl NetcRevmiiDll1 {
    #[doc = "Delay target of slave delay line"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_target(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay target of slave delay line"]
    #[inline(always)]
    pub const fn set_dly_target(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reference delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Reference delay line lock flag"]
    #[inline(always)]
    pub const fn set_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn slv_lock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave delay line lock flag"]
    #[inline(always)]
    pub const fn set_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for NetcRevmiiDll1 {
    #[inline(always)]
    fn default() -> NetcRevmiiDll1 {
        NetcRevmiiDll1(12u64 as u32)
    }
}
impl core::fmt::Debug for NetcRevmiiDll1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcRevmiiDll1")
            .field("dly_target", &self.dly_target())
            .field("ref_lock", &self.ref_lock())
            .field("slv_lock", &self.slv_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcRevmiiDll1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcRevmiiDll1 {
            dly_target: u8,
            ref_lock: bool,
            slv_lock: bool,
        }
        let proxy = NetcRevmiiDll1 {
            dly_target: self.dly_target(),
            ref_lock: self.ref_lock(),
            slv_lock: self.slv_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcRevmiiDll2(pub u32);
impl NetcRevmiiDll2 {
    #[doc = "Delay target of slave delay line"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_target(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay target of slave delay line"]
    #[inline(always)]
    pub const fn set_dly_target(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reference delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Reference delay line lock flag"]
    #[inline(always)]
    pub const fn set_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn slv_lock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave delay line lock flag"]
    #[inline(always)]
    pub const fn set_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for NetcRevmiiDll2 {
    #[inline(always)]
    fn default() -> NetcRevmiiDll2 {
        NetcRevmiiDll2(12u64 as u32)
    }
}
impl core::fmt::Debug for NetcRevmiiDll2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcRevmiiDll2")
            .field("dly_target", &self.dly_target())
            .field("ref_lock", &self.ref_lock())
            .field("slv_lock", &self.slv_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcRevmiiDll2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcRevmiiDll2 {
            dly_target: u8,
            ref_lock: bool,
            slv_lock: bool,
        }
        let proxy = NetcRevmiiDll2 {
            dly_target: self.dly_target(),
            ref_lock: self.ref_lock(),
            slv_lock: self.slv_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcRevmiiDll3(pub u32);
impl NetcRevmiiDll3 {
    #[doc = "Delay target of slave delay line"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_target(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay target of slave delay line"]
    #[inline(always)]
    pub const fn set_dly_target(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reference delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Reference delay line lock flag"]
    #[inline(always)]
    pub const fn set_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn slv_lock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave delay line lock flag"]
    #[inline(always)]
    pub const fn set_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for NetcRevmiiDll3 {
    #[inline(always)]
    fn default() -> NetcRevmiiDll3 {
        NetcRevmiiDll3(12u64 as u32)
    }
}
impl core::fmt::Debug for NetcRevmiiDll3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcRevmiiDll3")
            .field("dly_target", &self.dly_target())
            .field("ref_lock", &self.ref_lock())
            .field("slv_lock", &self.slv_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcRevmiiDll3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcRevmiiDll3 {
            dly_target: u8,
            ref_lock: bool,
            slv_lock: bool,
        }
        let proxy = NetcRevmiiDll3 {
            dly_target: self.dly_target(),
            ref_lock: self.ref_lock(),
            slv_lock: self.slv_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC RevMII RGMII delay line configuration for port4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcRevmiiDll4(pub u32);
impl NetcRevmiiDll4 {
    #[doc = "Delay target of slave delay line"]
    #[must_use]
    #[inline(always)]
    pub const fn dly_target(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Delay target of slave delay line"]
    #[inline(always)]
    pub const fn set_dly_target(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Reference delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn ref_lock(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Reference delay line lock flag"]
    #[inline(always)]
    pub const fn set_ref_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave delay line lock flag"]
    #[must_use]
    #[inline(always)]
    pub const fn slv_lock(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave delay line lock flag"]
    #[inline(always)]
    pub const fn set_slv_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for NetcRevmiiDll4 {
    #[inline(always)]
    fn default() -> NetcRevmiiDll4 {
        NetcRevmiiDll4(12u64 as u32)
    }
}
impl core::fmt::Debug for NetcRevmiiDll4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NetcRevmiiDll4")
            .field("dly_target", &self.dly_target())
            .field("ref_lock", &self.ref_lock())
            .field("slv_lock", &self.slv_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NetcRevmiiDll4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NetcRevmiiDll4 {
            dly_target: u8,
            ref_lock: bool,
            slv_lock: bool,
        }
        let proxy = NetcRevmiiDll4 {
            dly_target: self.dly_target(),
            ref_lock: self.ref_lock(),
            slv_lock: self.slv_lock(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Qtimer miscellaneous control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QtimerCtrl1(pub u32);
impl QtimerCtrl1 {
    #[doc = "QTIMER1 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_tmr_cnts_freeze(&self) -> super::vals::Qtimer1TmrCntsFreeze {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer1TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER1 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer1_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer1TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "QTIMER1 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_tmr0_input_sel(&self) -> super::vals::Qtimer1Tmr0InputSel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Qtimer1Tmr0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER1 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer1_tmr0_input_sel(&mut self, val: super::vals::Qtimer1Tmr0InputSel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "QTIMER1 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_tmr1_input_sel(&self) -> super::vals::Qtimer1Tmr1InputSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Qtimer1Tmr1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER1 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer1_tmr1_input_sel(&mut self, val: super::vals::Qtimer1Tmr1InputSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "QTIMER1 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_tmr2_input_sel(&self) -> super::vals::Qtimer1Tmr2InputSel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Qtimer1Tmr2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER1 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer1_tmr2_input_sel(&mut self, val: super::vals::Qtimer1Tmr2InputSel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "QTIMER1 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer1_tmr3_input_sel(&self) -> super::vals::Qtimer1Tmr3InputSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Qtimer1Tmr3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER1 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer1_tmr3_input_sel(&mut self, val: super::vals::Qtimer1Tmr3InputSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "QTIMER2 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_tmr_cnts_freeze(&self) -> super::vals::Qtimer2TmrCntsFreeze {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Qtimer2TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER2 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer2_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer2TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "QTIMER2 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_tmr0_input_sel(&self) -> super::vals::Qtimer2Tmr0InputSel {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Qtimer2Tmr0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER2 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer2_tmr0_input_sel(&mut self, val: super::vals::Qtimer2Tmr0InputSel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "QTIMER2 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_tmr1_input_sel(&self) -> super::vals::Qtimer2Tmr1InputSel {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Qtimer2Tmr1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER2 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer2_tmr1_input_sel(&mut self, val: super::vals::Qtimer2Tmr1InputSel) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "QTIMER2 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_tmr2_input_sel(&self) -> super::vals::Qtimer2Tmr2InputSel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Qtimer2Tmr2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER2 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer2_tmr2_input_sel(&mut self, val: super::vals::Qtimer2Tmr2InputSel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "QTIMER2 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer2_tmr3_input_sel(&self) -> super::vals::Qtimer2Tmr3InputSel {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Qtimer2Tmr3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER2 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer2_tmr3_input_sel(&mut self, val: super::vals::Qtimer2Tmr3InputSel) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "QTIMER3 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_tmr_cnts_freeze(&self) -> super::vals::Qtimer3TmrCntsFreeze {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Qtimer3TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER3 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer3_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer3TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "QTIMER3 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_tmr0_input_sel(&self) -> super::vals::Qtimer3Tmr0InputSel {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Qtimer3Tmr0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER3 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer3_tmr0_input_sel(&mut self, val: super::vals::Qtimer3Tmr0InputSel) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "QTIMER3 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_tmr1_input_sel(&self) -> super::vals::Qtimer3Tmr1InputSel {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Qtimer3Tmr1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER3 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer3_tmr1_input_sel(&mut self, val: super::vals::Qtimer3Tmr1InputSel) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "QTIMER3 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_tmr2_input_sel(&self) -> super::vals::Qtimer3Tmr2InputSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Qtimer3Tmr2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER3 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer3_tmr2_input_sel(&mut self, val: super::vals::Qtimer3Tmr2InputSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "QTIMER3 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer3_tmr3_input_sel(&self) -> super::vals::Qtimer3Tmr3InputSel {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Qtimer3Tmr3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER3 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer3_tmr3_input_sel(&mut self, val: super::vals::Qtimer3Tmr3InputSel) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "QTIMER4 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_tmr_cnts_freeze(&self) -> super::vals::Qtimer4TmrCntsFreeze {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Qtimer4TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER4 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer4_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer4TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "QTIMER4 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_tmr0_input_sel(&self) -> super::vals::Qtimer4Tmr0InputSel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Qtimer4Tmr0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER4 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer4_tmr0_input_sel(&mut self, val: super::vals::Qtimer4Tmr0InputSel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "QTIMER4 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_tmr1_input_sel(&self) -> super::vals::Qtimer4Tmr1InputSel {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Qtimer4Tmr1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER4 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer4_tmr1_input_sel(&mut self, val: super::vals::Qtimer4Tmr1InputSel) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "QTIMER4 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_tmr2_input_sel(&self) -> super::vals::Qtimer4Tmr2InputSel {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Qtimer4Tmr2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER4 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer4_tmr2_input_sel(&mut self, val: super::vals::Qtimer4Tmr2InputSel) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "QTIMER4 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer4_tmr3_input_sel(&self) -> super::vals::Qtimer4Tmr3InputSel {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Qtimer4Tmr3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER4 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer4_tmr3_input_sel(&mut self, val: super::vals::Qtimer4Tmr3InputSel) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for QtimerCtrl1 {
    #[inline(always)]
    fn default() -> QtimerCtrl1 {
        QtimerCtrl1(0u64 as u32)
    }
}
impl core::fmt::Debug for QtimerCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QtimerCtrl1")
            .field("qtimer1_tmr_cnts_freeze", &self.qtimer1_tmr_cnts_freeze())
            .field("qtimer1_tmr0_input_sel", &self.qtimer1_tmr0_input_sel())
            .field("qtimer1_tmr1_input_sel", &self.qtimer1_tmr1_input_sel())
            .field("qtimer1_tmr2_input_sel", &self.qtimer1_tmr2_input_sel())
            .field("qtimer1_tmr3_input_sel", &self.qtimer1_tmr3_input_sel())
            .field("qtimer2_tmr_cnts_freeze", &self.qtimer2_tmr_cnts_freeze())
            .field("qtimer2_tmr0_input_sel", &self.qtimer2_tmr0_input_sel())
            .field("qtimer2_tmr1_input_sel", &self.qtimer2_tmr1_input_sel())
            .field("qtimer2_tmr2_input_sel", &self.qtimer2_tmr2_input_sel())
            .field("qtimer2_tmr3_input_sel", &self.qtimer2_tmr3_input_sel())
            .field("qtimer3_tmr_cnts_freeze", &self.qtimer3_tmr_cnts_freeze())
            .field("qtimer3_tmr0_input_sel", &self.qtimer3_tmr0_input_sel())
            .field("qtimer3_tmr1_input_sel", &self.qtimer3_tmr1_input_sel())
            .field("qtimer3_tmr2_input_sel", &self.qtimer3_tmr2_input_sel())
            .field("qtimer3_tmr3_input_sel", &self.qtimer3_tmr3_input_sel())
            .field("qtimer4_tmr_cnts_freeze", &self.qtimer4_tmr_cnts_freeze())
            .field("qtimer4_tmr0_input_sel", &self.qtimer4_tmr0_input_sel())
            .field("qtimer4_tmr1_input_sel", &self.qtimer4_tmr1_input_sel())
            .field("qtimer4_tmr2_input_sel", &self.qtimer4_tmr2_input_sel())
            .field("qtimer4_tmr3_input_sel", &self.qtimer4_tmr3_input_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for QtimerCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct QtimerCtrl1 {
            qtimer1_tmr_cnts_freeze: super::vals::Qtimer1TmrCntsFreeze,
            qtimer1_tmr0_input_sel: super::vals::Qtimer1Tmr0InputSel,
            qtimer1_tmr1_input_sel: super::vals::Qtimer1Tmr1InputSel,
            qtimer1_tmr2_input_sel: super::vals::Qtimer1Tmr2InputSel,
            qtimer1_tmr3_input_sel: super::vals::Qtimer1Tmr3InputSel,
            qtimer2_tmr_cnts_freeze: super::vals::Qtimer2TmrCntsFreeze,
            qtimer2_tmr0_input_sel: super::vals::Qtimer2Tmr0InputSel,
            qtimer2_tmr1_input_sel: super::vals::Qtimer2Tmr1InputSel,
            qtimer2_tmr2_input_sel: super::vals::Qtimer2Tmr2InputSel,
            qtimer2_tmr3_input_sel: super::vals::Qtimer2Tmr3InputSel,
            qtimer3_tmr_cnts_freeze: super::vals::Qtimer3TmrCntsFreeze,
            qtimer3_tmr0_input_sel: super::vals::Qtimer3Tmr0InputSel,
            qtimer3_tmr1_input_sel: super::vals::Qtimer3Tmr1InputSel,
            qtimer3_tmr2_input_sel: super::vals::Qtimer3Tmr2InputSel,
            qtimer3_tmr3_input_sel: super::vals::Qtimer3Tmr3InputSel,
            qtimer4_tmr_cnts_freeze: super::vals::Qtimer4TmrCntsFreeze,
            qtimer4_tmr0_input_sel: super::vals::Qtimer4Tmr0InputSel,
            qtimer4_tmr1_input_sel: super::vals::Qtimer4Tmr1InputSel,
            qtimer4_tmr2_input_sel: super::vals::Qtimer4Tmr2InputSel,
            qtimer4_tmr3_input_sel: super::vals::Qtimer4Tmr3InputSel,
        }
        let proxy = QtimerCtrl1 {
            qtimer1_tmr_cnts_freeze: self.qtimer1_tmr_cnts_freeze(),
            qtimer1_tmr0_input_sel: self.qtimer1_tmr0_input_sel(),
            qtimer1_tmr1_input_sel: self.qtimer1_tmr1_input_sel(),
            qtimer1_tmr2_input_sel: self.qtimer1_tmr2_input_sel(),
            qtimer1_tmr3_input_sel: self.qtimer1_tmr3_input_sel(),
            qtimer2_tmr_cnts_freeze: self.qtimer2_tmr_cnts_freeze(),
            qtimer2_tmr0_input_sel: self.qtimer2_tmr0_input_sel(),
            qtimer2_tmr1_input_sel: self.qtimer2_tmr1_input_sel(),
            qtimer2_tmr2_input_sel: self.qtimer2_tmr2_input_sel(),
            qtimer2_tmr3_input_sel: self.qtimer2_tmr3_input_sel(),
            qtimer3_tmr_cnts_freeze: self.qtimer3_tmr_cnts_freeze(),
            qtimer3_tmr0_input_sel: self.qtimer3_tmr0_input_sel(),
            qtimer3_tmr1_input_sel: self.qtimer3_tmr1_input_sel(),
            qtimer3_tmr2_input_sel: self.qtimer3_tmr2_input_sel(),
            qtimer3_tmr3_input_sel: self.qtimer3_tmr3_input_sel(),
            qtimer4_tmr_cnts_freeze: self.qtimer4_tmr_cnts_freeze(),
            qtimer4_tmr0_input_sel: self.qtimer4_tmr0_input_sel(),
            qtimer4_tmr1_input_sel: self.qtimer4_tmr1_input_sel(),
            qtimer4_tmr2_input_sel: self.qtimer4_tmr2_input_sel(),
            qtimer4_tmr3_input_sel: self.qtimer4_tmr3_input_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Qtimer miscellaneous control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QtimerCtrl2(pub u32);
impl QtimerCtrl2 {
    #[doc = "QTIMER5 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer5_tmr_cnts_freeze(&self) -> super::vals::Qtimer5TmrCntsFreeze {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qtimer5TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER5 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer5_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer5TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "QTIMER5 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer5_tmr0_input_sel(&self) -> super::vals::Qtimer5Tmr0InputSel {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Qtimer5Tmr0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER5 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer5_tmr0_input_sel(&mut self, val: super::vals::Qtimer5Tmr0InputSel) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "QTIMER5 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer5_tmr1_input_sel(&self) -> super::vals::Qtimer5Tmr1InputSel {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Qtimer5Tmr1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER5 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer5_tmr1_input_sel(&mut self, val: super::vals::Qtimer5Tmr1InputSel) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "QTIMER5 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer5_tmr2_input_sel(&self) -> super::vals::Qtimer5Tmr2InputSel {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Qtimer5Tmr2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER5 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer5_tmr2_input_sel(&mut self, val: super::vals::Qtimer5Tmr2InputSel) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "QTIMER5 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer5_tmr3_input_sel(&self) -> super::vals::Qtimer5Tmr3InputSel {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Qtimer5Tmr3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER5 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer5_tmr3_input_sel(&mut self, val: super::vals::Qtimer5Tmr3InputSel) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "QTIMER6 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer6_tmr_cnts_freeze(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "QTIMER6 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer6_tmr_cnts_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "QTIMER6 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer6_tmr0_input_sel(&self) -> super::vals::Qtimer6Tmr0InputSel {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Qtimer6Tmr0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER6 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer6_tmr0_input_sel(&mut self, val: super::vals::Qtimer6Tmr0InputSel) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "QTIMER6 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer6_tmr1_input_sel(&self) -> super::vals::Qtimer6Tmr1InputSel {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Qtimer6Tmr1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER6 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer6_tmr1_input_sel(&mut self, val: super::vals::Qtimer6Tmr1InputSel) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "QTIMER6 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer6_tmr2_input_sel(&self) -> super::vals::Qtimer6Tmr2InputSel {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Qtimer6Tmr2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER6 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer6_tmr2_input_sel(&mut self, val: super::vals::Qtimer6Tmr2InputSel) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "QTIMER6 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer6_tmr3_input_sel(&self) -> super::vals::Qtimer6Tmr3InputSel {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Qtimer6Tmr3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER6 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer6_tmr3_input_sel(&mut self, val: super::vals::Qtimer6Tmr3InputSel) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "QTIMER7 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer7_tmr_cnts_freeze(&self) -> super::vals::Qtimer7TmrCntsFreeze {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Qtimer7TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER7 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer7_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer7TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "QTIMER7 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer7_tmr0_input_sel(&self) -> super::vals::Qtimer7Tmr0InputSel {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Qtimer7Tmr0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER7 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer7_tmr0_input_sel(&mut self, val: super::vals::Qtimer7Tmr0InputSel) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "QTIMER7 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer7_tmr1_input_sel(&self) -> super::vals::Qtimer7Tmr1InputSel {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Qtimer7Tmr1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER7 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer7_tmr1_input_sel(&mut self, val: super::vals::Qtimer7Tmr1InputSel) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "QTIMER7 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer7_tmr2_input_sel(&self) -> super::vals::Qtimer7Tmr2InputSel {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Qtimer7Tmr2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER7 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer7_tmr2_input_sel(&mut self, val: super::vals::Qtimer7Tmr2InputSel) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "QTIMER7 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer7_tmr3_input_sel(&self) -> super::vals::Qtimer7Tmr3InputSel {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Qtimer7Tmr3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER7 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer7_tmr3_input_sel(&mut self, val: super::vals::Qtimer7Tmr3InputSel) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "QTIMER8 timer counter freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer8_tmr_cnts_freeze(&self) -> super::vals::Qtimer8TmrCntsFreeze {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Qtimer8TmrCntsFreeze::from_bits(val as u8)
    }
    #[doc = "QTIMER8 timer counter freeze"]
    #[inline(always)]
    pub const fn set_qtimer8_tmr_cnts_freeze(&mut self, val: super::vals::Qtimer8TmrCntsFreeze) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "QTIMER8 TMR0 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer8_tmr0_input_sel(&self) -> super::vals::Qtimer8Tmr0InputSel {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Qtimer8Tmr0InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER8 TMR0 input select"]
    #[inline(always)]
    pub const fn set_qtimer8_tmr0_input_sel(&mut self, val: super::vals::Qtimer8Tmr0InputSel) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "QTIMER8 TMR1 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer8_tmr1_input_sel(&self) -> super::vals::Qtimer8Tmr1InputSel {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Qtimer8Tmr1InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER8 TMR1 input select"]
    #[inline(always)]
    pub const fn set_qtimer8_tmr1_input_sel(&mut self, val: super::vals::Qtimer8Tmr1InputSel) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "QTIMER8 TMR2 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer8_tmr2_input_sel(&self) -> super::vals::Qtimer8Tmr2InputSel {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Qtimer8Tmr2InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER8 TMR2 input select"]
    #[inline(always)]
    pub const fn set_qtimer8_tmr2_input_sel(&mut self, val: super::vals::Qtimer8Tmr2InputSel) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "QTIMER8 TMR3 input select"]
    #[must_use]
    #[inline(always)]
    pub const fn qtimer8_tmr3_input_sel(&self) -> super::vals::Qtimer8Tmr3InputSel {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Qtimer8Tmr3InputSel::from_bits(val as u8)
    }
    #[doc = "QTIMER8 TMR3 input select"]
    #[inline(always)]
    pub const fn set_qtimer8_tmr3_input_sel(&mut self, val: super::vals::Qtimer8Tmr3InputSel) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
}
impl Default for QtimerCtrl2 {
    #[inline(always)]
    fn default() -> QtimerCtrl2 {
        QtimerCtrl2(0u64 as u32)
    }
}
impl core::fmt::Debug for QtimerCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QtimerCtrl2")
            .field("qtimer5_tmr_cnts_freeze", &self.qtimer5_tmr_cnts_freeze())
            .field("qtimer5_tmr0_input_sel", &self.qtimer5_tmr0_input_sel())
            .field("qtimer5_tmr1_input_sel", &self.qtimer5_tmr1_input_sel())
            .field("qtimer5_tmr2_input_sel", &self.qtimer5_tmr2_input_sel())
            .field("qtimer5_tmr3_input_sel", &self.qtimer5_tmr3_input_sel())
            .field("qtimer6_tmr_cnts_freeze", &self.qtimer6_tmr_cnts_freeze())
            .field("qtimer6_tmr0_input_sel", &self.qtimer6_tmr0_input_sel())
            .field("qtimer6_tmr1_input_sel", &self.qtimer6_tmr1_input_sel())
            .field("qtimer6_tmr2_input_sel", &self.qtimer6_tmr2_input_sel())
            .field("qtimer6_tmr3_input_sel", &self.qtimer6_tmr3_input_sel())
            .field("qtimer7_tmr_cnts_freeze", &self.qtimer7_tmr_cnts_freeze())
            .field("qtimer7_tmr0_input_sel", &self.qtimer7_tmr0_input_sel())
            .field("qtimer7_tmr1_input_sel", &self.qtimer7_tmr1_input_sel())
            .field("qtimer7_tmr2_input_sel", &self.qtimer7_tmr2_input_sel())
            .field("qtimer7_tmr3_input_sel", &self.qtimer7_tmr3_input_sel())
            .field("qtimer8_tmr_cnts_freeze", &self.qtimer8_tmr_cnts_freeze())
            .field("qtimer8_tmr0_input_sel", &self.qtimer8_tmr0_input_sel())
            .field("qtimer8_tmr1_input_sel", &self.qtimer8_tmr1_input_sel())
            .field("qtimer8_tmr2_input_sel", &self.qtimer8_tmr2_input_sel())
            .field("qtimer8_tmr3_input_sel", &self.qtimer8_tmr3_input_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for QtimerCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct QtimerCtrl2 {
            qtimer5_tmr_cnts_freeze: super::vals::Qtimer5TmrCntsFreeze,
            qtimer5_tmr0_input_sel: super::vals::Qtimer5Tmr0InputSel,
            qtimer5_tmr1_input_sel: super::vals::Qtimer5Tmr1InputSel,
            qtimer5_tmr2_input_sel: super::vals::Qtimer5Tmr2InputSel,
            qtimer5_tmr3_input_sel: super::vals::Qtimer5Tmr3InputSel,
            qtimer6_tmr_cnts_freeze: bool,
            qtimer6_tmr0_input_sel: super::vals::Qtimer6Tmr0InputSel,
            qtimer6_tmr1_input_sel: super::vals::Qtimer6Tmr1InputSel,
            qtimer6_tmr2_input_sel: super::vals::Qtimer6Tmr2InputSel,
            qtimer6_tmr3_input_sel: super::vals::Qtimer6Tmr3InputSel,
            qtimer7_tmr_cnts_freeze: super::vals::Qtimer7TmrCntsFreeze,
            qtimer7_tmr0_input_sel: super::vals::Qtimer7Tmr0InputSel,
            qtimer7_tmr1_input_sel: super::vals::Qtimer7Tmr1InputSel,
            qtimer7_tmr2_input_sel: super::vals::Qtimer7Tmr2InputSel,
            qtimer7_tmr3_input_sel: super::vals::Qtimer7Tmr3InputSel,
            qtimer8_tmr_cnts_freeze: super::vals::Qtimer8TmrCntsFreeze,
            qtimer8_tmr0_input_sel: super::vals::Qtimer8Tmr0InputSel,
            qtimer8_tmr1_input_sel: super::vals::Qtimer8Tmr1InputSel,
            qtimer8_tmr2_input_sel: super::vals::Qtimer8Tmr2InputSel,
            qtimer8_tmr3_input_sel: super::vals::Qtimer8Tmr3InputSel,
        }
        let proxy = QtimerCtrl2 {
            qtimer5_tmr_cnts_freeze: self.qtimer5_tmr_cnts_freeze(),
            qtimer5_tmr0_input_sel: self.qtimer5_tmr0_input_sel(),
            qtimer5_tmr1_input_sel: self.qtimer5_tmr1_input_sel(),
            qtimer5_tmr2_input_sel: self.qtimer5_tmr2_input_sel(),
            qtimer5_tmr3_input_sel: self.qtimer5_tmr3_input_sel(),
            qtimer6_tmr_cnts_freeze: self.qtimer6_tmr_cnts_freeze(),
            qtimer6_tmr0_input_sel: self.qtimer6_tmr0_input_sel(),
            qtimer6_tmr1_input_sel: self.qtimer6_tmr1_input_sel(),
            qtimer6_tmr2_input_sel: self.qtimer6_tmr2_input_sel(),
            qtimer6_tmr3_input_sel: self.qtimer6_tmr3_input_sel(),
            qtimer7_tmr_cnts_freeze: self.qtimer7_tmr_cnts_freeze(),
            qtimer7_tmr0_input_sel: self.qtimer7_tmr0_input_sel(),
            qtimer7_tmr1_input_sel: self.qtimer7_tmr1_input_sel(),
            qtimer7_tmr2_input_sel: self.qtimer7_tmr2_input_sel(),
            qtimer7_tmr3_input_sel: self.qtimer7_tmr3_input_sel(),
            qtimer8_tmr_cnts_freeze: self.qtimer8_tmr_cnts_freeze(),
            qtimer8_tmr0_input_sel: self.qtimer8_tmr0_input_sel(),
            qtimer8_tmr1_input_sel: self.qtimer8_tmr1_input_sel(),
            qtimer8_tmr2_input_sel: self.qtimer8_tmr2_input_sel(),
            qtimer8_tmr3_input_sel: self.qtimer8_tmr3_input_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Safety clock monitor control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SafetyClkMonCs(pub u32);
impl SafetyClkMonCs {
    #[doc = "Monitor enable bit"]
    #[must_use]
    #[inline(always)]
    pub const fn mon_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor enable bit"]
    #[inline(always)]
    pub const fn set_mon_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn irq_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt enable"]
    #[inline(always)]
    pub const fn set_irq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Reset out enable"]
    #[must_use]
    #[inline(always)]
    pub const fn fast_rst_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Reset out enable"]
    #[inline(always)]
    pub const fn set_fast_rst_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Status clear"]
    #[must_use]
    #[inline(always)]
    pub const fn stat_clr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Status clear"]
    #[inline(always)]
    pub const fn set_stat_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "XBAR_OUT220 clock failure status"]
    #[must_use]
    #[inline(always)]
    pub const fn stat(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "XBAR_OUT220 clock failure status"]
    #[inline(always)]
    pub const fn set_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for SafetyClkMonCs {
    #[inline(always)]
    fn default() -> SafetyClkMonCs {
        SafetyClkMonCs(0u64 as u32)
    }
}
impl core::fmt::Debug for SafetyClkMonCs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SafetyClkMonCs")
            .field("mon_en", &self.mon_en())
            .field("irq_en", &self.irq_en())
            .field("fast_rst_en", &self.fast_rst_en())
            .field("stat_clr", &self.stat_clr())
            .field("stat", &self.stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SafetyClkMonCs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SafetyClkMonCs {
            mon_en: bool,
            irq_en: bool,
            fast_rst_en: bool,
            stat_clr: bool,
            stat: bool,
        }
        let proxy = SafetyClkMonCs {
            mon_en: self.mon_en(),
            irq_en: self.irq_en(),
            fast_rst_en: self.fast_rst_en(),
            stat_clr: self.stat_clr(),
            stat: self.stat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Safety clock monitor threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SafetyClkMonTh(pub u32);
impl SafetyClkMonTh {
    #[doc = "Threshold low value"]
    #[must_use]
    #[inline(always)]
    pub const fn th_low(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Threshold low value"]
    #[inline(always)]
    pub const fn set_th_low(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Threshold high value"]
    #[must_use]
    #[inline(always)]
    pub const fn th_high(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Threshold high value"]
    #[inline(always)]
    pub const fn set_th_high(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SafetyClkMonTh {
    #[inline(always)]
    fn default() -> SafetyClkMonTh {
        SafetyClkMonTh(0u64 as u32)
    }
}
impl core::fmt::Debug for SafetyClkMonTh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SafetyClkMonTh")
            .field("th_low", &self.th_low())
            .field("th_high", &self.th_high())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SafetyClkMonTh {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SafetyClkMonTh {
            th_low: u16,
            th_high: u16,
        }
        let proxy = SafetyClkMonTh {
            th_low: self.th_low(),
            th_high: self.th_high(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI2 MCLK control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai2MclkCtrl(pub u32);
impl Sai2MclkCtrl {
    #[doc = "SAI2 MCLK3 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_mclk3_sel(&self) -> super::vals::Sai2Mclk3Sel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai2Mclk3Sel::from_bits(val as u8)
    }
    #[doc = "SAI2 MCLK3 source select"]
    #[inline(always)]
    pub const fn set_sai2_mclk3_sel(&mut self, val: super::vals::Sai2Mclk3Sel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SAI2_MCLK IO direction control. IOMUX need select SAI2 MCLK function."]
    #[must_use]
    #[inline(always)]
    pub const fn sai2_mclk_dir(&self) -> super::vals::Sai2MclkDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sai2MclkDir::from_bits(val as u8)
    }
    #[doc = "SAI2_MCLK IO direction control. IOMUX need select SAI2 MCLK function."]
    #[inline(always)]
    pub const fn set_sai2_mclk_dir(&mut self, val: super::vals::Sai2MclkDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Sai2MclkCtrl {
    #[inline(always)]
    fn default() -> Sai2MclkCtrl {
        Sai2MclkCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai2MclkCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai2MclkCtrl")
            .field("sai2_mclk3_sel", &self.sai2_mclk3_sel())
            .field("sai2_mclk_dir", &self.sai2_mclk_dir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai2MclkCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai2MclkCtrl {
            sai2_mclk3_sel: super::vals::Sai2Mclk3Sel,
            sai2_mclk_dir: super::vals::Sai2MclkDir,
        }
        let proxy = Sai2MclkCtrl {
            sai2_mclk3_sel: self.sai2_mclk3_sel(),
            sai2_mclk_dir: self.sai2_mclk_dir(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI3 MCLK control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai3MclkCtrl(pub u32);
impl Sai3MclkCtrl {
    #[doc = "SAI3 MCLK3 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_mclk3_sel(&self) -> super::vals::Sai3Mclk3Sel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Sai3Mclk3Sel::from_bits(val as u8)
    }
    #[doc = "SAI3 MCLK3 source select"]
    #[inline(always)]
    pub const fn set_sai3_mclk3_sel(&mut self, val: super::vals::Sai3Mclk3Sel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "SAI3_MCLK IO direction control. IOMUX need select SAI3 MCLK function."]
    #[must_use]
    #[inline(always)]
    pub const fn sai3_mclk_dir(&self) -> super::vals::Sai3MclkDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Sai3MclkDir::from_bits(val as u8)
    }
    #[doc = "SAI3_MCLK IO direction control. IOMUX need select SAI3 MCLK function."]
    #[inline(always)]
    pub const fn set_sai3_mclk_dir(&mut self, val: super::vals::Sai3MclkDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Sai3MclkCtrl {
    #[inline(always)]
    fn default() -> Sai3MclkCtrl {
        Sai3MclkCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai3MclkCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai3MclkCtrl")
            .field("sai3_mclk3_sel", &self.sai3_mclk3_sel())
            .field("sai3_mclk_dir", &self.sai3_mclk_dir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai3MclkCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai3MclkCtrl {
            sai3_mclk3_sel: super::vals::Sai3Mclk3Sel,
            sai3_mclk_dir: super::vals::Sai3MclkDir,
        }
        let proxy = Sai3MclkCtrl {
            sai3_mclk3_sel: self.sai3_mclk3_sel(),
            sai3_mclk_dir: self.sai3_mclk_dir(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SAI4 MCLK control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai4MclkCtrl(pub u32);
impl Sai4MclkCtrl {
    #[doc = "SAI4 MCLK1 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai4_mclk1_sel(&self) -> super::vals::Sai4Mclk1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sai4Mclk1Sel::from_bits(val as u8)
    }
    #[doc = "SAI4 MCLK1 source select"]
    #[inline(always)]
    pub const fn set_sai4_mclk1_sel(&mut self, val: super::vals::Sai4Mclk1Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "SAI4 MCLK2 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai4_mclk2_sel(&self) -> super::vals::Sai4Mclk2Sel {
        let val = (self.0 >> 3usize) & 0x07;
        super::vals::Sai4Mclk2Sel::from_bits(val as u8)
    }
    #[doc = "SAI4 MCLK2 source select"]
    #[inline(always)]
    pub const fn set_sai4_mclk2_sel(&mut self, val: super::vals::Sai4Mclk2Sel) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
    }
    #[doc = "SAI4 MCLK3 source select"]
    #[must_use]
    #[inline(always)]
    pub const fn sai4_mclk3_sel(&self) -> super::vals::Sai4Mclk3Sel {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Sai4Mclk3Sel::from_bits(val as u8)
    }
    #[doc = "SAI4 MCLK3 source select"]
    #[inline(always)]
    pub const fn set_sai4_mclk3_sel(&mut self, val: super::vals::Sai4Mclk3Sel) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "SAI4_MCLK IO direction control. IOMUX need select SAI4 MCLK function."]
    #[must_use]
    #[inline(always)]
    pub const fn sai4_mclk_dir(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SAI4_MCLK IO direction control. IOMUX need select SAI4 MCLK function."]
    #[inline(always)]
    pub const fn set_sai4_mclk_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Sai4MclkCtrl {
    #[inline(always)]
    fn default() -> Sai4MclkCtrl {
        Sai4MclkCtrl(0u64 as u32)
    }
}
impl core::fmt::Debug for Sai4MclkCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sai4MclkCtrl")
            .field("sai4_mclk1_sel", &self.sai4_mclk1_sel())
            .field("sai4_mclk2_sel", &self.sai4_mclk2_sel())
            .field("sai4_mclk3_sel", &self.sai4_mclk3_sel())
            .field("sai4_mclk_dir", &self.sai4_mclk_dir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sai4MclkCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sai4MclkCtrl {
            sai4_mclk1_sel: super::vals::Sai4Mclk1Sel,
            sai4_mclk2_sel: super::vals::Sai4Mclk2Sel,
            sai4_mclk3_sel: super::vals::Sai4Mclk3Sel,
            sai4_mclk_dir: bool,
        }
        let proxy = Sai4MclkCtrl {
            sai4_mclk1_sel: self.sai4_mclk1_sel(),
            sai4_mclk2_sel: self.sai4_mclk2_sel(),
            sai4_mclk3_sel: self.sai4_mclk3_sel(),
            sai4_mclk_dir: self.sai4_mclk_dir(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPIO_SD_B1 bank IO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdB1IoCtrl(pub u32);
impl SdB1IoCtrl {
    #[doc = "Compensation code freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_freeze(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code freeze"]
    #[inline(always)]
    pub const fn set_gpio_sd1_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_comptq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_sd1_comptq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_compen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_sd1_compen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compensation code fast freeze enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_fastfrz_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast freeze enable"]
    #[inline(always)]
    pub const fn set_gpio_sd1_fastfrz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO_SD_B1 IO bank's 4-bit PMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_rasrcp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_SD_B1 IO bank's 4-bit PMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_sd1_rasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GPIO_SD_B1 IO bank's 4-bit NMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_rasrcn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_SD_B1 IO bank's 4-bit NMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_sd1_rasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "GPIO_SD1_NASRC selection"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_select_nasrc(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD1_NASRC selection"]
    #[inline(always)]
    pub const fn set_gpio_sd1_select_nasrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO_SD_B1 IO bank reference voltage generator cell sleep enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_refgen_sleep(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD_B1 IO bank reference voltage generator cell sleep enable"]
    #[inline(always)]
    pub const fn set_gpio_sd1_refgen_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO_SD_B1 IO bank power supply mode latch enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_suplydet_latch(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD_B1 IO bank power supply mode latch enable"]
    #[inline(always)]
    pub const fn set_gpio_sd1_suplydet_latch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Compensation code fast-freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_fastfrz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast-freeze"]
    #[inline(always)]
    pub const fn set_gpio_sd1_fastfrz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO_SD_B1 IO bank compensation OK flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_compok(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD_B1 IO bank compensation OK flag"]
    #[inline(always)]
    pub const fn set_gpio_sd1_compok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO_SD_B1 IO bank compensation codes"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd1_nasrc(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_SD_B1 IO bank compensation codes"]
    #[inline(always)]
    pub const fn set_gpio_sd1_nasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
}
impl Default for SdB1IoCtrl {
    #[inline(always)]
    fn default() -> SdB1IoCtrl {
        SdB1IoCtrl(22036480u64 as u32)
    }
}
impl core::fmt::Debug for SdB1IoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SdB1IoCtrl")
            .field("gpio_sd1_freeze", &self.gpio_sd1_freeze())
            .field("gpio_sd1_comptq", &self.gpio_sd1_comptq())
            .field("gpio_sd1_compen", &self.gpio_sd1_compen())
            .field("gpio_sd1_fastfrz_en", &self.gpio_sd1_fastfrz_en())
            .field("gpio_sd1_rasrcp", &self.gpio_sd1_rasrcp())
            .field("gpio_sd1_rasrcn", &self.gpio_sd1_rasrcn())
            .field("gpio_sd1_select_nasrc", &self.gpio_sd1_select_nasrc())
            .field("gpio_sd1_refgen_sleep", &self.gpio_sd1_refgen_sleep())
            .field("gpio_sd1_suplydet_latch", &self.gpio_sd1_suplydet_latch())
            .field("gpio_sd1_fastfrz", &self.gpio_sd1_fastfrz())
            .field("gpio_sd1_compok", &self.gpio_sd1_compok())
            .field("gpio_sd1_nasrc", &self.gpio_sd1_nasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SdB1IoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SdB1IoCtrl {
            gpio_sd1_freeze: bool,
            gpio_sd1_comptq: bool,
            gpio_sd1_compen: bool,
            gpio_sd1_fastfrz_en: bool,
            gpio_sd1_rasrcp: u8,
            gpio_sd1_rasrcn: u8,
            gpio_sd1_select_nasrc: bool,
            gpio_sd1_refgen_sleep: bool,
            gpio_sd1_suplydet_latch: bool,
            gpio_sd1_fastfrz: bool,
            gpio_sd1_compok: bool,
            gpio_sd1_nasrc: u8,
        }
        let proxy = SdB1IoCtrl {
            gpio_sd1_freeze: self.gpio_sd1_freeze(),
            gpio_sd1_comptq: self.gpio_sd1_comptq(),
            gpio_sd1_compen: self.gpio_sd1_compen(),
            gpio_sd1_fastfrz_en: self.gpio_sd1_fastfrz_en(),
            gpio_sd1_rasrcp: self.gpio_sd1_rasrcp(),
            gpio_sd1_rasrcn: self.gpio_sd1_rasrcn(),
            gpio_sd1_select_nasrc: self.gpio_sd1_select_nasrc(),
            gpio_sd1_refgen_sleep: self.gpio_sd1_refgen_sleep(),
            gpio_sd1_suplydet_latch: self.gpio_sd1_suplydet_latch(),
            gpio_sd1_fastfrz: self.gpio_sd1_fastfrz(),
            gpio_sd1_compok: self.gpio_sd1_compok(),
            gpio_sd1_nasrc: self.gpio_sd1_nasrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPIO_SD_B2 bank IO control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SdB2IoCtrl(pub u32);
impl SdB2IoCtrl {
    #[doc = "Compensation code freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_freeze(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code freeze"]
    #[inline(always)]
    pub const fn set_gpio_sd2_freeze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_comptq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_sd2_comptq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_compen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "COMPEN and COMPTQ control the operating modes of the compensation cell"]
    #[inline(always)]
    pub const fn set_gpio_sd2_compen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Compensation code fast freeze enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_fastfrz_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast freeze enable"]
    #[inline(always)]
    pub const fn set_gpio_sd2_fastfrz_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "GPIO_SD_B2 IO bank's 4-bit PMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_rasrcp(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_SD_B2 IO bank's 4-bit PMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_sd2_rasrcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "GPIO_SD_B2 IO bank's 4-bit NMOS compensation codes from core"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_rasrcn(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_SD_B2 IO bank's 4-bit NMOS compensation codes from core"]
    #[inline(always)]
    pub const fn set_gpio_sd2_rasrcn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "GPIO_SD2_NASRC selection"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_select_nasrc(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD2_NASRC selection"]
    #[inline(always)]
    pub const fn set_gpio_sd2_select_nasrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "GPIO_SD_B2 IO bank reference voltage generator cell sleep enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_refgen_sleep(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD_B2 IO bank reference voltage generator cell sleep enable"]
    #[inline(always)]
    pub const fn set_gpio_sd2_refgen_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "GPIO_SD_B2 IO bank power supply mode latch enable"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_suplydet_latch(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD_B2 IO bank power supply mode latch enable"]
    #[inline(always)]
    pub const fn set_gpio_sd2_suplydet_latch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Compensation code fast-freeze"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_fastfrz(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Compensation code fast-freeze"]
    #[inline(always)]
    pub const fn set_gpio_sd2_fastfrz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "GPIO_SD_B2 IO bank compensation OK flag"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_compok(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "GPIO_SD_B2 IO bank compensation OK flag"]
    #[inline(always)]
    pub const fn set_gpio_sd2_compok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "GPIO_SD_B2 IO bank compensation codes"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio_sd2_nasrc(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x0f;
        val as u8
    }
    #[doc = "GPIO_SD_B2 IO bank compensation codes"]
    #[inline(always)]
    pub const fn set_gpio_sd2_nasrc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
    }
}
impl Default for SdB2IoCtrl {
    #[inline(always)]
    fn default() -> SdB2IoCtrl {
        SdB2IoCtrl(22036480u64 as u32)
    }
}
impl core::fmt::Debug for SdB2IoCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SdB2IoCtrl")
            .field("gpio_sd2_freeze", &self.gpio_sd2_freeze())
            .field("gpio_sd2_comptq", &self.gpio_sd2_comptq())
            .field("gpio_sd2_compen", &self.gpio_sd2_compen())
            .field("gpio_sd2_fastfrz_en", &self.gpio_sd2_fastfrz_en())
            .field("gpio_sd2_rasrcp", &self.gpio_sd2_rasrcp())
            .field("gpio_sd2_rasrcn", &self.gpio_sd2_rasrcn())
            .field("gpio_sd2_select_nasrc", &self.gpio_sd2_select_nasrc())
            .field("gpio_sd2_refgen_sleep", &self.gpio_sd2_refgen_sleep())
            .field("gpio_sd2_suplydet_latch", &self.gpio_sd2_suplydet_latch())
            .field("gpio_sd2_fastfrz", &self.gpio_sd2_fastfrz())
            .field("gpio_sd2_compok", &self.gpio_sd2_compok())
            .field("gpio_sd2_nasrc", &self.gpio_sd2_nasrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SdB2IoCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SdB2IoCtrl {
            gpio_sd2_freeze: bool,
            gpio_sd2_comptq: bool,
            gpio_sd2_compen: bool,
            gpio_sd2_fastfrz_en: bool,
            gpio_sd2_rasrcp: u8,
            gpio_sd2_rasrcn: u8,
            gpio_sd2_select_nasrc: bool,
            gpio_sd2_refgen_sleep: bool,
            gpio_sd2_suplydet_latch: bool,
            gpio_sd2_fastfrz: bool,
            gpio_sd2_compok: bool,
            gpio_sd2_nasrc: u8,
        }
        let proxy = SdB2IoCtrl {
            gpio_sd2_freeze: self.gpio_sd2_freeze(),
            gpio_sd2_comptq: self.gpio_sd2_comptq(),
            gpio_sd2_compen: self.gpio_sd2_compen(),
            gpio_sd2_fastfrz_en: self.gpio_sd2_fastfrz_en(),
            gpio_sd2_rasrcp: self.gpio_sd2_rasrcp(),
            gpio_sd2_rasrcn: self.gpio_sd2_rasrcn(),
            gpio_sd2_select_nasrc: self.gpio_sd2_select_nasrc(),
            gpio_sd2_refgen_sleep: self.gpio_sd2_refgen_sleep(),
            gpio_sd2_suplydet_latch: self.gpio_sd2_suplydet_latch(),
            gpio_sd2_fastfrz: self.gpio_sd2_fastfrz(),
            gpio_sd2_compok: self.gpio_sd2_compok(),
            gpio_sd2_nasrc: self.gpio_sd2_nasrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Slave stop mode configure register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlaveStopModeCfg(pub u32);
impl SlaveStopModeCfg {
    #[doc = "ADC1 stop mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn adc1_ipg_stop_mode(&self) -> super::vals::Adc1IpgStopMode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Adc1IpgStopMode::from_bits(val as u8)
    }
    #[doc = "ADC1 stop mode selection."]
    #[inline(always)]
    pub const fn set_adc1_ipg_stop_mode(&mut self, val: super::vals::Adc1IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "ADC2 stop mode selection."]
    #[must_use]
    #[inline(always)]
    pub const fn adc2_ipg_stop_mode(&self) -> super::vals::Adc2IpgStopMode {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Adc2IpgStopMode::from_bits(val as u8)
    }
    #[doc = "ADC2 stop mode selection."]
    #[inline(always)]
    pub const fn set_adc2_ipg_stop_mode(&mut self, val: super::vals::Adc2IpgStopMode) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for SlaveStopModeCfg {
    #[inline(always)]
    fn default() -> SlaveStopModeCfg {
        SlaveStopModeCfg(0u64 as u32)
    }
}
impl core::fmt::Debug for SlaveStopModeCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SlaveStopModeCfg")
            .field("adc1_ipg_stop_mode", &self.adc1_ipg_stop_mode())
            .field("adc2_ipg_stop_mode", &self.adc2_ipg_stop_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SlaveStopModeCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SlaveStopModeCfg {
            adc1_ipg_stop_mode: super::vals::Adc1IpgStopMode,
            adc2_ipg_stop_mode: super::vals::Adc2IpgStopMode,
        }
        let proxy = SlaveStopModeCfg {
            adc1_ipg_stop_mode: self.adc1_ipg_stop_mode(),
            adc2_ipg_stop_mode: self.adc2_ipg_stop_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRAM Control Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr0(pub u32);
impl Sramcr0 {
    #[doc = "AHB Bus Timeout Wait Cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn bto(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "AHB Bus Timeout Wait Cycle"]
    #[inline(always)]
    pub const fn set_bto(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "AHB Bus Timeout Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn btoen(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "AHB Bus Timeout Enable"]
    #[inline(always)]
    pub const fn set_btoen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Port Size"]
    #[must_use]
    #[inline(always)]
    pub const fn ps(&self) -> super::vals::Ps {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Ps::from_bits(val as u8)
    }
    #[doc = "Port Size"]
    #[inline(always)]
    pub const fn set_ps(&mut self, val: super::vals::Ps) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Address Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn am(&self) -> super::vals::Am {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Am::from_bits(val as u8)
    }
    #[doc = "Address Mode"]
    #[inline(always)]
    pub const fn set_am(&mut self, val: super::vals::Am) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "ADV# polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn advp(&self) -> super::vals::Advp {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Advp::from_bits(val as u8)
    }
    #[doc = "ADV# polarity"]
    #[inline(always)]
    pub const fn set_advp(&mut self, val: super::vals::Advp) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "CE setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn ces(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "CE setup time"]
    #[inline(always)]
    pub const fn set_ces(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "CE hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ceh(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "CE hold time"]
    #[inline(always)]
    pub const fn set_ceh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Address setup time"]
    #[must_use]
    #[inline(always)]
    pub const fn as_(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Address setup time"]
    #[inline(always)]
    pub const fn set_as_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Address hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn ah(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Address hold time"]
    #[inline(always)]
    pub const fn set_ah(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Turnaround time"]
    #[must_use]
    #[inline(always)]
    pub const fn ta(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Turnaround time"]
    #[inline(always)]
    pub const fn set_ta(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Sramcr0 {
    #[inline(always)]
    fn default() -> Sramcr0 {
        Sramcr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Sramcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr0")
            .field("bto", &self.bto())
            .field("btoen", &self.btoen())
            .field("ps", &self.ps())
            .field("am", &self.am())
            .field("advp", &self.advp())
            .field("ces", &self.ces())
            .field("ceh", &self.ceh())
            .field("as_", &self.as_())
            .field("ah", &self.ah())
            .field("ta", &self.ta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sramcr0 {
            bto: u8,
            btoen: bool,
            ps: super::vals::Ps,
            am: super::vals::Am,
            advp: super::vals::Advp,
            ces: u8,
            ceh: u8,
            as_: u8,
            ah: u8,
            ta: u8,
        }
        let proxy = Sramcr0 {
            bto: self.bto(),
            btoen: self.btoen(),
            ps: self.ps(),
            am: self.am(),
            advp: self.advp(),
            ces: self.ces(),
            ceh: self.ceh(),
            as_: self.as_(),
            ah: self.ah(),
            ta: self.ta(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SRAM Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sramcr1(pub u32);
impl Sramcr1 {
    #[doc = "WE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn wel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "WE low time"]
    #[inline(always)]
    pub const fn set_wel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "WE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn weh(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "WE high time"]
    #[inline(always)]
    pub const fn set_weh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "RE low time"]
    #[must_use]
    #[inline(always)]
    pub const fn rel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "RE low time"]
    #[inline(always)]
    pub const fn set_rel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "RE high time"]
    #[must_use]
    #[inline(always)]
    pub const fn reh(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "RE high time"]
    #[inline(always)]
    pub const fn set_reh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Prescaler timer"]
    #[must_use]
    #[inline(always)]
    pub const fn pre(&self) -> super::vals::Pre {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Pre::from_bits(val as u8)
    }
    #[doc = "Prescaler timer"]
    #[inline(always)]
    pub const fn set_pre(&mut self, val: super::vals::Pre) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
}
impl Default for Sramcr1 {
    #[inline(always)]
    fn default() -> Sramcr1 {
        Sramcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Sramcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sramcr1")
            .field("wel", &self.wel())
            .field("weh", &self.weh())
            .field("rel", &self.rel())
            .field("reh", &self.reh())
            .field("pre", &self.pre())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sramcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sramcr1 {
            wel: u8,
            weh: u8,
            rel: u8,
            reh: u8,
            pre: super::vals::Pre,
        }
        let proxy = Sramcr1 {
            wel: self.wel(),
            weh: self.weh(),
            rel: self.rel(),
            reh: self.reh(),
            pre: self.pre(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SSI master low power mode control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssi(pub u32);
impl Ssi {
    #[doc = "WAKEUP Domain to M7 SSI master idle"]
    #[must_use]
    #[inline(always)]
    pub const fn ssi_idle(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "WAKEUP Domain to M7 SSI master idle"]
    #[inline(always)]
    pub const fn set_ssi_idle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "WAKEUP Domain to M7 SSI master blackhole mode"]
    #[must_use]
    #[inline(always)]
    pub const fn blkhole_mode_b(&self) -> super::vals::BlkholeModeB {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::BlkholeModeB::from_bits(val as u8)
    }
    #[doc = "WAKEUP Domain to M7 SSI master blackhole mode"]
    #[inline(always)]
    pub const fn set_blkhole_mode_b(&mut self, val: super::vals::BlkholeModeB) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "WAKEUP Domain to M7 SSI master pause mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pause_mode(&self) -> super::vals::PauseMode {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::PauseMode::from_bits(val as u8)
    }
    #[doc = "WAKEUP Domain to M7 SSI master pause mode"]
    #[inline(always)]
    pub const fn set_pause_mode(&mut self, val: super::vals::PauseMode) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Ssi {
    #[inline(always)]
    fn default() -> Ssi {
        Ssi(2u64 as u32)
    }
}
impl core::fmt::Debug for Ssi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssi")
            .field("ssi_idle", &self.ssi_idle())
            .field("blkhole_mode_b", &self.blkhole_mode_b())
            .field("pause_mode", &self.pause_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ssi {
            ssi_idle: bool,
            blkhole_mode_b: super::vals::BlkholeModeB,
            pause_mode: super::vals::PauseMode,
        }
        let proxy = Ssi {
            ssi_idle: self.ssi_idle(),
            blkhole_mode_b: self.blkhole_mode_b(),
            pause_mode: self.pause_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "USBPHY miscellaneous control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbphyMiscCtrl(pub u32);
impl UsbphyMiscCtrl {
    #[doc = "USBPHY1 register access clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy1_ipg_clk_active(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "USBPHY1 register access clock enable"]
    #[inline(always)]
    pub const fn set_usbphy1_ipg_clk_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USBPHY2 register access clock enable"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy2_ipg_clk_active(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "USBPHY2 register access clock enable"]
    #[inline(always)]
    pub const fn set_usbphy2_ipg_clk_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Clear USBPHY1 wakeup interrupt holding register"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy1_wakeup_irq_clear(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Clear USBPHY1 wakeup interrupt holding register"]
    #[inline(always)]
    pub const fn set_usbphy1_wakeup_irq_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Clear USBPHY2 wakeup interrupt holding register"]
    #[must_use]
    #[inline(always)]
    pub const fn usbphy2_wakeup_irq_clear(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Clear USBPHY2 wakeup interrupt holding register"]
    #[inline(always)]
    pub const fn set_usbphy2_wakeup_irq_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for UsbphyMiscCtrl {
    #[inline(always)]
    fn default() -> UsbphyMiscCtrl {
        UsbphyMiscCtrl(257u64 as u32)
    }
}
impl core::fmt::Debug for UsbphyMiscCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbphyMiscCtrl")
            .field("usbphy1_ipg_clk_active", &self.usbphy1_ipg_clk_active())
            .field("usbphy2_ipg_clk_active", &self.usbphy2_ipg_clk_active())
            .field("usbphy1_wakeup_irq_clear", &self.usbphy1_wakeup_irq_clear())
            .field("usbphy2_wakeup_irq_clear", &self.usbphy2_wakeup_irq_clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbphyMiscCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbphyMiscCtrl {
            usbphy1_ipg_clk_active: bool,
            usbphy2_ipg_clk_active: bool,
            usbphy1_wakeup_irq_clear: bool,
            usbphy2_wakeup_irq_clear: bool,
        }
        let proxy = UsbphyMiscCtrl {
            usbphy1_ipg_clk_active: self.usbphy1_ipg_clk_active(),
            usbphy2_ipg_clk_active: self.usbphy2_ipg_clk_active(),
            usbphy1_wakeup_irq_clear: self.usbphy1_wakeup_irq_clear(),
            usbphy2_wakeup_irq_clear: self.usbphy2_wakeup_irq_clear(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR and AOI write protect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XbarAoiWe(pub u32);
impl XbarAoiWe {
    #[doc = "Write Enable to XBAR and AOI"]
    #[must_use]
    #[inline(always)]
    pub const fn we(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write Enable to XBAR and AOI"]
    #[inline(always)]
    pub const fn set_we(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for XbarAoiWe {
    #[inline(always)]
    fn default() -> XbarAoiWe {
        XbarAoiWe(1u64 as u32)
    }
}
impl core::fmt::Debug for XbarAoiWe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XbarAoiWe").field("we", &self.we()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XbarAoiWe {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XbarAoiWe {
            we: bool,
        }
        let proxy = XbarAoiWe { we: self.we() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR IO direction control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XbarDirCtrl1(pub u32);
impl XbarDirCtrl1 {
    #[doc = "IOMUXC XBAR_INOUT4 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_4(&self) -> super::vals::IomuxcXbarDirSel4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::IomuxcXbarDirSel4::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT4 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_4(&mut self, val: super::vals::IomuxcXbarDirSel4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IOMUXC XBAR_INOUT5 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_5(&self) -> super::vals::IomuxcXbarDirSel5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::IomuxcXbarDirSel5::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT5 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_5(&mut self, val: super::vals::IomuxcXbarDirSel5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "IOMUXC XBAR_INOUT6 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_6(&self) -> super::vals::IomuxcXbarDirSel6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::IomuxcXbarDirSel6::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT6 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_6(&mut self, val: super::vals::IomuxcXbarDirSel6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "IOMUXC XBAR_INOUT7 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_7(&self) -> super::vals::IomuxcXbarDirSel7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::IomuxcXbarDirSel7::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT7 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_7(&mut self, val: super::vals::IomuxcXbarDirSel7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "IOMUXC XBAR_INOUT8 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_8(&self) -> super::vals::IomuxcXbarDirSel8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::IomuxcXbarDirSel8::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT8 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_8(&mut self, val: super::vals::IomuxcXbarDirSel8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "IOMUXC XBAR_INOUT9 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_9(&self) -> super::vals::IomuxcXbarDirSel9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::IomuxcXbarDirSel9::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT9 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_9(&mut self, val: super::vals::IomuxcXbarDirSel9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "IOMUXC XBAR_INOUT10 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_10(&self) -> super::vals::IomuxcXbarDirSel10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::IomuxcXbarDirSel10::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT10 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_10(&mut self, val: super::vals::IomuxcXbarDirSel10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "IOMUXC XBAR_INOUT11 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_11(&self) -> super::vals::IomuxcXbarDirSel11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::IomuxcXbarDirSel11::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT11 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_11(&mut self, val: super::vals::IomuxcXbarDirSel11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "IOMUXC XBAR_INOUT12 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_12(&self) -> super::vals::IomuxcXbarDirSel12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::IomuxcXbarDirSel12::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT12 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_12(&mut self, val: super::vals::IomuxcXbarDirSel12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "IOMUXC XBAR_INOUT13 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_13(&self) -> super::vals::IomuxcXbarDirSel13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::IomuxcXbarDirSel13::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT13 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_13(&mut self, val: super::vals::IomuxcXbarDirSel13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "IOMUXC XBAR_INOUT14 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_14(&self) -> super::vals::IomuxcXbarDirSel14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::IomuxcXbarDirSel14::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT14 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_14(&mut self, val: super::vals::IomuxcXbarDirSel14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "IOMUXC XBAR_INOUT15 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_15(&self) -> super::vals::IomuxcXbarDirSel15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::IomuxcXbarDirSel15::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT15 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_15(&mut self, val: super::vals::IomuxcXbarDirSel15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "IOMUXC XBAR_INOUT16 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_16(&self) -> super::vals::IomuxcXbarDirSel16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::IomuxcXbarDirSel16::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT16 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_16(&mut self, val: super::vals::IomuxcXbarDirSel16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "IOMUXC XBAR_INOUT17 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_17(&self) -> super::vals::IomuxcXbarDirSel17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::IomuxcXbarDirSel17::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT17 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_17(&mut self, val: super::vals::IomuxcXbarDirSel17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "IOMUXC XBAR_INOUT18 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_18(&self) -> super::vals::IomuxcXbarDirSel18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::IomuxcXbarDirSel18::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT18 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_18(&mut self, val: super::vals::IomuxcXbarDirSel18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "IOMUXC XBAR_INOUT19 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_19(&self) -> super::vals::IomuxcXbarDirSel19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::IomuxcXbarDirSel19::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT19 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_19(&mut self, val: super::vals::IomuxcXbarDirSel19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "IOMUXC XBAR_INOUT20 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_20(&self) -> super::vals::IomuxcXbarDirSel20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::IomuxcXbarDirSel20::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT20 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_20(&mut self, val: super::vals::IomuxcXbarDirSel20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "IOMUXC XBAR_INOUT21 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_21(&self) -> super::vals::IomuxcXbarDirSel21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::IomuxcXbarDirSel21::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT21 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_21(&mut self, val: super::vals::IomuxcXbarDirSel21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "IOMUXC XBAR_INOUT22 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_22(&self) -> super::vals::IomuxcXbarDirSel22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::IomuxcXbarDirSel22::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT22 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_22(&mut self, val: super::vals::IomuxcXbarDirSel22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "IOMUXC XBAR_INOUT23 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_23(&self) -> super::vals::IomuxcXbarDirSel23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::IomuxcXbarDirSel23::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT23 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_23(&mut self, val: super::vals::IomuxcXbarDirSel23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "IOMUXC XBAR_INOUT24 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_24(&self) -> super::vals::IomuxcXbarDirSel24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::IomuxcXbarDirSel24::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT24 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_24(&mut self, val: super::vals::IomuxcXbarDirSel24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "IOMUXC XBAR_INOUT25 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_25(&self) -> super::vals::IomuxcXbarDirSel25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::IomuxcXbarDirSel25::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT25 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_25(&mut self, val: super::vals::IomuxcXbarDirSel25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "IOMUXC XBAR_INOUT26 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_26(&self) -> super::vals::IomuxcXbarDirSel26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::IomuxcXbarDirSel26::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT26 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_26(&mut self, val: super::vals::IomuxcXbarDirSel26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "IOMUXC XBAR_INOUT27 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_27(&self) -> super::vals::IomuxcXbarDirSel27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::IomuxcXbarDirSel27::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT27 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_27(&mut self, val: super::vals::IomuxcXbarDirSel27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "IOMUXC XBAR_INOUT28 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_28(&self) -> super::vals::IomuxcXbarDirSel28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::IomuxcXbarDirSel28::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT28 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_28(&mut self, val: super::vals::IomuxcXbarDirSel28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "IOMUXC XBAR_INOUT29 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_29(&self) -> super::vals::IomuxcXbarDirSel29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::IomuxcXbarDirSel29::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT29 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_29(&mut self, val: super::vals::IomuxcXbarDirSel29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "IOMUXC XBAR_INOUT30 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_30(&self) -> super::vals::IomuxcXbarDirSel30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::IomuxcXbarDirSel30::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT30 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_30(&mut self, val: super::vals::IomuxcXbarDirSel30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "IOMUXC XBAR_INOUT31 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_31(&self) -> super::vals::IomuxcXbarDirSel31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::IomuxcXbarDirSel31::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT31 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_31(&mut self, val: super::vals::IomuxcXbarDirSel31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for XbarDirCtrl1 {
    #[inline(always)]
    fn default() -> XbarDirCtrl1 {
        XbarDirCtrl1(0u64 as u32)
    }
}
impl core::fmt::Debug for XbarDirCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XbarDirCtrl1")
            .field("iomuxc_xbar_dir_sel_4", &self.iomuxc_xbar_dir_sel_4())
            .field("iomuxc_xbar_dir_sel_5", &self.iomuxc_xbar_dir_sel_5())
            .field("iomuxc_xbar_dir_sel_6", &self.iomuxc_xbar_dir_sel_6())
            .field("iomuxc_xbar_dir_sel_7", &self.iomuxc_xbar_dir_sel_7())
            .field("iomuxc_xbar_dir_sel_8", &self.iomuxc_xbar_dir_sel_8())
            .field("iomuxc_xbar_dir_sel_9", &self.iomuxc_xbar_dir_sel_9())
            .field("iomuxc_xbar_dir_sel_10", &self.iomuxc_xbar_dir_sel_10())
            .field("iomuxc_xbar_dir_sel_11", &self.iomuxc_xbar_dir_sel_11())
            .field("iomuxc_xbar_dir_sel_12", &self.iomuxc_xbar_dir_sel_12())
            .field("iomuxc_xbar_dir_sel_13", &self.iomuxc_xbar_dir_sel_13())
            .field("iomuxc_xbar_dir_sel_14", &self.iomuxc_xbar_dir_sel_14())
            .field("iomuxc_xbar_dir_sel_15", &self.iomuxc_xbar_dir_sel_15())
            .field("iomuxc_xbar_dir_sel_16", &self.iomuxc_xbar_dir_sel_16())
            .field("iomuxc_xbar_dir_sel_17", &self.iomuxc_xbar_dir_sel_17())
            .field("iomuxc_xbar_dir_sel_18", &self.iomuxc_xbar_dir_sel_18())
            .field("iomuxc_xbar_dir_sel_19", &self.iomuxc_xbar_dir_sel_19())
            .field("iomuxc_xbar_dir_sel_20", &self.iomuxc_xbar_dir_sel_20())
            .field("iomuxc_xbar_dir_sel_21", &self.iomuxc_xbar_dir_sel_21())
            .field("iomuxc_xbar_dir_sel_22", &self.iomuxc_xbar_dir_sel_22())
            .field("iomuxc_xbar_dir_sel_23", &self.iomuxc_xbar_dir_sel_23())
            .field("iomuxc_xbar_dir_sel_24", &self.iomuxc_xbar_dir_sel_24())
            .field("iomuxc_xbar_dir_sel_25", &self.iomuxc_xbar_dir_sel_25())
            .field("iomuxc_xbar_dir_sel_26", &self.iomuxc_xbar_dir_sel_26())
            .field("iomuxc_xbar_dir_sel_27", &self.iomuxc_xbar_dir_sel_27())
            .field("iomuxc_xbar_dir_sel_28", &self.iomuxc_xbar_dir_sel_28())
            .field("iomuxc_xbar_dir_sel_29", &self.iomuxc_xbar_dir_sel_29())
            .field("iomuxc_xbar_dir_sel_30", &self.iomuxc_xbar_dir_sel_30())
            .field("iomuxc_xbar_dir_sel_31", &self.iomuxc_xbar_dir_sel_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XbarDirCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XbarDirCtrl1 {
            iomuxc_xbar_dir_sel_4: super::vals::IomuxcXbarDirSel4,
            iomuxc_xbar_dir_sel_5: super::vals::IomuxcXbarDirSel5,
            iomuxc_xbar_dir_sel_6: super::vals::IomuxcXbarDirSel6,
            iomuxc_xbar_dir_sel_7: super::vals::IomuxcXbarDirSel7,
            iomuxc_xbar_dir_sel_8: super::vals::IomuxcXbarDirSel8,
            iomuxc_xbar_dir_sel_9: super::vals::IomuxcXbarDirSel9,
            iomuxc_xbar_dir_sel_10: super::vals::IomuxcXbarDirSel10,
            iomuxc_xbar_dir_sel_11: super::vals::IomuxcXbarDirSel11,
            iomuxc_xbar_dir_sel_12: super::vals::IomuxcXbarDirSel12,
            iomuxc_xbar_dir_sel_13: super::vals::IomuxcXbarDirSel13,
            iomuxc_xbar_dir_sel_14: super::vals::IomuxcXbarDirSel14,
            iomuxc_xbar_dir_sel_15: super::vals::IomuxcXbarDirSel15,
            iomuxc_xbar_dir_sel_16: super::vals::IomuxcXbarDirSel16,
            iomuxc_xbar_dir_sel_17: super::vals::IomuxcXbarDirSel17,
            iomuxc_xbar_dir_sel_18: super::vals::IomuxcXbarDirSel18,
            iomuxc_xbar_dir_sel_19: super::vals::IomuxcXbarDirSel19,
            iomuxc_xbar_dir_sel_20: super::vals::IomuxcXbarDirSel20,
            iomuxc_xbar_dir_sel_21: super::vals::IomuxcXbarDirSel21,
            iomuxc_xbar_dir_sel_22: super::vals::IomuxcXbarDirSel22,
            iomuxc_xbar_dir_sel_23: super::vals::IomuxcXbarDirSel23,
            iomuxc_xbar_dir_sel_24: super::vals::IomuxcXbarDirSel24,
            iomuxc_xbar_dir_sel_25: super::vals::IomuxcXbarDirSel25,
            iomuxc_xbar_dir_sel_26: super::vals::IomuxcXbarDirSel26,
            iomuxc_xbar_dir_sel_27: super::vals::IomuxcXbarDirSel27,
            iomuxc_xbar_dir_sel_28: super::vals::IomuxcXbarDirSel28,
            iomuxc_xbar_dir_sel_29: super::vals::IomuxcXbarDirSel29,
            iomuxc_xbar_dir_sel_30: super::vals::IomuxcXbarDirSel30,
            iomuxc_xbar_dir_sel_31: super::vals::IomuxcXbarDirSel31,
        }
        let proxy = XbarDirCtrl1 {
            iomuxc_xbar_dir_sel_4: self.iomuxc_xbar_dir_sel_4(),
            iomuxc_xbar_dir_sel_5: self.iomuxc_xbar_dir_sel_5(),
            iomuxc_xbar_dir_sel_6: self.iomuxc_xbar_dir_sel_6(),
            iomuxc_xbar_dir_sel_7: self.iomuxc_xbar_dir_sel_7(),
            iomuxc_xbar_dir_sel_8: self.iomuxc_xbar_dir_sel_8(),
            iomuxc_xbar_dir_sel_9: self.iomuxc_xbar_dir_sel_9(),
            iomuxc_xbar_dir_sel_10: self.iomuxc_xbar_dir_sel_10(),
            iomuxc_xbar_dir_sel_11: self.iomuxc_xbar_dir_sel_11(),
            iomuxc_xbar_dir_sel_12: self.iomuxc_xbar_dir_sel_12(),
            iomuxc_xbar_dir_sel_13: self.iomuxc_xbar_dir_sel_13(),
            iomuxc_xbar_dir_sel_14: self.iomuxc_xbar_dir_sel_14(),
            iomuxc_xbar_dir_sel_15: self.iomuxc_xbar_dir_sel_15(),
            iomuxc_xbar_dir_sel_16: self.iomuxc_xbar_dir_sel_16(),
            iomuxc_xbar_dir_sel_17: self.iomuxc_xbar_dir_sel_17(),
            iomuxc_xbar_dir_sel_18: self.iomuxc_xbar_dir_sel_18(),
            iomuxc_xbar_dir_sel_19: self.iomuxc_xbar_dir_sel_19(),
            iomuxc_xbar_dir_sel_20: self.iomuxc_xbar_dir_sel_20(),
            iomuxc_xbar_dir_sel_21: self.iomuxc_xbar_dir_sel_21(),
            iomuxc_xbar_dir_sel_22: self.iomuxc_xbar_dir_sel_22(),
            iomuxc_xbar_dir_sel_23: self.iomuxc_xbar_dir_sel_23(),
            iomuxc_xbar_dir_sel_24: self.iomuxc_xbar_dir_sel_24(),
            iomuxc_xbar_dir_sel_25: self.iomuxc_xbar_dir_sel_25(),
            iomuxc_xbar_dir_sel_26: self.iomuxc_xbar_dir_sel_26(),
            iomuxc_xbar_dir_sel_27: self.iomuxc_xbar_dir_sel_27(),
            iomuxc_xbar_dir_sel_28: self.iomuxc_xbar_dir_sel_28(),
            iomuxc_xbar_dir_sel_29: self.iomuxc_xbar_dir_sel_29(),
            iomuxc_xbar_dir_sel_30: self.iomuxc_xbar_dir_sel_30(),
            iomuxc_xbar_dir_sel_31: self.iomuxc_xbar_dir_sel_31(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR IO direction control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XbarDirCtrl2(pub u32);
impl XbarDirCtrl2 {
    #[doc = "IOMUXC XBAR_INOUT32 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_32(&self) -> super::vals::IomuxcXbarDirSel32 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::IomuxcXbarDirSel32::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT32 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_32(&mut self, val: super::vals::IomuxcXbarDirSel32) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "IOMUXC XBAR_INOUT33 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_33(&self) -> super::vals::IomuxcXbarDirSel33 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::IomuxcXbarDirSel33::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT33 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_33(&mut self, val: super::vals::IomuxcXbarDirSel33) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "IOMUXC XBAR_INOUT34 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_34(&self) -> super::vals::IomuxcXbarDirSel34 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::IomuxcXbarDirSel34::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT34 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_34(&mut self, val: super::vals::IomuxcXbarDirSel34) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "IOMUXC XBAR_INOUT35 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_35(&self) -> super::vals::IomuxcXbarDirSel35 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::IomuxcXbarDirSel35::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT35 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_35(&mut self, val: super::vals::IomuxcXbarDirSel35) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "IOMUXC XBAR_INOUT36 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_36(&self) -> super::vals::IomuxcXbarDirSel36 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::IomuxcXbarDirSel36::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT36 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_36(&mut self, val: super::vals::IomuxcXbarDirSel36) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "IOMUXC XBAR_INOUT37 function direction select"]
    #[must_use]
    #[inline(always)]
    pub const fn iomuxc_xbar_dir_sel_37(&self) -> super::vals::IomuxcXbarDirSel37 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::IomuxcXbarDirSel37::from_bits(val as u8)
    }
    #[doc = "IOMUXC XBAR_INOUT37 function direction select"]
    #[inline(always)]
    pub const fn set_iomuxc_xbar_dir_sel_37(&mut self, val: super::vals::IomuxcXbarDirSel37) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for XbarDirCtrl2 {
    #[inline(always)]
    fn default() -> XbarDirCtrl2 {
        XbarDirCtrl2(0u64 as u32)
    }
}
impl core::fmt::Debug for XbarDirCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XbarDirCtrl2")
            .field("iomuxc_xbar_dir_sel_32", &self.iomuxc_xbar_dir_sel_32())
            .field("iomuxc_xbar_dir_sel_33", &self.iomuxc_xbar_dir_sel_33())
            .field("iomuxc_xbar_dir_sel_34", &self.iomuxc_xbar_dir_sel_34())
            .field("iomuxc_xbar_dir_sel_35", &self.iomuxc_xbar_dir_sel_35())
            .field("iomuxc_xbar_dir_sel_36", &self.iomuxc_xbar_dir_sel_36())
            .field("iomuxc_xbar_dir_sel_37", &self.iomuxc_xbar_dir_sel_37())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XbarDirCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XbarDirCtrl2 {
            iomuxc_xbar_dir_sel_32: super::vals::IomuxcXbarDirSel32,
            iomuxc_xbar_dir_sel_33: super::vals::IomuxcXbarDirSel33,
            iomuxc_xbar_dir_sel_34: super::vals::IomuxcXbarDirSel34,
            iomuxc_xbar_dir_sel_35: super::vals::IomuxcXbarDirSel35,
            iomuxc_xbar_dir_sel_36: super::vals::IomuxcXbarDirSel36,
            iomuxc_xbar_dir_sel_37: super::vals::IomuxcXbarDirSel37,
        }
        let proxy = XbarDirCtrl2 {
            iomuxc_xbar_dir_sel_32: self.iomuxc_xbar_dir_sel_32(),
            iomuxc_xbar_dir_sel_33: self.iomuxc_xbar_dir_sel_33(),
            iomuxc_xbar_dir_sel_34: self.iomuxc_xbar_dir_sel_34(),
            iomuxc_xbar_dir_sel_35: self.iomuxc_xbar_dir_sel_35(),
            iomuxc_xbar_dir_sel_36: self.iomuxc_xbar_dir_sel_36(),
            iomuxc_xbar_dir_sel_37: self.iomuxc_xbar_dir_sel_37(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR trigger synchronizer control register1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XbarTrigSyncCtrl1(pub u32);
impl XbarTrigSyncCtrl1 {
    #[doc = "Trigger out polarity select"]
    #[must_use]
    #[inline(always)]
    pub const fn pol_sel(&self) -> super::vals::PolSel {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::PolSel::from_bits(val as u8)
    }
    #[doc = "Trigger out polarity select"]
    #[inline(always)]
    pub const fn set_pol_sel(&mut self, val: super::vals::PolSel) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
    #[doc = "Asynchronous trigger in enable"]
    #[must_use]
    #[inline(always)]
    pub const fn async_en(&self) -> super::vals::AsyncEn {
        let val = (self.0 >> 8usize) & 0xff;
        super::vals::AsyncEn::from_bits(val as u8)
    }
    #[doc = "Asynchronous trigger in enable"]
    #[inline(always)]
    pub const fn set_async_en(&mut self, val: super::vals::AsyncEn) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
    }
    #[doc = "Trigger out synchronizer enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_enable(&self) -> super::vals::SyncEnable {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::SyncEnable::from_bits(val as u8)
    }
    #[doc = "Trigger out synchronizer enable"]
    #[inline(always)]
    pub const fn set_sync_enable(&mut self, val: super::vals::SyncEnable) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
}
impl Default for XbarTrigSyncCtrl1 {
    #[inline(always)]
    fn default() -> XbarTrigSyncCtrl1 {
        XbarTrigSyncCtrl1(0u64 as u32)
    }
}
impl core::fmt::Debug for XbarTrigSyncCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XbarTrigSyncCtrl1")
            .field("pol_sel", &self.pol_sel())
            .field("async_en", &self.async_en())
            .field("sync_enable", &self.sync_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XbarTrigSyncCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XbarTrigSyncCtrl1 {
            pol_sel: super::vals::PolSel,
            async_en: super::vals::AsyncEn,
            sync_enable: super::vals::SyncEnable,
        }
        let proxy = XbarTrigSyncCtrl1 {
            pol_sel: self.pol_sel(),
            async_en: self.async_en(),
            sync_enable: self.sync_enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "XBAR trigger synchronizer control register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XbarTrigSyncCtrl2(pub u32);
impl XbarTrigSyncCtrl2 {
    #[doc = "Pulse width control register of channel0"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_width0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse width control register of channel0"]
    #[inline(always)]
    pub const fn set_pulse_width0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Pulse width control register of channel1"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_width1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse width control register of channel1"]
    #[inline(always)]
    pub const fn set_pulse_width1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Pulse width control register of channel2"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_width2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse width control register of channel2"]
    #[inline(always)]
    pub const fn set_pulse_width2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Pulse width control register of channel3"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_width3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse width control register of channel3"]
    #[inline(always)]
    pub const fn set_pulse_width3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Pulse width control register of channel4"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_width4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse width control register of channel4"]
    #[inline(always)]
    pub const fn set_pulse_width4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Pulse width control register of channel5"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_width5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse width control register of channel5"]
    #[inline(always)]
    pub const fn set_pulse_width5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Pulse width control register of channel6"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_width6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse width control register of channel6"]
    #[inline(always)]
    pub const fn set_pulse_width6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Pulse width control register of channel7"]
    #[must_use]
    #[inline(always)]
    pub const fn pulse_width7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Pulse width control register of channel7"]
    #[inline(always)]
    pub const fn set_pulse_width7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for XbarTrigSyncCtrl2 {
    #[inline(always)]
    fn default() -> XbarTrigSyncCtrl2 {
        XbarTrigSyncCtrl2(0u64 as u32)
    }
}
impl core::fmt::Debug for XbarTrigSyncCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XbarTrigSyncCtrl2")
            .field("pulse_width0", &self.pulse_width0())
            .field("pulse_width1", &self.pulse_width1())
            .field("pulse_width2", &self.pulse_width2())
            .field("pulse_width3", &self.pulse_width3())
            .field("pulse_width4", &self.pulse_width4())
            .field("pulse_width5", &self.pulse_width5())
            .field("pulse_width6", &self.pulse_width6())
            .field("pulse_width7", &self.pulse_width7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for XbarTrigSyncCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct XbarTrigSyncCtrl2 {
            pulse_width0: u8,
            pulse_width1: u8,
            pulse_width2: u8,
            pulse_width3: u8,
            pulse_width4: u8,
            pulse_width5: u8,
            pulse_width6: u8,
            pulse_width7: u8,
        }
        let proxy = XbarTrigSyncCtrl2 {
            pulse_width0: self.pulse_width0(),
            pulse_width1: self.pulse_width1(),
            pulse_width2: self.pulse_width2(),
            pulse_width3: self.pulse_width3(),
            pulse_width4: self.pulse_width4(),
            pulse_width5: self.pulse_width5(),
            pulse_width6: self.pulse_width6(),
            pulse_width7: self.pulse_width7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
