#[doc = "Port MAC Merge Control and Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MacMergeMmcsr(pub u32);
impl MacMergeMmcsr {
    #[doc = "Local preemption supported"]
    #[must_use]
    #[inline(always)]
    pub const fn lps(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Local preemption supported"]
    #[inline(always)]
    pub const fn set_lps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Local preemption enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn lpe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Local preemption enabled"]
    #[inline(always)]
    pub const fn set_lpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Local preemption active"]
    #[must_use]
    #[inline(always)]
    pub const fn lpa(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Local preemption active"]
    #[inline(always)]
    pub const fn set_lpa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Local additional fragment size"]
    #[must_use]
    #[inline(always)]
    pub const fn lafs(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "Local additional fragment size"]
    #[inline(always)]
    pub const fn set_lafs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "Remote preemption supported"]
    #[must_use]
    #[inline(always)]
    pub const fn rps(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Remote preemption supported"]
    #[inline(always)]
    pub const fn set_rps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Remote preemption enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn rpe(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Remote preemption enabled"]
    #[inline(always)]
    pub const fn set_rpe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Remote preemption active"]
    #[must_use]
    #[inline(always)]
    pub const fn rpa(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Remote preemption active"]
    #[inline(always)]
    pub const fn set_rpa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Remote additional fragment size"]
    #[must_use]
    #[inline(always)]
    pub const fn rafs(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Remote additional fragment size"]
    #[inline(always)]
    pub const fn set_rafs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Merge enabled"]
    #[must_use]
    #[inline(always)]
    pub const fn me(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[doc = "Merge enabled"]
    #[inline(always)]
    pub const fn set_me(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
    }
    #[doc = "Verify disabled"]
    #[must_use]
    #[inline(always)]
    pub const fn vdis(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Verify disabled"]
    #[inline(always)]
    pub const fn set_vdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Verify status"]
    #[must_use]
    #[inline(always)]
    pub const fn vsts(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Verify status"]
    #[inline(always)]
    pub const fn set_vsts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Merge status"]
    #[must_use]
    #[inline(always)]
    pub const fn txsts(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[doc = "Merge status"]
    #[inline(always)]
    pub const fn set_txsts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[doc = "Verify Time"]
    #[must_use]
    #[inline(always)]
    pub const fn vt(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x7f;
        val as u8
    }
    #[doc = "Verify Time"]
    #[inline(always)]
    pub const fn set_vt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 23usize)) | (((val as u32) & 0x7f) << 23usize);
    }
    #[doc = "Link Fail"]
    #[must_use]
    #[inline(always)]
    pub const fn link_fail(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Link Fail"]
    #[inline(always)]
    pub const fn set_link_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MacMergeMmcsr {
    #[inline(always)]
    fn default() -> MacMergeMmcsr {
        MacMergeMmcsr(84017153u64 as u32)
    }
}
impl core::fmt::Debug for MacMergeMmcsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MacMergeMmcsr")
            .field("lps", &self.lps())
            .field("lpe", &self.lpe())
            .field("lpa", &self.lpa())
            .field("lafs", &self.lafs())
            .field("rps", &self.rps())
            .field("rpe", &self.rpe())
            .field("rpa", &self.rpa())
            .field("rafs", &self.rafs())
            .field("me", &self.me())
            .field("vdis", &self.vdis())
            .field("vsts", &self.vsts())
            .field("txsts", &self.txsts())
            .field("vt", &self.vt())
            .field("link_fail", &self.link_fail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MacMergeMmcsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MacMergeMmcsr {
            lps: bool,
            lpe: bool,
            lpa: bool,
            lafs: u8,
            rps: bool,
            rpe: bool,
            rpa: bool,
            rafs: u8,
            me: u8,
            vdis: bool,
            vsts: u8,
            txsts: u8,
            vt: u8,
            link_fail: bool,
        }
        let proxy = MacMergeMmcsr {
            lps: self.lps(),
            lpe: self.lpe(),
            lpa: self.lpa(),
            lafs: self.lafs(),
            rps: self.rps(),
            rpe: self.rpe(),
            rpa: self.rpa(),
            rafs: self.rafs(),
            me: self.me(),
            vdis: self.vdis(),
            vsts: self.vsts(),
            txsts: self.txsts(),
            vt: self.vt(),
            link_fail: self.link_fail(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port external MDIO configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pemdiocr(pub u32);
impl Pemdiocr {
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
    #[doc = "MDIO Read (and write) Error"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_rd_er(&self) -> super::vals::MdioRdEr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::MdioRdEr::from_bits(val as u8)
    }
    #[doc = "MDIO Read (and write) Error"]
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
    #[doc = "Returns the link ID"]
    #[must_use]
    #[inline(always)]
    pub const fn whoami(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Returns the link ID"]
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
    #[doc = "MDIO Command Completion Interrupt Mask"]
    #[must_use]
    #[inline(always)]
    pub const fn cim(&self) -> super::vals::Cim {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Cim::from_bits(val as u8)
    }
    #[doc = "MDIO Command Completion Interrupt Mask"]
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
impl Default for Pemdiocr {
    #[inline(always)]
    fn default() -> Pemdiocr {
        Pemdiocr(267336u64 as u32)
    }
}
impl core::fmt::Debug for Pemdiocr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pemdiocr")
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
impl defmt::Format for Pemdiocr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pemdiocr {
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
        let proxy = Pemdiocr {
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
#[doc = "Port external MDIO interface control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pemdioicr(pub u32);
impl Pemdioicr {
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
impl Default for Pemdioicr {
    #[inline(always)]
    fn default() -> Pemdioicr {
        Pemdioicr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pemdioicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pemdioicr")
            .field("dev_addr", &self.dev_addr())
            .field("port_addr", &self.port_addr())
            .field("post_inc", &self.post_inc())
            .field("read", &self.read())
            .field("bsy", &self.bsy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pemdioicr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pemdioicr {
            dev_addr: u8,
            port_addr: u8,
            post_inc: bool,
            read: bool,
            bsy: bool,
        }
        let proxy = Pemdioicr {
            dev_addr: self.dev_addr(),
            port_addr: self.port_addr(),
            post_inc: self.post_inc(),
            read: self.read(),
            bsy: self.bsy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port external MDIO interface data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pemdioidr(pub u32);
impl Pemdioidr {
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
impl Default for Pemdioidr {
    #[inline(always)]
    fn default() -> Pemdioidr {
        Pemdioidr(0u64 as u32)
    }
}
impl core::fmt::Debug for Pemdioidr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pemdioidr")
            .field("mdio_data", &self.mdio_data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pemdioidr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pemdioidr {
            mdio_data: u16,
        }
        let proxy = Pemdioidr {
            mdio_data: self.mdio_data(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port external MDIO register address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pemdiorar(pub u32);
impl Pemdiorar {
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
impl Default for Pemdiorar {
    #[inline(always)]
    fn default() -> Pemdiorar {
        Pemdiorar(0u64 as u32)
    }
}
impl core::fmt::Debug for Pemdiorar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pemdiorar")
            .field("regaddr", &self.regaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pemdiorar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pemdiorar {
            regaddr: u16,
        }
        let proxy = Pemdiorar {
            regaddr: self.regaddr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port external MDIO status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pemdiosr(pub u32);
impl Pemdiosr {
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
impl Default for Pemdiosr {
    #[inline(always)]
    fn default() -> Pemdiosr {
        Pemdiosr(32768u64 as u32)
    }
}
impl core::fmt::Debug for Pemdiosr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pemdiosr")
            .field("bsy", &self.bsy())
            .field("wht_list", &self.wht_list())
            .field("wht_list_ena", &self.wht_list_ena())
            .field("port_id", &self.port_id())
            .field("req_type", &self.req_type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pemdiosr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pemdiosr {
            bsy: bool,
            wht_list: u8,
            wht_list_ena: bool,
            port_id: u8,
            req_type: bool,
        }
        let proxy = Pemdiosr {
            bsy: self.bsy(),
            wht_list: self.wht_list(),
            wht_list_ena: self.wht_list_ena(),
            port_id: self.port_id(),
            req_type: self.req_type(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Command and Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0CommandConfig(pub u32);
impl Pm0CommandConfig {
    #[doc = "MAC transmit path enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MAC transmit path enable"]
    #[inline(always)]
    pub const fn set_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MAC receive path enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MAC receive path enable"]
    #[inline(always)]
    pub const fn set_rx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Terminate/forward received PAUSE frames"]
    #[must_use]
    #[inline(always)]
    pub const fn pause_fwd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Terminate/forward received PAUSE frames"]
    #[inline(always)]
    pub const fn set_pause_fwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Ignore PAUSE frame quanta"]
    #[must_use]
    #[inline(always)]
    pub const fn pause_ign(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore PAUSE frame quanta"]
    #[inline(always)]
    pub const fn set_pause_ign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmit source MAC address insertion"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_addr_ins(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit source MAC address insertion"]
    #[inline(always)]
    pub const fn set_tx_addr_ins(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Loopback enable"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_ena(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback enable"]
    #[inline(always)]
    pub const fn set_loop_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Loopback mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbk_mode(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "Loopback mode"]
    #[inline(always)]
    pub const fn set_lpbk_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Control frame reception enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_frm_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Control frame reception enable"]
    #[inline(always)]
    pub const fn set_cnt_frm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Timestamp Point"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_pnt(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Point"]
    #[inline(always)]
    pub const fn set_ts_pnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable padding of frames in transmit direction (1, default)."]
    #[must_use]
    #[inline(always)]
    pub const fn txp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable padding of frames in transmit direction (1, default)."]
    #[inline(always)]
    pub const fn set_txp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Half Duplex Flow Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hd_fcen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Half Duplex Flow Control Enable"]
    #[inline(always)]
    pub const fn set_hd_fcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Tx flush"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_flush(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Tx flush"]
    #[inline(always)]
    pub const fn set_tx_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Low Power Idle Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_lowp_ena(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Low Power Idle Enable."]
    #[inline(always)]
    pub const fn set_tx_lowp_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Software Reset. Self clearing bit."]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset. Self clearing bit."]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Ingress flush enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_flush(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress flush enable"]
    #[inline(always)]
    pub const fn set_rx_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Transmit timestamp mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_mode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit timestamp mode"]
    #[inline(always)]
    pub const fn set_ts_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Magic Packet detection enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mg(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Magic Packet detection enable."]
    #[inline(always)]
    pub const fn set_mg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm0CommandConfig {
    #[inline(always)]
    fn default() -> Pm0CommandConfig {
        Pm0CommandConfig(32768u64 as u32)
    }
}
impl core::fmt::Debug for Pm0CommandConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0CommandConfig")
            .field("tx_en", &self.tx_en())
            .field("rx_en", &self.rx_en())
            .field("pause_fwd", &self.pause_fwd())
            .field("pause_ign", &self.pause_ign())
            .field("tx_addr_ins", &self.tx_addr_ins())
            .field("loop_ena", &self.loop_ena())
            .field("lpbk_mode", &self.lpbk_mode())
            .field("cnt_frm_en", &self.cnt_frm_en())
            .field("ts_pnt", &self.ts_pnt())
            .field("txp", &self.txp())
            .field("hd_fcen", &self.hd_fcen())
            .field("tx_flush", &self.tx_flush())
            .field("tx_lowp_ena", &self.tx_lowp_ena())
            .field("swr", &self.swr())
            .field("rx_flush", &self.rx_flush())
            .field("ts_mode", &self.ts_mode())
            .field("mg", &self.mg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0CommandConfig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0CommandConfig {
            tx_en: bool,
            rx_en: bool,
            pause_fwd: bool,
            pause_ign: bool,
            tx_addr_ins: bool,
            loop_ena: bool,
            lpbk_mode: u8,
            cnt_frm_en: bool,
            ts_pnt: bool,
            txp: bool,
            hd_fcen: bool,
            tx_flush: bool,
            tx_lowp_ena: bool,
            swr: bool,
            rx_flush: bool,
            ts_mode: bool,
            mg: bool,
        }
        let proxy = Pm0CommandConfig {
            tx_en: self.tx_en(),
            rx_en: self.rx_en(),
            pause_fwd: self.pause_fwd(),
            pause_ign: self.pause_ign(),
            tx_addr_ins: self.tx_addr_ins(),
            loop_ena: self.loop_ena(),
            lpbk_mode: self.lpbk_mode(),
            cnt_frm_en: self.cnt_frm_en(),
            ts_pnt: self.ts_pnt(),
            txp: self.txp(),
            hd_fcen: self.hd_fcen(),
            tx_flush: self.tx_flush(),
            tx_lowp_ena: self.tx_lowp_ena(),
            swr: self.swr(),
            rx_flush: self.rx_flush(),
            ts_mode: self.ts_mode(),
            mg: self.mg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 half-duplex backoff entropy register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0HdBackoffEntropy(pub u32);
impl Pm0HdBackoffEntropy {
    #[doc = "Half duplex backoff entropy"]
    #[must_use]
    #[inline(always)]
    pub const fn hd_backoff_entropy(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Half duplex backoff entropy"]
    #[inline(always)]
    pub const fn set_hd_backoff_entropy(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "SW programmable entropy valid"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_entropy_valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SW programmable entropy valid"]
    #[inline(always)]
    pub const fn set_sw_entropy_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm0HdBackoffEntropy {
    #[inline(always)]
    fn default() -> Pm0HdBackoffEntropy {
        Pm0HdBackoffEntropy(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0HdBackoffEntropy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0HdBackoffEntropy")
            .field("hd_backoff_entropy", &self.hd_backoff_entropy())
            .field("sw_entropy_valid", &self.sw_entropy_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0HdBackoffEntropy {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0HdBackoffEntropy {
            hd_backoff_entropy: u16,
            sw_entropy_valid: bool,
        }
        let proxy = Pm0HdBackoffEntropy {
            hd_backoff_entropy: self.hd_backoff_entropy(),
            sw_entropy_valid: self.sw_entropy_valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Half-Duplex Flow Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0HdFlowCtrl(pub u32);
impl Pm0HdFlowCtrl {
    #[doc = "Half-Duplex Back-Pressure Off Minimum"]
    #[must_use]
    #[inline(always)]
    pub const fn hd_bp_off_min(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Half-Duplex Back-Pressure Off Minimum"]
    #[inline(always)]
    pub const fn set_hd_bp_off_min(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Half-Duplex Back-Pressure On Maximum"]
    #[must_use]
    #[inline(always)]
    pub const fn hd_bp_on_max(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Half-Duplex Back-Pressure On Maximum"]
    #[inline(always)]
    pub const fn set_hd_bp_on_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pm0HdFlowCtrl {
    #[inline(always)]
    fn default() -> Pm0HdFlowCtrl {
        Pm0HdFlowCtrl(198967316u64 as u32)
    }
}
impl core::fmt::Debug for Pm0HdFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0HdFlowCtrl")
            .field("hd_bp_off_min", &self.hd_bp_off_min())
            .field("hd_bp_on_max", &self.hd_bp_on_max())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0HdFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0HdFlowCtrl {
            hd_bp_off_min: u16,
            hd_bp_on_max: u16,
        }
        let proxy = Pm0HdFlowCtrl {
            hd_bp_off_min: self.hd_bp_off_min(),
            hd_bp_on_max: self.hd_bp_on_max(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Interrupt Event Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0Ievent(pub u32);
impl Pm0Ievent {
    #[doc = "Transmit fifo empty event"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit fifo empty event"]
    #[inline(always)]
    pub const fn set_tx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive idle event"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_empty(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive idle event"]
    #[inline(always)]
    pub const fn set_rx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO overflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_ovfl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO overflow event."]
    #[inline(always)]
    pub const fn set_tx_ovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmit FIFO underflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_unfl(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO underflow event."]
    #[inline(always)]
    pub const fn set_tx_unfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive FIFO overflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_ovfl(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow event."]
    #[inline(always)]
    pub const fn set_rx_ovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Magic packet detection indication event"]
    #[must_use]
    #[inline(always)]
    pub const fn mgi(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Magic packet detection indication event"]
    #[inline(always)]
    pub const fn set_mgi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Clock Stop Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_csd(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Clock Stop Detection"]
    #[inline(always)]
    pub const fn set_tx_csd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Rx Clock Stop Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_csd(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Clock Stop Detection"]
    #[inline(always)]
    pub const fn set_rx_csd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Speed/Duplex Change"]
    #[must_use]
    #[inline(always)]
    pub const fn spd_dup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Speed/Duplex Change"]
    #[inline(always)]
    pub const fn set_spd_dup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "MAC merge frame SMD error received event"]
    #[must_use]
    #[inline(always)]
    pub const fn mrg_serr(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "MAC merge frame SMD error received event"]
    #[inline(always)]
    pub const fn set_mrg_serr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "MAC merge frame assembly error event"]
    #[must_use]
    #[inline(always)]
    pub const fn mrg_aerr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "MAC merge frame assembly error event"]
    #[inline(always)]
    pub const fn set_mrg_aerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Pm0Ievent {
    #[inline(always)]
    fn default() -> Pm0Ievent {
        Pm0Ievent(96u64 as u32)
    }
}
impl core::fmt::Debug for Pm0Ievent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0Ievent")
            .field("tx_empty", &self.tx_empty())
            .field("rx_empty", &self.rx_empty())
            .field("tx_ovfl", &self.tx_ovfl())
            .field("tx_unfl", &self.tx_unfl())
            .field("rx_ovfl", &self.rx_ovfl())
            .field("mgi", &self.mgi())
            .field("tx_csd", &self.tx_csd())
            .field("rx_csd", &self.rx_csd())
            .field("spd_dup", &self.spd_dup())
            .field("mrg_serr", &self.mrg_serr())
            .field("mrg_aerr", &self.mrg_aerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0Ievent {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0Ievent {
            tx_empty: bool,
            rx_empty: bool,
            tx_ovfl: bool,
            tx_unfl: bool,
            rx_ovfl: bool,
            mgi: bool,
            tx_csd: bool,
            rx_csd: bool,
            spd_dup: bool,
            mrg_serr: bool,
            mrg_aerr: bool,
        }
        let proxy = Pm0Ievent {
            tx_empty: self.tx_empty(),
            rx_empty: self.rx_empty(),
            tx_ovfl: self.tx_ovfl(),
            tx_unfl: self.tx_unfl(),
            rx_ovfl: self.rx_ovfl(),
            mgi: self.mgi(),
            tx_csd: self.tx_csd(),
            rx_csd: self.rx_csd(),
            spd_dup: self.spd_dup(),
            mrg_serr: self.mrg_serr(),
            mrg_aerr: self.mrg_aerr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Interface Mode Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0IfMode(pub u32);
impl Pm0IfMode {
    #[doc = "Interface mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ifmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Interface mode"]
    #[inline(always)]
    pub const fn set_ifmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reverse Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> super::vals::Pm0IfModeRevmii {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pm0IfModeRevmii::from_bits(val as u8)
    }
    #[doc = "Reverse Mode"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: super::vals::Pm0IfModeRevmii) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "0 - 100 Mbps RMII/MII 1 - 10 Mbps RMII/MII (MII speed select is valid only in case of RevMII)"]
    #[must_use]
    #[inline(always)]
    pub const fn m10(&self) -> super::vals::Pm0IfModeM10 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pm0IfModeM10::from_bits(val as u8)
    }
    #[doc = "0 - 100 Mbps RMII/MII 1 - 10 Mbps RMII/MII (MII speed select is valid only in case of RevMII)"]
    #[inline(always)]
    pub const fn set_m10(&mut self, val: super::vals::Pm0IfModeM10) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Half-duplex"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> super::vals::Pm0IfModeHd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pm0IfModeHd::from_bits(val as u8)
    }
    #[doc = "Half-duplex"]
    #[inline(always)]
    pub const fn set_hd(&mut self, val: super::vals::Pm0IfModeHd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Clock Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_stop(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop"]
    #[inline(always)]
    pub const fn set_clk_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn ssp(&self) -> super::vals::Pm0IfModeSsp {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Pm0IfModeSsp::from_bits(val as u8)
    }
    #[doc = "Set Speed"]
    #[inline(always)]
    pub const fn set_ssp(&mut self, val: super::vals::Pm0IfModeSsp) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
}
impl Default for Pm0IfMode {
    #[inline(always)]
    fn default() -> Pm0IfMode {
        Pm0IfMode(16388u64 as u32)
    }
}
impl core::fmt::Debug for Pm0IfMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0IfMode")
            .field("ifmode", &self.ifmode())
            .field("revmii", &self.revmii())
            .field("m10", &self.m10())
            .field("hd", &self.hd())
            .field("clk_stop", &self.clk_stop())
            .field("ssp", &self.ssp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0IfMode {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0IfMode {
            ifmode: u8,
            revmii: super::vals::Pm0IfModeRevmii,
            m10: super::vals::Pm0IfModeM10,
            hd: super::vals::Pm0IfModeHd,
            clk_stop: bool,
            ssp: super::vals::Pm0IfModeSsp,
        }
        let proxy = Pm0IfMode {
            ifmode: self.ifmode(),
            revmii: self.revmii(),
            m10: self.m10(),
            hd: self.hd(),
            clk_stop: self.clk_stop(),
            ssp: self.ssp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Interrupt Mask Register(INT_MASK)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0Imask(pub u32);
impl Pm0Imask {
    #[doc = "Magic packet detection indication event mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mgi(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Magic packet detection indication event mask."]
    #[inline(always)]
    pub const fn set_mgi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Clock Stop Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_csd(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Clock Stop Detection"]
    #[inline(always)]
    pub const fn set_tx_csd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Rx Clock Stop Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_csd(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Clock Stop Detection"]
    #[inline(always)]
    pub const fn set_rx_csd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Speed/Duplex change event mask."]
    #[must_use]
    #[inline(always)]
    pub const fn spd_dup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Speed/Duplex change event mask."]
    #[inline(always)]
    pub const fn set_spd_dup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "MAC merge frame SMD error received event interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mrg_serr(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "MAC merge frame SMD error received event interrupt mask"]
    #[inline(always)]
    pub const fn set_mrg_serr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "MAC merge frame assembly error event interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mrg_aerr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "MAC merge frame assembly error event interrupt mask"]
    #[inline(always)]
    pub const fn set_mrg_aerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Pm0Imask {
    #[inline(always)]
    fn default() -> Pm0Imask {
        Pm0Imask(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0Imask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0Imask")
            .field("mgi", &self.mgi())
            .field("tx_csd", &self.tx_csd())
            .field("rx_csd", &self.rx_csd())
            .field("spd_dup", &self.spd_dup())
            .field("mrg_serr", &self.mrg_serr())
            .field("mrg_aerr", &self.mrg_aerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0Imask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0Imask {
            mgi: bool,
            tx_csd: bool,
            rx_csd: bool,
            spd_dup: bool,
            mrg_serr: bool,
            mrg_aerr: bool,
        }
        let proxy = Pm0Imask {
            mgi: self.mgi(),
            tx_csd: self.tx_csd(),
            rx_csd: self.rx_csd(),
            spd_dup: self.spd_dup(),
            mrg_serr: self.mrg_serr(),
            mrg_aerr: self.mrg_aerr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 EEE Low Power Wakeup Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0LpwakeTimer(pub u32);
impl Pm0LpwakeTimer {
    #[doc = "EEE System transmit wait time"]
    #[must_use]
    #[inline(always)]
    pub const fn tw_sys_tx(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "EEE System transmit wait time"]
    #[inline(always)]
    pub const fn set_tw_sys_tx(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Pm0LpwakeTimer {
    #[inline(always)]
    fn default() -> Pm0LpwakeTimer {
        Pm0LpwakeTimer(8192u64 as u32)
    }
}
impl core::fmt::Debug for Pm0LpwakeTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0LpwakeTimer")
            .field("tw_sys_tx", &self.tw_sys_tx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0LpwakeTimer {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0LpwakeTimer {
            tw_sys_tx: u32,
        }
        let proxy = Pm0LpwakeTimer {
            tw_sys_tx: self.tw_sys_tx(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 MAC Address Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0MacAddr1(pub u32);
impl Pm0MacAddr1 {
    #[doc = "MAC address 1"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address 1"]
    #[inline(always)]
    pub const fn set_mac_addr_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pm0MacAddr1 {
    #[inline(always)]
    fn default() -> Pm0MacAddr1 {
        Pm0MacAddr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0MacAddr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0MacAddr1")
            .field("mac_addr_1", &self.mac_addr_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0MacAddr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0MacAddr1 {
            mac_addr_1: u16,
        }
        let proxy = Pm0MacAddr1 {
            mac_addr_1: self.mac_addr_1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Maximum Frame Length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0Maxfrm(pub u32);
impl Pm0Maxfrm {
    #[doc = "Maximum supported received frame length."]
    #[must_use]
    #[inline(always)]
    pub const fn maxfrm(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum supported received frame length."]
    #[inline(always)]
    pub const fn set_maxfrm(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Maximum transmit frame length"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_mtu(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum transmit frame length"]
    #[inline(always)]
    pub const fn set_tx_mtu(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pm0Maxfrm {
    #[inline(always)]
    fn default() -> Pm0Maxfrm {
        Pm0Maxfrm(1536u64 as u32)
    }
}
impl core::fmt::Debug for Pm0Maxfrm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0Maxfrm")
            .field("maxfrm", &self.maxfrm())
            .field("tx_mtu", &self.tx_mtu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0Maxfrm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0Maxfrm {
            maxfrm: u16,
            tx_mtu: u16,
        }
        let proxy = Pm0Maxfrm {
            maxfrm: self.maxfrm(),
            tx_mtu: self.tx_mtu(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Internal MDIO Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0MdioCfg(pub u32);
impl Pm0MdioCfg {
    #[doc = "MDIO busy (same as bit 31)"]
    #[must_use]
    #[inline(always)]
    pub const fn bsy2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO busy (same as bit 31)"]
    #[inline(always)]
    pub const fn set_bsy2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MDIO hold time"]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_hold(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "MDIO hold time"]
    #[inline(always)]
    pub const fn set_mdio_hold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "MDIO preamble disable."]
    #[must_use]
    #[inline(always)]
    pub const fn pre_dis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO preamble disable."]
    #[inline(always)]
    pub const fn set_pre_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Clause 45 support."]
    #[must_use]
    #[inline(always)]
    pub const fn enc45(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Clause 45 support."]
    #[inline(always)]
    pub const fn set_enc45(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "MDIO clock divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn mdio_clk_div(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "MDIO clock divisor."]
    #[inline(always)]
    pub const fn set_mdio_clk_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
    #[doc = "MDIO command completion interrupt mask."]
    #[must_use]
    #[inline(always)]
    pub const fn cim(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO command completion interrupt mask."]
    #[inline(always)]
    pub const fn set_cim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "MDIO command completion event. Bit is cleared by writing `1'."]
    #[must_use]
    #[inline(always)]
    pub const fn cmp(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO command completion event. Bit is cleared by writing `1'."]
    #[inline(always)]
    pub const fn set_cmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "MDIO busy"]
    #[must_use]
    #[inline(always)]
    pub const fn bsy1(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO busy"]
    #[inline(always)]
    pub const fn set_bsy1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm0MdioCfg {
    #[inline(always)]
    fn default() -> Pm0MdioCfg {
        Pm0MdioCfg(5192u64 as u32)
    }
}
impl core::fmt::Debug for Pm0MdioCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0MdioCfg")
            .field("bsy2", &self.bsy2())
            .field("mdio_hold", &self.mdio_hold())
            .field("pre_dis", &self.pre_dis())
            .field("enc45", &self.enc45())
            .field("mdio_clk_div", &self.mdio_clk_div())
            .field("cim", &self.cim())
            .field("cmp", &self.cmp())
            .field("bsy1", &self.bsy1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0MdioCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0MdioCfg {
            bsy2: bool,
            mdio_hold: u8,
            pre_dis: bool,
            enc45: bool,
            mdio_clk_div: u16,
            cim: bool,
            cmp: bool,
            bsy1: bool,
        }
        let proxy = Pm0MdioCfg {
            bsy2: self.bsy2(),
            mdio_hold: self.mdio_hold(),
            pre_dis: self.pre_dis(),
            enc45: self.enc45(),
            mdio_clk_div: self.mdio_clk_div(),
            cim: self.cim(),
            cmp: self.cmp(),
            bsy1: self.bsy1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Internal MDIO Interface Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0MdioCtl(pub u32);
impl Pm0MdioCtl {
    #[doc = "MDIO register address (Clause 22)"]
    #[must_use]
    #[inline(always)]
    pub const fn dev_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "MDIO register address (Clause 22)"]
    #[inline(always)]
    pub const fn set_dev_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "MDIO PHY address (Clause 22)"]
    #[must_use]
    #[inline(always)]
    pub const fn port_addr(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "MDIO PHY address (Clause 22)"]
    #[inline(always)]
    pub const fn set_port_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
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
impl Default for Pm0MdioCtl {
    #[inline(always)]
    fn default() -> Pm0MdioCtl {
        Pm0MdioCtl(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0MdioCtl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0MdioCtl")
            .field("dev_addr", &self.dev_addr())
            .field("port_addr", &self.port_addr())
            .field("read", &self.read())
            .field("bsy", &self.bsy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0MdioCtl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0MdioCtl {
            dev_addr: u8,
            port_addr: u8,
            read: bool,
            bsy: bool,
        }
        let proxy = Pm0MdioCtl {
            dev_addr: self.dev_addr(),
            port_addr: self.port_addr(),
            read: self.read(),
            bsy: self.bsy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Internal MDIO Interface Data Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0MdioData(pub u32);
impl Pm0MdioData {
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
    #[doc = "MDIO busy bit. The state of this bit is also reflected in MDIO_CFG\\[BSY\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "MDIO busy bit. The state of this bit is also reflected in MDIO_CFG\\[BSY\\]."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm0MdioData {
    #[inline(always)]
    fn default() -> Pm0MdioData {
        Pm0MdioData(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0MdioData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0MdioData")
            .field("mdio_data", &self.mdio_data())
            .field("busy", &self.busy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0MdioData {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0MdioData {
            mdio_data: u16,
            busy: bool,
        }
        let proxy = Pm0MdioData {
            mdio_data: self.mdio_data(),
            busy: self.busy(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Minimum Frame Length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0Minfrm(pub u32);
impl Pm0Minfrm {
    #[doc = "Receive Minimum Frame Length size in bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Receive Minimum Frame Length size in bytes."]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Pm0Minfrm {
    #[inline(always)]
    fn default() -> Pm0Minfrm {
        Pm0Minfrm(64u64 as u32)
    }
}
impl core::fmt::Debug for Pm0Minfrm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0Minfrm")
            .field("num_bytes", &self.num_bytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0Minfrm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0Minfrm {
            num_bytes: u8,
        }
        let proxy = Pm0Minfrm {
            num_bytes: self.num_bytes(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Pause Quanta Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0PauseQuanta(pub u32);
impl Pm0PauseQuanta {
    #[doc = "Value to be used for the quanta value when XOFF is triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn pqnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value to be used for the quanta value when XOFF is triggered."]
    #[inline(always)]
    pub const fn set_pqnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pm0PauseQuanta {
    #[inline(always)]
    fn default() -> Pm0PauseQuanta {
        Pm0PauseQuanta(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0PauseQuanta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0PauseQuanta")
            .field("pqnt", &self.pqnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0PauseQuanta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0PauseQuanta {
            pqnt: u16,
        }
        let proxy = Pm0PauseQuanta { pqnt: self.pqnt() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Pause Quanta Threshold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0PauseThresh(pub u32);
impl Pm0PauseThresh {
    #[doc = "Quanta threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn qth(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Quanta threshold."]
    #[inline(always)]
    pub const fn set_qth(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pm0PauseThresh {
    #[inline(always)]
    fn default() -> Pm0PauseThresh {
        Pm0PauseThresh(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0PauseThresh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0PauseThresh")
            .field("qth", &self.qth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0PauseThresh {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0PauseThresh {
            qth: u16,
        }
        let proxy = Pm0PauseThresh { qth: self.qth() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Receive Pause Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0RxPauseStatus(pub u32);
impl Pm0RxPauseStatus {
    #[doc = "Pause status."]
    #[must_use]
    #[inline(always)]
    pub const fn pstat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pause status."]
    #[inline(always)]
    pub const fn set_pstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Pm0RxPauseStatus {
    #[inline(always)]
    fn default() -> Pm0RxPauseStatus {
        Pm0RxPauseStatus(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0RxPauseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0RxPauseStatus")
            .field("pstat", &self.pstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0RxPauseStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0RxPauseStatus {
            pstat: bool,
        }
        let proxy = Pm0RxPauseStatus {
            pstat: self.pstat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 IEEE1588 Single-Step Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0SingleStep(pub u32);
impl Pm0SingleStep {
    #[doc = "Checksum update"]
    #[must_use]
    #[inline(always)]
    pub const fn ch(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Checksum update"]
    #[inline(always)]
    pub const fn set_ch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Start offset from the beginning of a frame where the field to update is found (index to MS byte)"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "Start offset from the beginning of a frame where the field to update is found (index to MS byte)"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
    #[doc = "IEEE-1588 Single-Step enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IEEE-1588 Single-Step enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm0SingleStep {
    #[inline(always)]
    fn default() -> Pm0SingleStep {
        Pm0SingleStep(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0SingleStep {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0SingleStep")
            .field("ch", &self.ch())
            .field("offset", &self.offset())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0SingleStep {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0SingleStep {
            ch: bool,
            offset: u16,
            en: bool,
        }
        let proxy = Pm0SingleStep {
            ch: self.ch(),
            offset: self.offset(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Transmit EEE Low Power Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0SleepTimer(pub u32);
impl Pm0SleepTimer {
    #[doc = "Defines the number of NETC cycles (which represents time) where Tx is idle before mac transmits low power EEE"]
    #[must_use]
    #[inline(always)]
    pub const fn sleept(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Defines the number of NETC cycles (which represents time) where Tx is idle before mac transmits low power EEE"]
    #[inline(always)]
    pub const fn set_sleept(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Pm0SleepTimer {
    #[inline(always)]
    fn default() -> Pm0SleepTimer {
        Pm0SleepTimer(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0SleepTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0SleepTimer")
            .field("sleept", &self.sleept())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0SleepTimer {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0SleepTimer {
            sleept: u32,
        }
        let proxy = Pm0SleepTimer {
            sleept: self.sleept(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Statistics Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0StatnConfig(pub u32);
impl Pm0StatnConfig {
    #[doc = "0 - (default) counters are wrapping around1- the counters saturate at the maximum value"]
    #[must_use]
    #[inline(always)]
    pub const fn sat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 - (default) counters are wrapping around1- the counters saturate at the maximum value"]
    #[inline(always)]
    pub const fn set_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 - (default) counters are not affected by read.1 - a read to a counter resets it to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cod(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 - (default) counters are not affected by read.1 - a read to a counter resets it to 0."]
    #[inline(always)]
    pub const fn set_cod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1 - all counters will be reset to 0"]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 - all counters will be reset to 0"]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write enable for Tx/Rx stats registers"]
    #[must_use]
    #[inline(always)]
    pub const fn wen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write enable for Tx/Rx stats registers"]
    #[inline(always)]
    pub const fn set_wen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Pm0StatnConfig {
    #[inline(always)]
    fn default() -> Pm0StatnConfig {
        Pm0StatnConfig(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm0StatnConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0StatnConfig")
            .field("sat", &self.sat())
            .field("cod", &self.cod())
            .field("clr", &self.clr())
            .field("wen", &self.wen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0StatnConfig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0StatnConfig {
            sat: bool,
            cod: bool,
            clr: bool,
            wen: bool,
        }
        let proxy = Pm0StatnConfig {
            sat: self.sat(),
            cod: self.cod(),
            clr: self.clr(),
            wen: self.wen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 0 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm0TxIpgPreamble(pub u32);
impl Pm0TxIpgPreamble {
    #[doc = "Transmit inter-packet gap value."]
    #[must_use]
    #[inline(always)]
    pub const fn ipg_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmit inter-packet gap value."]
    #[inline(always)]
    pub const fn set_ipg_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Flexible Preamble Count"]
    #[must_use]
    #[inline(always)]
    pub const fn flex_preamble_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Flexible Preamble Count"]
    #[inline(always)]
    pub const fn set_flex_preamble_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Enable Flexible Preamble Count"]
    #[must_use]
    #[inline(always)]
    pub const fn flex_preamble_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Flexible Preamble Count"]
    #[inline(always)]
    pub const fn set_flex_preamble_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm0TxIpgPreamble {
    #[inline(always)]
    fn default() -> Pm0TxIpgPreamble {
        Pm0TxIpgPreamble(1804u64 as u32)
    }
}
impl core::fmt::Debug for Pm0TxIpgPreamble {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm0TxIpgPreamble")
            .field("ipg_len", &self.ipg_len())
            .field("flex_preamble_cnt", &self.flex_preamble_cnt())
            .field("flex_preamble_en", &self.flex_preamble_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm0TxIpgPreamble {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm0TxIpgPreamble {
            ipg_len: u8,
            flex_preamble_cnt: u8,
            flex_preamble_en: bool,
        }
        let proxy = Pm0TxIpgPreamble {
            ipg_len: self.ipg_len(),
            flex_preamble_cnt: self.flex_preamble_cnt(),
            flex_preamble_en: self.flex_preamble_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Command and Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1CommandConfig(pub u32);
impl Pm1CommandConfig {
    #[doc = "MAC transmit path enable"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "MAC transmit path enable"]
    #[inline(always)]
    pub const fn set_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "MAC receive path enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "MAC receive path enable"]
    #[inline(always)]
    pub const fn set_rx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Terminate/forward received PAUSE frames"]
    #[must_use]
    #[inline(always)]
    pub const fn pause_fwd(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Terminate/forward received PAUSE frames"]
    #[inline(always)]
    pub const fn set_pause_fwd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Ignore PAUSE frame quanta"]
    #[must_use]
    #[inline(always)]
    pub const fn pause_ign(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore PAUSE frame quanta"]
    #[inline(always)]
    pub const fn set_pause_ign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transmit source MAC address insertion"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_addr_ins(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit source MAC address insertion"]
    #[inline(always)]
    pub const fn set_tx_addr_ins(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Loopback enable"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_ena(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback enable"]
    #[inline(always)]
    pub const fn set_loop_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Loopback mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lpbk_mode(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "Loopback mode"]
    #[inline(always)]
    pub const fn set_lpbk_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Control frame reception enable"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt_frm_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Control frame reception enable"]
    #[inline(always)]
    pub const fn set_cnt_frm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Timestamp Point"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_pnt(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Timestamp Point"]
    #[inline(always)]
    pub const fn set_ts_pnt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Enable padding of frames in transmit direction (1, default)."]
    #[must_use]
    #[inline(always)]
    pub const fn txp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Enable padding of frames in transmit direction (1, default)."]
    #[inline(always)]
    pub const fn set_txp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Half Duplex Flow Control Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hd_fcen(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Half Duplex Flow Control Enable"]
    #[inline(always)]
    pub const fn set_hd_fcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Tx flush"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_flush(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Tx flush"]
    #[inline(always)]
    pub const fn set_tx_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Low Power Idle Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_lowp_ena(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit Low Power Idle Enable."]
    #[inline(always)]
    pub const fn set_tx_lowp_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Software Reset. Self clearing bit."]
    #[must_use]
    #[inline(always)]
    pub const fn swr(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Software Reset. Self clearing bit."]
    #[inline(always)]
    pub const fn set_swr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Ingress flush enable"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_flush(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Ingress flush enable"]
    #[inline(always)]
    pub const fn set_rx_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Transmit timestamp mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ts_mode(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit timestamp mode"]
    #[inline(always)]
    pub const fn set_ts_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Magic Packet detection enable."]
    #[must_use]
    #[inline(always)]
    pub const fn mg(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Magic Packet detection enable."]
    #[inline(always)]
    pub const fn set_mg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm1CommandConfig {
    #[inline(always)]
    fn default() -> Pm1CommandConfig {
        Pm1CommandConfig(32768u64 as u32)
    }
}
impl core::fmt::Debug for Pm1CommandConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1CommandConfig")
            .field("tx_en", &self.tx_en())
            .field("rx_en", &self.rx_en())
            .field("pause_fwd", &self.pause_fwd())
            .field("pause_ign", &self.pause_ign())
            .field("tx_addr_ins", &self.tx_addr_ins())
            .field("loop_ena", &self.loop_ena())
            .field("lpbk_mode", &self.lpbk_mode())
            .field("cnt_frm_en", &self.cnt_frm_en())
            .field("ts_pnt", &self.ts_pnt())
            .field("txp", &self.txp())
            .field("hd_fcen", &self.hd_fcen())
            .field("tx_flush", &self.tx_flush())
            .field("tx_lowp_ena", &self.tx_lowp_ena())
            .field("swr", &self.swr())
            .field("rx_flush", &self.rx_flush())
            .field("ts_mode", &self.ts_mode())
            .field("mg", &self.mg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1CommandConfig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1CommandConfig {
            tx_en: bool,
            rx_en: bool,
            pause_fwd: bool,
            pause_ign: bool,
            tx_addr_ins: bool,
            loop_ena: bool,
            lpbk_mode: u8,
            cnt_frm_en: bool,
            ts_pnt: bool,
            txp: bool,
            hd_fcen: bool,
            tx_flush: bool,
            tx_lowp_ena: bool,
            swr: bool,
            rx_flush: bool,
            ts_mode: bool,
            mg: bool,
        }
        let proxy = Pm1CommandConfig {
            tx_en: self.tx_en(),
            rx_en: self.rx_en(),
            pause_fwd: self.pause_fwd(),
            pause_ign: self.pause_ign(),
            tx_addr_ins: self.tx_addr_ins(),
            loop_ena: self.loop_ena(),
            lpbk_mode: self.lpbk_mode(),
            cnt_frm_en: self.cnt_frm_en(),
            ts_pnt: self.ts_pnt(),
            txp: self.txp(),
            hd_fcen: self.hd_fcen(),
            tx_flush: self.tx_flush(),
            tx_lowp_ena: self.tx_lowp_ena(),
            swr: self.swr(),
            rx_flush: self.rx_flush(),
            ts_mode: self.ts_mode(),
            mg: self.mg(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 half-duplex backoff entropy register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1HdBackoffEntropy(pub u32);
impl Pm1HdBackoffEntropy {
    #[doc = "Half duplex backoff entropy"]
    #[must_use]
    #[inline(always)]
    pub const fn hd_backoff_entropy(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Half duplex backoff entropy"]
    #[inline(always)]
    pub const fn set_hd_backoff_entropy(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "SW programmable entropy valid"]
    #[must_use]
    #[inline(always)]
    pub const fn sw_entropy_valid(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "SW programmable entropy valid"]
    #[inline(always)]
    pub const fn set_sw_entropy_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm1HdBackoffEntropy {
    #[inline(always)]
    fn default() -> Pm1HdBackoffEntropy {
        Pm1HdBackoffEntropy(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1HdBackoffEntropy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1HdBackoffEntropy")
            .field("hd_backoff_entropy", &self.hd_backoff_entropy())
            .field("sw_entropy_valid", &self.sw_entropy_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1HdBackoffEntropy {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1HdBackoffEntropy {
            hd_backoff_entropy: u16,
            sw_entropy_valid: bool,
        }
        let proxy = Pm1HdBackoffEntropy {
            hd_backoff_entropy: self.hd_backoff_entropy(),
            sw_entropy_valid: self.sw_entropy_valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Half-Duplex Flow Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1HdFlowCtrl(pub u32);
impl Pm1HdFlowCtrl {
    #[doc = "Half-Duplex Back-Pressure Off Minimum"]
    #[must_use]
    #[inline(always)]
    pub const fn hd_bp_off_min(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Half-Duplex Back-Pressure Off Minimum"]
    #[inline(always)]
    pub const fn set_hd_bp_off_min(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Half-Duplex Back-Pressure On Maximum"]
    #[must_use]
    #[inline(always)]
    pub const fn hd_bp_on_max(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Half-Duplex Back-Pressure On Maximum"]
    #[inline(always)]
    pub const fn set_hd_bp_on_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Pm1HdFlowCtrl {
    #[inline(always)]
    fn default() -> Pm1HdFlowCtrl {
        Pm1HdFlowCtrl(198967316u64 as u32)
    }
}
impl core::fmt::Debug for Pm1HdFlowCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1HdFlowCtrl")
            .field("hd_bp_off_min", &self.hd_bp_off_min())
            .field("hd_bp_on_max", &self.hd_bp_on_max())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1HdFlowCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1HdFlowCtrl {
            hd_bp_off_min: u16,
            hd_bp_on_max: u16,
        }
        let proxy = Pm1HdFlowCtrl {
            hd_bp_off_min: self.hd_bp_off_min(),
            hd_bp_on_max: self.hd_bp_on_max(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Interrupt Event Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1Ievent(pub u32);
impl Pm1Ievent {
    #[doc = "Transmit fifo empty event"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_empty(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit fifo empty event"]
    #[inline(always)]
    pub const fn set_tx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive idle event"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_empty(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive idle event"]
    #[inline(always)]
    pub const fn set_rx_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO overflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_ovfl(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO overflow event."]
    #[inline(always)]
    pub const fn set_tx_ovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Transmit FIFO underflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn tx_unfl(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO underflow event."]
    #[inline(always)]
    pub const fn set_tx_unfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Receive FIFO overflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn rx_ovfl(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow event."]
    #[inline(always)]
    pub const fn set_rx_ovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Magic packet detection indication event"]
    #[must_use]
    #[inline(always)]
    pub const fn mgi(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Magic packet detection indication event"]
    #[inline(always)]
    pub const fn set_mgi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Clock Stop Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_csd(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Clock Stop Detection"]
    #[inline(always)]
    pub const fn set_tx_csd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Rx Clock Stop Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_csd(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Clock Stop Detection"]
    #[inline(always)]
    pub const fn set_rx_csd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Speed/Duplex Change"]
    #[must_use]
    #[inline(always)]
    pub const fn spd_dup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Speed/Duplex Change"]
    #[inline(always)]
    pub const fn set_spd_dup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "MAC merge frame SMD error received event"]
    #[must_use]
    #[inline(always)]
    pub const fn mrg_serr(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "MAC merge frame SMD error received event"]
    #[inline(always)]
    pub const fn set_mrg_serr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "MAC merge frame assembly error event"]
    #[must_use]
    #[inline(always)]
    pub const fn mrg_aerr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "MAC merge frame assembly error event"]
    #[inline(always)]
    pub const fn set_mrg_aerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Pm1Ievent {
    #[inline(always)]
    fn default() -> Pm1Ievent {
        Pm1Ievent(96u64 as u32)
    }
}
impl core::fmt::Debug for Pm1Ievent {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1Ievent")
            .field("tx_empty", &self.tx_empty())
            .field("rx_empty", &self.rx_empty())
            .field("tx_ovfl", &self.tx_ovfl())
            .field("tx_unfl", &self.tx_unfl())
            .field("rx_ovfl", &self.rx_ovfl())
            .field("mgi", &self.mgi())
            .field("tx_csd", &self.tx_csd())
            .field("rx_csd", &self.rx_csd())
            .field("spd_dup", &self.spd_dup())
            .field("mrg_serr", &self.mrg_serr())
            .field("mrg_aerr", &self.mrg_aerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1Ievent {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1Ievent {
            tx_empty: bool,
            rx_empty: bool,
            tx_ovfl: bool,
            tx_unfl: bool,
            rx_ovfl: bool,
            mgi: bool,
            tx_csd: bool,
            rx_csd: bool,
            spd_dup: bool,
            mrg_serr: bool,
            mrg_aerr: bool,
        }
        let proxy = Pm1Ievent {
            tx_empty: self.tx_empty(),
            rx_empty: self.rx_empty(),
            tx_ovfl: self.tx_ovfl(),
            tx_unfl: self.tx_unfl(),
            rx_ovfl: self.rx_ovfl(),
            mgi: self.mgi(),
            tx_csd: self.tx_csd(),
            rx_csd: self.rx_csd(),
            spd_dup: self.spd_dup(),
            mrg_serr: self.mrg_serr(),
            mrg_aerr: self.mrg_aerr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Interface Mode Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1IfMode(pub u32);
impl Pm1IfMode {
    #[doc = "Interface mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ifmode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Interface mode"]
    #[inline(always)]
    pub const fn set_ifmode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Reverse Mode"]
    #[must_use]
    #[inline(always)]
    pub const fn revmii(&self) -> super::vals::Pm1IfModeRevmii {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pm1IfModeRevmii::from_bits(val as u8)
    }
    #[doc = "Reverse Mode"]
    #[inline(always)]
    pub const fn set_revmii(&mut self, val: super::vals::Pm1IfModeRevmii) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "0 - 100 Mbps RMII/MII 1 - 10 Mbps RMII/MII (MII speed select is valid only in case of RevMII)"]
    #[must_use]
    #[inline(always)]
    pub const fn m10(&self) -> super::vals::Pm1IfModeM10 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Pm1IfModeM10::from_bits(val as u8)
    }
    #[doc = "0 - 100 Mbps RMII/MII 1 - 10 Mbps RMII/MII (MII speed select is valid only in case of RevMII)"]
    #[inline(always)]
    pub const fn set_m10(&mut self, val: super::vals::Pm1IfModeM10) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Half-duplex"]
    #[must_use]
    #[inline(always)]
    pub const fn hd(&self) -> super::vals::Pm1IfModeHd {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Pm1IfModeHd::from_bits(val as u8)
    }
    #[doc = "Half-duplex"]
    #[inline(always)]
    pub const fn set_hd(&mut self, val: super::vals::Pm1IfModeHd) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Clock Stop"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_stop(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Clock Stop"]
    #[inline(always)]
    pub const fn set_clk_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set Speed"]
    #[must_use]
    #[inline(always)]
    pub const fn ssp(&self) -> super::vals::Pm1IfModeSsp {
        let val = (self.0 >> 13usize) & 0x03;
        super::vals::Pm1IfModeSsp::from_bits(val as u8)
    }
    #[doc = "Set Speed"]
    #[inline(always)]
    pub const fn set_ssp(&mut self, val: super::vals::Pm1IfModeSsp) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
    }
}
impl Default for Pm1IfMode {
    #[inline(always)]
    fn default() -> Pm1IfMode {
        Pm1IfMode(16388u64 as u32)
    }
}
impl core::fmt::Debug for Pm1IfMode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1IfMode")
            .field("ifmode", &self.ifmode())
            .field("revmii", &self.revmii())
            .field("m10", &self.m10())
            .field("hd", &self.hd())
            .field("clk_stop", &self.clk_stop())
            .field("ssp", &self.ssp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1IfMode {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1IfMode {
            ifmode: u8,
            revmii: super::vals::Pm1IfModeRevmii,
            m10: super::vals::Pm1IfModeM10,
            hd: super::vals::Pm1IfModeHd,
            clk_stop: bool,
            ssp: super::vals::Pm1IfModeSsp,
        }
        let proxy = Pm1IfMode {
            ifmode: self.ifmode(),
            revmii: self.revmii(),
            m10: self.m10(),
            hd: self.hd(),
            clk_stop: self.clk_stop(),
            ssp: self.ssp(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Interrupt Mask Register(INT_MASK)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1Imask(pub u32);
impl Pm1Imask {
    #[doc = "Magic packet detection indication event mask."]
    #[must_use]
    #[inline(always)]
    pub const fn mgi(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Magic packet detection indication event mask."]
    #[inline(always)]
    pub const fn set_mgi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Tx Clock Stop Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_csd(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Tx Clock Stop Detection"]
    #[inline(always)]
    pub const fn set_tx_csd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Rx Clock Stop Detection"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_csd(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Rx Clock Stop Detection"]
    #[inline(always)]
    pub const fn set_rx_csd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Speed/Duplex change event mask."]
    #[must_use]
    #[inline(always)]
    pub const fn spd_dup(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Speed/Duplex change event mask."]
    #[inline(always)]
    pub const fn set_spd_dup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "MAC merge frame SMD error received event interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mrg_serr(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "MAC merge frame SMD error received event interrupt mask"]
    #[inline(always)]
    pub const fn set_mrg_serr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "MAC merge frame assembly error event interrupt mask"]
    #[must_use]
    #[inline(always)]
    pub const fn mrg_aerr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "MAC merge frame assembly error event interrupt mask"]
    #[inline(always)]
    pub const fn set_mrg_aerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Pm1Imask {
    #[inline(always)]
    fn default() -> Pm1Imask {
        Pm1Imask(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1Imask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1Imask")
            .field("mgi", &self.mgi())
            .field("tx_csd", &self.tx_csd())
            .field("rx_csd", &self.rx_csd())
            .field("spd_dup", &self.spd_dup())
            .field("mrg_serr", &self.mrg_serr())
            .field("mrg_aerr", &self.mrg_aerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1Imask {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1Imask {
            mgi: bool,
            tx_csd: bool,
            rx_csd: bool,
            spd_dup: bool,
            mrg_serr: bool,
            mrg_aerr: bool,
        }
        let proxy = Pm1Imask {
            mgi: self.mgi(),
            tx_csd: self.tx_csd(),
            rx_csd: self.rx_csd(),
            spd_dup: self.spd_dup(),
            mrg_serr: self.mrg_serr(),
            mrg_aerr: self.mrg_aerr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 EEE Low Power Wakeup Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1LpwakeTimer(pub u32);
impl Pm1LpwakeTimer {
    #[doc = "EEE System transmit wait time"]
    #[must_use]
    #[inline(always)]
    pub const fn tw_sys_tx(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "EEE System transmit wait time"]
    #[inline(always)]
    pub const fn set_tw_sys_tx(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Pm1LpwakeTimer {
    #[inline(always)]
    fn default() -> Pm1LpwakeTimer {
        Pm1LpwakeTimer(8192u64 as u32)
    }
}
impl core::fmt::Debug for Pm1LpwakeTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1LpwakeTimer")
            .field("tw_sys_tx", &self.tw_sys_tx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1LpwakeTimer {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1LpwakeTimer {
            tw_sys_tx: u32,
        }
        let proxy = Pm1LpwakeTimer {
            tw_sys_tx: self.tw_sys_tx(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 MAC Address Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1MacAddr1(pub u32);
impl Pm1MacAddr1 {
    #[doc = "MAC address 1"]
    #[must_use]
    #[inline(always)]
    pub const fn mac_addr_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "MAC address 1"]
    #[inline(always)]
    pub const fn set_mac_addr_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pm1MacAddr1 {
    #[inline(always)]
    fn default() -> Pm1MacAddr1 {
        Pm1MacAddr1(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1MacAddr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1MacAddr1")
            .field("mac_addr_1", &self.mac_addr_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1MacAddr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1MacAddr1 {
            mac_addr_1: u16,
        }
        let proxy = Pm1MacAddr1 {
            mac_addr_1: self.mac_addr_1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Maximum Frame Length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1Maxfrm(pub u32);
impl Pm1Maxfrm {
    #[doc = "Maximum supported received frame length."]
    #[must_use]
    #[inline(always)]
    pub const fn maxfrm(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum supported received frame length."]
    #[inline(always)]
    pub const fn set_maxfrm(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Maximum transmit frame length"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_mtu(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Maximum transmit frame length"]
    #[inline(always)]
    pub const fn set_tx_mtu(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Pm1Maxfrm {
    #[inline(always)]
    fn default() -> Pm1Maxfrm {
        Pm1Maxfrm(1536u64 as u32)
    }
}
impl core::fmt::Debug for Pm1Maxfrm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1Maxfrm")
            .field("maxfrm", &self.maxfrm())
            .field("tx_mtu", &self.tx_mtu())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1Maxfrm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1Maxfrm {
            maxfrm: u16,
            tx_mtu: u16,
        }
        let proxy = Pm1Maxfrm {
            maxfrm: self.maxfrm(),
            tx_mtu: self.tx_mtu(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Minimum Frame Length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1Minfrm(pub u32);
impl Pm1Minfrm {
    #[doc = "Receive Minimum Frame Length size in bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn num_bytes(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Receive Minimum Frame Length size in bytes."]
    #[inline(always)]
    pub const fn set_num_bytes(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for Pm1Minfrm {
    #[inline(always)]
    fn default() -> Pm1Minfrm {
        Pm1Minfrm(64u64 as u32)
    }
}
impl core::fmt::Debug for Pm1Minfrm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1Minfrm")
            .field("num_bytes", &self.num_bytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1Minfrm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1Minfrm {
            num_bytes: u8,
        }
        let proxy = Pm1Minfrm {
            num_bytes: self.num_bytes(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Pause Quanta Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1PauseQuanta(pub u32);
impl Pm1PauseQuanta {
    #[doc = "Value to be used for the quanta value when XOFF is triggered."]
    #[must_use]
    #[inline(always)]
    pub const fn pqnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Value to be used for the quanta value when XOFF is triggered."]
    #[inline(always)]
    pub const fn set_pqnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pm1PauseQuanta {
    #[inline(always)]
    fn default() -> Pm1PauseQuanta {
        Pm1PauseQuanta(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1PauseQuanta {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1PauseQuanta")
            .field("pqnt", &self.pqnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1PauseQuanta {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1PauseQuanta {
            pqnt: u16,
        }
        let proxy = Pm1PauseQuanta { pqnt: self.pqnt() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Pause Quanta Threshold Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1PauseThresh(pub u32);
impl Pm1PauseThresh {
    #[doc = "Quanta threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn qth(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Quanta threshold."]
    #[inline(always)]
    pub const fn set_qth(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Pm1PauseThresh {
    #[inline(always)]
    fn default() -> Pm1PauseThresh {
        Pm1PauseThresh(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1PauseThresh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1PauseThresh")
            .field("qth", &self.qth())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1PauseThresh {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1PauseThresh {
            qth: u16,
        }
        let proxy = Pm1PauseThresh { qth: self.qth() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Receive Pause Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1RxPauseStatus(pub u32);
impl Pm1RxPauseStatus {
    #[doc = "Pause status."]
    #[must_use]
    #[inline(always)]
    pub const fn pstat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Pause status."]
    #[inline(always)]
    pub const fn set_pstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Pm1RxPauseStatus {
    #[inline(always)]
    fn default() -> Pm1RxPauseStatus {
        Pm1RxPauseStatus(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1RxPauseStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1RxPauseStatus")
            .field("pstat", &self.pstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1RxPauseStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1RxPauseStatus {
            pstat: bool,
        }
        let proxy = Pm1RxPauseStatus {
            pstat: self.pstat(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 IEEE1588 Single-Step Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1SingleStep(pub u32);
impl Pm1SingleStep {
    #[doc = "Checksum update"]
    #[must_use]
    #[inline(always)]
    pub const fn ch(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Checksum update"]
    #[inline(always)]
    pub const fn set_ch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Start offset from the beginning of a frame where the field to update is found (index to MS byte)"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u16 {
        let val = (self.0 >> 7usize) & 0x01ff;
        val as u16
    }
    #[doc = "Start offset from the beginning of a frame where the field to update is found (index to MS byte)"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
    }
    #[doc = "IEEE-1588 Single-Step enable."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IEEE-1588 Single-Step enable."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm1SingleStep {
    #[inline(always)]
    fn default() -> Pm1SingleStep {
        Pm1SingleStep(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1SingleStep {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1SingleStep")
            .field("ch", &self.ch())
            .field("offset", &self.offset())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1SingleStep {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1SingleStep {
            ch: bool,
            offset: u16,
            en: bool,
        }
        let proxy = Pm1SingleStep {
            ch: self.ch(),
            offset: self.offset(),
            en: self.en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Transmit EEE Low Power Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1SleepTimer(pub u32);
impl Pm1SleepTimer {
    #[doc = "Defines the number of NETC cycles (which represents time) where Tx is idle before mac transmits low power EEE"]
    #[must_use]
    #[inline(always)]
    pub const fn sleept(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Defines the number of NETC cycles (which represents time) where Tx is idle before mac transmits low power EEE"]
    #[inline(always)]
    pub const fn set_sleept(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Pm1SleepTimer {
    #[inline(always)]
    fn default() -> Pm1SleepTimer {
        Pm1SleepTimer(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1SleepTimer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1SleepTimer")
            .field("sleept", &self.sleept())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1SleepTimer {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1SleepTimer {
            sleept: u32,
        }
        let proxy = Pm1SleepTimer {
            sleept: self.sleept(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Statistics Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1StatnConfig(pub u32);
impl Pm1StatnConfig {
    #[doc = "0 - (default) counters are wrapping around1- the counters saturate at the maximum value"]
    #[must_use]
    #[inline(always)]
    pub const fn sat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 - (default) counters are wrapping around1- the counters saturate at the maximum value"]
    #[inline(always)]
    pub const fn set_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 - (default) counters are not affected by read.1 - a read to a counter resets it to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cod(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0 - (default) counters are not affected by read.1 - a read to a counter resets it to 0."]
    #[inline(always)]
    pub const fn set_cod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1 - all counters will be reset to 0"]
    #[must_use]
    #[inline(always)]
    pub const fn clr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 - all counters will be reset to 0"]
    #[inline(always)]
    pub const fn set_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write enable for Tx/Rx stats registers"]
    #[must_use]
    #[inline(always)]
    pub const fn wen(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write enable for Tx/Rx stats registers"]
    #[inline(always)]
    pub const fn set_wen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Pm1StatnConfig {
    #[inline(always)]
    fn default() -> Pm1StatnConfig {
        Pm1StatnConfig(0u64 as u32)
    }
}
impl core::fmt::Debug for Pm1StatnConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1StatnConfig")
            .field("sat", &self.sat())
            .field("cod", &self.cod())
            .field("clr", &self.clr())
            .field("wen", &self.wen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1StatnConfig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1StatnConfig {
            sat: bool,
            cod: bool,
            clr: bool,
            wen: bool,
        }
        let proxy = Pm1StatnConfig {
            sat: self.sat(),
            cod: self.cod(),
            clr: self.clr(),
            wen: self.wen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port MAC 1 Transmit Inter-Packet Gap Length and Flexible Preamble length Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pm1TxIpgPreamble(pub u32);
impl Pm1TxIpgPreamble {
    #[doc = "Transmit inter-packet gap value."]
    #[must_use]
    #[inline(always)]
    pub const fn ipg_len(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Transmit inter-packet gap value."]
    #[inline(always)]
    pub const fn set_ipg_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Flexible Preamble Count"]
    #[must_use]
    #[inline(always)]
    pub const fn flex_preamble_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Flexible Preamble Count"]
    #[inline(always)]
    pub const fn set_flex_preamble_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Enable Flexible Preamble Count"]
    #[must_use]
    #[inline(always)]
    pub const fn flex_preamble_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Flexible Preamble Count"]
    #[inline(always)]
    pub const fn set_flex_preamble_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pm1TxIpgPreamble {
    #[inline(always)]
    fn default() -> Pm1TxIpgPreamble {
        Pm1TxIpgPreamble(1804u64 as u32)
    }
}
impl core::fmt::Debug for Pm1TxIpgPreamble {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pm1TxIpgPreamble")
            .field("ipg_len", &self.ipg_len())
            .field("flex_preamble_cnt", &self.flex_preamble_cnt())
            .field("flex_preamble_en", &self.flex_preamble_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pm1TxIpgPreamble {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pm1TxIpgPreamble {
            ipg_len: u8,
            flex_preamble_cnt: u8,
            flex_preamble_en: bool,
        }
        let proxy = Pm1TxIpgPreamble {
            ipg_len: self.ipg_len(),
            flex_preamble_cnt: self.flex_preamble_cnt(),
            flex_preamble_en: self.flex_preamble_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "PHY status configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppscr(pub u32);
impl Ppscr {
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
impl Default for Ppscr {
    #[inline(always)]
    fn default() -> Ppscr {
        Ppscr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppscr")
            .field("bsy", &self.bsy())
            .field("mdio_rd_er", &self.mdio_rd_er())
            .field("status_interval", &self.status_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppscr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppscr {
            bsy: bool,
            mdio_rd_er: bool,
            status_interval: u16,
        }
        let proxy = Ppscr {
            bsy: self.bsy(),
            mdio_rd_er: self.mdio_rd_er(),
            status_interval: self.status_interval(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port PHY status control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppsctrlr(pub u32);
impl Ppsctrlr {
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
impl Default for Ppsctrlr {
    #[inline(always)]
    fn default() -> Ppsctrlr {
        Ppsctrlr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppsctrlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppsctrlr")
            .field("dev_addr", &self.dev_addr())
            .field("port_addr", &self.port_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppsctrlr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppsctrlr {
            dev_addr: u8,
            port_addr: u8,
        }
        let proxy = Ppsctrlr {
            dev_addr: self.dev_addr(),
            port_addr: self.port_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port PHY status data register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppsdr(pub u32);
impl Ppsdr {
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
impl Default for Ppsdr {
    #[inline(always)]
    fn default() -> Ppsdr {
        Ppsdr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppsdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppsdr")
            .field("mdio_data", &self.mdio_data())
            .field("curr_cnt", &self.curr_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppsdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppsdr {
            mdio_data: u16,
            curr_cnt: u16,
        }
        let proxy = Ppsdr {
            mdio_data: self.mdio_data(),
            curr_cnt: self.curr_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port PHY status event register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppser(pub u32);
impl Ppser {
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
impl Default for Ppser {
    #[inline(always)]
    fn default() -> Ppser {
        Ppser(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppser {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppser")
            .field("status_event_hl", &self.status_event_hl())
            .field("status_event_lh", &self.status_event_lh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppser {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppser {
            status_event_hl: u16,
            status_event_lh: u16,
        }
        let proxy = Ppser {
            status_event_hl: self.status_event_hl(),
            status_event_lh: self.status_event_lh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port PHY status mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppsmr(pub u32);
impl Ppsmr {
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
impl Default for Ppsmr {
    #[inline(always)]
    fn default() -> Ppsmr {
        Ppsmr(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppsmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppsmr")
            .field("status_mask_hl", &self.status_mask_hl())
            .field("status_mask_lh", &self.status_mask_lh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppsmr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppsmr {
            status_mask_hl: u16,
            status_mask_lh: u16,
        }
        let proxy = Ppsmr {
            status_mask_hl: self.status_mask_hl(),
            status_mask_lh: self.status_mask_lh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Port PHY status register address register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ppsrar(pub u32);
impl Ppsrar {
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
impl Default for Ppsrar {
    #[inline(always)]
    fn default() -> Ppsrar {
        Ppsrar(0u64 as u32)
    }
}
impl core::fmt::Debug for Ppsrar {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ppsrar")
            .field("regaddr", &self.regaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ppsrar {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ppsrar {
            regaddr: u16,
        }
        let proxy = Ppsrar {
            regaddr: self.regaddr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
