#[doc = "NETC PCI Express ECAM PF config"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NetcF1PciHdrType0 {
    ptr: *mut u8,
}
unsafe impl Send for NetcF1PciHdrType0 {}
unsafe impl Sync for NetcF1PciHdrType0 {}
impl NetcF1PciHdrType0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PCI device ID and vendor ID register"]
    #[inline(always)]
    pub const fn pci_cfh_did_vid(self) -> crate::common::Reg<regs::PciCfhDidVid, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PCI command register"]
    #[inline(always)]
    pub const fn pci_cfh_cmd(self) -> crate::common::Reg<regs::PciCfhCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PCI status register"]
    #[inline(always)]
    pub const fn pci_cfh_stat(self) -> crate::common::Reg<regs::PciCfhStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "PCI revision ID and classcode register"]
    #[inline(always)]
    pub const fn pci_cfh_revid_classcode(
        self,
    ) -> crate::common::Reg<regs::PciCfhRevidClasscode, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PCI cache line size register"]
    #[inline(always)]
    pub const fn pci_cfh_cl_size(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PCI header type register"]
    #[inline(always)]
    pub const fn pci_cfh_hdr_type(
        self,
    ) -> crate::common::Reg<regs::PciCfhHdrType, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0eusize) as _) }
    }
    #[doc = "PCI base address register 0"]
    #[inline(always)]
    pub const fn pci_cfh_bar0(self) -> crate::common::Reg<regs::PciCfhBar0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PCI base address register 1"]
    #[inline(always)]
    pub const fn pci_cfh_bar1(self) -> crate::common::Reg<regs::PciCfhBar1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PCI base address register 2"]
    #[inline(always)]
    pub const fn pci_cfh_bar2(self) -> crate::common::Reg<regs::PciCfhBar2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "PCI base address register 3"]
    #[inline(always)]
    pub const fn pci_cfh_bar3(self) -> crate::common::Reg<regs::PciCfhBar3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "PCI base address register 4"]
    #[inline(always)]
    pub const fn pci_cfh_bar4(self) -> crate::common::Reg<regs::PciCfhBar4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PCI base address register 5"]
    #[inline(always)]
    pub const fn pci_cfh_bar5(self) -> crate::common::Reg<regs::PciCfhBar5, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PCI subsystem vendor ID register"]
    #[inline(always)]
    pub const fn pci_cfh_subsys_vid(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "PCI subsystem ID register"]
    #[inline(always)]
    pub const fn pci_cfh_subsys_id(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2eusize) as _) }
    }
    #[doc = "PCI capabilities pointer register"]
    #[inline(always)]
    pub const fn pci_cfh_cap_ptr(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "PCI PCIe capabilities list register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_cap_list(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieCapList, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "PCI PCIe capabilities register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_cap(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieCap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x42usize) as _) }
    }
    #[doc = "PCI PCIe device capabilities register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_dev_cap(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieDevCap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "PCI PCIe device control register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_dev_ctl(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieDevCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "PCI PCIe device status register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_dev_stat(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieDevStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4ausize) as _) }
    }
    #[doc = "PCI PCIe device capabilities 2 register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_dev_cap2(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieDevCap2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "PCI PCIe device control 2 register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_dev_ctl2(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieDevCtl2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "PCI MSI-X capabilities list register"]
    #[inline(always)]
    pub const fn pci_cfc_msix_cap_list(
        self,
    ) -> crate::common::Reg<regs::PciCfcMsixCapList, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "PCI MSI-X message control register"]
    #[inline(always)]
    pub const fn pci_cfc_msix_msg_ctl(
        self,
    ) -> crate::common::Reg<regs::PciCfcMsixMsgCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x82usize) as _) }
    }
    #[doc = "PCI MSI-X table offset/BIR register"]
    #[inline(always)]
    pub const fn pci_cfc_msix_table_off_bir(
        self,
    ) -> crate::common::Reg<regs::PciCfcMsixTableOffBir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "PCI MSI-X PBA offset/BIR register"]
    #[inline(always)]
    pub const fn pci_cfc_msix_pba_off_bir(
        self,
    ) -> crate::common::Reg<regs::PciCfcMsixPbaOffBir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "PCI PCI-PM capabilities list register"]
    #[inline(always)]
    pub const fn pci_cfc_pcipm_cap_list(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcipmCapList, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "PCI PCI-PM capabilities register"]
    #[inline(always)]
    pub const fn pci_cfc_pcipm_cap(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcipmCap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x92usize) as _) }
    }
    #[doc = "PCI PCI-PM control and status register"]
    #[inline(always)]
    pub const fn pci_cfc_pcipm_ctl_stat(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcipmCtlStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "PCI PCI-PM capabilities data register"]
    #[inline(always)]
    pub const fn pci_cfc_pcipm_data(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x97usize) as _) }
    }
    #[doc = "PCI EA capabilities list register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_cap_list(
        self,
    ) -> crate::common::Reg<regs::PciCfcEaCapList, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "PCI EA capabilities register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_cap(self) -> crate::common::Reg<regs::PciCfcEaCap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9eusize) as _) }
    }
    #[doc = "PCI EA per-entry 0 format register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_pe0_fmt(
        self,
    ) -> crate::common::Reg<regs::PciCfcEaPe0Fmt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "PCI EA per-entry 0 base register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_pe0_base(
        self,
    ) -> crate::common::Reg<regs::PciCfcEaPe0Base, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "PCI EA per-entry 0 max offset register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_pe0_maxoff(
        self,
    ) -> crate::common::Reg<regs::PciCfcEaPe0Maxoff, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "PCI EA per-entry 0 extended base register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_pe0_ext_base(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "PCI EA per-entry 1 format register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_pe1_fmt(
        self,
    ) -> crate::common::Reg<regs::PciCfcEaPe1Fmt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "PCI EA per-entry 1 base register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_pe1_base(
        self,
    ) -> crate::common::Reg<regs::PciCfcEaPe1Base, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "PCI EA per-entry 1 max offset register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_pe1_maxoff(
        self,
    ) -> crate::common::Reg<regs::PciCfcEaPe1Maxoff, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "PCI EA per-entry 1 extended base register"]
    #[inline(always)]
    pub const fn pci_cfc_ea_pe1_ext_base(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "PCIe AER extended capability header"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_ext_cap_hdr(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerExtCapHdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "PCIe AER uncorrectable error status register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_ucorr_err_stat(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerUcorrErrStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "PCIe AER uncorrectable error mask register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_ucorr_err_mask(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerUcorrErrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "PCIe AER uncorrectable error severity register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_ucorr_err_sev(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerUcorrErrSev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "PCIe AER correctable error status register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_corr_err_stat(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerCorrErrStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "PCIe AER correctable error mask register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_corr_err_mask(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerCorrErrMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "PCIe AER capabilities and control register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_cap_ctl(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerCapCtl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "PCIe ACS capability header"]
    #[inline(always)]
    pub const fn pcie_cfc_acs_cap_hdr(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAcsCapHdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "PCIe ACS capability register"]
    #[inline(always)]
    pub const fn pcie_cfc_acs_cap(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAcsCap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "PCIe ACS control register"]
    #[inline(always)]
    pub const fn pcie_cfc_acs_ctl(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAcsCtl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0136usize) as _) }
    }
    #[doc = "PCIe readiness time reporting capability header"]
    #[inline(always)]
    pub const fn pcie_cfc_rtr_cap_hdr(
        self,
    ) -> crate::common::Reg<regs::PcieCfcRtrCapHdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "PCIe RTR readiness time reporting 1 register"]
    #[inline(always)]
    pub const fn pcie_cfc_rtr_rtr1(
        self,
    ) -> crate::common::Reg<regs::PcieCfcRtrRtr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "PCIe RTR readiness time reporting 2 register"]
    #[inline(always)]
    pub const fn pcie_cfc_rtr_rtr2(
        self,
    ) -> crate::common::Reg<regs::PcieCfcRtrRtr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
}
pub mod regs;
