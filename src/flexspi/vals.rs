#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Active {
    #[doc = "No suspended AHB read prefetch command."]
    VAL0 = 0x0,
    #[doc = "An AHB read prefetch command sequence has been suspended."]
    VAL1 = 0x01,
}
impl Active {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Active {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Active {
    #[inline(always)]
    fn from(val: u8) -> Active {
        Active::from_bits(val)
    }
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(val: Active) -> u8 {
        Active::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbbuserroren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ahbbuserroren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbbuserroren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbbuserroren {
    #[inline(always)]
    fn from(val: u8) -> Ahbbuserroren {
        Ahbbuserroren::from_bits(val)
    }
}
impl From<Ahbbuserroren> for u8 {
    #[inline(always)]
    fn from(val: Ahbbuserroren) -> u8 {
        Ahbbuserroren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmderrcode {
    #[doc = "No error"]
    VAL0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "AHB Write command with JMP_ON_CS instruction used in the sequence"]
    VAL2 = 0x02,
    #[doc = "Unknown instruction opcode in the sequence"]
    VAL3 = 0x03,
    #[doc = "DUMMY_SDR or DUMMY_RWDS_SDR instruction used in DDR sequence"]
    VAL4 = 0x04,
    #[doc = "DUMMY_DDR or DUMMY_RWDS_DDR instruction used in SDR sequence"]
    VAL5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout"]
    VAL6 = 0x0e,
    _RESERVED_f = 0x0f,
}
impl Ahbcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmderrcode {
        Ahbcmderrcode::from_bits(val)
    }
}
impl From<Ahbcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmderrcode) -> u8 {
        Ahbcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmderren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ahbcmderren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmderren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmderren {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmderren {
        Ahbcmderren::from_bits(val)
    }
}
impl From<Ahbcmderren> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmderren) -> u8 {
        Ahbcmderren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbcmdgeen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ahbcmdgeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbcmdgeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbcmdgeen {
    #[inline(always)]
    fn from(val: u8) -> Ahbcmdgeen {
        Ahbcmdgeen::from_bits(val)
    }
}
impl From<Ahbcmdgeen> for u8 {
    #[inline(always)]
    fn from(val: Ahbcmdgeen) -> u8 {
        Ahbcmdgeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AhbcrPrefetchen {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl AhbcrPrefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> AhbcrPrefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for AhbcrPrefetchen {
    #[inline(always)]
    fn from(val: u8) -> AhbcrPrefetchen {
        AhbcrPrefetchen::from_bits(val)
    }
}
impl From<AhbcrPrefetchen> for u8 {
    #[inline(always)]
    fn from(val: AhbcrPrefetchen) -> u8 {
        AhbcrPrefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf0cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf0cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf0cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf0cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf0cr0Prefetchen {
        Ahbrxbuf0cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf0cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf0cr0Prefetchen) -> u8 {
        Ahbrxbuf0cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf0cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf0cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf0cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf0cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf0cr0Regionen {
        Ahbrxbuf0cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf0cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf0cr0Regionen) -> u8 {
        Ahbrxbuf0cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf1cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf1cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf1cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf1cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf1cr0Prefetchen {
        Ahbrxbuf1cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf1cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf1cr0Prefetchen) -> u8 {
        Ahbrxbuf1cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf1cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf1cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf1cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf1cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf1cr0Regionen {
        Ahbrxbuf1cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf1cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf1cr0Regionen) -> u8 {
        Ahbrxbuf1cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf2cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf2cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf2cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf2cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf2cr0Prefetchen {
        Ahbrxbuf2cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf2cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf2cr0Prefetchen) -> u8 {
        Ahbrxbuf2cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf2cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf2cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf2cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf2cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf2cr0Regionen {
        Ahbrxbuf2cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf2cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf2cr0Regionen) -> u8 {
        Ahbrxbuf2cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf3cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf3cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf3cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf3cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf3cr0Prefetchen {
        Ahbrxbuf3cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf3cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf3cr0Prefetchen) -> u8 {
        Ahbrxbuf3cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf3cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf3cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf3cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf3cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf3cr0Regionen {
        Ahbrxbuf3cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf3cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf3cr0Regionen) -> u8 {
        Ahbrxbuf3cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf4cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf4cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf4cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf4cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf4cr0Prefetchen {
        Ahbrxbuf4cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf4cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf4cr0Prefetchen) -> u8 {
        Ahbrxbuf4cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf4cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf4cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf4cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf4cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf4cr0Regionen {
        Ahbrxbuf4cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf4cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf4cr0Regionen) -> u8 {
        Ahbrxbuf4cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf5cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf5cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf5cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf5cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf5cr0Prefetchen {
        Ahbrxbuf5cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf5cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf5cr0Prefetchen) -> u8 {
        Ahbrxbuf5cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf5cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf5cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf5cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf5cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf5cr0Regionen {
        Ahbrxbuf5cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf5cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf5cr0Regionen) -> u8 {
        Ahbrxbuf5cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf6cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf6cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf6cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf6cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf6cr0Prefetchen {
        Ahbrxbuf6cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf6cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf6cr0Prefetchen) -> u8 {
        Ahbrxbuf6cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf6cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf6cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf6cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf6cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf6cr0Regionen {
        Ahbrxbuf6cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf6cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf6cr0Regionen) -> u8 {
        Ahbrxbuf6cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf7cr0Prefetchen {
    #[doc = "Disabled"]
    VALUE0 = 0x0,
    #[doc = "Enabled when is enabled."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf7cr0Prefetchen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf7cr0Prefetchen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf7cr0Prefetchen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf7cr0Prefetchen {
        Ahbrxbuf7cr0Prefetchen::from_bits(val)
    }
}
impl From<Ahbrxbuf7cr0Prefetchen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf7cr0Prefetchen) -> u8 {
        Ahbrxbuf7cr0Prefetchen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ahbrxbuf7cr0Regionen {
    #[doc = "Disabled. The buffer hit is based on the value of MSTRID only."]
    VALUE0 = 0x0,
    #[doc = "Enabled. The buffer hit is based on the value of MSTRID and the address within AHBBUFREGIONSTARTn and AHBREGIONENDn."]
    VALUE1 = 0x01,
}
impl Ahbrxbuf7cr0Regionen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ahbrxbuf7cr0Regionen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ahbrxbuf7cr0Regionen {
    #[inline(always)]
    fn from(val: u8) -> Ahbrxbuf7cr0Regionen {
        Ahbrxbuf7cr0Regionen::from_bits(val)
    }
}
impl From<Ahbrxbuf7cr0Regionen> for u8 {
    #[inline(always)]
    fn from(val: Ahbrxbuf7cr0Regionen) -> u8 {
        Ahbrxbuf7cr0Regionen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Alignment {
    #[doc = "No limit"]
    BIT0 = 0x0,
    #[doc = "1 KB"]
    BIT1 = 0x01,
    #[doc = "512 bytes"]
    BIT2 = 0x02,
    #[doc = "256 bytes"]
    BIT3 = 0x03,
}
impl Alignment {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Alignment {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Alignment {
    #[inline(always)]
    fn from(val: u8) -> Alignment {
        Alignment::from_bits(val)
    }
}
impl From<Alignment> for u8 {
    #[inline(always)]
    fn from(val: Alignment) -> u8 {
        Alignment::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aparen {
    #[doc = "Flash is accessed in Individual mode."]
    INDIVIDUAL = 0x0,
    #[doc = "Flash is accessed in Parallel mode."]
    ENABLE = 0x01,
}
impl Aparen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aparen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aparen {
    #[inline(always)]
    fn from(val: u8) -> Aparen {
        Aparen::from_bits(val)
    }
}
impl From<Aparen> for u8 {
    #[inline(always)]
    fn from(val: Aparen) -> u8 {
        Aparen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arbcmdsrc {
    #[doc = "Trigger source is AHB read command."]
    VAL0 = 0x0,
    #[doc = "Trigger source is AHB write command."]
    VAL1 = 0x01,
    #[doc = "Trigger source is IP command (by writing 1 to IPCMD\\[TRG\\])."]
    VAL2 = 0x02,
    #[doc = "Trigger source is a suspended command that has resumed."]
    VAL3 = 0x03,
}
impl Arbcmdsrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arbcmdsrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arbcmdsrc {
    #[inline(always)]
    fn from(val: u8) -> Arbcmdsrc {
        Arbcmdsrc::from_bits(val)
    }
}
impl From<Arbcmdsrc> for u8 {
    #[inline(always)]
    fn from(val: Arbcmdsrc) -> u8 {
        Arbcmdsrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Arbidle {
    #[doc = "Not idle"]
    VALUE0 = 0x0,
    #[doc = "Idle"]
    VALUE1 = 0x01,
}
impl Arbidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Arbidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Arbidle {
    #[inline(always)]
    fn from(val: u8) -> Arbidle {
        Arbidle::from_bits(val)
    }
}
impl From<Arbidle> for u8 {
    #[inline(always)]
    fn from(val: Arbidle) -> u8 {
        Arbidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ardfen {
    #[doc = "AHB read access disabled. IP bus reads IP receive FIFO. AHB Bus read access to IP receive FIFO memory space produces bus error."]
    VAL0 = 0x0,
    #[doc = "AHB read access enabled. AHB bus reads IP receive FIFO. IP Bus read access to IP receive FIFO memory space returns data zero and causes no bus error."]
    VAL1 = 0x01,
}
impl Ardfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ardfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ardfen {
    #[inline(always)]
    fn from(val: u8) -> Ardfen {
        Ardfen::from_bits(val)
    }
}
impl From<Ardfen> for u8 {
    #[inline(always)]
    fn from(val: Ardfen) -> u8 {
        Ardfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Areflock {
    #[doc = "Not locked"]
    VAL0 = 0x0,
    #[doc = "Locked"]
    VAL1 = 0x01,
}
impl Areflock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Areflock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Areflock {
    #[inline(always)]
    fn from(val: u8) -> Areflock {
        Areflock::from_bits(val)
    }
}
impl From<Areflock> for u8 {
    #[inline(always)]
    fn from(val: Areflock) -> u8 {
        Areflock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Aslvlock {
    #[doc = "Not locked"]
    VAL0 = 0x0,
    #[doc = "Locked"]
    VAL1 = 0x01,
}
impl Aslvlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Aslvlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Aslvlock {
    #[inline(always)]
    fn from(val: u8) -> Aslvlock {
        Aslvlock::from_bits(val)
    }
}
impl From<Aslvlock> for u8 {
    #[inline(always)]
    fn from(val: Aslvlock) -> u8 {
        Aslvlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Atdfen {
    #[doc = "AHB write access disabled. IP bus writes to IP transmit FIFO. AHB bus write access to IP transmit FIFO memory space produces bus error."]
    VAL0 = 0x0,
    #[doc = "AHB write access enabled. AHB bus writes to IP transmit FIFO. IP Bus write access to IP transmit FIFO memory space is ignored and causes no bus error."]
    VAL1 = 0x01,
}
impl Atdfen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Atdfen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Atdfen {
    #[inline(always)]
    fn from(val: u8) -> Atdfen {
        Atdfen::from_bits(val)
    }
}
impl From<Atdfen> for u8 {
    #[inline(always)]
    fn from(val: Atdfen) -> u8 {
        Atdfen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Awrwaitunit {
    #[doc = "2"]
    VAL0 = 0x0,
    #[doc = "8"]
    VAL1 = 0x01,
    #[doc = "32"]
    VAL2 = 0x02,
    #[doc = "128"]
    VAL3 = 0x03,
    #[doc = "512"]
    VAL4 = 0x04,
    #[doc = "2048"]
    VAL5 = 0x05,
    #[doc = "8192"]
    VAL6 = 0x06,
    #[doc = "32768"]
    VAL7 = 0x07,
}
impl Awrwaitunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Awrwaitunit {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Awrwaitunit {
    #[inline(always)]
    fn from(val: u8) -> Awrwaitunit {
        Awrwaitunit::from_bits(val)
    }
}
impl From<Awrwaitunit> for u8 {
    #[inline(always)]
    fn from(val: Awrwaitunit) -> u8 {
        Awrwaitunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Breflock {
    #[doc = "Not locked"]
    VAL0 = 0x0,
    #[doc = "Locked"]
    VAL1 = 0x01,
}
impl Breflock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Breflock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Breflock {
    #[inline(always)]
    fn from(val: u8) -> Breflock {
        Breflock::from_bits(val)
    }
}
impl From<Breflock> for u8 {
    #[inline(always)]
    fn from(val: Breflock) -> u8 {
        Breflock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bslvlock {
    #[doc = "Not locked"]
    VAL0 = 0x0,
    #[doc = "Locked"]
    VAL1 = 0x01,
}
impl Bslvlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bslvlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bslvlock {
    #[inline(always)]
    fn from(val: u8) -> Bslvlock {
        Bslvlock::from_bits(val)
    }
}
impl From<Bslvlock> for u8 {
    #[inline(always)]
    fn from(val: Bslvlock) -> u8 {
        Bslvlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bufferableen {
    #[doc = "Disabled. For all AHB write accesses (bufferable or nonbufferable), FlexSPI returns AHB Bus Ready after transmitting all data and finishing command."]
    VAL0 = 0x0,
    #[doc = "Enabled. For AHB bufferable write access, FlexSPI returns AHB Bus Ready when the arbitrator grants the AHB command. FlexSPI does not wait for the AHB command to finish."]
    VAL1 = 0x01,
}
impl Bufferableen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bufferableen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bufferableen {
    #[inline(always)]
    fn from(val: u8) -> Bufferableen {
        Bufferableen::from_bits(val)
    }
}
impl From<Bufferableen> for u8 {
    #[inline(always)]
    fn from(val: Bufferableen) -> u8 {
        Bufferableen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Cachableen {
    #[doc = "Disabled. When an AHB bus cacheable read access occurs, FlexSPI does not check whether it hit the AHB transmit buffer."]
    VAL0 = 0x0,
    #[doc = "Enabled. When an AHB bus cacheable read access occurs, FlexSPI first checks whether the access hit the AHB transmit buffer."]
    VAL1 = 0x01,
}
impl Cachableen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cachableen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cachableen {
    #[inline(always)]
    fn from(val: u8) -> Cachableen {
        Cachableen::from_bits(val)
    }
}
impl From<Cachableen> for u8 {
    #[inline(always)]
    fn from(val: Cachableen) -> u8 {
        Cachableen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbbufopt {
    #[doc = "Not cleared automatically"]
    VAL0 = 0x0,
    #[doc = "Cleared automatically"]
    VAL1 = 0x01,
}
impl Clrahbbufopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbbufopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbbufopt {
    #[inline(always)]
    fn from(val: u8) -> Clrahbbufopt {
        Clrahbbufopt::from_bits(val)
    }
}
impl From<Clrahbbufopt> for u8 {
    #[inline(always)]
    fn from(val: Clrahbbufopt) -> u8 {
        Clrahbbufopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clrahbtxbuf {
    #[doc = "No impact."]
    VAL0 = 0x0,
    #[doc = "Enable clear operation."]
    VAL1 = 0x01,
}
impl Clrahbtxbuf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clrahbtxbuf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clrahbtxbuf {
    #[inline(always)]
    fn from(val: u8) -> Clrahbtxbuf {
        Clrahbtxbuf::from_bits(val)
    }
}
impl From<Clrahbtxbuf> for u8 {
    #[inline(always)]
    fn from(val: Clrahbtxbuf) -> u8 {
        Clrahbtxbuf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clriprxf {
    #[doc = "No function"]
    VALUE0 = 0x0,
    #[doc = "A clock cycle pulse clears all valid data entries in IP receive FIFO."]
    VALUE1 = 0x01,
}
impl Clriprxf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clriprxf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clriprxf {
    #[inline(always)]
    fn from(val: u8) -> Clriprxf {
        Clriprxf::from_bits(val)
    }
}
impl From<Clriprxf> for u8 {
    #[inline(always)]
    fn from(val: Clriprxf) -> u8 {
        Clriprxf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clriptxf {
    #[doc = "No function"]
    VALUE0 = 0x0,
    #[doc = "A clock cycle pulse clears all valid data entries in the IP transmit FIFO."]
    VALUE1 = 0x01,
}
impl Clriptxf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clriptxf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clriptxf {
    #[inline(always)]
    fn from(val: u8) -> Clriptxf {
        Clriptxf::from_bits(val)
    }
}
impl From<Clriptxf> for u8 {
    #[inline(always)]
    fn from(val: Clriptxf) -> u8 {
        Clriptxf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Csintervalunit {
    #[doc = "1 serial clock cycle"]
    VAL0 = 0x0,
    #[doc = "256 serial clock cycles"]
    VAL1 = 0x01,
}
impl Csintervalunit {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Csintervalunit {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Csintervalunit {
    #[inline(always)]
    fn from(val: u8) -> Csintervalunit {
        Csintervalunit::from_bits(val)
    }
}
impl From<Csintervalunit> for u8 {
    #[inline(always)]
    fn from(val: Csintervalunit) -> u8 {
        Csintervalunit::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dllen {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
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
pub enum Dllreset {
    #[doc = "No function"]
    VALUE0 = 0x0,
    #[doc = "Force DLL reset."]
    VALUE1 = 0x01,
}
impl Dllreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dllreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dllreset {
    #[inline(always)]
    fn from(val: u8) -> Dllreset {
        Dllreset::from_bits(val)
    }
}
impl From<Dllreset> for u8 {
    #[inline(always)]
    fn from(val: Dllreset) -> u8 {
        Dllreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Dozeen {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl Dozeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Dozeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Dozeen {
    #[inline(always)]
    fn from(val: u8) -> Dozeen {
        Dozeen::from_bits(val)
    }
}
impl From<Dozeen> for u8 {
    #[inline(always)]
    fn from(val: Dozeen) -> u8 {
        Dozeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hsen {
    #[doc = "Disable"]
    VAL0 = 0x0,
    #[doc = "Enable"]
    VAL1 = 0x01,
}
impl Hsen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hsen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hsen {
    #[inline(always)]
    fn from(val: u8) -> Hsen {
        Hsen::from_bits(val)
    }
}
impl From<Hsen> for u8 {
    #[inline(always)]
    fn from(val: Hsen) -> u8 {
        Hsen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmddoneen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
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
pub enum Ipcmderrcode {
    #[doc = "No error"]
    VAL0 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "IP command with JMP_ON_CS instruction used in the sequence"]
    VAL2 = 0x02,
    #[doc = "Unknown instruction opcode in the sequence"]
    VAL3 = 0x03,
    #[doc = "DUMMY_SDR or DUMMY_RWDS_SDR instruction used in DDR sequence"]
    VAL4 = 0x04,
    #[doc = "DUMMY_DDR or DUMMY_RWDS_DDR instruction used in SDR sequence"]
    VAL5 = 0x05,
    #[doc = "Flash memory access start address exceeds entire flash address range (A1, A2, B1, and B2)"]
    VAL6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    #[doc = "Sequence execution timeout"]
    VAL7 = 0x0e,
    #[doc = "Flash boundary crossed"]
    VAL8 = 0x0f,
}
impl Ipcmderrcode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmderrcode {
        unsafe { core::mem::transmute(val & 0x0f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmderrcode {
    #[inline(always)]
    fn from(val: u8) -> Ipcmderrcode {
        Ipcmderrcode::from_bits(val)
    }
}
impl From<Ipcmderrcode> for u8 {
    #[inline(always)]
    fn from(val: Ipcmderrcode) -> u8 {
        Ipcmderrcode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ipcmderren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
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
pub enum Ipcmdgeen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Ipcmdgeen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ipcmdgeen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ipcmdgeen {
    #[inline(always)]
    fn from(val: u8) -> Ipcmdgeen {
        Ipcmdgeen::from_bits(val)
    }
}
impl From<Ipcmdgeen> for u8 {
    #[inline(always)]
    fn from(val: Ipcmdgeen) -> u8 {
        Ipcmdgeen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iprxwaen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Iprxwaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iprxwaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iprxwaen {
    #[inline(always)]
    fn from(val: u8) -> Iprxwaen {
        Iprxwaen::from_bits(val)
    }
}
impl From<Iprxwaen> for u8 {
    #[inline(always)]
    fn from(val: Iprxwaen) -> u8 {
        Iprxwaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Iptxween {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Iptxween {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Iptxween {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Iptxween {
    #[inline(always)]
    fn from(val: u8) -> Iptxween {
        Iptxween::from_bits(val)
    }
}
impl From<Iptxween> for u8 {
    #[inline(always)]
    fn from(val: Iptxween) -> u8 {
        Iptxween::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Keydoneen {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Keydoneen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Keydoneen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Keydoneen {
    #[inline(always)]
    fn from(val: u8) -> Keydoneen {
        Keydoneen::from_bits(val)
    }
}
impl From<Keydoneen> for u8 {
    #[inline(always)]
    fn from(val: Keydoneen) -> u8 {
        Keydoneen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Keyerroren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Keyerroren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Keyerroren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Keyerroren {
    #[inline(always)]
    fn from(val: u8) -> Keyerroren {
        Keyerroren::from_bits(val)
    }
}
impl From<Keyerroren> for u8 {
    #[inline(always)]
    fn from(val: Keyerroren) -> u8 {
        Keyerroren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lock {
    #[doc = "LUT is unlocked (LUTCR\\[UNLOCK\\] must be 1)"]
    VALUE0 = 0x0,
    #[doc = "LUT is locked and cannot be written"]
    VALUE1 = 0x01,
}
impl Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lock {
    #[inline(always)]
    fn from(val: u8) -> Lock {
        Lock::from_bits(val)
    }
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(val: Lock) -> u8 {
        Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mdis {
    #[doc = "No impact"]
    VAL0 = 0x0,
    #[doc = "Module disable"]
    VAL1 = 0x01,
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
pub enum Ovrden {
    #[doc = "Disable"]
    VALUE0 = 0x0,
    #[doc = "Enable"]
    VALUE1 = 0x01,
}
impl Ovrden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ovrden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ovrden {
    #[inline(always)]
    fn from(val: u8) -> Ovrden {
        Ovrden::from_bits(val)
    }
}
impl From<Ovrden> for u8 {
    #[inline(always)]
    fn from(val: Ovrden) -> u8 {
        Ovrden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Readaddropt {
    #[doc = "AHB read burst start address alignment is limited when flash memory is accessed in parallel mode or flash is word-addressable."]
    VAL0 = 0x0,
    #[doc = "AHB read burst start address alignment is not limited. FlexSPI fetches more data than the AHB burst requires for address alignment."]
    VAL1 = 0x01,
}
impl Readaddropt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readaddropt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readaddropt {
    #[inline(always)]
    fn from(val: u8) -> Readaddropt {
        Readaddropt::from_bits(val)
    }
}
impl From<Readaddropt> for u8 {
    #[inline(always)]
    fn from(val: Readaddropt) -> u8 {
        Readaddropt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Readszalign {
    #[doc = "Register settings such as PREFETCH_EN and OTFAD_EN determine AHB read size."]
    VAL0 = 0x0,
    #[doc = "AHB read size to up size to 8 bytes aligned, no prefetching"]
    VAL1 = 0x01,
}
impl Readszalign {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Readszalign {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Readszalign {
    #[inline(always)]
    fn from(val: u8) -> Readszalign {
        Readszalign::from_bits(val)
    }
}
impl From<Readszalign> for u8 {
    #[inline(always)]
    fn from(val: Readszalign) -> u8 {
        Readszalign::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Resumedisable {
    #[doc = "Suspended AHB read prefetch resumes when AHB is IDLE."]
    VAL0 = 0x0,
    #[doc = "Suspended AHB read prefetch does not resume once aborted."]
    VAL1 = 0x01,
}
impl Resumedisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resumedisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resumedisable {
    #[inline(always)]
    fn from(val: u8) -> Resumedisable {
        Resumedisable::from_bits(val)
    }
}
impl From<Resumedisable> for u8 {
    #[inline(always)]
    fn from(val: Resumedisable) -> u8 {
        Resumedisable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxClkSrcDiff {
    #[doc = "Use MCR0\\[RXCLKSRC\\] for Port A and Port B. MCR2\\[RXCLKSRC_B\\] is ignored and MCR0\\[RXCLKSRC\\] selects the Sample Clock source for Flash Reading of both ports A and B."]
    VALUE0 = 0x0,
    #[doc = "Use MCR0\\[RXCLKSRC\\] for Port A, and MCR2\\[RXCLKSRC_B\\] for Port B. MCR0\\[RXCLKSRC\\] selects the Sample Clock source for Flash Reading of port A (A_SCLK) and MCR2\\[RXCLKSRC_B\\] selects the Sample Clock source for Flash Reading of port B (B_SCLK)."]
    VALUE1 = 0x01,
}
impl RxClkSrcDiff {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxClkSrcDiff {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxClkSrcDiff {
    #[inline(always)]
    fn from(val: u8) -> RxClkSrcDiff {
        RxClkSrcDiff::from_bits(val)
    }
}
impl From<RxClkSrcDiff> for u8 {
    #[inline(always)]
    fn from(val: RxClkSrcDiff) -> u8 {
        RxClkSrcDiff::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxclksrc {
    #[doc = "Dummy Read strobe that FlexSPI generates, looped back internally"]
    VAL0 = 0x0,
    #[doc = "Dummy Read strobe that FlexSPI generates, looped back from DQS pad"]
    VAL1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Flash-memory-provided read strobe and input from DQS pad"]
    VAL3 = 0x03,
}
impl Rxclksrc {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxclksrc {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxclksrc {
    #[inline(always)]
    fn from(val: u8) -> Rxclksrc {
        Rxclksrc::from_bits(val)
    }
}
impl From<Rxclksrc> for u8 {
    #[inline(always)]
    fn from(val: Rxclksrc) -> u8 {
        Rxclksrc::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxclksrcB {
    #[doc = "Dummy read strobe that FlexSPI generates, looped back internally."]
    VAL0 = 0x0,
    #[doc = "Dummy read strobe that FlexSPI generates, looped back from DQS pad."]
    VAL1 = 0x01,
    #[doc = "SCLK output clock and looped back from SCLK padReserved"]
    VAL2 = 0x02,
    #[doc = "Flash-memory-provided read strobe and input from DQS pad"]
    VAL3 = 0x03,
}
impl RxclksrcB {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RxclksrcB {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RxclksrcB {
    #[inline(always)]
    fn from(val: u8) -> RxclksrcB {
        RxclksrcB::from_bits(val)
    }
}
impl From<RxclksrcB> for u8 {
    #[inline(always)]
    fn from(val: RxclksrcB) -> u8 {
        RxclksrcB::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rxdmaen {
    #[doc = "Disabled. The processor reads the FIFO."]
    VAL0 = 0x0,
    #[doc = "Enabled. DMA reads the FIFO."]
    VAL1 = 0x01,
}
impl Rxdmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rxdmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rxdmaen {
    #[inline(always)]
    fn from(val: u8) -> Rxdmaen {
        Rxdmaen::from_bits(val)
    }
}
impl From<Rxdmaen> for u8 {
    #[inline(always)]
    fn from(val: Rxdmaen) -> u8 {
        Rxdmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Samedeviceen {
    #[doc = "In Individual mode, FLSHA1CRx and FLSHA2CRx, FLSHB1CRx and FLSHB2CRx settings are applied to Flash A1, A2, B1, B2 separately. In Parallel mode, FLSHA1CRx register setting is applied to Flash A1 and B1, FLSHA2CRx register setting is applied to Flash A2 and B2. FLSHB1CRx and FLSHB2CRx register settings are ignored."]
    INDIVIDUAL_PARALLEL = 0x0,
    #[doc = "FLSHA1CR0, FLSHA1CR1, and FLSHA1CR2 register settings are applied to Flash A1, A2, B1, B2. FLSHA2CRx, FLSHB1CRx, and FLSHB2CRx settings are ignored."]
    ENABLE = 0x01,
}
impl Samedeviceen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Samedeviceen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Samedeviceen {
    #[inline(always)]
    fn from(val: u8) -> Samedeviceen {
        Samedeviceen::from_bits(val)
    }
}
impl From<Samedeviceen> for u8 {
    #[inline(always)]
    fn from(val: Samedeviceen) -> u8 {
        Samedeviceen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckbdiffopt {
    #[doc = "Use B_SCLK pad as port B SCLK clock output. Port B flash memory access is available."]
    VAL1 = 0x0,
    #[doc = "Use B_SCLK pad as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash memory access is not available."]
    VAL0 = 0x01,
}
impl Sckbdiffopt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckbdiffopt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckbdiffopt {
    #[inline(always)]
    fn from(val: u8) -> Sckbdiffopt {
        Sckbdiffopt::from_bits(val)
    }
}
impl From<Sckbdiffopt> for u8 {
    #[inline(always)]
    fn from(val: Sckbdiffopt) -> u8 {
        Sckbdiffopt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckstopbyrden {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Sckstopbyrden {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckstopbyrden {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckstopbyrden {
    #[inline(always)]
    fn from(val: u8) -> Sckstopbyrden {
        Sckstopbyrden::from_bits(val)
    }
}
impl From<Sckstopbyrden> for u8 {
    #[inline(always)]
    fn from(val: Sckstopbyrden) -> u8 {
        Sckstopbyrden::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sckstopbywren {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Sckstopbywren {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sckstopbywren {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sckstopbywren {
    #[inline(always)]
    fn from(val: u8) -> Sckstopbywren {
        Sckstopbywren::from_bits(val)
    }
}
impl From<Sckstopbywren> for u8 {
    #[inline(always)]
    fn from(val: Sckstopbywren) -> u8 {
        Sckstopbywren::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seqidle {
    #[doc = "Not idle"]
    VALUE0 = 0x0,
    #[doc = "Idle"]
    VALUE1 = 0x01,
}
impl Seqidle {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seqidle {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seqidle {
    #[inline(always)]
    fn from(val: u8) -> Seqidle {
        Seqidle::from_bits(val)
    }
}
impl From<Seqidle> for u8 {
    #[inline(always)]
    fn from(val: Seqidle) -> u8 {
        Seqidle::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Seqtimeouten {
    #[doc = "Disable interrupt or no impact"]
    VALUE0 = 0x0,
    #[doc = "Enable interrupt"]
    VALUE1 = 0x01,
}
impl Seqtimeouten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Seqtimeouten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Seqtimeouten {
    #[inline(always)]
    fn from(val: u8) -> Seqtimeouten {
        Seqtimeouten::from_bits(val)
    }
}
impl From<Seqtimeouten> for u8 {
    #[inline(always)]
    fn from(val: Seqtimeouten) -> u8 {
        Seqtimeouten::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Serclkdiv {
    #[doc = "Divided by 1"]
    VAL0 = 0x0,
    #[doc = "Divided by 2"]
    VAL1 = 0x01,
    #[doc = "Divided by 3"]
    VAL2 = 0x02,
    #[doc = "Divided by 4"]
    VAL3 = 0x03,
    #[doc = "Divided by 5"]
    VAL4 = 0x04,
    #[doc = "Divided by 6"]
    VAL5 = 0x05,
    #[doc = "Divided by 7"]
    VAL6 = 0x06,
    #[doc = "Divided by 8"]
    VAL7 = 0x07,
}
impl Serclkdiv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Serclkdiv {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Serclkdiv {
    #[inline(always)]
    fn from(val: u8) -> Serclkdiv {
        Serclkdiv::from_bits(val)
    }
}
impl From<Serclkdiv> for u8 {
    #[inline(always)]
    fn from(val: Serclkdiv) -> u8 {
        Serclkdiv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Swreset {
    #[doc = "No impact"]
    VAL0 = 0x0,
    #[doc = "Software reset"]
    VAL1 = 0x01,
}
impl Swreset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Swreset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Swreset {
    #[inline(always)]
    fn from(val: u8) -> Swreset {
        Swreset::from_bits(val)
    }
}
impl From<Swreset> for u8 {
    #[inline(always)]
    fn from(val: Swreset) -> u8 {
        Swreset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg {
    #[doc = "No action"]
    VALUE0 = 0x0,
    #[doc = "Start the IP command that the IPCR0 and IPCR1 registers define."]
    VALUE1 = 0x01,
}
impl Trg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg {
    #[inline(always)]
    fn from(val: u8) -> Trg {
        Trg::from_bits(val)
    }
}
impl From<Trg> for u8 {
    #[inline(always)]
    fn from(val: Trg) -> u8 {
        Trg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Txdmaen {
    #[doc = "Processor"]
    VAL0 = 0x0,
    #[doc = "DMA"]
    VAL1 = 0x01,
}
impl Txdmaen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Txdmaen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Txdmaen {
    #[inline(always)]
    fn from(val: u8) -> Txdmaen {
        Txdmaen::from_bits(val)
    }
}
impl From<Txdmaen> for u8 {
    #[inline(always)]
    fn from(val: Txdmaen) -> u8 {
        Txdmaen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Unlock {
    #[doc = "LUT is locked (LUTCR\\[LOCK\\] must be 1)"]
    VALUE0 = 0x0,
    #[doc = "LUT is unlocked and can be written"]
    VALUE1 = 0x01,
}
impl Unlock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Unlock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Unlock {
    #[inline(always)]
    fn from(val: u8) -> Unlock {
        Unlock::from_bits(val)
    }
}
impl From<Unlock> for u8 {
    #[inline(always)]
    fn from(val: Unlock) -> u8 {
        Unlock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wa {
    #[doc = "Byte-addressable"]
    VALUE0 = 0x0,
    #[doc = "Word-addressable"]
    VALUE1 = 0x01,
}
impl Wa {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wa {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wa {
    #[inline(always)]
    fn from(val: u8) -> Wa {
        Wa::from_bits(val)
    }
}
impl From<Wa> for u8 {
    #[inline(always)]
    fn from(val: Wa) -> u8 {
        Wa::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmena {
    #[doc = "Disabled. When writing to external device, DQS(RWDS) pin is not driven."]
    VAL0 = 0x0,
    #[doc = "Enabled. When writing to external device, FlexSPI drives DQS(RWDS) pin as write mask output."]
    VAL1 = 0x01,
}
impl Wmena {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmena {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmena {
    #[inline(always)]
    fn from(val: u8) -> Wmena {
        Wmena::from_bits(val)
    }
}
impl From<Wmena> for u8 {
    #[inline(always)]
    fn from(val: Wmena) -> u8 {
        Wmena::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmenb {
    #[doc = "Disabled. When writing to external device, DQS(RWDS) pin is not driven."]
    VAL0 = 0x0,
    #[doc = "Enabled. When writing to external device, FlexSPI drives DQS(RWDS) pin as write mask output."]
    VAL1 = 0x01,
}
impl Wmenb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmenb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmenb {
    #[inline(always)]
    fn from(val: u8) -> Wmenb {
        Wmenb::from_bits(val)
    }
}
impl From<Wmenb> for u8 {
    #[inline(always)]
    fn from(val: Wmenb) -> u8 {
        Wmenb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Wmopt2 {
    #[doc = "When writing to an external device, DQS pin is used as write mask. When flash memory is accessed in individual mode, AHB or IP write burst length is not limited."]
    VAL0 = 0x0,
    #[doc = "When writing to an external device, DQS pin is not used as write mask. When flash memory is accessed in individual mode, AHB or IP write burst length is limited. The minimum write burst length should be 4."]
    VAL1 = 0x01,
}
impl Wmopt2 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wmopt2 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wmopt2 {
    #[inline(always)]
    fn from(val: u8) -> Wmopt2 {
        Wmopt2::from_bits(val)
    }
}
impl From<Wmopt2> for u8 {
    #[inline(always)]
    fn from(val: Wmopt2) -> u8 {
        Wmopt2::to_bits(val)
    }
}
