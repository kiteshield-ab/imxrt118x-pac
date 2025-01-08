#[doc = "ENETC capability register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecapr0(pub u32);
impl Ecapr0 {
    #[doc = "Receive flow steering support 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn rfs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive flow steering support 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_rfs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Support for time specific departure. 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn tsd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Support for time specific departure. 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_tsd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Support for RSS 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn rss(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Support for RSS 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_rss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Support for Wake-on-LAN in low-power mode 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn wo(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Support for Wake-on-LAN in low-power mode 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_wo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Functional safety capability supported."]
    #[must_use]
    #[inline(always)]
    pub const fn fs(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Functional safety capability supported."]
    #[inline(always)]
    pub const fn set_fs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Ecapr0 {
    #[inline(always)]
    fn default() -> Ecapr0 {
        Ecapr0(8224u64 as u32)
    }
}
impl core::fmt::Debug for Ecapr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecapr0")
            .field("rfs", &self.rfs())
            .field("tsd", &self.tsd())
            .field("rss", &self.rss())
            .field("wo", &self.wo())
            .field("fs", &self.fs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecapr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ecapr0 {
            rfs: bool,
            tsd: bool,
            rss: bool,
            wo: bool,
            fs: bool,
        }
        let proxy = Ecapr0 {
            rfs: self.rfs(),
            tsd: self.tsd(),
            rss: self.rss(),
            wo: self.wo(),
            fs: self.fs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC capability register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecapr1(pub u32);
impl Ecapr1 {
    #[doc = "Number of traffic classes 0 - 1 Traffic class (0) 1 - 2 Traffic classes (0-1)"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tcs(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Number of traffic classes 0 - 1 Traffic class (0) 1 - 2 Traffic classes (0-1)"]
    #[inline(always)]
    pub const fn set_num_tcs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Number of multi-cast hash entry (1bit/entry) per SI for L2 MAC Filtering 00: 64 multi-cast bins 01: 128 multi-cast bins 10: 256 multi-cast bins 11: 512 multi-cast bins"]
    #[must_use]
    #[inline(always)]
    pub const fn num_mch(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Number of multi-cast hash entry (1bit/entry) per SI for L2 MAC Filtering 00: 64 multi-cast bins 01: 128 multi-cast bins 10: 256 multi-cast bins 11: 512 multi-cast bins"]
    #[inline(always)]
    pub const fn set_num_mch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Number of uni-cast hash entry (1bit/entry) per SI for L2 MAC Filtering 00: 64 unicast bins 01: 128 unicast bins 10: 256 unicast bins 11: 512 unicast bins"]
    #[must_use]
    #[inline(always)]
    pub const fn num_uch(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Number of uni-cast hash entry (1bit/entry) per SI for L2 MAC Filtering 00: 64 unicast bins 01: 128 unicast bins 10: 256 unicast bins 11: 512 unicast bins"]
    #[inline(always)]
    pub const fn set_num_uch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Number of MSI-X"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x07ff;
        val as u16
    }
    #[doc = "Number of MSI-X"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 12usize)) | (((val as u32) & 0x07ff) << 12usize);
    }
    #[doc = "Indicates the number of VSI supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vsi(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the number of VSI supported"]
    #[inline(always)]
    pub const fn set_num_vsi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Indicates the number of IPVs supported. 0: 8 IPVs 1: 16 IPVs"]
    #[must_use]
    #[inline(always)]
    pub const fn num_ipv(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the number of IPVs supported. 0: 8 IPVs 1: 16 IPVs"]
    #[inline(always)]
    pub const fn set_num_ipv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ecapr1 {
    #[inline(always)]
    fn default() -> Ecapr1 {
        Ecapr1(45168u64 as u32)
    }
}
impl core::fmt::Debug for Ecapr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecapr1")
            .field("num_tcs", &self.num_tcs())
            .field("num_mch", &self.num_mch())
            .field("num_uch", &self.num_uch())
            .field("num_msix", &self.num_msix())
            .field("num_vsi", &self.num_vsi())
            .field("num_ipv", &self.num_ipv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecapr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ecapr1 {
            num_tcs: u8,
            num_mch: u8,
            num_uch: u8,
            num_msix: u16,
            num_vsi: u8,
            num_ipv: bool,
        }
        let proxy = Ecapr1 {
            num_tcs: self.num_tcs(),
            num_mch: self.num_mch(),
            num_uch: self.num_uch(),
            num_msix: self.num_msix(),
            num_vsi: self.num_vsi(),
            num_ipv: self.num_ipv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ENETC capability register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecapr2(pub u32);
impl Ecapr2 {
    #[doc = "Number of total transmit buffer descriptor rings assigned to ENETC Range: 0..1023"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tx_bdr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of total transmit buffer descriptor rings assigned to ENETC Range: 0..1023"]
    #[inline(always)]
    pub const fn set_num_tx_bdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Number of total receive buffer descriptor rings assigned to ENETC Range: 0..1023"]
    #[must_use]
    #[inline(always)]
    pub const fn num_rx_bdr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Number of total receive buffer descriptor rings assigned to ENETC Range: 0..1023"]
    #[inline(always)]
    pub const fn set_num_rx_bdr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for Ecapr2 {
    #[inline(always)]
    fn default() -> Ecapr2 {
        Ecapr2(262148u64 as u32)
    }
}
impl core::fmt::Debug for Ecapr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ecapr2")
            .field("num_tx_bdr", &self.num_tx_bdr())
            .field("num_rx_bdr", &self.num_rx_bdr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ecapr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ecapr2 {
            num_tx_bdr: u16,
            num_rx_bdr: u16,
        }
        let proxy = Ecapr2 {
            num_tx_bdr: self.num_tx_bdr(),
            num_rx_bdr: self.num_rx_bdr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive IPV to ICM priority mapping register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipv2icmpmr0(pub u32);
impl Ipv2icmpmr0 {
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv0icm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[inline(always)]
    pub const fn set_ipv0icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv1icm(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[inline(always)]
    pub const fn set_ipv1icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv2icm(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[inline(always)]
    pub const fn set_ipv2icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv3icm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[inline(always)]
    pub const fn set_ipv3icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv4icm(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[inline(always)]
    pub const fn set_ipv4icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv5icm(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[inline(always)]
    pub const fn set_ipv5icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv6icm(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[inline(always)]
    pub const fn set_ipv6icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv7icm(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal priority value (IPV) i={0"]
    #[inline(always)]
    pub const fn set_ipv7icm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Ipv2icmpmr0 {
    #[inline(always)]
    fn default() -> Ipv2icmpmr0 {
        Ipv2icmpmr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ipv2icmpmr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipv2icmpmr0")
            .field("ipv0icm", &self.ipv0icm())
            .field("ipv1icm", &self.ipv1icm())
            .field("ipv2icm", &self.ipv2icm())
            .field("ipv3icm", &self.ipv3icm())
            .field("ipv4icm", &self.ipv4icm())
            .field("ipv5icm", &self.ipv5icm())
            .field("ipv6icm", &self.ipv6icm())
            .field("ipv7icm", &self.ipv7icm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipv2icmpmr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipv2icmpmr0 {
            ipv0icm: bool,
            ipv1icm: bool,
            ipv2icm: bool,
            ipv3icm: bool,
            ipv4icm: bool,
            ipv5icm: bool,
            ipv6icm: bool,
            ipv7icm: bool,
        }
        let proxy = Ipv2icmpmr0 {
            ipv0icm: self.ipv0icm(),
            ipv1icm: self.ipv1icm(),
            ipv2icm: self.ipv2icm(),
            ipv3icm: self.ipv3icm(),
            ipv4icm: self.ipv4icm(),
            ipv5icm: self.ipv5icm(),
            ipv6icm: self.ipv6icm(),
            ipv7icm: self.ipv7icm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Parser custom Ethertype i configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parcecr(pub u32);
impl Parcecr {
    #[doc = "Code Point"]
    #[must_use]
    #[inline(always)]
    pub const fn cp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Code Point"]
    #[inline(always)]
    pub const fn set_cp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "ETYPE"]
    #[must_use]
    #[inline(always)]
    pub const fn etype(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "ETYPE"]
    #[inline(always)]
    pub const fn set_etype(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Parcecr {
    #[inline(always)]
    fn default() -> Parcecr {
        Parcecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Parcecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Parcecr")
            .field("cp", &self.cp())
            .field("en", &self.en())
            .field("etype", &self.etype())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Parcecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Parcecr {
            cp: u8,
            en: bool,
            etype: u16,
        }
        let proxy = Parcecr {
            cp: self.cp(),
            en: self.en(),
            etype: self.etype(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Parser checksum configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Parcscr(pub u32);
impl Parcscr {
    #[doc = "Layer 4 TCP and UDP checksum validation Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn l4cd(&self) -> super::vals::L4cd {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::L4cd::from_bits(val as u8)
    }
    #[doc = "Layer 4 TCP and UDP checksum validation Disable."]
    #[inline(always)]
    pub const fn set_l4cd(&mut self, val: super::vals::L4cd) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Layer 3 IPv4 Header checksum validation Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn l3cd(&self) -> super::vals::L3cd {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::L3cd::from_bits(val as u8)
    }
    #[doc = "Layer 3 IPv4 Header checksum validation Disable."]
    #[inline(always)]
    pub const fn set_l3cd(&mut self, val: super::vals::L3cd) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Parcscr {
    #[inline(always)]
    fn default() -> Parcscr {
        Parcscr(0u64 as u32)
    }
}
impl core::fmt::Debug for Parcscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Parcscr")
            .field("l4cd", &self.l4cd())
            .field("l3cd", &self.l3cd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Parcscr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Parcscr {
            l4cd: super::vals::L4cd,
            l3cd: super::vals::L3cd,
        }
        let proxy = Parcscr {
            l4cd: self.l4cd(),
            l3cd: self.l3cd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port ingress congestion priority discard status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Picpdsr(pub u32);
impl Picpdsr {
    #[doc = "DR0 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[must_use]
    #[inline(always)]
    pub const fn dr0_p0ds(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DR0 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[inline(always)]
    pub const fn set_dr0_p0ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DR0 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[must_use]
    #[inline(always)]
    pub const fn dr0_p1ds(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DR0 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[inline(always)]
    pub const fn set_dr0_p1ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DR1 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[must_use]
    #[inline(always)]
    pub const fn dr1_p0ds(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "DR1 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[inline(always)]
    pub const fn set_dr1_p0ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DR1 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[must_use]
    #[inline(always)]
    pub const fn dr1_p1ds(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "DR1 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[inline(always)]
    pub const fn set_dr1_p1ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DR2 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[must_use]
    #[inline(always)]
    pub const fn dr2_p0ds(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DR2 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[inline(always)]
    pub const fn set_dr2_p0ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DR2 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[must_use]
    #[inline(always)]
    pub const fn dr2_p1ds(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DR2 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[inline(always)]
    pub const fn set_dr2_p1ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DR3 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[must_use]
    #[inline(always)]
    pub const fn dr3_p0ds(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "DR3 priority 0 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[inline(always)]
    pub const fn set_dr3_p0ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "DR3 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[must_use]
    #[inline(always)]
    pub const fn dr3_p1ds(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "DR3 priority 1 discard status The bit will be set to 1 if one or more frames have been discarded at this DR and priority"]
    #[inline(always)]
    pub const fn set_dr3_p1ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Picpdsr {
    #[inline(always)]
    fn default() -> Picpdsr {
        Picpdsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Picpdsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Picpdsr")
            .field("dr0_p0ds", &self.dr0_p0ds())
            .field("dr0_p1ds", &self.dr0_p1ds())
            .field("dr1_p0ds", &self.dr1_p0ds())
            .field("dr1_p1ds", &self.dr1_p1ds())
            .field("dr2_p0ds", &self.dr2_p0ds())
            .field("dr2_p1ds", &self.dr2_p1ds())
            .field("dr3_p0ds", &self.dr3_p0ds())
            .field("dr3_p1ds", &self.dr3_p1ds())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Picpdsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Picpdsr {
            dr0_p0ds: bool,
            dr0_p1ds: bool,
            dr1_p0ds: bool,
            dr1_p1ds: bool,
            dr2_p0ds: bool,
            dr2_p1ds: bool,
            dr3_p0ds: bool,
            dr3_p1ds: bool,
        }
        let proxy = Picpdsr {
            dr0_p0ds: self.dr0_p0ds(),
            dr0_p1ds: self.dr0_p1ds(),
            dr1_p0ds: self.dr1_p0ds(),
            dr1_p1ds: self.dr1_p1ds(),
            dr2_p0ds: self.dr2_p0ds(),
            dr2_p1ds: self.dr2_p1ds(),
            dr3_p0ds: self.dr3_p0ds(),
            dr3_p1ds: self.dr3_p1ds(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port inner native VLAN register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pinvlanr(pub u32);
impl Pinvlanr {
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    #[must_use]
    #[inline(always)]
    pub const fn vid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    #[inline(always)]
    pub const fn set_vid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    #[must_use]
    #[inline(always)]
    pub const fn dei(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    #[inline(always)]
    pub const fn set_dei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    #[must_use]
    #[inline(always)]
    pub const fn pcp(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    #[inline(always)]
    pub const fn set_pcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Identifies which known VLAN Ethertype is used"]
    #[must_use]
    #[inline(always)]
    pub const fn tpid(&self) -> super::vals::PinvlanrTpid {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::PinvlanrTpid::from_bits(val as u8)
    }
    #[doc = "Identifies which known VLAN Ethertype is used"]
    #[inline(always)]
    pub const fn set_tpid(&mut self, val: super::vals::PinvlanrTpid) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Port Native VLAN Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pne(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Port Native VLAN Enable"]
    #[inline(always)]
    pub const fn set_pne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VID 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vze(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "VID 0 Enable"]
    #[inline(always)]
    pub const fn set_vze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Pinvlanr {
    #[inline(always)]
    fn default() -> Pinvlanr {
        Pinvlanr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pinvlanr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pinvlanr")
            .field("vid", &self.vid())
            .field("dei", &self.dei())
            .field("pcp", &self.pcp())
            .field("tpid", &self.tpid())
            .field("pne", &self.pne())
            .field("vze", &self.vze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pinvlanr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pinvlanr {
            vid: u16,
            dei: bool,
            pcp: u8,
            tpid: super::vals::PinvlanrTpid,
            pne: bool,
            vze: bool,
        }
        let proxy = Pinvlanr {
            vid: self.vid(),
            dei: self.dei(),
            pcp: self.pcp(),
            tpid: self.tpid(),
            pne: self.pne(),
            vze: self.vze(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port low power mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plpmr(pub u32);
impl Plpmr {
    #[doc = "Wake-on-LAN mode enable When Wake-on-LAN mode is enabled, ENETC will detect Wake-on-LAN events."]
    #[must_use]
    #[inline(always)]
    pub const fn wme(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-on-LAN mode enable When Wake-on-LAN mode is enabled, ENETC will detect Wake-on-LAN events."]
    #[inline(always)]
    pub const fn set_wme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Plpmr {
    #[inline(always)]
    fn default() -> Plpmr {
        Plpmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Plpmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Plpmr").field("wme", &self.wme()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Plpmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Plpmr {
            wme: bool,
        }
        let proxy = Plpmr { wme: self.wme() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmr(pub u32);
impl Pmr {
    #[doc = "Enable station interface 0"]
    #[must_use]
    #[inline(always)]
    pub const fn si0en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enable station interface 0"]
    #[inline(always)]
    pub const fn set_si0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Pmr {
    #[inline(always)]
    fn default() -> Pmr {
        Pmr(65536u64 as u32)
    }
}
impl core::fmt::Debug for Pmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmr").field("si0en", &self.si0en()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pmr {
            si0en: bool,
        }
        let proxy = Pmr {
            si0en: self.si0en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port outer native VLAN register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ponvlanr(pub u32);
impl Ponvlanr {
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    #[must_use]
    #[inline(always)]
    pub const fn vid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    #[inline(always)]
    pub const fn set_vid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    #[must_use]
    #[inline(always)]
    pub const fn dei(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    #[inline(always)]
    pub const fn set_dei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    #[must_use]
    #[inline(always)]
    pub const fn pcp(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    #[inline(always)]
    pub const fn set_pcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Identifies which known VLAN Ethertype is used"]
    #[must_use]
    #[inline(always)]
    pub const fn tpid(&self) -> super::vals::PonvlanrTpid {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::PonvlanrTpid::from_bits(val as u8)
    }
    #[doc = "Identifies which known VLAN Ethertype is used"]
    #[inline(always)]
    pub const fn set_tpid(&mut self, val: super::vals::PonvlanrTpid) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Port Native VLAN Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn pne(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Port Native VLAN Enable"]
    #[inline(always)]
    pub const fn set_pne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "VID 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vze(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "VID 0 Enable"]
    #[inline(always)]
    pub const fn set_vze(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Ponvlanr {
    #[inline(always)]
    fn default() -> Ponvlanr {
        Ponvlanr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ponvlanr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ponvlanr")
            .field("vid", &self.vid())
            .field("dei", &self.dei())
            .field("pcp", &self.pcp())
            .field("tpid", &self.tpid())
            .field("pne", &self.pne())
            .field("vze", &self.vze())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ponvlanr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ponvlanr {
            vid: u16,
            dei: bool,
            pcp: u8,
            tpid: super::vals::PonvlanrTpid,
            pne: bool,
            vze: bool,
        }
        let proxy = Ponvlanr {
            vid: self.vid(),
            dei: self.dei(),
            pcp: self.pcp(),
            tpid: self.tpid(),
            pne: self.pne(),
            vze: self.vze(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port pause OFF threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppauofftr(pub u32);
impl Ppauofftr {
    #[doc = "Monitors the amount of data accumulated in the receive buffer and if this amount goes below the Pause OFF threshold (expressed in words) it then enters the \"Pause OFF\" state if Pause is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Monitors the amount of data accumulated in the receive buffer and if this amount goes below the Pause OFF threshold (expressed in words) it then enters the \"Pause OFF\" state if Pause is enabled"]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ppauofftr {
    #[inline(always)]
    fn default() -> Ppauofftr {
        Ppauofftr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppauofftr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppauofftr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppauofftr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppauofftr {
            thresh: u32,
        }
        let proxy = Ppauofftr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port pause ON threshold register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppauontr(pub u32);
impl Ppauontr {
    #[doc = "Monitors the amount of data accumulated in the receive buffer and if this amount exceeds the Pause ON threshold (expressed in words), it then enters the \"Pause ON\" state if Pause is enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn thresh(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Monitors the amount of data accumulated in the receive buffer and if this amount exceeds the Pause ON threshold (expressed in words), it then enters the \"Pause ON\" state if Pause is enabled"]
    #[inline(always)]
    pub const fn set_thresh(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Ppauontr {
    #[inline(always)]
    fn default() -> Ppauontr {
        Ppauontr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppauontr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppauontr")
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppauontr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppauontr {
            thresh: u32,
        }
        let proxy = Ppauontr {
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Transmit priority to traffic class mapping register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prio2tcmr0(pub u32);
impl Prio2tcmr0 {
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn prio0tc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[inline(always)]
    pub const fn set_prio0tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn prio1tc(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[inline(always)]
    pub const fn set_prio1tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn prio2tc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[inline(always)]
    pub const fn set_prio2tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn prio3tc(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[inline(always)]
    pub const fn set_prio3tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn prio4tc(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[inline(always)]
    pub const fn set_prio4tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn prio5tc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[inline(always)]
    pub const fn set_prio5tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn prio6tc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[inline(always)]
    pub const fn set_prio6tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[must_use]
    #[inline(always)]
    pub const fn prio7tc(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit BD ring priority to traffic class mapping"]
    #[inline(always)]
    pub const fn set_prio7tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Prio2tcmr0 {
    #[inline(always)]
    fn default() -> Prio2tcmr0 {
        Prio2tcmr0(1985229328u64 as u32)
    }
}
impl core::fmt::Debug for Prio2tcmr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prio2tcmr0")
            .field("prio0tc", &self.prio0tc())
            .field("prio1tc", &self.prio1tc())
            .field("prio2tc", &self.prio2tc())
            .field("prio3tc", &self.prio3tc())
            .field("prio4tc", &self.prio4tc())
            .field("prio5tc", &self.prio5tc())
            .field("prio6tc", &self.prio6tc())
            .field("prio7tc", &self.prio7tc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prio2tcmr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prio2tcmr0 {
            prio0tc: u8,
            prio1tc: u8,
            prio2tc: u8,
            prio3tc: u8,
            prio4tc: u8,
            prio5tc: u8,
            prio6tc: u8,
            prio7tc: u8,
        }
        let proxy = Prio2tcmr0 {
            prio0tc: self.prio0tc(),
            prio1tc: self.prio1tc(),
            prio2tc: self.prio2tc(),
            prio3tc: self.prio3tc(),
            prio4tc: self.prio4tc(),
            prio5tc: self.prio5tc(),
            prio6tc: self.prio6tc(),
            prio7tc: self.prio7tc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port receive buffer count high watermark register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prxbchwmr(pub u32);
impl Prxbchwmr {
    #[doc = "High watermark in words for receive buffer usage since the last read of this register"]
    #[must_use]
    #[inline(always)]
    pub const fn watermark(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "High watermark in words for receive buffer usage since the last read of this register"]
    #[inline(always)]
    pub const fn set_watermark(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Prxbchwmr {
    #[inline(always)]
    fn default() -> Prxbchwmr {
        Prxbchwmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Prxbchwmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prxbchwmr")
            .field("watermark", &self.watermark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prxbchwmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prxbchwmr {
            watermark: u32,
        }
        let proxy = Prxbchwmr {
            watermark: self.watermark(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port receive buffer count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prxbcr(pub u32);
impl Prxbcr {
    #[doc = "Current receive buffer usage count in words for this port."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Current receive buffer usage count in words for this port."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Prxbcr {
    #[inline(always)]
    fn default() -> Prxbcr {
        Prxbcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Prxbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prxbcr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prxbcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prxbcr {
            count: u32,
        }
        let proxy = Prxbcr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port receive memory buffer entitlement register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prxmber(pub u32);
impl Prxmber {
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
impl Default for Prxmber {
    #[inline(always)]
    fn default() -> Prxmber {
        Prxmber(0u64 as u32)
    }
}
impl core::fmt::Debug for Prxmber {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prxmber")
            .field("amount", &self.amount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prxmber {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prxmber {
            amount: u32,
        }
        let proxy = Prxmber {
            amount: self.amount(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port receive memory buffer limit register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prxmblr(pub u32);
impl Prxmblr {
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
impl Default for Prxmblr {
    #[inline(always)]
    fn default() -> Prxmblr {
        Prxmblr(0u64 as u32)
    }
}
impl core::fmt::Debug for Prxmblr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prxmblr")
            .field("limit", &self.limit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prxmblr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prxmblr {
            limit: u32,
        }
        let proxy = Prxmblr {
            limit: self.limit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface 0 configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi0cfgr0(pub u32);
impl Psi0cfgr0 {
    #[doc = "Number of transmit buffer descriptor rings assigned to the SI Range: 0"]
    #[must_use]
    #[inline(always)]
    pub const fn num_tx_bdr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of transmit buffer descriptor rings assigned to the SI Range: 0"]
    #[inline(always)]
    pub const fn set_num_tx_bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Source Pruning Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn spe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Source Pruning Enable"]
    #[inline(always)]
    pub const fn set_spe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "VLAN Tag Extract"]
    #[must_use]
    #[inline(always)]
    pub const fn vte(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Tag Extract"]
    #[inline(always)]
    pub const fn set_vte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SI-based VLAN Insertion Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn sivie(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "SI-based VLAN Insertion Enable"]
    #[inline(always)]
    pub const fn set_sivie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Anti-spoofing enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ase(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Anti-spoofing enable"]
    #[inline(always)]
    pub const fn set_ase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Number of receive buffer descriptor rings assigned to the SI Range: 0"]
    #[must_use]
    #[inline(always)]
    pub const fn num_rx_bdr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[doc = "Number of receive buffer descriptor rings assigned to the SI Range: 0"]
    #[inline(always)]
    pub const fn set_num_rx_bdr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[doc = "Station interface VLAN control Determines which VLAN Ethertypes can be inserted by the SI driver (e"]
    #[must_use]
    #[inline(always)]
    pub const fn sivc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Station interface VLAN control Determines which VLAN Ethertypes can be inserted by the SI driver (e"]
    #[inline(always)]
    pub const fn set_sivc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Station interface traffic class bandwidth weight Frames are selected for transmission between station interfaces based on a per traffic class basis using the Weighted Fair Bandwidth Sharing algorithm"]
    #[must_use]
    #[inline(always)]
    pub const fn sibw(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Station interface traffic class bandwidth weight Frames are selected for transmission between station interfaces based on a per traffic class basis using the Weighted Fair Bandwidth Sharing algorithm"]
    #[inline(always)]
    pub const fn set_sibw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Psi0cfgr0 {
    #[inline(always)]
    fn default() -> Psi0cfgr0 {
        Psi0cfgr0(262148u64 as u32)
    }
}
impl core::fmt::Debug for Psi0cfgr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psi0cfgr0")
            .field("num_tx_bdr", &self.num_tx_bdr())
            .field("spe", &self.spe())
            .field("vte", &self.vte())
            .field("sivie", &self.sivie())
            .field("ase", &self.ase())
            .field("num_rx_bdr", &self.num_rx_bdr())
            .field("sivc", &self.sivc())
            .field("sibw", &self.sibw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psi0cfgr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psi0cfgr0 {
            num_tx_bdr: u8,
            spe: bool,
            vte: bool,
            sivie: bool,
            ase: bool,
            num_rx_bdr: u8,
            sivc: u8,
            sibw: u8,
        }
        let proxy = Psi0cfgr0 {
            num_tx_bdr: self.num_tx_bdr(),
            spe: self.spe(),
            vte: self.vte(),
            sivie: self.sivie(),
            ase: self.ase(),
            num_rx_bdr: self.num_rx_bdr(),
            sivc: self.sivc(),
            sibw: self.sibw(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface 0 configuration register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi0cfgr2(pub u32);
impl Psi0cfgr2 {
    #[doc = "Number of MSI-X"]
    #[must_use]
    #[inline(always)]
    pub const fn num_msix(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of MSI-X"]
    #[inline(always)]
    pub const fn set_num_msix(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Psi0cfgr2 {
    #[inline(always)]
    fn default() -> Psi0cfgr2 {
        Psi0cfgr2(11u64 as u32)
    }
}
impl core::fmt::Debug for Psi0cfgr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psi0cfgr2")
            .field("num_msix", &self.num_msix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psi0cfgr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psi0cfgr2 {
            num_msix: u8,
        }
        let proxy = Psi0cfgr2 {
            num_msix: self.num_msix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface 0 primary MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi0pmar1(pub u32);
impl Psi0pmar1 {
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    #[must_use]
    #[inline(always)]
    pub const fn prim_mac_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Primary MAC address This field is defined in network byte order (big-endian)"]
    #[inline(always)]
    pub const fn set_prim_mac_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Psi0pmar1 {
    #[inline(always)]
    fn default() -> Psi0pmar1 {
        Psi0pmar1(0u64 as u32)
    }
}
impl core::fmt::Debug for Psi0pmar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psi0pmar1")
            .field("prim_mac_addr", &self.prim_mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psi0pmar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psi0pmar1 {
            prim_mac_addr: u16,
        }
        let proxy = Psi0pmar1 {
            prim_mac_addr: self.prim_mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface 0 VLAN filtering configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi0vlanfcfgr(pub u32);
impl Psi0vlanfcfgr {
    #[doc = "Number of VLAN filter table entries assigned to the SI"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vlan_fte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of VLAN filter table entries assigned to the SI"]
    #[inline(always)]
    pub const fn set_num_vlan_fte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Psi0vlanfcfgr {
    #[inline(always)]
    fn default() -> Psi0vlanfcfgr {
        Psi0vlanfcfgr(4u64 as u32)
    }
}
impl core::fmt::Debug for Psi0vlanfcfgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psi0vlanfcfgr")
            .field("num_vlan_fte", &self.num_vlan_fte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psi0vlanfcfgr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psi0vlanfcfgr {
            num_vlan_fte: u8,
        }
        let proxy = Psi0vlanfcfgr {
            num_vlan_fte: self.num_vlan_fte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface 0 VLAN register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi0vlanr(pub u32);
impl Psi0vlanr {
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    #[must_use]
    #[inline(always)]
    pub const fn vid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "VLAN identifier The VLAN identifier is a 12-bit field specifying the VLAN to which the frame belongs"]
    #[inline(always)]
    pub const fn set_vid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    #[must_use]
    #[inline(always)]
    pub const fn dei(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Drop eligible indicator May be used separately or in conjunction with PCP to indicate frames eligible to be dropped in the presence of congestion"]
    #[inline(always)]
    pub const fn set_dei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    #[must_use]
    #[inline(always)]
    pub const fn pcp(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Priority code point A 3-bit field which refers to the IEEE 802"]
    #[inline(always)]
    pub const fn set_pcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Tag protocol identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn tpid(&self) -> super::vals::Psi0vlanrTpid {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Psi0vlanrTpid::from_bits(val as u8)
    }
    #[doc = "Tag protocol identifier"]
    #[inline(always)]
    pub const fn set_tpid(&mut self, val: super::vals::Psi0vlanrTpid) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn e(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub const fn set_e(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Psi0vlanr {
    #[inline(always)]
    fn default() -> Psi0vlanr {
        Psi0vlanr(0u64 as u32)
    }
}
impl core::fmt::Debug for Psi0vlanr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psi0vlanr")
            .field("vid", &self.vid())
            .field("dei", &self.dei())
            .field("pcp", &self.pcp())
            .field("tpid", &self.tpid())
            .field("e", &self.e())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psi0vlanr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psi0vlanr {
            vid: u16,
            dei: bool,
            pcp: u8,
            tpid: super::vals::Psi0vlanrTpid,
            e: bool,
        }
        let proxy = Psi0vlanr {
            vid: self.vid(),
            dei: self.dei(),
            pcp: self.pcp(),
            tpid: self.tpid(),
            e: self.e(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface 0 VSI MAC address filtering configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi0vmafcfgr(pub u32);
impl Psi0vmafcfgr {
    #[doc = "Number of SI MAC address filter table entries assigned to the SI"]
    #[must_use]
    #[inline(always)]
    pub const fn num_mac_afte(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Number of SI MAC address filter table entries assigned to the SI"]
    #[inline(always)]
    pub const fn set_num_mac_afte(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Psi0vmafcfgr {
    #[inline(always)]
    fn default() -> Psi0vmafcfgr {
        Psi0vmafcfgr(4u64 as u32)
    }
}
impl core::fmt::Debug for Psi0vmafcfgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psi0vmafcfgr")
            .field("num_mac_afte", &self.num_mac_afte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psi0vmafcfgr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psi0vmafcfgr {
            num_mac_afte: u8,
        }
        let proxy = Psi0vmafcfgr {
            num_mac_afte: self.num_mac_afte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface MAC address filtering capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psimafcapr(pub u32);
impl Psimafcapr {
    #[doc = "Number of SI MAC address filter rules per port."]
    #[must_use]
    #[inline(always)]
    pub const fn num_mac_afte(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Number of SI MAC address filter rules per port."]
    #[inline(always)]
    pub const fn set_num_mac_afte(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Psimafcapr {
    #[inline(always)]
    fn default() -> Psimafcapr {
        Psimafcapr(4u64 as u32)
    }
}
impl core::fmt::Debug for Psimafcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psimafcapr")
            .field("num_mac_afte", &self.num_mac_afte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psimafcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psimafcapr {
            num_mac_afte: u16,
        }
        let proxy = Psimafcapr {
            num_mac_afte: self.num_mac_afte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface promiscuous MAC mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psipmmr(pub u32);
impl Psipmmr {
    #[doc = "SI 0 MAC unicast promiscuous"]
    #[must_use]
    #[inline(always)]
    pub const fn si0_mac_up(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SI 0 MAC unicast promiscuous"]
    #[inline(always)]
    pub const fn set_si0_mac_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SI 0 MAC multicast promiscuous"]
    #[must_use]
    #[inline(always)]
    pub const fn si0_mac_mp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SI 0 MAC multicast promiscuous"]
    #[inline(always)]
    pub const fn set_si0_mac_mp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Psipmmr {
    #[inline(always)]
    fn default() -> Psipmmr {
        Psipmmr(65537u64 as u32)
    }
}
impl core::fmt::Debug for Psipmmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psipmmr")
            .field("si0_mac_up", &self.si0_mac_up())
            .field("si0_mac_mp", &self.si0_mac_mp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psipmmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psipmmr {
            si0_mac_up: bool,
            si0_mac_mp: bool,
        }
        let proxy = Psipmmr {
            si0_mac_up: self.si0_mac_up(),
            si0_mac_mp: self.si0_mac_mp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface promiscuous VLAN mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psipvmr(pub u32);
impl Psipvmr {
    #[doc = "SI 0 VLAN promiscuous. This field specifies if SI 0 qualifies for the reception of all VLAN tags."]
    #[must_use]
    #[inline(always)]
    pub const fn si0_vlan_p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SI 0 VLAN promiscuous. This field specifies if SI 0 qualifies for the reception of all VLAN tags."]
    #[inline(always)]
    pub const fn set_si0_vlan_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SI 0 Untagged frames (no VLAN) accepted"]
    #[must_use]
    #[inline(always)]
    pub const fn si0_vlan_uta(&self) -> super::vals::Si0VlanUta {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Si0VlanUta::from_bits(val as u8)
    }
    #[doc = "SI 0 Untagged frames (no VLAN) accepted"]
    #[inline(always)]
    pub const fn set_si0_vlan_uta(&mut self, val: super::vals::Si0VlanUta) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
}
impl Default for Psipvmr {
    #[inline(always)]
    fn default() -> Psipvmr {
        Psipvmr(65537u64 as u32)
    }
}
impl core::fmt::Debug for Psipvmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psipvmr")
            .field("si0_vlan_p", &self.si0_vlan_p())
            .field("si0_vlan_uta", &self.si0_vlan_uta())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psipvmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psipvmr {
            si0_vlan_p: bool,
            si0_vlan_uta: super::vals::Si0VlanUta,
        }
        let proxy = Psipvmr {
            si0_vlan_p: self.si0_vlan_p(),
            si0_vlan_uta: self.si0_vlan_uta(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface VLAN filtering capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psivlanfcapr(pub u32);
impl Psivlanfcapr {
    #[doc = "Number of VLAN filter table entries per port. Range: 0..4K-1"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vlan_fte(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Number of VLAN filter table entries per port. Range: 0..4K-1"]
    #[inline(always)]
    pub const fn set_num_vlan_fte(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Psivlanfcapr {
    #[inline(always)]
    fn default() -> Psivlanfcapr {
        Psivlanfcapr(4u64 as u32)
    }
}
impl core::fmt::Debug for Psivlanfcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psivlanfcapr")
            .field("num_vlan_fte", &self.num_vlan_fte())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psivlanfcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psivlanfcapr {
            num_vlan_fte: u16,
        }
        let proxy = Psivlanfcapr {
            num_vlan_fte: self.num_vlan_fte(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port station interface VLAN filtering mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psivlanfmr(pub u32);
impl Psivlanfmr {
    #[doc = "VLAN tag select"]
    #[must_use]
    #[inline(always)]
    pub const fn vs(&self) -> super::vals::Vs {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Vs::from_bits(val as u8)
    }
    #[doc = "VLAN tag select"]
    #[inline(always)]
    pub const fn set_vs(&mut self, val: super::vals::Vs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Psivlanfmr {
    #[inline(always)]
    fn default() -> Psivlanfmr {
        Psivlanfmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Psivlanfmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psivlanfmr")
            .field("vs", &self.vs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psivlanfmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psivlanfmr {
            vs: super::vals::Vs,
        }
        let proxy = Psivlanfmr { vs: self.vs() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port traffic class a time specific departure register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptctsdr(pub u32);
impl Ptctsdr {
    #[doc = "Time specific Departure Enable The 1588 timer must be configured and enabled, and time gate scheduling for the port must be enabled (PTGSCR\\[TGE\\] set to 1), before enabling time specific departure on any traffic class"]
    #[must_use]
    #[inline(always)]
    pub const fn tsde(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Time specific Departure Enable The 1588 timer must be configured and enabled, and time gate scheduling for the port must be enabled (PTGSCR\\[TGE\\] set to 1), before enabling time specific departure on any traffic class"]
    #[inline(always)]
    pub const fn set_tsde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ptctsdr {
    #[inline(always)]
    fn default() -> Ptctsdr {
        Ptctsdr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptctsdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptctsdr")
            .field("tsde", &self.tsde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptctsdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptctsdr {
            tsde: bool,
        }
        let proxy = Ptctsdr { tsde: self.tsde() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port VLAN classification control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pvclctr(pub u32);
impl Pvclctr {
    #[doc = "Outer as Inner"]
    #[must_use]
    #[inline(always)]
    pub const fn oai(&self) -> super::vals::Oai {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Oai::from_bits(val as u8)
    }
    #[doc = "Outer as Inner"]
    #[inline(always)]
    pub const fn set_oai(&mut self, val: super::vals::Oai) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Pvclctr {
    #[inline(always)]
    fn default() -> Pvclctr {
        Pvclctr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pvclctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pvclctr").field("oai", &self.oai()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pvclctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pvclctr {
            oai: super::vals::Oai,
        }
        let proxy = Pvclctr { oai: self.oai() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port wake-on status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwosr(pub u32);
impl Pwosr {
    #[doc = "Wake-On-LAN active"]
    #[must_use]
    #[inline(always)]
    pub const fn wola(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake-On-LAN active"]
    #[inline(always)]
    pub const fn set_wola(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ICM blocked"]
    #[must_use]
    #[inline(always)]
    pub const fn icmb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ICM blocked"]
    #[inline(always)]
    pub const fn set_icmb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Pwosr {
    #[inline(always)]
    fn default() -> Pwosr {
        Pwosr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pwosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwosr")
            .field("wola", &self.wola())
            .field("icmb", &self.icmb())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwosr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pwosr {
            wola: bool,
            icmb: bool,
        }
        let proxy = Pwosr {
            wola: self.wola(),
            icmb: self.icmb(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Switch management capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smcapr(pub u32);
impl Smcapr {
    #[doc = "Switch Management"]
    #[must_use]
    #[inline(always)]
    pub const fn sm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Management"]
    #[inline(always)]
    pub const fn set_sm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Smcapr {
    #[inline(always)]
    fn default() -> Smcapr {
        Smcapr(0u64 as u32)
    }
}
impl core::fmt::Debug for Smcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Smcapr").field("sm", &self.sm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Smcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Smcapr {
            sm: bool,
        }
        let proxy = Smcapr { sm: self.sm() };
        defmt::write!(f, "{}", proxy)
    }
}
