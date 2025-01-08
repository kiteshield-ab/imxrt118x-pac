#[doc = "Capability register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capr0(pub u32);
impl Capr0 {
    #[doc = "Number of Root Complexes supported. Range: 0..15"]
    #[must_use]
    #[inline(always)]
    pub const fn num_rc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Root Complexes supported. Range: 0..15"]
    #[inline(always)]
    pub const fn set_num_rc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Number of EMDIO instances supported. Range: 0..1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_emdio(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Number of EMDIO instances supported. Range: 0..1"]
    #[inline(always)]
    pub const fn set_num_emdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of timer instances supported. Range: 0..2"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tmr(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Number of timer instances supported. Range: 0..2"]
    #[inline(always)]
    pub const fn set_num_tmr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Indicates the number of links supported (internal and external). Range: 0..31"]
    #[must_use]
    #[inline(always)]
    pub const fn num_links(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the number of links supported (internal and external). Range: 0..31"]
    #[inline(always)]
    pub const fn set_num_links(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Number of switch instances supported. Range: 0..2"]
    #[must_use]
    #[inline(always)]
    pub const fn num_sw(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Number of switch instances supported. Range: 0..2"]
    #[inline(always)]
    pub const fn set_num_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Number of ENETC instances supported. Range: 0..23"]
    #[must_use]
    #[inline(always)]
    pub const fn num_enetc(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of ENETC instances supported. Range: 0..23"]
    #[inline(always)]
    pub const fn set_num_enetc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "Total number of ENETC VSI's instances supported. Range: 0..64"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vsi(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "Total number of ENETC VSI's instances supported. Range: 0..64"]
    #[inline(always)]
    pub const fn set_num_vsi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Capr0 {
    #[inline(always)]
    fn default() -> Capr0 {
        Capr0(17892945u64 as u32)
    }
}
impl core::fmt::Debug for Capr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capr0")
            .field("num_rc", &self.num_rc())
            .field("num_emdio", &self.num_emdio())
            .field("num_tmr", &self.num_tmr())
            .field("num_links", &self.num_links())
            .field("num_sw", &self.num_sw())
            .field("num_enetc", &self.num_enetc())
            .field("num_vsi", &self.num_vsi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Capr0 {
            num_rc: u8,
            num_emdio: bool,
            num_tmr: u8,
            num_links: u8,
            num_sw: u8,
            num_enetc: u8,
            num_vsi: u8,
        }
        let proxy = Capr0 {
            num_rc: self.num_rc(),
            num_emdio: self.num_emdio(),
            num_tmr: self.num_tmr(),
            num_links: self.num_links(),
            num_sw: self.num_sw(),
            num_enetc: self.num_enetc(),
            num_vsi: self.num_vsi(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capability register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capr1(pub u32);
impl Capr1 {
    #[doc = "Total number of receive BD rings supported by NETC. Range: 0..1023"]
    #[must_use]
    #[inline(always)]
    pub const fn num_rx_bdr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Total number of receive BD rings supported by NETC. Range: 0..1023"]
    #[inline(always)]
    pub const fn set_num_rx_bdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Total number of transmit BD rings supported by NETC. Range: 0..1023"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tx_bdr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Total number of transmit BD rings supported by NETC. Range: 0..1023"]
    #[inline(always)]
    pub const fn set_num_tx_bdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Capr1 {
    #[inline(always)]
    fn default() -> Capr1 {
        Capr1(917518u64 as u32)
    }
}
impl core::fmt::Debug for Capr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capr1")
            .field("num_rx_bdr", &self.num_rx_bdr())
            .field("num_tx_bdr", &self.num_tx_bdr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Capr1 {
            num_rx_bdr: u16,
            num_tx_bdr: u16,
        }
        let proxy = Capr1 {
            num_rx_bdr: self.num_rx_bdr(),
            num_tx_bdr: self.num_tx_bdr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capability register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capr2(pub u32);
impl Capr2 {
    #[doc = "Number of MSI-X table entries available for allocation by NETC functions"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Number of MSI-X table entries available for allocation by NETC functions"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for Capr2 {
    #[inline(always)]
    fn default() -> Capr2 {
        Capr2(52u64 as u32)
    }
}
impl core::fmt::Debug for Capr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capr2")
            .field("num_msix", &self.num_msix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Capr2 {
            num_msix: u16,
        }
        let proxy = Capr2 {
            num_msix: self.num_msix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Capability register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capr3(pub u32);
impl Capr3 {
    #[doc = "Total number of ENETC SI MAC address filter rules supported by NETC."]
    #[must_use]
    #[inline(always)]
    pub const fn num_mac_afte(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Total number of ENETC SI MAC address filter rules supported by NETC."]
    #[inline(always)]
    pub const fn set_num_mac_afte(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Total number of ENETC SI VLAN filter rules supported by NETC."]
    #[must_use]
    #[inline(always)]
    pub const fn num_vlan_fte(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Total number of ENETC SI VLAN filter rules supported by NETC."]
    #[inline(always)]
    pub const fn set_num_vlan_fte(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Capr3 {
    #[inline(always)]
    fn default() -> Capr3 {
        Capr3(524296u64 as u32)
    }
}
impl core::fmt::Debug for Capr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Capr3")
            .field("num_mac_afte", &self.num_mac_afte())
            .field("num_vlan_fte", &self.num_vlan_fte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Capr3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Capr3 {
            num_mac_afte: u16,
            num_vlan_fte: u16,
        }
        let proxy = Capr3 {
            num_mac_afte: self.num_mac_afte(),
            num_vlan_fte: self.num_vlan_fte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Common memory capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmcapr(pub u32);
impl Cmcapr {
    #[doc = "Total amount of common memory in words available to NETC"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Total amount of common memory in words available to NETC"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Word size in bytes 0: 24B 1-3: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn word_size(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Word size in bytes 0: 24B 1-3: reserved"]
    #[inline(always)]
    pub const fn set_word_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for Cmcapr {
    #[inline(always)]
    fn default() -> Cmcapr {
        Cmcapr(10240u64 as u32)
    }
}
impl core::fmt::Debug for Cmcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmcapr")
            .field("num_words", &self.num_words())
            .field("word_size", &self.word_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmcapr {
            num_words: u32,
            word_size: u8,
        }
        let proxy = Cmcapr {
            num_words: self.num_words(),
            word_size: self.word_size(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 config capability VF device ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0CfcVfdid(pub u32);
impl E0CfcVfdid {
    #[doc = "VF Device ID This field identifies the device ID for a virtual function as shown in the PCIe SR-IOV Capability Structure (20h)"]
    #[must_use]
    #[inline(always)]
    pub const fn vf_device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VF Device ID This field identifies the device ID for a virtual function as shown in the PCIe SR-IOV Capability Structure (20h)"]
    #[inline(always)]
    pub const fn set_vf_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for E0CfcVfdid {
    #[inline(always)]
    fn default() -> E0CfcVfdid {
        E0CfcVfdid(4009754624u64 as u32)
    }
}
impl core::fmt::Debug for E0CfcVfdid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0CfcVfdid")
            .field("vf_device_id", &self.vf_device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0CfcVfdid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0CfcVfdid {
            vf_device_id: u16,
        }
        let proxy = E0CfcVfdid {
            vf_device_id: self.vf_device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 config header device ID and vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0CfhDidvid(pub u32);
impl E0CfhDidvid {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[inline(always)]
    pub const fn set_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[inline(always)]
    pub const fn set_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for E0CfhDidvid {
    #[inline(always)]
    fn default() -> E0CfhDidvid {
        E0CfhDidvid(3774945623u64 as u32)
    }
}
impl core::fmt::Debug for E0CfhDidvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0CfhDidvid")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0CfhDidvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0CfhDidvid {
            vendor_id: u16,
            device_id: u16,
        }
        let proxy = E0CfhDidvid {
            vendor_id: self.vendor_id(),
            device_id: self.device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 config header subsystem ID and subsystem vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0CfhSidsvid(pub u32);
impl E0CfhSidsvid {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[inline(always)]
    pub const fn set_subsystem_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[inline(always)]
    pub const fn set_subsystem_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for E0CfhSidsvid {
    #[inline(always)]
    fn default() -> E0CfhSidsvid {
        E0CfhSidsvid(3774945623u64 as u32)
    }
}
impl core::fmt::Debug for E0CfhSidsvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0CfhSidsvid")
            .field("subsystem_vendor_id", &self.subsystem_vendor_id())
            .field("subsystem_device_id", &self.subsystem_device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0CfhSidsvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0CfhSidsvid {
            subsystem_vendor_id: u16,
            subsystem_device_id: u16,
        }
        let proxy = E0CfhSidsvid {
            subsystem_vendor_id: self.subsystem_vendor_id(),
            subsystem_device_id: self.subsystem_device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 access management qualifier register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0amqr(pub u32);
impl E0amqr {
    #[doc = "Address Read QoS"]
    #[must_use]
    #[inline(always)]
    pub const fn arqos(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Read QoS"]
    #[inline(always)]
    pub const fn set_arqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Address Write QoS"]
    #[must_use]
    #[inline(always)]
    pub const fn awqos(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Write QoS"]
    #[inline(always)]
    pub const fn set_awqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[must_use]
    #[inline(always)]
    pub const fn bmt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[inline(always)]
    pub const fn set_bmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for E0amqr {
    #[inline(always)]
    fn default() -> E0amqr {
        E0amqr(301989888u64 as u32)
    }
}
impl core::fmt::Debug for E0amqr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0amqr")
            .field("arqos", &self.arqos())
            .field("awqos", &self.awqos())
            .field("bmt", &self.bmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0amqr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0amqr {
            arqos: u8,
            awqos: u8,
            bmt: bool,
        }
        let proxy = E0amqr {
            arqos: self.arqos(),
            awqos: self.awqos(),
            bmt: self.bmt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 buffer cache attribute register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0bcar(pub u32);
impl E0bcar {
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_bd_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_bd_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_bd_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes frame data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn wrcache(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes frame data to memory"]
    #[inline(always)]
    pub const fn set_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes frame data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn wrdomain(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes frame data to memory"]
    #[inline(always)]
    pub const fn set_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes frame data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn wrsnp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes frame data to memory"]
    #[inline(always)]
    pub const fn set_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_bd_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when ENETC reads the buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when ENETC reads the buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_bd_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Buffer descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[inline(always)]
    pub const fn set_bd_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads frame data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn rdcache(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads frame data from memory"]
    #[inline(always)]
    pub const fn set_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads frame data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn rddomain(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads frame data from memory"]
    #[inline(always)]
    pub const fn set_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads frame data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn rdsnp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads frame data from memory"]
    #[inline(always)]
    pub const fn set_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for E0bcar {
    #[inline(always)]
    fn default() -> E0bcar {
        E0bcar(33686018u64 as u32)
    }
}
impl core::fmt::Debug for E0bcar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0bcar")
            .field("bd_wrcache", &self.bd_wrcache())
            .field("bd_wrdomain", &self.bd_wrdomain())
            .field("bd_wrsnp", &self.bd_wrsnp())
            .field("wrcache", &self.wrcache())
            .field("wrdomain", &self.wrdomain())
            .field("wrsnp", &self.wrsnp())
            .field("bd_rdcache", &self.bd_rdcache())
            .field("bd_rddomain", &self.bd_rddomain())
            .field("bd_rdsnp", &self.bd_rdsnp())
            .field("rdcache", &self.rdcache())
            .field("rddomain", &self.rddomain())
            .field("rdsnp", &self.rdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0bcar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0bcar {
            bd_wrcache: u8,
            bd_wrdomain: u8,
            bd_wrsnp: bool,
            wrcache: u8,
            wrdomain: u8,
            wrsnp: bool,
            bd_rdcache: u8,
            bd_rddomain: u8,
            bd_rdsnp: bool,
            rdcache: u8,
            rddomain: u8,
            rdsnp: bool,
        }
        let proxy = E0bcar {
            bd_wrcache: self.bd_wrcache(),
            bd_wrdomain: self.bd_wrdomain(),
            bd_wrsnp: self.bd_wrsnp(),
            wrcache: self.wrcache(),
            wrdomain: self.wrdomain(),
            wrsnp: self.wrsnp(),
            bd_rdcache: self.bd_rdcache(),
            bd_rddomain: self.bd_rddomain(),
            bd_rdsnp: self.bd_rdsnp(),
            rdcache: self.rdcache(),
            rddomain: self.rddomain(),
            rdsnp: self.rdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 binding configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0bcr0(pub u32);
impl E0bcr0 {
    #[doc = "Root complex instance this function is bound to."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Root complex instance this function is bound to."]
    #[inline(always)]
    pub const fn set_rc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "If set, this ENETC instance is associated to a link end."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If set, this ENETC instance is associated to a link end."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for E0bcr0 {
    #[inline(always)]
    fn default() -> E0bcr0 {
        E0bcr0(2147484416u64 as u32)
    }
}
impl core::fmt::Debug for E0bcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0bcr0")
            .field("rc_inst", &self.rc_inst())
            .field("fn_", &self.fn_())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0bcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0bcr0 {
            rc_inst: u8,
            fn_: u8,
            valid: bool,
        }
        let proxy = E0bcr0 {
            rc_inst: self.rc_inst(),
            fn_: self.fn_(),
            valid: self.valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 binding configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0bcr1(pub u32);
impl E0bcr1 {
    #[doc = "Number of Rx BD rings supported by ENETC"]
    #[must_use]
    #[inline(always)]
    pub const fn num_rx_bdr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of Rx BD rings supported by ENETC"]
    #[inline(always)]
    pub const fn set_num_rx_bdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Number of Tx BD rings supported by ENETC"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tx_bdr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of Tx BD rings supported by ENETC"]
    #[inline(always)]
    pub const fn set_num_tx_bdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for E0bcr1 {
    #[inline(always)]
    fn default() -> E0bcr1 {
        E0bcr1(262148u64 as u32)
    }
}
impl core::fmt::Debug for E0bcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0bcr1")
            .field("num_rx_bdr", &self.num_rx_bdr())
            .field("num_tx_bdr", &self.num_tx_bdr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0bcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0bcr1 {
            num_rx_bdr: u16,
            num_tx_bdr: u16,
        }
        let proxy = E0bcr1 {
            num_rx_bdr: self.num_rx_bdr(),
            num_tx_bdr: self.num_tx_bdr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 binding configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0bcr2(pub u32);
impl E0bcr2 {
    #[doc = "Number of ENETC SI MAC address filter rules supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_mac_afte(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Number of ENETC SI MAC address filter rules supported"]
    #[inline(always)]
    pub const fn set_num_mac_afte(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Number of ENETC SI VLAN filter rules supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vlan_fte(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Number of ENETC SI VLAN filter rules supported"]
    #[inline(always)]
    pub const fn set_num_vlan_fte(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for E0bcr2 {
    #[inline(always)]
    fn default() -> E0bcr2 {
        E0bcr2(262148u64 as u32)
    }
}
impl core::fmt::Debug for E0bcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0bcr2")
            .field("num_mac_afte", &self.num_mac_afte())
            .field("num_vlan_fte", &self.num_vlan_fte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0bcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0bcr2 {
            num_mac_afte: u16,
            num_vlan_fte: u16,
        }
        let proxy = E0bcr2 {
            num_mac_afte: self.num_mac_afte(),
            num_vlan_fte: self.num_vlan_fte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 command cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0car(pub u32);
impl E0car {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrcache(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrdomain(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrsnp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the command buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the command buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_cbd_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when ENETC reads the command buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when ENETC reads the command buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_cbd_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[inline(always)]
    pub const fn set_cbd_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crdcache(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads command data from memory"]
    #[inline(always)]
    pub const fn set_crdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crddomain(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads command data from memory"]
    #[inline(always)]
    pub const fn set_crddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crdsnp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads command data from memory"]
    #[inline(always)]
    pub const fn set_crdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for E0car {
    #[inline(always)]
    fn default() -> E0car {
        E0car(33686018u64 as u32)
    }
}
impl core::fmt::Debug for E0car {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0car")
            .field("cbd_wrcache", &self.cbd_wrcache())
            .field("cbd_wrdomain", &self.cbd_wrdomain())
            .field("cbd_wrsnp", &self.cbd_wrsnp())
            .field("cwrcache", &self.cwrcache())
            .field("cwrdomain", &self.cwrdomain())
            .field("cwrsnp", &self.cwrsnp())
            .field("cbd_rdcache", &self.cbd_rdcache())
            .field("cbd_rddomain", &self.cbd_rddomain())
            .field("cbd_rdsnp", &self.cbd_rdsnp())
            .field("crdcache", &self.crdcache())
            .field("crddomain", &self.crddomain())
            .field("crdsnp", &self.crdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0car {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0car {
            cbd_wrcache: u8,
            cbd_wrdomain: u8,
            cbd_wrsnp: bool,
            cwrcache: u8,
            cwrdomain: u8,
            cwrsnp: bool,
            cbd_rdcache: u8,
            cbd_rddomain: u8,
            cbd_rdsnp: bool,
            crdcache: u8,
            crddomain: u8,
            crdsnp: bool,
        }
        let proxy = E0car {
            cbd_wrcache: self.cbd_wrcache(),
            cbd_wrdomain: self.cbd_wrdomain(),
            cbd_wrsnp: self.cbd_wrsnp(),
            cwrcache: self.cwrcache(),
            cwrdomain: self.cwrdomain(),
            cwrsnp: self.cwrsnp(),
            cbd_rdcache: self.cbd_rdcache(),
            cbd_rddomain: self.cbd_rddomain(),
            cbd_rdsnp: self.cbd_rdsnp(),
            crdcache: self.crdcache(),
            crddomain: self.crddomain(),
            crdsnp: self.crdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 hash table memory allotment register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0htmar(pub u32);
impl E0htmar {
    #[doc = "Number of words allotted to exact match hash table from the common memory's shared region"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of words allotted to exact match hash table from the common memory's shared region"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for E0htmar {
    #[inline(always)]
    fn default() -> E0htmar {
        E0htmar(8u64 as u32)
    }
}
impl core::fmt::Debug for E0htmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0htmar")
            .field("num_words", &self.num_words())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0htmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0htmar {
            num_words: u16,
            mloc: u8,
        }
        let proxy = E0htmar {
            num_words: self.num_words(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 ingress port filter table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0ipftmar(pub u32);
impl E0ipftmar {
    #[doc = "Number of words allocated to the ENETC Ingress Port Filter table from ingress port filter ternary memory"]
    #[must_use]
    #[inline(always)]
    pub const fn alloc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of words allocated to the ENETC Ingress Port Filter table from ingress port filter ternary memory"]
    #[inline(always)]
    pub const fn set_alloc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E0ipftmar {
    #[inline(always)]
    fn default() -> E0ipftmar {
        E0ipftmar(28u64 as u32)
    }
}
impl core::fmt::Debug for E0ipftmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0ipftmar")
            .field("alloc", &self.alloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0ipftmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0ipftmar {
            alloc: u16,
        }
        let proxy = E0ipftmar {
            alloc: self.alloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 ingress stream counter index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0iscitmar(pub u32);
impl E0iscitmar {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E0iscitmar {
    #[inline(always)]
    fn default() -> E0iscitmar {
        E0iscitmar(8u64 as u32)
    }
}
impl core::fmt::Debug for E0iscitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0iscitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0iscitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0iscitmar {
            num_words: u16,
        }
        let proxy = E0iscitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 ingress stream index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0isitmar(pub u32);
impl E0isitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E0isitmar {
    #[inline(always)]
    fn default() -> E0isitmar {
        E0isitmar(8u64 as u32)
    }
}
impl core::fmt::Debug for E0isitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0isitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0isitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0isitmar {
            num_words: u16,
        }
        let proxy = E0isitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0itmar(pub u32);
impl E0itmar {
    #[doc = "Number of WORDS allocated to ENETC's index table memory"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of WORDS allocated to ENETC's index table memory"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for E0itmar {
    #[inline(always)]
    fn default() -> E0itmar {
        E0itmar(80u64 as u32)
    }
}
impl core::fmt::Debug for E0itmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0itmar")
            .field("num_words", &self.num_words())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0itmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0itmar {
            num_words: u16,
            mloc: u8,
        }
        let proxy = E0itmar {
            num_words: self.num_words(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 message cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0mcar(pub u32);
impl E0mcar {
    #[doc = "SI messaging write cache type This is the cache attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "SI messaging write cache type This is the cache attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[inline(always)]
    pub const fn set_msg_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "SI messaging write domain This is the domain attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "SI messaging write domain This is the domain attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[inline(always)]
    pub const fn set_msg_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "SI messaging write snoop This is the snoop attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SI messaging write snoop This is the snoop attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[inline(always)]
    pub const fn set_msg_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SI messaging read data cache type This is the cache attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "SI messaging read data cache type This is the cache attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[inline(always)]
    pub const fn set_msg_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "SI messaging read data domain This is the domain attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "SI messaging read data domain This is the domain attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[inline(always)]
    pub const fn set_msg_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "SI messaging read data snoop This is the snoop attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SI messaging read data snoop This is the snoop attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[inline(always)]
    pub const fn set_msg_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for E0mcar {
    #[inline(always)]
    fn default() -> E0mcar {
        E0mcar(131074u64 as u32)
    }
}
impl core::fmt::Debug for E0mcar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0mcar")
            .field("msg_wrcache", &self.msg_wrcache())
            .field("msg_wrdomain", &self.msg_wrdomain())
            .field("msg_wrsnp", &self.msg_wrsnp())
            .field("msg_rdcache", &self.msg_rdcache())
            .field("msg_rddomain", &self.msg_rddomain())
            .field("msg_rdsnp", &self.msg_rdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0mcar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0mcar {
            msg_wrcache: u8,
            msg_wrdomain: u8,
            msg_wrsnp: bool,
            msg_rdcache: u8,
            msg_rddomain: u8,
            msg_rdsnp: bool,
        }
        let proxy = E0mcar {
            msg_wrcache: self.msg_wrcache(),
            msg_wrdomain: self.msg_wrdomain(),
            msg_wrsnp: self.msg_wrsnp(),
            msg_rdcache: self.msg_rdcache(),
            msg_rddomain: self.msg_rddomain(),
            msg_rdsnp: self.msg_rdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 MSI-X configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0mcr(pub u32);
impl E0mcr {
    #[doc = "Number of MSI-X vectors supported for ENETC function"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Number of MSI-X vectors supported for ENETC function"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for E0mcr {
    #[inline(always)]
    fn default() -> E0mcr {
        E0mcr(11u64 as u32)
    }
}
impl core::fmt::Debug for E0mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0mcr")
            .field("num_msix", &self.num_msix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0mcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0mcr {
            num_msix: u16,
        }
        let proxy = E0mcr {
            num_msix: self.num_msix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 rate policer index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0rpitmar(pub u32);
impl E0rpitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E0rpitmar {
    #[inline(always)]
    fn default() -> E0rpitmar {
        E0rpitmar(8u64 as u32)
    }
}
impl core::fmt::Debug for E0rpitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0rpitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0rpitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0rpitmar {
            num_words: u16,
        }
        let proxy = E0rpitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 receive memory buffer entitlement register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0rxmber(pub u32);
impl E0rxmber {
    #[doc = "Receive memory buffer entitlement in words This receive Memory Buffer Entitlement is used in determining smart drop for ingress congestion control"]
    #[must_use]
    #[inline(always)]
    pub const fn amount(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Receive memory buffer entitlement in words This receive Memory Buffer Entitlement is used in determining smart drop for ingress congestion control"]
    #[inline(always)]
    pub const fn set_amount(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for E0rxmber {
    #[inline(always)]
    fn default() -> E0rxmber {
        E0rxmber(0u64 as u32)
    }
}
impl core::fmt::Debug for E0rxmber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0rxmber")
            .field("amount", &self.amount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0rxmber {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0rxmber {
            amount: u32,
        }
        let proxy = E0rxmber {
            amount: self.amount(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 receive memory buffer limit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0rxmblr(pub u32);
impl E0rxmblr {
    #[doc = "Receive buffer memory limit in words"]
    #[must_use]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Receive buffer memory limit in words"]
    #[inline(always)]
    pub const fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for E0rxmblr {
    #[inline(always)]
    fn default() -> E0rxmblr {
        E0rxmblr(0u64 as u32)
    }
}
impl core::fmt::Debug for E0rxmblr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0rxmblr")
            .field("limit", &self.limit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0rxmblr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0rxmblr {
            limit: u32,
        }
        let proxy = E0rxmblr {
            limit: self.limit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 stream gate control list index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0sgclitmar(pub u32);
impl E0sgclitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E0sgclitmar {
    #[inline(always)]
    fn default() -> E0sgclitmar {
        E0sgclitmar(48u64 as u32)
    }
}
impl core::fmt::Debug for E0sgclitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0sgclitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0sgclitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0sgclitmar {
            num_words: u16,
        }
        let proxy = E0sgclitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 stream gate instance index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0sgiitmar(pub u32);
impl E0sgiitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E0sgiitmar {
    #[inline(always)]
    fn default() -> E0sgiitmar {
        E0sgiitmar(8u64 as u32)
    }
}
impl core::fmt::Debug for E0sgiitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0sgiitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0sgiitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0sgiitmar {
            num_words: u16,
        }
        let proxy = E0sgiitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 time gate scheduling lookahead register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0tgslr(pub u32);
impl E0tgslr {
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler (at the frame scheduling timing point), to account for the time required to schedule, dequeue and load (or DMA) frames from the host memory to NETC internal memory"]
    #[must_use]
    #[inline(always)]
    pub const fn min_lookahead(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler (at the frame scheduling timing point), to account for the time required to schedule, dequeue and load (or DMA) frames from the host memory to NETC internal memory"]
    #[inline(always)]
    pub const fn set_min_lookahead(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Zero Lookahead"]
    #[must_use]
    #[inline(always)]
    pub const fn zero_lookahead(&self) -> super::vals::E0tgslrZeroLookahead {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::E0tgslrZeroLookahead::from_bits(val as u8)
    }
    #[doc = "Zero Lookahead"]
    #[inline(always)]
    pub const fn set_zero_lookahead(&mut self, val: super::vals::E0tgslrZeroLookahead) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for E0tgslr {
    #[inline(always)]
    fn default() -> E0tgslr {
        E0tgslr(10000u64 as u32)
    }
}
impl core::fmt::Debug for E0tgslr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0tgslr")
            .field("min_lookahead", &self.min_lookahead())
            .field("zero_lookahead", &self.zero_lookahead())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0tgslr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0tgslr {
            min_lookahead: u32,
            zero_lookahead: super::vals::E0tgslrZeroLookahead,
        }
        let proxy = E0tgslr {
            min_lookahead: self.min_lookahead(),
            zero_lookahead: self.zero_lookahead(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 time gate scheduling table allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0tgstar(pub u32);
impl E0tgstar {
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the ENETC instance a Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list for the ENETC instance"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the ENETC instance a Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list for the ENETC instance"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for E0tgstar {
    #[inline(always)]
    fn default() -> E0tgstar {
        E0tgstar(256u64 as u32)
    }
}
impl core::fmt::Debug for E0tgstar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0tgstar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0tgstar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0tgstar {
            num_words: u16,
        }
        let proxy = E0tgstar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 transmit high priority tier byte credit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0txhptbcr(pub u32);
impl E0txhptbcr {
    #[doc = "This register field is used to configure the maximum number of high priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_credit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register field is used to configure the maximum number of high priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    #[inline(always)]
    pub const fn set_byte_credit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E0txhptbcr {
    #[inline(always)]
    fn default() -> E0txhptbcr {
        E0txhptbcr(1500u64 as u32)
    }
}
impl core::fmt::Debug for E0txhptbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0txhptbcr")
            .field("byte_credit", &self.byte_credit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0txhptbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0txhptbcr {
            byte_credit: u16,
        }
        let proxy = E0txhptbcr {
            byte_credit: self.byte_credit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 transmit low priority tier byte credit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0txlptbcr(pub u32);
impl E0txlptbcr {
    #[doc = "This register field is used to configure the maximum number of low priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_credit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register field is used to configure the maximum number of low priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    #[inline(always)]
    pub const fn set_byte_credit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E0txlptbcr {
    #[inline(always)]
    fn default() -> E0txlptbcr {
        E0txlptbcr(1500u64 as u32)
    }
}
impl core::fmt::Debug for E0txlptbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0txlptbcr")
            .field("byte_credit", &self.byte_credit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0txlptbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0txlptbcr {
            byte_credit: u16,
        }
        let proxy = E0txlptbcr {
            byte_credit: self.byte_credit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 0 VSI binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E0vbcr(pub u32);
impl E0vbcr {
    #[doc = "Indicates the number of VSIs supported for this ENETC instance"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vsi(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the number of VSIs supported for this ENETC instance"]
    #[inline(always)]
    pub const fn set_num_vsi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for E0vbcr {
    #[inline(always)]
    fn default() -> E0vbcr {
        E0vbcr(0u64 as u32)
    }
}
impl core::fmt::Debug for E0vbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E0vbcr")
            .field("num_vsi", &self.num_vsi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E0vbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E0vbcr {
            num_vsi: u8,
        }
        let proxy = E0vbcr {
            num_vsi: self.num_vsi(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 config capability VF device ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1CfcVfdid(pub u32);
impl E1CfcVfdid {
    #[doc = "VF Device ID This field identifies the device ID for a virtual function as shown in the PCIe SR-IOV Capability Structure (20h)"]
    #[must_use]
    #[inline(always)]
    pub const fn vf_device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "VF Device ID This field identifies the device ID for a virtual function as shown in the PCIe SR-IOV Capability Structure (20h)"]
    #[inline(always)]
    pub const fn set_vf_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for E1CfcVfdid {
    #[inline(always)]
    fn default() -> E1CfcVfdid {
        E1CfcVfdid(4009754624u64 as u32)
    }
}
impl core::fmt::Debug for E1CfcVfdid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1CfcVfdid")
            .field("vf_device_id", &self.vf_device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1CfcVfdid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1CfcVfdid {
            vf_device_id: u16,
        }
        let proxy = E1CfcVfdid {
            vf_device_id: self.vf_device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 config header device ID and vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1CfhDidvid(pub u32);
impl E1CfhDidvid {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[inline(always)]
    pub const fn set_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[inline(always)]
    pub const fn set_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for E1CfhDidvid {
    #[inline(always)]
    fn default() -> E1CfhDidvid {
        E1CfhDidvid(3775928663u64 as u32)
    }
}
impl core::fmt::Debug for E1CfhDidvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1CfhDidvid")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1CfhDidvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1CfhDidvid {
            vendor_id: u16,
            device_id: u16,
        }
        let proxy = E1CfhDidvid {
            vendor_id: self.vendor_id(),
            device_id: self.device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 config header subsystem ID and subsystem vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1CfhSidsvid(pub u32);
impl E1CfhSidsvid {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[inline(always)]
    pub const fn set_subsystem_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[inline(always)]
    pub const fn set_subsystem_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for E1CfhSidsvid {
    #[inline(always)]
    fn default() -> E1CfhSidsvid {
        E1CfhSidsvid(3775928663u64 as u32)
    }
}
impl core::fmt::Debug for E1CfhSidsvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1CfhSidsvid")
            .field("subsystem_vendor_id", &self.subsystem_vendor_id())
            .field("subsystem_device_id", &self.subsystem_device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1CfhSidsvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1CfhSidsvid {
            subsystem_vendor_id: u16,
            subsystem_device_id: u16,
        }
        let proxy = E1CfhSidsvid {
            subsystem_vendor_id: self.subsystem_vendor_id(),
            subsystem_device_id: self.subsystem_device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 access management qualifier register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1amqr(pub u32);
impl E1amqr {
    #[doc = "Address Read QoS"]
    #[must_use]
    #[inline(always)]
    pub const fn arqos(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Read QoS"]
    #[inline(always)]
    pub const fn set_arqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Address Write QoS"]
    #[must_use]
    #[inline(always)]
    pub const fn awqos(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Write QoS"]
    #[inline(always)]
    pub const fn set_awqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[must_use]
    #[inline(always)]
    pub const fn bmt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[inline(always)]
    pub const fn set_bmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for E1amqr {
    #[inline(always)]
    fn default() -> E1amqr {
        E1amqr(301989888u64 as u32)
    }
}
impl core::fmt::Debug for E1amqr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1amqr")
            .field("arqos", &self.arqos())
            .field("awqos", &self.awqos())
            .field("bmt", &self.bmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1amqr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1amqr {
            arqos: u8,
            awqos: u8,
            bmt: bool,
        }
        let proxy = E1amqr {
            arqos: self.arqos(),
            awqos: self.awqos(),
            bmt: self.bmt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 buffer cache attribute register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1bcar(pub u32);
impl E1bcar {
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_bd_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Buffer descriptor write domain This is the domain attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_bd_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_bd_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes frame data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn wrcache(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes frame data to memory"]
    #[inline(always)]
    pub const fn set_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes frame data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn wrdomain(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes frame data to memory"]
    #[inline(always)]
    pub const fn set_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes frame data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn wrsnp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes frame data to memory"]
    #[inline(always)]
    pub const fn set_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_bd_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when ENETC reads the buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Buffer descriptor read domain This is the domain attribute setting used when ENETC reads the buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_bd_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Buffer descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[must_use]
    #[inline(always)]
    pub const fn bd_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Buffer descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[inline(always)]
    pub const fn set_bd_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads frame data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn rdcache(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads frame data from memory"]
    #[inline(always)]
    pub const fn set_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads frame data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn rddomain(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads frame data from memory"]
    #[inline(always)]
    pub const fn set_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads frame data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn rdsnp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads frame data from memory"]
    #[inline(always)]
    pub const fn set_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for E1bcar {
    #[inline(always)]
    fn default() -> E1bcar {
        E1bcar(33686018u64 as u32)
    }
}
impl core::fmt::Debug for E1bcar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1bcar")
            .field("bd_wrcache", &self.bd_wrcache())
            .field("bd_wrdomain", &self.bd_wrdomain())
            .field("bd_wrsnp", &self.bd_wrsnp())
            .field("wrcache", &self.wrcache())
            .field("wrdomain", &self.wrdomain())
            .field("wrsnp", &self.wrsnp())
            .field("bd_rdcache", &self.bd_rdcache())
            .field("bd_rddomain", &self.bd_rddomain())
            .field("bd_rdsnp", &self.bd_rdsnp())
            .field("rdcache", &self.rdcache())
            .field("rddomain", &self.rddomain())
            .field("rdsnp", &self.rdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1bcar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1bcar {
            bd_wrcache: u8,
            bd_wrdomain: u8,
            bd_wrsnp: bool,
            wrcache: u8,
            wrdomain: u8,
            wrsnp: bool,
            bd_rdcache: u8,
            bd_rddomain: u8,
            bd_rdsnp: bool,
            rdcache: u8,
            rddomain: u8,
            rdsnp: bool,
        }
        let proxy = E1bcar {
            bd_wrcache: self.bd_wrcache(),
            bd_wrdomain: self.bd_wrdomain(),
            bd_wrsnp: self.bd_wrsnp(),
            wrcache: self.wrcache(),
            wrdomain: self.wrdomain(),
            wrsnp: self.wrsnp(),
            bd_rdcache: self.bd_rdcache(),
            bd_rddomain: self.bd_rddomain(),
            bd_rdsnp: self.bd_rdsnp(),
            rdcache: self.rdcache(),
            rddomain: self.rddomain(),
            rdsnp: self.rdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 binding configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1bcr0(pub u32);
impl E1bcr0 {
    #[doc = "Root complex instance this function is bound to."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Root complex instance this function is bound to."]
    #[inline(always)]
    pub const fn set_rc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "If set, this ENETC instance is associated to a link end."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If set, this ENETC instance is associated to a link end."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for E1bcr0 {
    #[inline(always)]
    fn default() -> E1bcr0 {
        E1bcr0(2147484672u64 as u32)
    }
}
impl core::fmt::Debug for E1bcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1bcr0")
            .field("rc_inst", &self.rc_inst())
            .field("fn_", &self.fn_())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1bcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1bcr0 {
            rc_inst: u8,
            fn_: u8,
            valid: bool,
        }
        let proxy = E1bcr0 {
            rc_inst: self.rc_inst(),
            fn_: self.fn_(),
            valid: self.valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 binding configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1bcr1(pub u32);
impl E1bcr1 {
    #[doc = "Number of Rx BD rings supported by ENETC"]
    #[must_use]
    #[inline(always)]
    pub const fn num_rx_bdr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of Rx BD rings supported by ENETC"]
    #[inline(always)]
    pub const fn set_num_rx_bdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Number of Tx BD rings supported by ENETC"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tx_bdr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of Tx BD rings supported by ENETC"]
    #[inline(always)]
    pub const fn set_num_tx_bdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for E1bcr1 {
    #[inline(always)]
    fn default() -> E1bcr1 {
        E1bcr1(655370u64 as u32)
    }
}
impl core::fmt::Debug for E1bcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1bcr1")
            .field("num_rx_bdr", &self.num_rx_bdr())
            .field("num_tx_bdr", &self.num_tx_bdr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1bcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1bcr1 {
            num_rx_bdr: u16,
            num_tx_bdr: u16,
        }
        let proxy = E1bcr1 {
            num_rx_bdr: self.num_rx_bdr(),
            num_tx_bdr: self.num_tx_bdr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 binding configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1bcr2(pub u32);
impl E1bcr2 {
    #[doc = "Number of ENETC SI MAC address filter rules supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_mac_afte(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Number of ENETC SI MAC address filter rules supported"]
    #[inline(always)]
    pub const fn set_num_mac_afte(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Number of ENETC SI VLAN filter rules supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vlan_fte(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Number of ENETC SI VLAN filter rules supported"]
    #[inline(always)]
    pub const fn set_num_vlan_fte(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for E1bcr2 {
    #[inline(always)]
    fn default() -> E1bcr2 {
        E1bcr2(262148u64 as u32)
    }
}
impl core::fmt::Debug for E1bcr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1bcr2")
            .field("num_mac_afte", &self.num_mac_afte())
            .field("num_vlan_fte", &self.num_vlan_fte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1bcr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1bcr2 {
            num_mac_afte: u16,
            num_vlan_fte: u16,
        }
        let proxy = E1bcr2 {
            num_mac_afte: self.num_mac_afte(),
            num_vlan_fte: self.num_vlan_fte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 command cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1car(pub u32);
impl E1car {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when ENETC writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrcache(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write data cache type This is the cache attribute setting used when ENETC writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrdomain(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Write data domain This is the domain attribute setting used when ENETC writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrsnp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when ENETC writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the command buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when ENETC reads the command buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_cbd_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when ENETC reads the command buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when ENETC reads the command buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_cbd_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[inline(always)]
    pub const fn set_cbd_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crdcache(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Read data cache type This is the cache attribute setting used when ENETC reads command data from memory"]
    #[inline(always)]
    pub const fn set_crdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crddomain(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Read data domain This is the domain attribute setting used when ENETC reads command data from memory"]
    #[inline(always)]
    pub const fn set_crddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crdsnp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when ENETC reads command data from memory"]
    #[inline(always)]
    pub const fn set_crdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for E1car {
    #[inline(always)]
    fn default() -> E1car {
        E1car(33686018u64 as u32)
    }
}
impl core::fmt::Debug for E1car {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1car")
            .field("cbd_wrcache", &self.cbd_wrcache())
            .field("cbd_wrdomain", &self.cbd_wrdomain())
            .field("cbd_wrsnp", &self.cbd_wrsnp())
            .field("cwrcache", &self.cwrcache())
            .field("cwrdomain", &self.cwrdomain())
            .field("cwrsnp", &self.cwrsnp())
            .field("cbd_rdcache", &self.cbd_rdcache())
            .field("cbd_rddomain", &self.cbd_rddomain())
            .field("cbd_rdsnp", &self.cbd_rdsnp())
            .field("crdcache", &self.crdcache())
            .field("crddomain", &self.crddomain())
            .field("crdsnp", &self.crdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1car {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1car {
            cbd_wrcache: u8,
            cbd_wrdomain: u8,
            cbd_wrsnp: bool,
            cwrcache: u8,
            cwrdomain: u8,
            cwrsnp: bool,
            cbd_rdcache: u8,
            cbd_rddomain: u8,
            cbd_rdsnp: bool,
            crdcache: u8,
            crddomain: u8,
            crdsnp: bool,
        }
        let proxy = E1car {
            cbd_wrcache: self.cbd_wrcache(),
            cbd_wrdomain: self.cbd_wrdomain(),
            cbd_wrsnp: self.cbd_wrsnp(),
            cwrcache: self.cwrcache(),
            cwrdomain: self.cwrdomain(),
            cwrsnp: self.cwrsnp(),
            cbd_rdcache: self.cbd_rdcache(),
            cbd_rddomain: self.cbd_rddomain(),
            cbd_rdsnp: self.cbd_rdsnp(),
            crdcache: self.crdcache(),
            crddomain: self.crddomain(),
            crdsnp: self.crdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 hash table memory allotment register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1htmar(pub u32);
impl E1htmar {
    #[doc = "Number of words allotted to exact match hash table from the common memory's shared region"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of words allotted to exact match hash table from the common memory's shared region"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for E1htmar {
    #[inline(always)]
    fn default() -> E1htmar {
        E1htmar(16u64 as u32)
    }
}
impl core::fmt::Debug for E1htmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1htmar")
            .field("num_words", &self.num_words())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1htmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1htmar {
            num_words: u16,
            mloc: u8,
        }
        let proxy = E1htmar {
            num_words: self.num_words(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 ingress port filter table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1ipftmar(pub u32);
impl E1ipftmar {
    #[doc = "Number of words allocated to the ENETC Ingress Port Filter table from ingress port filter ternary memory"]
    #[must_use]
    #[inline(always)]
    pub const fn alloc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of words allocated to the ENETC Ingress Port Filter table from ingress port filter ternary memory"]
    #[inline(always)]
    pub const fn set_alloc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E1ipftmar {
    #[inline(always)]
    fn default() -> E1ipftmar {
        E1ipftmar(28u64 as u32)
    }
}
impl core::fmt::Debug for E1ipftmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1ipftmar")
            .field("alloc", &self.alloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1ipftmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1ipftmar {
            alloc: u16,
        }
        let proxy = E1ipftmar {
            alloc: self.alloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 ingress stream counter index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1iscitmar(pub u32);
impl E1iscitmar {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E1iscitmar {
    #[inline(always)]
    fn default() -> E1iscitmar {
        E1iscitmar(16u64 as u32)
    }
}
impl core::fmt::Debug for E1iscitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1iscitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1iscitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1iscitmar {
            num_words: u16,
        }
        let proxy = E1iscitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 ingress stream index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1isitmar(pub u32);
impl E1isitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E1isitmar {
    #[inline(always)]
    fn default() -> E1isitmar {
        E1isitmar(16u64 as u32)
    }
}
impl core::fmt::Debug for E1isitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1isitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1isitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1isitmar {
            num_words: u16,
        }
        let proxy = E1isitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1itmar(pub u32);
impl E1itmar {
    #[doc = "Number of WORDS allocated to ENETC's index table memory"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of WORDS allocated to ENETC's index table memory"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for E1itmar {
    #[inline(always)]
    fn default() -> E1itmar {
        E1itmar(48u64 as u32)
    }
}
impl core::fmt::Debug for E1itmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1itmar")
            .field("num_words", &self.num_words())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1itmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1itmar {
            num_words: u16,
            mloc: u8,
        }
        let proxy = E1itmar {
            num_words: self.num_words(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 message cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1mcar(pub u32);
impl E1mcar {
    #[doc = "SI messaging write cache type This is the cache attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "SI messaging write cache type This is the cache attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[inline(always)]
    pub const fn set_msg_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "SI messaging write domain This is the domain attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "SI messaging write domain This is the domain attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[inline(always)]
    pub const fn set_msg_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "SI messaging write snoop This is the snoop attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SI messaging write snoop This is the snoop attribute setting used when ENETC writes PSI-VSI message during partition copy"]
    #[inline(always)]
    pub const fn set_msg_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SI messaging read data cache type This is the cache attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "SI messaging read data cache type This is the cache attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[inline(always)]
    pub const fn set_msg_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "SI messaging read data domain This is the domain attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "SI messaging read data domain This is the domain attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[inline(always)]
    pub const fn set_msg_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "SI messaging read data snoop This is the snoop attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[must_use]
    #[inline(always)]
    pub const fn msg_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "SI messaging read data snoop This is the snoop attribute setting used when ENETC reads PSI-VSI message from memory during partition copy"]
    #[inline(always)]
    pub const fn set_msg_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for E1mcar {
    #[inline(always)]
    fn default() -> E1mcar {
        E1mcar(131074u64 as u32)
    }
}
impl core::fmt::Debug for E1mcar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1mcar")
            .field("msg_wrcache", &self.msg_wrcache())
            .field("msg_wrdomain", &self.msg_wrdomain())
            .field("msg_wrsnp", &self.msg_wrsnp())
            .field("msg_rdcache", &self.msg_rdcache())
            .field("msg_rddomain", &self.msg_rddomain())
            .field("msg_rdsnp", &self.msg_rdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1mcar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1mcar {
            msg_wrcache: u8,
            msg_wrdomain: u8,
            msg_wrsnp: bool,
            msg_rdcache: u8,
            msg_rddomain: u8,
            msg_rdsnp: bool,
        }
        let proxy = E1mcar {
            msg_wrcache: self.msg_wrcache(),
            msg_wrdomain: self.msg_wrdomain(),
            msg_wrsnp: self.msg_wrsnp(),
            msg_rdcache: self.msg_rdcache(),
            msg_rddomain: self.msg_rddomain(),
            msg_rdsnp: self.msg_rdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 MSI-X configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1mcr(pub u32);
impl E1mcr {
    #[doc = "Number of MSI-X vectors supported for ENETC function"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Number of MSI-X vectors supported for ENETC function"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for E1mcr {
    #[inline(always)]
    fn default() -> E1mcr {
        E1mcr(24u64 as u32)
    }
}
impl core::fmt::Debug for E1mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1mcr")
            .field("num_msix", &self.num_msix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1mcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1mcr {
            num_msix: u16,
        }
        let proxy = E1mcr {
            num_msix: self.num_msix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 rate policer index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1rpitmar(pub u32);
impl E1rpitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E1rpitmar {
    #[inline(always)]
    fn default() -> E1rpitmar {
        E1rpitmar(16u64 as u32)
    }
}
impl core::fmt::Debug for E1rpitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1rpitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1rpitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1rpitmar {
            num_words: u16,
        }
        let proxy = E1rpitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 receive memory buffer entitlement register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1rxmber(pub u32);
impl E1rxmber {
    #[doc = "Receive memory buffer entitlement in words This receive Memory Buffer Entitlement is used in determining smart drop for ingress congestion control"]
    #[must_use]
    #[inline(always)]
    pub const fn amount(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Receive memory buffer entitlement in words This receive Memory Buffer Entitlement is used in determining smart drop for ingress congestion control"]
    #[inline(always)]
    pub const fn set_amount(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for E1rxmber {
    #[inline(always)]
    fn default() -> E1rxmber {
        E1rxmber(0u64 as u32)
    }
}
impl core::fmt::Debug for E1rxmber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1rxmber")
            .field("amount", &self.amount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1rxmber {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1rxmber {
            amount: u32,
        }
        let proxy = E1rxmber {
            amount: self.amount(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 receive memory buffer limit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1rxmblr(pub u32);
impl E1rxmblr {
    #[doc = "Receive buffer memory limit in words"]
    #[must_use]
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Receive buffer memory limit in words"]
    #[inline(always)]
    pub const fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for E1rxmblr {
    #[inline(always)]
    fn default() -> E1rxmblr {
        E1rxmblr(0u64 as u32)
    }
}
impl core::fmt::Debug for E1rxmblr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1rxmblr")
            .field("limit", &self.limit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1rxmblr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1rxmblr {
            limit: u32,
        }
        let proxy = E1rxmblr {
            limit: self.limit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 stream gate control list index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1sgclitmar(pub u32);
impl E1sgclitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E1sgclitmar {
    #[inline(always)]
    fn default() -> E1sgclitmar {
        E1sgclitmar(0u64 as u32)
    }
}
impl core::fmt::Debug for E1sgclitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1sgclitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1sgclitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1sgclitmar {
            num_words: u16,
        }
        let proxy = E1sgclitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 stream gate instance index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1sgiitmar(pub u32);
impl E1sgiitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E1sgiitmar {
    #[inline(always)]
    fn default() -> E1sgiitmar {
        E1sgiitmar(0u64 as u32)
    }
}
impl core::fmt::Debug for E1sgiitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1sgiitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1sgiitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1sgiitmar {
            num_words: u16,
        }
        let proxy = E1sgiitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 time gate scheduling lookahead register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1tgslr(pub u32);
impl E1tgslr {
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler (at the frame scheduling timing point), to account for the time required to schedule, dequeue and load (or DMA) frames from the host memory to NETC internal memory"]
    #[must_use]
    #[inline(always)]
    pub const fn min_lookahead(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler (at the frame scheduling timing point), to account for the time required to schedule, dequeue and load (or DMA) frames from the host memory to NETC internal memory"]
    #[inline(always)]
    pub const fn set_min_lookahead(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Zero Lookahead"]
    #[must_use]
    #[inline(always)]
    pub const fn zero_lookahead(&self) -> super::vals::E1tgslrZeroLookahead {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::E1tgslrZeroLookahead::from_bits(val as u8)
    }
    #[doc = "Zero Lookahead"]
    #[inline(always)]
    pub const fn set_zero_lookahead(&mut self, val: super::vals::E1tgslrZeroLookahead) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for E1tgslr {
    #[inline(always)]
    fn default() -> E1tgslr {
        E1tgslr(2147493648u64 as u32)
    }
}
impl core::fmt::Debug for E1tgslr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1tgslr")
            .field("min_lookahead", &self.min_lookahead())
            .field("zero_lookahead", &self.zero_lookahead())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1tgslr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1tgslr {
            min_lookahead: u32,
            zero_lookahead: super::vals::E1tgslrZeroLookahead,
        }
        let proxy = E1tgslr {
            min_lookahead: self.min_lookahead(),
            zero_lookahead: self.zero_lookahead(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 time gate scheduling table allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1tgstar(pub u32);
impl E1tgstar {
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the ENETC instance a Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list for the ENETC instance"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the ENETC instance a Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list for the ENETC instance"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for E1tgstar {
    #[inline(always)]
    fn default() -> E1tgstar {
        E1tgstar(256u64 as u32)
    }
}
impl core::fmt::Debug for E1tgstar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1tgstar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1tgstar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1tgstar {
            num_words: u16,
        }
        let proxy = E1tgstar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 transmit high priority tier byte credit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1txhptbcr(pub u32);
impl E1txhptbcr {
    #[doc = "This register field is used to configure the maximum number of high priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_credit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register field is used to configure the maximum number of high priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    #[inline(always)]
    pub const fn set_byte_credit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E1txhptbcr {
    #[inline(always)]
    fn default() -> E1txhptbcr {
        E1txhptbcr(1500u64 as u32)
    }
}
impl core::fmt::Debug for E1txhptbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1txhptbcr")
            .field("byte_credit", &self.byte_credit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1txhptbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1txhptbcr {
            byte_credit: u16,
        }
        let proxy = E1txhptbcr {
            byte_credit: self.byte_credit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 transmit low priority tier byte credit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1txlptbcr(pub u32);
impl E1txlptbcr {
    #[doc = "This register field is used to configure the maximum number of low priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_credit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This register field is used to configure the maximum number of low priority byte credits for the port-per-HTA Transmit priority (2 priorities) byte credit-based flow control mechanism"]
    #[inline(always)]
    pub const fn set_byte_credit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for E1txlptbcr {
    #[inline(always)]
    fn default() -> E1txlptbcr {
        E1txlptbcr(1500u64 as u32)
    }
}
impl core::fmt::Debug for E1txlptbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1txlptbcr")
            .field("byte_credit", &self.byte_credit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1txlptbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1txlptbcr {
            byte_credit: u16,
        }
        let proxy = E1txlptbcr {
            byte_credit: self.byte_credit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC 1 VSI binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct E1vbcr(pub u32);
impl E1vbcr {
    #[doc = "Indicates the number of VSIs supported for this ENETC instance"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vsi(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the number of VSIs supported for this ENETC instance"]
    #[inline(always)]
    pub const fn set_num_vsi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for E1vbcr {
    #[inline(always)]
    fn default() -> E1vbcr {
        E1vbcr(1u64 as u32)
    }
}
impl core::fmt::Debug for E1vbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("E1vbcr")
            .field("num_vsi", &self.num_vsi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for E1vbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct E1vbcr {
            num_vsi: u8,
        }
        let proxy = E1vbcr {
            num_vsi: self.num_vsi(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EMDIO configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioCfg(pub u32);
impl EmdioCfg {
    #[doc = "MDIO pin mode 0 Push-pull 1 Open drain"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO pin mode 0 Push-pull 1 Open drain"]
    #[inline(always)]
    pub const fn set_mdio_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "MDC pin mode 0 Push-pull 1 Open drain"]
    #[must_use]
    #[inline(always)]
    pub const fn mdc_mode(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MDC pin mode 0 Push-pull 1 Open drain"]
    #[inline(always)]
    pub const fn set_mdc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for EmdioCfg {
    #[inline(always)]
    fn default() -> EmdioCfg {
        EmdioCfg(16u64 as u32)
    }
}
impl core::fmt::Debug for EmdioCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmdioCfg")
            .field("mdio_mode", &self.mdio_mode())
            .field("mdc_mode", &self.mdc_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmdioCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmdioCfg {
            mdio_mode: bool,
            mdc_mode: bool,
        }
        let proxy = EmdioCfg {
            mdio_mode: self.mdio_mode(),
            mdc_mode: self.mdc_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EMDIO config header device ID and vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioCfhDidvid(pub u32);
impl EmdioCfhDidvid {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[inline(always)]
    pub const fn set_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[inline(always)]
    pub const fn set_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EmdioCfhDidvid {
    #[inline(always)]
    fn default() -> EmdioCfhDidvid {
        EmdioCfhDidvid(3992983895u64 as u32)
    }
}
impl core::fmt::Debug for EmdioCfhDidvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmdioCfhDidvid")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmdioCfhDidvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmdioCfhDidvid {
            vendor_id: u16,
            device_id: u16,
        }
        let proxy = EmdioCfhDidvid {
            vendor_id: self.vendor_id(),
            device_id: self.device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EMDIO config header subsystem ID and subsystem vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioCfhSidsvid(pub u32);
impl EmdioCfhSidsvid {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[inline(always)]
    pub const fn set_subsystem_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[inline(always)]
    pub const fn set_subsystem_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for EmdioCfhSidsvid {
    #[inline(always)]
    fn default() -> EmdioCfhSidsvid {
        EmdioCfhSidsvid(3992983895u64 as u32)
    }
}
impl core::fmt::Debug for EmdioCfhSidsvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmdioCfhSidsvid")
            .field("subsystem_vendor_id", &self.subsystem_vendor_id())
            .field("subsystem_device_id", &self.subsystem_device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmdioCfhSidsvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmdioCfhSidsvid {
            subsystem_vendor_id: u16,
            subsystem_device_id: u16,
        }
        let proxy = EmdioCfhSidsvid {
            subsystem_vendor_id: self.subsystem_vendor_id(),
            subsystem_device_id: self.subsystem_device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EMDIO binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emdiobcr(pub u32);
impl Emdiobcr {
    #[doc = "Root complex instance number."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Root complex instance number."]
    #[inline(always)]
    pub const fn set_rc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "PCI device function number for global EMDIO controller. For assignment of function number, see ."]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PCI device function number for global EMDIO controller. For assignment of function number, see ."]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "If set, this EMDIO function is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If set, this EMDIO function is valid."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Emdiobcr {
    #[inline(always)]
    fn default() -> Emdiobcr {
        Emdiobcr(2147483904u64 as u32)
    }
}
impl core::fmt::Debug for Emdiobcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emdiobcr")
            .field("rc_inst", &self.rc_inst())
            .field("fn_", &self.fn_())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emdiobcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Emdiobcr {
            rc_inst: u8,
            fn_: u8,
            valid: bool,
        }
        let proxy = Emdiobcr {
            rc_inst: self.rc_inst(),
            fn_: self.fn_(),
            valid: self.valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "EMDIO MSI-X configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emdiomcr(pub u32);
impl Emdiomcr {
    #[doc = "Number of MSI-X vectors supported. Formula: NUM_MSIX+1 Range: 1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Number of MSI-X vectors supported. Formula: NUM_MSIX+1 Range: 1"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Emdiomcr {
    #[inline(always)]
    fn default() -> Emdiomcr {
        Emdiomcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Emdiomcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emdiomcr")
            .field("num_msix", &self.num_msix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emdiomcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Emdiomcr {
            num_msix: bool,
        }
        let proxy = Emdiomcr {
            num_msix: self.num_msix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC receive shared memory buffer allotment register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ersmbar(pub u32);
impl Ersmbar {
    #[doc = "Threshold in words for internal receive buffer memory used by all ENETC functions"]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Threshold in words for internal receive buffer memory used by all ENETC functions"]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ersmbar {
    #[inline(always)]
    fn default() -> Ersmbar {
        Ersmbar(1490u64 as u32)
    }
}
impl core::fmt::Debug for Ersmbar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ersmbar")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ersmbar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ersmbar {
            thresh: u32,
        }
        let proxy = Ersmbar {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Guaranteed hash table entry memory capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ghtemcapr(pub u32);
impl Ghtemcapr {
    #[doc = "Total amount of words available to store the guaranteed hash table entries"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total amount of words available to store the guaranteed hash table entries"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ghtemcapr {
    #[inline(always)]
    fn default() -> Ghtemcapr {
        Ghtemcapr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ghtemcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ghtemcapr")
            .field("num_words", &self.num_words())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ghtemcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ghtemcapr {
            num_words: u16,
            mloc: u8,
        }
        let proxy = Ghtemcapr {
            num_words: self.num_words(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Hash bucket table configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hbtcr(pub u32);
impl Hbtcr {
    #[doc = "Specifies the maximum EM/IM Hash collisions chain length allowed"]
    #[must_use]
    #[inline(always)]
    pub const fn max_col(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Specifies the maximum EM/IM Hash collisions chain length allowed"]
    #[inline(always)]
    pub const fn set_max_col(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Specifies the maximum number of Hash Entries Visited during a Search Table Management Command"]
    #[must_use]
    #[inline(always)]
    pub const fn max_visits(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the maximum number of Hash Entries Visited during a Search Table Management Command"]
    #[inline(always)]
    pub const fn set_max_visits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Hbtcr {
    #[inline(always)]
    fn default() -> Hbtcr {
        Hbtcr(50u64 as u32)
    }
}
impl core::fmt::Debug for Hbtcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hbtcr")
            .field("max_col", &self.max_col())
            .field("max_visits", &self.max_visits())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hbtcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hbtcr {
            max_col: u8,
            max_visits: u8,
        }
        let proxy = Hbtcr {
            max_col: self.max_col(),
            max_visits: self.max_visits(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Hash bucket table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hbtmar(pub u32);
impl Hbtmar {
    #[doc = "Number of words allocated from Common Memory"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Number of words allocated from Common Memory"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Minimum number of words required. NUM_WORDS should not be set to a value lower than MIN_WORDS."]
    #[must_use]
    #[inline(always)]
    pub const fn min_words(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minimum number of words required. NUM_WORDS should not be set to a value lower than MIN_WORDS."]
    #[inline(always)]
    pub const fn set_min_words(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Number of table entries per word."]
    #[must_use]
    #[inline(always)]
    pub const fn nepw(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Number of table entries per word."]
    #[inline(always)]
    pub const fn set_nepw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Hbtmar {
    #[inline(always)]
    fn default() -> Hbtmar {
        Hbtmar(104858112u64 as u32)
    }
}
impl core::fmt::Debug for Hbtmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hbtmar")
            .field("num_words", &self.num_words())
            .field("min_words", &self.min_words())
            .field("nepw", &self.nepw())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hbtmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hbtmar {
            num_words: u16,
            min_words: u8,
            nepw: u8,
            mloc: u8,
        }
        let proxy = Hbtmar {
            num_words: self.num_words(),
            min_words: self.min_words(),
            nepw: self.nepw(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HTA 0 HP configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hta0hpcr(pub u32);
impl Hta0hpcr {
    #[doc = "HTA global high priority byte limit setting"]
    #[must_use]
    #[inline(always)]
    pub const fn blimit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "HTA global high priority byte limit setting"]
    #[inline(always)]
    pub const fn set_blimit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "HTA global high priority frame limit"]
    #[must_use]
    #[inline(always)]
    pub const fn flimit(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "HTA global high priority frame limit"]
    #[inline(always)]
    pub const fn set_flimit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Hta0hpcr {
    #[inline(always)]
    fn default() -> Hta0hpcr {
        Hta0hpcr(3072u64 as u32)
    }
}
impl core::fmt::Debug for Hta0hpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hta0hpcr")
            .field("blimit", &self.blimit())
            .field("flimit", &self.flimit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hta0hpcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hta0hpcr {
            blimit: u16,
            flimit: u8,
        }
        let proxy = Hta0hpcr {
            blimit: self.blimit(),
            flimit: self.flimit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "HTA 0 LP configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hta0lpcr(pub u32);
impl Hta0lpcr {
    #[doc = "HTA global low priority byte limit setting"]
    #[must_use]
    #[inline(always)]
    pub const fn blimit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "HTA global low priority byte limit setting"]
    #[inline(always)]
    pub const fn set_blimit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "HTA global low priority Frame Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn flimit(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "HTA global low priority Frame Limit"]
    #[inline(always)]
    pub const fn set_flimit(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Hta0lpcr {
    #[inline(always)]
    fn default() -> Hta0lpcr {
        Hta0lpcr(3072u64 as u32)
    }
}
impl core::fmt::Debug for Hta0lpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Hta0lpcr")
            .field("blimit", &self.blimit())
            .field("flimit", &self.flimit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Hta0lpcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Hta0lpcr {
            blimit: u16,
            flimit: u8,
        }
        let proxy = Hta0lpcr {
            blimit: self.blimit(),
            flimit: self.flimit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress port filter ternary memory capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipftmcapr(pub u32);
impl Ipftmcapr {
    #[doc = "Total amount of ternary memory in words available to NETC for ingress port filtering"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total amount of ternary memory in words available to NETC for ingress port filtering"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Word size in bits 0: 48 bits 1-3: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn word_size(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Word size in bits 0: 48 bits 1-3: reserved"]
    #[inline(always)]
    pub const fn set_word_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for Ipftmcapr {
    #[inline(always)]
    fn default() -> Ipftmcapr {
        Ipftmcapr(196u64 as u32)
    }
}
impl core::fmt::Debug for Ipftmcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipftmcapr")
            .field("num_words", &self.num_words())
            .field("word_size", &self.word_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipftmcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipftmcapr {
            num_words: u16,
            word_size: u8,
        }
        let proxy = Ipftmcapr {
            num_words: self.num_words(),
            word_size: self.word_size(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 0 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0bcr(pub u32);
impl L0bcr {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_port_enetc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[inline(always)]
    pub const fn set_sw_port_enetc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[must_use]
    #[inline(always)]
    pub const fn netc_func(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[inline(always)]
    pub const fn set_netc_func(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_phyad_prtad(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[inline(always)]
    pub const fn set_mdio_phyad_prtad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[must_use]
    #[inline(always)]
    pub const fn spl_sw_port(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[inline(always)]
    pub const fn set_spl_sw_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for L0bcr {
    #[inline(always)]
    fn default() -> L0bcr {
        L0bcr(0u64 as u32)
    }
}
impl core::fmt::Debug for L0bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0bcr")
            .field("sw_port_enetc_inst", &self.sw_port_enetc_inst())
            .field("netc_func", &self.netc_func())
            .field("mdio_phyad_prtad", &self.mdio_phyad_prtad())
            .field("spl_sw_port", &self.spl_sw_port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L0bcr {
            sw_port_enetc_inst: u8,
            netc_func: bool,
            mdio_phyad_prtad: u8,
            spl_sw_port: u8,
        }
        let proxy = L0bcr {
            sw_port_enetc_inst: self.sw_port_enetc_inst(),
            netc_func: self.netc_func(),
            mdio_phyad_prtad: self.mdio_phyad_prtad(),
            spl_sw_port: self.spl_sw_port(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 0 capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0capr(pub u32);
impl L0capr {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[must_use]
    #[inline(always)]
    pub const fn link_type(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[inline(always)]
    pub const fn set_link_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[inline(always)]
    pub const fn set_num_tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[inline(always)]
    pub const fn set_num_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of congestion groups supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_cg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of congestion groups supported"]
    #[inline(always)]
    pub const fn set_num_cg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Time Gate Scheduling"]
    #[must_use]
    #[inline(always)]
    pub const fn tgs(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gate Scheduling"]
    #[inline(always)]
    pub const fn set_tgs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Credit Based Shaping"]
    #[must_use]
    #[inline(always)]
    pub const fn cbs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Based Shaping"]
    #[inline(always)]
    pub const fn set_cbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for L0capr {
    #[inline(always)]
    fn default() -> L0capr {
        L0capr(923234304u64 as u32)
    }
}
impl core::fmt::Debug for L0capr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0capr")
            .field("link_type", &self.link_type())
            .field("num_tc", &self.num_tc())
            .field("num_q", &self.num_q())
            .field("num_cg", &self.num_cg())
            .field("tgs", &self.tgs())
            .field("cbs", &self.cbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0capr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L0capr {
            link_type: bool,
            num_tc: u8,
            num_q: u8,
            num_cg: u8,
            tgs: bool,
            cbs: bool,
        }
        let proxy = L0capr {
            link_type: self.link_type(),
            num_tc: self.num_tc(),
            num_q: self.num_q(),
            num_cg: self.num_cg(),
            tgs: self.tgs(),
            cbs: self.cbs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 0 end 0 MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0e0mar1(pub u32);
impl L0e0mar1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L0e0mar1 {
    #[inline(always)]
    fn default() -> L0e0mar1 {
        L0e0mar1(0u64 as u32)
    }
}
impl core::fmt::Debug for L0e0mar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0e0mar1")
            .field("mac_addr", &self.mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0e0mar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L0e0mar1 {
            mac_addr: u16,
        }
        let proxy = L0e0mar1 {
            mac_addr: self.mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 0 I/O capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0iocapr(pub u32);
impl L0iocapr {
    #[doc = "PCS protocols supported"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs_prot(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PCS protocols supported"]
    #[inline(always)]
    pub const fn set_pcs_prot(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "IO Variants supported"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IO Variants supported"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "External MDIO supported."]
    #[must_use]
    #[inline(always)]
    pub const fn emdio(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "External MDIO supported."]
    #[inline(always)]
    pub const fn set_emdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "RevMII MII rate"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii_rate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RevMII MII rate"]
    #[inline(always)]
    pub const fn set_revmii_rate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L0iocapr {
    #[inline(always)]
    fn default() -> L0iocapr {
        L0iocapr(268435456u64 as u32)
    }
}
impl core::fmt::Debug for L0iocapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0iocapr")
            .field("pcs_prot", &self.pcs_prot())
            .field("io_var", &self.io_var())
            .field("emdio", &self.emdio())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0iocapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L0iocapr {
            pcs_prot: u16,
            io_var: u8,
            emdio: bool,
            revmii_rate: bool,
            revmii: bool,
        }
        let proxy = L0iocapr {
            pcs_prot: self.pcs_prot(),
            io_var: self.io_var(),
            emdio: self.emdio(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 0 MAC capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0mcapr(pub u32);
impl L0mcapr {
    #[doc = "MAC Variant"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_var(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "MAC Variant"]
    #[inline(always)]
    pub const fn set_mac_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Egress frame padding capability"]
    #[must_use]
    #[inline(always)]
    pub const fn efpad(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Egress frame padding capability"]
    #[inline(always)]
    pub const fn set_efpad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[must_use]
    #[inline(always)]
    pub const fn pipg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[inline(always)]
    pub const fn set_pipg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Half Duplex capability"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Half Duplex capability"]
    #[inline(always)]
    pub const fn set_hd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[must_use]
    #[inline(always)]
    pub const fn fp(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[inline(always)]
    pub const fn set_fp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[must_use]
    #[inline(always)]
    pub const fn min_mpdu(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[inline(always)]
    pub const fn set_min_mpdu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates the MII protocol supported"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the MII protocol supported"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for L0mcapr {
    #[inline(always)]
    fn default() -> L0mcapr {
        L0mcapr(5473u64 as u32)
    }
}
impl core::fmt::Debug for L0mcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0mcapr")
            .field("mac_var", &self.mac_var())
            .field("efpad", &self.efpad())
            .field("pipg", &self.pipg())
            .field("hd", &self.hd())
            .field("fp", &self.fp())
            .field("min_mpdu", &self.min_mpdu())
            .field("mii_prot", &self.mii_prot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0mcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L0mcapr {
            mac_var: u8,
            efpad: u8,
            pipg: bool,
            hd: bool,
            fp: u8,
            min_mpdu: bool,
            mii_prot: u8,
        }
        let proxy = L0mcapr {
            mac_var: self.mac_var(),
            efpad: self.efpad(),
            pipg: self.pipg(),
            hd: self.hd(),
            fp: self.fp(),
            min_mpdu: self.min_mpdu(),
            mii_prot: self.mii_prot(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 0 transmit byte credit comfort threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L0txbcctr(pub u32);
impl L0txbcctr {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L0txbcctr {
    #[inline(always)]
    fn default() -> L0txbcctr {
        L0txbcctr(512u64 as u32)
    }
}
impl core::fmt::Debug for L0txbcctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L0txbcctr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L0txbcctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L0txbcctr {
            thresh: u16,
        }
        let proxy = L0txbcctr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 1 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1bcr(pub u32);
impl L1bcr {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_port_enetc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[inline(always)]
    pub const fn set_sw_port_enetc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[must_use]
    #[inline(always)]
    pub const fn netc_func(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[inline(always)]
    pub const fn set_netc_func(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_phyad_prtad(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[inline(always)]
    pub const fn set_mdio_phyad_prtad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[must_use]
    #[inline(always)]
    pub const fn spl_sw_port(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[inline(always)]
    pub const fn set_spl_sw_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for L1bcr {
    #[inline(always)]
    fn default() -> L1bcr {
        L1bcr(1u64 as u32)
    }
}
impl core::fmt::Debug for L1bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1bcr")
            .field("sw_port_enetc_inst", &self.sw_port_enetc_inst())
            .field("netc_func", &self.netc_func())
            .field("mdio_phyad_prtad", &self.mdio_phyad_prtad())
            .field("spl_sw_port", &self.spl_sw_port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L1bcr {
            sw_port_enetc_inst: u8,
            netc_func: bool,
            mdio_phyad_prtad: u8,
            spl_sw_port: u8,
        }
        let proxy = L1bcr {
            sw_port_enetc_inst: self.sw_port_enetc_inst(),
            netc_func: self.netc_func(),
            mdio_phyad_prtad: self.mdio_phyad_prtad(),
            spl_sw_port: self.spl_sw_port(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 1 capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1capr(pub u32);
impl L1capr {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[must_use]
    #[inline(always)]
    pub const fn link_type(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[inline(always)]
    pub const fn set_link_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[inline(always)]
    pub const fn set_num_tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[inline(always)]
    pub const fn set_num_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of congestion groups supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_cg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of congestion groups supported"]
    #[inline(always)]
    pub const fn set_num_cg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Time Gate Scheduling"]
    #[must_use]
    #[inline(always)]
    pub const fn tgs(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gate Scheduling"]
    #[inline(always)]
    pub const fn set_tgs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Credit Based Shaping"]
    #[must_use]
    #[inline(always)]
    pub const fn cbs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Based Shaping"]
    #[inline(always)]
    pub const fn set_cbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for L1capr {
    #[inline(always)]
    fn default() -> L1capr {
        L1capr(923234304u64 as u32)
    }
}
impl core::fmt::Debug for L1capr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1capr")
            .field("link_type", &self.link_type())
            .field("num_tc", &self.num_tc())
            .field("num_q", &self.num_q())
            .field("num_cg", &self.num_cg())
            .field("tgs", &self.tgs())
            .field("cbs", &self.cbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1capr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L1capr {
            link_type: bool,
            num_tc: u8,
            num_q: u8,
            num_cg: u8,
            tgs: bool,
            cbs: bool,
        }
        let proxy = L1capr {
            link_type: self.link_type(),
            num_tc: self.num_tc(),
            num_q: self.num_q(),
            num_cg: self.num_cg(),
            tgs: self.tgs(),
            cbs: self.cbs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 1 end 0 MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1e0mar1(pub u32);
impl L1e0mar1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L1e0mar1 {
    #[inline(always)]
    fn default() -> L1e0mar1 {
        L1e0mar1(0u64 as u32)
    }
}
impl core::fmt::Debug for L1e0mar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1e0mar1")
            .field("mac_addr", &self.mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1e0mar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L1e0mar1 {
            mac_addr: u16,
        }
        let proxy = L1e0mar1 {
            mac_addr: self.mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 1 I/O capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1iocapr(pub u32);
impl L1iocapr {
    #[doc = "PCS protocols supported"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs_prot(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PCS protocols supported"]
    #[inline(always)]
    pub const fn set_pcs_prot(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "IO Variants supported"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IO Variants supported"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "External MDIO supported."]
    #[must_use]
    #[inline(always)]
    pub const fn emdio(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "External MDIO supported."]
    #[inline(always)]
    pub const fn set_emdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "RevMII MII rate"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii_rate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RevMII MII rate"]
    #[inline(always)]
    pub const fn set_revmii_rate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L1iocapr {
    #[inline(always)]
    fn default() -> L1iocapr {
        L1iocapr(268435456u64 as u32)
    }
}
impl core::fmt::Debug for L1iocapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1iocapr")
            .field("pcs_prot", &self.pcs_prot())
            .field("io_var", &self.io_var())
            .field("emdio", &self.emdio())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1iocapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L1iocapr {
            pcs_prot: u16,
            io_var: u8,
            emdio: bool,
            revmii_rate: bool,
            revmii: bool,
        }
        let proxy = L1iocapr {
            pcs_prot: self.pcs_prot(),
            io_var: self.io_var(),
            emdio: self.emdio(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 1 MAC capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1mcapr(pub u32);
impl L1mcapr {
    #[doc = "MAC Variant"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_var(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "MAC Variant"]
    #[inline(always)]
    pub const fn set_mac_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Egress frame padding capability"]
    #[must_use]
    #[inline(always)]
    pub const fn efpad(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Egress frame padding capability"]
    #[inline(always)]
    pub const fn set_efpad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[must_use]
    #[inline(always)]
    pub const fn pipg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[inline(always)]
    pub const fn set_pipg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Half Duplex capability"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Half Duplex capability"]
    #[inline(always)]
    pub const fn set_hd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[must_use]
    #[inline(always)]
    pub const fn fp(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[inline(always)]
    pub const fn set_fp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[must_use]
    #[inline(always)]
    pub const fn min_mpdu(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[inline(always)]
    pub const fn set_min_mpdu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates the MII protocol supported"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the MII protocol supported"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for L1mcapr {
    #[inline(always)]
    fn default() -> L1mcapr {
        L1mcapr(5473u64 as u32)
    }
}
impl core::fmt::Debug for L1mcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1mcapr")
            .field("mac_var", &self.mac_var())
            .field("efpad", &self.efpad())
            .field("pipg", &self.pipg())
            .field("hd", &self.hd())
            .field("fp", &self.fp())
            .field("min_mpdu", &self.min_mpdu())
            .field("mii_prot", &self.mii_prot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1mcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L1mcapr {
            mac_var: u8,
            efpad: u8,
            pipg: bool,
            hd: bool,
            fp: u8,
            min_mpdu: bool,
            mii_prot: u8,
        }
        let proxy = L1mcapr {
            mac_var: self.mac_var(),
            efpad: self.efpad(),
            pipg: self.pipg(),
            hd: self.hd(),
            fp: self.fp(),
            min_mpdu: self.min_mpdu(),
            mii_prot: self.mii_prot(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 1 transmit byte credit comfort threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L1txbcctr(pub u32);
impl L1txbcctr {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L1txbcctr {
    #[inline(always)]
    fn default() -> L1txbcctr {
        L1txbcctr(512u64 as u32)
    }
}
impl core::fmt::Debug for L1txbcctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1txbcctr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L1txbcctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L1txbcctr {
            thresh: u16,
        }
        let proxy = L1txbcctr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 2 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2bcr(pub u32);
impl L2bcr {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_port_enetc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[inline(always)]
    pub const fn set_sw_port_enetc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[must_use]
    #[inline(always)]
    pub const fn netc_func(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[inline(always)]
    pub const fn set_netc_func(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_phyad_prtad(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[inline(always)]
    pub const fn set_mdio_phyad_prtad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[must_use]
    #[inline(always)]
    pub const fn spl_sw_port(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[inline(always)]
    pub const fn set_spl_sw_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for L2bcr {
    #[inline(always)]
    fn default() -> L2bcr {
        L2bcr(2u64 as u32)
    }
}
impl core::fmt::Debug for L2bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2bcr")
            .field("sw_port_enetc_inst", &self.sw_port_enetc_inst())
            .field("netc_func", &self.netc_func())
            .field("mdio_phyad_prtad", &self.mdio_phyad_prtad())
            .field("spl_sw_port", &self.spl_sw_port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L2bcr {
            sw_port_enetc_inst: u8,
            netc_func: bool,
            mdio_phyad_prtad: u8,
            spl_sw_port: u8,
        }
        let proxy = L2bcr {
            sw_port_enetc_inst: self.sw_port_enetc_inst(),
            netc_func: self.netc_func(),
            mdio_phyad_prtad: self.mdio_phyad_prtad(),
            spl_sw_port: self.spl_sw_port(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 2 capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2capr(pub u32);
impl L2capr {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[must_use]
    #[inline(always)]
    pub const fn link_type(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[inline(always)]
    pub const fn set_link_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[inline(always)]
    pub const fn set_num_tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[inline(always)]
    pub const fn set_num_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of congestion groups supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_cg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of congestion groups supported"]
    #[inline(always)]
    pub const fn set_num_cg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Time Gate Scheduling"]
    #[must_use]
    #[inline(always)]
    pub const fn tgs(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gate Scheduling"]
    #[inline(always)]
    pub const fn set_tgs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Credit Based Shaping"]
    #[must_use]
    #[inline(always)]
    pub const fn cbs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Based Shaping"]
    #[inline(always)]
    pub const fn set_cbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for L2capr {
    #[inline(always)]
    fn default() -> L2capr {
        L2capr(923234304u64 as u32)
    }
}
impl core::fmt::Debug for L2capr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2capr")
            .field("link_type", &self.link_type())
            .field("num_tc", &self.num_tc())
            .field("num_q", &self.num_q())
            .field("num_cg", &self.num_cg())
            .field("tgs", &self.tgs())
            .field("cbs", &self.cbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2capr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L2capr {
            link_type: bool,
            num_tc: u8,
            num_q: u8,
            num_cg: u8,
            tgs: bool,
            cbs: bool,
        }
        let proxy = L2capr {
            link_type: self.link_type(),
            num_tc: self.num_tc(),
            num_q: self.num_q(),
            num_cg: self.num_cg(),
            tgs: self.tgs(),
            cbs: self.cbs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 2 end 0 MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2e0mar1(pub u32);
impl L2e0mar1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L2e0mar1 {
    #[inline(always)]
    fn default() -> L2e0mar1 {
        L2e0mar1(0u64 as u32)
    }
}
impl core::fmt::Debug for L2e0mar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2e0mar1")
            .field("mac_addr", &self.mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2e0mar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L2e0mar1 {
            mac_addr: u16,
        }
        let proxy = L2e0mar1 {
            mac_addr: self.mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 2 I/O capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2iocapr(pub u32);
impl L2iocapr {
    #[doc = "PCS protocols supported"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs_prot(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PCS protocols supported"]
    #[inline(always)]
    pub const fn set_pcs_prot(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "IO Variants supported"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IO Variants supported"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "External MDIO supported."]
    #[must_use]
    #[inline(always)]
    pub const fn emdio(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "External MDIO supported."]
    #[inline(always)]
    pub const fn set_emdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "RevMII MII rate"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii_rate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RevMII MII rate"]
    #[inline(always)]
    pub const fn set_revmii_rate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L2iocapr {
    #[inline(always)]
    fn default() -> L2iocapr {
        L2iocapr(268435456u64 as u32)
    }
}
impl core::fmt::Debug for L2iocapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2iocapr")
            .field("pcs_prot", &self.pcs_prot())
            .field("io_var", &self.io_var())
            .field("emdio", &self.emdio())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2iocapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L2iocapr {
            pcs_prot: u16,
            io_var: u8,
            emdio: bool,
            revmii_rate: bool,
            revmii: bool,
        }
        let proxy = L2iocapr {
            pcs_prot: self.pcs_prot(),
            io_var: self.io_var(),
            emdio: self.emdio(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 2 MAC capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2mcapr(pub u32);
impl L2mcapr {
    #[doc = "MAC Variant"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_var(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "MAC Variant"]
    #[inline(always)]
    pub const fn set_mac_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Egress frame padding capability"]
    #[must_use]
    #[inline(always)]
    pub const fn efpad(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Egress frame padding capability"]
    #[inline(always)]
    pub const fn set_efpad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[must_use]
    #[inline(always)]
    pub const fn pipg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[inline(always)]
    pub const fn set_pipg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Half Duplex capability"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Half Duplex capability"]
    #[inline(always)]
    pub const fn set_hd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[must_use]
    #[inline(always)]
    pub const fn fp(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[inline(always)]
    pub const fn set_fp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[must_use]
    #[inline(always)]
    pub const fn min_mpdu(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[inline(always)]
    pub const fn set_min_mpdu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates the MII protocol supported"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the MII protocol supported"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for L2mcapr {
    #[inline(always)]
    fn default() -> L2mcapr {
        L2mcapr(5473u64 as u32)
    }
}
impl core::fmt::Debug for L2mcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2mcapr")
            .field("mac_var", &self.mac_var())
            .field("efpad", &self.efpad())
            .field("pipg", &self.pipg())
            .field("hd", &self.hd())
            .field("fp", &self.fp())
            .field("min_mpdu", &self.min_mpdu())
            .field("mii_prot", &self.mii_prot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2mcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L2mcapr {
            mac_var: u8,
            efpad: u8,
            pipg: bool,
            hd: bool,
            fp: u8,
            min_mpdu: bool,
            mii_prot: u8,
        }
        let proxy = L2mcapr {
            mac_var: self.mac_var(),
            efpad: self.efpad(),
            pipg: self.pipg(),
            hd: self.hd(),
            fp: self.fp(),
            min_mpdu: self.min_mpdu(),
            mii_prot: self.mii_prot(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 2 transmit byte credit comfort threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L2txbcctr(pub u32);
impl L2txbcctr {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L2txbcctr {
    #[inline(always)]
    fn default() -> L2txbcctr {
        L2txbcctr(512u64 as u32)
    }
}
impl core::fmt::Debug for L2txbcctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2txbcctr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L2txbcctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L2txbcctr {
            thresh: u16,
        }
        let proxy = L2txbcctr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 3 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L3bcr(pub u32);
impl L3bcr {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_port_enetc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[inline(always)]
    pub const fn set_sw_port_enetc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[must_use]
    #[inline(always)]
    pub const fn netc_func(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[inline(always)]
    pub const fn set_netc_func(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_phyad_prtad(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[inline(always)]
    pub const fn set_mdio_phyad_prtad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[must_use]
    #[inline(always)]
    pub const fn spl_sw_port(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[inline(always)]
    pub const fn set_spl_sw_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for L3bcr {
    #[inline(always)]
    fn default() -> L3bcr {
        L3bcr(3u64 as u32)
    }
}
impl core::fmt::Debug for L3bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L3bcr")
            .field("sw_port_enetc_inst", &self.sw_port_enetc_inst())
            .field("netc_func", &self.netc_func())
            .field("mdio_phyad_prtad", &self.mdio_phyad_prtad())
            .field("spl_sw_port", &self.spl_sw_port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L3bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L3bcr {
            sw_port_enetc_inst: u8,
            netc_func: bool,
            mdio_phyad_prtad: u8,
            spl_sw_port: u8,
        }
        let proxy = L3bcr {
            sw_port_enetc_inst: self.sw_port_enetc_inst(),
            netc_func: self.netc_func(),
            mdio_phyad_prtad: self.mdio_phyad_prtad(),
            spl_sw_port: self.spl_sw_port(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 3 capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L3capr(pub u32);
impl L3capr {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[must_use]
    #[inline(always)]
    pub const fn link_type(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[inline(always)]
    pub const fn set_link_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[inline(always)]
    pub const fn set_num_tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[inline(always)]
    pub const fn set_num_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of congestion groups supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_cg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of congestion groups supported"]
    #[inline(always)]
    pub const fn set_num_cg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Time Gate Scheduling"]
    #[must_use]
    #[inline(always)]
    pub const fn tgs(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gate Scheduling"]
    #[inline(always)]
    pub const fn set_tgs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Credit Based Shaping"]
    #[must_use]
    #[inline(always)]
    pub const fn cbs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Based Shaping"]
    #[inline(always)]
    pub const fn set_cbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for L3capr {
    #[inline(always)]
    fn default() -> L3capr {
        L3capr(923234304u64 as u32)
    }
}
impl core::fmt::Debug for L3capr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L3capr")
            .field("link_type", &self.link_type())
            .field("num_tc", &self.num_tc())
            .field("num_q", &self.num_q())
            .field("num_cg", &self.num_cg())
            .field("tgs", &self.tgs())
            .field("cbs", &self.cbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L3capr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L3capr {
            link_type: bool,
            num_tc: u8,
            num_q: u8,
            num_cg: u8,
            tgs: bool,
            cbs: bool,
        }
        let proxy = L3capr {
            link_type: self.link_type(),
            num_tc: self.num_tc(),
            num_q: self.num_q(),
            num_cg: self.num_cg(),
            tgs: self.tgs(),
            cbs: self.cbs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 3 end 0 MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L3e0mar1(pub u32);
impl L3e0mar1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L3e0mar1 {
    #[inline(always)]
    fn default() -> L3e0mar1 {
        L3e0mar1(0u64 as u32)
    }
}
impl core::fmt::Debug for L3e0mar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L3e0mar1")
            .field("mac_addr", &self.mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L3e0mar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L3e0mar1 {
            mac_addr: u16,
        }
        let proxy = L3e0mar1 {
            mac_addr: self.mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 3 I/O capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L3iocapr(pub u32);
impl L3iocapr {
    #[doc = "PCS protocols supported"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs_prot(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PCS protocols supported"]
    #[inline(always)]
    pub const fn set_pcs_prot(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "IO Variants supported"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IO Variants supported"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "External MDIO supported."]
    #[must_use]
    #[inline(always)]
    pub const fn emdio(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "External MDIO supported."]
    #[inline(always)]
    pub const fn set_emdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "RevMII MII rate"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii_rate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RevMII MII rate"]
    #[inline(always)]
    pub const fn set_revmii_rate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L3iocapr {
    #[inline(always)]
    fn default() -> L3iocapr {
        L3iocapr(268435456u64 as u32)
    }
}
impl core::fmt::Debug for L3iocapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L3iocapr")
            .field("pcs_prot", &self.pcs_prot())
            .field("io_var", &self.io_var())
            .field("emdio", &self.emdio())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L3iocapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L3iocapr {
            pcs_prot: u16,
            io_var: u8,
            emdio: bool,
            revmii_rate: bool,
            revmii: bool,
        }
        let proxy = L3iocapr {
            pcs_prot: self.pcs_prot(),
            io_var: self.io_var(),
            emdio: self.emdio(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 3 MAC capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L3mcapr(pub u32);
impl L3mcapr {
    #[doc = "MAC Variant"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_var(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "MAC Variant"]
    #[inline(always)]
    pub const fn set_mac_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Egress frame padding capability"]
    #[must_use]
    #[inline(always)]
    pub const fn efpad(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Egress frame padding capability"]
    #[inline(always)]
    pub const fn set_efpad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[must_use]
    #[inline(always)]
    pub const fn pipg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[inline(always)]
    pub const fn set_pipg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Half Duplex capability"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Half Duplex capability"]
    #[inline(always)]
    pub const fn set_hd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[must_use]
    #[inline(always)]
    pub const fn fp(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[inline(always)]
    pub const fn set_fp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[must_use]
    #[inline(always)]
    pub const fn min_mpdu(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[inline(always)]
    pub const fn set_min_mpdu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates the MII protocol supported"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the MII protocol supported"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for L3mcapr {
    #[inline(always)]
    fn default() -> L3mcapr {
        L3mcapr(5473u64 as u32)
    }
}
impl core::fmt::Debug for L3mcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L3mcapr")
            .field("mac_var", &self.mac_var())
            .field("efpad", &self.efpad())
            .field("pipg", &self.pipg())
            .field("hd", &self.hd())
            .field("fp", &self.fp())
            .field("min_mpdu", &self.min_mpdu())
            .field("mii_prot", &self.mii_prot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L3mcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L3mcapr {
            mac_var: u8,
            efpad: u8,
            pipg: bool,
            hd: bool,
            fp: u8,
            min_mpdu: bool,
            mii_prot: u8,
        }
        let proxy = L3mcapr {
            mac_var: self.mac_var(),
            efpad: self.efpad(),
            pipg: self.pipg(),
            hd: self.hd(),
            fp: self.fp(),
            min_mpdu: self.min_mpdu(),
            mii_prot: self.mii_prot(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 3 transmit byte credit comfort threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L3txbcctr(pub u32);
impl L3txbcctr {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L3txbcctr {
    #[inline(always)]
    fn default() -> L3txbcctr {
        L3txbcctr(512u64 as u32)
    }
}
impl core::fmt::Debug for L3txbcctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L3txbcctr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L3txbcctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L3txbcctr {
            thresh: u16,
        }
        let proxy = L3txbcctr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 4 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L4bcr(pub u32);
impl L4bcr {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_port_enetc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[inline(always)]
    pub const fn set_sw_port_enetc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[must_use]
    #[inline(always)]
    pub const fn netc_func(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[inline(always)]
    pub const fn set_netc_func(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_phyad_prtad(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "This value indicate an MDIO PHY address"]
    #[inline(always)]
    pub const fn set_mdio_phyad_prtad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[must_use]
    #[inline(always)]
    pub const fn spl_sw_port(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[inline(always)]
    pub const fn set_spl_sw_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for L4bcr {
    #[inline(always)]
    fn default() -> L4bcr {
        L4bcr(64u64 as u32)
    }
}
impl core::fmt::Debug for L4bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L4bcr")
            .field("sw_port_enetc_inst", &self.sw_port_enetc_inst())
            .field("netc_func", &self.netc_func())
            .field("mdio_phyad_prtad", &self.mdio_phyad_prtad())
            .field("spl_sw_port", &self.spl_sw_port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L4bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L4bcr {
            sw_port_enetc_inst: u8,
            netc_func: bool,
            mdio_phyad_prtad: u8,
            spl_sw_port: u8,
        }
        let proxy = L4bcr {
            sw_port_enetc_inst: self.sw_port_enetc_inst(),
            netc_func: self.netc_func(),
            mdio_phyad_prtad: self.mdio_phyad_prtad(),
            spl_sw_port: self.spl_sw_port(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 4 capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L4capr(pub u32);
impl L4capr {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[must_use]
    #[inline(always)]
    pub const fn link_type(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[inline(always)]
    pub const fn set_link_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[inline(always)]
    pub const fn set_num_tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[inline(always)]
    pub const fn set_num_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of congestion groups supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_cg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of congestion groups supported"]
    #[inline(always)]
    pub const fn set_num_cg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Time Gate Scheduling"]
    #[must_use]
    #[inline(always)]
    pub const fn tgs(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gate Scheduling"]
    #[inline(always)]
    pub const fn set_tgs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Credit Based Shaping"]
    #[must_use]
    #[inline(always)]
    pub const fn cbs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Based Shaping"]
    #[inline(always)]
    pub const fn set_cbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for L4capr {
    #[inline(always)]
    fn default() -> L4capr {
        L4capr(923234304u64 as u32)
    }
}
impl core::fmt::Debug for L4capr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L4capr")
            .field("link_type", &self.link_type())
            .field("num_tc", &self.num_tc())
            .field("num_q", &self.num_q())
            .field("num_cg", &self.num_cg())
            .field("tgs", &self.tgs())
            .field("cbs", &self.cbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L4capr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L4capr {
            link_type: bool,
            num_tc: u8,
            num_q: u8,
            num_cg: u8,
            tgs: bool,
            cbs: bool,
        }
        let proxy = L4capr {
            link_type: self.link_type(),
            num_tc: self.num_tc(),
            num_q: self.num_q(),
            num_cg: self.num_cg(),
            tgs: self.tgs(),
            cbs: self.cbs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 4 end 0 MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L4e0mar1(pub u32);
impl L4e0mar1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L4e0mar1 {
    #[inline(always)]
    fn default() -> L4e0mar1 {
        L4e0mar1(0u64 as u32)
    }
}
impl core::fmt::Debug for L4e0mar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L4e0mar1")
            .field("mac_addr", &self.mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L4e0mar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L4e0mar1 {
            mac_addr: u16,
        }
        let proxy = L4e0mar1 {
            mac_addr: self.mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 4 I/O capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L4iocapr(pub u32);
impl L4iocapr {
    #[doc = "PCS protocols supported"]
    #[must_use]
    #[inline(always)]
    pub const fn pcs_prot(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "PCS protocols supported"]
    #[inline(always)]
    pub const fn set_pcs_prot(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "IO Variants supported"]
    #[must_use]
    #[inline(always)]
    pub const fn io_var(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "IO Variants supported"]
    #[inline(always)]
    pub const fn set_io_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "External MDIO supported."]
    #[must_use]
    #[inline(always)]
    pub const fn emdio(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "External MDIO supported."]
    #[inline(always)]
    pub const fn set_emdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "RevMII MII rate"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii_rate(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "RevMII MII rate"]
    #[inline(always)]
    pub const fn set_revmii_rate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reverse Mode Device Configuration"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for L4iocapr {
    #[inline(always)]
    fn default() -> L4iocapr {
        L4iocapr(268435456u64 as u32)
    }
}
impl core::fmt::Debug for L4iocapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L4iocapr")
            .field("pcs_prot", &self.pcs_prot())
            .field("io_var", &self.io_var())
            .field("emdio", &self.emdio())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L4iocapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L4iocapr {
            pcs_prot: u16,
            io_var: u8,
            emdio: bool,
            revmii_rate: bool,
            revmii: bool,
        }
        let proxy = L4iocapr {
            pcs_prot: self.pcs_prot(),
            io_var: self.io_var(),
            emdio: self.emdio(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 4 MAC capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L4mcapr(pub u32);
impl L4mcapr {
    #[doc = "MAC Variant"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_var(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "MAC Variant"]
    #[inline(always)]
    pub const fn set_mac_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Egress frame padding capability"]
    #[must_use]
    #[inline(always)]
    pub const fn efpad(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Egress frame padding capability"]
    #[inline(always)]
    pub const fn set_efpad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[must_use]
    #[inline(always)]
    pub const fn pipg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[inline(always)]
    pub const fn set_pipg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Half Duplex capability"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Half Duplex capability"]
    #[inline(always)]
    pub const fn set_hd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[must_use]
    #[inline(always)]
    pub const fn fp(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[inline(always)]
    pub const fn set_fp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[must_use]
    #[inline(always)]
    pub const fn min_mpdu(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[inline(always)]
    pub const fn set_min_mpdu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates the MII protocol supported"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the MII protocol supported"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for L4mcapr {
    #[inline(always)]
    fn default() -> L4mcapr {
        L4mcapr(5473u64 as u32)
    }
}
impl core::fmt::Debug for L4mcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L4mcapr")
            .field("mac_var", &self.mac_var())
            .field("efpad", &self.efpad())
            .field("pipg", &self.pipg())
            .field("hd", &self.hd())
            .field("fp", &self.fp())
            .field("min_mpdu", &self.min_mpdu())
            .field("mii_prot", &self.mii_prot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L4mcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L4mcapr {
            mac_var: u8,
            efpad: u8,
            pipg: bool,
            hd: bool,
            fp: u8,
            min_mpdu: bool,
            mii_prot: u8,
        }
        let proxy = L4mcapr {
            mac_var: self.mac_var(),
            efpad: self.efpad(),
            pipg: self.pipg(),
            hd: self.hd(),
            fp: self.fp(),
            min_mpdu: self.min_mpdu(),
            mii_prot: self.mii_prot(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 4 transmit byte credit comfort threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L4txbcctr(pub u32);
impl L4txbcctr {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L4txbcctr {
    #[inline(always)]
    fn default() -> L4txbcctr {
        L4txbcctr(512u64 as u32)
    }
}
impl core::fmt::Debug for L4txbcctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L4txbcctr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L4txbcctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L4txbcctr {
            thresh: u16,
        }
        let proxy = L4txbcctr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 5 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L5bcr(pub u32);
impl L5bcr {
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_port_enetc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "NETC_FUNC=ENETC: Primary link end's ENETC instance number mapped to this link end"]
    #[inline(always)]
    pub const fn set_sw_port_enetc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[must_use]
    #[inline(always)]
    pub const fn netc_func(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Primary link end's NETC Function Type"]
    #[inline(always)]
    pub const fn set_netc_func(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[must_use]
    #[inline(always)]
    pub const fn spl_sw_port(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Secondary pseudo link end's switch port number"]
    #[inline(always)]
    pub const fn set_spl_sw_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for L5bcr {
    #[inline(always)]
    fn default() -> L5bcr {
        L5bcr(262209u64 as u32)
    }
}
impl core::fmt::Debug for L5bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L5bcr")
            .field("sw_port_enetc_inst", &self.sw_port_enetc_inst())
            .field("netc_func", &self.netc_func())
            .field("spl_sw_port", &self.spl_sw_port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L5bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L5bcr {
            sw_port_enetc_inst: u8,
            netc_func: bool,
            spl_sw_port: u8,
        }
        let proxy = L5bcr {
            sw_port_enetc_inst: self.sw_port_enetc_inst(),
            netc_func: self.netc_func(),
            spl_sw_port: self.spl_sw_port(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 5 capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L5capr(pub u32);
impl L5capr {
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[must_use]
    #[inline(always)]
    pub const fn link_type(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the link type 0 External Link 1 Pseudo Link"]
    #[inline(always)]
    pub const fn set_link_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Traffic Classes (TCs) supported Formula: NUM_TC+1"]
    #[inline(always)]
    pub const fn set_num_tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported"]
    #[inline(always)]
    pub const fn set_num_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of congestion groups supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_cg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of congestion groups supported"]
    #[inline(always)]
    pub const fn set_num_cg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Time Gate Scheduling"]
    #[must_use]
    #[inline(always)]
    pub const fn tgs(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gate Scheduling"]
    #[inline(always)]
    pub const fn set_tgs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Credit Based Shaping"]
    #[must_use]
    #[inline(always)]
    pub const fn cbs(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Based Shaping"]
    #[inline(always)]
    pub const fn set_cbs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
}
impl Default for L5capr {
    #[inline(always)]
    fn default() -> L5capr {
        L5capr(923234320u64 as u32)
    }
}
impl core::fmt::Debug for L5capr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L5capr")
            .field("link_type", &self.link_type())
            .field("num_tc", &self.num_tc())
            .field("num_q", &self.num_q())
            .field("num_cg", &self.num_cg())
            .field("tgs", &self.tgs())
            .field("cbs", &self.cbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L5capr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L5capr {
            link_type: bool,
            num_tc: u8,
            num_q: u8,
            num_cg: u8,
            tgs: bool,
            cbs: bool,
        }
        let proxy = L5capr {
            link_type: self.link_type(),
            num_tc: self.num_tc(),
            num_q: self.num_q(),
            num_cg: self.num_cg(),
            tgs: self.tgs(),
            cbs: self.cbs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 5 end 0 MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L5e0mar1(pub u32);
impl L5e0mar1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L5e0mar1 {
    #[inline(always)]
    fn default() -> L5e0mar1 {
        L5e0mar1(0u64 as u32)
    }
}
impl core::fmt::Debug for L5e0mar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L5e0mar1")
            .field("mac_addr", &self.mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L5e0mar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L5e0mar1 {
            mac_addr: u16,
        }
        let proxy = L5e0mar1 {
            mac_addr: self.mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 5 end 1 MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L5e1mar1(pub u32);
impl L5e1mar1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L5e1mar1 {
    #[inline(always)]
    fn default() -> L5e1mar1 {
        L5e1mar1(0u64 as u32)
    }
}
impl core::fmt::Debug for L5e1mar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L5e1mar1")
            .field("mac_addr", &self.mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L5e1mar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L5e1mar1 {
            mac_addr: u16,
        }
        let proxy = L5e1mar1 {
            mac_addr: self.mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 5 MAC capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L5mcapr(pub u32);
impl L5mcapr {
    #[doc = "MAC Variant"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_var(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "MAC Variant"]
    #[inline(always)]
    pub const fn set_mac_var(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Egress frame padding capability"]
    #[must_use]
    #[inline(always)]
    pub const fn efpad(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Egress frame padding capability"]
    #[inline(always)]
    pub const fn set_efpad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[must_use]
    #[inline(always)]
    pub const fn pipg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Configurable preamble/IPG capability"]
    #[inline(always)]
    pub const fn set_pipg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Half Duplex capability"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Half Duplex capability"]
    #[inline(always)]
    pub const fn set_hd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[must_use]
    #[inline(always)]
    pub const fn fp(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates if frame preemption is supported"]
    #[inline(always)]
    pub const fn set_fp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[must_use]
    #[inline(always)]
    pub const fn min_mpdu(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Minimum MAC Protocol Data Unit (PDU) size check"]
    #[inline(always)]
    pub const fn set_min_mpdu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates the MII protocol supported"]
    #[must_use]
    #[inline(always)]
    pub const fn mii_prot(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the MII protocol supported"]
    #[inline(always)]
    pub const fn set_mii_prot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for L5mcapr {
    #[inline(always)]
    fn default() -> L5mcapr {
        L5mcapr(32u64 as u32)
    }
}
impl core::fmt::Debug for L5mcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L5mcapr")
            .field("mac_var", &self.mac_var())
            .field("efpad", &self.efpad())
            .field("pipg", &self.pipg())
            .field("hd", &self.hd())
            .field("fp", &self.fp())
            .field("min_mpdu", &self.min_mpdu())
            .field("mii_prot", &self.mii_prot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L5mcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L5mcapr {
            mac_var: u8,
            efpad: u8,
            pipg: bool,
            hd: bool,
            fp: u8,
            min_mpdu: bool,
            mii_prot: u8,
        }
        let proxy = L5mcapr {
            mac_var: self.mac_var(),
            efpad: self.efpad(),
            pipg: self.pipg(),
            hd: self.hd(),
            fp: self.fp(),
            min_mpdu: self.min_mpdu(),
            mii_prot: self.mii_prot(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Link 5 transmit byte credit comfort threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct L5txbcctr(pub u32);
impl L5txbcctr {
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Link's transmit byte credit comfort threshold from ETM towards port. Default value is 512B."]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for L5txbcctr {
    #[inline(always)]
    fn default() -> L5txbcctr {
        L5txbcctr(512u64 as u32)
    }
}
impl core::fmt::Debug for L5txbcctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L5txbcctr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for L5txbcctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct L5txbcctr {
            thresh: u16,
        }
        let proxy = L5txbcctr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC clock configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Netcclkcr(pub u32);
impl Netcclkcr {
    #[doc = "Frequency"]
    #[must_use]
    #[inline(always)]
    pub const fn freq(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Frequency"]
    #[inline(always)]
    pub const fn set_freq(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Period"]
    #[must_use]
    #[inline(always)]
    pub const fn period(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Period"]
    #[inline(always)]
    pub const fn set_period(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Netcclkcr {
    #[inline(always)]
    fn default() -> Netcclkcr {
        Netcclkcr(262384u64 as u32)
    }
}
impl core::fmt::Debug for Netcclkcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Netcclkcr")
            .field("freq", &self.freq())
            .field("period", &self.period())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Netcclkcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Netcclkcr {
            freq: u16,
            period: u16,
        }
        let proxy = Netcclkcr {
            freq: self.freq(),
            period: self.period(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "NETC FLR configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Netcflrcr(pub u32);
impl Netcflrcr {
    #[doc = "Time duration value expressed in SCALE units."]
    #[must_use]
    #[inline(always)]
    pub const fn value(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Time duration value expressed in SCALE units."]
    #[inline(always)]
    pub const fn set_value(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "Scale"]
    #[must_use]
    #[inline(always)]
    pub const fn scale(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Scale"]
    #[inline(always)]
    pub const fn set_scale(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
}
impl Default for Netcflrcr {
    #[inline(always)]
    fn default() -> Netcflrcr {
        Netcflrcr(1588u64 as u32)
    }
}
impl core::fmt::Debug for Netcflrcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Netcflrcr")
            .field("value", &self.value())
            .field("scale", &self.scale())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Netcflrcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Netcflrcr {
            value: u16,
            scale: u8,
        }
        let proxy = Netcflrcr {
            value: self.value(),
            scale: self.scale(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Root complex 0 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct R0bcr(pub u32);
impl R0bcr {
    #[doc = "Indicates the type of root complex and routing 0: RCiEP 1: PCIe RC"]
    #[must_use]
    #[inline(always)]
    pub const fn type_(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the type of root complex and routing 0: RCiEP 1: PCIe RC"]
    #[inline(always)]
    pub const fn set_type_(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port number Indicates how to address the PCIe target based on TYPE"]
    #[must_use]
    #[inline(always)]
    pub const fn port(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Port number Indicates how to address the PCIe target based on TYPE"]
    #[inline(always)]
    pub const fn set_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for R0bcr {
    #[inline(always)]
    fn default() -> R0bcr {
        R0bcr(0u64 as u32)
    }
}
impl core::fmt::Debug for R0bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R0bcr")
            .field("type_", &self.type_())
            .field("port", &self.port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for R0bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct R0bcr {
            type_: bool,
            port: u8,
        }
        let proxy = R0bcr {
            type_: self.type_(),
            port: self.port(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Root complex 0 MSI access management qualifier register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rc0msiamqr(pub u32);
impl Rc0msiamqr {
    #[doc = "Address Write QoS."]
    #[must_use]
    #[inline(always)]
    pub const fn awqos(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Write QoS."]
    #[inline(always)]
    pub const fn set_awqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[must_use]
    #[inline(always)]
    pub const fn bmt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[inline(always)]
    pub const fn set_bmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Rc0msiamqr {
    #[inline(always)]
    fn default() -> Rc0msiamqr {
        Rc0msiamqr(268435456u64 as u32)
    }
}
impl core::fmt::Debug for Rc0msiamqr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rc0msiamqr")
            .field("awqos", &self.awqos())
            .field("bmt", &self.bmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rc0msiamqr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rc0msiamqr {
            awqos: u8,
            bmt: bool,
        }
        let proxy = Rc0msiamqr {
            awqos: self.awqos(),
            bmt: self.bmt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Root complex 0 MSI-X cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rc0msicar(pub u32);
impl Rc0msicar {
    #[doc = "MSI-X write cache type This is the cache attribute setting used when NETC generates MSI-X events"]
    #[must_use]
    #[inline(always)]
    pub const fn msi_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "MSI-X write cache type This is the cache attribute setting used when NETC generates MSI-X events"]
    #[inline(always)]
    pub const fn set_msi_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "MSI-X write domain This is the domain attribute setting used when NETC generates MSI-X events"]
    #[must_use]
    #[inline(always)]
    pub const fn msi_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "MSI-X write domain This is the domain attribute setting used when NETC generates MSI-X events"]
    #[inline(always)]
    pub const fn set_msi_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "MSI-X write snoop This is the snoop attribute setting used when NETC generates MSI-X events"]
    #[must_use]
    #[inline(always)]
    pub const fn msi_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "MSI-X write snoop This is the snoop attribute setting used when NETC generates MSI-X events"]
    #[inline(always)]
    pub const fn set_msi_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Rc0msicar {
    #[inline(always)]
    fn default() -> Rc0msicar {
        Rc0msicar(2u64 as u32)
    }
}
impl core::fmt::Debug for Rc0msicar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rc0msicar")
            .field("msi_wrcache", &self.msi_wrcache())
            .field("msi_wrdomain", &self.msi_wrdomain())
            .field("msi_wrsnp", &self.msi_wrsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rc0msicar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rc0msicar {
            msi_wrcache: u8,
            msi_wrdomain: u8,
            msi_wrsnp: bool,
        }
        let proxy = Rc0msicar {
            msi_wrcache: self.msi_wrcache(),
            msi_wrdomain: self.msi_wrdomain(),
            msi_wrsnp: self.msi_wrsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 config header device ID and vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0CfhDidvid(pub u32);
impl S0CfhDidvid {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[inline(always)]
    pub const fn set_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[inline(always)]
    pub const fn set_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for S0CfhDidvid {
    #[inline(always)]
    fn default() -> S0CfhDidvid {
        S0CfhDidvid(4008843607u64 as u32)
    }
}
impl core::fmt::Debug for S0CfhDidvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0CfhDidvid")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0CfhDidvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0CfhDidvid {
            vendor_id: u16,
            device_id: u16,
        }
        let proxy = S0CfhDidvid {
            vendor_id: self.vendor_id(),
            device_id: self.device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 config header subsystem ID and subsystem vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0CfhSidsvid(pub u32);
impl S0CfhSidsvid {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[inline(always)]
    pub const fn set_subsystem_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[inline(always)]
    pub const fn set_subsystem_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for S0CfhSidsvid {
    #[inline(always)]
    fn default() -> S0CfhSidsvid {
        S0CfhSidsvid(4008843607u64 as u32)
    }
}
impl core::fmt::Debug for S0CfhSidsvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0CfhSidsvid")
            .field("subsystem_vendor_id", &self.subsystem_vendor_id())
            .field("subsystem_device_id", &self.subsystem_device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0CfhSidsvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0CfhSidsvid {
            subsystem_vendor_id: u16,
            subsystem_device_id: u16,
        }
        let proxy = S0CfhSidsvid {
            subsystem_vendor_id: self.subsystem_vendor_id(),
            subsystem_device_id: self.subsystem_device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 access management qualifier register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0amqr(pub u32);
impl S0amqr {
    #[doc = "Address Read QoS"]
    #[must_use]
    #[inline(always)]
    pub const fn arqos(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Read QoS"]
    #[inline(always)]
    pub const fn set_arqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Address Write QoS"]
    #[must_use]
    #[inline(always)]
    pub const fn awqos(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Write QoS"]
    #[inline(always)]
    pub const fn set_awqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[must_use]
    #[inline(always)]
    pub const fn bmt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[inline(always)]
    pub const fn set_bmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for S0amqr {
    #[inline(always)]
    fn default() -> S0amqr {
        S0amqr(301989888u64 as u32)
    }
}
impl core::fmt::Debug for S0amqr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0amqr")
            .field("arqos", &self.arqos())
            .field("awqos", &self.awqos())
            .field("bmt", &self.bmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0amqr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0amqr {
            arqos: u8,
            awqos: u8,
            bmt: bool,
        }
        let proxy = S0amqr {
            arqos: self.arqos(),
            awqos: self.awqos(),
            bmt: self.bmt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0bcr(pub u32);
impl S0bcr {
    #[doc = "Root complex instance number."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Root complex instance number."]
    #[inline(always)]
    pub const fn set_rc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set if switch instance is bounded to at least 1 link."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set if switch instance is bounded to at least 1 link."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for S0bcr {
    #[inline(always)]
    fn default() -> S0bcr {
        S0bcr(2147484160u64 as u32)
    }
}
impl core::fmt::Debug for S0bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0bcr")
            .field("rc_inst", &self.rc_inst())
            .field("fn_", &self.fn_())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0bcr {
            rc_inst: u8,
            fn_: u8,
            valid: bool,
        }
        let proxy = S0bcr {
            rc_inst: self.rc_inst(),
            fn_: self.fn_(),
            valid: self.valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 command cache attribute register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0ccar(pub u32);
impl S0ccar {
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when switch writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrcache(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor write cache type This is the cache attribute setting used when switch writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when switch writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrdomain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor write domain This is the domain attribute setting used when switch writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when switch writes the command buffer descriptor in memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_wrsnp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Command buffer descriptor write snoop This is the snoop attribute setting used when switch writes the command buffer descriptor in memory"]
    #[inline(always)]
    pub const fn set_cbd_wrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write data cache type This is the cache attribute setting used when switch writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrcache(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Write data cache type This is the cache attribute setting used when switch writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Write data domain This is the domain attribute setting used when switch writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrdomain(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Write data domain This is the domain attribute setting used when switch writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrdomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when switch writes command data to memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cwrsnp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Write data snoop This is the snoop attribute setting used when switch writes command data to memory"]
    #[inline(always)]
    pub const fn set_cwrsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when switch reads the command buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdcache(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Command buffer descriptor read cache type This is the cache attribute setting used when switch reads the command buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_cbd_rdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when switch reads the command buffer descriptor from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rddomain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Command buffer descriptor read domain This is the domain attribute setting used when switch reads the command buffer descriptor from memory"]
    #[inline(always)]
    pub const fn set_cbd_rddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[must_use]
    #[inline(always)]
    pub const fn cbd_rdsnp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Command descriptor read snoop See System Interface Read Transaction Attribute Definitions table in for valid settings"]
    #[inline(always)]
    pub const fn set_cbd_rdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Read data cache type This is the cache attribute setting used when switch reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crdcache(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Read data cache type This is the cache attribute setting used when switch reads command data from memory"]
    #[inline(always)]
    pub const fn set_crdcache(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Read data domain This is the domain attribute setting used when switch reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crddomain(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Read data domain This is the domain attribute setting used when switch reads command data from memory"]
    #[inline(always)]
    pub const fn set_crddomain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when switch reads command data from memory"]
    #[must_use]
    #[inline(always)]
    pub const fn crdsnp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Read data snoop This is the snoop attribute setting used when switch reads command data from memory"]
    #[inline(always)]
    pub const fn set_crdsnp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for S0ccar {
    #[inline(always)]
    fn default() -> S0ccar {
        S0ccar(33686018u64 as u32)
    }
}
impl core::fmt::Debug for S0ccar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0ccar")
            .field("cbd_wrcache", &self.cbd_wrcache())
            .field("cbd_wrdomain", &self.cbd_wrdomain())
            .field("cbd_wrsnp", &self.cbd_wrsnp())
            .field("cwrcache", &self.cwrcache())
            .field("cwrdomain", &self.cwrdomain())
            .field("cwrsnp", &self.cwrsnp())
            .field("cbd_rdcache", &self.cbd_rdcache())
            .field("cbd_rddomain", &self.cbd_rddomain())
            .field("cbd_rdsnp", &self.cbd_rdsnp())
            .field("crdcache", &self.crdcache())
            .field("crddomain", &self.crddomain())
            .field("crdsnp", &self.crdsnp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0ccar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0ccar {
            cbd_wrcache: u8,
            cbd_wrdomain: u8,
            cbd_wrsnp: bool,
            cwrcache: u8,
            cwrdomain: u8,
            cwrsnp: bool,
            cbd_rdcache: u8,
            cbd_rddomain: u8,
            cbd_rdsnp: bool,
            crdcache: u8,
            crddomain: u8,
            crdsnp: bool,
        }
        let proxy = S0ccar {
            cbd_wrcache: self.cbd_wrcache(),
            cbd_wrdomain: self.cbd_wrdomain(),
            cbd_wrsnp: self.cbd_wrsnp(),
            cwrcache: self.cwrcache(),
            cwrdomain: self.cwrdomain(),
            cwrsnp: self.cwrsnp(),
            cbd_rdcache: self.cbd_rdcache(),
            cbd_rddomain: self.cbd_rddomain(),
            cbd_rdsnp: self.cbd_rdsnp(),
            crdcache: self.crdcache(),
            crddomain: self.crddomain(),
            crdsnp: self.crdsnp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 frame modification data index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0fmditmar(pub u32);
impl S0fmditmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for S0fmditmar {
    #[inline(always)]
    fn default() -> S0fmditmar {
        S0fmditmar(256u64 as u32)
    }
}
impl core::fmt::Debug for S0fmditmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0fmditmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0fmditmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0fmditmar {
            num_words: u16,
        }
        let proxy = S0fmditmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 frame modification index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0fmitmar(pub u32);
impl S0fmitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for S0fmitmar {
    #[inline(always)]
    fn default() -> S0fmitmar {
        S0fmitmar(64u64 as u32)
    }
}
impl core::fmt::Debug for S0fmitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0fmitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0fmitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0fmitmar {
            num_words: u16,
        }
        let proxy = S0fmitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 hash table memory allotment register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0htmar(pub u32);
impl S0htmar {
    #[doc = "Maximum number of words allotted to the switch exact match hash tables from the common memory's shared region"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum number of words allotted to the switch exact match hash tables from the common memory's shared region"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for S0htmar {
    #[inline(always)]
    fn default() -> S0htmar {
        S0htmar(2241u64 as u32)
    }
}
impl core::fmt::Debug for S0htmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0htmar")
            .field("num_words", &self.num_words())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0htmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0htmar {
            num_words: u16,
            mloc: u8,
        }
        let proxy = S0htmar {
            num_words: self.num_words(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 ingress port filter table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0ipftmar(pub u32);
impl S0ipftmar {
    #[doc = "Number of words allocated to Ingress Port Filter table from ingress port filter ternary memory"]
    #[must_use]
    #[inline(always)]
    pub const fn alloc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of words allocated to Ingress Port Filter table from ingress port filter ternary memory"]
    #[inline(always)]
    pub const fn set_alloc(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for S0ipftmar {
    #[inline(always)]
    fn default() -> S0ipftmar {
        S0ipftmar(140u64 as u32)
    }
}
impl core::fmt::Debug for S0ipftmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0ipftmar")
            .field("alloc", &self.alloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0ipftmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0ipftmar {
            alloc: u16,
        }
        let proxy = S0ipftmar {
            alloc: self.alloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 ingress stream counter index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0iscitmar(pub u32);
impl S0iscitmar {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for S0iscitmar {
    #[inline(always)]
    fn default() -> S0iscitmar {
        S0iscitmar(384u64 as u32)
    }
}
impl core::fmt::Debug for S0iscitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0iscitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0iscitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0iscitmar {
            num_words: u16,
        }
        let proxy = S0iscitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 ingress stream index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0isitmar(pub u32);
impl S0isitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for S0isitmar {
    #[inline(always)]
    fn default() -> S0isitmar {
        S0isitmar(384u64 as u32)
    }
}
impl core::fmt::Debug for S0isitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0isitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0isitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0isitmar {
            num_words: u16,
        }
        let proxy = S0isitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 ingress sequence generation index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0isqgitmar(pub u32);
impl S0isqgitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for S0isqgitmar {
    #[inline(always)]
    fn default() -> S0isqgitmar {
        S0isqgitmar(48u64 as u32)
    }
}
impl core::fmt::Debug for S0isqgitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0isqgitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0isqgitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0isqgitmar {
            num_words: u16,
        }
        let proxy = S0isqgitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0itmar(pub u32);
impl S0itmar {
    #[doc = "Number of words allocated to the switch's index table memory"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of words allocated to the switch's index table memory"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for S0itmar {
    #[inline(always)]
    fn default() -> S0itmar {
        S0itmar(1488u64 as u32)
    }
}
impl core::fmt::Debug for S0itmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0itmar")
            .field("num_words", &self.num_words())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0itmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0itmar {
            num_words: u16,
            mloc: u8,
        }
        let proxy = S0itmar {
            num_words: self.num_words(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 MSI-X configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0mcr(pub u32);
impl S0mcr {
    #[doc = "Number of MSI-X vectors supported for switch function"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of MSI-X vectors supported for switch function"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for S0mcr {
    #[inline(always)]
    fn default() -> S0mcr {
        S0mcr(5u64 as u32)
    }
}
impl core::fmt::Debug for S0mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0mcr")
            .field("num_msix", &self.num_msix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0mcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0mcr {
            num_msix: u8,
        }
        let proxy = S0mcr {
            num_msix: self.num_msix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 management port configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0mpcr(pub u32);
impl S0mpcr {
    #[doc = "Specifies the destination port for frames identified as management"]
    #[must_use]
    #[inline(always)]
    pub const fn port(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Specifies the destination port for frames identified as management"]
    #[inline(always)]
    pub const fn set_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for S0mpcr {
    #[inline(always)]
    fn default() -> S0mpcr {
        S0mpcr(4u64 as u32)
    }
}
impl core::fmt::Debug for S0mpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0mpcr")
            .field("port", &self.port())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0mpcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0mpcr {
            port: u8,
        }
        let proxy = S0mpcr { port: self.port() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 rate policer index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0rpitmar(pub u32);
impl S0rpitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for S0rpitmar {
    #[inline(always)]
    fn default() -> S0rpitmar {
        S0rpitmar(128u64 as u32)
    }
}
impl core::fmt::Debug for S0rpitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0rpitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0rpitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0rpitmar {
            num_words: u16,
        }
        let proxy = S0rpitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 stream gate control list index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0sgclitmar(pub u32);
impl S0sgclitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for S0sgclitmar {
    #[inline(always)]
    fn default() -> S0sgclitmar {
        S0sgclitmar(192u64 as u32)
    }
}
impl core::fmt::Debug for S0sgclitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0sgclitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0sgclitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0sgclitmar {
            num_words: u16,
        }
        let proxy = S0sgclitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 stream gate instance index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0sgiitmar(pub u32);
impl S0sgiitmar {
    #[doc = "The number of words from index table memory assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for S0sgiitmar {
    #[inline(always)]
    fn default() -> S0sgiitmar {
        S0sgiitmar(32u64 as u32)
    }
}
impl core::fmt::Debug for S0sgiitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0sgiitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0sgiitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0sgiitmar {
            num_words: u16,
        }
        let proxy = S0sgiitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 shared memory buffer allotment register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0smbar(pub u32);
impl S0smbar {
    #[doc = "Number of words allotted for the switch frame buffering memory"]
    #[must_use]
    #[inline(always)]
    pub const fn alloc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Number of words allotted for the switch frame buffering memory"]
    #[inline(always)]
    pub const fn set_alloc(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates memory location 0: Common memory 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location 0: Common memory 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for S0smbar {
    #[inline(always)]
    fn default() -> S0smbar {
        S0smbar(3138u64 as u32)
    }
}
impl core::fmt::Debug for S0smbar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0smbar")
            .field("alloc", &self.alloc())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0smbar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0smbar {
            alloc: u32,
            mloc: u8,
        }
        let proxy = S0smbar {
            alloc: self.alloc(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 time gate scheduling lookahead register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0tgslr(pub u32);
impl S0tgslr {
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time gate scheduler (at the frame scheduling timing point), to account for the time required to schedule and dequeue a frame"]
    #[must_use]
    #[inline(always)]
    pub const fn min_lookahead(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Minimum lookahead This field specifies the amount of time to advance the IEEE 1588 time scale used by the time gate scheduler (at the frame scheduling timing point), to account for the time required to schedule and dequeue a frame"]
    #[inline(always)]
    pub const fn set_min_lookahead(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for S0tgslr {
    #[inline(always)]
    fn default() -> S0tgslr {
        S0tgslr(100u64 as u32)
    }
}
impl core::fmt::Debug for S0tgslr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0tgslr")
            .field("min_lookahead", &self.min_lookahead())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0tgslr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0tgslr {
            min_lookahead: u32,
        }
        let proxy = S0tgslr {
            min_lookahead: self.min_lookahead(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 time gate scheduling table allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0tgstar(pub u32);
impl S0tgstar {
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the switch Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list of each switch port"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "This field specifies the number of words in the Time Gate Scheduling internal memory (TGSMCAPR) allocated to support the switch Time Gate Scheduling table, which in turn contains the administrative gate control list and the operational gate control list of each switch port"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for S0tgstar {
    #[inline(always)]
    fn default() -> S0tgstar {
        S0tgstar(1280u64 as u32)
    }
}
impl core::fmt::Debug for S0tgstar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0tgstar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0tgstar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0tgstar {
            num_words: u16,
        }
        let proxy = S0tgstar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 VLAN Filter (hash) table default entry configuration registers 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0vfhtdecr0(pub u32);
impl S0vfhtdecr0 {
    #[doc = "Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port n."]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port n."]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port n."]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port n."]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port n."]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port n."]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Spanning Tree Group Member ID Refer to the VLAN Filter table entry STG_ID field description, for more details"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_id(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Spanning Tree Group Member ID Refer to the VLAN Filter table entry STG_ID field description, for more details"]
    #[inline(always)]
    pub const fn set_stg_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "IP Multicast Filtering Enable Refer to the VLAN Filter table entry IPMFE field description, for more details"]
    #[must_use]
    #[inline(always)]
    pub const fn ipmfe(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "IP Multicast Filtering Enable Refer to the VLAN Filter table entry IPMFE field description, for more details"]
    #[inline(always)]
    pub const fn set_ipmfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "IP Multicast Flooding Enable If IP multicast filtering is performed (IPMFE = 1b, and the frame is identified as a multicast IP packet), and there was no match found, then the frame is forwarded according to this field"]
    #[must_use]
    #[inline(always)]
    pub const fn ipmfle(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "IP Multicast Flooding Enable If IP multicast filtering is performed (IPMFE = 1b, and the frame is identified as a multicast IP packet), and there was no match found, then the frame is forwarded according to this field"]
    #[inline(always)]
    pub const fn set_ipmfle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for S0vfhtdecr0 {
    #[inline(always)]
    fn default() -> S0vfhtdecr0 {
        S0vfhtdecr0(0u64 as u32)
    }
}
impl core::fmt::Debug for S0vfhtdecr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0vfhtdecr0")
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .field("stg_id", &self.stg_id())
            .field("ipmfe", &self.ipmfe())
            .field("ipmfle", &self.ipmfle())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0vfhtdecr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0vfhtdecr0 {
            port0: bool,
            port1: bool,
            port2: bool,
            port3: bool,
            port4: bool,
            stg_id: u8,
            ipmfe: bool,
            ipmfle: bool,
        }
        let proxy = S0vfhtdecr0 {
            port0: self.port0(),
            port1: self.port1(),
            port2: self.port2(),
            port3: self.port3(),
            port4: self.port4(),
            stg_id: self.stg_id(),
            ipmfe: self.ipmfe(),
            ipmfle: self.ipmfle(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 VLAN filter hash table default entry configuration registers 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0vfhtdecr1(pub u32);
impl S0vfhtdecr1 {
    #[doc = "Filtering ID"]
    #[must_use]
    #[inline(always)]
    pub const fn fid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Filtering ID"]
    #[inline(always)]
    pub const fn set_fid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "VLAN Learning Mode 0: Independent VLAN learning: FID is set to the VID assigned to the frame 1: Shared VLAN learning: Use the FID specified in this register Used to determine the FID when doing a lookup in the FDB table"]
    #[must_use]
    #[inline(always)]
    pub const fn vl_mode(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Learning Mode 0: Independent VLAN learning: FID is set to the VID assigned to the frame 1: Shared VLAN learning: Use the FID specified in this register Used to determine the FID when doing a lookup in the FDB table"]
    #[inline(always)]
    pub const fn set_vl_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Base Egress Treatment Entry ID Refer to the VLAN Filter table entry BASE_ET_EID field description, for more details"]
    #[must_use]
    #[inline(always)]
    pub const fn base_eteid(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Base Egress Treatment Entry ID Refer to the VLAN Filter table entry BASE_ET_EID field description, for more details"]
    #[inline(always)]
    pub const fn set_base_eteid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for S0vfhtdecr1 {
    #[inline(always)]
    fn default() -> S0vfhtdecr1 {
        S0vfhtdecr1(4294901760u64 as u32)
    }
}
impl core::fmt::Debug for S0vfhtdecr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0vfhtdecr1")
            .field("fid", &self.fid())
            .field("vl_mode", &self.vl_mode())
            .field("base_eteid", &self.base_eteid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0vfhtdecr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0vfhtdecr1 {
            fid: u16,
            vl_mode: bool,
            base_eteid: u16,
        }
        let proxy = S0vfhtdecr1 {
            fid: self.fid(),
            vl_mode: self.vl_mode(),
            base_eteid: self.base_eteid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch 0 VLAN filter hash table default entry configuration registers 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct S0vfhtdecr2(pub u32);
impl S0vfhtdecr2 {
    #[doc = "Egress Treatment Applicability Port"]
    #[must_use]
    #[inline(always)]
    pub const fn es_port0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[inline(always)]
    pub const fn set_es_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[must_use]
    #[inline(always)]
    pub const fn es_port1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[inline(always)]
    pub const fn set_es_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[must_use]
    #[inline(always)]
    pub const fn es_port2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[inline(always)]
    pub const fn set_es_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[must_use]
    #[inline(always)]
    pub const fn es_port3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[inline(always)]
    pub const fn set_es_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[must_use]
    #[inline(always)]
    pub const fn es_port4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Treatment Applicability Port"]
    #[inline(always)]
    pub const fn set_es_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "MAC learning options: 0: Reserved 1: Disable MAC learning"]
    #[must_use]
    #[inline(always)]
    pub const fn mlo(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "MAC learning options: 0: Reserved 1: Disable MAC learning"]
    #[inline(always)]
    pub const fn set_mlo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "MAC forwarding options: 0: Reserved 1: No FDB lookup is performed, the frame is flooded 2: FDB lookup is performed, and if there is no match, the frame is flooded"]
    #[must_use]
    #[inline(always)]
    pub const fn mfo(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[doc = "MAC forwarding options: 0: Reserved 1: No FDB lookup is performed, the frame is flooded 2: FDB lookup is performed, and if there is no match, the frame is flooded"]
    #[inline(always)]
    pub const fn set_mfo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
}
impl Default for S0vfhtdecr2 {
    #[inline(always)]
    fn default() -> S0vfhtdecr2 {
        S0vfhtdecr2(301989888u64 as u32)
    }
}
impl core::fmt::Debug for S0vfhtdecr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S0vfhtdecr2")
            .field("es_port0", &self.es_port0())
            .field("es_port1", &self.es_port1())
            .field("es_port2", &self.es_port2())
            .field("es_port3", &self.es_port3())
            .field("es_port4", &self.es_port4())
            .field("mlo", &self.mlo())
            .field("mfo", &self.mfo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for S0vfhtdecr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct S0vfhtdecr2 {
            es_port0: bool,
            es_port1: bool,
            es_port2: bool,
            es_port3: bool,
            es_port4: bool,
            mlo: u8,
            mfo: u8,
        }
        let proxy = S0vfhtdecr2 {
            es_port0: self.es_port0(),
            es_port1: self.es_port1(),
            es_port2: self.es_port2(),
            es_port3: self.es_port3(),
            es_port4: self.es_port4(),
            mlo: self.mlo(),
            mfo: self.mfo(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "System bus configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbcr(pub u32);
impl Sbcr {
    #[doc = "System Bus Maximum Write Burst Size"]
    #[must_use]
    #[inline(always)]
    pub const fn wbs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "System Bus Maximum Write Burst Size"]
    #[inline(always)]
    pub const fn set_wbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "System Bus Maximum Read Burst Size"]
    #[must_use]
    #[inline(always)]
    pub const fn rbs(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "System Bus Maximum Read Burst Size"]
    #[inline(always)]
    pub const fn set_rbs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
}
impl Default for Sbcr {
    #[inline(always)]
    fn default() -> Sbcr {
        Sbcr(10u64 as u32)
    }
}
impl core::fmt::Debug for Sbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sbcr")
            .field("wbs", &self.wbs())
            .field("rbs", &self.rbs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sbcr {
            wbs: u8,
            rbs: u8,
        }
        let proxy = Sbcr {
            wbs: self.wbs(),
            rbs: self.rbs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stream gating lag time for refresh register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sglttr(pub u32);
impl Sglttr {
    #[doc = "Lag time is a nanosecond value = 2LAG_TIME"]
    #[must_use]
    #[inline(always)]
    pub const fn lag_time(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Lag time is a nanosecond value = 2LAG_TIME"]
    #[inline(always)]
    pub const fn set_lag_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Sglttr {
    #[inline(always)]
    fn default() -> Sglttr {
        Sglttr(20u64 as u32)
    }
}
impl core::fmt::Debug for Sglttr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sglttr")
            .field("lag_time", &self.lag_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sglttr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sglttr {
            lag_time: u8,
        }
        let proxy = Sglttr {
            lag_time: self.lag_time(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Shared memory depletion threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smdtr(pub u32);
impl Smdtr {
    #[doc = "Shared memory depletion threshold in Words"]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Shared memory depletion threshold in Words"]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Smdtr {
    #[inline(always)]
    fn default() -> Smdtr {
        Smdtr(64u64 as u32)
    }
}
impl core::fmt::Debug for Smdtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smdtr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smdtr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smdtr {
            thresh: u32,
        }
        let proxy = Smdtr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer 0 config header device ID and vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0CfhDidvid(pub u32);
impl T0CfhDidvid {
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[must_use]
    #[inline(always)]
    pub const fn vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Vendor ID This field identifies the manufacturer of the device as shown in the PCIe Vendor ID Register (00h)"]
    #[inline(always)]
    pub const fn set_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[must_use]
    #[inline(always)]
    pub const fn device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Device ID This field identifies the device ID of the device shown in the PCIe Device ID Register (02h)"]
    #[inline(always)]
    pub const fn set_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for T0CfhDidvid {
    #[inline(always)]
    fn default() -> T0CfhDidvid {
        T0CfhDidvid(3993114967u64 as u32)
    }
}
impl core::fmt::Debug for T0CfhDidvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0CfhDidvid")
            .field("vendor_id", &self.vendor_id())
            .field("device_id", &self.device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T0CfhDidvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct T0CfhDidvid {
            vendor_id: u16,
            device_id: u16,
        }
        let proxy = T0CfhDidvid {
            vendor_id: self.vendor_id(),
            device_id: self.device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer 0 config header subsystem ID and subsystem vendor ID register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0CfhSidsvid(pub u32);
impl T0CfhSidsvid {
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_vendor_id(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Vendor ID This field identifies the manufacturer of the subsystem as shown in the PCIe Subsystem Vendor ID Register (2Ch)"]
    #[inline(always)]
    pub const fn set_subsystem_vendor_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[must_use]
    #[inline(always)]
    pub const fn subsystem_device_id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Subsystem Device ID This field identifies the particular subsystem as shown in the PCIe Subsystem ID Register (2Eh)"]
    #[inline(always)]
    pub const fn set_subsystem_device_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for T0CfhSidsvid {
    #[inline(always)]
    fn default() -> T0CfhSidsvid {
        T0CfhSidsvid(3993114967u64 as u32)
    }
}
impl core::fmt::Debug for T0CfhSidsvid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0CfhSidsvid")
            .field("subsystem_vendor_id", &self.subsystem_vendor_id())
            .field("subsystem_device_id", &self.subsystem_device_id())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T0CfhSidsvid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct T0CfhSidsvid {
            subsystem_vendor_id: u16,
            subsystem_device_id: u16,
        }
        let proxy = T0CfhSidsvid {
            subsystem_vendor_id: self.subsystem_vendor_id(),
            subsystem_device_id: self.subsystem_device_id(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer 0 binding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0bcr(pub u32);
impl T0bcr {
    #[doc = "Root complex instance number."]
    #[must_use]
    #[inline(always)]
    pub const fn rc_inst(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Root complex instance number."]
    #[inline(always)]
    pub const fn set_rc_inst(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    #[must_use]
    #[inline(always)]
    pub const fn fn_(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "PCI device function number. For assignment of function number, see ."]
    #[inline(always)]
    pub const fn set_fn_(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "If set, this timer function is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If set, this timer function is valid."]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for T0bcr {
    #[inline(always)]
    fn default() -> T0bcr {
        T0bcr(2147483648u64 as u32)
    }
}
impl core::fmt::Debug for T0bcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0bcr")
            .field("rc_inst", &self.rc_inst())
            .field("fn_", &self.fn_())
            .field("valid", &self.valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T0bcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct T0bcr {
            rc_inst: u8,
            fn_: u8,
            valid: bool,
        }
        let proxy = T0bcr {
            rc_inst: self.rc_inst(),
            fn_: self.fn_(),
            valid: self.valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Timer 0 MSI-X configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0mcr(pub u32);
impl T0mcr {
    #[doc = "Number of MSI-X vectors supported for timer function"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Number of MSI-X vectors supported for timer function"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for T0mcr {
    #[inline(always)]
    fn default() -> T0mcr {
        T0mcr(0u64 as u32)
    }
}
impl core::fmt::Debug for T0mcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0mcr")
            .field("num_msix", &self.num_msix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for T0mcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct T0mcr {
            num_msix: bool,
        }
        let proxy = T0mcr {
            num_msix: self.num_msix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Time gate scheduling memory capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tgsmcapr(pub u32);
impl Tgsmcapr {
    #[doc = "Total amount of Time Gate Scheduling memory in words available to NETC"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total amount of Time Gate Scheduling memory in words available to NETC"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tgsmcapr {
    #[inline(always)]
    fn default() -> Tgsmcapr {
        Tgsmcapr(1792u64 as u32)
    }
}
impl core::fmt::Debug for Tgsmcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tgsmcapr")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tgsmcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tgsmcapr {
            num_words: u16,
        }
        let proxy = Tgsmcapr {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VSI 0 access management qualifier register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct V0amqr(pub u32);
impl V0amqr {
    #[doc = "Address Read QoS"]
    #[must_use]
    #[inline(always)]
    pub const fn arqos(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Read QoS"]
    #[inline(always)]
    pub const fn set_arqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Address Write QoS"]
    #[must_use]
    #[inline(always)]
    pub const fn awqos(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Address Write QoS"]
    #[inline(always)]
    pub const fn set_awqos(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[must_use]
    #[inline(always)]
    pub const fn bmt(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Bypass memory translation The bit is an indication to the SMMU to by-pass memory translation whenever the PF is performing memory transactions, effectively handling the memory address as a true physical address"]
    #[inline(always)]
    pub const fn set_bmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for V0amqr {
    #[inline(always)]
    fn default() -> V0amqr {
        V0amqr(301989888u64 as u32)
    }
}
impl core::fmt::Debug for V0amqr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("V0amqr")
            .field("arqos", &self.arqos())
            .field("awqos", &self.awqos())
            .field("bmt", &self.bmt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for V0amqr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct V0amqr {
            arqos: u8,
            awqos: u8,
            bmt: bool,
        }
        let proxy = V0amqr {
            arqos: self.arqos(),
            awqos: self.awqos(),
            bmt: self.bmt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VSI 0 primary MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct V0pmar1(pub u32);
impl V0pmar1 {
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for V0pmar1 {
    #[inline(always)]
    fn default() -> V0pmar1 {
        V0pmar1(0u64 as u32)
    }
}
impl core::fmt::Debug for V0pmar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("V0pmar1")
            .field("mac_addr", &self.mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for V0pmar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct V0pmar1 {
            mac_addr: u16,
        }
        let proxy = V0pmar1 {
            mac_addr: self.mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
