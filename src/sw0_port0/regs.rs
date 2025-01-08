#[doc = "Bridge port configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpcr(pub u32);
impl Bpcr {
    #[doc = "Dynamic Limit"]
    #[must_use]
    #[inline(always)]
    pub const fn dyn_limit(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Dynamic Limit"]
    #[inline(always)]
    pub const fn set_dyn_limit(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Unknown Unicast Storm Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn uucaste(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Unknown Unicast Storm Control Enable"]
    #[inline(always)]
    pub const fn set_uucaste(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Unknown Multicast Storm Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn umcaste(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Unknown Multicast Storm Control Enable"]
    #[inline(always)]
    pub const fn set_umcaste(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Multicast Storm Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn mcaste(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Multicast Storm Control Enable"]
    #[inline(always)]
    pub const fn set_mcaste(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Broadcast Storm Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bcaste(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Broadcast Storm Control Enable"]
    #[inline(always)]
    pub const fn set_bcaste(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Station Move Disallow"]
    #[must_use]
    #[inline(always)]
    pub const fn stamvd(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Station Move Disallow"]
    #[inline(always)]
    pub const fn set_stamvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Source Port Pruning Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn srcprnd(&self) -> super::vals::Srcprnd {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Srcprnd::from_bits(val as u8)
    }
    #[doc = "Source Port Pruning Disable"]
    #[inline(always)]
    pub const fn set_srcprnd(&mut self, val: super::vals::Srcprnd) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
}
impl Default for Bpcr {
    #[inline(always)]
    fn default() -> Bpcr {
        Bpcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Bpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpcr")
            .field("dyn_limit", &self.dyn_limit())
            .field("uucaste", &self.uucaste())
            .field("umcaste", &self.umcaste())
            .field("mcaste", &self.mcaste())
            .field("bcaste", &self.bcaste())
            .field("stamvd", &self.stamvd())
            .field("srcprnd", &self.srcprnd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpcr {
            dyn_limit: u16,
            uucaste: bool,
            umcaste: bool,
            mcaste: bool,
            bcaste: bool,
            stamvd: bool,
            srcprnd: super::vals::Srcprnd,
        }
        let proxy = Bpcr {
            dyn_limit: self.dyn_limit(),
            uucaste: self.uucaste(),
            umcaste: self.umcaste(),
            mcaste: self.mcaste(),
            bcaste: self.bcaste(),
            stamvd: self.stamvd(),
            srcprnd: self.srcprnd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge port discard count reason register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpdcrr0(pub u32);
impl Bpdcrr0 {
    #[doc = "Discard due to Bridge Port Acceptance Criteria Discard Reason Frame was discarded because it failed the Bridge Port Tag Acceptance Criteria"]
    #[must_use]
    #[inline(always)]
    pub const fn bpacdr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Discard due to Bridge Port Acceptance Criteria Discard Reason Frame was discarded because it failed the Bridge Port Tag Acceptance Criteria"]
    #[inline(always)]
    pub const fn set_bpacdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Ingress STG State Discard Reason Occurs if port's STG State is Disabled or Learning"]
    #[must_use]
    #[inline(always)]
    pub const fn istgsdr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress STG State Discard Reason Occurs if port's STG State is Disabled or Learning"]
    #[inline(always)]
    pub const fn set_istgsdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "VLAN Filter (Port VLAN Membership) Discard Reason Occurs when VID (after ingress frame modification if applicable) received is not a member of this port"]
    #[must_use]
    #[inline(always)]
    pub const fn bpvfltdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN Filter (Port VLAN Membership) Discard Reason Occurs when VID (after ingress frame modification if applicable) received is not a member of this port"]
    #[inline(always)]
    pub const fn set_bpvfltdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "MAC Learn Not Found Discard Reason Occurs if VFHTDECR2\\[MLO\\] or VLAN Filter Table Entry's MLO = 5 (Disable MAC learning with SMAC validation)"]
    #[must_use]
    #[inline(always)]
    pub const fn maclnfdr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Learn Not Found Discard Reason Occurs if VFHTDECR2\\[MLO\\] or VLAN Filter Table Entry's MLO = 5 (Disable MAC learning with SMAC validation)"]
    #[inline(always)]
    pub const fn set_maclnfdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Station Move Disallowed Discard Reason Occurs if STAMVD=1 and Source MAC found in FDB with port not matching received port"]
    #[must_use]
    #[inline(always)]
    pub const fn stamvddr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Station Move Disallowed Discard Reason Occurs if STAMVD=1 and Source MAC found in FDB with port not matching received port"]
    #[inline(always)]
    pub const fn set_stamvddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "MAC Forwarding Default Discard Reason Occurs if MFO = 3 (FDB lookup is performed, and if there is no match, the frame is discarded), MFO is settable by BPCR and VLAN Filter Table Entry"]
    #[must_use]
    #[inline(always)]
    pub const fn macfdddr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "MAC Forwarding Default Discard Reason Occurs if MFO = 3 (FDB lookup is performed, and if there is no match, the frame is discarded), MFO is settable by BPCR and VLAN Filter Table Entry"]
    #[inline(always)]
    pub const fn set_macfdddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "No Destination Discard Reason L2 forwarding decision resulted in discarding the frame since it resulted in no destination"]
    #[must_use]
    #[inline(always)]
    pub const fn nodestdr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "No Destination Discard Reason L2 forwarding decision resulted in discarding the frame since it resulted in no destination"]
    #[inline(always)]
    pub const fn set_nodestdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "IP Multicast Filter Discard Reason Frame was discarded due to a miss in L2 IPV4 Multicast Filter table"]
    #[must_use]
    #[inline(always)]
    pub const fn ipmfdr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "IP Multicast Filter Discard Reason Frame was discarded due to a miss in L2 IPV4 Multicast Filter table"]
    #[inline(always)]
    pub const fn set_ipmfdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Untagged Frame Modification Misconfiguration Discard Reason Frame was discarded due to bridge port configured as VLAN aware with a non-null Ingress Frame Modification Entry ID (IFM_EID) resulting in the frame containing no VLAN headers, or with an outer priority tag (VID=0)"]
    #[must_use]
    #[inline(always)]
    pub const fn ufmmdr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Untagged Frame Modification Misconfiguration Discard Reason Frame was discarded due to bridge port configured as VLAN aware with a non-null Ingress Frame Modification Entry ID (IFM_EID) resulting in the frame containing no VLAN headers, or with an outer priority tag (VID=0)"]
    #[inline(always)]
    pub const fn set_ufmmdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Misconfiguration Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn miscdr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Misconfiguration Discard Reason"]
    #[inline(always)]
    pub const fn set_miscdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Discard due to Storm Control Policer Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn strmctrldr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Discard due to Storm Control Policer Discard Reason"]
    #[inline(always)]
    pub const fn set_strmctrldr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Bpdcrr0 {
    #[inline(always)]
    fn default() -> Bpdcrr0 {
        Bpdcrr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Bpdcrr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpdcrr0")
            .field("bpacdr", &self.bpacdr())
            .field("istgsdr", &self.istgsdr())
            .field("bpvfltdr", &self.bpvfltdr())
            .field("maclnfdr", &self.maclnfdr())
            .field("stamvddr", &self.stamvddr())
            .field("macfdddr", &self.macfdddr())
            .field("nodestdr", &self.nodestdr())
            .field("ipmfdr", &self.ipmfdr())
            .field("ufmmdr", &self.ufmmdr())
            .field("miscdr", &self.miscdr())
            .field("strmctrldr", &self.strmctrldr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpdcrr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpdcrr0 {
            bpacdr: bool,
            istgsdr: bool,
            bpvfltdr: bool,
            maclnfdr: bool,
            stamvddr: bool,
            macfdddr: bool,
            nodestdr: bool,
            ipmfdr: bool,
            ufmmdr: bool,
            miscdr: bool,
            strmctrldr: bool,
        }
        let proxy = Bpdcrr0 {
            bpacdr: self.bpacdr(),
            istgsdr: self.istgsdr(),
            bpvfltdr: self.bpvfltdr(),
            maclnfdr: self.maclnfdr(),
            stamvddr: self.stamvddr(),
            macfdddr: self.macfdddr(),
            nodestdr: self.nodestdr(),
            ipmfdr: self.ipmfdr(),
            ufmmdr: self.ufmmdr(),
            miscdr: self.miscdr(),
            strmctrldr: self.strmctrldr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge port discard count reason register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpdcrr1(pub u32);
impl Bpdcrr1 {
    #[doc = "Entry ID who last incremented Discard Count"]
    #[must_use]
    #[inline(always)]
    pub const fn entryid(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Entry ID who last incremented Discard Count"]
    #[inline(always)]
    pub const fn set_entryid(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 0usize)) | (((val as u32) & 0x07ff_ffff) << 0usize);
    }
    #[doc = "Table type which caused the discard"]
    #[must_use]
    #[inline(always)]
    pub const fn tt(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Table type which caused the discard"]
    #[inline(always)]
    pub const fn set_tt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Bpdcrr1 {
    #[inline(always)]
    fn default() -> Bpdcrr1 {
        Bpdcrr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Bpdcrr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpdcrr1")
            .field("entryid", &self.entryid())
            .field("tt", &self.tt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpdcrr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpdcrr1 {
            entryid: u32,
            tt: u8,
        }
        let proxy = Bpdcrr1 {
            entryid: self.entryid(),
            tt: self.tt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge port default VLAN register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpdvr(pub u32);
impl Bpdvr {
    #[doc = "VLAN identifier Specifies the VID value to be used to construct or modify the internal VLAN header."]
    #[must_use]
    #[inline(always)]
    pub const fn vid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "VLAN identifier Specifies the VID value to be used to construct or modify the internal VLAN header."]
    #[inline(always)]
    pub const fn set_vid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Drop eligible indicator This field specifies the 1-bit DEI to be used to construct the internal VLAN header"]
    #[must_use]
    #[inline(always)]
    pub const fn dei(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Drop eligible indicator This field specifies the 1-bit DEI to be used to construct the internal VLAN header"]
    #[inline(always)]
    pub const fn set_dei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Priority code point This field specifies the 3-bit PCP to be used to construct the internal VLAN header"]
    #[must_use]
    #[inline(always)]
    pub const fn pcp(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Priority code point This field specifies the 3-bit PCP to be used to construct the internal VLAN header"]
    #[inline(always)]
    pub const fn set_pcp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Specifies the TPID value to be used to construct the internal VLAN header"]
    #[must_use]
    #[inline(always)]
    pub const fn tpid(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies the TPID value to be used to construct the internal VLAN header"]
    #[inline(always)]
    pub const fn set_tpid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Receive Bridge Port Tag Acceptance Criteria xxx0 Discard untagged xxx1 Accept untagged xx0x Discard priority tagged xx1x Accept priority tagged x0xx Discard single tagged x1xx Accept single tagged 0xxx Discard double tagged 1xxx Accept double tagged Frames discarded are counted against the bridge port discard count register (BPDCR) along with the setting of the Discard due to Bridge Port Acceptance Criteria Discard Reason (BPACDR) flag to 1 in the Bridge port discard count reason register 0 (BPDCRR0)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxtaga(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive Bridge Port Tag Acceptance Criteria xxx0 Discard untagged xxx1 Accept untagged xx0x Discard priority tagged xx1x Accept priority tagged x0xx Discard single tagged x1xx Accept single tagged 0xxx Discard double tagged 1xxx Accept double tagged Frames discarded are counted against the bridge port discard count register (BPDCR) along with the setting of the Discard due to Bridge Port Acceptance Criteria Discard Reason (BPACDR) flag to 1 in the Bridge port discard count reason register 0 (BPDCRR0)"]
    #[inline(always)]
    pub const fn set_rxtaga(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Receive VLAN Aware Mode All frames to be forwarded using the 802"]
    #[must_use]
    #[inline(always)]
    pub const fn rxvam(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Receive VLAN Aware Mode All frames to be forwarded using the 802"]
    #[inline(always)]
    pub const fn set_rxvam(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Transmit Bridge Port VLAN Tag Action This field applies only for a VLAN aware bridge, where the frame outer VLAN tag's VID is equal to the port default VID (VID field in this register)"]
    #[must_use]
    #[inline(always)]
    pub const fn txtaga(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "Transmit Bridge Port VLAN Tag Action This field applies only for a VLAN aware bridge, where the frame outer VLAN tag's VID is equal to the port default VID (VID field in this register)"]
    #[inline(always)]
    pub const fn set_txtaga(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
}
impl Default for Bpdvr {
    #[inline(always)]
    fn default() -> Bpdvr {
        Bpdvr(32505856u64 as u32)
    }
}
impl core::fmt::Debug for Bpdvr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpdvr")
            .field("vid", &self.vid())
            .field("dei", &self.dei())
            .field("pcp", &self.pcp())
            .field("tpid", &self.tpid())
            .field("rxtaga", &self.rxtaga())
            .field("rxvam", &self.rxvam())
            .field("txtaga", &self.txtaga())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpdvr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpdvr {
            vid: u16,
            dei: bool,
            pcp: u8,
            tpid: bool,
            rxtaga: u8,
            rxvam: bool,
            txtaga: u8,
        }
        let proxy = Bpdvr {
            vid: self.vid(),
            dei: self.dei(),
            pcp: self.pcp(),
            tpid: self.tpid(),
            rxtaga: self.rxtaga(),
            rxvam: self.rxvam(),
            txtaga: self.txtaga(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge port MAC learning failure status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpmlfsr(pub u32);
impl Bpmlfsr {
    #[doc = "Bridge Port MAC Learn Limit Reached Failure Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn bpmllrfr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bridge Port MAC Learn Limit Reached Failure Reason"]
    #[inline(always)]
    pub const fn set_bpmllrfr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Full FDB Table Reached Failure Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn ffdbtrfr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Full FDB Table Reached Failure Reason"]
    #[inline(always)]
    pub const fn set_ffdbtrfr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Hash Collision chain limit Reached Failure Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn hcclrfr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Hash Collision chain limit Reached Failure Reason"]
    #[inline(always)]
    pub const fn set_hcclrfr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Bpmlfsr {
    #[inline(always)]
    fn default() -> Bpmlfsr {
        Bpmlfsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Bpmlfsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpmlfsr")
            .field("bpmllrfr", &self.bpmllrfr())
            .field("ffdbtrfr", &self.ffdbtrfr())
            .field("hcclrfr", &self.hcclrfr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpmlfsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpmlfsr {
            bpmllrfr: bool,
            ffdbtrfr: bool,
            hcclrfr: bool,
        }
        let proxy = Bpmlfsr {
            bpmllrfr: self.bpmllrfr(),
            ffdbtrfr: self.ffdbtrfr(),
            hcclrfr: self.hcclrfr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge port operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpor(pub u32);
impl Bpor {
    #[doc = "Number of FDB entries dynamically learned against this port."]
    #[must_use]
    #[inline(always)]
    pub const fn num_dyn(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of FDB entries dynamically learned against this port."]
    #[inline(always)]
    pub const fn set_num_dyn(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bpor {
    #[inline(always)]
    fn default() -> Bpor {
        Bpor(0u64 as u32)
    }
}
impl core::fmt::Debug for Bpor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpor")
            .field("num_dyn", &self.num_dyn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpor {
            num_dyn: u16,
        }
        let proxy = Bpor {
            num_dyn: self.num_dyn(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge port storm control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpscr0(pub u32);
impl Bpscr0 {
    #[doc = "Unknown unicast Rate Policer Entry ID"]
    #[must_use]
    #[inline(always)]
    pub const fn uucastrpeid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Unknown unicast Rate Policer Entry ID"]
    #[inline(always)]
    pub const fn set_uucastrpeid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Broadcast Rate Policer Entry ID"]
    #[must_use]
    #[inline(always)]
    pub const fn bcastrpeid(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Broadcast Rate Policer Entry ID"]
    #[inline(always)]
    pub const fn set_bcastrpeid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Bpscr0 {
    #[inline(always)]
    fn default() -> Bpscr0 {
        Bpscr0(268374015u64 as u32)
    }
}
impl core::fmt::Debug for Bpscr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpscr0")
            .field("uucastrpeid", &self.uucastrpeid())
            .field("bcastrpeid", &self.bcastrpeid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpscr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpscr0 {
            uucastrpeid: u16,
            bcastrpeid: u16,
        }
        let proxy = Bpscr0 {
            uucastrpeid: self.uucastrpeid(),
            bcastrpeid: self.bcastrpeid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge port storm control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpscr1(pub u32);
impl Bpscr1 {
    #[doc = "Known multicast Rate Policer Entry ID"]
    #[must_use]
    #[inline(always)]
    pub const fn mcastrpeid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Known multicast Rate Policer Entry ID"]
    #[inline(always)]
    pub const fn set_mcastrpeid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Unknown multicast Rate Policer Entry ID"]
    #[must_use]
    #[inline(always)]
    pub const fn umcastrpeid(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Unknown multicast Rate Policer Entry ID"]
    #[inline(always)]
    pub const fn set_umcastrpeid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Bpscr1 {
    #[inline(always)]
    fn default() -> Bpscr1 {
        Bpscr1(268374015u64 as u32)
    }
}
impl core::fmt::Debug for Bpscr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpscr1")
            .field("mcastrpeid", &self.mcastrpeid())
            .field("umcastrpeid", &self.umcastrpeid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpscr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpscr1 {
            mcastrpeid: u16,
            umcastrpeid: u16,
        }
        let proxy = Bpscr1 {
            mcastrpeid: self.mcastrpeid(),
            umcastrpeid: self.umcastrpeid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bridge port spanning tree group state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bpstgsr(pub u32);
impl Bpstgsr {
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[must_use]
    #[inline(always)]
    pub const fn stg_state15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Each STG_STATEi field is defined as follows, where i = spanning tree protocol group = {0"]
    #[inline(always)]
    pub const fn set_stg_state15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Bpstgsr {
    #[inline(always)]
    fn default() -> Bpstgsr {
        Bpstgsr(2u64 as u32)
    }
}
impl core::fmt::Debug for Bpstgsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bpstgsr")
            .field("stg_state0", &self.stg_state0())
            .field("stg_state1", &self.stg_state1())
            .field("stg_state2", &self.stg_state2())
            .field("stg_state3", &self.stg_state3())
            .field("stg_state4", &self.stg_state4())
            .field("stg_state5", &self.stg_state5())
            .field("stg_state6", &self.stg_state6())
            .field("stg_state7", &self.stg_state7())
            .field("stg_state8", &self.stg_state8())
            .field("stg_state9", &self.stg_state9())
            .field("stg_state10", &self.stg_state10())
            .field("stg_state11", &self.stg_state11())
            .field("stg_state12", &self.stg_state12())
            .field("stg_state13", &self.stg_state13())
            .field("stg_state14", &self.stg_state14())
            .field("stg_state15", &self.stg_state15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bpstgsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bpstgsr {
            stg_state0: u8,
            stg_state1: u8,
            stg_state2: u8,
            stg_state3: u8,
            stg_state4: u8,
            stg_state5: u8,
            stg_state6: u8,
            stg_state7: u8,
            stg_state8: u8,
            stg_state9: u8,
            stg_state10: u8,
            stg_state11: u8,
            stg_state12: u8,
            stg_state13: u8,
            stg_state14: u8,
            stg_state15: u8,
        }
        let proxy = Bpstgsr {
            stg_state0: self.stg_state0(),
            stg_state1: self.stg_state1(),
            stg_state2: self.stg_state2(),
            stg_state3: self.stg_state3(),
            stg_state4: self.stg_state4(),
            stg_state5: self.stg_state5(),
            stg_state6: self.stg_state6(),
            stg_state7: self.stg_state7(),
            stg_state8: self.stg_state8(),
            stg_state9: self.stg_state9(),
            stg_state10: self.stg_state10(),
            stg_state11: self.stg_state11(),
            stg_state12: self.stg_state12(),
            stg_state13: self.stg_state13(),
            stg_state14: self.stg_state14(),
            stg_state15: self.stg_state15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port buffer pool mapping configuration register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pbpmcr0(pub u32);
impl Pbpmcr0 {
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv0_index(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[inline(always)]
    pub const fn set_ipv0_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv1_index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[inline(always)]
    pub const fn set_ipv1_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv2_index(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[inline(always)]
    pub const fn set_ipv2_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv3_index(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[inline(always)]
    pub const fn set_ipv3_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pbpmcr0 {
    #[inline(always)]
    fn default() -> Pbpmcr0 {
        Pbpmcr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Pbpmcr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pbpmcr0")
            .field("ipv0_index", &self.ipv0_index())
            .field("ipv1_index", &self.ipv1_index())
            .field("ipv2_index", &self.ipv2_index())
            .field("ipv3_index", &self.ipv3_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pbpmcr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pbpmcr0 {
            ipv0_index: u8,
            ipv1_index: u8,
            ipv2_index: u8,
            ipv3_index: u8,
        }
        let proxy = Pbpmcr0 {
            ipv0_index: self.ipv0_index(),
            ipv1_index: self.ipv1_index(),
            ipv2_index: self.ipv2_index(),
            ipv3_index: self.ipv3_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port buffer pool mapping configuration register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pbpmcr1(pub u32);
impl Pbpmcr1 {
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv4_index(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[inline(always)]
    pub const fn set_ipv4_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv5_index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[inline(always)]
    pub const fn set_ipv5_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv6_index(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[inline(always)]
    pub const fn set_ipv6_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv7_index(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Indicates its associated Buffer Pool table Entry ID. Range: 0..BPCAPR\\[NUM_BP\\]-1"]
    #[inline(always)]
    pub const fn set_ipv7_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Pbpmcr1 {
    #[inline(always)]
    fn default() -> Pbpmcr1 {
        Pbpmcr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Pbpmcr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pbpmcr1")
            .field("ipv4_index", &self.ipv4_index())
            .field("ipv5_index", &self.ipv5_index())
            .field("ipv6_index", &self.ipv6_index())
            .field("ipv7_index", &self.ipv7_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pbpmcr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pbpmcr1 {
            ipv4_index: u8,
            ipv5_index: u8,
            ipv6_index: u8,
            ipv7_index: u8,
        }
        let proxy = Pbpmcr1 {
            ipv4_index: self.ipv4_index(),
            ipv5_index: self.ipv5_index(),
            ipv6_index: self.ipv6_index(),
            ipv7_index: self.ipv7_index(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcapr(pub u32);
impl Pcapr {
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
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported Formula: NUM_Q+1 Valid if link is bound to a switch"]
    #[must_use]
    #[inline(always)]
    pub const fn num_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of Egress Traffic Management (ETM) class queues supported Formula: NUM_Q+1 Valid if link is bound to a switch"]
    #[inline(always)]
    pub const fn set_num_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Number of congestion groups supported Formula: NUM_CG+1 Valid if link end is bound to a switch port"]
    #[must_use]
    #[inline(always)]
    pub const fn num_cg(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of congestion groups supported Formula: NUM_CG+1 Valid if link end is bound to a switch port"]
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
impl Default for Pcapr {
    #[inline(always)]
    fn default() -> Pcapr {
        Pcapr(923234304u64 as u32)
    }
}
impl core::fmt::Debug for Pcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcapr")
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
impl defmt::Format for Pcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pcapr {
            link_type: bool,
            num_tc: u8,
            num_q: u8,
            num_cg: u8,
            tgs: bool,
            cbs: bool,
        }
        let proxy = Pcapr {
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
#[doc = "Port configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc = "Indicates the frame format received/transmitted on the port 0 Ethernet frame format 1 Reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn hdr_fmt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the frame format received/transmitted on the port 0 Ethernet frame format 1 Reserved"]
    #[inline(always)]
    pub const fn set_hdr_fmt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "L2 Ethernet DoS Protection Enable 0 L2 IP DoS protection is disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn l2dose(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "L2 Ethernet DoS Protection Enable 0 L2 IP DoS protection is disabled"]
    #[inline(always)]
    pub const fn set_l2dose(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Timer Clock Selection: On receive, this field determines which of the two Rx timestamps (synchronized or free running) is reported to the host"]
    #[must_use]
    #[inline(always)]
    pub const fn timer_cs(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Timer Clock Selection: On receive, this field determines which of the two Rx timestamps (synchronized or free running) is reported to the host"]
    #[inline(always)]
    pub const fn set_timer_cs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmit Port Speed The speed in 10Mbps increments at which the port is assumed to be running for scheduling purposes and to determine if cut-through forwarding can proceed"]
    #[must_use]
    #[inline(always)]
    pub const fn pspeed(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x3fff;
        val as u16
    }
    #[doc = "Transmit Port Speed The speed in 10Mbps increments at which the port is assumed to be running for scheduling purposes and to determine if cut-through forwarding can proceed"]
    #[inline(always)]
    pub const fn set_pspeed(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 16usize)) | (((val as u32) & 0x3fff) << 16usize);
    }
}
impl Default for Pcr {
    #[inline(always)]
    fn default() -> Pcr {
        Pcr(6488064u64 as u32)
    }
}
impl core::fmt::Debug for Pcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr")
            .field("hdr_fmt", &self.hdr_fmt())
            .field("l2dose", &self.l2dose())
            .field("timer_cs", &self.timer_cs())
            .field("pspeed", &self.pspeed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pcr {
            hdr_fmt: bool,
            l2dose: bool,
            timer_cs: bool,
            pspeed: u16,
        }
        let proxy = Pcr {
            hdr_fmt: self.hdr_fmt(),
            l2dose: self.l2dose(),
            timer_cs: self.timer_cs(),
            pspeed: self.pspeed(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port cut through forwarding configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pctfcr(pub u32);
impl Pctfcr {
    #[doc = "Ingress Cut Through State 0 Cut-through forwarding is allowed on this port 1 Cut-through forwarding is not permitted on this port"]
    #[must_use]
    #[inline(always)]
    pub const fn icts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Cut Through State 0 Cut-through forwarding is allowed on this port 1 Cut-through forwarding is not permitted on this port"]
    #[inline(always)]
    pub const fn set_icts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Egress Cut Through State 0 Cut-through frames are permitted on this port 1 Cut-through frames are not permitted on this port"]
    #[must_use]
    #[inline(always)]
    pub const fn ects(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Cut Through State 0 Cut-through frames are permitted on this port 1 Cut-through frames are not permitted on this port"]
    #[inline(always)]
    pub const fn set_ects(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Pctfcr {
    #[inline(always)]
    fn default() -> Pctfcr {
        Pctfcr(19u64 as u32)
    }
}
impl core::fmt::Debug for Pctfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pctfcr")
            .field("icts", &self.icts())
            .field("ects", &self.ects())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pctfcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pctfcr {
            icts: bool,
            ects: bool,
        }
        let proxy = Pctfcr {
            icts: self.icts(),
            ects: self.ects(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port default gate state register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdgsr(pub u32);
impl Pdgsr {
    #[doc = "Default Gate State for Traffic Class 0 Configures the default state of the port's traffic class gates"]
    #[must_use]
    #[inline(always)]
    pub const fn dgs_tc0(&self) -> super::vals::DgsTc0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::DgsTc0::from_bits(val as u8)
    }
    #[doc = "Default Gate State for Traffic Class 0 Configures the default state of the port's traffic class gates"]
    #[inline(always)]
    pub const fn set_dgs_tc0(&mut self, val: super::vals::DgsTc0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Default Gate State for Traffic Class 1 Configures the default state of the port's traffic class gates"]
    #[must_use]
    #[inline(always)]
    pub const fn dgs_tc1(&self) -> super::vals::DgsTc1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::DgsTc1::from_bits(val as u8)
    }
    #[doc = "Default Gate State for Traffic Class 1 Configures the default state of the port's traffic class gates"]
    #[inline(always)]
    pub const fn set_dgs_tc1(&mut self, val: super::vals::DgsTc1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Default Gate State for Traffic Class 2 Configures the default state of the port's traffic class gates"]
    #[must_use]
    #[inline(always)]
    pub const fn dgs_tc2(&self) -> super::vals::DgsTc2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::DgsTc2::from_bits(val as u8)
    }
    #[doc = "Default Gate State for Traffic Class 2 Configures the default state of the port's traffic class gates"]
    #[inline(always)]
    pub const fn set_dgs_tc2(&mut self, val: super::vals::DgsTc2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Default Gate State for Traffic Class 3 Configures the default state of the port's traffic class gates"]
    #[must_use]
    #[inline(always)]
    pub const fn dgs_tc3(&self) -> super::vals::DgsTc3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::DgsTc3::from_bits(val as u8)
    }
    #[doc = "Default Gate State for Traffic Class 3 Configures the default state of the port's traffic class gates"]
    #[inline(always)]
    pub const fn set_dgs_tc3(&mut self, val: super::vals::DgsTc3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Default Gate State for Traffic Class 4 Configures the default state of the port's traffic class gates"]
    #[must_use]
    #[inline(always)]
    pub const fn dgs_tc4(&self) -> super::vals::DgsTc4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::DgsTc4::from_bits(val as u8)
    }
    #[doc = "Default Gate State for Traffic Class 4 Configures the default state of the port's traffic class gates"]
    #[inline(always)]
    pub const fn set_dgs_tc4(&mut self, val: super::vals::DgsTc4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Default Gate State for Traffic Class 5 Configures the default state of the port's traffic class gates"]
    #[must_use]
    #[inline(always)]
    pub const fn dgs_tc5(&self) -> super::vals::DgsTc5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::DgsTc5::from_bits(val as u8)
    }
    #[doc = "Default Gate State for Traffic Class 5 Configures the default state of the port's traffic class gates"]
    #[inline(always)]
    pub const fn set_dgs_tc5(&mut self, val: super::vals::DgsTc5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Default Gate State for Traffic Class 6 Configures the default state of the port's traffic class gates"]
    #[must_use]
    #[inline(always)]
    pub const fn dgs_tc6(&self) -> super::vals::DgsTc6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::DgsTc6::from_bits(val as u8)
    }
    #[doc = "Default Gate State for Traffic Class 6 Configures the default state of the port's traffic class gates"]
    #[inline(always)]
    pub const fn set_dgs_tc6(&mut self, val: super::vals::DgsTc6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Default Gate State for Traffic Class 7 Configures the default state of the port's traffic class gates"]
    #[must_use]
    #[inline(always)]
    pub const fn dgs_tc7(&self) -> super::vals::DgsTc7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::DgsTc7::from_bits(val as u8)
    }
    #[doc = "Default Gate State for Traffic Class 7 Configures the default state of the port's traffic class gates"]
    #[inline(always)]
    pub const fn set_dgs_tc7(&mut self, val: super::vals::DgsTc7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
}
impl Default for Pdgsr {
    #[inline(always)]
    fn default() -> Pdgsr {
        Pdgsr(255u64 as u32)
    }
}
impl core::fmt::Debug for Pdgsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pdgsr")
            .field("dgs_tc0", &self.dgs_tc0())
            .field("dgs_tc1", &self.dgs_tc1())
            .field("dgs_tc2", &self.dgs_tc2())
            .field("dgs_tc3", &self.dgs_tc3())
            .field("dgs_tc4", &self.dgs_tc4())
            .field("dgs_tc5", &self.dgs_tc5())
            .field("dgs_tc6", &self.dgs_tc6())
            .field("dgs_tc7", &self.dgs_tc7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pdgsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pdgsr {
            dgs_tc0: super::vals::DgsTc0,
            dgs_tc1: super::vals::DgsTc1,
            dgs_tc2: super::vals::DgsTc2,
            dgs_tc3: super::vals::DgsTc3,
            dgs_tc4: super::vals::DgsTc4,
            dgs_tc5: super::vals::DgsTc5,
            dgs_tc6: super::vals::DgsTc6,
            dgs_tc7: super::vals::DgsTc7,
        }
        let proxy = Pdgsr {
            dgs_tc0: self.dgs_tc0(),
            dgs_tc1: self.dgs_tc1(),
            dgs_tc2: self.dgs_tc2(),
            dgs_tc3: self.dgs_tc3(),
            dgs_tc4: self.dgs_tc4(),
            dgs_tc5: self.dgs_tc5(),
            dgs_tc6: self.dgs_tc6(),
            dgs_tc7: self.dgs_tc7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port frame modification configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfmcr(pub u32);
impl Pfmcr {
    #[doc = "Frame Modification Misconfiguration Action"]
    #[must_use]
    #[inline(always)]
    pub const fn fmma(&self) -> super::vals::Fmma {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Fmma::from_bits(val as u8)
    }
    #[doc = "Frame Modification Misconfiguration Action"]
    #[inline(always)]
    pub const fn set_fmma(&mut self, val: super::vals::Fmma) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Pfmcr {
    #[inline(always)]
    fn default() -> Pfmcr {
        Pfmcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pfmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfmcr").field("fmma", &self.fmma()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfmcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pfmcr {
            fmma: super::vals::Fmma,
        }
        let proxy = Pfmcr { fmma: self.fmma() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port frame preemption configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfpcr(pub u32);
impl Pfpcr {
    #[doc = "Frame preemption enable for traffic class 0 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn fpe_tc0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Frame preemption enable for traffic class 0 0 Disabled"]
    #[inline(always)]
    pub const fn set_fpe_tc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Frame preemption enable for traffic class 1 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn fpe_tc1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Frame preemption enable for traffic class 1 0 Disabled"]
    #[inline(always)]
    pub const fn set_fpe_tc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Frame preemption enable for traffic class 2 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn fpe_tc2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Frame preemption enable for traffic class 2 0 Disabled"]
    #[inline(always)]
    pub const fn set_fpe_tc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Frame preemption enable for traffic class 3 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn fpe_tc3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Frame preemption enable for traffic class 3 0 Disabled"]
    #[inline(always)]
    pub const fn set_fpe_tc3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Frame preemption enable for traffic class 4 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn fpe_tc4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Frame preemption enable for traffic class 4 0 Disabled"]
    #[inline(always)]
    pub const fn set_fpe_tc4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame preemption enable for traffic class 5 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn fpe_tc5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame preemption enable for traffic class 5 0 Disabled"]
    #[inline(always)]
    pub const fn set_fpe_tc5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Frame preemption enable for traffic class 6 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn fpe_tc6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Frame preemption enable for traffic class 6 0 Disabled"]
    #[inline(always)]
    pub const fn set_fpe_tc6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Frame preemption enable for traffic class 7 0 Disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn fpe_tc7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Frame preemption enable for traffic class 7 0 Disabled"]
    #[inline(always)]
    pub const fn set_fpe_tc7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pfpcr {
    #[inline(always)]
    fn default() -> Pfpcr {
        Pfpcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pfpcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pfpcr")
            .field("fpe_tc0", &self.fpe_tc0())
            .field("fpe_tc1", &self.fpe_tc1())
            .field("fpe_tc2", &self.fpe_tc2())
            .field("fpe_tc3", &self.fpe_tc3())
            .field("fpe_tc4", &self.fpe_tc4())
            .field("fpe_tc5", &self.fpe_tc5())
            .field("fpe_tc6", &self.fpe_tc6())
            .field("fpe_tc7", &self.fpe_tc7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pfpcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pfpcr {
            fpe_tc0: bool,
            fpe_tc1: bool,
            fpe_tc2: bool,
            fpe_tc3: bool,
            fpe_tc4: bool,
            fpe_tc5: bool,
            fpe_tc6: bool,
            fpe_tc7: bool,
        }
        let proxy = Pfpcr {
            fpe_tc0: self.fpe_tc0(),
            fpe_tc1: self.fpe_tc1(),
            fpe_tc2: self.fpe_tc2(),
            fpe_tc3: self.fpe_tc3(),
            fpe_tc4: self.fpe_tc4(),
            fpe_tc5: self.fpe_tc5(),
            fpe_tc6: self.fpe_tc6(),
            fpe_tc7: self.fpe_tc7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port I/O capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Piocapr(pub u32);
impl Piocapr {
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
impl Default for Piocapr {
    #[inline(always)]
    fn default() -> Piocapr {
        Piocapr(268435456u64 as u32)
    }
}
impl core::fmt::Debug for Piocapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Piocapr")
            .field("pcs_prot", &self.pcs_prot())
            .field("io_var", &self.io_var())
            .field("emdio", &self.emdio())
            .field("revmii_rate", &self.revmii_rate())
            .field("revmii", &self.revmii())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Piocapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Piocapr {
            pcs_prot: u16,
            io_var: u8,
            emdio: bool,
            revmii_rate: bool,
            revmii: bool,
        }
        let proxy = Piocapr {
            pcs_prot: self.pcs_prot(),
            io_var: self.io_var(),
            emdio: self.emdio(),
            revmii_rate: self.revmii_rate(),
            revmii: self.revmii(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port ingress port filter configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipfcr(pub u32);
impl Pipfcr {
    #[doc = "0 Ingress Port Filter table lookup is disabled for this port 1 Ingress Port Filter table lookup is enabled for this port"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 Ingress Port Filter table lookup is disabled for this port 1 Ingress Port Filter table lookup is enabled for this port"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Pipfcr {
    #[inline(always)]
    fn default() -> Pipfcr {
        Pipfcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pipfcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipfcr").field("en", &self.en()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipfcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pipfcr {
            en: bool,
        }
        let proxy = Pipfcr { en: self.en() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port IPV to queue mapping register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pipv2qmr0(pub u32);
impl Pipv2qmr0 {
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv0_q(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[inline(always)]
    pub const fn set_ipv0_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv1_q(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[inline(always)]
    pub const fn set_ipv1_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv2_q(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[inline(always)]
    pub const fn set_ipv2_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv3_q(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[inline(always)]
    pub const fn set_ipv3_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv4_q(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[inline(always)]
    pub const fn set_ipv4_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv5_q(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[inline(always)]
    pub const fn set_ipv5_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv6_q(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[inline(always)]
    pub const fn set_ipv6_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[must_use]
    #[inline(always)]
    pub const fn ipv7_q(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Each IPVi_Q field is defined as follows (where i=IPV={0"]
    #[inline(always)]
    pub const fn set_ipv7_q(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Pipv2qmr0 {
    #[inline(always)]
    fn default() -> Pipv2qmr0 {
        Pipv2qmr0(1985229328u64 as u32)
    }
}
impl core::fmt::Debug for Pipv2qmr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pipv2qmr0")
            .field("ipv0_q", &self.ipv0_q())
            .field("ipv1_q", &self.ipv1_q())
            .field("ipv2_q", &self.ipv2_q())
            .field("ipv3_q", &self.ipv3_q())
            .field("ipv4_q", &self.ipv4_q())
            .field("ipv5_q", &self.ipv5_q())
            .field("ipv6_q", &self.ipv6_q())
            .field("ipv7_q", &self.ipv7_q())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pipv2qmr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pipv2qmr0 {
            ipv0_q: u8,
            ipv1_q: u8,
            ipv2_q: u8,
            ipv3_q: u8,
            ipv4_q: u8,
            ipv5_q: u8,
            ipv6_q: u8,
            ipv7_q: u8,
        }
        let proxy = Pipv2qmr0 {
            ipv0_q: self.ipv0_q(),
            ipv1_q: self.ipv1_q(),
            ipv2_q: self.ipv2_q(),
            ipv3_q: self.ipv3_q(),
            ipv4_q: self.ipv4_q(),
            ipv5_q: self.ipv5_q(),
            ipv6_q: self.ipv6_q(),
            ipv7_q: self.ipv7_q(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port ingress stream identification configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pisidcr(pub u32);
impl Pisidcr {
    #[doc = "Indicates which Ingress Stream Identification Key Construction pair to use for this port"]
    #[must_use]
    #[inline(always)]
    pub const fn kcpair(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates which Ingress Stream Identification Key Construction pair to use for this port"]
    #[inline(always)]
    pub const fn set_kcpair(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Key Construction 0 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn kc0en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Key Construction 0 Enable"]
    #[inline(always)]
    pub const fn set_kc0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Key Construction 1 Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn kc1en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Key Construction 1 Enable"]
    #[inline(always)]
    pub const fn set_kc1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Default Ingress Stream Entry ID"]
    #[must_use]
    #[inline(always)]
    pub const fn iseid(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Default Ingress Stream Entry ID"]
    #[inline(always)]
    pub const fn set_iseid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pisidcr {
    #[inline(always)]
    fn default() -> Pisidcr {
        Pisidcr(4294901760u64 as u32)
    }
}
impl core::fmt::Debug for Pisidcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pisidcr")
            .field("kcpair", &self.kcpair())
            .field("kc0en", &self.kc0en())
            .field("kc1en", &self.kc1en())
            .field("iseid", &self.iseid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pisidcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pisidcr {
            kcpair: bool,
            kc0en: bool,
            kc1en: bool,
            iseid: u16,
        }
        let proxy = Pisidcr {
            kcpair: self.kcpair(),
            kc0en: self.kc0en(),
            kc1en: self.kc1en(),
            iseid: self.iseid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port LANID configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Planidcr(pub u32);
impl Planidcr {
    #[doc = "LAN Identifier"]
    #[must_use]
    #[inline(always)]
    pub const fn lanid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "LAN Identifier"]
    #[inline(always)]
    pub const fn set_lanid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Planidcr {
    #[inline(always)]
    fn default() -> Planidcr {
        Planidcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Planidcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Planidcr")
            .field("lanid", &self.lanid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Planidcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Planidcr {
            lanid: u8,
        }
        let proxy = Planidcr {
            lanid: self.lanid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC address register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmar1(pub u32);
impl Pmar1 {
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
impl Default for Pmar1 {
    #[inline(always)]
    fn default() -> Pmar1 {
        Pmar1(0u64 as u32)
    }
}
impl core::fmt::Debug for Pmar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmar1")
            .field("prim_mac_addr", &self.prim_mac_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmar1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pmar1 {
            prim_mac_addr: u16,
        }
        let proxy = Pmar1 {
            prim_mac_addr: self.prim_mac_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC capability register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcapr(pub u32);
impl Pmcapr {
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
    #[doc = "Egress frame padding capability indicates if egress frames smaller than 64B are padded with zeroes"]
    #[must_use]
    #[inline(always)]
    pub const fn efpad(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Egress frame padding capability indicates if egress frames smaller than 64B are padded with zeroes"]
    #[inline(always)]
    pub const fn set_efpad(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Indicates if configurable Preamble and IPG is supported 0: Static Inter Frame Gap (IPG) size is 12B and Preamble is 7B 1: Configurable IPG and Preamble size Valid if MAC_VAR=1"]
    #[must_use]
    #[inline(always)]
    pub const fn pipg(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if configurable Preamble and IPG is supported 0: Static Inter Frame Gap (IPG) size is 12B and Preamble is 7B 1: Configurable IPG and Preamble size Valid if MAC_VAR=1"]
    #[inline(always)]
    pub const fn set_pipg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates if Half Duplex Mode is supported on this link"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if Half Duplex Mode is supported on this link"]
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
impl Default for Pmcapr {
    #[inline(always)]
    fn default() -> Pmcapr {
        Pmcapr(5473u64 as u32)
    }
}
impl core::fmt::Debug for Pmcapr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmcapr")
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
impl defmt::Format for Pmcapr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pmcapr {
            mac_var: u8,
            efpad: u8,
            pipg: bool,
            hd: bool,
            fp: u8,
            min_mpdu: bool,
            mii_prot: u8,
        }
        let proxy = Pmcapr {
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
#[doc = "Port mirror configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcr(pub u32);
impl Pmcr {
    #[doc = "Enable Ingress Mirroring on the port"]
    #[must_use]
    #[inline(always)]
    pub const fn imire(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Ingress Mirroring on the port"]
    #[inline(always)]
    pub const fn set_imire(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Pmcr {
    #[inline(always)]
    fn default() -> Pmcr {
        Pmcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmcr")
            .field("imire", &self.imire())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pmcr {
            imire: bool,
        }
        let proxy = Pmcr {
            imire: self.imire(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Por(pub u32);
impl Por {
    #[doc = "Tx Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn txdis(&self) -> super::vals::Txdis {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Txdis::from_bits(val as u8)
    }
    #[doc = "Tx Disable."]
    #[inline(always)]
    pub const fn set_txdis(&mut self, val: super::vals::Txdis) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Rx Disable."]
    #[must_use]
    #[inline(always)]
    pub const fn rxdis(&self) -> super::vals::Rxdis {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxdis::from_bits(val as u8)
    }
    #[doc = "Rx Disable."]
    #[inline(always)]
    pub const fn set_rxdis(&mut self, val: super::vals::Rxdis) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Por {
    #[inline(always)]
    fn default() -> Por {
        Por(0u64 as u32)
    }
}
impl core::fmt::Debug for Por {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Por")
            .field("txdis", &self.txdis())
            .field("rxdis", &self.rxdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Por {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Por {
            txdis: super::vals::Txdis,
            rxdis: super::vals::Rxdis,
        }
        let proxy = Por {
            txdis: self.txdis(),
            rxdis: self.rxdis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port PCP DEI mapping register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppcpdeimr(pub u32);
impl Ppcpdeimr {
    #[doc = "Ingress PCP to PCP Mapping Profile instance"]
    #[must_use]
    #[inline(always)]
    pub const fn ipcpmp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Ingress PCP to PCP Mapping Profile instance"]
    #[inline(always)]
    pub const fn set_ipcpmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Ingress PCP to PCP Mapping Profile is valid. Only applicable if outer VLAN tag is present."]
    #[must_use]
    #[inline(always)]
    pub const fn ipcpmpv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress PCP to PCP Mapping Profile is valid. Only applicable if outer VLAN tag is present."]
    #[inline(always)]
    pub const fn set_ipcpmpv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Egress PCP to PCP Mapping Profile instance"]
    #[must_use]
    #[inline(always)]
    pub const fn epcpmp(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Egress PCP to PCP Mapping Profile instance"]
    #[inline(always)]
    pub const fn set_epcpmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Egress PCP to PCP Mapping Profile is valid."]
    #[must_use]
    #[inline(always)]
    pub const fn epcpmpv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Egress PCP to PCP Mapping Profile is valid."]
    #[inline(always)]
    pub const fn set_epcpmpv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Mapping of internal QoS's DR value 0 to VLAN DEI."]
    #[must_use]
    #[inline(always)]
    pub const fn dr0dei(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal QoS's DR value 0 to VLAN DEI."]
    #[inline(always)]
    pub const fn set_dr0dei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mapping of internal QoS's DR value 1 to VLAN DEI."]
    #[must_use]
    #[inline(always)]
    pub const fn dr1dei(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal QoS's DR value 1 to VLAN DEI."]
    #[inline(always)]
    pub const fn set_dr1dei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Mapping of internal QoS's DR value 2 to VLAN DEI."]
    #[must_use]
    #[inline(always)]
    pub const fn dr2dei(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal QoS's DR value 2 to VLAN DEI."]
    #[inline(always)]
    pub const fn set_dr2dei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Mapping of internal QoS's DR value 3 to VLAN DEI."]
    #[must_use]
    #[inline(always)]
    pub const fn dr3dei(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Mapping of internal QoS's DR value 3 to VLAN DEI."]
    #[inline(always)]
    pub const fn set_dr3dei(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Indicates if mapping of internal QoS's DR value d to VLAN DEI is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn drme(&self) -> super::vals::Drme {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Drme::from_bits(val as u8)
    }
    #[doc = "Indicates if mapping of internal QoS's DR value d to VLAN DEI is enabled."]
    #[inline(always)]
    pub const fn set_drme(&mut self, val: super::vals::Drme) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
}
impl Default for Ppcpdeimr {
    #[inline(always)]
    fn default() -> Ppcpdeimr {
        Ppcpdeimr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppcpdeimr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppcpdeimr")
            .field("ipcpmp", &self.ipcpmp())
            .field("ipcpmpv", &self.ipcpmpv())
            .field("epcpmp", &self.epcpmp())
            .field("epcpmpv", &self.epcpmpv())
            .field("dr0dei", &self.dr0dei())
            .field("dr1dei", &self.dr1dei())
            .field("dr2dei", &self.dr2dei())
            .field("dr3dei", &self.dr3dei())
            .field("drme", &self.drme())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppcpdeimr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppcpdeimr {
            ipcpmp: u8,
            ipcpmpv: bool,
            epcpmp: u8,
            epcpmpv: bool,
            dr0dei: bool,
            dr1dei: bool,
            dr2dei: bool,
            dr3dei: bool,
            drme: super::vals::Drme,
        }
        let proxy = Ppcpdeimr {
            ipcpmp: self.ipcpmp(),
            ipcpmpv: self.ipcpmpv(),
            epcpmp: self.epcpmp(),
            epcpmpv: self.epcpmpv(),
            dr0dei: self.dr0dei(),
            dr1dei: self.dr1dei(),
            dr2dei: self.dr2dei(),
            dr3dei: self.dr3dei(),
            drme: self.drme(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port parser configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppcr(pub u32);
impl Ppcr {
    #[doc = "Unused"]
    #[must_use]
    #[inline(always)]
    pub const fn l1pfs(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[doc = "Unused"]
    #[inline(always)]
    pub const fn set_l1pfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[doc = "L2 payload fields size in bytes This is the largest L2 payload size used by any table lookup key by this port"]
    #[must_use]
    #[inline(always)]
    pub const fn l2pfs(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[doc = "L2 payload fields size in bytes This is the largest L2 payload size used by any table lookup key by this port"]
    #[inline(always)]
    pub const fn set_l2pfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[doc = "L3 header fields present"]
    #[must_use]
    #[inline(always)]
    pub const fn l3hfp(&self) -> super::vals::L3hfp {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::L3hfp::from_bits(val as u8)
    }
    #[doc = "L3 header fields present"]
    #[inline(always)]
    pub const fn set_l3hfp(&mut self, val: super::vals::L3hfp) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "L3 payload fields size in bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn l3pfs(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[doc = "L3 payload fields size in bytes"]
    #[inline(always)]
    pub const fn set_l3pfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[doc = "L4 Header fields present"]
    #[must_use]
    #[inline(always)]
    pub const fn l4hfp(&self) -> super::vals::L4hfp {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::L4hfp::from_bits(val as u8)
    }
    #[doc = "L4 Header fields present"]
    #[inline(always)]
    pub const fn set_l4hfp(&mut self, val: super::vals::L4hfp) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "L4 payload fields size in bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn l4pfs(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "L4 payload fields size in bytes"]
    #[inline(always)]
    pub const fn set_l4pfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for Ppcr {
    #[inline(always)]
    fn default() -> Ppcr {
        Ppcr(825307136u64 as u32)
    }
}
impl core::fmt::Debug for Ppcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppcr")
            .field("l1pfs", &self.l1pfs())
            .field("l2pfs", &self.l2pfs())
            .field("l3hfp", &self.l3hfp())
            .field("l3pfs", &self.l3pfs())
            .field("l4hfp", &self.l4hfp())
            .field("l4pfs", &self.l4pfs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppcr {
            l1pfs: u8,
            l2pfs: u8,
            l3hfp: super::vals::L3hfp,
            l3pfs: u8,
            l4hfp: super::vals::L4hfp,
            l4pfs: u8,
        }
        let proxy = Ppcr {
            l1pfs: self.l1pfs(),
            l2pfs: self.l2pfs(),
            l3hfp: self.l3hfp(),
            l3pfs: self.l3pfs(),
            l4hfp: self.l4hfp(),
            l4pfs: self.l4pfs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Queue Operational register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pqor(pub u32);
impl Pqor {
    #[doc = "Egress Traffic Management (ETM) class queue 0's state where i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn q0s(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Traffic Management (ETM) class queue 0's state where i={0"]
    #[inline(always)]
    pub const fn set_q0s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Egress Traffic Management (ETM) class queue 1's state where i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn q1s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Traffic Management (ETM) class queue 1's state where i={0"]
    #[inline(always)]
    pub const fn set_q1s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Egress Traffic Management (ETM) class queue 2's state where i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn q2s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Traffic Management (ETM) class queue 2's state where i={0"]
    #[inline(always)]
    pub const fn set_q2s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Egress Traffic Management (ETM) class queue 3's state where i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn q3s(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Traffic Management (ETM) class queue 3's state where i={0"]
    #[inline(always)]
    pub const fn set_q3s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Egress Traffic Management (ETM) class queue 4's state where i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn q4s(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Traffic Management (ETM) class queue 4's state where i={0"]
    #[inline(always)]
    pub const fn set_q4s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Egress Traffic Management (ETM) class queue 5's state where i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn q5s(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Traffic Management (ETM) class queue 5's state where i={0"]
    #[inline(always)]
    pub const fn set_q5s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Egress Traffic Management (ETM) class queue 6's state where i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn q6s(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Traffic Management (ETM) class queue 6's state where i={0"]
    #[inline(always)]
    pub const fn set_q6s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Egress Traffic Management (ETM) class queue 7's state where i={0"]
    #[must_use]
    #[inline(always)]
    pub const fn q7s(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Traffic Management (ETM) class queue 7's state where i={0"]
    #[inline(always)]
    pub const fn set_q7s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pqor {
    #[inline(always)]
    fn default() -> Pqor {
        Pqor(0u64 as u32)
    }
}
impl core::fmt::Debug for Pqor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pqor")
            .field("q0s", &self.q0s())
            .field("q1s", &self.q1s())
            .field("q2s", &self.q2s())
            .field("q3s", &self.q3s())
            .field("q4s", &self.q4s())
            .field("q5s", &self.q5s())
            .field("q6s", &self.q6s())
            .field("q7s", &self.q7s())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pqor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pqor {
            q0s: bool,
            q1s: bool,
            q2s: bool,
            q3s: bool,
            q4s: bool,
            q5s: bool,
            q6s: bool,
            q7s: bool,
        }
        let proxy = Pqor {
            q0s: self.q0s(),
            q1s: self.q1s(),
            q2s: self.q2s(),
            q3s: self.q3s(),
            q4s: self.q4s(),
            q5s: self.q5s(),
            q6s: self.q6s(),
            q7s: self.q7s(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port QoS mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pqosmr(pub u32);
impl Pqosmr {
    #[doc = "VLAN tag select: 0 Inner VLAN tag will be used if VE is set 1 Outer VLAN tag will be used if VE is set"]
    #[must_use]
    #[inline(always)]
    pub const fn vs(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN tag select: 0 Inner VLAN tag will be used if VE is set 1 Outer VLAN tag will be used if VE is set"]
    #[inline(always)]
    pub const fn set_vs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "VLAN enable 0 Defaults are used 1 Enables use of VLAN info to determine IPV and DR (based on VLANIPVMPaR0/1 and VLANDRMPaR)"]
    #[must_use]
    #[inline(always)]
    pub const fn ve(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "VLAN enable 0 Defaults are used 1 Enables use of VLAN info to determine IPV and DR (based on VLANIPVMPaR0/1 and VLANDRMPaR)"]
    #[inline(always)]
    pub const fn set_ve(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Default DR Sets the default DR for the port."]
    #[must_use]
    #[inline(always)]
    pub const fn ddr(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Default DR Sets the default DR for the port."]
    #[inline(always)]
    pub const fn set_ddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Default IPV Sets the default IPV for the port."]
    #[must_use]
    #[inline(always)]
    pub const fn dipv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Default IPV Sets the default IPV for the port."]
    #[inline(always)]
    pub const fn set_dipv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Mapping profile index"]
    #[must_use]
    #[inline(always)]
    pub const fn vqmp(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Mapping profile index"]
    #[inline(always)]
    pub const fn set_vqmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Mapping profile index"]
    #[must_use]
    #[inline(always)]
    pub const fn qvmp(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Mapping profile index"]
    #[inline(always)]
    pub const fn set_qvmp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for Pqosmr {
    #[inline(always)]
    fn default() -> Pqosmr {
        Pqosmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pqosmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pqosmr")
            .field("vs", &self.vs())
            .field("ve", &self.ve())
            .field("ddr", &self.ddr())
            .field("dipv", &self.dipv())
            .field("vqmp", &self.vqmp())
            .field("qvmp", &self.qvmp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pqosmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pqosmr {
            vs: bool,
            ve: bool,
            ddr: u8,
            dipv: u8,
            vqmp: u8,
            qvmp: u8,
        }
        let proxy = Pqosmr {
            vs: self.vs(),
            ve: self.ve(),
            ddr: self.ddr(),
            dipv: self.dipv(),
            vqmp: self.vqmp(),
            qvmp: self.qvmp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Rx discard count reason register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prxdcrr0(pub u32);
impl Prxdcrr0 {
    #[doc = "Pre-Classification Discard Reason Occurs if Parse Classifier Engine (PCE) block does not have any free processing thread to process the received frame"]
    #[must_use]
    #[inline(always)]
    pub const fn pcdr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pre-Classification Discard Reason Occurs if Parse Classifier Engine (PCE) block does not have any free processing thread to process the received frame"]
    #[inline(always)]
    pub const fn set_pcdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Shared Memory Resource Exhaustion Discard Reason Occurs if Ethernet Rx I/F cannot allocate shared internal buffer memory to store the entire received frame"]
    #[must_use]
    #[inline(always)]
    pub const fn smredr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Shared Memory Resource Exhaustion Discard Reason Occurs if Ethernet Rx I/F cannot allocate shared internal buffer memory to store the entire received frame"]
    #[inline(always)]
    pub const fn set_smredr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive disable discard reason due to port being in receive disabled state (POR\\[RXDIS\\]=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn rxdisdr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive disable discard reason due to port being in receive disabled state (POR\\[RXDIS\\]=1)"]
    #[inline(always)]
    pub const fn set_rxdisdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Ingress Port Filter Discard Reason. Cleared when written to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn ipfdr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Port Filter Discard Reason. Cleared when written to 1."]
    #[inline(always)]
    pub const fn set_ipfdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Rate Policer Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn rpdr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Rate Policer Discard Reason"]
    #[inline(always)]
    pub const fn set_rpdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Ingress Stream Forwarding Discard Reason Occurs if frame is associated with an ingress stream whose Forwarding Action (FA) is equal to discard"]
    #[must_use]
    #[inline(always)]
    pub const fn isfdr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Stream Forwarding Discard Reason Occurs if frame is associated with an ingress stream whose Forwarding Action (FA) is equal to discard"]
    #[inline(always)]
    pub const fn set_isfdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Stream Gate Closed Discard Reason Frame received on this port that is dropped because it didn't pass the stream gate"]
    #[must_use]
    #[inline(always)]
    pub const fn sgcdr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Stream Gate Closed Discard Reason Frame received on this port that is dropped because it didn't pass the stream gate"]
    #[inline(always)]
    pub const fn set_sgcdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Stream Gate Octet Exceeded Discard Reason Frame dropped due to octet count exceeded maximum specified for the open gate interval Cleared when written to 1"]
    #[must_use]
    #[inline(always)]
    pub const fn sgoedr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Stream Gate Octet Exceeded Discard Reason Frame dropped due to octet count exceeded maximum specified for the open gate interval Cleared when written to 1"]
    #[inline(always)]
    pub const fn set_sgoedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Ingress Stream Maximum Service Data Unit Exceeded Discard Reason Occurs if packet received is greater than the Maximum SDU size configured for the stream"]
    #[must_use]
    #[inline(always)]
    pub const fn msduedr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress Stream Maximum Service Data Unit Exceeded Discard Reason Occurs if packet received is greater than the Maximum SDU size configured for the stream"]
    #[inline(always)]
    pub const fn set_msduedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Frame Modification Misconfiguration Error Discard Reason Cleared when written to 1"]
    #[must_use]
    #[inline(always)]
    pub const fn fmmedr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Modification Misconfiguration Error Discard Reason Cleared when written to 1"]
    #[inline(always)]
    pub const fn set_fmmedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Congestion Management Discard Reason Frame discarded due to insufficient amount of memory space in buffer pool or in switch shared internal buffer memory"]
    #[must_use]
    #[inline(always)]
    pub const fn cmdr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Congestion Management Discard Reason Frame discarded due to insufficient amount of memory space in buffer pool or in switch shared internal buffer memory"]
    #[inline(always)]
    pub const fn set_cmdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Invalid Table Entry Discard Reason Table entry ID reference is not found in the table (entry has not been added to the table) Table entry ID is outside of its table allocation range Cleared when written to 1"]
    #[must_use]
    #[inline(always)]
    pub const fn itedr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Table Entry Discard Reason Table entry ID reference is not found in the table (entry has not been added to the table) Table entry ID is outside of its table allocation range Cleared when written to 1"]
    #[inline(always)]
    pub const fn set_itedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "ECC Error Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn eccedr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "ECC Error Discard Reason"]
    #[inline(always)]
    pub const fn set_eccedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Layer 2 Denial of Service Discard Reason Occurs if a packet is discarded due to L2DOS protection"]
    #[must_use]
    #[inline(always)]
    pub const fn l2dosdr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Layer 2 Denial of Service Discard Reason Occurs if a packet is discarded due to L2DOS protection"]
    #[inline(always)]
    pub const fn set_l2dosdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "No Destination Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn nodestdr(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "No Destination Discard Reason"]
    #[inline(always)]
    pub const fn set_nodestdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Prxdcrr0 {
    #[inline(always)]
    fn default() -> Prxdcrr0 {
        Prxdcrr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Prxdcrr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prxdcrr0")
            .field("pcdr", &self.pcdr())
            .field("smredr", &self.smredr())
            .field("rxdisdr", &self.rxdisdr())
            .field("ipfdr", &self.ipfdr())
            .field("rpdr", &self.rpdr())
            .field("isfdr", &self.isfdr())
            .field("sgcdr", &self.sgcdr())
            .field("sgoedr", &self.sgoedr())
            .field("msduedr", &self.msduedr())
            .field("fmmedr", &self.fmmedr())
            .field("cmdr", &self.cmdr())
            .field("itedr", &self.itedr())
            .field("eccedr", &self.eccedr())
            .field("l2dosdr", &self.l2dosdr())
            .field("nodestdr", &self.nodestdr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prxdcrr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prxdcrr0 {
            pcdr: bool,
            smredr: bool,
            rxdisdr: bool,
            ipfdr: bool,
            rpdr: bool,
            isfdr: bool,
            sgcdr: bool,
            sgoedr: bool,
            msduedr: bool,
            fmmedr: bool,
            cmdr: bool,
            itedr: bool,
            eccedr: bool,
            l2dosdr: bool,
            nodestdr: bool,
        }
        let proxy = Prxdcrr0 {
            pcdr: self.pcdr(),
            smredr: self.smredr(),
            rxdisdr: self.rxdisdr(),
            ipfdr: self.ipfdr(),
            rpdr: self.rpdr(),
            isfdr: self.isfdr(),
            sgcdr: self.sgcdr(),
            sgoedr: self.sgoedr(),
            msduedr: self.msduedr(),
            fmmedr: self.fmmedr(),
            cmdr: self.cmdr(),
            itedr: self.itedr(),
            eccedr: self.eccedr(),
            l2dosdr: self.l2dosdr(),
            nodestdr: self.nodestdr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Rx discard count reason register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prxdcrr1(pub u32);
impl Prxdcrr1 {
    #[doc = "Entry Id who last incremented discard count"]
    #[must_use]
    #[inline(always)]
    pub const fn entryid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Entry Id who last incremented discard count"]
    #[inline(always)]
    pub const fn set_entryid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Table type which caused the discard"]
    #[must_use]
    #[inline(always)]
    pub const fn tt(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Table type which caused the discard"]
    #[inline(always)]
    pub const fn set_tt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Prxdcrr1 {
    #[inline(always)]
    fn default() -> Prxdcrr1 {
        Prxdcrr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Prxdcrr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prxdcrr1")
            .field("entryid", &self.entryid())
            .field("tt", &self.tt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prxdcrr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prxdcrr1 {
            entryid: u16,
            tt: u8,
        }
        let proxy = Prxdcrr1 {
            entryid: self.entryid(),
            tt: self.tt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port receive SDU overhead register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prxsduor(pub u32);
impl Prxsduor {
    #[doc = "PPDU Byte count overhead"]
    #[must_use]
    #[inline(always)]
    pub const fn ppdu_bco(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "PPDU Byte count overhead"]
    #[inline(always)]
    pub const fn set_ppdu_bco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "MACSec byte count overhead"]
    #[must_use]
    #[inline(always)]
    pub const fn macsec_bco(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "MACSec byte count overhead"]
    #[inline(always)]
    pub const fn set_macsec_bco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for Prxsduor {
    #[inline(always)]
    fn default() -> Prxsduor {
        Prxsduor(20u64 as u32)
    }
}
impl core::fmt::Debug for Prxsduor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prxsduor")
            .field("ppdu_bco", &self.ppdu_bco())
            .field("macsec_bco", &self.macsec_bco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prxsduor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Prxsduor {
            ppdu_bco: u8,
            macsec_bco: u8,
        }
        let proxy = Prxsduor {
            ppdu_bco: self.ppdu_bco(),
            macsec_bco: self.macsec_bco(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port stream gate configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psgcr(pub u32);
impl Psgcr {
    #[doc = "Link propagation delay in nanoseconds"]
    #[must_use]
    #[inline(always)]
    pub const fn pdelay(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Link propagation delay in nanoseconds"]
    #[inline(always)]
    pub const fn set_pdelay(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Stream Gate Open Gate Check mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ogc(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Stream Gate Open Gate Check mode"]
    #[inline(always)]
    pub const fn set_ogc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Psgcr {
    #[inline(always)]
    fn default() -> Psgcr {
        Psgcr(0u64 as u32)
    }
}
impl core::fmt::Debug for Psgcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psgcr")
            .field("pdelay", &self.pdelay())
            .field("ogc", &self.ogc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psgcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psgcr {
            pdelay: u32,
            ogc: bool,
        }
        let proxy = Psgcr {
            pdelay: self.pdelay(),
            ogc: self.ogc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psr(pub u32);
impl Psr {
    #[doc = "Transmit busy."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_busy(&self) -> super::vals::TxBusy {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::TxBusy::from_bits(val as u8)
    }
    #[doc = "Transmit busy."]
    #[inline(always)]
    pub const fn set_tx_busy(&mut self, val: super::vals::TxBusy) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive busy."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_busy(&self) -> super::vals::RxBusy {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::RxBusy::from_bits(val as u8)
    }
    #[doc = "Receive busy."]
    #[inline(always)]
    pub const fn set_rx_busy(&mut self, val: super::vals::RxBusy) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Psr {
    #[inline(always)]
    fn default() -> Psr {
        Psr(0u64 as u32)
    }
}
impl core::fmt::Debug for Psr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Psr")
            .field("tx_busy", &self.tx_busy())
            .field("rx_busy", &self.rx_busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Psr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Psr {
            tx_busy: super::vals::TxBusy,
            rx_busy: super::vals::RxBusy,
        }
        let proxy = Psr {
            tx_busy: self.tx_busy(),
            rx_busy: self.rx_busy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port TPID acceptance register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptar(pub u32);
impl Ptar {
    #[doc = "Outer VLAN TPID List : bitmap identifying which TPIDs are acceptable as Outer VLAN tag xxx1 Standard C-VLAN 0x8100 xx1x Standard S-VLAN 0x88A8 x1xx Custom VLAN as defined by CVLANR1\\[ETYPE\\] 1xxx Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ovtpidl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Outer VLAN TPID List : bitmap identifying which TPIDs are acceptable as Outer VLAN tag xxx1 Standard C-VLAN 0x8100 xx1x Standard S-VLAN 0x88A8 x1xx Custom VLAN as defined by CVLANR1\\[ETYPE\\] 1xxx Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    #[inline(always)]
    pub const fn set_ovtpidl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Outer VLAN TPID List : bitmap identifying which TPIDs are acceptable as Inner VLAN tag xxx1 Standard C-VLAN 0x8100 xx1x Standard S-VLAN 0x88A8 x1xx Custom VLAN as defined by CVLANR1\\[ETYPE\\] 1xxx Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn ivtpidl(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Outer VLAN TPID List : bitmap identifying which TPIDs are acceptable as Inner VLAN tag xxx1 Standard C-VLAN 0x8100 xx1x Standard S-VLAN 0x88A8 x1xx Custom VLAN as defined by CVLANR1\\[ETYPE\\] 1xxx Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    #[inline(always)]
    pub const fn set_ivtpidl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Ptar {
    #[inline(always)]
    fn default() -> Ptar {
        Ptar(51u64 as u32)
    }
}
impl core::fmt::Debug for Ptar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptar")
            .field("ovtpidl", &self.ovtpidl())
            .field("ivtpidl", &self.ivtpidl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptar {
            ovtpidl: u8,
            ivtpidl: u8,
        }
        let proxy = Ptar {
            ovtpidl: self.ovtpidl(),
            ivtpidl: self.ivtpidl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port transmit traffic class a credit based shaper register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptccbsr0(pub u32);
impl Ptccbsr0 {
    #[doc = "Bandwidth"]
    #[must_use]
    #[inline(always)]
    pub const fn bw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Bandwidth"]
    #[inline(always)]
    pub const fn set_bw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Credit Based Shaper Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cbse(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Credit Based Shaper Enable"]
    #[inline(always)]
    pub const fn set_cbse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ptccbsr0 {
    #[inline(always)]
    fn default() -> Ptccbsr0 {
        Ptccbsr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptccbsr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptccbsr0")
            .field("bw", &self.bw())
            .field("cbse", &self.cbse())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptccbsr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptccbsr0 {
            bw: u8,
            cbse: bool,
        }
        let proxy = Ptccbsr0 {
            bw: self.bw(),
            cbse: self.cbse(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time capture maximum latency register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptcmaxlr(pub u32);
impl Ptcmaxlr {
    #[doc = "Indicates the maximum latency between the Tx Timestamp and Rx Timestamp captured by the Ethernet MAC relative to SFD"]
    #[must_use]
    #[inline(always)]
    pub const fn latency(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Indicates the maximum latency between the Tx Timestamp and Rx Timestamp captured by the Ethernet MAC relative to SFD"]
    #[inline(always)]
    pub const fn set_latency(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Ptcmaxlr {
    #[inline(always)]
    fn default() -> Ptcmaxlr {
        Ptcmaxlr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptcmaxlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptcmaxlr")
            .field("latency", &self.latency())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptcmaxlr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptcmaxlr {
            latency: u32,
        }
        let proxy = Ptcmaxlr {
            latency: self.latency(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time capture minimum latency register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptcminlr(pub u32);
impl Ptcminlr {
    #[doc = "Indicates the minimum latency between the Tx Timestamp and Rx Timestamp captured by the Ethernet MAC relative to SFD"]
    #[must_use]
    #[inline(always)]
    pub const fn latency(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Indicates the minimum latency between the Tx Timestamp and Rx Timestamp captured by the Ethernet MAC relative to SFD"]
    #[inline(always)]
    pub const fn set_latency(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
    #[doc = "Indicates the number of times a frame transmitted out of this port, has fulfilled the timestamp capture criteria"]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Indicates the number of times a frame transmitted out of this port, has fulfilled the timestamp capture criteria"]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Ptcminlr {
    #[inline(always)]
    fn default() -> Ptcminlr {
        Ptcminlr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptcminlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptcminlr")
            .field("latency", &self.latency())
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptcminlr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptcminlr {
            latency: u32,
            count: u8,
        }
        let proxy = Ptcminlr {
            latency: self.latency(),
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port traffic class a transmit maximum SDU register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptctmsdur(pub u32);
impl Ptctmsdur {
    #[doc = "Transmit Maximum SDU size in bytes (i"]
    #[must_use]
    #[inline(always)]
    pub const fn maxsdu(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit Maximum SDU size in bytes (i"]
    #[inline(always)]
    pub const fn set_maxsdu(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Specifies the type of PDU/SDU (Protocol/Service Data Unit) whose length is being validated as seen on the link"]
    #[must_use]
    #[inline(always)]
    pub const fn sdu_type(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Specifies the type of PDU/SDU (Protocol/Service Data Unit) whose length is being validated as seen on the link"]
    #[inline(always)]
    pub const fn set_sdu_type(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "When cleared, the Tx Max SDU check is performed for Store and Forward frames"]
    #[must_use]
    #[inline(always)]
    pub const fn sf_maxsdu_dis(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "When cleared, the Tx Max SDU check is performed for Store and Forward frames"]
    #[inline(always)]
    pub const fn set_sf_maxsdu_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ptctmsdur {
    #[inline(always)]
    fn default() -> Ptctmsdur {
        Ptctmsdur(67072u64 as u32)
    }
}
impl core::fmt::Debug for Ptctmsdur {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptctmsdur")
            .field("maxsdu", &self.maxsdu())
            .field("sdu_type", &self.sdu_type())
            .field("sf_maxsdu_dis", &self.sf_maxsdu_dis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptctmsdur {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptctmsdur {
            maxsdu: u16,
            sdu_type: u8,
            sf_maxsdu_dis: bool,
        }
        let proxy = Ptctmsdur {
            maxsdu: self.maxsdu(),
            sdu_type: self.sdu_type(),
            sf_maxsdu_dis: self.sf_maxsdu_dis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time gate scheduling admin gate list length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgagllr(pub u32);
impl Ptgagllr {
    #[doc = "Administrative gate control list length (number of entries)"]
    #[must_use]
    #[inline(always)]
    pub const fn admin_gate_list_length(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Administrative gate control list length (number of entries)"]
    #[inline(always)]
    pub const fn set_admin_gate_list_length(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ptgagllr {
    #[inline(always)]
    fn default() -> Ptgagllr {
        Ptgagllr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptgagllr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgagllr")
            .field("admin_gate_list_length", &self.admin_gate_list_length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgagllr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgagllr {
            admin_gate_list_length: u16,
        }
        let proxy = Ptgagllr {
            admin_gate_list_length: self.admin_gate_list_length(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time gate scheduling admin gate list status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgaglsr(pub u32);
impl Ptgaglsr {
    #[doc = "Time gated (state of the operational gate control list) 0 No operational gate control list is active 1 Operational gate control list is active"]
    #[must_use]
    #[inline(always)]
    pub const fn tg(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Time gated (state of the operational gate control list) 0 No operational gate control list is active 1 Operational gate control list is active"]
    #[inline(always)]
    pub const fn set_tg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Administrative gate control list pending 0 No administrative gate control list is configured 1 Administrative gate control list is pending (configured but not installed yet)"]
    #[must_use]
    #[inline(always)]
    pub const fn cfg_pend(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Administrative gate control list pending 0 No administrative gate control list is configured 1 Administrative gate control list is pending (configured but not installed yet)"]
    #[inline(always)]
    pub const fn set_cfg_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Ptgaglsr {
    #[inline(always)]
    fn default() -> Ptgaglsr {
        Ptgaglsr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptgaglsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgaglsr")
            .field("tg", &self.tg())
            .field("cfg_pend", &self.cfg_pend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgaglsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgaglsr {
            tg: bool,
            cfg_pend: bool,
        }
        let proxy = Ptgaglsr {
            tg: self.tg(),
            cfg_pend: self.cfg_pend(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time gating operational gate list length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgogllr(pub u32);
impl Ptgogllr {
    #[doc = "Operational gate control list length (number of entries)"]
    #[must_use]
    #[inline(always)]
    pub const fn oper_gate_list_length(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Operational gate control list length (number of entries)"]
    #[inline(always)]
    pub const fn set_oper_gate_list_length(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ptgogllr {
    #[inline(always)]
    fn default() -> Ptgogllr {
        Ptgogllr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptgogllr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgogllr")
            .field("oper_gate_list_length", &self.oper_gate_list_length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgogllr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgogllr {
            oper_gate_list_length: u16,
        }
        let proxy = Ptgogllr {
            oper_gate_list_length: self.oper_gate_list_length(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time gate scheduling control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgscr(pub u32);
impl Ptgscr {
    #[doc = "Time Gating Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tge(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gating Enable"]
    #[inline(always)]
    pub const fn set_tge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ptgscr {
    #[inline(always)]
    fn default() -> Ptgscr {
        Ptgscr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptgscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgscr").field("tge", &self.tge()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgscr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgscr {
            tge: bool,
        }
        let proxy = Ptgscr { tge: self.tge() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time gate scheduling hold advance register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgshar(pub u32);
impl Ptgshar {
    #[doc = "This field indicates the amount of time prior to the Set-And-Hold-MAC time slot for asserting the Hold"]
    #[must_use]
    #[inline(always)]
    pub const fn holdadvance(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the amount of time prior to the Set-And-Hold-MAC time slot for asserting the Hold"]
    #[inline(always)]
    pub const fn set_holdadvance(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ptgshar {
    #[inline(always)]
    fn default() -> Ptgshar {
        Ptgshar(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptgshar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgshar")
            .field("holdadvance", &self.holdadvance())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgshar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgshar {
            holdadvance: u16,
        }
        let proxy = Ptgshar {
            holdadvance: self.holdadvance(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time gate scheduling hold configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgshcr(pub u32);
impl Ptgshcr {
    #[doc = "Hold-Skew in nanoseconds"]
    #[must_use]
    #[inline(always)]
    pub const fn hold_skew(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Hold-Skew in nanoseconds"]
    #[inline(always)]
    pub const fn set_hold_skew(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Ptgshcr {
    #[inline(always)]
    fn default() -> Ptgshcr {
        Ptgshcr(16777216u64 as u32)
    }
}
impl core::fmt::Debug for Ptgshcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgshcr")
            .field("hold_skew", &self.hold_skew())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgshcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgshcr {
            hold_skew: u32,
        }
        let proxy = Ptgshcr {
            hold_skew: self.hold_skew(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time gate scheduling release advance register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgsrar(pub u32);
impl Ptgsrar {
    #[doc = "This field indicates the amount of time prior to the Set-And-Release-MAC time slot for asserting the Release"]
    #[must_use]
    #[inline(always)]
    pub const fn releaseadvance(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field indicates the amount of time prior to the Set-And-Release-MAC time slot for asserting the Release"]
    #[inline(always)]
    pub const fn set_releaseadvance(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ptgsrar {
    #[inline(always)]
    fn default() -> Ptgsrar {
        Ptgsrar(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptgsrar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgsrar")
            .field("releaseadvance", &self.releaseadvance())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgsrar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgsrar {
            releaseadvance: u16,
        }
        let proxy = Ptgsrar {
            releaseadvance: self.releaseadvance(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port time gate scheduling traffic class a status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgstcsr(pub u32);
impl Ptgstcsr {
    #[doc = "Look-ahead gate state 0 Gate closed 1 Gate open IERB registers SaTGSLR / EaTGSLR\\[MIN_LOOKAHEAD\\] + the per port register PTGSATOR\\[ADV_TIME_OFFSET\\] specify the advance time of the gate state"]
    #[must_use]
    #[inline(always)]
    pub const fn lh_state(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Look-ahead gate state 0 Gate closed 1 Gate open IERB registers SaTGSLR / EaTGSLR\\[MIN_LOOKAHEAD\\] + the per port register PTGSATOR\\[ADV_TIME_OFFSET\\] specify the advance time of the gate state"]
    #[inline(always)]
    pub const fn set_lh_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Ptgstcsr {
    #[inline(always)]
    fn default() -> Ptgstcsr {
        Ptgstcsr(65536u64 as u32)
    }
}
impl core::fmt::Debug for Ptgstcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgstcsr")
            .field("lh_state", &self.lh_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgstcsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgstcsr {
            lh_state: bool,
        }
        let proxy = Ptgstcsr {
            lh_state: self.lh_state(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Tx discard count reason register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptxdcrr0(pub u32);
impl Ptxdcrr0 {
    #[doc = "Transmit disable discard reason due to port being in transmit disabled state (POR\\[TXDIS\\]=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn txdisdr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit disable discard reason due to port being in transmit disabled state (POR\\[TXDIS\\]=1)"]
    #[inline(always)]
    pub const fn set_txdisdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ECC Error Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn eccedr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ECC Error Discard Reason"]
    #[inline(always)]
    pub const fn set_eccedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Parity Error Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn pedr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Parity Error Discard Reason"]
    #[inline(always)]
    pub const fn set_pedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Time Gate Scheduling Frame Too Large Discard Reason Time Gate scheduling is enabled, and the frame was discarded because one of the following conditions was encountered: The frame was too large to transmit during any gate-open interval"]
    #[must_use]
    #[inline(always)]
    pub const fn tgsftldr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Time Gate Scheduling Frame Too Large Discard Reason Time Gate scheduling is enabled, and the frame was discarded because one of the following conditions was encountered: The frame was too large to transmit during any gate-open interval"]
    #[inline(always)]
    pub const fn set_tgsftldr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Frame Modification Misconfiguration Discard Reason Software Misconfiguration due to: Invalid frame header modification (i"]
    #[must_use]
    #[inline(always)]
    pub const fn fmmdr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Frame Modification Misconfiguration Discard Reason Software Misconfiguration due to: Invalid frame header modification (i"]
    #[inline(always)]
    pub const fn set_fmmdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transmit Disable prior to Enqueue to ETM Discard Reason due to Port being in a Transmit disabled state (POR\\[TXDIS\\]=1)"]
    #[must_use]
    #[inline(always)]
    pub const fn txdisedr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Disable prior to Enqueue to ETM Discard Reason due to Port being in a Transmit disabled state (POR\\[TXDIS\\]=1)"]
    #[inline(always)]
    pub const fn set_txdisedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Egress Maximum Service Data Unit Exceeded Discard Reason Occurs if store-and-forward frame is greater than the port Traffic Class's configured Maximum SDU size"]
    #[must_use]
    #[inline(always)]
    pub const fn msduedr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Maximum Service Data Unit Exceeded Discard Reason Occurs if store-and-forward frame is greater than the port Traffic Class's configured Maximum SDU size"]
    #[inline(always)]
    pub const fn set_msduedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Queue Congestion Discard Reason Occurs if a frame is dropped because it couldn't be enqueued to an Egress Traffic Management (ETM) class queue"]
    #[must_use]
    #[inline(always)]
    pub const fn qcongdr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Queue Congestion Discard Reason Occurs if a frame is dropped because it couldn't be enqueued to an Egress Traffic Management (ETM) class queue"]
    #[inline(always)]
    pub const fn set_qcongdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Invalid Table Entry Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn itedr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Table Entry Discard Reason"]
    #[inline(always)]
    pub const fn set_itedr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Invalid Enqueue Port or Queue Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn inveqdr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid Enqueue Port or Queue Discard Reason"]
    #[inline(always)]
    pub const fn set_inveqdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Egress Stream Sequence Recovery Take No Sequence Discard Reason Occurs if the sequence recovery function discards a frame because it is tagless and ESQA_TGT_EID\\[SQR_TNSQ\\]=0"]
    #[must_use]
    #[inline(always)]
    pub const fn sqrtnsqdr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Stream Sequence Recovery Take No Sequence Discard Reason Occurs if the sequence recovery function discards a frame because it is tagless and ESQA_TGT_EID\\[SQR_TNSQ\\]=0"]
    #[inline(always)]
    pub const fn set_sqrtnsqdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Egress Stream Sequence Recovery Rogue Discard Reason Occurs if a packet is discarded by the vector recovery algorithm because its sequence number has fallen outside of the recovery history window"]
    #[must_use]
    #[inline(always)]
    pub const fn sqrrdr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Stream Sequence Recovery Rogue Discard Reason Occurs if a packet is discarded by the vector recovery algorithm because its sequence number has fallen outside of the recovery history window"]
    #[inline(always)]
    pub const fn set_sqrrdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Egress Stream Sequence Recovery Duplicate Discard Reason Occurs if packet's sequence_number is a duplicate applies for either Match or Vector Recovery Algorithm)"]
    #[must_use]
    #[inline(always)]
    pub const fn sqrddr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Egress Stream Sequence Recovery Duplicate Discard Reason Occurs if packet's sequence_number is a duplicate applies for either Match or Vector Recovery Algorithm)"]
    #[inline(always)]
    pub const fn set_sqrddr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Shared Memory Resource Exhaustion Discard Reason"]
    #[must_use]
    #[inline(always)]
    pub const fn smredr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Shared Memory Resource Exhaustion Discard Reason"]
    #[inline(always)]
    pub const fn set_smredr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Ptxdcrr0 {
    #[inline(always)]
    fn default() -> Ptxdcrr0 {
        Ptxdcrr0(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptxdcrr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptxdcrr0")
            .field("txdisdr", &self.txdisdr())
            .field("eccedr", &self.eccedr())
            .field("pedr", &self.pedr())
            .field("tgsftldr", &self.tgsftldr())
            .field("fmmdr", &self.fmmdr())
            .field("txdisedr", &self.txdisedr())
            .field("msduedr", &self.msduedr())
            .field("qcongdr", &self.qcongdr())
            .field("itedr", &self.itedr())
            .field("inveqdr", &self.inveqdr())
            .field("sqrtnsqdr", &self.sqrtnsqdr())
            .field("sqrrdr", &self.sqrrdr())
            .field("sqrddr", &self.sqrddr())
            .field("smredr", &self.smredr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptxdcrr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptxdcrr0 {
            txdisdr: bool,
            eccedr: bool,
            pedr: bool,
            tgsftldr: bool,
            fmmdr: bool,
            txdisedr: bool,
            msduedr: bool,
            qcongdr: bool,
            itedr: bool,
            inveqdr: bool,
            sqrtnsqdr: bool,
            sqrrdr: bool,
            sqrddr: bool,
            smredr: bool,
        }
        let proxy = Ptxdcrr0 {
            txdisdr: self.txdisdr(),
            eccedr: self.eccedr(),
            pedr: self.pedr(),
            tgsftldr: self.tgsftldr(),
            fmmdr: self.fmmdr(),
            txdisedr: self.txdisedr(),
            msduedr: self.msduedr(),
            qcongdr: self.qcongdr(),
            itedr: self.itedr(),
            inveqdr: self.inveqdr(),
            sqrtnsqdr: self.sqrtnsqdr(),
            sqrrdr: self.sqrrdr(),
            sqrddr: self.sqrddr(),
            smredr: self.smredr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port Tx discard count reason register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptxdcrr1(pub u32);
impl Ptxdcrr1 {
    #[doc = "Entry Id who last incremented discard count"]
    #[must_use]
    #[inline(always)]
    pub const fn entryid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Entry Id who last incremented discard count"]
    #[inline(always)]
    pub const fn set_entryid(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Table type which was last accessed when frame was discarded"]
    #[must_use]
    #[inline(always)]
    pub const fn tt(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Table type which was last accessed when frame was discarded"]
    #[inline(always)]
    pub const fn set_tt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ptxdcrr1 {
    #[inline(always)]
    fn default() -> Ptxdcrr1 {
        Ptxdcrr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptxdcrr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptxdcrr1")
            .field("entryid", &self.entryid())
            .field("tt", &self.tt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptxdcrr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptxdcrr1 {
            entryid: u16,
            tt: u8,
        }
        let proxy = Ptxdcrr1 {
            entryid: self.entryid(),
            tt: self.tt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port transmit SDU overhead register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptxsduor(pub u32);
impl Ptxsduor {
    #[doc = "PPDU Byte count overhead"]
    #[must_use]
    #[inline(always)]
    pub const fn ppdu_bco(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "PPDU Byte count overhead"]
    #[inline(always)]
    pub const fn set_ppdu_bco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "MACSec byte count overhead"]
    #[must_use]
    #[inline(always)]
    pub const fn macsec_bco(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "MACSec byte count overhead"]
    #[inline(always)]
    pub const fn set_macsec_bco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for Ptxsduor {
    #[inline(always)]
    fn default() -> Ptxsduor {
        Ptxsduor(20u64 as u32)
    }
}
impl core::fmt::Debug for Ptxsduor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptxsduor")
            .field("ppdu_bco", &self.ppdu_bco())
            .field("macsec_bco", &self.macsec_bco())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptxsduor {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptxsduor {
            ppdu_bco: u8,
            macsec_bco: u8,
        }
        let proxy = Ptxsduor {
            ppdu_bco: self.ppdu_bco(),
            macsec_bco: self.macsec_bco(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
