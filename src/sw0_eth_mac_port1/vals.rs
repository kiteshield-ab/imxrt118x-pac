#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AddrErr {
    #[doc = "Normal"]
    Zero = 0x0,
    #[doc = "Error. An access control violation has occurred. The request address used does not match the MDIO PHY's address (clause 22) or MDIO port address (clause 45) assigned."]
    One = 0x01,
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
    Zero = 0x0,
    #[doc = "An MDIO transaction is occurring."]
    One = 0x01,
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
    Zero = 0x0,
    #[doc = "An MDIO transaction is occurring."]
    One = 0x01,
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
    Zero = 0x0,
    #[doc = "Enabled"]
    One = 0x01,
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
    Zero = 0x0,
    #[doc = "An MDIO command completion occurred."]
    One = 0x01,
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
    #[doc = "Normal operation, MDIO hold time is as specified in PEMDIOCR\\[MDIO_HOLD\\]."]
    Zero = 0x0,
    #[doc = "Extended operation"]
    One = 0x01,
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
    Zero = 0x0,
    #[doc = "Clause 45 transactions are used."]
    One = 0x01,
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
    Netc1 = 0x0,
    #[doc = "3 NETC cycles"]
    Netc3 = 0x01,
    #[doc = "5 NETC cycles (default - recommended value)"]
    Netc5 = 0x02,
    #[doc = "7 NETC cycles"]
    Netc7 = 0x03,
    #[doc = "9 NETC cycles"]
    Netc9 = 0x04,
    #[doc = "11 NETC cycles"]
    Netc11 = 0x05,
    #[doc = "13 NETC cycles"]
    Netc13 = 0x06,
    #[doc = "15 NETC cycles"]
    Netc15 = 0x07,
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
    #[doc = "No error on last MDIO transaction (read or write)."]
    Zero = 0x0,
    #[doc = "An error was detected on the last MDIO transaction (read or write). Errors on internal MDIO accesses can be triggered by an access to an invalid device, or by a write to a shared on-die PHY device that has not been locked."]
    One = 0x01,
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
    #[doc = "normal operation - positive edge"]
    Zero = 0x0,
    #[doc = "MDIO is driven by master on MDC negative edge (default for external MDIOs)"]
    One = 0x01,
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
pub enum Pm0IfModeHd {
    #[doc = "full duplex"]
    Fd = 0x0,
    #[doc = "half duplex"]
    Hd = 0x01,
}
impl Pm0IfModeHd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm0IfModeHd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm0IfModeHd {
    #[inline(always)]
    fn from(val: u8) -> Pm0IfModeHd {
        Pm0IfModeHd::from_bits(val)
    }
}
impl From<Pm0IfModeHd> for u8 {
    #[inline(always)]
    fn from(val: Pm0IfModeHd) -> u8 {
        Pm0IfModeHd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm0IfModeM10 {
    #[doc = "100 Mbps"]
    M100 = 0x0,
    #[doc = "10 Mbps"]
    M10 = 0x01,
}
impl Pm0IfModeM10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm0IfModeM10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm0IfModeM10 {
    #[inline(always)]
    fn from(val: u8) -> Pm0IfModeM10 {
        Pm0IfModeM10::from_bits(val)
    }
}
impl From<Pm0IfModeM10> for u8 {
    #[inline(always)]
    fn from(val: Pm0IfModeM10) -> u8 {
        Pm0IfModeM10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm0IfModeRevmii {
    #[doc = "Reverse mode disabled - port is in MAC mode"]
    Mac = 0x0,
    #[doc = "Reverse mode enabled - port is in PHY mode"]
    Phy = 0x01,
}
impl Pm0IfModeRevmii {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm0IfModeRevmii {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm0IfModeRevmii {
    #[inline(always)]
    fn from(val: u8) -> Pm0IfModeRevmii {
        Pm0IfModeRevmii::from_bits(val)
    }
}
impl From<Pm0IfModeRevmii> for u8 {
    #[inline(always)]
    fn from(val: Pm0IfModeRevmii) -> u8 {
        Pm0IfModeRevmii::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm0IfModeSsp {
    #[doc = "100 Mbps"]
    M100 = 0x0,
    #[doc = "10 Mbps"]
    M10 = 0x01,
    #[doc = "1 Gbps"]
    G1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pm0IfModeSsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm0IfModeSsp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm0IfModeSsp {
    #[inline(always)]
    fn from(val: u8) -> Pm0IfModeSsp {
        Pm0IfModeSsp::from_bits(val)
    }
}
impl From<Pm0IfModeSsp> for u8 {
    #[inline(always)]
    fn from(val: Pm0IfModeSsp) -> u8 {
        Pm0IfModeSsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm1IfModeHd {
    #[doc = "full duplex"]
    Fd = 0x0,
    #[doc = "half duplex"]
    Hd = 0x01,
}
impl Pm1IfModeHd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm1IfModeHd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm1IfModeHd {
    #[inline(always)]
    fn from(val: u8) -> Pm1IfModeHd {
        Pm1IfModeHd::from_bits(val)
    }
}
impl From<Pm1IfModeHd> for u8 {
    #[inline(always)]
    fn from(val: Pm1IfModeHd) -> u8 {
        Pm1IfModeHd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm1IfModeM10 {
    #[doc = "100 Mbps"]
    M100 = 0x0,
    #[doc = "10 Mbps"]
    M10 = 0x01,
}
impl Pm1IfModeM10 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm1IfModeM10 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm1IfModeM10 {
    #[inline(always)]
    fn from(val: u8) -> Pm1IfModeM10 {
        Pm1IfModeM10::from_bits(val)
    }
}
impl From<Pm1IfModeM10> for u8 {
    #[inline(always)]
    fn from(val: Pm1IfModeM10) -> u8 {
        Pm1IfModeM10::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm1IfModeRevmii {
    #[doc = "Reverse mode disabled - port is in MAC mode"]
    Mac = 0x0,
    #[doc = "Reverse mode enabled - port is in PHY mode"]
    Phy = 0x01,
}
impl Pm1IfModeRevmii {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm1IfModeRevmii {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm1IfModeRevmii {
    #[inline(always)]
    fn from(val: u8) -> Pm1IfModeRevmii {
        Pm1IfModeRevmii::from_bits(val)
    }
}
impl From<Pm1IfModeRevmii> for u8 {
    #[inline(always)]
    fn from(val: Pm1IfModeRevmii) -> u8 {
        Pm1IfModeRevmii::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pm1IfModeSsp {
    #[doc = "100 Mbps"]
    M100 = 0x0,
    #[doc = "10 Mbps"]
    M10 = 0x01,
    #[doc = "1 Gbps"]
    G1 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Pm1IfModeSsp {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pm1IfModeSsp {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pm1IfModeSsp {
    #[inline(always)]
    fn from(val: u8) -> Pm1IfModeSsp {
        Pm1IfModeSsp::from_bits(val)
    }
}
impl From<Pm1IfModeSsp> for u8 {
    #[inline(always)]
    fn from(val: Pm1IfModeSsp) -> u8 {
        Pm1IfModeSsp::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PreDis {
    #[doc = "Generation of MDIO preamble is enabled (default operation)."]
    Enable = 0x0,
    #[doc = "Generation of MDIO preamble is disabled."]
    Disable = 0x01,
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
