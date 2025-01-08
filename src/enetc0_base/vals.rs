#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L3cd {
    #[doc = "Enabled"]
    ENABLE = 0x0,
    #[doc = "Disabled"]
    DISABLE = 0x01,
}
impl L3cd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L3cd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L3cd {
    #[inline(always)]
    fn from(val: u8) -> L3cd {
        L3cd::from_bits(val)
    }
}
impl From<L3cd> for u8 {
    #[inline(always)]
    fn from(val: L3cd) -> u8 {
        L3cd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum L4cd {
    #[doc = "Enabled"]
    ENABLE = 0x0,
    #[doc = "Disabled"]
    DISABLE = 0x01,
}
impl L4cd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> L4cd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for L4cd {
    #[inline(always)]
    fn from(val: u8) -> L4cd {
        L4cd::from_bits(val)
    }
}
impl From<L4cd> for u8 {
    #[inline(always)]
    fn from(val: L4cd) -> u8 {
        L4cd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Oai {
    #[doc = "Indicates that the Inner is not valid if only one tag is found"]
    NOT_VALID = 0x0,
    #[doc = "Indicates that the outer should be used as the Inner if only one tag is found"]
    OUTER = 0x01,
}
impl Oai {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oai {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oai {
    #[inline(always)]
    fn from(val: u8) -> Oai {
        Oai::from_bits(val)
    }
}
impl From<Oai> for u8 {
    #[inline(always)]
    fn from(val: Oai) -> u8 {
        Oai::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PinvlanrTpid {
    #[doc = "Standard C-VLAN 0x8100"]
    C_VLAN = 0x0,
    #[doc = "Standard S-VLAN 0x88A8"]
    S_VLAN = 0x01,
    #[doc = "Custom VLAN as defined by CVLANR1\\[ETYPE\\]"]
    CVLANR1_ETYPE = 0x02,
    #[doc = "Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    CVLANR2_ETYPE = 0x03,
}
impl PinvlanrTpid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PinvlanrTpid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PinvlanrTpid {
    #[inline(always)]
    fn from(val: u8) -> PinvlanrTpid {
        PinvlanrTpid::from_bits(val)
    }
}
impl From<PinvlanrTpid> for u8 {
    #[inline(always)]
    fn from(val: PinvlanrTpid) -> u8 {
        PinvlanrTpid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PonvlanrTpid {
    #[doc = "Standard C-VLAN 0x8100"]
    C_VLAN = 0x0,
    #[doc = "Standard S-VLAN 0x88A8"]
    S_VLAN = 0x01,
    #[doc = "Custom VLAN as defined by CVLANR1\\[ETYPE\\]"]
    CVLANR1_ETYPE = 0x02,
    #[doc = "Custom VLAN as defined by CVLANR2\\[ETYPE\\]"]
    CVLANR2_ETYPE = 0x03,
}
impl PonvlanrTpid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PonvlanrTpid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PonvlanrTpid {
    #[inline(always)]
    fn from(val: u8) -> PonvlanrTpid {
        PonvlanrTpid::from_bits(val)
    }
}
impl From<PonvlanrTpid> for u8 {
    #[inline(always)]
    fn from(val: PonvlanrTpid) -> u8 {
        PonvlanrTpid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Psi0vlanrTpid {
    #[doc = "Standard C-VLAN 0x8100"]
    C_VLAN = 0x0,
    #[doc = "Standard S-VLAN 0x88A8"]
    S_VLAN = 0x01,
    #[doc = "Custom VLAN as defined by CVLANR1\\[ETYPE\\]. Note that CVLANR1\\[V\\] is not checked for SI-based VLAN insertion; TPID value specified in CVLANR1\\[ETYPE\\] will be used to construct the VLAN header regardless of the value specified in CVLANR1\\[V\\]."]
    CVLANR1_V = 0x02,
    #[doc = "Custom VLAN as defined by CVLANR2\\[ETYPE\\]. Note that CVLANR2\\[V\\] is not checked for SI-based VLAN insertion; TPID value specified in CVLANR2\\[ETYPE\\] will be used to construct the VLAN header regardless of the value specified in CVLANR2\\[V\\]."]
    CVLANR2_V = 0x03,
}
impl Psi0vlanrTpid {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Psi0vlanrTpid {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Psi0vlanrTpid {
    #[inline(always)]
    fn from(val: u8) -> Psi0vlanrTpid {
        Psi0vlanrTpid::from_bits(val)
    }
}
impl From<Psi0vlanrTpid> for u8 {
    #[inline(always)]
    fn from(val: Psi0vlanrTpid) -> u8 {
        Psi0vlanrTpid::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Si0VlanUta {
    #[doc = "SI 0 does not qualify for reception of untagged frames"]
    NOT_QUALIFIED = 0x0,
    #[doc = "SI 0 does qualify for reception of untagged frames"]
    QUALIFY = 0x01,
}
impl Si0VlanUta {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Si0VlanUta {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Si0VlanUta {
    #[inline(always)]
    fn from(val: u8) -> Si0VlanUta {
        Si0VlanUta::from_bits(val)
    }
}
impl From<Si0VlanUta> for u8 {
    #[inline(always)]
    fn from(val: Si0VlanUta) -> u8 {
        Si0VlanUta::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vs {
    #[doc = "Inner VLAN tag will be used for VLAN filtering"]
    INNER_VLAN = 0x0,
    #[doc = "Outer VLAN tag will be used for VLAN filtering"]
    OUTER_VLAN = 0x01,
}
impl Vs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vs {
    #[inline(always)]
    fn from(val: u8) -> Vs {
        Vs::from_bits(val)
    }
}
impl From<Vs> for u8 {
    #[inline(always)]
    fn from(val: Vs) -> u8 {
        Vs::to_bits(val)
    }
}
