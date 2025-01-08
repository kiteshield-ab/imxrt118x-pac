#[doc = "Correctable memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmecr(pub u32);
impl Cmecr {
    #[doc = "Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub const fn set_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cmecr {
    #[inline(always)]
    fn default() -> Cmecr {
        Cmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmecr")
            .field("threshold", &self.threshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmecr {
            threshold: u8,
        }
        let proxy = Cmecr {
            threshold: self.threshold(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Correctable memory error count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmectr(pub u32);
impl Cmectr {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cmectr {
    #[inline(always)]
    fn default() -> Cmectr {
        Cmectr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmectr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmectr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmectr {
            count: u8,
        }
        let proxy = Cmectr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Correctable memory error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmesr(pub u32);
impl Cmesr {
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Single-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Single-bit ECC error"]
    #[inline(always)]
    pub const fn set_sbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cmesr {
    #[inline(always)]
    fn default() -> Cmesr {
        Cmesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Cmesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmesr")
            .field("mem_id", &self.mem_id())
            .field("sbee", &self.sbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cmesr {
            mem_id: u8,
            sbee: bool,
        }
        let proxy = Cmesr {
            mem_id: self.mem_id(),
            sbee: self.sbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Custom VLAN Ethertype register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cvlanr1(pub u32);
impl Cvlanr1 {
    #[doc = "Ethertype"]
    #[must_use]
    #[inline(always)]
    pub const fn etype(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ethertype"]
    #[inline(always)]
    pub const fn set_etype(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn v(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cvlanr1 {
    #[inline(always)]
    fn default() -> Cvlanr1 {
        Cvlanr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Cvlanr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cvlanr1")
            .field("etype", &self.etype())
            .field("v", &self.v())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cvlanr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cvlanr1 {
            etype: u16,
            v: bool,
        }
        let proxy = Cvlanr1 {
            etype: self.etype(),
            v: self.v(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Custom VLAN Ethertype register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cvlanr2(pub u32);
impl Cvlanr2 {
    #[doc = "Ethertype"]
    #[must_use]
    #[inline(always)]
    pub const fn etype(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Ethertype"]
    #[inline(always)]
    pub const fn set_etype(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn v(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_v(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cvlanr2 {
    #[inline(always)]
    fn default() -> Cvlanr2 {
        Cvlanr2(0u64 as u32)
    }
}
impl core::fmt::Debug for Cvlanr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cvlanr2")
            .field("etype", &self.etype())
            .field("v", &self.v())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cvlanr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cvlanr2 {
            etype: u16,
            v: bool,
        }
        let proxy = Cvlanr2 {
            etype: self.etype(),
            v: self.v(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DoS L2 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dosl2cr(pub u32);
impl Dosl2cr {
    #[doc = "This field specifies whether received frames with SMAC = DMAC are discarded"]
    #[must_use]
    #[inline(always)]
    pub const fn sameaddr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies whether received frames with SMAC = DMAC are discarded"]
    #[inline(always)]
    pub const fn set_sameaddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This field specifies whether received frames with Multicast SMAC address are discarded"]
    #[must_use]
    #[inline(always)]
    pub const fn msamcc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This field specifies whether received frames with Multicast SMAC address are discarded"]
    #[inline(always)]
    pub const fn set_msamcc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Dosl2cr {
    #[inline(always)]
    fn default() -> Dosl2cr {
        Dosl2cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Dosl2cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dosl2cr")
            .field("sameaddr", &self.sameaddr())
            .field("msamcc", &self.msamcc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dosl2cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dosl2cr {
            sameaddr: bool,
            msamcc: bool,
        }
        let proxy = Dosl2cr {
            sameaddr: self.sameaddr(),
            msamcc: self.msamcc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Egress counter table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ectcapr(pub u32);
impl Ectcapr {
    #[doc = "The number of entries assigned to this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Ectcapr {
    #[inline(always)]
    fn default() -> Ectcapr {
        Ectcapr(1048960u64 as u32)
    }
}
impl core::fmt::Debug for Ectcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ectcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ectcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ectcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Ectcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "External MDIO interrupt reason register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emdioirr(pub u32);
impl Emdioirr {
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by EMDIO controller 1: A port interrupt is pending by EMDIO controller due to completion of a PHY MDIO access"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Emdioirr {
    #[inline(always)]
    fn default() -> Emdioirr {
        Emdioirr(0u64 as u32)
    }
}
impl core::fmt::Debug for Emdioirr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emdioirr")
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emdioirr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Emdioirr {
            port0: bool,
            port1: bool,
            port2: bool,
            port3: bool,
            port4: bool,
        }
        let proxy = Emdioirr {
            port0: self.port0(),
            port1: self.port1(),
            port2: self.port2(),
            port3: self.port3(),
            port4: self.port4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "External MDIO MSI-X vector register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emdiomsivr(pub u32);
impl Emdiomsivr {
    #[doc = "Index into MSI-X address/data table"]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Index into MSI-X address/data table"]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Emdiomsivr {
    #[inline(always)]
    fn default() -> Emdiomsivr {
        Emdiomsivr(0u64 as u32)
    }
}
impl core::fmt::Debug for Emdiomsivr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Emdiomsivr")
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Emdiomsivr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Emdiomsivr {
            vector: u8,
        }
        let proxy = Emdiomsivr {
            vector: self.vector(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Egress port capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epcapr(pub u32);
impl Epcapr {
    #[doc = "Egress Treatment function supported. 0: Not supported 1: Supported See ETCAPR for more information."]
    #[must_use]
    #[inline(always)]
    pub const fn et(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Treatment function supported. 0: Not supported 1: Supported See ETCAPR for more information."]
    #[inline(always)]
    pub const fn set_et(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates support for various PDU/SDUs (Protocol/Service Data Unit) definitions."]
    #[must_use]
    #[inline(always)]
    pub const fn sdu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates support for various PDU/SDUs (Protocol/Service Data Unit) definitions."]
    #[inline(always)]
    pub const fn set_sdu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Specifies the number of transmit QoS to VLAN PCP mapping profiles supported; see register QOSVLANMPaR0/1/2/3 where a={0"]
    #[must_use]
    #[inline(always)]
    pub const fn num_qvmp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the number of transmit QoS to VLAN PCP mapping profiles supported; see register QOSVLANMPaR0/1/2/3 where a={0"]
    #[inline(always)]
    pub const fn set_num_qvmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Epcapr {
    #[inline(always)]
    fn default() -> Epcapr {
        Epcapr(132865u64 as u32)
    }
}
impl core::fmt::Debug for Epcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Epcapr")
            .field("et", &self.et())
            .field("sdu", &self.sdu())
            .field("num_qvmp", &self.num_qvmp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Epcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Epcapr {
            et: bool,
            sdu: u8,
            num_qvmp: u8,
        }
        let proxy = Epcapr {
            et: self.et(),
            sdu: self.sdu(),
            num_qvmp: self.num_qvmp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Egress sequence recovery capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esqrcapr(pub u32);
impl Esqrcapr {
    #[doc = "Support Individual Recovery function and/or Sequence Recovery function x1: Individual Recovery function supported 1x: Sequence Recovery function supported"]
    #[must_use]
    #[inline(always)]
    pub const fn sqr_type(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Support Individual Recovery function and/or Sequence Recovery function x1: Individual Recovery function supported 1x: Sequence Recovery function supported"]
    #[inline(always)]
    pub const fn set_sqr_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Sequence Recovery Supported Algorithm. x1: Match Sequence Algorithm 1x: Vector Sequence Algorithm"]
    #[must_use]
    #[inline(always)]
    pub const fn sqr_alg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Sequence Recovery Supported Algorithm. x1: Match Sequence Algorithm 1x: Vector Sequence Algorithm"]
    #[inline(always)]
    pub const fn set_sqr_alg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Maximum history length capability used by sequence recovery function. 0: 64 1: 128 2-7: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn sqr_max_hl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Maximum history length capability used by sequence recovery function. 0: 64 1: 128 2-7: Reserved"]
    #[inline(always)]
    pub const fn set_sqr_max_hl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Esqrcapr {
    #[inline(always)]
    fn default() -> Esqrcapr {
        Esqrcapr(271u64 as u32)
    }
}
impl core::fmt::Debug for Esqrcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Esqrcapr")
            .field("sqr_type", &self.sqr_type())
            .field("sqr_alg", &self.sqr_alg())
            .field("sqr_max_hl", &self.sqr_max_hl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esqrcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Esqrcapr {
            sqr_type: u8,
            sqr_alg: u8,
            sqr_max_hl: u8,
        }
        let proxy = Esqrcapr {
            sqr_type: self.sqr_type(),
            sqr_alg: self.sqr_alg(),
            sqr_max_hl: self.sqr_max_hl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Egress sequence recovery table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esqrtcapr(pub u32);
impl Esqrtcapr {
    #[doc = "The number of entries assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table"]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Esqrtcapr {
    #[inline(always)]
    fn default() -> Esqrtcapr {
        Esqrtcapr(1048960u64 as u32)
    }
}
impl core::fmt::Debug for Esqrtcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Esqrtcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Esqrtcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Esqrtcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Esqrtcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Egress treatment capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Etcapr(pub u32);
impl Etcapr {
    #[doc = "Egress Sequence Recovery supported"]
    #[must_use]
    #[inline(always)]
    pub const fn esqr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Sequence Recovery supported"]
    #[inline(always)]
    pub const fn set_esqr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Etcapr {
    #[inline(always)]
    fn default() -> Etcapr {
        Etcapr(1u64 as u32)
    }
}
impl core::fmt::Debug for Etcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Etcapr")
            .field("esqr", &self.esqr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Etcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Etcapr {
            esqr: bool,
        }
        let proxy = Etcapr { esqr: self.esqr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Egress treatment table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ettcapr(pub u32);
impl Ettcapr {
    #[doc = "The number of entries assigned to this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Ettcapr {
    #[inline(always)]
    fn default() -> Ettcapr {
        Ettcapr(1048960u64 as u32)
    }
}
impl core::fmt::Debug for Ettcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ettcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ettcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ettcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Ettcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Egress treatment table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ettor(pub u32);
impl Ettor {
    #[doc = "Number of entries allocated / in-use by this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of entries allocated / in-use by this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ettor {
    #[inline(always)]
    fn default() -> Ettor {
        Ettor(0u64 as u32)
    }
}
impl core::fmt::Debug for Ettor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ettor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ettor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ettor {
            num_entries: u16,
        }
        let proxy = Ettor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Frame modification data index table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmditcapr(pub u32);
impl Fmditcapr {
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
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Fmditcapr {
    #[inline(always)]
    fn default() -> Fmditcapr {
        Fmditcapr(1048832u64 as u32)
    }
}
impl core::fmt::Debug for Fmditcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmditcapr")
            .field("num_words", &self.num_words())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmditcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fmditcapr {
            num_words: u16,
            access_meth: u8,
        }
        let proxy = Fmditcapr {
            num_words: self.num_words(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Frame modification data index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmditmar(pub u32);
impl Fmditmar {
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
impl Default for Fmditmar {
    #[inline(always)]
    fn default() -> Fmditmar {
        Fmditmar(256u64 as u32)
    }
}
impl core::fmt::Debug for Fmditmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmditmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmditmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fmditmar {
            num_words: u16,
        }
        let proxy = Fmditmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Frame modification egress capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmecapr(pub u32);
impl Fmecapr {
    #[doc = "Layer 2 frame modification actions"]
    #[must_use]
    #[inline(always)]
    pub const fn l2_act(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Layer 2 frame modification actions"]
    #[inline(always)]
    pub const fn set_l2_act(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Layer 3 frame modification actions"]
    #[must_use]
    #[inline(always)]
    pub const fn l3_act(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Layer 3 frame modification actions"]
    #[inline(always)]
    pub const fn set_l3_act(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Fmecapr {
    #[inline(always)]
    fn default() -> Fmecapr {
        Fmecapr(196655u64 as u32)
    }
}
impl core::fmt::Debug for Fmecapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmecapr")
            .field("l2_act", &self.l2_act())
            .field("l3_act", &self.l3_act())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmecapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fmecapr {
            l2_act: u8,
            l3_act: u8,
        }
        let proxy = Fmecapr {
            l2_act: self.l2_act(),
            l3_act: self.l3_act(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Frame modification ingress capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmicapr(pub u32);
impl Fmicapr {
    #[doc = "Layer 2 frame modification actions supported"]
    #[must_use]
    #[inline(always)]
    pub const fn l2_act(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Layer 2 frame modification actions supported"]
    #[inline(always)]
    pub const fn set_l2_act(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Fmicapr {
    #[inline(always)]
    fn default() -> Fmicapr {
        Fmicapr(10u64 as u32)
    }
}
impl core::fmt::Debug for Fmicapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmicapr")
            .field("l2_act", &self.l2_act())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmicapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fmicapr {
            l2_act: u8,
        }
        let proxy = Fmicapr {
            l2_act: self.l2_act(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Frame modification index table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmitcapr(pub u32);
impl Fmitcapr {
    #[doc = "The number of entries assigned to this table. The reset value is taken from FMITMAR\\[NUM_WORDS\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table. The reset value is taken from FMITMAR\\[NUM_WORDS\\]."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Fmitcapr {
    #[inline(always)]
    fn default() -> Fmitcapr {
        Fmitcapr(1048640u64 as u32)
    }
}
impl core::fmt::Debug for Fmitcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmitcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmitcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fmitcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Fmitcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Frame modification index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmitmar(pub u32);
impl Fmitmar {
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table. Each entry occupies 1 word."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Fmitmar {
    #[inline(always)]
    fn default() -> Fmitmar {
        Fmitmar(64u64 as u32)
    }
}
impl core::fmt::Debug for Fmitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fmitmar {
            num_words: u16,
        }
        let proxy = Fmitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Frame modification index table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmitor(pub u32);
impl Fmitor {
    #[doc = "The number of entries in-use by this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "The number of entries in-use by this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for Fmitor {
    #[inline(always)]
    fn default() -> Fmitor {
        Fmitor(0u64 as u32)
    }
}
impl core::fmt::Debug for Fmitor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fmitor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fmitor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fmitor {
            num_entries: u16,
        }
        let proxy = Fmitor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Hash table memory capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Htmcapr(pub u32);
impl Htmcapr {
    #[doc = "Maximum number of words allotted to exact match hash table from the common memory's shared region"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum number of words allotted to exact match hash table from the common memory's shared region"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Word size in bytes. 0: 24 bytes 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn word_size(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Word size in bytes. 0: 24 bytes 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_word_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Htmcapr {
    #[inline(always)]
    fn default() -> Htmcapr {
        Htmcapr(2241u64 as u32)
    }
}
impl core::fmt::Debug for Htmcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Htmcapr")
            .field("num_words", &self.num_words())
            .field("word_size", &self.word_size())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Htmcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Htmcapr {
            num_words: u16,
            word_size: u8,
            mloc: u8,
        }
        let proxy = Htmcapr {
            num_words: self.num_words(),
            word_size: self.word_size(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Hash table memory operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Htmor(pub u32);
impl Htmor {
    #[doc = "Number of Words in use by this function which has been allocated by the various hash tables."]
    #[must_use]
    #[inline(always)]
    pub const fn amount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of Words in use by this function which has been allocated by the various hash tables."]
    #[inline(always)]
    pub const fn set_amount(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "High WaterMark of Words allocated. Value reset to AMOUNT when read."]
    #[must_use]
    #[inline(always)]
    pub const fn watermark(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "High WaterMark of Words allocated. Value reset to AMOUNT when read."]
    #[inline(always)]
    pub const fn set_watermark(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Htmor {
    #[inline(always)]
    fn default() -> Htmor {
        Htmor(0u64 as u32)
    }
}
impl core::fmt::Debug for Htmor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Htmor")
            .field("amount", &self.amount())
            .field("watermark", &self.watermark())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Htmor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Htmor {
            amount: u16,
            watermark: u16,
        }
        let proxy = Htmor {
            amount: self.amount(),
            watermark: self.watermark(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Internal MDIO interrupt reason register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imdioirr(pub u32);
impl Imdioirr {
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0: No port interrupt pending by Ethernet MAC MDIO 1: A port interrupt is pending by Ethernet MAC MDIO due to completion of an MDIO access"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Imdioirr {
    #[inline(always)]
    fn default() -> Imdioirr {
        Imdioirr(0u64 as u32)
    }
}
impl core::fmt::Debug for Imdioirr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imdioirr")
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imdioirr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Imdioirr {
            port0: bool,
            port1: bool,
            port2: bool,
            port3: bool,
            port4: bool,
        }
        let proxy = Imdioirr {
            port0: self.port0(),
            port1: self.port1(),
            port2: self.port2(),
            port3: self.port3(),
            port4: self.port4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Internal MDIO MSI-X vector register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imdiomsivr(pub u32);
impl Imdiomsivr {
    #[doc = "Index into MSI-X address/data table"]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Index into MSI-X address/data table"]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Imdiomsivr {
    #[inline(always)]
    fn default() -> Imdiomsivr {
        Imdiomsivr(0u64 as u32)
    }
}
impl core::fmt::Debug for Imdiomsivr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imdiomsivr")
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imdiomsivr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Imdiomsivr {
            vector: u8,
        }
        let proxy = Imdiomsivr {
            vector: self.vector(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress port capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcapr(pub u32);
impl Ipcapr {
    #[doc = "Rate Policer function supported. 0: Not supported 1: Supported See RPCAPR for more information."]
    #[must_use]
    #[inline(always)]
    pub const fn rp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rate Policer function supported. 0: Not supported 1: Supported See RPCAPR for more information."]
    #[inline(always)]
    pub const fn set_rp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ingress Port Filtering supported (that is,: Ingress Port Filter table lookup)"]
    #[must_use]
    #[inline(always)]
    pub const fn ipflt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Port Filtering supported (that is,: Ingress Port Filter table lookup)"]
    #[inline(always)]
    pub const fn set_ipflt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Ingress Stream Identification functionality supported"]
    #[must_use]
    #[inline(always)]
    pub const fn isid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Stream Identification functionality supported"]
    #[inline(always)]
    pub const fn set_isid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates support for various PDU/SDUs (Protocol/Service Data Unit) definitions."]
    #[must_use]
    #[inline(always)]
    pub const fn sdu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Indicates support for various PDU/SDUs (Protocol/Service Data Unit) definitions."]
    #[inline(always)]
    pub const fn set_sdu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Specifies the number a of receive VLAN PCP/DE to QoS mapping profiles supported; see registers VLANIPVMPaR0/1 and VLANDRMPaR, where a={0"]
    #[must_use]
    #[inline(always)]
    pub const fn num_vqmp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Specifies the number a of receive VLAN PCP/DE to QoS mapping profiles supported; see registers VLANIPVMPaR0/1 and VLANDRMPaR, where a={0"]
    #[inline(always)]
    pub const fn set_num_vqmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Ipcapr {
    #[inline(always)]
    fn default() -> Ipcapr {
        Ipcapr(132871u64 as u32)
    }
}
impl core::fmt::Debug for Ipcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipcapr")
            .field("rp", &self.rp())
            .field("ipflt", &self.ipflt())
            .field("isid", &self.isid())
            .field("sdu", &self.sdu())
            .field("num_vqmp", &self.num_vqmp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipcapr {
            rp: bool,
            ipflt: bool,
            isid: bool,
            sdu: u8,
            num_vqmp: u8,
        }
        let proxy = Ipcapr {
            rp: self.rp(),
            ipflt: self.ipflt(),
            isid: self.isid(),
            sdu: self.sdu(),
            num_vqmp: self.num_vqmp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress port filter capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipfcapr(pub u32);
impl Ipfcapr {
    #[doc = "Rate Policer function supported"]
    #[must_use]
    #[inline(always)]
    pub const fn rp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Rate Policer function supported"]
    #[inline(always)]
    pub const fn set_rp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ingress Stream Identification supported."]
    #[must_use]
    #[inline(always)]
    pub const fn isid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Stream Identification supported."]
    #[inline(always)]
    pub const fn set_isid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Ipfcapr {
    #[inline(always)]
    fn default() -> Ipfcapr {
        Ipfcapr(3u64 as u32)
    }
}
impl core::fmt::Debug for Ipfcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipfcapr")
            .field("rp", &self.rp())
            .field("isid", &self.isid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipfcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipfcapr {
            rp: bool,
            isid: bool,
        }
        let proxy = Ipfcapr {
            rp: self.rp(),
            isid: self.isid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress port filter table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipftcapr(pub u32);
impl Ipftcapr {
    #[doc = "Number of ternary memory words supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of ternary memory words supported"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates if table entries are managed by software driver or by hardware"]
    #[must_use]
    #[inline(always)]
    pub const fn mgmt(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if table entries are managed by software driver or by hardware"]
    #[inline(always)]
    pub const fn set_mgmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates which Configuration Access Methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which Configuration Access Methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Maximum number of consecutive words which can form a TM Entry"]
    #[must_use]
    #[inline(always)]
    pub const fn entry_max_words(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Maximum number of consecutive words which can form a TM Entry"]
    #[inline(always)]
    pub const fn set_entry_max_words(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Word size in bits of the ternary memory. 0: 48 bits 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn word_size(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Word size in bits of the ternary memory. 0: 48 bits 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_word_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for Ipftcapr {
    #[inline(always)]
    fn default() -> Ipftcapr {
        Ipftcapr(248578188u64 as u32)
    }
}
impl core::fmt::Debug for Ipftcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipftcapr")
            .field("num_words", &self.num_words())
            .field("mgmt", &self.mgmt())
            .field("access_meth", &self.access_meth())
            .field("entry_max_words", &self.entry_max_words())
            .field("word_size", &self.word_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipftcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipftcapr {
            num_words: u16,
            mgmt: bool,
            access_meth: u8,
            entry_max_words: u8,
            word_size: u8,
        }
        let proxy = Ipftcapr {
            num_words: self.num_words(),
            mgmt: self.mgmt(),
            access_meth: self.access_meth(),
            entry_max_words: self.entry_max_words(),
            word_size: self.word_size(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress port filter table memory operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipftmor(pub u32);
impl Ipftmor {
    #[doc = "Number of words in-use."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of words in-use."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ipftmor {
    #[inline(always)]
    fn default() -> Ipftmor {
        Ipftmor(0u64 as u32)
    }
}
impl core::fmt::Debug for Ipftmor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ipftmor")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ipftmor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ipftmor {
            num_words: u16,
        }
        let proxy = Ipftmor {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iscapr(pub u32);
impl Iscapr {
    #[doc = "Ingress Sequence Generation specification supported"]
    #[must_use]
    #[inline(always)]
    pub const fn isqg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Sequence Generation specification supported"]
    #[inline(always)]
    pub const fn set_isqg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Stream Gating specification is supported"]
    #[must_use]
    #[inline(always)]
    pub const fn sg(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Stream Gating specification is supported"]
    #[inline(always)]
    pub const fn set_sg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rate Policer function specification supported"]
    #[must_use]
    #[inline(always)]
    pub const fn rp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rate Policer function specification supported"]
    #[inline(always)]
    pub const fn set_rp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Maximum SDU check supported"]
    #[must_use]
    #[inline(always)]
    pub const fn maxsdu(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Maximum SDU check supported"]
    #[inline(always)]
    pub const fn set_maxsdu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "When set, can specify a set of destination to forward the frame."]
    #[must_use]
    #[inline(always)]
    pub const fn fwd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "When set, can specify a set of destination to forward the frame."]
    #[inline(always)]
    pub const fn set_fwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Egress Treatment table entries specification supported. 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn et(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Treatment table entries specification supported. 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_et(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Iscapr {
    #[inline(always)]
    fn default() -> Iscapr {
        Iscapr(1594u64 as u32)
    }
}
impl core::fmt::Debug for Iscapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iscapr")
            .field("isqg", &self.isqg())
            .field("sg", &self.sg())
            .field("rp", &self.rp())
            .field("maxsdu", &self.maxsdu())
            .field("fwd", &self.fwd())
            .field("et", &self.et())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iscapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iscapr {
            isqg: bool,
            sg: bool,
            rp: bool,
            maxsdu: bool,
            fwd: bool,
            et: bool,
        }
        let proxy = Iscapr {
            isqg: self.isqg(),
            sg: self.sg(),
            rp: self.rp(),
            maxsdu: self.maxsdu(),
            fwd: self.fwd(),
            et: self.et(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream counter index table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iscitcapr(pub u32);
impl Iscitcapr {
    #[doc = "The number of entries assigned to this table. The reset value of this field comes from ISCITMAR."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table. The reset value of this field comes from ISCITMAR."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Iscitcapr {
    #[inline(always)]
    fn default() -> Iscitcapr {
        Iscitcapr(1048960u64 as u32)
    }
}
impl core::fmt::Debug for Iscitcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iscitcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iscitcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iscitcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Iscitcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream counter index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iscitmar(pub u32);
impl Iscitmar {
    #[doc = "The number of words from index table memory assigned to this table. Each Entry consist of 1 word."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table. Each Entry consist of 1 word."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Iscitmar {
    #[inline(always)]
    fn default() -> Iscitmar {
        Iscitmar(384u64 as u32)
    }
}
impl core::fmt::Debug for Iscitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iscitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iscitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iscitmar {
            num_words: u16,
        }
        let proxy = Iscitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream counter index table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iscitor(pub u32);
impl Iscitor {
    #[doc = "The number of entries allocated / in-use by this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries allocated / in-use by this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Iscitor {
    #[inline(always)]
    fn default() -> Iscitor {
        Iscitor(0u64 as u32)
    }
}
impl core::fmt::Debug for Iscitor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iscitor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iscitor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Iscitor {
            num_entries: u16,
        }
        let proxy = Iscitor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream filter hash table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isfhtcapr(pub u32);
impl Isfhtcapr {
    #[doc = "Indicates which configuration access methods are supported: xxx1: Index xx1x: EntryId x1xx: Search 1xxx: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: Index xx1x: EntryId x1xx: Search 1xxx: Reserved"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Isfhtcapr {
    #[inline(always)]
    fn default() -> Isfhtcapr {
        Isfhtcapr(7340032u64 as u32)
    }
}
impl core::fmt::Debug for Isfhtcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isfhtcapr")
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isfhtcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isfhtcapr {
            access_meth: u8,
        }
        let proxy = Isfhtcapr {
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream filter hash table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isfhtor(pub u32);
impl Isfhtor {
    #[doc = "Number of entries in-use by this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of entries in-use by this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Isfhtor {
    #[inline(always)]
    fn default() -> Isfhtor {
        Isfhtor(0u64 as u32)
    }
}
impl core::fmt::Debug for Isfhtor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isfhtor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isfhtor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isfhtor {
            num_entries: u16,
        }
        let proxy = Isfhtor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidcapr(pub u32);
impl Isidcapr {
    #[doc = "Number of Exact Match Key Construction Instances supported for Ingress Stream Identification"]
    #[must_use]
    #[inline(always)]
    pub const fn num_kc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Number of Exact Match Key Construction Instances supported for Ingress Stream Identification"]
    #[inline(always)]
    pub const fn set_num_kc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Number of configurable Payload Fields supported"]
    #[must_use]
    #[inline(always)]
    pub const fn num_pf(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Number of configurable Payload Fields supported"]
    #[inline(always)]
    pub const fn set_num_pf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "Maximum Key Size in bytes which can be constructed using the frame's fields."]
    #[must_use]
    #[inline(always)]
    pub const fn max_ksize(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Maximum Key Size in bytes which can be constructed using the frame's fields."]
    #[inline(always)]
    pub const fn set_max_ksize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Unknown Frame Type (no header field parsing of the frame is necessary to construct the key) supported"]
    #[must_use]
    #[inline(always)]
    pub const fn uft(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Unknown Frame Type (no header field parsing of the frame is necessary to construct the key) supported"]
    #[inline(always)]
    pub const fn set_uft(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Ethernet Frame Type (frame begins with standard 802"]
    #[must_use]
    #[inline(always)]
    pub const fn ethft(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Ethernet Frame Type (frame begins with standard 802"]
    #[inline(always)]
    pub const fn set_ethft(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Isidcapr {
    #[inline(always)]
    fn default() -> Isidcapr {
        Isidcapr(200723u64 as u32)
    }
}
impl core::fmt::Debug for Isidcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidcapr")
            .field("num_kc", &self.num_kc())
            .field("num_pf", &self.num_pf())
            .field("max_ksize", &self.max_ksize())
            .field("uft", &self.uft())
            .field("ethft", &self.ethft())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidcapr {
            num_kc: u8,
            num_pf: u8,
            max_ksize: u8,
            uft: bool,
            ethft: bool,
        }
        let proxy = Isidcapr {
            num_kc: self.num_kc(),
            num_pf: self.num_pf(),
            max_ksize: self.max_ksize(),
            uft: self.uft(),
            ethft: self.ethft(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification hash table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidhtcapr(pub u32);
impl Isidhtcapr {
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Isidhtcapr {
    #[inline(always)]
    fn default() -> Isidhtcapr {
        Isidhtcapr(11534336u64 as u32)
    }
}
impl core::fmt::Debug for Isidhtcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidhtcapr")
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidhtcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidhtcapr {
            access_meth: u8,
        }
        let proxy = Isidhtcapr {
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 0 configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc0cr0(pub u32);
impl Isidkc0cr0 {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    #[must_use]
    #[inline(always)]
    pub const fn portp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    #[inline(always)]
    pub const fn set_portp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    #[must_use]
    #[inline(always)]
    pub const fn spmp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    #[inline(always)]
    pub const fn set_spmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Destination MAC (address) Present"]
    #[must_use]
    #[inline(always)]
    pub const fn dmacp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Destination MAC (address) Present"]
    #[inline(always)]
    pub const fn set_dmacp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Source MAC (address) Present."]
    #[must_use]
    #[inline(always)]
    pub const fn smacp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Source MAC (address) Present."]
    #[inline(always)]
    pub const fn set_smacp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Outer VID Present"]
    #[must_use]
    #[inline(always)]
    pub const fn ovidp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Outer VID Present"]
    #[inline(always)]
    pub const fn set_ovidp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Outer PCP Present"]
    #[must_use]
    #[inline(always)]
    pub const fn opcpp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Outer PCP Present"]
    #[inline(always)]
    pub const fn set_opcpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Inner VID Present."]
    #[must_use]
    #[inline(always)]
    pub const fn ividp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Inner VID Present."]
    #[inline(always)]
    pub const fn set_ividp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Inner PCP Present."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcpp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Inner PCP Present."]
    #[inline(always)]
    pub const fn set_ipcpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Sequence Tag (code point) Present."]
    #[must_use]
    #[inline(always)]
    pub const fn sqtp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Tag (code point) Present."]
    #[inline(always)]
    pub const fn set_sqtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EtherType Present."]
    #[must_use]
    #[inline(always)]
    pub const fn etp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EtherType Present."]
    #[inline(always)]
    pub const fn set_etp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Isidkc0cr0 {
    #[inline(always)]
    fn default() -> Isidkc0cr0 {
        Isidkc0cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc0cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc0cr0")
            .field("valid", &self.valid())
            .field("portp", &self.portp())
            .field("spmp", &self.spmp())
            .field("dmacp", &self.dmacp())
            .field("smacp", &self.smacp())
            .field("ovidp", &self.ovidp())
            .field("opcpp", &self.opcpp())
            .field("ividp", &self.ividp())
            .field("ipcpp", &self.ipcpp())
            .field("sqtp", &self.sqtp())
            .field("etp", &self.etp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc0cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc0cr0 {
            valid: bool,
            portp: bool,
            spmp: bool,
            dmacp: bool,
            smacp: bool,
            ovidp: bool,
            opcpp: bool,
            ividp: bool,
            ipcpp: bool,
            sqtp: bool,
            etp: bool,
        }
        let proxy = Isidkc0cr0 {
            valid: self.valid(),
            portp: self.portp(),
            spmp: self.spmp(),
            dmacp: self.dmacp(),
            smacp: self.smacp(),
            ovidp: self.ovidp(),
            opcpp: self.opcpp(),
            ividp: self.ividp(),
            ipcpp: self.ipcpp(),
            sqtp: self.sqtp(),
            etp: self.etp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 0 operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc0or(pub u32);
impl Isidkc0or {
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Isidkc0or {
    #[inline(always)]
    fn default() -> Isidkc0or {
        Isidkc0or(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc0or {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc0or")
            .field("num_entries", &self.num_entries())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc0or {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc0or {
            num_entries: u16,
            en: bool,
        }
        let proxy = Isidkc0or {
            num_entries: self.num_entries(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 0 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc0pf0cr(pub u32);
impl Isidkc0pf0cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc0pf0cr {
    #[inline(always)]
    fn default() -> Isidkc0pf0cr {
        Isidkc0pf0cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc0pf0cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc0pf0cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc0pf0cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc0pf0cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc0pf0cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 1 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc0pf1cr(pub u32);
impl Isidkc0pf1cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc0pf1cr {
    #[inline(always)]
    fn default() -> Isidkc0pf1cr {
        Isidkc0pf1cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc0pf1cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc0pf1cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc0pf1cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc0pf1cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc0pf1cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 2 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc0pf2cr(pub u32);
impl Isidkc0pf2cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc0pf2cr {
    #[inline(always)]
    fn default() -> Isidkc0pf2cr {
        Isidkc0pf2cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc0pf2cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc0pf2cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc0pf2cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc0pf2cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc0pf2cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 0 payload field 3 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc0pf3cr(pub u32);
impl Isidkc0pf3cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc0pf3cr {
    #[inline(always)]
    fn default() -> Isidkc0pf3cr {
        Isidkc0pf3cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc0pf3cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc0pf3cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc0pf3cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc0pf3cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc0pf3cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 1 configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc1cr0(pub u32);
impl Isidkc1cr0 {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    #[must_use]
    #[inline(always)]
    pub const fn portp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    #[inline(always)]
    pub const fn set_portp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    #[must_use]
    #[inline(always)]
    pub const fn spmp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    #[inline(always)]
    pub const fn set_spmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Destination MAC (address) Present"]
    #[must_use]
    #[inline(always)]
    pub const fn dmacp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Destination MAC (address) Present"]
    #[inline(always)]
    pub const fn set_dmacp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Source MAC (address) Present."]
    #[must_use]
    #[inline(always)]
    pub const fn smacp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Source MAC (address) Present."]
    #[inline(always)]
    pub const fn set_smacp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Outer VID Present"]
    #[must_use]
    #[inline(always)]
    pub const fn ovidp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Outer VID Present"]
    #[inline(always)]
    pub const fn set_ovidp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Outer PCP Present"]
    #[must_use]
    #[inline(always)]
    pub const fn opcpp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Outer PCP Present"]
    #[inline(always)]
    pub const fn set_opcpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Inner VID Present."]
    #[must_use]
    #[inline(always)]
    pub const fn ividp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Inner VID Present."]
    #[inline(always)]
    pub const fn set_ividp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Inner PCP Present."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcpp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Inner PCP Present."]
    #[inline(always)]
    pub const fn set_ipcpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Sequence Tag (code point) Present."]
    #[must_use]
    #[inline(always)]
    pub const fn sqtp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Tag (code point) Present."]
    #[inline(always)]
    pub const fn set_sqtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EtherType Present."]
    #[must_use]
    #[inline(always)]
    pub const fn etp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EtherType Present."]
    #[inline(always)]
    pub const fn set_etp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Isidkc1cr0 {
    #[inline(always)]
    fn default() -> Isidkc1cr0 {
        Isidkc1cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc1cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc1cr0")
            .field("valid", &self.valid())
            .field("portp", &self.portp())
            .field("spmp", &self.spmp())
            .field("dmacp", &self.dmacp())
            .field("smacp", &self.smacp())
            .field("ovidp", &self.ovidp())
            .field("opcpp", &self.opcpp())
            .field("ividp", &self.ividp())
            .field("ipcpp", &self.ipcpp())
            .field("sqtp", &self.sqtp())
            .field("etp", &self.etp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc1cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc1cr0 {
            valid: bool,
            portp: bool,
            spmp: bool,
            dmacp: bool,
            smacp: bool,
            ovidp: bool,
            opcpp: bool,
            ividp: bool,
            ipcpp: bool,
            sqtp: bool,
            etp: bool,
        }
        let proxy = Isidkc1cr0 {
            valid: self.valid(),
            portp: self.portp(),
            spmp: self.spmp(),
            dmacp: self.dmacp(),
            smacp: self.smacp(),
            ovidp: self.ovidp(),
            opcpp: self.opcpp(),
            ividp: self.ividp(),
            ipcpp: self.ipcpp(),
            sqtp: self.sqtp(),
            etp: self.etp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 1 operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc1or(pub u32);
impl Isidkc1or {
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Isidkc1or {
    #[inline(always)]
    fn default() -> Isidkc1or {
        Isidkc1or(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc1or {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc1or")
            .field("num_entries", &self.num_entries())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc1or {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc1or {
            num_entries: u16,
            en: bool,
        }
        let proxy = Isidkc1or {
            num_entries: self.num_entries(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 0 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc1pf0cr(pub u32);
impl Isidkc1pf0cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc1pf0cr {
    #[inline(always)]
    fn default() -> Isidkc1pf0cr {
        Isidkc1pf0cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc1pf0cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc1pf0cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc1pf0cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc1pf0cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc1pf0cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 1 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc1pf1cr(pub u32);
impl Isidkc1pf1cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc1pf1cr {
    #[inline(always)]
    fn default() -> Isidkc1pf1cr {
        Isidkc1pf1cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc1pf1cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc1pf1cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc1pf1cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc1pf1cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc1pf1cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 2 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc1pf2cr(pub u32);
impl Isidkc1pf2cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc1pf2cr {
    #[inline(always)]
    fn default() -> Isidkc1pf2cr {
        Isidkc1pf2cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc1pf2cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc1pf2cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc1pf2cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc1pf2cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc1pf2cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 1 payload field 3 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc1pf3cr(pub u32);
impl Isidkc1pf3cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc1pf3cr {
    #[inline(always)]
    fn default() -> Isidkc1pf3cr {
        Isidkc1pf3cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc1pf3cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc1pf3cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc1pf3cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc1pf3cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc1pf3cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 2 configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc2cr0(pub u32);
impl Isidkc2cr0 {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    #[must_use]
    #[inline(always)]
    pub const fn portp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    #[inline(always)]
    pub const fn set_portp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    #[must_use]
    #[inline(always)]
    pub const fn spmp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    #[inline(always)]
    pub const fn set_spmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Destination MAC (address) Present"]
    #[must_use]
    #[inline(always)]
    pub const fn dmacp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Destination MAC (address) Present"]
    #[inline(always)]
    pub const fn set_dmacp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Source MAC (address) Present."]
    #[must_use]
    #[inline(always)]
    pub const fn smacp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Source MAC (address) Present."]
    #[inline(always)]
    pub const fn set_smacp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Outer VID Present"]
    #[must_use]
    #[inline(always)]
    pub const fn ovidp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Outer VID Present"]
    #[inline(always)]
    pub const fn set_ovidp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Outer PCP Present"]
    #[must_use]
    #[inline(always)]
    pub const fn opcpp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Outer PCP Present"]
    #[inline(always)]
    pub const fn set_opcpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Inner VID Present."]
    #[must_use]
    #[inline(always)]
    pub const fn ividp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Inner VID Present."]
    #[inline(always)]
    pub const fn set_ividp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Inner PCP Present."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcpp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Inner PCP Present."]
    #[inline(always)]
    pub const fn set_ipcpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Sequence Tag (code point) Present."]
    #[must_use]
    #[inline(always)]
    pub const fn sqtp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Tag (code point) Present."]
    #[inline(always)]
    pub const fn set_sqtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EtherType Present."]
    #[must_use]
    #[inline(always)]
    pub const fn etp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EtherType Present."]
    #[inline(always)]
    pub const fn set_etp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Isidkc2cr0 {
    #[inline(always)]
    fn default() -> Isidkc2cr0 {
        Isidkc2cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc2cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc2cr0")
            .field("valid", &self.valid())
            .field("portp", &self.portp())
            .field("spmp", &self.spmp())
            .field("dmacp", &self.dmacp())
            .field("smacp", &self.smacp())
            .field("ovidp", &self.ovidp())
            .field("opcpp", &self.opcpp())
            .field("ividp", &self.ividp())
            .field("ipcpp", &self.ipcpp())
            .field("sqtp", &self.sqtp())
            .field("etp", &self.etp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc2cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc2cr0 {
            valid: bool,
            portp: bool,
            spmp: bool,
            dmacp: bool,
            smacp: bool,
            ovidp: bool,
            opcpp: bool,
            ividp: bool,
            ipcpp: bool,
            sqtp: bool,
            etp: bool,
        }
        let proxy = Isidkc2cr0 {
            valid: self.valid(),
            portp: self.portp(),
            spmp: self.spmp(),
            dmacp: self.dmacp(),
            smacp: self.smacp(),
            ovidp: self.ovidp(),
            opcpp: self.opcpp(),
            ividp: self.ividp(),
            ipcpp: self.ipcpp(),
            sqtp: self.sqtp(),
            etp: self.etp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 2 operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc2or(pub u32);
impl Isidkc2or {
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Isidkc2or {
    #[inline(always)]
    fn default() -> Isidkc2or {
        Isidkc2or(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc2or {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc2or")
            .field("num_entries", &self.num_entries())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc2or {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc2or {
            num_entries: u16,
            en: bool,
        }
        let proxy = Isidkc2or {
            num_entries: self.num_entries(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 0 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc2pf0cr(pub u32);
impl Isidkc2pf0cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc2pf0cr {
    #[inline(always)]
    fn default() -> Isidkc2pf0cr {
        Isidkc2pf0cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc2pf0cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc2pf0cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc2pf0cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc2pf0cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc2pf0cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 1 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc2pf1cr(pub u32);
impl Isidkc2pf1cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc2pf1cr {
    #[inline(always)]
    fn default() -> Isidkc2pf1cr {
        Isidkc2pf1cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc2pf1cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc2pf1cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc2pf1cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc2pf1cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc2pf1cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 2 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc2pf2cr(pub u32);
impl Isidkc2pf2cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc2pf2cr {
    #[inline(always)]
    fn default() -> Isidkc2pf2cr {
        Isidkc2pf2cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc2pf2cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc2pf2cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc2pf2cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc2pf2cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc2pf2cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 2 payload field 3 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc2pf3cr(pub u32);
impl Isidkc2pf3cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc2pf3cr {
    #[inline(always)]
    fn default() -> Isidkc2pf3cr {
        Isidkc2pf3cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc2pf3cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc2pf3cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc2pf3cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc2pf3cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc2pf3cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 3 configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc3cr0(pub u32);
impl Isidkc3cr0 {
    #[doc = "Valid"]
    #[must_use]
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub const fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    #[must_use]
    #[inline(always)]
    pub const fn portp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Source Port Present Specifies whether the source port is present in the key."]
    #[inline(always)]
    pub const fn set_portp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    #[must_use]
    #[inline(always)]
    pub const fn spmp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Switch Port Masquerading (flag) Present"]
    #[inline(always)]
    pub const fn set_spmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Destination MAC (address) Present"]
    #[must_use]
    #[inline(always)]
    pub const fn dmacp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Destination MAC (address) Present"]
    #[inline(always)]
    pub const fn set_dmacp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Source MAC (address) Present."]
    #[must_use]
    #[inline(always)]
    pub const fn smacp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Source MAC (address) Present."]
    #[inline(always)]
    pub const fn set_smacp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Outer VID Present"]
    #[must_use]
    #[inline(always)]
    pub const fn ovidp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Outer VID Present"]
    #[inline(always)]
    pub const fn set_ovidp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Outer PCP Present"]
    #[must_use]
    #[inline(always)]
    pub const fn opcpp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Outer PCP Present"]
    #[inline(always)]
    pub const fn set_opcpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Inner VID Present."]
    #[must_use]
    #[inline(always)]
    pub const fn ividp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Inner VID Present."]
    #[inline(always)]
    pub const fn set_ividp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Inner PCP Present."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcpp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Inner PCP Present."]
    #[inline(always)]
    pub const fn set_ipcpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Sequence Tag (code point) Present."]
    #[must_use]
    #[inline(always)]
    pub const fn sqtp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Sequence Tag (code point) Present."]
    #[inline(always)]
    pub const fn set_sqtp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "EtherType Present."]
    #[must_use]
    #[inline(always)]
    pub const fn etp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "EtherType Present."]
    #[inline(always)]
    pub const fn set_etp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Isidkc3cr0 {
    #[inline(always)]
    fn default() -> Isidkc3cr0 {
        Isidkc3cr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc3cr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc3cr0")
            .field("valid", &self.valid())
            .field("portp", &self.portp())
            .field("spmp", &self.spmp())
            .field("dmacp", &self.dmacp())
            .field("smacp", &self.smacp())
            .field("ovidp", &self.ovidp())
            .field("opcpp", &self.opcpp())
            .field("ividp", &self.ividp())
            .field("ipcpp", &self.ipcpp())
            .field("sqtp", &self.sqtp())
            .field("etp", &self.etp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc3cr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc3cr0 {
            valid: bool,
            portp: bool,
            spmp: bool,
            dmacp: bool,
            smacp: bool,
            ovidp: bool,
            opcpp: bool,
            ividp: bool,
            ipcpp: bool,
            sqtp: bool,
            etp: bool,
        }
        let proxy = Isidkc3cr0 {
            valid: self.valid(),
            portp: self.portp(),
            spmp: self.spmp(),
            dmacp: self.dmacp(),
            smacp: self.smacp(),
            ovidp: self.ovidp(),
            opcpp: self.opcpp(),
            ividp: self.ividp(),
            ipcpp: self.ipcpp(),
            sqtp: self.sqtp(),
            etp: self.etp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 3 operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc3or(pub u32);
impl Isidkc3or {
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates the number of Ingress Stream Identification, i"]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Operational state of this key construction register: 0: Disabled: Exact Match Lookup must not utilize this key construction"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Isidkc3or {
    #[inline(always)]
    fn default() -> Isidkc3or {
        Isidkc3or(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc3or {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc3or")
            .field("num_entries", &self.num_entries())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc3or {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc3or {
            num_entries: u16,
            en: bool,
        }
        let proxy = Isidkc3or {
            num_entries: self.num_entries(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 0 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc3pf0cr(pub u32);
impl Isidkc3pf0cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc3pf0cr {
    #[inline(always)]
    fn default() -> Isidkc3pf0cr {
        Isidkc3pf0cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc3pf0cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc3pf0cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc3pf0cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc3pf0cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc3pf0cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 1 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc3pf1cr(pub u32);
impl Isidkc3pf1cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc3pf1cr {
    #[inline(always)]
    fn default() -> Isidkc3pf1cr {
        Isidkc3pf1cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc3pf1cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc3pf1cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc3pf1cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc3pf1cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc3pf1cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 2 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc3pf2cr(pub u32);
impl Isidkc3pf2cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc3pf2cr {
    #[inline(always)]
    fn default() -> Isidkc3pf2cr {
        Isidkc3pf2cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc3pf2cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc3pf2cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc3pf2cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc3pf2cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc3pf2cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream identification key construction 3 payload field 3 configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isidkc3pf3cr(pub u32);
impl Isidkc3pf3cr {
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[must_use]
    #[inline(always)]
    pub const fn pfp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Payload field Present 0: This payload field register is not used for constructing a key"]
    #[inline(always)]
    pub const fn set_pfp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "The NUM_BYTES field is used to specify the size of the payload key field, as follows: Size of the payload key field = NUM_BYTES + 1; range is 1 to 16 bytes"]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_offset(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Payload Byte Offset where field extraction begins"]
    #[inline(always)]
    pub const fn set_byte_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn fbmask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Payload First Byte Mask: Number of most significant bits of first payload key byte to mask."]
    #[inline(always)]
    pub const fn set_fbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[must_use]
    #[inline(always)]
    pub const fn lbmask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "Payload Last Byte Mask: Number of least significant bits from the last payload key byte to mask."]
    #[inline(always)]
    pub const fn set_lbmask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for Isidkc3pf3cr {
    #[inline(always)]
    fn default() -> Isidkc3pf3cr {
        Isidkc3pf3cr(0u64 as u32)
    }
}
impl core::fmt::Debug for Isidkc3pf3cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isidkc3pf3cr")
            .field("pfp", &self.pfp())
            .field("num_bytes", &self.num_bytes())
            .field("byte_offset", &self.byte_offset())
            .field("fbmask", &self.fbmask())
            .field("lbmask", &self.lbmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isidkc3pf3cr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isidkc3pf3cr {
            pfp: bool,
            num_bytes: u8,
            byte_offset: u8,
            fbmask: u8,
            lbmask: u8,
        }
        let proxy = Isidkc3pf3cr {
            pfp: self.pfp(),
            num_bytes: self.num_bytes(),
            byte_offset: self.byte_offset(),
            fbmask: self.fbmask(),
            lbmask: self.lbmask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream index table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isitcapr(pub u32);
impl Isitcapr {
    #[doc = "The number of entries assigned to this table. The reset value of this field comes from ISITMAR."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table. The reset value of this field comes from ISITMAR."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Isitcapr {
    #[inline(always)]
    fn default() -> Isitcapr {
        Isitcapr(1048960u64 as u32)
    }
}
impl core::fmt::Debug for Isitcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isitcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isitcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isitcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Isitcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isitmar(pub u32);
impl Isitmar {
    #[doc = "The number of words from index table memory assigned to this table. Each Entry consist of 1 word."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words from index table memory assigned to this table. Each Entry consist of 1 word."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Isitmar {
    #[inline(always)]
    fn default() -> Isitmar {
        Isitmar(384u64 as u32)
    }
}
impl core::fmt::Debug for Isitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isitmar {
            num_words: u16,
        }
        let proxy = Isitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress stream index table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isitor(pub u32);
impl Isitor {
    #[doc = "The number of entries in-use by this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries in-use by this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Isitor {
    #[inline(always)]
    fn default() -> Isitor {
        Isitor(0u64 as u32)
    }
}
impl core::fmt::Debug for Isitor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isitor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isitor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isitor {
            num_entries: u16,
        }
        let proxy = Isitor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress sequence generation index table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isqgitcapr(pub u32);
impl Isqgitcapr {
    #[doc = "The number of entries assigned to this table"]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table"]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Isqgitcapr {
    #[inline(always)]
    fn default() -> Isqgitcapr {
        Isqgitcapr(1048960u64 as u32)
    }
}
impl core::fmt::Debug for Isqgitcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isqgitcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isqgitcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isqgitcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Isqgitcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress sequence generation index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isqgitmar(pub u32);
impl Isqgitmar {
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
impl Default for Isqgitmar {
    #[inline(always)]
    fn default() -> Isqgitmar {
        Isqgitmar(48u64 as u32)
    }
}
impl core::fmt::Debug for Isqgitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isqgitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isqgitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isqgitmar {
            num_words: u16,
        }
        let proxy = Isqgitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ingress sequence generation index table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isqgitor(pub u32);
impl Isqgitor {
    #[doc = "The number of entries allocated / in-use by this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries allocated / in-use by this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Isqgitor {
    #[inline(always)]
    fn default() -> Isqgitor {
        Isqgitor(0u64 as u32)
    }
}
impl core::fmt::Debug for Isqgitor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isqgitor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isqgitor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isqgitor {
            num_entries: u16,
        }
        let proxy = Isqgitor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Index table memory capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Itmcapr(pub u32);
impl Itmcapr {
    #[doc = "Number of Words in the Index table memory"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of Words in the Index table memory"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Word size in bytes. 0: 24 bytes 1-3: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn word_size(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Word size in bytes. 0: 24 bytes 1-3: Reserved"]
    #[inline(always)]
    pub const fn set_word_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1: Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn mloc(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates memory location of this table 0: Common memory 1: Reserved"]
    #[inline(always)]
    pub const fn set_mloc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Itmcapr {
    #[inline(always)]
    fn default() -> Itmcapr {
        Itmcapr(1488u64 as u32)
    }
}
impl core::fmt::Debug for Itmcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Itmcapr")
            .field("num_words", &self.num_words())
            .field("word_size", &self.word_size())
            .field("mloc", &self.mloc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Itmcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Itmcapr {
            num_words: u16,
            word_size: u8,
            mloc: u8,
        }
        let proxy = Itmcapr {
            num_words: self.num_words(),
            word_size: self.word_size(),
            mloc: self.mloc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Operational state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osr(pub u32);
impl Osr {
    #[doc = "Indicates the function's operational state 0: Function is operationally ready"]
    #[must_use]
    #[inline(always)]
    pub const fn state(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the function's operational state 0: Function is operationally ready"]
    #[inline(always)]
    pub const fn set_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the index table memory (common memory) operational state"]
    #[must_use]
    #[inline(always)]
    pub const fn itm_state(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the index table memory (common memory) operational state"]
    #[inline(always)]
    pub const fn set_itm_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Osr {
    #[inline(always)]
    fn default() -> Osr {
        Osr(1u64 as u32)
    }
}
impl core::fmt::Debug for Osr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Osr")
            .field("state", &self.state())
            .field("itm_state", &self.itm_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Osr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Osr {
            state: bool,
            itm_state: bool,
        }
        let proxy = Osr {
            state: self.state(),
            itm_state: self.itm_state(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Pre-Standard RTAG Ethertype register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psrtagetr(pub u32);
impl Psrtagetr {
    #[doc = "802"]
    #[must_use]
    #[inline(always)]
    pub const fn ethertype(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "802"]
    #[inline(always)]
    pub const fn set_ethertype(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Psrtagetr {
    #[inline(always)]
    fn default() -> Psrtagetr {
        Psrtagetr(0u64 as u32)
    }
}
impl core::fmt::Debug for Psrtagetr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psrtagetr")
            .field("ethertype", &self.ethertype())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psrtagetr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psrtagetr {
            ethertype: u16,
        }
        let proxy = Psrtagetr {
            ethertype: self.ethertype(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rate policer capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpcapr(pub u32);
impl Rpcapr {
    #[doc = "Two-Rate Three-Color Marker supported per MEF 10.3 standard."]
    #[must_use]
    #[inline(always)]
    pub const fn trtcm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Two-Rate Three-Color Marker supported per MEF 10.3 standard."]
    #[inline(always)]
    pub const fn set_trtcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Color Mode capability 0: Support Color Blind mode only 1: Support Color Blind and Color Aware modes"]
    #[must_use]
    #[inline(always)]
    pub const fn cm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Color Mode capability 0: Support Color Blind mode only 1: Support Color Blind and Color Aware modes"]
    #[inline(always)]
    pub const fn set_cm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Rpcapr {
    #[inline(always)]
    fn default() -> Rpcapr {
        Rpcapr(3u64 as u32)
    }
}
impl core::fmt::Debug for Rpcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rpcapr")
            .field("trtcm", &self.trtcm())
            .field("cm", &self.cm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rpcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rpcapr {
            trtcm: bool,
            cm: bool,
        }
        let proxy = Rpcapr {
            trtcm: self.trtcm(),
            cm: self.cm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rate policer index table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpitcapr(pub u32);
impl Rpitcapr {
    #[doc = "The number of entries assigned to this table. Reset value is specified by ROUNDDOWN(RPITMAR/4)."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table. Reset value is specified by ROUNDDOWN(RPITMAR/4)."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Rpitcapr {
    #[inline(always)]
    fn default() -> Rpitcapr {
        Rpitcapr(1048608u64 as u32)
    }
}
impl core::fmt::Debug for Rpitcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rpitcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rpitcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rpitcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Rpitcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rate policer index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpitmar(pub u32);
impl Rpitmar {
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
impl Default for Rpitmar {
    #[inline(always)]
    fn default() -> Rpitmar {
        Rpitmar(128u64 as u32)
    }
}
impl core::fmt::Debug for Rpitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rpitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rpitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rpitmar {
            num_words: u16,
        }
        let proxy = Rpitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Rate policer index table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpitor(pub u32);
impl Rpitor {
    #[doc = "The number of entries in-use by this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "The number of entries in-use by this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for Rpitor {
    #[inline(always)]
    fn default() -> Rpitor {
        Rpitor(0u64 as u32)
    }
}
impl core::fmt::Debug for Rpitor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rpitor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rpitor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rpitor {
            num_entries: u16,
        }
        let proxy = Rpitor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stream gate capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgcapr(pub u32);
impl Sgcapr {
    #[doc = "Support Administrative and Operational Gate Control List. 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn glc_ao(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Support Administrative and Operational Gate Control List. 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_glc_ao(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Support configurable option indicating if GCL's Gate Check is from SFD only or SFD until EOF"]
    #[must_use]
    #[inline(always)]
    pub const fn glc_gc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Support configurable option indicating if GCL's Gate Check is from SFD only or SFD until EOF"]
    #[inline(always)]
    pub const fn set_glc_gc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Each Gate Control List Entry supports Interval Max Octet check. 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn glc_io(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Each Gate Control List Entry supports Interval Max Octet check. 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_glc_io(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Each Gate Control List Entry supports configurable IPV. 0: Not supported 1: Supported"]
    #[must_use]
    #[inline(always)]
    pub const fn glc_ipv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Each Gate Control List Entry supports configurable IPV. 0: Not supported 1: Supported"]
    #[inline(always)]
    pub const fn set_glc_ipv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Each Gate Control List Entry supports configurable CTD (Cut-Through Disable state)"]
    #[must_use]
    #[inline(always)]
    pub const fn glc_ctd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Each Gate Control List Entry supports configurable CTD (Cut-Through Disable state)"]
    #[inline(always)]
    pub const fn set_glc_ctd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Sgcapr {
    #[inline(always)]
    fn default() -> Sgcapr {
        Sgcapr(31u64 as u32)
    }
}
impl core::fmt::Debug for Sgcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sgcapr")
            .field("glc_ao", &self.glc_ao())
            .field("glc_gc", &self.glc_gc())
            .field("glc_io", &self.glc_io())
            .field("glc_ipv", &self.glc_ipv())
            .field("glc_ctd", &self.glc_ctd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sgcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sgcapr {
            glc_ao: bool,
            glc_gc: bool,
            glc_io: bool,
            glc_ipv: bool,
            glc_ctd: bool,
        }
        let proxy = Sgcapr {
            glc_ao: self.glc_ao(),
            glc_gc: self.glc_gc(),
            glc_io: self.glc_io(),
            glc_ipv: self.glc_ipv(),
            glc_ctd: self.glc_ctd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stream gate control list index table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgclitcapr(pub u32);
impl Sgclitcapr {
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
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Sgclitcapr {
    #[inline(always)]
    fn default() -> Sgclitcapr {
        Sgclitcapr(1048768u64 as u32)
    }
}
impl core::fmt::Debug for Sgclitcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sgclitcapr")
            .field("num_words", &self.num_words())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sgclitcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sgclitcapr {
            num_words: u16,
            access_meth: u8,
        }
        let proxy = Sgclitcapr {
            num_words: self.num_words(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stream gate control list index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgclitmar(pub u32);
impl Sgclitmar {
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
impl Default for Sgclitmar {
    #[inline(always)]
    fn default() -> Sgclitmar {
        Sgclitmar(192u64 as u32)
    }
}
impl core::fmt::Debug for Sgclitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sgclitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sgclitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sgclitmar {
            num_words: u16,
        }
        let proxy = Sgclitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stream gate control list table memory operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgcltmor(pub u32);
impl Sgcltmor {
    #[doc = "Number of words in-use."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of words in-use."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Sgcltmor {
    #[inline(always)]
    fn default() -> Sgcltmor {
        Sgcltmor(0u64 as u32)
    }
}
impl core::fmt::Debug for Sgcltmor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sgcltmor")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sgcltmor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sgcltmor {
            num_words: u16,
        }
        let proxy = Sgcltmor {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stream gate instance index table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgiitcapr(pub u32);
impl Sgiitcapr {
    #[doc = "The number of entries assigned to this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries assigned to this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Indicates which configuration access methods are supported: xxx1: EntryId Match xx1x: Exact Match Key Element Match x1xx: Ternary Match Key Element Match 1xxx: Search"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Sgiitcapr {
    #[inline(always)]
    fn default() -> Sgiitcapr {
        Sgiitcapr(1048608u64 as u32)
    }
}
impl core::fmt::Debug for Sgiitcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sgiitcapr")
            .field("num_entries", &self.num_entries())
            .field("access_meth", &self.access_meth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sgiitcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sgiitcapr {
            num_entries: u16,
            access_meth: u8,
        }
        let proxy = Sgiitcapr {
            num_entries: self.num_entries(),
            access_meth: self.access_meth(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stream gate instance index table memory allocation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgiitmar(pub u32);
impl Sgiitmar {
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
impl Default for Sgiitmar {
    #[inline(always)]
    fn default() -> Sgiitmar {
        Sgiitmar(32u64 as u32)
    }
}
impl core::fmt::Debug for Sgiitmar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sgiitmar")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sgiitmar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sgiitmar {
            num_words: u16,
        }
        let proxy = Sgiitmar {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stream gate instance index table operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgiitor(pub u32);
impl Sgiitor {
    #[doc = "The number of entries allocated / in-use by this table."]
    #[must_use]
    #[inline(always)]
    pub const fn num_entries(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of entries allocated / in-use by this table."]
    #[inline(always)]
    pub const fn set_num_entries(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Sgiitor {
    #[inline(always)]
    fn default() -> Sgiitor {
        Sgiitor(0u64 as u32)
    }
}
impl core::fmt::Debug for Sgiitor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sgiitor")
            .field("num_entries", &self.num_entries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sgiitor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sgiitor {
            num_entries: u16,
        }
        let proxy = Sgiitor {
            num_entries: self.num_entries(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Time capture configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tccr(pub u32);
impl Tccr {
    #[doc = "Indicates the duration time in nanoseconds the timestamp capture function is armed"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Indicates the duration time in nanoseconds the timestamp capture function is armed"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Indicates if Timestamp Capture function is armed"]
    #[must_use]
    #[inline(always)]
    pub const fn arm(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates if Timestamp Capture function is armed"]
    #[inline(always)]
    pub const fn set_arm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Tccr {
    #[inline(always)]
    fn default() -> Tccr {
        Tccr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tccr")
            .field("timeout", &self.timeout())
            .field("arm", &self.arm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tccr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tccr {
            timeout: u32,
            arm: u8,
        }
        let proxy = Tccr {
            timeout: self.timeout(),
            arm: self.arm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Time capture interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcier(pub u32);
impl Tcier {
    #[doc = "Transmit interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn transmit(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt"]
    #[inline(always)]
    pub const fn set_transmit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Timeout interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout interrupt"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcier {
    #[inline(always)]
    fn default() -> Tcier {
        Tcier(0u64 as u32)
    }
}
impl core::fmt::Debug for Tcier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcier")
            .field("transmit", &self.transmit())
            .field("timeout", &self.timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcier {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tcier {
            transmit: bool,
            timeout: bool,
        }
        let proxy = Tcier {
            transmit: self.transmit(),
            timeout: self.timeout(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Time capture MSI-X vector register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcmsivr(pub u32);
impl Tcmsivr {
    #[doc = "Index into MSI-X address/data table"]
    #[must_use]
    #[inline(always)]
    pub const fn vector(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Index into MSI-X address/data table"]
    #[inline(always)]
    pub const fn set_vector(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Tcmsivr {
    #[inline(always)]
    fn default() -> Tcmsivr {
        Tcmsivr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tcmsivr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcmsivr")
            .field("vector", &self.vector())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcmsivr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tcmsivr {
            vector: u8,
        }
        let proxy = Tcmsivr {
            vector: self.vector(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Time capture receive port interrupt detect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcrpidr(pub u32);
impl Tcrpidr {
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_port0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[inline(always)]
    pub const fn set_tx_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_port1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[inline(always)]
    pub const fn set_tx_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_port2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[inline(always)]
    pub const fn set_tx_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_port3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[inline(always)]
    pub const fn set_tx_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_port4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bit vector Indicating the transmit ports who captured the latency"]
    #[inline(always)]
    pub const fn set_tx_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn transmit(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt"]
    #[inline(always)]
    pub const fn set_transmit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Timeout interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Timeout interrupt"]
    #[inline(always)]
    pub const fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Tcrpidr {
    #[inline(always)]
    fn default() -> Tcrpidr {
        Tcrpidr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tcrpidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcrpidr")
            .field("tx_port0", &self.tx_port0())
            .field("tx_port1", &self.tx_port1())
            .field("tx_port2", &self.tx_port2())
            .field("tx_port3", &self.tx_port3())
            .field("tx_port4", &self.tx_port4())
            .field("transmit", &self.transmit())
            .field("timeout", &self.timeout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcrpidr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tcrpidr {
            tx_port0: bool,
            tx_port1: bool,
            tx_port2: bool,
            tx_port3: bool,
            tx_port4: bool,
            transmit: bool,
            timeout: bool,
        }
        let proxy = Tcrpidr {
            tx_port0: self.tx_port0(),
            tx_port1: self.tx_port1(),
            tx_port2: self.tx_port2(),
            tx_port3: self.tx_port3(),
            tx_port4: self.tx_port4(),
            transmit: self.transmit(),
            timeout: self.timeout(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Time capture receive port status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcrpsr(pub u32);
impl Tcrpsr {
    #[doc = "Receive port which captured the receive timestamp stored in TCRPTSR"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_port(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive port which captured the receive timestamp stored in TCRPTSR"]
    #[inline(always)]
    pub const fn set_rx_port(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Count the number of times the time capture function was triggered since it was ARMed"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Count the number of times the time capture function was triggered since it was ARMed"]
    #[inline(always)]
    pub const fn set_rx_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
}
impl Default for Tcrpsr {
    #[inline(always)]
    fn default() -> Tcrpsr {
        Tcrpsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Tcrpsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tcrpsr")
            .field("rx_port", &self.rx_port())
            .field("rx_cnt", &self.rx_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tcrpsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tcrpsr {
            rx_port: u8,
            rx_cnt: u8,
        }
        let proxy = Tcrpsr {
            rx_port: self.rx_port(),
            rx_cnt: self.rx_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Time gate scheduling table capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tgstcapr(pub u32);
impl Tgstcapr {
    #[doc = "Number of Words"]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of Words"]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Access Method"]
    #[must_use]
    #[inline(always)]
    pub const fn access_meth(&self) -> super::vals::AccessMeth {
        let val = (self.0 >> 20usize) & 0x0f;
        super::vals::AccessMeth::from_bits(val as u8)
    }
    #[doc = "Access Method"]
    #[inline(always)]
    pub const fn set_access_meth(&mut self, val: super::vals::AccessMeth) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
    }
    #[doc = "Maximum Gate Control List Length"]
    #[must_use]
    #[inline(always)]
    pub const fn max_gcl_len(&self) -> super::vals::MaxGclLen {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::MaxGclLen::from_bits(val as u8)
    }
    #[doc = "Maximum Gate Control List Length"]
    #[inline(always)]
    pub const fn set_max_gcl_len(&mut self, val: super::vals::MaxGclLen) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Tgstcapr {
    #[inline(always)]
    fn default() -> Tgstcapr {
        Tgstcapr(35652864u64 as u32)
    }
}
impl core::fmt::Debug for Tgstcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tgstcapr")
            .field("num_words", &self.num_words())
            .field("access_meth", &self.access_meth())
            .field("max_gcl_len", &self.max_gcl_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tgstcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tgstcapr {
            num_words: u16,
            access_meth: super::vals::AccessMeth,
            max_gcl_len: super::vals::MaxGclLen,
        }
        let proxy = Tgstcapr {
            num_words: self.num_words(),
            access_meth: self.access_meth(),
            max_gcl_len: self.max_gcl_len(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Time gate scheduling table memory operation register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tgstmor(pub u32);
impl Tgstmor {
    #[doc = "The number of words in-use."]
    #[must_use]
    #[inline(always)]
    pub const fn num_words(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of words in-use."]
    #[inline(always)]
    pub const fn set_num_words(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Tgstmor {
    #[inline(always)]
    fn default() -> Tgstmor {
        Tgstmor(0u64 as u32)
    }
}
impl core::fmt::Debug for Tgstmor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tgstmor")
            .field("num_words", &self.num_words())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tgstmor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tgstmor {
            num_words: u16,
        }
        let proxy = Tgstmor {
            num_words: self.num_words(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable fatal memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ufmecr(pub u32);
impl Ufmecr {
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ufmecr {
    #[inline(always)]
    fn default() -> Ufmecr {
        Ufmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ufmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ufmecr").field("rd", &self.rd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ufmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ufmecr {
            rd: bool,
        }
        let proxy = Ufmecr { rd: self.rd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable fatal memory error status register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ufmesr0(pub u32);
impl Ufmesr0 {
    #[doc = "Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Syndrome"]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Multi-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn mbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-bit ECC error"]
    #[inline(always)]
    pub const fn set_mbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ufmesr0 {
    #[inline(always)]
    fn default() -> Ufmesr0 {
        Ufmesr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ufmesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ufmesr0")
            .field("syndrome", &self.syndrome())
            .field("mem_id", &self.mem_id())
            .field("m", &self.m())
            .field("mbee", &self.mbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ufmesr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ufmesr0 {
            syndrome: u16,
            mem_id: u8,
            m: bool,
            mbee: bool,
        }
        let proxy = Ufmesr0 {
            syndrome: self.syndrome(),
            mem_id: self.mem_id(),
            m: self.m(),
            mbee: self.mbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable fatal system bus error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ufsbecr(pub u32);
impl Ufsbecr {
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> super::vals::Rd {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Rd::from_bits(val as u8)
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: super::vals::Rd) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Ufsbecr {
    #[inline(always)]
    fn default() -> Ufsbecr {
        Ufsbecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ufsbecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ufsbecr").field("rd", &self.rd()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ufsbecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ufsbecr {
            rd: super::vals::Rd,
        }
        let proxy = Ufsbecr { rd: self.rd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable fatal system bus error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ufsbesr(pub u32);
impl Ufsbesr {
    #[doc = "System Bus ID"]
    #[must_use]
    #[inline(always)]
    pub const fn sb_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "System Bus ID"]
    #[inline(always)]
    pub const fn set_sb_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Multiple"]
    #[must_use]
    #[inline(always)]
    pub const fn m(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Multiple"]
    #[inline(always)]
    pub const fn set_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "System bus error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "System bus error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ufsbesr {
    #[inline(always)]
    fn default() -> Ufsbesr {
        Ufsbesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ufsbesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ufsbesr")
            .field("sb_id", &self.sb_id())
            .field("m", &self.m())
            .field("sbe", &self.sbe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ufsbesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ufsbesr {
            sb_id: u8,
            m: bool,
            sbe: bool,
        }
        let proxy = Ufsbesr {
            sb_id: self.sb_id(),
            m: self.m(),
            sbe: self.sbe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal MAC error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unmacecr(pub u32);
impl Unmacecr {
    #[doc = "Report disable port"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable port"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Report disable port"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable port"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Report disable port"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable port"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Report disable port"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable port"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Report disable port"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable port"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Unmacecr {
    #[inline(always)]
    fn default() -> Unmacecr {
        Unmacecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unmacecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unmacecr")
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unmacecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unmacecr {
            port0: bool,
            port1: bool,
            port2: bool,
            port3: bool,
            port4: bool,
        }
        let proxy = Unmacecr {
            port0: self.port0(),
            port1: self.port1(),
            port2: self.port2(),
            port3: self.port3(),
            port4: self.port4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal MAC error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unmacesr(pub u32);
impl Unmacesr {
    #[doc = "Port 0 MAC error"]
    #[must_use]
    #[inline(always)]
    pub const fn port0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Port 0 MAC error"]
    #[inline(always)]
    pub const fn set_port0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Port 1 MAC error"]
    #[must_use]
    #[inline(always)]
    pub const fn port1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Port 1 MAC error"]
    #[inline(always)]
    pub const fn set_port1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Port 2 MAC error"]
    #[must_use]
    #[inline(always)]
    pub const fn port2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Port 2 MAC error"]
    #[inline(always)]
    pub const fn set_port2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Port 3 MAC error"]
    #[must_use]
    #[inline(always)]
    pub const fn port3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Port 3 MAC error"]
    #[inline(always)]
    pub const fn set_port3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Port 4 MAC error"]
    #[must_use]
    #[inline(always)]
    pub const fn port4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Port 4 MAC error"]
    #[inline(always)]
    pub const fn set_port4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Unmacesr {
    #[inline(always)]
    fn default() -> Unmacesr {
        Unmacesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unmacesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unmacesr")
            .field("port0", &self.port0())
            .field("port1", &self.port1())
            .field("port2", &self.port2())
            .field("port3", &self.port3())
            .field("port4", &self.port4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unmacesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unmacesr {
            port0: bool,
            port1: bool,
            port2: bool,
            port3: bool,
            port4: bool,
        }
        let proxy = Unmacesr {
            port0: self.port0(),
            port1: self.port1(),
            port2: self.port2(),
            port3: self.port3(),
            port4: self.port4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal memory error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unmecr(pub u32);
impl Unmecr {
    #[doc = "Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub const fn set_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Report disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Report disable"]
    #[inline(always)]
    pub const fn set_rd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Unmecr {
    #[inline(always)]
    fn default() -> Unmecr {
        Unmecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unmecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unmecr")
            .field("threshold", &self.threshold())
            .field("rd", &self.rd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unmecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unmecr {
            threshold: u8,
            rd: bool,
        }
        let proxy = Unmecr {
            threshold: self.threshold(),
            rd: self.rd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal memory error count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unmectr(pub u32);
impl Unmectr {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Unmectr {
    #[inline(always)]
    fn default() -> Unmectr {
        Unmectr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unmectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unmectr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unmectr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unmectr {
            count: u8,
        }
        let proxy = Unmectr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal memory error status register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unmesr0(pub u32);
impl Unmesr0 {
    #[doc = "Syndrome"]
    #[must_use]
    #[inline(always)]
    pub const fn syndrome(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Syndrome"]
    #[inline(always)]
    pub const fn set_syndrome(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "Memory ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mem_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Memory ID"]
    #[inline(always)]
    pub const fn set_mem_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Multi-bit ECC error"]
    #[must_use]
    #[inline(always)]
    pub const fn mbee(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-bit ECC error"]
    #[inline(always)]
    pub const fn set_mbee(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Unmesr0 {
    #[inline(always)]
    fn default() -> Unmesr0 {
        Unmesr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Unmesr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unmesr0")
            .field("syndrome", &self.syndrome())
            .field("mem_id", &self.mem_id())
            .field("mbee", &self.mbee())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unmesr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unmesr0 {
            syndrome: u16,
            mem_id: u8,
            mbee: bool,
        }
        let proxy = Unmesr0 {
            syndrome: self.syndrome(),
            mem_id: self.mem_id(),
            mbee: self.mbee(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal system bus error configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unsbecr(pub u32);
impl Unsbecr {
    #[doc = "Threshold"]
    #[must_use]
    #[inline(always)]
    pub const fn threshold(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Threshold"]
    #[inline(always)]
    pub const fn set_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Unsbecr {
    #[inline(always)]
    fn default() -> Unsbecr {
        Unsbecr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unsbecr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unsbecr")
            .field("threshold", &self.threshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unsbecr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unsbecr {
            threshold: u8,
        }
        let proxy = Unsbecr {
            threshold: self.threshold(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal system bus error count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unsbectr(pub u32);
impl Unsbectr {
    #[doc = "Count"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Count"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Unsbectr {
    #[inline(always)]
    fn default() -> Unsbectr {
        Unsbectr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unsbectr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unsbectr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unsbectr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unsbectr {
            count: u8,
        }
        let proxy = Unsbectr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Uncorrectable non-fatal system bus error status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Unsbesr(pub u32);
impl Unsbesr {
    #[doc = "System Bus ID"]
    #[must_use]
    #[inline(always)]
    pub const fn sb_id(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "System Bus ID"]
    #[inline(always)]
    pub const fn set_sb_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "System bus error"]
    #[must_use]
    #[inline(always)]
    pub const fn sbe(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "System bus error"]
    #[inline(always)]
    pub const fn set_sbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Unsbesr {
    #[inline(always)]
    fn default() -> Unsbesr {
        Unsbesr(0u64 as u32)
    }
}
impl core::fmt::Debug for Unsbesr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Unsbesr")
            .field("sb_id", &self.sb_id())
            .field("sbe", &self.sbe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Unsbesr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Unsbesr {
            sb_id: u8,
            sbe: bool,
        }
        let proxy = Unsbesr {
            sb_id: self.sb_id(),
            sbe: self.sbe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VLAN to DR mapping profile v register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vlandrmpr(pub u32);
impl Vlandrmpr {
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "DR value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "DR value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Vlandrmpr {
    #[inline(always)]
    fn default() -> Vlandrmpr {
        Vlandrmpr(2290649224u64 as u32)
    }
}
impl core::fmt::Debug for Vlandrmpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vlandrmpr")
            .field("pcp_dei_0", &self.pcp_dei_0())
            .field("pcp_dei_1", &self.pcp_dei_1())
            .field("pcp_dei_2", &self.pcp_dei_2())
            .field("pcp_dei_3", &self.pcp_dei_3())
            .field("pcp_dei_4", &self.pcp_dei_4())
            .field("pcp_dei_5", &self.pcp_dei_5())
            .field("pcp_dei_6", &self.pcp_dei_6())
            .field("pcp_dei_7", &self.pcp_dei_7())
            .field("pcp_dei_8", &self.pcp_dei_8())
            .field("pcp_dei_9", &self.pcp_dei_9())
            .field("pcp_dei_10", &self.pcp_dei_10())
            .field("pcp_dei_11", &self.pcp_dei_11())
            .field("pcp_dei_12", &self.pcp_dei_12())
            .field("pcp_dei_13", &self.pcp_dei_13())
            .field("pcp_dei_14", &self.pcp_dei_14())
            .field("pcp_dei_15", &self.pcp_dei_15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vlandrmpr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vlandrmpr {
            pcp_dei_0: u8,
            pcp_dei_1: u8,
            pcp_dei_2: u8,
            pcp_dei_3: u8,
            pcp_dei_4: u8,
            pcp_dei_5: u8,
            pcp_dei_6: u8,
            pcp_dei_7: u8,
            pcp_dei_8: u8,
            pcp_dei_9: u8,
            pcp_dei_10: u8,
            pcp_dei_11: u8,
            pcp_dei_12: u8,
            pcp_dei_13: u8,
            pcp_dei_14: u8,
            pcp_dei_15: u8,
        }
        let proxy = Vlandrmpr {
            pcp_dei_0: self.pcp_dei_0(),
            pcp_dei_1: self.pcp_dei_1(),
            pcp_dei_2: self.pcp_dei_2(),
            pcp_dei_3: self.pcp_dei_3(),
            pcp_dei_4: self.pcp_dei_4(),
            pcp_dei_5: self.pcp_dei_5(),
            pcp_dei_6: self.pcp_dei_6(),
            pcp_dei_7: self.pcp_dei_7(),
            pcp_dei_8: self.pcp_dei_8(),
            pcp_dei_9: self.pcp_dei_9(),
            pcp_dei_10: self.pcp_dei_10(),
            pcp_dei_11: self.pcp_dei_11(),
            pcp_dei_12: self.pcp_dei_12(),
            pcp_dei_13: self.pcp_dei_13(),
            pcp_dei_14: self.pcp_dei_14(),
            pcp_dei_15: self.pcp_dei_15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VLAN to IPV mapping profile v register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vlanipvmpr0(pub u32);
impl Vlanipvmpr0 {
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_4(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_5(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_6(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_7(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Vlanipvmpr0 {
    #[inline(always)]
    fn default() -> Vlanipvmpr0 {
        Vlanipvmpr0(857870592u64 as u32)
    }
}
impl core::fmt::Debug for Vlanipvmpr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vlanipvmpr0")
            .field("pcp_dei_0", &self.pcp_dei_0())
            .field("pcp_dei_1", &self.pcp_dei_1())
            .field("pcp_dei_2", &self.pcp_dei_2())
            .field("pcp_dei_3", &self.pcp_dei_3())
            .field("pcp_dei_4", &self.pcp_dei_4())
            .field("pcp_dei_5", &self.pcp_dei_5())
            .field("pcp_dei_6", &self.pcp_dei_6())
            .field("pcp_dei_7", &self.pcp_dei_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vlanipvmpr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vlanipvmpr0 {
            pcp_dei_0: u8,
            pcp_dei_1: u8,
            pcp_dei_2: u8,
            pcp_dei_3: u8,
            pcp_dei_4: u8,
            pcp_dei_5: u8,
            pcp_dei_6: u8,
            pcp_dei_7: u8,
        }
        let proxy = Vlanipvmpr0 {
            pcp_dei_0: self.pcp_dei_0(),
            pcp_dei_1: self.pcp_dei_1(),
            pcp_dei_2: self.pcp_dei_2(),
            pcp_dei_3: self.pcp_dei_3(),
            pcp_dei_4: self.pcp_dei_4(),
            pcp_dei_5: self.pcp_dei_5(),
            pcp_dei_6: self.pcp_dei_6(),
            pcp_dei_7: self.pcp_dei_7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "VLAN to IPV mapping profile v register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vlanipvmpr1(pub u32);
impl Vlanipvmpr1 {
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_9(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_10(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_11(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_12(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_13(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_14(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "IPV value used for receive data path."]
    #[must_use]
    #[inline(always)]
    pub const fn pcp_dei_15(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "IPV value used for receive data path."]
    #[inline(always)]
    pub const fn set_pcp_dei_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for Vlanipvmpr1 {
    #[inline(always)]
    fn default() -> Vlanipvmpr1 {
        Vlanipvmpr1(2003195204u64 as u32)
    }
}
impl core::fmt::Debug for Vlanipvmpr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vlanipvmpr1")
            .field("pcp_dei_8", &self.pcp_dei_8())
            .field("pcp_dei_9", &self.pcp_dei_9())
            .field("pcp_dei_10", &self.pcp_dei_10())
            .field("pcp_dei_11", &self.pcp_dei_11())
            .field("pcp_dei_12", &self.pcp_dei_12())
            .field("pcp_dei_13", &self.pcp_dei_13())
            .field("pcp_dei_14", &self.pcp_dei_14())
            .field("pcp_dei_15", &self.pcp_dei_15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vlanipvmpr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Vlanipvmpr1 {
            pcp_dei_8: u8,
            pcp_dei_9: u8,
            pcp_dei_10: u8,
            pcp_dei_11: u8,
            pcp_dei_12: u8,
            pcp_dei_13: u8,
            pcp_dei_14: u8,
            pcp_dei_15: u8,
        }
        let proxy = Vlanipvmpr1 {
            pcp_dei_8: self.pcp_dei_8(),
            pcp_dei_9: self.pcp_dei_9(),
            pcp_dei_10: self.pcp_dei_10(),
            pcp_dei_11: self.pcp_dei_11(),
            pcp_dei_12: self.pcp_dei_12(),
            pcp_dei_13: self.pcp_dei_13(),
            pcp_dei_14: self.pcp_dei_14(),
            pcp_dei_15: self.pcp_dei_15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
