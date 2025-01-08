#[doc = "Block Control WAKEUP Domain"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BlkCtrlWakeupmix {
    ptr: *mut u8,
}
unsafe impl Send for BlkCtrlWakeupmix {}
unsafe impl Sync for BlkCtrlWakeupmix {}
impl BlkCtrlWakeupmix {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "IPG DEBUG mask bit"]
    #[inline(always)]
    pub const fn ipg_debug1(self) -> crate::common::Reg<regs::IpgDebug1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "IPG DEBUG mask bit"]
    #[inline(always)]
    pub const fn ipg_debug2(self) -> crate::common::Reg<regs::IpgDebug2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "IPG DEBUG mask bit"]
    #[inline(always)]
    pub const fn ipg_debug3(self) -> crate::common::Reg<regs::IpgDebug3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SSI master low power mode control"]
    #[inline(always)]
    pub const fn ssi(self) -> crate::common::Reg<regs::Ssi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "EtherCAT miscellaneous configuration"]
    #[inline(always)]
    pub const fn ecat_misc_cfg(self) -> crate::common::Reg<regs::EcatMiscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "DEXSC error response configuration"]
    #[inline(always)]
    pub const fn dexsc_err(self) -> crate::common::Reg<regs::DexscErr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "USBPHY miscellaneous control"]
    #[inline(always)]
    pub const fn usbphy_misc_ctrl(
        self,
    ) -> crate::common::Reg<regs::UsbphyMiscCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "NETC Port miscellaneous configuration"]
    #[inline(always)]
    pub const fn netc_port_misc_cfg(
        self,
    ) -> crate::common::Reg<regs::NetcPortMiscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "M7 NMI interrupt clear register"]
    #[inline(always)]
    pub const fn m7_nmi_clr(self) -> crate::common::Reg<regs::M7NmiClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Qtimer miscellaneous control register 1"]
    #[inline(always)]
    pub const fn qtimer_ctrl1(self) -> crate::common::Reg<regs::QtimerCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Qtimer miscellaneous control register 2"]
    #[inline(always)]
    pub const fn qtimer_ctrl2(self) -> crate::common::Reg<regs::QtimerCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "SAI2 MCLK control register"]
    #[inline(always)]
    pub const fn sai2_mclk_ctrl(self) -> crate::common::Reg<regs::Sai2MclkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "SAI3 MCLK control register"]
    #[inline(always)]
    pub const fn sai3_mclk_ctrl(self) -> crate::common::Reg<regs::Sai3MclkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "SAI4 MCLK control register"]
    #[inline(always)]
    pub const fn sai4_mclk_ctrl(self) -> crate::common::Reg<regs::Sai4MclkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "XBAR IO direction control register"]
    #[inline(always)]
    pub const fn xbar_dir_ctrl1(self) -> crate::common::Reg<regs::XbarDirCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "XBAR IO direction control register"]
    #[inline(always)]
    pub const fn xbar_dir_ctrl2(self) -> crate::common::Reg<regs::XbarDirCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "LPIT trigger input select register"]
    #[inline(always)]
    pub const fn lpit_trig_sel(self) -> crate::common::Reg<regs::LpitTrigSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "AXI bus attribute configuration register"]
    #[inline(always)]
    pub const fn axi_attr_cfg(self) -> crate::common::Reg<regs::AxiAttrCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "SRAM Control Register 0"]
    #[inline(always)]
    pub const fn sramcr0(self) -> crate::common::Reg<regs::Sramcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "SRAM Control Register 1"]
    #[inline(always)]
    pub const fn sramcr1(self) -> crate::common::Reg<regs::Sramcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Slave stop mode configure register"]
    #[inline(always)]
    pub const fn slave_stop_mode_cfg(
        self,
    ) -> crate::common::Reg<regs::SlaveStopModeCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "I3C2 async wakeup control register"]
    #[inline(always)]
    pub const fn i3c2_async_wakeup_ctrl(
        self,
    ) -> crate::common::Reg<regs::I3c2AsyncWakeupCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "XBAR and AOI write protect register"]
    #[inline(always)]
    pub const fn xbar_aoi_we(self) -> crate::common::Reg<regs::XbarAoiWe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "XBAR trigger synchronizer control register1"]
    #[inline(always)]
    pub const fn xbar_trig_sync_ctrl1(
        self,
    ) -> crate::common::Reg<regs::XbarTrigSyncCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "XBAR trigger synchronizer control register2"]
    #[inline(always)]
    pub const fn xbar_trig_sync_ctrl2(
        self,
    ) -> crate::common::Reg<regs::XbarTrigSyncCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "NETC link configuration for port0"]
    #[inline(always)]
    pub const fn netc_link_cfg0(self) -> crate::common::Reg<regs::NetcLinkCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "NETC link configuration for port1"]
    #[inline(always)]
    pub const fn netc_link_cfg1(self) -> crate::common::Reg<regs::NetcLinkCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "NETC link configuration for port2"]
    #[inline(always)]
    pub const fn netc_link_cfg2(self) -> crate::common::Reg<regs::NetcLinkCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "NETC link configuration for port3"]
    #[inline(always)]
    pub const fn netc_link_cfg3(self) -> crate::common::Reg<regs::NetcLinkCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "NETC link configuration for port4"]
    #[inline(always)]
    pub const fn netc_link_cfg4(self) -> crate::common::Reg<regs::NetcLinkCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "NETC RevMII RGMII delay line configuration for port0"]
    #[inline(always)]
    pub const fn netc_revmii_dll0(
        self,
    ) -> crate::common::Reg<regs::NetcRevmiiDll0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "NETC RevMII RGMII delay line configuration for port1"]
    #[inline(always)]
    pub const fn netc_revmii_dll1(
        self,
    ) -> crate::common::Reg<regs::NetcRevmiiDll1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "NETC RevMII RGMII delay line configuration for port2"]
    #[inline(always)]
    pub const fn netc_revmii_dll2(
        self,
    ) -> crate::common::Reg<regs::NetcRevmiiDll2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "NETC RevMII RGMII delay line configuration for port3"]
    #[inline(always)]
    pub const fn netc_revmii_dll3(
        self,
    ) -> crate::common::Reg<regs::NetcRevmiiDll3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "NETC RevMII RGMII delay line configuration for port4"]
    #[inline(always)]
    pub const fn netc_revmii_dll4(
        self,
    ) -> crate::common::Reg<regs::NetcRevmiiDll4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "Safety clock monitor control and status register"]
    #[inline(always)]
    pub const fn safety_clk_mon_cs(
        self,
    ) -> crate::common::Reg<regs::SafetyClkMonCs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Safety clock monitor threshold register"]
    #[inline(always)]
    pub const fn safety_clk_mon_th(
        self,
    ) -> crate::common::Reg<regs::SafetyClkMonTh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "GPIO_EMC_B1 bank IO control"]
    #[inline(always)]
    pub const fn emc_b1_io_ctrl(self) -> crate::common::Reg<regs::EmcB1IoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "GPIO_EMC_B2 bank IO control"]
    #[inline(always)]
    pub const fn emc_b2_io_ctrl(self) -> crate::common::Reg<regs::EmcB2IoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "GPIO_SD_B1 bank IO control"]
    #[inline(always)]
    pub const fn sd_b1_io_ctrl(self) -> crate::common::Reg<regs::SdB1IoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "GPIO_SD_B2 bank IO control"]
    #[inline(always)]
    pub const fn sd_b2_io_ctrl(self) -> crate::common::Reg<regs::SdB2IoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "GPIO_B1 bank IO control"]
    #[inline(always)]
    pub const fn gpio_b1_io_ctrl(
        self,
    ) -> crate::common::Reg<regs::GpioB1IoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "GPIO_B2 bank IO control"]
    #[inline(always)]
    pub const fn gpio_b2_io_ctrl(
        self,
    ) -> crate::common::Reg<regs::GpioB2IoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "Miscellaneous control register of IO"]
    #[inline(always)]
    pub const fn misc_io_ctrl(self) -> crate::common::Reg<regs::MiscIoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
}
pub mod regs;
pub mod vals;
