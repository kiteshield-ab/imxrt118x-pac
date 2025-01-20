#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisableAutoClkSwitch {
    #[doc = "If DISABLE_AUTO_CLK_SWITCH is set to 0 and 24M xtal is OK, the clock source will switch from internal ring OSC to 24M xtal automatically"]
    XtalClk = 0x0,
    #[doc = "If DISABLE_AUTO_CLK_SWITCH is set to 1, SEL_CLK will determine which clock source the DCDC uses"]
    SelClk = 0x01,
}
impl DisableAutoClkSwitch {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisableAutoClkSwitch {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisableAutoClkSwitch {
    #[inline(always)]
    fn from(val: u8) -> DisableAutoClkSwitch {
        DisableAutoClkSwitch::from_bits(val)
    }
}
impl From<DisableAutoClkSwitch> for u8 {
    #[inline(always)]
    fn from(val: DisableAutoClkSwitch) -> u8 {
        DisableAutoClkSwitch::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DisableIdleSkip {
    #[doc = "Enable the idle skip function."]
    Enable0 = 0x0,
    #[doc = "Disable the idle skip function."]
    Disable1 = 0x01,
}
impl DisableIdleSkip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> DisableIdleSkip {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for DisableIdleSkip {
    #[inline(always)]
    fn from(val: u8) -> DisableIdleSkip {
        DisableIdleSkip::from_bits(val)
    }
}
impl From<DisableIdleSkip> for u8 {
    #[inline(always)]
    fn from(val: DisableIdleSkip) -> u8 {
        DisableIdleSkip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnableFf {
    #[doc = "Enable the FF function."]
    Enable0 = 0x0,
    #[doc = "Disable the FF function."]
    Disable1 = 0x01,
}
impl EnableFf {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnableFf {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnableFf {
    #[inline(always)]
    fn from(val: u8) -> EnableFf {
        EnableFf::from_bits(val)
    }
}
impl From<EnableFf> for u8 {
    #[inline(always)]
    fn from(val: EnableFf) -> u8 {
        EnableFf::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum EnableOkCnt {
    #[doc = "Wait DCDC_OK for ACK"]
    Wait = 0x0,
    #[doc = "Enable internal count for DCDC_OK timeout"]
    EnableCount = 0x01,
}
impl EnableOkCnt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> EnableOkCnt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for EnableOkCnt {
    #[inline(always)]
    fn from(val: u8) -> EnableOkCnt {
        EnableOkCnt::from_bits(val)
    }
}
impl From<EnableOkCnt> for u8 {
    #[inline(always)]
    fn from(val: EnableOkCnt) -> u8 {
        EnableOkCnt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LpEn1p0 {
    #[doc = "DCDC 1P0 works in run mode. Its output voltage is controlled by VDD1P0CTRL_TRG."]
    Vdd1p0NormalMode = 0x0,
    #[doc = "DCDC 1P0 works in low power mode. Its output voltage is controlled by VDD1P0CTRL_LP_TRG and its output current is less than 50mA."]
    Vdd1p0LpMode = 0x01,
}
impl LpEn1p0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpEn1p0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpEn1p0 {
    #[inline(always)]
    fn from(val: u8) -> LpEn1p0 {
        LpEn1p0::from_bits(val)
    }
}
impl From<LpEn1p0> for u8 {
    #[inline(always)]
    fn from(val: LpEn1p0) -> u8 {
        LpEn1p0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MinpwrHalfFets {
    #[doc = "Donot use half switch FET."]
    NoHalfSFet = 0x0,
    #[doc = "Use half switch FET."]
    UseHalfSFet = 0x01,
}
impl MinpwrHalfFets {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MinpwrHalfFets {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MinpwrHalfFets {
    #[inline(always)]
    fn from(val: u8) -> MinpwrHalfFets {
        MinpwrHalfFets::from_bits(val)
    }
}
impl From<MinpwrHalfFets> for u8 {
    #[inline(always)]
    fn from(val: MinpwrHalfFets) -> u8 {
        MinpwrHalfFets::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OvercurTrigAdj {
    #[doc = "In Run Mode, 1.5 A. In LP Mode, 150 mA"]
    SelectZero = 0x0,
    #[doc = "In Run Mode, 1.5 A. In LP Mode, 130 mA"]
    SelectOne = 0x01,
    #[doc = "In Run Mode, 2 A. In LP Mode, 150 mA"]
    SelectTwo = 0x02,
    #[doc = "In Run Mode, 2 A. In LP Mode, 130 mA"]
    SelectThree = 0x03,
}
impl OvercurTrigAdj {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> OvercurTrigAdj {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for OvercurTrigAdj {
    #[inline(always)]
    fn from(val: u8) -> OvercurTrigAdj {
        OvercurTrigAdj::from_bits(val)
    }
}
impl From<OvercurTrigAdj> for u8 {
    #[inline(always)]
    fn from(val: OvercurTrigAdj) -> u8 {
        OvercurTrigAdj::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdCmpDcdcInDet {
    #[doc = "Low voltage detection comparator is enabled"]
    Enabled = 0x0,
    #[doc = "Low voltage detection comparator is disabled"]
    Disabled = 0x01,
}
impl PwdCmpDcdcInDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdCmpDcdcInDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdCmpDcdcInDet {
    #[inline(always)]
    fn from(val: u8) -> PwdCmpDcdcInDet {
        PwdCmpDcdcInDet::from_bits(val)
    }
}
impl From<PwdCmpDcdcInDet> for u8 {
    #[inline(always)]
    fn from(val: PwdCmpDcdcInDet) -> u8 {
        PwdCmpDcdcInDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdCmpOffset {
    #[doc = "Out-of-range comparator powered up"]
    PoweredUp = 0x0,
    #[doc = "Out-of-range comparator powered down"]
    PoweredDown = 0x01,
}
impl PwdCmpOffset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdCmpOffset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdCmpOffset {
    #[inline(always)]
    fn from(val: u8) -> PwdCmpOffset {
        PwdCmpOffset::from_bits(val)
    }
}
impl From<PwdCmpOffset> for u8 {
    #[inline(always)]
    fn from(val: PwdCmpOffset) -> u8 {
        PwdCmpOffset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdHighVdd1p0Det {
    #[doc = "Overvoltage detection comparator for the VDD1P0 output is enabled"]
    Enabled = 0x0,
    #[doc = "Overvoltage detection comparator for the VDD1P0 output is disabled"]
    Disabled = 0x01,
}
impl PwdHighVdd1p0Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdHighVdd1p0Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdHighVdd1p0Det {
    #[inline(always)]
    fn from(val: u8) -> PwdHighVdd1p0Det {
        PwdHighVdd1p0Det::from_bits(val)
    }
}
impl From<PwdHighVdd1p0Det> for u8 {
    #[inline(always)]
    fn from(val: PwdHighVdd1p0Det) -> u8 {
        PwdHighVdd1p0Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdHighVdd1p8Det {
    #[doc = "Overvoltage detection comparator for the VDD1P8 output is enabled"]
    Enabled = 0x0,
    #[doc = "Overvoltage detection comparator for the VDD1P8 output is disabled"]
    Disabled = 0x01,
}
impl PwdHighVdd1p8Det {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdHighVdd1p8Det {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdHighVdd1p8Det {
    #[inline(always)]
    fn from(val: u8) -> PwdHighVdd1p8Det {
        PwdHighVdd1p8Det::from_bits(val)
    }
}
impl From<PwdHighVdd1p8Det> for u8 {
    #[inline(always)]
    fn from(val: PwdHighVdd1p8Det) -> u8 {
        PwdHighVdd1p8Det::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdOscInt {
    #[doc = "Internal oscillator powered up"]
    PoweredUp = 0x0,
    #[doc = "Internal oscillator powered down"]
    PoweredDown = 0x01,
}
impl PwdOscInt {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdOscInt {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdOscInt {
    #[inline(always)]
    fn from(val: u8) -> PwdOscInt {
        PwdOscInt::from_bits(val)
    }
}
impl From<PwdOscInt> for u8 {
    #[inline(always)]
    fn from(val: PwdOscInt) -> u8 {
        PwdOscInt::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdOvercurDet {
    #[doc = "Overcurrent detection comparator is enabled"]
    Enabled = 0x0,
    #[doc = "Overcurrent detection comparator is disabled"]
    Disabled = 0x01,
}
impl PwdOvercurDet {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdOvercurDet {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdOvercurDet {
    #[inline(always)]
    fn from(val: u8) -> PwdOvercurDet {
        PwdOvercurDet::from_bits(val)
    }
}
impl From<PwdOvercurDet> for u8 {
    #[inline(always)]
    fn from(val: PwdOvercurDet) -> u8 {
        PwdOvercurDet::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwdZcd {
    #[doc = "Zero cross detection function powered up"]
    PoweredUp = 0x0,
    #[doc = "Zero cross detection function powered down"]
    PoweredDown = 0x01,
}
impl PwdZcd {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> PwdZcd {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for PwdZcd {
    #[inline(always)]
    fn from(val: u8) -> PwdZcd {
        PwdZcd::from_bits(val)
    }
}
impl From<PwdZcd> for u8 {
    #[inline(always)]
    fn from(val: PwdZcd) -> u8 {
        PwdZcd::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RloadRegEn {
    #[doc = "Resistor load disconnected"]
    RloadDisconnect = 0x0,
    #[doc = "Resistor load connected"]
    RloadConnect = 0x01,
}
impl RloadRegEn {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RloadRegEn {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RloadRegEn {
    #[inline(always)]
    fn from(val: u8) -> RloadRegEn {
        RloadRegEn::from_bits(val)
    }
}
impl From<RloadRegEn> for u8 {
    #[inline(always)]
    fn from(val: RloadRegEn) -> u8 {
        RloadRegEn::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SelClk {
    #[doc = "DCDC uses internal ring oscillator"]
    IntRngOsc = 0x0,
    #[doc = "DCDC uses 24M xtal"]
    Xtal24m = 0x01,
}
impl SelClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SelClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SelClk {
    #[inline(always)]
    fn from(val: u8) -> SelClk {
        SelClk::from_bits(val)
    }
}
impl From<SelClk> for u8 {
    #[inline(always)]
    fn from(val: SelClk) -> u8 {
        SelClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg0AuthenLockList {
    #[doc = "WHITE_LIST value can be changed."]
    Change = 0x0,
    #[doc = "LOCK_LIST and WHITE_LIST value cannot be changed."]
    NoChange = 0x01,
}
impl Trg0AuthenLockList {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg0AuthenLockList {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg0AuthenLockList {
    #[inline(always)]
    fn from(val: u8) -> Trg0AuthenLockList {
        Trg0AuthenLockList::from_bits(val)
    }
}
impl From<Trg0AuthenLockList> for u8 {
    #[inline(always)]
    fn from(val: Trg0AuthenLockList) -> u8 {
        Trg0AuthenLockList::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg0AuthenLockTz {
    #[doc = "TZ_NS and TZ_USER value can be changed."]
    Change = 0x0,
    #[doc = "LOCK_TZ, TZ_NS and TZ_USER value cannot be changed."]
    NoChange = 0x01,
}
impl Trg0AuthenLockTz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg0AuthenLockTz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg0AuthenLockTz {
    #[inline(always)]
    fn from(val: u8) -> Trg0AuthenLockTz {
        Trg0AuthenLockTz::from_bits(val)
    }
}
impl From<Trg0AuthenLockTz> for u8 {
    #[inline(always)]
    fn from(val: Trg0AuthenLockTz) -> u8 {
        Trg0AuthenLockTz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg0AuthenTzNs {
    #[doc = "TRG_0 registers can only be written in secure mode."]
    SecureOnly = 0x0,
    #[doc = "TRG_0 registers can be written either in secure mode or non-secure mode."]
    SecOrNonSec = 0x01,
}
impl Trg0AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg0AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg0AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> Trg0AuthenTzNs {
        Trg0AuthenTzNs::from_bits(val)
    }
}
impl From<Trg0AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: Trg0AuthenTzNs) -> u8 {
        Trg0AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg0AuthenTzUser {
    #[doc = "TRG_0 registers can only be written in privilege mode."]
    PrivOnly = 0x0,
    #[doc = "TRG_0 registers can be written either in privilege mode or user mode."]
    PrivOrUser = 0x01,
}
impl Trg0AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg0AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg0AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> Trg0AuthenTzUser {
        Trg0AuthenTzUser::from_bits(val)
    }
}
impl From<Trg0AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: Trg0AuthenTzUser) -> u8 {
        Trg0AuthenTzUser::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trg0AuthenWhiteList(pub u16);
impl Trg0AuthenWhiteList {
    #[doc = "Core with domain ID=0 can write TRG_0 registers."]
    pub const DOMAIN0: Self = Self(0x01);
    #[doc = "Core with domain ID=1 can write TRG_0 registers."]
    pub const DOMAIN1: Self = Self(0x02);
    #[doc = "Core with domain ID=2 can write TRG_0 registers."]
    pub const DOMAIN2: Self = Self(0x04);
    #[doc = "Core with domain ID=3 can write TRG_0 registers."]
    pub const DOMAIN3: Self = Self(0x08);
    #[doc = "Core with domain ID=4 can write TRG_0 registers."]
    pub const DOMAIN4: Self = Self(0x10);
    #[doc = "Core with domain ID=5 can write TRG_0 registers."]
    pub const DOMAIN5: Self = Self(0x20);
    #[doc = "Core with domain ID=6 can write TRG_0 registers."]
    pub const DOMAIN6: Self = Self(0x40);
    #[doc = "Core with domain ID=7 can write TRG_0 registers."]
    pub const DOMAIN7: Self = Self(0x80);
    #[doc = "Core with domain ID=8 can write TRG_0 registers."]
    pub const DOMAIN8: Self = Self(0x0100);
    #[doc = "Core with domain ID=9 can write TRG_0 registers."]
    pub const DOMAIN9: Self = Self(0x0200);
    #[doc = "Core with domain ID=10 can write TRG_0 registers."]
    pub const DOMAIN10: Self = Self(0x0400);
    #[doc = "Core with domain ID=11 can write TRG_0 registers."]
    pub const DOMAIN11: Self = Self(0x0800);
    #[doc = "Core with domain ID=12 can write TRG_0 registers."]
    pub const DOMAIN12: Self = Self(0x1000);
    #[doc = "Core with domain ID=13 can write TRG_0 registers."]
    pub const DOMAIN13: Self = Self(0x2000);
    #[doc = "Core with domain ID=14 can write TRG_0 registers."]
    pub const DOMAIN14: Self = Self(0x4000);
    #[doc = "Core with domain ID=15 can write TRG_0 registers."]
    pub const DOMAIN15: Self = Self(0x8000);
}
impl Trg0AuthenWhiteList {
    pub const fn from_bits(val: u16) -> Trg0AuthenWhiteList {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Trg0AuthenWhiteList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("DOMAIN0"),
            0x02 => f.write_str("DOMAIN1"),
            0x04 => f.write_str("DOMAIN2"),
            0x08 => f.write_str("DOMAIN3"),
            0x10 => f.write_str("DOMAIN4"),
            0x20 => f.write_str("DOMAIN5"),
            0x40 => f.write_str("DOMAIN6"),
            0x80 => f.write_str("DOMAIN7"),
            0x0100 => f.write_str("DOMAIN8"),
            0x0200 => f.write_str("DOMAIN9"),
            0x0400 => f.write_str("DOMAIN10"),
            0x0800 => f.write_str("DOMAIN11"),
            0x1000 => f.write_str("DOMAIN12"),
            0x2000 => f.write_str("DOMAIN13"),
            0x4000 => f.write_str("DOMAIN14"),
            0x8000 => f.write_str("DOMAIN15"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trg0AuthenWhiteList {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "DOMAIN0"),
            0x02 => defmt::write!(f, "DOMAIN1"),
            0x04 => defmt::write!(f, "DOMAIN2"),
            0x08 => defmt::write!(f, "DOMAIN3"),
            0x10 => defmt::write!(f, "DOMAIN4"),
            0x20 => defmt::write!(f, "DOMAIN5"),
            0x40 => defmt::write!(f, "DOMAIN6"),
            0x80 => defmt::write!(f, "DOMAIN7"),
            0x0100 => defmt::write!(f, "DOMAIN8"),
            0x0200 => defmt::write!(f, "DOMAIN9"),
            0x0400 => defmt::write!(f, "DOMAIN10"),
            0x0800 => defmt::write!(f, "DOMAIN11"),
            0x1000 => defmt::write!(f, "DOMAIN12"),
            0x2000 => defmt::write!(f, "DOMAIN13"),
            0x4000 => defmt::write!(f, "DOMAIN14"),
            0x8000 => defmt::write!(f, "DOMAIN15"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Trg0AuthenWhiteList {
    #[inline(always)]
    fn from(val: u16) -> Trg0AuthenWhiteList {
        Trg0AuthenWhiteList::from_bits(val)
    }
}
impl From<Trg0AuthenWhiteList> for u16 {
    #[inline(always)]
    fn from(val: Trg0AuthenWhiteList) -> u16 {
        Trg0AuthenWhiteList::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg1AuthenLockList {
    #[doc = "WHITE_LIST value can be changed."]
    Change = 0x0,
    #[doc = "LOCK_LIST and WHITE_LIST value cannot be changed."]
    NoChange = 0x01,
}
impl Trg1AuthenLockList {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg1AuthenLockList {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg1AuthenLockList {
    #[inline(always)]
    fn from(val: u8) -> Trg1AuthenLockList {
        Trg1AuthenLockList::from_bits(val)
    }
}
impl From<Trg1AuthenLockList> for u8 {
    #[inline(always)]
    fn from(val: Trg1AuthenLockList) -> u8 {
        Trg1AuthenLockList::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg1AuthenLockTz {
    #[doc = "TZ_NS and TZ_USER value can be changed."]
    Change = 0x0,
    #[doc = "LOCK_TZ, TZ_NS and TZ_USER value cannot be changed."]
    NoChange = 0x01,
}
impl Trg1AuthenLockTz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg1AuthenLockTz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg1AuthenLockTz {
    #[inline(always)]
    fn from(val: u8) -> Trg1AuthenLockTz {
        Trg1AuthenLockTz::from_bits(val)
    }
}
impl From<Trg1AuthenLockTz> for u8 {
    #[inline(always)]
    fn from(val: Trg1AuthenLockTz) -> u8 {
        Trg1AuthenLockTz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg1AuthenTzNs {
    #[doc = "TRG_1 registers can only be written in secure mode."]
    SecureOnly = 0x0,
    #[doc = "TRG_1 registers can be written either in secure mode or non-secure mode."]
    SecOrNonSec = 0x01,
}
impl Trg1AuthenTzNs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg1AuthenTzNs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg1AuthenTzNs {
    #[inline(always)]
    fn from(val: u8) -> Trg1AuthenTzNs {
        Trg1AuthenTzNs::from_bits(val)
    }
}
impl From<Trg1AuthenTzNs> for u8 {
    #[inline(always)]
    fn from(val: Trg1AuthenTzNs) -> u8 {
        Trg1AuthenTzNs::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Trg1AuthenTzUser {
    #[doc = "TRG_1 registers can only be written in privilege mode."]
    PrivOnly = 0x0,
    #[doc = "TRG_1 registers can be written either in privilege mode or user mode."]
    PrivOrUser = 0x01,
}
impl Trg1AuthenTzUser {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Trg1AuthenTzUser {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Trg1AuthenTzUser {
    #[inline(always)]
    fn from(val: u8) -> Trg1AuthenTzUser {
        Trg1AuthenTzUser::from_bits(val)
    }
}
impl From<Trg1AuthenTzUser> for u8 {
    #[inline(always)]
    fn from(val: Trg1AuthenTzUser) -> u8 {
        Trg1AuthenTzUser::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Trg1AuthenWhiteList(pub u16);
impl Trg1AuthenWhiteList {
    #[doc = "Core with domain ID=0 can write TRG_1 registers."]
    pub const DOMAIN0: Self = Self(0x01);
    #[doc = "Core with domain ID=1 can write TRG_1 registers."]
    pub const DOMAIN1: Self = Self(0x02);
    #[doc = "Core with domain ID=2 can write TRG_1 registers."]
    pub const DOMAIN2: Self = Self(0x04);
    #[doc = "Core with domain ID=3 can write TRG_1 registers."]
    pub const DOMAIN3: Self = Self(0x08);
    #[doc = "Core with domain ID=4 can write TRG_1 registers."]
    pub const DOMAIN4: Self = Self(0x10);
    #[doc = "Core with domain ID=5 can write TRG_1 registers."]
    pub const DOMAIN5: Self = Self(0x20);
    #[doc = "Core with domain ID=6 can write TRG_1 registers."]
    pub const DOMAIN6: Self = Self(0x40);
    #[doc = "Core with domain ID=7 can write TRG_1 registers."]
    pub const DOMAIN7: Self = Self(0x80);
    #[doc = "Core with domain ID=8 can write TRG_1 registers."]
    pub const DOMAIN8: Self = Self(0x0100);
    #[doc = "Core with domain ID=9 can write TRG_1 registers."]
    pub const DOMAIN9: Self = Self(0x0200);
    #[doc = "Core with domain ID=10 can write TRG_1 registers."]
    pub const DOMAIN10: Self = Self(0x0400);
    #[doc = "Core with domain ID=11 can write TRG_1 registers."]
    pub const DOMAIN11: Self = Self(0x0800);
    #[doc = "Core with domain ID=12 can write TRG_1 registers."]
    pub const DOMAIN12: Self = Self(0x1000);
    #[doc = "Core with domain ID=13 can write TRG_1 registers."]
    pub const DOMAIN13: Self = Self(0x2000);
    #[doc = "Core with domain ID=14 can write TRG_1 registers."]
    pub const DOMAIN14: Self = Self(0x4000);
    #[doc = "Core with domain ID=15 can write TRG_1 registers."]
    pub const DOMAIN15: Self = Self(0x8000);
}
impl Trg1AuthenWhiteList {
    pub const fn from_bits(val: u16) -> Trg1AuthenWhiteList {
        Self(val & 0xffff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl core::fmt::Debug for Trg1AuthenWhiteList {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x01 => f.write_str("DOMAIN0"),
            0x02 => f.write_str("DOMAIN1"),
            0x04 => f.write_str("DOMAIN2"),
            0x08 => f.write_str("DOMAIN3"),
            0x10 => f.write_str("DOMAIN4"),
            0x20 => f.write_str("DOMAIN5"),
            0x40 => f.write_str("DOMAIN6"),
            0x80 => f.write_str("DOMAIN7"),
            0x0100 => f.write_str("DOMAIN8"),
            0x0200 => f.write_str("DOMAIN9"),
            0x0400 => f.write_str("DOMAIN10"),
            0x0800 => f.write_str("DOMAIN11"),
            0x1000 => f.write_str("DOMAIN12"),
            0x2000 => f.write_str("DOMAIN13"),
            0x4000 => f.write_str("DOMAIN14"),
            0x8000 => f.write_str("DOMAIN15"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Trg1AuthenWhiteList {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x01 => defmt::write!(f, "DOMAIN0"),
            0x02 => defmt::write!(f, "DOMAIN1"),
            0x04 => defmt::write!(f, "DOMAIN2"),
            0x08 => defmt::write!(f, "DOMAIN3"),
            0x10 => defmt::write!(f, "DOMAIN4"),
            0x20 => defmt::write!(f, "DOMAIN5"),
            0x40 => defmt::write!(f, "DOMAIN6"),
            0x80 => defmt::write!(f, "DOMAIN7"),
            0x0100 => defmt::write!(f, "DOMAIN8"),
            0x0200 => defmt::write!(f, "DOMAIN9"),
            0x0400 => defmt::write!(f, "DOMAIN10"),
            0x0800 => defmt::write!(f, "DOMAIN11"),
            0x1000 => defmt::write!(f, "DOMAIN12"),
            0x2000 => defmt::write!(f, "DOMAIN13"),
            0x4000 => defmt::write!(f, "DOMAIN14"),
            0x8000 => defmt::write!(f, "DOMAIN15"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u16> for Trg1AuthenWhiteList {
    #[inline(always)]
    fn from(val: u16) -> Trg1AuthenWhiteList {
        Trg1AuthenWhiteList::from_bits(val)
    }
}
impl From<Trg1AuthenWhiteList> for u16 {
    #[inline(always)]
    fn from(val: Trg1AuthenWhiteList) -> u16 {
        Trg1AuthenWhiteList::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrimHold {
    #[doc = "Sample trim value from FUSE or value from REG1\\[VBG_TRIM\\] depending on FUSE select bit."]
    Sample = 0x0,
    #[doc = "Use value from REG1\\[VBG_TRIM\\] as trim value."]
    Hold = 0x01,
}
impl TrimHold {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimHold {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimHold {
    #[inline(always)]
    fn from(val: u8) -> TrimHold {
        TrimHold::from_bits(val)
    }
}
impl From<TrimHold> for u8 {
    #[inline(always)]
    fn from(val: TrimHold) -> u8 {
        TrimHold::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Vdd1p0ctrlDisableStep {
    #[doc = "Enable stepping for VDD1P0"]
    Enable = 0x0,
    #[doc = "Disable stepping for VDD1P0"]
    Disable = 0x01,
}
impl Vdd1p0ctrlDisableStep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Vdd1p0ctrlDisableStep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Vdd1p0ctrlDisableStep {
    #[inline(always)]
    fn from(val: u8) -> Vdd1p0ctrlDisableStep {
        Vdd1p0ctrlDisableStep::from_bits(val)
    }
}
impl From<Vdd1p0ctrlDisableStep> for u8 {
    #[inline(always)]
    fn from(val: Vdd1p0ctrlDisableStep) -> u8 {
        Vdd1p0ctrlDisableStep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Xtal24mOk {
    #[doc = "DCDC uses internal ring OSC"]
    IntRngOsc = 0x0,
    #[doc = "DCDC uses xtal 24M"]
    Xtal24m = 0x01,
}
impl Xtal24mOk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Xtal24mOk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Xtal24mOk {
    #[inline(always)]
    fn from(val: u8) -> Xtal24mOk {
        Xtal24mOk::from_bits(val)
    }
}
impl From<Xtal24mOk> for u8 {
    #[inline(always)]
    fn from(val: Xtal24mOk) -> u8 {
        Xtal24mOk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum XtalokDisable {
    #[doc = "Enable xtalok detection circuit"]
    Enabled = 0x0,
    #[doc = "Disable xtalok detection circuit and always outputs OK signal \"1\""]
    Disabled = 0x01,
}
impl XtalokDisable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> XtalokDisable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for XtalokDisable {
    #[inline(always)]
    fn from(val: u8) -> XtalokDisable {
        XtalokDisable::from_bits(val)
    }
}
impl From<XtalokDisable> for u8 {
    #[inline(always)]
    fn from(val: XtalokDisable) -> u8 {
        XtalokDisable::to_bits(val)
    }
}
