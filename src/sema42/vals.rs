#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate0Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate0Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate0Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate0Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate0Gtfsm {
        Gate0Gtfsm::from_bits(val)
    }
}
impl From<Gate0Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate0Gtfsm) -> u8 {
        Gate0Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate10Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate10Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate10Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate10Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate10Gtfsm {
        Gate10Gtfsm::from_bits(val)
    }
}
impl From<Gate10Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate10Gtfsm) -> u8 {
        Gate10Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate11Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate11Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate11Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate11Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate11Gtfsm {
        Gate11Gtfsm::from_bits(val)
    }
}
impl From<Gate11Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate11Gtfsm) -> u8 {
        Gate11Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate12Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate12Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate12Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate12Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate12Gtfsm {
        Gate12Gtfsm::from_bits(val)
    }
}
impl From<Gate12Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate12Gtfsm) -> u8 {
        Gate12Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate13Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate13Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate13Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate13Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate13Gtfsm {
        Gate13Gtfsm::from_bits(val)
    }
}
impl From<Gate13Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate13Gtfsm) -> u8 {
        Gate13Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate14Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate14Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate14Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate14Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate14Gtfsm {
        Gate14Gtfsm::from_bits(val)
    }
}
impl From<Gate14Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate14Gtfsm) -> u8 {
        Gate14Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate15Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate15Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate15Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate15Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate15Gtfsm {
        Gate15Gtfsm::from_bits(val)
    }
}
impl From<Gate15Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate15Gtfsm) -> u8 {
        Gate15Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate16Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate16Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate16Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate16Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate16Gtfsm {
        Gate16Gtfsm::from_bits(val)
    }
}
impl From<Gate16Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate16Gtfsm) -> u8 {
        Gate16Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate17Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate17Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate17Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate17Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate17Gtfsm {
        Gate17Gtfsm::from_bits(val)
    }
}
impl From<Gate17Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate17Gtfsm) -> u8 {
        Gate17Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate18Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate18Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate18Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate18Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate18Gtfsm {
        Gate18Gtfsm::from_bits(val)
    }
}
impl From<Gate18Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate18Gtfsm) -> u8 {
        Gate18Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate19Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate19Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate19Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate19Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate19Gtfsm {
        Gate19Gtfsm::from_bits(val)
    }
}
impl From<Gate19Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate19Gtfsm) -> u8 {
        Gate19Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate1Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate1Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate1Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate1Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate1Gtfsm {
        Gate1Gtfsm::from_bits(val)
    }
}
impl From<Gate1Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate1Gtfsm) -> u8 {
        Gate1Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate20Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate20Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate20Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate20Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate20Gtfsm {
        Gate20Gtfsm::from_bits(val)
    }
}
impl From<Gate20Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate20Gtfsm) -> u8 {
        Gate20Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate21Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate21Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate21Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate21Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate21Gtfsm {
        Gate21Gtfsm::from_bits(val)
    }
}
impl From<Gate21Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate21Gtfsm) -> u8 {
        Gate21Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate22Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate22Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate22Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate22Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate22Gtfsm {
        Gate22Gtfsm::from_bits(val)
    }
}
impl From<Gate22Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate22Gtfsm) -> u8 {
        Gate22Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate23Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate23Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate23Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate23Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate23Gtfsm {
        Gate23Gtfsm::from_bits(val)
    }
}
impl From<Gate23Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate23Gtfsm) -> u8 {
        Gate23Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate24Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate24Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate24Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate24Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate24Gtfsm {
        Gate24Gtfsm::from_bits(val)
    }
}
impl From<Gate24Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate24Gtfsm) -> u8 {
        Gate24Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate25Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate25Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate25Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate25Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate25Gtfsm {
        Gate25Gtfsm::from_bits(val)
    }
}
impl From<Gate25Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate25Gtfsm) -> u8 {
        Gate25Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate26Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate26Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate26Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate26Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate26Gtfsm {
        Gate26Gtfsm::from_bits(val)
    }
}
impl From<Gate26Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate26Gtfsm) -> u8 {
        Gate26Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate27Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate27Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate27Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate27Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate27Gtfsm {
        Gate27Gtfsm::from_bits(val)
    }
}
impl From<Gate27Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate27Gtfsm) -> u8 {
        Gate27Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate28Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate28Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate28Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate28Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate28Gtfsm {
        Gate28Gtfsm::from_bits(val)
    }
}
impl From<Gate28Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate28Gtfsm) -> u8 {
        Gate28Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate29Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate29Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate29Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate29Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate29Gtfsm {
        Gate29Gtfsm::from_bits(val)
    }
}
impl From<Gate29Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate29Gtfsm) -> u8 {
        Gate29Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate2Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate2Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate2Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate2Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate2Gtfsm {
        Gate2Gtfsm::from_bits(val)
    }
}
impl From<Gate2Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate2Gtfsm) -> u8 {
        Gate2Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate30Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate30Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate30Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate30Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate30Gtfsm {
        Gate30Gtfsm::from_bits(val)
    }
}
impl From<Gate30Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate30Gtfsm) -> u8 {
        Gate30Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate31Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate31Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate31Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate31Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate31Gtfsm {
        Gate31Gtfsm::from_bits(val)
    }
}
impl From<Gate31Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate31Gtfsm) -> u8 {
        Gate31Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate32Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate32Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate32Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate32Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate32Gtfsm {
        Gate32Gtfsm::from_bits(val)
    }
}
impl From<Gate32Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate32Gtfsm) -> u8 {
        Gate32Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate33Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate33Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate33Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate33Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate33Gtfsm {
        Gate33Gtfsm::from_bits(val)
    }
}
impl From<Gate33Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate33Gtfsm) -> u8 {
        Gate33Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate34Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate34Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate34Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate34Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate34Gtfsm {
        Gate34Gtfsm::from_bits(val)
    }
}
impl From<Gate34Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate34Gtfsm) -> u8 {
        Gate34Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate35Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate35Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate35Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate35Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate35Gtfsm {
        Gate35Gtfsm::from_bits(val)
    }
}
impl From<Gate35Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate35Gtfsm) -> u8 {
        Gate35Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate36Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate36Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate36Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate36Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate36Gtfsm {
        Gate36Gtfsm::from_bits(val)
    }
}
impl From<Gate36Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate36Gtfsm) -> u8 {
        Gate36Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate37Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate37Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate37Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate37Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate37Gtfsm {
        Gate37Gtfsm::from_bits(val)
    }
}
impl From<Gate37Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate37Gtfsm) -> u8 {
        Gate37Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate38Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate38Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate38Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate38Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate38Gtfsm {
        Gate38Gtfsm::from_bits(val)
    }
}
impl From<Gate38Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate38Gtfsm) -> u8 {
        Gate38Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate39Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate39Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate39Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate39Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate39Gtfsm {
        Gate39Gtfsm::from_bits(val)
    }
}
impl From<Gate39Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate39Gtfsm) -> u8 {
        Gate39Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate3Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate3Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate3Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate3Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate3Gtfsm {
        Gate3Gtfsm::from_bits(val)
    }
}
impl From<Gate3Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate3Gtfsm) -> u8 {
        Gate3Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate40Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate40Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate40Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate40Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate40Gtfsm {
        Gate40Gtfsm::from_bits(val)
    }
}
impl From<Gate40Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate40Gtfsm) -> u8 {
        Gate40Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate41Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate41Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate41Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate41Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate41Gtfsm {
        Gate41Gtfsm::from_bits(val)
    }
}
impl From<Gate41Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate41Gtfsm) -> u8 {
        Gate41Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate42Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate42Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate42Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate42Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate42Gtfsm {
        Gate42Gtfsm::from_bits(val)
    }
}
impl From<Gate42Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate42Gtfsm) -> u8 {
        Gate42Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate43Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate43Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate43Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate43Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate43Gtfsm {
        Gate43Gtfsm::from_bits(val)
    }
}
impl From<Gate43Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate43Gtfsm) -> u8 {
        Gate43Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate44Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate44Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate44Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate44Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate44Gtfsm {
        Gate44Gtfsm::from_bits(val)
    }
}
impl From<Gate44Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate44Gtfsm) -> u8 {
        Gate44Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate45Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate45Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate45Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate45Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate45Gtfsm {
        Gate45Gtfsm::from_bits(val)
    }
}
impl From<Gate45Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate45Gtfsm) -> u8 {
        Gate45Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate46Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate46Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate46Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate46Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate46Gtfsm {
        Gate46Gtfsm::from_bits(val)
    }
}
impl From<Gate46Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate46Gtfsm) -> u8 {
        Gate46Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate47Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate47Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate47Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate47Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate47Gtfsm {
        Gate47Gtfsm::from_bits(val)
    }
}
impl From<Gate47Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate47Gtfsm) -> u8 {
        Gate47Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate48Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate48Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate48Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate48Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate48Gtfsm {
        Gate48Gtfsm::from_bits(val)
    }
}
impl From<Gate48Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate48Gtfsm) -> u8 {
        Gate48Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate49Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate49Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate49Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate49Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate49Gtfsm {
        Gate49Gtfsm::from_bits(val)
    }
}
impl From<Gate49Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate49Gtfsm) -> u8 {
        Gate49Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate4Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate4Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate4Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate4Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate4Gtfsm {
        Gate4Gtfsm::from_bits(val)
    }
}
impl From<Gate4Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate4Gtfsm) -> u8 {
        Gate4Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate50Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate50Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate50Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate50Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate50Gtfsm {
        Gate50Gtfsm::from_bits(val)
    }
}
impl From<Gate50Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate50Gtfsm) -> u8 {
        Gate50Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate51Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate51Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate51Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate51Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate51Gtfsm {
        Gate51Gtfsm::from_bits(val)
    }
}
impl From<Gate51Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate51Gtfsm) -> u8 {
        Gate51Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate52Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate52Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate52Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate52Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate52Gtfsm {
        Gate52Gtfsm::from_bits(val)
    }
}
impl From<Gate52Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate52Gtfsm) -> u8 {
        Gate52Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate53Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate53Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate53Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate53Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate53Gtfsm {
        Gate53Gtfsm::from_bits(val)
    }
}
impl From<Gate53Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate53Gtfsm) -> u8 {
        Gate53Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate54Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate54Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate54Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate54Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate54Gtfsm {
        Gate54Gtfsm::from_bits(val)
    }
}
impl From<Gate54Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate54Gtfsm) -> u8 {
        Gate54Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate55Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate55Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate55Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate55Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate55Gtfsm {
        Gate55Gtfsm::from_bits(val)
    }
}
impl From<Gate55Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate55Gtfsm) -> u8 {
        Gate55Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate56Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate56Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate56Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate56Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate56Gtfsm {
        Gate56Gtfsm::from_bits(val)
    }
}
impl From<Gate56Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate56Gtfsm) -> u8 {
        Gate56Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate57Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate57Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate57Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate57Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate57Gtfsm {
        Gate57Gtfsm::from_bits(val)
    }
}
impl From<Gate57Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate57Gtfsm) -> u8 {
        Gate57Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate58Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate58Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate58Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate58Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate58Gtfsm {
        Gate58Gtfsm::from_bits(val)
    }
}
impl From<Gate58Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate58Gtfsm) -> u8 {
        Gate58Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate59Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate59Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate59Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate59Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate59Gtfsm {
        Gate59Gtfsm::from_bits(val)
    }
}
impl From<Gate59Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate59Gtfsm) -> u8 {
        Gate59Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate5Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate5Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate5Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate5Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate5Gtfsm {
        Gate5Gtfsm::from_bits(val)
    }
}
impl From<Gate5Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate5Gtfsm) -> u8 {
        Gate5Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate60Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate60Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate60Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate60Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate60Gtfsm {
        Gate60Gtfsm::from_bits(val)
    }
}
impl From<Gate60Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate60Gtfsm) -> u8 {
        Gate60Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate61Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate61Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate61Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate61Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate61Gtfsm {
        Gate61Gtfsm::from_bits(val)
    }
}
impl From<Gate61Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate61Gtfsm) -> u8 {
        Gate61Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate62Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate62Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate62Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate62Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate62Gtfsm {
        Gate62Gtfsm::from_bits(val)
    }
}
impl From<Gate62Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate62Gtfsm) -> u8 {
        Gate62Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate63Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate63Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate63Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate63Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate63Gtfsm {
        Gate63Gtfsm::from_bits(val)
    }
}
impl From<Gate63Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate63Gtfsm) -> u8 {
        Gate63Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate6Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate6Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate6Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate6Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate6Gtfsm {
        Gate6Gtfsm::from_bits(val)
    }
}
impl From<Gate6Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate6Gtfsm) -> u8 {
        Gate6Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate7Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate7Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate7Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate7Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate7Gtfsm {
        Gate7Gtfsm::from_bits(val)
    }
}
impl From<Gate7Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate7Gtfsm) -> u8 {
        Gate7Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate8Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate8Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate8Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate8Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate8Gtfsm {
        Gate8Gtfsm::from_bits(val)
    }
}
impl From<Gate8Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate8Gtfsm) -> u8 {
        Gate8Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gate9Gtfsm {
    #[doc = "The gate is unlocked (free)."]
    UNLOCKED = 0x0,
    #[doc = "Domain 0 locked the gate."]
    LOCKED_BY_D0 = 0x01,
    #[doc = "Domain 1 locked the gate."]
    LOCKED_BY_D1 = 0x02,
    #[doc = "Domain 2 locked the gate."]
    LOCKED_BY_D2 = 0x03,
    #[doc = "Domain 3 locked the gate."]
    LOCKED_BY_D3 = 0x04,
    #[doc = "Domain 4 locked the gate."]
    LOCKED_BY_D4 = 0x05,
    #[doc = "Domain 5 locked the gate."]
    LOCKED_BY_D5 = 0x06,
    #[doc = "Domain 6 locked the gate."]
    LOCKED_BY_D6 = 0x07,
    #[doc = "Domain 7 locked the gate."]
    LOCKED_BY_D7 = 0x08,
    #[doc = "Domain 8 locked the gate."]
    LOCKED_BY_D8 = 0x09,
    #[doc = "Domain 9 locked the gate."]
    LOCKED_BY_D9 = 0x0a,
    #[doc = "Domain 10 locked the gate."]
    LOCKED_BY_D10 = 0x0b,
    #[doc = "Domain 11 locked the gate."]
    LOCKED_BY_D11 = 0x0c,
    #[doc = "Domain 12 locked the gate."]
    LOCKED_BY_D12 = 0x0d,
    #[doc = "Domain 13 locked the gate."]
    LOCKED_BY_D13 = 0x0e,
    #[doc = "Domain 14 locked the gate."]
    LOCKED_BY_D14 = 0x0f,
}
impl Gate9Gtfsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gate9Gtfsm {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gate9Gtfsm {
    #[inline(always)]
    fn from(val: u8) -> Gate9Gtfsm {
        Gate9Gtfsm::from_bits(val)
    }
}
impl From<Gate9Gtfsm> for u8 {
    #[inline(always)]
    fn from(val: Gate9Gtfsm) -> u8 {
        Gate9Gtfsm::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rstgsm {
    #[doc = "Idle, waiting for the first data pattern write."]
    IDLE = 0x0,
    #[doc = "Waiting for the second data pattern write"]
    WAITING = 0x01,
    #[doc = "The 2-write sequence has completed. Generate the specified gate reset(s). After the reset is performed, this machine returns to the idle (waiting for first data pattern write) state."]
    TWO_WRITE_DONE = 0x02,
    _RESERVED_3 = 0x03,
}
impl Rstgsm {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rstgsm {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rstgsm {
    #[inline(always)]
    fn from(val: u8) -> Rstgsm {
        Rstgsm::from_bits(val)
    }
}
impl From<Rstgsm> for u8 {
    #[inline(always)]
    fn from(val: Rstgsm) -> u8 {
        Rstgsm::to_bits(val)
    }
}
