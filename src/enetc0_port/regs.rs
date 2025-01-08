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
        }
        let proxy = Pqosmr {
            vs: self.vs(),
            ve: self.ve(),
            ddr: self.ddr(),
            dipv: self.dipv(),
            vqmp: self.vqmp(),
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
    #[doc = "Station Interface Filter Discard Reason Occurs if packet is discarded by the L2 Filtering function"]
    #[must_use]
    #[inline(always)]
    pub const fn sifdr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Station Interface Filter Discard Reason Occurs if packet is discarded by the L2 Filtering function"]
    #[inline(always)]
    pub const fn set_sifdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
            .field("itedr", &self.itedr())
            .field("eccedr", &self.eccedr())
            .field("sifdr", &self.sifdr())
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
            itedr: bool,
            eccedr: bool,
            sifdr: bool,
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
            itedr: self.itedr(),
            eccedr: self.eccedr(),
            sifdr: self.sifdr(),
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
#[doc = "Port time gate scheduling advance time offset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptgsator(pub u32);
impl Ptgsator {
    #[doc = "Advance time offset This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler, to adjust for the latency encountered across the MAC plus if needed, delays outside of NETC (e"]
    #[must_use]
    #[inline(always)]
    pub const fn adv_time_offset(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Advance time offset This field specifies the amount of time to advance the IEEE 1588 time scale used by the time-based scheduler, to adjust for the latency encountered across the MAC plus if needed, delays outside of NETC (e"]
    #[inline(always)]
    pub const fn set_adv_time_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ptgsator {
    #[inline(always)]
    fn default() -> Ptgsator {
        Ptgsator(0u64 as u32)
    }
}
impl core::fmt::Debug for Ptgsator {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ptgsator")
            .field("adv_time_offset", &self.adv_time_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ptgsator {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ptgsator {
            adv_time_offset: u16,
        }
        let proxy = Ptgsator {
            adv_time_offset: self.adv_time_offset(),
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
