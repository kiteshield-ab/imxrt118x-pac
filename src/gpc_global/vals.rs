#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu0Master {
    #[doc = "CPU0 is not the master CPU of its domain"]
    B0 = 0x0,
    #[doc = "CPU0 is the master CPU of its domain"]
    B1 = 0x01,
}
impl Cpu0Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu0Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu0Master {
    #[inline(always)]
    fn from(val: u8) -> Cpu0Master {
        Cpu0Master::from_bits(val)
    }
}
impl From<Cpu0Master> for u8 {
    #[inline(always)]
    fn from(val: Cpu0Master) -> u8 {
        Cpu0Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cpu1Master {
    #[doc = "CPU1 is not the master CPU of its domain"]
    B0 = 0x0,
    #[doc = "CPU1 is the master CPU of its domain"]
    B1 = 0x01,
}
impl Cpu1Master {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cpu1Master {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cpu1Master {
    #[inline(always)]
    fn from(val: u8) -> Cpu1Master {
        Cpu1Master::from_bits(val)
    }
}
impl From<Cpu1Master> for u8 {
    #[inline(always)]
    fn from(val: Cpu1Master) -> u8 {
        Cpu1Master::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockCfg {
    #[doc = "The value of low power configuration fields are not locked."]
    B0 = 0x0,
    #[doc = "The value of low power configuration fields are locked. Refer to the function field of each gpc_global registers."]
    B1 = 0x01,
}
impl LockCfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockCfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockCfg {
    #[inline(always)]
    fn from(val: u8) -> LockCfg {
        LockCfg::from_bits(val)
    }
}
impl From<LockCfg> for u8 {
    #[inline(always)]
    fn from(val: LockCfg) -> u8 {
        LockCfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockList {
    #[doc = "WHITE_LIST is not locked"]
    B0 = 0x0,
    #[doc = "WHITE_LIST is locked"]
    B1 = 0x01,
}
impl LockList {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockList {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockList {
    #[inline(always)]
    fn from(val: u8) -> LockList {
        LockList::from_bits(val)
    }
}
impl From<LockList> for u8 {
    #[inline(always)]
    fn from(val: LockList) -> u8 {
        LockList::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LockSetting {
    #[doc = "NONSECURE and USER fields are not locked"]
    B0 = 0x0,
    #[doc = "NONSECURE and USER fields are locked"]
    B1 = 0x01,
}
impl LockSetting {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LockSetting {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LockSetting {
    #[inline(always)]
    fn from(val: u8) -> LockSetting {
        LockSetting::from_bits(val)
    }
}
impl From<LockSetting> for u8 {
    #[inline(always)]
    fn from(val: LockSetting) -> u8 {
        LockSetting::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Nonsecure {
    #[doc = "Allow only secure mode to access CPU mode registers"]
    B0 = 0x0,
    #[doc = "Allow both secure and non-secure mode to access CPU mode control registers."]
    B1 = 0x01,
}
impl Nonsecure {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Nonsecure {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Nonsecure {
    #[inline(always)]
    fn from(val: u8) -> Nonsecure {
        Nonsecure::from_bits(val)
    }
}
impl From<Nonsecure> for u8 {
    #[inline(always)]
    fn from(val: Nonsecure) -> u8 {
        Nonsecure::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RoscOffEn {
    #[doc = "Keep 24 MHz ROSC clock running during system sleep"]
    B0 = 0x0,
    #[doc = "Shut off 24 MHz ROSC clock during system sleep"]
    B1 = 0x01,
}
impl RoscOffEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RoscOffEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RoscOffEn {
    #[inline(always)]
    fn from(val: u8) -> RoscOffEn {
        RoscOffEn::from_bits(val)
    }
}
impl From<RoscOffEn> for u8 {
    #[inline(always)]
    fn from(val: RoscOffEn) -> u8 {
        RoscOffEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum User {
    #[doc = "Allow only privilege mode to access CPU mode control registers"]
    B0 = 0x0,
    #[doc = "Allow both privilege and user mode to access CPU mode control registers"]
    B1 = 0x01,
}
impl User {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> User {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for User {
    #[inline(always)]
    fn from(val: u8) -> User {
        User::from_bits(val)
    }
}
impl From<User> for u8 {
    #[inline(always)]
    fn from(val: User) -> u8 {
        User::to_bits(val)
    }
}
