#[doc = "External MDIO register address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioAddr(pub u32);
impl EmdioAddr {
    #[doc = "MDIO PHY register address."]
    #[must_use]
    #[inline(always)]
    pub const fn regaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MDIO PHY register address."]
    #[inline(always)]
    pub const fn set_regaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for EmdioAddr {
    #[inline(always)]
    fn default() -> EmdioAddr {
        EmdioAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for EmdioAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmdioAddr")
            .field("regaddr", &self.regaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmdioAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmdioAddr {
            regaddr: u16,
        }
        let proxy = EmdioAddr {
            regaddr: self.regaddr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "External MDIO configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioCfg(pub u32);
impl EmdioCfg {
    #[doc = "Busy 2 (same as bit 31)"]
    #[must_use]
    #[inline(always)]
    pub const fn bsy2(&self) -> super::vals::Bsy2 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Bsy2::from_bits(val as u8)
    }
    #[doc = "Busy 2 (same as bit 31)"]
    #[inline(always)]
    pub const fn set_bsy2(&mut self, val: super::vals::Bsy2) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "MDIO Read Error"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_rd_er(&self) -> super::vals::MdioRdEr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MdioRdEr::from_bits(val as u8)
    }
    #[doc = "MDIO Read Error"]
    #[inline(always)]
    pub const fn set_mdio_rd_er(&mut self, val: super::vals::MdioRdEr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "MDIO Hold Time"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_hold(&self) -> super::vals::MdioHold {
        let val = (self.0 >> 2usize) & 0x07;
        super::vals::MdioHold::from_bits(val as u8)
    }
    #[doc = "MDIO Hold Time"]
    #[inline(always)]
    pub const fn set_mdio_hold(&mut self, val: super::vals::MdioHold) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
    }
    #[doc = "MDIO Preamble Disable"]
    #[must_use]
    #[inline(always)]
    pub const fn pre_dis(&self) -> super::vals::PreDis {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::PreDis::from_bits(val as u8)
    }
    #[doc = "MDIO Preamble Disable"]
    #[inline(always)]
    pub const fn set_pre_dis(&mut self, val: super::vals::PreDis) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Clause 45 Support"]
    #[must_use]
    #[inline(always)]
    pub const fn enc45(&self) -> super::vals::Enc45 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Enc45::from_bits(val as u8)
    }
    #[doc = "Enable Clause 45 Support"]
    #[inline(always)]
    pub const fn set_enc45(&mut self, val: super::vals::Enc45) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "MDIO Clock Divisor"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_clk_div(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "MDIO Clock Divisor"]
    #[inline(always)]
    pub const fn set_mdio_clk_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
    #[doc = "Returns the virtual port ID."]
    #[must_use]
    #[inline(always)]
    pub const fn whoami(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Returns the virtual port ID."]
    #[inline(always)]
    pub const fn set_whoami(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Extended HOLD"]
    #[must_use]
    #[inline(always)]
    pub const fn ehold(&self) -> super::vals::Ehold {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Ehold::from_bits(val as u8)
    }
    #[doc = "Extended HOLD"]
    #[inline(always)]
    pub const fn set_ehold(&mut self, val: super::vals::Ehold) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Negative Edge"]
    #[must_use]
    #[inline(always)]
    pub const fn neg(&self) -> super::vals::Neg {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Neg::from_bits(val as u8)
    }
    #[doc = "Negative Edge"]
    #[inline(always)]
    pub const fn set_neg(&mut self, val: super::vals::Neg) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Address Error"]
    #[must_use]
    #[inline(always)]
    pub const fn addr_err(&self) -> super::vals::AddrErr {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::AddrErr::from_bits(val as u8)
    }
    #[doc = "Address Error"]
    #[inline(always)]
    pub const fn set_addr_err(&mut self, val: super::vals::AddrErr) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "MDIO Command Completion Interrupt Enable Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cim(&self) -> super::vals::Cim {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cim::from_bits(val as u8)
    }
    #[doc = "MDIO Command Completion Interrupt Enable Mask"]
    #[inline(always)]
    pub const fn set_cim(&mut self, val: super::vals::Cim) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "MDIO Command Completion"]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self) -> super::vals::Cmp {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Cmp::from_bits(val as u8)
    }
    #[doc = "MDIO Command Completion"]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: super::vals::Cmp) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Busy 1"]
    #[must_use]
    #[inline(always)]
    pub const fn bsy1(&self) -> super::vals::Bsy1 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Bsy1::from_bits(val as u8)
    }
    #[doc = "Busy 1"]
    #[inline(always)]
    pub const fn set_bsy1(&mut self, val: super::vals::Bsy1) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for EmdioCfg {
    #[inline(always)]
    fn default() -> EmdioCfg {
        EmdioCfg(332872u64 as u32)
    }
}
impl core::fmt::Debug for EmdioCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmdioCfg")
            .field("bsy2", &self.bsy2())
            .field("mdio_rd_er", &self.mdio_rd_er())
            .field("mdio_hold", &self.mdio_hold())
            .field("pre_dis", &self.pre_dis())
            .field("enc45", &self.enc45())
            .field("mdio_clk_div", &self.mdio_clk_div())
            .field("whoami", &self.whoami())
            .field("ehold", &self.ehold())
            .field("neg", &self.neg())
            .field("addr_err", &self.addr_err())
            .field("cim", &self.cim())
            .field("cmp", &self.cmp())
            .field("bsy1", &self.bsy1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmdioCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmdioCfg {
            bsy2: super::vals::Bsy2,
            mdio_rd_er: super::vals::MdioRdEr,
            mdio_hold: super::vals::MdioHold,
            pre_dis: super::vals::PreDis,
            enc45: super::vals::Enc45,
            mdio_clk_div: u16,
            whoami: u8,
            ehold: super::vals::Ehold,
            neg: super::vals::Neg,
            addr_err: super::vals::AddrErr,
            cim: super::vals::Cim,
            cmp: super::vals::Cmp,
            bsy1: super::vals::Bsy1,
        }
        let proxy = EmdioCfg {
            bsy2: self.bsy2(),
            mdio_rd_er: self.mdio_rd_er(),
            mdio_hold: self.mdio_hold(),
            pre_dis: self.pre_dis(),
            enc45: self.enc45(),
            mdio_clk_div: self.mdio_clk_div(),
            whoami: self.whoami(),
            ehold: self.ehold(),
            neg: self.neg(),
            addr_err: self.addr_err(),
            cim: self.cim(),
            cmp: self.cmp(),
            bsy1: self.bsy1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "External MDIO interface control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioCtl(pub u32);
impl EmdioCtl {
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    #[inline(always)]
    pub const fn set_dev_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    #[must_use]
    #[inline(always)]
    pub const fn port_addr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    #[inline(always)]
    pub const fn set_port_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "MDIO read with address post-increment initiation. Self-clearing once transaction is complete."]
    #[must_use]
    #[inline(always)]
    pub const fn post_inc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO read with address post-increment initiation. Self-clearing once transaction is complete."]
    #[inline(always)]
    pub const fn set_post_inc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "MDIO read initiation."]
    #[must_use]
    #[inline(always)]
    pub const fn read(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO read initiation."]
    #[inline(always)]
    pub const fn set_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "MDIO busy"]
    #[must_use]
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO busy"]
    #[inline(always)]
    pub const fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for EmdioCtl {
    #[inline(always)]
    fn default() -> EmdioCtl {
        EmdioCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for EmdioCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmdioCtl")
            .field("dev_addr", &self.dev_addr())
            .field("port_addr", &self.port_addr())
            .field("post_inc", &self.post_inc())
            .field("read", &self.read())
            .field("bsy", &self.bsy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmdioCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmdioCtl {
            dev_addr: u8,
            port_addr: u8,
            post_inc: bool,
            read: bool,
            bsy: bool,
        }
        let proxy = EmdioCtl {
            dev_addr: self.dev_addr(),
            port_addr: self.port_addr(),
            post_inc: self.post_inc(),
            read: self.read(),
            bsy: self.bsy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "External MDIO interface data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioData(pub u32);
impl EmdioData {
    #[doc = "16-bit MDIO data."]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "16-bit MDIO data."]
    #[inline(always)]
    pub const fn set_mdio_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for EmdioData {
    #[inline(always)]
    fn default() -> EmdioData {
        EmdioData(0u64 as u32)
    }
}
impl core::fmt::Debug for EmdioData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmdioData")
            .field("mdio_data", &self.mdio_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmdioData {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmdioData {
            mdio_data: u16,
        }
        let proxy = EmdioData {
            mdio_data: self.mdio_data(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "External MDIO status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmdioStat(pub u32);
impl EmdioStat {
    #[doc = "Global MDIO busy"]
    #[must_use]
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Global MDIO busy"]
    #[inline(always)]
    pub const fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PHY white list"]
    #[must_use]
    #[inline(always)]
    pub const fn wht_list(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "PHY white list"]
    #[inline(always)]
    pub const fn set_wht_list(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "PHY white list enable"]
    #[must_use]
    #[inline(always)]
    pub const fn wht_list_ena(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PHY white list enable"]
    #[inline(always)]
    pub const fn set_wht_list_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Port ID"]
    #[must_use]
    #[inline(always)]
    pub const fn port_id(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Port ID"]
    #[inline(always)]
    pub const fn set_port_id(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Port ID"]
    #[must_use]
    #[inline(always)]
    pub const fn req_type(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Port ID"]
    #[inline(always)]
    pub const fn set_req_type(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for EmdioStat {
    #[inline(always)]
    fn default() -> EmdioStat {
        EmdioStat(0u64 as u32)
    }
}
impl core::fmt::Debug for EmdioStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EmdioStat")
            .field("bsy", &self.bsy())
            .field("wht_list", &self.wht_list())
            .field("wht_list_ena", &self.wht_list_ena())
            .field("port_id", &self.port_id())
            .field("req_type", &self.req_type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EmdioStat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EmdioStat {
            bsy: bool,
            wht_list: u8,
            wht_list_ena: bool,
            port_id: u8,
            req_type: bool,
        }
        let proxy = EmdioStat {
            bsy: self.bsy(),
            wht_list: self.wht_list(),
            wht_list_ena: self.wht_list_ena(),
            port_id: self.port_id(),
            req_type: self.req_type(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "MDIO configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MdioCfg(pub u32);
impl MdioCfg {
    #[doc = "MDIO pin mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO pin mode"]
    #[inline(always)]
    pub const fn set_mdio_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "MDC pin mode"]
    #[must_use]
    #[inline(always)]
    pub const fn mdc_mode(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MDC pin mode"]
    #[inline(always)]
    pub const fn set_mdc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for MdioCfg {
    #[inline(always)]
    fn default() -> MdioCfg {
        MdioCfg(16u64 as u32)
    }
}
impl core::fmt::Debug for MdioCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MdioCfg")
            .field("mdio_mode", &self.mdio_mode())
            .field("mdc_mode", &self.mdc_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MdioCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MdioCfg {
            mdio_mode: bool,
            mdc_mode: bool,
        }
        let proxy = MdioCfg {
            mdio_mode: self.mdio_mode(),
            mdc_mode: self.mdc_mode(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY status register address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyStatusAddr(pub u32);
impl PhyStatusAddr {
    #[doc = "MDIO PHY register address. Address of the register within the Clause 45 PHY device from which data is to be read."]
    #[must_use]
    #[inline(always)]
    pub const fn regaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MDIO PHY register address. Address of the register within the Clause 45 PHY device from which data is to be read."]
    #[inline(always)]
    pub const fn set_regaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for PhyStatusAddr {
    #[inline(always)]
    fn default() -> PhyStatusAddr {
        PhyStatusAddr(0u64 as u32)
    }
}
impl core::fmt::Debug for PhyStatusAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyStatusAddr")
            .field("regaddr", &self.regaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyStatusAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyStatusAddr {
            regaddr: u16,
        }
        let proxy = PhyStatusAddr {
            regaddr: self.regaddr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY status configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyStatusCfg(pub u32);
impl PhyStatusCfg {
    #[doc = "MDIO busy"]
    #[must_use]
    #[inline(always)]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO busy"]
    #[inline(always)]
    pub const fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MDIO read error"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_rd_er(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO read error"]
    #[inline(always)]
    pub const fn set_mdio_rd_er(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "PHY status read interval"]
    #[must_use]
    #[inline(always)]
    pub const fn status_interval(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "PHY status read interval"]
    #[inline(always)]
    pub const fn set_status_interval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PhyStatusCfg {
    #[inline(always)]
    fn default() -> PhyStatusCfg {
        PhyStatusCfg(0u64 as u32)
    }
}
impl core::fmt::Debug for PhyStatusCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyStatusCfg")
            .field("bsy", &self.bsy())
            .field("mdio_rd_er", &self.mdio_rd_er())
            .field("status_interval", &self.status_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyStatusCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyStatusCfg {
            bsy: bool,
            mdio_rd_er: bool,
            status_interval: u16,
        }
        let proxy = PhyStatusCfg {
            bsy: self.bsy(),
            mdio_rd_er: self.mdio_rd_er(),
            status_interval: self.status_interval(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY status control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyStatusCtl(pub u32);
impl PhyStatusCtl {
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "5-bit MDIO device address (Clause 45) / register address (Clause 22)"]
    #[inline(always)]
    pub const fn set_dev_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    #[must_use]
    #[inline(always)]
    pub const fn port_addr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "5-bit MDIO port address (Clause 45) / PHY address (Clause 22)"]
    #[inline(always)]
    pub const fn set_port_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
}
impl Default for PhyStatusCtl {
    #[inline(always)]
    fn default() -> PhyStatusCtl {
        PhyStatusCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for PhyStatusCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyStatusCtl")
            .field("dev_addr", &self.dev_addr())
            .field("port_addr", &self.port_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyStatusCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyStatusCtl {
            dev_addr: u8,
            port_addr: u8,
        }
        let proxy = PhyStatusCtl {
            dev_addr: self.dev_addr(),
            port_addr: self.port_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY status data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyStatusData(pub u32);
impl PhyStatusData {
    #[doc = "16-bit MDIO data"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "16-bit MDIO data"]
    #[inline(always)]
    pub const fn set_mdio_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Current count"]
    #[must_use]
    #[inline(always)]
    pub const fn curr_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Current count"]
    #[inline(always)]
    pub const fn set_curr_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PhyStatusData {
    #[inline(always)]
    fn default() -> PhyStatusData {
        PhyStatusData(0u64 as u32)
    }
}
impl core::fmt::Debug for PhyStatusData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyStatusData")
            .field("mdio_data", &self.mdio_data())
            .field("curr_cnt", &self.curr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyStatusData {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyStatusData {
            mdio_data: u16,
            curr_cnt: u16,
        }
        let proxy = PhyStatusData {
            mdio_data: self.mdio_data(),
            curr_cnt: self.curr_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY status event register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyStatusEvent(pub u32);
impl PhyStatusEvent {
    #[doc = "Status event high-to-low. Set to 1 if a 1->0 transition on a corresponding data bit has occurred. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn status_event_hl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Status event high-to-low. Set to 1 if a 1->0 transition on a corresponding data bit has occurred. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_status_event_hl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Status event low-to-high. Set to 1 if a 0->1 transition on a corresponding data bit has occurred. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn status_event_lh(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Status event low-to-high. Set to 1 if a 0->1 transition on a corresponding data bit has occurred. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_status_event_lh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PhyStatusEvent {
    #[inline(always)]
    fn default() -> PhyStatusEvent {
        PhyStatusEvent(0u64 as u32)
    }
}
impl core::fmt::Debug for PhyStatusEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyStatusEvent")
            .field("status_event_hl", &self.status_event_hl())
            .field("status_event_lh", &self.status_event_lh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyStatusEvent {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyStatusEvent {
            status_event_hl: u16,
            status_event_lh: u16,
        }
        let proxy = PhyStatusEvent {
            status_event_hl: self.status_event_hl(),
            status_event_lh: self.status_event_lh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY status mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyStatusMask(pub u32);
impl PhyStatusMask {
    #[doc = "Status high-to-low mask. If set to 1, assert an interrupt if the corresponding event bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn status_mask_hl(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Status high-to-low mask. If set to 1, assert an interrupt if the corresponding event bit is set."]
    #[inline(always)]
    pub const fn set_status_mask_hl(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Status mask low-to-high. If set to 1, assert an interrupt if the corresponding event bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn status_mask_lh(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Status mask low-to-high. If set to 1, assert an interrupt if the corresponding event bit is set."]
    #[inline(always)]
    pub const fn set_status_mask_lh(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PhyStatusMask {
    #[inline(always)]
    fn default() -> PhyStatusMask {
        PhyStatusMask(0u64 as u32)
    }
}
impl core::fmt::Debug for PhyStatusMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PhyStatusMask")
            .field("status_mask_hl", &self.status_mask_hl())
            .field("status_mask_lh", &self.status_mask_lh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PhyStatusMask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PhyStatusMask {
            status_mask_hl: u16,
            status_mask_lh: u16,
        }
        let proxy = PhyStatusMask {
            status_mask_hl: self.status_mask_hl(),
            status_mask_lh: self.status_mask_lh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
