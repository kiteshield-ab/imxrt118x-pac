#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Buf21en {
    #[doc = "buffer is disabled"]
    BUFFER_DIS = 0x0,
    #[doc = "buffer is enabled"]
    BUFFER_ENB = 0x01,
}
impl Buf21en {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Buf21en {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Buf21en {
    #[inline(always)]
    fn from(val: u8) -> Buf21en {
        Buf21en::from_bits(val)
    }
}
impl From<Buf21en> for u8 {
    #[inline(always)]
    fn from(val: Buf21en) -> u8 {
        Buf21en::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Chopen {
    #[doc = "Chop oscillator is disabled."]
    CHP_OSC_DIS = 0x0,
    #[doc = "Chop oscillator is enabled."]
    CHP_OSC_ENB = 0x01,
}
impl Chopen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Chopen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Chopen {
    #[inline(always)]
    fn from(val: u8) -> Chopen {
        Chopen::from_bits(val)
    }
}
impl From<Chopen> for u8 {
    #[inline(always)]
    fn from(val: Chopen) -> u8 {
        Chopen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Hcbgen {
    #[doc = "HC Bandgap is disabled"]
    HC_BG_DIS = 0x0,
    #[doc = "HC Bandgap is enabled"]
    HC_BG_ENB = 0x01,
}
impl Hcbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Hcbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Hcbgen {
    #[inline(always)]
    fn from(val: u8) -> Hcbgen {
        Hcbgen::from_bits(val)
    }
}
impl From<Hcbgen> for u8 {
    #[inline(always)]
    fn from(val: Hcbgen) -> u8 {
        Hcbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum HiPwrLv {
    #[doc = "buffer is in low power mode"]
    LOW_MODE = 0x0,
    #[doc = "buffer is in high power mode"]
    HIGH_MODE = 0x01,
}
impl HiPwrLv {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HiPwrLv {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HiPwrLv {
    #[inline(always)]
    fn from(val: u8) -> HiPwrLv {
        HiPwrLv::from_bits(val)
    }
}
impl From<HiPwrLv> for u8 {
    #[inline(always)]
    fn from(val: HiPwrLv) -> u8 {
        HiPwrLv::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Icompen {
    #[doc = "Disabled"]
    DIS = 0x0,
    #[doc = "Enabled"]
    ENB = 0x01,
}
impl Icompen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Icompen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Icompen {
    #[inline(always)]
    fn from(val: u8) -> Icompen {
        Icompen::from_bits(val)
    }
}
impl From<Icompen> for u8 {
    #[inline(always)]
    fn from(val: Icompen) -> u8 {
        Icompen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Lpbgen {
    #[doc = "LP Bandgap is disabled"]
    LP_BG_DIS = 0x0,
    #[doc = "LP Bandgap is enabled"]
    LP_BG_ENB = 0x01,
}
impl Lpbgen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lpbgen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lpbgen {
    #[inline(always)]
    fn from(val: u8) -> Lpbgen {
        Lpbgen::from_bits(val)
    }
}
impl From<Lpbgen> for u8 {
    #[inline(always)]
    fn from(val: Lpbgen) -> u8 {
        Lpbgen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Regen {
    #[doc = "Internal 1.75 V regulator is disabled."]
    INT_REG_DIS = 0x0,
    #[doc = "Internal 1.75 V regulator is enabled."]
    INT_REG_ENB = 0x01,
}
impl Regen {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Regen {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Regen {
    #[inline(always)]
    fn from(val: u8) -> Regen {
        Regen::from_bits(val)
    }
}
impl From<Regen> for u8 {
    #[inline(always)]
    fn from(val: Regen) -> u8 {
        Regen::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vrefst {
    #[doc = "The module is disabled or not stable."]
    MOD_DISABLE = 0x0,
    #[doc = "The module is stable."]
    MOD_STABLE = 0x01,
}
impl Vrefst {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vrefst {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vrefst {
    #[inline(always)]
    fn from(val: u8) -> Vrefst {
        Vrefst::from_bits(val)
    }
}
impl From<Vrefst> for u8 {
    #[inline(always)]
    fn from(val: Vrefst) -> u8 {
        Vrefst::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Vreftrim(pub u8);
impl Vreftrim {
    #[doc = "Min"]
    pub const MIN: Self = Self(0x0);
    #[doc = "Max-31*(4/3) mV"]
    pub const MAX_31: Self = Self(0x01);
    #[doc = "Max"]
    pub const MAX: Self = Self(0x3f);
}
impl Vreftrim {
    pub const fn from_bits(val: u8) -> Vreftrim {
        Self(val & 0x3f)
    }
    pub const fn to_bits(self) -> u8 {
        self.0
    }
}
impl core::fmt::Debug for Vreftrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x0 => f.write_str("MIN"),
            0x01 => f.write_str("MAX_31"),
            0x3f => f.write_str("MAX"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vreftrim {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x0 => defmt::write!(f, "MIN"),
            0x01 => defmt::write!(f, "MAX_31"),
            0x3f => defmt::write!(f, "MAX"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u8> for Vreftrim {
    #[inline(always)]
    fn from(val: u8) -> Vreftrim {
        Vreftrim::from_bits(val)
    }
}
impl From<Vreftrim> for u8 {
    #[inline(always)]
    fn from(val: Vreftrim) -> u8 {
        Vreftrim::to_bits(val)
    }
}
