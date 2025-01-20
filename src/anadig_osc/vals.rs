#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Clkgate400meg {
    #[doc = "Not Gated"]
    Ng = 0x0,
    #[doc = "Gated"]
    Gate = 0x01,
}
impl Clkgate400meg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Clkgate400meg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Clkgate400meg {
    #[inline(always)]
    fn from(val: u8) -> Clkgate400meg {
        Clkgate400meg::from_bits(val)
    }
}
impl From<Clkgate400meg> for u8 {
    #[inline(always)]
    fn from(val: Clkgate400meg) -> u8 {
        Clkgate400meg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpEn {
    #[doc = "High Gain mode (HP)"]
    Hp = 0x0,
    #[doc = "Low-power mode (LP)"]
    Lp = 0x01,
}
impl LpEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpEn {
    #[inline(always)]
    fn from(val: u8) -> LpEn {
        LpEn::from_bits(val)
    }
}
impl From<LpEn> for u8 {
    #[inline(always)]
    fn from(val: LpEn) -> u8 {
        LpEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc24mControlMode {
    #[doc = "Software mode (default)"]
    Sw = 0x0,
    #[doc = "GPC mode"]
    Gpc = 0x01,
}
impl Osc24mControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc24mControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc24mControlMode {
    #[inline(always)]
    fn from(val: u8) -> Osc24mControlMode {
        Osc24mControlMode::from_bits(val)
    }
}
impl From<Osc24mControlMode> for u8 {
    #[inline(always)]
    fn from(val: Osc24mControlMode) -> u8 {
        Osc24mControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc24mGate {
    #[doc = "Not Gated"]
    Ng = 0x0,
    #[doc = "Gated"]
    Gate = 0x01,
}
impl Osc24mGate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc24mGate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc24mGate {
    #[inline(always)]
    fn from(val: u8) -> Osc24mGate {
        Osc24mGate::from_bits(val)
    }
}
impl From<Osc24mGate> for u8 {
    #[inline(always)]
    fn from(val: Osc24mGate) -> u8 {
        Osc24mGate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Osc24mStable {
    #[doc = "Not Stable"]
    Ns = 0x0,
    #[doc = "Stable"]
    Stable = 0x01,
}
impl Osc24mStable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Osc24mStable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Osc24mStable {
    #[inline(always)]
    fn from(val: u8) -> Osc24mStable {
        Osc24mStable::from_bits(val)
    }
}
impl From<Osc24mStable> for u8 {
    #[inline(always)]
    fn from(val: Osc24mStable) -> u8 {
        Osc24mStable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OscCompMode {
    #[doc = "Single-ended mode (default)"]
    Single = 0x0,
    #[doc = "Differential mode (test mode)"]
    Diff = 0x01,
}
impl OscCompMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OscCompMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OscCompMode {
    #[inline(always)]
    fn from(val: u8) -> OscCompMode {
        OscCompMode::from_bits(val)
    }
}
impl From<OscCompMode> for u8 {
    #[inline(always)]
    fn from(val: OscCompMode) -> u8 {
        OscCompMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pwd {
    #[doc = "No Power down"]
    Pd = 0x0,
    #[doc = "Power down"]
    Pu = 0x01,
}
impl Pwd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pwd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pwd {
    #[inline(always)]
    fn from(val: u8) -> Pwd {
        Pwd::from_bits(val)
    }
}
impl From<Pwd> for u8 {
    #[inline(always)]
    fn from(val: Pwd) -> u8 {
        Pwd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rc24mControlMode {
    #[doc = "Software mode (default)"]
    Sw = 0x0,
    #[doc = "GPC mode"]
    Gpc = 0x01,
}
impl Rc24mControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rc24mControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rc24mControlMode {
    #[inline(always)]
    fn from(val: u8) -> Rc24mControlMode {
        Rc24mControlMode::from_bits(val)
    }
}
impl From<Rc24mControlMode> for u8 {
    #[inline(always)]
    fn from(val: Rc24mControlMode) -> u8 {
        Rc24mControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Rc400mControlMode {
    #[doc = "Software mode (default)"]
    Sw = 0x0,
    #[doc = "GPC mode"]
    Gpc = 0x01,
}
impl Rc400mControlMode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Rc400mControlMode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Rc400mControlMode {
    #[inline(always)]
    fn from(val: u8) -> Rc400mControlMode {
        Rc400mControlMode::from_bits(val)
    }
}
impl From<Rc400mControlMode> for u8 {
    #[inline(always)]
    fn from(val: Rc400mControlMode) -> u8 {
        Rc400mControlMode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SourceSel24m {
    #[doc = "OSC_RC24M"]
    OscRc24m = 0x0,
    #[doc = "OSC24M"]
    Osc24m = 0x01,
}
impl SourceSel24m {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SourceSel24m {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SourceSel24m {
    #[inline(always)]
    fn from(val: u8) -> SourceSel24m {
        SourceSel24m::from_bits(val)
    }
}
impl From<SourceSel24m> for u8 {
    #[inline(always)]
    fn from(val: SourceSel24m) -> u8 {
        SourceSel24m::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Ten {
    #[doc = "Power down"]
    Pd = 0x0,
    #[doc = "Power up"]
    Pu = 0x01,
}
impl Ten {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ten {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ten {
    #[inline(always)]
    fn from(val: u8) -> Ten {
        Ten::from_bits(val)
    }
}
impl From<Ten> for u8 {
    #[inline(always)]
    fn from(val: Ten) -> u8 {
        Ten::to_bits(val)
    }
}
