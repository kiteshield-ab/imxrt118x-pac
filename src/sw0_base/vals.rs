#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Busy {
    #[doc = "Idle"]
    IDLE = 0x0,
    #[doc = "Busy"]
    BUSY = 0x01,
}
impl Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busy {
    #[inline(always)]
    fn from(val: u8) -> Busy {
        Busy::from_bits(val)
    }
}
impl From<Busy> for u8 {
    #[inline(always)]
    fn from(val: Busy) -> u8 {
        Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cbdc {
    #[doc = "No BD with CI=1 completed"]
    NO_BD = 0x0,
    #[doc = "Processed BD with CI=1"]
    BD_CI_1 = 0x01,
}
impl Cbdc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cbdc {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cbdc {
    #[inline(always)]
    fn from(val: u8) -> Cbdc {
        Cbdc::from_bits(val)
    }
}
impl From<Cbdc> for u8 {
    #[inline(always)]
    fn from(val: Cbdc) -> u8 {
        Cbdc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EtPort0 {
    #[doc = "Port has no entry in the Egress Treatment table"]
    NO_ENTRY = 0x0,
    #[doc = "Port has an entry in the Egress Treatment table"]
    PORT_ENTRY = 0x01,
}
impl EtPort0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EtPort0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EtPort0 {
    #[inline(always)]
    fn from(val: u8) -> EtPort0 {
        EtPort0::from_bits(val)
    }
}
impl From<EtPort0> for u8 {
    #[inline(always)]
    fn from(val: EtPort0) -> u8 {
        EtPort0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EtPort1 {
    #[doc = "Port has no entry in the Egress Treatment table"]
    NO_ENTRY = 0x0,
    #[doc = "Port has an entry in the Egress Treatment table"]
    PORT_ENTRY = 0x01,
}
impl EtPort1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EtPort1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EtPort1 {
    #[inline(always)]
    fn from(val: u8) -> EtPort1 {
        EtPort1::from_bits(val)
    }
}
impl From<EtPort1> for u8 {
    #[inline(always)]
    fn from(val: EtPort1) -> u8 {
        EtPort1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EtPort2 {
    #[doc = "Port has no entry in the Egress Treatment table"]
    NO_ENTRY = 0x0,
    #[doc = "Port has an entry in the Egress Treatment table"]
    PORT_ENTRY = 0x01,
}
impl EtPort2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EtPort2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EtPort2 {
    #[inline(always)]
    fn from(val: u8) -> EtPort2 {
        EtPort2::from_bits(val)
    }
}
impl From<EtPort2> for u8 {
    #[inline(always)]
    fn from(val: EtPort2) -> u8 {
        EtPort2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EtPort3 {
    #[doc = "Port has no entry in the Egress Treatment table"]
    NO_ENTRY = 0x0,
    #[doc = "Port has an entry in the Egress Treatment table"]
    PORT_ENTRY = 0x01,
}
impl EtPort3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EtPort3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EtPort3 {
    #[inline(always)]
    fn from(val: u8) -> EtPort3 {
        EtPort3::from_bits(val)
    }
}
impl From<EtPort3> for u8 {
    #[inline(always)]
    fn from(val: EtPort3) -> u8 {
        EtPort3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EtPort4 {
    #[doc = "Port has no entry in the Egress Treatment table"]
    NO_ENTRY = 0x0,
    #[doc = "Port has an entry in the Egress Treatment table"]
    PORT_ENTRY = 0x01,
}
impl EtPort4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EtPort4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EtPort4 {
    #[inline(always)]
    fn from(val: u8) -> EtPort4 {
        EtPort4::from_bits(val)
    }
}
impl From<EtPort4> for u8 {
    #[inline(always)]
    fn from(val: EtPort4) -> u8 {
        EtPort4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mloc {
    #[doc = "Common memory"]
    CM = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Mloc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mloc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mloc {
    #[inline(always)]
    fn from(val: u8) -> Mloc {
        Mloc::from_bits(val)
    }
}
impl From<Mloc> for u8 {
    #[inline(always)]
    fn from(val: Mloc) -> u8 {
        Mloc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WordSize {
    #[doc = "24 bytes"]
    B_24 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl WordSize {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> WordSize {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for WordSize {
    #[inline(always)]
    fn from(val: u8) -> WordSize {
        WordSize::from_bits(val)
    }
}
impl From<WordSize> for u8 {
    #[inline(always)]
    fn from(val: WordSize) -> u8 {
        WordSize::to_bits(val)
    }
}
