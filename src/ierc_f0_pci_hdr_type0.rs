#[doc = "PCI Express ECAM Event Collector config"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IercF0PciHdrType0 {
    ptr: *mut u8,
}
unsafe impl Send for IercF0PciHdrType0 {}
unsafe impl Sync for IercF0PciHdrType0 {}
impl IercF0PciHdrType0 {
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
    #[doc = "PCI interrupt line register"]
    #[inline(always)]
    pub const fn pci_cfh_int_line(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "PCI interrupt pin register"]
    #[inline(always)]
    pub const fn pci_cfh_int_pin(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3dusize) as _) }
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
    #[doc = "PCI PCIe device status register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_dev_stat(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieDevStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4ausize) as _) }
    }
    #[doc = "PCI PCIe root control register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_root_ctl(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieRootCtl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "PCI PCIe root status register"]
    #[inline(always)]
    pub const fn pci_cfc_pcie_root_stat(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcieRootStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "PCI PCI-PM capabilities list register"]
    #[inline(always)]
    pub const fn pci_cfc_pcipm_cap_list(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcipmCapList, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "PCI PCI-PM capabilities register"]
    #[inline(always)]
    pub const fn pci_cfc_pcipm_cap(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcipmCap, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x82usize) as _) }
    }
    #[doc = "PCI PCI-PM control and status register"]
    #[inline(always)]
    pub const fn pci_cfc_pcipm_ctl_stat(
        self,
    ) -> crate::common::Reg<regs::PciCfcPcipmCtlStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "PCI PCI-PM capabilities data register"]
    #[inline(always)]
    pub const fn pci_cfc_pcipm_data(self) -> crate::common::Reg<u8, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x87usize) as _) }
    }
    #[doc = "PCIe AER extended capability header"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_ext_cap_hdr(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerExtCapHdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "PCIe AER root error command register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_root_err_cmd(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerRootErrCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "PCIe AER root error status register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_root_err_stat(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerRootErrStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "PCIe AER error source identification register"]
    #[inline(always)]
    pub const fn pcie_cfc_aer_err_src_id(
        self,
    ) -> crate::common::Reg<regs::PcieCfcAerErrSrcId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "PCIe RCEC Endpoint association extended capability header"]
    #[inline(always)]
    pub const fn pcie_cfc_rcec_epa_ext_cap_hdr(
        self,
    ) -> crate::common::Reg<regs::PcieCfcRcecEpaExtCapHdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "PCIe RCEC Endpoint association bitmap registerr"]
    #[inline(always)]
    pub const fn pcie_cfc_rcec_epa_bitmap(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
}
pub mod regs;
