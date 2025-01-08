#[doc = "PCI PCIe capabilities register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieCap(pub u16);
impl PciCfcPcieCap {
    #[doc = "Capability Version Indicates PCI-SIG defined PCI Express Capability structure version number"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_ver(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Capability Version Indicates PCI-SIG defined PCI Express Capability structure version number"]
    #[inline(always)]
    pub const fn set_cap_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
    }
    #[doc = "Device/Port type Indicates the specific type of this PCI Express Function"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_port_type(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Device/Port type Indicates the specific type of this PCI Express Function"]
    #[inline(always)]
    pub const fn set_dev_port_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
    }
    #[doc = "Interrupt message number This field indicates which MSI/MSI-X vector is used for the interrupt message generated in association with any of the status bits of this capability structure"]
    #[must_use]
    #[inline(always)]
    pub const fn int_msg_num(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "Interrupt message number This field indicates which MSI/MSI-X vector is used for the interrupt message generated in association with any of the status bits of this capability structure"]
    #[inline(always)]
    pub const fn set_int_msg_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u16) & 0x1f) << 9usize);
    }
}
impl Default for PciCfcPcieCap {
    #[inline(always)]
    fn default() -> PciCfcPcieCap {
        PciCfcPcieCap(162u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieCap")
            .field("cap_ver", &self.cap_ver())
            .field("dev_port_type", &self.dev_port_type())
            .field("int_msg_num", &self.int_msg_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieCap {
            cap_ver: u8,
            dev_port_type: u8,
            int_msg_num: u8,
        }
        let proxy = PciCfcPcieCap {
            cap_ver: self.cap_ver(),
            dev_port_type: self.dev_port_type(),
            int_msg_num: self.int_msg_num(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe capabilities list register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieCapList(pub u16);
impl PciCfcPcieCapList {
    #[doc = "Indicates the PCI Express Capability structure. Hardwired to 10h."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the PCI Express Capability structure. Hardwired to 10h."]
    #[inline(always)]
    pub const fn set_cap_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_id(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for PciCfcPcieCapList {
    #[inline(always)]
    fn default() -> PciCfcPcieCapList {
        PciCfcPcieCapList(32784u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieCapList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieCapList")
            .field("cap_id", &self.cap_id())
            .field("next_cap_id", &self.next_cap_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieCapList {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieCapList {
            cap_id: u8,
            next_cap_id: u8,
        }
        let proxy = PciCfcPcieCapList {
            cap_id: self.cap_id(),
            next_cap_id: self.next_cap_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe device capabilities register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieDevCap(pub u32);
impl PciCfcPcieDevCap {
    #[doc = "Function level reset capability This bit applies to Endpoints only, hardwired to 0b."]
    #[must_use]
    #[inline(always)]
    pub const fn flr_cap(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Function level reset capability This bit applies to Endpoints only, hardwired to 0b."]
    #[inline(always)]
    pub const fn set_flr_cap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PciCfcPcieDevCap {
    #[inline(always)]
    fn default() -> PciCfcPcieDevCap {
        PciCfcPcieDevCap(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcPcieDevCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieDevCap")
            .field("flr_cap", &self.flr_cap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieDevCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieDevCap {
            flr_cap: bool,
        }
        let proxy = PciCfcPcieDevCap {
            flr_cap: self.flr_cap(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe device status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieDevStat(pub u16);
impl PciCfcPcieDevStat {
    #[doc = "Transaction pending When set, this bit indicates that the Event Collector has issued Non-Posted Requests that have not been completed"]
    #[must_use]
    #[inline(always)]
    pub const fn trans_pend(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transaction pending When set, this bit indicates that the Event Collector has issued Non-Posted Requests that have not been completed"]
    #[inline(always)]
    pub const fn set_trans_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
    }
}
impl Default for PciCfcPcieDevStat {
    #[inline(always)]
    fn default() -> PciCfcPcieDevStat {
        PciCfcPcieDevStat(0u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieDevStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieDevStat")
            .field("trans_pend", &self.trans_pend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieDevStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieDevStat {
            trans_pend: bool,
        }
        let proxy = PciCfcPcieDevStat {
            trans_pend: self.trans_pend(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe root control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieRootCtl(pub u16);
impl PciCfcPcieRootCtl {
    #[doc = "PME interrupt enable When Set, this bit enables PME interrupt generation upon receipt of a PME Message as reflected in the PME Status bit"]
    #[must_use]
    #[inline(always)]
    pub const fn pme_int_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PME interrupt enable When Set, this bit enables PME interrupt generation upon receipt of a PME Message as reflected in the PME Status bit"]
    #[inline(always)]
    pub const fn set_pme_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
}
impl Default for PciCfcPcieRootCtl {
    #[inline(always)]
    fn default() -> PciCfcPcieRootCtl {
        PciCfcPcieRootCtl(0u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcieRootCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieRootCtl")
            .field("pme_int_en", &self.pme_int_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieRootCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieRootCtl {
            pme_int_en: bool,
        }
        let proxy = PciCfcPcieRootCtl {
            pme_int_en: self.pme_int_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCIe root status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcieRootStat(pub u32);
impl PciCfcPcieRootStat {
    #[doc = "PME requester ID This field indicates the PCI Requester ID of the last PME Requester"]
    #[must_use]
    #[inline(always)]
    pub const fn pme_req_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PME requester ID This field indicates the PCI Requester ID of the last PME Requester"]
    #[inline(always)]
    pub const fn set_pme_req_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "PME status This bit indicates that PME was asserted by the PME Requester indicated in the PME Requester ID field"]
    #[must_use]
    #[inline(always)]
    pub const fn pme_status(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "PME status This bit indicates that PME was asserted by the PME Requester indicated in the PME Requester ID field"]
    #[inline(always)]
    pub const fn set_pme_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "PME pending This bit indicates that another PME is pending when the PME Status bit is Set"]
    #[must_use]
    #[inline(always)]
    pub const fn pme_pend(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "PME pending This bit indicates that another PME is pending when the PME Status bit is Set"]
    #[inline(always)]
    pub const fn set_pme_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for PciCfcPcieRootStat {
    #[inline(always)]
    fn default() -> PciCfcPcieRootStat {
        PciCfcPcieRootStat(0u64 as u32)
    }
}
impl core::fmt::Debug for PciCfcPcieRootStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcieRootStat")
            .field("pme_req_id", &self.pme_req_id())
            .field("pme_status", &self.pme_status())
            .field("pme_pend", &self.pme_pend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcieRootStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcieRootStat {
            pme_req_id: u16,
            pme_status: bool,
            pme_pend: bool,
        }
        let proxy = PciCfcPcieRootStat {
            pme_req_id: self.pme_req_id(),
            pme_status: self.pme_status(),
            pme_pend: self.pme_pend(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCI-PM capabilities register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcipmCap(pub u16);
impl PciCfcPcipmCap {
    #[doc = "Version RCEC complies with the PCI PM specification, rev 1.2."]
    #[must_use]
    #[inline(always)]
    pub const fn version(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Version RCEC complies with the PCI PM specification, rev 1.2."]
    #[inline(always)]
    pub const fn set_version(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u16) & 0x07) << 0usize);
    }
    #[doc = "PME support Event Collector does not support generating PM_PME notifications, hardwired to 00h."]
    #[must_use]
    #[inline(always)]
    pub const fn pme_support(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "PME support Event Collector does not support generating PM_PME notifications, hardwired to 00h."]
    #[inline(always)]
    pub const fn set_pme_support(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u16) & 0x1f) << 11usize);
    }
}
impl Default for PciCfcPcipmCap {
    #[inline(always)]
    fn default() -> PciCfcPcipmCap {
        PciCfcPcipmCap(3u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcipmCap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcipmCap")
            .field("version", &self.version())
            .field("pme_support", &self.pme_support())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcipmCap {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcipmCap {
            version: u8,
            pme_support: u8,
        }
        let proxy = PciCfcPcipmCap {
            version: self.version(),
            pme_support: self.pme_support(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCI-PM capabilities list register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcipmCapList(pub u16);
impl PciCfcPcipmCapList {
    #[doc = "Indicates the PCI-PM Capability structure. Hardwired to 01h."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates the PCI-PM Capability structure. Hardwired to 01h."]
    #[inline(always)]
    pub const fn set_cap_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_id(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "This field contains the offset to the next PCI Capability structure or 00h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
    }
}
impl Default for PciCfcPcipmCapList {
    #[inline(always)]
    fn default() -> PciCfcPcipmCapList {
        PciCfcPcipmCapList(1u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcipmCapList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcipmCapList")
            .field("cap_id", &self.cap_id())
            .field("next_cap_id", &self.next_cap_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcipmCapList {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcipmCapList {
            cap_id: u8,
            next_cap_id: u8,
        }
        let proxy = PciCfcPcipmCapList {
            cap_id: self.cap_id(),
            next_cap_id: self.next_cap_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI PCI-PM control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfcPcipmCtlStat(pub u16);
impl PciCfcPcipmCtlStat {
    #[doc = "Power state This field is used to set and report the power state of a function as follows: 00b = D0 01b = D1 (not supported by EC) 10b = D2 (not supported by EC) 11b = D3 If, for any reason, the operating system software attempts to put a function into a power management state that the function does not support, the function should handle this gracefully and remain in whatever state it was in before the request"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Power state This field is used to set and report the power state of a function as follows: 00b = D0 01b = D1 (not supported by EC) 10b = D2 (not supported by EC) 11b = D3 If, for any reason, the operating system software attempts to put a function into a power management state that the function does not support, the function should handle this gracefully and remain in whatever state it was in before the request"]
    #[inline(always)]
    pub const fn set_pwr_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u16) & 0x03) << 0usize);
    }
    #[doc = "No soft reset When set (\"1\"), this bit indicates that when RCEC transitions from D3hot to D0active because of modifying Power State bits in the PCI_CFC_PCIPM_CTL_STAT register, no internal reset is issued and Configuration Context is preserved"]
    #[must_use]
    #[inline(always)]
    pub const fn no_soft_rst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "No soft reset When set (\"1\"), this bit indicates that when RCEC transitions from D3hot to D0active because of modifying Power State bits in the PCI_CFC_PCIPM_CTL_STAT register, no internal reset is issued and Configuration Context is preserved"]
    #[inline(always)]
    pub const fn set_no_soft_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
}
impl Default for PciCfcPcipmCtlStat {
    #[inline(always)]
    fn default() -> PciCfcPcipmCtlStat {
        PciCfcPcipmCtlStat(8u64 as u16)
    }
}
impl core::fmt::Debug for PciCfcPcipmCtlStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfcPcipmCtlStat")
            .field("pwr_state", &self.pwr_state())
            .field("no_soft_rst", &self.no_soft_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfcPcipmCtlStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfcPcipmCtlStat {
            pwr_state: u8,
            no_soft_rst: bool,
        }
        let proxy = PciCfcPcipmCtlStat {
            pwr_state: self.pwr_state(),
            no_soft_rst: self.no_soft_rst(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhCmd(pub u16);
impl PciCfhCmd {
    #[doc = "Interrupt disable This field controls the ability of an Event Collect to assert the INTx wired interrupts"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_disable(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt disable This field controls the ability of an Event Collect to assert the INTx wired interrupts"]
    #[inline(always)]
    pub const fn set_intr_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
    }
}
impl Default for PciCfhCmd {
    #[inline(always)]
    fn default() -> PciCfhCmd {
        PciCfhCmd(0u64 as u16)
    }
}
impl core::fmt::Debug for PciCfhCmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhCmd")
            .field("intr_disable", &self.intr_disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhCmd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhCmd {
            intr_disable: bool,
        }
        let proxy = PciCfhCmd {
            intr_disable: self.intr_disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI device ID and vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhDidVid(pub u32);
impl PciCfhDidVid {
    #[doc = "Vendor ID This field identifies the manufacturer of the device"]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Vendor ID This field identifies the manufacturer of the device"]
    #[inline(always)]
    pub const fn set_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Device ID This field identifies the device ID of the device"]
    #[must_use]
    #[inline(always)]
    pub const fn device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Device ID This field identifies the device ID of the device"]
    #[inline(always)]
    pub const fn set_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PciCfhDidVid {
    #[inline(always)]
    fn default() -> PciCfhDidVid {
        PciCfhDidVid(3758168407u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhDidVid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhDidVid")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhDidVid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhDidVid {
            vendor_id: u16,
            device_id: u16,
        }
        let proxy = PciCfhDidVid {
            vendor_id: self.vendor_id(),
            device_id: self.device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI header type register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhHdrType(pub u8);
impl PciCfhHdrType {
    #[doc = "Header type This field identifies the layout of the second part of the predefined header (beginning at byte 10h in Configuration Space)"]
    #[must_use]
    #[inline(always)]
    pub const fn hdr_type(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Header type This field identifies the layout of the second part of the predefined header (beginning at byte 10h in Configuration Space)"]
    #[inline(always)]
    pub const fn set_hdr_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
    }
    #[doc = "Multi-function device When set, indicates that the Root Complex contains multiple Event Collectors."]
    #[must_use]
    #[inline(always)]
    pub const fn mult_func_dev(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-function device When set, indicates that the Root Complex contains multiple Event Collectors."]
    #[inline(always)]
    pub const fn set_mult_func_dev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
    }
}
impl Default for PciCfhHdrType {
    #[inline(always)]
    fn default() -> PciCfhHdrType {
        PciCfhHdrType(0u64 as u8)
    }
}
impl core::fmt::Debug for PciCfhHdrType {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhHdrType")
            .field("hdr_type", &self.hdr_type())
            .field("mult_func_dev", &self.mult_func_dev())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhHdrType {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhHdrType {
            hdr_type: u8,
            mult_func_dev: bool,
        }
        let proxy = PciCfhHdrType {
            hdr_type: self.hdr_type(),
            mult_func_dev: self.mult_func_dev(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI revision ID and classcode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhRevidClasscode(pub u32);
impl PciCfhRevidClasscode {
    #[doc = "Revision ID This register specifies a device specific revision identifier and is a vendor defined extension to the Device ID"]
    #[must_use]
    #[inline(always)]
    pub const fn rev_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Revision ID This register specifies a device specific revision identifier and is a vendor defined extension to the Device ID"]
    #[inline(always)]
    pub const fn set_rev_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Class code The Class Code register is read-only and is used to identify the generic function of the device and, in some cases, a specific register level programming interface"]
    #[must_use]
    #[inline(always)]
    pub const fn class_code(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Class code The Class Code register is read-only and is used to identify the generic function of the device and, in some cases, a specific register level programming interface"]
    #[inline(always)]
    pub const fn set_class_code(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for PciCfhRevidClasscode {
    #[inline(always)]
    fn default() -> PciCfhRevidClasscode {
        PciCfhRevidClasscode(134676482u64 as u32)
    }
}
impl core::fmt::Debug for PciCfhRevidClasscode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhRevidClasscode")
            .field("rev_id", &self.rev_id())
            .field("class_code", &self.class_code())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhRevidClasscode {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhRevidClasscode {
            rev_id: u8,
            class_code: u32,
        }
        let proxy = PciCfhRevidClasscode {
            rev_id: self.rev_id(),
            class_code: self.class_code(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCI status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PciCfhStat(pub u16);
impl PciCfhStat {
    #[doc = "Interrupt Status When set, indicates that an INTx interrupt is pending for the Event Collector"]
    #[must_use]
    #[inline(always)]
    pub const fn intr_status(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt Status When set, indicates that an INTx interrupt is pending for the Event Collector"]
    #[inline(always)]
    pub const fn set_intr_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
    }
    #[doc = "Capabilities List Indicates the presence of an Extended Capability list item"]
    #[must_use]
    #[inline(always)]
    pub const fn cap_list(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Capabilities List Indicates the presence of an Extended Capability list item"]
    #[inline(always)]
    pub const fn set_cap_list(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
    }
}
impl Default for PciCfhStat {
    #[inline(always)]
    fn default() -> PciCfhStat {
        PciCfhStat(16u64 as u16)
    }
}
impl core::fmt::Debug for PciCfhStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PciCfhStat")
            .field("intr_status", &self.intr_status())
            .field("cap_list", &self.cap_list())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PciCfhStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PciCfhStat {
            intr_status: bool,
            cap_list: bool,
        }
        let proxy = PciCfhStat {
            intr_status: self.intr_status(),
            cap_list: self.cap_list(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER error source identification register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerErrSrcId(pub u32);
impl PcieCfcAerErrSrcId {
    #[doc = "ERR_CORR Source Identification Loaded with the Requester ID indicated in the received ERR_CORR message when the ERR_CORR received bit is not already set"]
    #[must_use]
    #[inline(always)]
    pub const fn err_corr_src_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "ERR_CORR Source Identification Loaded with the Requester ID indicated in the received ERR_CORR message when the ERR_CORR received bit is not already set"]
    #[inline(always)]
    pub const fn set_err_corr_src_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "ERR_FATAL_NON_FATAL Source Identification Loaded with the Requester ID indicated in the received ERR_FATAL_NON_FATAL message when the ERR_FATAL_NON_FATAL received bit is not already set"]
    #[must_use]
    #[inline(always)]
    pub const fn err_fatal_non_fatal_src_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "ERR_FATAL_NON_FATAL Source Identification Loaded with the Requester ID indicated in the received ERR_FATAL_NON_FATAL message when the ERR_FATAL_NON_FATAL received bit is not already set"]
    #[inline(always)]
    pub const fn set_err_fatal_non_fatal_src_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PcieCfcAerErrSrcId {
    #[inline(always)]
    fn default() -> PcieCfcAerErrSrcId {
        PcieCfcAerErrSrcId(0u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerErrSrcId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerErrSrcId")
            .field("err_corr_src_id", &self.err_corr_src_id())
            .field(
                "err_fatal_non_fatal_src_id",
                &self.err_fatal_non_fatal_src_id(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerErrSrcId {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerErrSrcId {
            err_corr_src_id: u16,
            err_fatal_non_fatal_src_id: u16,
        }
        let proxy = PcieCfcAerErrSrcId {
            err_corr_src_id: self.err_corr_src_id(),
            err_fatal_non_fatal_src_id: self.err_fatal_non_fatal_src_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER extended capability header"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerExtCapHdr(pub u32);
impl PcieCfcAerExtCapHdr {
    #[doc = "The Extended Capability ID for the AER Extended Capability is 0001h."]
    #[must_use]
    #[inline(always)]
    pub const fn pcie_ext_cap_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The Extended Capability ID for the AER Extended Capability is 0001h."]
    #[inline(always)]
    pub const fn set_pcie_ext_cap_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_ver(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[inline(always)]
    pub const fn set_cap_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_off(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for PcieCfcAerExtCapHdr {
    #[inline(always)]
    fn default() -> PcieCfcAerExtCapHdr {
        PcieCfcAerExtCapHdr(327221249u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerExtCapHdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerExtCapHdr")
            .field("pcie_ext_cap_id", &self.pcie_ext_cap_id())
            .field("cap_ver", &self.cap_ver())
            .field("next_cap_off", &self.next_cap_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerExtCapHdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerExtCapHdr {
            pcie_ext_cap_id: u16,
            cap_ver: u8,
            next_cap_off: u16,
        }
        let proxy = PcieCfcAerExtCapHdr {
            pcie_ext_cap_id: self.pcie_ext_cap_id(),
            cap_ver: self.cap_ver(),
            next_cap_off: self.next_cap_off(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER root error command register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerRootErrCmd(pub u32);
impl PcieCfcAerRootErrCmd {
    #[doc = "Correctable error reporting enable When set, this bit enables the generation of an interrupt when a correctable error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    #[must_use]
    #[inline(always)]
    pub const fn corr_err_rpt_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Correctable error reporting enable When set, this bit enables the generation of an interrupt when a correctable error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    #[inline(always)]
    pub const fn set_corr_err_rpt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Non-fatal error reporting enable When set, this bit enables the generation of an interrupt when a non-fatal error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    #[must_use]
    #[inline(always)]
    pub const fn non_fatal_err_rpt_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Non-fatal error reporting enable When set, this bit enables the generation of an interrupt when a non-fatal error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    #[inline(always)]
    pub const fn set_non_fatal_err_rpt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fatal error reporting enable When set, this bit enables the generation of an interrupt when a fatal error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    #[must_use]
    #[inline(always)]
    pub const fn fatal_err_rpt_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal error reporting enable When set, this bit enables the generation of an interrupt when a fatal error is reported by any of the Integrated Endpoints (iEPs) or the Event Collector itself"]
    #[inline(always)]
    pub const fn set_fatal_err_rpt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for PcieCfcAerRootErrCmd {
    #[inline(always)]
    fn default() -> PcieCfcAerRootErrCmd {
        PcieCfcAerRootErrCmd(0u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerRootErrCmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerRootErrCmd")
            .field("corr_err_rpt_en", &self.corr_err_rpt_en())
            .field("non_fatal_err_rpt_en", &self.non_fatal_err_rpt_en())
            .field("fatal_err_rpt_en", &self.fatal_err_rpt_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerRootErrCmd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerRootErrCmd {
            corr_err_rpt_en: bool,
            non_fatal_err_rpt_en: bool,
            fatal_err_rpt_en: bool,
        }
        let proxy = PcieCfcAerRootErrCmd {
            corr_err_rpt_en: self.corr_err_rpt_en(),
            non_fatal_err_rpt_en: self.non_fatal_err_rpt_en(),
            fatal_err_rpt_en: self.fatal_err_rpt_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe AER root error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcAerRootErrStat(pub u32);
impl PcieCfcAerRootErrStat {
    #[doc = "Correctable error received Set when a correctable error message is received and this bit is not already set"]
    #[must_use]
    #[inline(always)]
    pub const fn err_corr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Correctable error received Set when a correctable error message is received and this bit is not already set"]
    #[inline(always)]
    pub const fn set_err_corr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Multiple correctable errors received Set when a correctable error message is received and ERR_CORR received is already set"]
    #[must_use]
    #[inline(always)]
    pub const fn mult_err_corr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple correctable errors received Set when a correctable error message is received and ERR_CORR received is already set"]
    #[inline(always)]
    pub const fn set_mult_err_corr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Fatal/Non-fatal error received Set when either a fatal or a non-fatal error message is received and this bit is not already set"]
    #[must_use]
    #[inline(always)]
    pub const fn err_fatal_non_fatal(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal/Non-fatal error received Set when either a fatal or a non-fatal error message is received and this bit is not already set"]
    #[inline(always)]
    pub const fn set_err_fatal_non_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Multiple fatal/non-fatal errors received Set when either a fatal or a non-fatal error is received and ERR_FATAL_NON_FATAL received is already set"]
    #[must_use]
    #[inline(always)]
    pub const fn mult_err_fatal_non_fatal(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple fatal/non-fatal errors received Set when either a fatal or a non-fatal error is received and ERR_FATAL_NON_FATAL received is already set"]
    #[inline(always)]
    pub const fn set_mult_err_fatal_non_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "First uncorrectable error fatal Set when the first uncorrectable error message received is for a fatal error"]
    #[must_use]
    #[inline(always)]
    pub const fn first_ucorr_fatal(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "First uncorrectable error fatal Set when the first uncorrectable error message received is for a fatal error"]
    #[inline(always)]
    pub const fn set_first_ucorr_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Non-fatal error received Set when one or more non-fatal uncorrectable error messages have been received"]
    #[must_use]
    #[inline(always)]
    pub const fn err_non_fatal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Non-fatal error received Set when one or more non-fatal uncorrectable error messages have been received"]
    #[inline(always)]
    pub const fn set_err_non_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Fatal error received Set when one or more fatal uncorrectable error messages have been received"]
    #[must_use]
    #[inline(always)]
    pub const fn err_fatal(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Fatal error received Set when one or more fatal uncorrectable error messages have been received"]
    #[inline(always)]
    pub const fn set_err_fatal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for PcieCfcAerRootErrStat {
    #[inline(always)]
    fn default() -> PcieCfcAerRootErrStat {
        PcieCfcAerRootErrStat(0u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcAerRootErrStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcAerRootErrStat")
            .field("err_corr", &self.err_corr())
            .field("mult_err_corr", &self.mult_err_corr())
            .field("err_fatal_non_fatal", &self.err_fatal_non_fatal())
            .field("mult_err_fatal_non_fatal", &self.mult_err_fatal_non_fatal())
            .field("first_ucorr_fatal", &self.first_ucorr_fatal())
            .field("err_non_fatal", &self.err_non_fatal())
            .field("err_fatal", &self.err_fatal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcAerRootErrStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcAerRootErrStat {
            err_corr: bool,
            mult_err_corr: bool,
            err_fatal_non_fatal: bool,
            mult_err_fatal_non_fatal: bool,
            first_ucorr_fatal: bool,
            err_non_fatal: bool,
            err_fatal: bool,
        }
        let proxy = PcieCfcAerRootErrStat {
            err_corr: self.err_corr(),
            mult_err_corr: self.mult_err_corr(),
            err_fatal_non_fatal: self.err_fatal_non_fatal(),
            mult_err_fatal_non_fatal: self.mult_err_fatal_non_fatal(),
            first_ucorr_fatal: self.first_ucorr_fatal(),
            err_non_fatal: self.err_non_fatal(),
            err_fatal: self.err_fatal(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PCIe RCEC Endpoint association extended capability header"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PcieCfcRcecEpaExtCapHdr(pub u32);
impl PcieCfcRcecEpaExtCapHdr {
    #[doc = "The Extended Capability ID for the Event Collector Endpoint Association Capability is 0007h."]
    #[must_use]
    #[inline(always)]
    pub const fn pcie_ext_cap_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The Extended Capability ID for the Event Collector Endpoint Association Capability is 0007h."]
    #[inline(always)]
    pub const fn set_pcie_ext_cap_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[must_use]
    #[inline(always)]
    pub const fn cap_ver(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Capability version Must be 1h for this version of the specification."]
    #[inline(always)]
    pub const fn set_cap_ver(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[must_use]
    #[inline(always)]
    pub const fn next_cap_off(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Next capability offset This field contains the offset to the next PCI Express Capability structure or 000h if no other items exist in the linked list of capabilities"]
    #[inline(always)]
    pub const fn set_next_cap_off(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for PcieCfcRcecEpaExtCapHdr {
    #[inline(always)]
    fn default() -> PcieCfcRcecEpaExtCapHdr {
        PcieCfcRcecEpaExtCapHdr(65543u64 as u32)
    }
}
impl core::fmt::Debug for PcieCfcRcecEpaExtCapHdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PcieCfcRcecEpaExtCapHdr")
            .field("pcie_ext_cap_id", &self.pcie_ext_cap_id())
            .field("cap_ver", &self.cap_ver())
            .field("next_cap_off", &self.next_cap_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PcieCfcRcecEpaExtCapHdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PcieCfcRcecEpaExtCapHdr {
            pcie_ext_cap_id: u16,
            cap_ver: u8,
            next_cap_off: u16,
        }
        let proxy = PcieCfcRcecEpaExtCapHdr {
            pcie_ext_cap_id: self.pcie_ext_cap_id(),
            cap_ver: self.cap_ver(),
            next_cap_off: self.next_cap_off(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
