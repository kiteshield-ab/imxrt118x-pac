#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axibuserren {
    #[doc = "Interrupt is disabled"]
    InterruptDisable = 0x0,
    #[doc = "Interrupt is enabled"]
    InterruptEnable = 0x01,
}
impl Axibuserren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Axibuserren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Axibuserren {
    #[inline(always)]
    fn from(val: u8) -> Axibuserren {
        Axibuserren::from_bits(val)
    }
}
impl From<Axibuserren> for u8 {
    #[inline(always)]
    fn from(val: Axibuserren) -> u8 {
        Axibuserren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Axicmderren {
    #[doc = "Interrupt is disabled"]
    InterruptDisable = 0x0,
    #[doc = "Interrupt is enabled"]
    InterruptEnable = 0x01,
}
impl Axicmderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Axicmderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Axicmderren {
    #[inline(always)]
    fn from(val: u8) -> Axicmderren {
        Axicmderren::from_bits(val)
    }
}
impl From<Axicmderren> for u8 {
    #[inline(always)]
    fn from(val: Axicmderren) -> u8 {
        Axicmderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bank2 {
    #[doc = "SDRAM device has 4 banks."]
    Bank4 = 0x0,
    #[doc = "SDRAM device has 2 banks."]
    Bank2 = 0x01,
}
impl Bank2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bank2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bank2 {
    #[inline(always)]
    fn from(val: u8) -> Bank2 {
        Bank2::from_bits(val)
    }
}
impl From<Bank2> for u8 {
    #[inline(always)]
    fn from(val: Bank2) -> u8 {
        Bank2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Br10Ms {
    #[doc = "4KB"]
    Size4kb = 0x0,
    #[doc = "8KB"]
    Size8kb = 0x01,
    #[doc = "16KB"]
    Size16kb = 0x02,
    #[doc = "32KB"]
    Size32kb = 0x03,
    #[doc = "64KB"]
    Size64kb = 0x04,
    #[doc = "128KB"]
    Size128kb = 0x05,
    #[doc = "256KB"]
    Size256kb = 0x06,
    #[doc = "512KB"]
    Size512kb = 0x07,
    #[doc = "1MB"]
    Size1mb = 0x08,
    #[doc = "2MB"]
    Size2mb = 0x09,
    #[doc = "4MB"]
    Size4mb = 0x0a,
    #[doc = "8MB"]
    Size8mb = 0x0b,
    #[doc = "16MB"]
    Size16mb = 0x0c,
    #[doc = "32MB"]
    Size32mb = 0x0d,
    #[doc = "64MB"]
    Size64mb = 0x0e,
    #[doc = "128MB"]
    Size128mb = 0x0f,
    #[doc = "256MB"]
    Size256mb = 0x10,
    #[doc = "512MB"]
    Size512mb = 0x11,
    #[doc = "1GB"]
    Size1gb = 0x12,
    #[doc = "2GB"]
    Size2gb = 0x13,
    #[doc = "4GB"]
    Size4gb20 = 0x14,
    #[doc = "4GB"]
    Size4gb21 = 0x15,
    #[doc = "4GB"]
    Size4gb22 = 0x16,
    #[doc = "4GB"]
    Size4gb23 = 0x17,
    #[doc = "4GB"]
    Size4gb24 = 0x18,
    #[doc = "4GB"]
    Size4gb25 = 0x19,
    #[doc = "4GB"]
    Size4gb26 = 0x1a,
    #[doc = "4GB"]
    Size4gb27 = 0x1b,
    #[doc = "4GB"]
    Size4gb28 = 0x1c,
    #[doc = "4GB"]
    Size4gb29 = 0x1d,
    #[doc = "4GB"]
    Size4gb30 = 0x1e,
    #[doc = "4GB"]
    Size4gb31 = 0x1f,
}
impl Br10Ms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Br10Ms {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Br10Ms {
    #[inline(always)]
    fn from(val: u8) -> Br10Ms {
        Br10Ms::from_bits(val)
    }
}
impl From<Br10Ms> for u8 {
    #[inline(always)]
    fn from(val: Br10Ms) -> u8 {
        Br10Ms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Br11Ms {
    #[doc = "4KB"]
    Size4kb = 0x0,
    #[doc = "8KB"]
    Size8kb = 0x01,
    #[doc = "16KB"]
    Size16kb = 0x02,
    #[doc = "32KB"]
    Size32kb = 0x03,
    #[doc = "64KB"]
    Size64kb = 0x04,
    #[doc = "128KB"]
    Size128kb = 0x05,
    #[doc = "256KB"]
    Size256kb = 0x06,
    #[doc = "512KB"]
    Size512kb = 0x07,
    #[doc = "1MB"]
    Size1mb = 0x08,
    #[doc = "2MB"]
    Size2mb = 0x09,
    #[doc = "4MB"]
    Size4mb = 0x0a,
    #[doc = "8MB"]
    Size8mb = 0x0b,
    #[doc = "16MB"]
    Size16mb = 0x0c,
    #[doc = "32MB"]
    Size32mb = 0x0d,
    #[doc = "64MB"]
    Size64mb = 0x0e,
    #[doc = "128MB"]
    Size128mb = 0x0f,
    #[doc = "256MB"]
    Size256mb = 0x10,
    #[doc = "512MB"]
    Size512mb = 0x11,
    #[doc = "1GB"]
    Size1gb = 0x12,
    #[doc = "2GB"]
    Size2gb = 0x13,
    #[doc = "4GB"]
    Size4gb20 = 0x14,
    #[doc = "4GB"]
    Size4gb21 = 0x15,
    #[doc = "4GB"]
    Size4gb22 = 0x16,
    #[doc = "4GB"]
    Size4gb23 = 0x17,
    #[doc = "4GB"]
    Size4gb24 = 0x18,
    #[doc = "4GB"]
    Size4gb25 = 0x19,
    #[doc = "4GB"]
    Size4gb26 = 0x1a,
    #[doc = "4GB"]
    Size4gb27 = 0x1b,
    #[doc = "4GB"]
    Size4gb28 = 0x1c,
    #[doc = "4GB"]
    Size4gb29 = 0x1d,
    #[doc = "4GB"]
    Size4gb30 = 0x1e,
    #[doc = "4GB"]
    Size4gb31 = 0x1f,
}
impl Br11Ms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Br11Ms {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Br11Ms {
    #[inline(always)]
    fn from(val: u8) -> Br11Ms {
        Br11Ms::from_bits(val)
    }
}
impl From<Br11Ms> for u8 {
    #[inline(always)]
    fn from(val: Br11Ms) -> u8 {
        Br11Ms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Br9Ms {
    #[doc = "4KB"]
    Size4kb = 0x0,
    #[doc = "8KB"]
    Size8kb = 0x01,
    #[doc = "16KB"]
    Size16kb = 0x02,
    #[doc = "32KB"]
    Size32kb = 0x03,
    #[doc = "64KB"]
    Size64kb = 0x04,
    #[doc = "128KB"]
    Size128kb = 0x05,
    #[doc = "256KB"]
    Size256kb = 0x06,
    #[doc = "512KB"]
    Size512kb = 0x07,
    #[doc = "1MB"]
    Size1mb = 0x08,
    #[doc = "2MB"]
    Size2mb = 0x09,
    #[doc = "4MB"]
    Size4mb = 0x0a,
    #[doc = "8MB"]
    Size8mb = 0x0b,
    #[doc = "16MB"]
    Size16mb = 0x0c,
    #[doc = "32MB"]
    Size32mb = 0x0d,
    #[doc = "64MB"]
    Size64mb = 0x0e,
    #[doc = "128MB"]
    Size128mb = 0x0f,
    #[doc = "256MB"]
    Size256mb = 0x10,
    #[doc = "512MB"]
    Size512mb = 0x11,
    #[doc = "1GB"]
    Size1gb = 0x12,
    #[doc = "2GB"]
    Size2gb = 0x13,
    #[doc = "4GB"]
    Size4gb20 = 0x14,
    #[doc = "4GB"]
    Size4gb21 = 0x15,
    #[doc = "4GB"]
    Size4gb22 = 0x16,
    #[doc = "4GB"]
    Size4gb23 = 0x17,
    #[doc = "4GB"]
    Size4gb24 = 0x18,
    #[doc = "4GB"]
    Size4gb25 = 0x19,
    #[doc = "4GB"]
    Size4gb26 = 0x1a,
    #[doc = "4GB"]
    Size4gb27 = 0x1b,
    #[doc = "4GB"]
    Size4gb28 = 0x1c,
    #[doc = "4GB"]
    Size4gb29 = 0x1d,
    #[doc = "4GB"]
    Size4gb30 = 0x1e,
    #[doc = "4GB"]
    Size4gb31 = 0x1f,
}
impl Br9Ms {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Br9Ms {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Br9Ms {
    #[inline(always)]
    fn from(val: u8) -> Br9Ms {
        Br9Ms::from_bits(val)
    }
}
impl From<Br9Ms> for u8 {
    #[inline(always)]
    fn from(val: Br9Ms) -> u8 {
        Br9Ms::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BrMs {
    #[doc = "4KB"]
    Size4kb = 0x0,
    #[doc = "8KB"]
    Size8kb = 0x01,
    #[doc = "16KB"]
    Size16kb = 0x02,
    #[doc = "32KB"]
    Size32kb = 0x03,
    #[doc = "64KB"]
    Size64kb = 0x04,
    #[doc = "128KB"]
    Size128kb = 0x05,
    #[doc = "256KB"]
    Size256kb = 0x06,
    #[doc = "512KB"]
    Size512kb = 0x07,
    #[doc = "1MB"]
    Size1mb = 0x08,
    #[doc = "2MB"]
    Size2mb = 0x09,
    #[doc = "4MB"]
    Size4mb = 0x0a,
    #[doc = "8MB"]
    Size8mb = 0x0b,
    #[doc = "16MB"]
    Size16mb = 0x0c,
    #[doc = "32MB"]
    Size32mb = 0x0d,
    #[doc = "64MB"]
    Size64mb = 0x0e,
    #[doc = "128MB"]
    Size128mb = 0x0f,
    #[doc = "256MB"]
    Size256mb = 0x10,
    #[doc = "512MB"]
    Size512mb = 0x11,
    #[doc = "1GB"]
    Size1gb = 0x12,
    #[doc = "2GB"]
    Size2gb = 0x13,
    #[doc = "4GB"]
    Size4gb20 = 0x14,
    #[doc = "4GB"]
    Size4gb21 = 0x15,
    #[doc = "4GB"]
    Size4gb22 = 0x16,
    #[doc = "4GB"]
    Size4gb23 = 0x17,
    #[doc = "4GB"]
    Size4gb24 = 0x18,
    #[doc = "4GB"]
    Size4gb25 = 0x19,
    #[doc = "4GB"]
    Size4gb26 = 0x1a,
    #[doc = "4GB"]
    Size4gb27 = 0x1b,
    #[doc = "4GB"]
    Size4gb28 = 0x1c,
    #[doc = "4GB"]
    Size4gb29 = 0x1d,
    #[doc = "4GB"]
    Size4gb30 = 0x1e,
    #[doc = "4GB"]
    Size4gb31 = 0x1f,
}
impl BrMs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BrMs {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BrMs {
    #[inline(always)]
    fn from(val: u8) -> BrMs {
        BrMs::from_bits(val)
    }
}
impl From<BrMs> for u8 {
    #[inline(always)]
    fn from(val: BrMs) -> u8 {
        BrMs::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Bto(pub u8);
impl Bto {
    #[doc = "255*1"]
    pub const BTO_0: Self = Self(0x0);
    #[doc = "255*2"]
    pub const BTO_1: Self = Self(0x01);
    #[doc = "255*2^31"]
    pub const BTO_1F: Self = Self(0x1f);
}
impl Bto {
    pub const fn from_bits(val: u8) -> Bto {
        Self(val & 0x1f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Bto {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("BTO_0"),
            0x01 => f.write_str("BTO_1"),
            0x1f => f.write_str("BTO_1F"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bto {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "BTO_0"),
            0x01 => defmt::write!(f, "BTO_1"),
            0x1f => defmt::write!(f, "BTO_1F"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Bto {
    #[inline(always)]
    fn from(val: u8) -> Bto {
        Bto::from_bits(val)
    }
}
impl From<Bto> for u8 {
    #[inline(always)]
    fn from(val: Bto) -> u8 {
        Bto::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bufen {
    #[doc = "AXI access to NAND device directly"]
    AxiDirect = 0x0,
    #[doc = "AXI access through NAND buffer. It must be enabled for error correction schemes."]
    AxiBuffer = 0x01,
}
impl Bufen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bufen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bufen {
    #[inline(always)]
    fn from(val: u8) -> Bufen {
        Bufen::from_bits(val)
    }
}
impl From<Bufen> for u8 {
    #[inline(always)]
    fn from(val: Bufen) -> u8 {
        Bufen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cl {
    #[doc = "1"]
    Cl10 = 0x0,
    #[doc = "1"]
    Cl11 = 0x01,
    #[doc = "2"]
    Cl2 = 0x02,
    #[doc = "3"]
    Cl3 = 0x03,
}
impl Cl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cl {
    #[inline(always)]
    fn from(val: u8) -> Cl {
        Cl::from_bits(val)
    }
}
impl From<Cl> for u8 {
    #[inline(always)]
    fn from(val: Cl) -> u8 {
        Cl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkx0Ao {
    #[doc = "SEMC_CLKX0 is controlled by MUX_CLKX0"]
    MuxClkx0Ctl = 0x0,
    #[doc = "SEMC_CLKX0 is always on"]
    AlwaysOn = 0x01,
}
impl Clkx0Ao {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkx0Ao {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkx0Ao {
    #[inline(always)]
    fn from(val: u8) -> Clkx0Ao {
        Clkx0Ao::from_bits(val)
    }
}
impl From<Clkx0Ao> for u8 {
    #[inline(always)]
    fn from(val: Clkx0Ao) -> u8 {
        Clkx0Ao::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkx1Ao {
    #[doc = "SEMC_CLKX1 is controlled by MUX_CLKX1"]
    MuxClkx1Ctl = 0x0,
    #[doc = "SEMC_CLKX1 is always on"]
    AlwaysOn = 0x01,
}
impl Clkx1Ao {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkx1Ao {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkx1Ao {
    #[inline(always)]
    fn from(val: u8) -> Clkx1Ao {
        Clkx1Ao::from_bits(val)
    }
}
impl From<Clkx1Ao> for u8 {
    #[inline(always)]
    fn from(val: Clkx1Ao) -> u8 {
        Clkx1Ao::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Col8 {
    #[doc = "Column address bit number is decided by COL field."]
    Colfield = 0x0,
    #[doc = "Column address bit number is 8. COL field is ignored."]
    Bit8 = 0x01,
}
impl Col8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Col8 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Col8 {
    #[inline(always)]
    fn from(val: u8) -> Col8 {
        Col8::from_bits(val)
    }
}
impl From<Col8> for u8 {
    #[inline(always)]
    fn from(val: Col8) -> u8 {
        Col8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Datsz {
    #[doc = "4"]
    Datasz4byte = 0x0,
    #[doc = "1"]
    Datasz1byte = 0x01,
    #[doc = "2"]
    Datasz2byte = 0x02,
    #[doc = "3"]
    Datasz3byte = 0x03,
    #[doc = "4"]
    Datasz4byte4 = 0x04,
    #[doc = "4"]
    Datasz4byte5 = 0x05,
    #[doc = "4"]
    Datasz4byte6 = 0x06,
    #[doc = "4"]
    Datasz4byte7 = 0x07,
}
impl Datsz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Datsz {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Datsz {
    #[inline(always)]
    fn from(val: u8) -> Datsz {
        Datsz::from_bits(val)
    }
}
impl From<Datsz> for u8 {
    #[inline(always)]
    fn from(val: Datsz) -> u8 {
        Datsz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbicr0Bl {
    #[doc = "1"]
    Burst1 = 0x0,
    #[doc = "2"]
    Burst2 = 0x01,
    #[doc = "4"]
    Burst4 = 0x02,
    #[doc = "8"]
    Burst8 = 0x03,
    #[doc = "16"]
    Burst16 = 0x04,
    #[doc = "32"]
    Burst32 = 0x05,
    #[doc = "64"]
    Burst646 = 0x06,
    #[doc = "64"]
    Burst647 = 0x07,
}
impl Dbicr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbicr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbicr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Dbicr0Bl {
        Dbicr0Bl::from_bits(val)
    }
}
impl From<Dbicr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Dbicr0Bl) -> u8 {
        Dbicr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbicr0Col {
    #[doc = "12 Bits"]
    Bitwidth120 = 0x0,
    #[doc = "11 Bits"]
    Bitwidth11 = 0x01,
    #[doc = "10 Bits"]
    Bitwidth10 = 0x02,
    #[doc = "9 Bits"]
    Bitwidth9 = 0x03,
    #[doc = "8 Bits"]
    Bitwidth8 = 0x04,
    #[doc = "7 Bits"]
    Bitwidth7 = 0x05,
    #[doc = "6 Bits"]
    Bitwidth6 = 0x06,
    #[doc = "5 Bits"]
    Bitwidth5 = 0x07,
    #[doc = "4 Bits"]
    Bitwidth4 = 0x08,
    #[doc = "3 Bits"]
    Bitwidth3 = 0x09,
    #[doc = "2 Bits"]
    Bitwidth2 = 0x0a,
    #[doc = "12 Bits"]
    Bitwidth12B = 0x0b,
    #[doc = "12 Bits"]
    Bitwidth12C = 0x0c,
    #[doc = "12 Bits"]
    Bitwidth12D = 0x0d,
    #[doc = "12 Bits"]
    Bitwidth12E = 0x0e,
    #[doc = "12 Bits"]
    Bitwidth12F = 0x0f,
}
impl Dbicr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbicr0Col {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbicr0Col {
    #[inline(always)]
    fn from(val: u8) -> Dbicr0Col {
        Dbicr0Col::from_bits(val)
    }
}
impl From<Dbicr0Col> for u8 {
    #[inline(always)]
    fn from(val: Dbicr0Col) -> u8 {
        Dbicr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dbicr0Ps {
    #[doc = "8bit"]
    Ps8bit = 0x0,
    #[doc = "16bit"]
    Ps16bit = 0x01,
}
impl Dbicr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dbicr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dbicr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Dbicr0Ps {
        Dbicr0Ps::from_bits(val)
    }
}
impl From<Dbicr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Dbicr0Ps) -> u8 {
        Dbicr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dllen {
    #[doc = "DLL calibration is disabled."]
    CalDisable = 0x0,
    #[doc = "DLL calibration is enabled."]
    CalEnable = 0x01,
}
impl Dllen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dllen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dllen {
    #[inline(always)]
    fn from(val: u8) -> Dllen {
        Dllen::from_bits(val)
    }
}
impl From<Dllen> for u8 {
    #[inline(always)]
    fn from(val: Dllen) -> u8 {
        Dllen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dqsmd {
    #[doc = "Dummy read strobe loopbacked internally"]
    Internal = 0x0,
    #[doc = "Dummy read strobe loopbacked from DQS pad"]
    DqsPad = 0x01,
}
impl Dqsmd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dqsmd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dqsmd {
    #[inline(always)]
    fn from(val: u8) -> Dqsmd {
        Dqsmd::from_bits(val)
    }
}
impl From<Dqsmd> for u8 {
    #[inline(always)]
    fn from(val: Dqsmd) -> u8 {
        Dqsmd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EccMode {
    #[doc = "No correction, ECC bypass"]
    NoEcc = 0x0,
    #[doc = "4-error correction (8 ECC bytes)"]
    Ecc8byte = 0x01,
    #[doc = "8-error correction (16 ECC bytes)"]
    Ecc16byte = 0x02,
    _RESERVED_3 = 0x03,
}
impl EccMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EccMode {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EccMode {
    #[inline(always)]
    fn from(val: u8) -> EccMode {
        EccMode::from_bits(val)
    }
}
impl From<EccMode> for u8 {
    #[inline(always)]
    fn from(val: EccMode) -> u8 {
        EccMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Edo {
    #[doc = "EDO mode disabled"]
    EdoDisable = 0x0,
    #[doc = "EDO mode enabled"]
    EdoEnable = 0x01,
}
impl Edo {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Edo {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Edo {
    #[inline(always)]
    fn from(val: u8) -> Edo {
        Edo::from_bits(val)
    }
}
impl From<Edo> for u8 {
    #[inline(always)]
    fn from(val: Edo) -> u8 {
        Edo::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmddoneen {
    #[doc = "Interrupt is disabled"]
    InterruptDisable = 0x0,
    #[doc = "Interrupt is enabled"]
    InterruptEnable = 0x01,
}
impl Ipcmddoneen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmddoneen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmddoneen {
    #[inline(always)]
    fn from(val: u8) -> Ipcmddoneen {
        Ipcmddoneen::from_bits(val)
    }
}
impl From<Ipcmddoneen> for u8 {
    #[inline(always)]
    fn from(val: Ipcmddoneen) -> u8 {
        Ipcmddoneen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmderren {
    #[doc = "Interrupt is disabled"]
    InterruptDisable = 0x0,
    #[doc = "Interrupt is enabled"]
    InterruptEnable = 0x01,
}
impl Ipcmderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderren {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderren {
        Ipcmderren::from_bits(val)
    }
}
impl From<Ipcmderren> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderren) -> u8 {
        Ipcmderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ito {
    #[doc = "IDLE timeout period is 256*Prescale period."]
    Prescalex256 = 0x0,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito1 = 0x01,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito2 = 0x02,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito3 = 0x03,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito4 = 0x04,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito5 = 0x05,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito6 = 0x06,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito7 = 0x07,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito8 = 0x08,
    #[doc = "IDLE timeout period is ITO*Prescale period."]
    Prescalexito9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Ito {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ito {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ito {
    #[inline(always)]
    fn from(val: u8) -> Ito {
        Ito::from_bits(val)
    }
}
impl From<Ito> for u8 {
    #[inline(always)]
    fn from(val: Ito) -> u8 {
        Ito::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "Module enabled"]
    Enable = 0x0,
    #[doc = "Module disabled"]
    Disable = 0x01,
}
impl Mdis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mdis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mdis {
    #[inline(always)]
    fn from(val: u8) -> Mdis {
        Mdis::from_bits(val)
    }
}
impl From<Mdis> for u8 {
    #[inline(always)]
    fn from(val: Mdis) -> u8 {
        Mdis::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxA8 {
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24030 = 0x0,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24031 = 0x01,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24032 = 0x02,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24033 = 0x03,
    #[doc = "NAND CE#"]
    NandCeb = 0x04,
    #[doc = "NOR CE#"]
    NorCeb = 0x05,
    #[doc = "SRAM CE# 0"]
    SramCeb0 = 0x06,
    #[doc = "DBI CSX"]
    DbiCsx = 0x07,
    #[doc = "SRAM CE# 1"]
    SramCeb1 = 0x08,
    #[doc = "SRAM CE# 2"]
    SramCeb2 = 0x09,
    #[doc = "SRAM CE# 3"]
    SramCeb3 = 0x0a,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24Bf11 = 0x0b,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24Bf12 = 0x0c,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24Bf13 = 0x0d,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24Bf14 = 0x0e,
    #[doc = "SDRAM Address bit 8 (A8) or NOR/SRAM Address bit 24 (A24) in ADMUX 16bit mode"]
    Sdram8Norsram24Bf15 = 0x0f,
}
impl MuxA8 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxA8 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxA8 {
    #[inline(always)]
    fn from(val: u8) -> MuxA8 {
        MuxA8::from_bits(val)
    }
}
impl From<MuxA8> for u8 {
    #[inline(always)]
    fn from(val: MuxA8) -> u8 {
        MuxA8::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxClkx0 {
    #[doc = "Keep low"]
    Keeplow = 0x0,
    #[doc = "NOR clock"]
    NorClk = 0x01,
    #[doc = "SRAM clock"]
    SramClk = 0x02,
    #[doc = "NOR and SRAM clock, suitable for Multi-Chip Product package"]
    NorsramClk = 0x03,
}
impl MuxClkx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxClkx0 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxClkx0 {
    #[inline(always)]
    fn from(val: u8) -> MuxClkx0 {
        MuxClkx0::from_bits(val)
    }
}
impl From<MuxClkx0> for u8 {
    #[inline(always)]
    fn from(val: MuxClkx0) -> u8 {
        MuxClkx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxClkx1 {
    #[doc = "Keep low"]
    Keeplow = 0x0,
    #[doc = "NOR clock"]
    NorClk = 0x01,
    #[doc = "SRAM clock"]
    SramClk = 0x02,
    #[doc = "NOR and SRAM clock, suitable for Multi-Chip Product package"]
    NorSramClk = 0x03,
}
impl MuxClkx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxClkx1 {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxClkx1 {
    #[inline(always)]
    fn from(val: u8) -> MuxClkx1 {
        MuxClkx1::from_bits(val)
    }
}
impl From<MuxClkx1> for u8 {
    #[inline(always)]
    fn from(val: MuxClkx1) -> u8 {
        MuxClkx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxCsx0 {
    #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
    Norsram2403 = 0x0,
    #[doc = "SDRAM CS1"]
    SdramCs1 = 0x01,
    #[doc = "SDRAM CS2"]
    SdramCs2 = 0x02,
    #[doc = "SDRAM CS3"]
    SdramCs3 = 0x03,
    #[doc = "NAND CE#"]
    NandCeb = 0x04,
    #[doc = "NOR CE#"]
    NorCeb = 0x05,
    #[doc = "SRAM CE# 0"]
    SramCeb0 = 0x06,
    #[doc = "DBI CSX"]
    DbiCsx = 0x07,
    #[doc = "SRAM CE# 1"]
    SramCeb1 = 0x08,
    #[doc = "SRAM CE# 2"]
    SramCeb2 = 0x09,
    #[doc = "SRAM CE# 3"]
    SramCeb3 = 0x0a,
    #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
    Norsram24Bf11 = 0x0b,
    #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
    Norsram24Bf12 = 0x0c,
    #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
    Norsram24Bf13 = 0x0d,
    #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
    Norsram24Bf14 = 0x0e,
    #[doc = "NOR/SRAM Address bit 24 (A24) in Non-ADMUX mode"]
    Norsram24Bf15 = 0x0f,
}
impl MuxCsx0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxCsx0 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxCsx0 {
    #[inline(always)]
    fn from(val: u8) -> MuxCsx0 {
        MuxCsx0::from_bits(val)
    }
}
impl From<MuxCsx0> for u8 {
    #[inline(always)]
    fn from(val: MuxCsx0) -> u8 {
        MuxCsx0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxCsx1 {
    #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
    Norsram250 = 0x0,
    #[doc = "SDRAM CS1"]
    SdramCs1 = 0x01,
    #[doc = "SDRAM CS2"]
    SdramCs2 = 0x02,
    #[doc = "SDRAM CS3"]
    SdramCs3 = 0x03,
    #[doc = "NAND CE#"]
    NandCeb = 0x04,
    #[doc = "NOR CE#"]
    NorCeb = 0x05,
    #[doc = "SRAM CE# 0"]
    SramCeb0 = 0x06,
    #[doc = "DBI CSX"]
    DbiCsx = 0x07,
    #[doc = "SRAM CE# 1"]
    SramCeb1 = 0x08,
    #[doc = "SRAM CE# 2"]
    SramCeb2 = 0x09,
    #[doc = "SRAM CE# 3"]
    SramCeb3 = 0x0a,
    #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
    Norsram25Bf11 = 0x0b,
    #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
    Norsram25Bf12 = 0x0c,
    #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
    Norsram25Bf13 = 0x0d,
    #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
    Norsram25Bf14 = 0x0e,
    #[doc = "NOR/SRAM Address bit 25 (A25) in Non-ADMUX mode"]
    Norsram25Bf15 = 0x0f,
}
impl MuxCsx1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxCsx1 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxCsx1 {
    #[inline(always)]
    fn from(val: u8) -> MuxCsx1 {
        MuxCsx1::from_bits(val)
    }
}
impl From<MuxCsx1> for u8 {
    #[inline(always)]
    fn from(val: MuxCsx1) -> u8 {
        MuxCsx1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxCsx2 {
    #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
    Norsram260 = 0x0,
    #[doc = "SDRAM CS1"]
    SdramCs1 = 0x01,
    #[doc = "SDRAM CS2"]
    SdramCs2 = 0x02,
    #[doc = "SDRAM CS3"]
    SdramCs3 = 0x03,
    #[doc = "NAND CE#"]
    NandCeb = 0x04,
    #[doc = "NOR CE#"]
    NorCeb = 0x05,
    #[doc = "SRAM CE# 0"]
    SramCeb0 = 0x06,
    #[doc = "DBI CSX"]
    DbiCsx = 0x07,
    #[doc = "SRAM CE# 1"]
    SramCeb1 = 0x08,
    #[doc = "SRAM CE# 2"]
    SramCeb2 = 0x09,
    #[doc = "SRAM CE# 3"]
    SramCeb3 = 0x0a,
    #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
    Norsram26Bf11 = 0x0b,
    #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
    Norsram26Bf12 = 0x0c,
    #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
    Norsram26Bf13 = 0x0d,
    #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
    Norsram26Bf14 = 0x0e,
    #[doc = "NOR/SRAM Address bit 26 (A26) in Non-ADMUX mode"]
    Norsram26Bf15 = 0x0f,
}
impl MuxCsx2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxCsx2 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxCsx2 {
    #[inline(always)]
    fn from(val: u8) -> MuxCsx2 {
        MuxCsx2::from_bits(val)
    }
}
impl From<MuxCsx2> for u8 {
    #[inline(always)]
    fn from(val: MuxCsx2) -> u8 {
        MuxCsx2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxCsx3 {
    #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
    Norsram270 = 0x0,
    #[doc = "SDRAM CS1"]
    SdramCs1 = 0x01,
    #[doc = "SDRAM CS2"]
    SdramCs2 = 0x02,
    #[doc = "SDRAM CS3"]
    SdramCs3 = 0x03,
    #[doc = "NAND CE#"]
    NandCeb = 0x04,
    #[doc = "NOR CE#"]
    NorCeb = 0x05,
    #[doc = "SRAM CE# 0"]
    SramCeb0 = 0x06,
    #[doc = "DBI CSX"]
    DbiCsx = 0x07,
    #[doc = "SRAM CE# 1"]
    SramCeb1 = 0x08,
    #[doc = "SRAM CE# 2"]
    SramCeb2 = 0x09,
    #[doc = "SRAM CE# 3"]
    SramCeb3 = 0x0a,
    #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
    Norsram27Bf11 = 0x0b,
    #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
    Norsram27Bf12 = 0x0c,
    #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
    Norsram27Bf13 = 0x0d,
    #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
    Norsram27Bf14 = 0x0e,
    #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
    Norsram27Bf15 = 0x0f,
}
impl MuxCsx3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxCsx3 {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxCsx3 {
    #[inline(always)]
    fn from(val: u8) -> MuxCsx3 {
        MuxCsx3::from_bits(val)
    }
}
impl From<MuxCsx3> for u8 {
    #[inline(always)]
    fn from(val: MuxCsx3) -> u8 {
        MuxCsx3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxRdy {
    #[doc = "NAND R/B# input"]
    NandRbb = 0x0,
    #[doc = "SDRAM CS1"]
    SdramCs1 = 0x01,
    #[doc = "SDRAM CS2"]
    SdramCs2 = 0x02,
    #[doc = "SDRAM CS3"]
    SdramCs3 = 0x03,
    #[doc = "NOR/SRAM Address bit 27 (A27) in Non-ADMUX mode"]
    Norsram274 = 0x04,
    #[doc = "NOR CE#"]
    NorCeb = 0x05,
    #[doc = "SRAM CE# 0"]
    SramCeb0 = 0x06,
    #[doc = "DBI CSX"]
    DbiCsx = 0x07,
    #[doc = "SRAM CE# 1"]
    SramCeb1 = 0x08,
    #[doc = "SRAM CE# 2"]
    SramCeb2 = 0x09,
    #[doc = "SRAM CE# 3"]
    SramCeb3 = 0x0a,
    #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
    Norsram27Bf11 = 0x0b,
    #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
    Norsram27Bf12 = 0x0c,
    #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
    Norsram27Bf13 = 0x0d,
    #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
    Norsram27Bf14 = 0x0e,
    #[doc = "NOR/SRAM Address bit 27 in Non-ADMUX mode"]
    Norsram27Bf15 = 0x0f,
}
impl MuxRdy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MuxRdy {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MuxRdy {
    #[inline(always)]
    fn from(val: u8) -> MuxRdy {
        MuxRdy::from_bits(val)
    }
}
impl From<MuxRdy> for u8 {
    #[inline(always)]
    fn from(val: MuxRdy) -> u8 {
        MuxRdy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandcr0Bl {
    #[doc = "1"]
    Burst1 = 0x0,
    #[doc = "2"]
    Burst2 = 0x01,
    #[doc = "4"]
    Burst4 = 0x02,
    #[doc = "8"]
    Burst8 = 0x03,
    #[doc = "16"]
    Burst16 = 0x04,
    #[doc = "32"]
    Burst32 = 0x05,
    #[doc = "64"]
    Burst646 = 0x06,
    #[doc = "64"]
    Burst647 = 0x07,
}
impl Nandcr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandcr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandcr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Nandcr0Bl {
        Nandcr0Bl::from_bits(val)
    }
}
impl From<Nandcr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Nandcr0Bl) -> u8 {
        Nandcr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandcr0Col {
    #[doc = "16"]
    Bit16 = 0x0,
    #[doc = "15"]
    Bit15 = 0x01,
    #[doc = "14"]
    Bit14 = 0x02,
    #[doc = "13"]
    Bit13 = 0x03,
    #[doc = "12"]
    Bit12 = 0x04,
    #[doc = "11"]
    Bit11 = 0x05,
    #[doc = "10"]
    Bit10 = 0x06,
    #[doc = "9"]
    Bit9 = 0x07,
}
impl Nandcr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandcr0Col {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandcr0Col {
    #[inline(always)]
    fn from(val: u8) -> Nandcr0Col {
        Nandcr0Col::from_bits(val)
    }
}
impl From<Nandcr0Col> for u8 {
    #[inline(always)]
    fn from(val: Nandcr0Col) -> u8 {
        Nandcr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandcr0Ps {
    #[doc = "8bit"]
    Ps8bit = 0x0,
    #[doc = "16bit"]
    Ps16bit = 0x01,
}
impl Nandcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Nandcr0Ps {
        Nandcr0Ps::from_bits(val)
    }
}
impl From<Nandcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Nandcr0Ps) -> u8 {
        Nandcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nandcr0Syncen {
    #[doc = "Asynchronous mode is enabled."]
    Async = 0x0,
    #[doc = "Synchronous mode is enabled."]
    Sync = 0x01,
}
impl Nandcr0Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nandcr0Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nandcr0Syncen {
    #[inline(always)]
    fn from(val: u8) -> Nandcr0Syncen {
        Nandcr0Syncen::from_bits(val)
    }
}
impl From<Nandcr0Syncen> for u8 {
    #[inline(always)]
    fn from(val: Nandcr0Syncen) -> u8 {
        Nandcr0Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndbufenden {
    #[doc = "Interrupt is disabled"]
    InterruptDisable = 0x0,
    #[doc = "Interrupt is enabled"]
    InterruptEnable = 0x01,
}
impl Ndbufenden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndbufenden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndbufenden {
    #[inline(always)]
    fn from(val: u8) -> Ndbufenden {
        Ndbufenden::from_bits(val)
    }
}
impl From<Ndbufenden> for u8 {
    #[inline(always)]
    fn from(val: Ndbufenden) -> u8 {
        Ndbufenden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndeccfail {
    #[doc = "NAND ECC data correction pass."]
    CorrectionPass = 0x0,
    #[doc = "NAND ECC data correction fail."]
    CorrectionFail = 0x01,
}
impl Ndeccfail {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndeccfail {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndeccfail {
    #[inline(always)]
    fn from(val: u8) -> Ndeccfail {
        Ndeccfail::from_bits(val)
    }
}
impl From<Ndeccfail> for u8 {
    #[inline(always)]
    fn from(val: Ndeccfail) -> u8 {
        Ndeccfail::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndeccfailen {
    #[doc = "Interrupt is disabled"]
    InterruptDisable = 0x0,
    #[doc = "Interrupt is enabled"]
    InterruptEnable = 0x01,
}
impl Ndeccfailen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndeccfailen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndeccfailen {
    #[inline(always)]
    fn from(val: u8) -> Ndeccfailen {
        Ndeccfailen::from_bits(val)
    }
}
impl From<Ndeccfailen> for u8 {
    #[inline(always)]
    fn from(val: Ndeccfailen) -> u8 {
        Ndeccfailen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndnopenden {
    #[doc = "Interrupt is disabled"]
    InterruptDisable = 0x0,
    #[doc = "Interrupt is enabled"]
    InterruptEnable = 0x01,
}
impl Ndnopenden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndnopenden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndnopenden {
    #[inline(always)]
    fn from(val: u8) -> Ndnopenden {
        Ndnopenden::from_bits(val)
    }
}
impl From<Ndnopenden> for u8 {
    #[inline(always)]
    fn from(val: Ndnopenden) -> u8 {
        Ndnopenden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ndpageenden {
    #[doc = "Interrupt is disabled"]
    InterruptDisable = 0x0,
    #[doc = "Interrupt is enabled"]
    InterruptEnable = 0x01,
}
impl Ndpageenden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ndpageenden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ndpageenden {
    #[inline(always)]
    fn from(val: u8) -> Ndpageenden {
        Ndpageenden::from_bits(val)
    }
}
impl From<Ndpageenden> for u8 {
    #[inline(always)]
    fn from(val: Ndpageenden) -> u8 {
        Ndpageenden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Advh {
    #[doc = "ADV# is high during address hold state."]
    High = 0x0,
    #[doc = "ADV# is low during address hold state."]
    Low = 0x01,
}
impl Norcr0Advh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Advh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Advh {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Advh {
        Norcr0Advh::from_bits(val)
    }
}
impl From<Norcr0Advh> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Advh) -> u8 {
        Norcr0Advh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Advp {
    #[doc = "ADV# is active low."]
    ActiveLow = 0x0,
    #[doc = "ADV# is active high."]
    ActiveHigh = 0x01,
}
impl Norcr0Advp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Advp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Advp {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Advp {
        Norcr0Advp::from_bits(val)
    }
}
impl From<Norcr0Advp> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Advp) -> u8 {
        Norcr0Advp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Am {
    #[doc = "Address/Data MUX mode (ADMUX)"]
    Admux = 0x0,
    #[doc = "Advanced Address/Data MUX mode (AADM)"]
    Aadm = 0x01,
    #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
    NonAdmux2 = 0x02,
    #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
    NonAdmux3 = 0x03,
}
impl Norcr0Am {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Am {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Am {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Am {
        Norcr0Am::from_bits(val)
    }
}
impl From<Norcr0Am> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Am) -> u8 {
        Norcr0Am::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Bl {
    #[doc = "1"]
    Burst1 = 0x0,
    #[doc = "2"]
    Burst2 = 0x01,
    #[doc = "4"]
    Burst4 = 0x02,
    #[doc = "8"]
    Burst8 = 0x03,
    #[doc = "16"]
    Burst16 = 0x04,
    #[doc = "32"]
    Burst32 = 0x05,
    #[doc = "64"]
    Burst646 = 0x06,
    #[doc = "64"]
    Burst647 = 0x07,
}
impl Norcr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Bl {
        Norcr0Bl::from_bits(val)
    }
}
impl From<Norcr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Bl) -> u8 {
        Norcr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Col {
    #[doc = "12 Bits"]
    Bitwidth120 = 0x0,
    #[doc = "11 Bits"]
    Bitwidth11 = 0x01,
    #[doc = "10 Bits"]
    Bitwidth10 = 0x02,
    #[doc = "9 Bits"]
    Bitwidth9 = 0x03,
    #[doc = "8 Bits"]
    Bitwidth8 = 0x04,
    #[doc = "7 Bits"]
    Bitwidth7 = 0x05,
    #[doc = "6 Bits"]
    Bitwidth6 = 0x06,
    #[doc = "5 Bits"]
    Bitwidth5 = 0x07,
    #[doc = "4 Bits"]
    Bitwidth4 = 0x08,
    #[doc = "3 Bits"]
    Bitwidth3 = 0x09,
    #[doc = "2 Bits"]
    Bitwidth2 = 0x0a,
    #[doc = "12 Bits"]
    Bitwidth12B = 0x0b,
    #[doc = "12 Bits"]
    Bitwidth12C = 0x0c,
    #[doc = "12 Bits"]
    Bitwidth12D = 0x0d,
    #[doc = "12 Bits"]
    Bitwidth12E = 0x0e,
    #[doc = "12 Bits"]
    Bitwidth12F = 0x0f,
}
impl Norcr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Col {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Col {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Col {
        Norcr0Col::from_bits(val)
    }
}
impl From<Norcr0Col> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Col) -> u8 {
        Norcr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Ps {
    #[doc = "8bit"]
    Ps8bit = 0x0,
    #[doc = "16bit"]
    Ps16bit = 0x01,
}
impl Norcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Ps {
        Norcr0Ps::from_bits(val)
    }
}
impl From<Norcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Ps) -> u8 {
        Norcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Norcr0Syncen {
    #[doc = "Asynchronous mode is enabled."]
    Async = 0x0,
    #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
    Sync = 0x01,
}
impl Norcr0Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Norcr0Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Norcr0Syncen {
    #[inline(always)]
    fn from(val: u8) -> Norcr0Syncen {
        Norcr0Syncen::from_bits(val)
    }
}
impl From<Norcr0Syncen> for u8 {
    #[inline(always)]
    fn from(val: Norcr0Syncen) -> u8 {
        Norcr0Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PrefetchEn {
    #[doc = "SDRAM prefetch function is disabled."]
    PrefetchDisable = 0x0,
    #[doc = "SDRAM prefetch function is enabled."]
    PrefetchEnable = 0x01,
}
impl PrefetchEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PrefetchEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PrefetchEn {
    #[inline(always)]
    fn from(val: u8) -> PrefetchEn {
        PrefetchEn::from_bits(val)
    }
}
impl From<PrefetchEn> for u8 {
    #[inline(always)]
    fn from(val: PrefetchEn) -> u8 {
        PrefetchEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prescale {
    #[doc = "(256*16+1) clock cycles"]
    Prescale256x16plus1 = 0x0,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus11 = 0x01,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus12 = 0x02,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus13 = 0x03,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus14 = 0x04,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus15 = 0x05,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus16 = 0x06,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus17 = 0x07,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus18 = 0x08,
    #[doc = "(PRESCALE*16+1) clock cycles"]
    Prescale16plus19 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Prescale {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prescale {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prescale {
    #[inline(always)]
    fn from(val: u8) -> Prescale {
        Prescale::from_bits(val)
    }
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(val: Prescale) -> u8 {
        Prescale::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rebl {
    #[doc = "1"]
    RefreshBurst1 = 0x0,
    #[doc = "2"]
    RefreshBurst2 = 0x01,
    #[doc = "3"]
    RefreshBurst3 = 0x02,
    #[doc = "4"]
    RefreshBurst4 = 0x03,
    #[doc = "5"]
    RefreshBurst5 = 0x04,
    #[doc = "6"]
    RefreshBurst6 = 0x05,
    #[doc = "7"]
    RefreshBurst7 = 0x06,
    #[doc = "8"]
    RefreshBurst8 = 0x07,
}
impl Rebl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rebl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rebl {
    #[inline(always)]
    fn from(val: u8) -> Rebl {
        Rebl::from_bits(val)
    }
}
impl From<Rebl> for u8 {
    #[inline(always)]
    fn from(val: Rebl) -> u8 {
        Rebl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rt {
    #[doc = "(256+1)*(Prescaler period)"]
    Rt256plus1xprescale = 0x0,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale1 = 0x01,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale2 = 0x02,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale3 = 0x03,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale4 = 0x04,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale5 = 0x05,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale6 = 0x06,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale7 = 0x07,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale8 = 0x08,
    #[doc = "(RT+1)*(Prescaler period)"]
    RtRtplus1xprescale9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Rt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rt {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rt {
    #[inline(always)]
    fn from(val: u8) -> Rt {
        Rt::from_bits(val)
    }
}
impl From<Rt> for u8 {
    #[inline(always)]
    fn from(val: Rt) -> u8 {
        Rt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdramcr0Bl {
    #[doc = "1"]
    Burst1 = 0x0,
    #[doc = "2"]
    Burst2 = 0x01,
    #[doc = "4"]
    Burst4 = 0x02,
    #[doc = "8"]
    Burst83 = 0x03,
    #[doc = "8"]
    Burst84 = 0x04,
    #[doc = "8"]
    Burst85 = 0x05,
    #[doc = "8"]
    Burst86 = 0x06,
    #[doc = "8"]
    Burst87 = 0x07,
}
impl Sdramcr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdramcr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdramcr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Sdramcr0Bl {
        Sdramcr0Bl::from_bits(val)
    }
}
impl From<Sdramcr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Sdramcr0Bl) -> u8 {
        Sdramcr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdramcr0Col {
    #[doc = "12"]
    Bit12 = 0x0,
    #[doc = "11"]
    Bit11 = 0x01,
    #[doc = "10"]
    Bit10 = 0x02,
    #[doc = "9"]
    Bit9 = 0x03,
}
impl Sdramcr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdramcr0Col {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdramcr0Col {
    #[inline(always)]
    fn from(val: u8) -> Sdramcr0Col {
        Sdramcr0Col::from_bits(val)
    }
}
impl From<Sdramcr0Col> for u8 {
    #[inline(always)]
    fn from(val: Sdramcr0Col) -> u8 {
        Sdramcr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sdramcr0Ps {
    #[doc = "8bit"]
    Ps8bit = 0x0,
    #[doc = "16bit"]
    Ps16bit = 0x01,
    #[doc = "32bit"]
    Ps32bit = 0x02,
    _RESERVED_3 = 0x03,
}
impl Sdramcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdramcr0Ps {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdramcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Sdramcr0Ps {
        Sdramcr0Ps::from_bits(val)
    }
}
impl From<Sdramcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Sdramcr0Ps) -> u8 {
        Sdramcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SectorNum {
    #[doc = "There is 1 sector in buffer"]
    Sector1 = 0x0,
    #[doc = "There are 2 sectors in buffer"]
    Sector2 = 0x01,
    #[doc = "There are 4 sectors in buffer"]
    Sector4 = 0x02,
    _RESERVED_3 = 0x03,
}
impl SectorNum {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SectorNum {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SectorNum {
    #[inline(always)]
    fn from(val: u8) -> SectorNum {
        SectorNum::from_bits(val)
    }
}
impl From<SectorNum> for u8 {
    #[inline(always)]
    fn from(val: SectorNum) -> u8 {
        SectorNum::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Advh {
    #[doc = "ADV# is high during address hold state."]
    HighHold = 0x0,
    #[doc = "ADV# is low during address hold state."]
    LowHold = 0x01,
}
impl Sramcr0Advh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Advh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Advh {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Advh {
        Sramcr0Advh::from_bits(val)
    }
}
impl From<Sramcr0Advh> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Advh) -> u8 {
        Sramcr0Advh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Advp {
    #[doc = "ADV# is active low."]
    ActiveLow = 0x0,
    #[doc = "ADV# is active high."]
    ActiveHigh = 0x01,
}
impl Sramcr0Advp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Advp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Advp {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Advp {
        Sramcr0Advp::from_bits(val)
    }
}
impl From<Sramcr0Advp> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Advp) -> u8 {
        Sramcr0Advp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Am {
    #[doc = "Address/Data MUX mode (ADMUX)"]
    Admux = 0x0,
    #[doc = "Advanced Address/Data MUX mode (AADM)"]
    Aadm = 0x01,
    #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
    NonAdmux2 = 0x02,
    #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
    NonAdmux3 = 0x03,
}
impl Sramcr0Am {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Am {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Am {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Am {
        Sramcr0Am::from_bits(val)
    }
}
impl From<Sramcr0Am> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Am) -> u8 {
        Sramcr0Am::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Bl {
    #[doc = "1"]
    Burst1 = 0x0,
    #[doc = "2"]
    Burst2 = 0x01,
    #[doc = "4"]
    Burst4 = 0x02,
    #[doc = "8"]
    Burst8 = 0x03,
    #[doc = "16"]
    Burst16 = 0x04,
    #[doc = "32"]
    Burst32 = 0x05,
    #[doc = "64"]
    Burst646 = 0x06,
    #[doc = "64"]
    Burst647 = 0x07,
}
impl Sramcr0Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Bl {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Bl {
        Sramcr0Bl::from_bits(val)
    }
}
impl From<Sramcr0Bl> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Bl) -> u8 {
        Sramcr0Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Col {
    #[doc = "12 Bits"]
    Bitwidth120 = 0x0,
    #[doc = "11 Bits"]
    Bitwidth11 = 0x01,
    #[doc = "10 Bits"]
    Bitwidth10 = 0x02,
    #[doc = "9 Bits"]
    Bitwidth9 = 0x03,
    #[doc = "8 Bits"]
    Bitwidth8 = 0x04,
    #[doc = "7 Bits"]
    Bitwidth7 = 0x05,
    #[doc = "6 Bits"]
    Bitwidth6 = 0x06,
    #[doc = "5 Bits"]
    Bitwidth5 = 0x07,
    #[doc = "4 Bits"]
    Bitwidth4 = 0x08,
    #[doc = "3 Bits"]
    Bitwidth3 = 0x09,
    #[doc = "2 Bits"]
    Bitwidth2 = 0x0a,
    #[doc = "12 Bits"]
    Bitwidth12B = 0x0b,
    #[doc = "12 Bits"]
    Bitwidth12C = 0x0c,
    #[doc = "12 Bits"]
    Bitwidth12D = 0x0d,
    #[doc = "12 Bits"]
    Bitwidth12E = 0x0e,
    #[doc = "12 Bits"]
    Bitwidth12F = 0x0f,
}
impl Sramcr0Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Col {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Col {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Col {
        Sramcr0Col::from_bits(val)
    }
}
impl From<Sramcr0Col> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Col) -> u8 {
        Sramcr0Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Ps {
    #[doc = "8bit"]
    Ps8bit = 0x0,
    #[doc = "16bit"]
    Ps16bit = 0x01,
}
impl Sramcr0Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Ps {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Ps {
        Sramcr0Ps::from_bits(val)
    }
}
impl From<Sramcr0Ps> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Ps) -> u8 {
        Sramcr0Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Syncen {
    #[doc = "Asynchronous mode is enabled."]
    Async = 0x0,
    #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
    Sync = 0x01,
}
impl Sramcr0Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Syncen {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Syncen {
        Sramcr0Syncen::from_bits(val)
    }
}
impl From<Sramcr0Syncen> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Syncen) -> u8 {
        Sramcr0Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr0Waitsp {
    #[doc = "Wait pin is directly used by the SEMC."]
    WaitDirect = 0x0,
    #[doc = "Wait pin is sampled by internal clock before it is used."]
    WaitSampled = 0x01,
}
impl Sramcr0Waitsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr0Waitsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr0Waitsp {
    #[inline(always)]
    fn from(val: u8) -> Sramcr0Waitsp {
        Sramcr0Waitsp::from_bits(val)
    }
}
impl From<Sramcr0Waitsp> for u8 {
    #[inline(always)]
    fn from(val: Sramcr0Waitsp) -> u8 {
        Sramcr0Waitsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr4Advh {
    #[doc = "ADV# is high during address hold state."]
    HighHold = 0x0,
    #[doc = "ADV# is low during address hold state."]
    LowHold = 0x01,
}
impl Sramcr4Advh {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr4Advh {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr4Advh {
    #[inline(always)]
    fn from(val: u8) -> Sramcr4Advh {
        Sramcr4Advh::from_bits(val)
    }
}
impl From<Sramcr4Advh> for u8 {
    #[inline(always)]
    fn from(val: Sramcr4Advh) -> u8 {
        Sramcr4Advh::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr4Advp {
    #[doc = "ADV# is active low."]
    ActiveLow = 0x0,
    #[doc = "ADV# is active high."]
    ActiveHigh = 0x01,
}
impl Sramcr4Advp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr4Advp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr4Advp {
    #[inline(always)]
    fn from(val: u8) -> Sramcr4Advp {
        Sramcr4Advp::from_bits(val)
    }
}
impl From<Sramcr4Advp> for u8 {
    #[inline(always)]
    fn from(val: Sramcr4Advp) -> u8 {
        Sramcr4Advp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr4Am {
    #[doc = "Address/Data MUX mode (ADMUX)"]
    Admux = 0x0,
    #[doc = "Advanced Address/Data MUX mode (AADM)"]
    Aadm = 0x01,
    #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
    NonAdmux2 = 0x02,
    #[doc = "Address/Data non-MUX mode (Non-ADMUX)"]
    NonAdmux3 = 0x03,
}
impl Sramcr4Am {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr4Am {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr4Am {
    #[inline(always)]
    fn from(val: u8) -> Sramcr4Am {
        Sramcr4Am::from_bits(val)
    }
}
impl From<Sramcr4Am> for u8 {
    #[inline(always)]
    fn from(val: Sramcr4Am) -> u8 {
        Sramcr4Am::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr4Bl {
    #[doc = "1"]
    Burst1 = 0x0,
    #[doc = "2"]
    Burst2 = 0x01,
    #[doc = "4"]
    Burst4 = 0x02,
    #[doc = "8"]
    Burst8 = 0x03,
    #[doc = "16"]
    Burst16 = 0x04,
    #[doc = "32"]
    Burst32 = 0x05,
    #[doc = "64"]
    Burst646 = 0x06,
    #[doc = "64"]
    Burst647 = 0x07,
}
impl Sramcr4Bl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr4Bl {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr4Bl {
    #[inline(always)]
    fn from(val: u8) -> Sramcr4Bl {
        Sramcr4Bl::from_bits(val)
    }
}
impl From<Sramcr4Bl> for u8 {
    #[inline(always)]
    fn from(val: Sramcr4Bl) -> u8 {
        Sramcr4Bl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr4Col {
    #[doc = "12 Bits"]
    Bitwidth120 = 0x0,
    #[doc = "11 Bits"]
    Bitwidth11 = 0x01,
    #[doc = "10 Bits"]
    Bitwidth10 = 0x02,
    #[doc = "9 Bits"]
    Bitwidth9 = 0x03,
    #[doc = "8 Bits"]
    Bitwidth8 = 0x04,
    #[doc = "7 Bits"]
    Bitwidth7 = 0x05,
    #[doc = "6 Bits"]
    Bitwidth6 = 0x06,
    #[doc = "5 Bits"]
    Bitwidth5 = 0x07,
    #[doc = "4 Bits"]
    Bitwidth4 = 0x08,
    #[doc = "3 Bits"]
    Bitwidth3 = 0x09,
    #[doc = "2 Bits"]
    Bitwidth2 = 0x0a,
    #[doc = "12 Bits"]
    Bitwidth12B = 0x0b,
    #[doc = "12 Bits"]
    Bitwidth12C = 0x0c,
    #[doc = "12 Bits"]
    Bitwidth12D = 0x0d,
    #[doc = "12 Bits"]
    Bitwidth12E = 0x0e,
    #[doc = "12 Bits"]
    Bitwidth12F = 0x0f,
}
impl Sramcr4Col {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr4Col {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr4Col {
    #[inline(always)]
    fn from(val: u8) -> Sramcr4Col {
        Sramcr4Col::from_bits(val)
    }
}
impl From<Sramcr4Col> for u8 {
    #[inline(always)]
    fn from(val: Sramcr4Col) -> u8 {
        Sramcr4Col::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr4Ps {
    #[doc = "8bit"]
    Ps8bit = 0x0,
    #[doc = "16bit"]
    Ps16bit = 0x01,
}
impl Sramcr4Ps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr4Ps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr4Ps {
    #[inline(always)]
    fn from(val: u8) -> Sramcr4Ps {
        Sramcr4Ps::from_bits(val)
    }
}
impl From<Sramcr4Ps> for u8 {
    #[inline(always)]
    fn from(val: Sramcr4Ps) -> u8 {
        Sramcr4Ps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr4Syncen {
    #[doc = "Asynchronous mode is enabled."]
    Async = 0x0,
    #[doc = "Synchronous mode is enabled. Only fixed latency mode is supported."]
    Sync = 0x01,
}
impl Sramcr4Syncen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr4Syncen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr4Syncen {
    #[inline(always)]
    fn from(val: u8) -> Sramcr4Syncen {
        Sramcr4Syncen::from_bits(val)
    }
}
impl From<Sramcr4Syncen> for u8 {
    #[inline(always)]
    fn from(val: Sramcr4Syncen) -> u8 {
        Sramcr4Syncen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sramcr4Waitsp {
    #[doc = "Wait pin is directly used by the SEMC."]
    WaitDirect = 0x0,
    #[doc = "Wait pin is sampled by internal clock before it is used."]
    WaitSampled = 0x01,
}
impl Sramcr4Waitsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sramcr4Waitsp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sramcr4Waitsp {
    #[inline(always)]
    fn from(val: u8) -> Sramcr4Waitsp {
        Sramcr4Waitsp::from_bits(val)
    }
}
impl From<Sramcr4Waitsp> for u8 {
    #[inline(always)]
    fn from(val: Sramcr4Waitsp) -> u8 {
        Sramcr4Waitsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ut {
    #[doc = "256*(Prescaler period)"]
    Prescalex256 = 0x0,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut1 = 0x01,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut2 = 0x02,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut3 = 0x03,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut4 = 0x04,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut5 = 0x05,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut6 = 0x06,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut7 = 0x07,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut8 = 0x08,
    #[doc = "UT*(Prescaler period)"]
    Prescalexut9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    _RESERVED_21 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Ut {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ut {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ut {
    #[inline(always)]
    fn from(val: u8) -> Ut {
        Ut::from_bits(val)
    }
}
impl From<Ut> for u8 {
    #[inline(always)]
    fn from(val: Ut) -> u8 {
        Ut::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wpol0 {
    #[doc = "WAIT/RDY polarity is not changed."]
    Unchanged = 0x0,
    #[doc = "WAIT/RDY polarity is inverted."]
    Inverted = 0x01,
}
impl Wpol0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wpol0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wpol0 {
    #[inline(always)]
    fn from(val: u8) -> Wpol0 {
        Wpol0::from_bits(val)
    }
}
impl From<Wpol0> for u8 {
    #[inline(always)]
    fn from(val: Wpol0) -> u8 {
        Wpol0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wpol1 {
    #[doc = "R/B# polarity is not changed."]
    Unchanged = 0x0,
    #[doc = "R/B# polarity is inverted."]
    Inverted = 0x01,
}
impl Wpol1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wpol1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wpol1 {
    #[inline(always)]
    fn from(val: u8) -> Wpol1 {
        Wpol1::from_bits(val)
    }
}
impl From<Wpol1> for u8 {
    #[inline(always)]
    fn from(val: Wpol1) -> u8 {
        Wpol1::to_bits(val)
    }
}
