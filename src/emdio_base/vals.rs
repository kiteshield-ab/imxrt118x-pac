#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr {
    #[doc = "Normal"]
    NORMAL = 0x0,
    #[doc = "Error. An access control violation has occurred. The request address used does not match the MDIO PHY's address (clause 22) or MDIO port address (clause 45) assigned."]
    ERROR = 0x01,
}
impl AddrErr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AddrErr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AddrErr {
    #[inline(always)]
    fn from(val: u8) -> AddrErr {
        AddrErr::from_bits(val)
    }
}
impl From<AddrErr> for u8 {
    #[inline(always)]
    fn from(val: AddrErr) -> u8 {
        AddrErr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bsy1 {
    #[doc = "An MDIO transaction is not occurring; software may access other MDIO registers."]
    ZERO = 0x0,
    #[doc = "An MDIO transaction is occurring."]
    ONE = 0x01,
}
impl Bsy1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bsy1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bsy1 {
    #[inline(always)]
    fn from(val: u8) -> Bsy1 {
        Bsy1::from_bits(val)
    }
}
impl From<Bsy1> for u8 {
    #[inline(always)]
    fn from(val: Bsy1) -> u8 {
        Bsy1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bsy2 {
    #[doc = "An MDIO transaction is not occurring; software may access other MDIO registers."]
    ZERO = 0x0,
    #[doc = "An MDIO transaction is occurring."]
    ONE = 0x01,
}
impl Bsy2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bsy2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bsy2 {
    #[inline(always)]
    fn from(val: u8) -> Bsy2 {
        Bsy2::from_bits(val)
    }
}
impl From<Bsy2> for u8 {
    #[inline(always)]
    fn from(val: Bsy2) -> u8 {
        Bsy2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cim {
    #[doc = "Masked"]
    MASKED = 0x0,
    #[doc = "Enabled"]
    ENABLED = 0x01,
}
impl Cim {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cim {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cim {
    #[inline(always)]
    fn from(val: u8) -> Cim {
        Cim::from_bits(val)
    }
}
impl From<Cim> for u8 {
    #[inline(always)]
    fn from(val: Cim) -> u8 {
        Cim::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cmp {
    #[doc = "An MDIO command completion did not occur."]
    ZERO = 0x0,
    #[doc = "An MDIO command completion occurred."]
    ONE = 0x01,
}
impl Cmp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cmp {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cmp {
    #[inline(always)]
    fn from(val: u8) -> Cmp {
        Cmp::from_bits(val)
    }
}
impl From<Cmp> for u8 {
    #[inline(always)]
    fn from(val: Cmp) -> u8 {
        Cmp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ehold {
    #[doc = "Normal operation. MDIO hold time is specified in ."]
    ZERO = 0x0,
    #[doc = "Extended Operation"]
    ONE = 0x01,
}
impl Ehold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ehold {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ehold {
    #[inline(always)]
    fn from(val: u8) -> Ehold {
        Ehold::from_bits(val)
    }
}
impl From<Ehold> for u8 {
    #[inline(always)]
    fn from(val: Ehold) -> u8 {
        Ehold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Enc45 {
    #[doc = "Clause 22 transactions are used."]
    ZERO = 0x0,
    #[doc = "Clause 45 transactions are used."]
    ONE = 0x01,
}
impl Enc45 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Enc45 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Enc45 {
    #[inline(always)]
    fn from(val: u8) -> Enc45 {
        Enc45::from_bits(val)
    }
}
impl From<Enc45> for u8 {
    #[inline(always)]
    fn from(val: Enc45) -> u8 {
        Enc45::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdioHold {
    #[doc = "1 NETC cycle"]
    NETC1 = 0x0,
    #[doc = "3 NETC cycles"]
    NETC3 = 0x01,
    #[doc = "5 NETC cycles (default - recommended value)"]
    NETC5 = 0x02,
    #[doc = "7 NETC cycles"]
    NETC7 = 0x03,
    #[doc = "9 NETC cycles"]
    NETC9 = 0x04,
    #[doc = "11 NETC cycles"]
    NETC11 = 0x05,
    #[doc = "13 NETC cycles"]
    NETC13 = 0x06,
    #[doc = "15 NETC cycles"]
    NETC15 = 0x07,
}
impl MdioHold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdioHold {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdioHold {
    #[inline(always)]
    fn from(val: u8) -> MdioHold {
        MdioHold::from_bits(val)
    }
}
impl From<MdioHold> for u8 {
    #[inline(always)]
    fn from(val: MdioHold) -> u8 {
        MdioHold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MdioRdEr {
    #[doc = "No error"]
    ZERO = 0x0,
    #[doc = "The last read transaction received no response from a PHY; any data read should be considered invalid (for example, the PHY address does not match any PHY available on the MDIO bus)."]
    ONE = 0x01,
}
impl MdioRdEr {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MdioRdEr {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MdioRdEr {
    #[inline(always)]
    fn from(val: u8) -> MdioRdEr {
        MdioRdEr::from_bits(val)
    }
}
impl From<MdioRdEr> for u8 {
    #[inline(always)]
    fn from(val: MdioRdEr) -> u8 {
        MdioRdEr::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Neg {
    #[doc = "Normal operation - positive edge"]
    NORMAL = 0x0,
    #[doc = "MDIO is driven by master on MDC negative edge (default for external MDIOs)"]
    NEGATIVE = 0x01,
}
impl Neg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Neg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Neg {
    #[inline(always)]
    fn from(val: u8) -> Neg {
        Neg::from_bits(val)
    }
}
impl From<Neg> for u8 {
    #[inline(always)]
    fn from(val: Neg) -> u8 {
        Neg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PreDis {
    #[doc = "Generation of MDIO preamble is enabled (default operation)."]
    ZERO = 0x0,
    #[doc = "Generation of MDIO preamble is disabled"]
    ONE = 0x01,
}
impl PreDis {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PreDis {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PreDis {
    #[inline(always)]
    fn from(val: u8) -> PreDis {
        PreDis::from_bits(val)
    }
}
impl From<PreDis> for u8 {
    #[inline(always)]
    fn from(val: PreDis) -> u8 {
        PreDis::to_bits(val)
    }
}
