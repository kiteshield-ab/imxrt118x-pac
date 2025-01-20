#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Arb {
    #[doc = "Fixed priority"]
    Fp = 0x0,
    #[doc = "Round-robin (rotating) priority"]
    Rr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs0Arb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Arb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Arb {
    #[inline(always)]
    fn from(val: u8) -> Crs0Arb {
        Crs0Arb::from_bits(val)
    }
}
impl From<Crs0Arb> for u8 {
    #[inline(always)]
    fn from(val: Crs0Arb) -> u8 {
        Crs0Arb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Hpe0 {
    #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
    M0 = 0x0,
    #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
    M1 = 0x01,
}
impl Crs0Hpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Hpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Hpe0 {
    #[inline(always)]
    fn from(val: u8) -> Crs0Hpe0 {
        Crs0Hpe0::from_bits(val)
    }
}
impl From<Crs0Hpe0> for u8 {
    #[inline(always)]
    fn from(val: Crs0Hpe0) -> u8 {
        Crs0Hpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Hpe1 {
    #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
    Hpe1 = 0x0,
    #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
    Hpe11 = 0x01,
}
impl Crs0Hpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Hpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Hpe1 {
    #[inline(always)]
    fn from(val: u8) -> Crs0Hpe1 {
        Crs0Hpe1::from_bits(val)
    }
}
impl From<Crs0Hpe1> for u8 {
    #[inline(always)]
    fn from(val: Crs0Hpe1) -> u8 {
        Crs0Hpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Hpe2 {
    #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
    Hpe2 = 0x0,
    #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
    Hpe21 = 0x01,
}
impl Crs0Hpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Hpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Hpe2 {
    #[inline(always)]
    fn from(val: u8) -> Crs0Hpe2 {
        Crs0Hpe2::from_bits(val)
    }
}
impl From<Crs0Hpe2> for u8 {
    #[inline(always)]
    fn from(val: Crs0Hpe2) -> u8 {
        Crs0Hpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Hpe3 {
    #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
    Hpe3 = 0x0,
    #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
    Hpe31 = 0x01,
}
impl Crs0Hpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Hpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Hpe3 {
    #[inline(always)]
    fn from(val: u8) -> Crs0Hpe3 {
        Crs0Hpe3::from_bits(val)
    }
}
impl From<Crs0Hpe3> for u8 {
    #[inline(always)]
    fn from(val: Crs0Hpe3) -> u8 {
        Crs0Hpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Hpe4 {
    #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
    Hpe4 = 0x0,
    #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
    Hpe41 = 0x01,
}
impl Crs0Hpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Hpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Hpe4 {
    #[inline(always)]
    fn from(val: u8) -> Crs0Hpe4 {
        Crs0Hpe4::from_bits(val)
    }
}
impl From<Crs0Hpe4> for u8 {
    #[inline(always)]
    fn from(val: Crs0Hpe4) -> u8 {
        Crs0Hpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Hpe5 {
    #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
    Hpe5 = 0x0,
    #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
    Hpe51 = 0x01,
}
impl Crs0Hpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Hpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Hpe5 {
    #[inline(always)]
    fn from(val: u8) -> Crs0Hpe5 {
        Crs0Hpe5::from_bits(val)
    }
}
impl From<Crs0Hpe5> for u8 {
    #[inline(always)]
    fn from(val: Crs0Hpe5) -> u8 {
        Crs0Hpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Park {
    #[doc = "Park on master port M0"]
    MasterPort0 = 0x0,
    #[doc = "Park on master port M1"]
    MasterPort1 = 0x01,
    #[doc = "Park on master port M2"]
    MasterPort2 = 0x02,
    #[doc = "Park on master port M3"]
    MasterPort3 = 0x03,
    #[doc = "Park on master port M4"]
    MasterPort4 = 0x04,
    #[doc = "Park on master port M5"]
    MasterPort5 = 0x05,
    #[doc = "Park on master port M6"]
    MasterPort6 = 0x06,
    #[doc = "Park on master port M7"]
    MasterPort7 = 0x07,
}
impl Crs0Park {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Park {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Park {
    #[inline(always)]
    fn from(val: u8) -> Crs0Park {
        Crs0Park::from_bits(val)
    }
}
impl From<Crs0Park> for u8 {
    #[inline(always)]
    fn from(val: Crs0Park) -> u8 {
        Crs0Park::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Pctl {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
    Park = 0x0,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
    SlavePort = 0x01,
    #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
    SafeState = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs0Pctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Pctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Pctl {
    #[inline(always)]
    fn from(val: u8) -> Crs0Pctl {
        Crs0Pctl::from_bits(val)
    }
}
impl From<Crs0Pctl> for u8 {
    #[inline(always)]
    fn from(val: Crs0Pctl) -> u8 {
        Crs0Pctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs0Ro {
    #[doc = "The CRSn and PRSn registers are writeable"]
    CrsPrsY = 0x0,
    #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
    CrsPrsN = 0x01,
}
impl Crs0Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs0Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs0Ro {
    #[inline(always)]
    fn from(val: u8) -> Crs0Ro {
        Crs0Ro::from_bits(val)
    }
}
impl From<Crs0Ro> for u8 {
    #[inline(always)]
    fn from(val: Crs0Ro) -> u8 {
        Crs0Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Arb {
    #[doc = "Fixed priority"]
    Fp = 0x0,
    #[doc = "Round-robin (rotating) priority"]
    Rr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs1Arb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Arb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Arb {
    #[inline(always)]
    fn from(val: u8) -> Crs1Arb {
        Crs1Arb::from_bits(val)
    }
}
impl From<Crs1Arb> for u8 {
    #[inline(always)]
    fn from(val: Crs1Arb) -> u8 {
        Crs1Arb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Hpe0 {
    #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
    M0 = 0x0,
    #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
    M1 = 0x01,
}
impl Crs1Hpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Hpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Hpe0 {
    #[inline(always)]
    fn from(val: u8) -> Crs1Hpe0 {
        Crs1Hpe0::from_bits(val)
    }
}
impl From<Crs1Hpe0> for u8 {
    #[inline(always)]
    fn from(val: Crs1Hpe0) -> u8 {
        Crs1Hpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Hpe1 {
    #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
    Hpe1 = 0x0,
    #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
    Hpe11 = 0x01,
}
impl Crs1Hpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Hpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Hpe1 {
    #[inline(always)]
    fn from(val: u8) -> Crs1Hpe1 {
        Crs1Hpe1::from_bits(val)
    }
}
impl From<Crs1Hpe1> for u8 {
    #[inline(always)]
    fn from(val: Crs1Hpe1) -> u8 {
        Crs1Hpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Hpe2 {
    #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
    Hpe2 = 0x0,
    #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
    Hpe21 = 0x01,
}
impl Crs1Hpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Hpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Hpe2 {
    #[inline(always)]
    fn from(val: u8) -> Crs1Hpe2 {
        Crs1Hpe2::from_bits(val)
    }
}
impl From<Crs1Hpe2> for u8 {
    #[inline(always)]
    fn from(val: Crs1Hpe2) -> u8 {
        Crs1Hpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Hpe3 {
    #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
    Hpe3 = 0x0,
    #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
    Hpe31 = 0x01,
}
impl Crs1Hpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Hpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Hpe3 {
    #[inline(always)]
    fn from(val: u8) -> Crs1Hpe3 {
        Crs1Hpe3::from_bits(val)
    }
}
impl From<Crs1Hpe3> for u8 {
    #[inline(always)]
    fn from(val: Crs1Hpe3) -> u8 {
        Crs1Hpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Hpe4 {
    #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
    Hpe4 = 0x0,
    #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
    Hpe41 = 0x01,
}
impl Crs1Hpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Hpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Hpe4 {
    #[inline(always)]
    fn from(val: u8) -> Crs1Hpe4 {
        Crs1Hpe4::from_bits(val)
    }
}
impl From<Crs1Hpe4> for u8 {
    #[inline(always)]
    fn from(val: Crs1Hpe4) -> u8 {
        Crs1Hpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Hpe5 {
    #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
    Hpe5 = 0x0,
    #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
    Hpe51 = 0x01,
}
impl Crs1Hpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Hpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Hpe5 {
    #[inline(always)]
    fn from(val: u8) -> Crs1Hpe5 {
        Crs1Hpe5::from_bits(val)
    }
}
impl From<Crs1Hpe5> for u8 {
    #[inline(always)]
    fn from(val: Crs1Hpe5) -> u8 {
        Crs1Hpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Park {
    #[doc = "Park on master port M0"]
    MasterPort0 = 0x0,
    #[doc = "Park on master port M1"]
    MasterPort1 = 0x01,
    #[doc = "Park on master port M2"]
    MasterPort2 = 0x02,
    #[doc = "Park on master port M3"]
    MasterPort3 = 0x03,
    #[doc = "Park on master port M4"]
    MasterPort4 = 0x04,
    #[doc = "Park on master port M5"]
    MasterPort5 = 0x05,
    #[doc = "Park on master port M6"]
    MasterPort6 = 0x06,
    #[doc = "Park on master port M7"]
    MasterPort7 = 0x07,
}
impl Crs1Park {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Park {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Park {
    #[inline(always)]
    fn from(val: u8) -> Crs1Park {
        Crs1Park::from_bits(val)
    }
}
impl From<Crs1Park> for u8 {
    #[inline(always)]
    fn from(val: Crs1Park) -> u8 {
        Crs1Park::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Pctl {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
    Park = 0x0,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
    SlavePort = 0x01,
    #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
    SafeState = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs1Pctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Pctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Pctl {
    #[inline(always)]
    fn from(val: u8) -> Crs1Pctl {
        Crs1Pctl::from_bits(val)
    }
}
impl From<Crs1Pctl> for u8 {
    #[inline(always)]
    fn from(val: Crs1Pctl) -> u8 {
        Crs1Pctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs1Ro {
    #[doc = "The CRSn and PRSn registers are writeable"]
    CrsPrsY = 0x0,
    #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
    CrsPrsN = 0x01,
}
impl Crs1Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs1Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs1Ro {
    #[inline(always)]
    fn from(val: u8) -> Crs1Ro {
        Crs1Ro::from_bits(val)
    }
}
impl From<Crs1Ro> for u8 {
    #[inline(always)]
    fn from(val: Crs1Ro) -> u8 {
        Crs1Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Arb {
    #[doc = "Fixed priority"]
    Fp = 0x0,
    #[doc = "Round-robin (rotating) priority"]
    Rr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs2Arb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Arb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Arb {
    #[inline(always)]
    fn from(val: u8) -> Crs2Arb {
        Crs2Arb::from_bits(val)
    }
}
impl From<Crs2Arb> for u8 {
    #[inline(always)]
    fn from(val: Crs2Arb) -> u8 {
        Crs2Arb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Hpe0 {
    #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
    M0 = 0x0,
    #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
    M1 = 0x01,
}
impl Crs2Hpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Hpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Hpe0 {
    #[inline(always)]
    fn from(val: u8) -> Crs2Hpe0 {
        Crs2Hpe0::from_bits(val)
    }
}
impl From<Crs2Hpe0> for u8 {
    #[inline(always)]
    fn from(val: Crs2Hpe0) -> u8 {
        Crs2Hpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Hpe1 {
    #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
    Hpe1 = 0x0,
    #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
    Hpe11 = 0x01,
}
impl Crs2Hpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Hpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Hpe1 {
    #[inline(always)]
    fn from(val: u8) -> Crs2Hpe1 {
        Crs2Hpe1::from_bits(val)
    }
}
impl From<Crs2Hpe1> for u8 {
    #[inline(always)]
    fn from(val: Crs2Hpe1) -> u8 {
        Crs2Hpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Hpe2 {
    #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
    Hpe2 = 0x0,
    #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
    Hpe21 = 0x01,
}
impl Crs2Hpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Hpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Hpe2 {
    #[inline(always)]
    fn from(val: u8) -> Crs2Hpe2 {
        Crs2Hpe2::from_bits(val)
    }
}
impl From<Crs2Hpe2> for u8 {
    #[inline(always)]
    fn from(val: Crs2Hpe2) -> u8 {
        Crs2Hpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Hpe3 {
    #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
    Hpe3 = 0x0,
    #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
    Hpe31 = 0x01,
}
impl Crs2Hpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Hpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Hpe3 {
    #[inline(always)]
    fn from(val: u8) -> Crs2Hpe3 {
        Crs2Hpe3::from_bits(val)
    }
}
impl From<Crs2Hpe3> for u8 {
    #[inline(always)]
    fn from(val: Crs2Hpe3) -> u8 {
        Crs2Hpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Hpe4 {
    #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
    Hpe4 = 0x0,
    #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
    Hpe41 = 0x01,
}
impl Crs2Hpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Hpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Hpe4 {
    #[inline(always)]
    fn from(val: u8) -> Crs2Hpe4 {
        Crs2Hpe4::from_bits(val)
    }
}
impl From<Crs2Hpe4> for u8 {
    #[inline(always)]
    fn from(val: Crs2Hpe4) -> u8 {
        Crs2Hpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Hpe5 {
    #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
    Hpe5 = 0x0,
    #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
    Hpe51 = 0x01,
}
impl Crs2Hpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Hpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Hpe5 {
    #[inline(always)]
    fn from(val: u8) -> Crs2Hpe5 {
        Crs2Hpe5::from_bits(val)
    }
}
impl From<Crs2Hpe5> for u8 {
    #[inline(always)]
    fn from(val: Crs2Hpe5) -> u8 {
        Crs2Hpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Park {
    #[doc = "Park on master port M0"]
    MasterPort0 = 0x0,
    #[doc = "Park on master port M1"]
    MasterPort1 = 0x01,
    #[doc = "Park on master port M2"]
    MasterPort2 = 0x02,
    #[doc = "Park on master port M3"]
    MasterPort3 = 0x03,
    #[doc = "Park on master port M4"]
    MasterPort4 = 0x04,
    #[doc = "Park on master port M5"]
    MasterPort5 = 0x05,
    #[doc = "Park on master port M6"]
    MasterPort6 = 0x06,
    #[doc = "Park on master port M7"]
    MasterPort7 = 0x07,
}
impl Crs2Park {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Park {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Park {
    #[inline(always)]
    fn from(val: u8) -> Crs2Park {
        Crs2Park::from_bits(val)
    }
}
impl From<Crs2Park> for u8 {
    #[inline(always)]
    fn from(val: Crs2Park) -> u8 {
        Crs2Park::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Pctl {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
    Park = 0x0,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
    SlavePort = 0x01,
    #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
    SafeState = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs2Pctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Pctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Pctl {
    #[inline(always)]
    fn from(val: u8) -> Crs2Pctl {
        Crs2Pctl::from_bits(val)
    }
}
impl From<Crs2Pctl> for u8 {
    #[inline(always)]
    fn from(val: Crs2Pctl) -> u8 {
        Crs2Pctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs2Ro {
    #[doc = "The CRSn and PRSn registers are writeable"]
    CrsPrsY = 0x0,
    #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
    CrsPrsN = 0x01,
}
impl Crs2Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs2Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs2Ro {
    #[inline(always)]
    fn from(val: u8) -> Crs2Ro {
        Crs2Ro::from_bits(val)
    }
}
impl From<Crs2Ro> for u8 {
    #[inline(always)]
    fn from(val: Crs2Ro) -> u8 {
        Crs2Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Arb {
    #[doc = "Fixed priority"]
    Fp = 0x0,
    #[doc = "Round-robin (rotating) priority"]
    Rr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs3Arb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Arb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Arb {
    #[inline(always)]
    fn from(val: u8) -> Crs3Arb {
        Crs3Arb::from_bits(val)
    }
}
impl From<Crs3Arb> for u8 {
    #[inline(always)]
    fn from(val: Crs3Arb) -> u8 {
        Crs3Arb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Hpe0 {
    #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
    M0 = 0x0,
    #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
    M1 = 0x01,
}
impl Crs3Hpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Hpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Hpe0 {
    #[inline(always)]
    fn from(val: u8) -> Crs3Hpe0 {
        Crs3Hpe0::from_bits(val)
    }
}
impl From<Crs3Hpe0> for u8 {
    #[inline(always)]
    fn from(val: Crs3Hpe0) -> u8 {
        Crs3Hpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Hpe1 {
    #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
    Hpe1 = 0x0,
    #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
    Hpe11 = 0x01,
}
impl Crs3Hpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Hpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Hpe1 {
    #[inline(always)]
    fn from(val: u8) -> Crs3Hpe1 {
        Crs3Hpe1::from_bits(val)
    }
}
impl From<Crs3Hpe1> for u8 {
    #[inline(always)]
    fn from(val: Crs3Hpe1) -> u8 {
        Crs3Hpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Hpe2 {
    #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
    Hpe2 = 0x0,
    #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
    Hpe21 = 0x01,
}
impl Crs3Hpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Hpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Hpe2 {
    #[inline(always)]
    fn from(val: u8) -> Crs3Hpe2 {
        Crs3Hpe2::from_bits(val)
    }
}
impl From<Crs3Hpe2> for u8 {
    #[inline(always)]
    fn from(val: Crs3Hpe2) -> u8 {
        Crs3Hpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Hpe3 {
    #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
    Hpe3 = 0x0,
    #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
    Hpe31 = 0x01,
}
impl Crs3Hpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Hpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Hpe3 {
    #[inline(always)]
    fn from(val: u8) -> Crs3Hpe3 {
        Crs3Hpe3::from_bits(val)
    }
}
impl From<Crs3Hpe3> for u8 {
    #[inline(always)]
    fn from(val: Crs3Hpe3) -> u8 {
        Crs3Hpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Hpe4 {
    #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
    Hpe4 = 0x0,
    #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
    Hpe41 = 0x01,
}
impl Crs3Hpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Hpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Hpe4 {
    #[inline(always)]
    fn from(val: u8) -> Crs3Hpe4 {
        Crs3Hpe4::from_bits(val)
    }
}
impl From<Crs3Hpe4> for u8 {
    #[inline(always)]
    fn from(val: Crs3Hpe4) -> u8 {
        Crs3Hpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Hpe5 {
    #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
    Hpe5 = 0x0,
    #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
    Hpe51 = 0x01,
}
impl Crs3Hpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Hpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Hpe5 {
    #[inline(always)]
    fn from(val: u8) -> Crs3Hpe5 {
        Crs3Hpe5::from_bits(val)
    }
}
impl From<Crs3Hpe5> for u8 {
    #[inline(always)]
    fn from(val: Crs3Hpe5) -> u8 {
        Crs3Hpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Park {
    #[doc = "Park on master port M0"]
    MasterPort0 = 0x0,
    #[doc = "Park on master port M1"]
    MasterPort1 = 0x01,
    #[doc = "Park on master port M2"]
    MasterPort2 = 0x02,
    #[doc = "Park on master port M3"]
    MasterPort3 = 0x03,
    #[doc = "Park on master port M4"]
    MasterPort4 = 0x04,
    #[doc = "Park on master port M5"]
    MasterPort5 = 0x05,
    #[doc = "Park on master port M6"]
    MasterPort6 = 0x06,
    #[doc = "Park on master port M7"]
    MasterPort7 = 0x07,
}
impl Crs3Park {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Park {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Park {
    #[inline(always)]
    fn from(val: u8) -> Crs3Park {
        Crs3Park::from_bits(val)
    }
}
impl From<Crs3Park> for u8 {
    #[inline(always)]
    fn from(val: Crs3Park) -> u8 {
        Crs3Park::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Pctl {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
    Park = 0x0,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
    SlavePort = 0x01,
    #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
    SafeState = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs3Pctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Pctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Pctl {
    #[inline(always)]
    fn from(val: u8) -> Crs3Pctl {
        Crs3Pctl::from_bits(val)
    }
}
impl From<Crs3Pctl> for u8 {
    #[inline(always)]
    fn from(val: Crs3Pctl) -> u8 {
        Crs3Pctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs3Ro {
    #[doc = "The CRSn and PRSn registers are writeable"]
    CrsPrsY = 0x0,
    #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
    CrsPrsN = 0x01,
}
impl Crs3Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs3Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs3Ro {
    #[inline(always)]
    fn from(val: u8) -> Crs3Ro {
        Crs3Ro::from_bits(val)
    }
}
impl From<Crs3Ro> for u8 {
    #[inline(always)]
    fn from(val: Crs3Ro) -> u8 {
        Crs3Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Arb {
    #[doc = "Fixed priority"]
    Fp = 0x0,
    #[doc = "Round-robin (rotating) priority"]
    Rr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs4Arb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Arb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Arb {
    #[inline(always)]
    fn from(val: u8) -> Crs4Arb {
        Crs4Arb::from_bits(val)
    }
}
impl From<Crs4Arb> for u8 {
    #[inline(always)]
    fn from(val: Crs4Arb) -> u8 {
        Crs4Arb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Hpe0 {
    #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
    M0 = 0x0,
    #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
    M1 = 0x01,
}
impl Crs4Hpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Hpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Hpe0 {
    #[inline(always)]
    fn from(val: u8) -> Crs4Hpe0 {
        Crs4Hpe0::from_bits(val)
    }
}
impl From<Crs4Hpe0> for u8 {
    #[inline(always)]
    fn from(val: Crs4Hpe0) -> u8 {
        Crs4Hpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Hpe1 {
    #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
    Hpe1 = 0x0,
    #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
    Hpe11 = 0x01,
}
impl Crs4Hpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Hpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Hpe1 {
    #[inline(always)]
    fn from(val: u8) -> Crs4Hpe1 {
        Crs4Hpe1::from_bits(val)
    }
}
impl From<Crs4Hpe1> for u8 {
    #[inline(always)]
    fn from(val: Crs4Hpe1) -> u8 {
        Crs4Hpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Hpe2 {
    #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
    Hpe2 = 0x0,
    #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
    Hpe21 = 0x01,
}
impl Crs4Hpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Hpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Hpe2 {
    #[inline(always)]
    fn from(val: u8) -> Crs4Hpe2 {
        Crs4Hpe2::from_bits(val)
    }
}
impl From<Crs4Hpe2> for u8 {
    #[inline(always)]
    fn from(val: Crs4Hpe2) -> u8 {
        Crs4Hpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Hpe3 {
    #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
    Hpe3 = 0x0,
    #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
    Hpe31 = 0x01,
}
impl Crs4Hpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Hpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Hpe3 {
    #[inline(always)]
    fn from(val: u8) -> Crs4Hpe3 {
        Crs4Hpe3::from_bits(val)
    }
}
impl From<Crs4Hpe3> for u8 {
    #[inline(always)]
    fn from(val: Crs4Hpe3) -> u8 {
        Crs4Hpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Hpe4 {
    #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
    Hpe4 = 0x0,
    #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
    Hpe41 = 0x01,
}
impl Crs4Hpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Hpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Hpe4 {
    #[inline(always)]
    fn from(val: u8) -> Crs4Hpe4 {
        Crs4Hpe4::from_bits(val)
    }
}
impl From<Crs4Hpe4> for u8 {
    #[inline(always)]
    fn from(val: Crs4Hpe4) -> u8 {
        Crs4Hpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Hpe5 {
    #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
    Hpe5 = 0x0,
    #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
    Hpe51 = 0x01,
}
impl Crs4Hpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Hpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Hpe5 {
    #[inline(always)]
    fn from(val: u8) -> Crs4Hpe5 {
        Crs4Hpe5::from_bits(val)
    }
}
impl From<Crs4Hpe5> for u8 {
    #[inline(always)]
    fn from(val: Crs4Hpe5) -> u8 {
        Crs4Hpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Park {
    #[doc = "Park on master port M0"]
    MasterPort0 = 0x0,
    #[doc = "Park on master port M1"]
    MasterPort1 = 0x01,
    #[doc = "Park on master port M2"]
    MasterPort2 = 0x02,
    #[doc = "Park on master port M3"]
    MasterPort3 = 0x03,
    #[doc = "Park on master port M4"]
    MasterPort4 = 0x04,
    #[doc = "Park on master port M5"]
    MasterPort5 = 0x05,
    #[doc = "Park on master port M6"]
    MasterPort6 = 0x06,
    #[doc = "Park on master port M7"]
    MasterPort7 = 0x07,
}
impl Crs4Park {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Park {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Park {
    #[inline(always)]
    fn from(val: u8) -> Crs4Park {
        Crs4Park::from_bits(val)
    }
}
impl From<Crs4Park> for u8 {
    #[inline(always)]
    fn from(val: Crs4Park) -> u8 {
        Crs4Park::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Pctl {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
    Park = 0x0,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
    SlavePort = 0x01,
    #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
    SafeState = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs4Pctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Pctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Pctl {
    #[inline(always)]
    fn from(val: u8) -> Crs4Pctl {
        Crs4Pctl::from_bits(val)
    }
}
impl From<Crs4Pctl> for u8 {
    #[inline(always)]
    fn from(val: Crs4Pctl) -> u8 {
        Crs4Pctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs4Ro {
    #[doc = "The CRSn and PRSn registers are writeable"]
    CrsPrsY = 0x0,
    #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
    CrsPrsN = 0x01,
}
impl Crs4Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs4Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs4Ro {
    #[inline(always)]
    fn from(val: u8) -> Crs4Ro {
        Crs4Ro::from_bits(val)
    }
}
impl From<Crs4Ro> for u8 {
    #[inline(always)]
    fn from(val: Crs4Ro) -> u8 {
        Crs4Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Arb {
    #[doc = "Fixed priority"]
    Fp = 0x0,
    #[doc = "Round-robin (rotating) priority"]
    Rr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs5Arb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Arb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Arb {
    #[inline(always)]
    fn from(val: u8) -> Crs5Arb {
        Crs5Arb::from_bits(val)
    }
}
impl From<Crs5Arb> for u8 {
    #[inline(always)]
    fn from(val: Crs5Arb) -> u8 {
        Crs5Arb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Hpe0 {
    #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
    M0 = 0x0,
    #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
    M1 = 0x01,
}
impl Crs5Hpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Hpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Hpe0 {
    #[inline(always)]
    fn from(val: u8) -> Crs5Hpe0 {
        Crs5Hpe0::from_bits(val)
    }
}
impl From<Crs5Hpe0> for u8 {
    #[inline(always)]
    fn from(val: Crs5Hpe0) -> u8 {
        Crs5Hpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Hpe1 {
    #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
    Hpe1 = 0x0,
    #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
    Hpe11 = 0x01,
}
impl Crs5Hpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Hpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Hpe1 {
    #[inline(always)]
    fn from(val: u8) -> Crs5Hpe1 {
        Crs5Hpe1::from_bits(val)
    }
}
impl From<Crs5Hpe1> for u8 {
    #[inline(always)]
    fn from(val: Crs5Hpe1) -> u8 {
        Crs5Hpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Hpe2 {
    #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
    Hpe2 = 0x0,
    #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
    Hpe21 = 0x01,
}
impl Crs5Hpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Hpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Hpe2 {
    #[inline(always)]
    fn from(val: u8) -> Crs5Hpe2 {
        Crs5Hpe2::from_bits(val)
    }
}
impl From<Crs5Hpe2> for u8 {
    #[inline(always)]
    fn from(val: Crs5Hpe2) -> u8 {
        Crs5Hpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Hpe3 {
    #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
    Hpe3 = 0x0,
    #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
    Hpe31 = 0x01,
}
impl Crs5Hpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Hpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Hpe3 {
    #[inline(always)]
    fn from(val: u8) -> Crs5Hpe3 {
        Crs5Hpe3::from_bits(val)
    }
}
impl From<Crs5Hpe3> for u8 {
    #[inline(always)]
    fn from(val: Crs5Hpe3) -> u8 {
        Crs5Hpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Hpe4 {
    #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
    Hpe4 = 0x0,
    #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
    Hpe41 = 0x01,
}
impl Crs5Hpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Hpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Hpe4 {
    #[inline(always)]
    fn from(val: u8) -> Crs5Hpe4 {
        Crs5Hpe4::from_bits(val)
    }
}
impl From<Crs5Hpe4> for u8 {
    #[inline(always)]
    fn from(val: Crs5Hpe4) -> u8 {
        Crs5Hpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Hpe5 {
    #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
    Hpe5 = 0x0,
    #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
    Hpe51 = 0x01,
}
impl Crs5Hpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Hpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Hpe5 {
    #[inline(always)]
    fn from(val: u8) -> Crs5Hpe5 {
        Crs5Hpe5::from_bits(val)
    }
}
impl From<Crs5Hpe5> for u8 {
    #[inline(always)]
    fn from(val: Crs5Hpe5) -> u8 {
        Crs5Hpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Park {
    #[doc = "Park on master port M0"]
    MasterPort0 = 0x0,
    #[doc = "Park on master port M1"]
    MasterPort1 = 0x01,
    #[doc = "Park on master port M2"]
    MasterPort2 = 0x02,
    #[doc = "Park on master port M3"]
    MasterPort3 = 0x03,
    #[doc = "Park on master port M4"]
    MasterPort4 = 0x04,
    #[doc = "Park on master port M5"]
    MasterPort5 = 0x05,
    #[doc = "Park on master port M6"]
    MasterPort6 = 0x06,
    #[doc = "Park on master port M7"]
    MasterPort7 = 0x07,
}
impl Crs5Park {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Park {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Park {
    #[inline(always)]
    fn from(val: u8) -> Crs5Park {
        Crs5Park::from_bits(val)
    }
}
impl From<Crs5Park> for u8 {
    #[inline(always)]
    fn from(val: Crs5Park) -> u8 {
        Crs5Park::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Pctl {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
    Park = 0x0,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
    SlavePort = 0x01,
    #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
    SafeState = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs5Pctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Pctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Pctl {
    #[inline(always)]
    fn from(val: u8) -> Crs5Pctl {
        Crs5Pctl::from_bits(val)
    }
}
impl From<Crs5Pctl> for u8 {
    #[inline(always)]
    fn from(val: Crs5Pctl) -> u8 {
        Crs5Pctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs5Ro {
    #[doc = "The CRSn and PRSn registers are writeable"]
    CrsPrsY = 0x0,
    #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
    CrsPrsN = 0x01,
}
impl Crs5Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs5Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs5Ro {
    #[inline(always)]
    fn from(val: u8) -> Crs5Ro {
        Crs5Ro::from_bits(val)
    }
}
impl From<Crs5Ro> for u8 {
    #[inline(always)]
    fn from(val: Crs5Ro) -> u8 {
        Crs5Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Arb {
    #[doc = "Fixed priority"]
    Fp = 0x0,
    #[doc = "Round-robin (rotating) priority"]
    Rr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs6Arb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Arb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Arb {
    #[inline(always)]
    fn from(val: u8) -> Crs6Arb {
        Crs6Arb::from_bits(val)
    }
}
impl From<Crs6Arb> for u8 {
    #[inline(always)]
    fn from(val: Crs6Arb) -> u8 {
        Crs6Arb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Hpe0 {
    #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
    M0 = 0x0,
    #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
    M1 = 0x01,
}
impl Crs6Hpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Hpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Hpe0 {
    #[inline(always)]
    fn from(val: u8) -> Crs6Hpe0 {
        Crs6Hpe0::from_bits(val)
    }
}
impl From<Crs6Hpe0> for u8 {
    #[inline(always)]
    fn from(val: Crs6Hpe0) -> u8 {
        Crs6Hpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Hpe1 {
    #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
    Hpe1 = 0x0,
    #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
    Hpe11 = 0x01,
}
impl Crs6Hpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Hpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Hpe1 {
    #[inline(always)]
    fn from(val: u8) -> Crs6Hpe1 {
        Crs6Hpe1::from_bits(val)
    }
}
impl From<Crs6Hpe1> for u8 {
    #[inline(always)]
    fn from(val: Crs6Hpe1) -> u8 {
        Crs6Hpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Hpe2 {
    #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
    Hpe2 = 0x0,
    #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
    Hpe21 = 0x01,
}
impl Crs6Hpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Hpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Hpe2 {
    #[inline(always)]
    fn from(val: u8) -> Crs6Hpe2 {
        Crs6Hpe2::from_bits(val)
    }
}
impl From<Crs6Hpe2> for u8 {
    #[inline(always)]
    fn from(val: Crs6Hpe2) -> u8 {
        Crs6Hpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Hpe3 {
    #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
    Hpe3 = 0x0,
    #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
    Hpe31 = 0x01,
}
impl Crs6Hpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Hpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Hpe3 {
    #[inline(always)]
    fn from(val: u8) -> Crs6Hpe3 {
        Crs6Hpe3::from_bits(val)
    }
}
impl From<Crs6Hpe3> for u8 {
    #[inline(always)]
    fn from(val: Crs6Hpe3) -> u8 {
        Crs6Hpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Hpe4 {
    #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
    Hpe4 = 0x0,
    #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
    Hpe41 = 0x01,
}
impl Crs6Hpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Hpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Hpe4 {
    #[inline(always)]
    fn from(val: u8) -> Crs6Hpe4 {
        Crs6Hpe4::from_bits(val)
    }
}
impl From<Crs6Hpe4> for u8 {
    #[inline(always)]
    fn from(val: Crs6Hpe4) -> u8 {
        Crs6Hpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Hpe5 {
    #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
    Hpe5 = 0x0,
    #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
    Hpe51 = 0x01,
}
impl Crs6Hpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Hpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Hpe5 {
    #[inline(always)]
    fn from(val: u8) -> Crs6Hpe5 {
        Crs6Hpe5::from_bits(val)
    }
}
impl From<Crs6Hpe5> for u8 {
    #[inline(always)]
    fn from(val: Crs6Hpe5) -> u8 {
        Crs6Hpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Park {
    #[doc = "Park on master port M0"]
    MasterPort0 = 0x0,
    #[doc = "Park on master port M1"]
    MasterPort1 = 0x01,
    #[doc = "Park on master port M2"]
    MasterPort2 = 0x02,
    #[doc = "Park on master port M3"]
    MasterPort3 = 0x03,
    #[doc = "Park on master port M4"]
    MasterPort4 = 0x04,
    #[doc = "Park on master port M5"]
    MasterPort5 = 0x05,
    #[doc = "Park on master port M6"]
    MasterPort6 = 0x06,
    #[doc = "Park on master port M7"]
    MasterPort7 = 0x07,
}
impl Crs6Park {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Park {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Park {
    #[inline(always)]
    fn from(val: u8) -> Crs6Park {
        Crs6Park::from_bits(val)
    }
}
impl From<Crs6Park> for u8 {
    #[inline(always)]
    fn from(val: Crs6Park) -> u8 {
        Crs6Park::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Pctl {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
    Park = 0x0,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
    SlavePort = 0x01,
    #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
    SafeState = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs6Pctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Pctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Pctl {
    #[inline(always)]
    fn from(val: u8) -> Crs6Pctl {
        Crs6Pctl::from_bits(val)
    }
}
impl From<Crs6Pctl> for u8 {
    #[inline(always)]
    fn from(val: Crs6Pctl) -> u8 {
        Crs6Pctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs6Ro {
    #[doc = "The CRSn and PRSn registers are writeable"]
    CrsPrsY = 0x0,
    #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
    CrsPrsN = 0x01,
}
impl Crs6Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs6Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs6Ro {
    #[inline(always)]
    fn from(val: u8) -> Crs6Ro {
        Crs6Ro::from_bits(val)
    }
}
impl From<Crs6Ro> for u8 {
    #[inline(always)]
    fn from(val: Crs6Ro) -> u8 {
        Crs6Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Arb {
    #[doc = "Fixed priority"]
    Fp = 0x0,
    #[doc = "Round-robin (rotating) priority"]
    Rr = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs7Arb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Arb {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Arb {
    #[inline(always)]
    fn from(val: u8) -> Crs7Arb {
        Crs7Arb::from_bits(val)
    }
}
impl From<Crs7Arb> for u8 {
    #[inline(always)]
    fn from(val: Crs7Arb) -> u8 {
        Crs7Arb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Hpe0 {
    #[doc = "Master high-priority elevation for master 0. is disabled on this slave port."]
    M0 = 0x0,
    #[doc = "Master high-priority elevation for master 0. is enabled on this slave port."]
    M1 = 0x01,
}
impl Crs7Hpe0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Hpe0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Hpe0 {
    #[inline(always)]
    fn from(val: u8) -> Crs7Hpe0 {
        Crs7Hpe0::from_bits(val)
    }
}
impl From<Crs7Hpe0> for u8 {
    #[inline(always)]
    fn from(val: Crs7Hpe0) -> u8 {
        Crs7Hpe0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Hpe1 {
    #[doc = "Master high-priority elevation for master 1. is disabled on this slave port."]
    Hpe1 = 0x0,
    #[doc = "Master high-priority elevation for master 1. is enabled on this slave port."]
    Hpe11 = 0x01,
}
impl Crs7Hpe1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Hpe1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Hpe1 {
    #[inline(always)]
    fn from(val: u8) -> Crs7Hpe1 {
        Crs7Hpe1::from_bits(val)
    }
}
impl From<Crs7Hpe1> for u8 {
    #[inline(always)]
    fn from(val: Crs7Hpe1) -> u8 {
        Crs7Hpe1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Hpe2 {
    #[doc = "Master high-priority elevation for master 2. is disabled on this slave port."]
    Hpe2 = 0x0,
    #[doc = "Master high-priority elevation for master 2. is enabled on this slave port."]
    Hpe21 = 0x01,
}
impl Crs7Hpe2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Hpe2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Hpe2 {
    #[inline(always)]
    fn from(val: u8) -> Crs7Hpe2 {
        Crs7Hpe2::from_bits(val)
    }
}
impl From<Crs7Hpe2> for u8 {
    #[inline(always)]
    fn from(val: Crs7Hpe2) -> u8 {
        Crs7Hpe2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Hpe3 {
    #[doc = "Master high-priority elevation for master 3. is disabled on this slave port."]
    Hpe3 = 0x0,
    #[doc = "Master high-priority elevation for master 3. is enabled on this slave port."]
    Hpe31 = 0x01,
}
impl Crs7Hpe3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Hpe3 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Hpe3 {
    #[inline(always)]
    fn from(val: u8) -> Crs7Hpe3 {
        Crs7Hpe3::from_bits(val)
    }
}
impl From<Crs7Hpe3> for u8 {
    #[inline(always)]
    fn from(val: Crs7Hpe3) -> u8 {
        Crs7Hpe3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Hpe4 {
    #[doc = "Master high-priority elevation for master 4. is disabled on this slave port."]
    Hpe4 = 0x0,
    #[doc = "Master high-priority elevation for master 4. is enabled on this slave port."]
    Hpe41 = 0x01,
}
impl Crs7Hpe4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Hpe4 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Hpe4 {
    #[inline(always)]
    fn from(val: u8) -> Crs7Hpe4 {
        Crs7Hpe4::from_bits(val)
    }
}
impl From<Crs7Hpe4> for u8 {
    #[inline(always)]
    fn from(val: Crs7Hpe4) -> u8 {
        Crs7Hpe4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Hpe5 {
    #[doc = "Master high-priority elevation for master 5. is disabled on this slave port."]
    Hpe5 = 0x0,
    #[doc = "Master high-priority elevation for master 5. is enabled on this slave port."]
    Hpe51 = 0x01,
}
impl Crs7Hpe5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Hpe5 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Hpe5 {
    #[inline(always)]
    fn from(val: u8) -> Crs7Hpe5 {
        Crs7Hpe5::from_bits(val)
    }
}
impl From<Crs7Hpe5> for u8 {
    #[inline(always)]
    fn from(val: Crs7Hpe5) -> u8 {
        Crs7Hpe5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Park {
    #[doc = "Park on master port M0"]
    MasterPort0 = 0x0,
    #[doc = "Park on master port M1"]
    MasterPort1 = 0x01,
    #[doc = "Park on master port M2"]
    MasterPort2 = 0x02,
    #[doc = "Park on master port M3"]
    MasterPort3 = 0x03,
    #[doc = "Park on master port M4"]
    MasterPort4 = 0x04,
    #[doc = "Park on master port M5"]
    MasterPort5 = 0x05,
    #[doc = "Park on master port M6"]
    MasterPort6 = 0x06,
    #[doc = "Park on master port M7"]
    MasterPort7 = 0x07,
}
impl Crs7Park {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Park {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Park {
    #[inline(always)]
    fn from(val: u8) -> Crs7Park {
        Crs7Park::from_bits(val)
    }
}
impl From<Crs7Park> for u8 {
    #[inline(always)]
    fn from(val: Crs7Park) -> u8 {
        Crs7Park::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Pctl {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK bit field."]
    Park = 0x0,
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port."]
    SlavePort = 0x01,
    #[doc = "Low-power park. When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state."]
    SafeState = 0x02,
    _RESERVED_3 = 0x03,
}
impl Crs7Pctl {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Pctl {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Pctl {
    #[inline(always)]
    fn from(val: u8) -> Crs7Pctl {
        Crs7Pctl::from_bits(val)
    }
}
impl From<Crs7Pctl> for u8 {
    #[inline(always)]
    fn from(val: Crs7Pctl) -> u8 {
        Crs7Pctl::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Crs7Ro {
    #[doc = "The CRSn and PRSn registers are writeable"]
    CrsPrsY = 0x0,
    #[doc = "The CRSn and PRSn registers are read-only and cannot be written (attempted writes have no effect on the registers and result in a bus error response)."]
    CrsPrsN = 0x01,
}
impl Crs7Ro {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Crs7Ro {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Crs7Ro {
    #[inline(always)]
    fn from(val: u8) -> Crs7Ro {
        Crs7Ro::from_bits(val)
    }
}
impl From<Crs7Ro> for u8 {
    #[inline(always)]
    fn from(val: Crs7Ro) -> u8 {
        Crs7Ro::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs0M0 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs0M0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs0M0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs0M0 {
    #[inline(always)]
    fn from(val: u8) -> Prs0M0 {
        Prs0M0::from_bits(val)
    }
}
impl From<Prs0M0> for u8 {
    #[inline(always)]
    fn from(val: Prs0M0) -> u8 {
        Prs0M0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs0M1 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs0M1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs0M1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs0M1 {
    #[inline(always)]
    fn from(val: u8) -> Prs0M1 {
        Prs0M1::from_bits(val)
    }
}
impl From<Prs0M1> for u8 {
    #[inline(always)]
    fn from(val: Prs0M1) -> u8 {
        Prs0M1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs0M2 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs0M2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs0M2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs0M2 {
    #[inline(always)]
    fn from(val: u8) -> Prs0M2 {
        Prs0M2::from_bits(val)
    }
}
impl From<Prs0M2> for u8 {
    #[inline(always)]
    fn from(val: Prs0M2) -> u8 {
        Prs0M2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs0M3 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs0M3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs0M3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs0M3 {
    #[inline(always)]
    fn from(val: u8) -> Prs0M3 {
        Prs0M3::from_bits(val)
    }
}
impl From<Prs0M3> for u8 {
    #[inline(always)]
    fn from(val: Prs0M3) -> u8 {
        Prs0M3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs0M4 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs0M4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs0M4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs0M4 {
    #[inline(always)]
    fn from(val: u8) -> Prs0M4 {
        Prs0M4::from_bits(val)
    }
}
impl From<Prs0M4> for u8 {
    #[inline(always)]
    fn from(val: Prs0M4) -> u8 {
        Prs0M4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs0M5 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs0M5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs0M5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs0M5 {
    #[inline(always)]
    fn from(val: u8) -> Prs0M5 {
        Prs0M5::from_bits(val)
    }
}
impl From<Prs0M5> for u8 {
    #[inline(always)]
    fn from(val: Prs0M5) -> u8 {
        Prs0M5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs1M0 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs1M0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs1M0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs1M0 {
    #[inline(always)]
    fn from(val: u8) -> Prs1M0 {
        Prs1M0::from_bits(val)
    }
}
impl From<Prs1M0> for u8 {
    #[inline(always)]
    fn from(val: Prs1M0) -> u8 {
        Prs1M0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs1M1 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs1M1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs1M1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs1M1 {
    #[inline(always)]
    fn from(val: u8) -> Prs1M1 {
        Prs1M1::from_bits(val)
    }
}
impl From<Prs1M1> for u8 {
    #[inline(always)]
    fn from(val: Prs1M1) -> u8 {
        Prs1M1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs1M2 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs1M2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs1M2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs1M2 {
    #[inline(always)]
    fn from(val: u8) -> Prs1M2 {
        Prs1M2::from_bits(val)
    }
}
impl From<Prs1M2> for u8 {
    #[inline(always)]
    fn from(val: Prs1M2) -> u8 {
        Prs1M2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs1M3 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs1M3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs1M3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs1M3 {
    #[inline(always)]
    fn from(val: u8) -> Prs1M3 {
        Prs1M3::from_bits(val)
    }
}
impl From<Prs1M3> for u8 {
    #[inline(always)]
    fn from(val: Prs1M3) -> u8 {
        Prs1M3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs1M4 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs1M4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs1M4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs1M4 {
    #[inline(always)]
    fn from(val: u8) -> Prs1M4 {
        Prs1M4::from_bits(val)
    }
}
impl From<Prs1M4> for u8 {
    #[inline(always)]
    fn from(val: Prs1M4) -> u8 {
        Prs1M4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs1M5 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs1M5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs1M5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs1M5 {
    #[inline(always)]
    fn from(val: u8) -> Prs1M5 {
        Prs1M5::from_bits(val)
    }
}
impl From<Prs1M5> for u8 {
    #[inline(always)]
    fn from(val: Prs1M5) -> u8 {
        Prs1M5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs2M0 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs2M0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs2M0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs2M0 {
    #[inline(always)]
    fn from(val: u8) -> Prs2M0 {
        Prs2M0::from_bits(val)
    }
}
impl From<Prs2M0> for u8 {
    #[inline(always)]
    fn from(val: Prs2M0) -> u8 {
        Prs2M0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs2M1 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs2M1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs2M1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs2M1 {
    #[inline(always)]
    fn from(val: u8) -> Prs2M1 {
        Prs2M1::from_bits(val)
    }
}
impl From<Prs2M1> for u8 {
    #[inline(always)]
    fn from(val: Prs2M1) -> u8 {
        Prs2M1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs2M2 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs2M2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs2M2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs2M2 {
    #[inline(always)]
    fn from(val: u8) -> Prs2M2 {
        Prs2M2::from_bits(val)
    }
}
impl From<Prs2M2> for u8 {
    #[inline(always)]
    fn from(val: Prs2M2) -> u8 {
        Prs2M2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs2M3 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs2M3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs2M3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs2M3 {
    #[inline(always)]
    fn from(val: u8) -> Prs2M3 {
        Prs2M3::from_bits(val)
    }
}
impl From<Prs2M3> for u8 {
    #[inline(always)]
    fn from(val: Prs2M3) -> u8 {
        Prs2M3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs2M4 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs2M4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs2M4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs2M4 {
    #[inline(always)]
    fn from(val: u8) -> Prs2M4 {
        Prs2M4::from_bits(val)
    }
}
impl From<Prs2M4> for u8 {
    #[inline(always)]
    fn from(val: Prs2M4) -> u8 {
        Prs2M4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs2M5 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs2M5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs2M5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs2M5 {
    #[inline(always)]
    fn from(val: u8) -> Prs2M5 {
        Prs2M5::from_bits(val)
    }
}
impl From<Prs2M5> for u8 {
    #[inline(always)]
    fn from(val: Prs2M5) -> u8 {
        Prs2M5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs3M0 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs3M0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs3M0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs3M0 {
    #[inline(always)]
    fn from(val: u8) -> Prs3M0 {
        Prs3M0::from_bits(val)
    }
}
impl From<Prs3M0> for u8 {
    #[inline(always)]
    fn from(val: Prs3M0) -> u8 {
        Prs3M0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs3M1 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs3M1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs3M1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs3M1 {
    #[inline(always)]
    fn from(val: u8) -> Prs3M1 {
        Prs3M1::from_bits(val)
    }
}
impl From<Prs3M1> for u8 {
    #[inline(always)]
    fn from(val: Prs3M1) -> u8 {
        Prs3M1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs3M2 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs3M2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs3M2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs3M2 {
    #[inline(always)]
    fn from(val: u8) -> Prs3M2 {
        Prs3M2::from_bits(val)
    }
}
impl From<Prs3M2> for u8 {
    #[inline(always)]
    fn from(val: Prs3M2) -> u8 {
        Prs3M2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs3M3 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs3M3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs3M3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs3M3 {
    #[inline(always)]
    fn from(val: u8) -> Prs3M3 {
        Prs3M3::from_bits(val)
    }
}
impl From<Prs3M3> for u8 {
    #[inline(always)]
    fn from(val: Prs3M3) -> u8 {
        Prs3M3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs3M4 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs3M4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs3M4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs3M4 {
    #[inline(always)]
    fn from(val: u8) -> Prs3M4 {
        Prs3M4::from_bits(val)
    }
}
impl From<Prs3M4> for u8 {
    #[inline(always)]
    fn from(val: Prs3M4) -> u8 {
        Prs3M4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs3M5 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs3M5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs3M5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs3M5 {
    #[inline(always)]
    fn from(val: u8) -> Prs3M5 {
        Prs3M5::from_bits(val)
    }
}
impl From<Prs3M5> for u8 {
    #[inline(always)]
    fn from(val: Prs3M5) -> u8 {
        Prs3M5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs4M0 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs4M0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs4M0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs4M0 {
    #[inline(always)]
    fn from(val: u8) -> Prs4M0 {
        Prs4M0::from_bits(val)
    }
}
impl From<Prs4M0> for u8 {
    #[inline(always)]
    fn from(val: Prs4M0) -> u8 {
        Prs4M0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs4M1 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs4M1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs4M1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs4M1 {
    #[inline(always)]
    fn from(val: u8) -> Prs4M1 {
        Prs4M1::from_bits(val)
    }
}
impl From<Prs4M1> for u8 {
    #[inline(always)]
    fn from(val: Prs4M1) -> u8 {
        Prs4M1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs4M2 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs4M2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs4M2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs4M2 {
    #[inline(always)]
    fn from(val: u8) -> Prs4M2 {
        Prs4M2::from_bits(val)
    }
}
impl From<Prs4M2> for u8 {
    #[inline(always)]
    fn from(val: Prs4M2) -> u8 {
        Prs4M2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs4M3 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs4M3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs4M3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs4M3 {
    #[inline(always)]
    fn from(val: u8) -> Prs4M3 {
        Prs4M3::from_bits(val)
    }
}
impl From<Prs4M3> for u8 {
    #[inline(always)]
    fn from(val: Prs4M3) -> u8 {
        Prs4M3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs4M4 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs4M4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs4M4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs4M4 {
    #[inline(always)]
    fn from(val: u8) -> Prs4M4 {
        Prs4M4::from_bits(val)
    }
}
impl From<Prs4M4> for u8 {
    #[inline(always)]
    fn from(val: Prs4M4) -> u8 {
        Prs4M4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs4M5 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs4M5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs4M5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs4M5 {
    #[inline(always)]
    fn from(val: u8) -> Prs4M5 {
        Prs4M5::from_bits(val)
    }
}
impl From<Prs4M5> for u8 {
    #[inline(always)]
    fn from(val: Prs4M5) -> u8 {
        Prs4M5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs5M0 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs5M0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs5M0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs5M0 {
    #[inline(always)]
    fn from(val: u8) -> Prs5M0 {
        Prs5M0::from_bits(val)
    }
}
impl From<Prs5M0> for u8 {
    #[inline(always)]
    fn from(val: Prs5M0) -> u8 {
        Prs5M0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs5M1 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs5M1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs5M1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs5M1 {
    #[inline(always)]
    fn from(val: u8) -> Prs5M1 {
        Prs5M1::from_bits(val)
    }
}
impl From<Prs5M1> for u8 {
    #[inline(always)]
    fn from(val: Prs5M1) -> u8 {
        Prs5M1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs5M2 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs5M2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs5M2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs5M2 {
    #[inline(always)]
    fn from(val: u8) -> Prs5M2 {
        Prs5M2::from_bits(val)
    }
}
impl From<Prs5M2> for u8 {
    #[inline(always)]
    fn from(val: Prs5M2) -> u8 {
        Prs5M2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs5M3 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs5M3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs5M3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs5M3 {
    #[inline(always)]
    fn from(val: u8) -> Prs5M3 {
        Prs5M3::from_bits(val)
    }
}
impl From<Prs5M3> for u8 {
    #[inline(always)]
    fn from(val: Prs5M3) -> u8 {
        Prs5M3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs5M4 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs5M4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs5M4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs5M4 {
    #[inline(always)]
    fn from(val: u8) -> Prs5M4 {
        Prs5M4::from_bits(val)
    }
}
impl From<Prs5M4> for u8 {
    #[inline(always)]
    fn from(val: Prs5M4) -> u8 {
        Prs5M4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs5M5 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs5M5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs5M5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs5M5 {
    #[inline(always)]
    fn from(val: u8) -> Prs5M5 {
        Prs5M5::from_bits(val)
    }
}
impl From<Prs5M5> for u8 {
    #[inline(always)]
    fn from(val: Prs5M5) -> u8 {
        Prs5M5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs6M0 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs6M0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs6M0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs6M0 {
    #[inline(always)]
    fn from(val: u8) -> Prs6M0 {
        Prs6M0::from_bits(val)
    }
}
impl From<Prs6M0> for u8 {
    #[inline(always)]
    fn from(val: Prs6M0) -> u8 {
        Prs6M0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs6M1 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs6M1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs6M1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs6M1 {
    #[inline(always)]
    fn from(val: u8) -> Prs6M1 {
        Prs6M1::from_bits(val)
    }
}
impl From<Prs6M1> for u8 {
    #[inline(always)]
    fn from(val: Prs6M1) -> u8 {
        Prs6M1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs6M2 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs6M2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs6M2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs6M2 {
    #[inline(always)]
    fn from(val: u8) -> Prs6M2 {
        Prs6M2::from_bits(val)
    }
}
impl From<Prs6M2> for u8 {
    #[inline(always)]
    fn from(val: Prs6M2) -> u8 {
        Prs6M2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs6M3 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs6M3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs6M3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs6M3 {
    #[inline(always)]
    fn from(val: u8) -> Prs6M3 {
        Prs6M3::from_bits(val)
    }
}
impl From<Prs6M3> for u8 {
    #[inline(always)]
    fn from(val: Prs6M3) -> u8 {
        Prs6M3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs6M4 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs6M4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs6M4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs6M4 {
    #[inline(always)]
    fn from(val: u8) -> Prs6M4 {
        Prs6M4::from_bits(val)
    }
}
impl From<Prs6M4> for u8 {
    #[inline(always)]
    fn from(val: Prs6M4) -> u8 {
        Prs6M4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs6M5 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs6M5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs6M5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs6M5 {
    #[inline(always)]
    fn from(val: u8) -> Prs6M5 {
        Prs6M5::from_bits(val)
    }
}
impl From<Prs6M5> for u8 {
    #[inline(always)]
    fn from(val: Prs6M5) -> u8 {
        Prs6M5::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs7M0 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or the lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs7M0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs7M0 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs7M0 {
    #[inline(always)]
    fn from(val: u8) -> Prs7M0 {
        Prs7M0::from_bits(val)
    }
}
impl From<Prs7M0> for u8 {
    #[inline(always)]
    fn from(val: Prs7M0) -> u8 {
        Prs7M0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs7M1 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs7M1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs7M1 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs7M1 {
    #[inline(always)]
    fn from(val: u8) -> Prs7M1 {
        Prs7M1::from_bits(val)
    }
}
impl From<Prs7M1> for u8 {
    #[inline(always)]
    fn from(val: Prs7M1) -> u8 {
        Prs7M1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs7M2 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs7M2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs7M2 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs7M2 {
    #[inline(always)]
    fn from(val: u8) -> Prs7M2 {
        Prs7M2::from_bits(val)
    }
}
impl From<Prs7M2> for u8 {
    #[inline(always)]
    fn from(val: Prs7M2) -> u8 {
        Prs7M2::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs7M3 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8the or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs7M3 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs7M3 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs7M3 {
    #[inline(always)]
    fn from(val: u8) -> Prs7M3 {
        Prs7M3::from_bits(val)
    }
}
impl From<Prs7M3> for u8 {
    #[inline(always)]
    fn from(val: Prs7M3) -> u8 {
        Prs7M3::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs7M4 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs7M4 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs7M4 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs7M4 {
    #[inline(always)]
    fn from(val: u8) -> Prs7M4 {
        Prs7M4::from_bits(val)
    }
}
impl From<Prs7M4> for u8 {
    #[inline(always)]
    fn from(val: Prs7M4) -> u8 {
        Prs7M4::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prs7M5 {
    #[doc = "This master has level 1 or highest priority when accessing the slave port."]
    SlavePort1 = 0x0,
    #[doc = "This master has level 2 priority when accessing the slave port."]
    SlavePort2 = 0x01,
    #[doc = "This master has level 3 priority when accessing the slave port."]
    SlavePort3 = 0x02,
    #[doc = "This master has level 4 priority when accessing the slave port."]
    SlavePort4 = 0x03,
    #[doc = "This master has level 5 priority when accessing the slave port."]
    SlavePort5 = 0x04,
    #[doc = "This master has level 6 priority when accessing the slave port."]
    SlavePort6 = 0x05,
    #[doc = "This master has level 7 priority when accessing the slave port."]
    SlavePort7 = 0x06,
    #[doc = "This master has level 8 or lowest priority when accessing the slave port."]
    SlavePort8 = 0x07,
}
impl Prs7M5 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Prs7M5 {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Prs7M5 {
    #[inline(always)]
    fn from(val: u8) -> Prs7M5 {
        Prs7M5::from_bits(val)
    }
}
impl From<Prs7M5> for u8 {
    #[inline(always)]
    fn from(val: Prs7M5) -> u8 {
        Prs7M5::to_bits(val)
    }
}
